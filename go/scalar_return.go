package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

// Predeclared Copy scalar types whose return slot can be emitted as a bare
// Rust value instead of the wrapper handle. Strings are excluded because of
// owned/borrowed nuance; named types are excluded so newtype semantics are
// preserved. Pointers, slices, maps, channels, interfaces, structs, arrays,
// and error are all out by definition.
func basicKindIsCopyScalar(kind types.BasicKind) bool {
	switch kind {
	case types.Bool,
		types.Int, types.Int8, types.Int16, types.Int32, types.Int64,
		types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64,
		types.Uintptr,
		types.Float32, types.Float64:
		return true
	}
	return false
}

// typeIsPredeclaredCopyScalar returns true when t is a predeclared scalar
// (i.e., *types.Basic with a Copy kind), not a named or aliased type wrapping
// one. Aliases are stripped because `type Foo = int` still refers to the
// predeclared type.
func typeIsPredeclaredCopyScalar(t types.Type) bool {
	if t == nil {
		return false
	}
	t = types.Unalias(t)
	basic, ok := t.(*types.Basic)
	if !ok {
		return false
	}
	return basicKindIsCopyScalar(basic.Kind())
}

func resultTypeExprType(expr ast.Expr) (types.Type, bool) {
	if expr == nil {
		return nil, false
	}
	if t, ok := typeInfoTypeForTypeExpr(expr); ok {
		return t, true
	}
	if GetTypeInfo() == nil {
		return nil, false
	}
	// Synthetic AST paths used by focused tests can miss Types entries for
	// predeclared result identifiers. Resolve only parsed identifiers through
	// go/types' Universe scope; untyped/bodyless synthetic declarations keep
	// the conservative wrapped fallback.
	if ident, ok := expr.(*ast.Ident); ok && ident.NamePos.IsValid() {
		if obj, ok := types.Universe.Lookup(ident.Name).(*types.TypeName); ok {
			return obj.Type(), true
		}
	}
	return nil, false
}

// resultTypeExprIsBareScalar reports whether a Go result type is represented
// as a bare Rust scalar at the function boundary.
func resultTypeExprIsBareScalar(expr ast.Expr) bool {
	t, ok := resultTypeExprType(expr)
	if !ok {
		return false
	}
	return typeIsPredeclaredCopyScalar(t)
}

// GoReturnTypeToRust emits the Rust type used at a function return boundary.
// This differs from GoTypeToRust for predeclared Copy scalars: internally the
// variable may still be a wrapper slot, but callers receive the scalar value.
func GoReturnTypeToRust(expr ast.Expr) string {
	if t, ok := resultTypeExprType(expr); ok && typeIsPredeclaredCopyScalar(t) {
		return goTypesTypeToRust(types.Unalias(t))
	}
	return GoTypeToRust(expr)
}

func writeFunctionResultTypes(out *strings.Builder, fnType *ast.FuncType) {
	if fnType == nil || fnType.Results == nil || len(fnType.Results.List) == 0 {
		return
	}
	out.WriteString(" -> ")
	total := 0
	for _, result := range fnType.Results.List {
		if len(result.Names) == 0 {
			total++
		} else {
			total += len(result.Names)
		}
	}
	// Each slot is lowered independently through GoReturnTypeToRust, so
	// predeclared Copy scalar positions emit as bare Rust scalars in both
	// single- and multi-result signatures while non-scalar positions stay
	// wrapped. The single-result fast path is just an output shape choice.
	if total == 1 {
		out.WriteString(GoReturnTypeToRust(fnType.Results.List[0].Type))
		return
	}
	out.WriteString("(")
	first := true
	for _, result := range fnType.Results.List {
		count := len(result.Names)
		if count == 0 {
			count = 1
		}
		for i := 0; i < count; i++ {
			if !first {
				out.WriteString(", ")
			}
			first = false
			out.WriteString(GoReturnTypeToRust(result.Type))
		}
	}
	out.WriteString(")")
}

func writeSignatureResultTypes(out *strings.Builder, sig *types.Signature) bool {
	if sig == nil || sig.Results() == nil || sig.Results().Len() == 0 {
		return false
	}
	results := sig.Results()
	out.WriteString(" -> ")
	if results.Len() == 1 {
		out.WriteString(goTypesReturnTypeToRust(results.At(0).Type()))
		return true
	}
	out.WriteString("(")
	for i := 0; i < results.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(goTypesReturnTypeToRust(results.At(i).Type()))
	}
	out.WriteString(")")
	return true
}

