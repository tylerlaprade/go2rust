package main

import (
	"go/build"
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
