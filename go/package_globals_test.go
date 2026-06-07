package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"path/filepath"
	"strings"
	"testing"
)

func TestPackageGlobalConstInitCastsToGlobalType(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

const CacheLinePadSize = 128

var CacheLineSize uintptr = CacheLinePadSize
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, "Some(CACHE_LINE_PAD_SIZE as usize)") {
		t.Fatalf("package global const initializer should cast to uintptr target type:\n%s", rust)
	}
}

func TestPackageGlobalInterfaceZeroValueIsNil(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

var sink any
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "Some(Default::default())") {
		t.Fatalf("package global interface zero value must not default a trait object:\n%s", rust)
	}
	if !strings.Contains(rust, " = None;") {
		t.Fatalf("package global interface zero value should initialize to nil:\n%s", rust)
	}
}

func TestPackageGlobalAnyDefinedTypeInitializersBoxConcreteValues(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type word uint16
type text string
type bytes []byte

var wordAny any = word(0)
var textAny any = text("")
var bytesAny any = bytes(nil)

func main() {
	go func() {}()
}
`)

	for _, bad := range []string{
		"Some(word(",
		"Some(text(",
		"Some(bytes(",
	} {
		if strings.Contains(rust, bad) {
			t.Fatalf("package-global any initializer should not store a concrete value directly in the Any slot (%q):\n%s", bad, rust)
		}
	}
	for _, want := range []string{
		"Some(Box::new(word(",
		"Some(Box::new(text(",
		"Some(Box::new(bytes(",
		"as Box<dyn Any + Send + Sync>",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("package-global any initializer should box defined concrete values, missing %q:\n%s", want, rust)
		}
	}
}

func TestPackageGlobalInterfaceHandleAssignmentWritesSlot(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Object interface {
	Name() string
}

var sink Object
var ch chan int

func lookup() Object {
	return nil
}

func init() {
	sink = lookup()
}
`)

	if strings.Contains(rust, "sink = lookup().clone()") {
		t.Fatalf("package-global interface assignment should not replace the global handle:\n%s", rust)
	}
	if !strings.Contains(rust, "*sink.lock().unwrap() = (*__iface_guard).clone()") &&
		!strings.Contains(rust, "*sink.lock().unwrap() = __iface_value") {
		t.Fatalf("package-global interface assignment should write through the global slot:\n%s", rust)
	}
}

func TestPackageGlobalInterfaceSelectorConstInitBoxesNamedValue(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22

require example.com/dep v0.0.0

replace example.com/dep => ./dep
`)
	writeTestFile(t, filepath.Join(tempDir, "dep", "go.mod"), `module example.com/dep

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "dep", "dep.go"), `package dep

type Sig int

const SIGINT Sig = 2

func (Sig) Signal() {}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type Signal interface {
	Signal()
}

var Interrupt Signal = dep.SIGINT
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "= Some(example_com_dep::S_I_G_I_N_T") || strings.Contains(mainRS, "= Some(SIGINT") {
		t.Fatalf("package-global interface selector const should not store the raw constant:\n%s", mainRS)
	}
	if (!strings.Contains(mainRS, "Box::new(example_com_dep::Sig(") && !strings.Contains(mainRS, "Box::new(example_com_dep::r#mod::Sig(")) ||
		!strings.Contains(mainRS, "S_I_G_I_N_T as i32") ||
		!strings.Contains(mainRS, "as Box<dyn Signal") {
		t.Fatalf("package-global interface selector const should box the named value as the interface:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "impl Signal for example_com_dep::Sig") && !strings.Contains(mainRS, "impl Signal for example_com_dep::r#mod::Sig") {
		t.Fatalf("package-global interface selector const should register the external concrete interface impl:\n%s", mainRS)
	}
}

func TestPackageGlobalPointerShortDeclCopiesStoredHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Scope struct{}

var Universe *Scope
var ch chan int

func use() {
	scope := Universe
	_ = scope
}
`)

	if strings.Contains(rust, "let mut scope = Universe.clone()") {
		t.Fatalf("short declaration from package-global pointer should not clone the global slot:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut scope = (*Universe.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("short declaration from package-global pointer should clone the stored pointer handle:\n%s", rust)
	}
}

