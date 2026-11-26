package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

// A function of the program that only reads and writes through its pointer
// parameter gets an element's value in a cell for the call (DEV-229). One
// that hands out an address into the pointee keeps the pointer, so it does
// not: the cell would outlive the call.
func TestElemPtrCallCellOnlyForFunctionsThatKeepNoPointer(t *testing.T) {
	src := `package main

import "fmt"

func bump(p *[1]int) { p[0]++ }

func keep(p *[1]int) *int { return &(*p)[0] }

func field(p *[1]int) []int { return p[:] }

func main() {
	rows := [][1]int{{1}, {2}}
	bump(&rows[0])
	fmt.Println(rows)
	q := keep(&rows[1])
	fmt.Println(q != nil, field(&rows[1]))
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
	// The project generator registers the program's functions.
	prevDecls := programFunctionDecls
	SetProgramFunctionDecls(functionDeclsOfFiles([]*ast.File{file}, typeInfo.info))
	t.Cleanup(func() { SetProgramFunctionDecls(prevDecls) })
	rust := transpileParsedRegression(t, file, fset, typeInfo)
	if !strings.Contains(rust, "bump(__arg0.clone())") {
		t.Fatalf("bump should get the element in a cell:\n%s", rust)
	}
	for _, name := range []string{"keep(__arg0", "field(__arg0"} {
		if strings.Contains(rust, name) {
			t.Fatalf("%s...: a function that hands out an address into its pointee must not get a cell:\n%s", name, rust)
		}
	}
}
