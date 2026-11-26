package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
	"strings"
	"testing"
)

// A plain operand that a later call can change is read after the call.
func TestFmtOperandsWithCallsAreBoundFirst(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type box struct{ n int }

func (b *box) inc() int { b.n++; return b.n }

func main() {
	b := &box{}
	fmt.Println(b.n, b.inc())
}
`)
	if !strings.Contains(rust, "let __go_print_arg_") {
		t.Fatalf("b.inc() should be bound before b.n is printed:\n%s", rust)
	}
}

// A plain local that no call can reach keeps the usual print.
func TestFmtOperandsUnreachableLocalKeepsItsPlace(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"strconv"
)

func main() {
	n := 2
	path := [1]byte{'a'}
	fmt.Println(n+1, strconv.Itoa(n*2), path[0], classify(path[0]))
}

func classify(b byte) string { return string(b) }
`)
	if strings.Contains(rust, "__go_print_arg_") {
		t.Fatalf("no operand needs binding first:\n%s", rust)
	}
}

// An element of a slice can change through another slice that shares it,
// so it is read after a call that is given that slice.
func TestFmtOperandsSliceElementReadAfterCallOnAlias(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func setFirst(s []int) int { s[0] = 9; return 0 }

func main() {
	s := []int{1}
	alias := s
	fmt.Println(s[0], setFirst(alias))
}
`)
	if !strings.Contains(rust, "let __go_print_arg_") {
		t.Fatalf("setFirst(alias) should be bound before s[0] is printed:\n%s", rust)
	}
}

// A call that only reads the elements of the slice it is given leaves a
// plain variable that it is given in its place.
func TestFmtOperandsElementReaderKeepsItsPlace(t *testing.T) {
	src := `package main

import "fmt"

func sum(s []int) int {
	t := 0
	for _, v := range s {
		t += v
	}
	return t + len(s)
}

func main() {
	t := []int{1, 2}
	fmt.Println(t, sum(t))
}
`
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	// The package loader registers the program's functions, so the
	// body of sum can be read.
	prevSourceFunctionDecls := sourceFunctionDeclsByFunc
	t.Cleanup(func() { sourceFunctionDeclsByFunc = prevSourceFunctionDecls })
	decls := map[*types.Func]sourceFunctionDeclInfo{}
	for _, decl := range file.Decls {
		if fnDecl, ok := decl.(*ast.FuncDecl); ok {
			if fn, ok := typeInfo.info.Defs[fnDecl.Name].(*types.Func); ok {
				decls[fn] = sourceFunctionDeclInfo{decl: fnDecl, info: typeInfo.info}
			}
		}
	}
	SetSourceFunctionDeclsByFunc(decls)
	rust := transpileParsedRegression(t, file, fset, typeInfo)
	if strings.Contains(rust, "__go_print_arg_") {
		t.Fatalf("sum only reads t:\n%s", rust)
	}
}
