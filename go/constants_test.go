package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func firstEquality(t *testing.T, file *ast.File) *ast.BinaryExpr {
	t.Helper()

	var comparison *ast.BinaryExpr
	ast.Inspect(file, func(node ast.Node) bool {
		if expr, ok := node.(*ast.BinaryExpr); ok && expr.Op == token.EQL {
			comparison = expr
			return false
		}
		return true
	})
	if comparison == nil {
		t.Fatal("comparison not found")
	}
	return comparison
}

func TestConstExpressionForByteBinaryPeer(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

const opType = '.'

func main() {
	var path []byte
	_ = path[0] == opType
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

	comparison := firstEquality(t, file)

	var out strings.Builder
	if !writeConstExpressionForBinaryPeer(&out, comparison.Y, comparison.X) {
		t.Fatal("const expression was not converted for byte peer")
	}
	if got, want := out.String(), "OP_TYPE as u8"; got != want {
		t.Fatalf("converted const = %q, want %q", got, want)
	}
}

func TestConstExpressionForNamedBytePeerUsesNamedConversion(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type VarKind uint8

const LocalVar VarKind = 1

func main() {
	var kind VarKind
	_ = kind == LocalVar
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

	comparison := firstEquality(t, file)

	var out strings.Builder
	if writeConstExpressionForBinaryPeer(&out, comparison.Y, comparison.X) {
		t.Fatalf("named byte-like peer should not use bare u8 conversion, got %q", out.String())
	}
}