func funcDeclSignatureFromTypeInfo(fn *ast.FuncDecl) (*types.Signature, bool) {
	if fn == nil {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	obj, ok := typeInfo.info.Defs[fn.Name].(*types.Func)
	if !ok || obj == nil {
		return nil, false
	}
	return signatureFromType(obj.Type())
}

func writeFuncDeclResultTypes(out *strings.Builder, fn *ast.FuncDecl) {
	// Register any anonymous struct types in the signature so go/types name
	// lookup finds them when emitting the return type.
	if fn != nil && fn.Type != nil {
		registerAnonymousStructTypesInFuncType(fn.Type)
	}
	if sig, ok := funcDeclSignatureFromTypeInfo(fn); ok && writeSignatureResultTypes(out, sig) {
		return
	}
	if fn != nil {
		writeFunctionResultTypes(out, fn.Type)
	}
}

// registerAnonymousStructTypesInFuncType walks the function signature's AST
// looking for `struct { ... }` shapes and pre-registers them via
// generateAnonymousStructType, so lookupAnonymousStructName can find them by
// shape when emitting the return / parameter types.
func registerAnonymousStructTypesInFuncType(fnType *ast.FuncType) {
	visit := func(expr ast.Expr) {
		ast.Inspect(expr, func(n ast.Node) bool {
			if st, ok := n.(*ast.StructType); ok {
				generateAnonymousStructType(st)
			}
			return true
		})
	}
	if fnType.Params != nil {
		for _, field := range fnType.Params.List {
			if field.Type != nil {
				visit(field.Type)
			}
		}
	}
	if fnType.Results != nil {
		for _, field := range fnType.Results.List {
			if field.Type != nil {
				visit(field.Type)
			}
		}
	}
}

// signatureResultIsBareScalar reports whether the result at index is emitted
// as a bare Rust scalar. Result names and the number of sibling results do not
// affect the boundary representation.
func signatureResultIsBareScalar(sig *types.Signature, index int) bool {
	if sig == nil {
		return false
	}
	res := sig.Results()
	if res == nil || index < 0 || index >= res.Len() {
		return false
	}
	return typeIsPredeclaredCopyScalar(res.At(index).Type())
}

// signatureReturnsBareScalar reports whether a Go function signature has
// exactly one result whose type is represented as a bare scalar.
func signatureReturnsBareScalar(sig *types.Signature) bool {
	if sig == nil {
		return false
	}
	res := sig.Results()
	if res == nil || res.Len() != 1 {
		return false
	}
	return signatureResultIsBareScalar(sig, 0)
}

// callReturnsBareScalar checks whether the callee of a CallExpr is itself a
// function whose Go signature returns a single predeclared Copy scalar. This
// lets caller-side lowering treat the result as already-bare instead of
// applying .borrow().as_ref().unwrap() in contexts that expect a raw value,
// and lets it wrap once when storing into a wrapped slot.
//
// Predeclared builtins (len, cap, copy, make, etc.) and stdlib calls with
// custom handlers are excluded: their lowering decides for itself whether
// the result is bare or wrapped, and overriding it here breaks long-standing
// call-site code that wraps the result.
func callReturnsBareScalar(call *ast.CallExpr) bool {
	if call == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if typeInfo.IsTypeConversion(call) {
		return false
	}
	if !callUsesGeneratedReturnSignature(call) {
		return false
	}
	sig, ok := callSignatureFromTypeInfo(call)
	if !ok {
		return false
	}
	return signatureReturnsBareScalar(sig)
}

// callUsesGeneratedReturnSignature returns true for calls whose callee
// signature is mirrored directly in generated Rust: user/source-mapped
// functions, external package stubs, and calls through function-typed
// values (closures, function parameters, function fields). It excludes
// predeclared builtins and stdlib calls that go through custom handlers.
func callUsesGeneratedReturnSignature(call *ast.CallExpr) bool {
	if call == nil {
		return false
	}
	if GetStdlibHandler(call) != nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	if fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call); ok && fn != nil {
		if fn.Pkg() == nil {
			return false
		}
		if fn.Pkg() == typeInfo.pkg {
			return true
		}
		pkgPath := fn.Pkg().Path()
		if !isStdlibPackage(pkgPath) || isSourceMappedPackagePath(pkgPath) {
			return true
		}
		return isStubBackedStdlibPackagePath(pkgPath)
	}
	// Calls through function-typed values (closures, params, function-type
	// fields) lower to `(*__f)(args)` against a generated `Box<dyn Fn...>`
	// whose return slot follows the same bare-scalar rule as direct calls.
	return callIsThroughFunctionValue(typeInfo, call)
}

