package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestTranspileNilSliceDeclarationPreservesNilSemantics(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func main() {
	var s4 []int
	s5 := []int{}
	println(s4 == nil, s5 == nil)
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)

	if !strings.Contains(rust, "let mut s4: Rc<RefCell<Option<Vec<i32>>>> = Rc::new(RefCell::new(None));") {
		t.Fatalf("nil slice declaration should transpile to None, got:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut s5 = Rc::new(RefCell::new(Some(Vec::<Rc<RefCell<Option<i32>>>>::new())));") {
		t.Fatalf("empty slice literal should remain a wrapped empty Vec, got:\n%s", rust)
	}
}

func TestTranspileAppendToNilSliceInitializesStorage(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func main() {
	var s []int
	s = append(s, 1)
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)

	if !strings.Contains(rust, "get_or_insert_with(Vec::new).push") {
		t.Fatalf("append to nil slice should initialize storage before push, got:\n%s", rust)
	}
}
