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
