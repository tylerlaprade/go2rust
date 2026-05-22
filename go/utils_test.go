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

func TestSanitizeRustCrateName(t *testing.T) {
	for _, tt := range []struct {
		in   string
		want string
	}{
		{in: "go2rust-source-types.S8JrZY", want: "go2rust_source_types_s8jrzy"},
		{in: "example.com/dep", want: "example_com_dep"},
		{in: "123/pkg", want: "pkg_123_pkg"},
		{in: "mod", want: "mod_"},
	} {
		got := SanitizeRustCrateName(tt.in)
		if got != tt.want {
			t.Fatalf("SanitizeRustCrateName(%q) = %q, want %q", tt.in, got, tt.want)
		}
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
