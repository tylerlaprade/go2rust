package main

import (
	"strings"
	"testing"
)

// A format held in a variable is only known when the program runs. The
// arguments must reach the formatter; they used to be dropped.
func TestRuntimeFormatCallsGoSprintfWithTheArguments(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	f := "value: %d\n"
	fmt.Printf(f, 42)
	fmt.Println(fmt.Sprintf(f, 43))
}
`)

	for _, want := range []string{
		`print!("{}", go_sprintf(&(*f.borrow().as_ref().unwrap()).clone(), &[Box::new(42i64) as Box<dyn Any>], false))`,
		`format!("{}", go_sprintf(&(*f.borrow().as_ref().unwrap()).clone(), &[Box::new(43i64) as Box<dyn Any>], false))`,
		"fn go_sprintf(format: &str, args: &[Box<dyn Any>], wraps: bool) -> String {",
		"fn format_any(value: &dyn Any) -> String {",
		"fn __go_type_name(val: &dyn Any) -> &'static str {",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "go_sprintf_spread") {
		t.Fatalf("no call spreads a slice, so the spread form is not needed:\n%s", rust)
	}
}

// f(format, args...) hands the slice it received to the formatter.
func TestRuntimeFormatPassesASpreadSliceOn(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func logf(format string, args ...interface{}) {
	fmt.Printf(format+"\n", args...)
}

func main() {
	logf("count %d of %d", 1, 2)
}
`)

	for _, want := range []string{
		`print!("{}", go_sprintf_spread(&format!("{}{}", (*format.borrow().as_ref().unwrap()), "\n".to_string()), &args, false))`,
		"fn go_sprintf_spread(format: &str, args: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>, wraps: bool) -> String {",
		"let args_guard = guard.as_ref().map(|args| args.as_slice());",
		"go_sprintf(format, args_guard.as_deref().unwrap_or(&[]), wraps)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
}

// In fmt.Errorf %w prints as %v. Everywhere else fmt reports it as a bad verb.
func TestErrorfWithARuntimeFormatLetsWPrintAsV(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func errorf(format string, args ...interface{}) error {
	return fmt.Errorf(format, args...)
}

func sprintf(format string, args ...interface{}) string {
	return fmt.Sprintf(format, args...)
}

func main() {
	_ = errorf("%w", fmt.Errorf("inner"))
	_ = sprintf("%d", 1)
}
`)

	if !strings.Contains(rust, `Box::<dyn StdError>::from(format!("{}", go_sprintf_spread(&(*format.borrow().as_ref().unwrap()).clone(), &args, true)))`) {
		t.Fatalf("fmt.Errorf must ask for %%w to print as %%v:\n%s", rust)
	}
	if !strings.Contains(rust, `go_sprintf_spread(&(*format.borrow().as_ref().unwrap()).clone(), &args, false)`) {
		t.Fatalf("fmt.Sprintf must not:\n%s", rust)
	}
}

// A constant format is known when the program is translated, whether it is
// written in place or named. It needs no formatter at run time.
func TestConstantFormatIsExpandedAtTranslationTime(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

const layout = "%s has %d items\n"

func main() {
	fmt.Printf(layout, "cart", 3)
	fmt.Printf("%s has %d items\n", "cart", 4)
	fmt.Println(fmt.Sprintf(layout, "cart", 5))
}
`)

	if got := strings.Count(rust, `print!("{} has {} items\n", "cart".to_string(), `); got != 2 {
		t.Fatalf("want both Printf calls expanded, got %d in:\n%s", got, rust)
	}
	if !strings.Contains(rust, `format!("{} has {} items\n", "cart".to_string(), 5)`) {
		t.Fatalf("want Sprintf with a named constant expanded:\n%s", rust)
	}
	if strings.Contains(rust, "go_sprintf") {
		t.Fatalf("a constant format must not use the run-time formatter:\n%s", rust)
	}
}

// With goroutines the arguments are Send + Sync boxes behind a Mutex.
func TestRuntimeFormatUsesTheLockedFormsWithGoroutines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "fmt"

func logf(format string, args ...interface{}) {
	fmt.Printf(format, args...)
}

