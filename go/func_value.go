package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"sort"
	"strings"
)

// A Go func value is immutable and is copied freely: `saved := act` copies
// the function, so a later `act = other` leaves `saved` calling the first
// function. Its Rust form is therefore a shared closure that can be cloned:
// Rc<dyn Fn> in a program without goroutines and Arc<dyn Fn + Send + Sync>
// in one with them. A func-typed variable is a cell holding one of these, as
// for every other type: copying the variable makes a new cell around a clone
// of the closure, and assigning the variable writes into its cell, so a
// closure that captured the variable sees the new function.

// funcValueRustType spells the Rust type of a Go func value with the given
// parameter list and result type.
func funcValueRustType(params, result string) string {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		return "Arc<dyn Fn(" + params + ") -> " + result + " + Send + Sync>"
	}
	TrackImport("Rc")
	return "Rc<dyn Fn(" + params + ") -> " + result + ">"
}

// funcValueRustTypeQualified is funcValueRustType spelled with full paths,
// for runtime code written where Rc or Arc may not be imported.
func funcValueRustTypeQualified(params, result string) string {
	if NeedsConcurrentWrapper() {
		return "std::sync::Arc<dyn Fn(" + params + ") -> " + result + " + Send + Sync>"
	}
	return "std::rc::Rc<dyn Fn(" + params + ") -> " + result + ">"
}

// funcValueConstructor opens the expression that makes a func value from a
// closure; the caller writes the closure and then ")".
func funcValueConstructor() string {
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		return "Arc::new("
	}
	TrackImport("Rc")
	return "Rc::new("
}

// funcValueTypePrefixes are the starts of every Rust type funcValueRustType
// writes.
var funcValueTypePrefixes = []string{"Rc<dyn Fn(", "Arc<dyn Fn(", "std::rc::Rc<dyn Fn(", "std::sync::Arc<dyn Fn("}

// isFuncValueRustType reports whether rustType is the Rust form of a Go func
// value.
func isFuncValueRustType(rustType string) bool {
	for _, prefix := range funcValueTypePrefixes {
		if strings.HasPrefix(rustType, prefix) {
			return true
		}
	}
	return false
}

// funcValueRustTypeIndex returns where the first func value type starts in
// rustType, or -1.
func funcValueRustTypeIndex(rustType string) int {
	best := -1
	for _, prefix := range funcValueTypePrefixes {
		if i := strings.Index(rustType, prefix); i >= 0 && (best < 0 || i < best) {
			// "std::rc::Rc<dyn Fn(" also contains "Rc<dyn Fn("; prefer the
			// earlier, longer match.
			best = i
		}
	}
	return best
}

// wrappedFuncValueRustType reports whether rustType is a cell holding a func
// value, such as Rc<RefCell<Option<Rc<dyn Fn(..) -> ..>>>>.
func wrappedFuncValueRustType(rustType string) bool {
	for _, wrapper := range []string{"Rc<RefCell<Option<", "Arc<Mutex<Option<"} {
		if strings.HasPrefix(rustType, wrapper) && isFuncValueRustType(strings.TrimPrefix(rustType, wrapper)) {
			return true
		}
	}
	return false
}

