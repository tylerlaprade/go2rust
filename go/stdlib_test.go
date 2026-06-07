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

func TestFmtErrorfFormatsPackageGlobalErrorSelectorByHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"fmt"
	"os/exec"
)

func wrap() error {
	return fmt.Errorf("missing: %s", exec.ErrNotFound)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"os/exec": "os_exec"})

	if strings.Contains(rust, "ErrNotFound.lock().unwrap().as_ref().unwrap()).clone()).lock()") ||
		strings.Contains(rust, "ErrNotFound.borrow().as_ref().unwrap()).clone()).borrow()") {
		t.Fatalf("fmt.Errorf should not clone the boxed error before formatting it:\n%s", rust)
	}
	if !strings.Contains(rust, "format!(\"{}\", (*os_exec::ErrNotFound.lock().unwrap().as_ref().unwrap()))") &&
		!strings.Contains(rust, "format!(\"{}\", (*os_exec::ErrNotFound.borrow().as_ref().unwrap()))") {
		t.Fatalf("fmt.Errorf should format the package-global error handle directly:\n%s", rust)
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

func TestSortSliceLowersToIndexSortWithoutReflectlite(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sort"

func order(names []string) {
	sort.Slice(names, func(i, j int) bool {
		return names[i] < names[j]
	})
}
`)

	if strings.Contains(rust, "sort::slice") ||
		strings.Contains(rust, "reflectlite") ||
		strings.Contains(rust, "Box::new(names") {
		t.Fatalf("sort.Slice should lower at the typed call site without boxing through reflectlite:\n%s", rust)
	}
	for _, want := range []string{
		"let __sort_target = ",
		".clone(); let __sort_less = ",
		"__less(",
		"__sort_values.swap(__sort_j, __sort_j - 1)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("sort.Slice lowering missing %q:\n%s", want, rust)
		}
	}
}

func TestSourceMappedSortSliceStillLowersToIndexSortWithoutReflectlite(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "sort"

func order(names []string) {
	sort.Slice(names, func(i, j int) bool {
		return names[i] < names[j]
	})
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"sort": "sort"})

	if strings.Contains(rust, "sort::slice") ||
		strings.Contains(rust, "internal_reflectlite") ||
		strings.Contains(rust, "Box::new(names") {
		t.Fatalf("source-mapped sort.Slice should still lower at the typed call site without reflectlite:\n%s", rust)
	}
	if !strings.Contains(rust, "__sort_values.swap(__sort_j, __sort_j - 1)") {
		t.Fatalf("source-mapped sort.Slice lowering should emit the typed in-place sort:\n%s", rust)
	}
}

func TestReflectTypeOfEmitsKindAndElemMetadata(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "reflect"

type Value interface {
	String() string
}

type Flag struct {
	Value Value
}

func typeName(flag *Flag) string {
	typ := reflect.TypeOf(flag.Value)
	if typ.Kind() == reflect.Pointer {
		typ = typ.Elem()
	}
	return typ.String()
}
`)

	for _, want := range []string{
		"kind: ",
		"elem: ",
		"fn kind(&self)",
		"fn elem(&self)",
		"reflect_Kind(20)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("reflect.TypeOf should emit kind/elem metadata %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "GoReflectType { name:") && !strings.Contains(rust, "fields:") {
		t.Fatalf("reflect.TypeOf metadata should preserve existing field metadata:\n%s", rust)
	}
}

func TestReflectValueOfUnsupportedInterfaceIsTypedLoudPanic(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "reflect"

type Text interface {
	MarshalText() ([]byte, error)
}

func kind(v Text) reflect.Kind {
	rv := reflect.ValueOf(v)
	return rv.Kind()
}
`)

	if strings.Contains(rust, `let mut rv = unimplemented!("reflect.ValueOf`) {
		t.Fatalf("unsupported reflect.ValueOf should not emit an untyped unimplemented value:\n%s", rust)
	}
	if !strings.Contains(rust, `unsupported: Some("reflect.ValueOf requires statically known pointer-to-struct type")`) {
		t.Fatalf("unsupported reflect.ValueOf should mark the local reflect value as unsupported:\n%s", rust)
	}
	if !strings.Contains(rust, `panic!("{}: {}", op, message)`) {
		t.Fatalf("unsupported reflect.ValueOf methods should remain loud panic paths:\n%s", rust)
	}
}

