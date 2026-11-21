Warning: truncated output (original token count: 145751)
Total output lines: 17733

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
func returnExpressionsBorrowWrappedLocals(results []ast.Expr) bool {
	for _, expr := range results {
		if expressionReadsWrappedLocal(expr) {
			return true
		}
	}
	return false
}

func expressionReadsWrappedLocal(expr ast.Expr) bool {
	found := false
	ast.Inspect(expr, func(n ast.Node) bool {
		if found {
			return false
		}
		ident, ok := n.(*ast.Ident)
		if !ok {
			return true
		}
		if identIsWrappedFunctionLocal(ident) {
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
func identIsWrappedFunctionLocal(ident *ast.Ident) bool {
	if ident == nil || ident.Name == "_" {
		return false
	}
	if info := lookupVarInfo(ident.Name); info != nil {
		return info.Source == SourceLocal && info.WrapLevel != WrapNone
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
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
	// Names declared at or before the function body's opening brace are
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
	if writeGoSliceIndexedMutationSequence(out, indexExpr.X, func() {
		out.WriteString("let __idx = ")
		writeExpressionAsUsize(out, indexExpr.Index)
		out.WriteString("; ")
	}) {
		out.WriteString("let __current = __seq.get_cloned(__idx); __seq.set(__idx, __current ")
		if op == token.INC {
			out.WriteString("+ ")
		} else {
			out.WriteString("- ")
		}
		out.WriteString(incDecStepForType(elemType))
		out.WriteString("); }")
		return true
	}
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

func writeAnyBoxInnerValue(out *strings.Builder, expr ast.Expr) {
	// A constant goes into the interface as a value of its type.
	if writeNamedIntegerConstNewtypeValue(out, expr) || writeIntegerConstForAnyBox(out, expr) {
		return
	}
	if ident, ok := expr.(*ast.Ident); ok {
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
	if writeStdlibInterfaceTypeConversionAnyBox(out, expr) {
		return
	}
	// An error goes into a []any element the way it goes into any other
	// interface value.
	if writeGoErrorInterfaceAnyBox(out, expr) {
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
	if writeTypeParamArraySliceElementAssignmentValue(out, rhs, expected) {
		return
	}
	// A []any element slot stores a bare Box<dyn Any>; box a concrete value into
	// it. (A value already of interface type is handled by the local/stdlib
	// interface paths or the generic clone below.)
	if isEmptyInterfaceType(expected) && !isEmptyInterfaceValueExpr(rhs) {
		ident, isIdent := rhs.(*ast.Ident)
		if !isIdent || ident.Name != "nil" {
			writeBareAnyBox(out, rhs)
			return
		}
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
	if call == nil || !isBareBuiltinReturn(call) {
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
		if !isBareBuiltinCallName(call, "min") && !isBareBuiltinCallName(call, "max") {
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

func writeMultiRhsShortDeclInitializer(out *strings.Builder, lhs ast.Expr, rhs ast.Expr, newBinding bool) {
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
	if rhsIsPointerType(rhs) {
		// Pointer identifiers in parallel short declarations copy the handle.
		if !writeSourceMappedPackageGlobalPointerHandleClone(out, rhs) {
			TranspileExpressionContext(out, rhs, AddressOf)
			out.WriteString(".clone()")
		}
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
	// A named slice ("type Matrix [][]float64") keeps its elements in the
	// handle inside the newtype.
	_, _, namedSequence := namedSliceTypeForExpr(root)
	out.WriteString("{ ")
	if namedSequence {
		out.WriteString("let __seq = ")
		writeNamedSliceInnerHandleClone(out, root)
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
	out.WriteString("; ")
	if goSliceModeEnabled() && typeInfo.IsSlice(indexExpr.X) {
		if holdsSequence {
			out.WriteString("__seq")
		} else {
			TranspileExpressionContext(out, root, LValue)
		}
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
		for i, sliceContainer := range nestedSequenceContainerKinds(indexExpr.X) {
			if sliceContainer {
				out.WriteString(".get_cloned(")
				out.WriteString(outerIndexTempName(i))
				out.WriteString(")")
			} else {
				out.WriteString("[")
				out.WriteString(outerIndexTempName(i))
				out.WriteString("].clone()")
			}
		}
		out.WriteString(".set(__idx, __rhs); }")
		return true
	}
	out.WriteString("(*")
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

// nestedSequenceChainUsesOnlySlices reports whether every container traversed
// by seq (a[i][j] in a[i][j][k]) is a Go slice rather than an array. A copied
// inner GoSlice is still the same header/backing pair, so it can safely perform
// the final mutation without borrowing a Rust reference out of an outer guard.
func nestedSequenceChainUsesOnlySlices(seq ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	cur := unwrapParens(seq)
	for {
		index, ok := cur.(*ast.IndexExpr)
		if !ok {
			return typeInfo.IsSlice(cur)
		}
		if !typeInfo.IsSlice(index.X) {
			return false
		}
		cur = unwrapParens(index.X)
	}
}

// nestedSequenceContainerKinds reports whether each container traversed from
// the root of seq to its innermost value is a slice. Its order matches the
// index list returned by nestedSequenceIndexChain.
func nestedSequenceContainerKinds(seq ast.Expr) []bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil
	}
	var reversed []*ast.IndexExpr
	cur := unwrapParens(seq)
	for {
		index, ok := cur.(*ast.IndexExpr)
		if !ok {
			break
		}
		reversed = append(reversed, index)
		cur = unwrapParens(index.X)
	}
	kinds := make([]bool, 0, len(reversed))
	for i := len(reversed) - 1; i >= 0; i-- {
		kinds = append(kinds, typeInfo.IsSlice(reversed[i].X))
	}
	return kinds
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
	if rangeTargetUsesGoSliceGuard(expr) {
		defer out.WriteString(".as_slice()")
	}
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

func rangeTargetUsesGoSliceGuard(expr ast.Expr) bool {
	if !goSliceModeEnabled() {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	_, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Slice)
	return ok
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
		if _, isSlice := types.Unalias(named.Underlying()).(*types.Slice); isSlice {
			return false
		}
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Interface, *types.Pointer, *types.Signature, *types.Slice:
		out.WriteString(" = ")
		WriteWrappedNone(out)
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
		if _, isSlice := types.Unalias(named.Underlying()).(*types.Slice); isSlice {
			return false
		}
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Interface, *types.Pointer, *types.Signature, *types.Slice:
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
	prevBareStructAliases := bareStructAliases
	localRangeElemRustTypes = make(map[string]string)
	localCollectionKinds = make(map[string]string)
	localMapKeyRustTypes = make(map[string]string)
	localMapValueRustTypes = make(map[string]string)
	localMapValueKeepHandle = make(map[string]bool)
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
		bareStructAliases = prevBareStructAliases
	}
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
							WriteWrapperSuffix(…95751 tokens truncated…field.Type)
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
		if wrapForScope {
			out.WriteString("{\n    ")
		}
		if s.Init != nil {
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
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
		if s.Cond != nil {
			out.WriteString("while ")
			transpileCondition(out, s.Cond)
		} else {
			out.WriteString("loop")
		}
		out.WriteString(" {\n")
		popForPost := pushForPost(s.Post)
		restoreLoopBreakTarget := pushBreakTarget("")

		currentLoopDepth++
		var prevStmt ast.Stmt
		var forBodyLastPos token.Pos = s.Body.Lbrace
		if stmtListHasPlannedGoto(s.Body.List) {
			prevStmt = TranspileGotoStatementList(out, s.Body.List, fnType, fileSet, comments, &forBodyLastPos, "        ")
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

				prevStmt = stmt
			}
		}

		// Add the post statement (increment) if present
		if s.Post != nil {
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

	case *ast.BlockStmt:
		out.WriteString("{\n")
		var prevStmt ast.Stmt
		var blockLastPos token.Pos = s.Lbrace
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

			prevStmt = stmt
		}
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
			_, isInterfaceChannel := channelInterfaceElementType(s.X)
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
			for i, stmt := range s.Body.List {
				out.WriteString("        ")
				TranspileStatement(out, stmt, fnType, fileSet, comments, &rangeBodyLastPos, "        ")
				writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.Body.List)-1)
				out.WriteString("\n")
			}
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
					valueType = "&" + goTypesTypeToRust(elemType)
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
			if !isNamedMapExpression(s.X) && (isExpressionResultBare(s.X) || (!NeedsConcurrentWrapper() && isBareMapSelectorExpression(s.X))) {
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
			out.WriteString("; let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map }")
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
			isBareStringRangeSource := isStringLit || isStringConstExpr(s.X) || hasNamedStringSource
			writeBareStringRangeSource := func() {
				if hasNamedStringSource {
					out.WriteString("(")
					out.WriteString(namedStringSource.String())
					out.WriteString(")")
					return
				}
				TranspileExpression(out, s.X)
			}
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
					out.WriteString(") in (*")
					// Use raw identifier to avoid double-unwrapping
					if ident, ok := s.X.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.X)
					}
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).char_indices()")
				}
			} else if s.Value != nil {
				// for _, c := range str
				writeStringRangeValueBinding()
				if isBareStringRangeSource {
					out.WriteString(" in ")
					writeBareStringRangeSource()
					out.WriteString(".chars()")
				} else {
					out.WriteString(" in (*")
					// Use raw identifier to avoid double-unwrapping
					if ident, ok := s.X.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.X)
					}
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).chars()")
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
					out.WriteString("(*")
					if ident, ok := s.X.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.X)
					}
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).char_indices()")
				}
			} else {
				// for range str
				out.WriteString("_ in ")
				if isBareStringRangeSource {
					writeBareStringRangeSource()
					out.WriteString(".chars()")
				} else {
					out.WriteString("(*")
					if ident, ok := s.X.(*ast.Ident); ok {
						out.WriteString(EscapeRustIdent(ident.Name))
					} else {
						TranspileExpression(out, s.X)
					}
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).chars()")
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
					if valCloned {
						out.WriteString(" in ")
						if rangeValuesVar != "" {
							out.WriteString(rangeValuesVar)
						} else {
							writeUnwrappedRangeTarget(out, s.X)
						}
						writeRangeOwnedValuesAdapter(out, s.X, rangeValuesVar != "", false)
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
						writeRangeOwnedValuesAdapter(out, s.X, rangeValuesVar != "", false)
					} else {
						rangeValueBoundByReference = valueBindingTemp == ""
						out.WriteString(" in ")
						if rangeValuesVar != "" {
							out.WriteString(rangeValuesVar)
							out.WriteString(".iter()")
						} else if isReferenceRangeTarget(s.X) || rangeTargetUsesGoSliceGuard(s.X) {
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
					if needsCloned {
						writeRangeOwnedValuesAdapter(out, s.X, rangeValuesVar != "", true)
					} else if needsCopied {
						out.WriteString(".iter().copied().enumerate()")
					} else if valueAssigned {
						writeRangeOwnedValuesAdapter(out, s.X, rangeValuesVar != "", true)
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
				if valCloned2 {
					out.WriteString(" in ")
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
					} else {
						writeUnwrappedRangeTarget(out, s.X)
					}
					writeRangeOwnedValuesAdapter(out, s.X, rangeValuesVar != "", false)
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
					writeRangeOwnedValuesAdapter(out, s.X, rangeValuesVar != "", false)
				} else {
					rangeValueBoundByReference = valueBindingTemp == ""
					out.WriteString(" in ")
					if rangeValuesVar != "" {
						out.WriteString(rangeValuesVar)
						out.WriteString(".iter()")
					} else if isReferenceRangeTarget(s.X) || rangeTargetUsesGoSliceGuard(s.X) {
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
			writeRangeValueIterationCopy(out, s.X, s.Value)
		}
		if isMap && valueBindingTemp == "" {
			writeMapRangeValueIterationCopy(out, mapValueGoType, s.Value)
		}
		restoreLoopBreakTarget := pushBreakTarget("")

		currentLoopDepth++
		var rangeBodyLastPos token.Pos = s.Body.Lbrace
		for i, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &rangeBodyLastPos, "        ")
			writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.Body.List)-1)
			out.WriteString("\n")
		}
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
		for i, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &ifBodyLastPos, "        ")
			writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(s.Body.List)-1)
			out.WriteString("\n")
		}
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
				for i, stmt := range block.List {
					out.WriteString("        ")
					TranspileStatement(out, stmt, fnType, fileSet, comments, &elseBodyLastPos, "        ")
					writeStatementSeparatorBeforeFollowingStatement(out, stmt, i < len(block.List)-1)
					out.WriteString("\n")
				}
				popElseBodyScope()
				out.WriteString("    }")
			}
			elseGuards = cloneMutexGuards(activeMutexGuards)
		}
		activeMutexGuards = mergeMutexGuardsAfterIf(beforeGuards, thenGuards, elseGuards)

	case *ast.SwitchStmt:
		// Handle init statement if present
		if s.Init != nil {
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
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
				for _, bodyStmt := range caseClause.Body {
					if branch, ok := bodyStmt.(*ast.BranchStmt); ok && branch.Tok == token.FALLTHROUGH {
						hasFallthrough = true
						break
					}
				}
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
					}

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
					for _, bodyStmt := range caseClause.Body {
						if isUnlabeledBreakStmt(bodyStmt) {
							break
						}
						out.WriteString("            ")
						TranspileStatement(out, bodyStmt, fnType, fileSet, comments, &caseBodyLastPos, "            ")
						out.WriteString("\n")
					}

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
				for _, bodyStmt := range defaultClause.Body {
					if isUnlabeledBreakStmt(bodyStmt) {
						break
					}
					out.WriteString("            ")
					TranspileStatement(out, bodyStmt, fnType, fileSet, comments, &caseBodyLastPos, "            ")
					out.WriteString("\n")
				}
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
			}
		case token.GOTO:
			if s.Label != nil {
				label := ToSnakeCase(s.Label.Name)
				if mode, ok := currentGotoLabelModes[label]; ok {
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

		out.WriteString("loop {\n")
		restoreSelectBreakTarget := pushBreakTarget("")

		for _, cc := range commClauses {
			if cc.Comm == nil {
				// Default case — handled at the end
				continue
			}

			// Determine the type of communication
			switch comm := cc.Comm.(type) {
			case *ast.AssignStmt:
				// case val := <-ch or case val, ok := <-ch
				if len(comm.Rhs) == 1 {
					if unary, ok := comm.Rhs[0].(*ast.UnaryExpr); ok && unary.Op == token.ARROW {
						// A closed channel is ready too: it gives the zero
						// value, and ok is false.
						out.WriteString("        if let Some(__select_recv) = ")
						writeChannelExpression(out, unary.X)
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
						for _, bodyStmt := range cc.Body {
							out.WriteString("            ")
							TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
							writeSelectBranchTerminator(out, bodyStmt)
							out.WriteString("\n")
						}
						if !stmtListTerminates(cc.Body) {
							out.WriteString("            break;\n")
						}
						out.WriteString("        }\n")
					}
				}

			case *ast.ExprStmt:
				// case <-ch (receive without assignment)
				if unary, ok := comm.X.(*ast.UnaryExpr); ok && unary.Op == token.ARROW {
					out.WriteString("        if let Some(_) = ")
					writeChannelExpression(out, unary.X)
					out.WriteString(".try_recv_select() {\n")

					for _, bodyStmt := range cc.Body {
						out.WriteString("            ")
						TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
						writeSelectBranchTerminator(out, bodyStmt)
						out.WriteString("\n")
					}
					if !stmtListTerminates(cc.Body) {
						out.WriteString("            break;\n")
					}
					out.WriteString("        }\n")
				}

			case *ast.SendStmt:
				// case ch <- val (send case) — use try_send for non-blocking
				out.WriteString("        if ")
				writeChannelExpression(out, comm.Chan)
				out.WriteString(".try_send(")
				if !writeInterfaceChannelSendValue(out, comm.Chan, comm.Value) {
					transpileChannelValue(out, comm.Value)
				}
				out.WriteString(") {\n")

				for _, bodyStmt := range cc.Body {
					out.WriteString("            ")
					TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
					writeSelectBranchTerminator(out, bodyStmt)
					out.WriteString("\n")
				}
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
					for _, bodyStmt := range cc.Body {
						out.WriteString("        ")
						TranspileStatementSimple(out, bodyStmt, fnType, fileSet)
						writeSelectBranchTerminator(out, bodyStmt)
						out.WriteString("\n")
					}
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

	case *ast.DeferStmt:
		// Check if this is defer mu.Unlock() — suppress it (RAII guard handles unlock)
		if isMutexUnlockDefer(s.Call) {
			out.WriteString("// mu.Unlock() handled by RAII guard")
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
			out.WriteString("let ")
			if assignedCaptured[varName] || currentReceiver != "" && varName == currentReceiver {
				out.WriteString("mut ")
			}
			out.WriteString(varName)
			out.WriteString("_thread = ")
			if currentReceiver != "" && varName == currentReceiver {
				out.WriteString(currentReceiverRustName())
				out.WriteString(".clone(); ")
			} else if channelCaptured[varName] || pointerCaptured[varName] || isVarBare(varName) || goStmtCapturedNameIsFunctionHandle(s.Call, varName) {
				// Channel, pointer, bare, and function-typed variables already have handle semantics.
				out.WriteString(varName)
				out.WriteString(".clone(); ")
			} else if goStmtSharesCapturedVariable(s.Call, varName) {
				// The goroutine works on the variable itself, as a closure does.
				out.WriteString(varName)
				out.WriteString(".clone(); ")
			} else {
				// A loop variable, or a value passed as an argument: Go copies
				// it when the go statement runs.
				WriteWrapperPrefix(out)
				out.WriteString("(*")
				out.WriteString(varName)
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
				writeFunctionSignatureCallArgument(out, arg, funcLiteralParamType(funcLit, i))
				out.WriteString("; ")
			}
		}

		// Store current capture renames for nested transpilation
		captureRenames := make(map[string]string)
		for _, varName := range capturedVars {
			captureRenames[varName] = varName + "_thread"
		}
		oldCaptureRenames := snapshotCaptureRenames()
		currentCaptureRenames = captureRenames

		// Generate the thread::spawn call
		out.WriteString("std::thread::spawn(move || {\n")
		out.WriteString("        ")

		// Check if it's an immediately invoked function literal
		if funcLit, ok := s.Call.Fun.(*ast.FuncLit); ok {
			hasClosureDefer := checkHasDefer(funcLit.Body.List)
			oldFunctionHasDefer := currentFunctionHasDefer
			currentFunctionHasDefer = hasClosureDefer
			defer func() { currentFunctionHasDefer = oldFunctionHasDefer }()
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

				// Body
				popParamScope := pushFuncLitParamVarScope(funcLit)
				restoreRangeShadows := shadowRangeLoopVars(funcLitParamNames(funcLit))
				for i, stmt := range funcLit.Body.List {
					if i > 0 {
						out.WriteString("\n            ")
					}
					TranspileStatementSimple(out, stmt, funcLit.Type, fileSet)
					out.WriteString(";")
				}
				restoreRangeShadows()
				popParamScope()

				if hasClosureDefer {
					out.WriteString("\n            while let Some(f) = __defer_stack.pop() {\n")
					out.WriteString("                f();\n")
					out.WriteString("            }")
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
				for i, stmt := range funcLit.Body.List {
					if i > 0 {
						out.WriteString("\n        ")
					}
					TranspileStatementSimple(out, stmt, funcLit.Type, fileSet)
					out.WriteString(";")
				}
				if hasClosureDefer {
					out.WriteString("\n        while let Some(f) = __defer_stack.pop() {\n")
					out.WriteString("            f();\n")
					out.WriteString("        }")
				}
			}
		} else {
			// Regular function call
			TranspileCall(out, s.Call)
		}

		out.WriteString(";\n")
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
			out.WriteString("    let _ts_is_nil = false;\n")
			if NeedsConcurrentWrapper() {
				out.WriteString("    let _ts_val: Option<&(dyn Any + Send + Sync)> = Some(_ts_ref.as_ref() as &(dyn Any + Send + Sync));\n")
			} else {
				out.WriteString("    let _ts_val: Option<&dyn Any> = Some(_ts_ref.as_ref() as &dyn Any);\n")
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
			out.WriteString("    let _ts_is_nil = _ts_guard.as_ref().is_none();\n")
			out.WriteString("    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {\n")
			out.WriteString("        let mut __any = __v.as_ref() as &dyn Any;\n")
			if NeedsConcurrentWrapper() {
				out.WriteString("        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {\n")
			} else {
				out.WriteString("        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {\n")
			}
			out.WriteString("            __any = __boxed.as_ref() as &dyn Any;\n")
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
			out.WriteString("    drop(_ts_guard);\n")
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
							writeErrorHandleCopy(out, func() {
								out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
								out.WriteString(handleType)
								out.WriteString(">()).unwrap()")
							})
						} else if handleType, ok := typeSwitchCaseAnyHandleRustType(typeInfo, caseClause.List[0], typeSwitchSubjectType); ok {
							// The box holds the handle itself, so the case
							// variable shares it with every other holder.
							out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
							out.WriteString(handleType)
							out.WriteString(">()).unwrap().clone()")
						} else {
							WriteWrapperPrefix(out)
							out.WriteString("_ts_val.and_then(|__v| __v.downcast_ref::<")
							out.WriteString(rustType)
							out.WriteString(">()).unwrap().clone()")
							WriteWrapperSuffix(out)
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
				out.WriteString("        drop(_ts_guard);\n")
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
		case *ast.SwitchStmt, *ast.TypeSwitchStmt:
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
