package main

import (
	"strings"
	"testing"
)

func TestElidedNamedSliceMapLiteralValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Numbers []int

func values() map[string]Numbers {
	return map[string]Numbers{"a": {1, 3: 4}, "empty": {}}
}
`)
	for _, want := range []string{
		"Some(Numbers(Rc::new(RefCell::new(Some(vec![1, 0, 0, 4])))))",
		"Some(Numbers(Rc::new(RefCell::new(Some(Vec::<i32>::new())))))",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("map value should contain a defined slice, missing %q in:\n%s", want, rust)
		}
	}
}
