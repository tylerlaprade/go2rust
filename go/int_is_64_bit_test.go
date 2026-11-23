package main

import (
	"strings"
	"testing"
)

func TestGoIntIsSixtyFourBits(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Kind int32

var table = [...]uint16{0, 2, 4}

func use(xs []int) (int, any, Kind) {
	big := 1 << 40
	n := 0
	var prefix int
	prefix = 'b'
	nums := []int{1, 2}
	var boxed any = 7
	_ = nums
	return big + n + prefix + len(xs), boxed, Kind(len(table) - 1)
}
`)

	for _, want := range []string{
		"pub fn r#use(xs: Rc<RefCell<Option<Vec<i64>>>>) -> (i64,",
		"Some(1099511627776i64)",
		"Some(0i64)",
		"Some(vec![1i64, 2i64])",
		"Box::new(7i64)",
		"98i64",
		"((*table.borrow().as_ref().unwrap()).len() as i64) - (1 as i64)) as i32",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("Go's int should be a Rust i64, missing %q:\n%s", want, rust)
		}
	}
}

func TestRuneStaysThirtyTwoBits(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const RuneError = '\uFFFD'

func runes(s string) []rune {
	var r rune = 'A'
	_ = r
	return []rune(s)
}
`)

	if !strings.Contains(rust, "RUNE_ERROR: i32") || !strings.Contains(rust, "Vec<i32>") {
		t.Fatalf("a rune should stay an i32:\n%s", rust)
	}
}
