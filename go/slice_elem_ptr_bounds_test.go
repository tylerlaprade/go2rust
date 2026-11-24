package main

import (
	"strings"
	"testing"
)

// &items[i] with i not below len(items) panics where it is taken, with
// Go's index-out-of-range text, before anything holds the pointer
// (DEV-149).
func TestSliceElemPointerChecksIndexWhenTaken(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Item struct{ Price int }

func main() {
	items := []Item{{1}}
	p := &items[0]
	p.Price++
	fmt.Println(items[0].Price)
}
`)
	for _, want := range []string{
		"GoSliceElemPtr::new(",
		"if let Ok(__guard) = slice.try_borrow() {",
		`panic!("index out of bounds: the len is {} but the index is {}", __len, index);`,
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}
