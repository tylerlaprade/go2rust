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

func TestLenOfPointerToSliceDerefUsesSliceHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type ranges struct {
	p *[]rune
}

func (ra ranges) Len() int {
	return len(*ra.p) / 2
}
`)

	if strings.Contains(rust, "(*(*self.p") {
		t.Fatalf("len of pointer-to-slice dereference should not borrow the raw Vec as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __slice_holder = self.p.clone()") {
		t.Fatalf("len of pointer-to-slice dereference should measure the cloned slice handle:\n%s", rust)
	}
}

func TestFmtFprintfSourceMappedStringsBuilderUsesGeneratedWriteString(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"fmt"
	"strings"
)

func build(s string) string {
	var buf strings.Builder
	fmt.Fprintf(&buf, "%s", s)
	return buf.String()
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"strings": "strings"})

	if strings.Contains(rust, ".push_str(") {
		t.Fatalf("source-mapped fmt.Fprintf should not use native String::push_str:\n%s", rust)
	}
	if !strings.Contains(rust, ".write_string(") {
		t.Fatalf("source-mapped fmt.Fprintf should call generated Builder.write_string:\n%s", rust)
	}
}

func TestSourceMappedRegexpMustCompileCallsGeneratedFunction(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "regexp"

func compile(pattern string) *regexp.Regexp {
	return regexp.MustCompile(pattern)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"regexp": "regexp"})

	if strings.Contains(rust, "GoRegexp") {
		t.Fatalf("source-mapped regexp.MustCompile should not use the GoRegexp bridge:\n%s", rust)
	}
	if !strings.Contains(rust, "regexp::must_compile(") {
		t.Fatalf("source-mapped regexp.MustCompile should call the generated regexp package:\n%s", rust)
	}
}

func TestFmtFprintlnStringsBuilderWritesToBuilder(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"fmt"
	"strings"
)

func build() string {
	var buf strings.Builder
	fmt.Fprintln(&buf)
	return buf.String()
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"strings": "strings"})

	if strings.Contains(rust, "println!(") {
		t.Fatalf("fmt.Fprintln with a Builder target should not print to stdout:\n%s", rust)
	}
	if !strings.Contains(rust, ".write_string(") {
		t.Fatalf("source-mapped fmt.Fprintln should call generated Builder.write_string:\n%s", rust)
	}
}

func TestTimeUnixExpandsMultiResultArgument(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "time"

type stamp struct{}

func (stamp) Unix() (int64, int64) {
	return 7, 9
}

func convert(s stamp) time.Time {
	return time.Unix(s.Unix())
}
`)

	if strings.Contains(rust, ".unix().borrow()") || strings.Contains(rust, ".unix().lock()") {
		t.Fatalf("time.Unix should not unwrap a multi-result call as a single wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "let (__multi_arg_0, __multi_arg_1) =") {
		t.Fatalf("time.Unix should bind the multi-result argument before expansion:\n%s", rust)
	}
	if !strings.Contains(rust, "GoTime::from_unix(__multi_arg_0 as i64, __multi_arg_1 as i64)") {
		t.Fatalf("time.Unix should pass expanded result slots as scalar arguments:\n%s", rust)
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

func TestFmtSprintfHexNamedIntegerUsesUnderlyingValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Word uint

func render(w Word) string {
	return fmt.Sprintf("%#x", w)
}
`)

	directWrapper := `format!("{:#x}", { let __v = (*w.borrow().as_ref().unwrap()).clone(); __v })`
	if strings.Contains(rust, directWrapper) {
		t.Fatalf("fmt.Sprintf %%#x should not format the named integer wrapper directly:\n%s", rust)
	}
	want := `format!("{:#x}", (*(*w.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()))`
	if !strings.Contains(rust, want) {
		t.Fatalf("fmt.Sprintf %%#x should format the named integer's underlying value, missing %q:\n%s", want, rust)
	}
}

func TestStrconvItoaParenthesizesConvertedNamedInteger(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "strconv"

type Kind uint

func render(k Kind) string {
	return strconv.Itoa(int(k))
}
`)

	if strings.Contains(rust, "as i32.to_string()") {
		t.Fatalf("strconv.Itoa should parenthesize converted operands before to_string:\n%s", rust)
	}
	if !strings.Contains(rust, " as i32).to_string()") {
		t.Fatalf("strconv.Itoa should call to_string on the converted value:\n%s", rust)
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

func TestBuiltinCopyNamedSliceUsesInnerHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func shift(z nat, m, n uint32) int {
	return copy(z, z[m-n:])
}
`)

	if strings.Contains(rust, "(*z.borrow().as_ref().unwrap()).len()") {
		t.Fatalf("copy named-slice destination should not call len on the named wrapper:\n%s", rust)
	}
	if strings.Contains(rust, "(*nat(") {
		t.Fatalf("copy named-slice source should not borrow a named slice value as a wrapper handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let _dst_holder = { let __named_slice = (*z.borrow().as_ref().unwrap()).0.clone(); __named_slice }") {
		t.Fatalf("copy named-slice destination should use the inner slice handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __slice_holder = { let __named_slice = (*z.borrow().as_ref().unwrap()).0.clone(); __named_slice }") {
		t.Fatalf("copy named-slice source slice should read from the inner slice handle:\n%s", rust)
	}
}

func TestBuiltinCopyNamedSliceDestinationSliceUsesInnerHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func fill(dst nat, src nat, d int) int {
	return copy(dst[d:], src)
}
`)

	if strings.Contains(rust, "(*nat(") {
		t.Fatalf("copy named-slice destination slice should not borrow a constructed named slice as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let _dst_holder = { let __named_slice = (*dst.borrow().as_ref().unwrap()).0.clone(); __named_slice }") {
		t.Fatalf("copy named-slice destination slice should use the inner slice handle:\n%s", rust)
	}
	if !strings.Contains(rust, "[_dst_start + _i]") {
		t.Fatalf("copy named-slice destination slice should write with the slice offset:\n%s", rust)
	}
}

func TestBuiltinCopyNamedSliceArrayElementSourceUsesBareNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func fill(z nat, powers [2]nat) int {
	return copy(z, powers[0])
}
`)

	if strings.Contains(rust, "}.borrow().as_ref().unwrap()).0.clone()") ||
		strings.Contains(rust, "}.lock().unwrap().as_ref().unwrap()).0.clone()") {
		t.Fatalf("copy named-slice array element source should not borrow the bare named value:\n%s", rust)
	}
	if !strings.Contains(rust, "__named_slice.0.clone()") {
		t.Fatalf("copy named-slice array element source should use the named value inner handle:\n%s", rust)
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

func TestSlicesContainsKeywordInterfaceNilChecksElementSlots(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "slices"

type Type interface {
	String() string
}

func hasNil(list []Type) bool {
	return slices.Contains(list, nil)
}
`)

	if strings.Contains(rust, "None.clone()") ||
		strings.Contains(rust, "__item.__go_eq_type_(") {
		t.Fatalf("slices.Contains over named-interface nil should inspect nullable element slots:\n%s", rust)
	}
	if !strings.Contains(rust, "__item_guard.as_ref().is_none()") {
		t.Fatalf("slices.Contains over named-interface nil should check element slots for None:\n%s", rust)
	}
}

