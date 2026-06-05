package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestNamedIntegerShiftCountLiteralAnnotatesRustType(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "xorshift.go", `package main

type xorshift uint64

func (r *xorshift) Next() uint64 {
	*r ^= *r << 13
	*r ^= *r >> 7
	return uint64(*r)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	var shifts []*ast.BinaryExpr
	ast.Inspect(file, func(n ast.Node) bool {
		binary, ok := n.(*ast.BinaryExpr)
		if ok && (binary.Op == token.SHL || binary.Op == token.SHR) {
			shifts = append(shifts, binary)
		}
		return true
	})
	if len(shifts) != 2 {
		t.Fatalf("found %d shift expression(s), want 2", len(shifts))
	}

	prevTypeInfo := GetTypeInfo()
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(prevTypeInfo)

	cases := []struct {
		op   token.Token
		want string
	}{
		{op: token.SHL, want: "13i32"},
		{op: token.SHR, want: "7i32"},
	}
	for i, tc := range cases {
		if shifts[i].Op != tc.op {
			t.Fatalf("shift %d op = %s, want %s", i, shifts[i].Op, tc.op)
		}
		var out strings.Builder
		if !writeShiftCountPrimitiveOperand(&out, shifts[i].Y, shifts[i]) {
			t.Fatalf("writeShiftCountPrimitiveOperand() returned false for %s", tc.op)
		}
		if got := out.String(); got != tc.want {
			t.Fatalf("shift count emission = %q, want %q", got, tc.want)
		}
	}
}
