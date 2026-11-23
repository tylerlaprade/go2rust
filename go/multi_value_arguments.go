package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"go/types"
)

var multiValueArgumentCounter int

// multiValueArgumentStatement rewrites a statement whose call passes a
// multi-value call as all its arguments, g(f()), such as
// fmt.Println(divmod(7, 3)) or a.add(pair()), into two statements:
// `__go_mv_N_0, __go_mv_N_1 := f()` and the original with g(__go_mv_N_0,
// __go_mv_N_1). Go evaluates f() before g's call, so binding its results
// first keeps the order. The statement is an expression statement, a
// return, or an assignment to plain names whose right side is that call;
// anything else is left alone, since moving f() ahead of other operands
// would change the order.
func multiValueArgumentStatement(stmt ast.Stmt) (ast.Stmt, ast.Stmt, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil, nil, false
	}
	var outer *ast.CallExpr
	var rebuild func(*ast.CallExpr) ast.Stmt
	switch s := stmt.(type) {
	case *ast.ExprStmt:
		call, ok := s.X.(*ast.CallExpr)
		if !ok {
			return nil, nil, false
		}
		outer = call
		rebuild = func(c *ast.CallExpr) ast.Stmt { return &ast.ExprStmt{X: c} }
	case *ast.ReturnStmt:
		if len(s.Results) != 1 {
			return nil, nil, false
		}
		call, ok := s.Results[0].(*ast.CallExpr)
		if !ok {
			return nil, nil, false
		}
		outer = call
		rebuild = func(c *ast.CallExpr) ast.Stmt { return &ast.ReturnStmt{Return: s.Return, Results: []ast.Expr{c}} }
	case *ast.AssignStmt:
		if len(s.Rhs) != 1 || (s.Tok != token.ASSIGN && s.Tok != token.DEFINE) {
			return nil, nil, false
		}
		for _, lhs := range s.Lhs {
			if _, ok := lhs.(*ast.Ident); !ok {
				return nil, nil, false
			}
		}
		call, ok := s.Rhs[0].(*ast.CallExpr)
		if !ok {
			return nil, nil, false
		}
		outer = call
		rebuild = func(c *ast.CallExpr) ast.Stmt {
			return &ast.AssignStmt{Lhs: s.Lhs, TokPos: s.TokPos, Tok: s.Tok, Rhs: []ast.Expr{c}}
		}
	default:
		return nil, nil, false
	}
	if len(outer.Args) != 1 || outer.Ellipsis.IsValid() {
		return nil, nil, false
	}
	inner, ok := outer.Args[0].(*ast.CallExpr)
	if !ok {
		return nil, nil, false
	}
	tuple, ok := typeInfo.info.TypeOf(inner).(*types.Tuple)
	if !ok || tuple.Len() < 2 {
		return nil, nil, false
	}
	if tv, ok := typeInfo.info.Types[outer.Fun]; ok && (tv.IsType() || tv.IsBuiltin()) {
		return nil, nil, false
	}
	// A plain function call whose parameters match the results one for one
	// is expanded where its arguments are written
	// (writeIdentFunctionCallWithExpandedMultiResultArg), and so is
	// time.Unix. This covers the rest: methods, other packages' functions
	// and variadic callees such as fmt.Println.
	if _, _, _, expanded := singleMultiResultCallArgument(outer); expanded {
		if ident, ok := outer.Fun.(*ast.Ident); ok && typeInfo.IsFunction(ident) {
			return nil, nil, false
		}
		if sel, ok := outer.Fun.(*ast.SelectorExpr); ok && sel.Sel.Name == "Unix" {
			if pkg, ok := sel.X.(*ast.Ident); ok && pkg.Name == "time" {
				return nil, nil, false
			}
		}
	}
	multiValueArgumentCounter++
	defs := make([]ast.Expr, tuple.Len())
	uses := make([]ast.Expr, tuple.Len())
	for i := 0; i < tuple.Len(); i++ {
		name := fmt.Sprintf("__go_mv_%d_%d", multiValueArgumentCounter, i)
		v := types.NewVar(outer.Pos(), typeInfo.pkg, name, tuple.At(i).Type())
		def := &ast.Ident{NamePos: outer.Pos(), Name: name}
		use := &ast.Ident{NamePos: outer.Pos(), Name: name}
		typeInfo.info.Defs[def] = v
		typeInfo.info.Uses[use] = v
		typeInfo.info.Types[use] = types.TypeAndValue{Type: v.Type()}
		defs[i] = def
		uses[i] = use
	}
	binding := &ast.AssignStmt{Lhs: defs, TokPos: outer.Pos(), Tok: token.DEFINE, Rhs: []ast.Expr{inner}}
	call := *outer
	call.Args = uses
	if tv, ok := typeInfo.info.Types[outer]; ok {
		typeInfo.info.Types[&call] = tv
	}
	return binding, rebuild(&call), true
}
