package main

import (
	"bytes"
	"fmt"
	"go/ast"
	"go/format"
	"go/token"
	"go/types"
	"slices"
	"strings"
	"unicode/utf8"
)

var currentReturnStatementIsTail bool
var currentForceWrappedReturnSlots map[int]bool

func pushForceWrappedReturnSlots(slots map[int]bool) func() {
	prev := currentForceWrappedReturnSlots
	currentForceWrappedReturnSlots = slots
	return func() {
		currentForceWrappedReturnSlots = prev
	}
}

func returnSlotForcesWrappedValue(index int) bool {
	if currentForceWrappedReturnSlots == nil {
		return false
	}
	return currentForceWrappedReturnSlots[index]
}

func TranspileTailStatement(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet, comments []*ast.CommentGroup, lastPos *token.Pos, indent string) {
	// Only flag the immediate return-as-tail position. A loop, type switch,
	// or if statement that happens to be the last body statement contains
	// nested return statements that must still emit explicit `return X;`,
	// not become the body's trailing expression.
	if _, isReturn := stmt.(*ast.ReturnStmt); isReturn {
		prev := currentReturnStatementIsTail
		currentReturnStatementIsTail = true
		defer func() { currentReturnStatementIsTail = prev }()
	}
	TranspileStatement(out, stmt, fnType, fileSet, comments, lastPos, indent)
}

// returnExpressionsBorrowWrappedLocals reports whether any of the return
// values would read a wrapped local through a .borrow()/lock() Ref temporary
// whose lifetime, if extended to the function body by tail-expression rules,
// would outlive the local it borrows from. Parameters drop after temporaries
// so they remain safe in tail position; only `let`-declared wrapped locals
// trigger the rejection.
func returnExpressionsBorrowWrappedLocals(results []ast.Expr, fnType *ast.FuncType) bool {
	for _, expr := range results {
		if expressionReadsWrappedLocal(expr, fnType) {
			return true
		}
	}
	return false
}

func expressionReadsWrappedLocal(expr ast.Expr, fnType *ast.FuncType) bool {
	found := false
	ast.Inspect(expr, func(n ast.Node) bool {
		if found {
			return false
		}
		ident, ok := n.(*ast.Ident)
		if !ok {
			return true
		}
		if identIsWrappedFunctionLocal(ident, fnType) {
			found = true
			return false
		}
		return true
	})
	return found
}

// identIsWrappedFunctionLocal reports whether `ident` resolves to a variable
// declared inside the current function body whose generated Rust value uses
// the wrapper shape (Rc/Arc<RefCell/Mutex<Option<T>>>). Such reads in tail
// position produce Ref temporaries that outlive the local under Rust's
// tail-expression scoping rules, so the caller can fall back to an explicit
// `return X;` to keep the temporary scoped to the statement.
//
// The transpiler's default short-decl path wraps locals even when the Go type
// is a Copy scalar (so `&local` and re-binding stay correct). The VarTable
// records the rare opt-outs (bare scalar slot from a bare-scalar call, range
// vars, etc.) as WrapNone. Anything else that's a body-declared local is
// assumed wrapped.
// paramIsReboundInBody reports whether ident is a parameter that the body
// reassigns. Such a parameter gets a cell of its own at the top of the body
// (writeGoSliceParamHeaderCopies), a local that drops before the
// temporaries of a tail expression, like any other body local.
func paramIsReboundInBody(ident *ast.Ident, fnType *ast.FuncType) bool {
	if fnType == nil || fnType.Params == nil || currentFunctionBody == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj := typeInfo.info.Uses[ident]
	if obj == nil {
		return false
	}
	if obj.Pos() < fnType.Params.Pos() || obj.Pos() >= fnType.Params.End() {
		return false
	}
	return funcBodyReassignsVar(currentFunctionBody, obj, typeInfo)
}

func identIsWrappedFunctionLocal(ident *ast.Ident, fnType *ast.FuncType) bool {
	if ident == nil || ident.Name == "_" {
		return false
	}
	if paramIsReboundInBody(ident, fnType) {
		return true
	}
	if info := lookupVarInfo(ident.Name); info != nil {
		return info.Source == SourceLocal && info.WrapLevel != WrapNone
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		// Syntax-only callers cannot distinguish a shadow from the universe
		// constant. Preserve the long-standing literal behavior; production
		// translation installs go/types information before lowering.
		return true
	}
	obj := typeInfo.info.Uses[ident]
	if obj == nil {
		obj = typeInfo.info.Defs[ident]
	}
	v, ok := obj.(*types.Var)
	if !ok || v == nil || v.IsField() {
		return false
	}
	// Package globals are not function locals.
	if v.Pkg() != nil && v.Parent() == v.Pkg().Scope() {
		return false
	}
	// A named result is declared in the Go signature but initialized by a
	// Rust let inside the function. Its borrow must end before that local drops.
	if fnType != nil && fnType.Results != nil && v.Pos() >= fnType.Results.Pos() && v.Pos() < fnType.Results.End() {
		return true
	}
	// Other names declared at or before the function body's opening brace are
	// parameters, receivers, or enclosing-scope variables — they drop after
	// the tail expression's temporaries, so they are safe.
	if !currentFunctionBodyLbrace.IsValid() || v.Pos() <= currentFunctionBodyLbrace {
		return false
	}
	return true
}

func fnHasMultipleResultSlots(fnType *ast.FuncType) bool {
	if fnType == nil || fnType.Results == nil {
		return false
	}
	total := 0
	for _, result := range fnType.Results.List {
		count := len(result.Names)
		if count == 0 {
			count = 1
		}
		total += count
		if total > 1 {
			return true
		}
	}
	return false
}

func callReturnsMultipleResults(call *ast.CallExpr) bool {
	if call == nil {
		return false
	}
	sig, ok := callSignatureFromTypeInfo(call)
	if !ok || sig.Results() == nil {
		return false
	}
	return sig.Results().Len() > 1
}

func callSingleResultType(call *ast.CallExpr) types.Type {
	if call == nil {
		return nil
	}
	if sig, ok := callSignatureFromTypeInfo(call); ok && sig.Results() != nil && sig.Results().Len() == 1 {
		return sig.Results().At(0).Type()
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil
	}
	if typ := typeInfo.GetType(call); typ != nil {
		return typ
	}
	return nil
}

// returnExpressionEmissionStartsWithBlock reports whether the given return
// value would emit Rust source that starts with `{`. Such expressions are
// ambiguous when used as a tail expression: Rust parses the leading `{` as a
// block statement, which breaks shapes like `{ ... } && { ... }` or
// `{ ... } != "x"`.
func returnExpressionEmissionStartsWithBlock(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.BinaryExpr:
		// In concurrent mode every binary expression (other than && / ||)
		// lowers to `{ let __tmp_x = ...; __tmp_x op __tmp_y }`. A logical
		// op then composes two such blocks, leaving the body starting with
		// `{`.
		if NeedsConcurrentWrapper() && e.Op != token.LAND && e.Op != token.LOR {
			return true
		}
		// A nil check on the left can emit a block that scopes its borrow.
		if left, ok := e.X.(*ast.BinaryExpr); ok && isRightNilComparison(left) {
			return true
		}
		// Whatever the operator, the LHS is emitted first; if it would
		// produce a leading `{`, the binary expression inherits the
		// parsing ambiguity.
		return returnExpressionEmissionStartsWithBlock(e.X)
	case *ast.Ident:
		return identExpressionEmissionStartsWithBlock(e)
	case *ast.ParenExpr:
		return returnExpressionEmissionStartsWithBlock(e.X)
	case *ast.SelectorExpr:
		// Named-type field accesses that need a value clone emit through
		// `writeSyntaxNamedSelectorValue` as `{ let __selector_holder = ... }`.
		if selectorSyntaxValueNeedsClone(e) {
			return true
		}
		// Cloneable wrapped field accesses lower through
		// `writeClonedWrappedExpression`, which also wraps the value in a
		// `{ let __selector_holder = ... }` block.
		if !isExpressionResultBare(expr) && isCloneableNonPointerExpr(expr) {
			return true
		}
	case *ast.CallExpr:
		// `len`/`cap` on a named slice/map lower as a `{ let __slice_holder
		// = ... }` block, and the bare-scalar return emitter appends ` as
		// i32` after the call — leaving the body's tail expression starting
		// with `{`.
		if isBareBuiltinCallName(e, "len") || isBareBuiltinCallName(e, "cap") {
			if len(e.Args) > 0 && (isNamedSliceExpression(e.Args[0]) || isNamedMapExpression(e.Args[0])) {
				return true
			}
		}
	case *ast.IndexExpr:
		// An element of a named array ("type Row [3]int") is read through
		// a `{ let __seq_holder = ... }` block.
		if isNamedArrayExpression(e.X) {
			return true
		}
	}
	return false
}

func identExpressionEmissionStartsWithBlock(ident *ast.Ident) bool {
	if ident == nil || !NeedsConcurrentWrapper() || !isCloneableNonPointerIdent(ident) {
		return false
	}
	if isCurrentReceiverIdent(ident) || isPackageGlobalIdent(ident) {
		return false
	}
	if ident.Name == "true" || ident.Name == "false" || ident.Name == "_" {
		return false
	}
	if isLocalConstantIdent(ident) || isConstIdent(ident) {
		return false
	}
	if ident.Name[0] >= 'A' && ident.Name[0] <= 'Z' && ident.Name != "String" && lookupVarInfo(ident.Name) == nil && !isLocalVarIdent(ident) {
		return false
	}
	if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
		return false
	}
	return !isVarBare(ident.Name)
}

// namedReturnsUnwrapBareScalar reports whether the function has at least one
// named return that lowers as a bare scalar slot fed by a wrapped local. Those
// slots use `(*name.borrow().as_ref().unwrap())` at the return boundary, which
// has the same lifetime issue as wrapped local reads inside explicit returns.
func namedReturnsUnwrapBareScalar(fnType *ast.FuncType) bool {
	if fnType == nil || fnType.Results == nil {
		return false
	}
	for _, result := range fnType.Results.List {
		if len(result.Names) == 0 {
			continue
		}
		if !resultTypeExprIsBareScalar(result.Type) {
			continue
		}
		for _, name := range result.Names {
			if name.Name == "_" {
				continue
			}
			return true
		}
	}
	return false
}

func sameExpressionSyntax(a ast.Expr, b ast.Expr) bool {
	var left bytes.Buffer
	var right bytes.Buffer
	if format.Node(&left, token.NewFileSet(), a) != nil {
		return false
	}
	if format.Node(&right, token.NewFileSet(), b) != nil {
		return false
	}
	return left.String() == right.String()
}

func registerBareShortDecl(lhs ast.Expr) {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" {
		return
	}
	if vt := GetVarTable(); vt != nil {
		vt.Register(ident.Name, &VarInfo{
			WrapLevel: WrapNone,
			Source:    SourceLocal,
		})
	}
}

func registerDefaultShortDeclBindings(lhs []ast.Expr) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	vt := GetVarTable()
	if vt == nil {
		return
	}
	for _, expr := range lhs {
		ident, ok := expr.(*ast.Ident)
		if !ok || ident.Name == "_" || isVarDeclaredInCurrentScope(ident.Name) {
			continue
		}
		obj, ok := typeInfo.info.Defs[ident]
		if !ok || obj == nil {
			continue
		}
		info := &VarInfo{
			WrapLevel: WrapFull,
			Source:    SourceLocal,
		}
		if typ := typeInfo.GetType(ident); typ != nil {
			info.RustType = goTypesTypeToRust(typ)
		}
		vt.Register(ident.Name, info)
	}
}

func shortDeclOuterRhsBindingNames(lhs []ast.Expr, rhs ast.Expr) []string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || rhs == nil {
		return nil
	}
	lhsDefs := make(map[string]types.Object)
	for _, expr := range lhs {
		ident, ok := expr.(*ast.Ident)
		if !ok || ident.Name == "_" {
			continue
		}
		obj := typeInfo.info.Defs[ident]
		if obj == nil {
			continue
		}
		lhsDefs[ident.Name] = obj
	}
	if len(lhsDefs) == 0 {
		return nil
	}
	names := make([]string, 0, len(lhsDefs))
	seen := make(map[string]bool)
	ast.Inspect(rhs, func(node ast.Node) bool {
		ident, ok := node.(*ast.Ident)
		if !ok {
			return true
		}
		lhsObj, ok := lhsDefs[ident.Name]
		if !ok {
			return true
		}
		rhsObj := typeInfo.info.Uses[ident]
		if rhsObj == nil || rhsObj == lhsObj || seen[ident.Name] {
			return true
		}
		seen[ident.Name] = true
		names = append(names, ident.Name)
		return true
	})
	return names
}

func hideShortDeclOuterRhsPrebindings(lhs []ast.Expr, rhs ast.Expr) func() {
	names := shortDeclOuterRhsBindingNames(lhs, rhs)
	vt := GetVarTable()
	if len(names) == 0 || vt == nil || len(vt.scopes) == 0 {
		return func() {}
	}
	scopeIndex := len(vt.scopes) - 1
	saved := make(map[string]*VarInfo)
	present := make(map[string]bool)
	for _, name := range names {
		if info, ok := vt.scopes[scopeIndex].vars[name]; ok {
			saved[name] = info
			present[name] = true
			delete(vt.scopes[scopeIndex].vars, name)
		}
	}
	return func() {
		if scopeIndex >= len(vt.scopes) {
			return
		}
		for _, name := range names {
			if present[name] {
				vt.scopes[scopeIndex].vars[name] = saved[name]
			} else {
				delete(vt.scopes[scopeIndex].vars, name)
			}
		}
	}
}

func withShortDeclOuterRhsBindings(lhs []ast.Expr, rhs ast.Expr, emit func()) {
	restore := hideShortDeclOuterRhsPrebindings(lhs, rhs)
	defer restore()
	emit()
}

const minStatementBuiltBinaryInitializerNodes = 4

func writeStatementBuiltLogicalShortDecl(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	binary, ok := unwrapParens(rhs).(*ast.BinaryExpr)
	if !ok || (binary.Op != token.LAND && binary.Op != token.LOR) {
		return false
	}
	if !shouldStatementBuildLogicalCondition(binary) {
		return false
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if _, ok := namedBoolType(typeInfo.GetType(binary)); ok {
			return false
		}
	}

	out.WriteString("let mut ")
	TranspileExpressionContext(out, lhs, LValue)
	out.WriteString(" = ")
	WriteWrapperPrefix(out)
	withShortDeclOuterRhsBindings([]ast.Expr{lhs}, rhs, func() {
		nextTemp := 0
		writeStatementBuiltLogicalConditionValue(out, binary, currentLineIndent(out), &nextTemp)
	})
	WriteWrapperSuffix(out)
	return true
}

func writeStatementBuiltConcurrentBinaryShortDecl(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	temps, result, ok := buildStatementBuiltConcurrentBinaryValue(rhs, []ast.Expr{lhs})
	if !ok {
		return false
	}

	out.WriteString(temps)
	out.WriteString("let mut ")
	TranspileExpressionContext(out, lhs, LValue)
	out.WriteString(" = ")
	WriteWrapperPrefix(out)
	out.WriteString(result)
	WriteWrapperSuffix(out)
	return true
}

func writeStatementBuiltConcurrentBinaryReturnValue(out *strings.Builder, expr ast.Expr) bool {
	temps, result, ok := buildStatementBuiltConcurrentBinaryValue(expr, nil)
	if !ok {
		return false
	}
	out.WriteString("{\n")
	writeIndentedStatementBuiltBinaryTemps(out, temps, "            ")
	out.WriteString("            ")
	out.WriteString(result)
	out.WriteString("\n        }")
	return true
}

func writeIndentedStatementBuiltBinaryTemps(out *strings.Builder, temps string, indent string) {
	temps = strings.TrimSuffix(temps, "\n")
	if temps == "" {
		return
	}
	for _, line := range strings.Split(temps, "\n") {
		if line == "" {
			continue
		}
		out.WriteString(indent)
		out.WriteString(line)
		out.WriteString("\n")
	}
}

func buildStatementBuiltConcurrentBinaryValue(rhs ast.Expr, hiddenShortDeclLhs []ast.Expr) (string, string, bool) {
	return buildStatementBuiltConcurrentBinaryValueWithMinNodes(rhs, hiddenShortDeclLhs, minStatementBuiltBinaryInitializerNodes)
}

func buildStatementBuiltConcurrentBinaryValueWithMinNodes(rhs ast.Expr, hiddenShortDeclLhs []ast.Expr, minNodes int) (string, string, bool) {
	if !NeedsConcurrentWrapper() {
		return "", "", false
	}
	binary, ok := unwrapParens(rhs).(*ast.BinaryExpr)
	if !ok {
		return "", "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", "", false
	}
	if countStatementBuiltConcurrentBinaryNodes(typeInfo, binary) < minNodes {
		return "", "", false
	}

	var temps strings.Builder
	nextTemp := 0
	nextTempName := func() string {
		name := fmt.Sprintf("__go_binary_%d", nextTemp)
		nextTemp++
		return name
	}

	var writeValue func(ast.Expr, *ast.BinaryExpr, ast.Expr) string
	writeValue = func(expr ast.Expr, parent *ast.BinaryExpr, other ast.Expr) string {
		if paren, ok := expr.(*ast.ParenExpr); ok {
			return writeValue(paren.X, parent, other)
		}
		if binary, ok := expr.(*ast.BinaryExpr); ok && statementBuiltConcurrentBinaryCanBuildNode(typeInfo, binary) {
			left := writeValue(binary.X, binary, binary.Y)
			right := writeValue(binary.Y, binary, binary.X)
			name := nextTempName()
			temps.WriteString("let ")
			temps.WriteString(name)
			temps.WriteString(" = ")
			temps.WriteString(left)
			temps.WriteString(" ")
			temps.WriteString(rustBinaryOp(binary.Op))
			temps.WriteString(" ")
			temps.WriteString(right)
			temps.WriteString(";\n")
			return name
		}

		name := nextTempName()
		temps.WriteString("let ")
		temps.WriteString(name)
		temps.WriteString(" = ")
		writeStatementBuiltConcurrentBinaryOperand(&temps, expr, other, parent)
		temps.WriteString(";\n")
		return name
	}

	var result string
	emit := func() {
		result = writeValue(binary, nil, nil)
	}
	if len(hiddenShortDeclLhs) > 0 {
		withShortDeclOuterRhsBindings(hiddenShortDeclLhs, rhs, emit)
	} else {
		emit()
	}
	if result == "" {
		return "", "", false
	}

	return temps.String(), result, true
}

func countStatementBuiltConcurrentBinaryNodes(typeInfo *TypeInfo, expr ast.Expr) int {
	binary, ok := unwrapParens(expr).(*ast.BinaryExpr)
	if !ok || !statementBuiltConcurrentBinaryCanBuildNode(typeInfo, binary) {
		return 0
	}
	return 1 +
		countStatementBuiltConcurrentBinaryNodes(typeInfo, binary.X) +
		countStatementBuiltConcurrentBinaryNodes(typeInfo, binary.Y)
}

func statementBuiltConcurrentBinaryCanBuildNode(typeInfo *TypeInfo, expr *ast.BinaryExpr) bool {
	if typeInfo == nil || expr == nil {
		return false
	}
	if expr.Op == token.LAND || expr.Op == token.LOR {
		return false
	}
	return isUnwrappedBasicIntegerType(typeInfo.GetType(expr))
}

func isUnwrappedBasicIntegerType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if _, isNamed := types.Unalias(typ).(*types.Named); isNamed {
		return false
	}
	basic, ok := types.Unalias(typ).Underlying().(*types.Basic)
	return ok && isIntegerBasicKind(basic.Kind())
}

func writeStatementBuiltConcurrentBinaryOperand(out *strings.Builder, expr ast.Expr, other ast.Expr, parent *ast.BinaryExpr) {
	typeInfo := GetTypeInfo()
	if parent != nil {
		if writeShiftCountPrimitiveOperand(out, expr, parent) {
			return
		}
		if writeConstShiftLeftOperandForResult(out, expr, parent) {
			return
		}
	}
	if other != nil && writeConstExpressionForBinaryPeer(out, expr, other) {
		return
	}

	var expected types.Type
	if typeInfo != nil {
		expected = typeInfo.GetType(expr)
		if parent != nil {
			if parent.Op == token.SHL || parent.Op == token.SHR {
				if expr == parent.X {
					if parentType := typeInfo.GetType(parent); parentType != nil {
						expected = parentType
					}
				}
			} else {
				if parentType := typeInfo.GetType(parent); parentType != nil {
					expected = parentType
				}
			}
		}
	}
	writeBareCompoundAssignValueForOp(out, expr, expected, token.ILLEGAL)
}

func writeBareScalarAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" || !isVarBare(ident.Name) {
		return false
	}
	var expected types.Type
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		expected = typeInfo.GetType(lhs)
	}
	out.WriteString("{ let new_val = ")
	writeBareCompoundAssignValue(out, rhs, expected)
	out.WriteString("; ")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" = new_val; }")
	return true
}

const maxInlineWrappedSlotAssignmentLen = 240

func expressionContainsFunctionValue(expr ast.Expr) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.Ident:
		_, ok := functionValueSignature(e)
		return ok
	case *ast.SelectorExpr:
		if _, ok := methodExpressionSignature(e); ok {
			return true
		}
		if _, ok := pointerMethodValueSignature(e); ok {
			return true
		}
		_, ok := selectorFunctionValueSignature(e)
		return ok
	case *ast.FuncLit:
		return true
	case *ast.CallExpr:
		for _, arg := range e.Args {
			if expressionContainsFunctionValue(arg) {
				return true
			}
		}
	case *ast.BinaryExpr:
		return expressionContainsFunctionValue(e.X) || expressionContainsFunctionValue(e.Y)
	case *ast.UnaryExpr:
		return expressionContainsFunctionValue(e.X)
	case *ast.StarExpr:
		return expressionContainsFunctionValue(e.X)
	case *ast.IndexExpr:
		return expressionContainsFunctionValue(e.X) || expressionContainsFunctionValue(e.Index)
	case *ast.SliceExpr:
		if expressionContainsFunctionValue(e.X) {
			return true
		}
		for _, bound := range []ast.Expr{e.Low, e.High, e.Max} {
			if bound != nil && expressionContainsFunctionValue(bound) {
				return true
			}
		}
	case *ast.TypeAssertExpr:
		return expressionContainsFunctionValue(e.X)
	case *ast.CompositeLit:
		for _, elt := range e.Elts {
			if expressionContainsFunctionValue(elt) {
				return true
			}
		}
	case *ast.KeyValueExpr:
		return expressionContainsFunctionValue(e.Key) || expressionContainsFunctionValue(e.Value)
	}
	return false
}

func writeWrappedSlotAssignmentBlock(out *strings.Builder, lhs ast.Expr, forceMultiline bool, writeValue func(*strings.Builder)) {
	linePrefix := currentLineText(out)
	valuePrefix := linePrefix + "{ let new_val = "
	var value strings.Builder
	value.WriteString(valuePrefix)
	writeValue(&value)
	valueText := strings.TrimPrefix(value.String(), valuePrefix)

	targetPrefix := linePrefix + "{ let new_val = " + valueText + "; *"
	var target strings.Builder
	target.WriteString(targetPrefix)
	TranspileExpressionContext(&target, lhs, LValue)
	WriteBorrowMethod(&target, true)
	targetText := "*" + strings.TrimPrefix(target.String(), targetPrefix)

	inlineLen := len("{ let new_val = ") + len(valueText) + len("; ") + len(targetText) + len(" = Some(new_val); }")
	multiline := forceMultiline && (strings.Contains(valueText, "\n") || strings.Contains(targetText, "\n") || inlineLen > maxInlineWrappedSlotAssignmentLen)
	if !multiline {
		out.WriteString("{ let new_val = ")
		out.WriteString(valueText)
		out.WriteString("; ")
		out.WriteString(targetText)
		out.WriteString(" = Some(new_val); }")
		return
	}

	indent := currentLineIndent(out)
	out.WriteString("{\n")
	out.WriteString(indent)
	out.WriteString("    let new_val = ")
	out.WriteString(valueText)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("    ")
	out.WriteString(targetText)
	out.WriteString(" = Some(new_val);\n")
	out.WriteString(indent)
	out.WriteString("}")
}

func wrappedSlotAssignmentShouldUseMultiline(rhs ast.Expr) bool {
	if expressionContainsFunctionValue(rhs) {
		return true
	}
	return selectorCloneAssignmentValueShouldUseMultiline(rhs)
}

func selectorCloneAssignmentValueShouldUseMultiline(rhs ast.Expr) bool {
	if !NeedsConcurrentWrapper() {
		return false
	}
	sel, ok := unwrapParens(rhs).(*ast.SelectorExpr)
	if !ok {
		return false
	}
	if _, ok := methodExpressionSignature(sel); ok {
		return false
	}
	if !selectorChainContainsCall(sel) {
		return false
	}
	if selectorSyntaxValueNeedsClone(sel) {
		return true
	}
	if isExpressionResultBare(sel) {
		return false
	}
	return isCloneableNonPointerExpr(sel)
}

func selectorChainContainsCall(sel *ast.SelectorExpr) bool {
	found := false
	ast.Inspect(sel.X, func(node ast.Node) bool {
		if found {
			return false
		}
		if _, ok := node.(*ast.CallExpr); ok {
			found = true
			return false
		}
		return true
	})
	return found
}

func wrappedSlotAssignmentRHSIsWrappedVar(rhs ast.Expr) bool {
	rhsIdent, ok := rhs.(*ast.Ident)
	if !ok || rhsIdent.Name == "true" || rhsIdent.Name == "false" || rhsIdent.Name == "nil" {
		return false
	}
	if _, isRange := rangeLoopVars[rhsIdent.Name]; isRange {
		return false
	}
	if _, isConst := localConstants[rhsIdent.Name]; isConst {
		return false
	}
	return !isConstIdent(rhsIdent) && !isVarBare(rhsIdent.Name)
}

func writeRegularWrappedSlotAssignmentValue(out *strings.Builder, lhs ast.Expr, rhs ast.Expr, rhsIsWrappedVar bool) {
	if writeCurrentReceiverAssignmentValue(out, rhs) {
		// Method receivers are Rust self, not wrapped locals named after the Go receiver.
	} else if rhsIsWrappedVar {
		// Use clone to avoid moving non-Copy types like String.
		rhsIdent := rhs.(*ast.Ident)
		rhsVarName := RustIdentForUse(rhsIdent)
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[rhsIdent.Name]; exists {
				rhsVarName = RustLocalIdent(renamed)
			}
		}
		if copyCall, known := goValueCopyCallForExpr(rhsIdent); known {
			out.WriteString(rhsVarName)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()")
			out.WriteString(copyCall)
		} else {
			out.WriteString(goValueCopyNeedsTypeInfo)
		}
	} else if writeNamedIntegerAssignmentValue(out, rhs) {
		// Named integer arithmetic returns the underlying scalar; assignments store the named value in the existing wrapper.
	} else if writeConstAssignmentValue(out, lhs, rhs) {
		// Constants assigned to wrapped slots need the target type from go/types.
	} else if writeByteConstAssignmentValue(out, lhs, rhs) {
		// Byte constants assigned to byte slots need the same go/types context as call arguments.
	} else if writeRangeIndexAssignmentValue(out, lhs, rhs) {
		// Range indexes emit usize, but Go int assignment targets use i32.
	} else if writeRangeCharAssignmentValue(out, lhs, rhs) {
		// String range runes are Rust char but Go rune targets are i32.
	} else if rhsIdent, ok := rhs.(*ast.Ident); ok && writeOwnedRangeValue(out, rhsIdent) {
		// Reference-style range values must be cloned into ordinary wrapped assignment targets.
	} else if writeOwnedExpressionValue(out, rhs) {
		// Copied by value from an existing wrapped field or handle.
	} else if writeValueOfVariableAsGoCopy(out, rhs) {
		// A struct variable is copied into the slot.
	} else if writeGoValueCopyOfElement(out, rhs) {
		// So is an element of a slice or an array, and the target of a pointer.
	} else {
		TranspileExpression(out, rhs)
	}
}

func typeIsPredeclaredMutableBareScalar(t types.Type) bool {
	t = types.Unalias(t)
	basic, ok := t.(*types.Basic)
	if !ok || basic.Kind() == types.Bool {
		return false
	}
	return basicKindIsCopyScalar(basic.Kind())
}

func bareScalarMutationTarget(expr ast.Expr) (*ast.Ident, types.Type, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || ident.Name == "_" || !isVarBare(ident.Name) {
		return nil, nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, nil, false
	}
	t := typeInfo.GetType(expr)
	if !typeIsPredeclaredMutableBareScalar(t) {
		return nil, nil, false
	}
	return ident, t, true
}

func writeBareScalarCompoundAssign(out *strings.Builder, lhs ast.Expr, op token.Token, rhs ast.Expr) bool {
	ident, expected, ok := bareScalarMutationTarget(lhs)
	if !ok {
		return false
	}
	out.WriteString("{ let __rhs = ")
	writeBareCompoundAssignValueForOp(out, rhs, expected, op)
	out.WriteString("; ")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" = ")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" ")
	writeCompoundAssignOperator(out, op)
	out.WriteString(" __rhs; }")
	return true
}

func bareExternalIntegerMutationTarget(expr ast.Expr) (*ast.Ident, *types.Named, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || ident.Name == "_" || !isVarBare(ident.Name) {
		return nil, nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, nil, false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok {
		return nil, nil, false
	}
	if _, ok := externalIntegerRustTypeForNamed(named); !ok {
		return nil, nil, false
	}
	return ident, named, true
}

func writeBareExternalIntegerCompoundAssign(out *strings.Builder, lhs ast.Expr, op token.Token, rhs ast.Expr) bool {
	ident, named, ok := bareExternalIntegerMutationTarget(lhs)
	if !ok {
		return false
	}
	out.WriteString("{ let __rhs = ")
	writeBareCompoundAssignValueForOp(out, rhs, named, op)
	out.WriteString("; ")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" = ")
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(".0 ")
	writeCompoundAssignOperator(out, op)
	out.WriteString(" __rhs); }")
	return true
}

func bareNamedIntegerMutationTarget(expr ast.Expr) (*ast.Ident, *types.Named, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || ident.Name == "_" {
		return nil, nil, false
	}
	if !isVarBare(ident.Name) {
		if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
			return nil, nil, false
		}
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, nil, false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return nil, nil, false
	}
	if _, ok := externalIntegerRustTypeForNamed(named); ok {
		return nil, nil, false
	}
	return ident, named, true
}

func writeBareNamedIntegerCompoundAssign(out *strings.Builder, lhs ast.Expr, op token.Token, rhs ast.Expr) bool {
	ident, named, ok := bareNamedIntegerMutationTarget(lhs)
	if !ok {
		return false
	}
	out.WriteString("{ let __rhs = ")
	writeBareCompoundAssignValueForOp(out, rhs, named, op)
	out.WriteString("; ")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" = ")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" ")
	writeCompoundAssignOperator(out, op)
	out.WriteString(" __rhs; }")
	return true
}

func writeBareScalarIncDec(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	ident, typ, ok := bareScalarMutationTarget(expr)
	if !ok {
		return false
	}
	step := "1"
	if basic, ok := types.Unalias(typ).(*types.Basic); ok {
		switch basic.Kind() {
		case types.Float32, types.Float64:
			step = "1.0"
		}
	}
	out.WriteString("{ ")
	out.WriteString(RustIdentForUse(ident))
	if op == token.INC {
		out.WriteString(" += ")
	} else {
		out.WriteString(" -= ")
	}
	out.WriteString(step)
	out.WriteString("; }")
	return true
}

func incDecStepForType(typ types.Type) string {
	if basic, ok := types.Unalias(typ).(*types.Basic); ok {
		switch basic.Kind() {
		case types.Float32, types.Float64:
			return "1.0"
		}
	}
	return "1"
}

func writeIndexedIncDec(out *strings.Builder, indexExpr *ast.IndexExpr, op token.Token) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Cannot determine indexed inc/dec target - type information required */ ")
		return true
	}
	if typeInfo.IsMap(indexExpr.X) {
		return false
	}
	elemType := typeInfo.GetArrayOrSliceElemType(indexExpr.X)
	if elemType == nil {
		out.WriteString("/* ERROR: Cannot determine indexed inc/dec element type - type information required */ unimplemented!(\"type info required for indexed inc/dec\")")
		return true
	}

	// The index is parenthesised before the cast: "s[len(s)-1]++" cast only
	// the 1, and did not compile.
	out.WriteString("{ ")
	writeIndexedSequenceElementPlace(out, indexExpr.X, func() {
		out.WriteString("let __idx = ")
		writeExpressionAsUsize(out, indexExpr.Index)
		out.WriteString("; ")
	})
	out.WriteString(" __seq[__idx] = ")
	writeIndexedCurrentElementValue(out, elemType)
	out.WriteString(" ")
	if op == token.INC {
		out.WriteString("+ ")
	} else {
		out.WriteString("- ")
	}
	out.WriteString(incDecStepForType(elemType))
	out.WriteString("; }")
	return true
}

func compositeLiteralEmitsBareStructValue(lit *ast.CompositeLit) bool {
	if lit == nil {
		return false
	}
	if _, ok := lit.Type.(*ast.SelectorExpr); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(lit)
	if typ == nil {
		return false
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Struct, *types.Array:
		return true
	default:
		return false
	}
}

// compositeLiteralNeedsLocalValueWrapper reports whether a composite literal
// emits a bare value that a local Go variable must store in its outer value
// cell. Named composite types emit their named Rust value around any inner
// collection handle, while an unnamed struct emits its bare struct value.
// Unnamed maps, slices, and arrays already emit their own local handle.
//
// The second result is false when authoritative type information is missing.
// Callers must emit a loud unsupported path instead of guessing from the AST
// shape; identifiers and selectors can denote structs, maps, slices, or arrays.
func compositeLiteralNeedsLocalValueWrapper(lit *ast.CompositeLit) (bool, bool) {
	if lit == nil {
		return false, true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false, false
	}
	typ := typeInfo.GetType(lit)
	if typ == nil {
		return false, false
	}
	typ = types.Unalias(typ)
	if _, ok := typ.(*types.Named); ok {
		return true, true
	}
	_, isStruct := typ.Underlying().(*types.Struct)
	return isStruct, true
}

// writeAnyBoxInnerValue emits the owned concrete value to box into a Box<dyn
// Any>, unwrapping a wrapped variable or wrapped-returning call, or stripping
// the wrapper off a literal/expression.
// writeNamedIntegerConstNewtypeValue writes a constant of a type the
// package defines over an integer, `Monday` with `type Weekday int`, as a
// value of that type. Its Rust constant is the integer.
func writeNamedIntegerConstNewtypeValue(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isConstantExpression(expr) {
		return false
	}
	// A conversion, Weekday(2), is written as a Weekday already.
	if _, isCall := unwrapParens(expr).(*ast.CallExpr); isCall {
		return false
	}
	typ := typeInfo.GetType(expr)
	if !isNamedIntegerType(typ) || !anyFormatTypeIsLocalNamedBasic(typ) {
		return false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return false
	}
	return writeExpressionForExpectedTypesType(out, expr, named)
}

// isReceiveExpr reports whether expr is a receive, <-ch.
func isReceiveExpr(expr ast.Expr) bool {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	return ok && unary.Op == token.ARROW
}

// writeNamedScalarConstChannelValue writes a constant of a named scalar
// type, such as the 5 of ch <- 5 on a chan Level, as a value of that type.
func writeNamedScalarConstChannelValue(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isConstantExpression(expr) {
		return false
	}
	// A conversion, Level(5), is written as a Level already.
	if _, isCall := unwrapParens(expr).(*ast.CallExpr); isCall {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil || !isNamedScalarNewtype(typ) {
		return false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return false
	}
	return writeExpressionForExpectedTypesType(out, expr, named)
}

func writeAnyBoxInnerValue(out *strings.Builder, expr ast.Expr) {
	// A constant goes into the interface as a value of its type.
	if writeNamedIntegerConstNewtypeValue(out, expr) || writeIntegerConstForAnyBox(out, expr) {
		return
	}
	if ident, ok := expr.(*ast.Ident); ok && !isConstIdent(ident) {
		switch ident.Name {
		case "nil", "true", "false":
		default:
			if _, isConst := localConstants[ident.Name]; !isConst {
				if _, isRange := rangeLoopVars[ident.Name]; !isRange {
					out.WriteString("(*")
					out.WriteString(EscapeRustIdent(ident.Name))
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).clone()")
					return
				}
			}
		}
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			out.WriteString("(*")
			TranspileExpression(out, call)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone()")
			return
		}
	}
	var buf strings.Builder
	TranspileExpression(&buf, expr)
	s := buf.String()
	wrapPrefix := GetOuterWrapperType() + "::new(" + GetInnerWrapperType() + "::new(Some("
	wrapSuffix := ")))"
	if strings.HasPrefix(s, wrapPrefix) && strings.HasSuffix(s, wrapSuffix) {
		out.WriteString(s[len(wrapPrefix) : len(s)-len(wrapSuffix)])
		return
	}
	out.WriteString(s)
}

// writeBareAnyBox emits `Box::new(<value>) as Box<dyn Any...>`, boxing a concrete
// expr into a bare Box<dyn Any> trait object (the element slot of a []any, which
// stores bare boxes).
func writeBareAnyBox(out *strings.Builder, expr ast.Expr) {
	if ident, ok := unwrapParens(expr).(*ast.Ident); ok && ident.Name == "nil" {
		writeNilAnyBox(out)
		return
	}
	// An interface value is copied into a box of its own, nil as GoNilAny.
	if isEmptyInterfaceValueExpr(expr) && writeExistingAnyBoxClone(out, expr) {
		return
	}
	if writeStdlibInterfaceTypeConversionAnyBox(out, expr) {
		return
	}
	// An error goes into a []any element the way it goes into any other
	// interface value.
	if writeGoErrorInterfaceAnyBox(out, expr) {
		return
	}
	if writeFmtStringerAnyBox(out, expr) {
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		RegisterAnyCloneType(typeInfo.GetType(expr))
	}
	out.WriteString("Box::new(")
	// A value that keeps its handle when boxed keeps it in a []any element
	// too, the same as every other way of storing it in an interface.
	if !writeAnyHandleBoxInnerValue(out, expr) && !writeDefinedCollectionOwnedValue(out, expr) {
		writeAnyBoxInnerValue(out, expr)
	}
	out.WriteString(") as ")
	out.WriteString(rustAnyTraitObject())
}

func writeArraySliceElementAssignmentValue(out *strings.Builder, rhs ast.Expr, expected types.Type) {
	if writeGoSliceElementHeaderFromVariable(out, rhs, expected) {
		return
	}
	// s[i] = m for a []map: the element is m's handle (#318).
	if writeMapHandleElementValue(out, rhs, expected) {
		return
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" && arraySliceElementExpectedStoresBareCollection(expected) {
		out.WriteString(zeroValueForTypesType(expected))
		return
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" && isUnsafePointerLikeType(expected) {
		out.WriteString(zeroValueForTypesType(expected))
		return
	}
	if isGoErrorType(expected) && writeGoErrorHandleValue(out, rhs) {
		return
	}
	if writeBuiltinLengthForIntegerSlot(out, rhs, expected) {
		return
	}
	if writeTypeParamArraySliceElementAssignmentValue(out, rhs, expected) {
		return
	}
	// A []any element slot stores a bare Box<dyn Any>; box a concrete value into
	// it. (A value already of interface type is handled by the local/stdlib
	// interface paths or the generic clone below.)
	if isEmptyInterfaceType(expected) && !isEmptyInterfaceValueExpr(rhs) {
		writeBareAnyBox(out, rhs)
		return
	}
	if writePointerArraySliceElementAssignmentValue(out, rhs, expected) {
		return
	}
	if expected != nil && writeLocalInterfaceFieldValue(out, rhs, nil, expected) {
		return
	}
	if expected != nil && writeConstExpressionForExpectedGoType(out, rhs, expected) {
		return
	}
	if writeBareValueForWrappedSlot(out, rhs) {
		return
	}
	if expected != nil {
		if named, ok := types.Unalias(expected).(*types.Named); ok && writeNamedIntegerValueForExpected(out, rhs, named) {
			return
		}
	}
	if expected != nil && writeRangeCharForExpectedType(out, rhs, expected) {
		return
	}

	if ident, ok := rhs.(*ast.Ident); ok {
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && varType == "usize" {
			if expected != nil {
				if basic, ok := types.Unalias(expected).Underlying().(*types.Basic); ok && basic.Kind() == types.Int {
					out.WriteString(RustLocalIdent(ident.Name))
					out.WriteString(" as " + rustIntType())
					return
				}
			}
		}
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && strings.HasPrefix(varType, "&") {
			out.WriteString(RustLocalIdent(ident.Name))
			out.WriteString(".clone()")
			return
		}
		// A string in a variable is copied into the element; reading it
		// out of its cell would move it ("s[i] = name").
		if typeInfo := GetTypeInfo(); typeInfo != nil && isWrappedValueIdent(ident) && typeInfo.IsString(ident) {
			writeIdentValueClone(out, ident)
			return
		}
	}

	// A struct held in a variable, or read from "s[i]" or "*p", is copied into
	// the element: the element and its source are two values from here on.
	if writeValueOfVariableAsGoCopy(out, rhs) || writeGoValueCopyOfElement(out, rhs) {
		return
	}

	needsUnwrap := false
	if _, ok := rhs.(*ast.SliceExpr); ok {
		// A slice expression lowers to a wrapped handle, but a value-typed
		// slice/array element slot (e.g. the inner []byte of [][]byte) stores a
		// raw Vec, so the wrapped slice value must be unwrapped to match it
		// (E0308 otherwise). The pointer- and interface-element cases are
		// already handled by the helpers above this point.
		needsUnwrap = true
	} else if _, ok := rhs.(*ast.CompositeLit); ok && arraySliceElementExpectedStoresBareCollection(expected) {
		needsUnwrap = true
	} else if call, ok := rhs.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if appendCallReturnsBareIndexedSlice(call) {
			needsUnwrap = false
		} else if callReturnsBareScalar(call) {
			// Calls whose generated signature already returns a bare scalar
			// don't need .borrow().as_ref().unwrap() at the assignment site.
			needsUnwrap = false
		} else if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			needsUnwrap = true
		} else if typeInfo != nil && typeInfo.IsTypeConversion(call) {
			// Type-driven conversions that emit bare values, such as Word(x),
			// already match value-typed element slots.
			needsUnwrap = false
		} else if ident, ok := call.Fun.(*ast.Ident); ok {
			if !isBuiltinCallTarget(ident) && !isFunctionName(ident) {
				needsUnwrap = true
			}
		}
	}

	if needsUnwrap {
		out.WriteString("(*")
		TranspileExpression(out, rhs)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
	} else {
		TranspileExpression(out, rhs)
	}
}

func indexedSequenceElementAssignmentType(typeInfo *TypeInfo, indexExpr *ast.IndexExpr) types.Type {
	if typeInfo == nil || indexExpr == nil {
		return nil
	}
	if elemType := typeInfo.GetArrayOrSliceElemTypePreservingTypeParam(indexExpr.X); elemType != nil {
		return elemType
	}
	return typeInfo.GetType(indexExpr)
}

func arraySliceElementExpectedStoresBareCollection(expected types.Type) bool {
	if expected == nil {
		return false
	}
	switch types.Unalias(expected).Underlying().(type) {
	case *types.Slice, *types.Map:
		return true
	default:
		return false
	}
}

func writeTypeParamArraySliceElementAssignmentValue(out *strings.Builder, rhs ast.Expr, expected types.Type) bool {
	if _, ok := types.Unalias(expected).(*types.TypeParam); !ok {
		return false
	}
	ident, ok := rhs.(*ast.Ident)
	if !ok || ident.Name == "_" || ident.Name == "nil" || isConstIdent(ident) {
		return false
	}
	name := RustIdentForUse(ident)
	if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && isWrappedRangeVarType(varType) {
		if strings.HasPrefix(varType, "&") {
			out.WriteString("(*")
			out.WriteString(name)
			out.WriteString(").clone()")
		} else {
			out.WriteString(name)
			out.WriteString(".clone()")
		}
		return true
	}
	out.WriteString(name)
	out.WriteString(".clone()")
	return true
}

func writeTypeParamHandleInitializer(out *strings.Builder, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if _, ok := types.Unalias(typeInfo.GetType(rhs)).(*types.TypeParam); !ok {
		return false
	}
	if writePointerConstrainedTypeParamDerefInitializer(out, rhs, typeInfo) {
		return true
	}
	return writeTypeParamHandleExpression(out, rhs)
}

func writeBareDirectTypeParamWrappedReturnValue(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || result == nil || expected == nil {
		return false
	}
	expectedType := typeInfo.GetType(expected)
	if expectedType == nil {
		out.WriteString(`/* ERROR: Type information required for type-parameter return value */ unimplemented!("type info required for type-parameter return value")`)
		return true
	}
	if !isDirectTypeParamType(expectedType) {
		return false
	}
	actualType := typeInfo.GetType(result)
	if actualType == nil {
		out.WriteString(`/* ERROR: Type information required for type-parameter return value */ unimplemented!("type info required for type-parameter return value")`)
		return true
	}
	if !isDirectTypeParamType(actualType) || !types.AssignableTo(actualType, expectedType) || !isExpressionResultBare(result) {
		return false
	}
	WriteWrapperPrefix(out)
	TranspileExpression(out, result)
	WriteWrapperSuffix(out)
	return true
}

func writePointerConstrainedTypeParamDerefInitializer(out *strings.Builder, rhs ast.Expr, typeInfo *TypeInfo) bool {
	star, ok := unwrapParens(rhs).(*ast.StarExpr)
	if !ok || typeInfo == nil {
		return false
	}
	if !goTypeParamHasPointerConstraint(typeInfo.GetType(star.X)) {
		return false
	}
	WriteWrapperPrefix(out)
	TranspileExpressionContext(out, star, RValue)
	WriteWrapperSuffix(out)
	return true
}

func writeNamedSliceSliceExprShortDeclInitializer(out *strings.Builder, rhs ast.Expr) bool {
	if _, ok := rhs.(*ast.SliceExpr); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if _, _, ok := namedSliceTypeFromType(typeInfo.GetType(rhs)); !ok {
		return false
	}
	WriteWrapperPrefix(out)
	TranspileExpression(out, rhs)
	WriteWrapperSuffix(out)
	return true
}

func writeByteConstAssignmentValue(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	expected := typeInfo.GetType(lhs)
	if expected == nil {
		return false
	}
	if !isByteLikeGoType(expected) {
		return false
	}
	return writeConstExpressionForExpectedGoType(out, rhs, expected)
}

func writeConstAssignmentValue(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	expected := typeInfo.GetType(lhs)
	if expected == nil {
		if ident, ok := lhs.(*ast.Ident); ok {
			if obj := typeInfo.GetObject(ident); obj != nil {
				expected = obj.Type()
			}
		}
	}
	if expected == nil {
		return false
	}
	return writeConstExpressionForExpectedGoType(out, rhs, expected)
}

func writePointerDerefAssignmentValue(out *strings.Builder, rhs ast.Expr, expected types.Type) bool {
	if isGoErrorType(expected) && writeGoErrorPointerDerefAssignmentValue(out, rhs) {
		return true
	}
	if call, ok := rhs.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			out.WriteString("(*")
			TranspileExpression(out, rhs)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone()")
			return true
		}
	}
	if expected != nil && writeConstExpressionForExpectedGoType(out, rhs, expected) {
		return true
	}
	if writeBareValueForWrappedSlot(out, rhs) {
		return true
	}
	return false
}

func writeGoPtrBorrowOptionForPointerValue(out *strings.Builder, rhs ast.Expr, expected types.Type) bool {
	info, ok := goPtrInfoForPointerType(expected)
	if !ok || !goPtrHandleValueExpression(rhs) {
		return false
	}
	var handle strings.Builder
	if !writeGoPtrCallArgumentForInfo(&handle, rhs, info) {
		return false
	}
	out.WriteString(handle.String())
	out.WriteString(".borrow()")
	return true
}

func goPtrHandleValueExpression(expr ast.Expr) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.Ident:
		return isGoPtrIdent(e)
	case *ast.CallExpr:
		_, ok := goPtrResultInfoForCall(e, 0)
		return ok
	case *ast.SelectorExpr:
		return generatedGoPtrFieldForSelector(e)
	case *ast.IndexExpr:
		_, ok := goPtrArrayFieldInfoForIndexExpr(e)
		return ok
	case *ast.StarExpr:
		_, ok := goPtrSlotDerefResultInfo(e)
		return ok
	default:
		return false
	}
}

func writePointerDerefPointerHandleAssignment(out *strings.Builder, star *ast.StarExpr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || star == nil {
		return false
	}
	expected := typeInfo.GetType(star)
	if expected == nil {
		return false
	}
	if _, ok := types.Unalias(expected).Underlying().(*types.Pointer); !ok {
		return false
	}
	operandType := typeInfo.GetType(star.X)
	if operandType == nil {
		return false
	}
	operandPtr, ok := types.Unalias(operandType).Underlying().(*types.Pointer)
	if !ok || !types.AssignableTo(expected, operandPtr.Elem()) {
		return false
	}

	rhsIsNil := false
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		rhsIsNil = true
	} else {
		actual := typeInfo.GetType(rhs)
		if actual == nil || !types.AssignableTo(actual, expected) {
			return false
		}
	}

	out.WriteString("{ ")
	rhsIsGoPtrHandle := false
	if rhsIsNil {
		out.WriteString("let new_val = None; ")
	} else {
		out.WriteString("let new_val = ")
		if writeGoPtrBorrowOptionForPointerValue(out, rhs, expected) {
			rhsIsGoPtrHandle = true
		} else {
			writePointerHandleValueClone(out, rhs)
		}
		out.WriteString("; ")
	}
	out.WriteString("let __dst = ")
	TranspileExpressionContext(out, star.X, LValue)
	out.WriteString(".clone(); let __dst_guard = __dst")
	WriteBorrowMethod(out, false)
	out.WriteString("; *__dst_guard.as_ref().unwrap()")
	WriteBorrowMethod(out, true)
	out.WriteString(" = ")
	if rhsIsNil || rhsIsGoPtrHandle {
		out.WriteString("new_val")
	} else {
		out.WriteString("(*new_val")
		WriteBorrowMethod(out, false)
		out.WriteString(").clone()")
	}
	out.WriteString("; }")
	return true
}

func writeGoErrorPointerDerefAssignmentValue(out *strings.Builder, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		writeErrorOptionFromHandleExpression(out, rhs)
		return true
	}
	rhsType := typeInfo.GetType(rhs)
	if isGoErrorType(rhsType) {
		writeErrorOptionFromHandleExpression(out, rhs)
		return true
	}
	if isConcreteGoErrorValue(rhsType) {
		out.WriteString("Some(")
		writeConcreteErrorBox(out, rhs)
		out.WriteString(")")
		return true
	}
	return false
}

func writeWrappedConstVarInitializer(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	if _, ok := rhs.(*ast.BasicLit); ok {
		return false
	}
	var raw strings.Builder
	if !writeConstAssignmentValue(&raw, lhs, rhs) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(raw.String())
	WriteWrapperSuffix(out)
	return true
}

func writeRangeIndexAssignmentValue(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	expected := typeInfo.GetType(lhs)
	if expected == nil {
		if ident, ok := lhs.(*ast.Ident); ok {
			if obj := typeInfo.GetObject(ident); obj != nil {
				expected = obj.Type()
			}
		}
	}
	return writeRangeIndexForExpectedType(out, rhs, expected)
}

func writeRangeCharAssignmentValue(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return writeRangeCharForExpectedType(out, rhs, assignmentTargetType(typeInfo, lhs))
}

func writePointerArraySliceElementAssignmentValue(out *strings.Builder, rhs ast.Expr, expected types.Type) bool {
	if expected == nil {
		return false
	}
	if _, ok := types.Unalias(expected).Underlying().(*types.Pointer); !ok {
		return false
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("Default::default()")
		return true
	}
	if unary, ok := rhs.(*ast.UnaryExpr); ok && unary.Op == token.AND {
		TranspileExpression(out, rhs)
		return true
	}
	if call, ok := rhs.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			TranspileExpression(out, rhs)
			return true
		}
	}
	if !rhsIsPointerType(rhs) {
		return false
	}
	switch rhs.(type) {
	case *ast.Ident, *ast.SelectorExpr:
		if ident, ok := packageGlobalPointerIdent(rhs); ok {
			writePackageGlobalPointerHandleClone(out, ident)
		} else if writeSourceMappedPackageGlobalPointerHandleClone(out, rhs) {
		} else {
			TranspileExpressionContext(out, rhs, LValue)
			out.WriteString(".clone()")
		}
	case *ast.IndexExpr:
		TranspileExpression(out, rhs)
	default:
		return false
	}
	return true
}

func writeBareBuiltinShortDeclInitializer(out *strings.Builder, call *ast.CallExpr, lhs ast.Expr) bool {
	if call == nil {
		return false
	}
	// real and imag give a bare float, as len gives a bare int.
	complexPart := isBareBuiltinCallName(call, "real") || isBareBuiltinCallName(call, "imag")
	if !isBareBuiltinReturn(call) && !complexPart {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	lhsType := typeInfo.GetType(lhs)
	if lhsType == nil {
		lhsType = typeInfo.GetType(call)
	}
	if lhsType == nil {
		return false
	}
	basic, ok := types.Unalias(lhsType).Underlying().(*types.Basic)
	if !ok || basic.Kind() != types.Int {
		if !isBareBuiltinCallName(call, "min") && !isBareBuiltinCallName(call, "max") && !complexPart {
			return false
		}
	}
	WriteWrapperPrefix(out)
	TranspileExpression(out, call)
	if isBareBuiltinCallName(call, "len") || isBareBuiltinCallName(call, "cap") {
		out.WriteString(" as " + rustIntType())
	}
	WriteWrapperSuffix(out)
	return true
}

// shortDeclTargetIsEmptyInterface reports whether lhs, a name a short
// variable declaration defines, has an empty interface type.
func shortDeclTargetIsEmptyInterface(lhs ast.Expr) bool {
	ident, ok := lhs.(*ast.Ident)
	typeInfo := GetTypeInfo()
	if !ok || typeInfo == nil || typeInfo.info == nil {
		return false
	}
	if obj := typeInfo.info.Defs[ident]; obj != nil {
		return isEmptyInterfaceType(obj.Type())
	}
	return isEmptyInterfaceValueExpr(lhs)
}

// shortDeclRhsIsMapElementHandle reports whether rhs reads an element of a
// slice or array of unnamed maps, which is the map's handle (#318).
func shortDeclRhsIsMapElementHandle(rhs ast.Expr) bool {
	index, ok := unwrapParens(rhs).(*ast.IndexExpr)
	return ok && mapElementIndexExpression(index)
}

func writeMultiRhsShortDeclInitializer(out *strings.Builder, lhs ast.Expr, rhs ast.Expr, newBinding bool) {
	if newBinding && shortDeclTargetIsEmptyInterface(lhs) && writeEmptyInterfaceHandleClone(out, rhs) {
		// An interface{} variable is its handle: a, b := <-ch, x gives a
		// the received handle and b a copy of x's, not a cell around one.
		return
	}
	if newBinding && isFuncValueExpr(lhs) {
		// f, g := a, m[k]: each variable gets a cell of its own holding a
		// copy of the func value.
		if !funcValueAssignmentKeepsLowering(rhs) || !writeFunctionValueHandleForExpected(out, rhs, funcValueExprType(lhs)) {
			writeFuncValueFreshCell(out, rhs)
		}
		return
	}
	if ident, ok := lhs.(*ast.Ident); ok && newBinding {
		// m, n := make(map[int]*T), 0 where m holds slice element pointers:
		// the initializer gives the map its GoPtr values, as a single
		// declaration's does.
		if writeSliceElemPtrMapInitializer(out, ident, rhs) {
			return
		}
		if elemRustType, ok := sliceElemPtrSliceCandidateForDecl(ident); ok {
			if writeSliceElemPtrSliceLiteral(out, rhs, elemRustType) {
				return
			}
			if call, ok := unwrapParens(rhs).(*ast.CallExpr); ok && isBuiltinCallNamed(call, "make") && writeSliceElemPtrSliceMake(out, call, elemRustType) {
				return
			}
		}
	}
	if call, isCall := rhs.(*ast.CallExpr); isCall {
		if writeBareBuiltinShortDeclInitializer(out, call, lhs) {
			return
		}
		if callReturnsBareScalar(call) {
			if newBinding {
				registerBareShortDecl(lhs)
				TranspileExpression(out, call)
			} else {
				WriteWrapperPrefix(out)
				TranspileExpression(out, call)
				WriteWrapperSuffix(out)
			}
			return
		}
		writeCallExpressionForInitializer(out, call)
		return
	}
	if _, isSlice := rhs.(*ast.SliceExpr); isSlice {
		// Plain slice expressions already return wrapped values; named-slice
		// slice expressions emit a bare named value that the local wrapper stores.
		if !writeNamedSliceSliceExprShortDeclInitializer(out, rhs) {
			TranspileExpression(out, rhs)
		}
		return
	}
	if unary, ok := rhs.(*ast.UnaryExpr); ok && unary.Op == token.AND {
		// Address-of expressions already return wrapped handles.
		TranspileExpression(out, rhs)
		return
	}
	if newBinding && mapIndexExpressionKeepsHandle(rhs) && isNamedSliceExpression(rhs) {
		// Copy the named slice into its own variable slot while retaining its
		// shared backing storage. Clearing the variable must not clear the map.
		out.WriteString("{ let __slice = ")
		TranspileExpression(out, rhs)
		WriteBorrowMethod(out, false)
		out.WriteString(".clone(); ")
		WriteWrapperOptionPrefix(out)
		out.WriteString("__slice")
		WriteWrapperOptionSuffix(out)
		out.WriteString(" }")
		return
	}
	if index, ok := unwrapParens(rhs).(*ast.IndexExpr); ok && newBinding && mapElementIndexExpression(index) {
		// m, n := s[0], 0 of a []map: the variable is the element's map
		// handle, as m := s[0] gives, and the element can be a nil map.
		if ident, ok := lhs.(*ast.Ident); ok {
			localZeroMapVars[ident.Name] = true
		}
		TranspileExpression(out, rhs)
		return
	}
	if rhsIsPointerType(rhs) {
		// Pointer identifiers in parallel short declarations copy the handle.
		if !writeSourceMappedPackageGlobalPointerHandleClone(out, rhs) {
			TranspileExpressionContext(out, rhs, AddressOf)
			out.WriteString(".clone()")
		}
		return
	}
	if ident, ok := rhs.(*ast.Ident); ok && newBinding && writeMultiShortDeclCollectionCopy(out, ident) {
		return
	}
	if ident, ok := rhs.(*ast.Ident); ok && writeWrappedValueCopyFromIdent(out, ident) {
		// Copied by value from an existing wrapped value.
		return
	}
	if writeNamedIntegerWrappedInitializer(out, rhs) {
		// Typed named integer constants need the named newtype, not the raw const.
		return
	}
	WriteWrapperPrefix(out)
	if !writeOwnedExpressionValue(out, rhs) {
		TranspileExpression(out, rhs)
	}
	WriteWrapperSuffix(out)
}

// writeMultiShortDeclCollectionCopy writes a, m := x, m0 for a variable m0
// of an unnamed map or slice type as m := m0 alone does: a map variable
// shares m0's map, and a slice variable gets a copy of m0's header. Either
// may be nil, which reading the value out of the cell would unwrap.
func writeMultiShortDeclCollectionCopy(out *strings.Builder, ident *ast.Ident) bool {
	if ident.Name == "nil" || isVarBare(ident.Name) || isCurrentReceiverIdent(ident) {
		return false
	}
	if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if _, isVar := typeInfo.GetObject(ident).(*types.Var); !isVar || isPackageGlobalObjectIdent(ident) {
		return false
	}
	switch t := types.Unalias(typeInfo.GetType(ident)).(type) {
	case *types.Map:
		out.WriteString(rustIdentForUseWithCapture(ident))
		out.WriteString(".clone()")
		return true
	case *types.Slice:
		if isEmptyInterfaceType(t.Elem()) {
			return false
		}
		WriteWrapperOptionPrefix(out)
		out.WriteString(rustIdentForUseWithCapture(ident))
		WriteBorrowMethod(out, false)
		out.WriteString(".clone()")
		WriteWrapperOptionSuffix(out)
		return true
	}
	return false
}

func writeNestedSliceElementAssignment(out *strings.Builder, indexExpr *ast.IndexExpr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	root, outer, nested := nestedSequenceIndexChain(indexExpr.X)
	if !nested {
		return false
	}

	// Every index and the value are evaluated before the outer sequence is
	// locked for writing: "grid[i][j] = grid[k][l]" reads the same grid.
	//
	// Go evaluates the outer sequence before its indexes. That shows when
	// the sequence calls a function ("rows()[i()][j()] = v()"), so its
	// handle is then taken first and held, not locked.
	holdsSequence := expressionCallsOrReceives(root)
	// A named slice or array ("type Matrix [][]float64", "type Board
	// [3][3]int") keeps its elements in the handle inside the newtype.
	_, _, namedSequence := namedSliceTypeForExpr(root)
	_, _, namedArray := namedArrayTypeForExpr(root)
	out.WriteString("{ ")
	if namedSequence {
		out.WriteString("let __seq = ")
		writeNamedSliceInnerHandleClone(out, root)
		out.WriteString("; ")
		holdsSequence = true
	} else if namedArray {
		out.WriteString("let __seq = ")
		writeNamedArrayInnerHandleClone(out, root)
		out.WriteString("; ")
		holdsSequence = true
	} else if holdsSequence {
		out.WriteString("let __seq = ")
		TranspileExpressionContext(out, root, LValue)
		out.WriteString(".clone(); ")
	}
	for i, index := range outer {
		out.WriteString("let ")
		out.WriteString(outerIndexTempName(i))
		out.WriteString(" = ")
		writeExpressionAsUsize(out, index)
		out.WriteString("; ")
	}
	out.WriteString("let __idx = ")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("; let __rhs = ")
	writeArraySliceElementAssignmentValue(out, rhs, indexedSequenceElementAssignmentType(typeInfo, indexExpr))
	out.WriteString("; (*")
	if holdsSequence {
		out.WriteString("__seq")
	} else {
		TranspileExpressionContext(out, root, LValue)
	}
	WriteBorrowMethod(out, true)
	out.WriteString(".as_mut().unwrap())")
	for i := range outer {
		out.WriteString("[")
		out.WriteString(outerIndexTempName(i))
		out.WriteString("]")
	}
	out.WriteString("[__idx] = __rhs; }")
	return true
}

func writePointerDerefSequenceElementAssignment(out *strings.Builder, indexExpr *ast.IndexExpr, rhs ast.Expr) bool {
	star, ok := unwrapParens(indexExpr.X).(*ast.StarExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for pointer sequence element assignment */ unimplemented!(\"type info required for pointer sequence element assignment\")")
		return true
	}
	starType := typeInfo.GetType(star)
	operandType := typeInfo.GetType(star.X)
	if starType == nil || operandType == nil {
		out.WriteString("/* ERROR: Type information required for pointer sequence element assignment */ unimplemented!(\"type info required for pointer sequence element assignment\")")
		return true
	}
	if _, isNamed := starType.(*types.Named); isNamed {
		return false
	}
	if !pointerDerefTargetsSequence(starType, operandType) {
		return false
	}

	out.WriteString("{ let __slice_holder = ")
	TranspileExpressionContext(out, star.X, LValue)
	out.WriteString(".clone(); let mut __slice_guard = __slice_holder")
	WriteBorrowMethod(out, true)
	out.WriteString("; let __slice = __slice_guard.as_mut().unwrap(); __slice[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("] = ")
	elemType := indexedSequenceElementAssignmentType(typeInfo, indexExpr)
	if elemRustType, ok := sliceElemPtrSliceCandidateForExpr(indexExpr.X); ok {
		if !writeSliceElemPtrSliceSlotValue(out, rhs, elemRustType) {
			out.WriteString(`unimplemented!("type info required to lower pointer slice assignment")`)
		}
	} else {
		writeArraySliceElementAssignmentValue(out, rhs, elemType)
	}
	out.WriteString("; }")
	return true
}

func writeArrayElemPtrPointedArrayElementAssignment(out *strings.Builder, indexExpr *ast.IndexExpr, rhs ast.Expr) bool {
	ident, ok := unwrapParens(indexExpr.X).(*ast.Ident)
	if !ok || !arrayElemPtrIdentPointsToArray(ident) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for array element pointer indexed assignment */ unimplemented!(\"type info required for array element pointer indexed assignment\")")
		return true
	}
	out.WriteString("{ let new_val = ")
	writeArraySliceElementAssignmentValue(out, rhs, indexedSequenceElementAssignmentType(typeInfo, indexExpr))
	out.WriteString("; (*")
	writeArrayElemPtrBorrow(out, ident, true)
	out.WriteString(".as_mut().unwrap())[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("] = new_val; }")
	return true
}

func writeGoPtrPointedArrayElementAssignment(out *strings.Builder, indexExpr *ast.IndexExpr, rhs ast.Expr) bool {
	ident, ok := unwrapParens(indexExpr.X).(*ast.Ident)
	if !ok {
		return false
	}
	if _, ok := goPtrIdentPointedArrayType(ident); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for GoPtr indexed array assignment */ unimplemented!(\"type info required for GoPtr indexed array assignment\")")
		return true
	}
	out.WriteString("{ let new_val = ")
	writeArraySliceElementAssignmentValue(out, rhs, indexedSequenceElementAssignmentType(typeInfo, indexExpr))
	out.WriteString("; ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".with_mut(|__seq| { __seq[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("] = new_val; }); }")
	return true
}

func pointerDerefTargetsSequence(starType types.Type, operandType types.Type) bool {
	switch coreUnderlyingType(starType).(type) {
	case *types.Array, *types.Slice:
	default:
		return false
	}
	ptr, ok := types.Unalias(operandType).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	switch coreUnderlyingType(ptr.Elem()).(type) {
	case *types.Array, *types.Slice:
		return true
	default:
		return false
	}
}

func writeIndexedSequenceCollectionFieldAssignment(out *strings.Builder, sel *ast.SelectorExpr, rhs ast.Expr) bool {
	indexExpr, ok := sel.X.(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.IsMap(indexExpr.X) {
		return false
	}
	elemType := typeInfo.GetArrayOrSliceElemType(indexExpr.X)
	if elemType == nil {
		return false
	}
	fieldType := typeInfo.GetType(sel)
	if !arraySliceElementExpectedStoresBareCollection(fieldType) {
		return false
	}
	fieldInfo := selectorFieldAccessInfo(sel)

	// A map field is the map's handle: s[i].M = m makes the field share m's
	// map, as r.M = m does, and s[i].M = nil gives it a nil map of its own
	// (#318), promoted or not.
	if isUnnamedMapType(fieldType) {
		out.WriteString("{ let new_val = ")
		writeMapHandleElementValue(out, rhs, fieldType)
		out.WriteString("; ")
		writeIndexedSequenceElementFieldPlace(out, indexExpr, fieldInfo, elemType)
		out.WriteString(" = new_val; }")
		return true
	}

	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("{ *")
		writeIndexedSequenceElementFieldHandle(out, indexExpr, fieldInfo, elemType)
		WriteBorrowMethod(out, true)
		out.WriteString(" = None; }")
		return true
	}

	out.WriteString("{ let new_val = ")
	writeArraySliceElementAssignmentValue(out, rhs, fieldType)
	out.WriteString("; *")
	writeIndexedSequenceElementFieldHandle(out, indexExpr, fieldInfo, elemType)
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(new_val); }")
	return true
}

// writeIndexedSequenceElementFieldPlace writes the field of s[i] as a place to
// assign to: a promoted field is reached through mutable borrows of the
// embedded structs.
func writeIndexedSequenceElementFieldPlace(out *strings.Builder, indexExpr *ast.IndexExpr, fieldInfo FieldAccessInfo, elemType types.Type) {
	if !fieldInfo.IsPromoted {
		writeIndexedSequenceElementFieldHandle(out, indexExpr, fieldInfo, elemType)
		return
	}
	out.WriteString("(*")
	writeIndexedSequenceElementFieldBase(out, indexExpr, elemType)
	for i, embedded := range fieldInfo.EmbeddedPath {
		out.WriteString(".")
		out.WriteString(ToSnakeCase(embedded))
		WriteBorrowMethod(out, true)
		if i < len(fieldInfo.EmbeddedPath)-1 {
			out.WriteString(".as_mut().unwrap()")
		} else {
			out.WriteString(".as_mut().unwrap()).")
		}
	}
	out.WriteString(fieldInfo.FieldName)
}

func writeIndexedSequenceElementFieldHandle(out *strings.Builder, indexExpr *ast.IndexExpr, fieldInfo FieldAccessInfo, elemType types.Type) {
	if fieldInfo.IsPromoted {
		out.WriteString("(*")
		writeIndexedSequenceElementFieldBase(out, indexExpr, elemType)
		for i, embedded := range fieldInfo.EmbeddedPath {
			out.WriteString(".")
			out.WriteString(ToSnakeCase(embedded))
			WriteBorrowMethod(out, false)
			if i < len(fieldInfo.EmbeddedPath)-1 {
				out.WriteString(".as_ref().unwrap()")
			} else {
				out.WriteString(".as_ref().unwrap()).")
			}
		}
		out.WriteString(fieldInfo.FieldName)
		return
	}
	writeIndexedSequenceElementFieldBase(out, indexExpr, elemType)
	out.WriteString(".")
	out.WriteString(fieldInfo.FieldName)
}

func writeIndexedSequenceElementFieldBase(out *strings.Builder, indexExpr *ast.IndexExpr, elemType types.Type) {
	if _, ok := types.Unalias(elemType).Underlying().(*types.Pointer); ok {
		out.WriteString("(*")
		writeIndexedSequenceElementValue(out, indexExpr)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap())")
		return
	}
	writeIndexedSequenceElementValue(out, indexExpr)
}

func writeIndexedSequenceElementValue(out *strings.Builder, indexExpr *ast.IndexExpr) {
	out.WriteString("(*")
	if subj := unwrapParens(indexExpr.X); isNamedSliceExpression(subj) {
		writeNamedSliceInnerHandleClone(out, subj)
	} else {
		TranspileExpressionContext(out, indexExpr.X, LValue)
	}
	WriteBorrowMethod(out, true)
	out.WriteString(".as_mut().unwrap())[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("]")
}

// writeUnwrappedRangeTarget writes a range target expression unwrapped for iteration.
// For CompositeLits (inline slices), generates the bare vec![...] without Rc wrapping.
// For identifiers (variables), delegates to TranspileExpressionContext which already unwraps.
func writeUnwrappedRangeTarget(out *strings.Builder, expr ast.Expr) {
	needsStrip := false
	switch expr.(type) {
	case *ast.CompositeLit:
		needsStrip = true
	case *ast.SliceExpr:
		needsStrip = true
	}

	if needsStrip {
		// Inline slice literal or slice expression - capture output and strip wrapper if present
		var buf strings.Builder
		indent := currentLineIndent(out)
		buf.WriteString(indent)
		TranspileExpressionContext(&buf, expr, RValue)
		s := buf.String()
		if indent != "" && strings.HasPrefix(s, indent) {
			s = s[len(indent):]
		}
		// Strip Rc::new(RefCell::new(Some(...))) wrapper
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		prefix := outerWrapper + "::new(" + innerWrapper + "::new(Some("
		suffix := ")))"
		if strings.HasPrefix(s, prefix) && strings.HasSuffix(s, suffix) {
			out.WriteString(s[len(prefix) : len(s)-len(suffix)])
		} else {
			out.WriteString(s)
		}
	} else if ident, ok := expr.(*ast.Ident); ok {
		if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			out.WriteString(RustLocalIdent(ident.Name))
			return
		}
		TranspileExpressionContext(out, expr, RValue)
	} else {
		TranspileExpressionContext(out, expr, RValue)
	}
}

func writeIntegerRangeLimit(out *strings.Builder, expr ast.Expr) {
	typeInfo := GetTypeInfo()
	if isConstantExpression(expr) {
		if typeInfo != nil {
			if expected := typeInfo.GetType(expr); expected != nil && writeConstExpressionForExpectedGoType(out, expr, expected) {
				return
			}
		}
		TranspileExpression(out, expr)
		return
	}
	if call, ok := expr.(*ast.CallExpr); ok && typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
		out.WriteString("{ let __v = ")
		TranspileExpression(out, call)
		out.WriteString("; let __owned = (*__v")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone(); __owned }")
		return
	}
	if typeInfo != nil && typeInfo.ReturnsWrappedValue(expr) {
		out.WriteString("{ let __range_limit = ")
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone(); let __range_value = { let __range_guard = __range_limit")
		WriteBorrowMethod(out, false)
		out.WriteString("; (*__range_guard.as_ref().unwrap()).clone() }; __range_value }")
		return
	}
	writeUnwrappedRangeTarget(out, expr)
}

func writeRangeLengthExpression(out *strings.Builder, expr ast.Expr) {
	if writePointerToArrayLen(out, expr) {
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		// An array's length is part of its type; a defined array type such
		// as strings' byteReplacer has no len() of its own.
		if typ := typeInfo.GetType(expr); typ != nil {
			if array, ok := types.Unalias(typ).Underlying().(*types.Array); ok {
				out.WriteString(fmt.Sprintf("(%d)", array.Len()))
				return
			}
		}
	}
	if rangeTargetNeedsWrappedSliceGuard(expr) {
		out.WriteString("({ let __range_holder = ")
		if isNamedSliceExpression(expr) {
			writeNamedSliceInnerHandleClone(out, expr)
		} else {
			writeWrappedHandleExpression(out, expr)
			out.WriteString(".clone()")
		}
		out.WriteString("; let __range_guard = __range_holder")
		WriteBorrowMethod(out, false)
		out.WriteString("; __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })")
		return
	}
	writeUnwrappedRangeTarget(out, expr)
	out.WriteString(".len()")
}

func writeNilZeroValueInitializerFromTypeInfo(out *strings.Builder, typeExpr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(typeExpr)
	if typ == nil {
		return false
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		switch types.Unalias(named.Underlying()).(type) {
		case *types.Slice:
			return false
		case *types.Map:
			out.WriteString(" = ")
			WriteWrapperPrefix(out)
			out.WriteString(zeroValueForTypesType(typ))
			WriteWrapperSuffix(out)
			return true
		}
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Interface, *types.Pointer, *types.Signature, *types.Slice, *types.Map:
		out.WriteString(" = ")
		WriteWrappedNone(out)
		return true
	case *types.Chan:
		// A channel variable is the bare GoChannel, and its Default is
		// the nil channel. A cell around it did not compile.
		out.WriteString(" = <")
		out.WriteString(GoTypeToRust(typeExpr))
		out.WriteString(">::default()")
		return true
	default:
		return false
	}
}

func writeWrappedZeroValueInitializerFromTypeInfo(out *strings.Builder, typeExpr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(typeExpr)
	if typ == nil {
		return false
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		switch types.Unalias(named.Underlying()).(type) {
		case *types.Slice, *types.Map:
			return false
		}
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Interface, *types.Pointer, *types.Signature, *types.Slice, *types.Map:
		return false
	}
	out.WriteString(" = ")
	WriteWrapperPrefix(out)
	out.WriteString(zeroValueForTypesType(typ))
	WriteWrapperSuffix(out)
	return true
}

func isWrappedSliceRangeVar(name string) bool {
	varType, ok := rangeLoopVars[name]
	if !ok {
		return false
	}
	prefix := GetOuterWrapperType() + "<" + GetInnerWrapperType() + "<Option<Vec<"
	return strings.HasPrefix(varType, prefix)
}

func isReferenceRangeTarget(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	varType, ok := rangeLoopVars[ident.Name]
	return ok && (varType == "ref_value" || strings.HasPrefix(varType, "&"))
}

func rangeElementUsesCopied(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if isNamedScalarNewtype(typ) {
		return false
	}
	switch underlying := types.Unalias(typ).Underlying().(type) {
	case *types.Basic:
		info := underlying.Info()
		return info&types.IsNumeric != 0 || info&types.IsBoolean != 0
	case *types.Array:
		return rangeElementUsesCopied(underlying.Elem())
	default:
		return false
	}
}

func rangeElementUsesCopiedForExpr(expr ast.Expr, typ types.Type) bool {
	if rangeElementUsesCopied(typ) {
		return true
	}
	return rangeElementSyntaxUsesCopied(expr)
}

func rangeElementSyntaxUsesCopied(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.CompositeLit:
		return rangeElementTypeSyntaxUsesCopied(e.Type)
	case *ast.ParenExpr:
		return rangeElementSyntaxUsesCopied(e.X)
	default:
		return false
	}
}

func rangeElementTypeSyntaxUsesCopied(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.ArrayType:
		return typeSyntaxIsRustCopyRangeElement(e.Elt)
	case *ast.ParenExpr:
		return rangeElementTypeSyntaxUsesCopied(e.X)
	default:
		return false
	}
}

func typeSyntaxIsRustCopyRangeElement(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	switch ident.Name {
	case "bool",
		"byte", "rune",
		"int", "int8", "int16", "int32", "int64",
		"uint", "uint8", "uint16", "uint32", "uint64", "uintptr",
		"float32", "float64":
		return true
	default:
		return false
	}
}

func rangeElementUsesCloned(typ types.Type) bool {
	return isGoErrorType(typ) || isFunctionSignatureType(typ) || isNamedScalarNewtype(typ)
}

func compositeLiteralRangeElemRustType(lit *ast.CompositeLit) (string, bool) {
	if lit == nil {
		return "", false
	}
	arrayType, ok := lit.Type.(*ast.ArrayType)
	if !ok {
		return "", false
	}
	return goCollectionElemTypeToRust(arrayType.Elt), true
}

func registerCompositeLiteralRangeElemType(lhs ast.Expr, lit *ast.CompositeLit) {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" {
		return
	}
	switch typ := lit.Type.(type) {
	case *ast.ArrayType:
		elemRustType := goCollectionElemTypeToRust(typ.Elt)
		localCollectionKinds[ident.Name] = "slice"
		localRangeElemRustTypes[ident.Name] = elemRustType
	case *ast.MapType:
		localCollectionKinds[ident.Name] = "map"
		localMapKeyRustTypes[ident.Name] = goMapKeyTypeToRustBase(typ.Key)
		valueRustType := GoTypeToRust(typ.Value)
		localMapValueRustTypes[ident.Name] = valueRustType
		localMapValueKeepHandle[ident.Name] = mapValueTypeExprKeepsHandle(typ.Value) || rustMapValueTypeKeepsHandle(valueRustType)
	}
}

func registerCompositeLiteralSyntaxVarInfo(lhs ast.Expr, lit *ast.CompositeLit) {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" || lit == nil {
		return
	}
	var rustType string
	switch typ := lit.Type.(type) {
	case *ast.Ident:
		rustType = RustTypeNameForUse(typ.Name)
	case *ast.SelectorExpr:
		rustType = goTypeToRustBase(typ)
	default:
		return
	}
	if vt := GetVarTable(); vt != nil {
		vt.Register(ident.Name, &VarInfo{
			WrapLevel: WrapFull,
			RustType:  rustType,
			Source:    SourceLocal,
		})
	}
}

func localCollectionKind(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return "", false
	}
	kind, ok := localCollectionKinds[ident.Name]
	return kind, ok
}

func localCollectionElemRustType(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || localCollectionKinds[ident.Name] != "slice" {
		return "", false
	}
	elemType := localRangeElemRustTypes[ident.Name]
	if elemType == "" {
		return "", false
	}
	return elemType, true
}

func localMapRangeTypes(expr ast.Expr) (string, string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || localCollectionKinds[ident.Name] != "map" {
		return "", "", false
	}
	keyType := localMapKeyRustTypes[ident.Name]
	valueType := localMapValueRustTypes[ident.Name]
	if keyType == "" || valueType == "" {
		return "", "", false
	}
	return keyType, valueType, true
}

func registerStdlibCallCollectionInfo(lhs ast.Expr, call *ast.CallExpr) {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" || call == nil {
		return
	}
	if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
		if pkg, ok := sel.X.(*ast.Ident); ok && pkg.Name == "strings" {
			switch sel.Sel.Name {
			case "Split", "Fields":
				localCollectionKinds[ident.Name] = "slice"
				localRangeElemRustTypes[ident.Name] = "String"
			}
		}
	}
	if fun, ok := call.Fun.(*ast.Ident); ok && fun.Name == "make" && len(call.Args) > 0 {
		if arrayType, ok := call.Args[0].(*ast.ArrayType); ok && arrayType.Len == nil {
			localCollectionKinds[ident.Name] = "slice"
			localRangeElemRustTypes[ident.Name] = goCollectionElemTypeToRust(arrayType.Elt)
		} else if mapType, ok := call.Args[0].(*ast.MapType); ok {
			localCollectionKinds[ident.Name] = "map"
			localMapKeyRustTypes[ident.Name] = goMapKeyTypeToRustBase(mapType.Key)
			valueRustType := GoTypeToRust(mapType.Value)
			localMapValueRustTypes[ident.Name] = valueRustType
			localMapValueKeepHandle[ident.Name] = mapValueTypeExprKeepsHandle(mapType.Value) || rustMapValueTypeKeepsHandle(valueRustType)
		} else if chanType, ok := call.Args[0].(*ast.ChanType); ok {
			localCollectionKinds[ident.Name] = "channel"
			localRangeElemRustTypes[ident.Name] = goCollectionElemTypeToRust(chanType.Value)
		}
	}
}

func registerCallResultSyntaxInfo(lhs ast.Expr, call *ast.CallExpr) {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" || call == nil {
		return
	}
	registerStdlibCallCollectionInfo(lhs, call)
	if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
		registerSliceElemPtrVar(ident.Name, info.elemRustType)
		return
	}
	if info, ok := arrayElemPtrResultInfoForCall(call, 0); ok {
		registerArrayElemPtrVar(ident.Name, info)
		return
	}
	if arrayType, ok := call.Fun.(*ast.ArrayType); ok && arrayType.Len == nil {
		localCollectionKinds[ident.Name] = "slice"
		localRangeElemRustTypes[ident.Name] = goCollectionElemTypeToRust(arrayType.Elt)
		return
	}
	resultType := callSingleReturnTypeExpr(call)
	if resultType == nil {
		return
	}
	registerTypeExprCollectionInfo(ident.Name, resultType)
	if vt := GetVarTable(); vt != nil {
		rustType := goTypeToRustBase(resultType)
		if functionRustType, ok := functionTypeRustNameFromTypeExpr(resultType); ok {
			rustType = functionRustType
		}
		vt.Register(ident.Name, &VarInfo{
			WrapLevel: WrapFull,
			RustType:  rustType,
			Source:    SourceLocal,
		})
	}
}

func registerCallTupleResultSyntaxInfo(lhs []ast.Expr, call *ast.CallExpr) {
	if call == nil {
		return
	}

	var resultTypes []string
	if key, ok := stdlibCallKey(call.Fun); ok {
		switch key {
		case "context.WithCancel", "context.WithTimeout":
			NeedGoContext()
			resultTypes = []string{"GoContext", "GoCancelFunc"}
		case "context.WithCancelCause":
			NeedGoContext()
			resultTypes = []string{"GoContext", "GoCancelCauseFunc"}
		case "encoding/json.Marshal", "json.Marshal", "encoding/json.MarshalIndent", "json.MarshalIndent":
			resultTypes = []string{"Vec<u8>", goTypeToRustBase(ast.NewIdent("error"))}
		}
	}
	vt := GetVarTable()
	if vt == nil {
		return
	}
	if len(resultTypes) == 0 {
		if results, ok := callResultTupleFromTypeInfo(call); ok {
			for i := 0; i < results.Len() && i < len(lhs); i++ {
				ident, ok := lhs[i].(*ast.Ident)
				if !ok || ident.Name == "_" {
					continue
				}
				if info, ok := sliceElemPtrResultInfoForCall(call, i); ok {
					registerSliceElemPtrVar(ident.Name, info.elemRustType)
					continue
				}
				if info, ok := arrayElemPtrResultInfoForCall(call, i); ok {
					registerArrayElemPtrVar(ident.Name, info)
					continue
				}
				if info, ok := goPtrResultInfoForCall(call, i); ok {
					registerGoPtrVar(ident.Name, goPtrResultElemRustType(info), results.At(i).Type())
					continue
				}
				resultType := types.Unalias(results.At(i).Type())
				registerTypesCollectionInfo(ident.Name, resultType)
				wrapLevel := WrapFull
				if typeIsPredeclaredCopyScalar(resultType) {
					wrapLevel = WrapNone
				}
				vt.Register(ident.Name, &VarInfo{
					WrapLevel: wrapLevel,
					RustType:  goTypesTypeToRust(resultType),
					Source:    SourceLocal,
				})
			}
		}
		return
	}
	for i, rustType := range resultTypes {
		if i >= len(lhs) {
			return
		}
		ident, ok := lhs[i].(*ast.Ident)
		if !ok || ident.Name == "_" {
			continue
		}
		if rustType == "Vec<u8>" {
			localCollectionKinds[ident.Name] = "slice"
			localRangeElemRustTypes[ident.Name] = "u8"
		}
		vt.Register(ident.Name, &VarInfo{
			WrapLevel: WrapFull,
			RustType:  rustType,
			Source:    SourceLocal,
		})
	}
}

func varInfoForStaticallyKnownInterfaceAssertionSource(typeAssert *ast.TypeAssertExpr) (*VarInfo, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeAssert == nil {
		return nil, false
	}
	var targetType types.Type
	source, _, targetType, ok := staticallyKnownInterfaceAssertionSourceAndTarget(typeAssert)
	if !ok {
		source, ok = staticallyKnownAnyInterfaceAssertionSource(typeAssert)
	}
	if !ok || source == nil {
		return nil, false
	}
	sourceType := typeInfo.GetType(source)
	if sourceType == nil {
		return nil, false
	}
	resultType := sourceType
	if targetType != nil {
		resultType = targetType
	}
	info := &VarInfo{
		WrapLevel: WrapFull,
		RustType:  goTypesTypeToRust(resultType),
		Source:    SourceLocal,
		GoType:    resultType,
	}
	if ident, ok := source.(*ast.Ident); ok {
		if sourceInfo := lookupVarInfo(ident.Name); sourceInfo != nil {
			clone := *sourceInfo
			clone.Source = SourceLocal
			clone.RustType = goTypesTypeToRust(resultType)
			clone.GoType = resultType
			info = &clone
		}
	}
	return info, true
}

func registerTypeAssertionResultInfo(lhs ast.Expr, typeAssert *ast.TypeAssertExpr) {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" {
		return
	}
	info, ok := varInfoForStaticallyKnownInterfaceAssertionSource(typeAssert)
	if !ok {
		return
	}
	if vt := GetVarTable(); vt != nil {
		vt.Register(ident.Name, info)
	}
}

func callResultTupleFromTypeInfo(call *ast.CallExpr) (*types.Tuple, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && call != nil {
		if results, ok := typeInfo.GetType(call).(*types.Tuple); ok {
			return results, true
		}
	}
	if sig, ok := callSignatureFromTypeInfo(call); ok {
		return sig.Results(), true
	}
	return nil, false
}

func registerTypesCollectionInfo(name string, typ types.Type) {
	if name == "_" || typ == nil {
		return
	}
	switch ut := types.Unalias(typ).Underlying().(type) {
	case *types.Slice:
		localCollectionKinds[name] = "slice"
		localRangeElemRustTypes[name] = goTypesCollectionElemTypeToRust(ut.Elem())
	case *types.Array:
		localCollectionKinds[name] = "array"
		localRangeElemRustTypes[name] = goTypesCollectionElemTypeToRust(ut.Elem())
	case *types.Map:
		localCollectionKinds[name] = "map"
		localMapKeyRustTypes[name] = goTypesMapKeyToRust(ut.Key())
		valueRustType := goTypesMapValueToRust(ut.Elem())
		localMapValueRustTypes[name] = valueRustType
		localMapValueKeepHandle[name] = mapValueTypeKeepsHandle(ut.Elem()) || rustMapValueTypeKeepsHandle(valueRustType)
	case *types.Chan:
		localCollectionKinds[name] = "channel"
		localRangeElemRustTypes[name] = goTypesCollectionElemTypeToRust(ut.Elem())
	}
}

func shortDeclLHSUsesExistingBinding(lhs ast.Expr) bool {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" {
		return false
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
		if _, ok := typeInfo.info.Uses[ident]; ok {
			return true
		}
		if obj, ok := typeInfo.info.Defs[ident]; ok {
			return obj == nil
		}
	}
	return isVarDeclaredInCurrentScope(ident.Name)
}

func shortDeclExistingBareScalarType(lhs ast.Expr) (types.Type, bool) {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" || !isVarBare(ident.Name) {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	typ := typeInfo.GetType(lhs)
	if !typeIsPredeclaredCopyScalar(typ) {
		return nil, false
	}
	return typ, true
}

func registerTypeExprCollectionInfo(name string, typeExpr ast.Expr) {
	if name == "_" || typeExpr == nil {
		return
	}
	switch typ := typeExpr.(type) {
	case *ast.ArrayType:
		if typ.Len == nil {
			localCollectionKinds[name] = "slice"
			localRangeElemRustTypes[name] = goCollectionElemTypeToRust(typ.Elt)
		} else {
			localCollectionKinds[name] = "array"
			localRangeElemRustTypes[name] = goCollectionElemTypeToRust(typ.Elt)
		}
	case *ast.MapType:
		localCollectionKinds[name] = "map"
		localMapKeyRustTypes[name] = goMapKeyTypeToRustBase(typ.Key)
		valueRustType := GoTypeToRust(typ.Value)
		localMapValueRustTypes[name] = valueRustType
		localMapValueKeepHandle[name] = mapValueTypeExprKeepsHandle(typ.Value) || rustMapValueTypeKeepsHandle(valueRustType)
	case *ast.ChanType:
		localCollectionKinds[name] = "channel"
		localRangeElemRustTypes[name] = goCollectionElemTypeToRust(typ.Value)
	case *ast.Ellipsis:
		localCollectionKinds[name] = "slice"
		localRangeElemRustTypes[name] = goCollectionElemTypeToRust(typ.Elt)
	}
}

func pushFunctionLocalSyntaxInfo() func() {
	prevRangeElemRustTypes := localRangeElemRustTypes
	prevCollectionKinds := localCollectionKinds
	prevMapKeyRustTypes := localMapKeyRustTypes
	prevMapValueRustTypes := localMapValueRustTypes
	prevMapValueKeepHandle := localMapValueKeepHandle
	prevZeroMapVars := localZeroMapVars
	prevBareStructAliases := bareStructAliases
	localRangeElemRustTypes = make(map[string]string)
	localCollectionKinds = make(map[string]string)
	localMapKeyRustTypes = make(map[string]string)
	localMapValueRustTypes = make(map[string]string)
	localMapValueKeepHandle = make(map[string]bool)
	localZeroMapVars = make(map[string]bool)
	bareStructAliases = make(map[string]bool)
	for name := range packageGlobalNames {
		if rustType, ok := prevRangeElemRustTypes[name]; ok {
			localRangeElemRustTypes[name] = rustType
		}
		if kind, ok := prevCollectionKinds[name]; ok {
			localCollectionKinds[name] = kind
		}
		if rustType, ok := prevMapKeyRustTypes[name]; ok {
			localMapKeyRustTypes[name] = rustType
		}
		if rustType, ok := prevMapValueRustTypes[name]; ok {
			localMapValueRustTypes[name] = rustType
		}
		if keepHandle, ok := prevMapValueKeepHandle[name]; ok {
			localMapValueKeepHandle[name] = keepHandle
		}
	}
	return func() {
		localRangeElemRustTypes = prevRangeElemRustTypes
		localCollectionKinds = prevCollectionKinds
		localMapKeyRustTypes = prevMapKeyRustTypes
		localMapValueRustTypes = prevMapValueRustTypes
		localMapValueKeepHandle = prevMapValueKeepHandle
		localZeroMapVars = prevZeroMapVars
		bareStructAliases = prevBareStructAliases
	}
}

func registerZeroValueLocalMap(name *ast.Ident, typeExpr ast.Expr, hasInitializer bool) {
	if name == nil || name.Name == "_" || hasInitializer || typeExpr == nil {
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return
	}
	typ := types.Unalias(typeInfo.GetType(typeExpr))
	if typ == nil {
		return
	}
	if _, named := typ.(*types.Named); named {
		return
	}
	if _, isMap := typ.Underlying().(*types.Map); isMap {
		localZeroMapVars[name.Name] = true
	}
}

func writeMapMutationUnwrap(out *strings.Builder, mapExpr ast.Expr) {
	ident, isIdent := unwrapParens(mapExpr).(*ast.Ident)
	if isIdent && !isPackageGlobalIdent(ident) && localZeroMapVars[ident.Name] {
		out.WriteString(".as_mut().unwrap_or_else(|| panic!(\"assignment to entry in nil map\"))")
		return
	}
	// An element of a []map can be a nil map (#318).
	if index, ok := unwrapParens(mapExpr).(*ast.IndexExpr); ok && mapElementIndexExpression(index) {
		out.WriteString(".as_mut().unwrap_or_else(|| panic!(\"assignment to entry in nil map\"))")
		return
	}
	out.WriteString(".as_mut().unwrap()")
}

func localMapKeyRustType(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || localCollectionKinds[ident.Name] != "map" {
		return "", false
	}
	keyType := localMapKeyRustTypes[ident.Name]
	hasKeyType := keyType != ""
	return keyType, hasKeyType
}

func localMapValueSyntaxKeepsHandle(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok || localCollectionKinds[ident.Name] != "map" {
		return false
	}
	return localMapValueKeepHandle[ident.Name]
}

func mapValueTypeExprKeepsHandle(expr ast.Expr) bool {
	switch typ := expr.(type) {
	case *ast.StarExpr, *ast.MapType, *ast.ChanType, *ast.FuncType, *ast.InterfaceType:
		return true
	case *ast.ArrayType:
		return typ.Len == nil
	case *ast.Ellipsis:
		return true
	case *ast.SelectorExpr:
		return true
	case *ast.Ident:
		if typ.Name == "error" || typ.Name == "any" {
			return true
		}
		if IsInterfaceType(typ.Name) || IsFunctionTypeAlias(typ.Name) {
			return true
		}
		underlying := typeDefinitions[typ.Name]
		return strings.HasPrefix(underlying, "[]") ||
			strings.HasPrefix(underlying, "map[") ||
			strings.HasPrefix(underlying, "*") ||
			strings.HasPrefix(underlying, "chan ") ||
			underlying == "func"
	default:
		return false
	}
}

func trackedRangeElemRustType(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return "", false
	}
	elemRustType, ok := localRangeElemRustTypes[ident.Name]
	if !ok {
		return "", false
	}
	return elemRustType, true
}

func rustRangeElemUsesCopied(rustType string) bool {
	switch rustType {
	case "bool", "char",
		"i8", "i16", "i32", "i64", "isize",
		"u8", "u16", "u32", "u64", "usize",
		"f32", "f64":
		return true
	default:
		return false
	}
}

func rangeValueTypeFromTrackedRustElem(rustType string) string {
	if rustRangeElemUsesCopied(rustType) {
		return rustType
	}
	return "&" + rustType
}

func rustVecElemType(rustType string) (string, bool) {
	rustType = strings.TrimPrefix(rustType, "&")
	if strings.HasPrefix(rustType, "Vec<") && strings.HasSuffix(rustType, ">") {
		return strings.TrimSuffix(strings.TrimPrefix(rustType, "Vec<"), ">"), true
	}
	return "", false
}

func rangeVarSliceElemRustType(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return "", false
	}
	varType, ok := rangeLoopVars[ident.Name]
	if !ok {
		return "", false
	}
	return rustVecElemType(varType)
}

func trackedRangeElemValueType(expr ast.Expr) (string, bool, bool) {
	elemRustType, ok := trackedRangeElemRustType(expr)
	if !ok {
		return "", false, false
	}
	valueType := rangeValueTypeFromTrackedRustElem(elemRustType)
	needsCopied := rustRangeElemUsesCopied(elemRustType)
	return valueType, needsCopied, true
}

func blockIdentAssigned(body *ast.BlockStmt, name string) bool {
	if body == nil || name == "" || name == "_" {
		return false
	}
	assigned := false
	ast.Inspect(body, func(n ast.Node) bool {
		if assigned {
			return false
		}
		switch node := n.(type) {
		case *ast.FuncLit:
			return false
		case *ast.AssignStmt:
			for _, lhs := range node.Lhs {
				if ident, ok := lhs.(*ast.Ident); ok && ident.Name == name {
					assigned = true
					return false
				}
			}
		case *ast.IncDecStmt:
			if ident, ok := node.X.(*ast.Ident); ok && ident.Name == name {
				assigned = true
				return false
			}
		}
		return true
	})
	return assigned
}

// blockCallsPointerMethodOnValue reports whether body calls a method with a
// pointer receiver on the variable name, which holds a value: `c.Inc()` with
// `func (c *Counter) Inc()` takes the address of c.
func blockCallsPointerMethodOnValue(body *ast.BlockStmt, name string) bool {
	typeInfo := GetTypeInfo()
	if body == nil || name == "" || name == "_" || typeInfo == nil || typeInfo.info == nil {
		return false
	}
	found := false
	ast.Inspect(body, func(n ast.Node) bool {
		if found {
			return false
		}
		if _, ok := n.(*ast.FuncLit); ok {
			return false
		}
		call, ok := n.(*ast.CallExpr)
		if !ok {
			return true
		}
		sel, ok := call.Fun.(*ast.SelectorExpr)
		if !ok {
			return true
		}
		ident, ok := sel.X.(*ast.Ident)
		if !ok || ident.Name != name {
			return true
		}
		selection, ok := typeInfo.info.Selections[sel]
		if !ok || selection.Kind() != types.MethodVal {
			return true
		}
		fn, ok := selection.Obj().(*types.Func)
		if !ok {
			return true
		}
		sig, ok := fn.Type().(*types.Signature)
		if !ok || sig.Recv() == nil {
			return true
		}
		if _, recvIsPointer := sig.Recv().Type().(*types.Pointer); !recvIsPointer {
			return true
		}
		if _, valueIsPointer := types.Unalias(typeInfo.GetType(ident)).Underlying().(*types.Pointer); valueIsPointer {
			return true
		}
		found = true
		return false
	})
	return found
}

func rangeLoopIdentAssigned(body *ast.BlockStmt, name string) bool {
	return blockIdentAssigned(body, name)
}

func rangeEmptyInterfaceValueMovedByIndexedWriteback(s *ast.RangeStmt) bool {
	if s == nil || s.Tok != token.DEFINE || !rangeSliceElementIsEmptyInterface(s.X) {
		return false
	}
	rangeIdent, ok := unwrapParens(s.X).(*ast.Ident)
	if !ok || rangeIdent.Name == "_" {
		return false
	}
	keyIdent, ok := s.Key.(*ast.Ident)
	if !ok || keyIdent.Name == "_" {
		return false
	}
	valueIdent, ok := s.Value.(*ast.Ident)
	if !ok || valueIdent.Name == "_" {
		return false
	}
	if s.Body == nil || len(s.Body.List) == 0 {
		return false
	}
	assign, ok := s.Body.List[len(s.Body.List)-1].(*ast.AssignStmt)
	if !ok || assign.Tok != token.ASSIGN || len(assign.Lhs) != 1 || len(assign.Rhs) != 1 {
		return false
	}
	index, ok := assign.Lhs[0].(*ast.IndexExpr)
	if !ok {
		return false
	}
	indexTarget, ok := unwrapParens(index.X).(*ast.Ident)
	if !ok || indexTarget.Name != rangeIdent.Name {
		return false
	}
	indexIdent, ok := unwrapParens(index.Index).(*ast.Ident)
	if !ok || indexIdent.Name != keyIdent.Name {
		return false
	}
	rhsIdent, ok := unwrapParens(assign.Rhs[0]).(*ast.Ident)
	return ok && rhsIdent.Name == valueIdent.Name
}

func writeRangeBinding(out *strings.Builder, expr ast.Expr, mutable bool) {
	if ident, ok := expr.(*ast.Ident); ok {
		if mutable && ident.Name != "_" {
			out.WriteString("mut ")
		}
		out.WriteString(RustLocalIdent(ident.Name))
		return
	}
	TranspileExpression(out, expr)
}

func rangeAssignmentTempName(expr ast.Expr, fallback string) string {
	if expr == nil {
		return ""
	}
	if ident, ok := expr.(*ast.Ident); ok {
		if ident.Name == "_" {
			return ""
		}
		return "__range_" + ToSnakeCase(ident.Name)
	}
	return fallback
}

func writeRangePatternBinding(out *strings.Builder, expr ast.Expr, mutable bool, tempName string) {
	if tempName != "" {
		out.WriteString(tempName)
		return
	}
	writeRangeBinding(out, expr, mutable)
}

func shortDeclNames(stmt ast.Stmt) []string {
	assign, ok := stmt.(*ast.AssignStmt)
	if !ok || assign.Tok != token.DEFINE {
		return nil
	}
	names := make([]string, 0, len(assign.Lhs))
	seen := make(map[string]bool)
	for _, lhs := range assign.Lhs {
		ident, ok := lhs.(*ast.Ident)
		if !ok || ident.Name == "_" || seen[ident.Name] {
			continue
		}
		seen[ident.Name] = true
		names = append(names, ident.Name)
	}
	return names
}

func registerFullShortDecls(names []string) {
	if len(names) == 0 {
		return
	}
	vt := GetVarTable()
	if vt == nil {
		return
	}
	for _, name := range names {
		vt.Register(name, &VarInfo{
			WrapLevel: WrapFull,
			Source:    SourceLocal,
		})
	}
}

func registerRangeLoopVar(name string, rustType string, goType types.Type) {
	rangeLoopVars[name] = rustType
	if goType != nil {
		rangeLoopVarGoTypes[name] = goType
	}
}

func deleteRangeLoopVar(name string) {
	delete(rangeLoopVars, name)
	delete(rangeLoopVarGoTypes, name)
}

func rangeLoopVarGoTypeIsPointer(name string) bool {
	typ := rangeLoopVarGoTypes[name]
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	return ok
}

func shadowRangeLoopVars(names []string) func() {
	if len(names) == 0 {
		return func() {}
	}
	saved := make(map[string]string)
	savedGoTypes := make(map[string]types.Type)
	for _, name := range names {
		if varType, ok := rangeLoopVars[name]; ok {
			saved[name] = varType
			if goType := rangeLoopVarGoTypes[name]; goType != nil {
				savedGoTypes[name] = goType
			}
			deleteRangeLoopVar(name)
		}
	}
	return func() {
		for name, varType := range saved {
			rangeLoopVars[name] = varType
			if goType := savedGoTypes[name]; goType != nil {
				rangeLoopVarGoTypes[name] = goType
			}
		}
	}
}

func funcLitParamNames(funcLit *ast.FuncLit) []string {
	if funcLit == nil || funcLit.Type == nil || funcLit.Type.Params == nil {
		return nil
	}
	var names []string
	for _, field := range funcLit.Type.Params.List {
		for _, name := range field.Names {
			if name != nil && name.Name != "_" {
				names = append(names, name.Name)
			}
		}
	}
	return names
}

// isFunctionLocalStructTypeName reports whether ident names a struct type
// defined inside a function. It is written as an alias of the anonymous
// struct of its fields, which, unlike a Go type alias, has no wrapper of its
// own: a value of it is wrapped as a value of any struct is (#325).
func isFunctionLocalStructTypeName(ident *ast.Ident) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj, ok := typeInfo.info.Uses[ident].(*types.TypeName)
	if !ok {
		obj, ok = typeInfo.info.Defs[ident].(*types.TypeName)
	}
	if !ok || obj.IsAlias() || obj.Pkg() == nil || obj.Parent() == obj.Pkg().Scope() {
		return false
	}
	_, isStruct := obj.Type().Underlying().(*types.Struct)
	return isStruct
}

// funcLiteralParamTypeExpr gives the type expression of the literal's i-th
// parameter, or nil.
func funcLiteralParamTypeExpr(funcLit *ast.FuncLit, i int) ast.Expr {
	if funcLit == nil || funcLit.Type == nil || funcLit.Type.Params == nil {
		return nil
	}
	n := 0
	for _, field := range funcLit.Type.Params.List {
		names := len(field.Names)
		if names == 0 {
			names = 1
		}
		if i < n+names {
			return field.Type
		}
		n += names
	}
	return nil
}

func pushFuncLitParamVarScope(funcLit *ast.FuncLit) func() {
	vt := GetVarTable()
	if vt == nil || funcLit == nil || funcLit.Type == nil || funcLit.Type.Params == nil {
		return func() {}
	}
	vt.PushScope()
	for _, field := range funcLit.Type.Params.List {
		for _, name := range field.Names {
			if name == nil || name.Name == "_" {
				continue
			}
			rustType := goTypeToRustBase(field.Type)
			if functionRustType, ok := functionTypeRustNameFromTypeExpr(field.Type); ok {
				rustType = functionRustType
			}
			registerTypeExprCollectionInfo(name.Name, field.Type)
			if varInfo, ok := interfaceParamVarInfo(field.Type); ok {
				varInfo.RustType = rustType
				vt.Register(name.Name, varInfo)
			} else if _, ok := field.Type.(*ast.ChanType); ok {
				vt.Register(name.Name, &VarInfo{
					WrapLevel: WrapNone,
					RustType:  rustType,
					Source:    SourceParam,
				})
			} else if isSyncParam(field.Type) {
				vt.Register(name.Name, &VarInfo{
					WrapLevel: WrapNone,
					RustType:  rustType,
					Source:    SourceParam,
				})
			} else if typeExprIsRegisteredBareStructAlias(field.Type) {
				vt.Register(name.Name, &VarInfo{
					WrapLevel: WrapNone,
					RustType:  rustType,
					Source:    SourceParam,
				})
			} else if typeExprIsOrderedTypeParam(field.Type) {
				vt.Register(name.Name, &VarInfo{
					WrapLevel: WrapNone,
					RustType:  rustType,
					Source:    SourceParam,
				})
			} else {
				vt.Register(name.Name, &VarInfo{
					WrapLevel: WrapFull,
					RustType:  rustType,
					Source:    SourceParam,
				})
			}
		}
	}
	return vt.PopScope
}

func shortDeclShadowsRangeVar(names []string) bool {
	for _, name := range names {
		if _, ok := rangeLoopVars[name]; ok {
			return true
		}
	}
	return false
}

func rangeTargetNeedsWrappedSliceGuard(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.Ident:
		_, isRangeVar := rangeLoopVars[e.Name]
		return !isRangeVar || isWrappedSliceRangeVar(e.Name)
	case *ast.SelectorExpr:
		return true
	case *ast.CallExpr:
		typeInfo := GetTypeInfo()
		return typeInfo != nil && typeInfo.ReturnsWrappedValue(e)
	case *ast.IndexExpr:
		return mapIndexExpressionKeepsHandle(e)
	case *ast.UnaryExpr:
		typeInfo := GetTypeInfo()
		return e.Op == token.AND && typeInfo != nil && typeInfo.IsPointerToArray(e)
	default:
		return false
	}
}

// writeFunctionTypeAliasWrappedReturn emits the wrapped interface form for a
// function-type-alias value being returned where a local interface is
// expected. Uses the per-interface wrapper struct (FuncType+"As"+Iface) so
// the return value satisfies Box<dyn Iface + Send + Sync>.
func writeFunctionTypeAliasWrappedReturn(out *strings.Builder, result ast.Expr, funcTypeName, ifaceName string) {
	WriteWrapperPrefix(out)
	out.WriteString("Box::new(")
	out.WriteString(funcTypeName)
	out.WriteString("As")
	out.WriteString(ifaceName)
	out.WriteString("(")
	if ident, ok := result.(*ast.Ident); ok {
		if isCurrentReceiverIdent(ident) {
			out.WriteString(currentReceiverRustName())
			out.WriteString(".clone()")
		} else {
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(".clone()")
		}
	} else {
		TranspileExpressionContext(out, result, LValue)
	}
	out.WriteString(")) as ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	WriteWrapperSuffix(out)
}

func writeSourceMappedPointerLocalInterfaceBox(out *strings.Builder, result ast.Expr, expected types.Type, interfaceName string) bool {
	var pointerWrapper strings.Builder
	if !writeSourceMappedPointerInterfaceWrapperValue(&pointerWrapper, result, expected) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString("Box::new(")
	out.WriteString(pointerWrapper.String())
	out.WriteString(") as ")
	out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
	WriteWrapperSuffix(out)
	return true
}

func writeCurrentPackagePointerTranspiledInterfaceBox(out *strings.Builder, result ast.Expr, expected types.Type, interfaceName string) bool {
	var pointerWrapper strings.Builder
	if !writeCurrentPackagePointerTranspiledInterfaceWrapperValue(&pointerWrapper, result, expected) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString("Box::new(")
	out.WriteString(pointerWrapper.String())
	out.WriteString(") as ")
	out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
	WriteWrapperSuffix(out)
	return true
}

// writeChannelTypeAssertionReturnValue writes return x.(chan T): the
// assertion gives the channel, which a function returns bare.
func writeChannelTypeAssertionReturnValue(out *strings.Builder, result ast.Expr) bool {
	assert, ok := unwrapParens(result).(*ast.TypeAssertExpr)
	if !ok || assert.Type == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if _, ok := types.Unalias(typeInfo.GetType(assert.Type)).(*types.Chan); !ok {
		return false
	}
	TranspileExpression(out, result)
	return true
}

func writeLocalInterfaceConcreteReturnConversion(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	interfaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(expectedTypeFromParamExpr(expected))
	if !ok {
		return false
	}
	if ident, ok := result.(*ast.Ident); ok && ident.Name == "nil" {
		WriteWrappedNone(out)
		return true
	}
	if isWrappedInterfaceSliceIndex(result) {
		TranspileExpression(out, result)
		return true
	}
	if mapIndexExpressionKeepsHandle(result) {
		TranspileExpression(out, result)
		return true
	}
	if isBareLocalInterfaceValue(result) {
		WriteWrapperPrefix(out)
		writeLocalInterfaceBareClone(out, result)
		WriteWrapperSuffix(out)
		return true
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		resultType := typeInfo.GetType(result)
		if _, ok := transpiledNamedInterfaceTypeNameFromTypes(resultType); ok {
			targetType := expectedTypeFromParamExpr(expected)
			if targetType != nil && types.AssignableTo(resultType, targetType) {
				return writeLocalInterfaceReferenceCallArgument(out, result, targetType)
			}
		}
		if funcTypeName, ok := functionTypeAliasNameFromTypes(typeInfo.GetType(result)); ok {
			writeFunctionTypeAliasWrappedReturn(out, result, funcTypeName, interfaceName)
			return true
		}
		targetType := expectedTypeFromParamExpr(expected)
		if targetType != nil {
			targetNamed, targetIsNamed := types.Unalias(targetType).(*types.Named)
			sourceType := typeInfo.GetType(result)
			if unary, ok := result.(*ast.UnaryExpr); ok && unary.Op == token.AND && sourceType != nil && types.AssignableTo(sourceType, targetType) {
				if _, isComposite := unary.X.(*ast.CompositeLit); !isComposite {
					WriteWrapperPrefix(out)
					if !writeConcreteLocalInterfaceBox(out, result, interfaceName) {
						return false
					}
					WriteWrapperSuffix(out)
					return true
				}
			}
			if targetIsNamed && targetNamed.Obj() != nil && sourceType != nil {
				if targetInterface, ok := targetNamed.Underlying().(*types.Interface); ok {
					sourceNamedType := sourceType
					if ptr, ok := sourceType.(*types.Pointer); ok {
						sourceNamedType = ptr.Elem()
					}
					sourceNamed, sourceIsNamed := types.Unalias(sourceNamedType).(*types.Named)
					targetInterface.Complete()
					if sourceIsNamed && sourceNamed.Obj() != targetNamed.Obj() && types.Implements(sourceType, targetInterface) {
						if writeCurrentPackagePointerTranspiledInterfaceBox(out, result, targetType, interfaceName) {
							return true
						}
						if writeSourceMappedPointerLocalInterfaceBox(out, result, targetType, interfaceName) {
							return true
						}
						if call, ok := result.(*ast.CallExpr); ok && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
							WriteWrapperPrefix(out)
							out.WriteString("Box::new((*")
							TranspileExpression(out, call)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()).clone()) as ")
							out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
							WriteWrapperSuffix(out)
							return true
						}
						if _, ok := result.(*ast.TypeAssertExpr); ok {
							var boxed strings.Builder
							if writeConcreteLocalInterfaceValue(&boxed, result, targetType, interfaceName) {
								WriteWrapperPrefix(out)
								out.WriteString(boxed.String())
								WriteWrapperSuffix(out)
								return true
							}
						}
						// A conversion/constructor call producing a concrete value that
						// implements the interface (e.g. `return boolVal(b)`) must be
						// boxed; it isn't a wrapped-returning call handled above.
						if _, ok := result.(*ast.CallExpr); ok {
							var boxed strings.Builder
							if writeConcreteLocalInterfaceValue(&boxed, result, targetType, interfaceName) {
								WriteWrapperPrefix(out)
								out.WriteString(boxed.String())
								WriteWrapperSuffix(out)
								return true
							}
						}
						var boxed strings.Builder
						if writeConcreteLocalInterfaceValue(&boxed, result, targetType, interfaceName) {
							WriteWrapperPrefix(out)
							out.WriteString(boxed.String())
							WriteWrapperSuffix(out)
							return true
						}
					}
				}
			}
		}
	}
	if ident, ok := result.(*ast.Ident); ok && ident.Name != "_" {
		WriteWrapperPrefix(out)
		if !writeConcreteLocalInterfaceBox(out, result, interfaceName) {
			out.WriteString("Box::new((*")
			out.WriteString(RustIdentForUse(ident))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone()) as ")
			out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
		}
		WriteWrapperSuffix(out)
		return true
	}
	if unary, ok := result.(*ast.UnaryExpr); ok && unary.Op == token.AND {
		if composite, ok := unary.X.(*ast.CompositeLit); ok {
			WriteWrapperPrefix(out)
			out.WriteString("Box::new(")
			TranspileExpressionContext(out, composite, AddressOf)
			out.WriteString(") as ")
			out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
			WriteWrapperSuffix(out)
			return true
		}
	}
	if composite, ok := result.(*ast.CompositeLit); ok {
		if _, isStructType := composite.Type.(*ast.Ident); isStructType {
			WriteWrapperPrefix(out)
			out.WriteString("Box::new(")
			TranspileExpressionContext(out, composite, AddressOf)
			out.WriteString(") as ")
			out.WriteString(rustLocalInterfaceTraitObject(interfaceName))
			WriteWrapperSuffix(out)
			return true
		}
	}
	return false
}

func writeEmptyInterfaceReturnConversion(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	if !isEmptyInterfaceTypeExpr(expected) {
		return false
	}
	writeEmptyInterfaceCallArgumentValue(out, result)
	return true
}

func writeCurrentReceiverStorage(out *strings.Builder, ident *ast.Ident) bool {
	if !isCurrentReceiverIdent(ident) {
		return false
	}
	if _, isTypeDef := LookupTypeDefinition(currentReceiverType); !isTypeDef {
		return false
	}
	out.WriteString(currentReceiverRustName())
	out.WriteString(".0")
	return true
}

func writeWrappedHandleExpression(out *strings.Builder, expr ast.Expr) {
	if ident, ok := expr.(*ast.Ident); ok {
		if writeCurrentReceiverStorage(out, ident) {
			return
		}
		name := RustIdentForUse(ident)
		if renamed, exists := captureRenameForIdent(ident); exists {
			name = RustLocalIdent(renamed)
		}
		out.WriteString(name)
		return
	}
	TranspileExpressionContext(out, expr, LValue)
}

func isIntegerRangeExpr(typeInfo *TypeInfo, expr ast.Expr) bool {
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	basic, ok := typ.Underlying().(*types.Basic)
	return ok && basic.Info()&types.IsInteger != 0
}

func typeSwitchCaseRustType(typeInfo *TypeInfo, typeExpr ast.Expr) (rustType string, isNil bool) {
	if ident, ok := typeExpr.(*ast.Ident); ok && ident.Name == "nil" {
		return "", true
	}
	if typeInfo == nil {
		return "", false
	}
	typ := typeInfo.GetType(typeExpr)
	if typ == nil {
		return "", false
	}
	if ptr, ok := typ.(*types.Pointer); ok {
		return goTypesTypeToRust(ptr.Elem()), false
	}
	return goTypesTypeToRust(typ), false
}

func typeSwitchCasePointerWrapper(typeInfo *TypeInfo, typeExpr ast.Expr, subjectType types.Type) (wrapperType string, ok bool) {
	if typeInfo == nil {
		return "", false
	}
	targetType := typeInfo.GetType(typeExpr)
	if targetType == nil {
		return "", false
	}
	wrapperType, _, ok = localInterfacePointerAssertionWrapperFor(subjectType, targetType)
	if ok {
		return wrapperType, true
	}
	return sourceMappedPointerInterfaceWrapperTypeForTypes(subjectType, targetType)
}

func typeSwitchSubjectIsNonEmptyInterface(subjectType types.Type) bool {
	if subjectType == nil {
		return false
	}
	iface, ok := types.Unalias(subjectType).Underlying().(*types.Interface)
	return ok && iface.NumMethods() > 0
}

func sourceMappedPointerInterfaceWrapperTypeForTypes(sourceType types.Type, targetType types.Type) (string, bool) {
	if sourceType == nil || targetType == nil {
		return "", false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(sourceType); !ok {
		return "", false
	}
	ptr, ok := types.Unalias(targetType).(*types.Pointer)
	if !ok || !types.AssignableTo(ptr, sourceType) {
		return "", false
	}
	elemNamed, ok := types.Unalias(ptr.Elem()).(*types.Named)
	if !ok || elemNamed.Obj() == nil || elemNamed.Obj().Pkg() == nil {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil && elemNamed.Obj().Pkg() == typeInfo.pkg {
		return "", false
	}
	if !isSourceMappedPackagePath(elemNamed.Obj().Pkg().Path()) {
		return "", false
	}
	if !sourceMappedPointerWrapperAvailableForInterface(elemNamed, sourceType) {
		return "", false
	}
	return sourceMappedPointerWrapperTypeName(elemNamed), true
}

func typeSwitchCaseResolvedType(typeInfo *TypeInfo, typeExpr ast.Expr) types.Type {
	if typeInfo == nil || typeExpr == nil {
		return nil
	}
	typ := typeInfo.GetType(typeExpr)
	if ident, ok := typeExpr.(*ast.Ident); ok {
		if obj, ok := typeInfo.GetObject(ident).(*types.TypeName); ok {
			typ = obj.Type()
		}
	} else if sel, ok := typeExpr.(*ast.SelectorExpr); ok {
		if obj, ok := typeInfo.GetObject(sel.Sel).(*types.TypeName); ok {
			typ = obj.Type()
		}
	}
	return typ
}

func typeSwitchNamedInterfaceFromType(typ types.Type) (*types.Interface, bool) {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return nil, false
	}
	iface, ok := named.Underlying().(*types.Interface)
	if !ok || iface.NumMethods() == 0 {
		return nil, false
	}
	return iface, true
}

func typeSwitchCaseLocalInterface(typeInfo *TypeInfo, typeExpr ast.Expr) (*types.Interface, bool) {
	typ := typeSwitchCaseResolvedType(typeInfo, typeExpr)
	if _, ok := localNamedInterfaceTypeNameFromTypes(typ); !ok {
		return nil, false
	}
	return typeSwitchNamedInterfaceFromType(typ)
}

func typeSwitchCaseTranspiledInterface(typeInfo *TypeInfo, typeExpr ast.Expr, subjectType types.Type) (*types.Interface, bool) {
	if !typeSwitchSubjectIsNonEmptyInterface(subjectType) {
		return nil, false
	}
	typ := typeSwitchCaseResolvedType(typeInfo, typeExpr)
	if _, ok := localNamedInterfaceTypeNameFromTypes(typ); ok {
		return nil, false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(typ); !ok {
		return nil, false
	}
	return typeSwitchNamedInterfaceFromType(typ)
}

// typeSwitchCaseInterface resolves a type-switch case expression to its method
// set when the case denotes an interface with at least one method. It covers
// named local interfaces (`case namer:`), source-mapped imported interfaces
// used in switches over non-empty interface subjects (`case ast.Decl:`), and
// anonymous interface literals (`case interface{ Name() string }:`).
func typeSwitchCaseInterface(typeInfo *TypeInfo, typeExpr ast.Expr, subjectType types.Type) (*types.Interface, bool) {
	if iface, ok := typeSwitchCaseLocalInterface(typeInfo, typeExpr); ok {
		return iface, true
	}
	if iface, ok := typeSwitchCaseTranspiledInterface(typeInfo, typeExpr, subjectType); ok {
		return iface, true
	}
	return anonInterfaceMethodSet(typeExpr)
}

func typeSwitchStaticallyKnownInterfaceCaseTarget(typeInfo *TypeInfo, typeExpr ast.Expr, subjectType types.Type) (string, bool) {
	targetType := typeSwitchCaseResolvedType(typeInfo, typeExpr)
	return staticallyKnownInterfaceRewrapTargetForTypes(subjectType, targetType)
}

func writeTypeSwitchLocalInterfaceCaseCondition(out *strings.Builder, typeInfo *TypeInfo, typeExpr ast.Expr, subjectType types.Type, canRewrapInterfaceSubject bool) bool {
	if writeTypeSwitchFmtStringerCaseCondition(out, typeInfo, typeExpr, subjectType) {
		return true
	}
	iface, ok := typeSwitchCaseInterface(typeInfo, typeExpr, subjectType)
	if !ok {
		return false
	}
	if canRewrapInterfaceSubject {
		if _, ok := typeSwitchStaticallyKnownInterfaceCaseTarget(typeInfo, typeExpr, subjectType); ok {
			out.WriteString("_ts_val.is_some()")
			return true
		}
	}
	candidates := localInterfaceAssertionCandidates(iface, subjectType)
	if len(candidates) == 0 {
		out.WriteString("false")
		return true
	}
	for i, candidate := range candidates {
		if i > 0 {
			out.WriteString(" || ")
		}
		out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
		out.WriteString(candidate.rustType)
		out.WriteString(">()).is_some()")
	}
	return true
}

func typeSwitchInterfaceBindingFallbackRustType(typeInfo *TypeInfo, typeExpr ast.Expr, iface *types.Interface, candidates []localInterfaceAssertionCandidate) string {
	if _, ok := typeExpr.(*ast.InterfaceType); ok {
		if ifaceName := registerAnonymousInterfaceAssertionTrait(iface, candidates); ifaceName != "" {
			return goTypesWrappedRustType(rustLocalInterfaceTraitObject(ifaceName))
		}
	}
	if typeInfo == nil {
		return ""
	}
	if typ := typeInfo.GetType(typeExpr); typ != nil {
		return goTypesTypeToRustWrapped(typ)
	}
	return ""
}

func typeSwitchInterfaceBindingTraitName(typeInfo *TypeInfo, typeExpr ast.Expr, subjectType types.Type, iface *types.Interface, candidates []localInterfaceAssertionCandidate) (string, bool) {
	if _, ok := typeExpr.(*ast.InterfaceType); ok {
		if ifaceName := registerAnonymousInterfaceAssertionTrait(iface, candidates); ifaceName != "" {
			return ifaceName, true
		}
		return "", false
	}
	if typeInfo == nil || typeExpr == nil {
		return "", false
	}
	typ := typeSwitchCaseResolvedType(typeInfo, typeExpr)
	if ifaceName, ok := localNamedInterfaceTypeNameFromTypes(typ); ok {
		return ifaceName, true
	}
	if !typeSwitchSubjectIsNonEmptyInterface(subjectType) {
		return "", false
	}
	return transpiledNamedInterfaceTypeNameFromTypes(typ)
}

// writeTypeSwitchInterfaceCaseBinding binds the case variable for an interface
// case (named or anonymous). Go gives the case variable the case type, even when
// the matched value is rewrapped from a statically known interface subject.
func writeTypeSwitchInterfaceCaseBinding(out *strings.Builder, typeInfo *TypeInfo, varName string, typeExpr ast.Expr, subjectType types.Type, mutable bool, canRewrapInterfaceSubject bool) bool {
	if writeTypeSwitchFmtStringerCaseBinding(out, typeInfo, varName, typeExpr, subjectType, mutable) {
		return true
	}
	iface, ok := typeSwitchCaseInterface(typeInfo, typeExpr, subjectType)
	if !ok {
		return false
	}
	candidates := localInterfaceAssertionCandidates(iface, subjectType)
	out.WriteString("        let ")
	if mutable {
		out.WriteString("mut ")
	}
	out.WriteString(varName)
	if canRewrapInterfaceSubject {
		if ifaceName, ok := typeSwitchStaticallyKnownInterfaceCaseTarget(typeInfo, typeExpr, subjectType); ok {
			out.WriteString(": ")
			out.WriteString(goTypesWrappedRustType(rustLocalInterfaceTraitObject(ifaceName)))
			out.WriteString(" = ")
			writeStaticallyKnownInterfaceAssertionWrappedSome(out, "_ts_owned.as_ref().unwrap()", ifaceName)
			out.WriteString(";\n")
			return true
		}
	}
	if len(candidates) > 0 {
		if ifaceName, ok := typeSwitchInterfaceBindingTraitName(typeInfo, typeExpr, subjectType, iface, candidates); ok {
			out.WriteString(": ")
			out.WriteString(goTypesWrappedRustType(rustLocalInterfaceTraitObject(ifaceName)))
			out.WriteString(" = ")
			for i, candidate := range candidates {
				if i == 0 {
					out.WriteString("if let Some(typed_val) = ")
				} else {
					out.WriteString(" else if let Some(typed_val) = ")
				}
				out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
				out.WriteString(candidate.rustType)
				out.WriteString(">()) {\n")
				out.WriteString("            ")
				writeLocalInterfaceAssertionWrappedCandidateSuccess(out, ifaceName, candidate)
				out.WriteString("\n")
				out.WriteString("        }")
			}
			out.WriteString(" else {\n")
			out.WriteString("            panic!(\"type switch interface case condition matched no concrete implementor\")\n")
			out.WriteString("        };\n")
			return true
		}
	}
	if len(candidates) == 0 {
		if ifaceName, ok := typeSwitchInterfaceBindingTraitName(typeInfo, typeExpr, subjectType, iface, candidates); ok {
			traitObject := rustLocalInterfaceTraitObject(ifaceName)
			out.WriteString(": ")
			out.WriteString(goTypesWrappedRustType(traitObject))
			out.WriteString(" = ")
			writeTypedWrappedNone(out, traitObject)
			out.WriteString(";\n")
			return true
		}
	}
	if len(candidates) != 1 {
		if fallbackType := typeSwitchInterfaceBindingFallbackRustType(typeInfo, typeExpr, iface, candidates); fallbackType != "" {
			out.WriteString(": ")
			out.WriteString(fallbackType)
		}
		out.WriteString(" = ")
		out.WriteString(fmt.Sprintf("unimplemented!(\"type info required: type switch on interface case with %d concrete implementors needs a synthesized trait object\");\n", len(candidates)))
		return true
	}
	out.WriteString(" = ")
	WriteWrapperPrefix(out)
	out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
	out.WriteString(candidates[0].rustType)
	out.WriteString(">()).unwrap().clone()")
	WriteWrapperSuffix(out)
	out.WriteString(";\n")
	return true
}

// typeSwitchCaseAnyHandleRustType reports the handle type a case of a type
// switch over an empty interface has to downcast to, for case types whose
// values keep their handle when boxed into the interface.
func typeSwitchCaseAnyHandleRustType(typeInfo *TypeInfo, typeExpr ast.Expr, subjectType types.Type) (string, bool) {
	if typeInfo == nil || !isEmptyInterfaceType(subjectType) {
		return "", false
	}
	caseType := typeInfo.GetType(typeExpr)
	// An error whose dynamic type cannot be named is boxed as its own handle
	// (see writeOpaqueGoErrorAnyBox). case error has to look for that handle,
	// and its variable is the handle, the same error the box holds.
	if isErrorInterfaceType(caseType) {
		return rustGoErrorHandleType(), true
	}
	return anyBoxedHandleRustType(caseType)
}

func writeTypeSwitchCaseCondition(out *strings.Builder, typeInfo *TypeInfo, typeExpr ast.Expr, subjectType types.Type, canRewrapInterfaceSubject bool) {
	rustType, isNil := typeSwitchCaseRustType(typeInfo, typeExpr)
	if isNil {
		out.WriteString("_ts_is_nil")
		return
	}
	if wrapperType, ok := typeSwitchCasePointerWrapper(typeInfo, typeExpr, subjectType); ok {
		out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
		out.WriteString(wrapperType)
		out.WriteString(">()).is_some()")
		return
	}
	if isEmptyInterfaceType(subjectType) && isErrorInterfaceType(typeInfo.GetType(typeExpr)) {
		out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
		out.WriteString(rustGoErrorHandleType())
		out.WriteString(">()).is_some()")
		for _, candidate := range goErrorAssertionCandidates(subjectType) {
			out.WriteString(" || _ts_val.and_then(|__v| __v.downcast_ref::<")
			out.WriteString(candidate.rustType)
			out.WriteString(">()).is_some()")
		}
		if programAnyRuntimeOpsEnabled() {
			NeedGoAnyRuntimeOps()
			out.WriteString(" || _ts_val.is_some_and(go_registered_any_is_error)")
		}
		return
	}
	if handleType, ok := typeSwitchCaseAnyHandleRustType(typeInfo, typeExpr, subjectType); ok {
		out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
		out.WriteString(handleType)
		out.WriteString(">()).is_some()")
		return
	}
	if writeTypeSwitchLocalInterfaceCaseCondition(out, typeInfo, typeExpr, subjectType, canRewrapInterfaceSubject) {
		return
	}
	if rustType == "" {
		out.WriteString("unimplemented!(\"type info required for type switch case\")")
		return
	}
	out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
	out.WriteString(rustType)
	out.WriteString(">()).is_some()")
}

func writeTypeSwitchOriginalBinding(out *strings.Builder, varName string, expr ast.Expr, originalBindingSubject string, isRangeVar bool, isStdlibRangeRef bool, mutable bool) {
	out.WriteString("        let ")
	if mutable {
		out.WriteString("mut ")
	}
	out.WriteString(varName)
	out.WriteString(" = ")
	if isRangeVar {
		out.WriteString("_ts_val.unwrap();\n")
		if vt := GetVarTable(); vt != nil {
			rustType := "&dyn Any"
			if NeedsConcurrentWrapper() {
				rustType = "&(dyn Any + Send + Sync)"
			}
			vt.Register(varName, &VarInfo{
				WrapLevel: WrapNone,
				RustType:  rustType,
				Source:    SourceLocal,
				IsRef:     true,
			})
		}
		return
	}
	if isStdlibRangeRef {
		WriteWrapperPrefix(out)
		out.WriteString("(*_ts_val.unwrap()).clone()")
		WriteWrapperSuffix(out)
		out.WriteString(";\n")
		return
	}
	bindingIsBare := isExpressionResultBare(expr)
	if originalBindingSubject != "" {
		out.WriteString(originalBindingSubject)
	} else {
		TranspileExpressionContext(out, expr, LValue)
	}
	out.WriteString(".clone();\n")
	if vt := GetVarTable(); vt != nil {
		wrapLevel := WrapFull
		if bindingIsBare {
			wrapLevel = WrapNone
		}
		info := &VarInfo{
			WrapLevel: wrapLevel,
			Source:    SourceLocal,
		}
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if typ := typeInfo.GetType(expr); typ != nil {
				info.RustType = goTypesTypeToRust(typ)
			}
		}
		vt.Register(varName, info)
	}
}

func typeSwitchBareInterfaceSubjectNeedsClone(expr ast.Expr) bool {
	if !isExpressionResultBare(expr) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Interface)
	return ok
}

func isLocalInterfaceRefIdent(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	vt := GetVarTable()
	if vt == nil {
		return false
	}
	info := vt.Lookup(ident.Name)
	return info != nil && info.IsRef
}

func isTranspiledInterfaceExpr(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	_, ok := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(expr))
	return ok
}

func pushTypeSwitchCaseVarScope(varName string, isTypedSingleCase bool) func() {
	if varName == "" {
		return func() {}
	}
	vt := GetVarTable()
	if vt == nil {
		return func() {}
	}
	vt.PushScope()
	if isTypedSingleCase {
		vt.Register(varName, &VarInfo{
			WrapLevel: WrapFull,
			Source:    SourceLocal,
		})
	}
	return vt.PopScope
}

func typeSwitchCaseVarAssigned(caseClause *ast.CaseClause, varName string) bool {
	if caseClause == nil || varName == "" {
		return false
	}
	return blockIdentAssigned(&ast.BlockStmt{List: caseClause.Body}, varName)
}

func isUnsafePointerDerefAssignmentTarget(expr ast.Expr) bool {
	star, ok := expr.(*ast.StarExpr)
	if !ok {
		return false
	}
	target := star.X
	if paren, ok := target.(*ast.ParenExpr); ok {
		target = paren.X
	}
	call, ok := target.(*ast.CallExpr)
	if !ok || len(call.Args) != 1 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isUnsafePointerLikeType(typeInfo.GetType(call.Args[0])) {
		return false
	}
	if typ := typeInfo.GetType(call); typ != nil {
		if _, ok := types.Unalias(typ).Underlying().(*types.Pointer); ok {
			return true
		}
	}
	_, ok = pointerTypeConversionTarget(call.Fun)
	return ok
}

func writeCurrentReceiverDerefAssignment(out *strings.Builder, star *ast.StarExpr, rhs ast.Expr) bool {
	ident, ok := star.X.(*ast.Ident)
	if !ok || !isCurrentReceiverIdent(ident) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		lhsType := typeInfo.GetType(star)
		rhsType := typeInfo.GetType(rhs)
		if lhsType != nil && rhsType != nil && !types.AssignableTo(rhsType, lhsType) {
			return false
		}
	}
	// For named-slice receivers (e.g. type ErrorList []*Error), the RHS
	// may emit either a wrapped Arc<Mutex<Option<ErrorList>>> (when going
	// through the generic wrap path) or a bare ErrorList constructor. *self
	// expects the bare ErrorList, so unwrap only when the RHS produces a
	// wrapped value.
	if isNamedSliceExpression(star) {
		rhsWrapped := false
		if call, isCall := rhs.(*ast.CallExpr); isCall {
			if isBuiltinCallNamed(call, "append") {
				rhsWrapped = true
			}
		}
		if rhsWrapped {
			out.WriteString("{ let new_val = ")
			TranspileExpression(out, rhs)
			out.WriteString("; *self = new_val")
			WriteBorrowMethod(out, true)
			out.WriteString(".take().unwrap_or_default(); }")
			return true
		}
		out.WriteString("{ let new_val = ")
		TranspileExpression(out, rhs)
		out.WriteString("; *self = new_val; }")
		return true
	}
	if isNamedMapExpression(star) {
		var targetNamed *types.Named
		rhsIsNamedMap := false
		if typeInfo != nil {
			lhsType := typeInfo.GetType(star)
			rhsType := typeInfo.GetType(rhs)
			targetNamed, _, _ = namedMapTypeFromType(lhsType)
			_, _, rhsIsNamedMap = namedMapTypeFromType(rhsType)
		}
		wrapUnnamedMap := targetNamed != nil && !rhsIsNamedMap
		out.WriteString("{ let new_val = ")
		TranspileExpression(out, rhs)
		out.WriteString("; *self = ")
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(rhs) {
			if wrapUnnamedMap {
				out.WriteString(goTypesNamedTypeToRust(targetNamed))
				out.WriteString("(")
				out.WriteString("new_val")
				out.WriteString(")")
				out.WriteString("; }")
				return true
			}
			out.WriteString("new_val")
			WriteBorrowMethod(out, true)
			out.WriteString(".take().unwrap_or_default()")
			out.WriteString("; }")
		} else {
			if wrapUnnamedMap {
				out.WriteString(goTypesNamedTypeToRust(targetNamed))
				out.WriteString("(")
			}
			out.WriteString("new_val")
			if wrapUnnamedMap {
				out.WriteString(")")
			}
			out.WriteString("; }")
		}
		return true
	}
	out.WriteString("{ let new_val = ")
	TranspileExpression(out, rhs)
	if currentReceiverScalarTypeDefinition() {
		// The receiver's cell may be another pointer's, as the cell of the
		// *int that (*intValue)(p) converts: write the value into it.
		out.WriteString("; let __new_value = new_val.0")
		WriteBorrowMethod(out, true)
		out.WriteString(".take(); *self.0")
		WriteBorrowMethod(out, true)
		out.WriteString(" = __new_value; }")
		return true
	}
	out.WriteString("; *self = new_val; }")
	return true
}

func isUnlabeledBreakStmt(stmt ast.Stmt) bool {
	branch, ok := stmt.(*ast.BranchStmt)
	return ok && branch.Tok == token.BREAK && branch.Label == nil
}

func pushBreakTarget(label string) func() {
	breakTargetStack = append(breakTargetStack, label)
	return func() {
		breakTargetStack = breakTargetStack[:len(breakTargetStack)-1]
	}
}

func currentBreakTarget() string {
	if len(breakTargetStack) == 0 {
		return ""
	}
	return breakTargetStack[len(breakTargetStack)-1]
}

// continueTargetStack holds the Rust label an unlabeled continue names, and
// the loop depth it belongs to. A for loop whose body is lowered with goto
// blocks and loops gets a label of its own: in Rust an unlabeled continue
// inside a labeled block does not compile, and inside a goto loop it would
// continue that loop instead of the for loop.
type continueTarget struct {
	label string
	depth int
}

var continueTargetStack []continueTarget

func pushContinueTarget(label string) func() {
	continueTargetStack = append(continueTargetStack, continueTarget{label: label, depth: currentLoopDepth})
	return func() {
		continueTargetStack = continueTargetStack[:len(continueTargetStack)-1]
	}
}

// currentContinueTarget returns the label an unlabeled continue at the
// current loop depth names, or "".
func currentContinueTarget() string {
	if len(continueTargetStack) == 0 {
		return ""
	}
	top := continueTargetStack[len(continueTargetStack)-1]
	if top.depth != currentLoopDepth {
		return ""
	}
	return top.label
}

var gotoLoopLabelCount int

func pushForPost(post ast.Stmt) func() {
	hasPost := post != nil
	if hasPost {
		forPostStack = append(forPostStack, post)
	}
	forPostHasPostStack = append(forPostHasPostStack, hasPost)
	return func() {
		if forPostHasPostStack[len(forPostHasPostStack)-1] {
			forPostStack = forPostStack[:len(forPostStack)-1]
		}
		forPostHasPostStack = forPostHasPostStack[:len(forPostHasPostStack)-1]
	}
}

func currentForPost() ast.Stmt {
	if len(forPostHasPostStack) == 0 || !forPostHasPostStack[len(forPostHasPostStack)-1] {
		return nil
	}
	return forPostStack[len(forPostStack)-1]
}

func nextSwitchBreakLabel() string {
	switchBreakLabelCounter++
	return fmt.Sprintf("__go_switch_%d", switchBreakLabelCounter)
}

func switchTagNamedInterface(tag ast.Expr) (types.Type, string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, "", false
	}
	tagType := typeInfo.GetType(tag)
	if tagType == nil {
		return nil, "", false
	}
	ifaceName, _, ok := namedInterfaceForTraitEquality(tagType)
	if !ok || ifaceName == "" {
		return nil, "", false
	}
	return tagType, ifaceName, true
}

func switchTagEmptyInterface(tag ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return isEmptyInterfaceType(typeInfo.GetType(tag))
}

func switchTagGoError(tag ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return isGoErrorType(typeInfo.GetType(tag))
}

func writeSwitchInterfaceTagHandle(out *strings.Builder, tag ast.Expr, tagType types.Type) {
	var value strings.Builder
	if !writeLocalInterfaceReferenceCallArgument(&value, tag, tagType) {
		out.WriteString(`unimplemented!("type info required for interface switch tag")`)
		return
	}
	out.WriteString(value.String())
}

func writeSwitchEmptyInterfaceTagHandle(out *strings.Builder, tag ast.Expr) {
	if !writeEmptyInterfaceHandleShare(out, tag) {
		out.WriteString(`unimplemented!("type info required for empty-interface switch tag")`)
	}
}

func writeSwitchGoErrorTagHandle(out *strings.Builder, tag ast.Expr) {
	if !writeGoErrorHandleValue(out, tag) {
		out.WriteString(`unimplemented!("type info required for error switch tag")`)
	}
}

// The binding that holds the tag is in scope for every case and its body.
func switchTagBindingName(body *ast.BlockStmt) string {
	used := make(map[string]bool)
	ast.Inspect(body, func(node ast.Node) bool {
		if id, ok := node.(*ast.Ident); ok {
			used[RustIdentForUse(id)] = true
			used[rustFunctionNameForUse(id.Name)] = true
		}
		return true
	})
	name := "_switch_val"
	for used[name] || isPackageTupleStructTypeName(name) || isPackageGlobalName(name) {
		name += "_"
	}
	return name
}

func writeSwitchPointerTagHandle(out *strings.Builder, tag ast.Expr) {
	writePointerHandleExpression(out, tag)
}

func writeSwitchPointerCaseComparison(out *strings.Builder, switchVal string, caseExpr ast.Expr) {
	if ident, ok := caseExpr.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("(*")
		out.WriteString(switchVal)
		WriteBorrowMethod(out, false)
		out.WriteString(").is_none()")
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(caseExpr) {
		out.WriteString(`unimplemented!("type info required for pointer switch case")`)
		return
	}
	trackWrapperImports()
	out.WriteString("{ let __case = ")
	writePointerHandleExpression(out, caseExpr)
	// Test identity first. For distinct handles, finish each nil check before
	// borrowing the other handle so concurrent switches cannot nest locks.
	out.WriteString("; let __same = ")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::ptr_eq(&")
	out.WriteString(switchVal)
	out.WriteString(", &__case); if __same { true } else { let __tag_nil = ")
	out.WriteString(switchVal)
	WriteBorrowMethod(out, false)
	out.WriteString(".is_none(); let __both_nil = __tag_nil && __case")
	WriteBorrowMethod(out, false)
	out.WriteString(".is_none(); __both_nil } }")
}

func switchGoPtrElemRustType(tag ast.Expr) (string, bool) {
	info, ok := switchGoPtrInfo(tag)
	return info.elemRustType, ok
}

func switchGoPtrInfo(tag ast.Expr) (goPtrResultInfo, bool) {
	ident, ok := unwrapParens(tag).(*ast.Ident)
	if !ok {
		return goPtrResultInfo{}, false
	}
	if info, ok := goPtrCandidateForDecl(ident); ok && info.elemRustType != "" {
		return info, true
	}
	elemRustType, ok := goPtrVarElemRustType(ident.Name)
	if !ok {
		return goPtrResultInfo{}, false
	}
	info := goPtrResultInfo{elemRustType: elemRustType}
	if varInfo, ok := goPtrVarInfo(ident.Name); ok && varInfo.GoType != nil {
		if ptr, ok := types.Unalias(varInfo.GoType).Underlying().(*types.Pointer); ok {
			info.elemType = ptr.Elem()
		}
	}
	return info, true
}

func writeSwitchGoPtrCaseComparison(out *strings.Builder, switchVal string, caseExpr ast.Expr, elemRustType string) {
	writeSwitchGoPtrCaseComparisonForInfo(out, switchVal, caseExpr, goPtrResultInfo{elemRustType: elemRustType})
}

func writeSwitchGoPtrCaseComparisonForInfo(out *strings.Builder, switchVal string, caseExpr ast.Expr, info goPtrResultInfo) {
	if ident, ok := caseExpr.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString(switchVal)
		out.WriteString(".is_nil()")
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(caseExpr) {
		out.WriteString(`unimplemented!("type info required for GoPtr switch case")`)
		return
	}
	out.WriteString("{ let __case = ")
	if !writeGoPtrCallArgumentForInfo(out, caseExpr, info) {
		out.WriteString(`unimplemented!("GoPtr switch case requires pointer-compatible value")`)
	} else {
		out.WriteString("; GoPtr::ptr_eq(&")
		out.WriteString(switchVal)
		out.WriteString(", &__case)")
	}
	out.WriteString(" }")
}

type switchInterfaceCaseKind int

const (
	switchInterfaceCaseWrappedInterface switchInterfaceCaseKind = iota
	switchInterfaceCaseConcretePointer
)

func switchInterfaceCaseKindFor(caseExpr ast.Expr, tagType types.Type) (switchInterfaceCaseKind, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || tagType == nil {
		return 0, false
	}
	caseType := typeInfo.GetType(caseExpr)
	if caseType == nil {
		return 0, false
	}
	if _, ok := types.Unalias(caseType).Underlying().(*types.Interface); ok && types.AssignableTo(caseType, tagType) {
		return switchInterfaceCaseWrappedInterface, true
	}
	if concreteAssignableToInterface(caseType, tagType) {
		return switchInterfaceCaseConcretePointer, true
	}
	if types.AssignableTo(caseType, tagType) {
		return switchInterfaceCaseWrappedInterface, true
	}
	return 0, false
}

func writeSwitchInterfaceCaseComparison(out *strings.Builder, switchVal string, caseExpr ast.Expr, tagType types.Type, ifaceName string) {
	if ident, ok := caseExpr.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("(*")
		out.WriteString(switchVal)
		WriteBorrowMethod(out, false)
		out.WriteString(").is_none()")
		return
	}

	caseKind, ok := switchInterfaceCaseKindFor(caseExpr, tagType)
	if !ok {
		out.WriteString(`unimplemented!("type info required for interface switch case")`)
		return
	}

	var right strings.Builder
	if caseKind == switchInterfaceCaseConcretePointer {
		writePointerConcreteInterfaceHandle(&right, caseExpr)
	} else {
		if !writeLocalInterfaceReferenceCallArgument(&right, caseExpr, tagType) {
			out.WriteString(`unimplemented!("type info required for interface switch case")`)
			return
		}
	}

	out.WriteString("{ let __left_holder = ")
	out.WriteString(switchVal)
	out.WriteString(".clone(); let __left_guard = __left_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __right_holder = ")
	out.WriteString(right.String())
	out.WriteString("; let __right_guard = __right_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => ")
	if caseKind == switchInterfaceCaseConcretePointer {
		out.WriteString("{ let __right_trait: ")
		out.WriteString(rustLocalInterfaceParamBare(ifaceName))
		out.WriteString(" = __right; __left.as_ref().__go_eq_")
		out.WriteString(traitMethodSuffix(ifaceName))
		out.WriteString("(__right_trait) }")
	} else {
		out.WriteString("__left.as_ref().__go_eq_")
		out.WriteString(traitMethodSuffix(ifaceName))
		out.WriteString("(__right.as_ref())")
	}
	out.WriteString(", (None, None) => true, _ => false }; __eq }")
}

func writeSwitchEmptyInterfaceCaseComparison(out *strings.Builder, switchVal string, caseExpr ast.Expr) {
	if ident, ok := caseExpr.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("(*")
		out.WriteString(switchVal)
		WriteBorrowMethod(out, false)
		out.WriteString(").is_none()")
		return
	}

	NeedAnyEq()
	out.WriteString("{ let __right_holder = ")
	writeEmptyInterfaceCallArgumentValue(out, caseExpr)
	out.WriteString("; go_any_eq(&")
	out.WriteString(switchVal)
	out.WriteString(", &__right_holder) }")
}

func writeSwitchGoErrorCaseComparison(out *strings.Builder, switchVal string, caseExpr ast.Expr) {
	if ident, ok := caseExpr.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("(*")
		out.WriteString(switchVal)
		WriteBorrowMethod(out, false)
		out.WriteString(").is_none()")
		return
	}

	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString(`unimplemented!("type info required for error switch case")`)
		return
	}
	caseType := typeInfo.GetType(caseExpr)
	if isConcreteGoErrorValue(caseType) && isConstantExpression(caseExpr) {
		if writeSwitchGoErrorConcreteCaseComparison(out, switchVal, caseExpr, caseType) {
			return
		}
	}
	if !isGoErrorType(caseType) {
		out.WriteString(`unimplemented!("type info required for error switch case")`)
		return
	}

	out.WriteString("{ let __left_holder = ")
	out.WriteString(switchVal)
	out.WriteString(".clone(); let __left_guard = __left_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __right_holder = ")
	if !writeGoErrorHandleValue(out, caseExpr) {
		out.WriteString(`unimplemented!("type info required for error switch case")`)
		out.WriteString("; ")
	} else {
		out.WriteString("; ")
	}
	out.WriteString("let __right_guard = __right_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => std::ptr::addr_eq(&**__left, &**__right), (None, None) => true, _ => false } }")
}

func writeSwitchGoErrorConcreteCaseComparison(out *strings.Builder, switchVal string, caseExpr ast.Expr, caseType types.Type) bool {
	named, ok := types.Unalias(caseType).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok || !isIntegerBasicKind(basic.Kind()) {
		return false
	}
	rustType, ok := rustCastTypeForDefinedUnderlying(basic.Name())
	if !ok {
		return false
	}
	out.WriteString("{ let __err_holder = ")
	out.WriteString(switchVal)
	out.WriteString(".clone(); let __err_guard = __err_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<")
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString(">()).map(|__e| *__e.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap() == (")
	if !writeConstExpressionWithRustIntegerOperands(out, caseExpr, rustType) {
		TranspileConstExpr(out, caseExpr, 0)
	}
	out.WriteString(" as ")
	out.WriteString(rustType)
	out.WriteString(")).unwrap_or(false) }")
	return true
}

func stmtContainsBreakForCurrentSwitch(stmt ast.Stmt) bool {
	switch s := stmt.(type) {
	case *ast.BranchStmt:
		return s.Tok == token.BREAK && s.Label == nil
	case *ast.BlockStmt:
		return stmtListContainsBreakForCurrentSwitch(s.List)
	case *ast.IfStmt:
		if stmtListContainsBreakForCurrentSwitch(s.Body.List) {
			return true
		}
		if s.Else != nil {
			return stmtContainsBreakForCurrentSwitch(s.Else)
		}
		return false
	case *ast.LabeledStmt:
		return stmtContainsBreakForCurrentSwitch(s.Stmt)
	case *ast.ForStmt, *ast.RangeStmt, *ast.SwitchStmt, *ast.TypeSwitchStmt:
		return false
	default:
		return false
	}
}

func stmtContainsBreakForCurrentLoop(stmt ast.Stmt) bool {
	switch s := stmt.(type) {
	case *ast.BranchStmt:
		return s.Tok == token.BREAK && s.Label == nil
	case *ast.BlockStmt:
		return stmtListContainsBreakForCurrentLoop(s.List)
	case *ast.IfStmt:
		if stmtListContainsBreakForCurrentLoop(s.Body.List) {
			return true
		}
		if s.Else != nil {
			return stmtContainsBreakForCurrentLoop(s.Else)
		}
		return false
	case *ast.LabeledStmt:
		return stmtContainsBreakForCurrentLoop(s.Stmt)
	case *ast.ForStmt, *ast.RangeStmt, *ast.SwitchStmt, *ast.TypeSwitchStmt, *ast.SelectStmt:
		return false
	default:
		return false
	}
}

func stmtListContainsBreakForCurrentLoop(stmts []ast.Stmt) bool {
	for _, stmt := range stmts {
		if stmtContainsBreakForCurrentLoop(stmt) {
			return true
		}
	}
	return false
}

func forStmtTerminates(stmt *ast.ForStmt) bool {
	return stmt != nil && stmt.Cond == nil && stmt.Body != nil && !stmtListContainsBreakForCurrentLoop(stmt.Body.List)
}

func stmtListContainsBreakForCurrentSwitch(stmts []ast.Stmt) bool {
	for _, stmt := range stmts {
		if stmtContainsBreakForCurrentSwitch(stmt) {
			return true
		}
	}
	return false
}

func switchNeedsSyntheticBreakTarget(body *ast.BlockStmt) bool {
	for _, stmt := range body.List {
		caseClause, ok := stmt.(*ast.CaseClause)
		if !ok {
			continue
		}
		for _, bodyStmt := range caseClause.Body {
			if isUnlabeledBreakStmt(bodyStmt) {
				continue
			}
			if stmtContainsBreakForCurrentSwitch(bodyStmt) {
				return true
			}
		}
	}
	return false
}

func stmtTerminates(stmt ast.Stmt) bool {
	switch s := stmt.(type) {
	case *ast.ReturnStmt:
		return true
	case *ast.ExprStmt:
		call, ok := s.X.(*ast.CallExpr)
		return ok && isBuiltinCallNamed(call, "panic")
	case *ast.BlockStmt:
		return stmtListTerminates(s.List)
	case *ast.SwitchStmt:
		return switchStmtTerminates(s)
	case *ast.TypeSwitchStmt:
		return typeSwitchStmtTerminates(s)
	case *ast.SelectStmt:
		return selectStmtTerminates(s)
	case *ast.ForStmt:
		return forStmtTerminates(s)
	case *ast.LabeledStmt:
		return stmtTerminates(s.Stmt)
	default:
		return false
	}
}

func stmtListTerminates(stmts []ast.Stmt) bool {
	for _, stmt := range stmts {
		if isUnlabeledBreakStmt(stmt) {
			return false
		}
		if stmtTerminates(stmt) {
			return true
		}
	}
	return false
}

func lastNonEmptyStmt(stmts []ast.Stmt) ast.Stmt {
	for i := len(stmts) - 1; i >= 0; i-- {
		if _, ok := stmts[i].(*ast.EmptyStmt); ok {
			continue
		}
		return stmts[i]
	}
	return nil
}

func stmtListFallsThrough(stmts []ast.Stmt) bool {
	for i := len(stmts) - 1; i >= 0; i-- {
		if _, ok := stmts[i].(*ast.EmptyStmt); ok {
			continue
		}
		if branch, ok := stmts[i].(*ast.BranchStmt); ok {
			return branch.Tok == token.FALLTHROUGH
		}
		return false
	}
	return false
}

func switchStmtTerminates(s *ast.SwitchStmt) bool {
	hasDefault := false
	nextTerminates := false
	for i := len(s.Body.List) - 1; i >= 0; i-- {
		stmt := s.Body.List[i]
		clause, ok := stmt.(*ast.CaseClause)
		if !ok {
			return false
		}
		if len(clause.List) == 0 {
			hasDefault = true
		}
		terminates := stmtListTerminates(clause.Body)
		if stmtListFallsThrough(clause.Body) {
			terminates = nextTerminates
		}
		if !terminates {
			return false
		}
		nextTerminates = true
	}
	return hasDefault
}

func typeSwitchStmtTerminates(s *ast.TypeSwitchStmt) bool {
	if s == nil || s.Body == nil {
		return false
	}
	hasDefault := false
	for _, stmt := range s.Body.List {
		clause, ok := stmt.(*ast.CaseClause)
		if !ok {
			return false
		}
		if len(clause.List) == 0 {
			hasDefault = true
		}
		if !stmtListTerminates(clause.Body) {
			return false
		}
	}
	return hasDefault
}

func selectStmtTerminates(s *ast.SelectStmt) bool {
	if s == nil || s.Body == nil {
		return false
	}
	if len(s.Body.List) == 0 {
		return true
	}
	for _, stmt := range s.Body.List {
		clause, ok := stmt.(*ast.CommClause)
		if !ok || !stmtListTerminates(clause.Body) {
			return false
		}
	}
	return true
}

func stmtNeedsSeparatorBeforeFollowingStatement(stmt ast.Stmt) bool {
	switch s := stmt.(type) {
	case *ast.TypeSwitchStmt:
		return typeSwitchStmtTerminates(s)
	case *ast.BranchStmt:
		return s.Tok == token.BREAK || s.Tok == token.CONTINUE || s.Tok == token.GOTO
	default:
		return false
	}
}

func writeStatementSeparatorBeforeFollowingStatement(out *strings.Builder, stmt ast.Stmt, hasFollowing bool) {
	if hasFollowing && stmtNeedsSeparatorBeforeFollowingStatement(stmt) {
		out.WriteString(";")
	}
}

func writeBareValueForWrappedSlot(out *strings.Builder, expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(sel)
	if typ == nil {
		return false
	}
	basic, ok := types.Unalias(typ).Underlying().(*types.Basic)
	if !ok {
		return false
	}
	switch basic.Kind() {
	case types.Bool, types.String,
		types.Int, types.Int8, types.Int16, types.Int32, types.Int64,
		types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64,
		types.Uintptr, types.Float32, types.Float64:
	default:
		return false
	}
	if ident, ok := sel.X.(*ast.Ident); ok && isGoPtrVar(ident.Name) {
		fieldInfo := selectorFieldAccessInfo(sel)
		if fieldInfo.Found && writeGoPtrLocalFieldSelector(out, ident, fieldInfo, sel, RValue) {
			return true
		}
	}
	if !selectorRValueReturnsWrappedHandle(sel) {
		TranspileExpression(out, sel)
		return true
	}
	out.WriteString("{ let __v = ")
	TranspileExpression(out, sel)
	out.WriteString("; let __owned = (*__v")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone(); __owned }")
	return true
}

func writeCurrentReceiverAssignmentValue(out *strings.Builder, expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	return writeCurrentReceiverClone(out, ident)
}

func assignmentRHSReturnsWrappedValue(rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.ReturnsWrappedValue(rhs) {
		return false
	}
	if call, ok := rhs.(*ast.CallExpr); ok {
		if isBareBuiltinReturn(call) {
			return false
		}
		if typeInfo.IsTypeConversion(call) && !typeConversionEmitsWrappedValue(call) {
			return false
		}
	}
	return true
}

func writeCurrentReceiverNamedTypeAssignmentValue(out *strings.Builder, rhs ast.Expr) bool {
	if currentReceiverObject == nil {
		return false
	}
	named, ok := types.Unalias(currentReceiverObject.Type()).(*types.Named)
	if !ok || named.Obj() == nil {
		return false
	}
	if isNamedIntegerType(named) {
		// "m = -m" in a method of "type Money int64": the receiver is a
		// Money, and the new value gets a cell of its own, so the caller's
		// value is not changed through the copy.
		typeInfo := GetTypeInfo()
		if typeInfo == nil {
			return false
		}
		rhsType := typeInfo.GetType(rhs)
		if rhsType == nil || !types.AssignableTo(rhsType, named) {
			return false
		}
		return writeNamedIntegerValueForExpected(out, rhs, named)
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok || basic.Kind() != types.String {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("unimplemented!(\"type info required for named receiver assignment\")")
		return true
	}
	rhsType := typeInfo.GetType(rhs)
	if rhsType == nil || !types.AssignableTo(rhsType, named) {
		return false
	}
	rustTypeName := goTypesNamedTypeToRust(named)
	if rustTypeName == "" {
		rustTypeName = RustTypeNameForUse(named.Obj().Name())
	}
	writeStringTypeDefinitionConstructor(out, rustTypeName, rhs)
	return true
}

func writeCurrentReceiverLocalAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	ident, ok := lhs.(*ast.Ident)
	if !ok || !isCurrentReceiverIdent(ident) || currentReceiverRustAlias == "" {
		return false
	}
	if currentReceiverRustAliasIsGoPtr {
		out.WriteString("{ let new_val = ")
		if !writeGoPtrCallArgumentForInfo(out, rhs, currentReceiverRustAliasGoPtrInfo) {
			out.WriteString(`unimplemented!("GoPtr receiver assignment requires compatible pointer value")`)
		}
		out.WriteString("; ")
		out.WriteString(currentReceiverRustAlias)
		out.WriteString(" = new_val; }")
		return true
	}
	if currentReceiverRustAliasIsPointerHandle {
		out.WriteString("{ let new_val = ")
		if rhsIdent, ok := rhs.(*ast.Ident); ok && rhsIdent.Name == "nil" {
			WriteWrappedNone(out)
		} else if assignmentRHSReturnsWrappedValue(rhs) {
			TranspileExpression(out, rhs)
		} else if !writeOwnedExpressionValue(out, rhs) {
			TranspileExpression(out, rhs)
		}
		out.WriteString("; ")
		out.WriteString(currentReceiverRustAlias)
		out.WriteString(" = new_val; }")
		return true
	}
	out.WriteString("{ let new_val = ")
	if writeCurrentReceiverNamedTypeAssignmentValue(out, rhs) {
		out.WriteString("; ")
		out.WriteString(currentReceiverRustAlias)
		out.WriteString(" = new_val; }")
		return true
	}
	if assignmentRHSReturnsWrappedValue(rhs) {
		// The receiver copy shares the new pointee's field cells; taking
		// the pointee out of its cell would empty it, so a walk such as
		// `for n.next != nil { n = n.next }` emptied the list (DEV-165).
		TranspileExpression(out, rhs)
		out.WriteString("; let __moved_val = { let __guard = new_val")
		WriteBorrowMethod(out, false)
		out.WriteString("; __guard.as_ref().unwrap().clone() }; ")
		out.WriteString(currentReceiverRustAlias)
		out.WriteString(" = __moved_val; }")
		return true
	}
	if writeCurrentReceiverAssignmentValue(out, rhs) {
		// Self-assigning the receiver copies the current local receiver value.
	} else if !writeOwnedExpressionValue(out, rhs) {
		TranspileExpression(out, rhs)
	}
	out.WriteString("; ")
	out.WriteString(currentReceiverRustAlias)
	out.WriteString(" = new_val; }")
	return true
}

// writeFunctionAssertionHandleAssignment writes "f = x.(func())", where x is
// an interface{}. A function value is boxed as its handle, so the variable
// takes that handle, as "f = g" takes g's.
func writeFunctionAssertionHandleAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	e, ok := unwrapParens(rhs).(*ast.TypeAssertExpr)
	if !ok {
		return false
	}
	if _, _, _, ok := functionAssertionAnyHandleTarget(e); !ok {
		return false
	}
	var value strings.Builder
	if !writeAnyHandleAssertionHandle(&value, e) {
		return false
	}
	out.WriteString("{ let new_val = ")
	out.WriteString(value.String())
	out.WriteString("; ")
	writePointerHandleAssignmentTargetFromValueName(out, lhs, "new_val")
	out.WriteString(" }")
	return true
}

func writeFunctionTypedIdentFieldAssignment(out *strings.Builder, lhs ast.Expr, rhsIdent *ast.Ident) bool {
	if rhsIdent.Name == "_" || rhsIdent.Name == "nil" || rhsIdent.Name == "true" || rhsIdent.Name == "false" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	rhsType := typeInfo.GetType(rhsIdent)
	if rhsType == nil {
		return false
	}
	if _, ok := rhsType.Underlying().(*types.Signature); !ok {
		return false
	}
	rhsName := RustIdentForUse(rhsIdent)
	if currentCaptureRenames != nil {
		if renamed, exists := currentCaptureRenames[rhsIdent.Name]; exists {
			rhsName = RustLocalIdent(renamed)
		}
	}
	out.WriteString("{ let new_val = ")
	out.WriteString(rhsName)
	out.WriteString(".clone(); ")
	writePointerHandleAssignmentTargetFromValueName(out, lhs, "new_val")
	out.WriteString(" }")
	return true
}

func writeWrappedValueCopyFromIdent(out *strings.Builder, ident *ast.Ident) bool {
	if ident.Name == "_" || ident.Name == "nil" || ident.Name == "true" || ident.Name == "false" {
		return false
	}
	if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
		return false
	}
	if _, isLocalConst := localConstants[ident.Name]; isLocalConst {
		return false
	}
	if isConstIdent(ident) {
		return false
	}
	if isVarBare(ident.Name) || rhsIsPointerType(ident) {
		return false
	}

	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(ident)
	if typ == nil {
		return false
	}

	if !goTypeIsCopiedByValue(typ) {
		return false
	}
	varName := RustIdentForUse(ident)
	if currentCaptureRenames != nil {
		if renamed, exists := currentCaptureRenames[ident.Name]; exists {
			varName = RustLocalIdent(renamed)
		}
	}
	out.WriteString("{ let __owned = ")
	out.WriteString(varName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	out.WriteString(goValueCopyCallForType(typ))
	out.WriteString("; ")
	WriteWrapperPrefix(out)
	out.WriteString("__owned")
	WriteWrapperSuffix(out)
	out.WriteString(" }")
	return true
}

// varSpecHasTypeOfValue reports whether the i-th name of a var spec has the
// type of its value: the spec names no type, or names the value's own type.
// "var n Numbers = x.([]int)" does not: n is a Numbers built from the
// []int, not the []int itself.
func varSpecHasTypeOfValue(valueSpec *ast.ValueSpec, i int) bool {
	if valueSpec.Type == nil {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || i >= len(valueSpec.Names) || i >= len(valueSpec.Values) {
		return false
	}
	obj := typeInfo.info.Defs[valueSpec.Names[i]]
	valueType := typeInfo.GetType(valueSpec.Values[i])
	return obj != nil && valueType != nil && types.Identical(obj.Type(), valueType)
}

// writeWrappedCollectionCopyFromIdent writes the initializer of a new
// variable that is a copy of a local slice or map variable: "t := s",
// "var t = s". Writing the read of s there moved the Vec or map out of a
// borrow, which rustc rejects, so "t := s" did not compile.
//
// A map variable holds a handle on the map. Passing it to a function and
// "d = m" share that handle, so the new variable shares it too, and
// "d := m; d[k] = v" changes m, as in Go.
//
// A slice is one Vec behind a handle with no header of its own (DEV-36):
// sharing the handle would make "next := prev; next = append(next, x)" grow
// prev, and "cur := queue; queue = nil" empty cur. The new variable gets a
// clone of the elements instead, which keeps both of those right. A write
// through one of them, "t[0] = x", does not reach the other.
func writeWrappedCollectionCopyFromIdent(out *strings.Builder, ident *ast.Ident, destination types.Type) bool {
	if !localCollectionIdentNeedsClone(ident) {
		return false
	}
	typ := GetTypeInfo().GetType(ident)
	if destination != nil && !types.Identical(destination, typ) {
		return false
	}
	if _, isMap := types.Unalias(typ).Underlying().(*types.Map); isMap {
		name := RustIdentForUse(ident)
		if renamed, exists := captureRenameForIdent(ident); exists {
			name = RustLocalIdent(renamed)
		}
		out.WriteString(name)
		out.WriteString(".clone()")
		return true
	}
	if _, isSlice := types.Unalias(typ).Underlying().(*types.Slice); isSlice && !isCurrentReceiverIdent(ident) {
		// The new cell holds a copy of s's Option, so a nil s gives a nil
		// t instead of unwrapping a None (DEV-182).
		name := RustIdentForUse(ident)
		if renamed, exists := captureRenameForIdent(ident); exists {
			name = RustLocalIdent(renamed)
		}
		out.WriteString(GetOuterWrapperType() + "::new(" + GetInnerWrapperType() + "::new(")
		out.WriteString(name)
		WriteBorrowMethod(out, false)
		out.WriteString(".clone()))")
		return true
	}
	WriteWrapperPrefix(out)
	writeIdentValueClone(out, ident)
	WriteWrapperSuffix(out)
	return true
}

// writeChannelCopyFromIdent writes the initializer of a new variable that is
// a copy of a channel variable: "ch2 := ch". A channel is a handle held
// bare, so the copy is a clone of the handle. Writing the variable itself
// moved the channel into the new one, and the next use of ch did not compile.
func writeChannelCopyFromIdent(out *strings.Builder, ident *ast.Ident, destination types.Type) bool {
	if ident == nil || ident.Name == "_" || ident.Name == "nil" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(ident)
	if typ == nil {
		return false
	}
	if _, ok := types.Unalias(typ).Underlying().(*types.Chan); !ok {
		return false
	}
	if destination != nil && !types.Identical(destination, typ) {
		return false
	}
	TranspileExpression(out, ident)
	out.WriteString(".clone()")
	return true
}

// localCollectionIdentNeedsClone reports whether ident is a local slice or
// map variable held in its own cell. Its read is a place, so storing it
// anywhere else takes a clone.
func localCollectionIdentNeedsClone(ident *ast.Ident) bool {
	if ident == nil || ident.Name == "_" || ident.Name == "nil" {
		return false
	}
	if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
		return false
	}
	if isVarBare(ident.Name) || isPackageGlobalIdent(ident) || isCurrentReceiverIdent(ident) {
		return false
	}
	if isSliceElemPtrVar(ident.Name) || isArrayElemPtrVar(ident.Name) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(ident)
	if typ == nil {
		return false
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Slice, *types.Map:
		return true
	}
	return false
}

// declaredVarType is the type of the i-th name a var spec declares, or nil
// when the spec has no type and the name takes the type of its value.
func declaredVarType(valueSpec *ast.ValueSpec, i int) types.Type {
	if valueSpec.Type == nil || i >= len(valueSpec.Names) {
		return nil
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	if obj := typeInfo.info.Defs[valueSpec.Names[i]]; obj != nil {
		return obj.Type()
	}
	return nil
}

func writePackageGlobalMapWrappedValueCopy(out *strings.Builder, ident *ast.Ident) bool {
	if ident == nil || !isPackageGlobalIdent(ident) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsMap(ident) {
		return false
	}
	// A copy of a package-level map variable shares its map (#328): the
	// variable's storage is the map's handle, as GoGlobalMap and the
	// goroutine program's Arc are.
	if _, ok := types.Unalias(typeInfo.GetType(ident)).(*types.Map); ok {
		out.WriteString(rustPackageGlobalName(ident.Name))
		out.WriteString(".clone()")
		return true
	}
	WriteWrapperPrefix(out)
	out.WriteString("(*")
	out.WriteString(rustPackageGlobalName(ident.Name))
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
	WriteWrapperSuffix(out)
	return true
}

func writePackageGlobalSliceWrappedValueCopy(out *strings.Builder, ident *ast.Ident) bool {
	if ident == nil || !isPackageGlobalIdent(ident) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsSlice(ident) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString("(*")
	out.WriteString(rustPackageGlobalName(ident.Name))
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
	WriteWrapperSuffix(out)
	return true
}

func writeWrappedOwnedExpressionValue(out *strings.Builder, expr ast.Expr) bool {
	if _, ok := expr.(*ast.SelectorExpr); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	if !namedCollectionSelectorNeedsOwnedValue(expr, typ) && !goTypeIsCopiedByValue(typ) {
		return false
	}
	var inner strings.Builder
	if !writeOwnedExpressionValue(&inner, expr) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(inner.String())
	WriteWrapperSuffix(out)
	return true
}

func namedCollectionSelectorNeedsOwnedValue(expr ast.Expr, typ types.Type) bool {
	if _, ok := expr.(*ast.SelectorExpr); !ok {
		return false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return false
	}
	switch named.Underlying().(type) {
	case *types.Slice, *types.Map:
		return true
	default:
		return false
	}
}

// writeConcurrentMapSelectorHandleClone writes m := s.data for a map field
// as a clone of the field's handle: a map is a reference, so the variable
// and the field are the same map (DEV-169). Copying the map's contents
// would let m["k"] = v miss s.data. In a program without goroutines only a
// field of an unnamed map type is shared this way; package-qualified
// variables and named map types keep their own paths.
func writeConcurrentMapSelectorHandleClone(out *strings.Builder, expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsMap(expr) {
		return false
	}
	if !NeedsConcurrentWrapper() {
		if typeInfo.info == nil {
			return false
		}
		selection, ok := typeInfo.info.Selections[sel]
		if !ok || selection.Kind() != types.FieldVal || isNamedMapExpression(expr) {
			return false
		}
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone()")
	return true
}

func writeMapHandleAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsMap(lhs) {
		return false
	}
	if ident, ok := lhs.(*ast.Ident); ok && isPackageGlobalIdent(ident) {
		return false
	}
	// A variable of a map type defined in this package holds the newtype in
	// a cell of its own, not the bare map handle: the value goes into that
	// cell, as for a defined slice type.
	if named, _, ok := namedMapTypeFromType(typeInfo.GetType(lhs)); ok && named.Obj().Pkg() == typeInfo.pkg {
		return false
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("{ let new_val = ")
		WriteWrappedNone(out)
		out.WriteString("; ")
		writePointerHandleAssignmentTargetFromValueName(out, lhs, "new_val")
		out.WriteString(" }")
		return true
	}
	if ident, ok := lhs.(*ast.Ident); ok && isCurrentReceiverIdent(ident) && currentReceiverRustAlias != "" {
		return false
	}
	if !typeInfo.IsMap(rhs) {
		return false
	}
	out.WriteString("{ let new_val = ")
	// m = (a[0]) is m = a[0].
	rhs = unwrapParens(rhs)
	switch rhs.(type) {
	case *ast.Ident, *ast.SelectorExpr:
		TranspileExpressionContext(out, rhs, LValue)
		out.WriteString(".clone()")
	default:
		if index, ok := rhs.(*ast.IndexExpr); ok && isExpressionResultBare(index) && !indexExpressionResultKeepsHandle(index, typeInfo) {
			// m = arr[0] of an array or slice of maps, which holds
			// them bare: the variable is the map's cell.
			WriteWrapperPrefix(out)
			TranspileExpression(out, rhs)
			WriteWrapperSuffix(out)
		} else {
			TranspileExpression(out, rhs)
		}
	}
	out.WriteString("; ")
	writePointerHandleAssignmentTargetFromValueName(out, lhs, "new_val")
	out.WriteString(" }")
	return true
}

func writeNamedMapOptionClone(out *strings.Builder, rhs ast.Expr) {
	out.WriteString("{ let __named_map_holder = ")
	TranspileExpression(out, rhs)
	out.WriteString("; let __named_map_guard = __named_map_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__named_map_guard).clone() }")
}

func writeNamedMapAssignmentOptionValue(out *strings.Builder, targetNamed *types.Named, rhs ast.Expr, rhsType types.Type) {
	if _, _, rhsIsNamedMap := namedMapTypeFromType(rhsType); rhsIsNamedMap {
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.ReturnsWrappedValue(rhs) {
			writeNamedMapOptionClone(out, rhs)
			return
		}
		out.WriteString("Some(")
		if !writeOwnedExpressionValue(out, rhs) {
			TranspileExpression(out, rhs)
		}
		out.WriteString(")")
		return
	}

	out.WriteString("Some(")
	out.WriteString(goTypesNamedTypeToRust(targetNamed))
	out.WriteString("(")
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.ReturnsWrappedValue(rhs) {
		TranspileExpression(out, rhs)
	} else {
		switch rhs.(type) {
		case *ast.Ident, *ast.SelectorExpr:
			TranspileExpressionContext(out, rhs, LValue)
			out.WriteString(".clone()")
		default:
			TranspileExpression(out, rhs)
		}
	}
	out.WriteString("))")
}

func writePackageGlobalNamedMapAssignment(out *strings.Builder, lhsIdent *ast.Ident, rhs ast.Expr, typeInfo *TypeInfo) bool {
	lhsType := typeInfo.GetType(lhsIdent)
	lhsNamed, _, ok := namedMapTypeFromType(lhsType)
	if !ok {
		return false
	}
	rhsType := typeInfo.GetType(rhs)
	if rhsType == nil || underlyingMapType(rhsType) == nil || !types.AssignableTo(rhsType, lhsType) {
		return false
	}
	out.WriteString("{ let new_val = ")
	writeNamedMapAssignmentOptionValue(out, lhsNamed, rhs, rhsType)
	out.WriteString("; *")
	out.WriteString(rustPackageGlobalName(lhsIdent.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = new_val; }")
	return true
}

func writePackageGlobalNamedSliceAssignment(out *strings.Builder, lhsIdent *ast.Ident, rhs ast.Expr, typeInfo *TypeInfo) bool {
	lhsType := typeInfo.GetType(lhsIdent)
	lhsNamed, _, ok := namedSliceTypeFromType(lhsType)
	if !ok {
		return false
	}
	rhsType := typeInfo.GetType(rhs)
	if rhsType == nil || underlyingSliceType(rhsType) == nil || !types.AssignableTo(rhsType, lhsType) {
		return false
	}
	out.WriteString("{ let new_val = ")
	if _, _, rhsIsNamedSlice := namedSliceTypeFromType(rhsType); rhsIsNamedSlice {
		out.WriteString("Some(")
		if !writeOwnedExpressionValue(out, rhs) {
			TranspileExpression(out, rhs)
		}
		out.WriteString(")")
	} else {
		out.WriteString("Some(")
		out.WriteString(goTypesNamedTypeToRust(lhsNamed))
		out.WriteString("(")
		writePlainSliceHandleForNamedSliceConversion(out, rhs)
		out.WriteString("))")
	}
	out.WriteString("; *")
	out.WriteString(rustPackageGlobalName(lhsIdent.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = new_val; }")
	return true
}

func writePackageGlobalCollectionAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	lhsIdent, ok := lhs.(*ast.Ident)
	if !ok || !isPackageGlobalIdent(lhsIdent) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	lhsIsMap := typeInfo.IsMap(lhs)
	lhsIsSlice := typeInfo.IsSlice(lhs)
	if !lhsIsMap && !lhsIsSlice {
		return false
	}
	// A package-level map variable takes the handle of the map assigned to
	// it, so the map it held stays with its other copies (#328).
	if packageGlobalIdentIsMapHandle(lhsIdent) {
		out.WriteString("{ let new_val = ")
		if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
			out.WriteString(GetOuterWrapperType())
			out.WriteString("::new(")
			out.WriteString(GetInnerWrapperType())
			out.WriteString("::new(None))")
			trackWrapperImports()
		} else {
			TranspileExpressionContext(out, rhs, LValue)
			out.WriteString(".clone()")
		}
		out.WriteString("; ")
		out.WriteString(rustPackageGlobalName(lhsIdent.Name))
		out.WriteString(".set(new_val); }")
		return true
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("{ let new_val = None; *")
		out.WriteString(rustPackageGlobalName(lhsIdent.Name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = new_val; }")
		return true
	}
	if lhsIsMap && writePackageGlobalNamedMapAssignment(out, lhsIdent, rhs, typeInfo) {
		return true
	}
	if lhsIsSlice && writePackageGlobalNamedSliceAssignment(out, lhsIdent, rhs, typeInfo) {
		return true
	}
	if lhsIsMap && !typeInfo.IsMap(rhs) || lhsIsSlice && !typeInfo.IsSlice(rhs) {
		return false
	}
	out.WriteString("{ let new_val = { let __collection_holder = ")
	TranspileExpressionContext(out, rhs, LValue)
	out.WriteString(".clone(); let __collection_guard = __collection_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__collection_guard).clone() }; *")
	out.WriteString(rustPackageGlobalName(lhsIdent.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = new_val; }")
	return true
}

func writeSliceHandleAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isPlainSliceExpression(lhs) || !isPlainSliceExpression(rhs) {
		return false
	}
	if writeConcreteSliceAssignmentFromSourceTypeParamSliceCall(out, lhs, rhs) {
		return true
	}
	if !sliceAssignmentRHSReturnsHandle(rhs) {
		return false
	}
	if ident, ok := lhs.(*ast.Ident); ok && isPackageGlobalIdent(ident) {
		return false
	}
	if ident, ok := lhs.(*ast.Ident); ok && currentCaptureRenames != nil {
		if _, captured := currentCaptureRenames[ident.Name]; captured {
			return false
		}
	}
	out.WriteString("{ let new_val = ")
	if writePointerDerefSliceHandleClone(out, rhs, typeInfo) {
		// Pointer-to-slice dereference is represented by the same slice handle.
	} else {
		switch rhs.(type) {
		case *ast.Ident, *ast.SelectorExpr:
			TranspileExpressionContext(out, rhs, LValue)
			out.WriteString(".clone()")
		default:
			TranspileExpression(out, rhs)
		}
	}
	out.WriteString("; ")
	writePointerHandleAssignmentTargetFromValueName(out, lhs, "new_val")
	out.WriteString(" }")
	return true
}

func writePointerDerefSliceHandleClone(out *strings.Builder, rhs ast.Expr, typeInfo *TypeInfo) bool {
	star, ok := unwrapParens(rhs).(*ast.StarExpr)
	if !ok {
		return false
	}
	starType := typeInfo.GetType(star)
	operandType := typeInfo.GetType(star.X)
	if starType == nil || operandType == nil {
		out.WriteString(`unimplemented!("type info required for pointer slice assignment")`)
		return true
	}
	if !pointerDerefTargetsSequence(starType, operandType) {
		return false
	}
	TranspileExpressionContext(out, star.X, LValue)
	out.WriteString(".clone()")
	return true
}

func writePointerDerefSequenceHandleAssignment(out *strings.Builder, star *ast.StarExpr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString(`unimplemented!("type info required for pointer slice assignment")`)
		return true
	}
	starType := typeInfo.GetType(star)
	operandType := typeInfo.GetType(star.X)
	if starType == nil || operandType == nil {
		out.WriteString(`unimplemented!("type info required for pointer slice assignment")`)
		return true
	}
	if _, isNamed := starType.(*types.Named); isNamed {
		return false
	}
	if _, ok := coreUnderlyingType(starType).(*types.Slice); !ok {
		return false
	}
	if !pointerDerefTargetsSequence(starType, operandType) {
		return false
	}

	out.WriteString("{ ")
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("*")
		TranspileExpressionContext(out, star.X, LValue)
		WriteBorrowMethod(out, true)
		out.WriteString(" = None; }")
		return true
	}
	out.WriteString("let new_val = ")
	if writePointerDerefSliceHandleClone(out, rhs, typeInfo) {
		// Pointer-to-slice dereference is represented by the same slice handle.
	} else if writePlainSliceHandleClone(out, rhs) {
		// Existing slice values are represented by handles; copy the handle before reading its option.
	} else {
		TranspileExpression(out, rhs)
	}
	out.WriteString("; let __cloned_val = { let __guard = new_val")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__guard).clone() }; *")
	TranspileExpressionContext(out, star.X, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = __cloned_val; }")
	return true
}

// writePointerDerefMapHandleAssignment writes *p = m for p a *map[K]V. A
// pointer to a map is the map's handle (#321), so the handle takes the map
// m holds; nil makes it a nil map.
func writePointerDerefMapHandleAssignment(out *strings.Builder, star *ast.StarExpr, rhs ast.Expr) bool {
	target, ok := pointerToUnnamedMapDerefTarget(star)
	if !ok {
		return false
	}
	// A package global's slot holds the pointer: the map is reached through
	// the handle read out of it.
	writeTarget := func() { TranspileExpressionContext(out, target, LValue) }
	out.WriteString("{ ")
	if mapPointerIsPackageGlobal(target) {
		out.WriteString("let __map_target = ")
		writeMapPointerHandle(out, target)
		out.WriteString("; ")
		writeTarget = func() { out.WriteString("__map_target") }
	}
	if ident, ok := unwrapParens(rhs).(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("*")
		writeTarget()
		WriteBorrowMethod(out, true)
		out.WriteString(" = None; }")
		return true
	}
	out.WriteString("let new_val = ")
	writeMapHandleCloneForOp(out, rhs)
	out.WriteString("; let __cloned_val = { let __guard = new_val")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__guard).clone() }; *")
	writeTarget()
	WriteBorrowMethod(out, true)
	out.WriteString(" = __cloned_val; }")
	return true
}

func writePlainSliceHandleClone(out *strings.Builder, expr ast.Expr) bool {
	if !isPlainSliceExpression(expr) {
		return false
	}
	switch expr.(type) {
	case *ast.Ident, *ast.SelectorExpr:
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone()")
		return true
	default:
		return false
	}
}

func writeConcreteSliceAssignmentFromSourceTypeParamSliceCall(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	call, ok := rhs.(*ast.CallExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	lhsType := typeInfo.GetType(lhs)
	lhsSlice, ok := types.Unalias(lhsType).Underlying().(*types.Slice)
	if !ok || collectionElemRustTypeIsWrappedHandle(lhsSlice.Elem()) {
		return false
	}
	rhsType := typeInfo.GetType(rhs)
	if rhsType == nil || !types.AssignableTo(rhsType, lhsType) {
		return false
	}
	sourceSig, ok := sourceFunctionSignatureForCall(call)
	if !ok || sourceSig.Results() == nil || sourceSig.Results().Len() != 1 {
		return false
	}
	resultElem, ok := goTypeParamSliceConstraintElem(sourceSig.Results().At(0).Type())
	if !ok {
		return false
	}
	if _, ok := types.Unalias(resultElem).(*types.TypeParam); !ok {
		return false
	}
	out.WriteString("{ let new_val = ")
	if !writeSourceTypeParamSliceCallAsConcreteSlice(out, call) {
		return false
	}
	out.WriteString("; ")
	writePointerHandleAssignmentTargetFromValueName(out, lhs, "new_val")
	out.WriteString(" }")
	return true
}

func sliceAssignmentRHSReturnsHandle(rhs ast.Expr) bool {
	switch rhs.(type) {
	case *ast.TypeAssertExpr:
		return false
	default:
		return true
	}
}

func isPlainSliceExpression(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	if _, isNamed := types.Unalias(typ).(*types.Named); isNamed {
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Slice)
	return ok
}

func writeSliceSelectorHandleClone(out *strings.Builder, expr ast.Expr) bool {
	if _, ok := expr.(*ast.SelectorExpr); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	if _, isNamed := types.Unalias(typ).(*types.Named); isNamed {
		return false
	}
	if _, ok := types.Unalias(typ).Underlying().(*types.Slice); !ok {
		return false
	}
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone()")
	return true
}

func writeFunctionSelectorHandleAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	rhsSel, ok := rhs.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isFunctionSignatureType(typeInfo.GetType(lhs)) || !isFunctionSignatureType(typeInfo.GetType(rhs)) {
		return false
	}
	out.WriteString("{ let new_val = ")
	if sig, ok := pointerMethodValueSignature(rhsSel); ok {
		WriteWrapperPrefix(out)
		writePointerMethodValueBox(out, rhsSel, sig)
		WriteWrapperSuffix(out)
	} else if sig, ok := selectorFunctionValueSignature(rhsSel); ok {
		WriteWrapperPrefix(out)
		writeFunctionValueExpressionBox(out, rhsSel, sig)
		WriteWrapperSuffix(out)
	} else {
		TranspileExpressionContext(out, rhs, LValue)
		out.WriteString(".clone()")
	}
	out.WriteString("; ")
	writePointerHandleAssignmentTargetFromValueName(out, lhs, "new_val")
	out.WriteString(" }")
	return true
}

func functionValueCallAssignmentUsesReturnedSlot(lhs ast.Expr, call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || call == nil {
		return false
	}
	return isFunctionSignatureType(typeInfo.GetType(lhs)) && isFunctionSignatureType(callSingleResultType(call))
}

func writeFunctionPointerDerefCallAssignment(out *strings.Builder, star *ast.StarExpr, rhs ast.Expr) bool {
	call, ok := rhs.(*ast.CallExpr)
	if !ok || !functionValueCallAssignmentUsesReturnedSlot(star, call) {
		return false
	}
	out.WriteString("{ let new_val = ")
	TranspileExpression(out, rhs)
	out.WriteString("; let __moved_val = { let mut __guard = new_val")
	WriteBorrowMethod(out, true)
	out.WriteString("; __guard.take() }; *")
	TranspileExpressionContext(out, star.X, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = __moved_val; }")
	return true
}

func writeCallExpressionForInitializer(out *strings.Builder, call *ast.CallExpr) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.IsTypeConversion(call) && !typeConversionEmitsWrappedValue(call) {
		WriteWrapperPrefix(out)
		TranspileExpression(out, call)
		WriteWrapperSuffix(out)
		return
	}
	TranspileExpression(out, call)
}

func isStdlibNamedInterfaceValueType(typ types.Type) bool {
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	if !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return false
	}
	_, ok = named.Underlying().(*types.Interface)
	return ok
}

func writeStdlibInterfaceFieldValueCopy(out *strings.Builder, expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isStdlibNamedInterfaceValueType(typeInfo.GetType(expr)) {
		return false
	}
	out.WriteString("{ let __src = ")
	TranspileExpressionContext(out, sel, LValue)
	out.WriteString(".clone(); ")
	out.WriteString("let __copied = (*__src")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone(); ")
	WriteWrapperPrefix(out)
	out.WriteString("__copied")
	WriteWrapperSuffix(out)
	out.WriteString(" }")
	return true
}

func writeWrappedStdlibInterfaceRangeValueCopy(out *strings.Builder, ident *ast.Ident) bool {
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar || !strings.HasPrefix(varType, "&") {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isStdlibNamedInterfaceValueType(typeInfo.GetType(ident)) {
		return false
	}
	WriteWrapperPrefix(out)
	writeOwnedRangeValue(out, ident)
	WriteWrapperSuffix(out)
	return true
}

func writeWrappedReferenceRangeValueCopy(out *strings.Builder, ident *ast.Ident) bool {
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar {
		return false
	}
	if varType != "ref_value" && (!strings.HasPrefix(varType, "&") || isWrappedRangeVarType(varType)) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(ident)
	if typ == nil {
		return false
	}
	if !isStdlibNamedInterfaceValueType(typ) {
		switch types.Unalias(typ).Underlying().(type) {
		case *types.Basic, *types.Struct, *types.Array:
		default:
			return false
		}
	}
	WriteWrapperPrefix(out)
	writeOwnedRangeValue(out, ident)
	WriteWrapperSuffix(out)
	return true
}

func writeWrappedBareStringRangeValueCopy(out *strings.Builder, ident *ast.Ident) bool {
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar || varType == "ref_value" || strings.HasPrefix(varType, "&") || isWrappedRangeVarType(varType) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsString(ident) {
		return false
	}
	WriteWrapperPrefix(out)
	writeOwnedRangeValue(out, ident)
	WriteWrapperSuffix(out)
	return true
}

func writeRangeHandleReturnValue(out *strings.Builder, ident *ast.Ident) bool {
	if ident == nil {
		return false
	}
	if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !mapValueTypeKeepsHandle(typeInfo.GetType(ident)) {
		return false
	}
	return writeOwnedRangeValue(out, ident)
}

func writeMapWrappedValue(out *strings.Builder, expr ast.Expr, valueType types.Type) {
	if isUnnamedSliceType(valueType) && writeGoSliceFreshCellFromVariable(out, expr) {
		return
	}
	// A value of a defined integer type stored in a map[K]tally, a constant
	// as in m[k] = 3 or arithmetic as in m[k] = t + 1, is written as a
	// declaration of one is: the newtype around its number (#336).
	// A receive gives the newtype already, and takes the usual path.
	if named, ok := types.Unalias(valueType).(*types.Named); ok && isNamedIntegerType(named) && !isChannelReceiveExpr(expr) && writeNamedIntegerWrappedInitializer(out, expr) {
		return
	}
	if isFunctionSignatureType(valueType) && writeFuncValueFreshCellFromStorage(out, expr, valueType) {
		return
	}
	if isGoErrorType(valueType) && writeGoErrorHandleValue(out, expr) {
		return
	}
	if writeChannelReceiveHandleFor(out, expr, valueType) {
		return
	}
	// The value of a range over a []any is the element's Box itself, which
	// has no Clone; the map gets a copy of its own.
	if isEmptyInterfaceType(valueType) && writeBareAnyReferenceWrappedClone(out, expr) {
		return
	}
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" && isNilableWrappedMapValueType(valueType) {
		WriteWrappedNone(out)
		return
	}
	// m[k] = v for a map of interface{} values boxes v, as a map literal
	// does; an interface{} value is cloned as the handle it already is.
	if isEmptyInterfaceType(valueType) {
		if writeEmptyInterfaceHandleClone(out, expr) {
			return
		}
		WriteWrapperPrefix(out)
		writeInterfaceBoxedValue(out, expr)
		WriteWrapperSuffix(out)
		return
	}
	if isPointerMapValueType(valueType) {
		if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsPointer(expr) {
			writePointerHandleExpression(out, expr)
			return
		}
		if unary, ok := expr.(*ast.UnaryExpr); ok && unary.Op == token.AND {
			if _, isComposite := unary.X.(*ast.CompositeLit); isComposite {
				TranspileExpression(out, expr)
				return
			}
		}
	}
	if isNilableWrappedMapValueType(valueType) && isTypedAssignmentSelfWrappingExpression(expr) {
		TranspileExpression(out, expr)
		return
	}
	// "m[k] = [2]int{a, b}": the array literal is already in a cell of its
	// own.
	if valueType != nil && isTypedAssignmentSelfWrappingExpression(expr) {
		if _, isArray := types.Unalias(valueType).Underlying().(*types.Array); isArray {
			TranspileExpression(out, expr)
			return
		}
	}
	if writeStdlibInterfaceCallArgumentConversion(out, expr, valueType) {
		return
	}
	if writeLocalInterfaceWrappedValue(out, expr, valueType) {
		return
	}
	if writeFunctionMapValue(out, expr, nil, valueType) {
		return
	}
	if ident, ok := expr.(*ast.Ident); ok {
		// A struct variable stored in a map is copied into it.
		if copyCall, known := goValueCopyCallForExpr(ident); known && copyCall != ".clone()" && writeWrappedValueCopyFromIdent(out, ident) {
			return
		}
	}
	if ident, ok := expr.(*ast.Ident); ok &&
		ident.Name != "_" && ident.Name != "nil" && isVarBare(ident.Name) &&
		valueType != nil && !mapValueTypeKeepsHandle(valueType) {
		if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
			varName := RustIdentForUse(ident)
			if currentCaptureRenames != nil {
				if renamed, exists := currentCaptureRenames[ident.Name]; exists {
					varName = RustLocalIdent(renamed)
				}
			}
			WriteWrapperPrefix(out)
			if writeLenCapCallArgumentForExpectedType(out, expr, valueType) {
				// len/cap emits usize, but Go int map values use i32.
			} else if writeRangeIndexForExpectedType(out, expr, valueType) {
				// range indexes emit usize, but Go int map values use i32.
			} else {
				out.WriteString(varName)
			}
			if !rangeElementUsesCopied(valueType) {
				out.WriteString(".clone()")
			}
			WriteWrapperSuffix(out)
			return
		}
	}
	if ident, ok := expr.(*ast.Ident); ok && isCloneableNonPointerExpr(ident) {
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			if isWrappedRangeVarType(varType) && !mapValueTypeKeepsHandle(valueType) {
				WriteWrapperPrefix(out)
				writeWrappedRangeValueClone(out, ident, varType)
				WriteWrapperSuffix(out)
				return
			}
			if varType == "ref_value" || strings.HasPrefix(varType, "&") {
				WriteWrapperPrefix(out)
				out.WriteString("(*")
				out.WriteString(RustIdentForUse(ident))
				out.WriteString(").clone()")
				WriteWrapperSuffix(out)
				return
			}
		}
	}
	if ident, ok := expr.(*ast.Ident); ok &&
		ident.Name != "_" && ident.Name != "nil" && ident.Name != "true" && ident.Name != "false" {
		if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
			if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
				varName := RustIdentForUse(ident)
				if currentCaptureRenames != nil {
					if renamed, exists := currentCaptureRenames[ident.Name]; exists {
						varName = RustLocalIdent(renamed)
					}
				}
				if valueType != nil && !mapValueTypeKeepsHandle(valueType) {
					// Go copies map values. For a value-typed variable, snapshot
					// its current value into a fresh handle rather than aliasing
					// the variable's handle, because the stored entry would track
					// later mutations of the variable (e.g. a loop counter, as in
					// go/token's `keywords[tokens[i]] = i`, which left every entry
					// pointing at the loop's final value).
					WriteWrapperPrefix(out)
					out.WriteString("(*")
					out.WriteString(varName)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).clone()")
					WriteWrapperSuffix(out)
					return
				}
				out.WriteString(varName)
				out.WriteString(".clone()")
				return
			}
		}
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) {
			TranspileExpression(out, expr)
			return
		}
	}
	if _, ok := expr.(*ast.SliceExpr); ok {
		TranspileExpression(out, expr)
		return
	}
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(sel) {
			TranspileExpressionContext(out, expr, LValue)
			out.WriteString(".clone()")
			return
		}
	}

	WriteWrapperPrefix(out)
	if writeLenCapCallArgumentForExpectedType(out, expr, valueType) {
		// len/cap emits usize, but Go int map values use i32.
	} else if writeRangeIndexForExpectedType(out, expr, valueType) {
		// range indexes emit usize, but Go int map values use i32.
	} else if ident, ok := expr.(*ast.Ident); ok && writeOwnedRangeValue(out, ident) {
		// range value cloned above
	} else if !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
		// owned expression written above
	} else {
		TranspileExpression(out, expr)
	}
	WriteWrapperSuffix(out)
}

func isPointerMapValueType(valueType types.Type) bool {
	if valueType == nil {
		return false
	}
	_, ok := types.Unalias(valueType).Underlying().(*types.Pointer)
	return ok
}

func writeMapKeyExpression(out *strings.Builder, expr ast.Expr) {
	writeMapKeyExpressionWithType(out, expr, nil)
}

func writeMapKeyExpressionWithType(out *strings.Builder, expr ast.Expr, keyType types.Type) {
	if keyType != nil && writeStdlibInterfaceComparableConversion(out, expr, keyType) {
		return
	}
	if writeRangeCharMapKey(out, expr, keyType, "", "") {
		return
	}
	if keyType != nil {
		if ifaceName, ok := localNamedInterfaceTypeNameFromTypes(keyType); ok {
			out.WriteString(localInterfaceMapKeyTypeName(RustTypeNameForUse(ifaceName)))
			out.WriteString("::new(")
			if !writeLocalInterfaceMapKeyHandle(out, expr, keyType) {
				TranspileExpressionContext(out, expr, LValue)
				out.WriteString(".clone()")
			}
			out.WriteString(")")
			return
		}
	}
	// Interface-typed map keys: wrap with GoLocalPtrKey to match the map's
	// key type. The wrapped interface handle's identity is the Arc/Rc
	// pointer, which matches Go's interface identity for pointer-backed
	// dynamic values.
	if keyType != nil {
		if _, ok := transpiledNamedInterfaceTypeNameFromTypes(keyType); ok {
			NeedGoPtrKey()
			out.WriteString("GoLocalPtrKey::new(")
			if !writeLocalInterfaceMapKeyHandle(out, expr, keyType) {
				TranspileExpressionContext(out, expr, LValue)
				out.WriteString(".clone()")
			}
			out.WriteString(")")
			return
		}
	}
	if writeSliceElemPtrMapKeyExpression(out, expr) {
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsPointer(expr) {
		out.WriteString(goPtrKeyHelperNameForType(typeInfo.GetType(expr)))
		out.WriteString("::new(")
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone())")
		return
	}
	if keyType != nil {
		if isEmptyInterfaceType(keyType) {
			NeedGoPtrKey()
			out.WriteString("GoLocalPtrKey::new(")
			TranspileExpressionContext(out, expr, LValue)
			out.WriteString(".clone())")
			return
		}
	}
	if keyType != nil && writeMapKeyForExpectedType(out, expr, keyType) {
		return
	}
	if writeOwnedMapKeyExpression(out, expr) {
		return
	}
	if !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
		return
	}
	TranspileExpression(out, expr)
}

func writeMapAssignmentKeyExpression(out *strings.Builder, key ast.Expr, keyType types.Type, rhs ast.Expr) {
	if ident, ok := key.(*ast.Ident); ok && mapAssignmentKeyNeedsClone(ident, keyType, rhs) {
		out.WriteString(rustIdentForUseWithCapture(ident))
		out.WriteString(".clone()")
		return
	}
	writeMapKeyExpressionWithType(out, key, keyType)
}

func writeMapAssignmentKeyExpressionWithRustType(out *strings.Builder, key ast.Expr, keyRustType string) bool {
	keyHelper, ok := mapPointerKeyHelperFromRustType(keyRustType)
	if !ok {
		return false
	}
	if keyHelper == "GoLocalPtrKey" && writeSliceElemPtrMapKeyExpression(out, key) {
		return true
	}
	out.WriteString(keyHelper)
	out.WriteString("::new(")
	TranspileExpressionContext(out, key, LValue)
	out.WriteString(".clone())")
	return true
}

func mapAssignmentKeyRequiresTypeDrivenConversion(keyType types.Type) bool {
	if keyType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(keyType); ok {
		return true
	}
	return isEmptyInterfaceType(keyType)
}

func mapAssignmentKeyNeedsClone(ident *ast.Ident, keyType types.Type, rhs ast.Expr) bool {
	if ident == nil || rhs == nil || ident.Name == "_" || ident.Name == "nil" {
		return false
	}
	if isCopyTypeExpression(ident) || !isCloneableNonPointerExpr(ident) {
		return false
	}
	if isWrappedValueIdent(ident) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.IsPointer(ident) {
		return false
	}
	if keyType != nil && stdlibInterfaceArgumentConversionExists(ident, keyType) {
		return false
	}
	if mapAssignmentRangeKeyNeedsClone(ident) {
		return true
	}
	return expressionReferencesIdentObject(rhs, ident)
}

func mapAssignmentRangeKeyNeedsClone(ident *ast.Ident) bool {
	varType, isRangeVar := rangeLoopVars[ident.Name]
	if !isRangeVar {
		return false
	}
	if varType == "ref_value" || strings.HasPrefix(varType, "&") || isWrappedRangeVarType(varType) {
		return false
	}
	return true
}

func expressionReferencesIdentObject(expr ast.Expr, ident *ast.Ident) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || expr == nil || ident == nil {
		return false
	}
	target := typeInfo.GetObject(ident)
	if target == nil {
		return false
	}
	found := false
	ast.Inspect(expr, func(n ast.Node) bool {
		if found {
			return false
		}
		usedIdent, ok := n.(*ast.Ident)
		if !ok {
			return true
		}
		if typeInfo.GetObject(usedIdent) == target {
			found = true
			return false
		}
		return true
	})
	return found
}

func isMapIndexExpression(expr ast.Expr) (*ast.IndexExpr, bool) {
	indexExpr, ok := expr.(*ast.IndexExpr)
	if !ok {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.IsMap(indexExpr.X) {
		return indexExpr, true
	}
	if kind, ok := localCollectionKind(indexExpr.X); ok {
		return indexExpr, kind == "map"
	}
	return indexExpr, false
}

func isBareBuiltinReturn(call *ast.CallExpr) bool {
	return isBareBuiltinCallName(call, "len") ||
		isBareBuiltinCallName(call, "cap") ||
		isBareBuiltinCallName(call, "min") ||
		isBareBuiltinCallName(call, "max")
}

func expectsGoInt(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	return ok && ident.Name == "int"
}

func writeBareBuiltinReturnForExpectedType(out *strings.Builder, call *ast.CallExpr, expected ast.Expr) bool {
	if !isBareBuiltinReturn(call) || !expectsGoInt(expected) {
		return false
	}
	TranspileExpression(out, call)
	out.WriteString(" as " + rustIntType())
	return true
}

func returnResultTypeExpr(fnType *ast.FuncType, index int) ast.Expr {
	if fnType == nil || fnType.Results == nil || index < 0 {
		return nil
	}
	for _, result := range fnType.Results.List {
		count := len(result.Names)
		if count == 0 {
			count = 1
		}
		if index < count {
			return result.Type
		}
		index -= count
	}
	return nil
}

func writeBareStructAliasValue(out *strings.Builder, expr ast.Expr) {
	if writeOwnedExpressionValue(out, expr) {
		return
	}
	TranspileExpression(out, expr)
}

func writeNamedSliceInnerHandleReturnValue(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	return writeNamedSliceInnerHandleForExpectedType(out, result, expectedTypeFromParamExpr(expected))
}

func writeNamedSliceWrappedReturnValue(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	if _, ok := result.(*ast.SliceExpr); !ok {
		return false
	}
	expectedType := expectedTypeFromParamExpr(expected)
	expectedNamed, _, ok := namedSliceTypeFromType(expectedType)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("unimplemented!(\"type info required for named slice return\")")
		return true
	}
	actual := typeInfo.GetType(result)
	if actual == nil {
		out.WriteString("unimplemented!(\"type info required for named slice return\")")
		return true
	}
	if !types.AssignableTo(actual, expectedType) {
		return false
	}
	actualNamed, _, ok := namedSliceTypeFromType(actual)
	if !ok || !sameNamedTypeDefinition(actualNamed, expectedNamed) {
		return false
	}

	WriteWrapperPrefix(out)
	TranspileExpression(out, result)
	WriteWrapperSuffix(out)
	return true
}

func typeExprIsPointer(expr ast.Expr) bool {
	if expr == nil {
		return false
	}
	if _, ok := expr.(*ast.StarExpr); ok {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	return ok
}

func writePointerNamedReturnAssignment(out *strings.Builder, name *ast.Ident, resultType ast.Expr, rhs ast.Expr) bool {
	if name == nil || name.Name == "_" || !typeExprIsPointer(resultType) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(rhs) {
		return false
	}
	out.WriteString("{ let new_val = ")
	if isSliceElementAddress(rhs) {
		out.WriteString(`unimplemented!("slice element pointer return requires pointer representation support")`)
	} else if writeSourceMappedPackageGlobalPointerHandleClone(out, rhs) {
	} else {
		TranspileExpressionContext(out, rhs, AddressOf)
		out.WriteString(".clone()")
	}
	out.WriteString("; ")
	out.WriteString(RustLocalIdent(name.Name))
	out.WriteString(" = new_val; }")
	return true
}

func writeDirectTypeParamNamedReturnAssignment(out *strings.Builder, name *ast.Ident, resultType ast.Expr, rhs ast.Expr) bool {
	if name == nil || name.Name == "_" || resultType == nil || rhs == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	expected := typeInfo.GetType(resultType)
	actual := typeInfo.GetType(rhs)
	if expected == nil || actual == nil {
		return false
	}
	if !isDirectTypeParamType(expected) || !isDirectTypeParamType(actual) || !types.AssignableTo(actual, expected) {
		return false
	}
	out.WriteString(RustLocalIdent(name.Name))
	out.WriteString(" = ")
	if ident, ok := unwrapParens(rhs).(*ast.Ident); ok {
		out.WriteString(RustIdentForUse(ident))
		out.WriteString(".clone()")
		return true
	}
	if !writeTypeParamHandleExpression(out, rhs) {
		out.WriteString(`/* ERROR: type-parameter named return assignment requires handle expression */ unimplemented!("type-parameter named return assignment")`)
	}
	return true
}

func writeStdlibInterfaceNamedReturnAssignment(out *strings.Builder, name *ast.Ident, resultType ast.Expr, rhs ast.Expr) bool {
	if name == nil || name.Name == "_" {
		return false
	}
	var converted strings.Builder
	if !writeStdlibInterfaceCallArgumentConversion(&converted, rhs, expectedTypeFromParamExpr(resultType)) {
		return false
	}
	out.WriteString("{ let new_val = ")
	out.WriteString(converted.String())
	out.WriteString("; let __moved_val = { let mut __guard = new_val")
	WriteBorrowMethod(out, true)
	out.WriteString("; __guard.take() }; *")
	out.WriteString(RustLocalIdent(name.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = __moved_val; }")
	return true
}

// writeEmptyInterfaceNamedReturnAssignment stores a concrete value that a
// return statement gives an interface{} result: the value goes into the
// result's cell boxed, as an assignment to the result would box it.
func writeEmptyInterfaceNamedReturnAssignment(out *strings.Builder, name *ast.Ident, resultType ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if name == nil || name.Name == "_" || typeInfo == nil || resultType == nil {
		return false
	}
	if ident, ok := unwrapParens(rhs).(*ast.Ident); ok && ident.Name == "nil" {
		return false
	}
	if resultTyp := typeInfo.GetType(resultType); resultTyp == nil || !isEmptyInterfaceType(resultTyp) {
		return false
	}
	if rhsType := typeInfo.GetType(rhs); rhsType == nil || types.IsInterface(rhsType) {
		return false
	}
	out.WriteString("{ let new_val = ")
	writeBareAnyBox(out, rhs)
	out.WriteString("; *")
	out.WriteString(RustLocalIdent(name.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(new_val); }")
	return true
}

func writeLocalInterfaceNamedReturnAssignment(out *strings.Builder, name *ast.Ident, resultType ast.Expr, rhs ast.Expr) bool {
	if name == nil || name.Name == "_" {
		return false
	}
	var converted strings.Builder
	if !writeLocalInterfaceConcreteReturnConversion(&converted, rhs, resultType) {
		return false
	}
	out.WriteString("{ let new_val = ")
	out.WriteString(converted.String())
	out.WriteString("; let __moved_val = { let mut __guard = new_val")
	WriteBorrowMethod(out, true)
	out.WriteString("; __guard.take() }; *")
	out.WriteString(RustLocalIdent(name.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = __moved_val; }")
	return true
}

func writeLocalInterfaceHandleAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || lhs == nil || rhs == nil {
		return false
	}
	lhsType := typeInfo.GetType(lhs)
	rhsType := typeInfo.GetType(rhs)
	if lhsType == nil || rhsType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(lhsType); !ok {
		return false
	}
	rhsIsTranspiledInterface := false
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(rhsType); ok {
		rhsIsTranspiledInterface = true
	}
	if !rhsIsTranspiledInterface && !isBareLocalInterfaceValue(rhs) {
		return false
	}
	if !types.AssignableTo(rhsType, lhsType) {
		return false
	}

	if lhsIdent, ok := lhs.(*ast.Ident); ok && isPackageGlobalIdent(lhsIdent) {
		writePackageGlobalInterfaceHandleAssignment(out, lhsIdent, rhs, lhsType)
		return true
	}

	writeTranspiledInterfaceSlotAssignmentFromHandle(out, lhs, func() {
		writeLocalInterfaceHandleAssignmentValue(out, lhs, rhs, lhsType, true)
	})
	return true
}

func writeLocalInterfaceHandleAssignmentValue(out *strings.Builder, lhs ast.Expr, rhs ast.Expr, lhsType types.Type, allowSelfCallClone bool) {
	if isBareLocalInterfaceValue(rhs) {
		WriteWrapperPrefix(out)
		writeLocalInterfaceBareClone(out, rhs)
		WriteWrapperSuffix(out)
	} else if allowSelfCallClone && writeLocalInterfaceAssignedCallHandleClone(out, lhs, rhs) {
		// RHS emitted by helper.
	} else if writeLocalInterfaceReferenceCallArgument(out, rhs, lhsType) {
		// Interface-to-interface assignment may need re-boxing when the Rust
		// trait-object types differ, such as imported Expr assigned to positioner.
	} else {
		writeLocalInterfaceHandleShare(out, rhs)
	}
}

func writePackageGlobalInterfaceHandleAssignment(out *strings.Builder, lhs *ast.Ident, rhs ast.Expr, lhsType types.Type) {
	out.WriteString("{ let __iface_handle = ")
	writeLocalInterfaceHandleAssignmentValue(out, lhs, rhs, lhsType, false)
	out.WriteString("; let __iface_value = { let __iface_guard = __iface_handle")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__iface_guard).clone() }; *")
	out.WriteString(rustPackageGlobalName(lhs.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = __iface_value; }")
}

func transpiledInterfaceHandleAssignmentTypes(lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || lhs == nil || rhs == nil {
		return false
	}
	lhsType := typeInfo.GetType(lhs)
	rhsType := typeInfo.GetType(rhs)
	if lhsType == nil || rhsType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(lhsType); !ok {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(rhsType); !ok {
		return false
	}
	return types.AssignableTo(rhsType, lhsType)
}

func writeTranspiledInterfaceAssignmentTarget(out *strings.Builder, lhs ast.Expr) {
	if sel, ok := lhs.(*ast.SelectorExpr); ok && writePointerHandleSelectorTarget(out, sel) {
		// Selector handle fields need the outer struct borrowed mutably before
		// the field slot can be updated.
	} else {
		TranspileExpressionContext(out, lhs, LValue)
	}
}

func writeTranspiledInterfaceSlotAssignmentFromHandle(out *strings.Builder, lhs ast.Expr, writeHandle func()) {
	out.WriteString("{ let __iface_handle = ")
	writeHandle()
	out.WriteString("; let __iface_value = { let __iface_guard = __iface_handle")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__iface_guard).clone() }; *")
	writeTranspiledInterfaceAssignmentTarget(out, lhs)
	WriteBorrowMethod(out, true)
	if bridge := stubIoFieldAssignmentBridge(lhs); bridge != "" {
		out.WriteString(" = ")
		writeStubIoFieldBridgeValue(out, bridge)
		out.WriteString("; }")
		return
	}
	out.WriteString(" = __iface_value; }")
}

func writeTranspiledInterfaceHandleAssignmentTarget(out *strings.Builder, lhs ast.Expr, value string) {
	writeTranspiledInterfaceSlotAssignmentFromHandle(out, lhs, func() {
		out.WriteString(value)
	})
}

// writeConcreteToTranspiledInterfaceAssignment boxes a concrete value assigned
// to an interface-typed target. writeLocalInterfaceHandleAssignment only covers
// an interface LHS with an interface/bare-interface RHS (handle clone);
// reassigning a concrete value (`typ = name` where typ is ast.Expr and name is
// *ast.Ident) needs `Box::new(value) as Box<dyn Iface>` and works for an
// imported interface (go_ast::Expr) too. go/parser hits this on nearly every
// node-building path.
func writeConcreteToTranspiledInterfaceAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || lhs == nil || rhs == nil {
		return false
	}
	lhsType := typeInfo.GetType(lhs)
	rhsType := typeInfo.GetType(rhs)
	if lhsType == nil || rhsType == nil {
		return false
	}
	ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(lhsType)
	if !ok {
		return false
	}
	// Interface-to-interface and bare-interface RHS are the handle-clone case
	// handled by writeLocalInterfaceHandleAssignment before this runs.
	if _, rhsIsIface := transpiledNamedInterfaceTypeNameFromTypes(rhsType); rhsIsIface {
		return false
	}
	if isBareLocalInterfaceValue(rhs) {
		return false
	}
	// nil is handled earlier in the assignment dispatch.
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		return false
	}
	if !types.AssignableTo(rhsType, lhsType) {
		return false
	}
	writeTranspiledInterfaceSlotAssignmentFromHandle(out, lhs, func() {
		writeLocalInterfaceWrappedConstruction(out, rhs, ifaceName, lhsType)
	})
	return true
}

// writeConcreteToTranspiledInterfaceValue boxes a concrete rhs into the wrapped
// transpiled-interface form when ifaceType is a transpiled named interface and
// rhs is an assignable concrete (non-interface, non-nil) value. It emits only
// the value (no `lhs =`), for use at initializer/return positions. Returns true
// if it emitted.
func writeConcreteToTranspiledInterfaceValue(out *strings.Builder, rhs ast.Expr, ifaceType types.Type) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || rhs == nil || ifaceType == nil {
		return false
	}
	ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(ifaceType)
	if !ok {
		return false
	}
	rhsType := typeInfo.GetType(rhs)
	if rhsType == nil {
		return false
	}
	// Interface-to-interface and bare-interface RHS keep the handle; nil is
	// handled by the caller.
	if _, rhsIsIface := transpiledNamedInterfaceTypeNameFromTypes(rhsType); rhsIsIface {
		return false
	}
	if isBareLocalInterfaceValue(rhs) {
		return false
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		return false
	}
	if !types.AssignableTo(rhsType, ifaceType) {
		return false
	}
	writeLocalInterfaceWrappedConstruction(out, rhs, ifaceName, ifaceType)
	return true
}

func writeTranspiledInterfaceInitializerValue(out *strings.Builder, rhs ast.Expr, ifaceType types.Type) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || rhs == nil || ifaceType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(ifaceType); !ok {
		return false
	}
	rhsType := typeInfo.GetType(rhs)
	if rhsType == nil {
		return false
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		return false
	}
	if !types.AssignableTo(rhsType, ifaceType) {
		return false
	}
	return writeLocalInterfaceReferenceCallArgument(out, rhs, ifaceType)
}

func writeTranspiledInterfaceHandleClone(out *strings.Builder, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || rhs == nil {
		return false
	}
	rhsType := typeInfo.GetType(rhs)
	if rhsType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(rhsType); !ok {
		return false
	}
	if isBareLocalInterfaceValue(rhs) {
		WriteWrapperPrefix(out)
		writeLocalInterfaceBareClone(out, rhs)
		WriteWrapperSuffix(out)
		return true
	}
	writeLocalInterfaceHandleClone(out, rhs)
	return true
}

func writeLocalInterfaceAssignedCallHandleClone(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	lhsIdent, ok := lhs.(*ast.Ident)
	if !ok {
		return false
	}
	call, ok := rhs.(*ast.CallExpr)
	if !ok {
		return false
	}
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	recvIdent, ok := sel.X.(*ast.Ident)
	if !ok || recvIdent.Name != lhsIdent.Name {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		lhsObj := typeInfo.GetObject(lhsIdent)
		recvObj := typeInfo.GetObject(recvIdent)
		if lhsObj != nil && recvObj != nil && lhsObj != recvObj {
			return false
		}
	}

	out.WriteString("{ let __recv = ")
	TranspileExpressionContext(out, recvIdent, LValue)
	out.WriteString(".clone(); let __result = ")
	oldRenames := currentCaptureRenames
	nextRenames := make(map[string]string, len(oldRenames)+1)
	for name, renamed := range oldRenames {
		nextRenames[name] = renamed
	}
	nextRenames[recvIdent.Name] = "__recv"
	currentCaptureRenames = nextRenames
	TranspileExpressionContext(out, rhs, LValue)
	currentCaptureRenames = oldRenames
	out.WriteString(".clone(); __result }")
	return true
}

func writeErrorChannelNamedReturnAssignment(out *strings.Builder, name *ast.Ident, resultType ast.Expr, rhs ast.Expr) bool {
	if name == nil || name.Name == "_" {
		return false
	}
	unary, ok := rhs.(*ast.UnaryExpr)
	if !ok || unary.Op != token.ARROW || !channelReceiveFillsHandleOf(unary.X, expectedTypeFromParamExpr(resultType)) {
		return false
	}
	out.WriteString("{ let new_val = ")
	writeChannelExpression(out, unary.X)
	out.WriteString(".recv().unwrap_or_default(); *")
	out.WriteString(RustLocalIdent(name.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = new_val; }")
	return true
}

func writeFunctionNamedReturnAssignment(out *strings.Builder, name *ast.Ident, resultType ast.Expr, rhs ast.Expr) bool {
	if name == nil || name.Name == "_" {
		return false
	}
	if _, ok := functionSignatureFromTypeExpr(resultType); !ok {
		return false
	}
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("*")
		out.WriteString(RustLocalIdent(name.Name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = None")
		return true
	}
	funcLit, ok := rhs.(*ast.FuncLit)
	if !ok {
		return false
	}
	out.WriteString("{ let new_val = ")
	TranspileFuncLitBox(out, funcLit)
	out.WriteString("; *")
	out.WriteString(RustLocalIdent(name.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(new_val); }")
	return true
}

func writeFunctionReturnValue(out *strings.Builder, result ast.Expr, resultType ast.Expr) bool {
	if _, ok := functionSignatureFromTypeExpr(resultType); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString(`unimplemented!("type info required to lower function return value")`)
		return true
	}
	typ := typeInfo.GetType(result)
	if typ == nil {
		out.WriteString(`unimplemented!("type info required to lower function return value")`)
		return true
	}
	if !isFunctionSignatureType(typ) {
		return false
	}
	return writeFunctionValueHandle(out, result)
}

func writePointerReturnValue(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	if !isPointerReturnExpression(result, expected) {
		return false
	}
	if writeCurrentReceiverPointerHandleReturnValue(out, result, expected) {
		return true
	}
	if writeSliceElemPtrReturnValue(out, result) {
		return true
	}
	if indexExpr, ok := unwrapParens(result).(*ast.IndexExpr); ok {
		indent := currentLineIndent(out)
		if writeMultilineGenericConcurrentSequenceIndexValue(out, indexExpr, indent) {
			return true
		}
	}
	if isSliceElementAddress(result) {
		out.WriteString(`unimplemented!("slice element pointer return requires pointer representation support")`)
		return true
	}
	switch result.(type) {
	case *ast.Ident, *ast.SelectorExpr:
		if writeSourceMappedPackageGlobalPointerHandleClone(out, result) {
			return true
		}
		TranspileExpressionContext(out, result, LValue)
		out.WriteString(".clone()")
	default:
		TranspileExpression(out, result)
	}
	return true
}

func writeCurrentReceiverPointerHandleReturnValue(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	ident, ok := result.(*ast.Ident)
	if !ok || !currentReceiverRustAliasIsPointerHandle || !isCurrentReceiverIdent(ident) {
		return false
	}
	if !isPointerReturnExpression(result, expected) {
		return false
	}
	out.WriteString(currentReceiverRustName())
	out.WriteString(".clone()")
	return true
}

func writeNilReturnValue(out *strings.Builder, expected ast.Expr) {
	if typ := GetTypeInfo().GetType(expected); isChannelFieldType(typ) {
		writeChannelNilDefault(out, goTypesTypeToRust(typ))
		return
	}
	WriteWrappedNone(out)
}

func writeBlankNamedReturnValue(out *strings.Builder, result ast.Expr, expected ast.Expr) {
	if ident, ok := result.(*ast.Ident); ok && ident.Name == "nil" {
		writeNilReturnValue(out, expected)
		return
	}
	if resultTypeExprIsBareScalar(expected) {
		writeBareScalarReturnValue(out, result, expected)
		return
	}
	if writeLocalInterfaceConcreteReturnConversion(out, result, expected) {
		return
	}
	if writeStdlibInterfaceReturnConversion(out, result, expected) {
		return
	}
	if writeGoErrorReturnValue(out, result, expected) {
		return
	}
	if isConcreteErrorReturnValue(result, expected) {
		WriteWrapperPrefix(out)
		writeConcreteErrorBox(out, result)
		WriteWrapperSuffix(out)
		return
	}
	if writePointerReturnValue(out, result, expected) {
		return
	}
	if compositeLit, ok := result.(*ast.CompositeLit); ok && isCompositeLitSelfWrapping(compositeLit) {
		TranspileExpression(out, result)
		return
	}
	if _, ok := result.(*ast.SliceExpr); ok {
		TranspileExpression(out, result)
		return
	}
	if selectorExpressionKeepsHandle(result) {
		TranspileExpressionContext(out, result, LValue)
		out.WriteString(".clone()")
		return
	}
	if mapIndexExpressionKeepsHandle(result) {
		TranspileExpression(out, result)
		return
	}
	if call, ok := result.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
			TranspileExpression(out, result)
			return
		}
	}
	if ident, ok := result.(*ast.Ident); ok {
		if globalIdent, ok := packageGlobalPointerIdent(ident); ok {
			writePackageGlobalPointerHandleClone(out, globalIdent)
			return
		}
		if writeRangeHandleReturnValue(out, ident) {
			return
		}
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(result) && !isConstIdent(ident) {
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(".clone()")
			return
		}
	}

	WriteWrapperPrefix(out)
	if call, ok := result.(*ast.CallExpr); ok && writeBareBuiltinReturnForExpectedType(out, call, expected) {
		// Builtin emitted in the expected Go result representation.
	} else if writeExpressionForExpectedType(out, result, expected) {
		// Constant or typed expression emitted in the expected representation.
	} else if lit, ok := result.(*ast.BasicLit); ok && lit.Kind == token.STRING {
		out.WriteString(RustStringLiteral(lit.Value))
		out.WriteString(".to_string()")
	} else if !isCopyTypeExpression(result) && writeOwnedExpressionValue(out, result) {
		// Owned non-copy value emitted above.
	} else {
		TranspileExpression(out, result)
	}
	WriteWrapperSuffix(out)
}

func returnResultsShouldUseMultiline(results []ast.Expr) bool {
	if !NeedsConcurrentWrapper() || len(results) < 2 {
		return false
	}
	for _, result := range results {
		if compositeLiteralElementIsComplex(result) {
			return true
		}
	}
	return false
}

func writeNamedReturnAssignmentFromTemp(out *strings.Builder, name *ast.Ident, resultType ast.Expr, tempName string) {
	if name == nil || name.Name == "_" {
		return
	}
	if resultTypeExprIsBareScalar(resultType) {
		out.WriteString("        *")
		out.WriteString(RustLocalIdent(name.Name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = Some(")
		out.WriteString(tempName)
		out.WriteString(");\n")
		return
	}
	if isGoErrorType(expectedTypeFromParamExpr(resultType)) {
		out.WriteString("        { let __moved_val = { let mut __guard = ")
		out.WriteString(tempName)
		WriteBorrowMethod(out, true)
		out.WriteString("; __guard.take() }; *")
		out.WriteString(RustLocalIdent(name.Name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = __moved_val; }\n")
		return
	}
	out.WriteString("        ")
	out.WriteString(RustLocalIdent(name.Name))
	out.WriteString(" = ")
	out.WriteString(tempName)
	out.WriteString(";\n")
}

func writeNamedReturnValuesWithBlankTemps(out *strings.Builder, fnType *ast.FuncType, blankTemps []string) {
	names := namedReturnIdents(fnType)
	if len(names) == 0 {
		return
	}
	if len(names) > 1 {
		out.WriteString("(")
	}
	first := true
	resultIndex := 0
	for _, result := range fnType.Results.List {
		for _, name := range result.Names {
			if !first {
				out.WriteString(", ")
			}
			first = false
			if name.Name == "_" {
				if resultIndex < len(blankTemps) && blankTemps[resultIndex] != "" {
					out.WriteString(blankTemps[resultIndex])
				} else if resultTypeExprIsBareScalar(result.Type) {
					writeBareScalarZeroValue(out, result.Type)
				} else {
					writeNamedReturnZeroValue(out, result.Type)
				}
			} else {
				if resultTypeExprIsBareScalar(result.Type) {
					out.WriteString("(*")
					out.WriteString(RustLocalIdent(name.Name))
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())")
				} else {
					out.WriteString(RustLocalIdent(name.Name))
					out.WriteString(".clone()")
				}
			}
			resultIndex++
		}
	}
	if len(names) > 1 {
		out.WriteString(")")
	}
}

func namedTypeForTypeExpr(expr ast.Expr) (*types.Named, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return nil, false
	}
	if typ := typeInfo.GetType(expr); typ != nil {
		if named, ok := typ.(*types.Named); ok {
			return named, true
		}
	}
	if typeInfo.info == nil {
		return nil, false
	}
	switch e := expr.(type) {
	case *ast.Ident:
		if obj, ok := typeInfo.info.Uses[e].(*types.TypeName); ok {
			if named, ok := obj.Type().(*types.Named); ok {
				return named, true
			}
		}
	case *ast.SelectorExpr:
		if obj, ok := typeInfo.info.Uses[e.Sel].(*types.TypeName); ok {
			if named, ok := obj.Type().(*types.Named); ok {
				return named, true
			}
		}
	}
	return nil, false
}

func stdlibInterfaceReturnConversion(result ast.Expr, expected ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || result == nil || expected == nil {
		return false
	}
	targetNamed, ok := namedTypeForTypeExpr(expected)
	if !ok || targetNamed.Obj() == nil || targetNamed.Obj().Pkg() == nil {
		return false
	}
	if !isStubBackedStdlibPackagePath(targetNamed.Obj().Pkg().Path()) {
		return false
	}
	targetInterface, ok := targetNamed.Underlying().(*types.Interface)
	if !ok {
		return false
	}
	sourceType := typeInfo.GetType(result)
	if sourceType == nil {
		return false
	}
	sourceNamedType := sourceType
	if ptr, ok := sourceType.(*types.Pointer); ok {
		sourceNamedType = ptr.Elem()
	}
	sourceNamed, ok := sourceNamedType.(*types.Named)
	if !ok || sourceNamed.Obj() == nil || sourceNamed.Obj().Pkg() == nil {
		return false
	}
	if sourceNamed.Obj() == targetNamed.Obj() {
		return false
	}
	if !isStubBackedStdlibPackagePath(sourceNamed.Obj().Pkg().Path()) {
		return false
	}
	if isKnownStdlibHelperType(sourceNamed.Obj().Pkg().Path(), sourceNamed.Obj().Name()) &&
		!stdlibHelperTypeAllowsInterfaceConversion(
			sourceNamed.Obj().Pkg().Path(),
			sourceNamed.Obj().Name(),
			targetNamed.Obj().Pkg().Path(),
			targetNamed.Obj().Name(),
		) {
		return false
	}
	targetInterface.Complete()
	if !types.Implements(sourceType, targetInterface) {
		return false
	}
	targetRust := goTypesNamedTypeToRust(targetNamed)
	sourceRust := goTypesNamedTypeToRust(sourceNamed)
	RegisterExternalTypeStubConversion(targetRust, sourceRust)
	return true
}

func writeStdlibInterfaceIdentReturnConversion(out *strings.Builder, ident *ast.Ident, expected ast.Expr) bool {
	if writeStdlibInterfaceCallArgumentConversion(out, ident, expectedTypeFromParamExpr(expected)) {
		return true
	}
	if !stdlibInterfaceReturnConversion(ident, expected) {
		return false
	}
	varName := RustIdentForUse(ident)
	if currentCaptureRenames != nil {
		if renamed, exists := currentCaptureRenames[ident.Name]; exists {
			varName = RustLocalIdent(renamed)
		}
	}
	WriteWrapperPrefix(out)
	out.WriteString("(*")
	out.WriteString(varName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone().into()")
	WriteWrapperSuffix(out)
	return true
}

func writeStdlibInterfaceReturnConversion(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	if writeStdlibInterfaceCallArgumentConversion(out, result, expectedTypeFromParamExpr(expected)) {
		return true
	}

	unaryExpr, ok := result.(*ast.UnaryExpr)
	if !ok || unaryExpr.Op != token.AND {
		return false
	}
	compositeLit, ok := unaryExpr.X.(*ast.CompositeLit)
	if !ok {
		return false
	}
	if !stdlibInterfaceReturnConversion(result, expected) {
		return false
	}
	WriteWrapperPrefix(out)
	TranspileExpressionContext(out, compositeLit, AddressOf)
	out.WriteString(".into()")
	WriteWrapperSuffix(out)
	return true
}

func isPointerReturnExpected(expected ast.Expr) bool {
	if expected == nil {
		return false
	}
	if _, ok := expected.(*ast.StarExpr); ok {
		return true
	}
	expectedType := expectedTypeFromParamExpr(expected)
	if expectedType == nil {
		return false
	}
	_, ok := types.Unalias(expectedType).Underlying().(*types.Pointer)
	return ok
}

func isPointerReturnExpression(result ast.Expr, expected ast.Expr) bool {
	if !isPointerReturnExpected(expected) {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.IsPointer(result)
}

func isSliceElementAddress(expr ast.Expr) bool {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.IsSlice(indexExpr.X)
}

func writeStdlibInterfaceAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	lhsType := typeInfo.GetType(lhs)
	var value strings.Builder
	if !writeStdlibInterfaceBareConversion(&value, rhs, lhsType) {
		return false
	}

	out.WriteString("{ let new_val = ")
	out.WriteString(value.String())
	out.WriteString("; *")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(new_val); }")
	return true
}

func isBuiltinCallNamed(call *ast.CallExpr, name string) bool {
	ident, ok := call.Fun.(*ast.Ident)
	if !ok || ident.Name != name {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj, ok := typeInfo.info.Uses[ident]
	if !ok {
		return false
	}
	builtin, ok := obj.(*types.Builtin)
	return ok && builtin.Name() == name
}

func localMakeSliceTypeAnnotation(rhs ast.Expr) (string, bool) {
	call, ok := rhs.(*ast.CallExpr)
	if !ok || len(call.Args) < 2 || !isBuiltinCallNamed(call, "make") {
		return "", false
	}
	arrayType, ok := call.Args[0].(*ast.ArrayType)
	if !ok || arrayType.Len != nil {
		return "", false
	}
	if structElt, ok := arrayType.Elt.(*ast.StructType); ok {
		generateAnonymousStructType(structElt)
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", false
	}
	typ := typeInfo.GetType(call)
	if typ == nil {
		typ = typeInfo.GetType(call.Args[0])
	}
	if typ == nil {
		return "", false
	}
	if typeContainsTypeParam(typ) {
		return "", false
	}
	sliceType, ok := types.Unalias(typ).Underlying().(*types.Slice)
	if !ok {
		return "", false
	}
	elem := sliceType.Elem()
	if zeroValueForTypesType(elem) != "Default::default()" && !isGoErrorType(elem) && !isPointerTypeOrUnderlying(elem) {
		return "", false
	}
	return goTypesTypeToRustWrapped(typ), true
}

func isPointerTypeOrUnderlying(typ types.Type) bool {
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	return ok
}

func typeContainsTypeParam(typ types.Type) bool {
	if typ == nil {
		return false
	}
	switch t := types.Unalias(typ).(type) {
	case *types.TypeParam:
		return true
	case *types.Named:
		if args := t.TypeArgs(); args != nil {
			for i := 0; i < args.Len(); i++ {
				if typeContainsTypeParam(args.At(i)) {
					return true
				}
			}
		}
		return false
	}
	switch t := types.Unalias(typ).Underlying().(type) {
	case *types.Slice:
		return typeContainsTypeParam(t.Elem())
	case *types.Array:
		return typeContainsTypeParam(t.Elem())
	case *types.Pointer:
		return typeContainsTypeParam(t.Elem())
	case *types.Map:
		return typeContainsTypeParam(t.Key()) || typeContainsTypeParam(t.Elem())
	}
	return false
}

func isAssignmentSelfWrappingExpression(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.CompositeLit:
		return isCompositeLitSelfWrapping(e)
	case *ast.SliceExpr:
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			return typeInfo.ReturnsWrappedValue(e)
		}
		return true
	case *ast.CallExpr:
		return isBuiltinCallNamed(e, "make") && !isMakeChannelCall(e)
	case *ast.IndexExpr:
		return mapIndexExpressionKeepsHandle(e)
	default:
		return false
	}
}

func isTypedAssignmentSelfWrappingExpression(expr ast.Expr) bool {
	if lit, ok := expr.(*ast.CompositeLit); ok && lit.Type == nil {
		return false
	}
	return isAssignmentSelfWrappingExpression(expr)
}

// writeDefinedMapFromElementHandle writes the initializer of var c Counts =
// s[0], where Counts is a defined map type and s a slice or array of its
// unnamed map type: the newtype around the element's handle, so c and the
// element share one map. It reports whether it wrote it.
func writeDefinedMapFromElementHandle(out *strings.Builder, declType ast.Expr, value ast.Expr) bool {
	typeInfo := GetTypeInfo()
	index, ok := unwrapParens(value).(*ast.IndexExpr)
	if declType == nil || typeInfo == nil || !ok || !mapElementIndexExpression(index) {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(declType)).(*types.Named)
	if !ok || !types.Identical(named.Underlying(), typeInfo.GetType(value)) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(goTypesTypeToRust(named))
	out.WriteString("(")
	TranspileExpression(out, value)
	out.WriteString(")")
	WriteWrapperSuffix(out)
	return true
}

// varDeclConvertsToOtherType reports whether var x T = value declares x of a
// type T other than value's own, as var c Counts = s[0] for a []map: the
// value is then converted to T, not taken as the handle it is.
func varDeclConvertsToOtherType(declType ast.Expr, value ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if declType == nil || typeInfo == nil {
		return false
	}
	declared, valueType := typeInfo.GetType(declType), typeInfo.GetType(value)
	return declared != nil && valueType != nil && !types.Identical(declared, valueType)
}

// mapIndexExpressionKeepsHandle reports whether expr reads a handle out of
// a collection: a map value that is a map, slice, pointer or the like, or an
// element of an array or slice of an unnamed map type, which is the map's
// handle (#318). A declaration, assignment or return takes the handle as it
// is.
func mapIndexExpressionKeepsHandle(expr ast.Expr) bool {
	indexExpr, ok := unwrapParens(expr).(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if typeInfo.IsMap(indexExpr.X) {
		return mapValueTypeKeepsHandle(typeInfo.GetType(indexExpr))
	}
	return mapElementIndexExpression(indexExpr)
}

// mapElementIndexExpression reports whether expr reads an element of an
// array or slice of an unnamed map type, which is the map's handle (#318).
func mapElementIndexExpression(expr *ast.IndexExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.IsMap(expr.X) {
		return false
	}
	elemType, ok := sequenceElementTypeForIndexExpr(expr.X, typeInfo)
	return ok && isUnnamedMapType(elemType)
}

func isWrappedInterfaceSliceIndex(expr ast.Expr) bool {
	indexExpr, ok := expr.(*ast.IndexExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if typeInfo.IsMap(indexExpr.X) {
		return false
	}
	_, ok = transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(indexExpr))
	return ok
}

// goTypeIsCopiedByValue reports whether assigning a value of this type gives
// the destination a value of its own: a string, a number, a bool, a struct
// or an array, or a defined type over one of those. Every place that decides
// between copying a value and sharing its handle asks here, so that they
// cannot disagree.
func goTypeIsCopiedByValue(typ types.Type) bool {
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Basic, *types.Struct, *types.Array:
		return true
	}
	return false
}

// receiverFieldIsReturnedByCopy reports whether a field of the receiver is
// a value that Go copies when a method returns it. The caller's variable is
// then its own. Returning the field's handle would make the caller's
// variable and the field one variable, so that "name := u.Name(); name =
// strings.ToUpper(name)" changed the struct.
//
// known is false when the type checker has no type for the field. The
// caller must not guess then: a wrong guess is a silent aliasing bug.
func receiverFieldIsReturnedByCopy(expr ast.Expr) (copied bool, known bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false, false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false, false
	}
	return goTypeIsCopiedByValue(typ), true
}

func receiverFieldTypeIsKnown(expr ast.Expr) bool {
	_, known := receiverFieldIsReturnedByCopy(expr)
	return known
}

func receiverFieldIsCopied(expr ast.Expr) bool {
	copied, _ := receiverFieldIsReturnedByCopy(expr)
	return copied
}

func selectorExpressionKeepsHandle(expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if typ := typeInfo.GetType(expr); typ != nil {
			return mapValueTypeKeepsHandle(typ)
		}
	}
	fieldExpr, ok := selectorFieldTypeExpr(sel)
	return ok && fieldExprKeepsHandle(fieldExpr)
}

func assignmentTargetIsEmptyInterface(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if typ := typeInfo.GetType(expr); typ != nil {
			return isEmptyInterfaceType(typ)
		}
	}
	return isEmptyInterfaceValueExpr(expr)
}

func writeEmptyInterfaceAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	if !assignmentTargetIsEmptyInterface(lhs) {
		return false
	}
	if writeEmptyInterfaceIdentAssignment(out, lhs, rhs) {
		return true
	}
	out.WriteString("{ ")
	out.WriteString("let new_val = ")
	writeInterfaceBoxedValue(out, rhs)
	out.WriteString("; ")
	out.WriteString("*")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(new_val); }")
	return true
}

func selectorFieldTypeExpr(sel *ast.SelectorExpr) (ast.Expr, bool) {
	typeName, ok := selectorBaseSyntaxTypeName(sel.X)
	if !ok {
		return nil, false
	}
	structDef := structDefs[typeName]
	if structDef == nil {
		structDef = structDefs[strings.TrimPrefix(typeName, "*")]
	}
	if structDef == nil {
		return nil, false
	}
	fieldExpr := structDefFieldTypeExpr(structDef, sel.Sel.Name)
	return fieldExpr, fieldExpr != nil
}

func structDefFieldTypeExpr(structDef *StructDef, fieldName string) ast.Expr {
	if structDef == nil {
		return nil
	}
	if len(structDef.FieldTypes) > 0 {
		if fieldExpr := structDef.FieldTypes[fieldName]; fieldExpr != nil {
			return fieldExpr
		}
		for name, fieldExpr := range structDef.FieldTypes {
			if fieldExpr != nil && ToSnakeCase(name) == ToSnakeCase(fieldName) {
				return fieldExpr
			}
		}
	}
	return findStructFieldExpr(structDef.ASTType, fieldName)
}

func selectorBaseSyntaxTypeName(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return "", false
	}
	if isCurrentReceiverIdent(ident) && currentReceiverType != "" {
		return currentReceiverType, true
	}
	if vt := GetVarTable(); vt != nil {
		if info := vt.Lookup(ident.Name); info != nil && info.RustType != "" {
			return unwrapStoredRustType(info.RustType), true
		}
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if typeName, ok := localNamedTypeNameFromGoType(typeInfo.GetType(expr)); ok {
			return typeName, true
		}
	}
	return "", false
}

func selectorReceiverTypeKnown(sel *ast.SelectorExpr) bool {
	if sel == nil {
		return false
	}
	_, ok := selectorBaseSyntaxTypeName(sel.X)
	return ok
}

func localNamedTypeNameFromGoType(typ types.Type) (string, bool) {
	if typ == nil {
		return "", false
	}
	typ = types.Unalias(typ)
	if ptr, ok := typ.(*types.Pointer); ok {
		return localNamedTypeNameFromGoType(ptr.Elem())
	}
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil {
		return "", false
	}
	return named.Obj().Name(), true
}

func expressionFunctionSignature(expr ast.Expr) (*types.Signature, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return nil, false
	}
	sig, ok := typ.Underlying().(*types.Signature)
	return sig, ok
}

func funcLitAssignmentTargetNeedsClone(lhs ast.Expr, funcLit *ast.FuncLit) bool {
	if lhs == nil || funcLit == nil {
		return false
	}
	captured := capturedVarsForFuncLit(funcLit)
	if len(captured) == 0 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}

	needsClone := false
	var inspect func(ast.Node)
	inspect = func(node ast.Node) {
		if node == nil || needsClone {
			return
		}
		ast.Inspect(node, func(n ast.Node) bool {
			if n == nil || needsClone {
				return false
			}
			switch node := n.(type) {
			case *ast.SelectorExpr:
				inspect(node.X)
				return false
			case *ast.Ident:
				if !captured[node.Name] {
					return true
				}
				obj, ok := typeInfo.GetObject(node).(*types.Var)
				if ok && !isPackageScopeObject(obj) {
					needsClone = true
					return false
				}
			}
			return true
		})
	}
	inspect(lhs)
	return needsClone
}

func functionTypeRustNameFromTypeExpr(expr ast.Expr) (string, bool) {
	switch t := expr.(type) {
	case *ast.FuncType:
		return generateClosureType(t), true
	case *ast.Ident:
		if rustType, ok := FunctionTypeAliasBox(t.Name); ok {
			return rustType, true
		}
		if IsFunctionTypeAlias(t.Name) {
			return RustTypeNameForUse(t.Name), true
		}
		if underlying, isTypeDef := LookupTypeDefinition(t.Name); isTypeDef && underlying == "func" {
			return RustTypeNameForUse(t.Name), true
		}
	}
	return "", false
}

func expressionHasFunctionSignatureSyntax(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return false
	}
	info := lookupVarInfo(ident.Name)
	if info == nil {
		return false
	}
	rustType := strings.TrimPrefix(info.RustType, "&")
	return isFunctionValueRustType(rustType)
}

func isFunctionValueRustType(rustType string) bool {
	rustType = strings.TrimPrefix(rustType, "&")
	switch rustType {
	case "GoCancelFunc", "GoCancelCauseFunc":
		return true
	}
	return isFuncValueRustType(rustType) || IsFunctionTypeAlias(rustType)
}

func fieldTypeExprCanBeFunctionValue(fieldExpr ast.Expr) bool {
	if isFunctionSignatureTypeExpr(fieldExpr) {
		return true
	}
	if _, ok := functionTypeRustNameFromTypeExpr(fieldExpr); ok {
		return true
	}
	_, ok := namedFieldTypeFallbackFunctionRustName(fieldExpr)
	return ok
}

func uniqueFunctionStructFieldTypeExpr(fieldName string) (ast.Expr, bool) {
	var found ast.Expr
	for _, def := range structDefs {
		if def == nil || def.ASTType == nil {
			continue
		}
		fieldExpr := structDefFieldTypeExpr(def, fieldName)
		if fieldExpr == nil {
			continue
		}
		if !fieldTypeExprCanBeFunctionValue(fieldExpr) {
			continue
		}
		if found != nil {
			return nil, false
		}
		found = fieldExpr
	}
	return found, found != nil
}

func callSingleReturnTypeExpr(call *ast.CallExpr) ast.Expr {
	if call == nil {
		return nil
	}
	ident, ok := call.Fun.(*ast.Ident)
	if !ok {
		return nil
	}
	sig := GetFunctionSignature(ident.Name)
	if sig == nil || len(sig.Results) != 1 {
		return nil
	}
	return sig.Results[0].Type
}

func functionSignatureFromTypeExpr(expr ast.Expr) (*types.Signature, bool) {
	if _, ok := expr.(*ast.FuncType); ok {
		return nil, true
	}
	typ := expectedTypeFromParamExpr(expr)
	if typ == nil {
		return nil, false
	}
	sig, ok := types.Unalias(typ).Underlying().(*types.Signature)
	return sig, ok
}

func functionTypeExprUsesWrappedHandle(expr ast.Expr) (bool, bool) {
	if expr == nil {
		return false, true
	}
	if _, ok := expr.(*ast.FuncType); ok {
		return false, true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false, false
	}
	typ := expectedTypeFromParamExpr(expr)
	if typ == nil {
		return false, false
	}
	_, ok := signatureFromType(typ)
	return ok, true
}

// writeUnnamedSliceToNamedSliceAssignment writes "b = raw" or
// "b = []byte("x")" for a variable of type Bytes []byte: the slice goes
// into the defined type.
func writeUnnamedSliceToNamedSliceAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	var value strings.Builder
	if !writeUnnamedSliceToNamedSliceInitializer(&value, rhs, typeInfo.GetType(lhs)) {
		return false
	}
	out.WriteString("{ let new_val = ")
	out.WriteString(value.String())
	out.WriteString("; let __moved_val = { let mut __guard = new_val")
	WriteBorrowMethod(out, true)
	out.WriteString("; __guard.take() }; *")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = __moved_val; }")
	return true
}

func writeMoveWrappedInnerAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) {
	if writeCurrentReceiverLocalAssignment(out, lhs, rhs) {
		return
	}
	if writeUnnamedSliceToNamedSliceAssignment(out, lhs, rhs) {
		return
	}
	out.WriteString("{ ")
	out.WriteString("let new_val = ")
	TranspileExpression(out, rhs)
	out.WriteString("; let __moved_val = { let mut __guard = new_val")
	WriteBorrowMethod(out, true)
	out.WriteString("; __guard.take() }; ")
	out.WriteString("*")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = __moved_val; }")
}

func writeCapturedCurrentReceiverAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	ident, ok := lhs.(*ast.Ident)
	if !ok || !isCurrentReceiverIdent(ident) || currentCaptureRenames == nil {
		return false
	}
	renamed, ok := currentCaptureRenames[ident.Name]
	if !ok || renamed == "" || renamed == ident.Name {
		return false
	}
	target := RustLocalIdent(renamed)
	typeInfo := GetTypeInfo()
	out.WriteString("{ ")
	call, isCall := rhs.(*ast.CallExpr)
	if typeInfo != nil && typeInfo.ReturnsWrappedValue(rhs) && (!isCall || !isBareBuiltinReturn(call)) {
		out.WriteString("let new_val = ")
		TranspileExpression(out, rhs)
		out.WriteString("; let __moved_val = { let mut __guard = new_val")
		WriteBorrowMethod(out, true)
		out.WriteString("; __guard.take().unwrap() }; ")
		out.WriteString(target)
		out.WriteString(" = __moved_val; }")
		return true
	}
	out.WriteString(target)
	out.WriteString(" = ")
	TranspileExpression(out, rhs)
	out.WriteString("; }")
	return true
}

func writeMoveWrappedInnerAssignmentFromTemp(out *strings.Builder, lhs ast.Expr, tmpName string) {
	if ident, ok := lhs.(*ast.Ident); ok && ident.Name == "_" {
		return
	}
	if indexExpr, ok := lhs.(*ast.IndexExpr); ok && writeIndexedSequenceAssignmentFromTemp(out, indexExpr, tmpName, true, nil) {
		return
	}
	if writePointerHandleAssignmentFromTemp(out, lhs, tmpName) {
		return
	}
	if writeCurrentReceiverDerefAssignmentFromTemp(out, lhs, tmpName) {
		return
	}
	movedName := "__moved_" + strings.TrimLeft(tmpName, "_")
	out.WriteString(" let ")
	out.WriteString(movedName)
	out.WriteString(" = { let mut __guard = ")
	out.WriteString(tmpName)
	WriteBorrowMethod(out, true)
	out.WriteString("; __guard.take() };")
	out.WriteString(" *")
	writeTempAssignmentTargetRef(out, lhs)
	out.WriteString(" = ")
	out.WriteString(movedName)
	out.WriteString(";")
}

// writeCurrentReceiverDerefAssignmentFromTemp writes *t = <temp> for the
// receiver t of a pointer method, as in "*t, err = parse(data)": the
// receiver is &mut T, so the value goes into *self, as a plain "*t = v"
// does in writeCurrentReceiverDerefAssignment.
func writeCurrentReceiverDerefAssignmentFromTemp(out *strings.Builder, lhs ast.Expr, tmpName string) bool {
	star, ok := lhs.(*ast.StarExpr)
	if !ok {
		return false
	}
	ident, ok := star.X.(*ast.Ident)
	if !ok || !isCurrentReceiverIdent(ident) || isNamedSliceExpression(star) || isNamedMapExpression(star) {
		return false
	}
	out.WriteString(" *self = { let mut __guard = ")
	out.WriteString(tmpName)
	WriteBorrowMethod(out, true)
	out.WriteString("; __guard.take().unwrap_or_default() };")
	return true
}

func writeTupleAssignmentFromTemp(out *strings.Builder, lhs ast.Expr, tmpName string, tmpBareScalar bool) {
	if ident, ok := lhs.(*ast.Ident); ok && ident.Name == "_" {
		return
	}
	if ident, ok := lhs.(*ast.Ident); ok && isArrayElemPtrVar(ident.Name) {
		out.WriteString(" ")
		out.WriteString(rustIdentForUseWithCapture(ident))
		out.WriteString(" = ")
		out.WriteString(tmpName)
		out.WriteString(".clone();")
		return
	}
	if indexExpr, ok := lhs.(*ast.IndexExpr); ok && writeIndexedSequenceAssignmentFromTemp(out, indexExpr, tmpName, !tmpBareScalar, nil) {
		return
	}
	if writeParallelCurrentReceiverAssignmentTarget(out, lhs, tmpName, !tmpBareScalar) {
		return
	}
	if ident, ok := lhs.(*ast.Ident); ok && isVarBare(ident.Name) {
		out.WriteString(" ")
		out.WriteString(rustIdentForUseWithCapture(ident))
		out.WriteString(" = ")
		if tupleTempNeedsUnwrapForBareAssignment(lhs, tmpBareScalar) {
			writeClonedTupleTempInner(out, tmpName)
		} else {
			out.WriteString(tmpName)
		}
		out.WriteString(";")
		return
	}
	// A package-level map variable takes the handle of the map the call
	// returned (#328).
	if ident, ok := lhs.(*ast.Ident); ok && !tmpBareScalar && packageGlobalIdentIsMapHandle(ident) && !isNamedMapExpression(lhs) {
		out.WriteString(" ")
		out.WriteString(rustPackageGlobalName(ident.Name))
		out.WriteString(".set(")
		out.WriteString(tmpName)
		out.WriteString(");")
		return
	}
	if tmpBareScalar {
		out.WriteString(" *")
		writeTempAssignmentTargetRef(out, lhs)
		out.WriteString(" = Some(")
		out.WriteString(tmpName)
		out.WriteString(");")
		return
	}
	writeMoveWrappedInnerAssignmentFromTemp(out, lhs, tmpName)
}

func writeMultilineTupleReassignment(out *strings.Builder, s *ast.AssignStmt) bool {
	if s == nil || len(s.Rhs) != 1 || !tupleReassignmentCallShouldUseMultiline(s.Rhs[0]) {
		return false
	}
	call, ok := unwrapParens(s.Rhs[0]).(*ast.CallExpr)
	if !ok {
		return false
	}
	indent := currentLineIndent(out)
	out.WriteString("{\n")
	out.WriteString(indent)
	out.WriteString("    let (")
	for i := range s.Lhs {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(fmt.Sprintf("__tmp_%d", i))
	}
	out.WriteString(") = ")
	TranspileExpression(out, s.Rhs[0])
	out.WriteString(";\n")
	for i, lhs := range s.Lhs {
		var line strings.Builder
		tmpName := fmt.Sprintf("__tmp_%d", i)
		if !writeTupleSliceElemPtrFieldAssignmentFromTemp(&line, lhs, tmpName, call, i) {
			if !writeTupleSliceElemPtrVarAssignmentFromTemp(&line, lhs, tmpName, call, i) {
				if !writeTupleGoPtrAssignmentFromTemp(&line, lhs, tmpName, call, i) {
					writeTupleAssignmentFromTemp(&line, lhs, tmpName, callResultIsBareScalar(call, i))
				}
			}
		}
		writeIndentedTupleAssignmentText(out, line.String(), indent+"    ")
	}
	out.WriteString(indent)
	out.WriteString("}")
	return true
}

func tupleReassignmentCallShouldUseMultiline(rhs ast.Expr) bool {
	if !NeedsConcurrentWrapper() {
		return false
	}
	call, ok := unwrapParens(rhs).(*ast.CallExpr)
	if !ok {
		return false
	}
	return methodCallReceiverIsTypedFieldSelector(call)
}

func methodCallReceiverIsTypedFieldSelector(call *ast.CallExpr) bool {
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	receiverSel, ok := unwrapParens(sel.X).(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	selection, ok := typeInfo.info.Selections[receiverSel]
	return ok && selection.Kind() == types.FieldVal
}

func writeIndentedTupleAssignmentText(out *strings.Builder, text string, indent string) {
	text = strings.TrimSpace(text)
	if text == "" {
		return
	}
	out.WriteString(indent)
	out.WriteString(text)
	out.WriteString("\n")
}

func tupleTempNeedsUnwrapForBareAssignment(lhs ast.Expr, tmpBareScalar bool) bool {
	if tmpBareScalar {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if typeInfo.IsString(lhs) {
		return true
	}
	return typeIsPredeclaredCopyScalar(typeInfo.GetType(lhs))
}

func writeClonedTupleTempInner(out *strings.Builder, tmpName string) {
	out.WriteString("{ let __tmp_holder = ")
	out.WriteString(tmpName)
	out.WriteString(".clone(); let __tmp_guard = __tmp_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__tmp_guard.as_ref().unwrap()).clone() }")
}

func writeTempAssignmentTargetRef(out *strings.Builder, lhs ast.Expr) {
	if star, ok := lhs.(*ast.StarExpr); ok {
		TranspileExpressionContext(out, star.X, LValue)
		WriteBorrowMethod(out, true)
		return
	}
	if ident, ok := lhs.(*ast.Ident); ok {
		out.WriteString(rustIdentForUseWithCapture(ident))
		WriteBorrowMethod(out, true)
		return
	}
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
}

func writeIndexedSequenceAssignmentFromTemp(out *strings.Builder, indexExpr *ast.IndexExpr, tmpName string, tmpWrapped bool, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	var elemType types.Type
	if typeInfo != nil {
		if typeInfo.IsMap(indexExpr.X) {
			return false
		}
		elemType = typeInfo.GetArrayOrSliceElemType(indexExpr.X)
	} else {
		out.WriteString(" /* ERROR: Cannot determine indexed assignment target - type information required */ unimplemented!(\"type info required for indexed assignment\")")
		return true
	}
	elemKeepsHandle := indexedSequenceElementKeepsHandle(elemType)
	if !elemKeepsHandle {
		elemKeepsHandle = tupleTempAssignsHandleToElementBySyntax(indexExpr.X)
	}
	if writeGoPtrPointedArrayAssignmentFromTemp(out, indexExpr, tmpName, tmpWrapped, rhs, elemKeepsHandle) {
		return true
	}
	// The index is evaluated before the sequence is locked for writing:
	// "s[len(s)-1], x = a, b" reads the sequence in its index. A sequence
	// that calls a function is evaluated before its index, as in Go: its
	// handle is taken first and held.
	holdsSequence := expressionCallsOrReceives(indexExpr.X)
	writeSequenceHandle := func() {
		if subj := unwrapParens(indexExpr.X); isNamedSliceExpression(subj) {
			writeNamedSliceInnerHandleClone(out, subj)
		} else {
			TranspileExpressionContext(out, indexExpr.X, LValue)
		}
	}
	out.WriteString(" { ")
	if holdsSequence {
		out.WriteString("let __seq = ")
		writeSequenceHandle()
		out.WriteString(".clone(); ")
	}
	out.WriteString("let __idx = ")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("; (*")
	if holdsSequence {
		out.WriteString("__seq")
	} else {
		writeSequenceHandle()
	}
	WriteBorrowMethod(out, true)
	out.WriteString(".as_mut().unwrap())[__idx] = ")
	writeIndexedSequenceTempAssignmentValue(out, indexExpr, tmpName, tmpWrapped, rhs, elemKeepsHandle)
	out.WriteString("; }")
	return true
}

func writeGoPtrPointedArrayAssignmentFromTemp(out *strings.Builder, indexExpr *ast.IndexExpr, tmpName string, tmpWrapped bool, rhs ast.Expr, elemKeepsHandle bool) bool {
	ident, ok := unwrapParens(indexExpr.X).(*ast.Ident)
	if !ok {
		return false
	}
	if _, ok := goPtrIdentPointedArrayType(ident); !ok {
		return false
	}
	out.WriteString(" ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".with_mut(|__seq| { __seq[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("] = ")
	writeIndexedSequenceTempAssignmentValue(out, indexExpr, tmpName, tmpWrapped, rhs, elemKeepsHandle)
	out.WriteString("; });")
	return true
}

func writeIndexedSequenceTempAssignmentValue(out *strings.Builder, indexExpr *ast.IndexExpr, tmpName string, tmpWrapped bool, rhs ast.Expr, elemKeepsHandle bool) {
	if elemKeepsHandle {
		out.WriteString(tmpName)
	} else if tmpWrapped {
		out.WriteString(tmpName)
		WriteBorrowMethod(out, true)
		out.WriteString(".take().unwrap_or_default()")
	} else if rhs != nil {
		writeIndexedSequenceBareTempValue(out, indexExpr, tmpName, rhs)
	} else {
		out.WriteString(tmpName)
	}
}

func writeIndexedSequenceBareTempValue(out *strings.Builder, indexExpr *ast.IndexExpr, tmpName string, rhs ast.Expr) {
	if wrapper := parallelTempNamedIntegerWrap(indexExpr, rhs); wrapper != "" {
		out.WriteString(wrapper)
		out.WriteString("(")
		WriteWrapperPrefix(out)
		out.WriteString(tmpName)
		if cast := parallelTempNamedIntegerCast(indexExpr); cast != "" {
			out.WriteString(" as ")
			out.WriteString(cast)
		}
		WriteWrapperSuffix(out)
		out.WriteString(")")
		return
	}
	out.WriteString(tmpName)
	if cast := parallelTempBareCast(indexExpr, rhs); cast != "" {
		out.WriteString(" as ")
		out.WriteString(cast)
	}
}

func indexedSequenceElementKeepsHandle(elemType types.Type) bool {
	if elemType == nil {
		return false
	}
	if _, ok := types.Unalias(elemType).Underlying().(*types.Pointer); ok {
		return true
	}
	return rustMapValueTypeKeepsHandle(goTypesCollectionElemTypeToRust(elemType))
}

func tupleTempAssignsHandleToElementBySyntax(expr ast.Expr) bool {
	elemType, ok := localCollectionElemRustType(expr)
	return ok && rustMapValueTypeKeepsHandle(elemType)
}

func tupleTempAssignsHandleToElement(elemType types.Type) bool {
	if elemType == nil {
		return false
	}
	if isGoErrorType(elemType) {
		return true
	}
	switch types.Unalias(elemType).Underlying().(type) {
	case *types.Pointer, *types.Slice, *types.Map, *types.Chan, *types.Signature, *types.Interface:
		return true
	default:
		return false
	}
}

func writeBareRangeVarAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	ident, ok := lhs.(*ast.Ident)
	if !ok || ident.Name == "_" {
		return false
	}
	varType, ok := rangeLoopVars[ident.Name]
	if !ok || varType == "ref_value" || strings.HasPrefix(varType, "&") || isWrappedRangeVarType(varType) {
		return false
	}

	var expected types.Type
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		expected = typeInfo.GetType(lhs)
		if expected == nil {
			if ident, ok := lhs.(*ast.Ident); ok {
				if obj := typeInfo.GetObject(ident); obj != nil {
					expected = obj.Type()
				}
			}
		}
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(expected); ok {
		return false
	}
	if isEmptyInterfaceType(expected) && !isEmptyInterfaceValueExpr(rhs) {
		out.WriteString("{ let new_val = ")
		writeBareAnyBox(out, rhs)
		out.WriteString("; ")
		out.WriteString(RustIdentForUse(ident))
		out.WriteString(" = new_val; }")
		return true
	}

	out.WriteString("{ let new_val = ")
	writeBareCompoundAssignValue(out, rhs, expected)
	out.WriteString("; ")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" = new_val; }")
	return true
}

func writePointerHandleAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(lhs) {
		return false
	}
	rhsIsPointer := typeInfo.IsPointer(rhs)
	if !rhsIsPointer {
		if ident, ok := rhs.(*ast.Ident); ok {
			rhsIsPointer = rangeLoopVarGoTypeIsPointer(ident.Name)
		}
	}
	if !rhsIsPointer {
		return false
	}
	if ident, ok := lhs.(*ast.Ident); ok && isCurrentReceiverIdent(ident) && currentReceiverRustAlias != "" {
		return false
	}
	if _, ok := lhs.(*ast.SelectorExpr); ok {
		var unsupported strings.Builder
		if writeUnsupportedSliceElemPointerHandleValue(&unsupported, rhs, "slice element pointer cannot assign to pointer field") {
			out.WriteString("{ let new_val = ")
			out.WriteString(unsupported.String())
			out.WriteString("; ")
			writePointerHandleAssignmentTargetFromValueName(out, lhs, "new_val")
			out.WriteString(" }")
			return true
		}
	}
	if ident, ok := packageGlobalPointerIdent(lhs); ok {
		out.WriteString("{ let new_val = ")
		writePointerHandleValueClone(out, rhs)
		out.WriteString("; *")
		out.WriteString(rustPackageGlobalName(ident.Name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = Some(new_val); }")
		return true
	}
	out.WriteString("{ let new_val = ")
	writePointerHandleValueClone(out, rhs)
	out.WriteString("; ")
	writePointerHandleAssignmentTargetFromValueName(out, lhs, "new_val")
	out.WriteString(" }")
	return true
}

// writePointerNilRebind writes `p = nil` for a pointer variable or field.
// A pointer is the handle of its pointee's cell, and a nil pointer is a
// handle whose cell is empty. Writing None into the cell p holds emptied
// the pointee for every other pointer to it: after `q := p; q = nil`, p
// was nil too, and `item = nil` in a range loop cleared the slice element
// (DEV-142). Like any other pointer assignment, nil now replaces the
// handle, with a fresh empty one.
func writePointerNilRebind(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	if ident, ok := rhs.(*ast.Ident); !ok || ident.Name != "nil" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(lhs) {
		return false
	}
	lhsType := typeInfo.GetType(lhs)
	if lhsType == nil {
		return false
	}
	if _, ok := types.Unalias(lhsType).Underlying().(*types.Pointer); !ok {
		return false
	}
	switch target := unwrapParens(lhs).(type) {
	case *ast.Ident:
		if target.Name == "_" || isCurrentReceiverIdent(target) {
			return false
		}
		if _, isGlobal := packageGlobalPointerIdent(target); isGlobal {
			return false
		}
	case *ast.SelectorExpr:
		if selection, ok := typeInfo.info.Selections[target]; !ok || selection.Kind() != types.FieldVal {
			return false
		}
	default:
		return false
	}
	nilHandle := zeroValueForTypesType(lhsType)
	if nilHandle == "" {
		return false
	}
	out.WriteString("{ let new_val = ")
	out.WriteString(nilHandle)
	out.WriteString("; ")
	writePointerHandleAssignmentTargetFromValueName(out, lhs, "new_val")
	out.WriteString(" }")
	return true
}

// typedPointerVarTakesPointerHandle reports whether "var p *T = value"
// copies value's pointer handle into p. That is so when T is a pointer type
// and value is a pointer variable, field, element or package variable such as
// os.Stdout: wrapping it again made a pointer to a pointer, which does not
// compile, and copied the pointee instead of sharing it.
func typedPointerVarTakesPointerHandle(typeExpr ast.Expr, value ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || typeExpr == nil || value == nil {
		return false
	}
	declared := typeInfo.info.TypeOf(typeExpr)
	if declared == nil {
		return false
	}
	if _, ok := types.Unalias(declared).Underlying().(*types.Pointer); !ok {
		return false
	}
	switch v := unwrapParens(value).(type) {
	case *ast.Ident:
		if v.Name == "nil" {
			return false
		}
		if rangeLoopVarGoTypeIsPointer(v.Name) {
			return true
		}
	case *ast.SelectorExpr:
	case *ast.IndexExpr:
		if _, isMap := types.Unalias(typeInfo.info.TypeOf(v.X)).Underlying().(*types.Map); isMap {
			return false
		}
	default:
		return false
	}
	return typeInfo.IsPointer(value)
}

func writePointerHandleValueClone(out *strings.Builder, rhs ast.Expr) {
	if writeCurrentPointerReceiverHandleClone(out, rhs) {
		return
	}
	if ident, ok := rhs.(*ast.Ident); ok && rangeLoopVarGoTypeIsPointer(ident.Name) {
		varType := rangeLoopVars[ident.Name]
		if strings.HasPrefix(varType, "&") {
			out.WriteString("(*")
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(").clone()")
			return
		}
	}
	if ident, ok := packageGlobalPointerIdent(rhs); ok {
		writePackageGlobalPointerHandleClone(out, ident)
		return
	}
	if writeSourceMappedPackageGlobalPointerHandleClone(out, rhs) {
		return
	}
	if sel, ok := unwrapParens(rhs).(*ast.SelectorExpr); ok {
		if writeGoPtrSelectorReadHandle(out, sel) {
			return
		}
	}
	TranspileExpressionContext(out, rhs, AddressOf)
	out.WriteString(".clone()")
}

func writeTypeParamWrappedIdentValueClone(out *strings.Builder, ident *ast.Ident, rustVarName string) bool {
	if ident == nil || rustVarName == "" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(ident)
	if typ == nil {
		out.WriteString(`/* ERROR: Type information required for type-parameter value clone */ unimplemented!("type info required for type-parameter value clone")`)
		return true
	}
	if !isDirectTypeParamType(typ) {
		return false
	}
	NeedGoValueClone()
	out.WriteString("(*")
	out.WriteString(rustVarName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).go_value_clone()")
	return true
}

func writeCurrentPointerReceiverHandleClone(out *strings.Builder, rhs ast.Expr) bool {
	ident, ok := unwrapParens(rhs).(*ast.Ident)
	if !ok || !isCurrentReceiverIdent(ident) || currentReceiverObject == nil {
		return false
	}
	if _, isPtr := types.Unalias(currentReceiverObject.Type()).(*types.Pointer); !isPtr {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(currentReceiverRustName())
	out.WriteString(".clone()")
	WriteWrapperSuffix(out)
	return true
}

func writePackageGlobalPointerNilAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	rhsIdent, ok := rhs.(*ast.Ident)
	if !ok || rhsIdent.Name != "nil" {
		return false
	}
	lhsIdent, ok := packageGlobalPointerIdent(lhs)
	if !ok {
		return false
	}
	var lhsType types.Type
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		lhsType = typeInfo.GetType(lhs)
	}
	out.WriteString("*")
	out.WriteString(rustPackageGlobalName(lhsIdent.Name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(")
	out.WriteString(zeroValueForTypesType(lhsType))
	out.WriteString(")")
	return true
}

func writePointerHandleAssignmentTarget(out *strings.Builder, lhs ast.Expr) {
	if sel, ok := lhs.(*ast.SelectorExpr); ok && writePointerHandleSelectorTarget(out, sel) {
		return
	}
	TranspileExpressionContext(out, lhs, LValue)
}

func writePointerHandleAssignmentTargetFromValueName(out *strings.Builder, lhs ast.Expr, valueName string) {
	if sel, ok := unwrapParens(lhs).(*ast.SelectorExpr); ok {
		if writeGoPtrLocalSelectorHandleReplacement(out, sel, valueName) {
			return
		}
		if writeGoPtrFieldSelectorHandleReplacement(out, sel, valueName) {
			return
		}
	}
	writePointerHandleAssignmentTarget(out, lhs)
	out.WriteString(" = ")
	out.WriteString(valueName)
	out.WriteString(";")
}

func writeGoPtrFieldSelectorHandleReplacement(out *strings.Builder, sel *ast.SelectorExpr, valueName string) bool {
	base, ok := unwrapParens(sel.X).(*ast.SelectorExpr)
	if !ok || !generatedGoPtrFieldForSelector(base) {
		return false
	}
	fieldInfo := selectorFieldAccessInfo(sel)
	out.WriteString("{ let __ptr_target = ")
	writeGeneratedGoPtrFieldHandleClone(out, base)
	out.WriteString("; __ptr_target.with_mut(|__ptr_value| { ")
	if fieldInfo.IsPromoted {
		writePromotedHandleAssignmentTarget(out, "__ptr_value", fieldInfo, false)
	} else {
		out.WriteString("__ptr_value.")
		out.WriteString(fieldInfo.FieldName)
	}
	out.WriteString(" = ")
	out.WriteString(valueName)
	out.WriteString("; }); }")
	return true
}

func selectorFieldAccessInfo(sel *ast.SelectorExpr) FieldAccessInfo {
	fieldInfo := FieldAccessInfo{
		FieldName: ToSnakeCase(sel.Sel.Name),
	}
	typeInfo := GetTypeInfo()
	if typedFieldInfo, ok := fieldAccessInfoFromSelection(sel, typeInfo); ok {
		return typedFieldInfo
	}
	if typeInfo == nil {
		return fieldInfo
	}
	if t := typeInfo.GetType(sel.X); t != nil {
		typeStr := t.String()
		if idx := strings.LastIndex(typeStr, "."); idx >= 0 {
			typeStr = typeStr[idx+1:]
		}
		typeStr = strings.TrimPrefix(typeStr, "*")
		fieldInfo = resolveFieldAccess(typeStr, sel.Sel.Name)
	}
	return fieldInfo
}

func writePromotedHandleAssignmentTarget(out *strings.Builder, baseName string, fieldInfo FieldAccessInfo, baseWrapped bool) {
	if baseWrapped {
		out.WriteString("(*(*")
		out.WriteString(baseName)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).")
	} else {
		out.WriteString("(*")
		out.WriteString(baseName)
		out.WriteString(".")
	}
	for i, embedded := range fieldInfo.EmbeddedPath {
		out.WriteString(ToSnakeCase(embedded))
		WriteBorrowMethod(out, true)
		if i < len(fieldInfo.EmbeddedPath)-1 {
			out.WriteString(".as_mut().unwrap().")
		} else {
			out.WriteString(".as_mut().unwrap()).")
		}
	}
	out.WriteString(fieldInfo.FieldName)
}

func writePointerHandleSelectorTarget(out *strings.Builder, sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	fieldInfo := selectorFieldAccessInfo(sel)

	if ident, ok := sel.X.(*ast.Ident); ok {
		if isCurrentReceiverIdent(ident) {
			baseName := currentReceiverRustName()
			fieldInfo := resolveFieldAccess(currentReceiverType, sel.Sel.Name)
			if fieldInfo.IsPromoted {
				writePromotedHandleAssignmentTarget(out, baseName, fieldInfo, false)
				return true
			}
			out.WriteString(baseName)
			out.WriteString(".")
			out.WriteString(fieldInfo.FieldName)
			return true
		}

		needsUnwrap := false
		if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
			needsUnwrap = isWrappedRangeVarType(varType)
			// rangeLoopVars is keyed by name and doesn't track shadowing.
			// A short-decl like `d, ok := d.(*Foo)` shadows the outer range
			// var with a wrapped-pointer Decl; trust go/types for that case.
			if !needsUnwrap && identTypeIsWrappedPointer(ident) {
				needsUnwrap = true
			}
		} else if !isLocalConstantIdent(ident) && !isVarBare(ident.Name) {
			needsUnwrap = true
		}

		baseName := RustIdentForUse(ident)
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[ident.Name]; exists {
				baseName = RustLocalIdent(renamed)
			}
		}

		if isSliceElemPtrVar(ident.Name) {
			out.WriteString("(*")
			writeSliceElemPtrBorrow(out, ident, true)
			out.WriteString(".as_mut().unwrap()).")
			out.WriteString(fieldInfo.FieldName)
			return true
		}
		if isArrayElemPtrVar(ident.Name) {
			out.WriteString("(*")
			writeArrayElemPtrBorrow(out, ident, true)
			out.WriteString(".as_mut().unwrap()).")
			out.WriteString(fieldInfo.FieldName)
			return true
		}
		if isGoPtrVar(ident.Name) {
			if fieldInfo.IsPromoted {
				writeGoPtrLocalPromotedFieldHandle(out, ident, fieldInfo)
			} else {
				writeGoPtrLocalFieldHandle(out, ident, fieldInfo)
			}
			return true
		}
		if globalIdent, ok := packageGlobalPointerIdent(ident); ok && !fieldInfo.IsPromoted {
			writePackageGlobalPointerSelectorAssignmentTarget(out, globalIdent, fieldInfo)
			return true
		}
		if fieldInfo.IsPromoted {
			writePromotedHandleAssignmentTarget(out, baseName, fieldInfo, needsUnwrap)
			return true
		}
		if needsUnwrap {
			out.WriteString("(*")
			out.WriteString(baseName)
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap()).")
			out.WriteString(fieldInfo.FieldName)
		} else {
			out.WriteString(baseName)
			out.WriteString(".")
			out.WriteString(fieldInfo.FieldName)
		}
		return true
	}

	if _, ok := sel.X.(*ast.SelectorExpr); ok && !fieldInfo.IsPromoted {
		out.WriteString("(*")
		TranspileExpressionContext(out, sel.X, LValue)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).")
		out.WriteString(fieldInfo.FieldName)
		return true
	}

	if typeInfo != nil && typeInfo.IsPointer(sel.X) {
		if fieldInfo.IsPromoted {
			out.WriteString("(*")
			out.WriteString("(*")
			TranspileExpressionContext(out, sel.X, LValue)
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap())")
			for i, embedded := range fieldInfo.EmbeddedPath {
				out.WriteString(".")
				out.WriteString(ToSnakeCase(embedded))
				WriteBorrowMethod(out, true)
				if i < len(fieldInfo.EmbeddedPath)-1 {
					out.WriteString(".as_mut().unwrap()")
				} else {
					out.WriteString(".as_mut().unwrap()).")
				}
			}
			out.WriteString(fieldInfo.FieldName)
			return true
		}
		out.WriteString("(*")
		TranspileExpressionContext(out, sel.X, LValue)
		WriteBorrowMethod(out, true)
		out.WriteString(".as_mut().unwrap()).")
		out.WriteString(fieldInfo.FieldName)
		return true
	}

	return false
}

func writePackageGlobalPointerSelectorAssignmentTarget(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo) {
	out.WriteString("(*(*")
	out.WriteString(rustPackageGlobalName(ident.Name))
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
	WriteBorrowMethod(out, true)
	out.WriteString(".as_mut().unwrap()).")
	out.WriteString(fieldInfo.FieldName)
}

// rangeSliceElementIsEmptyInterface reports whether ranging rangeExpr yields
// elements of an empty interface type (any / interface{}), which lower to
// Box<dyn Any> — a non-Clone type the plain .cloned() materialization can't
// snapshot.
func rangeSliceElementIsEmptyInterface(rangeExpr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	t := typeInfo.GetType(rangeExpr)
	if t == nil {
		return false
	}
	slice, ok := types.Unalias(t).Underlying().(*types.Slice)
	if !ok {
		return false
	}
	return isEmptyInterfaceType(slice.Elem())
}

// writeRangeValuesMaterialization emits the `let __range_values = ...;
// drop(__range_guard);` slice-range snapshot. A []any slice holds Box<dyn Any>
// elements, which aren't Clone. When the loop writes each value back to its
// indexed slot, move the element out of the slice. Otherwise retain the
// original length and reread each element when its iteration begins, matching
// Go slice-range semantics when the loop mutates a later backing-array slot.
func writeRangeValuesMaterialization(out *strings.Builder, rangeExpr ast.Expr, moveEmptyInterfaceValues bool) {
	if rangeSliceElementIsEmptyInterface(rangeExpr) {
		if moveEmptyInterfaceValues {
			out.WriteString("; let __range_values = { let __range_len = __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); std::mem::drop(__range_guard); let mut __range_values = " + stdVecPath() + "::with_capacity(__range_len); for __range_index in 0..__range_len { let mut __range_guard = __range_holder")
			WriteBorrowMethod(out, true)
			out.WriteString("; let __range_value = std::mem::replace(&mut __range_guard.as_mut().unwrap()[__range_index], Box::new(()) as ")
			out.WriteString(rustAnyTraitObject())
			out.WriteString("); std::mem::drop(__range_guard); __range_values.push(__range_value); } __range_values }; ")
			return
		}
		NeedAnyClone()
		out.WriteString("; let __range_len = __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); std::mem::drop(__range_guard); let __range_values = 0..__range_len; ")
		return
	}
	copyCall, known := elementGoValueCopyCall(rangeExpr)
	if !known {
		out.WriteString("; let __range_values: Vec<()> = ")
		out.WriteString(goValueCopyNeedsTypeInfo)
		out.WriteString("; std::mem::drop(__range_guard); ")
		return
	}
	if copyCall != ".clone()" && rangeExprIsArrayValue(rangeExpr) {
		// Go copies an array before it ranges over it, so the loop does not
		// see writes made to the array while it runs. Cloning the array
		// would clone each struct's handles and let those writes through.
		out.WriteString("; let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| __e")
		out.WriteString(copyCall)
		out.WriteString(").collect::<Vec<_>>()).unwrap_or_default(); std::mem::drop(__range_guard); ")
		return
	}
	// A slice is not copied: each element is read when its turn comes. The
	// snapshot holds second handles on the elements, so a field written
	// during the loop is seen, and the loop variable takes its copy of the
	// element at the start of its iteration (writeRangeOwnedValuesAdapter
	// and writeRangeValueIterationCopy).
	if rangeExprIsArrayValue(rangeExpr) {
		// An array's cell always holds it. Rust has no Default for arrays
		// longer than 32 elements, such as a [256]byte table.
		out.WriteString("; let __range_values = __range_guard.as_ref().cloned().expect(\"array value\"); std::mem::drop(__range_guard); ")
		return
	}
	out.WriteString("; let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); ")
}

// writeRangeValueIterationCopy makes the loop variable of a range over
// structs Go's copy of its element, at the start of the iteration. It is for
// the loops that bind the variable to a reference to the element; without
// it "p.X = 1" in the loop writes into the slice. The loops that bind an
// owned value copy in writeRangeOwnedValuesAdapter.
func writeRangeValueIterationCopy(out *strings.Builder, rangeExpr ast.Expr, value ast.Expr, body *ast.BlockStmt) {
	ident, ok := value.(*ast.Ident)
	if !ok || ident.Name == "_" {
		return
	}
	copyCall, known := elementGoValueCopyCall(rangeExpr)
	if !known || copyCall == ".clone()" {
		return
	}
	name := RustLocalIdent(ident.Name)
	out.WriteString("        let ")
	out.WriteString(name)
	if rangeBodyCallsPointerMethodOn(body, ident) {
		// The copy is the loop's own: a pointer method called on it borrows
		// it mutably, and changes only the copy.
		out.WriteString(" = &mut ")
	} else {
		out.WriteString(" = &")
	}
	out.WriteString(name)
	out.WriteString(copyCall)
	out.WriteString(";\n")
}

// rangeBodyCallsPointerMethodOn reports whether body calls a method with a
// pointer receiver on the range variable ident, as v.Scale(2) does.
func rangeBodyCallsPointerMethodOn(body *ast.BlockStmt, ident *ast.Ident) bool {
	typeInfo := GetTypeInfo()
	if body == nil || typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj := typeInfo.info.Defs[ident]
	if obj == nil {
		obj = typeInfo.info.Uses[ident]
	}
	if obj == nil {
		return false
	}
	found := false
	ast.Inspect(body, func(n ast.Node) bool {
		if found {
			return false
		}
		call, ok := n.(*ast.CallExpr)
		if !ok {
			return true
		}
		sel, ok := unwrapParens(call.Fun).(*ast.SelectorExpr)
		if !ok {
			return true
		}
		recv, ok := unwrapParens(sel.X).(*ast.Ident)
		if !ok || typeInfo.info.Uses[recv] != obj {
			return true
		}
		if selection := typeInfo.info.Selections[sel]; selection != nil && selection.Kind() == types.MethodVal {
			if sig, ok := selection.Obj().Type().(*types.Signature); ok && sig.Recv() != nil {
				if _, isPtr := types.Unalias(sig.Recv().Type()).(*types.Pointer); isPtr {
					found = true
				}
			}
		}
		return true
	})
	return found
}

// writeMapRangeValueIterationCopy makes the value variable of a range over a
// map of structs Go's copy of the map's value, at the start of the
// iteration. The loop binds the variable to the map's own cell for that
// value; without this "v.X = 1" in the loop writes into the map.
func writeMapRangeValueIterationCopy(out *strings.Builder, valueType types.Type, value ast.Expr) {
	ident, ok := value.(*ast.Ident)
	if !ok || ident.Name == "_" {
		return
	}
	name := RustLocalIdent(ident.Name)
	if valueType == nil {
		out.WriteString("        let ")
		out.WriteString(name)
		out.WriteString(" = ")
		WriteWrapperPrefix(out)
		out.WriteString(goValueCopyNeedsTypeInfo)
		WriteWrapperSuffix(out)
		out.WriteString(";\n")
		return
	}
	copyCall := goValueCopyCallForType(valueType)
	if copyCall == ".clone()" {
		return
	}
	out.WriteString("        let ")
	out.WriteString(name)
	out.WriteString(" = ")
	WriteWrapperPrefix(out)
	out.WriteString(name)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	out.WriteString(copyCall)
	WriteWrapperSuffix(out)
	out.WriteString(";\n")
}

// writeRangeOwnedValuesAdapter emits the iterator adapter that yields owned
// loop values from a materialized range snapshot. A normal []any slice stores
// its original index range, then rereads and clones the current element at the
// start of each iteration. The indexed-writeback special case owns a Vec of
// moved Box<dyn Any> values and consumes it instead. enumerate adds the
// (index, value) pairing.
func writeRangeOwnedValuesAdapter(out *strings.Builder, rangeExpr ast.Expr, usingRangeValues bool, moveEmptyInterfaceValues bool, enumerate bool) {
	if usingRangeValues && rangeSliceElementIsEmptyInterface(rangeExpr) {
		if moveEmptyInterfaceValues {
			out.WriteString(".into_iter()")
		} else {
			out.WriteString(".map(|__range_index| { let __range_guard = __range_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; go_any_clone(__range_guard.as_ref().unwrap()[__range_index].as_ref()) })")
		}
	} else if copyCall, known := elementGoValueCopyCall(rangeExpr); known && copyCall != ".clone()" {
		// The loop variable is Go's copy of the element, taken when the
		// iteration starts: the map runs once for each step of the loop.
		out.WriteString(".iter().map(|__e| __e")
		out.WriteString(copyCall)
		out.WriteString(")")
	} else {
		out.WriteString(".iter().cloned()")
	}
	if enumerate {
		out.WriteString(".enumerate()")
	}
}

// writePointerShortDeclRhsValue emits the value for `lhs := rhs` where rhs is a
// pointer-typed expression. A pointer variable already lowers to a wrapped
// handle, so its handle is cloned to preserve aliasing. The current pointer
// receiver, however, is `&self` (a bare borrow, not a handle), so a clone of it
// is wrapped into a fresh handle (the clone shares the receiver's wrapped field
// handles, like the defer-capture / method-value patterns).
func writePointerShortDeclRhsValue(out *strings.Builder, rhs ast.Expr) {
	if writeCurrentPointerReceiverHandleClone(out, rhs) {
		return
	}
	if ident, ok := packageGlobalPointerIdent(rhs); ok {
		writePackageGlobalPointerHandleClone(out, ident)
		return
	}
	if writeSourceMappedPackageGlobalPointerHandleClone(out, rhs) {
		return
	}
	TranspileExpressionContext(out, rhs, AddressOf)
	out.WriteString(".clone()")
}

func tempHoldsWrappedValue(rhs ast.Expr) bool {
	if isAssignmentSelfWrappingExpression(rhs) {
		return true
	}
	if call, ok := rhs.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		return typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call))
	}
	if sel, ok := rhs.(*ast.SelectorExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo == nil || typeInfo.info == nil {
			return false
		}
		// Package-qualified consts/vars don't emit wrapped values.
		if ident, ok := sel.X.(*ast.Ident); ok {
			if obj, ok := typeInfo.info.Uses[ident]; ok {
				if _, isPkg := obj.(*types.PkgName); isPkg {
					return false
				}
			}
		}
		obj, ok := typeInfo.info.Uses[sel.Sel]
		if !ok {
			return false
		}
		field, isVar := obj.(*types.Var)
		if !isVar {
			return false
		}
		if _, _, ok := namedSliceTypeFromType(field.Type()); ok {
			return false
		}
		// Handle-typed fields (pointer/slice/map/chan/func/interface) always keep
		// their wrapped Arc<Mutex<Option<T>>> handle when read.
		if selectorFieldValueKeepsHandle(field.Type()) {
			return true
		}
		// A value-typed field (Basic, named scalars like token.Pos, structs,
		// arrays) is unwrapped to a bare value snapshot when read into a parallel
		// assignment temp: writeOwnedExpressionValue clones it out of the handle
		// (isCloneableNonPointerExpr), and Go's parallel-assignment semantics
		// require a value snapshot of the RHS rather than an alias to the field's
		// handle. So the temp holds a value, not the wrapped handle, whether the
		// field is read off the current receiver (self.field) or a wrapped pointer
		// variable (v.field).
		return false
	}
	typeInfo := GetTypeInfo()
	if index, ok := unwrapParens(rhs).(*ast.IndexExpr); ok && typeInfo != nil && !typeInfo.IsMap(index.X) {
		// A row of a [][]T, or any element a slice or an array stores bare
		// rather than in its own cell, is read out as a bare value.
		if elemType := indexedSequenceElementAssignmentType(typeInfo, index); elemType != nil && !indexedSequenceElementKeepsHandle(elemType) {
			return false
		}
	}
	return typeInfo != nil && parallelTempTypeKeepsHandle(typeInfo.GetType(rhs))
}

func isErrorAssignment(lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return isGoErrorType(assignmentTargetType(typeInfo, lhs)) && isGoErrorType(typeInfo.GetType(rhs))
}

func isConcreteErrorInterfaceAssignment(lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if !isGoErrorType(assignmentTargetType(typeInfo, lhs)) {
		return false
	}
	return isConcreteGoErrorValue(typeInfo.GetType(rhs))
}

func assignmentTargetType(typeInfo *TypeInfo, expr ast.Expr) types.Type {
	if typeInfo == nil || expr == nil {
		return nil
	}
	if typ := typeInfo.GetType(expr); typ != nil {
		return typ
	}
	if ident, ok := expr.(*ast.Ident); ok {
		if obj := typeInfo.GetObject(ident); obj != nil {
			return obj.Type()
		}
	}
	return nil
}

func isConcreteErrorReturnValue(result ast.Expr, expected ast.Expr) bool {
	if expected == nil {
		return false
	}
	targetType := expectedTypeFromParamExpr(expected)
	if !isGoErrorType(targetType) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return isConcreteGoErrorValue(typeInfo.GetType(result))
}

func writeGoErrorReturnValue(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	targetType := expectedTypeFromParamExpr(expected)
	if !isGoErrorType(targetType) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isGoErrorType(typeInfo.GetType(result)) {
		return false
	}
	return writeGoErrorHandleValue(out, result)
}

func isGoErrorType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	errorObj := types.Universe.Lookup("error")
	if errorObj == nil {
		return false
	}
	if types.Identical(types.Unalias(typ), types.Unalias(errorObj.Type())) {
		return true
	}
	// Go interfaces are satisfied structurally. A named or anonymous
	// interface whose complete method set is exactly Error() string accepts
	// precisely the same values as the predeclared error interface, so it can
	// use the same run-time representation. Keeping it on the generic local
	// trait-object path loses errors.New/fmt.Errorf values because those are
	// carried by the dedicated error handle.
	iface, ok := types.Unalias(typ).Underlying().(*types.Interface)
	if !ok {
		return false
	}
	errorIface, ok := types.Unalias(errorObj.Type()).Underlying().(*types.Interface)
	if !ok {
		return false
	}
	iface.Complete()
	errorIface.Complete()
	if !iface.IsMethodSet() || iface.NumMethods() != errorIface.NumMethods() {
		return false
	}
	for i := 0; i < iface.NumMethods(); i++ {
		method := iface.Method(i)
		errorMethod := errorIface.Method(i)
		if method.Id() != errorMethod.Id() || !types.Identical(method.Type(), errorMethod.Type()) {
			return false
		}
	}
	return true
}

func isConcreteGoErrorValue(typ types.Type) bool {
	if typ == nil || isGoErrorType(typ) {
		return false
	}
	errorObj := types.Universe.Lookup("error")
	if errorObj == nil {
		return false
	}
	errorInterface, ok := errorObj.Type().Underlying().(*types.Interface)
	if !ok {
		return false
	}
	errorInterface.Complete()
	return types.Implements(typ, errorInterface)
}

func concreteGoErrorConversionOperand(expr ast.Expr, typeInfo *TypeInfo) (ast.Expr, bool) {
	call, ok := unwrapParens(expr).(*ast.CallExpr)
	if !ok || typeInfo == nil || len(call.Args) != 1 || !typeInfo.IsTypeConversion(call) {
		return nil, false
	}
	if !isGoErrorType(typeInfo.GetType(call)) || !isConcreteGoErrorValue(typeInfo.GetType(call.Args[0])) {
		return nil, false
	}
	return call.Args[0], true
}

func writeConcreteErrorValue(out *strings.Builder, expr ast.Expr) {
	if unary, ok := expr.(*ast.UnaryExpr); ok && unary.Op == token.AND {
		if composite, ok := unary.X.(*ast.CompositeLit); ok {
			TranspileExpressionContext(out, composite, AddressOf)
			return
		}
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if typ := typeInfo.GetType(expr); typ != nil && isConcreteGoErrorValue(typ) {
			if _, ok := types.Unalias(typ).(*types.Pointer); ok {
				out.WriteString("(*")
				TranspileExpressionContext(out, expr, LValue)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
				return
			}
		}
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && isConstantExpression(expr) {
		if named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named); ok && isNamedIntegerType(named) {
			if writeExpressionForExpectedTypesType(out, expr, named) {
				return
			}
		}
	}
	if !writeOwnedExpressionValue(out, expr) {
		TranspileExpression(out, expr)
	}
}

// writeConcreteErrorVarInitializer writes the initializer of
// "var name error = value" when value is not an error already but a struct
// or pointer that implements error. It is boxed as an assignment to an error
// variable boxes it.
// writeUnnamedSliceToNamedSliceInitializer writes the value of
// var b Bytes = []byte("abc"), an unnamed slice assigned to a variable of a
// defined slice type, as the defined type holding the slice, as the
// conversion Bytes([]byte("abc")) would.
func writeUnnamedSliceToNamedSliceInitializer(out *strings.Builder, value ast.Expr, declared types.Type) bool {
	targetNamed, _, ok := namedSliceTypeFromType(declared)
	if !ok {
		return false
	}
	// A type assertion here yields the asserted slice's elements, which the
	// declaration's own path wraps.
	if _, isAssertion := unwrapParens(value).(*ast.TypeAssertExpr); isAssertion {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(value)
	if actual == nil {
		return false
	}
	if _, ok := types.Unalias(actual).(*types.Named); ok {
		return false
	}
	if _, ok := actual.Underlying().(*types.Slice); !ok {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(goTypesNamedTypeToRust(targetNamed))
	out.WriteString("(")
	switch unwrapParens(value).(type) {
	case *ast.Ident, *ast.SelectorExpr:
		// "var u Numbers = s" copies s's header into a cell of its own:
		// appending to u does not change s, and a nil s gives a nil u.
		out.WriteString(GetOuterWrapperType() + "::new(" + GetInnerWrapperType() + "::new(")
		TranspileExpressionContext(out, unwrapParens(value), LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".clone()))")
	default:
		// An element of a [][]byte is the row itself, not a handle: it goes
		// into a cell of its own.
		if typeInfo.ReturnsWrappedValue(unwrapParens(value)) {
			writePlainSliceHandleForNamedSliceConversion(out, value)
		} else {
			WriteWrapperPrefix(out)
			TranspileExpression(out, value)
			out.WriteString(".clone()")
			WriteWrapperSuffix(out)
		}
	}
	out.WriteString(")")
	WriteWrapperSuffix(out)
	return true
}

func writeConcreteErrorVarInitializer(out *strings.Builder, typeExpr ast.Expr, value ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	valueType := typeInfo.GetType(value)
	if !isGoErrorType(typeInfo.GetType(typeExpr)) || !isConcreteGoErrorValue(valueType) {
		return false
	}
	// An interface that embeds error also implements error, but its value is
	// a trait object, not a struct to box.
	if types.IsInterface(valueType) {
		return false
	}
	return writeGoErrorHandleValue(out, value)
}

func writeConcreteErrorBox(out *strings.Builder, expr ast.Expr) {
	TrackImport("Error")
	out.WriteString(rustErrorFromValueStart())
	writeConcreteErrorValue(out, expr)
	out.WriteString(rustErrorFromValueEnd())
}

func writeGoErrorHandleValue(out *strings.Builder, expr ast.Expr) bool {
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
		writeEmptyErrorHandle(out)
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if isGoErrorType(typ) {
		if errorObj := types.Universe.Lookup("error"); errorObj != nil {
			if writeGoErrorCallArgument(out, expr, errorObj.Type()) {
				return true
			}
		}
		TranspileExpression(out, expr)
		return true
	}
	if isConcreteGoErrorValue(typ) {
		WriteWrapperPrefix(out)
		writeConcreteErrorBox(out, expr)
		WriteWrapperSuffix(out)
		return true
	}
	return false
}

func writeConcreteErrorInterfaceAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) {
	out.WriteString("{ let new_val = ")
	writeConcreteErrorBox(out, rhs)
	out.WriteString("; *")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(new_val); }")
}

// writeErrorCopyAssignment assigns one error variable to another. The target
// gets its own pointer to the same error and the source keeps its own, as in
// Go, where "last = err" leaves err as it was.
func writeErrorCopyAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) {
	out.WriteString("{ let __rhs_holder = ")
	TranspileExpressionContext(out, rhs, LValue)
	out.WriteString(".clone(); let new_val = (*__rhs_holder")
	WriteBorrowMethod(out, false)
	out.WriteString(").clone(); *")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = new_val; }")
}

// writeErrorHandleCopy writes a new handle that holds the same error as the
// handle writeSource writes. An error variable that is passed, returned,
// stored or used to start another variable is copied this way: the two
// variables then change independently, and they are still the same error.
func writeErrorHandleCopy(out *strings.Builder, writeSource func()) {
	out.WriteString("{ let __err_source = ")
	writeSource()
	out.WriteString("; let __err_copy = (*__err_source")
	WriteBorrowMethod(out, false)
	out.WriteString(").clone(); ")
	writeErrorHandleFromOptionValue(out, "__err_copy")
	out.WriteString(" }")
}

// blankAssignmentOnlyNamesAVariable reports whether the right side of
// "_ = x" is a plain variable. Go evaluates nothing there that can fail, so
// the generated code must not read the variable's value: a nil error,
// pointer, slice, map, function or interface has none to read.
//
// A field of a variable is the same: "_ = t.err" reads nothing from the
// field. Reaching the field through a nil pointer still stops the program,
// as it does in Go.
func blankAssignmentOnlyNamesAVariable(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	switch e := unwrapParens(expr).(type) {
	case *ast.Ident:
		if e.Name == "_" {
			return false
		}
		variable, ok := typeInfo.info.Uses[e].(*types.Var)
		return ok && !variable.IsField()
	case *ast.SelectorExpr:
		selection := typeInfo.info.Selections[e]
		if selection == nil || selection.Kind() != types.FieldVal {
			return false
		}
		return blankAssignmentOnlyNamesAVariable(e.X)
	}
	return false
}

// expressionCallsOrReceives reports whether evaluating expr calls a function
// or receives from a channel, so that evaluating it one more time, or one
// time less, changes what the program does.
func expressionCallsOrReceives(expr ast.Expr) bool {
	found := false
	ast.Inspect(expr, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.CallExpr:
			found = true
		case *ast.UnaryExpr:
			if n.Op == token.ARROW {
				found = true
			}
		case *ast.FuncLit:
			return false
		}
		return !found
	})
	return found
}

// mapOperandHandleCanBeHeld reports whether the map operand of "m[k] op= v"
// is a handle that can be cloned and held while the key and the operand are
// evaluated. A package-level variable is not: it is a static that owns its
// map, and cloning it copies the map. Without type information there is no
// telling a package-level variable of another package from a field.
func mapOperandHandleCanBeHeld(mapExpr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	switch operand := unwrapParens(mapExpr).(type) {
	case *ast.Ident:
		return !isPackageGlobalIdent(operand)
	case *ast.SelectorExpr:
		if variable, ok := typeInfo.info.Uses[operand.Sel].(*types.Var); ok && !variable.IsField() {
			return false
		}
	}
	return true
}

// exprHasFuncLit reports whether expr contains a function literal.
func exprHasFuncLit(expr ast.Expr) bool {
	found := false
	ast.Inspect(expr, func(n ast.Node) bool {
		if _, ok := n.(*ast.FuncLit); ok {
			found = true
		}
		return !found
	})
	return found
}

func writeMapElementUpdate(out *strings.Builder, indexExpr *ast.IndexExpr, op token.Token, rhs ast.Expr) {
	typeInfo := GetTypeInfo()
	defaultValue := "Default::default()"
	var keyType types.Type
	if typeInfo != nil {
		keyType, _ = typeInfo.GetMapTypes(indexExpr.X)
		defaultValue = zeroValueForTypesType(typeInfo.GetMapValueType(indexExpr.X))
	}

	// The key and the operand are evaluated before the map is locked for
	// writing. Either may read the map ("m[a] += m[b]"), and a read under the
	// write lock panics, or hangs in a program with goroutines.
	//
	// Go evaluates the map operand first, so its handle is taken before the
	// key and the operand, without locking it. A package-level map has no
	// handle to take: it is written in place, and naming it calls nothing.
	holdsMapHandle := mapOperandHandleCanBeHeld(indexExpr.X)
	out.WriteString("{ ")
	if holdsMapHandle {
		out.WriteString("let __map_holder = ")
		writeMapHandleCloneForOp(out, indexExpr.X)
		out.WriteString("; ")
	}
	out.WriteString("let __key = ")
	writeMapAssignmentKeyExpression(out, indexExpr.Index, keyType, rhs)
	out.WriteString("; ")
	operandIsRHS := op != token.INC && op != token.DEC && rhs != nil
	if operandIsRHS {
		out.WriteString("let __rhs = ")
		TranspileExpression(out, rhs)
		out.WriteString("; ")
	}
	out.WriteString("let mut __map_guard = ")
	if holdsMapHandle {
		out.WriteString("__map_holder")
	} else {
		writeMapHandleForOp(out, indexExpr.X)
	}
	WriteBorrowMethod(out, true)
	out.WriteString("; let __map = __map_guard")
	writeMapMutationUnwrap(out, indexExpr.X)
	out.WriteString("; let __entry = __map.entry(__key")
	out.WriteString(").or_insert_with(|| ")
	WriteWrapperPrefix(out)
	out.WriteString(defaultValue)
	WriteWrapperSuffix(out)
	out.WriteString("); let mut __value = __entry")
	WriteBorrowMethod(out, true)
	out.WriteString("; * __value = Some((*__value.as_ref().unwrap()).clone() ")
	switch op {
	case token.INC, token.ADD_ASSIGN:
		out.WriteString("+")
	case token.DEC, token.SUB_ASSIGN:
		out.WriteString("-")
	case token.MUL_ASSIGN:
		out.WriteString("*")
	case token.QUO_ASSIGN:
		out.WriteString("/")
	case token.REM_ASSIGN:
		out.WriteString("%")
	case token.AND_ASSIGN:
		out.WriteString("&")
	case token.AND_NOT_ASSIGN:
		out.WriteString("& !")
	case token.OR_ASSIGN:
		out.WriteString("|")
	case token.XOR_ASSIGN:
		out.WriteString("^")
	case token.SHL_ASSIGN:
		out.WriteString("<<")
	case token.SHR_ASSIGN:
		out.WriteString(">>")
	default:
		out.WriteString("+")
	}
	out.WriteString(" ")
	var valueType types.Type
	if typeInfo != nil {
		valueType = typeInfo.GetMapValueType(indexExpr.X)
	}
	if op == token.INC || op == token.DEC {
		if valueType != nil {
			out.WriteString(incDecStepForType(valueType))
		} else {
			out.WriteString("1")
		}
	} else if operandIsRHS {
		// String + &str: m[k] += "x" appends.
		if op == token.ADD_ASSIGN && valueType != nil && isStringTypesType(valueType) {
			out.WriteString("&")
		}
		out.WriteString("__rhs")
	} else {
		out.WriteString("0")
	}
	out.WriteString("); }")
}

// isStringTypesType reports whether typ is a string, or a type defined as
// one whose values are plain Rust Strings.
func isStringTypesType(typ types.Type) bool {
	if _, isNamed := types.Unalias(typ).(*types.Named); isNamed {
		basic, ok := types.Unalias(typ).Underlying().(*types.Basic)
		return ok && basic.Info()&types.IsString != 0 && !isNamedScalarNewtype(typ)
	}
	basic, ok := types.Unalias(typ).(*types.Basic)
	return ok && basic.Info()&types.IsString != 0
}

func writeCompoundAssignOperator(out *strings.Builder, op token.Token) {
	switch op {
	case token.ADD_ASSIGN:
		out.WriteString("+")
	case token.SUB_ASSIGN:
		out.WriteString("-")
	case token.MUL_ASSIGN:
		out.WriteString("*")
	case token.QUO_ASSIGN:
		out.WriteString("/")
	case token.REM_ASSIGN:
		out.WriteString("%")
	case token.AND_ASSIGN:
		out.WriteString("&")
	case token.AND_NOT_ASSIGN:
		out.WriteString("& !")
	case token.OR_ASSIGN:
		out.WriteString("|")
	case token.XOR_ASSIGN:
		out.WriteString("^")
	case token.SHL_ASSIGN:
		out.WriteString("<<")
	case token.SHR_ASSIGN:
		out.WriteString(">>")
	}
}

func compoundAssignUsesOwnedNamedIntegerValue(lhs ast.Expr, op token.Token) bool {
	switch op {
	case token.ADD_ASSIGN, token.SUB_ASSIGN, token.MUL_ASSIGN, token.QUO_ASSIGN, token.REM_ASSIGN,
		token.AND_ASSIGN, token.AND_NOT_ASSIGN, token.OR_ASSIGN, token.XOR_ASSIGN, token.SHL_ASSIGN, token.SHR_ASSIGN:
	default:
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	lhsNamed, ok := types.Unalias(typeInfo.GetType(lhs)).(*types.Named)
	if !ok || !isNamedScalarNewtype(lhsNamed) {
		return false
	}
	return true
}

func compoundAssignRHSExpectedType(lhs ast.Expr, expected types.Type) types.Type {
	if !currentReceiverScalarTypeDefinition() {
		return expected
	}
	var ident *ast.Ident
	switch e := unwrapParens(lhs).(type) {
	case *ast.Ident:
		ident = e
	case *ast.StarExpr:
		ident, _ = unwrapParens(e.X).(*ast.Ident)
	}
	if ident == nil || !isCurrentReceiverIdent(ident) {
		return expected
	}
	named, ok := types.Unalias(expected).(*types.Named)
	if !ok {
		return expected
	}
	return named.Underlying()
}

func compoundAssignConstCanUseNamedInteger(expr ast.Expr, named *types.Named) bool {
	if expr == nil || named == nil || !isConstantExpression(expr) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	exprType := typeInfo.GetType(expr)
	if exprType == nil {
		return false
	}
	if exprNamed, ok := types.Unalias(exprType).(*types.Named); ok {
		return sameNamedTypeDefinition(exprNamed, named)
	}
	basic, ok := types.Unalias(exprType).(*types.Basic)
	return ok && basic.Info()&types.IsUntyped != 0 && isIntegerBasicKind(basic.Kind())
}

func writeWrappedMutationTargetClone(out *strings.Builder, expr ast.Expr) {
	out.WriteString("let __target = ")
	TranspileExpressionContext(out, expr, LValue)
	out.WriteString(".clone(); ")
}

func writeWrappedMutationTargetPrelude(out *strings.Builder, expr ast.Expr) bool {
	if _, ok := expr.(*ast.SelectorExpr); !ok {
		return false
	}
	writeWrappedMutationTargetClone(out, expr)
	return true
}

func writeWrappedMutationTargetRef(out *strings.Builder, expr ast.Expr, mutable bool) {
	switch e := expr.(type) {
	case *ast.SelectorExpr:
		out.WriteString("__target")
	case *ast.StarExpr:
		// `*p` mutation (increment/compound-assign): the pointer operand is
		// itself the wrapper handle (Arc<Mutex<Option<T>>> /
		// Rc<RefCell<Option<T>>>), so lock/borrow it directly to obtain a
		// guard over Option<T>. Lower the operand as an LValue to get the bare
		// handle place; lowering the StarExpr itself (or the operand as a
		// value) would emit the fully-dereferenced scalar, and the trailing
		// WriteBorrowMethod would then call .lock()/.borrow() on a scalar
		// (E0599: no method named lock/borrow found for type i32).
		TranspileExpressionContext(out, e.X, LValue)
	default:
		TranspileExpressionContext(out, expr, LValue)
	}
	WriteBorrowMethod(out, mutable)
}

func writeBareCompoundAssignValue(out *strings.Builder, expr ast.Expr, expected types.Type) {
	writeBareCompoundAssignValueForOp(out, expr, expected, token.ILLEGAL)
}

func writeBareStringSliceValue(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	if expected == nil {
		return false
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok || basic.Kind() != types.String {
		return false
	}
	slice, ok := expr.(*ast.SliceExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsString(slice.X) {
		return false
	}
	writeStringSliceValue(out, slice.X, slice.Low, slice.High)
	return true
}

func rangeLoopVarIsUsize(name string) bool {
	return rangeLoopVars[name] == "usize"
}

func rustCastForExpectedBasic(expected types.Type) string {
	if expected == nil {
		return ""
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok {
		return ""
	}
	switch basic.Kind() {
	case types.Int:
		return rustIntType()
	case types.Int32:
		return "i32"
	case types.Int8:
		return "i8"
	case types.Int16:
		return "i16"
	case types.Int64:
		return "i64"
	case types.Uint:
		return rustUintType()
	case types.Uint32:
		return "u32"
	case types.Uint8:
		return "u8"
	case types.Uint16:
		return "u16"
	case types.Uint64:
		return "u64"
	case types.Uintptr:
		return "usize"
	case types.Float32:
		return "f32"
	case types.Float64:
		return "f64"
	}
	return ""
}

func writeBareCompoundAssignValueForOp(out *strings.Builder, expr ast.Expr, expected types.Type, op token.Token) {
	if writeNamedIntegerConstCompoundAssignValue(out, expr, expected, op) {
		return
	}
	if writeNamedIntegerCompoundAssignUnderlyingValue(out, expr, expected) {
		return
	}
	if writeNamedFloatCompoundAssignUnderlyingValue(out, expr, expected) {
		return
	}
	if writeNamedIntegerCompoundAssignRHSForUnderlyingSlot(out, expr, expected) {
		return
	}
	if writeBareStringSliceValue(out, expr, expected) {
		return
	}
	if writeRuneLiteralAssignmentValue(out, expr, expected) {
		return
	}
	if writeConstExpressionForExpectedGoType(out, expr, expected) {
		return
	}
	if writeBareValueForWrappedSlot(out, expr) {
		return
	}
	if ident, ok := expr.(*ast.Ident); ok {
		// true and false are constants of the universe scope; the constant
		// path below would give them a package constant's escaped name.
		if isPredeclaredBoolConstant(ident) {
			out.WriteString(ident.Name)
			return
		}
		_, isRangeVar := rangeLoopVars[ident.Name]
		isLocalConst := isLocalConstantIdent(ident)
		varName := rustIdentForUseWithCapture(ident)
		if isRangeVar && rangeLoopVarIsUsize(ident.Name) && expected != nil {
			if rustCast := rustCastForExpectedBasic(expected); rustCast != "" {
				out.WriteString("(")
				out.WriteString(varName)
				out.WriteString(" as ")
				out.WriteString(rustCast)
				out.WriteString(")")
				return
			}
		}
		if isLocalConst {
			if writeConstIdentForCompoundExpected(out, ident, expected, RustIdentForUse(ident)) {
				return
			}
			out.WriteString(RustIdentForUse(ident))
			return
		}
		if isConstIdent(ident) {
			if writeConstIdentForCompoundExpected(out, ident, expected, rustConstName(ident.Name)) {
				return
			}
			out.WriteString(rustConstName(ident.Name))
			return
		}
		if isVarBare(ident.Name) {
			if expected != nil {
				if typeInfo := GetTypeInfo(); typeInfo != nil {
					if expectedNamed, ok := types.Unalias(expected).(*types.Named); ok {
						if exprNamed, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named); ok && sameNamedTypeDefinition(exprNamed, expectedNamed) {
							if _, ok := externalIntegerRustTypeForNamed(expectedNamed); ok {
								out.WriteString(varName)
								out.WriteString(".0")
								return
							}
						}
					}
				}
			}
			out.WriteString(varName)
			return
		}
		if !isRangeVar && !isLocalConst && ident.Name != "true" && ident.Name != "false" &&
			ident.Name != "nil" && ident.Name != "_" {
			out.WriteString("(*")
			out.WriteString(varName)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			if !isCopyTypeExpression(expr) && isCloneableNonPointerExpr(expr) {
				out.WriteString(".clone()")
			}
			return
		}
		out.WriteString(varName)
		return
	}
	if lit, ok := expr.(*ast.BasicLit); ok {
		if lit.Kind == token.INT {
			if basic, ok := types.Unalias(expected).Underlying().(*types.Basic); ok {
				switch basic.Kind() {
				case types.Float32, types.Float64:
					out.WriteString(RustFloatFromIntegerLiteral(lit.Value))
					return
				}
			}
			out.WriteString(RustIntegerLiteral(lit.Value))
			return
		}
		out.WriteString(lit.Value)
		return
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		if expected != nil {
			if basic, ok := types.Unalias(expected).Underlying().(*types.Basic); ok && basic.Kind() == types.Int {
				if isBareBuiltinCallName(call, "len") || isBareBuiltinCallName(call, "cap") {
					TranspileExpression(out, call)
					out.WriteString(" as " + rustIntType())
					return
				}
			}
		}
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) {
			out.WriteString("(*")
			TranspileExpression(out, call)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			if !isCopyTypeExpression(expr) && isCloneableNonPointerExpr(expr) {
				out.WriteString(".clone()")
			}
			return
		}
	}
	if !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
		return
	}
	TranspileExpression(out, expr)
}

func writeRuneLiteralAssignmentValue(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	lit, ok := expr.(*ast.BasicLit)
	if !ok || lit.Kind != token.CHAR || expected == nil {
		return false
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok || basic.Kind() != types.Int32 {
		return false
	}
	out.WriteString("(")
	out.WriteString(RustCharLiteral(lit.Value))
	out.WriteString(" as i32)")
	return true
}

func writeNamedIntegerCompoundAssignUnderlyingValue(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	if expr == nil || expected == nil {
		return false
	}
	switch expr.(type) {
	case *ast.BasicLit, *ast.Ident:
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil {
		return false
	}
	if call, ok := expr.(*ast.CallExpr); ok && typeInfo.ReturnsWrappedValue(call) {
		return false
	}
	expectedNamed, ok := types.Unalias(expected).(*types.Named)
	if !ok || expectedNamed.Obj() == nil || expectedNamed.Obj().Pkg() != typeInfo.pkg {
		return false
	}
	if _, ok := types.Unalias(expectedNamed.Underlying()).(*types.Basic); !ok {
		return false
	}
	actualNamed, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !sameNamedTypeDefinition(actualNamed, expectedNamed) {
		return false
	}
	var value strings.Builder
	TranspileExpression(&value, expr)
	out.WriteString("(*")
	out.WriteString("(")
	out.WriteString(value.String())
	out.WriteString(")")
	out.WriteString(".0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
	return true
}

func writeNamedIntegerCompoundAssignRHSForUnderlyingSlot(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	if expr == nil || expected == nil {
		return false
	}
	if isConstantExpression(expr) {
		return false
	}
	expectedBasic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok || !isIntegerBasicKind(expectedBasic.Kind()) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if call, ok := expr.(*ast.CallExpr); ok && typeInfo.ReturnsWrappedValue(call) {
		return false
	}
	actualNamed, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(actualNamed) {
		return false
	}
	if _, ok := externalIntegerRustTypeForNamed(actualNamed); ok {
		return false
	}
	actualBasic, ok := types.Unalias(actualNamed.Underlying()).(*types.Basic)
	if !ok || actualBasic.Kind() != expectedBasic.Kind() {
		return false
	}
	var value strings.Builder
	TranspileExpression(&value, expr)
	out.WriteString("(*(")
	out.WriteString(value.String())
	out.WriteString(").0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
	return true
}

func writeNamedIntegerConstCompoundAssignValue(out *strings.Builder, expr ast.Expr, expected types.Type, op token.Token) bool {
	switch op {
	case token.AND_ASSIGN, token.AND_NOT_ASSIGN, token.OR_ASSIGN, token.XOR_ASSIGN:
	default:
		return false
	}
	if expected == nil {
		return false
	}
	named, ok := types.Unalias(expected).(*types.Named)
	if !ok || !compoundAssignConstCanUseNamedInteger(expr, named) {
		return false
	}
	if _, ok := externalIntegerRustTypeForNamed(named); ok {
		return writeExpressionForExpectedTypesType(out, expr, named)
	}
	return writeNamedIntegerConstForExpected(out, expr, named)
}

// isChannelReceiveExpr reports whether expr is a receive, <-ch.
func isChannelReceiveExpr(expr ast.Expr) bool {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	return ok && unary.Op == token.ARROW
}

func writeNamedIntegerWrappedInitializer(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	WriteWrapperPrefix(out)
	if !writeNamedIntegerValueForExpected(out, expr, named) {
		TranspileExpression(out, expr)
	}
	WriteWrapperSuffix(out)
	return true
}

func writeStringConstWrappedInitializer(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	expected := typeInfo.GetType(lhs)
	if expected == nil {
		if ident, ok := lhs.(*ast.Ident); ok {
			if obj := typeInfo.GetObject(ident); obj != nil {
				expected = obj.Type()
			}
		}
	}
	if !isBasicStringGoType(expected) {
		return false
	}
	if _, ok := constStringLiteral(rhs); !ok {
		return false
	}
	WriteWrapperPrefix(out)
	writeConstExpressionForExpectedString(out, rhs, expected)
	WriteWrapperSuffix(out)
	return true
}

func writeNamedIntegerAssignmentValue(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	if !writeNamedIntegerValueForExpected(out, expr, named) {
		TranspileExpression(out, expr)
	}
	return true
}

func writeNamedIntegerIncDec(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok {
		return false
	}
	rustType, ok := rustCastTypeForDefinedUnderlying(basic.Name())
	if !ok {
		return false
	}

	out.WriteString("{ ")
	writeWrappedMutationTargetPrelude(out, expr)
	out.WriteString("let mut guard = ")
	writeWrappedMutationTargetRef(out, expr, true)
	// `guard.as_ref().unwrap().clone()` is already the named newtype value,
	// and the generated `Add<scalar>`/`Sub<scalar>` impls return the newtype
	// (see writeScalarTypeDefinitionNumericOps). Re-wrapping in another
	// `NewType(wrap(...))` constructor would place a newtype where the inner
	// Option expects the raw scalar (E0308). Lower it like a bare scalar
	// increment and let the operator impl produce the newtype.
	out.WriteString("; *guard = Some(guard.as_ref().unwrap().clone() ")
	if op == token.INC {
		out.WriteString("+")
	} else {
		out.WriteString("-")
	}
	out.WriteString(" 1 as ")
	out.WriteString(rustType)
	out.WriteString("); }")
	return true
}

func writeIntegerTypeParamIncDec(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !goTypeParamHasIntegerConstraint(typeInfo.GetType(expr)) {
		return false
	}
	NeedGoInteger()
	out.WriteString("{ ")
	writeWrappedMutationTargetPrelude(out, expr)
	out.WriteString("let mut guard = ")
	writeWrappedMutationTargetRef(out, expr, true)
	out.WriteString("; *guard = Some(")
	if op == token.INC {
		out.WriteString("go_integer_add_one")
	} else {
		out.WriteString("go_integer_sub_one")
	}
	out.WriteString("(*guard.as_ref().unwrap())); }")
	return true
}

func writeConstIdentForCompoundExpected(out *strings.Builder, ident *ast.Ident, expected types.Type, rustName string) bool {
	if ident == nil || expected == nil {
		return false
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok {
		return false
	}
	rustType, ok := rustCastTypeForDefinedUnderlying(basic.Name())
	if !ok {
		return false
	}
	out.WriteString(rustName)
	out.WriteString(" as ")
	out.WriteString(rustType)
	return true
}

func writeIndexedCompoundAssign(out *strings.Builder, indexExpr *ast.IndexExpr, op token.Token, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("/* ERROR: Cannot determine indexed compound assignment target - type information required */ ")
		return true
	}
	if typeInfo.IsMap(indexExpr.X) {
		return false
	}
	elemType := typeInfo.GetArrayOrSliceElemType(indexExpr.X)

	out.WriteString("{ ")
	writeIndexedSequenceElementPlace(out, indexExpr.X, func() {
		out.WriteString("let __idx = ")
		writeExpressionAsUsize(out, indexExpr.Index)
		out.WriteString("; let __rhs = ")
		writeBareCompoundAssignValueForOp(out, rhs, elemType, op)
		out.WriteString("; ")
	})
	out.WriteString(" __seq[__idx] = ")
	writeIndexedCurrentElementValue(out, elemType)
	out.WriteString(" ")
	writeCompoundAssignOperator(out, op)
	// String + &str: s[i] += "x" appends.
	if op == token.ADD_ASSIGN && elemType != nil && isStringTypesType(elemType) {
		out.WriteString(" &__rhs; }")
	} else {
		out.WriteString(" __rhs; }")
	}
	return true
}

// nestedSequenceIndexChain splits seq, the sequence an element is written
// through (a[i][j] in a[i][j][k] += v), into the outermost sequence a and
// the indexes i, j that pick the inner sequence out of it. An inner slice or
// array is stored by value in the one around it (Vec<Vec<T>>), so a write
// locks a and indexes down to the element. It reports false when seq is not
// itself an element of a slice or array.
func nestedSequenceIndexChain(seq ast.Expr) (ast.Expr, []ast.Expr, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, nil, false
	}
	var indexes []ast.Expr
	cur := unwrapParens(seq)
	for {
		ix, ok := cur.(*ast.IndexExpr)
		if !ok || typeInfo.IsMap(ix.X) {
			break
		}
		// The inner sequence has to be a plain slice or array: a named one,
		// "type Row []int", keeps its elements behind its own handle.
		switch types.Unalias(typeInfo.GetType(ix)).(type) {
		case *types.Array, *types.Slice:
		default:
			return nil, nil, false
		}
		containerType := typeInfo.GetType(ix.X)
		if containerType == nil {
			return nil, nil, false
		}
		switch types.Unalias(containerType).Underlying().(type) {
		case *types.Array, *types.Slice:
		case *types.Pointer:
			// A pointer to an array ("b *[3][3]string", or "b *Board", the
			// receiver of "func (b *Board) Place") indexes the array it
			// points to.
			if _, isNamed := types.Unalias(containerType).(*types.Named); isNamed {
				return nil, nil, false
			}
			if _, pointsToArray := types.Unalias(containerType).Underlying().(*types.Pointer).Elem().Underlying().(*types.Array); !pointsToArray {
				return nil, nil, false
			}
		default:
			return nil, nil, false
		}
		indexes = append([]ast.Expr{ix.Index}, indexes...)
		cur = unwrapParens(ix.X)
	}
	if len(indexes) == 0 {
		return nil, nil, false
	}
	if ident, ok := cur.(*ast.Ident); ok {
		if _, isGoPtr := goPtrIdentPointedArrayType(ident); isGoPtr || isArrayElemPtrVar(ident.Name) {
			return nil, nil, false
		}
	}
	return cur, indexes, true
}

// outerIndexTempName names the temp that holds the index of depth i in
// writeIndexedSequenceElementPlace and writeNestedSliceElementAssignment.
func outerIndexTempName(i int) string {
	if i == 0 {
		return "__outer_idx"
	}
	return fmt.Sprintf("__outer_idx_%d", i)
}

// writeIndexedSequenceElementPlace writes the statements that bind __seq to
// the sequence that seq names, locked for writing, so that __seq[__idx] is
// the element. For a[i][j] it locks a and borrows its element i. outerIdx
// is written before the element's own index, which is the order Go
// evaluates them in; idx writes that index.
func writeIndexedSequenceElementPlace(out *strings.Builder, seq ast.Expr, idx func()) {
	root, outer, nested := nestedSequenceIndexChain(seq)
	if nested {
		for i, index := range outer {
			out.WriteString("let ")
			out.WriteString(outerIndexTempName(i))
			out.WriteString(" = ")
			writeExpressionAsUsize(out, index)
			out.WriteString("; ")
		}
	}
	idx()
	if !nested {
		writeIndexedSequenceMutationGuard(out, seq)
		out.WriteString("; let __seq = __seq_guard.as_mut().unwrap();")
		return
	}
	writeIndexedSequenceMutationGuard(out, root)
	out.WriteString("; let __seq = &mut __seq_guard.as_mut().unwrap()")
	for i := range outer {
		out.WriteString("[")
		out.WriteString(outerIndexTempName(i))
		out.WriteString("]")
	}
	out.WriteString(";")
}

func writeIndexedSequenceMutationGuard(out *strings.Builder, expr ast.Expr) {
	if subj := unwrapParens(expr); isNamedArrayExpression(subj) {
		out.WriteString("let __seq_holder = ")
		writeNamedArrayInnerHandleClone(out, subj)
		out.WriteString("; let mut __seq_guard = __seq_holder")
		WriteBorrowMethod(out, true)
		return
	}
	if subj := unwrapParens(expr); isNamedSliceExpression(subj) {
		out.WriteString("let __seq_holder = ")
		writeNamedSliceInnerHandleClone(out, subj)
		out.WriteString("; let mut __seq_guard = __seq_holder")
		WriteBorrowMethod(out, true)
		return
	}
	if _, isIdent := unwrapParens(expr).(*ast.Ident); !isIdent {
		// A field, s.items: the handle is taken out of the borrow of s
		// first, so the borrow does not have to outlive the statement.
		out.WriteString("let __seq_holder = ")
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone(); let mut __seq_guard = __seq_holder")
		WriteBorrowMethod(out, true)
		return
	}
	out.WriteString("let mut __seq_guard = ")
	TranspileExpressionContext(out, expr, LValue)
	WriteBorrowMethod(out, true)
}

func writeIndexedCurrentElementValue(out *strings.Builder, elemType types.Type) {
	out.WriteString("__seq[__idx]")
	if !typeIsPredeclaredCopyScalar(elemType) {
		out.WriteString(".clone()")
	}
}

// mapCommaOkAssignTakesOverHandle reports whether "v, ok = m[k]" makes the
// existing slice or map variable or field v the same slice or map as the
// entry by replacing its handle, as the plain "v = m[k]" does.
func mapCommaOkAssignTakesOverHandle(lhs ast.Expr, indexExpr *ast.IndexExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	switch target := lhs.(type) {
	case *ast.Ident:
		if target.Name == "_" || isPackageGlobalIdent(target) {
			return false
		}
		if isCurrentReceiverIdent(target) && currentReceiverRustAlias != "" {
			return false
		}
		if currentCaptureRenames != nil {
			if _, captured := currentCaptureRenames[target.Name]; captured {
				return false
			}
		}
	case *ast.SelectorExpr:
	default:
		return false
	}
	valueType := typeInfo.GetMapValueType(indexExpr.X)
	if valueType == nil {
		return false
	}
	switch types.Unalias(valueType).Underlying().(type) {
	case *types.Slice:
		return isPlainSliceExpression(lhs)
	case *types.Map:
		return typeInfo.IsMap(lhs) && !isNamedMapExpression(lhs)
	}
	return false
}

// mapCommaOkValueIsSliceOrMap reports whether the values of the map read by
// indexExpr are slices or maps, of a named type or not. The map's cell for
// such a value can be cloned: the clone of a named slice shares its backing
// array, as a copied slice header does in Go.
func mapCommaOkValueIsSliceOrMap(indexExpr *ast.IndexExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	valueType := typeInfo.GetMapValueType(indexExpr.X)
	if valueType == nil {
		return false
	}
	switch types.Unalias(valueType).Underlying().(type) {
	case *types.Slice, *types.Map:
		return true
	}
	return false
}

// writeMapCommaOkFoundValue writes the first result of "v, ok := m[k]" for a
// key that is in the map, where the map's cell for the value is bound to v
// in the generated match arm. A pointer, slice, map, channel, function or
// interface value is its handle. Any other value is copied into a cell of
// its own: handing out the map's cell let "v = 5" after the read change the
// map, and let "v, ok = m[k]" into existing variables take the value out of
// the map.
func writeMapCommaOkFoundValue(out *strings.Builder, indexExpr *ast.IndexExpr, syntaxKeepsHandle bool, valueDiscarded bool, heldInOwnCell bool) {
	if valueDiscarded {
		// "_, ok := m[k]" drops the value, so the handle will do.
		out.WriteString("v.clone()")
		return
	}
	if heldInOwnCell {
		// The value is about to be moved into an existing target. Moving it
		// out of the map's own handle would empty the map entry, so the
		// target gets a clone of what the handle holds, in a cell of its own.
		out.WriteString("{ let __held = v")
		WriteBorrowMethod(out, false)
		out.WriteString(".clone(); ")
		WriteWrapperOptionPrefix(out)
		out.WriteString("__held")
		WriteWrapperOptionSuffix(out)
		out.WriteString(" }")
		return
	}
	if syntaxKeepsHandle {
		out.WriteString("v.clone()")
		return
	}
	var valueType types.Type
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		valueType = typeInfo.GetMapValueType(indexExpr.X)
	}
	if valueType == nil {
		out.WriteString(goValueCopyNeedsTypeInfo)
		return
	}
	if mapValueTypeKeepsHandle(valueType) {
		out.WriteString("v.clone()")
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("v")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	out.WriteString(goValueCopyCallForType(valueType))
	WriteWrapperSuffix(out)
}

func writeMapCommaOkMissingValue(out *strings.Builder, indexExpr *ast.IndexExpr, syntaxKeepsHandle bool) {
	typeInfo := GetTypeInfo()
	if syntaxKeepsHandle && typeInfo == nil {
		out.WriteString("Default::default()")
		return
	}
	if typeInfo == nil {
		out.WriteString("/* ERROR: Type information required for map comma-ok zero value */ unimplemented!(\"type info required for map comma-ok zero value\")")
		return
	}

	valueType := typeInfo.GetMapValueType(indexExpr.X)
	if valueType == nil {
		out.WriteString("/* ERROR: Map value type required for map comma-ok zero value */ unimplemented!(\"map value type required for map comma-ok zero value\")")
		return
	}
	if syntaxKeepsHandle {
		writeMapLookupHandleZero(out, valueType, zeroValueForTypesType(valueType))
		return
	}
	if mapValueTypeKeepsHandle(valueType) {
		writeMapLookupHandleZero(out, valueType, zeroValueForTypesType(valueType))
		return
	}

	WriteWrapperPrefix(out)
	out.WriteString(zeroValueForTypesType(valueType))
	WriteWrapperSuffix(out)
}

func writeParallelAssignmentTarget(out *strings.Builder, lhs ast.Expr, tmpName string, rhs ast.Expr) {
	if ident, ok := lhs.(*ast.Ident); ok && ident.Name == "_" {
		return
	}

	tmpWrapped := tempHoldsWrappedValue(rhs)
	if writeParallelCurrentReceiverAssignmentTarget(out, lhs, tmpName, tmpWrapped) {
		return
	}
	if indexExpr, ok := lhs.(*ast.IndexExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo == nil {
			out.WriteString(" /* ERROR: Cannot determine indexed assignment target - type information required */ ")
			return
		}
		if writeIndexedSequenceAssignmentFromTemp(out, indexExpr, tmpName, tmpWrapped, rhs) {
			return
		}
	}
	if transpiledInterfaceHandleAssignmentTypes(lhs, rhs) {
		out.WriteString(" ")
		writeTranspiledInterfaceHandleAssignmentTarget(out, lhs, tmpName)
		return
	}
	if writeParallelSliceElemPtrAssignmentTarget(out, lhs, tmpName, rhs) {
		return
	}
	if writeParallelMapHandleAssignmentTarget(out, lhs, tmpName, rhs) {
		return
	}
	if writeParallelNilAssignmentTarget(out, lhs, tmpName, rhs) {
		return
	}
	if writeParallelPointerHandleAssignmentTarget(out, lhs, tmpName, rhs) {
		return
	}
	if ident, ok := lhs.(*ast.Ident); ok && isVarBare(ident.Name) {
		out.WriteString(" ")
		out.WriteString(rustIdentForUseWithCapture(ident))
		out.WriteString(" = ")
		if tmpWrapped {
			out.WriteString("(*")
			out.WriteString(tmpName)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone()")
		} else {
			out.WriteString(tmpName)
		}
		if cast := parallelTempBareCast(lhs, rhs); cast != "" {
			out.WriteString(" as ")
			out.WriteString(cast)
		}
		out.WriteString(";")
		return
	}

	out.WriteString(" *")
	writeTempAssignmentTargetRef(out, lhs)
	out.WriteString(" = ")
	if tmpWrapped {
		out.WriteString(tmpName)
		WriteBorrowMethod(out, true)
		out.WriteString(".take();")
	} else {
		out.WriteString("Some(")
		if wrapper := parallelTempNamedIntegerWrap(lhs, rhs); wrapper != "" {
			out.WriteString(wrapper)
			out.WriteString("(")
			WriteWrapperPrefix(out)
			out.WriteString(tmpName)
			if cast := parallelTempNamedIntegerCast(lhs); cast != "" {
				out.WriteString(" as ")
				out.WriteString(cast)
			}
			WriteWrapperSuffix(out)
			out.WriteString(")")
		} else {
			out.WriteString(tmpName)
			if cast := parallelTempBareCast(lhs, rhs); cast != "" {
				out.WriteString(" as ")
				out.WriteString(cast)
			}
		}
		out.WriteString(");")
	}
}

func writeParallelAssignmentTargetLine(out *strings.Builder, lhs ast.Expr, tmpName string, rhs ast.Expr, indent string) {
	var line strings.Builder
	writeParallelAssignmentTarget(&line, lhs, tmpName, rhs)
	text := strings.TrimLeft(line.String(), " \t")
	if text == "" {
		return
	}
	out.WriteString(indent)
	out.WriteString(text)
	out.WriteString("\n")
}

type parallelIndexedAssignmentTarget struct {
	indexExpr     *ast.IndexExpr
	holdsSequence bool
	sequenceName  string
	indexName     string
	// For a[i][j], the outermost sequence a and the names of the temps that
	// hold i: the inner sequence is stored by value inside a.
	nestedRoot      ast.Expr
	outerIndexNames []string
}

type selectOperands struct {
	channel string
	value   string
}

// Go evaluates all channel operands and send values, in source order,
// before choosing a ready arm. The polling loop can then reuse those values.
func prepareSelectOperands(out *strings.Builder, clauses []*ast.CommClause) []selectOperands {
	operands := make([]selectOperands, len(clauses))
	for i, cc := range clauses {
		var channel, value ast.Expr
		var receive ast.Expr
		switch comm := cc.Comm.(type) {
		case *ast.AssignStmt:
			if len(comm.Rhs) == 1 {
				receive = comm.Rhs[0]
			}
		case *ast.ExprStmt:
			receive = comm.X
		case *ast.SendStmt:
			channel, value = comm.Chan, comm.Value
		}
		if unary, ok := unwrapParens(receive).(*ast.UnaryExpr); ok && unary.Op == token.ARROW {
			channel = unary.X
		}
		if channel == nil {
			continue
		}
		name := freshParallelAssignmentTempName(fmt.Sprintf("__go_select_channel_%d", i))
		operands[i].channel = name
		fmt.Fprintf(out, "let %s = ", name)
		writeChannelExpression(out, channel)
		out.WriteString(".clone();\n    ")
		if value != nil {
			NeedGoSelectSend()
			name = freshParallelAssignmentTempName(fmt.Sprintf("__go_select_value_%d", i))
			operands[i].value = name
			fmt.Fprintf(out, "let mut %s = Some(", name)
			if !writeInterfaceChannelSendValue(out, channel, value) && !writeFuncChannelSendValue(out, channel, value) {
				transpileChannelValue(out, value)
			}
			out.WriteString(");\n    ")
		}
	}
	return operands
}

func freshParallelAssignmentTempName(base string) string {
	name := base
	for lookupVarInfo(name) != nil || packageGlobalNames[name] {
		name += "_"
	}
	return name
}

// parallelMapAssignmentTarget is a map element m[k] on the left of a
// parallel assignment. Go evaluates the map operand and the key before any
// value is stored, and stores by inserting, as m[k] = v does.
type parallelMapAssignmentTarget struct {
	indexExpr *ast.IndexExpr
	mapName   string
	keyName   string
}

func prepareParallelMapAssignmentTarget(out *strings.Builder, lhs ast.Expr, rhs ast.Expr, position int, indent string) (parallelMapAssignmentTarget, bool) {
	indexExpr, ok := unwrapParens(lhs).(*ast.IndexExpr)
	if !ok {
		return parallelMapAssignmentTarget{}, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsMap(indexExpr.X) {
		return parallelMapAssignmentTarget{}, false
	}
	target := parallelMapAssignmentTarget{
		indexExpr: indexExpr,
		keyName:   freshParallelAssignmentTempName(fmt.Sprintf("__go_lhs_key_%d", position)),
	}
	if mapOperandHandleCanBeHeld(indexExpr.X) {
		target.mapName = freshParallelAssignmentTempName(fmt.Sprintf("__go_lhs_map_%d", position))
		out.WriteString(indent)
		out.WriteString("let ")
		out.WriteString(target.mapName)
		out.WriteString(" = ")
		writeMapHandleCloneForOp(out, indexExpr.X)
		out.WriteString(";\n")
	}
	var keyRustType string
	if syntaxKeyType, ok := localMapKeyRustType(indexExpr.X); ok {
		keyRustType = syntaxKeyType
	}
	keyType, _ := typeInfo.GetMapTypes(indexExpr.X)
	out.WriteString(indent)
	out.WriteString("let ")
	out.WriteString(target.keyName)
	out.WriteString(" = ")
	if mapAssignmentKeyRequiresTypeDrivenConversion(keyType) || !writeMapAssignmentKeyExpressionWithRustType(out, indexExpr.Index, keyRustType) {
		writeMapAssignmentKeyExpression(out, indexExpr.Index, keyType, rhs)
	}
	out.WriteString(";\n")
	return target, true
}

// writeParallelMapAssignmentTempValue writes the value a parallel assignment
// stores in a map element, built as m[k] = v builds it, so the temp is a
// fresh map value rather than a handle to another element's cell.
func writeParallelMapAssignmentTempValue(out *strings.Builder, indexExpr *ast.IndexExpr, rhs ast.Expr) {
	if writeSliceElemPtrMapValue(out, indexExpr.X, rhs) {
		return
	}
	var valueType types.Type
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		_, valueType = typeInfo.GetMapTypes(indexExpr.X)
	}
	writeMapWrappedValue(out, rhs, valueType)
}

func writeParallelMapAssignmentTargetLine(out *strings.Builder, target parallelMapAssignmentTarget, tmpName string, indent string) {
	out.WriteString(indent)
	out.WriteString("(*")
	if target.mapName != "" {
		out.WriteString(target.mapName)
	} else {
		writeMapHandleForOp(out, target.indexExpr.X)
	}
	WriteBorrowMethod(out, true)
	writeMapMutationUnwrap(out, target.indexExpr.X)
	out.WriteString(").insert(")
	out.WriteString(target.keyName)
	out.WriteString(", ")
	out.WriteString(tmpName)
	out.WriteString(");\n")
}

func prepareParallelIndexedAssignmentTarget(out *strings.Builder, lhs ast.Expr, position int, indent string) (parallelIndexedAssignmentTarget, bool) {
	indexExpr, ok := unwrapParens(lhs).(*ast.IndexExpr)
	if !ok {
		return parallelIndexedAssignmentTarget{}, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.IsMap(indexExpr.X) {
		return parallelIndexedAssignmentTarget{}, false
	}
	sequenceType := typeInfo.GetType(indexExpr.X)
	if sequenceType == nil {
		return parallelIndexedAssignmentTarget{}, false
	}
	switch types.Unalias(sequenceType).Underlying().(type) {
	case *types.Array, *types.Slice:
	default:
		return parallelIndexedAssignmentTarget{}, false
	}
	if ident, ok := unwrapParens(indexExpr.X).(*ast.Ident); ok {
		if _, isGoPtr := goPtrIdentPointedArrayType(ident); isGoPtr || isArrayElemPtrVar(ident.Name) {
			return parallelIndexedAssignmentTarget{}, false
		}
	}

	target := parallelIndexedAssignmentTarget{
		indexExpr: indexExpr,
		indexName: freshParallelAssignmentTempName(fmt.Sprintf("__go_lhs_index_%d", position)),
	}
	if root, outer, nested := nestedSequenceIndexChain(indexExpr.X); nested {
		return prepareParallelNestedIndexedAssignmentTarget(out, target, root, outer, position, indent)
	}
	// Local sequence values and call results are cloneable handles. Holding the
	// handle is required when an earlier assignment in this statement rebinds
	// the slice variable: Go has already evaluated the indexed target by then.
	// Package globals own their value in GoGlobal rather than exposing such a
	// handle, but naming one has no side effects, so only its index needs a temp.
	target.holdsSequence = mapOperandHandleCanBeHeld(indexExpr.X)
	if expressionCallsOrReceives(indexExpr.X) && !target.holdsSequence {
		return parallelIndexedAssignmentTarget{}, false
	}
	if target.holdsSequence {
		target.sequenceName = freshParallelAssignmentTempName(fmt.Sprintf("__go_lhs_sequence_%d", position))
		out.WriteString(indent)
		out.WriteString("let ")
		out.WriteString(target.sequenceName)
		out.WriteString(" = ")
		if subj := unwrapParens(indexExpr.X); isNamedSliceExpression(subj) {
			writeNamedSliceInnerHandleClone(out, subj)
		} else if subj := unwrapParens(indexExpr.X); isNamedArrayExpression(subj) {
			writeNamedArrayInnerHandleClone(out, subj)
		} else {
			TranspileExpressionContext(out, indexExpr.X, LValue)
		}
		out.WriteString(".clone();\n")
	}
	out.WriteString(indent)
	out.WriteString("let ")
	out.WriteString(target.indexName)
	out.WriteString(" = ")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString(";\n")
	return target, true
}

// prepareParallelNestedIndexedAssignmentTarget evaluates the target
// a[i][j] of a parallel assignment: the handle of a when it can be held, then
// i and j, all before any value is stored.
func prepareParallelNestedIndexedAssignmentTarget(out *strings.Builder, target parallelIndexedAssignmentTarget, root ast.Expr, outer []ast.Expr, position int, indent string) (parallelIndexedAssignmentTarget, bool) {
	target.nestedRoot = root
	target.holdsSequence = mapOperandHandleCanBeHeld(root)
	if expressionCallsOrReceives(root) && !target.holdsSequence {
		return parallelIndexedAssignmentTarget{}, false
	}
	if target.holdsSequence {
		target.sequenceName = freshParallelAssignmentTempName(fmt.Sprintf("__go_lhs_sequence_%d", position))
		out.WriteString(indent)
		out.WriteString("let ")
		out.WriteString(target.sequenceName)
		out.WriteString(" = ")
		if isNamedSliceExpression(root) {
			writeNamedSliceInnerHandleClone(out, root)
		} else if isNamedArrayExpression(root) {
			writeNamedArrayInnerHandleClone(out, root)
		} else {
			TranspileExpressionContext(out, root, LValue)
		}
		out.WriteString(".clone();\n")
	}
	for i, index := range outer {
		name := freshParallelAssignmentTempName(fmt.Sprintf("__go_lhs_outer_index_%d_%d", position, i))
		target.outerIndexNames = append(target.outerIndexNames, name)
		out.WriteString(indent)
		out.WriteString("let ")
		out.WriteString(name)
		out.WriteString(" = ")
		writeExpressionAsUsize(out, index)
		out.WriteString(";\n")
	}
	out.WriteString(indent)
	out.WriteString("let ")
	out.WriteString(target.indexName)
	out.WriteString(" = ")
	writeExpressionAsUsize(out, target.indexExpr.Index)
	out.WriteString(";\n")
	return target, true
}

func writeParallelAssignmentPreparedIndexTargetLine(out *strings.Builder, target parallelIndexedAssignmentTarget, tmpName string, rhs ast.Expr, indent string) {
	tmpWrapped := tempHoldsWrappedValue(rhs)
	typeInfo := GetTypeInfo()
	elemType := indexedSequenceElementAssignmentType(typeInfo, target.indexExpr)
	elemKeepsHandle := indexedSequenceElementKeepsHandle(elemType)
	if !elemKeepsHandle {
		elemKeepsHandle = tupleTempAssignsHandleToElementBySyntax(target.indexExpr.X)
	}

	out.WriteString(indent)
	out.WriteString("(*")
	if target.holdsSequence {
		out.WriteString(target.sequenceName)
	} else if target.nestedRoot != nil {
		if isNamedSliceExpression(target.nestedRoot) {
			writeNamedSliceInnerHandleClone(out, target.nestedRoot)
		} else if isNamedArrayExpression(target.nestedRoot) {
			writeNamedArrayInnerHandleClone(out, target.nestedRoot)
		} else {
			TranspileExpressionContext(out, target.nestedRoot, LValue)
		}
	} else if subj := unwrapParens(target.indexExpr.X); isNamedSliceExpression(subj) {
		writeNamedSliceInnerHandleClone(out, subj)
	} else if subj := unwrapParens(target.indexExpr.X); isNamedArrayExpression(subj) {
		writeNamedArrayInnerHandleClone(out, subj)
	} else {
		TranspileExpressionContext(out, target.indexExpr.X, LValue)
	}
	WriteBorrowMethod(out, true)
	out.WriteString(".as_mut().unwrap())")
	for _, name := range target.outerIndexNames {
		out.WriteString("[")
		out.WriteString(name)
		out.WriteString("]")
	}
	out.WriteString("[")
	out.WriteString(target.indexName)
	out.WriteString("] = ")
	writeIndexedSequenceTempAssignmentValue(out, target.indexExpr, tmpName, tmpWrapped, rhs, elemKeepsHandle)
	out.WriteString(";\n")
}

func writePointerHandleAssignmentFromTemp(out *strings.Builder, lhs ast.Expr, tmpName string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(lhs) {
		return false
	}
	switch lhs.(type) {
	case *ast.Ident, *ast.SelectorExpr:
	default:
		return false
	}
	if ident, ok := lhs.(*ast.Ident); ok && isCurrentReceiverIdent(ident) && currentReceiverRustAlias != "" {
		return false
	}
	out.WriteString(" ")
	writePointerHandleAssignmentTempTarget(out, lhs, tmpName)
	return true
}

func writeParallelPointerHandleAssignmentTarget(out *strings.Builder, lhs ast.Expr, tmpName string, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(lhs) || !typeInfo.IsPointer(rhs) {
		return false
	}
	switch lhs.(type) {
	case *ast.Ident, *ast.SelectorExpr:
	default:
		return false
	}
	if ident, ok := lhs.(*ast.Ident); ok && isCurrentReceiverIdent(ident) && currentReceiverRustAlias != "" {
		return false
	}
	out.WriteString(" ")
	writePointerHandleAssignmentTempTarget(out, lhs, tmpName)
	return true
}

func writePointerHandleAssignmentTempTarget(out *strings.Builder, lhs ast.Expr, tmpName string) {
	if sel, ok := unwrapParens(lhs).(*ast.SelectorExpr); ok {
		valueName := tmpName + ".clone()"
		if writeGoPtrLocalSelectorHandleReplacement(out, sel, valueName) {
			return
		}
		if writeGoPtrFieldSelectorHandleReplacement(out, sel, valueName) {
			return
		}
	}
	if ident, ok := packageGlobalPointerIdent(lhs); ok {
		out.WriteString("*")
		out.WriteString(rustPackageGlobalName(ident.Name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = Some(")
		out.WriteString(tmpName)
		out.WriteString(".clone());")
		return
	}
	writePointerHandleAssignmentTarget(out, lhs)
	out.WriteString(" = ")
	out.WriteString(tmpName)
	out.WriteString(".clone();")
}

func writeParallelSliceElemPtrAssignmentTarget(out *strings.Builder, lhs ast.Expr, tmpName string, rhs ast.Expr) bool {
	ident, ok := lhs.(*ast.Ident)
	if !ok || !isSliceElemPtrVar(ident.Name) {
		return false
	}
	out.WriteString(" ")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(" = ")
	if rhsIdent, ok := unwrapParens(rhs).(*ast.Ident); ok {
		if rhsIdent.Name == "nil" {
			out.WriteString("None")
			out.WriteString(";")
			return true
		}
		if isSliceElemPtrVar(rhsIdent.Name) {
			out.WriteString(tmpName)
			out.WriteString(";")
			return true
		}
	}
	if _, sawSliceAddr := isSliceElemPtrAssignmentValue(rhs); sawSliceAddr {
		out.WriteString("Some(")
		out.WriteString(tmpName)
		out.WriteString(");")
		return true
	}
	if call, ok := unwrapParens(rhs).(*ast.CallExpr); ok {
		if _, ok := sliceElemPtrReturnInfoForCall(call); ok {
			out.WriteString(tmpName)
			out.WriteString(";")
			return true
		}
	}
	out.WriteString("/* ERROR: slice element pointer assignment requires nil or &slice[index] */ unimplemented!(\"slice element pointer assignment\")")
	out.WriteString(";")
	return true
}

// writeParallelMapHandleAssignmentTarget stores a map, or nil, into a map
// variable or field of a parallel assignment by rebinding its handle, as
// writeMapHandleAssignment does for a single assignment. The handle is the
// map itself, so writing into it would change every variable that shares
// the old map, and a, b = b, a would empty one of them.
func writeParallelMapHandleAssignmentTarget(out *strings.Builder, lhs ast.Expr, tmpName string, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsMap(lhs) {
		return false
	}
	globalTarget := ""
	switch target := unwrapParens(lhs).(type) {
	case *ast.Ident:
		// A package-level map variable takes the temp's handle (#328).
		if packageGlobalIdentIsMapHandle(target) {
			globalTarget = rustPackageGlobalName(target.Name)
		} else if isPackageGlobalIdent(target) || isVarBare(target.Name) {
			return false
		}
		if isCurrentReceiverIdent(target) && currentReceiverRustAlias != "" {
			return false
		}
	case *ast.SelectorExpr:
	default:
		return false
	}
	if ident, ok := unwrapParens(rhs).(*ast.Ident); !(ok && ident.Name == "nil") && !typeInfo.IsMap(rhs) {
		return false
	}
	if isNamedMapExpression(lhs) || isNamedMapExpression(rhs) {
		return false
	}
	if globalTarget != "" {
		out.WriteString(" ")
		out.WriteString(globalTarget)
		out.WriteString(".set(")
		out.WriteString(tmpName)
		out.WriteString(");")
		return true
	}
	// For nil, writeParallelAssignmentTempValueForTarget gave the temp a
	// nil map of its own.
	out.WriteString(" ")
	writePointerHandleAssignmentTargetFromValueName(out, lhs, tmpName)
	return true
}

func writeParallelNilAssignmentTarget(out *strings.Builder, lhs ast.Expr, tmpName string, rhs ast.Expr) bool {
	ident, ok := rhs.(*ast.Ident)
	if !ok || ident.Name != "nil" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString(" /* ERROR: Type information required for parallel nil assignment */ unimplemented!(\"type info required for parallel nil assignment\");")
		return true
	}
	lhsType := typeInfo.GetType(lhs)
	if lhsType == nil {
		out.WriteString(" /* ERROR: Type information required for parallel nil assignment target */ unimplemented!(\"type info required for parallel nil assignment target\");")
		return true
	}
	if sel, ok := parallelNilGoPtrFieldAssignmentTarget(lhs, rhs); ok {
		out.WriteString(" ")
		writePointerHandleAssignmentTargetFromValueName(out, sel, tmpName+".clone()")
		return true
	}
	if target, ok := parallelNilGoPtrIdentAssignmentTarget(lhs, rhs); ok {
		out.WriteString(" ")
		out.WriteString(rustIdentForUseWithCapture(target))
		out.WriteString(" = ")
		out.WriteString(tmpName)
		out.WriteString(".clone();")
		return true
	}
	if !tupleTempAssignsHandleToElement(lhsType) {
		return false
	}
	out.WriteString(" *")
	writeTempAssignmentTargetRef(out, lhs)
	out.WriteString(" = ")
	out.WriteString(tmpName)
	out.WriteString(";")
	return true
}

func writeParallelCurrentReceiverAssignmentTarget(out *strings.Builder, lhs ast.Expr, tmpName string, tmpWrapped bool) bool {
	ident, ok := lhs.(*ast.Ident)
	if !ok || !isCurrentReceiverIdent(ident) || currentReceiverRustAlias == "" {
		return false
	}
	out.WriteString(" ")
	if tmpWrapped {
		out.WriteString("{ let __moved_val = { let mut __guard = ")
		out.WriteString(tmpName)
		WriteBorrowMethod(out, true)
		out.WriteString("; __guard.take().unwrap() }; ")
		out.WriteString(currentReceiverRustAlias)
		out.WriteString(" = __moved_val; }")
		return true
	}
	out.WriteString(currentReceiverRustAlias)
	out.WriteString(" = ")
	out.WriteString(tmpName)
	out.WriteString(";")
	return true
}

func writeParallelAssignmentTempValueForTarget(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) {
	if _, ok := parallelNilGoPtrFieldAssignmentTarget(lhs, rhs); ok {
		out.WriteString("GoPtr::nil()")
		return
	}
	if _, ok := parallelNilGoPtrIdentAssignmentTarget(lhs, rhs); ok {
		out.WriteString("GoPtr::nil()")
		return
	}
	if parallelMapHandleTargetTakesNil(lhs, rhs) {
		WriteWrappedNone(out)
		return
	}
	writeParallelAssignmentTempValue(out, rhs)
}

// parallelMapHandleTargetTakesNil reports whether lhs = nil in a parallel
// assignment rebinds a map handle (writeParallelMapHandleAssignmentTarget),
// whose temp is then a wrapped nil map.
func parallelMapHandleTargetTakesNil(lhs ast.Expr, rhs ast.Expr) bool {
	ident, ok := unwrapParens(rhs).(*ast.Ident)
	if !ok || ident.Name != "nil" {
		return false
	}
	var probe strings.Builder
	return writeParallelMapHandleAssignmentTarget(&probe, lhs, "__probe", rhs)
}

func writeParallelAssignmentTempValue(out *strings.Builder, rhs ast.Expr) {
	if writeParallelInterfaceHandleTempValue(out, rhs) {
		return
	}
	if writeParallelHandleTempValue(out, rhs) {
		return
	}
	if writeOwnedExpressionValue(out, rhs) {
		return
	}
	TranspileExpression(out, rhs)
}

func parallelNilGoPtrFieldAssignmentTarget(lhs ast.Expr, rhs ast.Expr) (*ast.SelectorExpr, bool) {
	ident, ok := unwrapParens(rhs).(*ast.Ident)
	if !ok || ident.Name != "nil" {
		return nil, false
	}
	sel, ok := unwrapParens(lhs).(*ast.SelectorExpr)
	if !ok || !generatedGoPtrFieldForSelector(sel) {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	lhsType := typeInfo.GetType(lhs)
	if lhsType == nil || !tupleTempAssignsHandleToElement(lhsType) {
		return nil, false
	}
	return sel, true
}

func parallelNilGoPtrIdentAssignmentTarget(lhs ast.Expr, rhs ast.Expr) (*ast.Ident, bool) {
	rhsIdent, ok := unwrapParens(rhs).(*ast.Ident)
	if !ok || rhsIdent.Name != "nil" {
		return nil, false
	}
	lhsIdent, ok := unwrapParens(lhs).(*ast.Ident)
	if !ok || lhsIdent.Name == "_" || !isGoPtrVar(lhsIdent.Name) {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	lhsType := typeInfo.GetType(lhs)
	if lhsType == nil {
		if info, ok := goPtrVarInfo(lhsIdent.Name); ok {
			lhsType = info.GoType
		}
	}
	if lhsType == nil || !tupleTempAssignsHandleToElement(lhsType) {
		return nil, false
	}
	return lhsIdent, true
}

func writeParallelInterfaceHandleTempValue(out *strings.Builder, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(rhs)); !ok {
		return false
	}
	TranspileExpressionContext(out, rhs, LValue)
	out.WriteString(".clone()")
	return true
}

func writeParallelHandleTempValue(out *strings.Builder, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	rhsType := typeInfo.GetType(rhs)
	if writeParallelNamedCollectionTempValue(out, rhs, rhsType) {
		return true
	}
	if parallelTempTypeIsNamedCollection(rhsType) {
		return false
	}
	if !parallelTempTypeKeepsHandle(rhsType) {
		return false
	}
	switch expr := rhs.(type) {
	case *ast.Ident:
		if expr.Name == "nil" || expr.Name == "_" {
			return false
		}
		if _, ok := types.Unalias(rhsType).Underlying().(*types.Pointer); ok {
			writePointerShortDeclRhsValue(out, rhs)
			return true
		}
		TranspileExpressionContext(out, rhs, LValue)
		out.WriteString(".clone()")
	case *ast.SelectorExpr:
		writeSelectorHandleClone(out, expr)
	case *ast.IndexExpr, *ast.TypeAssertExpr:
		TranspileExpression(out, rhs)
	default:
		return false
	}
	return true
}

func writeParallelNamedCollectionTempValue(out *strings.Builder, rhs ast.Expr, rhsType types.Type) bool {
	if !parallelTempTypeIsNamedCollection(rhsType) {
		return false
	}
	ident, ok := rhs.(*ast.Ident)
	if !ok || ident.Name == "nil" || ident.Name == "_" {
		return false
	}
	if isCurrentReceiverIdent(ident) {
		WriteWrapperPrefix(out)
		out.WriteString(currentReceiverRustName())
		out.WriteString(".clone()")
		WriteWrapperSuffix(out)
		return true
	}
	TranspileExpressionContext(out, rhs, LValue)
	out.WriteString(".clone()")
	return true
}

func parallelTempTypeIsNamedCollection(typ types.Type) bool {
	if _, _, ok := namedSliceTypeFromType(typ); ok {
		return true
	}
	if _, _, ok := namedMapTypeFromType(typ); ok {
		return true
	}
	return false
}

func parallelTempTypeKeepsHandle(typ types.Type) bool {
	if typ == nil {
		return false
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Pointer, *types.Slice, *types.Map, *types.Chan, *types.Signature:
		return true
	default:
		return false
	}
}

func parallelTempNamedIntegerWrap(lhs ast.Expr, rhs ast.Expr) string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return ""
	}
	lhsType := typeInfo.GetType(lhs)
	if lhsType == nil {
		return ""
	}
	named, ok := types.Unalias(lhsType).(*types.Named)
	if !ok || !isNamedIntegerType(named) {
		return ""
	}
	// RHS already a named-typed expression of the same type? Then no wrap.
	// Named constants still emit as primitive const values, so they must be
	// reconstructed at the assignment target.
	if rhsType := typeInfo.GetType(rhs); rhsType != nil {
		if rhsNamed, ok := types.Unalias(rhsType).(*types.Named); ok && rhsNamed.Obj() == named.Obj() {
			if _, isLit := rhs.(*ast.BasicLit); !isLit && !isConstObjectExpr(rhs) {
				return ""
			}
		}
	}
	return goTypesNamedTypeToRust(named)
}

func parallelTempNamedIntegerCast(lhs ast.Expr) string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return ""
	}
	lhsType := typeInfo.GetType(lhs)
	if lhsType == nil {
		return ""
	}
	named, ok := types.Unalias(lhsType).(*types.Named)
	if !ok {
		return ""
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok {
		return ""
	}
	cast, _ := rustCastTypeForDefinedUnderlying(basic.Name())
	return cast
}

func isConstObjectExpr(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	switch e := expr.(type) {
	case *ast.Ident:
		_, ok := typeInfo.GetObject(e).(*types.Const)
		return ok
	case *ast.SelectorExpr:
		_, ok := typeInfo.GetObject(e.Sel).(*types.Const)
		return ok
	default:
		return false
	}
}

func parallelTempBareCast(lhs ast.Expr, rhs ast.Expr) string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return ""
	}
	lhsType := typeInfo.GetType(lhs)
	if lhsType == nil {
		return ""
	}
	lhsCast := rustCastForExpectedBasic(lhsType)
	if lhsCast == "" {
		return ""
	}
	rhsCast := rhsEmittedRustCast(rhs)
	if rhsCast != "" && rhsCast == lhsCast {
		return ""
	}
	if rhsCast == "" && (lhsCast == "i32" || lhsCast == rustIntType()) {
		// Untyped expression emits as i32 by default; matches LHS.
		return ""
	}
	return lhsCast
}

// rhsEmittedRustCast returns the Rust basic type the RHS expression actually
// emits, or "" if it's an untyped literal that defaults to i32. Unlike
// rustCastForExpectedBasic(typeInfo.GetType(rhs)), this avoids Go's contextual
// type adjustment (e.g. an untyped const taking the LHS type in a
// multi-assignment) so we can decide whether an explicit cast is needed.
func rhsEmittedRustCast(rhs ast.Expr) string {
	if isBareLenCapCall(unwrapParens(rhs)) {
		return "usize"
	}
	if _, ok := rhs.(*ast.BasicLit); ok {
		return "" // untyped literal
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return ""
	}
	switch e := rhs.(type) {
	case *ast.SelectorExpr:
		if obj, ok := typeInfo.info.Uses[e.Sel]; ok {
			return rustCastForExpectedBasic(obj.Type())
		}
	case *ast.Ident:
		if obj, ok := typeInfo.info.Uses[e]; ok {
			return rustCastForExpectedBasic(obj.Type())
		}
	}
	if typ := typeInfo.GetType(rhs); typ != nil {
		return rustCastForExpectedBasic(typ)
	}
	return ""
}

type mutexReceiverInfo struct {
	expr         ast.Expr
	fields       []string
	sourceMapped bool
}

// mutexLockReceiver returns the receiver for a Lock() call on a sync.Mutex field.
func mutexLockReceiver(expr ast.Expr) (mutexReceiverInfo, bool) {
	return syncMutexMethodReceiver(expr, "Lock")
}

func mutexUnlockReceiver(expr ast.Expr) (mutexReceiverInfo, bool) {
	return syncMutexMethodReceiver(expr, "Unlock")
}

func syncMutexMethodReceiver(expr ast.Expr, methodName string) (mutexReceiverInfo, bool) {
	call, ok := expr.(*ast.CallExpr)
	if !ok {
		return mutexReceiverInfo{}, false
	}
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != methodName {
		return mutexReceiverInfo{}, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return mutexReceiverInfo{}, false
	}
	fieldType := typeInfo.GetType(sel.X)
	if isSyncMutexType(fieldType) {
		return mutexReceiverInfo{expr: sel.X, sourceMapped: isSourceMappedSyncMutexType(fieldType)}, true
	}
	if typeInfo.info == nil {
		return mutexReceiverInfo{}, false
	}
	selection := typeInfo.info.Selections[sel]
	if selection == nil {
		return mutexReceiverInfo{}, false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok || fn.Name() != methodName {
		return mutexReceiverInfo{}, false
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Recv() == nil || !isSyncMutexType(sig.Recv().Type()) {
		return mutexReceiverInfo{}, false
	}
	indexes := selection.Index()
	if len(indexes) < 2 {
		return mutexReceiverInfo{}, false
	}
	fields, ok := promotedFieldPath(selection.Recv(), indexes[:len(indexes)-1])
	if !ok {
		return mutexReceiverInfo{}, false
	}
	return mutexReceiverInfo{expr: sel.X, fields: fields, sourceMapped: isSourceMappedSyncMutexType(sig.Recv().Type())}, true
}

func syncMutexNamedType(typ types.Type) (*types.Named, bool) {
	if typ == nil {
		return nil, false
	}
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		typ = ptr.Elem()
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return nil, false
	}
	if named.Obj().Pkg().Path() != "sync" || named.Obj().Name() != "Mutex" {
		return nil, false
	}
	return named, true
}

func isSyncMutexType(typ types.Type) bool {
	_, ok := syncMutexNamedType(typ)
	return ok
}

func isSourceMappedSyncMutexType(typ types.Type) bool {
	named, ok := syncMutexNamedType(typ)
	return ok && isSourceMappedPackagePath(named.Obj().Pkg().Path())
}

func isLocalSourceMappedSyncMutexFieldSelector(sel *ast.SelectorExpr) bool {
	if sel == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	named, ok := syncMutexNamedType(typeInfo.GetType(sel))
	if !ok || !isSourceMappedPackagePath(named.Obj().Pkg().Path()) {
		return false
	}
	selection := typeInfo.info.Selections[sel]
	if selection == nil || selection.Kind() != types.FieldVal {
		return false
	}
	field, ok := selection.Obj().(*types.Var)
	return ok && field.Pkg() == named.Obj().Pkg()
}

func promotedFieldPath(recv types.Type, indexes []int) ([]string, bool) {
	var fields []string
	current := recv
	for _, index := range indexes {
		if ptr, ok := types.Unalias(current).(*types.Pointer); ok {
			current = ptr.Elem()
		}
		structType, ok := types.Unalias(current).Underlying().(*types.Struct)
		if !ok || index < 0 || index >= structType.NumFields() {
			return nil, false
		}
		field := structType.Field(index)
		fields = append(fields, ToSnakeCase(field.Name()))
		current = field.Type()
	}
	return fields, len(fields) > 0
}

func mutexReceiverKey(receiver mutexReceiverInfo) (string, bool) {
	var buf bytes.Buffer
	if format.Node(&buf, token.NewFileSet(), receiver.expr) != nil {
		return "", false
	}
	for _, field := range receiver.fields {
		buf.WriteByte('.')
		buf.WriteString(field)
	}
	return buf.String(), true
}

func mutexAliasIdentKey(expr ast.Expr) (string, types.Type, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || ident.Name == "_" {
		return "", nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", nil, false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return "", nil, false
	}
	if _, ok := types.Unalias(typ).(*types.Pointer); !ok {
		return "", nil, false
	}
	return ident.Name, typ, true
}

func noteMutexGuardAliasAssignment(lhs ast.Expr, rhs ast.Expr) {
	lhsKey, lhsType, ok := mutexAliasIdentKey(lhs)
	if !ok {
		return
	}
	rhsKey, rhsType, ok := mutexAliasIdentKey(rhs)
	if !ok || !types.AssignableTo(rhsType, lhsType) {
		return
	}
	type aliasUpdate struct {
		from  string
		to    string
		guard string
	}
	var updates []aliasUpdate
	for key, guardName := range activeMutexGuards {
		if key == rhsKey {
			updates = append(updates, aliasUpdate{from: key, to: lhsKey, guard: guardName})
			continue
		}
		prefix := rhsKey + "."
		if strings.HasPrefix(key, prefix) {
			updates = append(updates, aliasUpdate{from: key, to: lhsKey + strings.TrimPrefix(key, rhsKey), guard: guardName})
		}
	}
	for _, update := range updates {
		delete(activeMutexGuards, update.from)
		activeMutexGuards[update.to] = update.guard
	}
}

func writeMutexReceiver(out *strings.Builder, receiver mutexReceiverInfo) {
	if len(receiver.fields) == 0 {
		TranspileExpressionContext(out, receiver.expr, LValue)
		return
	}
	if ident, ok := unwrapParens(receiver.expr).(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
		out.WriteString(currentReceiverRustName())
	} else {
		out.WriteString("(*")
		TranspileExpressionContext(out, receiver.expr, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
	}
	for _, field := range receiver.fields {
		out.WriteString(".")
		out.WriteString(field)
	}
}

func mutexReceiverIsWrappedIdent(receiver mutexReceiverInfo) bool {
	if len(receiver.fields) != 0 {
		return false
	}
	// A local "var mu sync.Mutex" is a bare GoMutex, not a cell.
	ident, ok := receiver.expr.(*ast.Ident)
	return ok && !isVarBare(ident.Name)
}

func writeMutexReceiverRef(out *strings.Builder, receiver mutexReceiverInfo) {
	if !mutexReceiverIsWrappedIdent(receiver) {
		writeMutexReceiver(out, receiver)
		return
	}
	out.WriteString("(*")
	writeMutexReceiver(out, receiver)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
}

func writeSourceMappedMutexReceiverRef(out *strings.Builder, receiver mutexReceiverInfo) {
	if sel, ok := unwrapParens(receiver.expr).(*ast.SelectorExpr); ok && isLocalSourceMappedSyncMutexFieldSelector(sel) {
		out.WriteString("(*")
		writeMutexReceiver(out, receiver)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
		return
	}
	writeMutexReceiverRef(out, receiver)
}

func writeMutexReceiverCloneValue(out *strings.Builder, receiver mutexReceiverInfo) {
	writeMutexReceiverRef(out, receiver)
	out.WriteString(".clone()")
}

func cloneMutexGuards(guards map[string]string) map[string]string {
	cloned := make(map[string]string, len(guards))
	for key, guardName := range guards {
		cloned[key] = guardName
	}
	return cloned
}

// mergeMutexGuardsAfterIfStmt is mergeMutexGuardsAfterIf for the if
// statement stmt. A branch that ends the function, or leaves the loop, has
// no say in the guards held after the statement: in
// "mu.Lock(); if done { mu.Unlock(); return }; ...; mu.Unlock()" the guard
// is still held after the if, and the last Unlock drops it.
func mergeMutexGuardsAfterIfStmt(stmt *ast.IfStmt, before, thenGuards, elseGuards map[string]string) map[string]string {
	thenEnds := stmt != nil && stmt.Body != nil && stmtListTerminates(stmt.Body.List)
	elseEnds := stmt != nil && stmt.Else != nil && stmtTerminates(stmt.Else)
	switch {
	case thenEnds && elseEnds:
		return cloneMutexGuards(before)
	case thenEnds:
		return mergeMutexGuardsAfterIf(before, elseGuards, elseGuards)
	case elseEnds:
		return mergeMutexGuardsAfterIf(before, thenGuards, thenGuards)
	}
	return mergeMutexGuardsAfterIf(before, thenGuards, elseGuards)
}

func mergeMutexGuardsAfterIf(before, thenGuards, elseGuards map[string]string) map[string]string {
	merged := make(map[string]string)
	for key, guardName := range before {
		if thenGuard, ok := thenGuards[key]; !ok || thenGuard != guardName {
			continue
		}
		if elseGuard, ok := elseGuards[key]; !ok || elseGuard != guardName {
			continue
		}
		merged[key] = guardName
	}
	return merged
}

// goStmtCapturedNameIsEmptyInterface reports whether name, a variable a go
// statement captures, has an empty interface type, as goFunc's arg does in
// "go arg.(func())()".
func goStmtCapturedNameIsEmptyInterface(call *ast.CallExpr, name string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || call == nil {
		return false
	}
	found := false
	ast.Inspect(call, func(n ast.Node) bool {
		if found {
			return false
		}
		ident, ok := n.(*ast.Ident)
		if !ok || ident.Name != name {
			return true
		}
		obj, ok := typeInfo.info.Uses[ident].(*types.Var)
		if !ok {
			return true
		}
		if iface, ok := types.Unalias(obj.Type()).Underlying().(*types.Interface); ok && iface.NumMethods() == 0 {
			found = true
		}
		return false
	})
	return found
}

// stringRangeSourceIsBareVariable reports whether the source of a string
// range is a variable that holds the String itself, not a cell: a range
// variable over a []string ("for _, line := range lines { for _, ch :=
// range line }") or a bare local.
func stringRangeSourceIsBareVariable(source ast.Expr) bool {
	ident, ok := unwrapParens(source).(*ast.Ident)
	if !ok {
		return false
	}
	if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && !identShadowsRangeVar(ident) {
		return !isWrappedRangeVarType(varType)
	}
	return isVarBare(ident.Name)
}

func writeMutexLockStatement(out *strings.Builder, expr ast.Expr) bool {
	receiver, ok := mutexLockReceiver(expr)
	if !ok {
		return false
	}
	if receiver.sourceMapped {
		writeSourceMappedMutexReceiverRef(out, receiver)
		out.WriteString(".lock();")
		return true
	}
	if currentLoopDepth > 0 {
		writeMutexReceiverRef(out, receiver)
		out.WriteString(".lock();")
		return true
	}
	id := stableStatementPosID(expr.Pos())
	sourceName := fmt.Sprintf("__mutex_guard_source_%d", id)
	guardName := fmt.Sprintf("__mutex_guard_%d", id)

	out.WriteString("let ")
	out.WriteString(sourceName)
	out.WriteString(" = ")
	writeMutexReceiverCloneValue(out, receiver)
	out.WriteString("; ")
	out.WriteString("let ")
	out.WriteString(guardName)
	out.WriteString(" = ")
	out.WriteString(sourceName)
	out.WriteString(".guard();")
	if key, ok := mutexReceiverKey(receiver); ok {
		activeMutexGuards[key] = guardName
	}
	return true
}

func stableStatementPosID(pos token.Pos) int {
	if pos == token.NoPos {
		return 0
	}
	ctx := GetTranspileContext()
	if ctx != nil && ctx.File != nil && ctx.File.FileSet != nil {
		position := ctx.File.FileSet.Position(pos)
		if position.IsValid() {
			return position.Offset
		}
	}
	// The statement is not in the file set at hand. Its raw position counts
	// every file loaded before it, the Go toolchain's own sources among them,
	// so it changes with the Go version. Count from the function body
	// instead, which comes from the same file set.
	if currentFunctionBody != nil && currentFunctionBody.Pos().IsValid() && pos >= currentFunctionBody.Pos() {
		return int(pos - currentFunctionBody.Pos())
	}
	return int(pos)
}

func writeMutexUnlockStatement(out *strings.Builder, expr ast.Expr) bool {
	receiver, ok := mutexUnlockReceiver(expr)
	if !ok {
		return false
	}
	if receiver.sourceMapped {
		writeSourceMappedMutexReceiverRef(out, receiver)
		out.WriteString(".unlock();")
		return true
	}
	key, ok := mutexReceiverKey(receiver)
	if !ok {
		return false
	}
	guardName, ok := activeMutexGuards[key]
	if !ok {
		writeMutexReceiverRef(out, receiver)
		out.WriteString(".unlock();")
		return true
	}
	out.WriteString("std::mem::drop(")
	out.WriteString(guardName)
	out.WriteString(");")
	delete(activeMutexGuards, key)
	return true
}

// isMutexUnlockDefer checks if a defer statement is mu.Unlock() on a sync.Mutex
// mutexUnlockDeferHasGuard reports whether the mutex that call unlocks was
// locked with a guard that is still held.
func mutexUnlockDeferHasGuard(call *ast.CallExpr) bool {
	receiver, ok := syncMutexMethodReceiver(call, "Unlock")
	if !ok {
		return false
	}
	key, ok := mutexReceiverKey(receiver)
	if !ok {
		return false
	}
	_, guarded := activeMutexGuards[key]
	return guarded
}

func isMutexUnlockDefer(call *ast.CallExpr) bool {
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "Unlock" {
		return false
	}
	receiver, ok := syncMutexMethodReceiver(call, "Unlock")
	return ok && !receiver.sourceMapped
}

// channelElementIsGoError reports whether a channel expression carries Go error values.
func channelElementIsGoError(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	ch, ok := types.Unalias(typ).Underlying().(*types.Chan)
	return ok && isGoErrorType(ch.Elem())
}

func writeErrorOptionFromHandleExpression(out *strings.Builder, expr ast.Expr) {
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("None::<")
		out.WriteString(rustErrorPayloadType())
		out.WriteString(">")
		return
	}
	out.WriteString("{ let __err_handle = ")
	if ident, ok := expr.(*ast.Ident); ok && ident.Name != "nil" {
		out.WriteString(rustIdentForUseWithCapture(ident))
		out.WriteString(".clone()")
	} else if call, ok := expr.(*ast.CallExpr); ok {
		TranspileExpression(out, call)
	} else {
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone()")
	}
	out.WriteString("; let __err_value = (*__err_handle")
	WriteBorrowMethod(out, false)
	out.WriteString(").clone(); __err_value }")
}

func writeErrorHandleFromOptionValue(out *strings.Builder, value string) {
	trackWrapperImports()
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(Mutex::new(")
		out.WriteString(value)
		out.WriteString("))")
		return
	}
	out.WriteString("Rc::new(RefCell::new(")
	out.WriteString(value)
	out.WriteString("))")
}

func writeErrorHandleFromChannelReceive(out *strings.Builder, channel ast.Expr) {
	trackWrapperImports()
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(Mutex::new(")
	} else {
		out.WriteString("Rc::new(RefCell::new(")
	}
	writeChannelExpression(out, channel)
	out.WriteString(".recv().unwrap_or_default()))")
}

func writeErrorChannelReceiveReturnValue(out *strings.Builder, result ast.Expr, expected ast.Expr) bool {
	unary, ok := result.(*ast.UnaryExpr)
	if !ok || unary.Op != token.ARROW {
		return false
	}
	if !channelReceiveFillsHandleOf(unary.X, expectedTypeFromParamExpr(expected)) {
		return false
	}
	writeErrorHandleFromChannelReceive(out, unary.X)
	return true
}

func writeEmptyErrorHandle(out *strings.Builder) {
	trackWrapperImports()
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(Mutex::new(None::<")
		out.WriteString(rustErrorPayloadType())
		out.WriteString(">))")
		return
	}
	out.WriteString("Rc::new(RefCell::new(None::<")
	out.WriteString(rustErrorPayloadType())
	out.WriteString(">))")
}

func rustEmptyErrorHandleValue() string {
	var out strings.Builder
	writeEmptyErrorHandle(&out)
	return out.String()
}

func isRightNilComparison(expr *ast.BinaryExpr) bool {
	if expr == nil || expr.Op != token.EQL && expr.Op != token.NEQ {
		return false
	}
	ident, ok := expr.Y.(*ast.Ident)
	return ok && ident.Name == "nil"
}

func expressionReturnsWrappedValueForExtraction(typeInfo *TypeInfo, expr ast.Expr) bool {
	if typeInfo == nil || !typeInfo.ReturnsWrappedValue(expr) {
		return false
	}
	if call, ok := expr.(*ast.CallExpr); ok && typeInfo.IsTypeConversion(call) && !typeConversionEmitsWrappedValue(call) {
		return false
	}
	return true
}

func writeErrorChannelReceiveAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	unary, ok := rhs.(*ast.UnaryExpr)
	if !ok || unary.Op != token.ARROW || !channelElementIsHandleOption(unary.X) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !channelReceiveFillsHandleOf(unary.X, assignmentTargetType(typeInfo, lhs)) {
		return false
	}
	out.WriteString("{ let new_val = ")
	writeChannelExpression(out, unary.X)
	out.WriteString(".recv().unwrap_or_default(); *")
	TranspileExpressionContext(out, lhs, LValue)
	WriteBorrowMethod(out, true)
	out.WriteString(" = new_val; }")
	return true
}

// writeGoroutinePanicExit ends the program for a panic that left a goroutine,
// the way Go does: "panic: " and the value on stderr, then exit status 2.
func writeGoroutinePanicExit(out *strings.Builder, indent string) {
	out.WriteString(indent)
	out.WriteString("let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from(\"goroutine panicked\") };\n")
	out.WriteString(indent)
	out.WriteString("eprintln!(\"panic: {}\", __go_panic_text);\n")
	out.WriteString(indent)
	out.WriteString("std::process::exit(2);\n")
}

func isPredeclaredBoolConstant(ident *ast.Ident) bool {
	if ident == nil || (ident.Name != "true" && ident.Name != "false") {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		// Without type information only a declared local can shadow the
		// predeclared constant.
		return lookupVarInfo(ident.Name) == nil
	}
	return typeInfo.GetObject(ident) == types.Universe.Lookup(ident.Name)
}

// transpileChannelValue writes a value suitable for sending on a channel.
// Channel values are bare (unwrapped), so we need to unwrap wrapped variables.
// channelOfBareCollections reports whether ch's elements are unnamed slices
// or maps, which it holds bare.
func channelOfBareCollections(ch ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	chanType, ok := types.Unalias(typeInfo.GetType(ch)).Underlying().(*types.Chan)
	if !ok {
		return false
	}
	switch t := types.Unalias(chanType.Elem()).(type) {
	case *types.Slice:
		return !isEmptyInterfaceType(t.Elem())
	case *types.Map:
		return true
	}
	return false
}

func transpileChannelValue(out *strings.Builder, expr ast.Expr) {
	// A constant sent on a chan Level is a Level, the newtype over a cell,
	// not the bare number (DEV-220).
	if writeNamedScalarConstChannelValue(out, expr) {
		return
	}
	// For string literals, just output the value directly
	if lit, ok := expr.(*ast.BasicLit); ok {
		if lit.Kind == token.STRING {
			out.WriteString(RustStringLiteral(lit.Value))
			out.WriteString(".to_string()")
		} else if lit.Kind == token.INT {
			out.WriteString(RustIntegerLiteral(lit.Value))
		} else {
			out.WriteString(lit.Value)
		}
		return
	}

	// Only the predeclared boolean constants are Rust boolean literals. Go
	// permits local variables named true and false; those use ordinary Rust
	// bindings (renamed to true_/false_ because Rust reserves the literals).
	if ident, ok := expr.(*ast.Ident); ok {
		if isPredeclaredBoolConstant(ident) {
			out.WriteString(ident.Name)
			return
		}
	}

	// Check if this is a Copy type using TypeInfo
	isCopyType := false
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		exprType := typeInfo.GetType(expr)
		if exprType != nil {
			if isGoErrorType(exprType) {
				writeErrorOptionFromHandleExpression(out, expr)
				return
			}
			if basic, ok := exprType.Underlying().(*types.Basic); ok {
				switch basic.Kind() {
				case types.Int, types.Int8, types.Int16, types.Int32, types.Int64,
					types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64,
					types.Float32, types.Float64, types.Bool,
					types.UntypedInt, types.UntypedFloat, types.UntypedBool:
					isCopyType = true
				}
			}
		}
	}

	if call, ok := expr.(*ast.CallExpr); ok {
		if callReturnsWrappedBoolBySyntax(call) || (typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !callReturnsBareChannelValue(call)) {
			out.WriteString("(*")
			TranspileExpression(out, call)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
			if !isCopyType {
				out.WriteString(".clone()")
			}
			return
		}
	}

	// For identifiers that are wrapped, unwrap them
	if ident, ok := expr.(*ast.Ident); ok {
		// A local constant can shadow a captured outer variable with the same
		// spelling. Its go/types object, rather than the name-only capture map,
		// determines that this use names the local constant.
		if isLocalConstantIdent(ident) {
			out.WriteString(RustIdentForUse(ident))
			return
		}
		rustName := rustIdentForUseWithCapture(ident)
		// Check if it's a range loop variable, constant, or bare variable
		if rangeKind, isRange := rangeLoopVars[ident.Name]; isRange {
			// A range value over a slice of maps is the map's handle (#318),
			// and a channel carries the map bare. A value received from a
			// channel or in a select is already bare.
			if typeInfo != nil && isUnnamedMapType(typeInfo.GetType(ident)) && rangeKind != "channel_val" && rangeKind != "select_val" {
				out.WriteString("(*")
				out.WriteString(rustName)
				WriteBorrowMethod(out, false)
				out.WriteString(").clone().unwrap_or_default()")
				return
			}
			// A range over a slice or array can bind a reference to the
			// iterator element. Sending the range variable still copies its
			// Go value into the channel rather than sending that reference.
			if writeOwnedRangeValue(out, ident) {
				return
			}
			out.WriteString(rustName)
			return
		}
		if isVarBare(ident.Name) {
			out.WriteString(rustName)
			return
		}
		// Wrapped variable — unwrap and copy if needed. A value sent on a
		// channel is a copy of the variable.
		if isCopyType {
			out.WriteString("(*")
			out.WriteString(rustName)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		} else if copyCall, known := goValueCopyCallForExpr(ident); known {
			out.WriteString(rustName)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()")
			out.WriteString(copyCall)
		} else {
			out.WriteString(goValueCopyNeedsTypeInfo)
		}
		return
	}

	if _, ok := expr.(*ast.SelectorExpr); ok {
		// The field's cell, which a read of a field through a pointer
		// variable would already have unwrapped. The value is read in a let
		// of its own, so the lock on the struct the field is read out of is
		// released before a send that waits.
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if _, isPointer := types.Unalias(typeInfo.GetType(expr)).(*types.Pointer); isPointer {
				// A pointer field is the pointer's handle, which is sent.
				out.WriteString("{ let __sent = ")
				TranspileExpressionContext(out, expr, LValue)
				out.WriteString(".clone(); __sent }")
				return
			}
		}
		if isCopyType {
			out.WriteString("{ let __sent = (*")
			TranspileExpressionContext(out, expr, LValue)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()); __sent }")
		} else if copyCall, known := goValueCopyCallForExpr(expr); known {
			out.WriteString("{ let __sent = ")
			TranspileExpressionContext(out, expr, LValue)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()")
			out.WriteString(copyCall)
			out.WriteString("; __sent }")
		} else {
			out.WriteString(goValueCopyNeedsTypeInfo)
		}
		return
	}

	// An element of a slice or an array, and the target of a pointer, is
	// copied when it is sent.
	if writeGoValueCopyOfElement(out, expr) {
		return
	}

	// A channel holds its slices and maps bare: ch <- []int{1, 2} and
	// ch <- s[1:] send the collection out of its new handle.
	if channelElemIsBareCollection(expr) && !isExpressionResultBare(expr) {
		out.WriteString("{ let __sent = ")
		TranspileExpression(out, expr)
		out.WriteString("; let __sent_guard = __sent")
		WriteBorrowMethod(out, false)
		out.WriteString("; __sent_guard.as_ref().cloned().unwrap_or_default() }")
		return
	}

	// For other expressions, try to unwrap
	TranspileExpression(out, expr)
}

func callReturnsBareChannelValue(call *ast.CallExpr) bool {
	if ident, ok := call.Fun.(*ast.Ident); ok {
		switch ident.Name {
		case "len", "cap":
			return true
		case "make":
			return isMakeChannelCall(call)
		}
	}

	if key, ok := stdlibCallKey(call.Fun); ok {
		switch key {
		case "time.After", "time.Tick", "context.WithTimeout", "context.WithCancel", "context.WithCancelCause":
			return true
		}
	}

	return false
}

// callOfTranslatedFuncReturnsChannel reports whether call is of a function
// written in Go that the program translates, or of a func value, and has one
// result, a channel. A channel is the bare GoChannel, and such a function
// returns it as it is: an element of fanIn(gen(5), gen(5)) took a lock on
// the channel each call returned.
func callOfTranslatedFuncReturnsChannel(call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	funType := typeInfo.GetType(call.Fun)
	if funType == nil {
		return false
	}
	// type Factory func() <-chan int is a signature underneath.
	sig, ok := funType.Underlying().(*types.Signature)
	if !ok || sig.Results().Len() != 1 {
		return false
	}
	if _, ok := sig.Results().At(0).Type().Underlying().(*types.Chan); !ok {
		return false
	}
	var name *ast.Ident
	fun := unwrapParens(call.Fun)
	// g[int](1) names g with its type arguments.
	switch index := fun.(type) {
	case *ast.IndexExpr:
		fun = unwrapParens(index.X)
	case *ast.IndexListExpr:
		fun = unwrapParens(index.X)
	}
	switch fun := fun.(type) {
	case *ast.Ident:
		name = fun
	case *ast.SelectorExpr:
		name = fun.Sel
	case *ast.FuncLit:
		return true
	default:
		return false
	}
	switch obj := typeInfo.GetObject(name).(type) {
	case *types.Var:
		return true
	case *types.Func:
		return obj.Pkg() != nil && !isStubBackedStdlibPackagePath(obj.Pkg().Path())
	}
	return false
}

func isMakeChannelCall(call *ast.CallExpr) bool {
	if call == nil || len(call.Args) == 0 {
		return false
	}
	ident, ok := call.Fun.(*ast.Ident)
	if !ok || ident.Name != "make" {
		return false
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
		if obj, ok := typeInfo.info.Uses[ident]; ok {
			builtin, ok := obj.(*types.Builtin)
			if !ok || builtin.Name() != "make" {
				return false
			}
		}
	}
	if _, ok := call.Args[0].(*ast.ChanType); ok {
		return true
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if typ := typeInfo.GetType(call); typ != nil {
			if _, ok := types.Unalias(typ).Underlying().(*types.Chan); ok {
				return true
			}
		}
	}
	return false
}

// hasBlankLineBetween checks if there's more than one line between two positions
func hasBlankLineBetween(fileSet *token.FileSet, pos1, pos2 token.Pos) bool {
	if fileSet == nil || pos1 == token.NoPos || pos2 == token.NoPos {
		return false
	}

	p1 := fileSet.Position(pos1)
	p2 := fileSet.Position(pos2)

	// If there's more than 1 line between the positions, there's at least one blank line
	return p2.Line-p1.Line > 1
}

// outputCommentsBeforePos outputs any comments that appear before the given position
func outputCommentsBeforePos(out *strings.Builder, comments []*ast.CommentGroup, fileSet *token.FileSet, pos token.Pos, indent string, lastPos *token.Pos) {
	if fileSet == nil || pos == token.NoPos {
		return
	}

	targetLine := fileSet.Position(pos).Line
	startLine := 0
	if *lastPos != token.NoPos {
		startLine = fileSet.Position(*lastPos).Line
	}

	for _, cg := range comments {
		cgLine := fileSet.Position(cg.Pos()).Line
		// Output comments that are after our last position but before the current position
		if cgLine > startLine && cgLine < targetLine {
			for _, comment := range cg.List {
				out.WriteString(indent)
				out.WriteString(rustStatementCommentText(comment.Text))
				out.WriteString("\n")
				out.WriteString(indent)
			}
		}
	}

	*lastPos = pos
}

func rustStatementCommentText(text string) string {
	if strings.HasPrefix(text, "///") {
		return "//" + strings.TrimLeft(text[2:], "/")
	}
	if strings.HasPrefix(text, "//!") {
		return "//" + text[3:]
	}
	return text
}

func outputComment(out *strings.Builder, cg *ast.CommentGroup, indent string, isDoc bool) {
	if cg == nil {
		return
	}

	for _, comment := range cg.List {
		out.WriteString(indent)

		text := comment.Text
		if isDoc && strings.HasPrefix(text, "//") {
			// Convert doc comment to Rust format
			out.WriteString("///")
			out.WriteString(text[2:]) // Skip the "//"
		} else {
			// Keep as-is for regular comments
			out.WriteString(text)
		}
		out.WriteString("\n")
	}
}

func funcLiteralParamType(funcLit *ast.FuncLit, index int) types.Type {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || funcLit == nil {
		return nil
	}
	sig, ok := typeInfo.GetType(funcLit).(*types.Signature)
	if !ok || sig.Params() == nil || index >= sig.Params().Len() {
		return nil
	}
	return sig.Params().At(index).Type()
}

func writeDeferFuncLiteralArgumentCapture(out *strings.Builder, arg ast.Expr, expectedType types.Type) {
	if expectedType != nil && mapValueTypeKeepsHandle(expectedType) {
		if ident, ok := arg.(*ast.Ident); ok && ident.Name == "nil" {
			WriteWrappedNone(out)
			return
		}
		if writeStdlibInterfaceCallArgumentConversion(out, arg, expectedType) {
			return
		}
		if writeLocalInterfaceWrappedValue(out, arg, expectedType) {
			return
		}
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if argType := typeInfo.GetType(arg); argType != nil && mapValueTypeKeepsHandle(argType) && types.AssignableTo(argType, expectedType) {
				TranspileExpressionContext(out, arg, LValue)
				out.WriteString(".clone()")
				return
			}
		}
	}

	if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "_" {
		if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
			if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
				WriteWrapperPrefix(out)
				out.WriteString("(*")
				out.WriteString(ident.Name)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
				WriteWrapperSuffix(out)
				return
			}
			WriteWrapperPrefix(out)
			TranspileExpression(out, arg)
			WriteWrapperSuffix(out)
			return
		}
		WriteWrapperPrefix(out)
		TranspileExpression(out, arg)
		WriteWrapperSuffix(out)
		return
	}

	WriteWrapperPrefix(out)
	if !writeOwnedExpressionValue(out, arg) {
		TranspileExpression(out, arg)
	}
	WriteWrapperSuffix(out)
}

// TranspileStatementSimple is a wrapper for backward compatibility
func TranspileStatementSimple(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet) {
	var lastPos token.Pos
	TranspileStatement(out, stmt, fnType, fileSet, nil, &lastPos, "")
}

func transpileElseBranch(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet) {
	if elseIf, ok := stmt.(*ast.IfStmt); ok {
		if elseIf.Init != nil {
			transpileIfWithInitAsBlock(out, elseIf, fnType, fileSet)
		} else {
			TranspileStatementSimple(out, elseIf, fnType, fileSet)
		}
		return
	}
	if block, ok := stmt.(*ast.BlockStmt); ok {
		out.WriteString("{\n")
		popElseBodyScope := func() {}
		if vt := GetVarTable(); vt != nil {
			vt.PushScope()
			popElseBodyScope = vt.PopScope
		}
		var captureShadowRestores []func()
		for _, stmt := range block.List {
			out.WriteString("            ")
			TranspileStatementSimple(out, stmt, fnType, fileSet)
			out.WriteString(";\n")
			trackCaptureShadows(&captureShadowRestores, stmt)
		}
		restoreCaptureShadows(captureShadowRestores)
		popElseBodyScope()
		out.WriteString("        }")
	}
}

func transpileIfWithInitAsBlock(out *strings.Builder, stmt *ast.IfStmt, fnType *ast.FuncType, fileSet *token.FileSet) {
	if vt := GetVarTable(); vt != nil {
		vt.PushScope()
		defer vt.PopScope()
	}
	restoreRangeLoopVars := func() {}
	defer func() {
		restoreRangeLoopVars()
	}()
	out.WriteString("{\n        ")
	TranspileStatementSimple(out, stmt.Init, fnType, fileSet)
	restoreInitCaptureShadow := suppressCaptureRenamesDeclaredBy(stmt.Init)
	defer restoreInitCaptureShadow()
	restoreRangeLoopVars = shadowRangeLoopVars(shortDeclNames(stmt.Init))
	out.WriteString(";\n        if ")
	transpileCondition(out, stmt.Cond)
	out.WriteString(" {\n")
	beforeGuards := cloneMutexGuards(activeMutexGuards)
	activeMutexGuards = cloneMutexGuards(beforeGuards)
	popIfBodyScope := func() {}
	if vt := GetVarTable(); vt != nil {
		vt.PushScope()
		popIfBodyScope = vt.PopScope
	}
	var captureShadowRestores []func()
	for _, bodyStmt := range stmt.Body.List {
		out.WriteString("            ")
		TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
		out.WriteString(";\n")
		trackCaptureShadows(&captureShadowRestores, bodyStmt)
	}
	restoreCaptureShadows(captureShadowRestores)
	popIfBodyScope()
	thenGuards := cloneMutexGuards(activeMutexGuards)
	out.WriteString("        }")
	elseGuards := cloneMutexGuards(beforeGuards)
	if stmt.Else != nil {
		activeMutexGuards = cloneMutexGuards(beforeGuards)
		out.WriteString(" else ")
		transpileElseBranch(out, stmt.Else, fnType, fileSet)
		elseGuards = cloneMutexGuards(activeMutexGuards)
	}
	activeMutexGuards = mergeMutexGuardsAfterIfStmt(stmt, beforeGuards, thenGuards, elseGuards)
	out.WriteString("\n    }")
}

const minStatementBuiltLogicalConditionNodes = 3
const minStatementBuiltNegatedLogicalConditionNodes = 2
const minStatementBuiltLogicalConditionOperandComplexity = 10
const minStatementBuiltLogicalConditionFieldComparisons = 4
const minStatementBuiltLogicalConditionFieldBitmaskComparisons = 2
const minStatementBuiltLogicalConditionCallOperands = 4
const concurrentFieldSelectorLogicalConditionComplexity = 4
const concurrentFieldSelectorConversionLogicalConditionComplexity = 4
const concurrentSequenceIndexLogicalConditionComplexity = 8
const concurrentNamedIntegerLogicalComparisonOperandComplexity = 4
const concurrentGoErrorLogicalComparisonComplexity = 10

func transpileCondition(out *strings.Builder, expr ast.Expr) {
	if shouldStatementBuildLogicalCondition(expr) {
		nextTemp := 0
		writeStatementBuiltLogicalConditionValue(out, expr, currentLineIndent(out), &nextTemp)
		return
	}
	transpileConditionInline(out, expr)
}

func shouldStatementBuildLogicalCondition(expr ast.Expr) bool {
	if !NeedsConcurrentWrapper() {
		return false
	}
	if unary, ok := unwrapParens(expr).(*ast.UnaryExpr); ok && unary.Op == token.NOT {
		return shouldStatementBuildNegatedLogicalCondition(unary.X)
	}
	if countLogicalConditionNodes(expr) >= minStatementBuiltLogicalConditionNodes {
		return true
	}
	if countLogicalConditionFieldComparisons(expr) >= minStatementBuiltLogicalConditionFieldComparisons {
		return true
	}
	if countLogicalConditionFieldComparisons(expr) >= minStatementBuiltLogicalConditionNodes &&
		countLogicalConditionFieldBitmaskComparisons(expr) >= minStatementBuiltLogicalConditionFieldBitmaskComparisons {
		return true
	}
	if countLogicalConditionCallOperands(expr) >= minStatementBuiltLogicalConditionCallOperands {
		return true
	}
	return logicalConditionHasComplexOperand(expr)
}

func shouldStatementBuildNegatedLogicalCondition(expr ast.Expr) bool {
	if countLogicalConditionNodes(expr) >= minStatementBuiltNegatedLogicalConditionNodes {
		return true
	}
	return shouldStatementBuildLogicalCondition(expr)
}

func countLogicalConditionNodes(expr ast.Expr) int {
	binary, ok := unwrapParens(expr).(*ast.BinaryExpr)
	if !ok || (binary.Op != token.LAND && binary.Op != token.LOR) {
		return 0
	}
	return 1 +
		countLogicalConditionNodes(binary.X) +
		countLogicalConditionNodes(binary.Y)
}

func logicalConditionHasComplexOperand(expr ast.Expr) bool {
	binary, ok := unwrapParens(expr).(*ast.BinaryExpr)
	if !ok || (binary.Op != token.LAND && binary.Op != token.LOR) {
		return false
	}
	if countLogicalConditionOperandComplexity(binary.X) >= minStatementBuiltLogicalConditionOperandComplexity {
		return true
	}
	if countLogicalConditionOperandComplexity(binary.Y) >= minStatementBuiltLogicalConditionOperandComplexity {
		return true
	}
	return logicalConditionHasComplexOperand(binary.X) || logicalConditionHasComplexOperand(binary.Y)
}

func countLogicalConditionFieldComparisons(expr ast.Expr) int {
	binary, ok := unwrapParens(expr).(*ast.BinaryExpr)
	if !ok {
		return 0
	}
	if binary.Op == token.LAND || binary.Op == token.LOR {
		return countLogicalConditionFieldComparisons(binary.X) + countLogicalConditionFieldComparisons(binary.Y)
	}
	if logicalConditionBinaryIsComparison(binary) && exprContainsTypedFieldSelector(binary) {
		return 1
	}
	return 0
}

func countLogicalConditionFieldBitmaskComparisons(expr ast.Expr) int {
	binary, ok := unwrapParens(expr).(*ast.BinaryExpr)
	if !ok {
		return 0
	}
	if binary.Op == token.LAND || binary.Op == token.LOR {
		return countLogicalConditionFieldBitmaskComparisons(binary.X) + countLogicalConditionFieldBitmaskComparisons(binary.Y)
	}
	if logicalConditionBinaryIsComparison(binary) && exprContainsTypedFieldSelector(binary) && exprContainsBitwiseAnd(binary) {
		return 1
	}
	return 0
}

func exprContainsBitwiseAnd(expr ast.Expr) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.BinaryExpr:
		return e.Op == token.AND || exprContainsBitwiseAnd(e.X) || exprContainsBitwiseAnd(e.Y)
	case *ast.CallExpr:
		if exprContainsBitwiseAnd(e.Fun) {
			return true
		}
		for _, arg := range e.Args {
			if exprContainsBitwiseAnd(arg) {
				return true
			}
		}
	case *ast.IndexExpr:
		return exprContainsBitwiseAnd(e.X) || exprContainsBitwiseAnd(e.Index)
	case *ast.SliceExpr:
		if exprContainsBitwiseAnd(e.X) {
			return true
		}
		for _, bound := range []ast.Expr{e.Low, e.High, e.Max} {
			if bound != nil && exprContainsBitwiseAnd(bound) {
				return true
			}
		}
	case *ast.SelectorExpr:
		return exprContainsBitwiseAnd(e.X)
	case *ast.UnaryExpr:
		return exprContainsBitwiseAnd(e.X)
	case *ast.StarExpr:
		return exprContainsBitwiseAnd(e.X)
	case *ast.TypeAssertExpr:
		return exprContainsBitwiseAnd(e.X)
	case *ast.CompositeLit:
		for _, elt := range e.Elts {
			if exprContainsBitwiseAnd(elt) {
				return true
			}
		}
	case *ast.KeyValueExpr:
		return exprContainsBitwiseAnd(e.Key) || exprContainsBitwiseAnd(e.Value)
	}
	return false
}

func countLogicalConditionCallOperands(expr ast.Expr) int {
	switch e := unwrapParens(expr).(type) {
	case *ast.BinaryExpr:
		if e.Op == token.LAND || e.Op == token.LOR {
			return countLogicalConditionCallOperands(e.X) + countLogicalConditionCallOperands(e.Y)
		}
	case *ast.CallExpr:
		return 1
	}
	return 0
}

func countLogicalConditionOperandComplexity(expr ast.Expr) int {
	switch e := unwrapParens(expr).(type) {
	case *ast.BinaryExpr:
		if e.Op == token.LAND || e.Op == token.LOR {
			return 0
		}
		count := 1 + countLogicalConditionOperandComplexity(e.X) + countLogicalConditionOperandComplexity(e.Y)
		if logicalConditionBinaryIsComparison(e) {
			count += countNamedIntegerLogicalComparisonOperandComplexity(e.X)
			count += countNamedIntegerLogicalComparisonOperandComplexity(e.Y)
			count += countGoErrorLogicalComparisonComplexity(e)
		}
		return count
	case *ast.CallExpr:
		count := 2 + countLogicalConditionOperandComplexity(e.Fun)
		for _, arg := range e.Args {
			count += countLogicalConditionOperandComplexity(arg)
		}
		if logicalConditionCallConvertsFieldSelector(e) {
			count += concurrentFieldSelectorConversionLogicalConditionComplexity
		}
		return count
	case *ast.IndexExpr:
		count := 2 + countLogicalConditionOperandComplexity(e.X) + countLogicalConditionOperandComplexity(e.Index)
		if indexReadsConcurrentSequence(e) || indexReadsConcurrentString(e) {
			count += concurrentSequenceIndexLogicalConditionComplexity
		}
		return count
	case *ast.SelectorExpr:
		if concurrentLogicalConditionSelectorIsField(e) {
			return concurrentFieldSelectorLogicalConditionComplexity + countLogicalConditionOperandComplexity(e.X)
		}
		return 1 + countLogicalConditionOperandComplexity(e.X)
	case *ast.SliceExpr:
		count := 2 + countLogicalConditionOperandComplexity(e.X)
		if e.Low != nil {
			count += countLogicalConditionOperandComplexity(e.Low)
		}
		if e.High != nil {
			count += countLogicalConditionOperandComplexity(e.High)
		}
		if e.Max != nil {
			count += countLogicalConditionOperandComplexity(e.Max)
		}
		return count
	case *ast.StarExpr:
		return 1 + countLogicalConditionOperandComplexity(e.X)
	case *ast.UnaryExpr:
		return 1 + countLogicalConditionOperandComplexity(e.X)
	case *ast.ParenExpr:
		return countLogicalConditionOperandComplexity(e.X)
	case *ast.Ident, *ast.BasicLit:
		return 0
	default:
		if expr == nil {
			return 0
		}
		return 1
	}
}

func indexReadsConcurrentSequence(index *ast.IndexExpr) bool {
	if index == nil || !NeedsConcurrentWrapper() {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return typeInfo.IsSlice(index.X) || typeInfo.IsArray(index.X) || typeInfo.IsPointerToArray(index.X)
}

func indexReadsConcurrentString(index *ast.IndexExpr) bool {
	if index == nil || !NeedsConcurrentWrapper() {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return typeInfo.IsString(index.X)
}

func logicalConditionCallConvertsFieldSelector(call *ast.CallExpr) bool {
	if call == nil || len(call.Args) != 1 || !NeedsConcurrentWrapper() {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsTypeConversion(call) {
		return false
	}
	return exprContainsTypedFieldSelector(call.Args[0])
}

func logicalConditionBinaryIsComparison(expr *ast.BinaryExpr) bool {
	if expr == nil {
		return false
	}
	return expr.Op == token.EQL || expr.Op == token.NEQ || expr.Op == token.LSS ||
		expr.Op == token.GTR || expr.Op == token.LEQ || expr.Op == token.GEQ
}

func countNamedIntegerLogicalComparisonOperandComplexity(expr ast.Expr) int {
	if expr == nil || !NeedsConcurrentWrapper() {
		return 0
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return 0
	}
	typ := typeInfo.GetType(expr)
	if typ == nil || !isNamedIntegerType(typ) {
		return 0
	}
	return concurrentNamedIntegerLogicalComparisonOperandComplexity
}

func countGoErrorLogicalComparisonComplexity(expr *ast.BinaryExpr) int {
	if expr == nil || !NeedsConcurrentWrapper() || expr.Op != token.EQL && expr.Op != token.NEQ {
		return 0
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return 0
	}
	leftType := typeInfo.GetType(expr.X)
	rightType := typeInfo.GetType(expr.Y)
	if isGoErrorType(leftType) && (isConcreteGoErrorValue(rightType) || isGoErrorType(rightType)) {
		return concurrentGoErrorLogicalComparisonComplexity
	}
	if isGoErrorType(rightType) && isConcreteGoErrorValue(leftType) {
		return concurrentGoErrorLogicalComparisonComplexity
	}
	return 0
}

func concurrentLogicalConditionSelectorIsField(sel *ast.SelectorExpr) bool {
	if sel == nil || !NeedsConcurrentWrapper() {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	selection, ok := typeInfo.info.Selections[sel]
	return ok && selection.Kind() == types.FieldVal
}

func currentLineIndent(out *strings.Builder) string {
	line := currentLineText(out)
	indentEnd := 0
	for indentEnd < len(line) && (line[indentEnd] == ' ' || line[indentEnd] == '\t') {
		indentEnd++
	}
	return line[:indentEnd]
}

func currentLineText(out *strings.Builder) string {
	text := out.String()
	lineStart := strings.LastIndex(text, "\n") + 1
	return text[lineStart:]
}

func writeStatementBuiltLogicalConditionValue(out *strings.Builder, expr ast.Expr, indent string, nextTemp *int) {
	if unary, ok := unwrapParens(expr).(*ast.UnaryExpr); ok && unary.Op == token.NOT && shouldStatementBuildNegatedLogicalCondition(unary.X) {
		out.WriteString("!(")
		writeStatementBuiltLogicalConditionValue(out, unary.X, indent, nextTemp)
		out.WriteString(")")
		return
	}

	binary, ok := unwrapParens(expr).(*ast.BinaryExpr)
	if !ok || (binary.Op != token.LAND && binary.Op != token.LOR) {
		transpileConditionInline(out, expr)
		return
	}

	out.WriteString("{\n")
	left := writeStatementBuiltLogicalConditionTemp(out, binary.X, indent+"    ", nextTemp)
	out.WriteString(indent)
	out.WriteString("    if ")
	out.WriteString(left)
	out.WriteString(" {\n")
	if binary.Op == token.LAND {
		right := writeStatementBuiltLogicalConditionTemp(out, binary.Y, indent+"        ", nextTemp)
		out.WriteString(indent)
		out.WriteString("        ")
		out.WriteString(right)
		out.WriteString("\n")
		out.WriteString(indent)
		out.WriteString("    } else {\n")
		out.WriteString(indent)
		out.WriteString("        false\n")
	} else {
		out.WriteString(indent)
		out.WriteString("        true\n")
		out.WriteString(indent)
		out.WriteString("    } else {\n")
		right := writeStatementBuiltLogicalConditionTemp(out, binary.Y, indent+"        ", nextTemp)
		out.WriteString(indent)
		out.WriteString("        ")
		out.WriteString(right)
		out.WriteString("\n")
	}
	out.WriteString(indent)
	out.WriteString("    }\n")
	out.WriteString(indent)
	out.WriteString("}")
}

func writeStatementBuiltLogicalConditionTemp(out *strings.Builder, expr ast.Expr, indent string, nextTemp *int) string {
	name := fmt.Sprintf("__go_cond_%d", *nextTemp)
	*nextTemp++
	out.WriteString(indent)
	out.WriteString("let ")
	out.WriteString(name)
	out.WriteString(" = ")
	if shouldWriteStatementBuiltLogicalConditionTemp(expr) {
		writeStatementBuiltLogicalConditionValue(out, expr, indent, nextTemp)
	} else {
		transpileConditionInline(out, expr)
	}
	out.WriteString(";\n")
	return name
}

func shouldWriteStatementBuiltLogicalConditionTemp(expr ast.Expr) bool {
	if countLogicalConditionNodes(expr) > 0 {
		return true
	}
	if unary, ok := unwrapParens(expr).(*ast.UnaryExpr); ok && unary.Op == token.NOT {
		return shouldWriteStatementBuiltLogicalConditionTemp(unary.X)
	}
	return false
}

func transpileConditionInline(out *strings.Builder, expr ast.Expr) {
	switch e := expr.(type) {
	case *ast.ParenExpr:
		out.WriteString("(")
		transpileConditionInline(out, e.X)
		out.WriteString(")")
		return
	case *ast.BinaryExpr:
		if e.Op == token.LAND || e.Op == token.LOR {
			transpileConditionInline(out, e.X)
			out.WriteString(" ")
			out.WriteString(rustBinaryOp(e.Op))
			out.WriteString(" ")
			transpileConditionInline(out, e.Y)
			return
		}
	case *ast.UnaryExpr:
		if e.Op == token.NOT {
			out.WriteString("!")
			transpileConditionInline(out, e.X)
			return
		}
	case *ast.CallExpr:
		if exprNeedsBoolWrapperUnwrap(e) {
			writeUnwrappedBoolExpression(out, e)
			return
		}
	}
	if writeNamedBoolUnderlyingValue(out, expr) {
		return
	}
	if exprNeedsBoolWrapperUnwrap(expr) {
		writeUnwrappedBoolExpression(out, expr)
		return
	}
	TranspileExpression(out, expr)
}

func callReturnsWrappedBool(call *ast.CallExpr) bool {
	if callReturnsWrappedBoolBySyntax(call) {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || callReturnsBareChannelValue(call) || !typeInfo.ReturnsWrappedValue(call) {
		return false
	}
	callType := typeInfo.GetType(call)
	if callType == nil {
		return false
	}
	basic, ok := callType.Underlying().(*types.Basic)
	return ok && basic.Kind() == types.Bool
}

func callReturnsWrappedBoolBySyntax(call *ast.CallExpr) bool {
	if call == nil {
		return false
	}
	// With bare-scalar return widening, calls whose signature returns a
	// predeclared bool now produce a bare Rust bool — they don't need the
	// wrap-then-unwrap dance some call sites still apply.
	if callReturnsBareScalar(call) {
		return false
	}
	if ident, ok := call.Fun.(*ast.Ident); ok {
		if sig := GetFunctionSignature(ident.Name); sig != nil && len(sig.Results) == 1 {
			if resultIdent, ok := sig.Results[0].Type.(*ast.Ident); ok && resultIdent.Name == "bool" {
				return true
			}
		}
		if info := lookupVarInfo(ident.Name); info != nil && functionBoxTypeReturnsWrappedBool(info.RustType) {
			return true
		}
	}
	if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
		if selectorMethodReturnsBoolBySyntax(sel) {
			return true
		}
		if fieldExpr, ok := selectorFieldTypeExpr(sel); ok {
			if ident, ok := fieldExpr.(*ast.Ident); ok {
				if rustType, ok := FunctionTypeAliasBox(ident.Name); ok && functionBoxTypeReturnsWrappedBool(rustType) {
					return true
				}
			}
			if fnType, ok := fieldExpr.(*ast.FuncType); ok && fnType.Results != nil && len(fnType.Results.List) == 1 {
				if resultIdent, ok := fnType.Results.List[0].Type.(*ast.Ident); ok && resultIdent.Name == "bool" {
					return true
				}
			}
		}
	}
	return false
}

func selectorMethodReturnsBoolBySyntax(sel *ast.SelectorExpr) bool {
	sig := selectorMethodSignatureBySyntax(sel)
	if sig == nil || len(sig.Results) != 1 {
		return false
	}
	resultIdent, ok := sig.Results[0].Type.(*ast.Ident)
	return ok && resultIdent.Name == "bool"
}

func selectorMethodSignatureBySyntax(sel *ast.SelectorExpr) *FunctionSignature {
	if sel == nil {
		return nil
	}
	typeName, ok := selectorBaseSyntaxTypeName(sel.X)
	if !ok {
		return uniqueMethodSignatureByName(sel.Sel.Name)
	}

	candidates := []string{typeName}
	if strings.HasPrefix(typeName, "*") {
		candidates = append(candidates, strings.TrimPrefix(typeName, "*"))
	} else {
		candidates = append(candidates, "*"+typeName)
	}

	for _, candidate := range candidates {
		if sig := methodSignatureFromDecls(methodsForReceiverType(candidate), sel.Sel.Name); sig != nil {
			return sig
		}
	}
	return uniqueMethodSignatureByName(sel.Sel.Name)
}

func methodSignatureFromDecls(methods []*ast.FuncDecl, name string) *FunctionSignature {
	for _, method := range methods {
		if method == nil || method.Name == nil || method.Name.Name != name {
			continue
		}
		return methodSignatureFromDecl(method)
	}
	return nil
}

func uniqueMethodSignatureByName(name string) *FunctionSignature {
	var found *FunctionSignature
	seen := make(map[*ast.FuncDecl]bool)
	if ctx := GetTranspileContext(); ctx != nil && ctx.Package != nil {
		for _, methods := range ctx.Package.MethodsByType {
			for _, method := range methods {
				if seen[method] {
					continue
				}
				seen[method] = true
				if method == nil || method.Name == nil || method.Name.Name != name {
					continue
				}
				if found != nil {
					return nil
				}
				found = methodSignatureFromDecl(method)
			}
		}
	}
	for _, method := range currentTypeMethods {
		if seen[method] {
			continue
		}
		seen[method] = true
		if method == nil || method.Name == nil || method.Name.Name != name {
			continue
		}
		if found != nil {
			return nil
		}
		found = methodSignatureFromDecl(method)
	}
	return found
}

func methodSignatureFromDecl(method *ast.FuncDecl) *FunctionSignature {
	if method == nil || method.Type == nil {
		return nil
	}
	var params []*ast.Field
	if method.Type.Params != nil {
		params = method.Type.Params.List
	}
	var results []*ast.Field
	if method.Type.Results != nil {
		results = method.Type.Results.List
	}
	return &FunctionSignature{Params: params, Results: results}
}

func functionBoxTypeReturnsWrappedBool(rustType string) bool {
	rustType = strings.TrimPrefix(rustType, "&")
	return strings.Contains(rustType, "-> "+GetOuterWrapperType()+"<"+GetInnerWrapperType()+"<Option<bool>>>")
}

func hasStatementPreprocessor() bool {
	return statementPreprocessor != nil
}

func rangeTypeFacts(expr ast.Expr) (bool, bool, bool, bool, bool, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false, false, false, false, false, false
	}
	isMap := typeInfo.IsMap(expr)
	isString := typeInfo.IsString(expr)
	isInteger := isIntegerRangeExpr(typeInfo, expr)
	isSlice := typeInfo.IsSlice(expr)
	isArray := typeInfo.IsArray(expr)
	if !isArray {
		isArray = typeInfo.IsPointerToArray(expr)
	}
	isChannel := typeInfo.IsChannel(expr)
	if !isString {
		if lit, ok := expr.(*ast.BasicLit); ok && lit.Kind == token.STRING {
			isString = true
		}
	}
	return isMap, isString, isInteger, isSlice, isArray, isChannel
}

func rangeMapKeyValueTypes(expr ast.Expr) (types.Type, types.Type) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, nil
	}
	valueType := typeInfo.GetMapValueType(expr)
	if valueType == nil {
		return nil, nil
	}
	keyType, _ := typeInfo.GetMapTypes(expr)
	return keyType, valueType
}

func rangeExprGoType(expr ast.Expr) types.Type {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil
	}
	return typeInfo.GetType(expr)
}

func rangeArrayOrSliceElemType(expr ast.Expr) types.Type {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil
	}
	return typeInfo.GetArrayOrSliceElemTypePreservingTypeParam(expr)
}

func TranspileStatement(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet, comments []*ast.CommentGroup, lastPos *token.Pos, indent string) {
	if prelude, rewritten, ok := multiValueArgumentStatement(stmt); ok {
		// g(f()) with a multi-value f: bind f's results, then call g.
		for i, binding := range prelude {
			if i == 0 {
				TranspileStatement(out, binding, fnType, fileSet, comments, lastPos, indent)
			} else {
				TranspileStatement(out, binding, fnType, fileSet, nil, nil, indent)
			}
			out.WriteString("\n")
			out.WriteString(indent)
		}
		TranspileStatement(out, rewritten, fnType, fileSet, nil, nil, indent)
		return
	}
	markDeclaredIntConstants(stmt)
	if fileSet != nil {
		if ctx := GetTranspileContext(); ctx != nil && ctx.File != nil {
			ctx.File.FileSet = fileSet
		}
	}
	// Output any comments before this statement
	if stmt != nil && comments != nil && lastPos != nil {
		outputCommentsBeforePos(out, comments, fileSet, stmt.Pos(), indent, lastPos)
	}

	// Preprocess the statement to find closures and generate clone statements
	// Skip defer/go statements as they handle captures themselves
	var captureInfo *CaptureInfo
	_, isDefer := stmt.(*ast.DeferStmt)
	_, isGo := stmt.(*ast.GoStmt)
	_, isIf := stmt.(*ast.IfStmt)
	isSyncOnceDo := false
	if exprStmt, ok := stmt.(*ast.ExprStmt); ok {
		if call, ok := exprStmt.X.(*ast.CallExpr); ok {
			isSyncOnceDo = isSyncOnceDoFuncLitCall(call)
		}
	}
	if !isDefer && !isGo && !isIf && !isSyncOnceDo && hasStatementPreprocessor() {
		captureInfo = statementPreprocessor.PreprocessStatement(stmt, fnType)
		if captureInfo != nil && len(captureInfo.CapturedVars) > 0 {
			// Generate clone statements before the actual statement
			statementPreprocessor.GenerateCloneStatements(out, captureInfo)

			// Set up capture renames for this statement
			oldCaptureRenames := snapshotCaptureRenames()
			currentCaptureRenames = mergeCaptureRenames(oldCaptureRenames, captureInfo.CapturedVars)
			defer func() { currentCaptureRenames = oldCaptureRenames }()
			defer pushSiblingClosureCaptureClones(captureInfo, stmt)()
		}
	}

	if assign, ok := stmt.(*ast.AssignStmt); ok && assign.Tok == token.DEFINE && goSliceModeEnabled() {
		defer writeGoSliceDefineShadows(out, assign, indent)
	}
	if assign, ok := stmt.(*ast.AssignStmt); ok && assign.Tok == token.DEFINE {
		defer writeFuncValueDefineShadows(out, assign.Lhs, assign.Rhs, indent)
	}
	if decl, ok := stmt.(*ast.DeclStmt); ok {
		if genDecl, ok := decl.Decl.(*ast.GenDecl); ok && genDecl.Tok == token.VAR {
			for _, spec := range genDecl.Specs {
				if valueSpec, ok := spec.(*ast.ValueSpec); ok && len(valueSpec.Values) > 0 {
					names := make([]ast.Expr, len(valueSpec.Names))
					for i, name := range valueSpec.Names {
						names[i] = name
					}
					defer writeFuncValueDefineShadows(out, names, valueSpec.Values, indent)
				}
			}
		}
	}

	switch s := stmt.(type) {
	case *ast.SendStmt:
		// Channel send: ch <- val
		writeChannelExpression(out, s.Chan)
		out.WriteString(".send(")
		// Unwrap the value if it's wrapped
		typeInfo := GetTypeInfo()
		if ident, ok := s.Value.(*ast.Ident); ok && ident.Name == "nil" && channelOfBareCollections(s.Chan) {
			// The channel holds its slices and maps bare, and a bare one
			// cannot be nil: an empty one is sent.
			out.WriteString("Default::default()")
		} else if ident, ok := s.Value.(*ast.Ident); ok && ident.Name == "nil" && channelElemIsGoError(s.Chan) {
			// A channel holds its errors bare: nil is the error None.
			out.WriteString("None")
		} else if writeInterfaceChannelSendValue(out, s.Chan, s.Value) {
		} else if writeFuncChannelSendValue(out, s.Chan, s.Value) {
		} else if writeChannelHandleCopy(out, s.Value, nil) {
		} else if typeInfo != nil && typeInfo.IsChannel(s.Chan) {
			// The value being sent needs to be unwrapped from its wrapper
			transpileChannelValue(out, s.Value)
		} else if call, ok := s.Value.(*ast.CallExpr); ok && callReturnsWrappedBoolBySyntax(call) {
			transpileChannelValue(out, s.Value)
		} else {
			TranspileExpression(out, s.Value)
		}
		out.WriteString(");")

	case *ast.ExprStmt:
		// Check if this is a mutex Lock() call — needs guard binding
		if writeMutexLockStatement(out, s.X) {
			break
		}
		if writeMutexUnlockStatement(out, s.X) {
			break
		}
		TranspileExpression(out, s.X)
		out.WriteString(";")

	case *ast.ReturnStmt:
		if currentFunctionHasDefer && len(s.Results) > 0 && hasNamedReturns(fnType) {
			names := namedReturnIdents(fnType)
			blankTemps := make([]string, len(names))
			out.WriteString("{\n")
			if len(s.Results) == 1 && len(names) > 1 {
				out.WriteString("        let (")
				for i := range names {
					if i > 0 {
						out.WriteString(", ")
					}
					out.WriteString("mut __return_")
					out.WriteString(fmt.Sprintf("%d", i))
				}
				out.WriteString(") = ")
				TranspileExpression(out, s.Results[0])
				out.WriteString(";\n")
				for i, name := range names {
					tempName := fmt.Sprintf("__return_%d", i)
					if name.Name == "_" {
						blankTemps[i] = tempName
						continue
					}
					writeNamedReturnAssignmentFromTemp(out, name, returnResultTypeExpr(fnType, i), tempName)
				}
				out.WriteString("        // Execute deferred functions\n")
				out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
				out.WriteString("            f();\n")
				out.WriteString("        }\n")
				out.WriteString("        return ")
				writeNamedReturnValuesWithBlankTemps(out, fnType, blankTemps)
				out.WriteString(";\n    }")
				break
			}
			for i, result := range s.Results {
				if i >= len(names) {
					break
				}
				if names[i].Name == "_" {
					tempName := fmt.Sprintf("__return_%d", i)
					blankTemps[i] = tempName
					out.WriteString("        let ")
					out.WriteString(tempName)
					out.WriteString(" = ")
					writeBlankNamedReturnValue(out, result, returnResultTypeExpr(fnType, i))
					out.WriteString(";\n")
					continue
				}
				if ident, ok := result.(*ast.Ident); ok && ident.Name == names[i].Name {
					continue
				}
				out.WriteString("        ")
				resultType := returnResultTypeExpr(fnType, i)
				if !writeStdlibInterfaceNamedReturnAssignment(out, names[i], resultType, result) &&
					!writeLocalInterfaceNamedReturnAssignment(out, names[i], resultType, result) &&
					!writeEmptyInterfaceNamedReturnAssignment(out, names[i], resultType, result) &&
					!writeErrorChannelNamedReturnAssignment(out, names[i], resultType, result) &&
					!writeFunctionNamedReturnAssignment(out, names[i], resultType, result) &&
					!writePointerNamedReturnAssignment(out, names[i], resultType, result) &&
					!writeDirectTypeParamNamedReturnAssignment(out, names[i], resultType, result) {
					TranspileStatementSimple(out, &ast.AssignStmt{
						Lhs: []ast.Expr{names[i]},
						Tok: token.ASSIGN,
						Rhs: []ast.Expr{result},
					}, fnType, fileSet)
				}
				out.WriteString(";\n")
			}
			out.WriteString("        // Execute deferred functions\n")
			out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
			out.WriteString("            f();\n")
			out.WriteString("        }\n")
			out.WriteString("        return ")
			writeNamedReturnValuesWithBlankTemps(out, fnType, blankTemps)
			out.WriteString(";\n    }")
			break
		}

		tailReturn := currentReturnStatementIsTail && !currentFunctionHasDefer
		if tailReturn {
			// Tail expressions in a function body extend their temporaries
			// to the body's scope, which outlives `let`-declared locals.
			// `(*x.borrow().as_ref().unwrap())` patterns produce Ref
			// temporaries whose drop depends on the wrapped local — if the
			// local drops first, rustc rejects the body. Falling back to an
			// explicit `return <expr>;` keeps the temporaries scoped to
			// the statement so locals can drop afterward safely.
			if len(s.Results) > 0 && returnExpressionsBorrowWrappedLocals(s.Results, fnType) {
				tailReturn = false
			} else if len(s.Results) == 0 && namedReturnsUnwrapBareScalar(fnType) {
				// Naked returns lower named-return locals through the same
				// (*x.borrow().as_ref().unwrap()) pattern when the slot is a
				// bare scalar, with the same lifetime conflict.
				tailReturn = false
			} else if len(s.Results) > 0 && returnExpressionEmissionStartsWithBlock(s.Results[0]) {
				// Emitting the return value as the body's trailing expression
				// would leave the function body starting with `{`, which Rust
				// parses as a statement-block rather than the LHS of a binary
				// expression. Keep the explicit `return X;` shape so the
				// expression is unambiguous.
				tailReturn = false
			}
		}

		// Go evaluates the results, then runs the deferred calls, then
		// returns: `defer note(); return compute()` calls compute first. The
		// results are staged in a local before the defer stack drains.
		stagedDeferReturn := currentFunctionHasDefer && len(s.Results) > 0
		if stagedDeferReturn {
			out.WriteString("{\n")
			out.WriteString("        let __return_value =")
		} else if currentFunctionHasDefer {
			out.WriteString("{\n")
			out.WriteString("        // Execute deferred functions\n")
			out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
			out.WriteString("            f();\n")
			out.WriteString("        }\n")
			out.WriteString("        return")
		} else if !tailReturn {
			out.WriteString("return")
		}

		// Handle naked return (no explicit values but function has named returns)
		if len(s.Results) == 0 && fnType.Results != nil {
			hasNamedReturns := false
			for _, result := range fnType.Results.List {
				if len(result.Names) > 0 {
					hasNamedReturns = true
					break
				}
			}

			if hasNamedReturns {
				if !tailReturn {
					out.WriteString(" ")
				}
				writeNamedReturnValues(out, fnType)
			}
		} else if len(s.Results) > 0 {
			if !tailReturn {
				out.WriteString(" ")
			}
			// A tail value whose Rust starts with a block, { let __seq_holder
			// = ...; ... } < { ... } for h[i] < h[j] of a named slice, would
			// be read by Rust as a block statement and then a stray <. The
			// value is written aside and put in parentheses when more of the
			// expression follows that block (tailValueNeedsParens). The aside
			// builder starts with the current line, whose indentation the
			// multiline writers read (currentLineIndent).
			tailOut := out
			var tailValue *strings.Builder
			tailLinePrefix := ""
			if tailReturn && len(s.Results) == 1 && !fnHasMultipleResultSlots(fnType) {
				tailLinePrefix = currentLineText(out)
				tailValue = &strings.Builder{}
				tailValue.WriteString(tailLinePrefix)
				out = tailValue
			}
			// Check if we need a tuple for multiple return values
			needsTuple := len(s.Results) > 1
			multilineReturnTuple := needsTuple && returnResultsShouldUseMultiline(s.Results)
			returnTupleIndent := ""
			if needsTuple {
				out.WriteString("(")
				if multilineReturnTuple {
					returnTupleIndent = currentLineIndent(out)
					out.WriteString("\n")
				}
			}

			// `return multiResultCall(...)` keeps a single Result expression.
			// Capture it only when a slot needs typed conversion; otherwise the
			// call's tuple can become the return value directly.
			if len(s.Results) == 1 && fnHasMultipleResultSlots(fnType) {
				if call, ok := s.Results[0].(*ast.CallExpr); ok && callReturnsMultipleResults(call) {
					if !writeMultiResultCallReturnWithSlotConversions(out, call, fnType) {
						TranspileExpression(out, call)
					}
					if stagedDeferReturn {
						writeStagedDeferReturnClose(out)
					} else if currentFunctionHasDefer {
						out.WriteString(";\n    }")
					} else if !tailReturn {
						out.WriteString(";")
					}
					break
				}
			}

			for i, result := range s.Results {
				if multilineReturnTuple {
					if i > 0 {
						out.WriteString(",\n")
					}
					out.WriteString(returnTupleIndent)
					out.WriteString("    ")
				} else if i > 0 {
					out.WriteString(", ")
				}

				// Check if this is nil being returned
				isNil := false
				if ident, ok := result.(*ast.Ident); ok && ident.Name == "nil" {
					isNil = true
				}

				if writeSliceElemPtrSliceReturnValue(out, result, i) {
					// Pointer-slice return slot preserves slice-element pointer values.
				} else if writeSliceElemPtrResultReturnValue(out, result, i) {
					// Pointer return slots can preserve slice-element identity in multi-result tuples.
				} else if writeArrayElemPtrReturnValue(out, result, i) {
					// Array-element pointer return slots preserve array identity and index.
				} else if writeSliceElemPtrReturnValue(out, result) {
					// Direct slice-element pointer returns use the narrower slice element representation.
				} else if writeGoPtrReturnValue(out, result, i) {
					// GoPtr return slots preserve pointer identity through local variables and fields.
				} else if isNil {
					writeNilReturnValue(out, returnResultTypeExpr(fnType, i))
				} else if writeEmptyInterfaceReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
				} else if writeNamedSliceInnerHandleReturnValue(out, result, returnResultTypeExpr(fnType, i)) {
				} else if writeNamedSliceWrappedReturnValue(out, result, returnResultTypeExpr(fnType, i)) {
				} else if resultType := returnResultTypeExpr(fnType, i); typeExprIsRegisteredBareStructAlias(resultType) {
					writeBareStructAliasValue(out, result)
				} else if resultType := returnResultTypeExpr(fnType, i); resultTypeExprIsBareScalar(resultType) && !returnSlotForcesWrappedValue(i) {
					writeBareScalarReturnValue(out, result, resultType)
				} else if writeInterfaceReturnValueCopy(out, result, returnResultTypeExpr(fnType, i)) {
					// An interface value returned from a field, an element or a
					// package variable is a copy of it.
				} else {
					// Check if this is a field access on self (already wrapped)
					if isConcreteErrorReturnValue(result, returnResultTypeExpr(fnType, i)) {
						WriteWrapperPrefix(out)
						writeConcreteErrorBox(out, result)
						WriteWrapperSuffix(out)
					} else if sel, ok := result.(*ast.SelectorExpr); ok {
						if writeLocalInterfaceConcreteReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
							continue
						}
						if writeStdlibInterfaceReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
							continue
						}
						selectorTemp := needsTuple
						selectorTempName := fmt.Sprintf("__return_value_%d", i)
						if selectorTemp {
							out.WriteString("{ let ")
							out.WriteString(selectorTempName)
							out.WriteString(" = ")
						}
						if isFunctionSignatureExpression(result) && writeFunctionValueHandle(out, result) {
							// Function selector values are represented by cloneable handles or boxed method values.
						} else if typeInfo := GetTypeInfo(); typeInfo != nil && isGoErrorType(typeInfo.GetType(result)) && writeGoErrorReturnValue(out, result, returnResultTypeExpr(fnType, i)) {
							// An error held in a field is returned as a copy. The
							// caller's variable is its own, so assigning to it
							// does not change the field.
						} else if ident, ok := sel.X.(*ast.Ident); ok && isCurrentReceiverIdent(ident) && !receiverFieldTypeIsKnown(result) {
							// Without a type there is no telling a copied field
							// from a shared one. Say so instead of guessing.
							out.WriteString("unimplemented!(\"type info required to return a receiver field\")")
						} else if ident, ok := sel.X.(*ast.Ident); ok && isCurrentReceiverIdent(ident) && !receiverFieldIsCopied(result) {
							// Returning a receiver field that Go shares, such as a
							// pointer, slice or map, clones the handle. Route through
							// typed selector lowering so promoted fields traverse embeds.
							TranspileExpressionContext(out, result, LValue)
							out.WriteString(".clone()")
						} else if writeSourceMappedPackageGlobalPointerHandleClone(out, result) {
						} else if selectorExpressionKeepsHandle(result) {
							TranspileExpressionContext(out, result, LValue)
							out.WriteString(".clone()")
						} else if writeGoErrorReturnValue(out, result, returnResultTypeExpr(fnType, i)) {
						} else if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsPointer(result) {
							if !writeSourceMappedPackageGlobalPointerHandleClone(out, result) {
								TranspileExpressionContext(out, result, LValue)
								out.WriteString(".clone()")
							}
						} else if typeInfo := GetTypeInfo(); typeInfo != nil && isEmptyInterfaceType(typeInfo.GetType(result)) && isEmptyInterfaceExpr(returnResultTypeExpr(fnType, i)) {
							TranspileExpressionContext(out, result, LValue)
							out.WriteString(".clone()")
						} else if writeNamedIntegerWrappedInitializer(out, result) {
							// Named integer constants returned through wrapped slots
							// need to be wrapped in their newtype constructor.
						} else {
							// Regular selector - wrap it
							WriteWrapperPrefix(out)
							if !writeOwnedExpressionValue(out, result) {
								TranspileExpression(out, result)
							}
							WriteWrapperSuffix(out)
						}
						if selectorTemp {
							out.WriteString("; ")
							out.WriteString(selectorTempName)
							out.WriteString(" }")
						}
					} else if callExpr, ok := result.(*ast.CallExpr); ok {
						if writeLocalInterfaceConcreteReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
							continue
						}
						if writeStdlibInterfaceReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
							continue
						}

						// Check if this is a function that returns an already-wrapped value
						needsWrapping := true

						// Check if it's errors.New or a function returning error
						if sel, ok := callExpr.Fun.(*ast.SelectorExpr); ok {
							if ident, ok := sel.X.(*ast.Ident); ok {
								if ident.Name == "errors" && sel.Sel.Name == "New" {
									needsWrapping = false
								} else if ident.Name == "fmt" && sel.Sel.Name == "Errorf" {
									needsWrapping = false
								} else if ident.Name == "fmt" && sel.Sel.Name == "Sprintf" {
									// fmt.Sprintf already wraps its result
									needsWrapping = false
								}
							}
						}
						if GetStdlibHandler(callExpr) != nil && !isBareBuiltinReturn(callExpr) {
							needsWrapping = false
						}
						if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.ReturnsWrappedValue(callExpr) && !isBareBuiltinReturn(callExpr) && (!typeInfo.IsTypeConversion(callExpr) || typeConversionEmitsWrappedValue(callExpr)) {
							needsWrapping = false
						}

						// Check if it's a call to a user-defined function (already returns wrapped type)
						if fnIdent, ok := callExpr.Fun.(*ast.Ident); ok {
							if GetFunctionSignature(fnIdent.Name) != nil {
								needsWrapping = false
							}
						}

						// Check if it's a user function that returns error
						if fnType.Results != nil && i < len(fnType.Results.List) {
							if resultType, ok := fnType.Results.List[i].Type.(*ast.Ident); ok && resultType.Name == "error" {
								// This position is an error type, and we have a function call
								needsWrapping = false
							}
						}

						if needsWrapping {
							WriteWrapperPrefix(out)
							var expected ast.Expr
							if fnType.Results != nil && i < len(fnType.Results.List) {
								expected = fnType.Results.List[i].Type
							}
							if !writeBareBuiltinReturnForExpectedType(out, callExpr, expected) {
								TranspileExpression(out, result)
							}
							WriteWrapperSuffix(out)
						} else {
							// Already wrapped
							TranspileExpression(out, result)
						}
					} else if _, ok := result.(*ast.FuncLit); ok {
						// Function literal - already wrapped by TranspileFuncLit
						TranspileExpression(out, result)
					} else if ident, ok := result.(*ast.Ident); ok {
						if writeSliceElemPtrSliceReturnValue(out, result, i) {
							continue
						}
						if writeSliceElemPtrReturnValue(out, result) {
							continue
						}
						if writeFunctionReturnValue(out, result, returnResultTypeExpr(fnType, i)) {
							continue
						}
						if isCurrentReceiverIdent(ident) {
							if writeCurrentReceiverPointerHandleReturnValue(out, ident, returnResultTypeExpr(fnType, i)) {
								continue
							}
							if writeStdlibInterfaceCallArgumentConversion(out, ident, expectedTypeFromParamExpr(returnResultTypeExpr(fnType, i))) {
								continue
							}
							if writeLocalInterfaceConcreteReturnConversion(out, ident, returnResultTypeExpr(fnType, i)) {
								continue
							}
							WriteWrapperPrefix(out)
							out.WriteString(currentReceiverRustName())
							out.WriteString(".clone()")
							WriteWrapperSuffix(out)
							continue
						}
						if writeStdlibInterfaceIdentReturnConversion(out, ident, returnResultTypeExpr(fnType, i)) {
							continue
						}
						if writeLocalInterfaceConcreteReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
							continue
						}
						if globalIdent, ok := packageGlobalPointerIdent(ident); ok {
							writePackageGlobalPointerHandleClone(out, globalIdent)
							continue
						}
						if writeRangeHandleReturnValue(out, ident) {
							continue
						}
						if writeWrappedReferenceRangeValueCopy(out, ident) {
							continue
						}
						if writeBareDirectTypeParamWrappedReturnValue(out, ident, returnResultTypeExpr(fnType, i)) {
							continue
						}
						if typeInfo := GetTypeInfo(); typeInfo != nil && isGoErrorType(typeInfo.GetType(ident)) && writeGoErrorReturnValue(out, result, returnResultTypeExpr(fnType, i)) {
							// An error variable is returned as a copy. The caller's
							// variable is its own, so assigning to it does not
							// change this one, which may be a package-level error.
							continue
						}
						if writeWrappedValueCopyFromIdent(out, ident) {
							continue
						}
						// Check if this is a wrapped variable that needs cloning
						// Use a combination of TypeInfo and heuristics
						isWrappedVariable := false

						// First check TypeInfo
						typeInfo := GetTypeInfo()
						if isConstIdent(ident) {
							isWrappedVariable = false
						} else if typeInfo != nil && typeInfo.ReturnsWrappedValue(result) {
							isWrappedVariable = true
						} else {
							// Fallback: check if this looks like a local variable
							// (not a special identifier and not a range variable)
							if ident.Name != "true" && ident.Name != "false" && ident.Name != "nil" {
								if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
									if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
										// This is likely a wrapped variable
										isWrappedVariable = true
									}
								}
							}
						}

						if isWrappedVariable {
							// This is a wrapped variable - clone it to avoid move errors
							// Check if this variable has been renamed (captured in closure)
							varName := RustIdentForUse(ident)
							if currentCaptureRenames != nil {
								if renamed, exists := currentCaptureRenames[ident.Name]; exists {
									varName = RustLocalIdent(renamed)
								}
							}
							out.WriteString(varName)
							out.WriteString(".clone()")
						} else {
							// This needs wrapping (constants, literals, etc.)
							if ident.Name == "nil" {
								WriteWrappedNone(out)
							} else {
								var expected ast.Expr
								if fnType.Results != nil && i < len(fnType.Results.List) {
									expected = fnType.Results.List[i].Type
								}
								if isConstIdent(ident) {
									writeWrappedExpressionForExpectedType(out, result, expected)
								} else {
									WriteWrapperPrefix(out)
									// Cast usize range index to i32 when wrapping
									if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && varType == "usize" {
										out.WriteString(EscapeRustIdent(ident.Name) + " as " + rustIntType())
									} else if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && varType == "ref_value" {
										out.WriteString(EscapeRustIdent(ident.Name))
										out.WriteString(".clone()")
									} else {
										TranspileExpression(out, result)
									}
									WriteWrapperSuffix(out)
								}
							}
						}
					} else if writeChannelTypeAssertionReturnValue(out, result) {
					} else if writeLocalInterfaceConcreteReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
					} else if writeTypeParamNewDerefExpression(out, result) {
					} else if writePointerReturnValue(out, result, returnResultTypeExpr(fnType, i)) {
					} else if writeNamedIntegerWrappedInitializer(out, result) {
					} else if compositeLit, ok := result.(*ast.CompositeLit); ok && isCompositeLitSelfWrapping(compositeLit) {
						// Slice and map literals already return wrapped values.
						TranspileExpression(out, result)
					} else if _, ok := result.(*ast.SliceExpr); ok {
						// Slice expressions already return wrapped values (Arc<Mutex<Option<Vec<T>>>>)
						TranspileExpression(out, result)
					} else if mapIndexExpressionKeepsHandle(result) {
						// Map values that are maps/slices/pointers/etc. already return cloneable handles.
						TranspileExpression(out, result)
					} else if writeStdlibInterfaceReturnConversion(out, result, returnResultTypeExpr(fnType, i)) {
					} else if writeErrorChannelReceiveReturnValue(out, result, returnResultTypeExpr(fnType, i)) {
					} else if unaryExpr, ok := result.(*ast.UnaryExpr); ok {
						// Check if this is address-of a struct literal
						if unaryExpr.Op == token.AND {
							if _, isCompositeLit := unaryExpr.X.(*ast.CompositeLit); isCompositeLit {
								// Already wrapped by UnaryExpr handling, don't double-wrap
								TranspileExpression(out, result)
							} else {
								// Regular address-of, already returns wrapped value
								TranspileExpression(out, result)
							}
						} else {
							// Other unary expressions, wrap them
							WriteWrapperPrefix(out)
							TranspileExpression(out, result)
							WriteWrapperSuffix(out)
						}
					} else if binExpr, ok := result.(*ast.BinaryExpr); ok {
						if binExpr.Op == token.LAND || binExpr.Op == token.LOR {
							WriteWrapperPrefix(out)
							TranspileExpression(out, result)
							WriteWrapperSuffix(out)
							continue
						}
						if binExpr.Op == token.ADD {
							if isSyntaxStringConcatExpr(binExpr) {
								WriteWrapperPrefix(out)
								TranspileExpression(out, result)
								WriteWrapperSuffix(out)
								continue
							}
							if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.IsString(binExpr) {
								WriteWrapperPrefix(out)
								TranspileExpression(out, result)
								WriteWrapperSuffix(out)
								continue
							}
						}
						if binExpr.Op == token.EQL || binExpr.Op == token.NEQ {
							if isRightNilComparison(binExpr) {
								WriteWrapperPrefix(out)
								TranspileExpression(out, result)
								WriteWrapperSuffix(out)
								continue
							}
							var cmp strings.Builder
							if writeCurrentReceiverPointerComparison(&cmp, binExpr) {
								WriteWrapperPrefix(out)
								out.WriteString(cmp.String())
								WriteWrapperSuffix(out)
								continue
							}
							if writeGoErrorEquality(&cmp, binExpr) {
								WriteWrapperPrefix(out)
								out.WriteString(cmp.String())
								WriteWrapperSuffix(out)
								continue
							}
							if writeEmptyInterfaceEquality(&cmp, binExpr) {
								WriteWrapperPrefix(out)
								out.WriteString(cmp.String())
								WriteWrapperSuffix(out)
								continue
							}
							if writeLocalInterfaceEquality(&cmp, binExpr.X, binExpr.Y, binExpr.Op) {
								WriteWrapperPrefix(out)
								out.WriteString(cmp.String())
								WriteWrapperSuffix(out)
								continue
							}
						}

						// Binary expressions need special handling to avoid multiple locks
						// Check if operands are identifiers that would need unwrapping
						needsExtraction := false
						if ident, ok := binExpr.X.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "true" && ident.Name != "false" {
							if _, isConst := localConstants[ident.Name]; !isConst {
								if _, isRange := rangeLoopVars[ident.Name]; !isRange {
									needsExtraction = true
								}
							}
						}
						if ident, ok := binExpr.Y.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "true" && ident.Name != "false" {
							if _, isConst := localConstants[ident.Name]; !isConst {
								if _, isRange := rangeLoopVars[ident.Name]; !isRange {
									needsExtraction = true
								}
							}
						}

						if needsExtraction {
							// Extract values first to avoid multiple locks
							// We need both operands to be unwrapped for the binary operation
							out.WriteString("{\n")

							// Get TypeInfo to check if expressions return wrapped values
							typeInfo := GetTypeInfo()
							writeTempOperand := func(expr ast.Expr, other ast.Expr) {
								if typeInfo != nil && writeStdlibInterfaceComparableConversion(out, expr, typeInfo.GetType(other)) {
									// Concrete stdlib value converted for comparison with stdlib interface.
								} else if writeLenCapBinaryOperand(out, expr, other) {
									// len/cap emitted as Go int representation for this return expression.
								} else if writeIntPeerForLenCapBinaryOperand(out, expr, other, typeInfo != nil && typeInfo.ReturnsWrappedValue(expr)) {
									// typed int peer emitted as Go int representation for this return expression.
								} else if writeNamedConstForBinaryPeer(out, expr, other) {
									// typed named constants are constructed as their named newtype when compared with named values.
								} else if sel, ok := expr.(*ast.SelectorExpr); ok && writeSyntaxNamedSelectorValue(out, sel) {
									// Named selector fields need a syntax fallback when type info only proves the field is wrapped.
								} else if expressionReturnsWrappedValueForExtraction(typeInfo, expr) {
									// Expression returns wrapped value, unwrap it.
									out.WriteString("(*")
									writeExpressionForBorrow(out, expr)
									WriteBorrowMethod(out, false)
									out.WriteString(".as_ref().unwrap())")
									if isCloneableNonPointerExpr(expr) && !isCopyTypeExpression(expr) {
										out.WriteString(".clone()")
									}
								} else if !isCopyTypeExpression(expr) && writeOwnedExpressionValue(out, expr) {
									// Wrapped identifiers/selectors with owned values must clone out of the borrow.
								} else {
									// Either a literal/constant or an identifier that will unwrap itself in RValue context
									TranspileExpressionContext(out, expr, RValue)
								}
							}

							// Extract X operand
							out.WriteString("            let __tmp_x = ")
							writeTempOperand(binExpr.X, binExpr.Y)
							out.WriteString(";\n")

							// Extract Y operand
							out.WriteString("            let __tmp_y = ")
							writeTempOperand(binExpr.Y, binExpr.X)
							out.WriteString(";\n")

							out.WriteString("            ")
							WriteWrapperPrefix(out)
							out.WriteString("__tmp_x ")
							out.WriteString(rustBinaryOp(binExpr.Op))
							out.WriteString(" __tmp_y")
							WriteWrapperSuffix(out)
							out.WriteString("\n")
							out.WriteString("        }")
						} else {
							// No extraction needed
							WriteWrapperPrefix(out)
							TranspileExpression(out, result)
							WriteWrapperSuffix(out)
						}
					} else {
						// Check if this return position expects an error type
						// and the result is a struct literal implementing Error
						isErrorReturn := false
						if fnType.Results != nil && i < len(fnType.Results.List) {
							resultField := fnType.Results.List[i]
							if resultType, ok := resultField.Type.(*ast.Ident); ok && resultType.Name == "error" {
								if _, isComposite := result.(*ast.CompositeLit); isComposite {
									isErrorReturn = true
								}
							}
						}

						if isErrorReturn {
							// Struct implementing error interface - box it
							TrackImport("Error")
							WriteWrapperPrefix(out)
							out.WriteString(rustErrorFromValueStart())
							TranspileExpression(out, result)
							out.WriteString(rustErrorFromValueEnd())
							WriteWrapperSuffix(out)
						} else if writeGoErrorReturnValue(out, result, returnResultTypeExpr(fnType, i)) {
						} else if isConcreteErrorReturnValue(result, returnResultTypeExpr(fnType, i)) {
							WriteWrapperPrefix(out)
							writeConcreteErrorBox(out, result)
							WriteWrapperSuffix(out)
						} else {
							// Wrap all other return values in Arc<Mutex<Option<>>>
							WriteWrapperPrefix(out)

							// Special handling for string literals
							if writeExpressionForExpectedType(out, result, returnResultTypeExpr(fnType, i)) {
							} else if lit, ok := result.(*ast.BasicLit); ok && lit.Kind == token.STRING {
								out.WriteString(RustStringLiteral(lit.Value))
								out.WriteString(".to_string()")
							} else {
								TranspileExpression(out, result)
							}

							WriteWrapperSuffix(out)
						}
					}
				}
			}

			if needsTuple {
				if multilineReturnTuple {
					out.WriteString("\n")
					out.WriteString(returnTupleIndent)
				}
				out.WriteString(")")
			}
			if tailValue != nil {
				out = tailOut
				value := strings.TrimPrefix(tailValue.String(), tailLinePrefix)
				if tailValueNeedsParens(value) {
					out.WriteString("(")
					out.WriteString(value)
					out.WriteString(")")
				} else {
					out.WriteString(value)
				}
			}
		} else if tailReturn {
			out.WriteString("()")
		}

		// Close the defer execution block if needed
		if stagedDeferReturn {
			writeStagedDeferReturnClose(out)
		} else if currentFunctionHasDefer {
			// The `;` after `return X` keeps the return value's temporaries
			// scoped to the statement so the surrounding wrapped locals can
			// drop afterward. Without it the block's tail expression rules
			// extend Refs past the locals they borrow from.
			out.WriteString(";\n    }")
		} else if !tailReturn {
			out.WriteString(";")
		}

	case *ast.AssignStmt:
		if s.Tok == token.DEFINE {
			registerDefaultShortDeclBindings(s.Lhs)
		}
		if s.Tok == token.DEFINE && len(s.Lhs) == 1 && len(s.Rhs) == 1 {
			if ident, ok := s.Lhs[0].(*ast.Ident); ok {
				var init strings.Builder
				if writeSliceElemPtrMapInitializer(&init, ident, s.Rhs[0]) {
					out.WriteString("let mut ")
					out.WriteString(RustLocalIdent(ident.Name))
					out.WriteString(" = ")
					out.WriteString(init.String())
					out.WriteString(";")
					break
				}
			}
		}
		if rewritten, ok := goShiftCompoundAsAssignment(s); ok {
			TranspileStatement(out, rewritten, fnType, fileSet, comments, lastPos, indent)
			return
		}

		// Check if this is a map index assignment using type information
		isMapIndexAssign := false
		if len(s.Lhs) == 1 && len(s.Rhs) == 1 && s.Tok == token.ASSIGN {
			if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok {
				// Use TypeInfo to determine if this is actually a map
				typeInfo := GetTypeInfo()
				if typeInfo != nil {
					isMapIndexAssign = typeInfo.IsMap(indexExpr.X)
				}
				if !isMapIndexAssign {
					if kind, ok := localCollectionKind(indexExpr.X); ok {
						isMapIndexAssign = kind == "map"
					}
				}
				if !isMapIndexAssign && typeInfo == nil {
					// Type info not available - can't determine if it's a map
					// Generate an error comment to make this obvious
					out.WriteString("/* ERROR: Cannot determine if map assignment - type information required */ ")
				}
			}
		}

		if isMapIndexAssign {
			// Handle map[key] = value as map.insert(key, value)
			if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok {
				var keyType types.Type
				var valueType types.Type
				var keyRustType string
				if syntaxKeyType, ok := localMapKeyRustType(indexExpr.X); ok {
					keyRustType = syntaxKeyType
				}
				if typeInfo := GetTypeInfo(); typeInfo != nil {
					keyType, valueType = typeInfo.GetMapTypes(indexExpr.X)
				}
				// A closure on the right ("m.handlers[k] = func() { m.n++ }")
				// moves the captured clone of m into itself, so the map's
				// handle is taken first, as Go evaluates the map operand
				// before the value.
				holdsMapHandle := exprHasFuncLit(s.Rhs[0]) && mapOperandHandleCanBeHeld(indexExpr.X)
				out.WriteString("{ ")
				if holdsMapHandle {
					out.WriteString("let __map_holder = ")
					writeMapHandleCloneForOp(out, indexExpr.X)
					out.WriteString("; ")
				}
				out.WriteString("let __map_key = ")
				if mapAssignmentKeyRequiresTypeDrivenConversion(keyType) || !writeMapAssignmentKeyExpressionWithRustType(out, indexExpr.Index, keyRustType) {
					writeMapAssignmentKeyExpression(out, indexExpr.Index, keyType, s.Rhs[0])
				}
				out.WriteString("; let __map_value = ")
				if !writeSliceElemPtrMapValue(out, indexExpr.X, s.Rhs[0]) {
					writeMapWrappedValue(out, s.Rhs[0], valueType)
				}
				out.WriteString("; (*")
				// For map access, we need the raw identifier, not the unwrapped value
				if holdsMapHandle {
					out.WriteString("__map_holder")
				} else {
					writeMapHandleForOp(out, indexExpr.X)
				}
				WriteBorrowMethod(out, true)
				writeMapMutationUnwrap(out, indexExpr.X)
				out.WriteString(").insert(__map_key, __map_value); }")
			}
		} else if writeGoSliceHeaderAssignment(out, s) {
			// A slice variable keeps its cell and takes the new header.
		} else if isChannelAssignment(s) {
			if writeGoPtrFieldChannelAssignment(out, s.Lhs[0], s.Rhs[0]) {
				// t.C = c through a GoPtr, as time.NewTicker sets its channel.
			} else {
				writeChannelAssignmentTarget(out, s.Lhs[0])
				out.WriteString(" = ")
				writeChannelAssignmentValue(out, s.Rhs[0])
			}
		} else if s.Tok == token.ADD_ASSIGN || s.Tok == token.SUB_ASSIGN ||
			s.Tok == token.MUL_ASSIGN || s.Tok == token.QUO_ASSIGN || s.Tok == token.REM_ASSIGN ||
			s.Tok == token.AND_ASSIGN || s.Tok == token.AND_NOT_ASSIGN || s.Tok == token.OR_ASSIGN || s.Tok == token.XOR_ASSIGN ||
			s.Tok == token.SHL_ASSIGN || s.Tok == token.SHR_ASSIGN {
			// Compound assignment operators
			if indexExpr, isMapIndex := isMapIndexExpression(s.Lhs[0]); isMapIndex {
				writeMapElementUpdate(out, indexExpr, s.Tok, s.Rhs[0])
			} else if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok && writeIndexedCompoundAssign(out, indexExpr, s.Tok, s.Rhs[0]) {
				// array/slice element compound assignment mutates the underlying sequence directly.
			} else if writeBareExternalIntegerCompoundAssign(out, s.Lhs[0], s.Tok, s.Rhs[0]) {
				// Bare external named integer locals mutate by rewrapping their primitive field.
			} else if writeBareNamedIntegerCompoundAssign(out, s.Lhs[0], s.Tok, s.Rhs[0]) {
				// Bare local named integer values use their generated operator impls directly.
			} else if writeBareScalarCompoundAssign(out, s.Lhs[0], s.Tok, s.Rhs[0]) {
				// Bare scalar locals mutate directly.
			} else if writeGoPtrDerefCompoundAssign(out, s.Lhs[0], s.Tok, s.Rhs[0]) {
				// Dereferenced GoPtr values mutate through the GoPtr handle.
			} else {

				isString := false
				if s.Tok == token.ADD_ASSIGN {
					typeInfo := GetTypeInfo()
					lhsTyped := false
					if typeInfo != nil {
						isString = typeInfo.IsString(s.Lhs[0])
						lhsTyped = typeInfo.GetType(s.Lhs[0]) != nil
					}
					// The variable table is keyed by name, so a string variable of
					// an inner scope can stand in for an int of the same name
					// outside it; the type checker's answer comes first.
					if !isString && !lhsTyped && (isSyntaxStringValue(s.Lhs[0]) || isSyntaxStringConversion(s.Rhs[0])) {
						isString = true
					}
					// A named string newtype concatenates through its Add impl.
					// In a method of the type itself, `*n += "!"` updates the String
					// the receiver holds, which push_str does.
					namedStringNewtype := typeInfo != nil && isNamedScalarNewtype(compoundAssignRHSExpectedType(s.Lhs[0], typeInfo.GetType(s.Lhs[0])))
					if namedStringNewtype {
						isString = false
					}
					if !isString && !namedStringNewtype {
						// Type info not available - check if RHS is a string literal at least
						if lit, ok := s.Rhs[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
							isString = true
							out.WriteString("/* WARNING: Assuming string type based on literal */ ")
						}
					}
				}

				if isString {
					// For string +=, we need mutable access to the LHS
					out.WriteString("{ ")
					writeWrappedMutationTargetPrelude(out, s.Lhs[0])
					out.WriteString("(*")
					writeWrappedMutationTargetRef(out, s.Lhs[0], true)
					out.WriteString(".as_mut().unwrap()).push_str(&")
					writeStringAppendExpression(out, s.Rhs[0])
					out.WriteString("); }")
				} else {
					// Numeric compound assignment for wrapped values.
					// Evaluate RHS before taking the LHS borrow so RHS expressions
					// that read LHS (e.g. `i += copy(dst[i:], src)`) don't deadlock
					// on RefCell.
					out.WriteString("{ ")
					writeWrappedMutationTargetPrelude(out, s.Lhs[0])
					out.WriteString("let __rhs = ")
					typeInfo := GetTypeInfo()
					var expected types.Type
					if typeInfo != nil {
						expected = typeInfo.GetType(s.Lhs[0])
					}
					expected = compoundAssignRHSExpectedType(s.Lhs[0], expected)
					writeBareCompoundAssignValueForOp(out, s.Rhs[0], expected, s.Tok)
					out.WriteString("; let mut guard = ")
					writeWrappedMutationTargetRef(out, s.Lhs[0], true)
					out.WriteString("; *guard = Some(")
					if compoundAssignUsesOwnedNamedIntegerValue(s.Lhs[0], s.Tok) {
						out.WriteString("guard.as_ref().unwrap().clone()")
					} else {
						out.WriteString("guard.as_ref().unwrap()")
					}
					out.WriteString(" ")

					// Output the appropriate operator
					switch s.Tok {
					case token.ADD_ASSIGN:
						out.WriteString("+")
					case token.SUB_ASSIGN:
						out.WriteString("-")
					case token.MUL_ASSIGN:
						out.WriteString("*")
					case token.QUO_ASSIGN:
						out.WriteString("/")
					case token.REM_ASSIGN:
						out.WriteString("%")
					case token.AND_ASSIGN:
						out.WriteString("&")
					case token.AND_NOT_ASSIGN:
						out.WriteString("& !")
					case token.OR_ASSIGN:
						out.WriteString("|")
					case token.XOR_ASSIGN:
						out.WriteString("^")
					case token.SHL_ASSIGN:
						out.WriteString("<<")
					case token.SHR_ASSIGN:
						out.WriteString(">>")
					}

					out.WriteString(" __rhs); }")
				}
			}
		} else { // Check if we have multiple LHS with single RHS (tuple unpacking)
			needsTupleUnpack := len(s.Lhs) > 1 && len(s.Rhs) == 1

			// Check if this is a map access with existence check: value, ok := map[key]
			isMapAccess := false
			isTypeAssertion := false
			isChannelRecv := false
			if needsTupleUnpack && len(s.Lhs) == 2 {
				if indexExpr, ok := s.Rhs[0].(*ast.IndexExpr); ok {
					// Check if the indexed expression is a map
					typeInfo := GetTypeInfo()
					if typeInfo != nil {
						isMapAccess = typeInfo.IsMap(indexExpr.X)
					}
					if !isMapAccess {
						if kind, ok := localCollectionKind(indexExpr.X); ok {
							isMapAccess = kind == "map"
						}
					}
					if !isMapAccess && typeInfo == nil {
						// Type info not available - cannot determine
						out.WriteString("/* ERROR: Cannot determine if map access - type information required */ ")
						isMapAccess = false
					}
				} else if _, ok := s.Rhs[0].(*ast.TypeAssertExpr); ok {
					// This is a type assertion with comma-ok
					isTypeAssertion = true
				} else if unary, ok := s.Rhs[0].(*ast.UnaryExpr); ok && unary.Op == token.ARROW {
					// This is a channel receive with comma-ok: value, ok := <-ch
					isChannelRecv = true
				}
			}

			if isChannelRecv && needsTupleUnpack {
				// Handle channel receive with comma-ok: value, ok := <-ch
				unary := s.Rhs[0].(*ast.UnaryExpr)
				isErrorChannelRecv := channelElementIsHandleOption(unary.X)
				if s.Tok == token.DEFINE {
					// Generate: let (value, ok) = match ch.recv() { ... }
					out.WriteString("let (")
					writeCommaOkBindingName(out, s.Lhs[0])
					out.WriteString(", ")
					writeCommaOkBindingName(out, s.Lhs[1])
					out.WriteString(") = match ")
					writeChannelExpression(out, unary.X)
					out.WriteString(".recv() { Some(v) => (")
					if isErrorChannelRecv {
						writeErrorHandleFromOptionValue(out, "v")
					} else {
						WriteWrapperPrefix(out)
						out.WriteString("v")
						WriteWrapperSuffix(out)
					}
					out.WriteString(", ")
					WriteWrapperPrefix(out)
					out.WriteString("true")
					WriteWrapperSuffix(out)
					out.WriteString("), None => (")
					if isErrorChannelRecv {
						writeEmptyChannelHandle(out, unary.X)
					} else {
						WriteWrapperPrefix(out)
						out.WriteString("Default::default()")
						WriteWrapperSuffix(out)
					}
					out.WriteString(", ")
					WriteWrapperPrefix(out)
					out.WriteString("false")
					WriteWrapperSuffix(out)
					out.WriteString(") }")
				} else {
					// Reassignment: value, ok = <-ch. A blank target discards its
					// value.
					valueTarget := !isBlankIdentExpr(s.Lhs[0])
					okTarget := !isBlankIdentExpr(s.Lhs[1])
					writeTarget := func(lhs ast.Expr) {
						out.WriteString(" *")
						TranspileExpressionContext(out, lhs, LValue)
						WriteBorrowMethod(out, true)
					}
					out.WriteString("match ")
					writeChannelExpression(out, unary.X)
					out.WriteString(".recv() { Some(")
					if valueTarget {
						out.WriteString("v")
					} else {
						out.WriteString("_")
					}
					out.WriteString(") => {")
					if valueTarget {
						writeTarget(s.Lhs[0])
						if isErrorChannelRecv {
							out.WriteString(" = v;")
						} else {
							out.WriteString(" = Some(v);")
						}
					}
					if okTarget {
						writeTarget(s.Lhs[1])
						out.WriteString(" = Some(true);")
					}
					out.WriteString(" }, None => {")
					if valueTarget {
						writeTarget(s.Lhs[0])
						if isErrorChannelRecv {
							out.WriteString(" = None::<")
							out.WriteString(channelHandleOptionPayloadType(unary.X))
							out.WriteString(">;")
						} else {
							out.WriteString(" = Some(Default::default());")
						}
					}
					if okTarget {
						writeTarget(s.Lhs[1])
						out.WriteString(" = Some(false);")
					}
					out.WriteString(" } }")
				}
			} else if isTypeAssertion && needsTupleUnpack {
				// Handle type assertion with comma-ok: value, ok := x.(Type)
				typeAssert := s.Rhs[0].(*ast.TypeAssertExpr)

				if s.Tok == token.DEFINE {
					out.WriteString("let (")
					// First variable for value
					if ident, ok := s.Lhs[0].(*ast.Ident); ok && ident.Name != "_" {
						out.WriteString("mut ")
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						out.WriteString("_")
					}
					out.WriteString(", ")
					// Second variable for ok
					if ident, ok := s.Lhs[1].(*ast.Ident); ok && ident.Name != "_" {
						out.WriteString("mut ")
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						out.WriteString("_")
					}
					out.WriteString(") = ")

					// Generate type assertion code with comma-ok
					TranspileTypeAssertionCommaOk(out, typeAssert)
					registerTypeAssertionResultInfo(s.Lhs[0], typeAssert)
					registerBareShortDecl(s.Lhs[1])
				} else {
					out.WriteString("{ let (__tmp_0, __tmp_1) = ")
					TranspileTypeAssertionCommaOk(out, typeAssert)
					out.WriteString(";")
					writeTupleAssignmentFromTemp(out, s.Lhs[0], "__tmp_0", false)
					writeTupleAssignmentFromTemp(out, s.Lhs[1], "__tmp_1", true)
					out.WriteString(" }")
				}
			} else if isMapAccess && needsTupleUnpack {
				// Handle map access with existence check
				indexExpr := s.Rhs[0].(*ast.IndexExpr)
				var keyType types.Type
				var keyRustType string
				valueKeepsHandle := mapValueSyntaxKeepsHandle(indexExpr.X)
				if syntaxKeyType, ok := localMapKeyRustType(indexExpr.X); ok {
					keyRustType = syntaxKeyType
				}
				if typeInfo := GetTypeInfo(); typeInfo != nil {
					keyType, _ = typeInfo.GetMapTypes(indexExpr.X)
				}

				if s.Tok == token.DEFINE {
					out.WriteString("let (")
					// First variable for value
					if ident, ok := s.Lhs[0].(*ast.Ident); ok && ident.Name != "_" {
						out.WriteString("mut ")
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						out.WriteString("_")
					}
					out.WriteString(", ")
					// Second variable for existence
					if ident, ok := s.Lhs[1].(*ast.Ident); ok && ident.Name != "_" {
						out.WriteString("mut ")
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						out.WriteString("_")
					}
					out.WriteString(") = ")
				} else {
					out.WriteString("{ let (__tmp_0, __tmp_1) = ")
				}

				// Generate the map access code.
				mapIsBareValue := isExpressionResultBare(indexExpr.X)
				if mapIsBareValue {
					out.WriteString("match ")
					TranspileExpression(out, indexExpr.X)
					out.WriteString(".get(")
				} else if NeedsConcurrentWrapper() {
					out.WriteString("{ let __map = ")
					writeOptionalClonedMapExpression(out, indexExpr.X)
					out.WriteString("; match __map.as_ref().and_then(|__map| __map.get(")
				} else {
					out.WriteString("{ let __map_holder = ")
					writeMapHandleCloneForOp(out, indexExpr.X)
					out.WriteString("; let __map_guard = __map_holder")
					WriteBorrowMethod(out, false)
					out.WriteString("; match __map_guard.as_ref().and_then(|__map| __map.get(")
				}
				if !writeMapLookupKeyWithRustType(out, indexExpr.X, indexExpr.Index, keyRustType, keyType) {
					writeMapLookupKeyWithMapExpr(out, indexExpr.X, indexExpr.Index, keyType)
				}
				if mapIsBareValue {
					// "match map.get(key) {": only the call to get is open.
					out.WriteString(") { /* MAP_COMMA_OK */ Some(v) => (")
				} else {
					out.WriteString(")) { /* MAP_COMMA_OK */ Some(v) => (")
				}
				valueTarget, valueTargetIsIdent := s.Lhs[0].(*ast.Ident)
				valueDiscarded := valueTargetIsIdent && valueTarget.Name == "_"
				// "v, ok = m[k]" for a slice or map value: a plain slice or
				// map variable takes over the entry's handle. Every other
				// target (a named slice, a package variable, "dst[i]") gets a
				// clone of the value in a cell of its own to move it from.
				takesOverHandle := s.Tok != token.DEFINE && mapCommaOkAssignTakesOverHandle(s.Lhs[0], indexExpr)
				heldInOwnCell := s.Tok != token.DEFINE && !valueDiscarded && !takesOverHandle && mapCommaOkValueIsSliceOrMap(indexExpr)
				writeMapCommaOkFoundValue(out, indexExpr, valueKeepsHandle, valueDiscarded, heldInOwnCell)
				out.WriteString(", ")
				out.WriteString("true")
				out.WriteString("), None => (")
				writeMapCommaOkMissingValue(out, indexExpr, valueKeepsHandle)
				out.WriteString(", ")
				out.WriteString("false")
				out.WriteString(") }")
				if !isExpressionResultBare(indexExpr.X) {
					out.WriteString(" }")
				}
				if s.Tok == token.DEFINE {
					registerBareShortDecl(s.Lhs[1])
				} else {
					out.WriteString(";")
					if takesOverHandle {
						out.WriteString(" ")
						writePointerHandleAssignmentTargetFromValueName(out, s.Lhs[0], "__tmp_0")
					} else {
						// A function, interface, error or channel value is
						// still moved out of the map's handle here (DEV-58).
						writeTupleAssignmentFromTemp(out, s.Lhs[0], "__tmp_0", false)
					}
					writeTupleAssignmentFromTemp(out, s.Lhs[1], "__tmp_1", true)
					out.WriteString(" }")
				}
			} else if needsTupleUnpack {
				if s.Tok == token.DEFINE {
					existingShortDeclLHS := make([]bool, len(s.Lhs))
					wrappedScalarTempLHS := make([]bool, len(s.Lhs))
					anyExistingShortDeclLHS := false
					anyWrappedScalarTempLHS := false
					var tupleCall *ast.CallExpr
					if call, ok := s.Rhs[0].(*ast.CallExpr); ok {
						tupleCall = call
					}
					for i, lhs := range s.Lhs {
						if shortDeclLHSUsesExistingBinding(lhs) {
							existingShortDeclLHS[i] = true
							anyExistingShortDeclLHS = true
							continue
						}
						if tupleCall != nil && callResultNeedsBareScalarUnwrapFromGeneratedWrapper(tupleCall, i) {
							wrappedScalarTempLHS[i] = true
							anyWrappedScalarTempLHS = true
						}
					}
					out.WriteString("let ")
					out.WriteString("(")
					for i, lhs := range s.Lhs {
						if i > 0 {
							out.WriteString(", ")
						}
						if existingShortDeclLHS[i] || wrappedScalarTempLHS[i] {
							out.WriteString(fmt.Sprintf("__tmp_%d", i))
							continue
						}
						// Don't add mut before blank identifier
						if ident, ok := lhs.(*ast.Ident); !ok || ident.Name != "_" {
							out.WriteString("mut ")
						}
						TranspileExpressionContext(out, lhs, LValue)
					}
					out.WriteString(")")

					out.WriteString(" = ")

					withShortDeclOuterRhsBindings(s.Lhs, s.Rhs[0], func() {
						TranspileExpression(out, s.Rhs[0])
					})
					if call, ok := s.Rhs[0].(*ast.CallExpr); ok {
						registrationLHS := s.Lhs
						if anyExistingShortDeclLHS || anyWrappedScalarTempLHS {
							registrationLHS = append([]ast.Expr(nil), s.Lhs...)
							for i := range registrationLHS {
								if existingShortDeclLHS[i] {
									registrationLHS[i] = ast.NewIdent("_")
								}
							}
						}
						registerCallTupleResultSyntaxInfo(registrationLHS, call)
						if anyExistingShortDeclLHS || anyWrappedScalarTempLHS {
							out.WriteString(";")
						}
						for i, lhs := range s.Lhs {
							if existingShortDeclLHS[i] {
								writeTupleAssignmentFromTemp(out, lhs, fmt.Sprintf("__tmp_%d", i), callResultIsBareScalar(call, i))
								continue
							}
							if wrappedScalarTempLHS[i] {
								if ident, ok := lhs.(*ast.Ident); ok && ident.Name != "_" {
									out.WriteString(" let mut ")
									out.WriteString(RustLocalIdent(ident.Name))
									out.WriteString(" = ")
									writeClonedTupleTempInner(out, fmt.Sprintf("__tmp_%d", i))
									out.WriteString(";")
								}
							}
						}
					}
				} else {
					if !writeMultilineTupleReassignment(out, s) {
						out.WriteString("{ let (")
						for i := range s.Lhs {
							if i > 0 {
								out.WriteString(", ")
							}
							out.WriteString(fmt.Sprintf("__tmp_%d", i))
						}
						out.WriteString(") = ")
						TranspileExpression(out, s.Rhs[0])
						out.WriteString(";")
						for i, lhs := range s.Lhs {
							tmpBareScalar := false
							if call, ok := s.Rhs[0].(*ast.CallExpr); ok {
								if writeTupleSliceElemPtrFieldAssignmentFromTemp(out, lhs, fmt.Sprintf("__tmp_%d", i), call, i) {
									continue
								}
								if writeTupleSliceElemPtrVarAssignmentFromTemp(out, lhs, fmt.Sprintf("__tmp_%d", i), call, i) {
									continue
								}
								if writeTupleGoPtrAssignmentFromTemp(out, lhs, fmt.Sprintf("__tmp_%d", i), call, i) {
									continue
								}
								tmpBareScalar = callResultIsBareScalar(call, i)
							}
							writeTupleAssignmentFromTemp(out, lhs, fmt.Sprintf("__tmp_%d", i), tmpBareScalar)
						}
						out.WriteString(" }")
					}
				}
			} else if len(s.Lhs) > 1 && len(s.Rhs) > 1 {
				// Multiple assignments - need to handle specially
				// For now, just handle the simple case of parallel assignment
				if s.Tok == token.DEFINE {
					existingShortDeclLHS := make([]bool, len(s.Lhs))
					existingBareScalarType := make([]types.Type, len(s.Lhs))
					anyExistingShortDeclLHS := false
					for i, lhs := range s.Lhs {
						if !shortDeclLHSUsesExistingBinding(lhs) {
							continue
						}
						existingShortDeclLHS[i] = true
						anyExistingShortDeclLHS = true
						if typ, ok := shortDeclExistingBareScalarType(lhs); ok {
							existingBareScalarType[i] = typ
						}
					}
					out.WriteString("let (")
					for i, lhs := range s.Lhs {
						if i > 0 {
							out.WriteString(", ")
						}
						if existingShortDeclLHS[i] {
							out.WriteString(fmt.Sprintf("__tmp_%d", i))
							continue
						}
						// Don't add mut before blank identifier
						if ident, ok := lhs.(*ast.Ident); !ok || ident.Name != "_" {
							out.WriteString("mut ")
						}
						TranspileExpressionContext(out, lhs, LValue)
					}
					out.WriteString(") = (")
					for i, rhs := range s.Rhs {
						if i > 0 {
							out.WriteString(", ")
						}
						var value strings.Builder
						if existingShortDeclLHS[i] && existingBareScalarType[i] != nil {
							writeBareCompoundAssignValue(&value, rhs, existingBareScalarType[i])
						} else if existingShortDeclLHS[i] && shortDeclRhsIsMapElementHandle(rhs) {
							// The existing variable takes the element's map
							// handle, as m = s[0] does.
							writeParallelAssignmentTempValueForTarget(&value, s.Lhs[i], rhs)
						} else {
							writeMultiRhsShortDeclInitializer(&value, s.Lhs[i], rhs, !existingShortDeclLHS[i])
						}
						writeTupleElementDroppingGuards(out, value.String())
					}
					out.WriteString(")")
					if anyExistingShortDeclLHS {
						out.WriteString(";")
						for i, lhs := range s.Lhs {
							if !existingShortDeclLHS[i] {
								continue
							}
							if existingBareScalarType[i] == nil && shortDeclRhsIsMapElementHandle(s.Rhs[i]) {
								var line strings.Builder
								writeParallelAssignmentTarget(&line, lhs, fmt.Sprintf("__tmp_%d", i), s.Rhs[i])
								out.WriteString(" ")
								out.WriteString(strings.TrimSpace(line.String()))
								continue
							}
							writeTupleAssignmentFromTemp(out, lhs, fmt.Sprintf("__tmp_%d", i), existingBareScalarType[i] != nil)
						}
					}
				} else {
					// For reassignment, use temporaries to handle swaps correctly
					indent := currentLineIndent(out)
					out.WriteString("{\n")
					// Go evaluates the operands of indexed LHS targets before any
					// RHS expression. Capture them in source order before building
					// the RHS temporaries, then perform the writes left to right.
					preparedIndexTargets := make([]parallelIndexedAssignmentTarget, len(s.Lhs))
					preparedMapTargets := make([]parallelMapAssignmentTarget, len(s.Lhs))
					for i, lhs := range s.Lhs {
						if target, ok := prepareParallelMapAssignmentTarget(out, lhs, s.Rhs[i], i, indent+"    "); ok {
							preparedMapTargets[i] = target
						} else if target, ok := prepareParallelIndexedAssignmentTarget(out, lhs, i, indent+"    "); ok {
							preparedIndexTargets[i] = target
						}
					}
					// First, capture all RHS values into temporaries
					for i, rhs := range s.Rhs {
						out.WriteString(indent)
						out.WriteString("    ")
						out.WriteString(fmt.Sprintf("let __tmp_%d = ", i))
						if preparedMapTargets[i].indexExpr != nil {
							writeParallelMapAssignmentTempValue(out, preparedMapTargets[i].indexExpr, rhs)
						} else {
							writeParallelAssignmentTempValueForTarget(out, s.Lhs[i], rhs)
						}
						out.WriteString(";\n")
					}
					// Then assign all LHS from temporaries
					for i, lhs := range s.Lhs {
						if preparedMapTargets[i].indexExpr != nil {
							writeParallelMapAssignmentTargetLine(out, preparedMapTargets[i], fmt.Sprintf("__tmp_%d", i), indent+"    ")
						} else if preparedIndexTargets[i].indexExpr != nil {
							writeParallelAssignmentPreparedIndexTargetLine(out, preparedIndexTargets[i], fmt.Sprintf("__tmp_%d", i), s.Rhs[i], indent+"    ")
						} else {
							writeParallelAssignmentTargetLine(out, lhs, fmt.Sprintf("__tmp_%d", i), s.Rhs[i], indent+"    ")
						}
					}
					out.WriteString(indent)
					out.WriteString("}")
				}
			} else {
				// Single assignment
				// Check if we're assigning to just blank identifier
				if len(s.Lhs) == 1 {
					if ident, ok := s.Lhs[0].(*ast.Ident); ok && ident.Name == "_" {
						// Assignment to _ only - just evaluate the RHS for side effects
						out.WriteString("let _ = ")
						for i, rhs := range s.Rhs {
							if i > 0 {
								out.WriteString(", ")
							}
							if blankAssignmentOnlyNamesAVariable(rhs) {
								// "_ = err" uses the variable and reads
								// nothing from it. Reading its value here
								// would stop the program when it is nil.
								out.WriteString("&")
								TranspileExpressionContext(out, rhs, LValue)
								continue
							}
							TranspileExpression(out, rhs)
						}
					} else {
						// Normal single assignment
						if s.Tok == token.ASSIGN {
							// Assignment to wrapped variable
							if ident, ok := s.Lhs[0].(*ast.Ident); ok && isSliceElemPtrVar(ident.Name) {
								TranspileExpressionContext(out, ident, LValue)
								out.WriteString(" = ")
								if !writeSliceElemPtrOptionValue(out, s.Rhs[0]) {
									out.WriteString("/* ERROR: slice element pointer assignment requires nil or &slice[index] */ unimplemented!(\"slice element pointer assignment\")")
								}
							} else if ident, ok := s.Lhs[0].(*ast.Ident); ok && isArrayElemPtrVar(ident.Name) {
								TranspileExpressionContext(out, ident, LValue)
								out.WriteString(" = ")
								if !writeArrayElemPtrOptionValue(out, s.Rhs[0]) {
									out.WriteString("/* ERROR: array element pointer assignment requires nil or &array[index] */ unimplemented!(\"array element pointer assignment\")")
								}
							} else if writeErrorChannelReceiveAssignment(out, s.Lhs[0], s.Rhs[0]) {
							} else if star, ok := s.Lhs[0].(*ast.StarExpr); ok {
								// Check if LHS is a dereference (*p = value)
								// Assignment through pointer: *p = value
								if writeCurrentReceiverDerefAssignment(out, star, s.Rhs[0]) {
									// Pointer-receiver self is already &mut T, not a wrapped pointer handle.
								} else if ident, ok := star.X.(*ast.Ident); ok && isSliceElemPtrVar(ident.Name) {
									out.WriteString("{ ")
									out.WriteString("let new_val = ")
									if !writeSliceElemPtrDerefAssignmentValue(out, star, s.Rhs[0]) {
										TranspileExpression(out, s.Rhs[0])
									}
									out.WriteString("; *")
									out.WriteString(rustIdentForUseWithCapture(ident))
									out.WriteString(".as_ref().unwrap().borrow_mut() = Some(new_val); }")
								} else if ident, ok := star.X.(*ast.Ident); ok && isArrayElemPtrVar(ident.Name) {
									out.WriteString("{ ")
									out.WriteString("let new_val = ")
									var expected types.Type
									if typeInfo := GetTypeInfo(); typeInfo != nil {
										expected = typeInfo.GetType(star)
									}
									if !writePointerDerefAssignmentValue(out, s.Rhs[0], expected) {
										TranspileExpression(out, s.Rhs[0])
									}
									out.WriteString("; *")
									out.WriteString(rustIdentForUseWithCapture(ident))
									out.WriteString(".as_ref().unwrap().borrow_mut() = Some(new_val); }")
								} else if writeGoPtrSlotDerefAssignment(out, star, s.Rhs[0]) {
									// Pointer-to-GoPtr-slot assignment replaces the GoPtr stored in the slot.
								} else if ident, ok := star.X.(*ast.Ident); ok && isGoPtrVar(ident.Name) {
									writeGoPtrDerefAssignment(out, ident, star, s.Rhs[0])
								} else if writeGoPtrFieldDerefAssignment(out, star, s.Rhs[0]) {
									// Dereference assignment through a generated GoPtr field mutates the selected pointee.
								} else if writePointerDerefSequenceHandleAssignment(out, star, s.Rhs[0]) {
									// Pointer-to-slice assignment writes the RHS slice option into the pointee handle.
								} else if writePointerDerefMapHandleAssignment(out, star, s.Rhs[0]) {
									// Pointer-to-map assignment writes the RHS map into the map handle the pointer is.
								} else if writePointerDerefPointerHandleAssignment(out, star, s.Rhs[0]) {
									// Pointer-to-pointer assignment writes through the pointer handle stored in the slot.
								} else if writeFunctionPointerDerefCallAssignment(out, star, s.Rhs[0]) {
									// Pointer-to-function assignment writes the returned function option into the slot.
								} else if isUnsafePointerDerefAssignmentTarget(star) {
									out.WriteString("{ ")
									out.WriteString("unimplemented!(\"unsafe.Pointer dereference assignment\"); }")
								} else {
									out.WriteString("{ ")
									out.WriteString("let new_val = ")
									var expected types.Type
									if typeInfo := GetTypeInfo(); typeInfo != nil {
										expected = typeInfo.GetType(star)
									}
									if !writePointerDerefAssignmentValue(out, s.Rhs[0], expected) {
										TranspileExpression(out, s.Rhs[0])
									}
									out.WriteString("; ")
									if globalIdent, ok := packageGlobalPointerIdent(star.X); ok {
										// A package pointer variable is a cell holding
										// the pointer: write through the handle it holds.
										out.WriteString("*")
										writePackageGlobalPointerHandleClone(out, globalIdent)
									} else if _, isIdent := unwrapParens(star.X).(*ast.Ident); !isIdent {
										// The pointer is read first, in a let of its own,
										// so the locks its selector chain takes are let
										// go before the pointee is: `*team.Lead.Dept = v`
										// with Lead.Dept pointing at team's own Dept
										// otherwise locks that Dept twice.
										out.WriteString("let __deref_target = ")
										TranspileExpressionContext(out, star.X, LValue)
										out.WriteString(".clone(); *__deref_target")
									} else {
										out.WriteString("*")
										TranspileExpressionContext(out, star.X, LValue)
									}
									WriteBorrowMethod(out, true)
									if isGoErrorType(expected) {
										out.WriteString(" = new_val; }")
									} else {
										out.WriteString(" = Some(new_val); }")
									}
								}
							} else if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok && !isMapIndexAssign {
								// Array/slice element assignment: arr[i] = value
								if writePointerDerefSequenceElementAssignment(out, indexExpr, s.Rhs[0]) {
									// Pointer-to-slice/array element assignment mutates the pointee handle.
								} else if writeGoPtrPointedArrayElementAssignment(out, indexExpr, s.Rhs[0]) {
									// GoPtr pointer-to-array element assignment mutates through the GoPtr handle.
								} else if writeArrayElemPtrPointedArrayElementAssignment(out, indexExpr, s.Rhs[0]) {
									// Array element pointer to an array mutates the pointed-to array value.
								} else if writeNestedSliceElementAssignment(out, indexExpr, s.Rhs[0]) {
									// Nested slice element assignment emitted by helper.
								} else if call, ok := s.Rhs[0].(*ast.CallExpr); ok && appendCallReturnsBareIndexedSlice(call) && !goSliceModeEnabled() {
									// With GoSlice, append(x[i], v) is a new header that the
									// ordinary element assignment below stores.
									if appendTarget, ok := call.Args[0].(*ast.IndexExpr); ok && sameExpressionSyntax(indexExpr, appendTarget) {
										// "x[i] = append(x[i], v)" is written as the append
										// alone, which evaluates i once. Go evaluates the i
										// of the target as well, and first; that shows when
										// i calls a function.
										if expressionCallsOrReceives(indexExpr.Index) {
											out.WriteString("{ let _ = ")
											writeExpressionAsUsize(out, indexExpr.Index)
											out.WriteString("; ")
											TranspileExpression(out, call)
											out.WriteString("; }")
										} else {
											TranspileExpression(out, call)
										}
									} else {
										out.WriteString("(*")
										TranspileExpressionContext(out, indexExpr.X, LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(".as_mut().unwrap())[")
										writeExpressionAsUsize(out, indexExpr.Index)
										out.WriteString("] = ")
										TranspileExpression(out, call)
									}
								} else {
									// The index and the value are evaluated before the
									// sequence is locked for writing. Either may read the
									// sequence ("s[len(s)-1] = x", "s[0] = s[1]"), and a
									// read under the write lock panics, or hangs in a
									// program with goroutines.
									//
									// Go evaluates the sequence operand before the
									// index. That shows when the operand calls a
									// function ("rows()[i()] = v()"), so its handle is
									// then taken first and held, not locked. An operand
									// that calls something is never a package-level
									// variable, which has no handle to take.
									holdsSequence := expressionCallsOrReceives(indexExpr.X)
									writeSequenceHandle := func() {
										if subj := unwrapParens(indexExpr.X); isNamedSliceExpression(subj) {
											writeNamedSliceInnerHandleClone(out, subj)
										} else if subj := unwrapParens(indexExpr.X); isNamedArrayExpression(subj) {
											writeNamedArrayInnerHandleClone(out, subj)
										} else {
											TranspileExpressionContext(out, indexExpr.X, LValue)
										}
									}
									out.WriteString("{ ")
									if holdsSequence {
										out.WriteString("let __seq = ")
										writeSequenceHandle()
										out.WriteString(".clone(); ")
									}
									out.WriteString("let __idx = ")
									writeExpressionAsUsize(out, indexExpr.Index)
									out.WriteString("; let __rhs = ")

									var elemType types.Type
									if typeInfo := GetTypeInfo(); typeInfo != nil {
										elemType = indexedSequenceElementAssignmentType(typeInfo, indexExpr)
									}
									if elemRustType, ok := sliceElemPtrSliceCandidateForExpr(indexExpr.X); ok {
										if !writeSliceElemPtrSliceSlotValue(out, s.Rhs[0], elemRustType) {
											out.WriteString(`unimplemented!("type info required to lower pointer slice assignment")`)
										}
									} else if fieldInfo, ok := sliceElemPtrSliceFieldInfoForIndexExpr(indexExpr); ok {
										if !writeSliceElemPtrSliceSlotValue(out, s.Rhs[0], sliceElemPtrSliceFieldElemRustType(fieldInfo)) {
											out.WriteString(`unimplemented!("type info required to lower pointer slice field assignment")`)
										}
									} else if info, ok := goPtrArrayFieldInfoForIndexExpr(indexExpr); ok {
										if !writeGoPtrCallArgumentForInfo(out, s.Rhs[0], goPtrResultInfo{elemRustType: info.elemRustType, elemType: info.elemType}) {
											out.WriteString(`unimplemented!("type info required to lower GoPtr array field assignment")`)
										}
									} else {
										writeArraySliceElementAssignmentValue(out, s.Rhs[0], elemType)
									}

									out.WriteString("; (*")
									if holdsSequence {
										out.WriteString("__seq")
									} else {
										writeSequenceHandle()
									}
									WriteBorrowMethod(out, true)
									out.WriteString(".as_mut().unwrap())[__idx] = __rhs; }")
								}
							} else if sel, ok := s.Lhs[0].(*ast.SelectorExpr); ok && writeIndexedSequenceCollectionFieldAssignment(out, sel, s.Rhs[0]) {
								// Collection field on an indexed struct element mutates that element in place.
							} else {
								// Direct assignment: x = value
								noteMutexGuardAliasAssignment(s.Lhs[0], s.Rhs[0])
								// Check if RHS is nil
								if writePackageGlobalCollectionAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Package-global map/slice slots copy the current option value, preserving nil.
								} else if writeMapHandleAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Map assignment replaces the map handle, matching Go map-header semantics.
								} else if writePackageGlobalPointerNilAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Package-global pointer nil preserves the global slot and replaces the stored handle.
								} else if writeSliceElemPtrFieldAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// GoPtr fields preserve pointer identity for slice element addresses.
								} else if writeGoPtrAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// GoPtr locals preserve pointer identity across pointer assignment.
								} else if writePointerNilRebind(out, s.Lhs[0], s.Rhs[0]) {
									// A pointer variable or field takes a nil handle of its own.
								} else if ident, ok := s.Rhs[0].(*ast.Ident); ok && ident.Name == "nil" {
									// Assigning nil to pointer
									out.WriteString("*")
									TranspileExpressionContext(out, s.Lhs[0], LValue)
									WriteBorrowMethod(out, true)
									out.WriteString(" = None")
								} else if writeFuncValueAssignment(out, s) {
									// A func variable keeps its cell and takes a copy of the func value.
								} else if writePointerHandleAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Pointer assignment replaces the handle to preserve aliasing.
								} else if writeUnnamedSliceToNamedSliceAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// b = raw for a variable of a defined slice type.
								} else if writeSliceHandleAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Slice assignment replaces the handle to preserve slice-header aliasing.
								} else if writeCurrentReceiverLocalAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Reassigned Go value receivers are mutable Rust local copies, not wrapper slots.
								} else if writeCapturedCurrentReceiverAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Captured value receivers are bare closure-local copies, not wrapper slots.
								} else if writeLocalInterfaceHandleAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Local interface assignment copies the existing interface handle.
								} else if writeBareRangeVarAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Assigned range variables are local bare Rust bindings, not wrapper handles.
								} else if writeBareScalarAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Bare scalar locals stay bare when reassigned.
								} else if writeStdlibInterfaceAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Converted concrete stdlib values assigned through a stdlib interface handle.
								} else if isConcreteErrorInterfaceAssignment(s.Lhs[0], s.Rhs[0]) {
									writeConcreteErrorInterfaceAssignment(out, s.Lhs[0], s.Rhs[0])
								} else if writeConcreteToTranspiledInterfaceAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Concrete value reassigned into an interface-typed target boxes into the interface handle.
								} else if unary, ok := s.Rhs[0].(*ast.UnaryExpr); ok && unary.Op == token.AND {
									if _, isComposite := unary.X.(*ast.CompositeLit); isComposite {
										if writeEmptyInterfaceAssignment(out, s.Lhs[0], s.Rhs[0]) {
											// Addressed composite literals assigned to any must box the concrete value.
										} else {
											writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
										}
									} else {
										// Special case: p = &x where p is a pointer
										// We need to extract the value from x, not clone the whole Arc
										out.WriteString("{ ")
										out.WriteString("let new_val = (*")
										TranspileExpressionContext(out, unary.X, LValue)
										WriteBorrowMethod(out, false)
										out.WriteString(").clone(); ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = new_val; }")
									}
								} else if funcLit, ok := s.Rhs[0].(*ast.FuncLit); ok {
									_, isFuncLHS := expressionFunctionSignature(s.Lhs[0])
									if isFuncLHS || expressionHasFunctionSignatureSyntax(s.Lhs[0]) {
										out.WriteString("{ ")
										cloneFuncLitTarget := funcLitAssignmentTargetNeedsClone(s.Lhs[0], funcLit)
										if cloneFuncLitTarget {
											out.WriteString("let __func_lit_target = ")
											TranspileExpressionContext(out, s.Lhs[0], LValue)
											out.WriteString(".clone(); ")
										}
										out.WriteString("let new_val = ")
										TranspileFuncLitBox(out, funcLit)
										out.WriteString("; ")
										out.WriteString("*")
										if cloneFuncLitTarget {
											out.WriteString("__func_lit_target")
										} else {
											TranspileExpressionContext(out, s.Lhs[0], LValue)
										}
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									} else {
										out.WriteString("{ ")
										out.WriteString("let new_val = ")
										TranspileExpression(out, s.Rhs[0])
										out.WriteString("; ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									}
								} else if isErrorAssignment(s.Lhs[0], s.Rhs[0]) {
									writeErrorCopyAssignment(out, s.Lhs[0], s.Rhs[0])
								} else if writeFunctionAssertionHandleAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// "f = x.(func())" takes the handle the interface value holds.
								} else if writeFunctionSelectorHandleAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Function selector values are represented by handles; copy the handle.
								} else if rhsIdent, ok := s.Rhs[0].(*ast.Ident); ok {
									if sig, isFuncValue := functionValueSignature(rhsIdent); isFuncValue {
										out.WriteString("{ ")
										out.WriteString("let new_val = ")
										writeFunctionValueBox(out, rhsIdent, sig)
										out.WriteString("; ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									} else if isAssignmentSelfWrappingExpression(s.Rhs[0]) {
										writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
									} else {
										if writeEmptyInterfaceAssignment(out, s.Lhs[0], s.Rhs[0]) {
											// Assignment to interface{} boxes non-interface values and clones interface handles.
										} else if writeFunctionTypedIdentFieldAssignment(out, s.Lhs[0], rhsIdent) {
											// Function-typed values aren't Clone via .as_ref().clone(); share
											// the outer Arc handle instead.
										} else {
											// Check if RHS is a wrapped variable - use clone for non-Copy types
											rhsIsWrappedVar := false
											if rhsIdent.Name != "true" && rhsIdent.Name != "false" && rhsIdent.Name != "nil" {
												if _, isRange := rangeLoopVars[rhsIdent.Name]; !isRange {
													if _, isConst := localConstants[rhsIdent.Name]; !isConst {
														if !isConstIdent(rhsIdent) && !isVarBare(rhsIdent.Name) {
															rhsIsWrappedVar = true
														}
													}
												}
											}
											out.WriteString("{ ")
											out.WriteString("let new_val = ")
											if writeCurrentReceiverAssignmentValue(out, s.Rhs[0]) {
												// Method receivers are Rust self, not wrapped locals named after the Go receiver.
											} else if rhsIsWrappedVar {
												// Use clone to avoid moving non-Copy types like String
												rhsVarName := RustIdentForUse(rhsIdent)
												if currentCaptureRenames != nil {
													if renamed, exists := currentCaptureRenames[rhsIdent.Name]; exists {
														rhsVarName = RustLocalIdent(renamed)
													}
												}
												if writeTypeParamWrappedIdentValueClone(out, rhsIdent, rhsVarName) {
													// A value of a type parameter has its own lowering.
												} else if copyCall, known := goValueCopyCallForExpr(rhsIdent); known {
													out.WriteString(rhsVarName)
													WriteBorrowMethod(out, false)
													out.WriteString(".as_ref().unwrap()")
													out.WriteString(copyCall)
												} else {
													out.WriteString(goValueCopyNeedsTypeInfo)
												}
											} else if writeConstAssignmentValue(out, s.Lhs[0], s.Rhs[0]) {
												// Constants assigned to wrapped slots need the target type from go/types.
											} else if writeByteConstAssignmentValue(out, s.Lhs[0], s.Rhs[0]) {
												// Byte constants assigned to byte slots need the same go/types context as call arguments.
											} else if writeRangeIndexAssignmentValue(out, s.Lhs[0], s.Rhs[0]) {
												// Range indexes emit usize, but Go int assignment targets use i32.
											} else if writeRangeCharAssignmentValue(out, s.Lhs[0], s.Rhs[0]) {
												// String range runes are Rust char but Go rune targets are i32.
											} else if writeOwnedRangeValue(out, rhsIdent) {
												// Reference-style range values must be cloned into ordinary wrapped assignment targets.
											} else {
												TranspileExpression(out, s.Rhs[0])
											}
											out.WriteString("; ")
											out.WriteString("*")
											TranspileExpressionContext(out, s.Lhs[0], LValue)
											WriteBorrowMethod(out, true)
											out.WriteString(" = Some(new_val); }")
										}
									}
								} else if writeEmptyInterfaceAssignment(out, s.Lhs[0], s.Rhs[0]) {
									// Self-wrapping values such as map/slice literals still need boxing
									// when the assignment target is interface{}.
								} else if isAssignmentSelfWrappingExpression(s.Rhs[0]) {
									writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
								} else if call, ok := s.Rhs[0].(*ast.CallExpr); ok {
									// Check if it's an append call using TypeInfo
									isAppend := false
									isErrorFunc := false
									typeInfo := GetTypeInfo()
									if typeInfo != nil && typeInfo.info != nil {
										if ident, ok := call.Fun.(*ast.Ident); ok {
											// Check if this is the builtin append function
											if obj, ok := typeInfo.info.Uses[ident]; ok {
												if builtin, ok := obj.(*types.Builtin); ok {
													isAppend = builtin.Name() == "append"
												}
											}
										}
									}

									// Check if it's fmt.Errorf or errors.New which return error types
									if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
										if pkg, ok := sel.X.(*ast.Ident); ok {
											if (pkg.Name == "fmt" && sel.Sel.Name == "Errorf") ||
												(pkg.Name == "errors" && sel.Sel.Name == "New") {
												isErrorFunc = true
											}
										}
									}

									// Check if the LHS variable is an error type -
									// user functions returning error already return wrapped values
									if !isErrorFunc && typeInfo != nil {
										if lhsType := typeInfo.GetType(s.Lhs[0]); lhsType != nil {
											if named, ok := lhsType.(*types.Named); ok && named.Obj().Name() == "error" {
												isErrorFunc = true
											}
										}
									}

									if writeEmptyInterfaceAssignment(out, s.Lhs[0], s.Rhs[0]) {
										// Assignment to interface{} boxes non-interface values before error-call handling.
									} else if functionValueCallAssignmentUsesReturnedSlot(s.Lhs[0], call) {
										writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
									} else if isAppend {
										if isNamedSliceExpression(s.Rhs[0]) {
											writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
										} else {
											// append() returns the same wrapped type, don't wrap in Some()
											// Just execute the append for its side effect
											TranspileExpression(out, s.Rhs[0])
										}
									} else if isErrorFunc {
										// Error functions return the full wrapped type: the error handle.
										// Just replace the Rc pointer directly
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										out.WriteString(" = ")
										TranspileExpression(out, s.Rhs[0])
									} else if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
										writeMoveWrappedInnerAssignment(out, s.Lhs[0], s.Rhs[0])
									} else { // Regular function call
										isLenCapCall := isBareLenCapCall(s.Rhs[0])
										writeWrappedSlotAssignmentBlock(out, s.Lhs[0], wrappedSlotAssignmentShouldUseMultiline(s.Rhs[0]), func(valueOut *strings.Builder) {
											TranspileExpression(valueOut, s.Rhs[0])
											if isLenCapCall {
												valueOut.WriteString(" as " + rustIntType())
											}
										})
									}
								} else {
									if writeEmptyInterfaceAssignment(out, s.Lhs[0], s.Rhs[0]) {
										// Assignment to interface{} boxes non-interface values and clones interface handles.
									} else {
										rhsIsWrappedVar := wrappedSlotAssignmentRHSIsWrappedVar(s.Rhs[0])
										writeWrappedSlotAssignmentBlock(out, s.Lhs[0], wrappedSlotAssignmentShouldUseMultiline(s.Rhs[0]), func(valueOut *strings.Builder) {
											writeRegularWrappedSlotAssignmentValue(valueOut, s.Lhs[0], s.Rhs[0], rhsIsWrappedVar)
										})
									}
								}
							}
						} else {
							// Check if this is a channel variable definition
							isChannelVar := false
							if s.Tok == token.DEFINE && len(s.Lhs) == 1 && len(s.Rhs) == 1 {
								if lhsIdent, ok := s.Lhs[0].(*ast.Ident); ok {
									typeInfo := GetTypeInfo()
									if typeInfo != nil {
										// Check if LHS has channel type
										if typeInfo.IsChannel(s.Rhs[0]) {
											isChannelVar = true
											rustType := ""
											if callExpr, ok := s.Rhs[0].(*ast.CallExpr); ok && len(callExpr.Args) > 0 {
												rustType = GoTypeToRust(callExpr.Args[0])
											} else if typ := typeInfo.GetType(s.Rhs[0]); typ != nil {
												rustType = goTypesTypeToRust(typ)
											}
											// Register as bare variable
											if vt := GetVarTable(); vt != nil {
												vt.Register(lhsIdent.Name, &VarInfo{
													WrapLevel: WrapNone,
													RustType:  rustType,
													Source:    SourceLocal,
												})
											}
										}
									}
									if !isChannelVar {
										if callExpr, ok := s.Rhs[0].(*ast.CallExpr); ok && isMakeChannelCall(callExpr) {
											isChannelVar = true
											registerStdlibCallCollectionInfo(lhsIdent, callExpr)
											if vt := GetVarTable(); vt != nil {
												vt.Register(lhsIdent.Name, &VarInfo{
													WrapLevel: WrapNone,
													RustType:  GoTypeToRust(callExpr.Args[0]),
													Source:    SourceLocal,
												})
											}
										}
									}
								}
							}

							if isChannelVar {
								// Channel variables are bare - no wrapping
								out.WriteString("let mut ")
								TranspileExpressionContext(out, s.Lhs[0], LValue)
								out.WriteString(" = ")
								writeChannelAssignmentValue(out, s.Rhs[0])
							} else {
								// Check for slice element pointer short declaration
								// e.g. `alt := &slice[i]` -> Option<GoSliceElemPtr<T>>
								isSliceElemPtrShortDecl := false
								var sliceElemPtrRustType string
								isGoPtrShortDecl := false
								var goPtrShortDeclElemRustType string
								var goPtrShortDeclInfo goPtrResultInfo
								isArrayElemPtrShortDecl := false
								var arrayElemPtrShortDeclInfo arrayElemPtrInfo
								if s.Tok == token.DEFINE && len(s.Lhs) == 1 && len(s.Rhs) == 1 {
									if lhsIdent, ok := s.Lhs[0].(*ast.Ident); ok && lhsIdent.Name != "_" {
										if elemType, ok := sliceElemPtrCandidateForDecl(lhsIdent); ok {
											if rhsOk, sawSliceAddr := isSliceElemPtrAssignmentValue(s.Rhs[0]); rhsOk && sawSliceAddr {
												isSliceElemPtrShortDecl = true
												sliceElemPtrRustType = elemType
												registerSliceElemPtrVar(lhsIdent.Name, sliceElemPtrRustType)
											}
										}
										if !isSliceElemPtrShortDecl {
											if call, ok := s.Rhs[0].(*ast.CallExpr); ok {
												if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
													isSliceElemPtrShortDecl = true
													sliceElemPtrRustType = info.elemRustType
													registerSliceElemPtrVar(lhsIdent.Name, sliceElemPtrRustType)
												}
											}
										}
										if !isSliceElemPtrShortDecl && !isGoPtrShortDecl {
											if call, ok := s.Rhs[0].(*ast.CallExpr); ok {
												if info, ok := goPtrResultInfoForCall(call, 0); ok {
													isGoPtrShortDecl = true
													goPtrShortDeclInfo = info
													goPtrShortDeclElemRustType = goPtrResultElemRustType(info)
													var goType types.Type
													if typeInfo := GetTypeInfo(); typeInfo != nil {
														goType = typeInfo.GetType(s.Rhs[0])
													}
													registerGoPtrVar(lhsIdent.Name, goPtrShortDeclElemRustType, goType)
												}
											}
										}
										if !isSliceElemPtrShortDecl && !isGoPtrShortDecl {
											if rhsIdent, ok := unwrapParens(s.Rhs[0]).(*ast.Ident); ok && isGoPtrVar(rhsIdent.Name) {
												if elemRustType, ok := goPtrVarElemRustType(rhsIdent.Name); ok {
													isGoPtrShortDecl = true
													goPtrShortDeclInfo = goPtrInfoForLocalIdent(rhsIdent, elemRustType)
													goPtrShortDeclElemRustType = goPtrResultElemRustType(goPtrShortDeclInfo)
													var goType types.Type
													if typeInfo := GetTypeInfo(); typeInfo != nil {
														goType = typeInfo.GetType(s.Rhs[0])
													}
													registerGoPtrVar(lhsIdent.Name, goPtrShortDeclElemRustType, goType)
												}
											}
										}
										if !isSliceElemPtrShortDecl {
											if sel, ok := unwrapParens(s.Rhs[0]).(*ast.SelectorExpr); ok {
												if info, ok := sliceElemPtrFieldInfoForSelector(sel); ok {
													isGoPtrShortDecl = true
													goPtrShortDeclInfo = goPtrResultInfo{elemRustType: info.elemRustType, elemType: info.elemType}
													goPtrShortDeclElemRustType = goPtrResultElemRustType(goPtrShortDeclInfo)
													var goType types.Type
													if typeInfo := GetTypeInfo(); typeInfo != nil {
														goType = typeInfo.GetType(s.Rhs[0])
													}
													registerGoPtrVar(lhsIdent.Name, goPtrShortDeclElemRustType, goType)
												}
											}
										}
										if !isSliceElemPtrShortDecl && !isGoPtrShortDecl {
											if info, ok := goPtrSlotDerefResultInfo(s.Rhs[0]); ok {
												isGoPtrShortDecl = true
												goPtrShortDeclInfo = info
												goPtrShortDeclElemRustType = goPtrResultElemRustType(info)
												var goType types.Type
												if typeInfo := GetTypeInfo(); typeInfo != nil {
													goType = typeInfo.GetType(s.Rhs[0])
												}
												registerGoPtrVar(lhsIdent.Name, goPtrShortDeclElemRustType, goType)
											}
										}
										if !isSliceElemPtrShortDecl {
											if info, ok := arrayElemPtrCandidateForDecl(lhsIdent); ok {
												if _, rhsOk, sawArrayAddr := isArrayElemPtrAssignmentValue(s.Rhs[0]); rhsOk && sawArrayAddr {
													isArrayElemPtrShortDecl = true
													arrayElemPtrShortDeclInfo = info
													registerArrayElemPtrVar(lhsIdent.Name, arrayElemPtrShortDeclInfo)
												}
											}
										}
										if !isSliceElemPtrShortDecl && !isGoPtrShortDecl && !isArrayElemPtrShortDecl {
											if info, ok := goPtrCandidateForDecl(lhsIdent); ok {
												isGoPtrShortDecl = true
												goPtrShortDeclInfo = info
												goPtrShortDeclElemRustType = goPtrResultElemRustType(info)
												var goType types.Type
												if typeInfo := GetTypeInfo(); typeInfo != nil {
													goType = typeInfo.GetType(s.Rhs[0])
												}
												registerGoPtrVar(lhsIdent.Name, goPtrShortDeclElemRustType, goType)
											}
										}
									}
								}
								// Pre-register short-decl LHS idents so writeIdentExpression
								// emits the local name (with _local suffix if it shadows a
								// package global) instead of the package-global path.
								if s.Tok == token.DEFINE {
									for _, lhs := range s.Lhs {
										if ident, ok := lhs.(*ast.Ident); ok && ident.Name != "_" {
											if isPackageGlobalName(ident.Name) {
												if vt := GetVarTable(); vt != nil && vt.Lookup(ident.Name) == nil {
													vt.Register(ident.Name, &VarInfo{
														WrapLevel: WrapFull,
														Source:    SourceLocal,
													})
												}
											}
										}
									}
								}
								if s.Tok == token.DEFINE && len(s.Lhs) == 1 && len(s.Rhs) == 1 &&
									(writeStatementBuiltLogicalShortDecl(out, s.Lhs[0], s.Rhs[0]) ||
										writeStatementBuiltConcurrentBinaryShortDecl(out, s.Lhs[0], s.Rhs[0])) {
									// The helper emitted the complete declaration.
								} else {
									// Regular assignment or definition
									for i, lhs := range s.Lhs {
										if i > 0 {
											out.WriteString(", ")
										}
										if s.Tok == token.DEFINE {
											out.WriteString("let mut ")
										}
										TranspileExpressionContext(out, lhs, LValue)
										if isSliceElemPtrShortDecl && i == 0 {
											out.WriteString(": Option<GoSliceElemPtr<")
											out.WriteString(sliceElemPtrRustType)
											out.WriteString(">>")
										} else if isGoPtrShortDecl && i == 0 {
											out.WriteString(": GoPtr<")
											out.WriteString(goPtrShortDeclElemRustType)
											out.WriteString(">")
										} else if isArrayElemPtrShortDecl && i == 0 {
											out.WriteString(": ")
											out.WriteString(arrayElemPtrOptionRustType(arrayElemPtrShortDeclInfo))
										} else if s.Tok == token.DEFINE && len(s.Lhs) == 1 && len(s.Rhs) == 1 {
											if ident, ok := lhs.(*ast.Ident); ok && ident.Name != "_" {
												if elemRustType, ok := sliceElemPtrSliceCandidateForDecl(ident); ok {
													out.WriteString(": ")
													out.WriteString(sliceElemPtrSliceRustType(elemRustType))
												} else if rustType, ok := arrayElemAddressPointerRustType(s.Rhs[0]); ok {
													out.WriteString(": ")
													out.WriteString(rustType)
												} else if rustType, ok := localMakeSliceTypeAnnotation(s.Rhs[0]); ok {
													out.WriteString(": ")
													out.WriteString(rustType)
												}
											}
										}
									}

									out.WriteString(" = ")

									for i, rhs := range s.Rhs {
										if i > 0 {
											out.WriteString(", ")
										}
										if s.Tok == token.DEFINE {
											if i < len(s.Lhs) {
												if lhsIdent, ok := s.Lhs[i].(*ast.Ident); ok {
													if elemRustType, ok := sliceElemPtrSliceCandidateForDecl(lhsIdent); ok && writeSliceElemPtrSliceLiteral(out, rhs, elemRustType) {
														continue
													}
													if writeSliceElemPtrMapInitializer(out, lhsIdent, rhs) {
														continue
													}
												}
											}
											// Check if RHS is nil
											if isGoPtrShortDecl && i == 0 {
												wroteGoPtrInitializer := false
												withShortDeclOuterRhsBindings(s.Lhs, rhs, func() {
													wroteGoPtrInitializer = writeGoPtrCallArgumentForInfo(out, rhs, goPtrShortDeclInfo)
												})
												if !wroteGoPtrInitializer {
													out.WriteString(`unimplemented!("GoPtr initializer requires compatible pointer value")`)
												}
											} else if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
												WriteWrappedNone(out)
											} else if unary, ok := rhs.(*ast.UnaryExpr); ok && unary.Op == token.AND {
												if isSliceElemPtrShortDecl && i == 0 {
													out.WriteString("Some(")
													TranspileExpression(out, rhs)
													out.WriteString(")")
												} else if isArrayElemPtrShortDecl && i == 0 {
													if !writeArrayElemPtrOptionValue(out, rhs) {
														out.WriteString("/* ERROR: array element pointer initializer requires nil or &array[index] */ unimplemented!(\"array element pointer initializer\")")
													}
												} else {
													// Taking address - don't wrap, the & operator will handle it
													if isBareSyncValueAddress(unary.X) {
														registerBareShortDecl(s.Lhs[i])
													}
													TranspileExpression(out, rhs)
												}
											} else if unary, ok := rhs.(*ast.UnaryExpr); ok && unary.Op == token.ARROW && channelElementIsHandleOption(unary.X) {
												writeErrorHandleFromChannelReceive(out, unary.X)
											} else if callExpr, isCall := rhs.(*ast.CallExpr); isCall {
												if i < len(s.Lhs) {
													registerCallResultSyntaxInfo(s.Lhs[i], callExpr)
												}
												if i < len(s.Lhs) {
													if lhsIdent, ok := s.Lhs[i].(*ast.Ident); ok {
														if elemRustType, ok := sliceElemPtrSliceCandidateForDecl(lhsIdent); ok && isBuiltinCallNamed(callExpr, "make") {
															writeSliceElemPtrSliceMake(out, callExpr, elemRustType)
															continue
														}
													}
												}
												if len(s.Lhs) == 1 && writeBareBuiltinShortDeclInitializer(out, callExpr, s.Lhs[0]) {
													continue
												}
												if len(s.Lhs) == 1 && callReturnsBareScalar(callExpr) {
													registerBareShortDecl(s.Lhs[0])
													TranspileExpression(out, callExpr)
													continue
												}
												// len()/cap() return bare primitives — register LHS as bare
												if callIdent, ok := callExpr.Fun.(*ast.Ident); ok {
													if callIdent.Name == "len" || callIdent.Name == "cap" {
														typeInfo := GetTypeInfo()
														if typeInfo != nil && typeInfo.info != nil {
															if obj, ok := typeInfo.info.Uses[callIdent]; ok {
																if builtin, ok := obj.(*types.Builtin); ok && (builtin.Name() == "len" || builtin.Name() == "cap") {
																	if len(s.Lhs) == 1 {
																		if lhsIdent, ok := s.Lhs[0].(*ast.Ident); ok {
																			if vt := GetVarTable(); vt != nil {
																				vt.Register(lhsIdent.Name, &VarInfo{
																					WrapLevel: WrapNone,
																					Source:    SourceLocal,
																				})
																			}
																		}
																	}
																}
															}
														}
													}
												}
												if len(s.Lhs) == 1 {
													if lhsIdent, ok := s.Lhs[0].(*ast.Ident); ok {
														if target, ok := pointerTypeConversionTarget(callExpr.Fun); ok && isFunctionSignatureTypeExpr(target) {
															if vt := GetVarTable(); vt != nil {
																vt.Register(lhsIdent.Name, &VarInfo{
																	WrapLevel: WrapFull,
																	RustType:  "function_signature_pointer",
																	Source:    SourceLocal,
																})
															}
														} else if rustType, ok := functionTypeRustNameFromTypeExpr(callSingleReturnTypeExpr(callExpr)); ok {
															if vt := GetVarTable(); vt != nil {
																vt.Register(lhsIdent.Name, &VarInfo{
																	WrapLevel: WrapFull,
																	RustType:  rustType,
																	Source:    SourceLocal,
																})
															}
														}
													}
												}
												// Function calls already return wrapped values, don't wrap again
												withShortDeclOuterRhsBindings(s.Lhs, rhs, func() {
													writeCallExpressionForInitializer(out, callExpr)
												})
											} else if funcLit, isFuncLit := rhs.(*ast.FuncLit); isFuncLit {
												if i < len(s.Lhs) {
													if lhsIdent, ok := s.Lhs[i].(*ast.Ident); ok {
														if vt := GetVarTable(); vt != nil {
															vt.Register(lhsIdent.Name, &VarInfo{
																WrapLevel: WrapFull,
																RustType:  generateFuncLitClosureType(funcLit),
																Source:    SourceLocal,
															})
														}
													}
												}
												// Function literals are already wrapped by TranspileFuncLit
												TranspileExpression(out, rhs)
											} else if compositeLit, isCompositeLit := rhs.(*ast.CompositeLit); isCompositeLit {
												if i < len(s.Lhs) {
													registerCompositeLiteralRangeElemType(s.Lhs[i], compositeLit)
													registerCompositeLiteralSyntaxVarInfo(s.Lhs[i], compositeLit)
												}
												needsLocalWrapper, hasTypeInfo := compositeLiteralNeedsLocalValueWrapper(compositeLit)
												emitsBareSpecialValue := compositeLiteralEmitsBareStructValue(compositeLit)
												if emitsBareSpecialValue {
													registerBareShortDecl(s.Lhs[0])
												}

												if !hasTypeInfo {
													out.WriteString(`unimplemented!("type info required for composite literal initializer")`)
												} else if needsLocalWrapper && !emitsBareSpecialValue {
													// Named composites and unnamed structs need an outer local value wrapper.
													WriteWrapperPrefix(out)
													TranspileExpression(out, rhs)
													WriteWrapperSuffix(out)
												} else {
													// Array/slice/map literals already wrap themselves
													TranspileExpression(out, rhs)
												}
											} else if _, isSliceExpr := rhs.(*ast.SliceExpr); isSliceExpr {
												// Plain slice expressions already return wrapped values; named-slice
												// slice expressions emit a bare named value that the local wrapper stores.
												if !writeNamedSliceSliceExprShortDeclInitializer(out, rhs) {
													TranspileExpression(out, rhs)
												}
											} else if writeTypeParamHandleInitializer(out, rhs) {
												// Type-parameter values are represented by handles; clone the handle into the new local.
											} else if isWrappedInterfaceSliceIndex(rhs) {
												// IndexExpr on a slice of wrapped local-interface
												// elements returns the element's handle; the
												// variable gets a copy of the value in a handle of
												// its own.
												writeInterfaceHandleCopy(out, func() { TranspileExpression(out, rhs) })
											} else if mapIndexExpressionKeepsHandle(rhs) && isTranspiledInterfaceValue(rhs) {
												// An interface value read out of a map is copied
												// into the variable's own handle.
												writeInterfaceHandleCopy(out, func() { TranspileExpression(out, rhs) })
											} else if mapIndexExpressionKeepsHandle(rhs) {
												// Map values that are maps/slices/pointers/etc. already return cloneable handles.
												// An element of a []map can be a nil map (#318).
												if index, ok := unwrapParens(rhs).(*ast.IndexExpr); ok && mapElementIndexExpression(index) && i < len(s.Lhs) {
													if lhsIdent, ok := s.Lhs[i].(*ast.Ident); ok {
														localZeroMapVars[lhsIdent.Name] = true
													}
												}
												TranspileExpression(out, rhs)
											} else if typeInfo := GetTypeInfo(); typeInfo != nil && isFunctionSignatureType(typeInfo.GetType(rhs)) && writeFunctionValueHandle(out, rhs) {
												// Function values are already represented by cloneable handles.
											} else if writeConcurrentMapSelectorHandleClone(out, rhs) {
												// Concurrent map fields are map handles; clone the handle.
											} else if writeSliceSelectorHandleClone(out, rhs) {
												// Slice fields are already wrapped handles; clone the handle.
											} else if writeTranspiledInterfaceHandleClone(out, rhs) {
												// Existing named-interface values are already represented by a handle.
											} else if writeEmptyInterfaceHandleClone(out, rhs) {
												// Existing interface values are already represented by a handle.
											} else if writeAnyHandleAssertionHandle(out, rhs) {
												// A slice or map taken out of an interface value shares its data with it.
											} else if typeInfo := GetTypeInfo(); typeInfo != nil && isGoErrorType(typeInfo.GetType(rhs)) && writeGoErrorHandleValue(out, rhs) {
												// Existing error values are already represented by a handle.
											} else if writeStdlibInterfaceFieldValueCopy(out, rhs) {
												// Copied by value from an existing stdlib interface field.
											} else if writeStringConstWrappedInitializer(out, s.Lhs[i], rhs) {
												// Untyped string constants default to owned Go strings in local variables.
											} else if writeWrappedOwnedExpressionValue(out, rhs) {
												// Copied by value from an existing wrapped field or handle.
											} else if writeWrappedGoValueCopyOfElement(out, rhs) {
												// An element of a slice, array or map of structs is copied into the new variable.
											} else if ident, ok := rhs.(*ast.Ident); ok {
												if sig, isFuncValue := functionValueSignature(ident); isFuncValue {
													writeWrappedFunctionValueBox(out, ident, sig)
												} else if writeWrappedStdlibInterfaceRangeValueCopy(out, ident) {
													// Range values over stdlib-interface slices are references; clone into the new wrapper.
												} else if writeWrappedReferenceRangeValueCopy(out, ident) {
													// Reference-style range values need an owned clone for wrapped short declarations.
												} else if writeWrappedBareStringRangeValueCopy(out, ident) {
													// Bare string range values need cloning before storing in a new wrapper.
												} else if writePackageGlobalMapWrappedValueCopy(out, ident) {
													// Package-global maps are stored as global values; clone the current map into the local wrapper.
												} else if writePackageGlobalSliceWrappedValueCopy(out, ident) {
													// Package-global slices are stored as global values; clone the current slice into the local wrapper.
												} else if writeWrappedValueCopyFromIdent(out, ident) {
													// Copied by value from an existing wrapped value
												} else if writeWrappedCollectionCopyFromIdent(out, ident, nil) {
													// A copy of a local slice or map variable.
												} else if writeChannelCopyFromIdent(out, ident, nil) {
													// A channel variable: share the channel.
												} else if writeNamedIntegerWrappedInitializer(out, rhs) {
													// Typed named integer const identifiers need the named newtype, not the raw const.
												} else if rhsIsPointerType(rhs) {
													// RHS is a pointer-typed variable (e.g., z := y where y is *int)
													// Clone the Rc to preserve aliasing instead of copying the inner value
													writePointerShortDeclRhsValue(out, rhs)
												} else {
													// Wrap new variables
													WriteWrapperPrefix(out)
													TranspileExpression(out, rhs)
													WriteWrapperSuffix(out)
												}
											} else if rhsIsPointerType(rhs) {
												// RHS is a pointer-typed variable (e.g., z := y where y is *int)
												// Clone the Rc to preserve aliasing instead of copying the inner value
												writePointerShortDeclRhsValue(out, rhs)
											} else if writeStringConstWrappedInitializer(out, s.Lhs[i], rhs) {
												// Untyped string constants default to owned Go strings in local variables.
											} else if writeNamedIntegerWrappedInitializer(out, rhs) {
												// Named integer arithmetic returns the underlying scalar; short declarations store the named value.
											} else {
												// Wrap new variables
												WriteWrapperPrefix(out)
												TranspileExpression(out, rhs)
												WriteWrapperSuffix(out)
											}
										} else {
											TranspileExpression(out, rhs)
										}
									}
								}
							} // end else (non-channel var)
						}
					}
				} else {
					// Multiple LHS
					for i, lhs := range s.Lhs {
						if i > 0 {
							out.WriteString(", ")
						}
						if s.Tok == token.DEFINE {
							out.WriteString("let mut ")
						}
						TranspileExpression(out, lhs)
					}

					out.WriteString(" = ")

					for i, rhs := range s.Rhs {
						if i > 0 {
							out.WriteString(", ")
						}
						if s.Tok == token.DEFINE {
							// Check if RHS is an expression that already returns wrapped values
							if call, isCall := rhs.(*ast.CallExpr); isCall {
								if i < len(s.Lhs) && writeBareBuiltinShortDeclInitializer(out, call, s.Lhs[i]) {
									continue
								}
								// Function calls already return wrapped values, don't wrap again
								writeCallExpressionForInitializer(out, call)
							} else if _, isSlice := rhs.(*ast.SliceExpr); isSlice {
								// Plain slice expressions already return wrapped values; named-slice
								// slice expressions emit a bare named value that the local wrapper stores.
								if !writeNamedSliceSliceExprShortDeclInitializer(out, rhs) {
									TranspileExpression(out, rhs)
								}
							} else if writeTypeParamHandleInitializer(out, rhs) {
								// Type-parameter values are represented by handles; clone the handle into the new local.
							} else if mapIndexExpressionKeepsHandle(rhs) {
								// Map values that are maps/slices/pointers/etc. already return cloneable handles.
								TranspileExpression(out, rhs)
							} else if writeNamedIntegerWrappedInitializer(out, rhs) {
								// Named integer arithmetic returns the underlying scalar; short declarations store the named value.
							} else {
								// Wrap new variables in Arc<Mutex<Option<>>>
								WriteWrapperPrefix(out)
								TranspileExpression(out, rhs)
								WriteWrapperSuffix(out)
							}
						} else {
							TranspileExpression(out, rhs)
						}
					}
				}
			}
		}
		out.WriteString(";")
	case *ast.DeclStmt:
		if genDecl, ok := s.Decl.(*ast.GenDecl); ok {
			switch genDecl.Tok {
			case token.VAR:
			specLoop:
				for _, spec := range genDecl.Specs {
					if valueSpec, ok := spec.(*ast.ValueSpec); ok {
						// Multi-name, single function call: var q, r = divmod(a, b)
						// Generate: let (mut q, mut r) = divmod(a, b);
						if len(valueSpec.Names) > 1 && len(valueSpec.Values) == 1 {
							if callExpr, isCall := valueSpec.Values[0].(*ast.CallExpr); isCall {
								// Record bare-scalar tuple slot info so later uses
								// of the LHS names don't synthesize .borrow()/.lock().
								lhsExprs := make([]ast.Expr, len(valueSpec.Names))
								for i, name := range valueSpec.Names {
									lhsExprs[i] = name
								}
								registerCallTupleResultSyntaxInfo(lhsExprs, callExpr)
								out.WriteString("let (")
								for i, name := range valueSpec.Names {
									if i > 0 {
										out.WriteString(", ")
									}
									if name.Name != "_" {
										out.WriteString("mut ")
									}
									out.WriteString(RustLocalIdent(name.Name))
								}
								out.WriteString(") = ")
								TranspileExpression(out, valueSpec.Values[0])
								out.WriteString(";")
								continue specLoop
							}
						}
						for i, name := range valueSpec.Names {
							registerTypeExprCollectionInfo(name.Name, valueSpec.Type)
							registerZeroValueLocalMap(name, valueSpec.Type, i < len(valueSpec.Values))
							if rustType, ok := functionTypeRustNameFromTypeExpr(valueSpec.Type); ok {
								if vt := GetVarTable(); vt != nil {
									vt.Register(name.Name, &VarInfo{
										WrapLevel: WrapFull,
										RustType:  rustType,
										Source:    SourceLocal,
									})
								}
							}
							// Check if this is a sync type - bare, not wrapped
							isSyncType := false
							if sel, ok := valueSpec.Type.(*ast.SelectorExpr); ok {
								if pkgIdent, ok := sel.X.(*ast.Ident); ok && pkgIdent.Name == "sync" {
									if isBareSyncTypeName(sel.Sel.Name) {
										isSyncType = true
										rustType := ""
										switch sel.Sel.Name {
										case "WaitGroup":
											NeedWaitGroup()
											rustType = "WaitGroup"
										case "Mutex":
											NeedGoMutex()
											rustType = "GoMutex"
										case "RWMutex":
											NeedGoRWMutex()
											rustType = "GoRWMutex"
										case "Once":
											NeedGoOnce()
											rustType = "GoOnce"
										}
										if vt := GetVarTable(); vt != nil {
											vt.Register(name.Name, &VarInfo{
												WrapLevel: WrapNone,
												RustType:  rustType,
												Source:    SourceLocal,
											})
										}
									}
								}
							}

							if name.Name == "_" {
								out.WriteString("let ")
							} else {
								out.WriteString("let mut ")
							}
							out.WriteString(RustLocalIdent(name.Name))
							sliceElemPtrRustType, isSliceElemPtr := sliceElemPtrCandidateForDecl(name)
							arrayElemPtrInfo, isArrayElemPtr := arrayElemPtrCandidateForDecl(name)
							goPtrInfo, isGoPtr := goPtrCandidateForDecl(name)
							goPtrArrayLocalInfo, isGoPtrArrayLocal := goPtrArrayLocalInfoForDecl(name)
							if isSliceElemPtr {
								registerSliceElemPtrVar(name.Name, sliceElemPtrRustType)
							} else if valueSpec.Type == nil && len(valueSpec.Values) > i {
								if call, ok := valueSpec.Values[i].(*ast.CallExpr); ok {
									if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
										isSliceElemPtr = true
										sliceElemPtrRustType = info.elemRustType
										registerSliceElemPtrVar(name.Name, sliceElemPtrRustType)
									}
								}
							}
							if isArrayElemPtr {
								registerArrayElemPtrVar(name.Name, arrayElemPtrInfo)
							}
							if isGoPtr && !isSliceElemPtr && !isArrayElemPtr {
								registerGoPtrVar(name.Name, goPtrResultElemRustType(goPtrInfo), nil)
							}
							if isGoPtrArrayLocal {
								if vt := GetVarTable(); vt != nil {
									vt.Register(name.Name, &VarInfo{
										WrapLevel: WrapFull,
										RustType:  goPtrArrayFieldRustType(goPtrArrayLocalInfo),
										Source:    SourceLocal,
									})
								}
							}

							// Add type annotation if type is specified (skip for sync types and local interfaces)
							isLocalInterface := false
							if typeIdent, ok := valueSpec.Type.(*ast.Ident); ok && localInterfaces[typeIdent.Name] {
								isLocalInterface = true
							}
							if valueSpec.Type != nil && name.Name != "_" && !isSyncType && !isSliceElemPtr && !isArrayElemPtr && !isGoPtr && !isGoPtrArrayLocal && !isLocalInterface {
								if _, isFunctionType := functionTypeRustNameFromTypeExpr(valueSpec.Type); !isFunctionType {
									if vt := GetVarTable(); vt != nil {
										vt.Register(name.Name, &VarInfo{
											WrapLevel: WrapFull,
											RustType:  GoTypeToRust(valueSpec.Type),
											Source:    SourceLocal,
										})
									}
								}
							}
							if valueSpec.Type != nil && !isSyncType && !isLocalInterface {
								out.WriteString(": ")
								if isSliceElemPtr {
									out.WriteString("Option<GoSliceElemPtr<")
									out.WriteString(sliceElemPtrRustType)
									out.WriteString(">>")
								} else if isArrayElemPtr {
									out.WriteString(arrayElemPtrOptionRustType(arrayElemPtrInfo))
								} else if isGoPtr {
									out.WriteString("GoPtr<")
									out.WriteString(goPtrResultElemRustType(goPtrInfo))
									out.WriteString(">")
								} else if isGoPtrArrayLocal {
									out.WriteString(goPtrArrayFieldRustType(goPtrArrayLocalInfo))
								} else if elemRustType, ok := sliceElemPtrSliceCandidateForDecl(name); ok {
									out.WriteString(sliceElemPtrSliceRustType(elemRustType))
								} else {
									out.WriteString(GoTypeToRust(valueSpec.Type))
								}
							} else if valueSpec.Type == nil && !isSliceElemPtr && !isArrayElemPtr && !isGoPtr && len(valueSpec.Values) > i {
								if rustType, ok := localMakeSliceTypeAnnotation(valueSpec.Values[i]); ok {
									out.WriteString(": ")
									out.WriteString(rustType)
								}
							}

							if len(valueSpec.Values) > i {
								out.WriteString(" = ")
								// Check if value is nil
								if isSliceElemPtr {
									if !writeSliceElemPtrOptionValue(out, valueSpec.Values[i]) {
										out.WriteString("/* ERROR: slice element pointer initializer requires nil or &slice[index] */ unimplemented!(\"slice element pointer initializer\")")
									}
								} else if isArrayElemPtr {
									if !writeArrayElemPtrOptionValue(out, valueSpec.Values[i]) {
										out.WriteString("/* ERROR: array element pointer initializer requires nil or &array[index] */ unimplemented!(\"array element pointer initializer\")")
									}
								} else if isGoPtr {
									if !writeGoPtrCallArgumentForInfo(out, valueSpec.Values[i], goPtrInfo) {
										out.WriteString(`unimplemented!("GoPtr initializer requires compatible pointer value")`)
									}
								} else if ident, ok := valueSpec.Values[i].(*ast.Ident); ok && ident.Name == "nil" {
									// Initializing with nil
									WriteWrappedNone(out)
								} else if valueSpec.Type != nil && writeUnnamedSliceToNamedSliceInitializer(out, valueSpec.Values[i], expectedTypeFromParamExpr(valueSpec.Type)) {
									// var b Bytes = []byte("abc"): the slice goes into the defined type.
								} else if valueSpec.Type != nil && writeConcreteErrorVarInitializer(out, valueSpec.Type, valueSpec.Values[i]) {
									// A struct or pointer that implements error, stored in an error variable.
								} else if valueSpec.Type != nil && writeTranspiledInterfaceInitializerValue(out, valueSpec.Values[i], expectedTypeFromParamExpr(valueSpec.Type)) {
									// Values assigned into a transpiled named interface variable may need re-boxing.
								} else if valueSpec.Type != nil && writeConcreteToTranspiledInterfaceValue(out, valueSpec.Values[i], expectedTypeFromParamExpr(valueSpec.Type)) {
									// Concrete value boxed into a transpiled named interface variable.
								} else if valueSpec.Type != nil && writeStdlibInterfaceCallArgumentConversion(out, valueSpec.Values[i], expectedTypeFromParamExpr(valueSpec.Type)) {
									// Converted concrete stdlib values assigned to a stdlib interface variable.
								} else if valueSpec.Type != nil && isEmptyInterfaceTypeExpr(valueSpec.Type) {
									writeEmptyInterfaceCallArgumentValue(out, valueSpec.Values[i])
								} else if isLocalInterface {
									// Assigning to a local interface variable - keep wrapped, just clone the Rc
									if ident, ok := valueSpec.Values[i].(*ast.Ident); ok {
										out.WriteString(ident.Name + ".clone()")
									} else {
										TranspileExpression(out, valueSpec.Values[i])
									}
								} else if call, isCall := valueSpec.Values[i].(*ast.CallExpr); isCall {
									registerCallResultSyntaxInfo(name, call)
									if elemRustType, ok := sliceElemPtrSliceCandidateForDecl(name); ok && isBuiltinCallNamed(call, "make") {
										writeSliceElemPtrSliceMake(out, call, elemRustType)
									} else if valueSpec.Type == nil && callReturnsBareScalar(call) {
										registerBareShortDecl(name)
										TranspileExpression(out, call)
									} else if writeBareBuiltinShortDeclInitializer(out, call, name) {
										// len/cap/min/max var initializers use normal Go value wrappers.
									} else {
										// Function calls already return wrapped values, don't wrap again
										writeCallExpressionForInitializer(out, call)
									}
								} else if funcLit, isFuncLit := valueSpec.Values[i].(*ast.FuncLit); isFuncLit {
									if valueSpec.Type == nil {
										if vt := GetVarTable(); vt != nil {
											vt.Register(name.Name, &VarInfo{
												WrapLevel: WrapFull,
												RustType:  generateFuncLitClosureType(funcLit),
												Source:    SourceLocal,
											})
										}
										TranspileExpression(out, valueSpec.Values[i])
									} else if _, directFuncType := valueSpec.Type.(*ast.FuncType); directFuncType {
										TranspileFuncLitBox(out, funcLit)
									} else if usesWrappedHandle, hasTypeInfo := functionTypeExprUsesWrappedHandle(valueSpec.Type); usesWrappedHandle {
										WriteWrapperPrefix(out)
										TranspileFuncLitBox(out, funcLit)
										WriteWrapperSuffix(out)
									} else if !hasTypeInfo {
										out.WriteString(`unimplemented!("type info required to lower function literal initializer")`)
									} else {
										TranspileExpression(out, valueSpec.Values[i])
									}
								} else if compositeLit, isCompositeLit := valueSpec.Values[i].(*ast.CompositeLit); isCompositeLit {
									registerCompositeLiteralSyntaxVarInfo(name, compositeLit)
									if valueSpec.Type == nil && compositeLiteralEmitsBareStructValue(compositeLit) {
										// "var mu = sync.Mutex{}" is the bare value, as "mu := sync.Mutex{}" is.
										registerBareShortDecl(name)
									}
									needsLocalWrapper, hasTypeInfo := compositeLiteralNeedsLocalValueWrapper(compositeLit)
									emitsBareSpecialValue := valueSpec.Type == nil && compositeLiteralEmitsBareStructValue(compositeLit)

									if !hasTypeInfo {
										out.WriteString(`unimplemented!("type info required for composite literal initializer")`)
									} else if needsLocalWrapper && !emitsBareSpecialValue {
										// Named composites and unnamed structs need an outer local value wrapper.
										WriteWrapperPrefix(out)
										TranspileExpression(out, valueSpec.Values[i])
										WriteWrapperSuffix(out)
									} else {
										// Array/slice/map literals already wrap themselves
										TranspileExpression(out, valueSpec.Values[i])
									}
								} else if unary, ok := valueSpec.Values[i].(*ast.UnaryExpr); ok && unary.Op == token.AND {
									// Address-of operator already produces wrapped value
									if isBareSyncValueAddress(unary.X) {
										registerBareShortDecl(name)
									}
									TranspileExpression(out, valueSpec.Values[i])
								} else if writeDefinedMapFromElementHandle(out, valueSpec.Type, valueSpec.Values[i]) {
									// var c Counts = s[0]: the defined map shares the element's map.
								} else if mapIndexExpressionKeepsHandle(valueSpec.Values[i]) && !varDeclConvertsToOtherType(valueSpec.Type, valueSpec.Values[i]) {
									// Map values that are maps/slices/pointers/etc. already return cloneable handles.
									TranspileExpression(out, valueSpec.Values[i])
								} else if writeConcurrentMapSelectorHandleClone(out, valueSpec.Values[i]) {
									// Concurrent map fields are map handles; clone the handle.
								} else if writeSliceSelectorHandleClone(out, valueSpec.Values[i]) {
									// Slice fields are already wrapped handles; clone the handle.
								} else if writeNamedIntegerWrappedInitializer(out, valueSpec.Values[i]) {
									// Inferred named-integer vars store the named value, not the emitted primitive const.
								} else if valueSpec.Type != nil && writeWrappedConstVarInitializer(out, name, valueSpec.Values[i]) {
									// Typed const initializers are cast through the LHS type from go/types.
								} else if valueSpec.Type != nil && typedPointerVarTakesPointerHandle(valueSpec.Type, valueSpec.Values[i]) {
									// var p *T = q: p holds q's handle, as p := q does.
									writePointerHandleValueClone(out, valueSpec.Values[i])
								} else if ident, ok := valueSpec.Values[i].(*ast.Ident); ok {
									isInterface := false
									if valueSpec.Type != nil {
										isInterface = isEmptyInterfaceTypeExpr(valueSpec.Type)
									}
									if isInterface {
										// For interface{}, box the value
										WriteWrapperPrefix(out)
										writeInterfaceBoxedValue(out, valueSpec.Values[i])
										WriteWrapperSuffix(out)
									} else if sig, isFuncValue := functionValueSignature(ident); isFuncValue {
										writeWrappedFunctionValueBox(out, ident, sig)
									} else if writeWrappedStdlibInterfaceRangeValueCopy(out, ident) {
										// Range values over stdlib-interface slices are references; clone into the new wrapper.
									} else if writeWrappedBareStringRangeValueCopy(out, ident) {
										// Bare string range values need cloning before storing in a new wrapper.
									} else if writePackageGlobalMapWrappedValueCopy(out, ident) {
										// Package-global maps are stored as global values; clone the current map into the local wrapper.
									} else if writePackageGlobalSliceWrappedValueCopy(out, ident) {
										// Package-global slices are stored as global values; clone the current slice into the local wrapper.
									} else if writeWrappedValueCopyFromIdent(out, ident) {
										// Copied by value from an existing wrapped value
									} else if writeWrappedCollectionCopyFromIdent(out, ident, declaredVarType(valueSpec, i)) {
										// A copy of a local slice or map variable.
									} else if writeChannelCopyFromIdent(out, ident, declaredVarType(valueSpec, i)) {
										// A channel variable: share the channel.
									} else {
										if valueSpec.Type != nil {
											if typeIdent, ok := valueSpec.Type.(*ast.Ident); ok {
												if writeNamedScalarVarSpecValue(out, valueSpec, i) {
													// A float, string or bool of a type the package defines.
												} else if underlyingType, isTypeDef := LookupTypeDefinition(typeIdent.Name); isTypeDef {
													WriteWrapperPrefix(out)
													out.WriteString(typeIdent.Name)
													out.WriteString("(")
													WriteWrapperPrefix(out)
													isFloatType := underlyingType == "float64" || underlyingType == "float32"
													if isFloatType {
														if lit, ok := valueSpec.Values[i].(*ast.BasicLit); ok && lit.Kind == token.INT {
															out.WriteString(RustFloatFromIntegerLiteral(lit.Value))
														} else {
															TranspileExpression(out, valueSpec.Values[i])
														}
													} else if localCollectionIdentNeedsClone(ident) {
														writeIdentValueClone(out, ident)
													} else {
														TranspileExpression(out, valueSpec.Values[i])
													}
													WriteWrapperSuffix(out)
													out.WriteString(")")
													WriteWrapperSuffix(out)
												} else {
													WriteWrapperPrefix(out)
													TranspileExpression(out, valueSpec.Values[i])
													WriteWrapperSuffix(out)
												}
											} else {
												WriteWrapperPrefix(out)
												TranspileExpression(out, valueSpec.Values[i])
												WriteWrapperSuffix(out)
											}
										} else {
											WriteWrapperPrefix(out)
											TranspileExpression(out, valueSpec.Values[i])
											WriteWrapperSuffix(out)
										}
									}
								} else {
									// Check if the target type is interface{}
									isInterface := false
									if valueSpec.Type != nil {
										isInterface = isEmptyInterfaceTypeExpr(valueSpec.Type)
									}

									if isInterface {
										// For interface{}, box the value
										WriteWrapperPrefix(out)
										writeInterfaceBoxedValue(out, valueSpec.Values[i])
										WriteWrapperSuffix(out)
									} else if varSpecHasTypeOfValue(valueSpec, i) && writeAnyHandleAssertionHandle(out, valueSpec.Values[i]) {
										// A slice or map taken out of an interface value shares its data with it.
									} else if valueSpec.Type != nil {
										if typeIdent, ok := valueSpec.Type.(*ast.Ident); ok {
											if writeNamedScalarVarSpecValue(out, valueSpec, i) {
												// A float, string or bool of a type the package defines.
											} else if underlyingType, isTypeDef := LookupTypeDefinition(typeIdent.Name); isTypeDef {
												WriteWrapperPrefix(out)
												out.WriteString(typeIdent.Name)
												out.WriteString("(")
												WriteWrapperPrefix(out)
												// Check if int literal needs float conversion
												isFloatType := underlyingType == "float64" || underlyingType == "float32"
												if isFloatType {
													if lit, ok := valueSpec.Values[i].(*ast.BasicLit); ok && lit.Kind == token.INT {
														out.WriteString(RustFloatFromIntegerLiteral(lit.Value))
													} else {
														TranspileExpression(out, valueSpec.Values[i])
													}
												} else {
													TranspileExpression(out, valueSpec.Values[i])
												}
												WriteWrapperSuffix(out)
												out.WriteString(")")
												WriteWrapperSuffix(out)
											} else {
												WriteWrapperPrefix(out)
												TranspileExpression(out, valueSpec.Values[i])
												WriteWrapperSuffix(out)
											}
										} else {
											WriteWrapperPrefix(out)
											TranspileExpression(out, valueSpec.Values[i])
											WriteWrapperSuffix(out)
										}
									} else {
										WriteWrapperPrefix(out)
										TranspileExpression(out, valueSpec.Values[i])
										WriteWrapperSuffix(out)
									}
								}
							} else {
								// Default initialization for uninitialized vars
								if isSliceElemPtr {
									out.WriteString(" = None")
								} else if isArrayElemPtr {
									out.WriteString(" = None")
								} else if isGoPtr {
									out.WriteString(" = GoPtr::nil()")
								} else if isGoPtrArrayLocal {
									out.WriteString(" = ")
									writeGoPtrArrayFieldDefaultValue(out, goPtrArrayLocalInfo)
								} else if valueSpec.Type != nil && writeNilZeroValueInitializerFromTypeInfo(out, valueSpec.Type) {
									// nil zero value supplied from go/types
								} else if valueSpec.Type != nil && !isSyncType && writeWrappedZeroValueInitializerFromTypeInfo(out, valueSpec.Type) {
									// non-nil zero value supplied from go/types
								} else if valueSpec.Type != nil {
									switch t := valueSpec.Type.(type) {
									case *ast.Ident:
										switch t.Name {
										case "string":
											out.WriteString(" = ")
											WriteWrapperPrefix(out)
											out.WriteString("String::new()")
											WriteWrapperSuffix(out)
										case "int":
											out.WriteString(" = ")
											WriteWrapperPrefix(out)
											out.WriteString("0")
											WriteWrapperSuffix(out)
										default:
											// Named function types have nil zero value in Go
											if IsTypeAlias(t.Name) {
												typeInfo := GetTypeInfo()
												if IsFunctionTypeAlias(t.Name) || (typeInfo != nil && typeInfo.IsFunctionType(t)) {
													out.WriteString(" = ")
													WriteWrappedNone(out)
												} else {
													out.WriteString(" = ")
													WriteWrapperPrefix(out)
													out.WriteString("Default::default()")
													WriteWrapperSuffix(out)
												}
											} else {
												out.WriteString(" = ")
												WriteWrapperPrefix(out)
												out.WriteString("Default::default()")
												WriteWrapperSuffix(out)
											}
										}
									case *ast.StarExpr:
										// Pointer type - initialize with None
										out.WriteString(" = ")
										WriteWrappedNone(out)
									case *ast.InterfaceType:
										// interface{} - initialize with None
										if len(t.Methods.List) == 0 {
											out.WriteString(" = ")
											WriteWrappedNone(out)
										}
									case *ast.StructType:
										// Anonymous struct type - initialize with default
										out.WriteString(" = ")
										WriteWrapperPrefix(out)
										out.WriteString("Default::default()")
										WriteWrapperSuffix(out)
									case *ast.ArrayType:
										out.WriteString(" = ")
										if t.Len == nil {
											// Slices default to nil in Go; preserve that distinction from []T{}.
											WriteWrappedNone(out)
										} else {
											WriteWrapperPrefix(out)
											out.WriteString(zeroValueForGoType(t))
											WriteWrapperSuffix(out)
										}
									case *ast.MapType:
										// A map's zero value is nil, unlike an empty map literal.
										out.WriteString(" = ")
										WriteWrappedNone(out)
									case *ast.FuncType:
										// Function variables have a nil zero value in Go.
										out.WriteString(" = ")
										WriteWrappedNone(out)
									case *ast.SelectorExpr:
										// Package-qualified types like sync.WaitGroup, strings.Builder
										if pkgIdent, ok := t.X.(*ast.Ident); ok {
											if pkgIdent.Name == "sync" {
												switch t.Sel.Name {
												case "WaitGroup":
													NeedWaitGroup()
													out.WriteString(" = WaitGroup::new()")
												case "Mutex":
													NeedGoMutex()
													out.WriteString(" = GoMutex::new()")
												case "RWMutex":
													NeedGoRWMutex()
													out.WriteString(" = GoRWMutex::new()")
												case "Once":
													NeedGoOnce()
													out.WriteString(" = GoOnce::new()")
												}
											} else if pkgIdent.Name == "strings" && t.Sel.Name == "Builder" {
												out.WriteString(" = ")
												WriteWrapperPrefix(out)
												out.WriteString("String::new()")
												WriteWrapperSuffix(out)
											} else if pkgIdent.Name == "bytes" && t.Sel.Name == "Buffer" {
												out.WriteString(" = ")
												WriteWrapperPrefix(out)
												out.WriteString("Default::default()")
												WriteWrapperSuffix(out)
											}
										}
									}
								}
							}
							out.WriteString(";")
						}
					}
				}
			case token.CONST:
				// Handle local const declarations - keep original case
				transpileConstDeclWithCase(out, genDecl, false)
			case token.TYPE:
				// Handle local type declarations
				for _, spec := range genDecl.Specs {
					if typeSpec, ok := spec.(*ast.TypeSpec); ok {
						// Skip local interface type declarations - they can't be Rust type aliases
						if _, isIface := typeSpec.Type.(*ast.InterfaceType); isIface {
							localInterfaces[typeSpec.Name.Name] = true
							continue
						}
						if structType, isStruct := typeSpec.Type.(*ast.StructType); isStruct {
							structDef := &StructDef{
								Fields:        make(map[string]string),
								FieldTypes:    make(map[string]ast.Expr),
								EmbeddedTypes: []string{},
								ASTType:       structType,
							}
							for _, field := range structType.Fields.List {
								if len(field.Names) > 0 {
									for _, name := range field.Names {
										structDef.Fields[name.Name] = "regular"
										structDef.FieldTypes[name.Name] = field.Type
									}
								} else {
									typeName := getEmbeddedFieldName(field.Type)
									structDef.EmbeddedTypes = append(structDef.EmbeddedTypes, typeName)
								}
							}
							structDefs[typeSpec.Name.Name] = structDef
							RegisterTypeAlias(typeSpec.Name.Name)
							RegisterBareStructAlias(typeSpec.Name.Name)
							structRustType := goTypeToRustBase(typeSpec.Type)
							// The struct is written as an anonymous struct;
							// when the local type is compared, so is it.
							if comparableStructTypes[typeSpec.Name.Name] {
								comparableStructTypes[structRustType] = true
							}
							out.WriteString("type ")
							out.WriteString(RustTypeNameForUse(typeSpec.Name.Name))
							out.WriteString(" = ")
							out.WriteString(structRustType)
							out.WriteString(";")
							continue
						}
						if localNamedCollectionTypeNeedsNewtype(typeSpec) {
							// The rest of the translation treats a defined
							// array, slice or map type as its newtype, as at
							// package level, so it is declared the same way.
							TranspileTypeDecl(out, typeSpec, genDecl)
							continue
						}
						// For now, just generate type aliases inside functions
						// These should be hoisted to module level in a real implementation
						RegisterTypeAlias(typeSpec.Name.Name)
						if _, isFuncType := typeSpec.Type.(*ast.FuncType); isFuncType {
							RegisterFunctionTypeAlias(typeSpec.Name.Name)
						}
						out.WriteString("type ")
						out.WriteString(RustTypeNameForUse(typeSpec.Name.Name))
						out.WriteString(" = ")
						out.WriteString(GoTypeToRust(typeSpec.Type))
						out.WriteString(";")
					}
				}
			}
		}

	case *ast.ForStmt:
		registerLoopVariables(s)
		shortNames := shortDeclNames(s.Init)
		wrapForScope := shortDeclShadowsRangeVar(shortNames)
		var popForInitScope func()
		if s.Init != nil {
			if vt := GetVarTable(); vt != nil {
				vt.PushScope()
				popForInitScope = vt.PopScope
				registerFullShortDecls(shortNames)
			}
		}
		restoreRangeLoopVars := func() {}
		restoreInitCaptureShadow := func() {}
		if wrapForScope {
			out.WriteString("{\n    ")
		}
		if s.Init != nil {
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			restoreInitCaptureShadow = suppressCaptureRenamesDeclaredBy(s.Init)
			restoreRangeLoopVars = shadowRangeLoopVars(shortNames)
			out.WriteString("\n    ")
		}

		// Emit loop label if set by LabeledStmt
		var currentLoopLabel string
		if pendingLoopLabel != "" {
			currentLoopLabel = pendingLoopLabel
			out.WriteString("'" + pendingLoopLabel + ": ")
			// Track post-statement for labeled continue
			if s.Post != nil {
				labeledLoopPost[pendingLoopLabel] = s.Post
			}
			pendingLoopLabel = ""
		}
		// A body lowered with goto blocks names the loop in its break and
		// continue statements.
		gotoBody := stmtListHasPlannedGoto(s.Body.List)
		bodyLoopLabel := currentLoopLabel
		if gotoBody && bodyLoopLabel == "" {
			for {
				gotoLoopLabelCount++
				bodyLoopLabel = fmt.Sprintf("__go_loop_%d", gotoLoopLabelCount)
				if !gotoSyntheticNames[bodyLoopLabel] {
					break
				}
			}
			out.WriteString("'" + bodyLoopLabel + ": ")
		}
		if s.Cond != nil {
			out.WriteString("while ")
			transpileCondition(out, s.Cond)
		} else {
			out.WriteString("loop")
		}
		out.WriteString(" {\n")
		popForPost := pushForPost(s.Post)
		breakTarget := ""
		if gotoBody {
			breakTarget = bodyLoopLabel
		}
		restoreLoopBreakTarget := pushBreakTarget(breakTarget)

		currentLoopDepth++
		var prevStmt ast.Stmt
		var forBodyLastPos token.Pos = s.Body.Lbrace
		var captureShadowRestores []func()
		if gotoBody {
			func() {
				defer pushContinueTarget(bodyLoopLabel)()
				prevStmt = TranspileGotoStatementList(out, s.Body.List, fnType, fileSet, comments, &forBodyLastPos, "        ")
			}()
		} else {
			for i, stmt := range s.Body.List {
				// Add blank line if there was one in the source
				if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
					out.WriteString("\n")
				}

				out.WriteString("        ")
				TranspileStatement(out, stmt, fnType, fileSet, comments, &forBodyLastPos, "        ")
				writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.Body.List)-1 || s.Post != nil)
				out.WriteString("\n")
				trackCaptureShadows(&captureShadowRestores, stmt)

				prevStmt = stmt
			}
		}
		restoreCaptureShadows(captureShadowRestores)

		// Add the post statement (increment) if present
		if s.Post != nil {
			if gotoBody {
				// The goto lowering leaves its last statement open; a
				// break or continue there would take the post statement's
				// block as its operand.
				if body := strings.TrimRight(out.String(), " \t\n"); !strings.HasSuffix(body, ";") && !strings.HasSuffix(body, "}") && !strings.HasSuffix(body, "{") {
					out.Reset()
					out.WriteString(body)
					out.WriteString(";\n")
				}
			}
			out.WriteString("        ")
			TranspileStatementSimple(out, s.Post, fnType, fileSet)
			out.WriteString("\n")
		}
		currentLoopDepth--
		restoreLoopBreakTarget()
		popForPost()

		out.WriteString("    }")

		// Clean up label tracking
		if currentLoopLabel != "" {
			delete(labeledLoopPost, currentLoopLabel)
		}
		if wrapForScope {
			out.WriteString("\n    }")
		}
		restoreRangeLoopVars()
		if popForInitScope != nil {
			popForInitScope()
		}
		restoreInitCaptureShadow()

	case *ast.BlockStmt:
		out.WriteString("{\n")
		var prevStmt ast.Stmt
		var blockLastPos token.Pos = s.Lbrace
		var captureShadowRestores []func()
		for i, stmt := range s.List {
			// Add blank line if there was one in the source
			if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
				out.WriteString("\n")
			}

			out.WriteString(indent)
			out.WriteString("    ")
			// Pass comments through for nested blocks
			TranspileStatement(out, stmt, fnType, fileSet, comments, &blockLastPos, indent+"    ")
			writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.List)-1)
			out.WriteString("\n")
			trackCaptureShadows(&captureShadowRestores, stmt)

			prevStmt = stmt
		}
		restoreCaptureShadows(captureShadowRestores)
		out.WriteString(indent)
		out.WriteString("}")

	case *ast.EmptyStmt:
		return

	case *ast.IncDecStmt:
		if indexExpr, isMapIndex := isMapIndexExpression(s.X); isMapIndex {
			writeMapElementUpdate(out, indexExpr, s.Tok, nil)
		} else if indexExpr, ok := s.X.(*ast.IndexExpr); ok && writeIndexedIncDec(out, indexExpr, s.Tok) {
			// array/slice element inc/dec mutates the underlying sequence directly.
		} else if writeIntegerTypeParamIncDec(out, s.X, s.Tok) {
			// Generic integer increments use the typed helper instead of a raw Rust integer literal.
		} else if writeNamedIntegerIncDec(out, s.X, s.Tok) {
			// Named integer arithmetic returns the underlying scalar; preserve the named wrapper on mutation.
		} else if writeBareScalarIncDec(out, s.X, s.Tok) {
			// Bare scalar locals mutate directly.
		} else {
			// For wrapped variables, we need to update the value inside
			step := "1"
			if typeInfo := GetTypeInfo(); typeInfo != nil {
				step = incDecStepForType(typeInfo.GetType(s.X))
			}
			out.WriteString("{ ")
			writeWrappedMutationTargetPrelude(out, s.X)
			out.WriteString("let mut guard = ")
			writeWrappedMutationTargetRef(out, s.X, true)
			out.WriteString("; ")
			out.WriteString("*guard = Some(guard.as_ref().unwrap() ")
			if s.Tok == token.INC {
				out.WriteString("+ ")
			} else {
				out.WriteString("- ")
			}
			out.WriteString(step)
			out.WriteString("); }")
		}

	case *ast.RangeStmt:
		// A range over a function is told by the Go type of s.X, before
		// any name-keyed collection kind: an inner seq of func type can
		// shadow an outer seq slice.
		if writeRangeOverFuncStatement(out, s, "", fnType, fileSet, comments, lastPos, indent) {
			return
		}
		if restore, needsTypeInfo := rewriteChannelRangeAssignment(s); needsTypeInfo {
			out.WriteString(`unimplemented!("type info required for a range that assigns to an existing variable")`)
			return
		} else if restore != nil {
			defer restore()
		}
		registerLoopVariables(s)
		// Track range loop variables so we don't try to unwrap them
		var keyName, valueName string
		keyAssigned := false
		valueAssigned := false
		// valueMutated is also true when the body calls a pointer method
		// on the value, which needs the binding mutable.
		valueMutated := false
		rangeAssignsExisting := s.Tok == token.ASSIGN

		// Use type information to determine what we're iterating over
		isMap := false
		isString := false
		isInteger := false
		isSlice := false
		isArray := false
		isChannel := false

		isMap, isString, isInteger, isSlice, isArray, isChannel = rangeTypeFacts(s.X)
		if !isMap && !isString && !isInteger && !isSlice && !isArray && !isChannel {
			if lit, ok := s.X.(*ast.BasicLit); ok && lit.Kind == token.STRING {
				isString = true
			} else if kind, ok := localCollectionKind(s.X); ok {
				switch kind {
				case "map":
					isMap = true
				case "slice":
					isSlice = true
				case "channel":
					isChannel = true
				}
			} else if ident, ok := s.X.(*ast.Ident); ok && lookupVarInfo(ident.Name) != nil && strings.TrimPrefix(lookupVarInfo(ident.Name).RustType, "&") == "String" {
				isString = true
			} else if _, ok := rangeVarSliceElemRustType(s.X); ok {
				isSlice = true
			} else if _, ok := trackedRangeElemRustType(s.X); ok {
				isSlice = true
			}
		}
		if !isMap && !isString && !isInteger && !isSlice && !isArray && !isChannel {
			out.WriteString("/* ERROR: Cannot determine range type - type information required */\n")
			out.WriteString("unimplemented!(\"type info required for range statement\");")
			return
		}
		popForPost := pushForPost(nil)

		rangeValuesVar := ""
		rangePrelude := ""
		// Set when the loop binds its value variable to a reference to the
		// element rather than to an owned value.
		rangeValueBoundByReference := false
		stringRangeValueTemp := ""
		closeRangeGuard := false
		needsSliceValues := !(s.Key != nil && s.Value == nil)
		moveEmptyInterfaceRangeValues := rangeEmptyInterfaceValueMovedByIndexedWriteback(s)
		if needsSliceValues && !isMap && !isString && isSlice && isNamedSliceExpression(s.X) {
			out.WriteString("{ let __range_holder = ")
			writeNamedSliceInnerHandleClone(out, s.X)
			out.WriteString("; let __range_guard = __range_holder")
			WriteBorrowMethod(out, false)
			writeRangeValuesMaterialization(out, s.X, moveEmptyInterfaceRangeValues)
			rangeValuesVar = "__range_values"
			closeRangeGuard = true
		} else if needsSliceValues && !isMap && !isString && isArray && isNamedArrayExpression(s.X) {
			out.WriteString("{ let __range_holder = ")
			writeNamedArrayInnerHandleClone(out, s.X)
			out.WriteString("; let __range_guard = __range_holder")
			WriteBorrowMethod(out, false)
			writeRangeValuesMaterialization(out, s.X, moveEmptyInterfaceRangeValues)
			rangeValuesVar = "__range_values"
			closeRangeGuard = true
		} else if needsSliceValues && !isMap && !isString && (isSlice || isArray) {
			if ident, ok := unwrapParens(s.X).(*ast.Ident); ok && arrayElemPtrIdentPointsToArray(ident) {
				out.WriteString("{ let __range_values = ")
				writeArrayElemPtrPointedArrayClone(out, ident)
				out.WriteString("; ")
				rangeValuesVar = "__range_values"
				closeRangeGuard = true
			} else if rangeTargetNeedsWrappedSliceGuard(s.X) {
				out.WriteString("{ let __range_holder = ")
				writeWrappedHandleExpression(out, s.X)
				out.WriteString(".clone(); let __range_guard = __range_holder")
				WriteBorrowMethod(out, false)
				writeRangeValuesMaterialization(out, s.X, moveEmptyInterfaceRangeValues)
				rangeValuesVar = "__range_values"
				closeRangeGuard = true
			}
		}

		// Emit loop label if set by LabeledStmt
		if pendingLoopLabel != "" {
			out.WriteString("'" + pendingLoopLabel + ": ")
			pendingLoopLabel = ""
		}
		// Handle for range loops
		out.WriteString("for ")

		// Channel range: for val := range ch
		if isChannel {
			// A value received from a channel of an interface gets a handle
			// of its own, like any other interface variable.
			isInterfaceChannel := channelElementFillsNewHandle(s.X)
			// Register value variable as range loop var
			if s.Key != nil {
				if ident, ok := s.Key.(*ast.Ident); ok && ident.Name != "_" {
					valueName = ident.Name
					if !isInterfaceChannel {
						rangeLoopVars[valueName] = "channel_val"
					}
				}
			}
			if valueName != "" && isInterfaceChannel {
				out.WriteString("__iface_value")
			} else if valueName != "" {
				out.WriteString(RustLocalIdent(valueName))
			} else {
				out.WriteString("_")
			}
			out.WriteString(" in ")
			// The iterator lives for the whole loop. A channel read from a
			// field comes out of a block that has already ended the lock on
			// the struct, so the body can lock it again.
			var rangeChannel strings.Builder
			writeChannelExpression(&rangeChannel, s.X)
			out.WriteString(rangeChannel.String())
			if !strings.HasPrefix(rangeChannel.String(), "{ let __channel = ") {
				out.WriteString(".clone()")
			}
			out.WriteString(" {\n")
			if valueName != "" && isInterfaceChannel {
				out.WriteString("        let mut ")
				out.WriteString(RustLocalIdent(valueName))
				out.WriteString(" = ")
				writeErrorHandleFromOptionValue(out, "__iface_value")
				out.WriteString(";\n")
			}
			restoreLoopBreakTarget := pushBreakTarget("")

			currentLoopDepth++
			var rangeBodyLastPos token.Pos = s.Body.Lbrace
			var captureShadowRestores []func()
			for i, stmt := range s.Body.List {
				out.WriteString("        ")
				TranspileStatement(out, stmt, fnType, fileSet, comments, &rangeBodyLastPos, "        ")
				writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.Body.List)-1)
				out.WriteString("\n")
				trackCaptureShadows(&captureShadowRestores, stmt)
			}
			restoreCaptureShadows(captureShadowRestores)
			currentLoopDepth--

			out.WriteString("    }")
			restoreLoopBreakTarget()

			if valueName != "" {
				delete(rangeLoopVars, valueName)
			}
			break
		}

		// Determine types based on what we're iterating over
		keyType := "usize" // Default for slice indices
		valueType := "T"   // Generic placeholder
		keyRangeVarType := keyType
		var mapKeyType types.Type
		var mapValueGoType types.Type
		mapKeyNeedsValueBinding := false
		mapKeyNeedsWrappedBinding := false

		if isMap {
			keyType = "String"
			valueType = GetOuterWrapperType() + "<" + GetInnerWrapperType() + "<Option<T>>>"
			keyRangeVarType = keyType
			if localKeyType, localValueType, ok := localMapRangeTypes(s.X); ok {
				keyType = localKeyType
				keyRangeVarType = localKeyType
				valueType = localValueType
			}
			if key, mapValueType := rangeMapKeyValueTypes(s.X); mapValueType != nil {
				mapKeyType = key
				mapValueGoType = mapValueType
				if mapKeyType != nil {
					keyType = goTypesMapKeyToRust(mapKeyType)
					keyRangeVarType = keyType
					if _, ok := types.Unalias(mapKeyType).Underlying().(*types.Pointer); ok {
						keyRangeVarType = goTypesTypeToRust(mapKeyType)
						mapKeyNeedsValueBinding = true
					} else if _, ok := transpiledNamedInterfaceTypeNameFromTypes(mapKeyType); ok {
						keyRangeVarType = goTypesTypeToRustWrapped(mapKeyType)
						mapKeyNeedsValueBinding = true
					} else if isEmptyInterfaceType(mapKeyType) {
						keyRangeVarType = goTypesTypeToRustWrapped(mapKeyType)
						mapKeyNeedsValueBinding = true
					} else if isStdlibNamedInterfaceValueType(mapKeyType) {
						keyRangeVarType = goTypesTypeToRustWrapped(mapKeyType)
						mapKeyNeedsWrappedBinding = true
					}
				}
				valueType = goTypesMapValueToRust(mapValueType)
			}
			if mapKeyType != nil {
				keyType = goTypesMapKeyToRust(mapKeyType)
			}
		} else if isInteger {
			if rangeType := rangeExprGoType(s.X); rangeType != nil {
				keyType = goTypesTypeToRust(rangeType)
			} else {
				keyType = rustIntType()
			}
		} else if isString {
			valueType = "char"
		} else if isSlice || isArray {
			// Check if it's a slice of interface{} or named interface
			elemType := rangeArrayOrSliceElemType(s.X)
			if elemType != nil {
				if isGoErrorType(elemType) {
					valueType = goTypesTypeToRustWrapped(elemType)
				} else if _, ok := types.Unalias(elemType).(*types.TypeParam); ok {
					valueType = "&" + goTypesTypeToRustWrapped(elemType)
				} else if isStdlibNamedInterfaceValueType(elemType) {
					valueType = "&" + goTypesTypeToRust(elemType)
				} else if _, ok := elemType.Underlying().(*types.Pointer); ok {
					valueType = "&" + goTypesTypeToRust(elemType)
				} else if _, ok := elemType.Underlying().(*types.Slice); ok {
					valueType = "&" + goTypesTypeToRust(elemType)
				} else if _, ok := elemType.Underlying().(*types.Array); ok {
					if rangeElementUsesCopied(elemType) {
						valueType = goTypesTypeToRust(elemType)
					} else {
						valueType = "&" + goTypesTypeToRust(elemType)
					}
				} else if _, ok := elemType.Underlying().(*types.Map); ok {
					// An element of an unnamed map type is the map's handle
					// (#318).
					valueType = "&" + goTypesCollectionElemTypeToRust(elemType)
				} else if _, ok := elemType.Underlying().(*types.Struct); ok {
					valueType = "&" + goTypesTypeToRust(elemType)
				} else if intf, ok := elemType.Underlying().(*types.Interface); ok {
					if intf.NumMethods() == 0 {
						// It's []interface{} - elements are Box<dyn Any>
						// When iterating with &, we get &Box<dyn Any>
						valueType = "&" + rustAnyTraitObject()
					} else if _, ok := transpiledNamedInterfaceTypeNameFromTypes(elemType); ok {
						// Local named interface slice elements are wrapped to
						// preserve Go's nullable interface semantics, so the
						// range value is &Rc<RefCell<Option<Box<dyn Trait>>>>.
						valueType = "&" + goTypesTypeToRustWrapped(elemType)
					} else {
						// It's a slice of named interface - elements are Box<dyn InterfaceName>
						// We need to get the interface name
						if namedType, ok := elemType.(*types.Named); ok {
							valueType = "&" + rustLocalInterfaceTraitObject(namedType.Obj().Name())
						} else {
							// Generic named interface
							valueType = "&Box<dyn Trait>"
						}
					}
				} else if elemRustType, ok := trackedRangeElemRustType(s.X); ok {
					valueType = rangeValueTypeFromTrackedRustElem(elemRustType)
				}
			}
			if elemType == nil {
				if elemRustType, ok := trackedRangeElemRustType(s.X); ok {
					valueType = rangeValueTypeFromTrackedRustElem(elemRustType)
				} else if elemRustType, ok := rangeVarSliceElemRustType(s.X); ok {
					valueType = rangeValueTypeFromTrackedRustElem(elemRustType)
				}
			}
		}
		if !isMap {
			keyRangeVarType = keyType
		}
		keyBindingTemp := ""
		valueBindingTemp := ""
		if rangeAssignsExisting {
			keyBindingTemp = rangeAssignmentTempName(s.Key, "__range_key")
			valueBindingTemp = rangeAssignmentTempName(s.Value, "__range_value")
		}
		var keyBindingGoType types.Type
		var valueBindingGoType types.Type
		if keyBindingTemp != "" {
			switch {
			case isMap:
				keyBindingGoType = mapKeyType
			case isString || isSlice || isArray:
				keyBindingGoType = types.Typ[types.Int]
			case isInteger:
				keyBindingGoType = rangeExprGoType(s.X)
				if keyBindingGoType == nil {
					keyBindingGoType = types.Typ[types.Int]
				}
			}
		}
		if valueBindingTemp != "" {
			switch {
			case isMap:
				valueBindingGoType = mapValueGoType
			case isString:
				valueBindingGoType = types.Typ[types.Int32]
			case isSlice || isArray:
				valueBindingGoType = rangeArrayOrSliceElemType(s.X)
			}
		}

		// Classify a string range source before the loop's own variables are
		// registered: in "for s := range s" the source is the outer s.
		stringSourceIsBareVariable := isString && stringRangeSourceIsBareVariable(s.X)

		var popRangeVarScope func()
		if vt := GetVarTable(); vt != nil {
			vt.PushScope()
			popRangeVarScope = vt.PopScope
		}

		keyRangeRegistered := false
		keyTempRegistered := false
		valueRangeRegistered := false
		valueTempRegistered := false
		if s.Key != nil {
			if ident, ok := s.Key.(*ast.Ident); ok {
				keyName = ident.Name
				keyAssigned = !rangeAssignsExisting && rangeLoopIdentAssigned(s.Body, keyName)
				if keyBindingTemp != "" {
					registerRangeLoopVar(keyBindingTemp, keyRangeVarType, keyBindingGoType)
					keyTempRegistered = true
				} else {
					registerRangeLoopVar(keyName, keyRangeVarType, nil)
					keyRangeRegistered = true
					if keyName != "_" {
						if vt := GetVarTable(); vt != nil {
							vt.Register(keyName, &VarInfo{
								WrapLevel: WrapNone,
								RustType:  keyRangeVarType,
								Source:    SourceRangeKey,
							})
						}
					}
				}
			} else if keyBindingTemp != "" {
				registerRangeLoopVar(keyBindingTemp, keyRangeVarType, keyBindingGoType)
				keyTempRegistered = true
			}
		}
		// Track whether we need .copied() on the iterator to get owned values
		needsCopied := false
		needsCloned := false
		if s.Value != nil {
			if ident, ok := s.Value.(*ast.Ident); ok {
				valueName = ident.Name
				valueAssigned = !rangeAssignsExisting && rangeLoopIdentAssigned(s.Body, valueName)
				valueMutated = valueAssigned || (!rangeAssignsExisting && blockCallsPointerMethodOnValue(s.Body, valueName))
				registeredValueType := valueType
				// When using iter().enumerate(), the value is a reference
				// For basic/Copy types, use .copied() to get owned values
				if s.Key != nil && !isMap && !isString {
					// Check if element type is a numeric/bool (Rust Copy) type
					elemType := rangeArrayOrSliceElemType(s.X)
					if rangeElementUsesCopiedForExpr(s.X, elemType) {
						needsCopied = true
					}
					if elemType == nil {
						if elemRustType, ok := trackedRangeElemRustType(s.X); ok && rustRangeElemUsesCopied(elemRustType) {
							needsCopied = true
						} else if elemRustType, ok := rangeVarSliceElemRustType(s.X); ok && rustRangeElemUsesCopied(elemRustType) {
							needsCopied = true
						}
					}
					if rangeElementUsesCloned(elemType) {
						needsCloned = true
					}
					if valueType == "T" {
						if trackedValueType, trackedNeedsCopied, ok := trackedRangeElemValueType(s.X); ok {
							valueType = trackedValueType
							if trackedNeedsCopied {
								needsCopied = true
							}
						} else if elemRustType, ok := rangeVarSliceElemRustType(s.X); ok {
							valueType = rangeValueTypeFromTrackedRustElem(elemRustType)
							if rustRangeElemUsesCopied(elemRustType) {
								needsCopied = true
							}
						}
					}
					if (valueAssigned || needsCloned || needsCopied) && strings.HasPrefix(valueType, "&") && !isWrappedRangeVarType(valueType) {
						// .cloned() and .copied() yield owned values.
						registeredValueType = strings.TrimPrefix(valueType, "&")
					} else if valueType == "T" && (needsCopied || needsCloned || valueAssigned) {
						registeredValueType = valueType
					} else if valueType == "T" {
						registeredValueType = "ref_value"
					} else {
						registeredValueType = valueType
					}
				} else if valueAssigned && strings.HasPrefix(valueType, "&") {
					registeredValueType = strings.TrimPrefix(valueType, "&")
				} else if !rangeAssignsExisting && isString && valueAssigned && valueName != "_" {
					registeredValueType = "i32"
					stringRangeValueTemp = "__range_" + ToSnakeCase(valueName)
					rangePrelude = fmt.Sprintf("        let mut %s = %s as i32;\n", RustLocalIdent(valueName), stringRangeValueTemp)
				} else {
					registeredValueType = valueType
				}
				if valueBindingTemp != "" {
					registerRangeLoopVar(valueBindingTemp, registeredValueType, valueBindingGoType)
					valueTempRegistered = true
				} else {
					registerRangeLoopVar(valueName, registeredValueType, nil)
					valueRangeRegistered = true
					if valueName != "_" {
						if vt := GetVarTable(); vt != nil {
							vt.Register(valueName, &VarInfo{
								WrapLevel: WrapNone,
								RustType:  registeredValueType,
								Source:    SourceRangeVal,
							})
						}
					}
				}
			} else if valueBindingTemp != "" {
				registerRangeLoopVar(valueBindingTemp, valueType, valueBindingGoType)
				valueTempRegistered = true
			}
		}
		writeRangeKeyBinding := func() {
			writeRangePatternBinding(out, s.Key, keyAssigned, keyBindingTemp)
		}
		writeRangeValueBinding := func() {
			writeRangePatternBinding(out, s.Value, valueMutated, valueBindingTemp)
		}
		writeMapRangeKeyBinding := func() {
			if keyBindingTemp != "" {
				out.WriteString(keyBindingTemp)
				return
			}
			if (mapKeyNeedsValueBinding || mapKeyNeedsWrappedBinding) && keyName != "" && keyName != "_" {
				out.WriteString("__range_key")
				return
			}
			writeRangeKeyBinding()
		}
		writeStringRangeValueBinding := func() {
			if valueBindingTemp != "" {
				out.WriteString(valueBindingTemp)
				return
			}
			if stringRangeValueTemp != "" {
				out.WriteString(stringRangeValueTemp)
				return
			}
			writeRangeValueBinding()
		}
		if !rangeAssignsExisting && (mapKeyNeedsValueBinding || mapKeyNeedsWrappedBinding) && keyName != "" && keyName != "_" {
			var prelude strings.Builder
			prelude.WriteString("        let ")
			if keyAssigned {
				prelude.WriteString("mut ")
			}
			prelude.WriteString(RustLocalIdent(keyName))
			prelude.WriteString(" = ")
			if mapKeyNeedsValueBinding {
				prelude.WriteString("__range_key.value()")
			} else {
				WriteWrapperPrefix(&prelude)
				prelude.WriteString("__range_key.clone()")
				WriteWrapperSuffix(&prelude)
			}
			prelude.WriteString(";\n")
			rangePrelude = prelude.String()
		}
		appendRangeAssignmentPrelude := func(target ast.Expr, tempName string) {
			if target == nil || tempName == "" {
				return
			}
			var prelude strings.Builder
			prelude.WriteString("        ")
			assign := &ast.AssignStmt{
				Lhs: []ast.Expr{target},
				Tok: token.ASSIGN,
				Rhs: []ast.Expr{ast.NewIdent(tempName)},
			}
			TranspileStatement(&prelude, assign, fnType, fileSet, comments, nil, "        ")
			if strings.HasSuffix(strings.TrimSpace(prelude.String()), ";") {
				prelude.WriteString("\n")
			} else {
				prelude.WriteString(";\n")
			}
			rangePrelude += prelude.String()
		}
		if rangeAssignsExisting {
			appendRangeAssignmentPrelude(s.Key, keyBindingTemp)
			appendRangeAssignmentPrelude(s.Value, valueBindingTemp)
		}
		writeMapRangeSource := func() {
			if !isNamedMapExpression(s.X) && isExpressionResultBare(s.X) {
				out.WriteString("(")
				TranspileExpression(out, s.X)
				out.WriteString(").clone()")
				return
			}
			out.WriteString("{ let __range_holder = ")
			if isNamedMapExpression(s.X) {
				writeNilTolerantNamedMapInnerHandleClone(out, s.X)
			} else if ident, ok := s.X.(*ast.Ident); ok {
				name := RustIdentForUse(ident)
				if renamed, ok := captureRenameForIdent(ident); ok && renamed != "" {
					name = RustLocalIdent(renamed)
				}
				out.WriteString(name)
				out.WriteString(".clone()")
			} else {
				TranspileExpressionContext(out, s.X, LValue)
				out.WriteString(".clone()")
			}
			out.WriteString("; let __range_guard = __range_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map }")
		}

		if isInteger {
			if s.Value != nil {
				out.WriteString("/* ERROR: integer range permits at most one iteration variable */\n")
				out.WriteString("unimplemented!(\"invalid integer range\")")
			} else {
				if s.Key != nil {
					writeRangeKeyBinding()
				} else {
					out.WriteString("_")
				}
				out.WriteString(" in 0..(")
				var rangeLimit strings.Builder
				writeIntegerRangeLimit(&rangeLimit, s.X)
				writeRangeLengthDroppingTemporaries(out, rangeLimit.String())
				out.WriteString(")")
			}
		} else if isString {
			// String iteration - iterate over chars
			// Check if the range target is already a bare string (no wrapping needed).
			_, isStringLit := s.X.(*ast.BasicLit)
			// A named string ranges over a copy of the String it holds.
			var namedStringSource strings.Builder
			hasNamedStringSource := writeNamedStringNewtypeInnerValue(&namedStringSource, s.X)
			isBareStringVariable := stringSourceIsBareVariable && !hasNamedStringSource
			isBareStringRangeSource := isStringLit || isStringConstExpr(s.X) || hasNamedStringSource || isBareStringVariable
			writeBareStringRangeSource := func() {
				if hasNamedStringSource {
					out.WriteString("(")
					out.WriteString(namedStringSource.String())
					out.WriteString(")")
					return
				}
				if isBareStringVariable {
					// The loop ranges over a copy, so the body may assign
					// the variable.
					out.WriteString("(")
					out.WriteString(rustIdentForUseWithCapture(unwrapParens(s.X).(*ast.Ident)))
					out.WriteString(").clone()")
					return
				}
				TranspileExpression(out, s.X)
			}
			// Go ranges over the string's value as it was when the loop began,
			// so the loop reads a copy: holding the variable's cell for the
			// whole loop would deadlock (in the Arc/Mutex form) a body that
			// reads the string again, as strings.Map does.
			if s.Key != nil && s.Value != nil {
				// for i, c := range str
				out.WriteString("(")
				writeRangeKeyBinding()
				out.WriteString(", ")
				writeStringRangeValueBinding()
				if isBareStringRangeSource {
					out.WriteString(") in ")
					writeBareStringRangeSource()
					out.WriteString(".char_indices()")
				} else {
					out.WriteString(") in { let __range_string = (*")
					// Use raw identifier to avoid double-unwrapping
					if ident, ok := s.X.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.X)
					}
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).clone(); __range_string }.char_indices()")
				}
			} else if s.Value != nil {
				// for _, c := range str
				writeStringRangeValueBinding()
				if isBareStringRangeSource {
					out.WriteString(" in ")
					writeBareStringRangeSource()
					out.WriteString(".chars()")
				} else {
					out.WriteString(" in { let __range_string = (*")
					// Use raw identifier to avoid double-unwrapping
					if ident, ok := s.X.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.X)
					}
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).clone(); __range_string }.chars()")
				}
			} else if s.Key != nil {
				// for i := range str
				out.WriteString("(")
				writeRangeKeyBinding()
				out.WriteString(", _) in ")
				if isBareStringRangeSource {
					writeBareStringRangeSource()
					out.WriteString(".char_indices()")
				} else {
					out.WriteString("{ let __range_string = (*")
					if ident, ok := s.X.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.X)
					}
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).clone(); __range_string }.char_indices()")
				}
			} else {
				// for range str
				out.WriteString("_ in ")
				if isBareStringRangeSource {
					writeBareStringRangeSource()
					out.WriteString(".chars()")
				} else {
					out.WriteString("{ let __range_string = (*")
					if ident, ok := s.X.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.X)
					}
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).clone(); __range_string }.chars()")
				}
			}
		} else if isMap {
			// Map iteration - need to unwrap the Arc<Mutex<Option<HashMap>>>
			if s.Key != nil && s.Value != nil {
				// for k, v := range map
				out.WriteString("(")
				writeMapRangeKeyBinding()
				out.WriteString(", ")
				writeRangeValueBinding()
				out.WriteString(") in ")
				writeMapRangeSource()
			} else if s.Value != nil {
				// for _, v := range map (values only)
				out.WriteString("(_, ")
				writeRangeValueBinding()
				out.WriteString(") in ")
				writeMapRangeSource()
			} else if s.Key != nil {
				// for k := range map (keys only)
				out.WriteString("(")
				writeMapRangeKeyBinding()
				out.WriteString(", _) in ")
				writeMapRangeSource()
			} else {
				// for range map
				out.WriteString("_ in ")
				writeMapRangeSource()
			}
		} else {
			// Array/slice iteration
			if s.Key != nil && s.Value != nil {
				// Check if key is blank identifier
				if keyIdent, ok := s.Key.(*ast.Ident); ok && keyIdent.Name == "_" {
					// for _, v := range arr - just iterate values
					writeRangeValueBinding()
					// For numeric/bool (Rust Copy) types, use .iter().copied()
					// to get owned values instead of &(...) which gives references
					elemTypeV := rangeArrayOrSliceElemType(s.X)
					valCopied := rangeElementUsesCopiedForExpr(s.X, elemTypeV)
					valCloned := rangeElementUsesCloned(elemTypeV)
					if elemTypeV == nil {
						if elemRustType, ok := trackedRangeElemRustType(s.X); ok && rustRangeElemUsesCopied(elemRustType) {
							valCopied = true
						} else if elemRustType, ok := rangeVarSliceElemRustType(s.X); ok && rustRangeElemUsesCopied(elemRustType) {
							valCopied = true
						}
					}
					if rangeValuesVar != "" && rangeSliceElementIsEmptyInterface(s.X) {
						out.WriteString(" in ")
						out.WriteString(rangeValuesVar)
						writeRangeOwnedValuesAdapter(out, s.X, true, moveEmptyInterfaceRangeValues, false)
					} else if valCloned {
						out.WriteString(" in ")
						if rangeValuesVar != "" {
							out.WriteString(rangeValuesVar)
						} else {
							writeUnwrappedRangeTarget(out, s.X)
						}
						writeRangeOwnedValuesAdapter(out, s.X, rangeValuesVar != "", moveEmptyInterfaceRangeValues, false)
					} else if valCopied {
						out.WriteString(" in ")
						if rangeValuesVar != "" {
							out.WriteString(rangeValuesVar)
						} else {
							writeUnwrappedRangeTarget(out, s.X)
						}
						out.WriteString(".iter().copied()")
					} else if valueAssigned {
						out.WriteString(" in ")
						if rangeValuesVar != "" {
							out.WriteString(rangeValuesVar)
						} else {
							writeUnwrappedRangeTarget(out, s.X)
						}
						writeRangeOwnedValuesAdapter(out, s.X, rangeValuesVar != "", moveEmptyInterfaceRangeValues, false)
					} else {
						rangeValueBoundByReference = valueBindingTemp == ""
						out.WriteString(" in ")
						if rangeValuesVar != "" {
							out.WriteString(rangeValuesVar)
							out.WriteString(".iter()")
						} else if isReferenceRangeTarget(s.X) {
							writeUnwrappedRangeTarget(out, s.X)
							out.WriteString(".iter()")
						} else {
							out.WriteString("&")
							writeUnwrappedRangeTarget(out, s.X)
						}
					}
				} else {
					// for i, v := range arr
					out.WriteString("(")
					// Just output the identifier names, don't wrap them
					writeRangeKeyBinding()
					out.WriteString(", ")
					writeRangeValueBinding()
					out.WriteString(") in ")
					// Need to unwrap the collection
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
					} else {
						writeUnwrappedRangeTarget(out, s.X)
					}
					if rangeValuesVar != "" && rangeSliceElementIsEmptyInterface(s.X) {
						writeRangeOwnedValuesAdapter(out, s.X, true, moveEmptyInterfaceRangeValues, true)
					} else if needsCloned {
						writeRangeOwnedValuesAdapter(out, s.X, rangeValuesVar != "", moveEmptyInterfaceRangeValues, true)
					} else if needsCopied {
						out.WriteString(".iter().copied().enumerate()")
					} else if valueAssigned {
						writeRangeOwnedValuesAdapter(out, s.X, rangeValuesVar != "", moveEmptyInterfaceRangeValues, true)
					} else {
						rangeValueBoundByReference = valueBindingTemp == ""
						out.WriteString(".iter().enumerate()")
					}
				}
			} else if s.Value != nil {
				// for _, v := range arr
				writeRangeValueBinding()
				// For numeric/bool (Rust Copy) types, use .iter().copied()
				elemTypeV2 := rangeArrayOrSliceElemType(s.X)
				valCopied2 := rangeElementUsesCopiedForExpr(s.X, elemTypeV2)
				valCloned2 := rangeElementUsesCloned(elemTypeV2)
				if elemTypeV2 == nil {
					if elemRustType, ok := trackedRangeElemRustType(s.X); ok && rustRangeElemUsesCopied(elemRustType) {
						valCopied2 = true
					} else if elemRustType, ok := rangeVarSliceElemRustType(s.X); ok && rustRangeElemUsesCopied(elemRustType) {
						valCopied2 = true
					}
				}
				if rangeValuesVar != "" && rangeSliceElementIsEmptyInterface(s.X) {
					out.WriteString(" in ")
					out.WriteString(rangeValuesVar)
					writeRangeOwnedValuesAdapter(out, s.X, true, moveEmptyInterfaceRangeValues, false)
				} else if valCloned2 {
					out.WriteString(" in ")
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
					} else {
						writeUnwrappedRangeTarget(out, s.X)
					}
					writeRangeOwnedValuesAdapter(out, s.X, rangeValuesVar != "", moveEmptyInterfaceRangeValues, false)
				} else if valCopied2 {
					out.WriteString(" in ")
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
					} else {
						writeUnwrappedRangeTarget(out, s.X)
					}
					out.WriteString(".iter().copied()")
				} else if valueAssigned {
					out.WriteString(" in ")
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
					} else {
						writeUnwrappedRangeTarget(out, s.X)
					}
					writeRangeOwnedValuesAdapter(out, s.X, rangeValuesVar != "", moveEmptyInterfaceRangeValues, false)
				} else {
					rangeValueBoundByReference = valueBindingTemp == ""
					out.WriteString(" in ")
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
						out.WriteString(".iter()")
					} else if isReferenceRangeTarget(s.X) {
						writeUnwrappedRangeTarget(out, s.X)
						out.WriteString(".iter()")
					} else {
						out.WriteString("&")
						writeUnwrappedRangeTarget(out, s.X)
					}
				}
			} else if s.Key != nil {
				// for i := range arr
				writeRangeKeyBinding()
				out.WriteString(" in 0..(")
				if rangeValuesVar != "" {
					out.WriteString(rangeValuesVar)
					out.WriteString(".len()")
				} else {
					var rangeLen strings.Builder
					writeRangeLengthExpression(&rangeLen, s.X)
					writeRangeLengthDroppingTemporaries(out, rangeLen.String())
				}
				out.WriteString(")")
			} else {
				// for range arr
				out.WriteString("_ in ")
				if rangeValuesVar != "" {
					out.WriteString(rangeValuesVar)
				} else {
					writeUnwrappedRangeTarget(out, s.X)
				}
				out.WriteString(".iter()")
			}
		}
		out.WriteString(" {\n")
		out.WriteString(rangePrelude)
		if rangeValueBoundByReference {
			writeRangeValueIterationCopy(out, s.X, s.Value, s.Body)
		}
		if isMap && valueBindingTemp == "" {
			writeMapRangeValueIterationCopy(out, mapValueGoType, s.Value)
		}
		restoreLoopBreakTarget := pushBreakTarget("")

		currentLoopDepth++
		var rangeBodyLastPos token.Pos = s.Body.Lbrace
		var captureShadowRestores []func()
		for i, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &rangeBodyLastPos, "        ")
			writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.Body.List)-1)
			out.WriteString("\n")
			trackCaptureShadows(&captureShadowRestores, stmt)
		}
		restoreCaptureShadows(captureShadowRestores)
		currentLoopDepth--

		out.WriteString("    }")
		restoreLoopBreakTarget()
		if closeRangeGuard {
			out.WriteString(" }")
		}

		// Clean up range loop variables
		if keyRangeRegistered && keyName != "" {
			deleteRangeLoopVar(keyName)
		}
		if keyTempRegistered && keyBindingTemp != "" {
			deleteRangeLoopVar(keyBindingTemp)
		}
		if valueRangeRegistered && valueName != "" {
			deleteRangeLoopVar(valueName)
		}
		if valueTempRegistered && valueBindingTemp != "" {
			deleteRangeLoopVar(valueBindingTemp)
		}
		if popRangeVarScope != nil {
			popRangeVarScope()
		}
		popForPost()

	case *ast.IfStmt:
		// Handle init statement if present
		if s.Init != nil {
			transpileIfWithInitAsBlock(out, s, fnType, fileSet)
			return
		}
		beforeGuards := cloneMutexGuards(activeMutexGuards)

		out.WriteString("if ")
		transpileCondition(out, s.Cond)
		out.WriteString(" {\n")

		// Use comment-aware transpilation for the body
		activeMutexGuards = cloneMutexGuards(beforeGuards)
		popIfBodyScope := func() {}
		if vt := GetVarTable(); vt != nil {
			vt.PushScope()
			popIfBodyScope = vt.PopScope
		}
		var ifBodyLastPos token.Pos = s.Body.Lbrace
		var captureShadowRestores []func()
		for i, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &ifBodyLastPos, "        ")
			writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.Body.List)-1)
			out.WriteString("\n")
			trackCaptureShadows(&captureShadowRestores, stmt)
		}
		restoreCaptureShadows(captureShadowRestores)
		popIfBodyScope()
		thenGuards := cloneMutexGuards(activeMutexGuards)

		out.WriteString("    }")

		elseGuards := cloneMutexGuards(beforeGuards)
		if s.Else != nil {
			activeMutexGuards = cloneMutexGuards(beforeGuards)
			out.WriteString(" else ")
			if elseIf, ok := s.Else.(*ast.IfStmt); ok {
				// else if case
				if elseIf.Init != nil {
					transpileIfWithInitAsBlock(out, elseIf, fnType, fileSet)
				} else {
					// No init statement, handle normally
					TranspileStatementSimple(out, elseIf, fnType, fileSet)
				}
			} else if block, ok := s.Else.(*ast.BlockStmt); ok {
				// else block
				out.WriteString("{\n")
				popElseBodyScope := func() {}
				if vt := GetVarTable(); vt != nil {
					vt.PushScope()
					popElseBodyScope = vt.PopScope
				}
				var elseBodyLastPos token.Pos = block.Lbrace
				var captureShadowRestores []func()
				for i, stmt := range block.List {
					out.WriteString("        ")
					TranspileStatement(out, stmt, fnType, fileSet, comments, &elseBodyLastPos, "        ")
					writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(block.List)-1)
					out.WriteString("\n")
					trackCaptureShadows(&captureShadowRestores, stmt)
				}
				restoreCaptureShadows(captureShadowRestores)
				popElseBodyScope()
				out.WriteString("    }")
			}
			elseGuards = cloneMutexGuards(activeMutexGuards)
		}
		activeMutexGuards = mergeMutexGuardsAfterIfStmt(s, beforeGuards, thenGuards, elseGuards)

	case *ast.SwitchStmt:
		// Handle init statement if present
		restoreInitCaptureShadow := func() {}
		if s.Init != nil {
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			restoreInitCaptureShadow = suppressCaptureRenamesDeclaredBy(s.Init)
			out.WriteString("\n    ")
		}

		needsBreakTarget := switchNeedsSyntheticBreakTarget(s.Body)
		var restoreBreakTarget func()
		if needsBreakTarget {
			breakLabel := nextSwitchBreakLabel()
			restoreBreakTarget = pushBreakTarget(breakLabel)
			out.WriteString("'")
			out.WriteString(breakLabel)
			out.WriteString(": loop {\n        ")
		}

		// Check if any case has fallthrough
		hasFallthrough := false
		for _, stmt := range s.Body.List {
			if caseClause, ok := stmt.(*ast.CaseClause); ok {
				popCaseScope := pushCaseClauseScope()
				for _, bodyStmt := range caseClause.Body {
					if branch, ok := bodyStmt.(*ast.BranchStmt); ok && branch.Tok == token.FALLTHROUGH {
						hasFallthrough = true
						break
					}
				}
				popCaseScope()
				if hasFallthrough {
					break
				}
			}
		}

		var switchVal string
		var switchTagInterfaceType types.Type
		var switchTagInterfaceName string
		switchTagIsInterface := false
		switchTagIsEmptyInterface := false
		switchTagIsGoError := false
		switchTagIsPointer := false
		switchTagGoPtrInfo := goPtrResultInfo{}
		switchTagIsGoPtr := false
		if s.Tag != nil {
			switchVal = switchTagBindingName(s.Body)
			switchTagInterfaceType, switchTagInterfaceName, switchTagIsInterface = switchTagNamedInterface(s.Tag)
			switchTagIsEmptyInterface = switchTagEmptyInterface(s.Tag)
			switchTagIsGoError = switchTagGoError(s.Tag)
			if info, ok := switchGoPtrInfo(s.Tag); ok {
				switchTagGoPtrInfo = info
				switchTagIsGoPtr = true
			}
			if typeInfo := GetTypeInfo(); typeInfo != nil {
				switchTagIsPointer = typeInfo.IsPointer(s.Tag)
			}
		}

		if hasFallthrough {
			// Rust match doesn't support fallthrough — use if-chain with flags
			out.WriteString("{\n")

			if s.Tag != nil {
				out.WriteString("        let " + switchVal + " = ")
				if switchTagIsEmptyInterface {
					writeSwitchEmptyInterfaceTagHandle(out, s.Tag)
				} else if switchTagIsInterface {
					writeSwitchInterfaceTagHandle(out, s.Tag, switchTagInterfaceType)
				} else if switchTagIsGoError {
					writeSwitchGoErrorTagHandle(out, s.Tag)
				} else if switchTagIsPointer {
					writeSwitchPointerTagHandle(out, s.Tag)
				} else {
					writeSwitchTagValue(out, s.Tag)
				}
				out.WriteString(";\n")
			}
			out.WriteString("        let mut _fallthrough = false;\n")
			out.WriteString("        let mut _matched = false;\n")

			for _, stmt := range s.Body.List {
				if caseClause, ok := stmt.(*ast.CaseClause); ok {
					out.WriteString("        ")
					if len(caseClause.List) == 0 {
						// default case
						out.WriteString("if !_matched || _fallthrough {\n")
					} else {
						out.WriteString("if !_matched && (")
						for i, expr := range caseClause.List {
							if i > 0 {
								out.WriteString(" || ")
							}
							if s.Tag != nil {
								if switchTagIsEmptyInterface {
									writeSwitchEmptyInterfaceCaseComparison(out, switchVal, expr)
								} else if switchTagIsInterface {
									writeSwitchInterfaceCaseComparison(out, switchVal, expr, switchTagInterfaceType, switchTagInterfaceName)
								} else if switchTagIsGoError {
									writeSwitchGoErrorCaseComparison(out, switchVal, expr)
								} else if switchTagIsGoPtr {
									writeSwitchGoPtrCaseComparisonForInfo(out, switchVal, expr, switchTagGoPtrInfo)
								} else if switchTagIsPointer {
									writeSwitchPointerCaseComparison(out, switchVal, expr)
								} else {
									out.WriteString(switchVal + " == ")
									writeSwitchCaseValueForTag(out, expr, s.Tag)
								}
							} else {
								writeSwitchCaseValue(out, expr)
							}
						}
						out.WriteString(") || _fallthrough {\n")
					}

					out.WriteString("            _matched = true;\n")
					out.WriteString("            _fallthrough = false;\n")

					// Case body — replace fallthrough with flag set
					var caseBodyLastPos token.Pos = caseClause.Colon
					var captureShadowRestores []func()
					popCaseScope := pushCaseClauseScope()
					for _, bodyStmt := range caseClause.Body {
						if isUnlabeledBreakStmt(bodyStmt) {
							break
						}
						if branch, ok := bodyStmt.(*ast.BranchStmt); ok && branch.Tok == token.FALLTHROUGH {
							out.WriteString("            _fallthrough = true;\n")
							continue
						}
						out.WriteString("            ")
						TranspileStatement(out, bodyStmt, fnType, fileSet, comments, &caseBodyLastPos, "            ")
						out.WriteString("\n")
						trackCaptureShadows(&captureShadowRestores, bodyStmt)
					}
					popCaseScope()
					restoreCaptureShadows(captureShadowRestores)

					out.WriteString("        }\n")
				}
			}

			if switchStmtTerminates(s) {
				out.WriteString("        unreachable!()\n")
			}
			out.WriteString("    }")
		} else {
			// Use an if/else chain instead of Rust match patterns. Go case
			// expressions can be strings, calls, named constants, or other
			// non-pattern values, so explicit comparisons are the general form.
			if s.Tag != nil {
				out.WriteString("{ let " + switchVal + " = ")
				if switchTagIsEmptyInterface {
					writeSwitchEmptyInterfaceTagHandle(out, s.Tag)
				} else if switchTagIsInterface {
					writeSwitchInterfaceTagHandle(out, s.Tag, switchTagInterfaceType)
				} else if switchTagIsGoError {
					writeSwitchGoErrorTagHandle(out, s.Tag)
				} else if switchTagIsPointer {
					writeSwitchPointerTagHandle(out, s.Tag)
				} else {
					writeSwitchTagValue(out, s.Tag)
				}
				out.WriteString(";\n    ")
			}

			emittedCase := false
			var defaultClause *ast.CaseClause
			for _, stmt := range s.Body.List {
				if caseClause, ok := stmt.(*ast.CaseClause); ok {
					if len(caseClause.List) == 0 {
						defaultClause = caseClause
						continue
					}
					if emittedCase {
						out.WriteString(" else ")
					}
					out.WriteString("if ")
					for i, expr := range caseClause.List {
						if i > 0 {
							out.WriteString(" || ")
						}
						if s.Tag != nil {
							if switchTagIsEmptyInterface {
								writeSwitchEmptyInterfaceCaseComparison(out, switchVal, expr)
							} else if switchTagIsInterface {
								writeSwitchInterfaceCaseComparison(out, switchVal, expr, switchTagInterfaceType, switchTagInterfaceName)
							} else if switchTagIsGoError {
								writeSwitchGoErrorCaseComparison(out, switchVal, expr)
							} else if switchTagIsGoPtr {
								writeSwitchGoPtrCaseComparisonForInfo(out, switchVal, expr, switchTagGoPtrInfo)
							} else if switchTagIsPointer {
								writeSwitchPointerCaseComparison(out, switchVal, expr)
							} else {
								out.WriteString(switchVal + " == (")
								writeSwitchCaseValueForTag(out, expr, s.Tag)
								out.WriteString(")")
							}
						} else {
							transpileCondition(out, expr)
						}
					}
					out.WriteString(" {\n")

					var caseBodyLastPos token.Pos = caseClause.Colon
					var captureShadowRestores []func()
					popCaseScope := pushCaseClauseScope()
					for _, bodyStmt := range caseClause.Body {
						if isUnlabeledBreakStmt(bodyStmt) {
							break
						}
						out.WriteString("            ")
						TranspileStatement(out, bodyStmt, fnType, fileSet, comments, &caseBodyLastPos, "            ")
						out.WriteString("\n")
						trackCaptureShadows(&captureShadowRestores, bodyStmt)
					}
					popCaseScope()
					restoreCaptureShadows(captureShadowRestores)

					out.WriteString("        }")
					emittedCase = true
				}
			}

			if defaultClause != nil {
				if emittedCase {
					out.WriteString(" else {\n")
				} else {
					out.WriteString("{\n")
				}
				var caseBodyLastPos token.Pos = defaultClause.Colon
				var captureShadowRestores []func()
				popCaseScope := pushCaseClauseScope()
				for _, bodyStmt := range defaultClause.Body {
					if isUnlabeledBreakStmt(bodyStmt) {
						break
					}
					out.WriteString("            ")
					TranspileStatement(out, bodyStmt, fnType, fileSet, comments, &caseBodyLastPos, "            ")
					out.WriteString("\n")
					trackCaptureShadows(&captureShadowRestores, bodyStmt)
				}
				popCaseScope()
				restoreCaptureShadows(captureShadowRestores)
				out.WriteString("        }")
			}

			if s.Tag != nil {
				out.WriteString("\n    }")
			}
		}
		if needsBreakTarget {
			out.WriteString(";\n        break;\n    }")
			restoreBreakTarget()
		}
		restoreInitCaptureShadow()

	case *ast.BranchStmt:
		switch s.Tok {
		case token.BREAK:
			out.WriteString("break")
			if s.Label != nil {
				out.WriteString(" '" + ToSnakeCase(s.Label.Name))
			} else if target := currentBreakTarget(); target != "" {
				out.WriteString(" '")
				out.WriteString(target)
			}
		case token.CONTINUE:
			// In Go, `continue label` executes the for-loop's post-statement.
			// In Rust, `continue 'label` does not, so emit it explicitly.
			if s.Label != nil {
				label := ToSnakeCase(s.Label.Name)
				if postStmt, ok := labeledLoopPost[label]; ok {
					TranspileStatementSimple(out, postStmt, fnType, fileSet)
					out.WriteString("; ")
				}
				out.WriteString("continue '" + label)
			} else {
				if postStmt := currentForPost(); postStmt != nil {
					TranspileStatementSimple(out, postStmt, fnType, fileSet)
					out.WriteString("; ")
				}
				out.WriteString("continue")
				if target := currentContinueTarget(); target != "" {
					out.WriteString(" '")
					out.WriteString(target)
				}
			}
		case token.GOTO:
			if s.Label != nil {
				label := ToSnakeCase(s.Label.Name)
				if mode, ok := currentGotoLabelModes[label]; ok && mode == gotoReenterMode {
					// A goto back to a label whose loop was entered
					// early, at a forward goto before it, skips the
					// statements between the two on the way round.
					out.WriteString("{ ")
					out.WriteString(gotoReenterFlag(label))
					out.WriteString(" = true; continue '")
					out.WriteString(label)
					out.WriteString("; }")
				} else if ok {
					out.WriteString(mode)
					out.WriteString(" '")
					out.WriteString(label)
					out.WriteString(";")
				} else {
					out.WriteString("// TODO: unsupported goto ")
					out.WriteString(label)
				}
			} else {
				out.WriteString("// TODO: malformed goto")
			}
		case token.FALLTHROUGH:
			out.WriteString("// TODO: fallthrough not supported")
		}

	case *ast.SelectStmt:
		// Select statement — poll-based approach
		hasDefault := false
		var commClauses []*ast.CommClause
		for _, stmt := range s.Body.List {
			if cc, ok := stmt.(*ast.CommClause); ok {
				commClauses = append(commClauses, cc)
				if cc.Comm == nil {
					hasDefault = true
				}
			}
		}

		out.WriteString("{\n    ")
		operands := prepareSelectOperands(out, commClauses)
		out.WriteString("loop {\n")
		restoreSelectBreakTarget := pushBreakTarget("")

		for i, cc := range commClauses {
			if cc.Comm == nil {
				// Default case — handled at the end
				continue
			}
			if restore, needsTypeInfo := rewriteSelectReceiveAssignment(cc); needsTypeInfo {
				out.WriteString(`        unimplemented!("type info required for a select case that assigns a receive");` + "\n")
				continue
			} else if restore != nil {
				defer restore()
			}

			// Determine the type of communication
			switch comm := cc.Comm.(type) {
			case *ast.AssignStmt:
				// case val := <-ch or case val, ok := <-ch
				if len(comm.Rhs) == 1 {
					if unary, ok := unwrapParens(comm.Rhs[0]).(*ast.UnaryExpr); ok && unary.Op == token.ARROW {
						// A closed channel is ready too: it gives the zero
						// value, and ok is false.
						out.WriteString("        if let Some(__select_recv) = ")
						out.WriteString(operands[i].channel)
						out.WriteString(".try_recv_select() {\n")
						out.WriteString("            let __select_ok = __select_recv.is_some();\n")
						if ident, ok := comm.Lhs[0].(*ast.Ident); ok && ident.Name != "_" {
							out.WriteString("            let ")
							out.WriteString(EscapeRustIdent(ident.Name))
							out.WriteString(" = match __select_recv { Some(__v) => __v, None => ")
							out.WriteString(selectReceiveZeroValue(unary.X))
							out.WriteString(" };\n")
							// Register as range var so it's not unwrapped
							rangeLoopVars[ident.Name] = "select_val"
						}

						// Handle ok variable if present
						if len(comm.Lhs) == 2 {
							if okIdent, ok := comm.Lhs[1].(*ast.Ident); ok && okIdent.Name != "_" {
								out.WriteString("            let mut ")
								out.WriteString(EscapeRustIdent(okIdent.Name))
								out.WriteString(" = ")
								WriteWrapperPrefix(out)
								out.WriteString("__select_ok")
								WriteWrapperSuffix(out)
								out.WriteString(";\n")
							}
						}

						// Wrap the received value if needed for use in body
						if len(comm.Lhs) >= 1 {
							if ident, ok := comm.Lhs[0].(*ast.Ident); ok && ident.Name != "_" {
								out.WriteString("            let mut ")
								out.WriteString(EscapeRustIdent(ident.Name))
								out.WriteString(" = ")
								if channelElementIsHandleOption(unary.X) {
									writeErrorHandleFromOptionValue(out, EscapeRustIdent(ident.Name))
								} else {
									WriteWrapperPrefix(out)
									out.WriteString(EscapeRustIdent(ident.Name))
									WriteWrapperSuffix(out)
								}
								out.WriteString(";\n")
								// Now it's wrapped, remove from rangeLoopVars
								delete(rangeLoopVars, ident.Name)
							}
						}

						// Body
						var captureShadowRestores []func()
						popCaseScope := pushCaseClauseScope()
						for _, bodyStmt := range cc.Body {
							out.WriteString("            ")
							TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
							writeSelectBranchTerminator(out, bodyStmt)
							out.WriteString("\n")
							trackCaptureShadows(&captureShadowRestores, bodyStmt)
						}
						popCaseScope()
						restoreCaptureShadows(captureShadowRestores)
						if !stmtListTerminates(cc.Body) {
							out.WriteString("            break;\n")
						}
						out.WriteString("        }\n")
					}
				}

			case *ast.ExprStmt:
				// case <-ch (receive without assignment)
				if unary, ok := unwrapParens(comm.X).(*ast.UnaryExpr); ok && unary.Op == token.ARROW {
					out.WriteString("        if let Some(_) = ")
					out.WriteString(operands[i].channel)
					out.WriteString(".try_recv_select() {\n")

					var captureShadowRestores []func()
					popCaseScope := pushCaseClauseScope()
					for _, bodyStmt := range cc.Body {
						out.WriteString("            ")
						TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
						writeSelectBranchTerminator(out, bodyStmt)
						out.WriteString("\n")
						trackCaptureShadows(&captureShadowRestores, bodyStmt)
					}
					popCaseScope()
					restoreCaptureShadows(captureShadowRestores)
					if !stmtListTerminates(cc.Body) {
						out.WriteString("            break;\n")
					}
					out.WriteString("        }\n")
				}

			case *ast.SendStmt:
				// Keep a non-Copy payload available when a send has to retry.
				fmt.Fprintf(out, "        if %s.try_send_select(&mut %s) {\n", operands[i].channel, operands[i].value)

				var captureShadowRestores []func()
				popCaseScope := pushCaseClauseScope()
				for _, bodyStmt := range cc.Body {
					out.WriteString("            ")
					TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
					writeSelectBranchTerminator(out, bodyStmt)
					out.WriteString("\n")
					trackCaptureShadows(&captureShadowRestores, bodyStmt)
				}
				popCaseScope()
				restoreCaptureShadows(captureShadowRestores)
				if !stmtListTerminates(cc.Body) {
					out.WriteString("            break;\n")
				}
				out.WriteString("        }\n")
			}
		}

		// Default case
		if hasDefault {
			for _, cc := range commClauses {
				if cc.Comm == nil {
					var captureShadowRestores []func()
					popCaseScope := pushCaseClauseScope()
					for _, bodyStmt := range cc.Body {
						out.WriteString("        ")
						TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
						writeSelectBranchTerminator(out, bodyStmt)
						out.WriteString("\n")
						trackCaptureShadows(&captureShadowRestores, bodyStmt)
					}
					popCaseScope()
					restoreCaptureShadows(captureShadowRestores)
					if !stmtListTerminates(cc.Body) {
						out.WriteString("        break;\n")
					}
				}
			}
		} else {
			// No default — sleep briefly and retry
			out.WriteString("        std::thread::sleep(std::time::Duration::from_millis(1));\n")
		}

		restoreSelectBreakTarget()
		out.WriteString("    }")
		if selectStmtTerminates(s) {
			out.WriteString("\n    unreachable!()")
		}
		out.WriteString("\n    }")

	case *ast.DeferStmt:
		// Check if this is defer mu.Unlock() — suppress it (RAII guard handles
		// unlock). With no guard for the mutex, a function that has a defer
		// stack runs the Unlock from it.
		if isMutexUnlockDefer(s.Call) && (!currentFunctionHasDefer || mutexUnlockDeferHasGuard(s.Call)) {
			out.WriteString("// mu.Unlock() handled by RAII guard")
			break
		}
		// defer r.Unlock() through an embedded sync.RWMutex: Go follows the
		// embedded fields to the mutex when the defer runs, so the deferred
		// call unlocks that mutex even if one of those fields is reassigned
		// before the function returns.
		if sel, ok := s.Call.Fun.(*ast.SelectorExpr); ok && len(s.Call.Args) == 0 && isForwardedRWMutexMethodCall(sel) {
			out.WriteString("let __defer_mutex = ")
			writeForwardedRWMutexOf(out, sel.X)
			out.WriteString("; __defer_stack.push(Box::new(move || {\n")
			out.WriteString("        __defer_mutex.")
			out.WriteString(rustMethodSelectorName(sel))
			out.WriteString("();\n")
			out.WriteString("    }));")
			break
		}

		// Check if the defer contains a closure that captures variables
		captured := findCapturedInCall(s.Call)

		// Generate clones for captured variables
		// Sort variable names for deterministic output
		var capturedVars []string
		for varName := range captured {
			capturedVars = append(capturedVars, varName)
		}
		slices.Sort(capturedVars)

		assignedCaptured := assignedCapturedVarsInCall(s.Call, captured)
		captureRenames := make(map[string]string)
		for _, varName := range capturedVars {
			cloneName := varName + "_defer_captured"
			captureRenames[varName] = cloneName
			out.WriteString("let ")
			if assignedCaptured[varName] || currentReceiver != "" && varName == currentReceiver {
				out.WriteString("mut ")
			}
			out.WriteString(cloneName)
			out.WriteString(" = ")
			source := varName
			if currentCaptureRenames != nil {
				if renamed, exists := currentCaptureRenames[varName]; exists {
					source = RustLocalIdent(renamed)
				}
			} else if currentReceiver != "" && varName == currentReceiver {
				source = currentReceiverRustName()
			}
			if !assignedCaptured[varName] && !(currentReceiver != "" && varName == currentReceiver) && deferCapturedNameCopiesValue(s.Call, varName) {
				// The value the variable has when the defer statement runs.
				if isVarBare(varName) {
					// A bare local, such as the int a (uintptr, error) call
					// returns, is its own value.
					out.WriteString(source)
					out.WriteString(".clone(); ")
					continue
				}
				writeCapturedValueCopy(out, source)
				out.WriteString("; ")
				continue
			}
			out.WriteString(source)
			out.WriteString(".clone(); ")
		}

		// Store current capture renames for nested transpilation
		oldCaptureRenames := snapshotCaptureRenames()
		currentCaptureRenames = captureRenames

		// Check if the defer is calling an immediately invoked function literal
		// e.g., defer func(x int) { ... }(y)
		if funcLit, ok := s.Call.Fun.(*ast.FuncLit); ok && len(s.Call.Args) > 0 {
			// It's an immediately invoked function literal with arguments
			// We need to capture the argument values immediately

			// Generate captures for the arguments
			argCaptures := make([]string, len(s.Call.Args))
			for i, arg := range s.Call.Args {
				captureVar := fmt.Sprintf("__defer_arg_%d", i)
				argCaptures[i] = captureVar
				out.WriteString("let ")
				out.WriteString(captureVar)
				out.WriteString(" = ")
				writeDeferFuncLiteralArgumentCapture(out, arg, funcLiteralParamType(funcLit, i))
				out.WriteString("; ")
			}

			// Now generate the defer with the captured arguments
			out.WriteString("__defer_stack.push(Box::new(move || {\n")
			out.WriteString("        ")

			// Generate the closure directly (without Arc wrapper)
			out.WriteString("(move |")
			// Parameters
			if funcLit.Type.Params != nil {
				var params []string
				for _, field := range funcLit.Type.Params.List {
					paramType := GoTypeToRust(field.Type)
					for _, name := range field.Names {
						params = append(params, name.Name+": "+paramType)
					}
				}
				out.WriteString(strings.Join(params, ", "))
			}
			out.WriteString("| {\n        ")

			// Body
			for i, stmt := range funcLit.Body.List {
				TranspileStatementSimple(out, stmt, funcLit.Type, fileSet)
				out.WriteString(";")
				if i < len(funcLit.Body.List)-1 {
					out.WriteString("\n        ")
				}
			}

			out.WriteString("\n        })(")
			for i, capture := range argCaptures {
				if i > 0 {
					out.WriteString(", ")
				}
				out.WriteString(capture)
			}
			out.WriteString(");\n")
			out.WriteString("    }))")
		} else {
			// Regular defer call
			out.WriteString("__defer_stack.push(Box::new(move || {\n")
			out.WriteString("        ")
			if receiver, ok := mutexUnlockReceiver(s.Call); ok && receiver.sourceMapped {
				writeSourceMappedMutexReceiverRef(out, receiver)
				out.WriteString(".unlock()")
			} else {
				TranspileCall(out, s.Call)
			}
			out.WriteString(";\n")
			out.WriteString("    }))")
		}
		out.WriteString(";")

		// Restore previous capture renames
		currentCaptureRenames = oldCaptureRenames

	case *ast.GoStmt:
		// Track that we need thread import
		TrackImport("thread")

		// Check if the go statement contains a closure that captures variables
		captured := findCapturedInCall(s.Call)
		pointerCaptured := pointerCapturedVarsInCall(s.Call)
		channelCaptured := make(map[string]bool)
		if funcLit, ok := s.Call.Fun.(*ast.FuncLit); ok {
			channelCaptured = channelCapturesInFuncLitSyntax(funcLit)
			if len(channelCaptured) > 0 && captured == nil {
				captured = make(map[string]bool)
			}
			for name := range channelCaptured {
				captured[name] = true
			}
		}

		// Also find any channel-typed arguments in the function call
		// These need to be cloned before the move closure
		if _, isFuncLit := s.Call.Fun.(*ast.FuncLit); !isFuncLit {
			// Non-closure goroutine call - check args for channels
			for _, arg := range s.Call.Args {
				var argIdent *ast.Ident
				if ident, ok := arg.(*ast.Ident); ok {
					argIdent = ident
				} else if unary, ok := arg.(*ast.UnaryExpr); ok && unary.Op == token.AND {
					// &var — check the inner variable
					if ident, ok := unary.X.(*ast.Ident); ok {
						argIdent = ident
					}
				}
				if argIdent != nil {
					// All variable arguments need cloning for the move closure
					if _, isConst := localConstants[argIdent.Name]; !isConst {
						if argIdent.Name != "true" && argIdent.Name != "false" && argIdent.Name != "nil" {
							if captured == nil {
								captured = make(map[string]bool)
							}
							captured[argIdent.Name] = true
						}
					}
				}
			}
		}

		// Generate clones for captured variables
		// Sort variable names for deterministic output
		var capturedVars []string
		for varName := range captured {
			capturedVars = append(capturedVars, varName)
		}
		slices.Sort(capturedVars)

		assignedCaptured := assignedCapturedVarsInCall(s.Call, captured)
		for _, varName := range capturedVars {
			source := RustLocalIdent(varName)
			out.WriteString("let ")
			if assignedCaptured[varName] || currentReceiver != "" && varName == currentReceiver {
				out.WriteString("mut ")
			}
			out.WriteString(varName)
			out.WriteString("_thread = ")
			if currentReceiver != "" && varName == currentReceiver {
				out.WriteString(currentReceiverRustName())
				out.WriteString(".clone(); ")
			} else if !assignedCaptured[varName] && !isVarBare(varName) && deferredFuncValueName(s.Call, varName) {
				// "go f(x)" calls the func value f holds when the go
				// statement runs.
				writeCapturedValueCopy(out, source)
				out.WriteString("; ")
			} else if channelCaptured[varName] || pointerCaptured[varName] || isVarBare(varName) || goStmtCapturedNameIsFunctionHandle(s.Call, varName) {
				// Channel, pointer, bare, and function-typed variables already have handle semantics.
				out.WriteString(source)
				out.WriteString(".clone(); ")
			} else if goStmtSharesCapturedVariable(s.Call, varName) {
				// The goroutine works on the variable itself, as a closure does.
				out.WriteString(source)
				out.WriteString(".clone(); ")
			} else if goStmtCapturedNameIsEmptyInterface(s.Call, varName) {
				// An interface{} value in Box<dyn Any> has no Clone: copy it
				// through the value registry, as an assignment of it does.
				NeedAnyHandleCopy()
				out.WriteString("go_any_handle_copy(&")
				out.WriteString(source)
				out.WriteString("); ")
			} else {
				// A loop variable, or a value passed as an argument: Go copies
				// it when the go statement runs.
				WriteWrapperPrefix(out)
				out.WriteString("(*")
				out.WriteString(source)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
				WriteWrapperSuffix(out)
				out.WriteString("; ")
			}
		}

		var goFuncLitArgCaptures []string
		if funcLit, ok := s.Call.Fun.(*ast.FuncLit); ok && len(s.Call.Args) > 0 {
			goFuncLitArgCaptures = make([]string, len(s.Call.Args))
			for i, arg := range s.Call.Args {
				captureVar := fmt.Sprintf("__go_arg_%d", i)
				goFuncLitArgCaptures[i] = captureVar
				out.WriteString("let ")
				out.WriteString(captureVar)
				out.WriteString(" = ")
				if ident, ok := funcLiteralParamTypeExpr(funcLit, i).(*ast.Ident); ok && isFunctionLocalStructTypeName(ident) {
					// A struct type declared in the function is a bare
					// parameter of the literal, as in a call of the literal
					// that is not a go statement (#325): the goroutine gets
					// a copy of the value, taken out of the fresh cell the
					// copy is written in.
					out.WriteString("{ let __go_arg_cell = ")
					writeFunctionSignatureCallArgument(out, arg, funcLiteralParamType(funcLit, i))
					out.WriteString("; let __go_arg_value = __go_arg_cell")
					WriteBorrowMethod(out, true)
					out.WriteString(".take().unwrap(); __go_arg_value }")
				} else {
					writeFunctionSignatureCallArgument(out, arg, funcLiteralParamType(funcLit, i))
				}
				out.WriteString("; ")
			}
		}

		// go r.Unlock() through an embedded sync.RWMutex: Go follows the
		// embedded fields to the mutex before the goroutine starts, so the
		// goroutine unlocks that mutex even if one of those fields is
		// reassigned before it runs.
		goMutexCall := ""
		if sel, ok := s.Call.Fun.(*ast.SelectorExpr); ok && len(s.Call.Args) == 0 && isForwardedRWMutexMethodCall(sel) {
			out.WriteString("let __go_mutex = ")
			writeForwardedRWMutexOf(out, sel.X)
			out.WriteString("; ")
			goMutexCall = "__go_mutex." + rustMethodSelectorName(sel) + "()"
		}

		// Store current capture renames for nested transpilation
		captureRenames := make(map[string]string)
		for _, varName := range capturedVars {
			captureRenames[varName] = varName + "_thread"
		}
		oldCaptureRenames := snapshotCaptureRenames()
		currentCaptureRenames = captureRenames

		// Generate the thread::spawn call. A panic that nothing in the
		// goroutine recovers ends the whole program, as in Go, instead of
		// only its thread, which left the rest waiting on it forever.
		out.WriteString("std::thread::spawn(move || {\n")
		out.WriteString("    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {\n")
		out.WriteString("        ")

		// Check if it's an immediately invoked function literal
		if funcLit, ok := s.Call.Fun.(*ast.FuncLit); ok {
			hasClosureDefer := checkHasDefer(funcLit.Body.List)
			oldFunctionHasDefer := currentFunctionHasDefer
			currentFunctionHasDefer = hasClosureDefer
			defer func() { currentFunctionHasDefer = oldFunctionHasDefer }()
			defer enterClosureBody()()
			if hasClosureDefer {
				out.WriteString(deferStackDecl() + "\n        ")
			}

			// Generate the closure body inline
			if len(s.Call.Args) > 0 {
				// Has arguments - need to create parameter bindings
				out.WriteString("let ")
				// The closure pushes onto the defer stack it owns, so calling
				// it mutates it.
				if funcLitCallsFunctionValue(funcLit) || hasClosureDefer {
					out.WriteString("mut ")
				}
				out.WriteString("__closure = move |")
				// Parameters
				if funcLit.Type.Params != nil {
					var params []string
					for _, field := range funcLit.Type.Params.List {
						paramType := GoTypeToRust(field.Type)
						for _, name := range field.Names {
							params = append(params, RustLocalIdent(name.Name)+": "+paramType)
						}
					}
					out.WriteString(strings.Join(params, ", "))
				}
				out.WriteString("| {\n            ")
				recoverCatch := hasClosureDefer && shouldWrapDeferPanicRecover(true)
				if recoverCatch {
					beginPanicRecoverCatch(out, "")
				}

				// Body
				popParamScope := pushFuncLitParamVarScope(funcLit)
				restoreRangeShadows := shadowRangeLoopVars(funcLitParamNames(funcLit))
				var restoreCaptureShadows []func()
				for i, stmt := range funcLit.Body.List {
					if i > 0 {
						out.WriteString("\n            ")
					}
					TranspileStatementSimple(out, stmt, funcLit.Type, fileSet)
					out.WriteString(";")
					restoreCaptureShadows = append(restoreCaptureShadows, suppressCaptureRenamesDeclaredBy(stmt))
				}
				for i := len(restoreCaptureShadows) - 1; i >= 0; i-- {
					restoreCaptureShadows[i]()
				}
				restoreRangeShadows()
				popParamScope()

				if hasClosureDefer {
					out.WriteString("\n            while let Some(f) = __defer_stack.pop() {\n")
					out.WriteString("                f();\n")
					out.WriteString("            }")
				}
				if recoverCatch {
					out.WriteString("\n")
					finishPanicRecoverCatchWith(out, "            ", func() { out.WriteString("()") })
				}

				out.WriteString("\n        };\n")
				out.WriteString("        __closure(")

				// Arguments
				for i, capture := range goFuncLitArgCaptures {
					if i > 0 {
						out.WriteString(", ")
					}
					out.WriteString(capture)
				}
				out.WriteString(")")
			} else {
				// No arguments - just inline the body
				recoverCatch := hasClosureDefer && shouldWrapDeferPanicRecover(true)
				if recoverCatch {
					beginPanicRecoverCatch(out, "")
				}
				var restoreCaptureShadows []func()
				for i, stmt := range funcLit.Body.List {
					if i > 0 {
						out.WriteString("\n        ")
					}
					TranspileStatementSimple(out, stmt, funcLit.Type, fileSet)
					out.WriteString(";")
					restoreCaptureShadows = append(restoreCaptureShadows, suppressCaptureRenamesDeclaredBy(stmt))
				}
				for i := len(restoreCaptureShadows) - 1; i >= 0; i-- {
					restoreCaptureShadows[i]()
				}
				if hasClosureDefer {
					out.WriteString("\n        while let Some(f) = __defer_stack.pop() {\n")
					out.WriteString("            f();\n")
					out.WriteString("        }")
				}
				if recoverCatch {
					out.WriteString("\n")
					finishPanicRecoverCatchWith(out, "        ", func() { out.WriteString("()") })
				}
			}
		} else if goMutexCall != "" {
			out.WriteString(goMutexCall)
		} else {
			// Regular function call
			TranspileCall(out, s.Call)
		}

		out.WriteString(";\n")
		out.WriteString("    })) {\n")
		writeGoroutinePanicExit(out, "        ")
		out.WriteString("    }\n")
		out.WriteString("    })")
		out.WriteString(";")

		// Restore previous capture renames
		currentCaptureRenames = oldCaptureRenames

	case *ast.TypeSwitchStmt:
		// Type switch: switch v := x.(type) { ... }
		// We'll convert this to a series of if-else type checks

		// Extract the variable name and expression
		var varName string
		var expr ast.Expr

		if s.Assign != nil {
			// Has assignment: v := x.(type)
			if exprStmt, ok := s.Assign.(*ast.ExprStmt); ok {
				if typeAssert, ok := exprStmt.X.(*ast.TypeAssertExpr); ok {
					expr = typeAssert.X
				}
			} else if assign, ok := s.Assign.(*ast.AssignStmt); ok && len(assign.Lhs) == 1 && len(assign.Rhs) == 1 {
				if ident, ok := assign.Lhs[0].(*ast.Ident); ok {
					varName = ident.Name
				}
				if typeAssert, ok := assign.Rhs[0].(*ast.TypeAssertExpr); ok {
					expr = typeAssert.X
				}
			}
		} else if s.Init != nil {
			// Has init statement
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			out.WriteString(";\n")
		}

		if expr == nil {
			out.WriteString("// ERROR: Invalid type switch format")
			break
		}

		needsBreakTarget := switchNeedsSyntheticBreakTarget(s.Body)
		var restoreBreakTarget func()
		if needsBreakTarget {
			breakLabel := nextSwitchBreakLabel()
			restoreBreakTarget = pushBreakTarget(breakLabel)
			out.WriteString("'")
			out.WriteString(breakLabel)
			out.WriteString(": loop {\n    ")
		}

		typeInfo := GetTypeInfo()
		var typeSwitchSubjectType types.Type
		if typeInfo != nil {
			typeSwitchSubjectType = typeInfo.GetType(expr)
		}
		subjectUsesAny := isEmptyInterfaceValueExpr(expr)
		subjectIsLocalInterfaceRef := isLocalInterfaceRefIdent(expr) || isBareLocalInterfaceValue(expr)
		subjectIsTranspiledInterface := !subjectIsLocalInterfaceRef && isTranspiledInterfaceExpr(expr)
		typeSwitchCanRewrapInterfaceSubject := false
		typeSwitchSubjectHasGuard := false
		originalBindingSubject := ""
		if subjectUsesAny {
			TrackImport("Any")
		}
		if subjectIsLocalInterfaceRef || subjectIsTranspiledInterface {
			TrackImport("Any")
		}

		// Check if this is a range variable from an interface{} slice
		isRangeVar := false
		rangeVarType := ""
		isStdlibRangeRef := isStdlibInterfaceReferenceRangeValue(expr)
		if ident, ok := expr.(*ast.Ident); ok {
			if varType, exists := rangeLoopVars[ident.Name]; exists && strings.Contains(varType, "dyn Any") {
				isRangeVar = true
				rangeVarType = varType
				subjectUsesAny = true
				TrackImport("Any")
			}
		}

		// Open a block and borrow the value once for all cases
		out.WriteString("{\n")
		if isRangeVar {
			out.WriteString("    let _ts_ref = ")
			if !strings.HasPrefix(rangeVarType, "&") {
				out.WriteString("&")
			}
			TranspileExpression(out, expr)
			out.WriteString(";\n")
			if NeedsConcurrentWrapper() {
				out.WriteString("    let _ts_val: Option<&(dyn Any + Send + Sync)> = Some(_ts_ref.as_ref() as &(dyn Any + Send + Sync));\n")
			} else {
				out.WriteString("    let _ts_val: Option<&dyn Any> = Some(_ts_ref.as_ref() as &dyn Any);\n")
			}
			if strings.Contains(rangeVarType, "Box<dyn Any") {
				// A range variable over a []any: its nil holds GoNilAny. The
				// variable can be a reference or an owned clone, so test the
				// borrowed trait object rather than dereferencing it.
				NeedGoNilAny()
				out.WriteString("    let _ts_is_nil = _ts_val.map_or(false, |__v| __v.is::<GoNilAny>());\n")
			} else {
				out.WriteString("    let _ts_is_nil = false;\n")
			}
		} else if isStdlibRangeRef {
			out.WriteString("    let _ts_is_nil = false;\n")
			out.WriteString("    let _ts_val: Option<&")
			if typeInfo != nil {
				out.WriteString(goTypesTypeToRust(typeInfo.GetType(expr)))
			} else {
				out.WriteString("/* ERROR: Type information required for type switch subject */")
			}
			out.WriteString("> = Some(")
			writeStdlibInterfaceReferenceRangeValue(out, expr)
			out.WriteString(");\n")
		} else if subjectUsesAny {
			typeSwitchSubjectHasGuard = true
			originalBindingSubject = "_ts_subject"
			out.WriteString("    let _ts_subject = ")
			TranspileExpressionContext(out, expr, LValue)
			out.WriteString(".clone();\n")
			out.WriteString("    let _ts_guard = _ts_subject")
			WriteBorrowMethod(out, false)
			out.WriteString(";\n")
			// A nil element of a []any holds GoNilAny.
			NeedGoNilAny()
			out.WriteString("    let _ts_is_nil = _ts_guard.as_ref().map_or(true, |__v| (**__v).is::<GoNilAny>());\n")
			anyRef := "&dyn Any"
			if programAnyRuntimeOpsEnabled() && NeedsConcurrentWrapper() {
				anyRef = "&(dyn Any + Send + Sync)"
			}
			out.WriteString("    let _ts_val: Option<" + anyRef + "> = _ts_guard.as_ref().map(|__v| {\n")
			out.WriteString("        let mut __any = __v.as_ref() as " + anyRef + ";\n")
			out.WriteString("        while let Some(__boxed) = __any.downcast_ref::<" + rustAnyTraitObject() + ">() {\n")
			out.WriteString("            __any = __boxed.as_ref() as " + anyRef + ";\n")
			out.WriteString("        }\n")
			out.WriteString("        __any\n")
			out.WriteString("    });\n")
		} else if subjectIsLocalInterfaceRef {
			originalBindingSubject = "_ts_subject"
			out.WriteString("    let _ts_subject = ")
			TranspileExpressionContext(out, expr, LValue)
			if typeSwitchBareInterfaceSubjectNeedsClone(expr) {
				out.WriteString(".clone()")
			}
			out.WriteString(";\n")
			out.WriteString("    let _ts_is_nil = false;\n")
			out.WriteString("    let _ts_val: Option<&dyn Any> = Some(")
			out.WriteString(goAsAnyCall(goAsAnyQualifyingTrait(typeSwitchSubjectType), "_ts_subject", "&*_ts_subject"))
			out.WriteString(");\n")
		} else if subjectIsTranspiledInterface {
			typeSwitchCanRewrapInterfaceSubject = true
			originalBindingSubject = "_ts_subject"
			out.WriteString("    let _ts_subject = ")
			TranspileExpressionContext(out, expr, LValue)
			out.WriteString(".clone();\n")
			out.WriteString("    let _ts_guard = _ts_subject")
			WriteBorrowMethod(out, false)
			out.WriteString(";\n")
			out.WriteString("    let _ts_is_nil = _ts_guard.as_ref().is_none();\n")
			out.WriteString("    let _ts_owned = _ts_guard.as_ref().cloned();\n")
			out.WriteString("    std::mem::drop(_ts_guard);\n")
			out.WriteString("    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {\n")
			out.WriteString("        let __any = ")
			out.WriteString(goAsAnyCall(goAsAnyQualifyingTrait(typeSwitchSubjectType), "__v.as_ref()", "__v.as_ref()"))
			out.WriteString(";\n")
			if _, ok := transpiledNamedInterfaceTypeNameFromTypes(typeSwitchSubjectType); ok {
				out.WriteString("        if let Some(__boxed) = __any.downcast_ref::<")
				out.WriteString(goTypesTypeToRust(typeSwitchSubjectType))
				out.WriteString(">() {\n")
				out.WriteString("            ")
				out.WriteString(goAsAnyCall(goAsAnyQualifyingTrait(typeSwitchSubjectType), "__boxed.as_ref()", "__boxed.as_ref()"))
				out.WriteString("\n")
				out.WriteString("        } else {\n")
				out.WriteString("            __any\n")
				out.WriteString("        }\n")
			} else {
				out.WriteString("        __any\n")
			}
			out.WriteString("    });\n")
		} else {
			typeSwitchSubjectHasGuard = true
			originalBindingSubject = "_ts_subject"
			out.WriteString("    let _ts_subject = ")
			TranspileExpressionContext(out, expr, LValue)
			out.WriteString(".clone();\n")
			out.WriteString("    let _ts_guard = _ts_subject")
			WriteBorrowMethod(out, false)
			out.WriteString(";\n")
			out.WriteString("    let _ts_is_nil = _ts_guard.as_ref().is_none();\n")
			out.WriteString("    let _ts_val = _ts_guard.as_ref();\n")
		}

		// Generate if-else chain for type cases
		firstCase := true
		typeSwitchClauses := make([]*ast.CaseClause, 0, len(s.Body.List))
		var defaultClause *ast.CaseClause
		for _, clause := range s.Body.List {
			caseClause := clause.(*ast.CaseClause)
			if len(caseClause.List) == 0 {
				defaultClause = caseClause
				continue
			}
			typeSwitchClauses = append(typeSwitchClauses, caseClause)
		}
		if defaultClause != nil {
			typeSwitchClauses = append(typeSwitchClauses, defaultClause)
		}
		for _, caseClause := range typeSwitchClauses {
			isTypedSingleCase := varName != "" && len(caseClause.List) == 1
			if isTypedSingleCase {
				if _, isNil := typeSwitchCaseRustType(typeInfo, caseClause.List[0]); isNil {
					isTypedSingleCase = false
				}
			}
			caseVarAssigned := typeSwitchCaseVarAssigned(caseClause, varName)
			popCaseVarScope := pushTypeSwitchCaseVarScope(varName, isTypedSingleCase)

			if len(caseClause.List) == 0 {
				// default case
				if !firstCase {
					out.WriteString(" else {\n")
				} else {
					out.WriteString("    {\n")
				}
				if varName != "" {
					// In default case, v is the original interface{} value
					writeTypeSwitchOriginalBinding(out, varName, expr, originalBindingSubject, isRangeVar, isStdlibRangeRef, caseVarAssigned)
				}
			} else {
				// Type case(s)
				if !firstCase {
					out.WriteString(" else ")
				} else {
					out.WriteString("    ")
				}
				firstCase = false

				if len(caseClause.List) == 1 {
					rustType, isNil := typeSwitchCaseRustType(typeInfo, caseClause.List[0])
					out.WriteString("if ")
					writeTypeSwitchCaseCondition(out, typeInfo, caseClause.List[0], typeSwitchSubjectType, typeSwitchCanRewrapInterfaceSubject)
					out.WriteString(" {\n")

					// Create typed variable if needed
					if varName != "" && isNil {
						writeTypeSwitchOriginalBinding(out, varName, expr, originalBindingSubject, isRangeVar, isStdlibRangeRef, caseVarAssigned)
					} else if varName != "" && writeTypeSwitchInterfaceCaseBinding(out, typeInfo, varName, caseClause.List[0], typeSwitchSubjectType, caseVarAssigned, typeSwitchCanRewrapInterfaceSubject) {
						// interface case (named or anonymous): binding handled by the interface case path
					} else if varName != "" && rustType == "" {
						out.WriteString("        let ")
						if caseVarAssigned {
							out.WriteString("mut ")
						}
						out.WriteString(varName)
						out.WriteString(" = unimplemented!(\"type info required for type switch case\");\n")
					} else if varName != "" {
						out.WriteString("        let ")
						if caseVarAssigned {
							out.WriteString("mut ")
						}
						out.WriteString(varName)
						out.WriteString(" = ")
						if wrapperType, ok := typeSwitchCasePointerWrapper(typeInfo, caseClause.List[0], typeSwitchSubjectType); ok {
							out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
							out.WriteString(wrapperType)
							out.WriteString(">()).unwrap().0.clone()")
						} else if handleType, ok := typeSwitchCaseAnyHandleRustType(typeInfo, caseClause.List[0], typeSwitchSubjectType); ok && isErrorInterfaceType(typeInfo.GetType(caseClause.List[0])) {
							// An error is a value. The case variable gets a
							// handle of its own to the same error, so
							// assigning to it does not change what the
							// interface value holds.
							out.WriteString("if let Some(__err_handle) = _ts_val.and_then(|__v| __v.downcast_ref::<")
							out.WriteString(handleType)
							out.WriteString(">()) { ")
							writeErrorHandleCopy(out, func() { out.WriteString("__err_handle") })
							out.WriteString(" }")
							for _, candidate := range goErrorAssertionCandidates(typeSwitchSubjectType) {
								out.WriteString(" else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<")
								out.WriteString(candidate.rustType)
								out.WriteString(">()) { ")
								writeGoErrorHandleFromAssertionCandidate(out, candidate)
								out.WriteString(" }")
							}
							if programAnyRuntimeOpsEnabled() {
								NeedGoAnyRuntimeOps()
								out.WriteString(" else if let Some(__registered_error) = _ts_val.and_then(go_registered_any_error) { ")
								writeErrorHandleFromOptionValue(out, "Some(__registered_error)")
								out.WriteString(" }")
							}
							out.WriteString(" else { unreachable!(\"type switch error case matched no error payload\") }")
						} else if handleType, ok := typeSwitchCaseAnyHandleRustType(typeInfo, caseClause.List[0], typeSwitchSubjectType); ok {
							// The box holds the handle itself, so the case
							// variable shares it with every other holder.
							out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
							out.WriteString(handleType)
							out.WriteString(">()).unwrap().clone()")
						} else {
							// The case variable is a copy of the value the
							// interface holds, a struct's with fields of
							// its own (type_assertion_copy.go).
							if caseType := typeInfo.GetType(caseClause.List[0]); caseType == nil {
								out.WriteString(goValueCopyNeedsTypeInfo)
							} else {
								WriteWrapperPrefix(out)
								out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
								out.WriteString(rustType)
								out.WriteString(">()).unwrap()")
								out.WriteString(goValueCopyCallForType(caseType))
								WriteWrapperSuffix(out)
							}
						}
						out.WriteString(";\n")
					}
				} else {
					// Multiple types in one case
					out.WriteString("if ")
					for j, typeExpr := range caseClause.List {
						if j > 0 {
							out.WriteString(" || ")
						}
						writeTypeSwitchCaseCondition(out, typeInfo, typeExpr, typeSwitchSubjectType, typeSwitchCanRewrapInterfaceSubject)
					}
					out.WriteString(" {\n")
					if varName != "" {
						writeTypeSwitchOriginalBinding(out, varName, expr, originalBindingSubject, isRangeVar, isStdlibRangeRef, caseVarAssigned)
					}
				}
			}

			// Case body
			if typeSwitchSubjectHasGuard {
				out.WriteString("        std::mem::drop(_ts_guard);\n")
			}
			if varName != "" && len(caseClause.List) == 1 {
				writeGoSliceTypeSwitchBindingCopy(out, typeInfo, varName, caseClause.List[0])
			}
			restoreRangeLoopShadow := func() {}
			if varName != "" {
				restoreRangeLoopShadow = shadowRangeLoopVars([]string{varName})
			}
			restoreCaptureRename := suppressCaptureRename(varName)
			for _, stmt := range caseClause.Body {
				if isUnlabeledBreakStmt(stmt) {
					break
				}
				out.WriteString("        ")
				TranspileStatementSimple(out, stmt, fnType, fileSet)
				out.WriteString(";\n")
			}
			restoreCaptureRename()
			restoreRangeLoopShadow()
			popCaseVarScope()

			out.WriteString("    }")
		}
		out.WriteString("\n    }")
		if typeSwitchStmtTerminates(s) {
			out.WriteString("\n    unreachable!()")
		}
		if needsBreakTarget {
			out.WriteString(";\n    break;\n}")
			restoreBreakTarget()
		}

	case *ast.LabeledStmt:
		label := ToSnakeCase(s.Label.Name)
		switch s.Stmt.(type) {
		case *ast.SwitchStmt, *ast.TypeSwitchStmt, *ast.SelectStmt:
			out.WriteString("'")
			out.WriteString(label)
			out.WriteString(": {\n")
			out.WriteString(indent)
			out.WriteString("    ")
			TranspileStatement(out, s.Stmt, fnType, fileSet, comments, lastPos, indent+"    ")
			out.WriteString("\n")
			out.WriteString(indent)
			out.WriteString("}")
			return
		}
		if rangeStmt, ok := s.Stmt.(*ast.RangeStmt); ok && writeRangeOverFuncStatement(out, rangeStmt, s.Label.Name, fnType, fileSet, comments, lastPos, indent) {
			return
		}
		// Set the pending label for the next loop statement to consume
		pendingLoopLabel = label
		// Transpile the inner statement (usually a for/range loop)
		TranspileStatement(out, s.Stmt, fnType, fileSet, comments, lastPos, indent)
		pendingLoopLabel = "" // clear if not consumed

	default:
		out.WriteString("// TODO: Unhandled statement type: " + strings.TrimPrefix(fmt.Sprintf("%T", s), "*ast."))
	}
	if captureInfo != nil && !stmtTerminates(stmt) {
		statementPreprocessor.GeneratePostCaptureStatements(out, captureInfo)
	}
}

type multiResultReturnSlotConversion int

const (
	multiResultReturnSlotNoConversion multiResultReturnSlotConversion = iota
	multiResultReturnSlotWrapBareScalar
	multiResultReturnSlotBoxTranspiledInterface
	multiResultReturnSlotBoxSourceMappedPointerInterface
	multiResultReturnSlotBoxStdlibInterface
	multiResultReturnSlotUnsupportedStdlibInterface
)

type multiResultReturnSlotPlan struct {
	conversion multiResultReturnSlotConversion
	ifaceName  string
	targetRust string
	sourceRust string
	message    string
}

func writeMultiResultCallReturnWithSlotConversions(out *strings.Builder, call *ast.CallExpr, fnType *ast.FuncType) bool {
	sig, ok := callSignatureFromTypeInfo(call)
	if !ok || sig.Results() == nil || sig.Results().Len() == 0 {
		return false
	}
	results := sig.Results()
	plans := make([]multiResultReturnSlotPlan, results.Len())
	needsConversion := false
	for i := 0; i < results.Len(); i++ {
		actualType := results.At(i).Type()
		if ifaceName, wrapperType, ok := multiResultReturnSlotSourceMappedPointerInterfaceConversion(returnResultTypeExpr(fnType, i), actualType); ok {
			plans[i] = multiResultReturnSlotPlan{
				conversion: multiResultReturnSlotBoxSourceMappedPointerInterface,
				ifaceName:  ifaceName,
				sourceRust: wrapperType,
			}
			needsConversion = true
			continue
		}
		if ifaceName, ok := multiResultReturnSlotInterfaceConversion(returnResultTypeExpr(fnType, i), actualType); ok {
			plans[i] = multiResultReturnSlotPlan{
				conversion: multiResultReturnSlotBoxTranspiledInterface,
				ifaceName:  ifaceName,
			}
			needsConversion = true
			continue
		}
		if targetRust, ok := multiResultReturnSlotStdlibInterfaceConversion(returnResultTypeExpr(fnType, i), actualType); ok {
			plans[i] = multiResultReturnSlotPlan{
				conversion: multiResultReturnSlotBoxStdlibInterface,
				targetRust: targetRust,
			}
			needsConversion = true
			continue
		}
		if targetRust, message, ok := multiResultReturnSlotSourceMappedExternalStdlibInterfaceUnsupported(returnResultTypeExpr(fnType, i), actualType); ok {
			plans[i] = multiResultReturnSlotPlan{
				conversion: multiResultReturnSlotUnsupportedStdlibInterface,
				targetRust: targetRust,
				message:    message,
			}
			needsConversion = true
			continue
		}
		if returnSlotForcesWrappedValue(i) && callResultIsBareScalar(call, i) {
			plans[i] = multiResultReturnSlotPlan{conversion: multiResultReturnSlotWrapBareScalar}
			needsConversion = true
		}
	}
	if !needsConversion {
		return false
	}
	out.WriteString("{ let (")
	for i := 0; i < results.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(fmt.Sprintf("__return_tmp_%d", i))
	}
	out.WriteString(") = ")
	TranspileExpression(out, call)
	for i := 0; i < results.Len(); i++ {
		if plans[i].conversion == multiResultReturnSlotNoConversion {
			continue
		}
		out.WriteString("; let ")
		out.WriteString(fmt.Sprintf("__return_slot_%d", i))
		out.WriteString(" = ")
		writeMultiResultReturnSlotConversion(out, fmt.Sprintf("__return_tmp_%d", i), plans[i])
	}
	out.WriteString("; (")
	for i := 0; i < results.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		tmpName := fmt.Sprintf("__return_tmp_%d", i)
		if plans[i].conversion == multiResultReturnSlotNoConversion {
			out.WriteString(tmpName)
		} else {
			out.WriteString(fmt.Sprintf("__return_slot_%d", i))
		}
	}
	out.WriteString(") }")
	return true
}

func writeMultiResultReturnSlotConversion(out *strings.Builder, tmpName string, plan multiResultReturnSlotPlan) {
	switch plan.conversion {
	case multiResultReturnSlotWrapBareScalar:
		WriteWrapperPrefix(out)
		out.WriteString(tmpName)
		WriteWrapperSuffix(out)
	case multiResultReturnSlotBoxTranspiledInterface:
		writeTranspiledInterfaceBoxedReturnTemp(out, tmpName, plan.ifaceName)
	case multiResultReturnSlotBoxSourceMappedPointerInterface:
		writeSourceMappedPointerInterfaceBoxedReturnTemp(out, tmpName, plan.sourceRust, plan.ifaceName)
	case multiResultReturnSlotBoxStdlibInterface:
		writeStdlibInterfaceBoxedReturnTemp(out, tmpName, plan.targetRust)
	case multiResultReturnSlotUnsupportedStdlibInterface:
		writeUnsupportedStdlibInterfaceBoxedReturnTemp(out, plan.targetRust, plan.message)
	default:
		out.WriteString(tmpName)
	}
}

func multiResultReturnSlotInterfaceConversion(expected ast.Expr, actualType types.Type) (string, bool) {
	expectedType := expectedTypeFromParamExpr(expected)
	if expectedType == nil || actualType == nil {
		return "", false
	}
	if _, actualIsInterface := transpiledNamedInterfaceTypeNameFromTypes(actualType); actualIsInterface {
		return "", false
	}
	if !types.AssignableTo(actualType, expectedType) {
		return "", false
	}
	return transpiledNamedInterfaceTypeNameFromTypes(expectedType)
}

func multiResultReturnSlotSourceMappedPointerInterfaceConversion(expected ast.Expr, actualType types.Type) (string, string, bool) {
	expectedType := expectedTypeFromParamExpr(expected)
	if expectedType == nil || actualType == nil {
		return "", "", false
	}
	if !types.AssignableTo(actualType, expectedType) {
		return "", "", false
	}
	ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(expectedType)
	if !ok {
		return "", "", false
	}
	ptr, ok := types.Unalias(actualType).(*types.Pointer)
	if !ok {
		return "", "", false
	}
	elemNamed, ok := types.Unalias(ptr.Elem()).(*types.Named)
	if !ok || elemNamed.Obj() == nil || elemNamed.Obj().Pkg() == nil {
		return "", "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil && elemNamed.Obj().Pkg() == typeInfo.pkg {
		return "", "", false
	}
	if !isSourceMappedPackagePath(elemNamed.Obj().Pkg().Path()) {
		return "", "", false
	}
	if !sourceMappedPointerWrapperAvailableForInterface(elemNamed, expectedType) {
		return "", "", false
	}
	return ifaceName, sourceMappedPointerWrapperTypeName(elemNamed), true
}

func multiResultReturnSlotStdlibInterfaceConversion(expected ast.Expr, actualType types.Type) (string, bool) {
	expectedType := expectedTypeFromParamExpr(expected)
	targetRust, _, ok := stdlibInterfaceConversionForTypes(actualType, expectedType)
	return targetRust, ok
}

func multiResultReturnSlotSourceMappedExternalStdlibInterfaceUnsupported(expected ast.Expr, actualType types.Type) (string, string, bool) {
	expectedType := expectedTypeFromParamExpr(expected)
	if expectedType == nil || actualType == nil {
		return "", "", false
	}
	targetNamed, ok := types.Unalias(expectedType).(*types.Named)
	if !ok || targetNamed.Obj() == nil || targetNamed.Obj().Pkg() == nil {
		return "", "", false
	}
	targetPkgPath := targetNamed.Obj().Pkg().Path()
	if !isStubBackedStdlibPackagePath(targetPkgPath) {
		return "", "", false
	}
	targetInterface, ok := targetNamed.Underlying().(*types.Interface)
	if !ok {
		return "", "", false
	}
	sourceType := actualType
	if ptr, ok := types.Unalias(sourceType).(*types.Pointer); ok {
		sourceType = ptr.Elem()
	}
	sourceNamed, ok := types.Unalias(sourceType).(*types.Named)
	if !ok || sourceNamed.Obj() == nil || sourceNamed.Obj().Pkg() == nil {
		return "", "", false
	}
	sourcePkgPath := sourceNamed.Obj().Pkg().Path()
	if !isSourceMappedPackagePath(sourcePkgPath) || !isStdlibPackage(sourcePkgPath) {
		return "", "", false
	}
	targetInterface.Complete()
	if !types.Implements(actualType, targetInterface) {
		return "", "", false
	}
	message := fmt.Sprintf(
		"source-mapped %s.%s to external %s.%s requires source-mapped %s; transpile %s instead",
		sourcePkgPath,
		sourceNamed.Obj().Name(),
		targetPkgPath,
		targetNamed.Obj().Name(),
		targetPkgPath,
		targetPkgPath,
	)
	return goTypesNamedTypeToRust(targetNamed), message, true
}

func writeTranspiledInterfaceBoxedReturnTemp(out *strings.Builder, tmpName string, ifaceName string) {
	WriteWrapperPrefix(out)
	out.WriteString("Box::new((*")
	out.WriteString(tmpName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()) as ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	WriteWrapperSuffix(out)
}

func writeSourceMappedPointerInterfaceBoxedReturnTemp(out *strings.Builder, tmpName string, wrapperType string, ifaceName string) {
	WriteWrapperPrefix(out)
	out.WriteString("Box::new(")
	out.WriteString(wrapperType)
	out.WriteString("(")
	out.WriteString(tmpName)
	out.WriteString(".clone())) as ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	WriteWrapperSuffix(out)
}

func writeStdlibInterfaceBoxedReturnTemp(out *strings.Builder, tmpName string, targetRust string) {
	out.WriteString("{ let __arg_guard = ")
	out.WriteString(tmpName)
	WriteBorrowMethod(out, false)
	out.WriteString("; let __converted: Option<")
	out.WriteString(targetRust)
	out.WriteString("> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); ")
	WriteWrapperOptionPrefix(out)
	out.WriteString("__converted")
	WriteWrapperOptionSuffix(out)
	out.WriteString(" }")
}

func writeUnsupportedStdlibInterfaceBoxedReturnTemp(out *strings.Builder, targetRust string, message string) {
	out.WriteString("{ let __converted: Option<")
	out.WriteString(targetRust)
	out.WriteString("> = Some(unimplemented!(\"")
	out.WriteString(message)
	out.WriteString("\")); ")
	WriteWrapperOptionPrefix(out)
	out.WriteString("__converted")
	WriteWrapperOptionSuffix(out)
	out.WriteString(" }")
}

func writeStringAppendExpression(out *strings.Builder, rhs ast.Expr) {
	// A named string appends the String it holds.
	if writeNamedStringNewtypeInnerValue(out, rhs) {
		return
	}
	if writeBareStringSliceValue(out, rhs, types.Typ[types.String]) {
		return
	}
	if call, ok := rhs.(*ast.CallExpr); ok {
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.IsString(call) && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && !callReturnsBareChannelValue(call) {
			out.WriteString("{ let __s = ")
			TranspileExpression(out, rhs)
			out.WriteString("; let __value = (*__s")
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone(); __value }")
			return
		}
		if isPredeclaredTypeConversionTarget(call.Fun) {
			out.WriteString("{ let __s = ")
			TranspileExpression(out, rhs)
			out.WriteString("; let __value = (*__s")
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone(); __value }")
			return
		}
	}
	TranspileExpression(out, rhs)
}

func isSyntaxStringValue(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.Ident:
		if info := lookupVarInfo(e.Name); info != nil {
			return syntaxRustTypeIsString(info.RustType)
		}
	case *ast.SelectorExpr:
		if fieldExpr, ok := selectorFieldTypeExpr(e); ok {
			if ident, ok := fieldExpr.(*ast.Ident); ok {
				return ident.Name == "string"
			}
		}
	}
	return false
}

func syntaxRustTypeIsString(rustType string) bool {
	rustType = strings.TrimPrefix(rustType, "&")
	return rustType == "String" ||
		(strings.Contains(rustType, "Option<String>") &&
			!strings.Contains(rustType, "Vec<") &&
			!strings.Contains(rustType, "BTreeMap<") &&
			!strings.Contains(rustType, "HashMap<"))
}

func isSyntaxStringConversion(expr ast.Expr) bool {
	call, ok := expr.(*ast.CallExpr)
	if !ok {
		return false
	}
	ident, ok := call.Fun.(*ast.Ident)
	return ok && ident.Name == "string" && isPredeclaredTypeConversionTarget(call.Fun)
}

func isChannelAssignment(s *ast.AssignStmt) bool {
	if len(s.Lhs) != 1 || len(s.Rhs) != 1 || s.Tok != token.ASSIGN {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.IsChannel(s.Lhs[0])
}

// writeGoPtrFieldChannelAssignment writes p.C = c for a channel field of the
// struct a GoPtr local points at. A channel field holds its handle bare, so
// the assignment replaces the field inside the pointee.
func writeGoPtrFieldChannelAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	sel, ok := unwrapParens(lhs).(*ast.SelectorExpr)
	if !ok {
		return false
	}
	ident, ok := unwrapParens(sel.X).(*ast.Ident)
	if !ok || !isGoPtrVar(ident.Name) {
		return false
	}
	fieldInfo := selectorFieldAccessInfo(sel)
	if fieldInfo.IsPromoted {
		return false
	}
	out.WriteString("{ let __new_channel = ")
	writeChannelAssignmentValue(out, rhs)
	out.WriteString("; ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".with_mut(|__ptr_value| __ptr_value.")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(" = __new_channel); }")
	return true
}

func writeChannelAssignmentTarget(out *strings.Builder, lhs ast.Expr) {
	if sel, ok := unwrapParens(lhs).(*ast.SelectorExpr); ok && writePointerHandleSelectorTarget(out, sel) {
		return
	}
	TranspileExpressionContext(out, lhs, LValue)
}

func writeChannelAssignmentValue(out *strings.Builder, rhs ast.Expr) {
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("Default::default()")
		return
	}
	TranspileExpression(out, rhs)
	if channelAssignmentRHSNeedsClone(rhs) {
		out.WriteString(".clone()")
	}
}

func channelAssignmentRHSNeedsClone(rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || rhs == nil || !typeInfo.IsChannel(rhs) {
		return false
	}
	switch unwrapParens(rhs).(type) {
	case *ast.Ident, *ast.SelectorExpr, *ast.IndexExpr:
		return true
	default:
		return false
	}
}

// loopVariableObjects holds the variables a for or range statement
// declares. Go gives each iteration its own copy of them, so a goroutine
// started in the loop keeps the value of its iteration. The generated loop
// has one cell for the whole loop, so a goroutine copies it instead of
// sharing it.
var loopVariableObjects = map[types.Object]bool{}

// forClauseLoopVariableObjects holds the variables the init statement of a
// three-clause for loop declares. Since Go 1.22 each iteration has its own
// copy; the generated loop has one cell for the whole loop.
var forClauseLoopVariableObjects = map[types.Object]bool{}

// capturedNameCopiesValue reports whether a closure or a deferred call made
// in nodes should capture a copy of the value of the variable name rather
// than its cell. It does for a three-clause loop variable, which is a
// variable of its own in each iteration (Go 1.22), and, when argsOnly is
// set, for a variable named only in the arguments of a deferred call, which
// Go evaluates when the defer statement runs. Only numbers, strings, bools
// and types defined over them are copied; their Rust values clone as values.
func capturedNameCopiesValue(name string, loopVarNodes []ast.Node, argNodes []ast.Node, otherNodes []ast.Node) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || name == "" {
		return false
	}
	find := func(nodes []ast.Node) types.Object {
		var found types.Object
		for _, node := range nodes {
			if node == nil {
				continue
			}
			ast.Inspect(node, func(n ast.Node) bool {
				if found != nil {
					return false
				}
				if ident, ok := n.(*ast.Ident); ok && ident.Name == name {
					if obj, ok := typeInfo.info.Uses[ident].(*types.Var); ok {
						found = obj
					}
				}
				return true
			})
		}
		return found
	}
	scalar := func(obj types.Object) bool {
		basic, ok := types.Unalias(obj.Type()).Underlying().(*types.Basic)
		return ok && basic.Info()&(types.IsBoolean|types.IsNumeric|types.IsString) != 0
	}
	if obj := find(loopVarNodes); obj != nil && forClauseLoopVariableObjects[obj] && scalar(obj) {
		return true
	}
	if obj := find(argNodes); obj != nil && find(otherNodes) == nil && scalar(obj) {
		return true
	}
	return false
}

// writeCapturedValueCopy writes a new cell holding a copy of what the cell
// named source holds.
func writeCapturedValueCopy(out *strings.Builder, source string) {
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(")
		out.WriteString(GetInnerWrapperType())
		out.WriteString("::new((*")
		out.WriteString(source)
		out.WriteString(".lock().unwrap()).clone()))")
		return
	}
	out.WriteString("Rc::new(RefCell::new((*")
	out.WriteString(source)
	out.WriteString(".borrow()).clone()))")
}

func deferCapturedNameCopiesValue(call *ast.CallExpr, name string) bool {
	if call == nil {
		return false
	}
	if funcLit, ok := call.Fun.(*ast.FuncLit); ok {
		nodes := []ast.Node{funcLit.Body}
		for _, arg := range call.Args {
			nodes = append(nodes, arg)
		}
		return capturedNameCopiesValue(name, nodes, nil, nil)
	}
	args := make([]ast.Node, 0, len(call.Args))
	addressTaken := false
	for _, arg := range call.Args {
		args = append(args, arg)
		ast.Inspect(arg, func(n ast.Node) bool {
			if unary, ok := n.(*ast.UnaryExpr); ok && unary.Op == token.AND {
				if ident, ok := unwrapParens(unary.X).(*ast.Ident); ok && ident.Name == name {
					addressTaken = true
				}
			}
			return !addressTaken
		})
	}
	// "defer handle(&err)" passes a pointer to the variable itself.
	if addressTaken {
		return false
	}
	// "defer act(1)" calls the func value act holds when the defer runs,
	// and a func value passed as an argument is copied then too.
	if deferredFuncValueName(call, name) {
		return true
	}
	return capturedNameCopiesValue(name, append([]ast.Node{call.Fun}, args...), args, []ast.Node{call.Fun})
}

func registerLoopVariables(stmt ast.Stmt) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	var idents []ast.Expr
	switch s := stmt.(type) {
	case *ast.ForStmt:
		if init, ok := s.Init.(*ast.AssignStmt); ok && init.Tok == token.DEFINE {
			idents = init.Lhs
		}
	case *ast.RangeStmt:
		if s.Tok == token.DEFINE {
			idents = []ast.Expr{s.Key, s.Value}
		}
	}
	_, forClause := stmt.(*ast.ForStmt)
	for _, expr := range idents {
		if ident, ok := expr.(*ast.Ident); ok {
			if obj := typeInfo.info.Defs[ident]; obj != nil {
				loopVariableObjects[obj] = true
				if forClause {
					forClauseLoopVariableObjects[obj] = true
				}
			}
		}
	}
}

// goStmtSharesCapturedVariable reports whether the goroutine a go statement
// starts uses the variable name itself rather than a copy of its value.
//
// A function literal captures the variables it names, as any closure does,
// so "go func() { total += n }()" changes total. Copying it lost the write.
// The exceptions are loop variables (see loopVariableObjects) and the
// arguments of the call, which Go evaluates when the go statement runs. A
// slice or map argument is still shared, since its value is a handle, and so
// is a variable whose address is an argument: "go f(&n)".
func goStmtSharesCapturedVariable(call *ast.CallExpr, name string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	var obj types.Object
	inBody := false
	// visit finds the variable name refers to in node. In a function
	// literal, a name declared inside the literal is not the captured one.
	visit := func(node ast.Node, funcLit *ast.FuncLit) {
		ast.Inspect(node, func(n ast.Node) bool {
			ident, ok := n.(*ast.Ident)
			if !ok || ident.Name != name {
				return true
			}
			used, isVar := typeInfo.info.Uses[ident].(*types.Var)
			if !isVar {
				return true
			}
			if funcLit != nil && used.Pos() >= funcLit.Pos() && used.Pos() < funcLit.End() {
				return true
			}
			obj = used
			if funcLit != nil {
				inBody = true
			}
			return true
		})
	}
	addressTaken := false
	for _, arg := range call.Args {
		if funcLit, ok := arg.(*ast.FuncLit); ok {
			visit(funcLit.Body, funcLit)
		} else {
			visit(arg, nil)
			if unary, ok := unwrapParens(arg).(*ast.UnaryExpr); ok && unary.Op == token.AND {
				if ident, ok := unwrapParens(unary.X).(*ast.Ident); ok && ident.Name == name {
					addressTaken = true
				}
			}
		}
	}
	if funcLit, ok := call.Fun.(*ast.FuncLit); ok {
		visit(funcLit.Body, funcLit)
	}
	if obj == nil || loopVariableObjects[obj] {
		return false
	}
	// "go f(&n)" passes a pointer to n itself.
	if inBody || addressTaken {
		return true
	}
	switch types.Unalias(obj.Type()).Underlying().(type) {
	case *types.Slice, *types.Map:
		return true
	}
	return false
}

func goStmtImmediateArgIsFunctionHandle(ident *ast.Ident) bool {
	return isFunctionValueIdent(ident)
}

func goStmtCapturedNameIsFunctionHandle(call *ast.CallExpr, name string) bool {
	if call == nil || name == "" {
		return false
	}
	found := false
	ast.Inspect(call, func(n ast.Node) bool {
		if found {
			return false
		}
		ident, ok := n.(*ast.Ident)
		if !ok || ident.Name != name {
			return true
		}
		if isFunctionValueIdent(ident) {
			found = true
			return false
		}
		return true
	})
	return found
}

func isFunctionValueIdent(ident *ast.Ident) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || ident == nil || ident.Name == "_" || typeInfo.IsFunction(ident) {
		return false
	}
	typ := typeInfo.GetType(ident)
	if typ == nil {
		if obj := typeInfo.GetObject(ident); obj != nil {
			typ = obj.Type()
		}
	}
	return isFunctionSignatureType(typ)
}

func funcLitCallsFunctionValue(funcLit *ast.FuncLit) bool {
	if funcLit == nil || funcLit.Body == nil {
		return false
	}
	found := false
	ast.Inspect(funcLit.Body, func(n ast.Node) bool {
		if found {
			return false
		}
		call, ok := n.(*ast.CallExpr)
		if !ok {
			return true
		}
		if callTargetIsFunctionValue(call.Fun) {
			found = true
			return false
		}
		return true
	})
	return found
}

func callTargetIsFunctionValue(fun ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || fun == nil {
		return false
	}
	switch target := unwrapParens(fun).(type) {
	case *ast.Ident:
		return isFunctionValueIdent(target)
	case *ast.SelectorExpr:
		if obj := typeInfo.GetObject(target.Sel); obj != nil {
			if _, ok := obj.(*types.Func); ok {
				return false
			}
		}
	default:
		if objIdent, ok := fun.(*ast.Ident); ok && typeInfo.IsFunction(objIdent) {
			return false
		}
	}
	return isFunctionSignatureType(typeInfo.GetType(fun))
}

func suppressCaptureRename(name string) func() {
	if name == "" || currentCaptureRenames == nil {
		return func() {}
	}
	renamed, exists := currentCaptureRenames[name]
	if !exists {
		return func() {}
	}
	delete(currentCaptureRenames, name)
	return func() {
		currentCaptureRenames[name] = renamed
	}
}

// suppressCaptureRenamesDeclaredBy removes name-only outer capture aliases
// after a statement introduces block-local bindings with the same spelling.
// go/types Defs distinguishes new bindings in a short declaration from names
// that the statement merely assigns. The returned function restores aliases
// when the lexical block ends.
func suppressCaptureRenamesDeclaredBy(stmt ast.Stmt) func() {
	if stmt == nil || currentCaptureRenames == nil {
		return func() {}
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return func() {}
	}

	var names []*ast.Ident
	switch s := stmt.(type) {
	case *ast.AssignStmt:
		if s.Tok == token.DEFINE {
			for _, lhs := range s.Lhs {
				if ident, ok := lhs.(*ast.Ident); ok && ident.Name != "_" && typeInfo.info.Defs[ident] != nil {
					names = append(names, ident)
				}
			}
		}
	case *ast.DeclStmt:
		if decl, ok := s.Decl.(*ast.GenDecl); ok && (decl.Tok == token.VAR || decl.Tok == token.CONST) {
			for _, spec := range decl.Specs {
				if values, ok := spec.(*ast.ValueSpec); ok {
					for _, ident := range values.Names {
						if ident.Name != "_" && typeInfo.info.Defs[ident] != nil {
							names = append(names, ident)
						}
					}
				}
			}
		}
	}

	var restores []func()
	for _, ident := range names {
		if _, captured := currentCaptureRenames[ident.Name]; captured {
			restores = append(restores, suppressCaptureRename(ident.Name))
		}
	}
	return func() {
		for i := len(restores) - 1; i >= 0; i-- {
			restores[i]()
		}
	}
}

func trackCaptureShadows(restores *[]func(), stmt ast.Stmt) {
	*restores = append(*restores, suppressCaptureRenamesDeclaredBy(stmt))
}

func restoreCaptureShadows(restores []func()) {
	for i := len(restores) - 1; i >= 0; i-- {
		restores[i]()
	}
}

// selectReceiveZeroValue writes the value a receive from the closed channel
// ch gives, the zero value of its element type, as a comma-ok receive does.
func selectReceiveZeroValue(ch ast.Expr) string {
	return "Default::default()"
}

// writeSelectBranchTerminator ends a break, continue or goto in a select
// clause with a semicolon. Without it, "break 'outer" followed by the
// clause's own "break" was read as break 'outer with the value of break,
// which left only the select.
func writeSelectBranchTerminator(out *strings.Builder, stmt ast.Stmt) {
	if _, ok := stmt.(*ast.BranchStmt); ok {
		out.WriteString(";")
	}
}

// declaredIntConstants holds the constant expressions a variable of type int
// is declared with and nothing else types: x := 0, var n = 5. Rust gives an
// unsuffixed literal with no other constraint the type i32, so the variable
// would be an i32: it wraps at 32 bits, and in an interface it is not an int.
var declaredIntConstants = map[ast.Expr]bool{}

func markDeclaredIntConstants(stmt ast.Stmt) {
	mark := func(exprs []ast.Expr) {
		for _, expr := range exprs {
			declaredIntConstants[expr] = true
		}
	}
	switch s := stmt.(type) {
	case *ast.AssignStmt:
		if s.Tok == token.DEFINE {
			mark(s.Rhs)
		}
	case *ast.DeclStmt:
		if gen, ok := s.Decl.(*ast.GenDecl); ok && gen.Tok == token.VAR {
			for _, spec := range gen.Specs {
				if vs, ok := spec.(*ast.ValueSpec); ok && vs.Type == nil {
					mark(vs.Values)
				}
			}
		}
	case *ast.ForStmt:
		if s.Init != nil {
			markDeclaredIntConstants(s.Init)
		}
	case *ast.IfStmt:
		if s.Init != nil {
			markDeclaredIntConstants(s.Init)
		}
	case *ast.SwitchStmt:
		if s.Init != nil {
			markDeclaredIntConstants(s.Init)
		}
	}
}

// writeRangeLengthDroppingTemporaries writes the bound of a `for i in 0..n`
// loop. The temporaries of a for loop's iterator expression live until the
// loop ends, so a borrow or lock taken to read the length would still be held
// while the body writes the elements. A let statement drops them as soon as
// the length is read.
func writeRangeLengthDroppingTemporaries(out *strings.Builder, length string) {
	guarded := strings.HasPrefix(length, "({ let __range_holder") || strings.HasPrefix(length, "{ let __range_holder")
	if guarded || (!strings.Contains(length, ".borrow") && !strings.Contains(length, ".lock()") && !strings.Contains(length, ".read()")) {
		out.WriteString(length)
		return
	}
	out.WriteString("{ let __range_len = ")
	out.WriteString(length)
	out.WriteString("; __range_len }")
}

// writeTupleElementDroppingGuards writes one element of a tuple whose
// elements are evaluated in turn, as Go evaluates "b, a := st.Pop(),
// st.Pop()". A tuple keeps every temporary until the end of the statement,
// so a lock or mutable borrow taken by one element was still held when the
// next took it again. A let statement in a block drops it at once.
func writeTupleElementDroppingGuards(out *strings.Builder, value string) {
	if !strings.Contains(value, ".borrow_mut()") && !strings.Contains(value, ".lock()") {
		out.WriteString(value)
		return
	}
	out.WriteString("{ let __tuple_value = ")
	out.WriteString(value)
	out.WriteString("; __tuple_value }")
}

// writeStagedDeferReturnClose ends `let __return_value = <results>` and
// returns the staged results once the deferred calls have run.
func writeStagedDeferReturnClose(out *strings.Builder) {
	out.WriteString(";\n")
	out.WriteString("        // Execute deferred functions\n")
	out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
	out.WriteString("            f();\n")
	out.WriteString("        }\n")
	out.WriteString("        return __return_value;\n    }")
}

// writeBuiltinLengthForIntegerSlot writes len(x) or cap(x) stored into an
// integer element, as in skip[i] = len(pattern). Rust's len() is a usize,
// so it takes the element's integer type.
func writeBuiltinLengthForIntegerSlot(out *strings.Builder, rhs ast.Expr, expected types.Type) bool {
	call, ok := unwrapParens(rhs).(*ast.CallExpr)
	if !ok || expected == nil {
		return false
	}
	ident, ok := call.Fun.(*ast.Ident)
	if !ok || (ident.Name != "len" && ident.Name != "cap") || !isBuiltinCallTarget(ident) {
		return false
	}
	if _, isNamed := types.Unalias(expected).(*types.Named); isNamed {
		return false
	}
	basic, ok := types.Unalias(expected).Underlying().(*types.Basic)
	if !ok || basic.Info()&types.IsInteger == 0 {
		return false
	}
	var length strings.Builder
	TranspileExpression(&length, call)
	text := length.String()
	rustType := goTypesTypeToRust(expected)
	if strings.HasSuffix(text, " as "+rustType) {
		out.WriteString(text)
		return true
	}
	out.WriteString("(")
	out.WriteString(text)
	out.WriteString(") as ")
	out.WriteString(rustType)
	return true
}

// tailValueNeedsParens reports whether a tail expression starts with a
// block that more of the expression follows, as "{ a } < { b }" does. Rust
// parses such a leading block as a statement, so the expression needs
// parentheses. A value that is one block is a valid tail expression as it
// is. When the scan cannot tell, the value is parenthesized.
func tailValueNeedsParens(value string) bool {
	trimmed := strings.TrimSpace(value)
	if !strings.HasPrefix(trimmed, "{") {
		return false
	}
	end, ok := rustBlockEnd(trimmed)
	return !ok || end != len(trimmed)-1
}

// rustBlockEnd returns the index of the brace that closes the block opening
// at code[0], skipping string and character literals. It reports false for
// a raw string or anything else it does not scan.
func rustBlockEnd(code string) (int, bool) {
	depth := 0
	for i := 0; i < len(code); i++ {
		switch c := code[i]; c {
		case '{':
			depth++
		case '}':
			depth--
			if depth == 0 {
				return i, true
			}
		case '"':
			if i > 0 && (code[i-1] == 'r' || code[i-1] == '#') {
				return 0, false
			}
			for i++; i < len(code) && code[i] != '"'; i++ {
				if code[i] == '\\' {
					i++
				}
			}
		case '\'':
			// A character literal: 'x', '\n', '\u{7b}'. A lifetime such as
			// 'a has no closing quote and is skipped as one character.
			if i+3 < len(code) && code[i+1] == '\\' {
				j := strings.IndexByte(code[i+3:], '\'')
				if j < 0 {
					return 0, false
				}
				i += 3 + j
			} else if _, size := utf8.DecodeRuneInString(code[i+1:]); i+1+size < len(code) && code[i+1+size] == '\'' {
				i += 1 + size
			}
		}
	}
	return 0, false
}

// localNamedCollectionTypeNeedsNewtype reports whether a function-local
// type declaration defines an array, slice or map type outside a generic
// function, whose type parameters an item in its body could not use.
func localNamedCollectionTypeNeedsNewtype(typeSpec *ast.TypeSpec) bool {
	if typeSpec.Assign.IsValid() || typeSpec.TypeParams != nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	obj, ok := typeInfo.info.Defs[typeSpec.Name].(*types.TypeName)
	if !ok {
		return false
	}
	underlying := types.Unalias(obj.Type()).Underlying()
	switch underlying.(type) {
	case *types.Array, *types.Slice, *types.Map:
	default:
		return false
	}
	return !typeMentionsTypeParam(underlying)
}

// channelElemIsGoError reports whether ch is a channel of error.
func channelElemIsGoError(ch ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	chanType, ok := types.Unalias(typeInfo.GetType(ch)).Underlying().(*types.Chan)
	return ok && isGoErrorType(chanType.Elem())
}
