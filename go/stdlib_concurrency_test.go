package main

import (
	"errors"
	"fmt"
	"go/build"
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
