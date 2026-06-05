package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestStubBackedImportedOrderedConstraintUsesPartialOrdOnly(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "sort.go", `package slices

import "cmp"

func Sort[E cmp.Ordered](x []E) {}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"slices": "slices"})

	if strings.Contains(rust, "cmp_Ordered") {
		t.Fatalf("stub-backed ordered constraint should not emit bridge struct as a trait bound:\n%s", rust)
	}
	if !strings.Contains(rust, "E: PartialOrd") {
		t.Fatalf("stub-backed ordered constraint should lower to Rust ordering bounds:\n%s", rust)
	}
}