func TestSlicesContainsKeywordInterfaceUsesSafeEqualitySuffix(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "slices"

type Type interface {
	String() string
}

func has(list []Type, needle Type) bool {
	return slices.Contains(list, needle)
}
`)

	if strings.Contains(rust, "__go_eq_r#type") {
		t.Fatalf("slices.Contains over keyword-named interface should not use raw identifier suffix:\n%s", rust)
	}
	if strings.Contains(rust, "__item.__go_eq_type_(") {
		t.Fatalf("slices.Contains over named-interface values should unwrap element handles before equality:\n%s", rust)
	}
	if !strings.Contains(rust, "__left.as_ref().__go_eq_type_(__right.as_ref())") {
		t.Fatalf("slices.Contains over keyword-named interface should use identifier-safe equality suffix:\n%s", rust)
	}
}

func TestSlicesContainsPointerSliceUsesPointerIdentity(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "slices"

type Var struct {
	name string
}

func has(lhs []*Var, obj *Var) bool {
	return slices.Contains(lhs, obj)
}
`)

	if strings.Contains(rust, "__slice.contains(&__value)") {
		t.Fatalf("slices.Contains over pointer slices should not compare pointee values:\n%s", rust)
	}
	if !strings.Contains(rust, "::ptr_eq(__item, &__value)") || !strings.Contains(rust, "__both_nil") {
		t.Fatalf("slices.Contains over pointer slices should use handle identity with nil handling:\n%s", rust)
	}
}

func TestBuiltinNewSliceUsesTurbofishDefault(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func alloc() *[]int {
	return new([]int)
}
`)

	if strings.Contains(rust, "Vec<i32>::default()") {
		t.Fatalf("new([]T) should not emit generic type path without turbofish:\n%s", rust)
	}
	if !strings.Contains(rust, "Vec::<i32>::default()") {
		t.Fatalf("new([]T) should emit a turbofish default constructor:\n%s", rust)
	}
}
