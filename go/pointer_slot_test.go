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

// rewritePointerSlotsSource runs the **T rewrite on one file and returns
// the rewritten source, or false when the rewrite keeps the file.
func rewritePointerSlotsSource(t *testing.T, src string) (string, bool) {
	t.Helper()
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile: %v", err)
	}
	ti, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo: %v", err)
	}
	files, ok := rewritePointerSlots("", []*ast.File{file}, fset, ti.info, nil)
	if !ok {
		return "", false
	}
	if _, err := NewTypeInfo(files, fset); err != nil {
		t.Fatalf("the rewrite does not type-check: %v", err)
	}
	var buf bytes.Buffer
	if err := printer.Fprint(&buf, fset, files[0]); err != nil {
		t.Fatalf("Fprint: %v", err)
	}
	return buf.String(), true
}

func TestPointerSlotsRebindThePointer(t *testing.T) {
	out, ok := rewritePointerSlotsSource(t, `package main

type Node struct {
	v    int
	next *Node
}

func push(pp **Node, v int) { *pp = &Node{v, *pp} }

func main() {
	var head *Node
	push(&head, 1)
	links := [2]*Node{}
	push(&links[1], 2)
	n := &Node{}
	push(&n.next, 3)
	if pp := new(*Node); *pp == nil {
		push(pp, 4)
	}
}
`)
	if !ok {
		t.Fatal("the rewrite kept the file")
	}
	for _, want := range []string{
		"type __ptrSlot_Node struct",
		"pp __ptrSlot_Node, v int", "pp.set(&Node{v, pp.get()})",
		"head = __slot_v",
		"(&links, 1)",
		"(n)",
		"pp.get() == nil",
	} {
		if !strings.Contains(out, want) {
			t.Errorf("missing %q in:\n%s", want, out)
		}
	}
}

func TestPointerSlotsKeepAPackageTheyCannotRewrite(t *testing.T) {
	cases := map[string]string{
		"two **T compared": `package main

func same(a, b **int) bool { return a == b }

func main() {
	var p *int
	_ = same(&p, &p)
}
`,
		"a field of a pointer receiver": `package main

type List struct{ head *int }

func set(pp **int, v *int) { *pp = v }

func (l *List) reset(v int) { set(&l.head, &v) }

func main() { (&List{}).reset(1) }
`,
		"a slotted variable whose address is also kept": `package main

import "fmt"

func set(pp **int, v *int) { *pp = v }

func main() {
	var p *int
	v := 1
	set(&p, &v)
	fmt.Printf("%T\n", &p)
}
`,
		"&*pp, which panics for a nil pp": `package main

func same(pp **int) **int { return &*pp }

func main() {
	var p *int
	_ = same(&p)
}
`,
		"a field promoted through an embedded pointer": `package main

type Inner struct{ P *int }

type Outer struct{ *Inner }

func set(pp **int, v *int) { *pp = v }

func main() {
	o := Outer{&Inner{}}
	v := 1
	set(&o.P, &v)
}
`,
		"a method another package's interface may name": `package main

import "flag"

type S struct{}

func (S) Set(pp **int) error { return nil }

func main() {
	var p *int
	_ = S{}.Set(&p)
	_ = flag.Parse
}
`,
		"only an address converted to an interface": `package main

import "fmt"

func main() {
	var p *int
	fmt.Printf("%T\n", &p)
}
`,
	}
	for name, src := range cases {
		t.Run(name, func(t *testing.T) {
			if out, ok := rewritePointerSlotsSource(t, src); ok {
				t.Fatalf("the rewrite changed the file:\n%s", out)
			}
		})
	}
}

func TestPointerSlotsCoverMapPointers(t *testing.T) {
	out, ok := rewritePointerSlotsSource(t, `package main

func add(m *map[string]int, k string) {
	if *m == nil {
		*m = map[string]int{}
	}
	(*m)[k]++
}

func main() {
	var counts map[string]int
	add(&counts, "a")
}
`)
	if !ok {
		t.Fatal("the rewrite kept the file")
	}
	for _, want := range []string{"type __ptrSlot_map_string_int struct", "m.set(map[string]int{})", "(m.get())[k]++"} {
		if !strings.Contains(out, want) {
			t.Errorf("missing %q in:\n%s", want, out)
		}
	}
}

// &map[K]V{...} and &M{...} are slots of a new variable the literal starts
// in, so *p = nil leaves p itself non-nil.
func TestPointerSlotsCoverMapLiteralAddresses(t *testing.T) {
	out, ok := rewritePointerSlotsSource(t, `package main

import "fmt"

type M map[string]int

func main() {
	p := &map[string]int{"a": 1}
	*p = nil
	q := &M{"b": 2}
	fmt.Println(p == nil, *p == nil, len(*q))
}
`)
	if !ok {
		t.Fatal("the rewrite kept the file")
	}
	for _, want := range []string{
		`var __slot_p1 map[string]int = map[string]int{"a": 1}`,
		`var __slot_p3 M = M{"b": 2}`,
		"p.set(nil)",
		"p.get == nil",
	} {
		if !strings.Contains(out, want) {
			t.Errorf("missing %q in:\n%s", want, out)
		}
	}
}

func TestPointerSlotsKeepTheIndexTypeAndAvoidSourceNames(t *testing.T) {
	out, ok := rewritePointerSlotsSource(t, `package main

type Index int

func set(pp **int, v *int) { *pp = v }

func main() {
	__slot_v3 := 7
	links := make([]*int, 2)
	var i Index = 1
	set(&links[i], &__slot_v3)
}
`)
	if !ok {
		t.Fatal("the rewrite kept the file")
	}
	if !strings.Contains(out, "Index) __ptrSlot_int") {
		t.Errorf("the index parameter does not keep its type:\n%s", out)
	}
	if strings.Count(out, "__slot_v3") != 2 {
		t.Errorf("a generated name shadows the program's __slot_v3:\n%s", out)
	}
}
