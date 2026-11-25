package main

import (
	"bytes"
	"go/ast"
	"go/parser"
	"go/printer"
	"go/token"
	"strings"
	"testing"
)

// boxedGo rewrites src with boxCapturedReboundVariables and prints it back
// as Go, with each run of white space made one space.
func boxedGo(t *testing.T, src string) string {
	t.Helper()
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
	if err != nil {
		t.Fatalf("ParseFile error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo error = %v", err)
	}
	boxCapturedReboundVariables(typeInfo, fset, []*ast.File{file})
	var out bytes.Buffer
	if err := printer.Fprint(&out, fset, file); err != nil {
		t.Fatalf("printer error = %v", err)
	}
	return strings.Join(strings.Fields(out.String()), " ")
}

func TestClosureAssignedPointerGetsACell(t *testing.T) {
	got := boxedGo(t, `package main

type Node struct{ Name string }

func walk(n *Node, f func(*Node)) { f(n) }

func find(root *Node) *Node {
	var found *Node
	walk(root, func(n *Node) {
		if n.Name == "x" {
			found = n
		}
	})
	return found
}

func main() { find(&Node{}) }
`)
	for _, want := range []string{
		"var found *Node __cell_found_1 := struct { v *Node }{found}",
		"__cell_found_1.v = n",
		"return __cell_found_1.v",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestClosureAssignedMapAndChannelParametersGetCells(t *testing.T) {
	got := boxedGo(t, `package main

func f(m map[string]int, ch chan int) {
	func() {
		m = map[string]int{}
		ch = make(chan int)
	}()
	_, _ = m, ch
}

func main() { f(nil, nil) }
`)
	for _, want := range []string{
		"__cell_m_1 := struct { v map[string]int }{m}",
		"__cell_ch_2 := struct { v chan int }{ch}",
		"__cell_m_1.v = map[string]int{}",
		"_, _ = __cell_m_1.v, __cell_ch_2.v",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

// A loop that assigns the variable before it makes a closure assigns it
// again, on the next iteration, after that closure was made.
func TestLoopAssignmentBeforeTheClosureGetsACell(t *testing.T) {
	got := boxedGo(t, `package main

type Node struct{ V int }

func main() {
	var cur *Node
	var fs []func() int
	for i := 0; i < 3; i++ {
		cur = &Node{V: i}
		fs = append(fs, func() int { return cur.V })
	}
	_ = fs
}
`)
	if !strings.Contains(got, "__cell_cur_1.v = &Node{V: i}") || !strings.Contains(got, "return __cell_cur_1.v.V") {
		t.Fatalf("cur was not given a cell:\n%s", got)
	}
}

// The body of a range over a function is the yield function, so an
// assignment to the range variable is one inside a closure.
func TestRangeOverFunctionAssignmentGetsACell(t *testing.T) {
	got := boxedGo(t, `package main

type Node struct{ V int }

func seq(yield func(*Node) bool) { yield(&Node{}) }

func main() {
	var p *Node
	for p = range seq {
		p = nil
		break
	}
	_ = p
}
`)
	if !strings.Contains(got, "for __cell_p_1.v = range seq { __cell_p_1.v = nil") {
		t.Fatalf("p was not given a cell:\n%s", got)
	}
}

func TestVariablesThatAreNotReboundKeepTheirHandle(t *testing.T) {
	src := `package main

type Node struct{ V int }

func main() {
	p := &Node{}
	f := func() int { return p.V }
	f()
	q := &Node{}
	q = &Node{V: 1}
	g := func() int { return q.V }
	g()
	var r *Node
	pr := &r
	func() { r = &Node{} }()
	_ = pr
	var n int
	func() { n = 1 }()
	_ = n
}
`
	if got := boxedGo(t, src); strings.Contains(got, "__cell_") {
		t.Fatalf("a cell was added:\n%s", got)
	}
}

// The Rust struct of an anonymous struct type is chosen by its field types
// too: struct{ v *int } and struct{ v string } were one Rust struct.
func TestAnonymousStructsWithDifferentFieldTypesAreTwoRustStructs(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	x := 1
	a := struct{ v *int }{&x}
	b := struct{ v string }{"s"}
	fmt.Println(*a.v, b.v)
}
`)
	if !strings.Contains(rust, "AnonymousStruct1 {") || !strings.Contains(rust, "AnonymousStruct2 {") {
		t.Fatalf("want two anonymous structs:\n%s", rust)
	}
}
