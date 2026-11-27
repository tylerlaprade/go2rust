package main

import (
	"fmt"
	"go/ast"
	"go/types"
	"strings"
)

// fmt is bridged, so fmt.Stringer is the stub interface fmt_Stringer. A
// value the program converts to it carries its text with it: a closure
// over the value's Display impl, which calls its String method. String()
// and printing call that closure (DEV-152).

const fmtStringerStubName = "fmt_Stringer"

// Bridge debt fmt-stringer (docs/bridge_debt.md): fmt is bridged for its
// printer, which walks values with reflect, and its Stringer interface is
// the one name from it a program uses as a type.
func writeFmtStringerStub(out *strings.Builder, methods map[string]externalTypeStubMethod) {
	holder, fromBound, pointer, textFn := "Rc<dyn std::any::Any>", "T: std::fmt::Display + 'static", "Rc", "Rc<dyn Fn() -> String>"
	cell, read := "Rc<RefCell<Option<T>>>", ".borrow()"
	boxed, boxFn := "Box<dyn std::any::Any>", "Rc<dyn Fn() -> Box<dyn std::any::Any>>"
	if NeedsConcurrentWrapper() {
		holder, fromBound, pointer, textFn = "Arc<dyn std::any::Any + Send + Sync>", "T: std::fmt::Display + Send + Sync + 'static", "Arc", "Arc<dyn Fn() -> String + Send + Sync>"
		cell, read = "Arc<Mutex<Option<T>>>", ".lock().unwrap()"
		boxed, boxFn = "Box<dyn std::any::Any + Send + Sync>", "Arc<dyn Fn() -> Box<dyn std::any::Any + Send + Sync> + Send + Sync>"
	}
	stringType := wrappedExternalStubType("String")
	if method, ok := methods["string"]; ok && len(method.ReturnTypes) == 1 {
		stringType = method.ReturnTypes[0]
	}
	fmt.Fprintf(out, `#[derive(Clone)]
pub struct fmt_Stringer {
    pub __go_id: usize,
    pub __go_value: %[1]s,
    /// The text of the value's String method.
    pub __go_text: Option<%[4]s>,
    /// Boxes the value for an interface{}, as its own type.
    pub __go_box: Option<%[11]s>,
}

impl fmt_Stringer {
    pub fn __go_from<T: 'static%[7]s>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: %[3]s::new(value), __go_text: None, __go_box: None }
    }
    /// A value whose type has a String method, which its Display impl calls.
    pub fn __go_from_display<%[2]s + Clone>(value: T) -> Self {
        let shared = %[3]s::new(value);
        let text = shared.clone();
        let boxed = shared.clone();
        let __into_any: %[11]s = %[3]s::new(move || Box::new((*boxed).clone()) as %[10]s);
        Self { __go_id: __go_next_external_interface_id(), __go_value: shared, __go_text: Some(%[3]s::new(move || text.to_string())), __go_box: Some(__into_any) }
    }
    /// A pointer whose element type has a String method.
    pub fn __go_from_display_cell<%[2]s>(cell: %[5]s) -> Self {
        let text = cell.clone();
        let boxed = cell.clone();
        let __into_any: %[11]s = %[3]s::new(move || Box::new(boxed.clone()) as %[10]s);
        Self { __go_id: __go_next_external_interface_id(), __go_value: %[3]s::new(cell), __go_text: Some(%[3]s::new(move || text%[6]s.as_ref().map(|__v| __v.to_string()).unwrap_or_default())), __go_box: Some(__into_any) }
    }
    /// The value as an interface{} holds it: the value of its own type, so
    /// an assertion to that type finds it.
    pub fn __go_into_any(&self) -> %[10]s {
        match &self.__go_box {
            Some(boxed) => boxed(),
            None => Box::new(self.clone()) as %[10]s,
        }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
    fn __go_string(&self) -> String {
        match &self.__go_text {
            Some(text) => text(),
            None => panic!("fmt.Stringer value of a type with no String method"),
        }
    }
    pub fn string(&self) -> %[8]s {
        %[9]s
    }
}

impl Default for fmt_Stringer {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: %[3]s::new(()), __go_text: None, __go_box: None }
    }
}

impl std::fmt::Debug for fmt_Stringer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fmt_Stringer>")
    }
}

impl std::fmt::Display for fmt_Stringer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.__go_text {
            Some(text) => write!(f, "{}", text()),
            None => write!(f, "<fmt_Stringer>"),
        }
    }
}

impl PartialEq for fmt_Stringer {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for fmt_Stringer {}

impl PartialOrd for fmt_Stringer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Ord for fmt_Stringer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}
`, holder, fromBound, pointer, textFn, cell, read, concurrentAnyBound(), stringType, wrappedExternalStubExpr("String", "self.__go_string()"), boxed, boxFn)
}