func TestPackageGlobalPointerCallArgumentUsesShortGuard(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Scope struct {
	parent *Scope
}

var Universe *Scope
var ch chan int

func NewScope(parent *Scope) *Scope {
	if parent != nil && parent != Universe {
		return &Scope{parent: parent}
	}
	return &Scope{}
}

func NewPackage() *Scope {
	return NewScope(Universe)
}
`)

	bad := "new_scope((*Universe.lock().unwrap().as_ref().unwrap()).clone())"
	if strings.Contains(rust, bad) {
		t.Fatalf("package-global pointer call argument should not keep the global lock across the call:\n%s", rust)
	}
	want := "new_scope({ let __arg_holder = Universe.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })"
	if !strings.Contains(rust, want) {
		t.Fatalf("package-global pointer call argument should clone through a short guard block:\n%s", rust)
	}
}

func TestPackageGlobalPointerInterfaceWrapperUsesShortGuard(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Object interface {
	Name() string
}

type TypeName struct{}

func (*TypeName) Name() string { return "" }

var global *TypeName
var ch chan int

func def(obj Object) {}

func init() {
	def(global)
}
`)

	bad := "TypeNamePtr((*global.lock().unwrap().as_ref().unwrap()).clone().clone())"
	if strings.Contains(rust, bad) {
		t.Fatalf("package-global pointer interface wrapper should not keep the global lock across wrapper construction:\n%s", rust)
	}
	want := "TypeNamePtr({ let __arg_holder = global.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })"
	if !strings.Contains(rust, want) {
		t.Fatalf("package-global pointer interface wrapper should clone through a short guard block:\n%s", rust)
	}
}

func TestPackageGlobalPointerMapValueCopiesStoredHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type RangeTable struct {
	Lo int
	Hi int
}

var ch chan int
var _C = &RangeTable{Lo: 1, Hi: 2}
var C = _C
var Tables = map[string]*RangeTable{"C": C}
`)

	if strings.Contains(rust, `__go_map.insert("C".to_string(), C.clone())`) {
		t.Fatalf("package-global pointer map value should not clone the global slot:\n%s", rust)
	}
	if !strings.Contains(rust, `__go_map.insert("C".to_string(), (*C.lock().unwrap().as_ref().unwrap()).clone())`) {
		t.Fatalf("package-global pointer map value should clone the stored pointer handle:\n%s", rust)
	}
}

func TestPackageGlobalPointerInitFromSelectorCopiesHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "unsafe"

type Type struct{}

type eface struct {
	typ *Type
}

var sink any

func efaceOf(ep *any) *eface {
	return (*eface)(unsafe.Pointer(ep))
}

var typ *Type = efaceOf(&sink).typ

func main() {
	go func() {}()
}
`)

	if strings.Contains(rust, ".typ.lock().unwrap().as_ref().unwrap()") ||
		strings.Contains(rust, "Some((*(*eface_of(") ||
		strings.Contains(rust, "eface_of(sink.clone()).lock().unwrap().as_ref().unwrap()") ||
		strings.Contains(rust, "eface_of(sink.clone()).borrow().as_ref().unwrap()") {
		t.Fatalf("package-global pointer initializer should not unwrap the selected pointer pointee:\n%s", rust)
	}
	if !strings.Contains(rust, ".typ.clone()") {
		t.Fatalf("package-global pointer initializer should clone the selected pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __ptr = eface_of(sink.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().typ.clone()") {
		t.Fatalf("package-global pointer initializer should borrow GoPtr call result before selecting the field:\n%s", rust)
	}
}

