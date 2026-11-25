package main

import (
	"strings"
	"testing"
)

// append(s, xs...) keeps a nil s nil when xs is empty (DEV-187).
func TestVariadicAppendOfNothingKeepsNil(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func main() {
	var t []int
	var none []int
	t = append(t, none...)
	_ = t
}
`)
	if !strings.Contains(rust, "if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) }") {
		t.Errorf("a variadic append should keep a nil slice nil when it adds nothing:\n%s", rust)
	}
}