func main() {
	done := make(chan bool, 1)
	go func() { done <- true }()
	<-done
	logf("%d\n", 1)
}
`)

	for _, want := range []string{
		"fn go_sprintf(format: &str, args: &[Box<dyn Any + Send + Sync>], wraps: bool) -> String {",
		"fn go_sprintf_spread(format: &str, args: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>, wraps: bool) -> String {",
		"let guard = args.lock().unwrap();",
		"fn go_format_argument(spec: &GoFormatSpec, verb: char, value: &(dyn Any + Send + Sync)) -> Option<String> {",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
}

// What the formatter cannot do must stop the program, not print something
// that looks right.
func TestRuntimeFormatterFailsLoudlyOnWhatItDoesNotSupport(t *testing.T) {
	for _, want := range []string{
		`panic!("go2rust: %p is not supported in a format that is only known at run time");`,
		`panic!("go2rust: an argument index such as %[1]d is not supported in a format that is only known at run time");`,
		`panic!("go2rust: the # flag on %{} of a float is not supported in a format that is only known at run time", verb);`,
		`panic!("go2rust: %{}{}{} of a {} is not supported in a format that is only known at run time"`,
	} {
		if !strings.Contains(goSprintfFormatting+goSprintfEntry, want) {
			t.Fatalf("the run-time formatter lost a loud failure: %s", want)
		}
	}
}

// panic(fmt.Errorf(format, args...)) has its own lowering. It must hand the
// arguments to the formatter too.
func TestPanicWithARuntimeFormatKeepsTheArguments(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func panicf(format string, args ...interface{}) {
	panic(fmt.Errorf(format, args...))
}

func mustf(format string, code int) {
	panic(fmt.Sprintf(format, code))
}

func main() {
	panicf("code %d", 1)
	mustf("code %d", 2)
}
`)

	for _, want := range []string{
		`panic!("{}", go_sprintf_spread(&(*format.borrow().as_ref().unwrap()).clone(), &args, true))`,
		`panic!("{}", go_sprintf(&(*format.borrow().as_ref().unwrap()).clone(), &[Box::new({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any>], false))`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
}

// A []byte argument is recognized by its handle type, which differs between
// the two wrapper modes.
func TestRuntimeFormatterKnowsTheByteSliceHandleOfEachMode(t *testing.T) {
	source := `package main

import "fmt"

func main() {
	done := make(chan bool, 1)
	done <- true
	<-done
	f := "%s\n"
	fmt.Printf(f, []byte("hi"))
}
`
	plain := transpileTypedRegression(t, strings.Replace(source, "\tdone := make(chan bool, 1)\n\tdone <- true\n\t<-done\n", "", 1))
	if !strings.Contains(plain, "value.downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>()") || !strings.Contains(plain, "let guard = v.borrow();") {
		t.Fatalf("want the Rc handle of a []byte:\n%s", plain)
	}
	locked := transpileTypedConcurrentRegression(t, source)
	if !strings.Contains(locked, "value.downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>()") || !strings.Contains(locked, "let guard = v.lock().unwrap();") {
		t.Fatalf("want the Arc handle of a []byte:\n%s", locked)
	}
}

// f(format, args...) cannot be expanded while translating even when the
// format is known then: the number of arguments and their types are only
// known when the call runs. It used to pass the slice as one argument of a
// Rust format macro, which did not compile.
func TestSpreadArgumentsGoToTheRuntimeFormatterWhateverTheFormat(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"os"
)

const layout = "%v=%v\n"

func showPair(args ...interface{}) {
	fmt.Printf(layout, args...)
	text := fmt.Sprintf("%v|%v", args...)
	fmt.Println(text)
	fmt.Fprintf(os.Stdout, "%v+%v\n", args...)
	err := fmt.Errorf("%v/%v", args...)
	fmt.Println(err)
}

