package main

import (
	"go/types"
	"strings"
)

// Printing a value held in one of the program's interfaces.
//
// Each trait that declares `__go_as_any` also declares `__go_print`, which
// returns the text fmt prints for the value at the top level of a print: %v
// when plus is false, %+v when it is true. Its default is the value's Display.
// The pointer wrapper (`TPtr`) that boxes a *T overrides it, because Go prints
// a *T held in an interface as T's String() or Error() when it has one, and a
// pointer to a struct as &{...}.
//
// The wrapper's Display stays the pointer's address. That is what Go prints
// for a pointer nested inside another value, and interface-keyed maps order
// their keys by Display, so it must not call String() on the pointee.

// writeGoPrintTraitMethod writes the default `__go_print` of a trait that
// declares `__go_as_any`.
func writeGoPrintTraitMethod(out *strings.Builder) {
	out.WriteString("    fn __go_print(&self, plus: bool) -> String { if plus { format!(\"{:#}\", self) } else { format!(\"{}\", self) } }\n")
}

// writePointerWrapperGoPrintMethod writes the `__go_print` override of the
// pointer wrapper for the local type typeName.
func writePointerWrapperGoPrintMethod(out *strings.Builder, typeName string) {
	pointee, ok := pointerWrapperPointeePrint(typeName)
	if !ok {
		return
	}
	out.WriteString("    fn __go_print(&self, plus: bool) -> String {\n")
	out.WriteString("        let __guard = self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        match __guard.as_ref() { Some(__v) => ")
	out.WriteString(pointee)
	out.WriteString(", None => \"<nil>\".to_string() }\n")
	out.WriteString("    }\n")
}

// pointerWrapperPointeePrint returns the Rust expression, in terms of the
// pointee __v and the flag plus, that prints a non-nil *T held in an
// interface. It reports false when Go prints the pointer's address, which the
// default `__go_print` already does.
func pointerWrapperPointeePrint(typeName string) (string, bool) {
	// T's Display calls Error() or String() when T or *T has one; Go calls
	// the same method through the pointer.
	if IsErrorImplType(typeName) || namedTypeHasGoStringMethod(typeName) {
		return "format!(\"{}\", __v)", true
	}
	if !localTypeIsStruct(typeName) {
		return "", false
	}
	return "if plus { format!(\"&{:#}\", __v) } else { format!(\"&{}\", __v) }", true
}

func localTypeIsStruct(typeName string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return false
	}
	obj, ok := typeInfo.pkg.Scope().Lookup(typeName).(*types.TypeName)
	if !ok || obj.Type() == nil {
		return false
	}
	named, ok := obj.Type().(*types.Named)
	if !ok || named.TypeParams().Len() != 0 {
		return false
	}
	_, ok = named.Underlying().(*types.Struct)
	return ok
}

// goPrintCall returns the Rust String expression that prints recv, a trait
// object of the interface type typ, at the top level of a print. methodRecv
// is written before `.__go_print(..)` when method syntax is unambiguous;
// refRecv, an expression of type &dyn Trait, is passed to the qualified call
// otherwise. It reports false when typ's trait has no `__go_print`.
func goPrintCall(typ types.Type, methodRecv, refRecv string, plus bool) (string, bool) {
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(typ); !ok {
		return "", false
	}
	flag := "false"
	if plus {
		flag = "true"
	}
	if qualifyingTrait := goAsAnyQualifyingTrait(typ); qualifyingTrait != "" {
		return qualifyingTrait + "::__go_print(" + refRecv + ", " + flag + ")", true
	}
	return methodRecv + ".__go_print(" + flag + ")", true
}