func TestNamedStringTypeConversionClonesStoredString(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type stringValue string

func (s *stringValue) String() string {
	return string(*s)
}

func copyString(v stringValue) string {
	return string(v)
}
`)

	if strings.Contains(rust, "(*self).clone().to_string()") {
		t.Fatalf("string conversion from named string receiver should not route through Display:\n%s", rust)
	}
	if !strings.Contains(rust, "self.0") || !strings.Contains(rust, ".as_ref().unwrap()).clone()") {
		t.Fatalf("string conversion from named string should clone the stored inner String:\n%s", rust)
	}
}

func TestSortSearchLowersBinarySearchWithoutBridge(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sort"

func findAtLeast(values []int, target int) int {
	return sort.Search(len(values), func(i int) bool {
		return values[i] >= target
	})
}
`)

	if strings.Contains(rust, "sort::search") {
		t.Fatalf("sort.Search should lower at the typed call site without using the bridge:\n%s", rust)
	}
	for _, want := range []string{
		"while __sort_i < __sort_j",
		"let __sort_h =",
		"__pred(",
		"__sort_i }",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("sort.Search lowering missing %q:\n%s", want, rust)
		}
	}
}

func TestSortSearchShortDeclResultRegistersBareIndex(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sort"

func contains(values []int, target int) bool {
	i := sort.Search(len(values), func(pos int) bool {
		return values[pos] >= target
	})
	return i < len(values) && values[i] == target
}
`)

	if strings.Contains(rust, "sort::search") {
		t.Fatalf("sort.Search should lower at the typed call site without using the bridge:\n%s", rust)
	}
	if strings.Contains(rust, "i.borrow()") || strings.Contains(rust, "i.lock()") {
		t.Fatalf("sort.Search result short declaration should register i as a bare scalar:\n%s", rust)
	}
	for _, want := range []string{
		"let mut i = { let mut __sort_i: i32 = 0;",
		"i <",
		"[(i) as usize]",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("sort.Search short-decl lowering missing %q:\n%s", want, rust)
		}
	}
}

func TestSortFindLowersBinarySearchTupleWithoutBridge(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sort"

func findValue(values []int, target int) (int, bool) {
	return sort.Find(len(values), func(i int) int {
		if values[i] < target {
			return 1
		}
		if values[i] > target {
			return -1
		}
		return 0
	})
}
`)

	if strings.Contains(rust, "sort::find") {
		t.Fatalf("sort.Find should lower at the typed call site without using the bridge:\n%s", rust)
	}
	for _, want := range []string{
		"while __sort_i < __sort_j",
		"let __sort_found =",
		"__cmp(",
		"(__sort_i, __sort_found)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("sort.Find lowering missing %q:\n%s", want, rust)
		}
	}
}

func TestSortInterfaceSortLowersToLenLessSwapWithoutBridge(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sort"

type items []int

func (x items) Len() int {
	return len(x)
}

func (x items) Less(i, j int) bool {
	return x[i] < x[j]
}

func (x items) Swap(i, j int) {
	x[i], x[j] = x[j], x[i]
}

func order(x items) {
	sort.Sort(x)
}
`)

	if strings.Contains(rust, "sort::sort") {
		t.Fatalf("sort.Sort should lower at the typed call site without using the bridge:\n%s", rust)
	}
	for _, want := range []string{
		"let mut __sort_data = (*x",
		"let __sort_len = __sort_data.len()",
		"__sort_data.less(",
		"__sort_data.swap(",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("sort.Sort lowering missing %q:\n%s", want, rust)
		}
	}
}

func TestSortInterfaceSortPointerReceiverBorrowsReceiverWithoutBridge(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sort"

type items []int

func (x *items) Len() int {
	return len(*x)
}

func (x *items) Less(i, j int) bool {
	return (*x)[i] < (*x)[j]
}

func (x *items) Swap(i, j int) {
	(*x)[i], (*x)[j] = (*x)[j], (*x)[i]
}

func order(x *items) {
	sort.Sort(x)
}
`)

	if strings.Contains(rust, "sort::sort") {
		t.Fatalf("sort.Sort on pointer receiver should lower without using the bridge:\n%s", rust)
	}
	if strings.Contains(rust, "__recv_ptr") {
		t.Fatalf("sort.Sort on pointer receiver should not cast the wrapped pointee to a raw named-slice pointer:\n%s", rust)
	}
	for _, want := range []string{
		"let mut __sort_data = (*x",
		"let __sort_len = __sort_data.len()",
		"__sort_data.less(",
		"__sort_data.swap(",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("sort.Sort pointer lowering missing %q:\n%s", want, rust)
		}
	}
}

