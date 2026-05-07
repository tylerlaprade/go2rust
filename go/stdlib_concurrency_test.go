package main

import "testing"

func TestIsStdlibPackageUsesGoBuild(t *testing.T) {
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