// generateGoFuncValue writes GoFuncValue, which reads the func value out of
// whatever a func-typed expression lowers to: a variable's cell, an Option
// of a func value, or a func value itself.
func generateGoFuncValue(out *strings.Builder) {
	visibility := ""
	if generatingPublicHelpers {
		visibility = "pub "
	}
	pointer, cell, read, bound := "std::rc::Rc", "std::rc::Rc<std::cell::RefCell<Option<", ".borrow().clone()", ""
	if NeedsConcurrentWrapper() {
		pointer, cell, read, bound = "std::sync::Arc", "std::sync::Arc<std::sync::Mutex<Option<", ".lock().unwrap().clone()", " + Send + Sync"
	}
	code := `
/// The Go func value a func-typed expression holds, or None for a nil func.
/// A copy of a func value is a clone of the same closure.
VIS trait GoFuncValue {
    type Func: ?Sized;
    fn go_func_value(&self) -> Option<PTR<Self::Func>>;
}

macro_rules! impl_go_func_value {
    ($(($($arg:ident),*)),* $(,)?) => { $(
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for PTR<dyn Fn($($arg),*) -> R BOUND> {
            type Func = dyn Fn($($arg),*) -> R BOUND;
            fn go_func_value(&self) -> Option<PTR<Self::Func>> { Some(self.clone()) }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for Option<PTR<dyn Fn($($arg),*) -> R BOUND>> {
            type Func = dyn Fn($($arg),*) -> R BOUND;
            fn go_func_value(&self) -> Option<PTR<Self::Func>> { self.clone() }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for CELLPTR<dyn Fn($($arg),*) -> R BOUND>>>> {
            type Func = dyn Fn($($arg),*) -> R BOUND;
            fn go_func_value(&self) -> Option<PTR<Self::Func>> { selfREAD }
        }
    )* };
}
impl_go_func_value!((), (A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7), (A0, A1, A2, A3, A4, A5, A6, A7, A8), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11));
`
	code = strings.NewReplacer("VIS ", visibility, "CELLPTR", cell+pointer, "PTR", pointer, " BOUND", bound, "READ", read).Replace(code)
	out.WriteString(code)
}

// writeFuncValueOption writes the Option of the func value that the
// func-typed expr holds: a copy of it, which later assignments to the
// variable or field expr reads do not change.
func writeFuncValueOption(out *strings.Builder, expr ast.Expr) {
	NeedGoFuncValue()
	expr = unwrapParens(expr)
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("None")
		return
	}
	if funcValueExprIsStorage(expr) {
		// A variable's or field's cell holds the Option itself, read
		// here without GoFuncValue: its impls cannot name a func type
		// whose parameter is a reference, as func(context.Context) is.
		out.WriteString("(*")
		TranspileExpressionContext(out, expr, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(").clone()")
		return
	}
	out.WriteString("GoFuncValue::go_func_value(&(")
	switch e := expr.(type) {
	case *ast.Ident, *ast.SelectorExpr:
		if ident, ok := e.(*ast.Ident); ok && isPackageGlobalIdent(ident) {
			// The global's Option, read without unwrapping.
			out.WriteString("*")
			TranspileExpressionContext(out, expr, LValue)
			WriteBorrowMethod(out, false)
			break
		}
		if funcValueExprIsStorage(expr) {
			// The variable's or field's cell, so that a nil func is read
			// as nil rather than unwrapped.
			TranspileExpressionContext(out, expr, LValue)
			break
		}
		TranspileExpression(out, expr)
	case *ast.TypeAssertExpr:
		// x.(func()) is the cell the interface value holds.
		if !writeAnyHandleAssertionHandle(out, e) {
			TranspileExpression(out, expr)
		}
	default:
		TranspileExpression(out, expr)
	}
	out.WriteString("))")
}

