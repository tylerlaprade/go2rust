package main

import (
	"errors"
	"fmt"
	"go/ast"
	"go/build"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestIsStdlibPackageChecksGOROOTSource(t *testing.T) {
	stdlibPackages := []string{
		"fmt",
		"go/ast",
		"syscall",
		"slices",
		"cmp",
		"iter",
	}
	for _, pkg := range stdlibPackages {
		if !isStdlibPackage(pkg) {
			t.Fatalf("isStdlibPackage(%q) = false, want true", pkg)
		}
	}

	nonStdlibPackages := []string{
		"main",
		"localpkg",
		"golang.org/x/tools/go/packages",
	}
	for _, pkg := range nonStdlibPackages {
		if isStdlibPackage(pkg) {
			t.Fatalf("isStdlibPackage(%q) = true, want false", pkg)
		}
	}
}

func TestIsStdlibPackageDoesNotShellOutForUnknownPackage(t *testing.T) {
	prevBuild := build.Default
	prevWd, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	prevGo111Module, hadGo111Module := os.LookupEnv("GO111MODULE")
	t.Cleanup(func() {
		build.Default = prevBuild
		if err := os.Chdir(prevWd); err != nil {
			t.Fatalf("restore working directory: %v", err)
		}
		if hadGo111Module {
			if err := os.Setenv("GO111MODULE", prevGo111Module); err != nil {
				t.Fatalf("restore GO111MODULE: %v", err)
			}
		} else if err := os.Unsetenv("GO111MODULE"); err != nil {
			t.Fatalf("restore GO111MODULE: %v", err)
		}
	})

	root := t.TempDir()
	if err := os.MkdirAll(filepath.Join(root, "bin"), 0o755); err != nil {
		t.Fatal(err)
	}
	marker := filepath.Join(root, "go-invoked")
	goBin := filepath.Join(root, "bin", "go")
	script := fmt.Sprintf("#!/bin/sh\nprintf invoked > %q\nexit 1\n", marker)
	if err := os.WriteFile(goBin, []byte(script), 0o755); err != nil {
		t.Fatal(err)
	}
	moduleDir := filepath.Join(root, "module")
	if err := os.MkdirAll(moduleDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(moduleDir, "go.mod"), []byte("module example.com/test\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.Chdir(moduleDir); err != nil {
		t.Fatal(err)
	}
	if err := os.Setenv("GO111MODULE", "on"); err != nil {
		t.Fatal(err)
	}
	build.Default.GOROOT = root
	build.Default.GOPATH = filepath.Join(root, "gopath")

	if isStdlibPackage("localpkg") {
		t.Fatalf("local package must not be classified as stdlib")
	}
	if _, err := os.Stat(marker); err == nil {
		t.Fatalf("isStdlibPackage must not invoke GOROOT/bin/go for unknown packages")
	} else if !errors.Is(err, os.ErrNotExist) {
		t.Fatal(err)
	}
}

func TestStdlibPackageCacheAvoidsSyncMapBridgeDependency(t *testing.T) {
	data, err := os.ReadFile("stdlib_concurrency.go")
	if err != nil {
		t.Fatalf("ReadFile(stdlib_concurrency.go) error = %v", err)
	}
	source := string(data)
	if strings.Contains(source, `"sync"`) || strings.Contains(source, "sync.Map") {
		t.Fatalf("stdlib package cache should not force the self-transpiled binary through sync.Map bridge methods")
	}
}

func TestPureStdlibPackagesDoNotMakeProgramConcurrent(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"errors"
	"strings"
	"unicode"
)

func main() {
	_ = strings.TrimPrefix("go2rust", "go")
	_ = unicode.IsLetter('x')
	_ = errors.Join(errors.New("a"))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	cd := NewConcurrencyDetector()
	cd.AnalyzeProject([]*ast.File{file})
	if cd.NeedsConcurrency() {
		t.Fatalf("calls into strings, unicode and errors should not make a program concurrent")
	}
	if !isAsyncStdlibFunction("time", "AfterFunc") {
		t.Fatalf("time.AfterFunc runs its callback on another goroutine")
	}
}

func TestUnknownSynchronousStdlibCallDoesNotMakeProgramConcurrent(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"path"
	"path/filepath"
)

func main() {
	_ = path.Base("/tmp/go2rust")
	_ = filepath.Base("/tmp/go2rust")
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	cd := NewConcurrencyDetector()
	cd.AnalyzeProject([]*ast.File{file})
	if cd.NeedsConcurrency() {
		t.Fatalf("unlisted calls from pure path packages should not make a program concurrent")
	}
}

func TestUnlistedAsyncStdlibCallbacksRemainConcurrent(t *testing.T) {
	for _, call := range []struct {
		pkg  string
		name string
	}{
		{pkg: "runtime", name: "SetFinalizer"},
		{pkg: "runtime", name: "AddCleanup"},
		{pkg: "context", name: "AfterFunc"},
		{pkg: "net/http/httptest", name: "NewServer"},
		{pkg: "net/http/httptest", name: "NewTLSServer"},
	} {
		if !isAsyncStdlibFunction(call.pkg, call.name) {
			t.Errorf("%s.%s runs user work concurrently", call.pkg, call.name)
		}
	}
}
