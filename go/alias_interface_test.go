package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestLocalAliasToSourceMappedInterfaceCompositeLiteralUsesAliasedHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/ast"

func collect(a, b ast.Expr) [2]ast.Expr {
	type Expr = ast.Expr
	return [2]Expr{a, b}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/ast": "go_ast"})

	if strings.Contains(rust, "Box<dyn Expr") {
		t.Fatalf("local alias to source-mapped interface should not be treated as a Rust trait object:\n%s", rust)
	}
	if !strings.Contains(rust, "type Expr = Rc<RefCell<Option<Box<dyn go_ast::") {
		t.Fatalf("local alias should preserve the aliased source interface handle type:\n%s", rust)
	}
}

func TestLocalAliasToStubBackedExternalInterfaceCompositeLiteralUsesAliasedHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/ast"

func collect(a, b ast.Expr) [2]ast.Expr {
	type Expr = ast.Expr
	return [2]Expr{a, b}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)

	if strings.Contains(rust, "Box<dyn Expr") {
		t.Fatalf("local alias to stub-backed external interface should not be treated as a Rust trait object:\n%s", rust)
	}
	if !strings.Contains(rust, "type Expr = Rc<RefCell<Option<ast_Expr>>>") {
		t.Fatalf("local alias should preserve the aliased external interface handle type:\n%s", rust)
	}
}
