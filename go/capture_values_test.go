package main

import (
	"strings"
	"testing"
)

func TestClosureInForClauseLoopCopiesTheLoopVariable(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	var funcs []func() int
	for i := 0; i < 3; i++ {
		funcs = append(funcs, func() int { return i * i })
	}
	n := 0
	for j := 0; j < 3; j++ {
		f := func() { n += j }
		f()
	}
	fmt.Println(len(funcs), n)
}
`)
	if !strings.Contains(rust, "let i_closure_clone = Rc::new(RefCell::new((*i.borrow()).clone()));") {
		t.Fatalf("each iteration's closure should capture a copy of i:\n%s", rust)
	}
	if !strings.Contains(rust, "n_closure_clone = n.clone()") {
		t.Fatalf("a variable declared outside the loop is still shared:\n%s", rust)
	}
}

func TestDeferredCallArgumentsAreEvaluatedAtTheDefer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	x := 1
	defer fmt.Println(x)
	x = 2
}
`)
	if !strings.Contains(rust, "let x_defer_captured = Rc::new(RefCell::new((*x.borrow()).clone()));") {
		t.Fatalf("the deferred call should get the value x has at the defer:\n%s", rust)
	}
}

func TestDeferredCallCopiesBareScalarWithoutBorrowingIt(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func open() uintptr { return 1 }
func closefd(fd uintptr) {}

func use() {
	go func() {}()
	fd := open()
	defer closefd(fd)
}
`)

	if strings.Contains(rust, "fd.lock()") || strings.Contains(rust, "fd.borrow()") {
		t.Fatalf("deferred call should copy a bare scalar directly instead of borrowing it as a wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "let fd_defer_captured = fd;") {
		t.Fatalf("deferred call should snapshot the bare scalar value at the defer statement:\n%s", rust)
	}
}

func TestNamedIntegerConstantPrintsThroughItsStringMethod(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Weekday int

const Monday Weekday = 1

func (d Weekday) String() string { return "Mon" }

func main() {
	var x interface{} = Monday
	fmt.Println(Monday, x)
}
`)
	if strings.Contains(rust, `format!("{}", MONDAY)`) {
		t.Fatalf("Monday printed as its integer:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(Weekday(") {
		t.Fatalf("Monday should go into the interface as a Weekday:\n%s", rust)
	}
}
