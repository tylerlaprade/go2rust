package main

import (
	"go/ast"
	"go/types"
	"strings"
)

// A value of one of the package's interfaces is a cell of a Box<dyn Trait>.
// Boxing that cell for go_sprintf gave format_any a type it has no case for,
// so fmt.Printf("%v %T", shape, shape) panicked. fmt sees the interface's
// dynamic value: the trait object is cloned and upcast to Any, which holds
// the Rect or the Circle itself, and format_any gets a case for each type
// of the package that implements the interface.

// writeLocalInterfaceDynamicValueBox writes a Box<dyn Any> of the dynamic
// value of arg, a non-nil value of one of the package's interfaces, and
// reports whether arg is one.
func writeLocalInterfaceDynamicValueBox(out *strings.Builder, arg ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil {
		return false
	}
	argType := typeInfo.GetType(arg)
	named, ok := types.Unalias(argType).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() != typeInfo.pkg {
		return false
	}
	iface, ok := named.Underlying().(*types.Interface)
	if !ok || iface.NumMethods() == 0 {
		return false
	}
	ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(argType)
	if !ok {
		return false
	}
	switch unwrapParens(arg).(type) {
	case *ast.Ident, *ast.SelectorExpr:
	default:
		return false
	}
	if ident, ok := unwrapParens(arg).(*ast.Ident); ok {
		// The variable must be the interface's cell: a range variable over
		// a slice of interface values is a reference to one.
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			if !isWrappedRangeVarType(varType) {
				return false
			}
		} else if isVarBare(ident.Name) {
			return false
		}
	}
	registerLocalInterfaceImplementerFormatCases(iface)
	anyBox := rustAnyTraitObject()
	out.WriteString("{ let __iface_holder = ")
	TranspileExpressionContext(out, arg, LValue)
	out.WriteString(".clone(); let __iface_guard = __iface_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __iface: ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString(" = __iface_guard.as_ref().unwrap().__go_clone_box_")
	out.WriteString(traitMethodSuffix(ifaceName))
	out.WriteString("(); __iface as ")
	out.WriteString(anyBox)
	out.WriteString(" }")
	return true
}

// registerLocalInterfaceImplementerFormatCases gives format_any and %T a
// case for each type of the package that implements iface. A type whose
// methods have pointer receivers is held in the interface as its Ptr
// wrapper, which prints through its Display as the pointer does.
func registerLocalInterfaceImplementerFormatCases(iface *types.Interface) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return
	}
	scope := typeInfo.pkg.Scope()
	for _, name := range scope.Names() {
		obj, ok := scope.Lookup(name).(*types.TypeName)
		if !ok || obj.IsAlias() {
			continue
		}
		named, ok := obj.Type().(*types.Named)
		if !ok || named.TypeParams().Len() > 0 {
			continue
		}
		if _, isInterface := named.Underlying().(*types.Interface); isInterface {
			continue
		}
		if types.Implements(named, iface) {
			RegisterAnyCloneType(named)
			continue
		}
		pointer := types.NewPointer(named)
		if !types.Implements(pointer, iface) {
			continue
		}
		goName, ok := goDynamicTypeName(pointer)
		if !ok {
			continue
		}
		wrapper := pointerLocalInterfaceWrapperNameForUse(name, "")
		registerAnyFormatArm(wrapper, anyFormatArm{expr: `format!("{}", v)`, text: typeHasGoStringMethod(pointer) || isConcreteGoErrorValue(pointer)})
		registerGoTypeNameForRustType(wrapper, goName)
	}
}

// registerAnyFormatArm adds a format_any case for a Rust type.
func registerAnyFormatArm(rustType string, arm anyFormatArm) {
	NeedFormatAny()
	if helpers := activeHelperTracker(); helpers != nil {
		if helpers.anyFormatArms == nil {
			helpers.anyFormatArms = make(map[string]anyFormatArm)
		}
		helpers.anyFormatArms[rustType] = arm
	}
}
