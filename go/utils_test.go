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

func TestRustStringLiteralUsesRustEscapes(t *testing.T) {
	got := RustStringLiteral("\"\\v\\a\\x7f\\u2028\"")
	want := "\"\\u{b}\\u{7}\\u{7f}\\u{2028}\""
	if got != want {
		t.Fatalf("RustStringLiteral() = %q, want %q", got, want)
	}
}

func TestRustStringLiteralUsesLossyBytesForInvalidUTF8(t *testing.T) {
	got := RustStringLiteral("\"\\x80\"")
	want := "String::from_utf8_lossy(&[0x80u8]).into_owned()"
	if got != want {
		t.Fatalf("RustStringLiteral() = %q, want %q", got, want)
	}
}

func TestRustConstNameDoesNotRawEscapeUppercaseKeywords(t *testing.T) {
	got := rustConstName("type")
	if got != "TYPE" {
		t.Fatalf("rustConstName(\"type\") = %q, want TYPE", got)
	}
}
