package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestExternalStubOrderedTypeParamArgumentUsesRawValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "cmp_arg.go", `package main

import "cmp"

func compareX(x int) int {
	return cmp.Compare(1, x)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"main": "main"})
	if strings.Contains(rust, "cmp::compare(1, x.clone())") {
		t.Fatalf("ordered external stub argument should pass the raw value, not the wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "cmp::compare(1, { let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })") &&
		!strings.Contains(rust, "cmp::compare(1, { let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })") {
		t.Fatalf("ordered external stub argument should unwrap through the established value path:\n%s", rust)
	}
}
