package main

import (
	"regexp"
	"strings"
	"testing"
)

func TestGoSliceModeSharesBackingAndCopiesHeaders(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "")
	rust := transpileTypedRegression(t, `package main

import "fmt"

func reset(xs []int) {
	xs = nil
	_ = xs
}

func main() {
	s := []int{1, 2, 3}
	view := s[1:]
	view[0] = 9
	grown := append(view, 4)
	var other []int
	other = s
	reset(s)
	buf := make([]byte, 0, 8)
	fmt.Println(s, view, grown, other, buf)
}
`)
	for _, want := range []string{
		"__seq.reslice3(__low, __high, __max)",
		"let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended)))",
		"let mut xs = Rc::new(RefCell::new((*xs.borrow()).clone()));",
		"*other.borrow_mut() = __slice_header;",
		"(0..(8) as usize).map(|_| 0).collect(); v.truncate((0) as usize); v",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("GoSlice mode output is missing %q:\n%s", want, rust)
		}
	}
}

func TestGoSliceModeTurnsOffWithZero(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "0")
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	s := []int{1, 2, 3}
	fmt.Println(s[1:])
}

`)
	if strings.Contains(rust, "GoSlice") {
		t.Fatalf("GO2RUST_GOSLICE=0 must turn GoSlice off:\n%s", rust)
	}
}

func TestGoSliceInterfaceAppendCopiesHeadersAndPreflightsGrowth(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "")
	rust := transpileTypedRegression(t, `package main

func main() {
	base := []any{1, 2}
	_ = append(base[:1:1], 3)
	_ = append(base[:1:2], 3, 4)
	_ = append(base[:1:2], 3)
}
`)
	for _, want := range []string{
		"let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default()",
		"self.ensure_boxed_capacity(1);",
		"self.ensure_boxed_capacity(__values.len());",
		"let __values = __values.into_boxed_values();\n        self.ensure_boxed_capacity(__values.len());",
		"None => self.as_slice().iter().map(|__value| go_any_clone(__value.as_ref())).collect()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("boxed GoSlice append output is missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "push_without_copy") ||
		strings.Contains(rust, "__seq_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone") {
		t.Fatalf("boxed GoSlice append must preserve the reslice header and detach before a full write:\n%s", rust)
	}
}

func TestGoSliceBackingAccessCannotEscapeItsLockGuard(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "")
	rust := transpileTypedRegression(t, `package main

func main() {
	s := []int{1, 2}
	t := s[:]
	t[0] = s[1]
	for _, value := range t {
		_ = value
	}
}
`)

	for _, want := range []string{
		"backing: std::sync::Arc<std::sync::RwLock<std::vec::Vec<T>>>",
		"pub struct GoSliceReadGuard<'a, T>",
		"pub struct GoSliceWriteGuard<'a, T>",
		"s.borrow().as_ref().unwrap().get_cloned((1) as usize)",
		"t.borrow_mut().as_mut().unwrap().set(__idx, __rhs)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("guarded GoSlice output is missing %q:\n%s", want, rust)
		}
	}
	for _, forbidden := range []string{
		"Arc<std::cell::UnsafeCell<std::vec::Vec<T>>>",
		"unsafe impl<T: Send> Send for GoSlice<T>",
		"unsafe impl<T: Sync> Sync for GoSlice<T>",
		"impl<T> std::ops::Deref for GoSlice<T>",
		"impl<T> std::ops::DerefMut for GoSlice<T>",
	} {
		if strings.Contains(rust, forbidden) {
			t.Fatalf("GoSlice backing access must stay behind a lock guard; found %q:\n%s", forbidden, rust)
		}
	}
}

func TestGoSliceCompositeLiteralRangeIteratesThroughGuard(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "")
	rust := transpileTypedRegression(t, `package main

func main() {
	for _, item := range []struct{ name string }{{"widget"}, {"gadget"}} {
		_ = item
	}
}
`)

	if !strings.Contains(rust, `].as_slice().iter()`) {
		t.Fatalf("slice literal range must iterate through its read guard:\n%s", rust)
	}
	if strings.Contains(rust, ` in &vec![`) {
		t.Fatalf("slice literal range cannot borrow a GoSlice read guard as IntoIterator:\n%s", rust)
	}
}

func TestGoSliceMethodResultIndexClonesInsideGuard(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "")
	rust := transpileTypedRegression(t, `package main

type item struct{}
type list struct{}

func (*list) items() []*item { return nil }
func pick(values *list, index int) *item {
	return values.items()[index]
}
`)

	if !strings.Contains(rust, `.get_cloned(`) {
		t.Fatalf("indexing a slice returned by a method must clone inside its read guard:\n%s", rust)
	}

	concurrent := transpileTypedConcurrentRegression(t, `package main

type item struct{}
type list struct{}

func (*list) items() []*item { return nil }
func pick(values *list, index int) *item {
	go func() {}()
	return values.items()[index]
}
`)
	if !strings.Contains(concurrent, `__seq.get_cloned(`) {
		t.Fatalf("concurrent multiline indexing of a method-returned slice must clone inside its read guard:\n%s", concurrent)
	}
}

func TestGoSliceStringerFormatterKeepsViewGuardAlive(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "")
	got := goSliceHelperText("format_slice_wrapped_stringer_values(s.as_ref())")
	want := "{ let __view = s.go_slice_view(); format_slice_wrapped_stringer_values(__view.as_ref()) }"
	if got != want {
		t.Fatalf("GoSlice stringer formatter = %q, want %q", got, want)
	}
}

func TestInvalidUTF8StringConversionsUseLosslessByteHelpers(t *testing.T) {
	t.Setenv("GO2RUST_GOSLICE", "")
	rust := transpileTypedRegression(t, `package main

func roundTrip(b []byte) []byte {
	s := string(b)
	_ = len(s)
	_ = s[0]
	_ = s[1:]
	return []byte(s)
}
`)

	for _, want := range []string{
		"__go_string_from_bytes(std::vec::Vec::from(",
		"__go_string_len(&__s)",
		"__go_string_bytes(__s)[",
		"__go_string_slice(__s, __low, __go_string_len(__s))",
		"Vec::from(__go_string_bytes(&",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("invalid UTF-8 string lowering is missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "String::from_utf8(") {
		t.Fatalf("[]byte to string must not reject invalid Go string bytes:\n%s", rust)
	}
	if !strings.Contains(goSliceRuntime, "const __GO_STRING_BYTES_PREFIX") {
		t.Fatal("GoSlice runtime is missing the reversible arbitrary-byte string encoding")
	}
}

// withoutGoSliceUses rewrites every `use crate::{...};` in code without the
// names that GoSlice adds to each of them, so that a test can check the
// helpers a module imports for its own use.
func withoutGoSliceUses(code string) string {
	goSliceNames := map[string]bool{}
	for _, name := range goSliceImportNames {
		goSliceNames[name] = true
	}
	re := regexp.MustCompile(`use crate::\{([^}]*)\};\n`)
	return re.ReplaceAllStringFunc(code, func(block string) string {
		inner := re.FindStringSubmatch(block)[1]
		var names []string
		for _, name := range strings.FieldsFunc(inner, func(r rune) bool { return r == ',' || r == ' ' || r == '\n' }) {
			if !goSliceNames[name] {
				names = append(names, name)
			}
		}
		if len(names) == 0 {
			return ""
		}
		return formatCrateUse(names)
	})
}