// writeFuncValueFreshCell writes a new cell holding a copy of the func value
// expr holds.
func writeFuncValueFreshCell(out *strings.Builder, expr ast.Expr) {
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::new(")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("::new(")
	writeFuncValueOption(out, expr)
	out.WriteString("))")
}

// funcValueExprIsStorage reports whether expr names a variable or a struct
// field of func type, whose LValue lowering is its cell.
func funcValueExprIsStorage(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	switch e := unwrapParens(expr).(type) {
	case *ast.Ident:
		obj, ok := typeInfo.info.Uses[e].(*types.Var)
		if !ok || !isFunctionSignatureType(obj.Type()) {
			return false
		}
		if _, isRangeVar := rangeLoopVars[e.Name]; isRangeVar {
			return false
		}
		if isVarBare(e.Name) || isPackageGlobalIdent(e) {
			return false
		}
		return true
	case *ast.SelectorExpr:
		selection, ok := typeInfo.info.Selections[e]
		return ok && selection.Kind() == types.FieldVal && isFunctionSignatureType(selection.Type())
	}
	return false
}

// isFuncValueExpr reports whether expr has a func type. A defining
// identifier has the type of the variable it defines.
func isFuncValueExpr(expr ast.Expr) bool {
	return isFunctionSignatureType(funcValueExprType(expr))
}

func funcValueExprType(expr ast.Expr) types.Type {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	if ident, ok := unwrapParens(expr).(*ast.Ident); ok {
		if obj := typeInfo.info.Defs[ident]; obj != nil {
			return obj.Type()
		}
	}
	return typeInfo.GetType(expr)
}

// funcValueAssignmentKeepsLowering reports whether the existing lowering of
// `f = rhs` already stores a func value of its own: a function literal, a
// named function, or a method value or expression, each of which builds a
// new closure.
func funcValueAssignmentKeepsLowering(rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return true
	}
	switch e := unwrapParens(rhs).(type) {
	case *ast.FuncLit:
		return true
	case *ast.StarExpr:
		// *(*func())(p) reads through a pointer, which has its own
		// lowering (and fails loudly for an unsafe.Pointer).
		return true
	case *ast.Ident:
		_, isFunc := typeInfo.info.Uses[e].(*types.Func)
		return isFunc
	case *ast.SelectorExpr:
		if selection, ok := typeInfo.info.Selections[e]; ok {
			return selection.Kind() != types.FieldVal
		}
		// A package-qualified function, or a package variable of another
		// package.
		_, isFunc := typeInfo.info.Uses[e.Sel].(*types.Func)
		return isFunc
	case *ast.IndexExpr, *ast.IndexListExpr:
		// An instantiated generic function.
		if ident, ok := unwrapParens(indexExprBase(e)).(*ast.Ident); ok {
			_, isFunc := typeInfo.info.Uses[ident].(*types.Func)
			return isFunc
		}
	}
	return false
}

func indexExprBase(expr ast.Expr) ast.Expr {
	switch e := expr.(type) {
	case *ast.IndexExpr:
		return e.X
	case *ast.IndexListExpr:
		return e.X
	}
	return nil
}

// writeFuncValueAssignment writes `f = v` for a func-typed variable or field
// f and reports whether it did. A variable keeps its cell, which closures
// that captured f hold, and takes a copy of v's func value; a field takes a
// new cell of its own, since copies of the struct share the old one.
func writeFuncValueAssignment(out *strings.Builder, s *ast.AssignStmt) bool {
	if s.Tok != token.ASSIGN || len(s.Lhs) != 1 || len(s.Rhs) != 1 {
		return false
	}
	lhs, rhs := unwrapParens(s.Lhs[0]), s.Rhs[0]
	if !isFuncValueExpr(lhs) {
		return false
	}
	buildsClosure := funcValueAssignmentKeepsLowering(rhs)
	switch target := lhs.(type) {
	case *ast.Ident:
		if target.Name == "_" || buildsClosure {
			return false
		}
		if !funcValueExprIsStorage(target) && !isPackageGlobalIdent(target) {
			return false
		}
		out.WriteString("{ let __func_value = ")
		writeFuncValueOption(out, rhs)
		out.WriteString("; *")
		TranspileExpressionContext(out, target, LValue)
		WriteBorrowMethod(out, true)
		out.WriteString(" = __func_value; }")
		return true
	case *ast.SelectorExpr:
		if !funcValueExprIsStorage(target) {
			return false
		}
		if _, isFuncLit := unwrapParens(rhs).(*ast.FuncLit); isFuncLit {
			// The literal's lowering writes the new closure into the
			// field's cell, which no copy of the struct shares, through
			// a handle it takes before the closure moves its captures.
			return false
		}
		out.WriteString("{ let new_val = ")
		if !buildsClosure || !writeFunctionValueHandleForExpected(out, rhs, GetTypeInfo().GetType(target)) {
			writeFuncValueFreshCell(out, rhs)
		}
		out.WriteString("; ")
		writePointerHandleAssignmentTargetFromValueName(out, target, "new_val")
		out.WriteString(" }")
		return true
	}
	return false
}

// writeFuncValueDefineShadows follows a definition of func-typed variables,
// `f := v` or `var f = v`, with a new cell for each one whose value may be a
// cell that belongs to something else: another variable, a field, or the
// result of a call. Assignments to f write into its cell, so the cell must
// be f's own.
func writeFuncValueDefineShadows(out *strings.Builder, lhs []ast.Expr, rhs []ast.Expr, indent string) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || len(rhs) == 0 {
		return
	}
	if len(lhs) == len(rhs) && len(lhs) == 1 && funcValueAssignmentKeepsLowering(rhs[0]) {
		return
	}
	if len(rhs) == 1 && len(lhs) == 1 {
		if ident, ok := unwrapParens(rhs[0]).(*ast.Ident); ok && ident.Name == "nil" {
			return
		}
	}
	for i, expr := range lhs {
		ident, ok := expr.(*ast.Ident)
		if !ok || ident.Name == "_" {
			continue
		}
		obj, ok := typeInfo.info.Defs[ident].(*types.Var)
		if !ok || !isFunctionSignatureType(obj.Type()) {
			continue
		}
		if len(lhs) == len(rhs) && (len(lhs) > 1 || funcValueAssignmentKeepsLowering(rhs[i])) {
			// Parallel definitions copy each func value as they make the
			// variable's cell.
			continue
		}
		if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			continue
		}
		if info := lookupVarInfo(ident.Name); info == nil || info.WrapLevel != WrapFull || info.PointerKind != PointerDefault || info.IsRef {
			continue
		}
		if len(lhs) == 1 && len(rhs) == 1 {
			if source, ok := unwrapParens(rhs[i]).(*ast.Ident); ok && funcValueExprIsStorage(source) {
				// The definition already copied the variable's func
				// value into a cell of its own.
				continue
			}
		}
		NeedGoFuncValue()
		name := RustLocalIdent(ident.Name)
		out.WriteString("\n")
		out.WriteString(indent)
		out.WriteString("let mut ")
		out.WriteString(name)
		out.WriteString(" = ")
		if len(lhs) == len(rhs) {
			if source, ok := unwrapParens(rhs[i]).(*ast.Ident); ok && isPackageGlobalIdent(source) {
				// Read the variable again rather than the copy just
				// made, which for a package variable is a closure that
				// calls it.
				writeFuncValueFreshCell(out, source)
				out.WriteString(";")
				continue
			}
		}
		out.WriteString(GetOuterWrapperType())
		out.WriteString("::new(")
		out.WriteString(GetInnerWrapperType())
		out.WriteString("::new(GoFuncValue::go_func_value(&")
		out.WriteString(name)
		out.WriteString(")));")
	}
}

