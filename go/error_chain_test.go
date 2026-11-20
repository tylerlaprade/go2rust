package main

import (
	"strings"
	"testing"
)

func TestErrorsIsAndWrapKeepTheWrappedError(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"errors"
	"fmt"
)

var ErrNotFound = errors.New("not found")

type PE struct{ Line int }

func (e *PE) Error() string { return "pe" }

func main() {
	err := fmt.Errorf("find: %w", ErrNotFound)
	var pe *PE
	fmt.Println(errors.Is(err, ErrNotFound), errors.As(err, &pe), errors.Unwrap(err), errors.Join(err, nil))
}
`)
	for _, want := range []string{"go_error_wrap(format!(", "go_errors_is(", "go_error_chain(", ".downcast_ref::<PE>()", "go_errors_unwrap(", "go_errors_join(vec!["} {
		if !strings.Contains(rust, want) {
			t.Fatalf("expected %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "is bridge: generic stub") {
		t.Fatalf("errors.Is went to the panicking stub:\n%s", rust)
	}
}

func TestFormatVerbArgIndices(t *testing.T) {
	got := formatVerbArgIndices(`"%q: %w and %%w %v %w"`, 'w')
	if len(got) != 2 || got[0] != 1 || got[1] != 3 {
		t.Fatalf("got %v, want [1 3]", got)
	}
}