func main() {
	showPair("k", 1)
	fmt.Printf(layout, "direct", 2)
}
`)

	for _, want := range []string{
		`print!("{}", go_sprintf_spread(&`,
		`go_sprintf_spread(&"%v|%v".to_string(), &args, false)`,
		`go_sprintf_spread(&"%v+%v\n".to_string(), &args, false)`,
		`go_sprintf_spread(&"%v/%v".to_string(), &args, true)`,
		// Without a spread the constant format is still expanded.
		`print!("{}={}\n", "direct"`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
	for _, unwanted := range []string{
		`format_any_slice(&args)`,
	} {
		if strings.Contains(rust, unwanted) {
			t.Fatalf("the slice must not be one argument of a format macro, found %q in:\n%s", unwanted, rust)
		}
	}
}

// A closure gets its own clone of each variable it captures, under another
// name. The spread slice has to be that clone, or the closure takes the
// variable itself and the function cannot use it afterwards.
func TestSpreadSliceInAClosureIsTheCapturedClone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func later(format string, args ...interface{}) {
	build := func() string {
		return fmt.Sprintf(format, args...)
	}
	fmt.Println(build())
	fmt.Println(len(args))
}

func main() {
	later("%d and %s", 1, "two")
}
`)

	if !strings.Contains(rust, "let args_closure_clone = args.clone();") {
		t.Fatalf("the closure should capture a clone of args:\n%s", rust)
	}
	if !strings.Contains(rust, "&args_closure_clone, false)") {
		t.Fatalf("the formatter should receive the captured clone:\n%s", rust)
	}
	if strings.Contains(rust, ", &args, false)") {
		t.Fatalf("the closure must not use the outer variable:\n%s", rust)
	}
}

// fmt prints a value that has a String or an Error method as the text the
// method returns, so a width, a precision, %q and %x apply to that text. Any
// other struct, slice or map has them applied to each field or element,
// which the helper does not do: it stops the program for those. The helper
// can only tell the two apart if it is told which types have the method.
func TestRuntimeFormatterKnowsWhichValuesAreOneText(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"errors"
	"fmt"
)

type Point struct{ X, Y int }

type Status struct{ Code int }

func (s Status) String() string { return "status" }

func logf(format string, args ...interface{}) {
	fmt.Printf(format, args...)
}

func main() {
	logf("%v %v %v %v\n", Point{1, 2}, Status{1}, &Status{2}, errors.New("x"))
}
`)

	for _, want := range []string{
		"fn go_format_is_text(__value: &dyn Any) -> bool {",
		"|| __value.is::<Status>()",
		"|| __value.is::<Rc<RefCell<Option<Status>>>>()",
		"StdError>>>>>()",
		"if go_format_is_text(value) && !sharp_v && matches!(verb, 'v' | 's' | 'q' | 'x' | 'X') {",
		// What is left stops the program; it does not print an approximation.
		"if spec.width.is_none() && spec.precision.is_none() && !spec.space {",
		`panic!("go2rust: %s of a []byte that is not UTF-8 is not supported in a format that is only known at run time")`,
		"let verb = if verb == 'w' && wraps && !go_format_is_basic(arg.as_ref()) { 'v' } else { verb };",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
	for _, unwanted := range []string{
		"value.is::<Point>()",
		"__value.is::<Point>()",
	} {
		if strings.Contains(rust, unwanted) {
			t.Fatalf("found %q in:\n%s", unwanted, rust)
		}
	}
}

// The crate root imports every package-level variable and constant of the Go
// package, and Rust does not let a parameter or a local be the name of one.
// The run-time format helpers have many locals, so they live in a module of
// their own that imports only what it uses. A package with "var prefix", as
// go/scanner has, used to stop the generated crate from compiling.
func TestRuntimeFormatHelpersCannotClashWithPackageLevelNames(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

var prefix = "line "

func main() {
	f := "%s%d\n"
	fmt.Printf(f, prefix, 1)
}
`)

	for _, want := range []string{
		"fn go_format_is_text(__value: &dyn Any) -> bool {",
		"#[allow(unused_imports)]\nmod __go_runtime_format {\n    use super::{format_any, __go_type_name, go_format_is_text, go_format_named_basic_value};",
		"\npub(super) fn go_sprintf(format: &str, args: &[Box<dyn Any>], wraps: bool) -> String {",
		"}\nuse __go_runtime_format::go_sprintf;\n",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
}

// A literal format is turned into a Rust format only when every verb has a
// Rust format with the same output. Any other goes to go_sprintf at run
// time, as a format that is not a literal does.
func TestFormatExpandsToRustOnlyForVerbsWithTheSameOutput(t *testing.T) {
	for format, want := range map[string]bool{
		`"%d %s %v %t %x %X %c %q"`:  false,
		`"%d %s %v %t %x %X %c"`:     true,
		`"%5d|%-5d|%05d|%8s|%-8s"`:   true,
		`"%.2f %+d %+.1f %#x %T %%"`: true,
		`"%8.2f"`:                    false,
		`"%05.1f"`:                   false,
		`"%e %E %g %G %o"`:           false,
		`"%.3d"`:                     false,
		`"%.3g"`:                     false,
		`"%+5d"`:                     false,
		`"% d"`:                      false,
		`"%*d"`:                      false,
		`"%[1]d"`:                    false,
		`"%10v"`:                     false,
		"`%d\n`":                     true,
		`"100%"`:                     false,
	} {
		if got := formatExpandsToRust(format); got != want {
			t.Errorf("formatExpandsToRust(%s) = %v, want %v", format, got, want)
		}
	}
}

// fmt.Print puts a space between two operands only when neither is a
// string.
func TestFmtPrintSpacesOnlyBetweenNonStrings(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Name string

func main() {
	fmt.Print(1, 2, "a", 3, Name("b"), "c", 4.5)
}
`)
	if !strings.Contains(rust, `print!("{} {}{}{}{}{}{}"`) {
		t.Fatalf("expected spaces only between the two integers:\n%s", rust)
	}
}
