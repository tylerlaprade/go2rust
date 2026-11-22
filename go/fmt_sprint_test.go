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

	for _, want := range []string{
		"let __go_string_method_result = __go_string_method_receiver.string()",
		"let __go_string_method_result = __go_string_method_receiver.error()",
		"__go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0))",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("fmt.Sprint/Sprintln named-string method path missing %q:\n%s", want, rust)
		}
	}
}

func TestFmtSprintPreservesBytesReturnedByNamedStringMethods(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type stringerText string
func (stringerText) String() string { return string([]byte{0xfb}) }

type errorText string
func (errorText) Error() string { return string([]byte{0xfc}) }

func main() {
	fmt.Sprint(stringerText("x"))
	fmt.Sprintln(errorText("x"))
}
`)

	for _, want := range []string{
		"let __go_string_method_result = __go_string_method_receiver.string()",
		"let __go_string_method_result = __go_string_method_receiver.error()",
		"__go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0))",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("Sprint-family formatting must preserve bytes returned by String/Error; missing %q:\n%s", want, rust)
		}
	}
}

func TestFmtSprintfPreservesBytesReturnedByNamedStringMethods(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type stringerText string
func (stringerText) String() string { return string([]byte{0xfb}) }

type errorText string
func (errorText) Error() string { return string([]byte{0xfc}) }

func main() {
	fmt.Sprintf("<%s>", stringerText("x"))
	fmt.Sprintf("<%s>", errorText("x"))
}
`)

	for _, want := range []string{
		"let __go_string_method_result = __go_string_method_receiver.string()",
		"let __go_string_method_result = __go_string_method_receiver.error()",
		"__go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0))",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("Sprintf %%s must preserve bytes returned by String/Error; missing %q:\n%s", want, rust)
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

func TestFmtSprintfPercentSWidthAndPrecisionPreserveArbitraryStringBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	s := string([]byte{0xfa, 'A'})
	_ = fmt.Sprintf("[%4s][%-4s][%.1s][%4.1s]", s, s, s, s)
}
`)

	for _, want := range []string{
		"__go_string_range(&__go_sprintf_arg_piece_0).len()",
		"4usize.saturating_sub",
		"__go_string_range(&__go_sprintf_arg_2).get(1)",
		"__go_sprintf_bytes.extend_from_slice(__go_sprintf_arg_slice_3)",
		"__go_string_from_bytes(__go_sprintf_bytes)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("fmt.Sprintf %%s width/precision must preserve Go bytes and rune counts; missing %q:\n%s", want, rust)
		}
	}
}

func TestFmtSprintfPercentSZeroPaddingUsesGeneralFormatting(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	s := "xy"
	_ = fmt.Sprintf("[%04s]", s)
}
`)

	if strings.Contains(rust, "__go_sprintf_arg_piece_") {
		t.Fatalf("fmt.Sprintf %%0s must not use the space-padding byte path:\n%s", rust)
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

	for _, want := range []string{
		"let __go_string_method_result = __go_string_method_receiver.string()",
		"let __go_string_method_result = __go_string_method_receiver.error()",
		"__go_sprintf_bytes.extend(__go_string_bytes(&__go_sprintf_arg_0))",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("fmt.Sprintf %%s named-string method path missing %q:\n%s", want, rust)
		}
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
		"__sprint_bytes.extend(__go_string_bytes(&__sprint_text))",
		"__go_string_from_bytes(__sprint_bytes)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("fmt.Sprint variadic slice must iterate inside its guard; missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "__sprint_guard.iter().flatten()") {
		t.Fatalf("fmt.Sprint cannot require borrowed IntoIterator for GoSlice:\n%s", rust)
	}
}

func TestFmtSprintPreservesBytesReturnedThroughAny(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"os"
)

type textValue struct{}
func (textValue) String() string { return string([]byte{0xfa}) }

type errorValue struct{}
func (errorValue) Error() string { return string([]byte{0xfb}) }

func main() {
	var text any = textValue{}
	var problem any = errorValue{}
	values := []any{text, problem}
	fmt.Print(text)
	fmt.Println(problem)
	fmt.Print(values...)
	fmt.Println(values...)
	fmt.Fprint(os.Stdout, values...)
	fmt.Fprintln(os.Stderr, values...)
	_ = fmt.Sprint(text)
	_ = fmt.Sprintln(problem)
	_ = fmt.Sprint(values...)
	_ = fmt.Sprintln(values...)
}
`)

	for _, want := range []string{
		"let __go_sprint_arg_0 = format_any(",
		"__go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0))",
		"__sprint_bytes.extend(__go_string_bytes(&__sprint_text))",
		"__go_string_from_bytes(__sprint_bytes)",
		"std::io::Write::write_all(&mut std::io::stdout()",
		"std::io::Write::write_all(&mut std::io::stderr()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("interface-held String/Error bytes must stay canonical; missing %q:\n%s", want, rust)
		}
	}
}

func TestFmtVariadicAnyWritersUseCanonicalBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"bytes"
	"fmt"
	"strings"
)

type capture struct{ data []byte }
func (c *capture) Write(p []byte) (int, error) { c.data = append(c.data, p...); return len(p), nil }

type textValue struct{}
func (textValue) String() string { return string([]byte{0xfa}) }

func main() {
	values := []any{textValue{}, 1}
	var c capture
	var b bytes.Buffer
	var s strings.Builder
	fmt.Fprint(&c, values...)
	fmt.Fprintln(&b, values...)
	fmt.Fprint(&s, values...)
}
`)

	for _, want := range []string{
		"__sprint_bytes.extend(__go_string_bytes(&__sprint_text))",
		".write(",
		"__go_write_bytes(&__bytes)",
		"__go_string_concat(__go_builder, &__go_builder_part)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("variadic []any writer formatting must use canonical bytes; missing %q:\n%s", want, rust)
		}
	}
}

