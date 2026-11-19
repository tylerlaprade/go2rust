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
			return anyFormatArm{expr: `format!("{}", v)`}, true
		}
	case *types.Pointer:
		if anyFormatTypeIsLocalStruct(t.Elem()) {
			// go_format_any_pointer expands the pointee only at the top
			// level. A pointer reached while another is being expanded is
			// printed as its address, as Go does, so a value that points
			// back to itself does not print forever. The nil check below
			// that level must not wait for the handle: the expansion that
			// is in progress may be holding it.
			var expr strings.Builder
			if NeedsConcurrentWrapper() {
				expr.WriteString("go_format_any_pointer(Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v")
			} else {
				expr.WriteString("go_format_any_pointer(Rc::as_ptr(v) as *const (), || v.try_borrow().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v")
			}
			WriteBorrowMethod(&expr, false)
			expr.WriteString(`).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })`)
			return anyFormatArm{expr: expr.String(), needsFormatPointer: true}, true
		}
	case *types.Slice:
		if anyFormatElemImplementsDisplay(t.Elem()) {
			return anyFormatArm{expr: "format_slice(v)", needsFormatSlice: true}, true
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

func anyFormatElemImplementsDisplay(typ types.Type) bool {
	if basic, ok := types.Unalias(typ).(*types.Basic); ok {
		return basic.Info()&(types.IsBoolean|types.IsInteger|types.IsFloat|types.IsString) != 0 &&
			basic.Info()&types.IsUntyped == 0
	}
	return anyFormatTypeIsLocalStruct(typ)
}

func anyFormatKeyIsOrderedBasic(typ types.Type) bool {
	basic, ok := types.Unalias(typ).(*types.Basic)
	return ok && basic.Info()&(types.IsInteger|types.IsString) != 0 && basic.Info()&types.IsUntyped == 0
}