// deferredFuncValueName reports whether name is a func-typed local variable
// that the deferred call `call` uses only as its function or as a whole
// argument. Go evaluates both when the defer statement runs, so the defer
// keeps a copy of the func value rather than the variable.
func deferredFuncValueName(call *ast.CallExpr, name string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || call == nil {
		return false
	}
	isName := func(expr ast.Expr) bool {
		ident, ok := unwrapParens(expr).(*ast.Ident)
		if !ok || ident.Name != name {
			return false
		}
		obj, ok := typeInfo.info.Uses[ident].(*types.Var)
		return ok && isFunctionSignatureType(obj.Type()) && obj.Parent() != nil && obj.Pkg() != nil && obj.Parent() != obj.Pkg().Scope()
	}
	found := false
	if isName(call.Fun) {
		found = true
	}
	for _, arg := range call.Args {
		if isName(arg) {
			found = true
		}
	}
	if !found {
		return false
	}
	// Any other use of the name, inside an argument expression, reads the
	// variable later than Go would; leave those calls as they are.
	others := 0
	inspect := func(node ast.Node) {
		ast.Inspect(node, func(n ast.Node) bool {
			if ident, ok := n.(*ast.Ident); ok && ident.Name == name {
				others++
			}
			return true
		})
	}
	direct := 0
	if isName(call.Fun) {
		direct++
	} else {
		inspect(call.Fun)
	}
	for _, arg := range call.Args {
		if isName(arg) {
			direct++
		} else {
			inspect(arg)
		}
	}
	return others == 0 && direct > 0
}