func callIsThroughFunctionValue(typeInfo *TypeInfo, call *ast.CallExpr) bool {
	if typeInfo == nil || typeInfo.info == nil || call == nil {
		return false
	}
	t := typeInfo.GetType(call.Fun)
	if t == nil {
		return false
	}
	_, ok := types.Unalias(t).Underlying().(*types.Signature)
	return ok
}

func callFunctionObjectFromTypeInfo(typeInfo *TypeInfo, call *ast.CallExpr) (*types.Func, bool) {
	if typeInfo == nil || typeInfo.info == nil || call == nil {
		return nil, false
	}
	switch fun := call.Fun.(type) {
	case *ast.Ident:
		obj := typeInfo.info.Uses[fun]
		if obj == nil {
			return nil, false
		}
		if _, isBuiltin := obj.(*types.Builtin); isBuiltin {
			return nil, false
		}
		fn, ok := obj.(*types.Func)
		if !ok {
			return nil, false
		}
		return fn, true
	case *ast.SelectorExpr:
		if selection := typeInfo.info.Selections[fun]; selection != nil {
			if fn, ok := selection.Obj().(*types.Func); ok {
				return fn, true
			}
		}
		if fn, ok := typeInfo.info.Uses[fun.Sel].(*types.Func); ok {
			return fn, true
		}
	default:
	}
	return nil, false
}

func callResultIsBareScalar(call *ast.CallExpr, index int) bool {
	if call == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.IsTypeConversion(call) {
		return false
	}
	if !callUsesGeneratedReturnSignature(call) && !stdlibHandlerResultUsesBareScalar(call, index) {
		return false
	}
	sig, ok := callSignatureFromTypeInfo(call)
	if !ok {
		return false
	}
	return signatureResultIsBareScalar(sig, index)
}

func stdlibHandlerResultUsesBareScalar(call *ast.CallExpr, index int) bool {
	key, ok := stdlibCallKey(call.Fun)
	if !ok {
		return false
	}
	switch key {
	case "strconv.Atoi":
		return index == 0
	case "strings.Cut":
		return index == 2
	default:
		return false
	}
}

func writeBareScalarConstSelectorReturnValue(out *strings.Builder, expr ast.Expr, expectedType ast.Expr) bool {
	if expectedType == nil {
		return false
	}
	expected, ok := resultTypeExprType(expectedType)
	if !ok {
		return false
	}
	if unary, ok := expr.(*ast.UnaryExpr); ok && unary.Op == token.SUB {
		var inner strings.Builder
		if !writeBareScalarConstSelectorReturnValue(&inner, unary.X, expectedType) {
			return false
		}
		out.WriteString("-(")
		out.WriteString(inner.String())
		out.WriteString(")")
		return true
	}
	if _, ok := expr.(*ast.SelectorExpr); !ok {
		return false
	}
	if !isConstantExpression(expr) {
		return false
	}
	if writeConstExpressionForExpectedInteger(out, expr, expected) {
		return true
	}
	if rustType, ok := rustFloatTypeForGoType(expected); ok {
		TranspileExpression(out, expr)
		out.WriteString(" as ")
		out.WriteString(rustType)
		return true
	}
	return false
}

