package main

import (
	"go/types"
	"strings"
)

// anyFormatArmForType says how format_any prints a boxed value of type typ,
// or reports false to leave the type to format_any's fallback. It answers
// only for shapes the typed print path prints with one known helper, and only
// for element types that are certain to implement Display, because a case
// that does not compile breaks a program that otherwise builds.
func anyFormatArmForType(typ types.Type) (anyFormatArm, bool) {
	if typ == nil || typeMentionsTypeParam(typ) {
		return anyFormatArm{}, false
	}
	switch t := types.Unalias(typ).(type) {
	case *types.Named:
		if anyFormatTypeIsLocalStruct(t) {
			text := typeHasGoStringMethod(t) || isConcreteGoErrorValue(t)
			return anyFormatArm{expr: `format!("{}", v)`, text: text, fields: !text}, true
		}
		if anyFormatTypeIsLocalNamedBasic(t) {
			return anyFormatArm{expr: `format!("{}", v)`, text: typeHasGoStringMethod(t) || isConcreteGoErrorValue(t), namedBasic: true}, true
		}
		if anyFormatTypeIsLocalNamedCollection(t) && typeHasGoStringMethod(t) {
			// A named slice, array or map with a String method: its
			// Display calls String (DEV-234).
			return anyFormatArm{expr: `format!("{}", v)`, text: true}, true
		}
		if anyFormatTypeIsLocalNamedCollection(t) && !isConcreteGoErrorValue(t) && namedCollectionHasGeneratedDisplay(t) {
			// A named slice, array or map prints through the Display of
			// its newtype, as the typed print path does.
			return anyFormatArm{expr: `format!("{}", v)`}, true
		}
	case *types.Pointer:
		if anyFormatTypeIsLocalNamedCollection(t.Elem()) && typeHasGoStringMethod(t.Elem()) {
			// A pointer to a named slice, array or map with a
			// value-receiver String method prints through the method, as
			// the pointee does; fmt prints a nil one as <nil>, since the
			// method cannot be called on it.
			var expr strings.Builder
			expr.WriteString("match (*v")
			WriteBorrowMethod(&expr, false)
			expr.WriteString(`).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }`)
			return anyFormatArm{expr: expr.String(), text: true}, true
		}
		if anyFormatTypeIsLocalStruct(t.Elem()) {
			var expr strings.Builder
			if typeHasGoStringMethod(t) || isConcreteGoErrorValue(t) {
				// Go prints a pointer that has a String or Error method
				// through that method, wherever the pointer sits. The
				// pointee's Display calls the method.
				expr.WriteString("match (*v")
				WriteBorrowMethod(&expr, false)
				expr.WriteString(`).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }`)
				return anyFormatArm{expr: expr.String(), text: true}, true
			}
			// go_format_any_pointer expands the pointee only when the
			// pointer is the value being printed. Inside another value it
			// prints the address, as Go does, so a value that points back
			// to itself does not print forever. The nil check there must
			// not wait for the handle: the expansion that reached it may be
			// holding it.
			if NeedsConcurrentWrapper() {
				expr.WriteString("go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v")
			} else {
				expr.WriteString("go_format_any_pointer(nested, Rc::as_ptr(v) as *const (), || v.try_borrow().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v")
			}
			WriteBorrowMethod(&expr, false)
			expr.WriteString(`).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })`)
			return anyFormatArm{expr: expr.String(), needsFormatPointer: true}, true
		}
	case *types.Slice:
		if anyFormatElemImplementsDisplay(t.Elem()) {
			return anyFormatArm{expr: "format_slice(v)", needsFormatSlice: true}, true
		}
	case *types.Array:
		// An array of scalars is boxed as the Rust array itself.
		if anyArrayHasCloneableScalars(t) && anyFormatElemImplementsDisplay(t.Elem()) {
			_, flat := types.Unalias(t.Elem()).(*types.Basic)
			return anyFormatArm{expr: "format_slice_values(&v[..])", needsFormatSlice: true, elements: flat}, true
		}
	case *types.Struct:
		// An anonymous struct has a Display that prints as %v and %+v do.
		if _, ok := anonymousStructRustType(t); ok {
			return anyFormatArm{expr: `format!("{}", v)`, fields: true}, true
		}
	case *types.Map:
		if anyFormatKeyIsOrderedBasic(t.Key()) && anyFormatElemImplementsDisplay(t.Elem()) {
			return anyFormatArm{expr: "format_map(v)", needsFormatMap: true}, true
		}
	}
	return anyFormatArm{}, false
}

// anyFormatTypeIsLocalStruct reports whether typ is a struct type declared at
// package level in the package being transpiled. Those always get a Display
// impl that prints the way Go's %v does, and a helper can name them.
func anyFormatTypeIsLocalStruct(typ types.Type) bool {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	if args := named.TypeArgs(); args != nil && args.Len() > 0 {
		return false
	}
	if _, ok := named.Underlying().(*types.Struct); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || named.Obj().Pkg() != typeInfo.pkg {
		return false
	}
	return named.Obj().Parent() == named.Obj().Pkg().Scope()
}

// anyFormatTypeIsLocalNamedBasic reports whether typ is a type the package
// declares with a basic underlying type, "type Level int". Its newtype
// implements Display: through String when the type has one, otherwise as
// the value it holds, which is how Go prints both (DEV-47).
// anyFormatTypeIsLocalNamedCollection reports whether typ is a named slice,
// array or map type of the package being written.
func anyFormatTypeIsLocalNamedCollection(typ types.Type) bool {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	if args := named.TypeArgs(); args != nil && args.Len() > 0 {
		return false
	}
	switch named.Underlying().(type) {
	case *types.Slice, *types.Array, *types.Map:
	default:
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.pkg != nil && named.Obj().Pkg() == typeInfo.pkg
}

func anyFormatTypeIsLocalNamedBasic(typ types.Type) bool {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	if args := named.TypeArgs(); args != nil && args.Len() > 0 {
		return false
	}
	basic, ok := named.Underlying().(*types.Basic)
	if !ok || basic.Info()&(types.IsBoolean|types.IsInteger|types.IsFloat|types.IsString) == 0 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil {
		return false
	}
	// A package translated from source gives its exported types the same
	// Display, as time does time.Duration.
	if pkg := named.Obj().Pkg(); pkg != typeInfo.pkg && (!named.Obj().Exported() || isStubBackedStdlibPackagePath(pkg.Path())) {
		return false
	}
	return named.Obj().Parent() == named.Obj().Pkg().Scope()
}

func anyFormatElemImplementsDisplay(typ types.Type) bool {
	if basic, ok := types.Unalias(typ).(*types.Basic); ok {
		return basic.Info()&(types.IsBoolean|types.IsInteger|types.IsFloat|types.IsString) != 0 &&
			basic.Info()&types.IsUntyped == 0
	}
	return anyFormatTypeIsLocalStruct(typ)
}

// anyFormatKeyIsOrderedBasic reports whether typ is a key type format_map can
// print in Go's order. Go prints a map sorted by key: integers and strings in
// their own order, false before true. Rust orders all three the same way.
func anyFormatKeyIsOrderedBasic(typ types.Type) bool {
	basic, ok := types.Unalias(typ).(*types.Basic)
	return ok && basic.Info()&(types.IsBoolean|types.IsInteger|types.IsString) != 0 && basic.Info()&types.IsUntyped == 0
}
