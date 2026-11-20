package main

import (
	"path/filepath"
	"strings"
	"testing"
)

func generateMonomorphizedProject(t *testing.T, files map[string]string) map[string]string {
	t.Helper()
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), "module example.com/main\n\ngo 1.22\n")
	var paths []string
	for name, src := range files {
		path := filepath.Join(tempDir, name)
		writeTestFile(t, path, src)
		paths = append(paths, path)
	}
	generator := NewProjectGenerator(paths)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}
	out := map[string]string{}
	for name := range files {
		rsName := strings.TrimSuffix(name, ".go") + ".rs"
		out[rsName] = mustReadFile(t, filepath.Join(tempDir, rsName))
	}
	return out
}

func TestGenericFunctionsAreWrittenOncePerInstantiation(t *testing.T) {
	rs := generateMonomorphizedProject(t, map[string]string{"main.go": `package main

import "fmt"

func Map[T, U any](xs []T, f func(T) U) []U {
	out := make([]U, 0, len(xs))
	for _, x := range xs {
		out = append(out, f(x))
	}
	return out
}

type Stack[T any] struct{ items []T }

func (s *Stack[T]) Push(v T) { s.items = append(s.items, v) }

func main() {
	fmt.Println(Map([]string{"go", "rust"}, func(s string) int { return len(s) }))
	var s Stack[int]
	s.Push(1)
	fmt.Println(s.items)
}
`})["main.rs"]

	for _, want := range []string{
		"pub fn map__string__int(xs: Rc<RefCell<Option<Vec<String>>>>",
		"pub struct Stack__int {",
		"impl Stack__int {",
	} {
		if !strings.Contains(rs, want) {
			t.Fatalf("a generic declaration should be written for each instantiation, missing %q:\n%s", want, rs)
		}
	}
	if strings.Contains(rs, "pub fn map<") || strings.Contains(rs, "struct Stack<") {
		t.Fatalf("an instantiated generic declaration should not also be written as a Rust generic:\n%s", rs)
	}
}

func TestGenericsStayGenericWhenATypeArgumentNestsASlice(t *testing.T) {
	rs := generateMonomorphizedProject(t, map[string]string{"main.go": `package main

type Box[T any] struct{ V T }

func main() {
	b := Box[[1][]int]{}
	_ = b
}
`})["main.rs"]

	if !strings.Contains(rs, "Box_<T") && !strings.Contains(rs, "Box<T") {
		t.Fatalf("a type argument with a slice inside an array should keep the Rust generic, which keeps the slice shared:\n%s", rs)
	}
}

func TestGenericInstantiationsAcrossFilesAndPackages(t *testing.T) {
	rs := generateMonomorphizedProject(t, map[string]string{
		"box.go": `package main

type Box[T any] struct{ V T }

func (b Box[T]) Get() T { return b.V }
`,
		"main.go": `package main

import (
	"fmt"
	"time"
)

func main() {
	d := Box[time.Duration]{V: 2 * time.Second}
	p := Box[*Box[int]]{V: &Box[int]{V: 3}}
	fmt.Println(d.Get(), p.Get().Get())
}
`})

	box := rs["box.rs"]
	for _, want := range []string{"pub struct Box__time_Duration", "pub struct Box__ptr_Box_int", "pub struct Box__int"} {
		if !strings.Contains(box, want) {
			t.Fatalf("instantiations should be written where the generic type is declared, missing %q:\n%s", want, box)
		}
	}
}