// writeBareScalarReturnValue emits a single return expression in a context
// that expects a raw Rust scalar (not a wrapper handle). It handles the
// common AST shapes directly and falls back to a wrap-then-unwrap pattern
// for shapes we have not specialized, keeping behavior correct while the
// optimization is rolled out.
func writeBareScalarReturnValue(out *strings.Builder, expr ast.Expr, expectedType ast.Expr) {
	if writeBareScalarConstSelectorReturnValue(out, expr, expectedType) {
		return
	}
	switch e := expr.(type) {
	case *ast.BasicLit:
		writeBareScalarBasicLit(out, e, expectedType)
		return
	case *ast.Ident:
		if expectedType != nil {
			if expected, ok := resultTypeExprType(expectedType); ok && writeRangeIndexForExpectedType(out, e, expected) {
				return
			}
		}
		// RValue context unwraps wrapped locals to a bare value. Constants
		// and literals also lower correctly through this path.
		TranspileExpressionContext(out, e, RValue)
		return
	case *ast.BinaryExpr:
		// Binary expressions on scalar operands compose bare operands into a
		// bare result via the existing RValue lowering.
		TranspileExpression(out, e)
		return
	case *ast.UnaryExpr:
		if e.Op == token.SUB || e.Op == token.NOT || e.Op == token.XOR {
			TranspileExpression(out, e)
			return
		}
	case *ast.ParenExpr:
		out.WriteString("(")
		writeBareScalarReturnValue(out, e.X, expectedType)
		out.WriteString(")")
		return
	case *ast.CallExpr:
		typeInfo := GetTypeInfo()
		if isBareBuiltinReturn(e) {
			if !writeBareBuiltinReturnForExpectedType(out, e, expectedType) {
				TranspileExpression(out, e)
			}
			return
		}
		if callReturnsBareScalar(e) {
			TranspileExpression(out, e)
			return
		}
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(e) {
			out.WriteString("(*")
			TranspileExpression(out, e)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			return
		}
		TranspileExpression(out, e)
		return
	case *ast.SelectorExpr:
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.ReturnsWrappedValue(e) {
			out.WriteString("(*")
			TranspileExpressionContext(out, e, LValue)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			return
		}
		TranspileExpression(out, e)
		return
	case *ast.IndexExpr:
		// Indexing a slice/array of scalars yields a bare scalar in the
		// existing RValue lowering.
		TranspileExpression(out, e)
		return
	}
	// Fallback: emit the expression. If it produces a wrapper, unwrap once.
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.ReturnsWrappedValue(expr) {
		out.WriteString("(*")
		TranspileExpression(out, expr)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
		return
	}
	TranspileExpression(out, expr)
}

// writeBareScalarBasicLit emits a literal with the right Rust scalar type
// for the function's declared return type. Without the cast, `2024` would
// be `i32` by Rust's defaulting, which happens to match Go's `int` -> `i32`
// but breaks for `int64`, `uint`, floats with `int` exponent style, etc.
func writeBareScalarBasicLit(out *strings.Builder, lit *ast.BasicLit, expectedType ast.Expr) {
	if lit == nil {
		return
	}
	rustType := ""
	if expectedType != nil {
		rustType = goTypeToRustBase(expectedType)
	}
	switch lit.Kind {
	case token.INT:
		// Integer literal. For float-typed returns, emit `.0` suffix so the
		// literal parses as float in Rust.
		if rustType == "f32" || rustType == "f64" {
			out.WriteString(lit.Value)
			out.WriteString(".0")
			if rustType == "f32" {
				out.WriteString("_f32")
			}
			return
		}
		// Untyped Go int literals translate to untyped Rust int literals.
		// Rust's type inference resolves the literal to the surrounding
		// scalar return type without an explicit `as <type>` — the cast is
		// redundant noise that the Twitter critique correctly flagged.
		out.WriteString(lit.Value)
		return
	case token.FLOAT:
		out.WriteString(lit.Value)
		if rustType == "f32" {
			out.WriteString("_f32")
		}
		return
	case token.CHAR:
		// Rare for return types, but support it: emit as scalar.
		out.WriteString("(")
		out.WriteString(lit.Value)
		out.WriteString(" as ")
		if rustType == "" {
			rustType = "i32"
		}
		out.WriteString(rustType)
		out.WriteString(")")
		return
	default:
		out.WriteString(lit.Value)
	}
}

func writeBareScalarZeroValue(out *strings.Builder, typeExpr ast.Expr) {
	rustType := ""
	if typeExpr != nil {
		rustType = GoReturnTypeToRust(typeExpr)
	}
	switch rustType {
	case "bool":
		out.WriteString("false")
	case "f32":
		out.WriteString("0.0_f32")
	case "f64":
		out.WriteString("0.0")
	default:
		out.WriteString("0")
		if rustType != "" {
			out.WriteString(" as ")
			out.WriteString(rustType)
		}
	}
}
