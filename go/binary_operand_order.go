package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"go/types"
	"sort"
	"strings"
)

// Go evaluates the calls in an expression before the plain operands that
// sit beside them: in `s[0] + bump(s)` the gc compiler runs bump and then
// reads s[0], so a write bump makes to s[0] is seen. The Rust of the left
// operand borrowed s while bump ran, which panicked with "RefCell already
// borrowed" in a program without goroutines (DEV-224), and read s[0] before
// the write in one with them.
//
// writeBinaryWithRightOperandFirst writes such an expression as a block
// that evaluates the right operand into a temporary first and then applies
// the operator to the left operand and the temporary.

// binaryRightOperandTemps holds the binary expressions being written with
// their right operand in a temporary, so that writing one again goes the
// usual way.
var binaryRightOperandTemps = map[*ast.BinaryExpr]bool{}

var binaryRightOperandTempCount int

// binaryNeedsRightOperandFirst reports whether e's right operand has a call
// and its left operand has none but reads a variable, a field or an element
// whose value the call can change.
func binaryNeedsRightOperandFirst(e *ast.BinaryExpr) bool {
	if e == nil || binaryRightOperandTemps[e] {
		return false
	}
	switch e.Op {
	case token.LAND, token.LOR:
		// The right operand may not run at all.
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || typeInfo.pkg == nil {
		return false
	}
	if isConstantExpression(e) || isConstantExpression(e.X) {
		return false
	}
	if !binaryOperandHasCall(e.Y) || binaryOperandHasCall(e.X) || !binaryOperandReadsStorage(e.X) {
		return false
	}
	// The temporary is a variable of the right operand's type; keep to
	// the types whose values every binary lowering reads from a variable.
	return binaryOperandTypeIsBasic(typeInfo.GetType(e.Y)) && binaryOperandTypeIsBasic(typeInfo.GetType(e.X))
}

func binaryOperandTypeIsBasic(typ types.Type) bool {
	if typ == nil {
		return false
	}
	basic, ok := types.Unalias(typ).(*types.Basic)
	if !ok {
		return false
	}
	return basic.Info()&(types.IsNumeric|types.IsString|types.IsBoolean) != 0 && basic.Info()&types.IsUntyped == 0 && basic.Info()&types.IsComplex == 0
}

// binaryOperandHasCall reports whether expr calls a function other than a
// builtin that only reads, such as len, or converts a value.
func binaryOperandHasCall(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	found := false
	ast.Inspect(expr, func(node ast.Node) bool {
		if found {
			return false
		}
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.CallExpr:
			if typeInfo != nil && typeInfo.IsTypeConversion(n) {
				return true
			}
			if ident, ok := unwrapParens(n.Fun).(*ast.Ident); ok && typeInfo != nil && typeInfo.info != nil {
				if _, isBuiltin := typeInfo.info.Uses[ident].(*types.Builtin); isBuiltin {
					return true
				}
			}
			found = true
			return false
		case *ast.UnaryExpr:
			if n.Op == token.ARROW {
				// A receive is ordered like a call.
				found = true
				return false
			}
		}
		return true
	})
	return found
}

// binaryOperandReadsStorage reports whether expr reads storage that a call
// can change: an element, a field or a pointer's target, a package
// variable, or a local variable that a closure captures or whose address is
// taken. A call cannot reach any other local variable.
func binaryOperandReadsStorage(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	found := false
	ast.Inspect(expr, func(node ast.Node) bool {
		if found {
			return false
		}
		switch n := node.(type) {
		case *ast.Ident:
			if v, ok := typeInfo.info.Uses[n].(*types.Var); ok && varReachableByCalls(typeInfo, v) {
				found = true
			}
		case *ast.IndexExpr, *ast.StarExpr:
			found = true
		case *ast.SelectorExpr:
			if selection, ok := typeInfo.info.Selections[n]; ok && selection.Kind() == types.FieldVal {
				found = true
			}
		}
		return !found
	})
	return found
}

// callReachableVarsByInfo holds, for each types.Info, the local variables
// that a call can change. A key keeps its types.Info alive, so a new one is
// never found under the address of an old one.
var callReachableVarsByInfo = map[*types.Info]map[*types.Var]bool{}

func varReachableByCalls(typeInfo *TypeInfo, v *types.Var) bool {
	if v.Pkg() == nil || v.Parent() == nil || v.Parent() == v.Pkg().Scope() || v.IsField() {
		// A package variable, or one this analysis cannot place.
		return true
	}
	vars, ok := callReachableVarsByInfo[typeInfo.info]
	if !ok {
		vars = collectCallReachableVars(typeInfo.info)
		callReachableVarsByInfo[typeInfo.info] = vars
	}
	return vars[v]
}

