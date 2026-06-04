package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestStubBackedInterfaceDefaultBindingCallArgumentBoxesBareValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"go/ast"
	"go/token"
)

type positioner interface {
	Pos() token.Pos
}

func report(pos positioner, value any) {}

func walk(specs []ast.Spec) {
	for _, spec := range specs {
		switch spec := spec.(type) {
		default:
			report(spec, spec)
		}
	}
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

	if strings.Contains(rust, "let __arg_guard = __arg_holder.borrow()") ||
		strings.Contains(rust, "let __arg_guard = __arg_holder.lock().unwrap()") {
		t.Fatalf("stub-backed interface default binding should not be boxed through a wrapper handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let _ts_subject = spec.clone();") {
		t.Fatalf("stub-backed interface type switch should not move the original bare interface value:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(spec.clone()) as Box<dyn positioner") {
		t.Fatalf("stub-backed interface default binding should box the bare interface value as the local interface:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(spec.clone()) as Box<dyn Any") {
		t.Fatalf("stub-backed interface default binding should box the bare interface value as any:\n%s", rust)
	}
}
