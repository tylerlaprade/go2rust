package main

import (
	"strings"
	"testing"
)

func TestFmtSprintSpacesOnlyBetweenNonStrings(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	s := fmt.Sprint("a", 1, 2, "b")
	l := fmt.Sprintln("a", 1)
	fmt.Println(s, l)
}
`)
	if strings.Contains(rust, "sprint bridge") || strings.Contains(rust, "sprintln bridge") {
		t.Fatalf("fmt.Sprint went to the panicking stub:\n%s", rust)
	}
	if !strings.Contains(rust, `__go_sprint_bytes.push(b' ');`) {
		t.Fatalf("expected Print spacing for Sprint:\n%s", rust)
	}
	if !strings.Contains(rust, `__go_sprint_bytes.push(b'\n');`) {
		t.Fatalf("expected Println spacing and a newline for Sprintln:\n%s", rust)
	}
}

func TestFmtSprintPreservesArbitraryStringBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	s := string([]byte{0xfa})
	_ = fmt.Sprint("x", s)
	_ = fmt.Sprintln(s)
}
`)

	for _, want := range []string{
		"__go_string_bytes(&__go_sprint_arg_0)",
		"__go_string_bytes(&__go_sprint_arg_1)",
		"__go_string_from_bytes(__go_sprint_bytes)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("fmt.Sprint must preserve Go string bytes; missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, `format!("{}{}", format!("{}", "x".to_string())`) {
		t.Fatalf("fmt.Sprint must not display the internal Rust string encoding:\n%s", rust)
	}
}

func TestFmtSprintVariadicAnyIteratesInsideSliceGuard(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func render(values ...any) string { return fmt.Sprint(values...) }
`)

	for _, want := range []string{
		"if let Some(__sprint_values) = __sprint_guard.as_ref()",
		"let __sprint_slice = __sprint_values.as_slice()",
		"__sprint_slice.iter().enumerate()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("fmt.Sprint variadic slice must iterate inside its guard; missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "__sprint_guard.iter().flatten()") {
		t.Fatalf("fmt.Sprint cannot require borrowed IntoIterator for GoSlice:\n%s", rust)
	}
}

func TestFmtFprintWritesPrintOperands(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	fmt.Fprint(os.Stdout, "a", 1, 2)
	var sb strings.Builder
	fmt.Fprint(&sb, 1, "x")
	fmt.Println(sb.String())
}
`)
	if strings.Contains(rust, "fprint bridge") {
		t.Fatalf("fmt.Fprint went to the panicking stub:\n%s", rust)
	}
	if !strings.Contains(rust, `print!("{}{} {}"`) {
		t.Fatalf("expected Print spacing for Fprint to stdout:\n%s", rust)
	}
}