func concurrentAnyBound() string {
	if NeedsConcurrentWrapper() {
		return " + Send + Sync"
	}
	return ""
}

// writeFmtStringerConversion writes a program value converted to
// fmt.Stringer, with its text when its type has a String method.
func writeFmtStringerConversion(out *strings.Builder, arg ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	sourceType := typeInfo.GetType(arg)
	if sourceType == nil {
		return false
	}
	constructor := "__go_from_display("
	if _, isPointer := types.Unalias(sourceType).(*types.Pointer); isPointer {
		constructor = "__go_from_display_cell("
	}
	out.WriteString(fmtStringerStubName)
	out.WriteString("::")
	out.WriteString(constructor)
	if call, isCall := unwrapParens(arg).(*ast.CallExpr); isCall && constructor == "__go_from_display(" && typeInfo.ReturnsWrappedValue(call) && !typeInfo.IsTypeConversion(call) {
		// A call returns its value in a cell; the Stringer holds the value.
		out.WriteString("(*")
		TranspileExpression(out, call)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
	} else {
		writeLocalConcreteStdlibInterfaceSource(out, arg)
	}
	out.WriteString(")")
	return true
}

// fmtStringerCaseInterface returns fmt.Stringer's method set when a type
// switch case or an assertion names fmt.Stringer.
func fmtStringerCaseInterface(typeInfo *TypeInfo, typeExpr ast.Expr) (*types.Interface, bool) {
	if typeInfo == nil || typeExpr == nil {
		return nil, false
	}
	typ := typeInfo.GetType(typeExpr)
	if !isFmtStringerType(typ) {
		return nil, false
	}
	iface, ok := types.Unalias(typ).Underlying().(*types.Interface)
	return iface, ok
}

func isFmtStringerType(typ types.Type) bool {
	named, ok := types.Unalias(typ).(*types.Named)
	return ok && named.Obj() != nil && named.Obj().Pkg() != nil && named.Obj().Pkg().Path() == "fmt" && named.Obj().Name() == "Stringer"
}

// fmtStringerCandidates are the program's types an interface value can hold
// that have a String method: a value of one matches case fmt.Stringer.
func fmtStringerCandidates(iface *types.Interface, subjectType types.Type) []localInterfaceAssertionCandidate {
	var out []localInterfaceAssertionCandidate
	for _, candidate := range localInterfaceAssertionCandidates(iface, subjectType) {
		// A type of a package translated from source has its own Rust
		// type and Display impl, like the program's; a stub's type may not.
		if candidate.external && !fmtStringerCandidateIsTranslated(candidate.typ) {
			continue
		}
		// An interface value holds a *T as T's cell, not as the pointer
		// newtype a source-translated interface's impl uses.
		if ptr, isPointer := types.Unalias(candidate.typ).(*types.Pointer); isPointer {
			if named, ok := types.Unalias(ptr.Elem()).(*types.Named); ok {
				candidate.rustType = goTypesWrappedRustType(goTypesNamedTypeToRust(named))
			}
			out = append(out, candidate)
			continue
		}
		out = append(out, candidate)
		// A value receiver's String is in *T's method set too, so a *T
		// held in an interface is a Stringer as well.
		if named, ok := types.Unalias(candidate.typ).(*types.Named); ok {
			out = append(out, localInterfaceAssertionCandidate{
				rustType: goTypesWrappedRustType(goTypesNamedTypeToRust(named)),
				typ:      types.NewPointer(named),
			})
		}
	}
	return out
}