func TestSortInterfaceSortCurrentNamedSliceReceiverUsesInnerHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sort"

type items []int

func (x items) Len() int {
	return len(x)
}

func (x items) Less(i, j int) bool {
	return x[i] < x[j]
}

func (x items) Swap(i, j int) {
	x[i], x[j] = x[j], x[i]
}

func (x items) order() {
	sort.Sort(x)
}
`)

	if strings.Contains(rust, "sort::sort") {
		t.Fatalf("sort.Sort on a named-slice receiver should lower without using the bridge:\n%s", rust)
	}
	for _, want := range []string{
		"let mut __sort_data = items(self.0.clone())",
		"let __sort_len = __sort_data.len()",
		"__sort_data.less(",
		"__sort_data.swap(",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("sort.Sort receiver lowering missing %q:\n%s", want, rust)
		}
	}
}

func TestSortInterfaceSortNamedStructLiteralUsesBareReceiver(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sort"

type ranges struct {
	p *[]int
}

func (ra ranges) Len() int {
	return len(*ra.p)
}

func (ra ranges) Less(i, j int) bool {
	return (*ra.p)[i] < (*ra.p)[j]
}

func (ra ranges) Swap(i, j int) {
	(*ra.p)[i], (*ra.p)[j] = (*ra.p)[j], (*ra.p)[i]
}

func order(p *[]int) {
	sort.Sort(ranges{p})
}
`)

	if strings.Contains(rust, "sort::sort") {
		t.Fatalf("sort.Sort on a named struct literal should lower without using the bridge:\n%s", rust)
	}
	if strings.Contains(rust, "ranges { p: p.clone(), ..Default::default() }.borrow()") ||
		strings.Contains(rust, "ranges { p: p.clone(), ..Default::default() }.lock()") {
		t.Fatalf("sort.Sort on a named struct literal should use the bare literal as the receiver:\n%s", rust)
	}
	for _, want := range []string{
		"let mut __sort_data = ranges { p: p.clone(), ..Default::default() }",
		"let __sort_len = __sort_data.len()",
		"__sort_data.less(",
		"__sort_data.swap(",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("sort.Sort struct-literal lowering missing %q:\n%s", want, rust)
		}
	}
}

func TestSortInterfaceSortNamedSliceConversionUsesBareReceiver(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sort"

type pkg struct {
	path string
}

type byPath []*pkg

func (p byPath) Len() int {
	return len(p)
}

func (p byPath) Less(i, j int) bool {
	return p[i].path < p[j].path
}

func (p byPath) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func order(list []*pkg) {
	sort.Sort(byPath(list))
}
`)

	if strings.Contains(rust, "sort::sort") {
		t.Fatalf("sort.Sort on a named slice conversion should lower without using the bridge:\n%s", rust)
	}
	if strings.Contains(rust, "byPath(list.clone()).borrow()") ||
		strings.Contains(rust, "byPath(list.clone()).lock()") {
		t.Fatalf("sort.Sort on a named slice conversion should use the bare conversion as the receiver:\n%s", rust)
	}
	for _, want := range []string{
		"let mut __sort_data = byPath(list.clone())",
		"let __sort_len = __sort_data.len()",
		"__sort_data.less(",
		"__sort_data.swap(",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("sort.Sort named-slice conversion lowering missing %q:\n%s", want, rust)
		}
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

func TestFmtFprintfSourceMappedBytesBufferUsesGeneratedWrite(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"bytes"
	"fmt"
)

func build(s string) string {
	var buf bytes.Buffer
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

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"bytes": "bytes"})

	if strings.Contains(rust, "__go_write_bytes") {
		t.Fatalf("source-mapped fmt.Fprintf should not call the bytes bridge helper:\n%s", rust)
	}
	if !strings.Contains(rust, ".write(") || !strings.Contains(rust, "__s.into_bytes()") {
		t.Fatalf("source-mapped fmt.Fprintf should call generated Buffer.write with formatted bytes:\n%s", rust)
	}
}

func TestSourceMappedBytesBufferAsIoWriterUsesWriteCallback(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"bytes"
	"fmt"
	"io"
)

type printer struct {
	output io.Writer
}

func build(s string) string {
	var buf bytes.Buffer
	p := printer{output: &buf}
	fmt.Fprintf(p.output, "%s", s)
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

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"bytes": "bytes"})

	if !strings.Contains(rust, "io_Writer::__go_from_with_write") {
		t.Fatalf("source-mapped bytes.Buffer assigned to io.Writer should carry a write callback:\n%s", rust)
	}
	if strings.Contains(rust, "bytes_Buffer") {
		t.Fatalf("source-mapped bytes.Buffer io.Writer conversion should not route through bytes_Buffer:\n%s", rust)
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

func TestTimeNewTimerUnwrapsDurationSelectorArgument(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "time"

type Cmd struct {
	WaitDelay time.Duration
}

func (c *Cmd) watch() {
	_ = time.NewTimer(c.WaitDelay)
}

func forceConcurrent() {
	go func() {}()
}
`)

	if strings.Contains(rust, "go_new_timer(self.wait_delay.clone())") {
		t.Fatalf("time.NewTimer should receive the raw duration value, not the wrapped field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "go_new_timer((*self.wait_delay.lock().unwrap().as_ref().unwrap()).clone())") {
		t.Fatalf("time.NewTimer should unwrap a duration selector argument:\n%s", rust)
	}
}

