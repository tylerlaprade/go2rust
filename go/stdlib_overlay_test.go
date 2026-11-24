package main

import (
	"go/build"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
)

// On an architecture with math's assembly, the loader replaces each
// assembly-backed file with the one the other architectures build, so the
// translated package uses its generic Go code.
func TestMathLoadsWithoutAssemblyBackedFiles(t *testing.T) {
	if runtime.GOARCH != "amd64" && runtime.GOARCH != "arm64" {
		t.Skip("math has no assembly-backed floor on this architecture")
	}
	overlay := stdlibNoAsmOverlay()
	floor := overlay[filepath.Join(build.Default.GOROOT, "src", "math", "floor_asm.go")]
	if floor == nil {
		t.Fatalf("floor_asm.go has no overlay: %v", overlay)
	}
	text := string(floor)
	if !strings.Contains(text, "const haveArchFloor = false") || strings.Contains(text, "//go:build") {
		t.Fatalf("floor_asm.go should read as floor_noasm.go without its build line:\n%s", text)
	}
}

// A local that has the name of a package constant is the local.
func TestLocalShadowingPackageConstantIsNotTheConstant(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

const s1 = 0x02

func decode(b1 byte) rune {
	s1 := b1
	return rune(s1 & 0x3f)
}

func main() {
	fmt.Println(decode(0xa9), s1)
}
`)
	if strings.Contains(rust, "S1 as") {
		t.Errorf("the local s1 is written as the constant S1:\n%s", rust)
	}
}

// A constant float expression is the value Go computes, rounded once.
func TestConstantFloatExpressionIsFolded(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	x := 0.1 + 0.2
	fmt.Println(x, []float64{1.1 * 1.1})
}
`)
	if strings.Contains(rust, "0.1 + 0.2") || strings.Contains(rust, "1.1 * 1.1") {
		t.Errorf("constant float expressions are computed at run time:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(0.3)") || !strings.Contains(rust, "1.21") {
		t.Errorf("missing the folded constants:\n%s", rust)
	}
}

// A return that reads a named result is not the body's tail expression:
// named results are locals of the body, and a tail expression's temporary
// borrows outlive them.
func TestReturnOfNamedResultsIsNotATailExpression(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func split(n int) (q int, r int) {
	q = n / 3
	r = n % 3
	return q, r
}

func main() {
	q, r := split(7)
	fmt.Println(q, r)
}
`)
	if !strings.Contains(rust, "return ((*q.borrow().as_ref().unwrap()), (*r.borrow().as_ref().unwrap()));") {
		t.Errorf("the return of named results should be an explicit return:\n%s", rust)
	}
}

// internal/bytealg loads with its generic Go files in place of the
// assembly-backed ones, and without the internal/cpu offsets only assembly
// reads.
func TestStdlibOverlayReplacesBytealgAssembly(t *testing.T) {
	overlay := stdlibNoAsmOverlay()
	dir := filepath.Join(build.Default.GOROOT, "src", "internal", "bytealg")
	if _, err := os.Stat(filepath.Join(dir, "index_native.go")); err != nil {
		t.Skip("GOROOT has no internal/bytealg sources")
	}
	native, ok := overlay[filepath.Join(dir, "index_native.go")]
	if !ok || !strings.Contains(string(native), `panic("unimplemented")`) || strings.Contains(string(native), "//go:build") {
		t.Fatalf("index_native.go should hold index_generic.go without its build line:\n%s", native)
	}
	if amd64, ok := overlay[filepath.Join(dir, "index_amd64.go")]; !ok || strings.TrimSpace(string(amd64)) != "package bytealg" {
		t.Fatalf("index_amd64.go should be only a package clause:\n%s", amd64)
	}
	bytealg, ok := overlay[filepath.Join(dir, "bytealg.go")]
	if !ok {
		t.Fatal("bytealg.go has no overlay")
	}
	text := string(bytealg)
	if strings.Contains(text, "internal/cpu") || strings.Contains(text, "Offsetof") {
		t.Fatalf("bytealg.go should drop the internal/cpu offsets:\n%s", text)
	}
	if !strings.Contains(text, `_ "unsafe"`) || !strings.Contains(text, "var MaxLen int") {
		t.Fatalf("bytealg.go should keep its linkname import and MaxLen:\n%s", text)
	}
}
