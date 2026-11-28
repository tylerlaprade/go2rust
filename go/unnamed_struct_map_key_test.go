package main

import (
	"os"
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

// An unnamed struct type written in one file of a package and used as a map
// key only in another gets the key's equality and order.
func TestUnnamedStructMapKeyFromAnotherFileIsOrdered(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "helpers.go"), `package main

func origin() struct{ x, y int } { return struct{ x, y int }{0, 0} }
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "fmt"

func main() {
	seen := map[struct{ x, y int }]bool{}
	seen[origin()] = true
	fmt.Println(len(seen))
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go"), filepath.Join(tempDir, "helpers.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}
	files, err := filepath.Glob(filepath.Join(tempDir, "*.rs"))
	if err != nil || len(files) == 0 {
		t.Fatalf("no Rust files written: %v", err)
	}
	var written string
	for _, name := range files {
		rs, err := os.ReadFile(name)
		if err != nil {
			t.Fatalf("read %s: %v", name, err)
		}
		written += string(rs)
	}
	if !strings.Contains(written, "#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]\npub struct AnonymousStruct1 {") {
		t.Fatalf("the struct should get the key's equality and order wherever it is written:\n%s", written)
	}
}

// A named struct field of an unnamed struct map key, P in
// map[struct{ P Point }]bool, gets the order the key's Ord needs.
func TestUnnamedStructMapKeyNamedStructFieldIsOrdered(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "fmt"

type Point struct{ X int }

func main() {
	m := map[struct{ P Point }]bool{}
	m[struct{ P Point }{Point{1}}] = true
	fmt.Println(len(m))
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}
	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]\npub struct Point {") {
		t.Fatalf("a named struct field of a map key should be ordered:\n%s", mainRS)
	}
}

// A struct in an array field of an unnamed struct map key, Point in
// map[struct{ P [1]Point }]bool, gets the equality and order the key's
// derives need.
func TestUnnamedStructMapKeyArrayFieldElementIsOrdered(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "fmt"

type Point struct{ X int }

func main() {
	m := map[struct{ P [1]Point }]bool{}
	m[struct{ P [1]Point }{[1]Point{{1}}}] = true
	fmt.Println(len(m))
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}
	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]\npub struct Point {") {
		t.Fatalf("a struct in an array field of a map key should be ordered:\n%s", mainRS)
	}
}