func writeTypeSwitchFmtStringerCaseCondition(out *strings.Builder, typeInfo *TypeInfo, typeExpr ast.Expr, subjectType types.Type) bool {
	iface, ok := fmtStringerCaseInterface(typeInfo, typeExpr)
	if !ok {
		return false
	}
	out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<" + fmtStringerStubName + ">()).is_some()")
	for _, candidate := range fmtStringerCandidates(iface, subjectType) {
		out.WriteString(" || _ts_val.and_then(|__v| __v.downcast_ref::<")
		out.WriteString(candidate.rustType)
		out.WriteString(">()).is_some()")
	}
	return true
}

func writeTypeSwitchFmtStringerCaseBinding(out *strings.Builder, typeInfo *TypeInfo, varName string, typeExpr ast.Expr, subjectType types.Type, mutable bool) bool {
	iface, ok := fmtStringerCaseInterface(typeInfo, typeExpr)
	if !ok {
		return false
	}
	out.WriteString("        let ")
	if mutable {
		out.WriteString("mut ")
	}
	out.WriteString(varName)
	out.WriteString(": ")
	out.WriteString(goTypesWrappedRustType(fmtStringerStubName))
	out.WriteString(" = ")
	writeFmtStringerFromAny(out, "_ts_val", iface, subjectType)
	out.WriteString(";\n")
	return true
}

// writeFmtStringerFromAny writes the fmt.Stringer cell for anyVal, an
// Option<&dyn Any> known to hold a fmt.Stringer or one of the candidates.
func writeFmtStringerFromAny(out *strings.Builder, anyVal string, iface *types.Interface, subjectType types.Type) {
	out.WriteString("if let Some(typed_val) = " + anyVal + ".and_then(|__v| __v.downcast_ref::<" + fmtStringerStubName + ">()) { ")
	WriteWrapperPrefix(out)
	out.WriteString("typed_val.clone()")
	WriteWrapperSuffix(out)
	out.WriteString(" }")
	for _, candidate := range fmtStringerCandidates(iface, subjectType) {
		out.WriteString(" else if let Some(typed_val) = " + anyVal + ".and_then(|__v| __v.downcast_ref::<")
		out.WriteString(candidate.rustType)
		out.WriteString(">()) { ")
		WriteWrapperPrefix(out)
		out.WriteString(fmtStringerStubName)
		if _, isPointer := types.Unalias(candidate.typ).(*types.Pointer); isPointer {
			out.WriteString("::__go_from_display_cell(typed_val.clone())")
		} else {
			out.WriteString("::__go_from_display(typed_val.clone())")
		}
		WriteWrapperSuffix(out)
		out.WriteString(" }")
	}
	out.WriteString(" else { ")
	WriteWrappedNone(out)
	out.WriteString(" }")
}

// fmtStringerAssertionTarget reports whether e is x.(fmt.Stringer) on an
// empty interface x, with fmt.Stringer's method set and x's type.
func fmtStringerAssertionTarget(e *ast.TypeAssertExpr) (*types.Interface, types.Type, bool) {
	typeInfo := GetTypeInfo()
	if e == nil || e.Type == nil || typeInfo == nil {
		return nil, nil, false
	}
	iface, ok := fmtStringerCaseInterface(typeInfo, e.Type)
	if !ok {
		return nil, nil, false
	}
	sourceType := typeInfo.GetType(e.X)
	if sourceType == nil || !isEmptyInterfaceType(sourceType) {
		return nil, nil, false
	}
	return iface, sourceType, true
}

