package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

// An assertion to a struct type copies the struct, in each form, and an
// assertion to a type named Vec names the Rust type Vec_.
func TestTypeAssertionToStructCopiesTheValue(t *testing.T) {
	src := `package main

type Vec struct{ X int }

func main() {
	var a interface{} = Vec{1}
	v := a.(Vec)
	w, ok := a.(Vec)
	switch u := a.(type) {
	case Vec:
		u.X = 3
	}
	v.X, w.X = 2, 2
	_ = ok
}
`
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
	if err != nil {
		t.Fatal(err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatal(err)
	}
	rust := transpileParsedRegression(t, file, fset, typeInfo)
	for _, want := range []string{
		`any_val.downcast_ref::<Vec_>().unwrap_or_else(`,
		`"main.Vec")).__go_value_clone()`,
		`Some(typed_val.__go_value_clone())`,
		`_ts_val.and_then(|__v| __v.downcast_ref::<Vec_>()).unwrap().__go_value_clone()`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "downcast_ref::<Vec>()") {
		t.Fatalf("the assertion should name Vec_:\n%s", rust)
	}
}
