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

	stdModuleName := SanitizeRustModuleName("std")
	if stdModuleName != "std_" {
		t.Fatalf("SanitizeRustModuleName(\"std\") = %q, want std_", stdModuleName)
	}
	stdFileName := SanitizeRustModuleFileName(stdModuleName)
	if stdFileName != "std_" {
		t.Fatalf("SanitizeRustModuleFileName(%q) = %q, want std_", stdModuleName, stdFileName)
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

func TestRustIntegerLiteralPreservesGoValues(t *testing.T) {
	for _, tt := range []struct {
		in   string
		want string
	}{
		{in: "0", want: "0"},
		{in: "0640", want: "0o640"},
		{in: "0_640", want: "0o640"},
		{in: "0o6_40", want: "0o640"},
		{in: "0O640", want: "0o640"},
		{in: "0B1010", want: "0b1010"},
		{in: "0X1A", want: "0x1A"},
		{in: "123_456", want: "123_456"},
	} {
		if got := RustIntegerLiteral(tt.in); got != tt.want {
			t.Fatalf("RustIntegerLiteral(%q) = %q, want %q", tt.in, got, tt.want)
		}
	}
}

func TestRustFloatFromIntegerLiteralUsesGoValue(t *testing.T) {
	for _, tt := range []struct {
		in   string
		want string
	}{
		{in: "0640", want: "416.0"},
		{in: "0_640", want: "416.0"},
		{in: "0O640", want: "416.0"},
		{in: "0X1A", want: "26.0"},
		{in: "123", want: "123.0"},
	} {
		if got := RustFloatFromIntegerLiteral(tt.in); got != tt.want {
			t.Fatalf("RustFloatFromIntegerLiteral(%q) = %q, want %q", tt.in, got, tt.want)
		}
	}
}

func TestRustConstNameDoesNotRawEscapeUppercaseKeywords(t *testing.T) {
	got := rustConstName("type")
	if got != "TYPE" {
		t.Fatalf("rustConstName(\"type\") = %q, want TYPE", got)
	}
}