// writeFmtStringerDowncastChain writes an if/else chain over any_val, the
// &Box<dyn Any> an interface value holds: a fmt_Stringer it already holds,
// or one made from a candidate value with a String method. hit writes the
// arm's result from the fmt_Stringer expression it is given; miss writes
// the last arm.
func writeFmtStringerDowncastChain(out *strings.Builder, iface *types.Interface, sourceType types.Type, hit func(stringer string), miss func()) {
	out.WriteString("if let Some(typed_val) = any_val.downcast_ref::<" + fmtStringerStubName + ">() { ")
	hit("typed_val.clone()")
	out.WriteString(" }")
	for _, candidate := range fmtStringerCandidates(iface, sourceType) {
		out.WriteString(" else if let Some(typed_val) = any_val.downcast_ref::<")
		out.WriteString(candidate.rustType)
		out.WriteString(">() { ")
		constructor := "::__go_from_display(typed_val.clone())"
		if _, isPointer := types.Unalias(candidate.typ).(*types.Pointer); isPointer {
			constructor = "::__go_from_display_cell(typed_val.clone())"
		}
		hit(fmtStringerStubName + constructor)
		out.WriteString(" }")
	}
	out.WriteString(" else { ")
	miss()
	out.WriteString(" }")
}

func writeFmtStringerAssertionOpen(out *strings.Builder, e *ast.TypeAssertExpr) {
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	writeTypeAssertionInputClone(out, e.X)
	out.WriteString(";\n")
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n            ")
}

// writeFmtStringerAssertionCommaOk writes "s, ok := x.(fmt.Stringer)" for
// an empty interface x.
func writeFmtStringerAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr, iface *types.Interface, sourceType types.Type) {
	writeFmtStringerAssertionOpen(out, e)
	writeFmtStringerDowncastChain(out, iface, sourceType, func(stringer string) {
		out.WriteString("(")
		WriteWrapperPrefix(out)
		out.WriteString(stringer)
		WriteWrapperSuffix(out)
		out.WriteString(", true)")
	}, func() {
		out.WriteString("(")
		writeTypedWrappedNone(out, fmtStringerStubName)
		out.WriteString(", false)")
	})
	out.WriteString("\n        } else {\n            (")
	writeTypedWrappedNone(out, fmtStringerStubName)
	out.WriteString(", false)\n        }\n    })")
}

// writeFmtStringerAssertionValue writes "x.(fmt.Stringer)" for an empty
// interface x: the fmt_Stringer, or Go's panic when x is nil or its value
// has no String method.
//
// The interface's lock is let go before the panic, so a program that
// recovers can still use the interface value.
func writeFmtStringerAssertionValue(out *strings.Builder, e *ast.TypeAssertExpr, iface *types.Interface, sourceType types.Type) {
	NeedGoTypeName()
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	writeTypeAssertionInputClone(out, e.X)
	out.WriteString(";\n")
	out.WriteString("        let __stringer: Result<" + fmtStringerStubName + ", String> = {\n")
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n            ")
	writeFmtStringerDowncastChain(out, iface, sourceType, func(stringer string) {
		out.WriteString("Ok(" + stringer + ")")
	}, func() {
		out.WriteString("Err(format!(\"interface conversion: {} is not fmt.Stringer: missing method String\", ")
		out.WriteString(goTypeNameHelperRustName)
		out.WriteString("(&**any_val)))")
	})
	out.WriteString("\n        } else {\n            Err(\"interface conversion: interface is nil, not fmt.Stringer\".to_string())\n        }\n        };\n")
	out.WriteString("        match __stringer { Ok(__s) => __s, Err(__message) => panic!(\"{}\", __message) }\n    })")
}

