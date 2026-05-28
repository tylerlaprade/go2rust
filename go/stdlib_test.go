package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestFmtFprintfPointerVerbUsesInterfaceHandleAddress(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"fmt"
	"strings"
)

type Node interface {
	Pos() int
}

type CommentMap map[Node][]string

func Write(buf *strings.Builder, cmap CommentMap, s string) {
	for node := range cmap {
		fmt.Fprintf(buf, "\t%p  %20s:  %s\n", node, s, "ok")
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

	if !strings.Contains(rust, `format!("\t{:p}  {:>20}:  {}\n"`) {
		t.Fatalf("%%p should lower to Rust pointer formatting:\n%s", rust)
	}
	if !strings.Contains(rust, "::as_ptr(&node)") {
		t.Fatalf("%%p for a wrapped interface handle should format the handle address:\n%s", rust)
	}
	if strings.Contains(rust, `format!("\t{}  {:>20}:  {}\n", node`) {
		t.Fatalf("%%p should not display-format the wrapped interface handle:\n%s", rust)
	}
}

func TestBuiltinPrintUsesRustStderrMacro(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func warn(field string) {
	print("missing ", field, "\n")
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
	if !strings.Contains(rust, `eprint!("{}{}{}"`) {
		t.Fatalf("builtin print should emit Rust stderr macro without inserted spaces:\n%s", rust)
	}
	if strings.Contains(rust, "print(Rc::") || strings.Contains(rust, "print(Arc::") {
		t.Fatalf("builtin print must not emit a call to a nonexistent Rust print function:\n%s", rust)
	}
}

func TestCopyToSliceTypeAssertionUsesBareDestination(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func fill(v any, src []int) int {
	return copy(v.([]int), src)
}
`)

	if strings.Contains(rust, "downcast_ref::<Vec<i32>>().expect(\"type assertion failed\").clone()\n        } else") &&
		(strings.Contains(rust, "}).borrow()") || strings.Contains(rust, "}).lock()")) {
		t.Fatalf("copy destination from slice type assertion should not be borrowed as a wrapped handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut _dst =") {
		t.Fatalf("copy destination from slice type assertion should use a mutable bare Vec temp:\n%s", rust)
	}
}
