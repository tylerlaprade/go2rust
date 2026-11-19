package main

import "go/types"

// A program made of several packages gets one copy of the interface-value
// helpers (format_any, go_any_clone, the %T names) per package, and a copy
// starts out knowing only the types its own package puts into an interface.
// A value does not stay in the package that boxed it. main hands an error to
// a logging package, and the logging package prints it.
//
// programAnyCases collects, while the packages of one program are translated,
// the cases that every copy of a helper can carry. Each helper is then
// written with its own package's cases plus the collected ones.
//
// Two limits, both deliberate:
//
// Only a case whose type is spelled with standard types can be shared this
// way. Each package is a separate crate, and a dependency cannot name a type
// that its caller declares. A struct boxed in main therefore still prints
// <unknown> in another package; that needs a registry filled at run time.
//
// Only the program's own packages take part: main and the packages it
// imports that are not the standard library. A standard-library package
// translated from source keeps exactly the cases it had. Those packages
// exchange interface values too, but giving every one of them every other
// one's cases rewrites the helpers of most of the translated library for a
// flow no fixture exercises yet.
type programAnyCases struct {
	cloneTypes map[string]bool
	formatArms map[string]anyFormatArm
	typeNames  map[string]string

	// currentPackage is the import path of the dependency being translated.
	// It is empty while the main package is translated.
	currentPackage string
	// currentPackageIsStandardLibrary says the dependency being translated
	// is a standard-library package translated from source.
	currentPackageIsStandardLibrary bool

	// written records, per dependency, how many shared cases each of its
	// helpers had when it was written. A dependency is translated before the
	// packages that import it, so a later package can add a case that an
	// earlier helper was written without.
	written map[string]*programAnyCasesWritten
}

type programAnyCasesWritten struct {
	hasClone, hasFormat, hasTypeName  bool
	cloneTypes, formatArms, typeNames int
}

// currentProgramAnyCases is nil unless a program of several packages is
// being translated.
var currentProgramAnyCases *programAnyCases

func beginProgramAnyCases() {
	currentProgramAnyCases = &programAnyCases{
		cloneTypes: make(map[string]bool),
		formatArms: make(map[string]anyFormatArm),
		typeNames:  make(map[string]string),
		written:    make(map[string]*programAnyCasesWritten),
	}
}

func endProgramAnyCases() {
	currentProgramAnyCases = nil
}

// takesPart reports whether the helpers being written now, or the value
// being boxed now, belong to one of the program's own packages. The crate of
// helpers every package shares is written with generatingPublicHelpers set
// and is left as it was.
func (shared *programAnyCases) takesPart() bool {
	return !shared.currentPackageIsStandardLibrary && !generatingPublicHelpers
}

// recordProgramAnyCase adds the cases for one boxed payload type to the
// program's shared set. The caller has established from the Go type that the
// payload is the same Rust type in every package.
func recordProgramAnyCase(rustType string, arm anyFormatArm, hasArm bool, goName string) {
	shared := currentProgramAnyCases
	if shared == nil || !shared.takesPart() || !anyPayloadTypeUsesOnlyStandardNames(rustType) {
		return
	}
	shared.cloneTypes[rustType] = true
	if hasArm {
		shared.formatArms[rustType] = arm
	}
	if goName != "" {
		if existing, seen := shared.typeNames[rustType]; !seen || goName < existing {
			shared.typeNames[rustType] = goName
		}
	}
}

// casesFor returns the cases to write into ht's helpers: its own, plus the
// shared ones for each helper it has. It never changes ht's own maps.
func (shared *programAnyCases) casesFor(ht *HelperTracker) (map[string]bool, map[string]anyFormatArm, map[string]string) {
	cloneTypes, formatArms, typeNames := ht.anyCloneTypes, ht.anyFormatArms, ht.anyTypeNames
	if !shared.takesPart() {
		return cloneTypes, formatArms, typeNames
	}
	if ht.needsAnyClone && len(shared.cloneTypes) > 0 {
		cloneTypes = make(map[string]bool, len(shared.cloneTypes)+len(ht.anyCloneTypes))
		for rustType := range shared.cloneTypes {
			cloneTypes[rustType] = true
		}
		for rustType := range ht.anyCloneTypes {
			cloneTypes[rustType] = true
		}
	}
	if ht.needsFormatAny && len(shared.formatArms) > 0 {
		formatArms = make(map[string]anyFormatArm, len(shared.formatArms)+len(ht.anyFormatArms))
		for rustType, arm := range shared.formatArms {
			formatArms[rustType] = arm
		}
		for rustType, arm := range ht.anyFormatArms {
			formatArms[rustType] = arm
		}
	}
	if ht.needsGoTypeName && len(shared.typeNames) > 0 {
		typeNames = make(map[string]string, len(shared.typeNames)+len(ht.anyTypeNames))
		for rustType, goName := range shared.typeNames {
			typeNames[rustType] = goName
		}
		for rustType, goName := range ht.anyTypeNames {
			typeNames[rustType] = goName
		}
	}
	shared.noteWritten(ht)
	return cloneTypes, formatArms, typeNames
}