func TestPackageGlobalPointerInitFromGoPtrFieldUsesGoPtrStorage(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "unsafe"

type Type struct{}

type eface struct {
	typ *Type
}

var sink any

func raw() *Type {
	return (*Type)(unsafe.Pointer(uintptr(0)))
}

func forceGoPtrField(e *eface) {
	e.typ = raw()
}

func efaceOf(ep *any) *eface {
	return (*eface)(unsafe.Pointer(ep))
}

var typ *Type = efaceOf(&sink).typ

func main() {
	go func() {}()
}
`)

	if !strings.Contains(rust, "static typ: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoPtr<Type>>>>>") {
		t.Fatalf("package-global pointer initialized from a GoPtr field should use GoPtr storage:\n%s", rust)
	}
	if strings.Contains(rust, "static typ: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<Type>>>>>>>") {
		t.Fatalf("package-global pointer initialized from a GoPtr field should not keep wrapper storage:\n%s", rust)
	}
	if !strings.Contains(rust, "*typ.lock().unwrap() = Some(GoPtr::nil());") {
		t.Fatalf("package-global GoPtr zero value should be GoPtr::nil():\n%s", rust)
	}
	if !strings.Contains(rust, "let __ptr = eface_of(sink.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().typ.clone()") {
		t.Fatalf("package-global GoPtr initializer should clone the selected GoPtr field handle:\n%s", rust)
	}
}

func TestPackageGlobalGoPtrFieldAssignmentUsesStoredHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "unsafe"

type Type struct{}

type eface struct {
	typ *Type
}

var sink any

func raw() *Type {
	return (*Type)(unsafe.Pointer(uintptr(0)))
}

func forceGoPtrField(e *eface) {
	e.typ = raw()
}

func efaceOf(ep *any) *eface {
	return (*eface)(unsafe.Pointer(ep))
}

var typ *Type = efaceOf(&sink).typ

func store(x *eface) {
	x.typ = typ
}

func main() {
	go func() {}()
}
`)

	if !strings.Contains(rust, "static typ: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoPtr<Type>>>>>") {
		t.Fatalf("package-global pointer initialized from a GoPtr field should use GoPtr storage:\n%s", rust)
	}
	if strings.Contains(rust, "GoPtr::local((*typ") {
		t.Fatalf("assignment from package-global GoPtr storage should not rewrap the stored handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = (*typ.lock().unwrap().as_ref().unwrap()).clone();") {
		t.Fatalf("assignment from package-global GoPtr storage should clone the existing handle:\n%s", rust)
	}
}

func TestPackageGlobalGoPtrZeroHelperUsesGoPtrNil(t *testing.T) {
	var out strings.Builder
	transpilePackageGlobalZeroHelper(&out, []packageGlobal{{
		name:         "typ",
		rustType:     "GoPtr<Type>",
		goPtrStorage: true,
	}})
	rust := out.String()
	if !strings.Contains(rust, "*typ.borrow_mut() = Some(GoPtr::nil());") {
		t.Fatalf("package-global GoPtr reset helper should use GoPtr::nil():\n%s", rust)
	}
}

func TestLargePackageGlobalMapLiteralIsChunked(t *testing.T) {
	var src strings.Builder
	src.WriteString("package main\n\n")
	src.WriteString("var ch chan int\n")
	src.WriteString("var PackageSymbols = map[string][]int{\n")
	for i := 0; i < packageGlobalMapLiteralChunkSize+1; i++ {
		fmt.Fprintf(&src, "\t\"pkg%d\": {%d},\n", i, i)
	}
	src.WriteString("}\n")

	rust := transpileTypedConcurrentRegression(t, src.String())

	if !strings.Contains(rust, "fn __go_init_PackageSymbols_map_chunk_0(__go_map: &mut BTreeMap<") {
		t.Fatalf("large package-global map literal should emit first chunk helper:\n%s", rust)
	}
	if !strings.Contains(rust, "fn __go_init_PackageSymbols_map_chunk_1(__go_map: &mut BTreeMap<") {
		t.Fatalf("large package-global map literal should emit second chunk helper:\n%s", rust)
	}
	if !strings.Contains(rust, "__go_init_PackageSymbols_map_chunk_0(&mut __go_map);") ||
		!strings.Contains(rust, "__go_init_PackageSymbols_map_chunk_1(&mut __go_map);") {
		t.Fatalf("large package-global map literal should call chunk helpers:\n%s", rust)
	}
}

