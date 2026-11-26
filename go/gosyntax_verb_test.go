package main

import (
	"bytes"
	"go/ast"
	"go/parser"
	"go/printer"
	"go/token"
	"go/types"
	"strings"
	"testing"
)

// rewriteGoSyntaxSource type-checks a one-file main package, rewrites its
// %#v verbs and returns the result as Go source; ok is false when nothing
// was rewritten.
func rewriteGoSyntaxSource(t *testing.T, source string) (string, bool) {
	t.Helper()
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", source, parser.ParseComments)
	if err != nil {
		t.Fatalf("parse: %v", err)
	}
	files := []*ast.File{file}
	ti, err := NewTypeInfo(files, fset)
	if err != nil {
		t.Fatalf("type-check: %v", err)
	}
	out, _, ok := rewriteGoSyntaxVerbs("", files, fset, ti.info, ti.pkg, nil)
	if !ok {
		return "", false
	}
	var buf bytes.Buffer
	if err := printer.Fprint(&buf, fset, out[0]); err != nil {
		t.Fatalf("print: %v", err)
	}
	return buf.String(), true
}

func TestGoSyntaxVerbBecomesHelperCall(t *testing.T) {
	src, ok := rewriteGoSyntaxSource(t, `package main

import "fmt"

type Point struct{ X, Y int }
type IDs []uint8

func main() {
	p := Point{1, 2}
	fmt.Printf("%d %#v|%#v %%#v\n", 7, p, &p)
	s := fmt.Sprintf("%#v", IDs{1})
	fmt.Println(s, fmt.Sprintf("%v", p))
}
`)
	if !ok {
		t.Fatal("nothing was rewritten")
	}
	for _, want := range []string{
		`fmt.Printf("%d %s|%s %%#v\n", 7, __go2rust_gosyntax0_1(p), __go2rust_gosyntax0_3(&p))`,
		`fmt.Sprintf("%s", __go2rust_gosyntax0_4(IDs{1}))`,
		`fmt.Sprintf("%v", p)`,
		`s := "main.Point{"`,
		`s += "X:" + __go2rust_gosyntax0_2(v.X)`,
		`s += ", Y:" + __go2rust_gosyntax0_2(v.Y)`,
		`return "(*main.Point)(nil)"`,
		`return "&" + __go2rust_gosyntax0_1(*v)`,
		`return "main.IDs(nil)"`,
		`return fmt.Sprintf("0x%x", uint64(v))`,
	} {
		if !strings.Contains(src, want) {
			t.Errorf("missing %q in:\n%s", want, src)
		}
	}
}

func TestGoSyntaxVerbSwitchesOnTypesConvertedToInterfaces(t *testing.T) {
	src, ok := rewriteGoSyntaxSource(t, `package main

import "fmt"

type Celsius float64

func show(v any) { fmt.Printf("%#v\n", v) }

func main() {
	show(Celsius(1.5))
	show("s")
	fmt.Println("not kept", 3)
}
`)
	if !ok {
		t.Fatal("nothing was rewritten")
	}
	for _, want := range []string{
		`return "<nil>"`,
		"case Celsius:",
		"case string:",
		`return fmt.Sprintf("%v", v)`,
	} {
		if !strings.Contains(src, want) {
			t.Errorf("missing %q in:\n%s", want, src)
		}
	}
	if strings.Contains(src, "case int:") {
		t.Errorf("an argument of fmt.Println is a dynamic type of the switch:\n%s", src)
	}
}

func TestGoSyntaxVerbOfLocalTypeIsDeclaredAfterIt(t *testing.T) {
	src, ok := rewriteGoSyntaxSource(t, `package main

import "fmt"

func main() {
	type pair struct{ A, B string }
	fmt.Printf("%#v\n", []pair{{"a", "b"}})
}
`)
	if !ok {
		t.Fatal("nothing was rewritten")
	}
	decl := strings.Index(src, "type pair struct")
	helper := strings.Index(src, "__go2rust_gosyntaxL0_")
	call := strings.Index(src, `fmt.Printf("%s\n"`)
	if decl < 0 || helper < decl || call < helper {
		t.Errorf("the helper of the local type is not between its declaration and the call:\n%s", src)
	}
	if !strings.Contains(src, `"main.pair{"`) || !strings.Contains(src, `"[]main.pair{"`) {
		t.Errorf("the local type is not named as reflect names it:\n%s", src)
	}
}

func TestGoSyntaxVerbKeepsFormatsItCannotFollow(t *testing.T) {
	for _, source := range []string{
		`package main

import "fmt"

func main() {
	format := "%#v\n"
	fmt.Printf(format, 1)
}
`,
		`package main

import "fmt"

func main() {
	fmt.Printf("%[1]#v %[1]d\n", 1)
}
`,
		`package main

import "fmt"

func main() {
	args := []any{1}
	fmt.Printf("%#v\n", args...)
}
`,
	} {
		if src, ok := rewriteGoSyntaxSource(t, source); ok {
			t.Errorf("rewrote a format it cannot follow:\n%s", src)
		}
	}
}

func TestGoFormatVerbsCountsStarArguments(t *testing.T) {
	verbs, ok := goFormatVerbs("%*d|%#v|%.*f %% %#v")
	if !ok {
		t.Fatal("format not parsed")
	}
	var got []string
	for _, verb := range verbs {
		got = append(got, verb.spec+"@"+string(rune('0'+verb.arg)))
	}
	want := "%*d@1 %#v@2 %.*f@4 %#v@5"
	if strings.Join(got, " ") != want {
		t.Errorf("verbs = %q, want %q", strings.Join(got, " "), want)
	}
}

func TestGoReflectTypeStringSpellsTypesAsReflect(t *testing.T) {
	pkg := types.NewPackage("example.com/shapes", "shapes")
	named := types.NewNamed(types.NewTypeName(token.NoPos, pkg, "Point", nil), types.NewStruct(nil, nil), nil)
	for _, tc := range []struct {
		typ  types.Type
		want string
	}{
		{types.NewSlice(types.Typ[types.Byte]), "[]uint8"},
		{types.NewMap(types.Typ[types.String], types.NewPointer(named)), "map[string]*shapes.Point"},
		{types.NewInterfaceType(nil, nil), "interface {}"},
		{types.NewChan(types.RecvOnly, types.Typ[types.Rune]), "<-chan int32"},
		{types.NewStruct([]*types.Var{
			types.NewField(token.NoPos, nil, "A", types.Typ[types.Int], false),
			types.NewField(token.NoPos, nil, "B", types.Typ[types.String], false),
		}, nil), "struct { A int; B string }"},
		{types.NewArray(types.Typ[types.Bool], 2), "[2]bool"},
	} {
		if got := goReflectTypeString(tc.typ); got != tc.want {
			t.Errorf("goReflectTypeString(%s) = %q, want %q", tc.typ, got, tc.want)
		}
	}
}
