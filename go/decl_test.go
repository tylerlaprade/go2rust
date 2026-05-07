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
