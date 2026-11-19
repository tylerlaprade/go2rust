package main

import (
	"fmt"
	"go/ast"
	"go/importer"
	"go/parser"
	"go/token"
	"go/types"
	"strings"
	"testing"
	"unsafe"
)

type dynamicTypeNamePoint struct{ X, Y int }

type dynamicTypeNameList []int

type dynamicTypeNameHandler func(int) string

type dynamicTypeNamePair[K comparable, V any] struct {
	Key   K
	Value V
}

// The declarations the cases below are type-checked against. They repeat the
// test's own types under the same names in a package also called main, so
// reflect and go/types are describing the same thing.
const dynamicTypeNameDecls = `package main

import "unsafe"

type dynamicTypeNamePoint struct{ X, Y int }

type dynamicTypeNameList []int

type dynamicTypeNameHandler func(int) string

type dynamicTypeNamePair[K comparable, V any] struct {
	Key   K
	Value V
}

var _ unsafe.Pointer
`

func dynamicTypeNameCaseTypes(t *testing.T, typeExprs []string) []types.Type {
	t.Helper()
	var src strings.Builder
	src.WriteString(dynamicTypeNameDecls)
	for i, typeExpr := range typeExprs {
		fmt.Fprintf(&src, "var v%d %s\n", i, typeExpr)
	}
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "cases.go", src.String(), 0)
	if err != nil {
		t.Fatalf("parse: %v", err)
	}
	// The only import is unsafe, which the default importer resolves itself.
	pkg, err := (&types.Config{Importer: importer.Default()}).Check("main", fset, []*ast.File{file}, nil)
	if err != nil {
		t.Fatalf("type check: %v", err)
	}
	result := make([]types.Type, len(typeExprs))
	for i := range typeExprs {
		obj := pkg.Scope().Lookup(fmt.Sprintf("v%d", i))
		if obj == nil {
			t.Fatalf("v%d not declared", i)
		}
		result[i] = obj.Type()
	}
	return result
}

// TestGoDynamicTypeNameMatchesReflect checks the name the translator bakes in
// for %T against what fmt prints for a real value of the same type.
func TestGoDynamicTypeNameMatchesReflect(t *testing.T) {
	cases := []struct {
		typeExpr string
		value    any
	}{
		{"int", int(0)},
		{"int8", int8(0)},
		{"byte", byte(0)},
		{"rune", rune(0)},
		{"uint64", uint64(0)},
		{"uintptr", uintptr(0)},
		{"float32", float32(0)},
		{"float64", float64(0)},
		{"complex128", complex128(0)},
		{"string", ""},
		{"bool", false},
		{"unsafe.Pointer", unsafe.Pointer(nil)},
		{"[]int", []int(nil)},
		{"[]byte", []byte(nil)},
		{"[3]string", [3]string{}},
		{"[][]float64", [][]float64(nil)},
		{"map[string]int", map[string]int(nil)},
		{"map[string][]int", map[string][]int(nil)},
		{"*int", (*int)(nil)},
		{"dynamicTypeNamePoint", dynamicTypeNamePoint{}},
		{"*dynamicTypeNamePoint", (*dynamicTypeNamePoint)(nil)},
		{"**dynamicTypeNamePoint", (**dynamicTypeNamePoint)(nil)},
		{"[]*dynamicTypeNamePoint", []*dynamicTypeNamePoint(nil)},
		{"map[dynamicTypeNamePoint]bool", map[dynamicTypeNamePoint]bool(nil)},
		{"dynamicTypeNameList", dynamicTypeNameList(nil)},
		{"dynamicTypeNameHandler", dynamicTypeNameHandler(nil)},
		{"chan int", (chan int)(nil)},
		{"<-chan int", (<-chan int)(nil)},
		{"chan<- string", (chan<- string)(nil)},
		{"chan (<-chan int)", (chan (<-chan int))(nil)},
		{"func()", (func())(nil)},
		{"func(int, string) (int, error)", (func(int, string) (int, error))(nil)},
		{"func(a int, b ...string) bool", (func(int, ...string) bool)(nil)},
		{"func(func(int) int) []int", (func(func(int) int) []int)(nil)},
		{"[]interface{}", []interface{}(nil)},
		{"map[string]any", map[string]any(nil)},
		{"[]error", []error(nil)},
	}
	typeExprs := make([]string, len(cases))
	for i, tc := range cases {
		typeExprs[i] = tc.typeExpr
	}
	for i, typ := range dynamicTypeNameCaseTypes(t, typeExprs) {
		want := fmt.Sprintf("%T", cases[i].value)
		got, ok := goDynamicTypeName(typ)
		if !ok {
			t.Errorf("%s: no name; fmt prints %q", cases[i].typeExpr, want)
			continue
		}
		if got != want {
			t.Errorf("%s: got %q, fmt prints %q", cases[i].typeExpr, got, want)
		}
	}
}

// TestGoDynamicTypeNameDeclinesUnsupportedSpellings pins the types the helper
// refuses to name. Each has a reflect spelling it does not reproduce, and a
// wrong name is worse than none.
func TestGoDynamicTypeNameDeclinesUnsupportedSpellings(t *testing.T) {
	typeExprs := []string{
		"struct{ X int }",
		"interface{ String() string }",
		"dynamicTypeNamePair[string, int]",
		"[]dynamicTypeNamePair[string, int]",
		"func(struct{ X int })",
	}
	for i, typ := range dynamicTypeNameCaseTypes(t, typeExprs) {
		if got, ok := goDynamicTypeName(typ); ok {
			t.Errorf("%s: named %q, want no name", typeExprs[i], got)
		}
	}
}

func TestTypeMentionsTypeParam(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "generic.go", `package main

func generic[T any](plain []int, elems []T, byKey map[string]*T, fn func(T) int, ch chan T, fields []struct{ V T }, plainFields []struct{ V int }) {
}
`, 0)
	if err != nil {
		t.Fatalf("parse: %v", err)
	}
	pkg, err := (&types.Config{}).Check("main", fset, []*ast.File{file}, nil)
	if err != nil {
		t.Fatalf("type check: %v", err)
	}
	params := pkg.Scope().Lookup("generic").Type().(*types.Signature).Params()
	want := map[string]bool{
		"plain": false, "elems": true, "byKey": true, "fn": true, "ch": true,
		"fields": true, "plainFields": false,
	}
	for i := 0; i < params.Len(); i++ {
		param := params.At(i)
		if got := typeMentionsTypeParam(param.Type()); got != want[param.Name()] {
			t.Errorf("%s (%s): got %v, want %v", param.Name(), param.Type(), got, want[param.Name()])
		}
	}
}
