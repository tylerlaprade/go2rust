package main

import (
	"go/ast"
	"go/token"
	"strings"
	"testing"
)

func TestTranspileFunctionWithoutBodyDoesNotPanic(t *testing.T) {
	var out strings.Builder
	fn := &ast.FuncDecl{
		Name: ast.NewIdent("externalFunc"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{},
			Results: &ast.FieldList{
				List: []*ast.Field{{Type: ast.NewIdent("int")}},
			},
		},
	}

	TranspileFunction(&out, fn, token.NewFileSet(), nil)

	got := out.String()
	if !strings.Contains(got, "pub fn external_func() -> Rc<RefCell<Option<i32>>>") {
		t.Fatalf("missing function signature in:\n%s", got)
	}
	if !strings.Contains(got, "unimplemented!(\"Go function declaration has no body\")") {
		t.Fatalf("missing bodyless function fallback in:\n%s", got)
	}
}

func TestTranspileConstDeclUsesPackageVisibility(t *testing.T) {
	var out strings.Builder
	TranspileConstDecl(&out, &ast.GenDecl{
		Tok: token.CONST,
		Specs: []ast.Spec{&ast.ValueSpec{
			Names: []*ast.Ident{ast.NewIdent("Future"), ast.NewIdent("privateValue")},
			Values: []ast.Expr{
				&ast.BasicLit{Kind: token.STRING, Value: `""`},
				&ast.BasicLit{Kind: token.INT, Value: "1"},
			},
		}},
	})

	got := out.String()
	if !strings.Contains(got, `pub const FUTURE: &'static str = "";`) {
		t.Fatalf("exported package const should be public, got:\n%s", got)
	}
	if !strings.Contains(got, `pub(crate) const PRIVATE_VALUE: i32 = 1;`) {
		t.Fatalf("private package const should be crate-visible, got:\n%s", got)
	}
}

func TestMethodMutatesReceiverDetectsReceiverAssignments(t *testing.T) {
	readOnly := &ast.FuncDecl{
		Body: &ast.BlockStmt{List: []ast.Stmt{
			&ast.ReturnStmt{Results: []ast.Expr{
				&ast.SelectorExpr{X: ast.NewIdent("k"), Sel: ast.NewIdent("name")},
			}},
		}},
	}
	if methodMutatesReceiver(readOnly, "k") {
		t.Fatalf("read-only receiver field access should not require &mut self")
	}

	mutating := &ast.FuncDecl{
		Body: &ast.BlockStmt{List: []ast.Stmt{
			&ast.AssignStmt{
				Lhs: []ast.Expr{&ast.SelectorExpr{X: ast.NewIdent("k"), Sel: ast.NewIdent("name")}},
				Tok: token.ASSIGN,
				Rhs: []ast.Expr{&ast.BasicLit{Kind: token.STRING, Value: `"updated"`}},
			},
		}},
	}
	if !methodMutatesReceiver(mutating, "k") {
		t.Fatalf("receiver field assignment should require &mut self")
	}

	caller := &ast.FuncDecl{
		Name: ast.NewIdent("Update"),
		Recv: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("k")},
			Type:  &ast.StarExpr{X: ast.NewIdent("Thing")},
		}}},
		Body: &ast.BlockStmt{List: []ast.Stmt{
			&ast.ExprStmt{X: &ast.CallExpr{
				Fun: &ast.SelectorExpr{X: ast.NewIdent("k"), Sel: ast.NewIdent("Set")},
			}},
		}},
	}
	mutating.Name = ast.NewIdent("Set")
	mutating.Recv = &ast.FieldList{List: []*ast.Field{{
		Names: []*ast.Ident{ast.NewIdent("k")},
		Type:  &ast.StarExpr{X: ast.NewIdent("Thing")},
	}}}

	previousTypeMethods := currentTypeMethods
	currentTypeMethods = []*ast.FuncDecl{caller, mutating}
	defer func() { currentTypeMethods = previousTypeMethods }()

	if !methodMutatesReceiver(caller, "k") {
		t.Fatalf("receiver call to mutating receiver method should require &mut self")
	}
}

func TestMethodMutatesReceiverUsesPackageMethods(t *testing.T) {
	caller := &ast.FuncDecl{
		Name: ast.NewIdent("Read"),
		Recv: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("k")},
			Type:  &ast.StarExpr{X: ast.NewIdent("Thing")},
		}}},
		Body: &ast.BlockStmt{List: []ast.Stmt{
			&ast.ExprStmt{X: &ast.CallExpr{
				Fun: &ast.SelectorExpr{X: ast.NewIdent("k"), Sel: ast.NewIdent("Set")},
			}},
		}},
	}
	mutating := &ast.FuncDecl{
		Name: ast.NewIdent("Set"),
		Recv: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("k")},
			Type:  &ast.StarExpr{X: ast.NewIdent("Thing")},
		}}},
		Body: &ast.BlockStmt{List: []ast.Stmt{
			&ast.AssignStmt{
				Lhs: []ast.Expr{&ast.SelectorExpr{X: ast.NewIdent("k"), Sel: ast.NewIdent("name")}},
				Tok: token.ASSIGN,
				Rhs: []ast.Expr{&ast.BasicLit{Kind: token.STRING, Value: `"updated"`}},
			},
		}},
	}

	previousTypeMethods := currentTypeMethods
	currentTypeMethods = []*ast.FuncDecl{caller}
	defer func() { currentTypeMethods = previousTypeMethods }()

	previousCtx := GetTranspileContext()
	ctx := &TranspileContext{Package: NewPackageState()}
	ctx.Package.MethodsByType = map[string][]*ast.FuncDecl{"Thing": {caller, mutating}}
	SetTranspileContext(ctx)
	defer SetTranspileContext(previousCtx)

	if !methodMutatesReceiver(caller, "k") {
		t.Fatalf("receiver calls to mutating methods from another file should require &mut self")
	}
}
