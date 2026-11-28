package main

import (
	"strings"
	"testing"
)

// A range over a map with struct keys binds the key variable to Go's copy of
// the key, so writing a field of the variable leaves the map's key alone
// (#356).
func TestMapRangeStructKeyIsACopy(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type K struct{ x, y int }

func main() {
	m := map[K]int{{3, 4}: 7}
	for k, v := range m {
		k.x = 9
		_ = v
	}
	fmt.Println(m[K{3, 4}], len(m))
}
`)
	if !strings.Contains(rust, "for (__range_key, v) in") || !strings.Contains(rust, "let k = __range_key.__go_value_clone();") {
		t.Fatalf("the key should be bound to Go's copy of the map's key:\n%s", rust)
	}
}
