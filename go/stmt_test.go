package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestTypeSwitchDropsSubjectGuardBeforeCaseBody(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func isString(v interface{}) bool {
	switch v.(type) {
	case string:
		return true
	}
	return false
}

func classify(v interface{}) bool {
	switch v.(type) {
	case string:
		return isString(v)
	default:
		return false
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

	callIndex := strings.Index(rust, "return is_string(")
	if callIndex < 0 {
		t.Fatalf("generated Rust did not contain reentrant type switch call:\n%s", rust)
	}
	if dropIndex := strings.LastIndex(rust[:callIndex], "drop(_ts_guard);"); dropIndex < 0 {
		t.Fatalf("type switch case body should release subject guard before reusing subject:\n%s", rust)
	}
}

func TestTypeSwitchUsesSyntaxCaseTypeWithoutTypeInfo(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func classify(v interface{}) string {
	switch v.(type) {
	default:
		return "other"
	case int:
		return "int"
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	rust, _, _ := Transpile(file, fset, nil)

	if strings.Contains(rust, "Type information required for type switch case") {
		t.Fatalf("type switch case should use syntax fallback without type info:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<i32>()") {
		t.Fatalf("type switch case did not lower builtin int syntax:\n%s", rust)
	}
}

func TestUnsafeSizeofUsesSyntaxVarTypeWithoutTypeInfo(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "unsafe"

func size() uintptr {
	var ptr uintptr
	return unsafe.Sizeof(ptr)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	rust, _, _ := Transpile(file, fset, nil)

	if strings.Contains(rust, "Type information unavailable for unsafe.Sizeof") {
		t.Fatalf("unsafe.Sizeof should use syntax var type fallback without type info:\n%s", rust)
	}
	if !strings.Contains(rust, "std::mem::size_of::<usize>()") {
		t.Fatalf("unsafe.Sizeof did not lower uintptr var syntax:\n%s", rust)
	}
}

func TestTrailingEmptyStmtAfterTerminatingSwitchSkipsFinalDefer(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func pick(v int) (res int) {
	if v < 0 {
		defer func() {}()
	}
	switch v {
	default:
		return 1
	case 0:
		return 0
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	fn := file.Decls[0].(*ast.FuncDecl)
	fn.Body.List = append(fn.Body.List, &ast.EmptyStmt{})

	rust, _, _ := Transpile(file, fset, nil)

	if strings.Contains(rust, "Unhandled statement type") {
		t.Fatalf("empty statement should not emit a TODO:\n%s", rust)
	}
	if got := strings.Count(rust, "while let Some(f) = __defer_stack.pop()"); got != 2 {
		t.Fatalf("terminating switch with trailing empty statement should not emit final defer drain, got %d drains:\n%s", got, rust)
	}
}

func TestNoTypeInfoConcurrentPointerMapCommaOkKeepsSliceHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"fmt"
	"go/types"
)

type scope struct {
	id int
}

func lookup(m map[*scope][]types.Object, s *scope) []types.Object {
	objs, ok := m[s]
	if !ok {
		objs = make([]types.Object, 1)
		m[s] = objs
	}
	return objs
}

func forceConcurrent() {
	go func() {}()
	if false {
		fmt.Println(lookup(nil, nil))
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}

	prevDetector := GetConcurrencyDetector()
	detector := NewConcurrencyDetector()
	detector.AnalyzeFile(file)
	SetConcurrencyDetector(detector)
	defer SetConcurrencyDetector(prevDetector)

	rust, _, _ := Transpile(file, fset, nil)

	if strings.Contains(rust, "type information required") || strings.Contains(rust, "Cannot determine if map") {
		t.Fatalf("map comma-ok should use syntax fallback without type info:\n%s", rust)
	}
	if !strings.Contains(rust, "match __map.get(&GoLocalPtrKey::new(s.clone()))") {
		t.Fatalf("map comma-ok should use pointer-key syntax fallback:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(v) => (v.clone(),") || !strings.Contains(rust, "None => (Default::default(),") {
		t.Fatalf("map comma-ok should keep the stored slice handle:\n%s", rust)
	}
	if !strings.Contains(rust, "vec![Default::default(); (1) as usize]") {
		t.Fatalf("make([]types.Object, n) should use selector element zero value without type info:\n%s", rust)
	}
	if !strings.Contains(rust, "let __map_key = GoLocalPtrKey::new(s.clone()); let __map_value = objs.clone();") {
		t.Fatalf("map assignment should use pointer-key syntax fallback and store the slice handle:\n%s", rust)
	}
	if !strings.Contains(rust, "format_slice(&lookup(") {
		t.Fatalf("fmt.Println should use syntax-known slice return without type info:\n%s", rust)
	}
	if strings.Contains(rust, "s as usize") || strings.Contains(rust, "s) as usize") {
		t.Fatalf("map assignment fell through to slice indexing:\n%s", rust)
	}
}

func TestLocalMapKeyRustTypeReportsTrackedPointerKey(t *testing.T) {
	prevCollections := localCollectionKinds
	prevMapKeys := localMapKeyRustTypes
	defer func() {
		localCollectionKinds = prevCollections
		localMapKeyRustTypes = prevMapKeys
	}()

	localCollectionKinds = map[string]string{"m": "map"}
	localMapKeyRustTypes = map[string]string{"m": "GoLocalPtrKey<scope>"}

	keyType, ok := localMapKeyRustType(ast.NewIdent("m"))
	if !ok || keyType != "GoLocalPtrKey<scope>" {
		t.Fatalf("localMapKeyRustType() = (%q, %v), want tracked pointer key", keyType, ok)
	}
}