func TestTimeAfterFuncUsesGoTimerHelper(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "time"

func warn() {}

func read() {
	t := time.AfterFunc(time.Minute, warn)
	defer t.Stop()
}

func forceConcurrent() {
	go func() {}()
}
`)

	if strings.Contains(rust, "time::after_func") {
		t.Fatalf("time.AfterFunc should use the GoTimer helper, not an external package stub:\n%s", rust)
	}
	if !strings.Contains(rust, "go_after_func(std::time::Duration::from_secs(60), warn)") {
		t.Fatalf("time.AfterFunc should pass a raw duration and named callback to the helper:\n%s", rust)
	}
	if !strings.Contains(rust, "let __recv_ptr: *mut GoTimer") {
		t.Fatalf("deferred Stop should use the same GoTimer helper type:\n%s", rust)
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

func TestBuiltinPrintlnNamedIntegerConversionUsesBareValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type hex uint64

func log(p uintptr) {
	println("addr", hex(uint64(p)))
}
`)

	if strings.Contains(rust, "hex(") &&
		(strings.Contains(rust, ")).borrow().as_ref().unwrap()") ||
			strings.Contains(rust, ")).lock().unwrap().as_ref().unwrap()")) {
		t.Fatalf("builtin println should not unwrap a named integer conversion as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, `format!("{}", hex(`) {
		t.Fatalf("builtin println should format the named integer conversion directly:\n%s", rust)
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

func TestStrconvParseBoolLowersTupleWithBareBool(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"os"
	"strconv"
)

func debugEnabled() bool {
	debug, _ := strconv.ParseBool(os.Getenv("GOPACKAGESDEBUG"))
	return debug
}
`)

	if strings.Contains(rust, "strconv::parse_bool") {
		t.Fatalf("strconv.ParseBool should lower at the typed call site without using the bridge:\n%s", rust)
	}
	for _, want := range []string{
		"let __parse_bool_input =",
		"\"1\" | \"t\" | \"T\" | \"TRUE\" | \"true\" | \"True\" => (true,",
		"\"0\" | \"f\" | \"F\" | \"FALSE\" | \"false\" | \"False\" => (false,",
		"let (mut debug, _) =",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("strconv.ParseBool lowering missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "debug.borrow()") || strings.Contains(rust, "debug.lock()") {
		t.Fatalf("strconv.ParseBool bool result should remain bare after short declaration:\n%s", rust)
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

func TestBuiltinPrintPointerSelectorFormatsHandleOnce(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type node struct {
	next *node
}

func warn(n *node) {
	print(" next=", n.next)
}

func forceConcurrent(ch chan bool) {
	go func() {
		ch <- true
	}()
}
`)

	if strings.Contains(rust, ".next.clone(); __field }.lock().unwrap().as_ref().unwrap()).lock().unwrap()") {
		t.Fatalf("printing a pointer selector should not unwrap the pointee and then lock it again:\n%s", rust)
	}
	if !strings.Contains(rust, `format!("{}", format!("&{}", (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).next.clone(); __field }.lock().unwrap().as_ref().unwrap())))`) {
		t.Fatalf("printing a pointer selector should format the selector handle once:\n%s", rust)
	}
}

