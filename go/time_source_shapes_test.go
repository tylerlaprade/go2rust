package main

import (
	"strings"
	"testing"
)

func timeSourceShapesRust(t *testing.T) string {
	t.Helper()
	return transpileTypedRegression(t, `package main

import "fmt"

type Duration int64

const maxDuration Duration = 1<<63 - 1

type Stamp struct{ Sec int64 }

func (t *Stamp) sec() int64 { return t.Sec }

func parse(s string) (Stamp, error) { return Stamp{int64(len(s))}, nil }

func (t *Stamp) Set(s string) error {
	var err error
	*t, err = parse(s)
	return err
}

func half(d Duration) int64 { return int64(d / 1e9) }

const Second Duration = 1000000000

func even(d Duration) bool { return d < Second && Second%(d+d) == 0 }

func split(t Stamp, big bool) uint64 {
	sec := t.sec()
	switch {
	case big:
		sec := uint64(sec)
		return sec
	}
	return 0
}

func trim(p []byte) string {
	if len(p) > 1 {
		p = p[:1]
	}
	return string(p)
}

func open() (uintptr, error) { return 3, nil }

func closefd(fd uintptr) {}

func load() int {
	fd, _ := open()
	defer closefd(fd)
	return int(fd)
}

func main() {
	var s Stamp
	fmt.Println(maxDuration, s.Set("ab"), half(5), even(1), split(s, true), trim([]byte("go")), load())
}
`)
}

// Integer constants whose expression shifts past the type's width are
// written as their value (DEV-146).
func TestConstWithOverflowingShiftIsWrittenAsItsValue(t *testing.T) {
	rust := timeSourceShapesRust(t)
	if !strings.Contains(rust, "const MAX_DURATION: i64 = 9223372036854775807;") {
		t.Fatalf("1<<63 - 1 should be written as its value:\n%s", rust)
	}
}

// A float literal the type checker made an integer is the integer.
func TestIntegerTypedFloatLiteralIsAnInteger(t *testing.T) {
	rust := timeSourceShapesRust(t)
	if strings.Contains(rust, "/ 1e9") || !strings.Contains(rust, "/ 1000000000") {
		t.Fatalf("d / 1e9 should divide by the integer 1000000000:\n%s", rust)
	}
}

// *t, err = f() in a pointer method stores into *self.
func TestTupleAssignmentToDerefReceiverStoresIntoSelf(t *testing.T) {
	rust := timeSourceShapesRust(t)
	if !strings.Contains(rust, " *self = { let mut __guard = ") || strings.Contains(rust, "*self.borrow_mut() =") {
		t.Fatalf("*t in a tuple assignment should be *self:\n%s", rust)
	}
}

// d + d on a named integer clones the left operand.
func TestSameNamedOperandBinaryClonesLeft(t *testing.T) {
	rust := timeSourceShapesRust(t)
	if !strings.Contains(rust, "__bin_d.clone() + __bin_d") {
		t.Fatalf("d + d should clone the newtype on the left:\n%s", rust)
	}
}

// A case clause is a scope: sec := uint64(sec) reads the outer sec.
func TestCaseClauseDeclarationReadsOuterVariable(t *testing.T) {
	rust := timeSourceShapesRust(t)
	if !strings.Contains(rust, "Some(sec as u64)") {
		t.Fatalf("the clause's sec should be made from the outer bare sec:\n%s", rust)
	}
}

// A tail return that reads a reassigned parameter is an explicit return:
// the parameter's own cell is a body local.
func TestTailReturnOfReassignedParameterIsExplicit(t *testing.T) {
	rust := timeSourceShapesRust(t)
	body := rust[strings.Index(rust, "pub fn trim("):]
	body = body[:strings.Index(body, "\n}\n")]
	if !strings.Contains(body, "return Rc::new(RefCell::new(Some(String::from_utf8(") {
		t.Fatalf("the final return should stay a return statement:\n%s", body)
	}
}

// defer copies a bare local into its capture without unwrapping it.
func TestDeferCapturesBareLocalDirectly(t *testing.T) {
	rust := timeSourceShapesRust(t)
	if !strings.Contains(rust, "let fd_defer_captured = fd.clone();") {
		t.Fatalf("the bare fd should be captured as itself:\n%s", rust)
	}
}