// collectCallReachableVars returns the local variables that a call can
// change: those a function literal uses from outside it, and those whose
// address is taken, by & or by calling a pointer method.
func collectCallReachableVars(info *types.Info) map[*types.Var]bool {
	vars := map[*types.Var]bool{}
	var literals []*ast.FuncLit
	for expr := range info.Types {
		switch e := expr.(type) {
		case *ast.FuncLit:
			literals = append(literals, e)
		case *ast.UnaryExpr:
			if e.Op == token.AND {
				if v := rootLocalVar(info, e.X); v != nil {
					vars[v] = true
				}
			}
		}
	}
	for sel, selection := range info.Selections {
		if selectionTakesReceiverAddress(selection) {
			if v := rootLocalVar(info, sel.X); v != nil {
				vars[v] = true
			}
		}
	}
	if len(literals) == 0 {
		return vars
	}
	// Literals nest, so the innermost one around a position is the last
	// one that starts before it and has not ended.
	sort.Slice(literals, func(i, j int) bool { return literals[i].Pos() < literals[j].Pos() })
	for ident, obj := range info.Uses {
		v, ok := obj.(*types.Var)
		if !ok || v.IsField() || vars[v] {
			continue
		}
		pos := ident.Pos()
		i := sort.Search(len(literals), func(i int) bool { return literals[i].Pos() > pos }) - 1
		for ; i >= 0; i-- {
			if pos < literals[i].End() {
				break
			}
		}
		if i < 0 {
			continue
		}
		if literal := literals[i]; v.Pos() < literal.Pos() || v.Pos() >= literal.End() {
			vars[v] = true
		}
	}
	return vars
}

// selectionTakesReceiverAddress reports whether selection is a pointer
// method selected on an addressable value, which Go calls with the value's
// address.
func selectionTakesReceiverAddress(selection *types.Selection) bool {
	if selection == nil || selection.Kind() != types.MethodVal {
		return false
	}
	sig, ok := selection.Obj().Type().(*types.Signature)
	if !ok || sig.Recv() == nil {
		return false
	}
	if _, ptrRecv := types.Unalias(sig.Recv().Type()).(*types.Pointer); !ptrRecv {
		return false
	}
	_, recvIsPointer := types.Unalias(selection.Recv()).Underlying().(*types.Pointer)
	return !recvIsPointer
}

// rootLocalVar returns the variable that x.f.g or x[i] is stored in, when x
// is a variable and no pointer is followed on the way.
func rootLocalVar(info *types.Info, expr ast.Expr) *types.Var {
	for {
		switch e := unwrapParens(expr).(type) {
		case *ast.Ident:
			v, _ := info.Uses[e].(*types.Var)
			return v
		case *ast.SelectorExpr:
			selection, ok := info.Selections[e]
			if !ok || selection.Kind() != types.FieldVal || selection.Indirect() {
				return nil
			}
			expr = e.X
		case *ast.IndexExpr:
			if _, isArray := types.Unalias(info.TypeOf(e.X)).Underlying().(*types.Array); !isArray {
				return nil
			}
			expr = e.X
		default:
			return nil
		}
	}
}

// writeBinaryWithRightOperandFirst writes e as
// { let __go_rhs_N = <right>; let __go_binary_value = <left> op __go_rhs_N; __go_binary_value }.
func writeBinaryWithRightOperandFirst(out *strings.Builder, e *ast.BinaryExpr, ctx ExprContext) {
	typeInfo := GetTypeInfo()
	rightType := typeInfo.GetType(e.Y)
	binaryRightOperandTempCount++
	name := fmt.Sprintf("__go_rhs_%d", binaryRightOperandTempCount)
	use := &ast.Ident{NamePos: e.Y.Pos(), Name: name}
	v := types.NewVar(e.Y.Pos(), typeInfo.pkg, name, rightType)
	typeInfo.info.Uses[use] = v
	typeInfo.info.Types[use] = types.TypeAndValue{Type: rightType}
	if vt := GetVarTable(); vt != nil {
		vt.PushScope()
		defer vt.PopScope()
		vt.Register(name, &VarInfo{WrapLevel: WrapFull, RustType: goTypesTypeToRust(rightType), Source: SourceLocal, GoType: rightType})
	}

	out.WriteString("{ let ")
	out.WriteString(name)
	out.WriteString(" = ")
	// ReturnsWrappedValue does not look through parentheses; the Rust of
	// (f()) is the Rust of f().
	if typeInfo.ReturnsWrappedValue(unwrapParens(e.Y)) {
		TranspileExpression(out, e.Y)
	} else {
		WriteWrapperPrefix(out)
		if !writeLenCapCallArgumentForExpectedType(out, unwrapParens(e.Y), rightType) {
			// len and cap give a usize; the temporary is a Go int.
			TranspileExpression(out, e.Y)
		}
		WriteWrapperSuffix(out)
	}
	out.WriteString("; ")

	right := e.Y
	e.Y = use
	binaryRightOperandTemps[e] = true
	// The value goes into a let as well: the guards a block's tail
	// expression takes would outlive the temporary they borrow.
	out.WriteString("let __go_binary_value = ")
	TranspileExpressionContext(out, e, ctx)
	delete(binaryRightOperandTemps, e)
	e.Y = right
	out.WriteString("; __go_binary_value }")
}
