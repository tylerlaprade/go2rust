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

func TestFmtSprintUsesNamedStringMethods(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type StringerText string
func (StringerText) String() string { return "string method" }

type ErrorText string
func (ErrorText) Error() string { return "error method" }

func main() {
	_ = fmt.Sprint(StringerText("raw"))
	_ = fmt.Sprintln(ErrorText("raw"))
}
`)

	if strings.Contains(rust, "__go_string_bytes(&__go_sprint_arg_0)") {
		t.Fatalf("fmt.Sprint/Sprintln must invoke String/Error methods on named strings:\n%s", rust)
	}
	for _, want := range []string{"format!(\"{}\",", "StringerText", "ErrorText"} {
		if !strings.Contains(rust, want) {
			t.Fatalf("fmt.Sprint/Sprintln named-string method path missing %q:\n%s", want, rust)
		}
	}
}

func TestFmtSprintfPercentSPreservesArbitraryStringBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	s := string([]byte{0xfa})
	_ = fmt.Sprintf("pre%%:%s:%s", "x", s)
}
`)

	for _, want := range []string{
		"__go_sprintf_bytes.extend_from_slice(&[112u8, 114u8, 101u8, 37u8, 58u8]",
		"__go_string_bytes(&__go_sprintf_arg_0)",
		"__go_string_bytes(&__go_sprintf_arg_1)",
		"__go_string_from_bytes(__go_sprintf_bytes)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("fmt.Sprintf %%s must preserve Go string bytes; missing %q:\n%s", want, rust)
		}
	}
}

func TestFmtSprintfPercentSUsesNamedStringMethods(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type StringerText string
func (StringerText) String() string { return "string method" }

type ErrorText string
func (ErrorText) Error() string { return "error method" }

func main() {
	_ = fmt.Sprintf("%s", StringerText("raw"))
	_ = fmt.Sprintf("%s", ErrorText("raw"))
}
`)

	if strings.Contains(rust, "__go_sprintf_bytes") {
		t.Fatalf("fmt.Sprintf %%s must still invoke String/Error methods on named strings:\n%s", rust)
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

func TestFmtWriterPathsPreserveArbitraryStringBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type writer struct { data []byte }
func (w *writer) Write(p []byte) (int, error) {
	w.data = append(w.data, p...)
	return len(p), nil
}

func main() {
	w := &writer{}
	s := string([]byte{0xfa})
	fmt.Fprint(w, "x", s)
	fmt.Fprintf(w, "<%s>", s)
	fmt.Fprintln(w, s, "z")
}
`)

	if strings.Count(rust, "let __bytes = __go_string_bytes(&__s)") < 3 {
		t.Fatalf("Fprint, Fprintln and simple %%s Fprintf must write canonical Go bytes:\n%s", rust)
	}
}

func TestFmtFprintOsFileUsesCanonicalGoBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"os"
)

func main() {
	f, _ := os.Create("out.bin")
	s := string([]byte{0xfa})
	fmt.Fprint(f, s)
	fmt.Fprintln(f, s)
}
`)

	for _, want := range []string{
		"let __bytes = __go_string_bytes(&__s)",
		".write(",
		"Vec::from(__bytes)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("fmt.Fprint to *os.File must write canonical Go bytes; missing %q:\n%s", want, rust)
		}
	}
	if strings.Count(rust, "Vec::from(__bytes)") < 2 {
		t.Fatalf("fmt.Fprint/Fprintln to *os.File must both pass canonical bytes:\n%s", rust)
	}
}

func TestFmtStringsBuilderPathsPreserveArbitraryStringBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"strings"
)

func main() {
	var b strings.Builder
	s := string([]byte{0xfa})
	fmt.Fprint(&b, "x", s)
	fmt.Fprintln(&b, s)
	fmt.Fprintf(&b, "<%s>", s)
}
`)

	if strings.Count(rust, "__go_string_from_bytes(__go_sprint_bytes)") < 2 {
		t.Fatalf("Fprint and Fprintln to strings.Builder must preserve canonical Go bytes:\n%s", rust)
	}
	if !strings.Contains(rust, "__go_string_from_bytes(__go_sprintf_bytes)") {
		t.Fatalf("simple %%s Fprintf to strings.Builder must preserve canonical Go bytes:\n%s", rust)
	}
}

func TestFmtHexStringUsesCanonicalGoBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	s := string([]byte{0xfa})
	fmt.Printf("%x %X", s, s)
}
`)

	if strings.Count(rust, "let __bytes = __go_string_bytes(&__text)") < 2 {
		t.Fatalf("fmt %%x/%%X strings must format canonical Go bytes:\n%s", rust)
	}
}
