package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

// A literal that leaves out a struct-typed field leaves the struct's other
// fields at their zero values too, through the struct's Default: an int is
// 0 and an array its zeros, not a nil cell.
func TestStructLiteralOmittingAStructFieldZeroesTheRest(t *testing.T) {
	src := `package main

type Vec struct{ X, Y int }

type Player struct {
	Name  string
	Pos   Vec
	HP    int
	Score [2]int
}

func main() {
	p := Player{Name: "a"}
	_ = p
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
	if strings.Contains(rust, "h_p: Default::default()") || strings.Contains(rust, "score: Default::default()") {
		t.Fatalf("omitted fields should not get the zero of their cell:\n%s", rust)
	}
	if !strings.Contains(rust, `Player { name: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() }`) {
		t.Fatalf("omitted fields should come from Player's Default:\n%s", rust)
	}
}
