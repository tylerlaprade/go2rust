package main

import (
	"path/filepath"
	"strings"
	"testing"
)

// An unnamed struct type that keys a map is written as an anonymous struct
// with the equality and order a BTreeMap key needs, and a package-level alias
// of it is the struct of the alias's name (#326).
func TestUnnamedStructMapKeyIsAnOrderedAnonymousStruct(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "fmt"

type pos = struct{ a, b int }

func main() {
	seen := map[struct{ x, y int }]bool{}
	seen[struct{ x, y int }{1, 2}] = true
	byPos := map[struct{ a, b int }]string{}
	byPos[pos{1, 2}] = "p"
	fmt.Println(len(seen), len(byPos))
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}
	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	for _, want := range []string{
		"#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]\npub struct AnonymousStruct1 {",
		"BTreeMap::<AnonymousStruct1, Rc<RefCell<Option<bool>>>>",
		"#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]\npub struct pos {",
		"BTreeMap::<pos, Rc<RefCell<Option<String>>>>",
	} {
		if !strings.Contains(mainRS, want) {
			t.Fatalf("missing %q:\n%s", want, mainRS)
		}
	}
	if strings.Contains(mainRS, "unknown struct") {
		t.Fatalf("a map key's struct type should have a name:\n%s", mainRS)
	}
}

// An unnamed struct field of an unnamed struct map key gets the equality and
// order the key's own derives need.
func TestUnnamedStructMapKeyFieldOfUnnamedStructIsOrdered(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "fmt"

func main() {
	m := map[struct{ p struct{ x int } }]bool{}
	m[struct{ p struct{ x int } }{}] = true
	fmt.Println(len(m))
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}
	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	ordered := strings.Count(mainRS, "#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]\npub struct AnonymousStruct")
	if ordered != 2 {
		t.Fatalf("the key struct and its unnamed struct field should both be ordered (got %d):\n%s", ordered, mainRS)
	}
}
