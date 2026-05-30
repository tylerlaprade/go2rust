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

func TestPackageGlobalInterfaceHandleAssignmentWritesSlot(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Object interface {
	Name() string
}

var sink Object
var ch chan int

func lookup() Object {
	return nil
}

func init() {
	sink = lookup()
}
`)

	if strings.Contains(rust, "sink = lookup().clone()") {
		t.Fatalf("package-global interface assignment should not replace the global handle:\n%s", rust)
	}
	if !strings.Contains(rust, "*sink.lock().unwrap() = (*__iface_guard).clone()") {
		t.Fatalf("package-global interface assignment should write through the global slot:\n%s", rust)
	}
}

func TestPackageGlobalPointerShortDeclCopiesStoredHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Scope struct{}

var Universe *Scope
var ch chan int

func use() {
	scope := Universe
	_ = scope
}
`)

	if strings.Contains(rust, "let mut scope = Universe.clone()") {
		t.Fatalf("short declaration from package-global pointer should not clone the global slot:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut scope = (*Universe.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("short declaration from package-global pointer should clone the stored pointer handle:\n%s", rust)
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

func TestPackageGlobalErrorSelectorInitMovesHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "io/fs"

var Skip error = fs.SkipDir
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "Some(fs::SkipDir())") {
		t.Fatalf("package global error selector should move from the error handle, not wrap the handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __rhs_holder = fs::SkipDir().clone()") || !strings.Contains(rust, "*Skip.borrow_mut() = new_val") {
		t.Fatalf("package global error selector should move the option payload into the global slot:\n%s", rust)
	}
}

func TestPackageGlobalNamedIntegerErrorConstInitBoxesNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Errno uintptr

const EINVAL = Errno(22)

func (e Errno) Error() string { return "" }

var errEINVAL error = EINVAL
`)

	if strings.Contains(rust, "Some(E_I_N_V_A_L)") {
		t.Fatalf("package global error const init should not store the raw const:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(Box::new(Errno(") {
		t.Fatalf("package global error const init should box the named error value:\n%s", rust)
	}
}

func TestPackageGlobalExplicitNilErrorInitStaysNone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

var errGlobal error = nil
`)

	if strings.Contains(rust, "Some(nil)") {
		t.Fatalf("package global error nil init should not store nil as a payload:\n%s", rust)
	}
	if !strings.Contains(rust, "*errGlobal.borrow_mut() = None;") &&
		!strings.Contains(rust, "*errGlobal.lock().unwrap() = None;") {
		t.Fatalf("package global error nil init should leave the slot empty:\n%s", rust)
	}
}

func TestPackageGlobalFunctionSelectorInitBoxesFunctionObject(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "os"

var lstat = os.Lstat
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "Some(os::lstat);") {
		t.Fatalf("package global function selector should box the function object:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(Box::new(os::lstat));") {
		t.Fatalf("package global function selector should initialize with Box::new:\n%s", rust)
	}
}
