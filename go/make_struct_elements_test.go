package main

import (
	"strings"
	"testing"
)

// make([]T, n) for a struct T makes each element on its own: a struct's
// Clone shares its field cells, so cloning one zero value n times would
// give every element the same fields.
func TestMakeStructSliceMakesEachElement(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Cell struct{ Val int }

func main() {
	cells := make([]Cell, 3)
	nums := make([]int, 3)
	cells[0].Val = 1
	fmt.Println(cells, nums)
}
`)
	if !strings.Contains(rust, "(0..(3) as usize).map(|_| Default::default()).collect::<Vec<Cell>>()") {
		t.Errorf("each Cell should be made on its own:\n%s", rust)
	}
	if strings.Contains(rust, "vec![Default::default(); (3) as usize]") {
		t.Errorf("the Cells should not be clones of one value:\n%s", rust)
	}
	if !strings.Contains(rust, "vec![0i64; (3) as usize]") {
		t.Errorf("a slice of ints can still be filled with vec!:\n%s", rust)
	}
}
