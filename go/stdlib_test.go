package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestFmtFprintfPointerVerbUsesInterfaceHandleAddress(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"fmt"
	"strings"
)

type Node interface {
	Pos() int
}

type CommentMap map[Node][]string

func Write(buf *strings.Builder, cmap CommentMap, s string) {
	for node := range cmap {
		fmt.Fprintf(buf, "\t%p  %20s:  %s\n", node, s, "ok")
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)

	if !strings.Contains(rust, `format!("\t{:p}  {:>20}:  {}\n"`) {
		t.Fatalf("%%p should lower to Rust pointer formatting:\n%s", rust)
	}
	if !strings.Contains(rust, "::as_ptr(&node)") {
		t.Fatalf("%%p for a wrapped interface handle should format the handle address:\n%s", rust)
	}
	if strings.Contains(rust, `format!("\t{}  {:>20}:  {}\n", node`) {
		t.Fatalf("%%p should not display-format the wrapped interface handle:\n%s", rust)
	}
}

func TestFmtSprintfPrecisionGAndSignedDecimalVerbs(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "fmt"

func render(m float64, e int) string {
	return fmt.Sprintf("%.6ge%+d", m, e) + fmt.Sprintf("%g", m)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "%.6g") || strings.Contains(rust, "%+d") || strings.Contains(rust, `"%g"`) {
		t.Fatalf("fmt.Sprintf should not leave Go precision/sign verbs in Rust format strings:\n%s", rust)
	}
	if !strings.Contains(rust, `format!("{:.6}e{:+}"`) {
		t.Fatalf("fmt.Sprintf should lower Go %%g precision and signed decimal verbs:\n%s", rust)
	}
}

func TestBuiltinPrintUsesRustStderrMacro(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func warn(field string) {
	print("missing ", field, "\n")
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, `eprint!("{}{}{}"`) {
		t.Fatalf("builtin print should emit Rust stderr macro without inserted spaces:\n%s", rust)
	}
	if strings.Contains(rust, "print(Rc::") || strings.Contains(rust, "print(Arc::") {
		t.Fatalf("builtin print must not emit a call to a nonexistent Rust print function:\n%s", rust)
	}
}

func TestConcurrentFmtPrintlnVariadicAnyUsesVariadicFormatter(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "fmt"

func log(values ...any) {
	fmt.Println(values...)
}

func main() {
	go func() {}()
	log("x", 7, true)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	prevDetector := GetConcurrencyDetector()
	detector := NewConcurrencyDetector()
	detector.AnalyzeFile(file)
	SetConcurrencyDetector(detector)
	defer SetConcurrencyDetector(prevDetector)

	rust, _, _ := Transpile(file, fset, typeInfo)

	if !strings.Contains(rust, "format_any_variadic(&values)") {
		t.Fatalf("fmt.Println(values...) should use variadic any formatting:\n%s", rust)
	}
	if !strings.Contains(rust, "Vec<Box<dyn Any + Send + Sync>>") {
		t.Fatalf("concurrent any slice formatter should accept Send+Sync any elements:\n%s", rust)
	}
	if strings.Contains(rust, "format_any_slice(&values)") {
		t.Fatalf("fmt.Println(values...) should not format the variadic slice with brackets:\n%s", rust)
	}
}

func TestPanicAnyFormatsInterfacePayload(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func fail(value any) {
	panic(value)
}
`)

	if strings.Contains(rust, "panic!(\"{}\", (*value.") {
		t.Fatalf("panic(any) should not display the raw Box<dyn Any> payload:\n%s", rust)
	}
	if !strings.Contains(rust, "format_any(") {
		t.Fatalf("panic(any) should format through format_any in single-threaded lowering:\n%s", rust)
	}
}

func TestConcurrentPanicAnyUsesPanicAnyPayload(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func fail(value any) {
	done := make(chan bool)
	_ = done
	panic(value)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	prevDetector := GetConcurrencyDetector()
	detector := NewConcurrencyDetector()
	detector.AnalyzeFile(file)
	SetConcurrencyDetector(detector)
	defer SetConcurrencyDetector(prevDetector)

	rust, _, _ := Transpile(file, fset, typeInfo)

	if strings.Contains(rust, "panic!(\"{}\", (*value.") {
		t.Fatalf("concurrent panic(any) should not display the raw Box<dyn Any> payload:\n%s", rust)
	}
	if !strings.Contains(rust, "std::panic::panic_any(") || !strings.Contains(rust, "go_any_clone(") {
		t.Fatalf("concurrent panic(any) should preserve the payload through panic_any/go_any_clone:\n%s", rust)
	}
}

func TestCopyToSliceTypeAssertionUsesBareDestination(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func fill(v any, src []int) int {
	return copy(v.([]int), src)
}
`)

	if strings.Contains(rust, "downcast_ref::<Vec<i32>>().expect(\"type assertion failed\").clone()\n        } else") &&
		(strings.Contains(rust, "}).borrow()") || strings.Contains(rust, "}).lock()")) {
		t.Fatalf("copy destination from slice type assertion should not be borrowed as a wrapped handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut _dst =") {
		t.Fatalf("copy destination from slice type assertion should use a mutable bare Vec temp:\n%s", rust)
	}
}

func TestBuiltinClearNamedSliceAndSliceExprUsesTypedBuiltin(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func wipe(z nat, buf []byte) {
	clear(z)
	clear(buf[1:len(buf)])
}

func clearValue(x int) int {
	return x + 1
}
`)

	if strings.Contains(rust, "clear.lock()") || strings.Contains(rust, "__f_ptr") {
		t.Fatalf("builtin clear should not lower as a function value call:\n%s", rust)
	}
	if !strings.Contains(rust, "let __clear_holder = { let __named_slice =") {
		t.Fatalf("clear(named slice) should mutate the named slice inner handle:\n%s", rust)
	}
	if !strings.Contains(rust, "*__clear_elem = Word(") {
		t.Fatalf("clear(named slice) should write the typed element zero value:\n%s", rust)
	}
	if !strings.Contains(rust, "let __clear_start = (1) as usize") ||
		!strings.Contains(rust, "__clear_seq[__clear_i] = 0;") {
		t.Fatalf("clear(slice expr) should zero the selected byte range:\n%s", rust)
	}
}

func TestUserFunctionNamedClearDoesNotUseBuiltinClear(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func clear(x int) int {
	return x + 1
}

func use() int {
	return clear(1)
}
`)

	if strings.Contains(rust, "__clear_holder") {
		t.Fatalf("user-defined clear should not lower as the builtin clear:\n%s", rust)
	}
	if !strings.Contains(rust, "clear(") {
		t.Fatalf("user-defined clear should remain a normal function call:\n%s", rust)
	}
}
