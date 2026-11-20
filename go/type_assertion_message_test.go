package main

import (
	"strings"
	"testing"
)

func TestFailedTypeAssertionPanicsWithGoMessage(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Rect struct{ W int }

func main() {
	var x interface{} = 42
	s := x.(string)
	p := x.(*Rect)
	fmt.Println(s, p)
}
`)
	if !strings.Contains(rust, `panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "string")`) {
		t.Fatalf("expected Go's message for a failed assertion to string:\n%s", rust)
	}
	if !strings.Contains(rust, `"*main.Rect"`) {
		t.Fatalf("expected the pointer target named as Go names it:\n%s", rust)
	}
	if !strings.Contains(rust, `panic!("interface conversion: {} is nil, not {}", "interface {}", "string")`) {
		t.Fatalf("expected Go's message for an assertion on a nil interface:\n%s", rust)
	}
}
