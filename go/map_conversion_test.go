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

func TestConvertedMapElementHasOneHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Span int64
const Tick Span = 3
var units = map[string]uint64{"tick": uint64(Tick)}
`)
	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, `__go_map.insert("tick".to_string(),`) {
			if strings.Count(line, "RefCell::new(Some(") != 1 {
				t.Fatalf("converted map element should have one handle: %s", line)
			}
			return
		}
	}
	t.Fatal("missing map element for tick")
}
