package main

import "testing"

func TestSanitizeRustModuleFileName(t *testing.T) {
	moduleName := SanitizeRustModuleName("mod")
	if moduleName != "r#mod" {
		t.Fatalf("SanitizeRustModuleName(\"mod\") = %q, want r#mod", moduleName)
	}

	fileName := SanitizeRustModuleFileName(moduleName)
	if fileName != "mod" {
		t.Fatalf("SanitizeRustModuleFileName(%q) = %q, want mod", moduleName, fileName)
	}
}
