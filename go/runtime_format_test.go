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
		`print!("{}", go_sprintf(&(*f.borrow().as_ref().unwrap()).clone(), &[Box::new(42) as Box<dyn Any>], false))`,
		`format!("{}", go_sprintf(&(*f.borrow().as_ref().unwrap()).clone(), &[Box::new(43) as Box<dyn Any>], false))`,
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
		"go_sprintf(format, guard.as_deref().unwrap_or(&[]), wraps)",
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