// writeFuncValueFreshCellFromStorage writes a new cell holding a copy of the
// func value the variable or field expr holds, and reports whether expr is
// one. A func value stored in a struct field, a map or a slice is a copy of
// its own: later assignments to the variable must not reach it.
func writeFuncValueFreshCellFromStorage(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	if expected != nil && !isFunctionSignatureType(expected) {
		return false
	}
	switch e := unwrapParens(expr).(type) {
	case *ast.Ident:
		if !funcValueExprIsStorage(e) && !(isPackageGlobalIdent(e) && isFuncValueExpr(e)) {
			return false
		}
	case *ast.SelectorExpr:
		if !funcValueExprIsStorage(e) {
			return false
		}
	default:
		return false
	}
	writeFuncValueFreshCell(out, expr)
	return true
}

// writeFuncValueCallTarget writes the start of a call of the func value in
// the cell named holder: it takes a clone of the closure out of the cell,
// so that the cell is free while the function runs (which may assign the
// variable it was called through), and leaves "(__f)" for the arguments.
func writeFuncValueCallTarget(out *strings.Builder, holder string, funcType string) {
	out.WriteString("let __f: ")
	out.WriteString(funcType)
	out.WriteString(" = ")
	out.WriteString(holder)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap().clone(); (__f)")
}

// writeFuncValueCallTargetLines is writeFuncValueCallTarget over two lines,
// the second indented by indent.
func writeFuncValueCallTargetLines(out *strings.Builder, holder string, funcType string, indent string) {
	out.WriteString("let __f: ")
	out.WriteString(funcType)
	out.WriteString(" = ")
	out.WriteString(holder)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap().clone();\n")
	out.WriteString(indent)
	out.WriteString("(__f)")
}

// structCloneFieldCopiesFuncValue reports whether the struct field declared
// with type expression fieldType holds a func value, which a copy of the
// struct copies into a cell of its own.
func structCloneFieldCopiesFuncValue(fieldType ast.Expr) bool {
	if fieldType == nil {
		return false
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if typ := typeInfo.GetType(fieldType); typ != nil {
			return isFunctionSignatureType(typ)
		}
	}
	return false
}

// writeFuncLitMutableCaptureShadows starts the body of a function literal
// with a mutable copy of each capture the body changes in place: the
// receiver it calls pointer methods on, or a capture it assigns. A func value
// is an Fn closure, which cannot change what it captured; the copy shares
// the captured value's cells, so the changes the body makes through them
// reach the variable as before.
func writeFuncLitMutableCaptureShadows(out *strings.Builder, funcLit *ast.FuncLit, captured map[string]bool, captureRenames map[string]string, assigned map[string]bool) {
	names := make([]string, 0, len(captured))
	for varName := range captured {
		names = append(names, varName)
	}
	sort.Strings(names)
	for _, varName := range names {
		capturesReceiver := currentReceiver != "" && varName == currentReceiver && funcLitCapturesCurrentReceiver(funcLit)
		if !capturesReceiver && !assigned[varName] {
			continue
		}
		name := captureRenames[varName]
		if name == "" {
			continue
		}
		rustName := RustLocalIdent(name)
		out.WriteString("        let mut ")
		out.WriteString(rustName)
		out.WriteString(" = ")
		out.WriteString(rustName)
		out.WriteString(".clone();\n")
	}
}
