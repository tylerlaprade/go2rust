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