func TestFmtVariadicAnyStringsBuilderReturnsCountAndError(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"strings"
)

type textValue struct{}
func (textValue) String() string { return string([]byte{0xfa}) }

func main() {
	values := []any{textValue{}, 1}
	var b strings.Builder
	n, err := fmt.Fprint(&b, values...)
	fmt.Println(n, err == nil)
}
`)

	for _, want := range []string{
		"let (mut n, mut err) = { let __go_builder_part =",
		"let __go_builder_n = __go_string_len(&__go_builder_part)",
		"(__go_builder_n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>)))",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("variadic strings.Builder write must return fmt's byte count and nil error; missing %q:\n%s", want, rust)
		}
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
	if !strings.Contains(rust, `__go_sprint_bytes.push(b' ');`) ||
		!strings.Contains(rust, `std::io::Write::write_all(&mut std::io::stdout(), &__bytes)`) {
		t.Fatalf("expected byte-preserving Print spacing for Fprint to stdout:\n%s", rust)
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

func TestFmtStdStreamsPreserveArbitraryStringBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"os"
)

func main() {
	s := string([]byte{0xfa})
	fmt.Print("<", s, ">")
	fmt.Println(s)
	fmt.Printf("[%s]", s)
	fmt.Fprint(os.Stdout, "{", s, "}")
	fmt.Fprintln(os.Stderr, s)
	fmt.Fprintf(os.Stdout, "(%s)", s)
}
`)

	if strings.Count(rust, "let __bytes = __go_string_bytes(&__s)") < 6 {
		t.Fatalf("fmt stdout/stderr paths must write canonical Go bytes:\n%s", rust)
	}
	for _, want := range []string{
		"std::io::Write::write_all(&mut std::io::stdout(), &__bytes)",
		"std::io::Write::write_all(&mut std::io::stderr(), &__bytes)",
		"Err(e) => (0,",
		"Box::<dyn StdError>::from(e)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("fmt stdout/stderr byte path missing %q:\n%s", want, rust)
		}
	}
}

func TestFmtStdStreamResultReassignmentUsesBareCount(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"os"
)

func main() {
	var n int
	var err error
	n, err = fmt.Print("a")
	n, err = fmt.Println("b")
	n, err = fmt.Printf("%s", "c")
	n, err = fmt.Fprint(os.Stdout, "d")
	n, err = fmt.Fprintln(os.Stderr, "e")
	n, err = fmt.Fprintf(os.Stdout, "%s", "f")
	_, _ = n, err
}
`)

	if strings.Contains(rust, "__tmp_0.borrow()") || strings.Contains(rust, "__tmp_0.borrow_mut()") ||
		strings.Contains(rust, "__tmp_0.lock()") {
		t.Fatalf("fmt result reassignment must not borrow the bare integer count:\n%s", rust)
	}
	if strings.Count(rust, "= Some(__tmp_0)") < 6 {
		t.Fatalf("fmt result reassignment must store each bare integer count into the wrapped local:\n%s", rust)
	}
}

func TestFmtStdStreamsPreserveNonStringMethodBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"os"
)

type stringer struct{}
func (stringer) String() string { return string([]byte{0xfa}) }

type failure struct{}
func (failure) Error() string { return string([]byte{0xfb}) }

func main() {
	fmt.Print(stringer{})
	fmt.Println(failure{})
	fmt.Fprint(os.Stdout, stringer{})
	fmt.Fprintln(os.Stderr, failure{})
}
`)

	if strings.Count(rust, "__go_string_method_result") < 4 ||
		strings.Count(rust, "__go_string_bytes(&__go_sprint_arg_0)") < 4 {
		t.Fatalf("non-string Stringer/error output must write canonical method-result bytes:\n%s", rust)
	}
}

func TestFmtStaticPercentSPreservesNonStringMethodBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"os"
)

type stringer struct{}
func (stringer) String() string { return string([]byte{0xfa}) }

type failure struct{}
func (failure) Error() string { return string([]byte{0xfb}) }

func main() {
	fmt.Printf("<%s>", stringer{})
	fmt.Fprintf(os.Stderr, "<%s>", failure{})
	_ = fmt.Sprintf("<%s>", stringer{})
}
`)

	if strings.Count(rust, "__go_sprintf_arg_0") < 3 ||
		strings.Count(rust, "__go_string_method_result") < 3 ||
		strings.Count(rust, "__go_string_bytes(&__go_sprintf_arg_0)") < 3 {
		t.Fatalf("static %%s must append canonical bytes returned by non-string Stringer/error values:\n%s", rust)
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
