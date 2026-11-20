package main

import (
	"path/filepath"
	"strings"
	"testing"

	"golang.org/x/tools/go/packages"
)

func TestStdlibSourceDefaultAndBridgeOptOut(t *testing.T) {
	t.Setenv("GO2RUST_BRIDGE_STDLIB_PACKAGES", "fmt,go/parser")
	for _, path := range []string{"go/types", "go/token", "unicode/utf8", "runtime/debug"} {
		if !shouldTranspileStdlibPackage(path) {
			t.Errorf("%s should use Go source", path)
		}
	}
	for _, path := range []string{"fmt", "go/parser", "runtime", "unsafe", "example.org/foo"} {
		if shouldTranspileStdlibPackage(path) {
			t.Errorf("%s should not be selected for stdlib source translation", path)
		}
	}
}

func TestStdlibBoundaryUsesLoadedPackageSignals(t *testing.T) {
	pkgs, err := packages.Load(&packages.Config{
		Mode:       packages.LoadAllSyntax,
		BuildFlags: []string{"-tags=purego"},
	}, "math", "sync", "internal/cpu")
	if err != nil {
		t.Fatalf("load standard library packages: %v", err)
	}
	byPath := make(map[string]*packages.Package, len(pkgs))
	for _, pkg := range pkgs {
		byPath[pkg.PkgPath] = pkg
	}
	for _, path := range []string{"math", "sync", "internal/cpu"} {
		pkg := byPath[path]
		if pkg == nil {
			t.Fatalf("packages.Load omitted %s", path)
		}
		if len(pkg.Errors) != 0 {
			t.Fatalf("load %s: %v", path, pkg.Errors)
		}
	}
	if len(byPath["internal/cpu"].OtherFiles) == 0 {
		t.Fatal("NeedFiles did not expose internal/cpu assembly files")
	}
	for _, tc := range []struct {
		path string
		want bool
	}{
		{path: "math", want: true},
		{path: "sync", want: false},
		{path: "internal/cpu", want: false},
	} {
		got, err := shouldTranspileLoadedStdlibPackage(byPath[tc.path])
		if err != nil {
			t.Fatalf("classify %s: %v", tc.path, err)
		}
		if got != tc.want {
			t.Errorf("shouldTranspileLoadedStdlibPackage(%s) = %v, want %v", tc.path, got, tc.want)
		}
	}
}

func TestStdlibDefaultProjectCallsGeneratedCrate(t *testing.T) {
	t.Setenv("GO2RUST_BRIDGE_STDLIB_PACKAGES", "")
	dir := t.TempDir()
	// A standalone program using only the standard library needs no go.mod.
	main := filepath.Join(dir, "main.go")
	writeTestFile(t, main, `package main
import u "unicode/utf8"
func main() { println(u.RuneCountInString("hé")) }
`)
	if err := NewProjectGenerator([]string{main}).Generate(); err != nil {
		t.Fatal(err)
	}
	rust := mustReadFile(t, filepath.Join(dir, "main.rs"))
	if !strings.Contains(rust, "unicode_utf8::rune_count_in_string(") {
		t.Fatalf("main should call the generated crate:\n%s", rust)
	}
	mustReadFile(t, filepath.Join(dir, "vendor", "unicode_utf8", "lib.rs"))
}

func TestSourceTimeDurationKeepsGoRepresentation(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "example.org/caller", `package caller
import "time"
func nanos(d time.Duration) int64 { return int64(d) }
func second() int64 { return int64(time.Second) }
`, map[string]string{"time": "time"})
	for _, bridge := range []string{".as_nanos()", "std::time::Duration::"} {
		if strings.Contains(rust, bridge) {
			t.Fatalf("source time.Duration used bridge %s:\n%s", bridge, rust)
		}
	}
}
