package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strings"
	"testing"
)

func TestBasicMapConversionRequiresTypeInformation(t *testing.T) {
	previous := GetTypeInfo()
	SetTypeInfo(nil)
	t.Cleanup(func() { SetTypeInfo(previous) })
	call := &ast.CallExpr{Fun: ast.NewIdent("uint16"), Args: []ast.Expr{&ast.BasicLit{Kind: token.INT, Value: "7"}}}
	var out strings.Builder
	writeWrappedMapValue(&out, call, nil, types.Typ[types.Uint16])
	if !strings.Contains(out.String(), "type info required for map element call") {
		t.Fatalf("missing type information produced %s", out.String())
	}
}