func TestSourceMappedPointerGlobalSelectorCopiesStoredHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/types"

func unsafePkg() *types.Package {
	return types.Unsafe
}

func universeScope(pkg *types.Package) *types.Scope {
	if pkg != nil {
		return pkg.Scope()
	}
	return types.Universe
}

func assignUnsafe(path string) *types.Package {
	var pkg *types.Package
	if path == "unsafe" {
		pkg = types.Unsafe
	}
	return pkg
}

func sameUnsafe(pkg *types.Package) bool {
	return pkg == types.Unsafe
}

func unsafePair(path string) (pkg *types.Package, err error) {
	defer func() {}()
	if path == "unsafe" {
		return types.Unsafe, nil
	}
	return nil, nil
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/types": "go_types"})
	if strings.Contains(rust, "go_types::Unsafe.clone()") || strings.Contains(rust, "go_types::Universe.clone()") {
		t.Fatalf("source-mapped pointer global selector should not clone the global slot:\n%s", rust)
	}
	if !strings.Contains(rust, "(*go_types::Unsafe.borrow().as_ref().unwrap()).clone()") {
		t.Fatalf("source-mapped pointer global selector should clone the stored pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "(*go_types::Universe.borrow().as_ref().unwrap()).clone()") {
		t.Fatalf("source-mapped pointer global selector should clone the stored pointer handle:\n%s", rust)
	}
}

func TestPackageGlobalAnyAssignmentBoxesGenericValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

var sink any

func store[T any](x T) {
	sink = x
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
	if strings.Contains(rust, "sink = x.clone()") {
		t.Fatalf("assignment to package-global any should not replace the global handle:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn store<T: Any + GoValueClone + 'static>") {
		t.Fatalf("generic any assignment should bound the Rust type parameter for boxing:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = Box::new(") ||
		!strings.Contains(rust, ".go_value_clone()") ||
		!strings.Contains(rust, "*sink.borrow_mut() = Some(new_val)") {
		t.Fatalf("assignment to package-global any should box into the global slot:\n%s", rust)
	}
}

func TestPackageGlobalBareScalarTupleSlotAssignsDirectly(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

var enabled, _ = parseEnabled()

func parseEnabled() (bool, error) {
	return true, nil
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
	if strings.Contains(rust, "__go_pkg_init_0.borrow().as_ref().unwrap()") ||
		strings.Contains(rust, "__go_pkg_init_0.lock().unwrap().as_ref().unwrap()") {
		t.Fatalf("bare-scalar tuple slot should not be treated as a wrapped handle:\n%s", rust)
	}
	if !strings.Contains(rust, "*enabled.borrow_mut() = Some(__go_pkg_init_0);") &&
		!strings.Contains(rust, "*enabled.lock().unwrap() = Some(__go_pkg_init_0);") {
		t.Fatalf("bare-scalar tuple slot should assign the temp directly into the global slot:\n%s", rust)
	}
}

func TestPackageGlobalErrorSelectorInitMovesHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "io/fs"

var Skip error = fs.SkipDir
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "Some(fs::SkipDir())") {
		t.Fatalf("package global error selector should move from the error handle, not wrap the handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __rhs_holder = fs::SkipDir().clone()") || !strings.Contains(rust, "*Skip.borrow_mut() = new_val") {
		t.Fatalf("package global error selector should move the option payload into the global slot:\n%s", rust)
	}
}

func TestPackageGlobalNamedIntegerErrorConstInitBoxesNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Errno uintptr

const EINVAL = Errno(22)

func (e Errno) Error() string { return "" }

var errEINVAL error = EINVAL
`)

	if strings.Contains(rust, "Some(E_I_N_V_A_L)") {
		t.Fatalf("package global error const init should not store the raw const:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(Box::new(Errno(") {
		t.Fatalf("package global error const init should box the named error value:\n%s", rust)
	}
}

func TestPackageGlobalErrorConversionInitBoxesConcreteValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type errorString string

func (e errorString) Error() string { return string(e) }

var errGlobal = error(errorString("bad"))
`)

	if strings.Contains(rust, "let __rhs_holder = errorString(") {
		t.Fatalf("package global error conversion from concrete value should not move from a concrete newtype handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(Box::new(errorString(") {
		t.Fatalf("package global error conversion from concrete value should box the concrete error:\n%s", rust)
	}
}

func TestPackageGlobalExplicitNilErrorInitStaysNone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

var errGlobal error = nil
`)

	if strings.Contains(rust, "Some(nil)") {
		t.Fatalf("package global error nil init should not store nil as a payload:\n%s", rust)
	}
	if !strings.Contains(rust, "*errGlobal.borrow_mut() = None;") &&
		!strings.Contains(rust, "*errGlobal.lock().unwrap() = None;") {
		t.Fatalf("package global error nil init should leave the slot empty:\n%s", rust)
	}
}

func TestPackageGlobalFunctionSelectorInitBoxesFunctionObject(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "os"

var lstat = os.Lstat
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "Some(os::lstat);") {
		t.Fatalf("package global function selector should box the function object:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(Box::new(os::lstat));") {
		t.Fatalf("package global function selector should initialize with Box::new:\n%s", rust)
	}
}

func TestPackageGlobalNameCollisionWithFunctionRenamesGlobal(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

var logger int

func Logger() int {
	return logger
}
`)

	if !strings.Contains(rust, "static logger_1:") {
		t.Fatalf("package global should be renamed away from colliding function:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn logger(") {
		t.Fatalf("exported function should keep the base Rust function name:\n%s", rust)
	}
	if !strings.Contains(rust, "logger_1.borrow()") && !strings.Contains(rust, "logger_1.lock().unwrap()") {
		t.Fatalf("function body should read the renamed package global:\n%s", rust)
	}
	if strings.Contains(rust, "static logger:") {
		t.Fatalf("package global should not keep the colliding Rust name:\n%s", rust)
	}
}

func TestPackageGlobalMethodReceiverUsesRenamedGlobal(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync/atomic"

var logger atomic.Pointer[int]

func Logger() *int {
	return logger.Load()
}
`)

	if !strings.Contains(rust, "static logger_1:") {
		t.Fatalf("package global should be renamed away from colliding function:\n%s", rust)
	}
	if strings.Contains(rust, "logger.lock()") || strings.Contains(rust, "logger.borrow()") {
		t.Fatalf("method receiver should not use the colliding package-global name:\n%s", rust)
	}
	if !strings.Contains(rust, "logger_1.lock()") && !strings.Contains(rust, "logger_1.borrow") {
		t.Fatalf("method receiver should use the renamed package global:\n%s", rust)
	}
}

func TestSourceMappedStdlibPointerGlobalMethodReceiverUsesPointee(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "os"

func log() {
	os.Stderr.WriteString("x")
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"os": "os"})

	if strings.Contains(rust, "(*os::Stderr.lock().unwrap().as_ref().unwrap()).write_string") ||
		strings.Contains(rust, "(*os::Stderr.borrow_mut().as_mut().unwrap()).write_string") {
		t.Fatalf("source-mapped pointer global method receiver should not call the method on the pointer handle:\n%s", rust)
	}
	if (!strings.Contains(rust, "let __recv_holder = os::Stderr.lock().unwrap().as_ref().unwrap().clone()") &&
		!strings.Contains(rust, "let __recv_holder = os::Stderr.borrow().as_ref().unwrap().clone()")) ||
		(!strings.Contains(rust, "(*__recv_holder.lock().unwrap().as_mut().unwrap()).write_string") &&
			!strings.Contains(rust, "(*__recv_holder.borrow_mut().as_mut().unwrap()).write_string")) {
		t.Fatalf("source-mapped pointer global method receiver should unwrap the pointee before the call:\n%s", rust)
	}
}
