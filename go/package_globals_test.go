package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestPackageGlobalConstInitCastsToGlobalType(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

const CacheLinePadSize = 128

var CacheLineSize uintptr = CacheLinePadSize
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, "Some(CACHE_LINE_PAD_SIZE as usize)") {
		t.Fatalf("package global const initializer should cast to uintptr target type:\n%s", rust)
	}
}

func TestPackageGlobalInterfaceZeroValueIsNil(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

var sink any
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "Some(Default::default())") {
		t.Fatalf("package global interface zero value must not default a trait object:\n%s", rust)
	}
	if !strings.Contains(rust, " = None;") {
		t.Fatalf("package global interface zero value should initialize to nil:\n%s", rust)
	}
}

func TestPackageGlobalAnyAssignmentBoxesGenericValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

var sink any

func store[T any](x T) {
	sink = x
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
	if strings.Contains(rust, "sink = x.clone()") {
		t.Fatalf("assignment to package-global any should not replace the global handle:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn store<T: Any + Clone + 'static>") {
		t.Fatalf("generic any assignment should bound the Rust type parameter for boxing:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = Box::new(") || !strings.Contains(rust, "*sink.borrow_mut() = Some(new_val)") {
		t.Fatalf("assignment to package-global any should box into the global slot:\n%s", rust)
	}
}

func TestPackageGlobalBareScalarTupleSlotAssignsDirectly(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

var enabled, _ = parseEnabled()

func parseEnabled() (bool, error) {
	return true, nil
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
	if strings.Contains(rust, "__go_pkg_init_0.borrow().as_ref().unwrap()") ||
		strings.Contains(rust, "__go_pkg_init_0.lock().unwrap().as_ref().unwrap()") {
		t.Fatalf("bare-scalar tuple slot should not be treated as a wrapped handle:\n%s", rust)
	}
	if !strings.Contains(rust, "*enabled.borrow_mut() = Some(__go_pkg_init_0);") &&
		!strings.Contains(rust, "*enabled.lock().unwrap() = Some(__go_pkg_init_0);") {
		t.Fatalf("bare-scalar tuple slot should assign the temp directly into the global slot:\n%s", rust)
	}
}
