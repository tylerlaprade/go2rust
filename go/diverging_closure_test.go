package main

import (
	"strings"
	"testing"
)

// Since Rust 1.100 a closure whose body ends in panic! infers ! as its
// return type, which is not FnMut(). A Go func literal with no results that
// never finishes gets an explicit -> () so it stays a closure returning ().
func TestDivergingFuncLitReturnsUnit(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"log"
	"os"
)

func run(f func()) {
	defer func() { _ = recover() }()
	f()
}

func main() {
	run(func() { panic("boom") })
	run(func() { fmt.Println("done") })
	run(func() { os.Exit(0) })
	run(func() { log.Fatalf("bad %d", 1) })
	run(func() {
		for {
		}
	})
	run(func() {
		for {
			break
		}
	})
}
`)
	if got := strings.Count(rust, "|| -> () {"); got != 4 {
		t.Errorf("want 4 closures annotated -> (), got %d:\n%s", got, rust)
	}
}

// (*fp)() through a pointer made from unsafe.Pointer: fp is a GoPtr to the
// function's cell, so the call goes through the cell it points to rather
// than locking the GoPtr itself.
func TestCallThroughUnsafeFunctionPointer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"sync/atomic"
	"unsafe"
)

type Exporter func() string

var exporter unsafe.Pointer

func use() string {
	exporterPtr := (*Exporter)(atomic.LoadPointer(&exporter))
	if exporterPtr == nil {
		return "nil"
	}
	return (*exporterPtr)()
}

func main() {
	fmt.Println(use())
}
`)
	want := `let __f_holder = exporterPtr.borrow().expect("invalid memory address or nil pointer dereference");`
	if !strings.Contains(rust, want) {
		t.Errorf("missing %q:\n%s", want, rust)
	}
}