func (shared *programAnyCases) noteWritten(ht *HelperTracker) {
	if shared.currentPackage == "" || !(ht.needsAnyClone || ht.needsFormatAny || ht.needsGoTypeName) {
		return
	}
	written := shared.written[shared.currentPackage]
	if written == nil {
		written = &programAnyCasesWritten{}
		shared.written[shared.currentPackage] = written
	}
	// A package can write helpers more than once, one copy per file. Keep
	// the smallest count: the copy that knew the least decides.
	if ht.needsAnyClone {
		if !written.hasClone || len(shared.cloneTypes) < written.cloneTypes {
			written.cloneTypes = len(shared.cloneTypes)
		}
		written.hasClone = true
	}
	if ht.needsFormatAny {
		if !written.hasFormat || len(shared.formatArms) < written.formatArms {
			written.formatArms = len(shared.formatArms)
		}
		written.hasFormat = true
	}
	if ht.needsGoTypeName {
		if !written.hasTypeName || len(shared.typeNames) < written.typeNames {
			written.typeNames = len(shared.typeNames)
		}
		written.hasTypeName = true
	}
}

// packageIsMissingCases reports whether a helper of the dependency pkgPath
// was written before some shared case it should carry was known. The shared
// sets only grow, so a smaller count then than now means a case is missing.
func (shared *programAnyCases) packageIsMissingCases(pkgPath string) bool {
	written := shared.written[pkgPath]
	if written == nil {
		return false
	}
	return (written.hasClone && written.cloneTypes < len(shared.cloneTypes)) ||
		(written.hasFormat && written.formatArms < len(shared.formatArms)) ||
		(written.hasTypeName && written.typeNames < len(shared.typeNames))
}

// anyPayloadGoTypeIsShareable reports whether a value of Go type typ has the
// same Rust type in every generated crate. That holds for a type built only
// from predeclared types: the basic types, error, and pointers, slices,
// arrays, maps and functions of those. A declared type belongs to its
// package's crate. An anonymous struct and a channel are lowered to a type
// that each crate generates for itself.
func anyPayloadGoTypeIsShareable(typ types.Type) bool {
	switch t := types.Unalias(typ).(type) {
	case *types.Basic:
		return t.Kind() != types.Invalid && t.Kind() != types.UnsafePointer &&
			t.Info()&(types.IsUntyped|types.IsComplex) == 0
	case *types.Named:
		return isErrorInterfaceType(t)
	case *types.Interface:
		return t.Empty()
	case *types.Pointer:
		return anyPayloadGoTypeIsShareable(t.Elem())
	case *types.Slice:
		return anyPayloadGoTypeIsShareable(t.Elem())
	case *types.Array:
		return anyPayloadGoTypeIsShareable(t.Elem())
	case *types.Map:
		return anyPayloadGoTypeIsShareable(t.Key()) && anyPayloadGoTypeIsShareable(t.Elem())
	case *types.Signature:
		if t.Recv() != nil || t.TypeParams().Len() > 0 {
			return false
		}
		for _, tuple := range []*types.Tuple{t.Params(), t.Results()} {
			for i := 0; i < tuple.Len(); i++ {
				if !anyPayloadGoTypeIsShareable(tuple.At(i).Type()) {
					return false
				}
			}
		}
		return true
	}
	return false
}

// anyPayloadTypeUsesOnlyStandardNames reports whether rustType is spelled
// only with names that mean the same type in every generated crate: the
// primitive types and the standard library's. go/types decides which Go types
// can be shared (anyPayloadGoTypeIsShareable). This is the check on the other
// side of the lowering, so that a helper type one crate generates for itself
// is never written into another crate's helper. A name this function does
// not know keeps the case in the package it came from.
func anyPayloadTypeUsesOnlyStandardNames(rustType string) bool {
	if rustType == "" {
		return false
	}
	for i := 0; i < len(rustType); {
		if !isRustIdentStart(rustType[i]) {
			i++
			continue
		}
		start := i
		for i < len(rustType) && isRustIdentPart(rustType[i]) {
			i++
		}
		name := rustType[start:i]
		// A lifetime ('static) is not a type name.
		if start >= 1 && rustType[start-1] == '\'' {
			continue
		}
		// A path is standard if it starts in the standard library.
		if start >= 2 && rustType[start-2:start] == "::" {
			continue
		}
		if i+2 <= len(rustType) && rustType[i:i+2] == "::" {
			if name != "std" && name != "core" && name != "alloc" {
				return false
			}
			// Skip the rest of the path.
			for i < len(rustType) && (isRustIdentPart(rustType[i]) || rustType[i] == ':') {
				i++
			}
			continue
		}
		if !standardRustTypeNames[name] {
			return false
		}
	}
	return true
}

var standardRustTypeNames = map[string]bool{
	"i8": true, "i16": true, "i32": true, "i64": true, "isize": true,
	"u8": true, "u16": true, "u32": true, "u64": true, "usize": true,
	"f32": true, "f64": true, "bool": true, "char": true, "str": true, "String": true,
	"dyn": true, "Box": true, "Option": true, "Vec": true,
	"Rc": true, "RefCell": true, "Arc": true, "Mutex": true,
	"BTreeMap": true, "Any": true, "StdError": true, "Send": true, "Sync": true,
	"Fn": true, "FnMut": true, "FnOnce": true,
}