func TestBuiltinPrintAddressOfPointerFormatsHandleAddress(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type node struct{}

func warn(n *node) {
	print(" slot=", &n)
}

func forceConcurrent(ch chan bool) {
	go func() {
		ch <- true
	}()
}
`)

	if strings.Contains(rust, `format!("{}", Arc::new(Mutex::new(Some(n.clone()))))`) ||
		strings.Contains(rust, `format!("{}", Rc::new(RefCell::new(Some(n.clone()))))`) {
		t.Fatalf("printing address of pointer variable should not format a freshly wrapped pointer slot:\n%s", rust)
	}
	if !strings.Contains(rust, `format!("{}", format!("0x{:x}", Arc::as_ptr(&n) as usize))`) &&
		!strings.Contains(rust, `format!("{}", format!("0x{:x}", Rc::as_ptr(&n) as usize))`) {
		t.Fatalf("printing address of pointer variable should format the pointer handle address:\n%s", rust)
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

func TestConcurrentFmtFprintfFormatsBareAnyRangeValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"fmt"
	"os"
)

func log(args ...any) {
	go func() {}()
	for _, arg := range args {
		fmt.Fprintf(os.Stderr, "unsupported %v (%T)\n", arg, arg)
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
	prevDetector := GetConcurrencyDetector()
	detector := NewConcurrencyDetector()
	detector.AnalyzeFile(file)
	SetConcurrencyDetector(detector)
	defer SetConcurrencyDetector(prevDetector)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "format_any(arg.lock()") || strings.Contains(rust, "format_any(arg.borrow()") {
		t.Fatalf("fmt.Fprintf should not treat a bare any range value as a wrapper handle:\n%s", rust)
	}
	if !strings.Contains(rust, "format_any(arg.as_ref())") {
		t.Fatalf("fmt.Fprintf should format a bare any range value through its boxed payload:\n%s", rust)
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

func TestPanicRecoverHelperUnwrapsNestedAnyPayload(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func fail() {
	done := make(chan bool)
	_ = done
	defer func() {
		_ = recover()
	}()
	panic("boom")
}
`)

	if strings.Contains(rust, "__GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(*boxed));\n            return;") {
		t.Fatalf("recover helper should not stop after the first boxed any payload:\n%s", rust)
	}
	for _, want := range []string{
		"let mut payload = *boxed;",
		"loop {",
		"match payload.downcast::<Box<dyn Any + Send + Sync>>()",
		"__GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(payload));",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("recover helper missing nested boxed any unwrap fragment %q:\n%s", want, rust)
		}
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

func TestBuiltinCopyWrappedSourceAllowsNilSlice(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func fill(dst []byte, src []byte) int {
	return copy(dst, src)
}
`)

	if strings.Contains(rust, "src.borrow().as_ref().unwrap()).clone()") ||
		strings.Contains(rust, "src.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("copy source should not unwrap nil slices:\n%s", rust)
	}
	if !strings.Contains(rust, "__copy_src_guard.as_ref().cloned().unwrap_or_default()") {
		t.Fatalf("copy source should clone-or-default nil slices:\n%s", rust)
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

func TestBuiltinCopyPointerToNamedArraySourceSliceUsesInnerHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type callers [4]uintptr

type M struct {
	callers *callers
}

func fill(m *M, dst []uintptr, n int) int {
	return copy(dst[:], m.callers[:n])
}

func main() {
	go func() {}()
}
`)

	if strings.Contains(rust, "__seq[..") && !strings.Contains(rust, ".0.clone(); __named_array }") {
		t.Fatalf("copy source slice from pointer to named array should not slice the named wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "let __array_holder = { let __named_array = (*") ||
		!strings.Contains(rust, ".0.clone(); __named_array }") {
		t.Fatalf("copy source slice from pointer to named array should use the inner array handle:\n%s", rust)
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