// writeFmtStringerTypeName writes %T of a fmt.Stringer variable: the type of
// the value it holds, which fmt_Stringer keeps in __go_value, or <nil>.
func writeFmtStringerTypeName(out *strings.Builder, arg ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(arg)
	if !isFmtStringerType(typ) {
		return false
	}
	iface, ok := typ.Underlying().(*types.Interface)
	if !ok {
		return false
	}
	var cell strings.Builder
	switch e := arg.(type) {
	case *ast.Ident:
		if isVarBare(e.Name) {
			return false
		}
		cell.WriteString(RustIdentForUse(e))
	case *ast.SelectorExpr:
		TranspileExpressionContext(&cell, e, LValue)
	case *ast.CallExpr:
		if !typeInfo.ReturnsWrappedValue(e) {
			return false
		}
		TranspileExpression(&cell, e)
	case *ast.TypeAssertExpr:
		if !isFmtStringerAssertion(e) {
			return false
		}
		// x.(fmt.Stringer) yields the bare fmt_Stringer, or panics.
		for _, candidate := range fmtStringerCandidates(iface, typ) {
			RegisterAnyCloneType(candidate.typ)
		}
		NeedGoTypeName()
		out.WriteString(goTypeNameHelperRustName)
		out.WriteString("(&*(")
		TranspileExpression(out, e)
		out.WriteString(").__go_value)")
		return true
	default:
		return false
	}
	for _, candidate := range fmtStringerCandidates(iface, typ) {
		RegisterAnyCloneType(candidate.typ)
	}
	NeedGoTypeName()
	out.WriteString("match (")
	out.WriteString(cell.String())
	out.WriteString(")")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref() { Some(__stringer) => ")
	out.WriteString(goTypeNameHelperRustName)
	out.WriteString("(&*__stringer.__go_value), None => \"<nil>\" }")
	return true
}

// isFmtStringerAssertion reports whether e is x.(fmt.Stringer) on an empty
// interface x, written by writeFmtStringerAssertionValue as a bare
// fmt_Stringer.
func isFmtStringerAssertion(e *ast.TypeAssertExpr) bool {
	_, _, ok := fmtStringerAssertionTarget(e)
	return ok
}

// fmtStringerCandidateIsTranslated reports whether a candidate type outside
// the current package is translated from Go source rather than a stub.
func fmtStringerCandidateIsTranslated(typ types.Type) bool {
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		typ = ptr.Elem()
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	return !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path())
}

// writeFmtStringerAnyBox boxes a fmt.Stringer value for an interface{} as
// the value it holds, so an assertion to the value's own type finds it; a
// box of the fmt_Stringer itself matched only fmt.Stringer (DEV-155).
func writeFmtStringerAnyBox(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil || !isFmtStringerType(typeInfo.GetType(expr)) {
		return false
	}
	if ident, ok := unwrapParens(expr).(*ast.Ident); ok && ident.Name == "nil" {
		return false
	}
	// The box holds one of the types with a String method, which
	// go_any_clone must be able to copy.
	typ := typeInfo.GetType(expr)
	if iface, ok := types.Unalias(typ).Underlying().(*types.Interface); ok {
		for _, candidate := range fmtStringerCandidates(iface, typ) {
			RegisterAnyCloneType(candidate.typ)
		}
	}
	// A nil fmt.Stringer variable is a nil interface{}.
	if ident, ok := unwrapParens(expr).(*ast.Ident); ok && !isVarBare(ident.Name) {
		if _, isRange := rangeLoopVars[ident.Name]; !isRange {
			out.WriteString("match (*")
			out.WriteString(rustIdentForUseWithCapture(ident))
			WriteBorrowMethod(out, false)
			out.WriteString(").as_ref() { Some(__stringer) => __stringer.__go_into_any(), None => ")
			writeNilAnyBox(out)
			out.WriteString(" }")
			return true
		}
	}
	out.WriteString("(")
	writeAnyBoxInnerValue(out, expr)
	out.WriteString(").__go_into_any()")
	return true
}
