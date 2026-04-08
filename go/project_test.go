package main

import (
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"golang.org/x/tools/go/packages"
)

func TestGenerateWithExternalPackagesPreservesMainFileMapping(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22

require example.com/dep v0.0.0

replace example.com/dep => ./dep
`)
	writeTestFile(t, filepath.Join(tempDir, "dep", "go.mod"), `module example.com/dep

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "dep", "dep.go"), `package dep

func Value() string { return "dep" }
`)
	writeTestFile(t, filepath.Join(tempDir, "aa.go"), `package main

func A() string { return "a" }
`)
	writeTestFile(t, filepath.Join(tempDir, "zz.go"), `package main

import "example.com/dep"

func Z() string { return dep.Value() }
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {
	println(A())
	println(Z())
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "aa.go"),
		filepath.Join(tempDir, "zz.go"),
		filepath.Join(tempDir, "main.go"),
	})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	zzRS := mustReadFile(t, filepath.Join(tempDir, "zz.rs"))

	if !strings.Contains(mainRS, "fn main()") {
		t.Fatalf("main.rs should contain fn main(), got:\n%s", mainRS)
	}
	if !strings.Contains(zzRS, "pub fn z()") {
		t.Fatalf("zz.rs should contain pub fn z(), got:\n%s", zzRS)
	}
}

func TestGenerateCargoTomlIsDeterministic(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "main.go"), "package main\nfunc main() {}\n")

	pg := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	pg.projectImports = NewImportTracker()
	pg.packageMapping = map[string]string{
		"example.com/zeta":    "zeta",
		"example.com/alpha":   "alpha",
		"example.com/mu":      "mu",
		"example.com/beta":    "beta",
		"example.com/epsilon": "epsilon",
	}

	seen := map[string]bool{}
	for range 50 {
		if err := pg.generateCargoToml(); err != nil {
			t.Fatalf("generateCargoToml() error = %v", err)
		}
		seen[mustReadFile(t, filepath.Join(tempDir, "Cargo.toml"))] = true
	}

	if len(seen) != 1 {
		t.Fatalf("generateCargoToml() should be deterministic, saw %d distinct outputs", len(seen))
	}
}

func TestTranspileWithMappingReturnsPerFileExternalPackages(t *testing.T) {
	fset := token.NewFileSet()
	fileA, err := parser.ParseFile(fset, "a.go", `package main

import "example.com/a"

func A() {}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(a.go) error = %v", err)
	}
	fileB, err := parser.ParseFile(fset, "b.go", `package main

import "example.com/b"

func B() {}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(b.go) error = %v", err)
	}

	_, _, pkgsA := TranspileWithMapping(fileA, fset, nil, nil)
	_, _, pkgsB := TranspileWithMapping(fileB, fset, nil, nil)

	if len(pkgsA) != 1 || !pkgsA["example.com/a"] {
		t.Fatalf("first file should report only example.com/a, got %#v", pkgsA)
	}
	if len(pkgsB) != 1 || !pkgsB["example.com/b"] {
		t.Fatalf("second file should report only example.com/b, got %#v", pkgsB)
	}
}

func TestPackageLoaderOrderedPackagePaths(t *testing.T) {
	loader := &PackageLoader{
		allPackages: map[string]*packages.Package{
			"github.com/zeta/lib":  {},
			"github.com/alpha/lib": {},
			"main":                 {},
			"fmt":                  {},
		},
	}

	got := loader.orderedPackagePaths()
	want := []string{"github.com/alpha/lib", "github.com/zeta/lib"}
	if strings.Join(got, ",") != strings.Join(want, ",") {
		t.Fatalf("orderedPackagePaths() = %v, want %v", got, want)
	}
}

func writeTestFile(t *testing.T, path string, content string) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		t.Fatalf("MkdirAll(%q) error = %v", path, err)
	}
	if err := os.WriteFile(path, []byte(content), 0644); err != nil {
		t.Fatalf("WriteFile(%q) error = %v", path, err)
	}
}

func mustReadFile(t *testing.T, path string) string {
	t.Helper()
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("ReadFile(%q) error = %v", path, err)
	}
	return string(data)
}
