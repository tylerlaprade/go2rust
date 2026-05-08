package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"golang.org/x/tools/go/packages"
)

func TestPackageLoaderMainASTByPathUsesWorkDirForRelativeCompiledFiles(t *testing.T) {
	tempDir := t.TempDir()
	workDir := filepath.Join(tempDir, "go")
	file, err := parser.ParseFile(token.NewFileSet(), filepath.Join(workDir, "captures.go"), "package main\n", 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}

	loader := &PackageLoader{
		workDir: workDir,
		mainPkg: &packages.Package{
			Syntax:          []*ast.File{file},
			CompiledGoFiles: []string{"captures.go"},
		},
	}

	astByPath := loader.GetMainASTByPath()
	if got := astByPath[normalizeFilePath(filepath.Join(workDir, "captures.go"))]; got != file {
		t.Fatalf("GetMainASTByPath() did not key relative compiled file against workDir; got %#v", astByPath)
	}
}

func TestPackageLoaderNormalizePackageFilePathDoesNotDoubleRelativeWorkDir(t *testing.T) {
	loader := &PackageLoader{workDir: "go"}

	if got, want := loader.normalizePackageFilePath("captures.go"), normalizeFilePath("go/captures.go"); got != want {
		t.Fatalf("normalizePackageFilePath(captures.go) = %q, want %q", got, want)
	}
	if got, want := loader.normalizePackageFilePath("go/captures.go"), normalizeFilePath("go/captures.go"); got != want {
		t.Fatalf("normalizePackageFilePath(go/captures.go) = %q, want %q", got, want)
	}
}

func TestGenerateWithExternalPackagesPreservesMainFileMapping(t *testing.T) {
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

func Value() string { return "dep" }
`)
	writeTestFile(t, filepath.Join(tempDir, "aa.go"), `package main

func A() string { return "a" }
`)
	writeTestFile(t, filepath.Join(tempDir, "zz.go"), `package main

import "example.com/dep"

func Z() string { return dep.Value() }
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {
	println(A())
	println(Z())
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "aa.go"),
		filepath.Join(tempDir, "zz.go"),
		filepath.Join(tempDir, "main.go"),
	})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	zzRS := mustReadFile(t, filepath.Join(tempDir, "zz.rs"))

	if !strings.Contains(mainRS, "fn main()") {
		t.Fatalf("main.rs should contain fn main(), got:\n%s", mainRS)
	}
	if !strings.Contains(zzRS, "pub fn z()") {
		t.Fatalf("zz.rs should contain pub fn z(), got:\n%s", zzRS)
	}
}

func TestExternalPackageUsesOwnConcurrencyDetector(t *testing.T) {
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

func Run(value int) int {
	done := make(chan bool)
	go func() {
		done <- true
	}()
	<-done
	return value
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func main() {
	println(dep.Run(3))
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "mod.rs"))
	if !strings.Contains(depRS, "use std::sync::{Arc, Mutex};") {
		t.Fatalf("external package with goroutine should import Arc/Mutex wrappers, got:\n%s", depRS)
	}
	if strings.Contains(depRS, "Rc<RefCell") {
		t.Fatalf("external package with goroutine should not emit Rc<RefCell> wrappers, got:\n%s", depRS)
	}
	if !strings.Contains(depRS, "pub fn run(value: Arc<Mutex<Option<i32>>>)") {
		t.Fatalf("external package function signature should use Arc/Mutex wrappers, got:\n%s", depRS)
	}
}

func TestTranspiledExternalPackagesShareWorkspaceConcurrencyDetector(t *testing.T) {
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

func Value(value int) int {
	return value
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func main() {
	done := make(chan bool)
	go func() {
		done <- true
	}()
	<-done
	println(dep.Value(3))
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "mod.rs"))
	if !strings.Contains(depRS, "pub fn value(value: Arc<Mutex<Option<i32>>>)") {
		t.Fatalf("external package should use the workspace wrapper policy, got:\n%s", depRS)
	}
	if strings.Contains(depRS, "Rc<RefCell") {
		t.Fatalf("external package should not emit Rc<RefCell> when the workspace needs Arc/Mutex, got:\n%s", depRS)
	}
}

func TestTranspiledExternalPackageExportedGlobalUsesGoName(t *testing.T) {
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

var Public = 7

func Value() int {
	return Public
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func main() {
	println(dep.Public)
	println(dep.Value())
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "mod.rs"))
	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(depRS, "pub static Public:") {
		t.Fatalf("exported external package global should be public and keep its Go name, got:\n%s", depRS)
	}
	if !strings.Contains(mainRS, "example_com_dep::Public") {
		t.Fatalf("external package global selector should use the generated global name, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "example_com_dep::public") {
		t.Fatalf("external package global selector should not be snake-cased, got:\n%s", mainRS)
	}
}

func TestTranspiledExternalPackagePointerGlobalMethodCall(t *testing.T) {
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

type Counter struct {
	n int
}

func NewCounter() *Counter {
	return &Counter{n: 7}
}

func (c *Counter) Value() int {
	return c.n
}

var Public = NewCounter()
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func main() {
	println(dep.Public.Value())
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "let __recv_holder = example_com_dep::Public") {
		t.Fatalf("external package pointer global method call should clone the stored pointer handle, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "example_com_dep::Public.lock().unwrap().as_mut().unwrap()).value") {
		t.Fatalf("external package pointer global method call should not call methods on the outer global slot, got:\n%s", mainRS)
	}
}

func TestArrayLiteralUnwrapsExternalMethodCallElement(t *testing.T) {
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

type Label struct {
	Value string
}

type Key struct {
	name string
}

func NewKey(name string) *Key {
	return &Key{name: name}
}

func (k *Key) Of(value string) Label {
	return Label{Value: k.name + ":" + value}
}

var Msg = NewKey("msg")
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func Use(labels [2]dep.Label) dep.Label {
	return labels[0]
}

func main() {
	_ = Use([2]dep.Label{
		dep.Msg.Of("hello"),
	})
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "let __owned = (*__v") {
		t.Fatalf("array literal method-call element should unwrap the returned handle into the raw element value, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Default::default()") {
		t.Fatalf("fixed array literal should fill omitted elements with zero values, got:\n%s", mainRS)
	}
}

func TestMethodReturningFunctionValueIsNotFunctionFieldCall(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "sync/atomic"

type Exporter func() int

var exporter atomic.Pointer[Exporter]

func Use() int {
	exporterPtr := exporter.Load()
	if exporterPtr == nil {
		return 0
	}
	return (*exporterPtr)()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "EXPORTER") {
		t.Fatalf("method selector returning a function type should not be treated as an uppercase function-valued field, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "let __f_holder") && strings.Contains(mainRS, "atomic::load_pointer") {
		t.Fatalf("atomic.Pointer.Load should be emitted as a method/std-wrapper call, not a function-field call, got:\n%s", mainRS)
	}
}

func TestNamedFunctionTypeConversionDoesNotCallConstant(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type Exporter func(int) int

func Use(fn func(int) int) Exporter {
	return Exporter(fn)
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "EXPORTER") {
		t.Fatalf("named function type conversion should not be emitted as a call to an uppercase value, got:\n%s", mainRS)
	}
}

func TestFunctionTypeAliasUsesImportedInterfaceTraitObject(t *testing.T) {
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

type Mapper interface {
	Find() int
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type Exporter func(dep.Mapper) int
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "&dyn example_com_dep::Mapper") {
		t.Fatalf("function type alias should use a trait object for imported interface parameters, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "Option<Box<dyn example_com_dep::Mapper") {
		t.Fatalf("function type alias should not wrap imported interface parameters, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "Option<example_com_dep::Mapper>") {
		t.Fatalf("function type alias should not wrap an imported interface trait name as a concrete type, got:\n%s", mainRS)
	}
}

func TestFunctionTypeAliasCallPassesConcreteImportedInterfaceArgument(t *testing.T) {
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

type Mapper interface {
	Find() int
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type Event struct{}

func (Event) Find() int {
	return 1
}

type Exporter func(Event, dep.Mapper)

func Deliver(exporter Exporter, ev Event) {
	exporter(ev, ev)
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "Box<dyn Fn(Rc<RefCell<Option<Event>>>, &dyn example_com_dep::Mapper)") {
		t.Fatalf("function type alias should keep imported interface params as trait refs, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "(*__f)(ev.clone(), ev.borrow().as_ref().unwrap())") {
		t.Fatalf("function type call should pass concrete values as imported interface refs, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "use std::any::Any;") {
		t.Fatalf("imported interface impl support should import Any, got:\n%s", mainRS)
	}
}

func TestImportedInterfaceImplCanBeDiscoveredFromSiblingFileCall(t *testing.T) {
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

type Mapper interface {
	Find() int
}
`)
	writeTestFile(t, filepath.Join(tempDir, "event.go"), `package main

type Event struct{}

func (Event) Find() int {
	return 1
}
`)
	writeTestFile(t, filepath.Join(tempDir, "export.go"), `package main

import "example.com/dep"

type Exporter func(Event, dep.Mapper)

func Deliver(exporter Exporter, ev Event) {
	exporter(ev, ev)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "event.go"),
		filepath.Join(tempDir, "export.go"),
		filepath.Join(tempDir, "main.go"),
	})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	eventRS := mustReadFile(t, filepath.Join(tempDir, "event.rs"))
	if !strings.Contains(eventRS, "impl example_com_dep::Mapper for Event") {
		t.Fatalf("imported interface impl should be emitted with the concrete type, got:\n%s", eventRS)
	}
}

func TestUnsafePointerToNamedFunctionTypeConversionUsesPointerDefault(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import (
	"sync/atomic"
	"unsafe"
)

type Exporter func()

var exporter unsafe.Pointer

func Call(e Exporter) {}

func Use() {
	exporterPtr := (*Exporter)(atomic.LoadPointer(&exporter))
	if exporterPtr == nil {
		return
	}
	Call(*exporterPtr)
	(*exporterPtr)()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "Exporter::default()") {
		t.Fatalf("pointer conversion to named function type should emit a typed pointer default, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "atomic::load_pointer") && strings.Contains(mainRS, "as Box<dyn Fn") {
		t.Fatalf("pointer conversion should not cast a raw unsafe pointer to a function box, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "call(Rc::new(RefCell::new(Some({ let __v =") ||
		strings.Contains(mainRS, "call(Arc::new(Mutex::new(Some({ let __v =") {
		t.Fatalf("function alias pointer dereference arguments should not be double-wrapped, got:\n%s", mainRS)
	}
}

func TestImportedInterfaceEqualityUsesTraitMethod(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22

require example.com/label v0.0.0

replace example.com/label => ./label
`)
	writeTestFile(t, filepath.Join(tempDir, "label", "go.mod"), `module example.com/label

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "label", "label.go"), `package label

type Key interface {
	Name() string
}

type Label struct {
	key Key
}

func (l Label) Key() Key {
	return l.key
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/label"

func Same(l label.Label, key label.Key) bool {
	return l.Key() == key
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "__left.__go_eq(__right)") {
		t.Fatalf("imported interface equality should dispatch through trait equality, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, " == key") {
		t.Fatalf("imported interface equality should not emit raw Rust ==, got:\n%s", mainRS)
	}
}

func TestFunctionTypeAliasUsesKnownStdlibContextHelper(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "context"

type Exporter func(context.Context)

func Use(exporter Exporter) {
	exporter(context.Background())
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "GoContext") {
		t.Fatalf("function type alias should use the context helper type, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "context_Context") {
		t.Fatalf("function type alias should not mix context.Context stubs with GoContext helpers, got:\n%s", mainRS)
	}
}

func TestGoTimeHelperIncludesIsZeroAndFormatMethods(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "time"

func main() {
	var zero time.Time
	println(zero.IsZero())
	println(time.Unix(1, 0).Format("2006-01-02"))
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "fn is_zero(&self)") {
		t.Fatalf("GoTime helper should include IsZero support, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "fn format(&self") {
		t.Fatalf("GoTime helper should include Format support, got:\n%s", mainRS)
	}
}

func TestWrappedArrayFieldRangeAndFormatting(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "fmt"

type Label struct {
	Value int
}

type Event struct {
	Static [3]Label
}

func (ev Event) Sum() int {
	total := 0
	for _, label := range ev.Static {
		total += label.Value
	}
	return total
}

func main() {
	ev := Event{Static: [3]Label{{Value: 1}, {Value: 2}, {Value: 3}}}
	fmt.Println(ev.Static)
	println(ev.Sum())
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]);") {
		t.Fatalf("range over wrapped array fields should borrow a slice view, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "fn format_slice<T, C>") || !strings.Contains(mainRS, "C: AsRef<[T]>") {
		t.Fatalf("format_slice should accept arrays and slices through AsRef, got:\n%s", mainRS)
	}
}

func TestTranspiledExternalPackagesUseSharedStdlibStubs(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22

require (
	example.com/aliases v0.0.0
	example.com/typeparams v0.0.0
)

replace example.com/aliases => ./aliases
replace example.com/typeparams => ./typeparams
`)
	writeTestFile(t, filepath.Join(tempDir, "aliases", "go.mod"), `module example.com/aliases

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "aliases", "aliases.go"), `package aliases

import "go/types"

func Tuple() *types.Tuple {
	return nil
}
`)
	writeTestFile(t, filepath.Join(tempDir, "typeparams", "go.mod"), `module example.com/typeparams

go 1.22

require example.com/aliases v0.0.0

replace example.com/aliases => ../aliases
`)
	writeTestFile(t, filepath.Join(tempDir, "typeparams", "typeparams.go"), `package typeparams

import "example.com/aliases"

func Count() int {
	tuple := aliases.Tuple()
	if tuple == nil {
		return 0
	}
	return tuple.Len()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/typeparams"

func main() {
	println(typeparams.Count())
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	sharedLib := mustReadFile(t, filepath.Join(tempDir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	if !strings.Contains(sharedLib, "pub struct types_Tuple") {
		t.Fatalf("shared stdlib stub crate should contain the returned stdlib type, got:\n%s", sharedLib)
	}
	if !strings.Contains(sharedLib, "pub fn len(&self)") {
		t.Fatalf("shared stdlib stub crate should contain methods needed by dependent crates, got:\n%s", sharedLib)
	}

	aliasesLib := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_aliases", "lib.rs"))
	if strings.Contains(aliasesLib, "pub mod go2rust_stdlib_stubs") {
		t.Fatalf("external package should not declare a private stdlib stub module, got:\n%s", aliasesLib)
	}
	if !strings.Contains(aliasesLib, "pub use go2rust_stdlib_stubs::*;") {
		t.Fatalf("external package should re-export shared stdlib stubs, got:\n%s", aliasesLib)
	}

	typeparamsRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_typeparams", "mod.rs"))
	if !strings.Contains(typeparamsRS, "use go2rust_stdlib_stubs::*;") {
		t.Fatalf("external package module should import shared stdlib stubs, got:\n%s", typeparamsRS)
	}

	typeparamsCargo := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_typeparams", "Cargo.toml"))
	if !strings.Contains(typeparamsCargo, `go2rust_stdlib_stubs = { path = "../go2rust_stdlib_stubs" }`) {
		t.Fatalf("external package should depend on shared stdlib stub crate, got:\n%s", typeparamsCargo)
	}

	rootCargo := mustReadFile(t, filepath.Join(tempDir, "Cargo.toml"))
	if !strings.Contains(rootCargo, `"vendor/go2rust_stdlib_stubs"`) {
		t.Fatalf("root workspace should include shared stdlib stub crate, got:\n%s", rootCargo)
	}
	if !strings.Contains(rootCargo, `go2rust_stdlib_stubs = { path = "vendor/go2rust_stdlib_stubs" }`) {
		t.Fatalf("root package should depend on shared stdlib stub crate, got:\n%s", rootCargo)
	}
}

func TestMultiFileNamedScalarLiteralFieldsUseAccessibleNewtypes(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "defs.go"), `package main

type Kind int8
type Version int8

const (
	Invalid Kind = iota
	Func
)

type Symbol struct {
	Name    string
	Kind    Kind
	Version Version
}
`)
	writeTestFile(t, filepath.Join(tempDir, "manifest.go"), `package main

func Symbols() []Symbol {
	return []Symbol{{Name: "Println", Kind: Func, Version: 0}}
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {
	println(Symbols()[0].Name)
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "defs.go"),
		filepath.Join(tempDir, "manifest.go"),
		filepath.Join(tempDir, "main.go"),
	})

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	defsRS := mustReadFile(t, filepath.Join(tempDir, "defs.rs"))
	if !strings.Contains(defsRS, "pub struct Kind(pub Rc<RefCell<Option<i8>>>);") {
		t.Fatalf("named scalar newtype should expose its tuple field for sibling modules, got:\n%s", defsRS)
	}
	if !strings.Contains(defsRS, "pub struct Version(pub Rc<RefCell<Option<i8>>>);") {
		t.Fatalf("named scalar newtype should expose its tuple field for sibling modules, got:\n%s", defsRS)
	}

	manifestRS := mustReadFile(t, filepath.Join(tempDir, "manifest.rs"))
	if !strings.Contains(manifestRS, "kind: Rc::new(RefCell::new(Some(Kind(") {
		t.Fatalf("typed constant field should convert through Kind, got:\n%s", manifestRS)
	}
	if !strings.Contains(manifestRS, "version: Rc::new(RefCell::new(Some(Version(") {
		t.Fatalf("untyped literal field should convert through Version, got:\n%s", manifestRS)
	}
}

func TestMultiFileStdlibHelpersUseSharedCrateRootInclude(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "clock.go"), `package main

import "time"

func Made() time.Time {
	return time.Unix(1, 0)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "consume.go"), `package main

import "time"

func UnixSecond(t time.Time) int64 {
	return t.Unix()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {
	println(UnixSecond(Made()))
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "clock.go"),
		filepath.Join(tempDir, "consume.go"),
		filepath.Join(tempDir, "main.go"),
	})

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, `include!("__go2rust_helpers.rs");`) {
		t.Fatalf("main.rs should include package-scoped helpers, got:\n%s", mainRS)
	}

	helpersRS := mustReadFile(t, filepath.Join(tempDir, packageHelperIncludeFile))
	if !strings.Contains(helpersRS, "struct GoTime") {
		t.Fatalf("package helper include should define GoTime once, got:\n%s", helpersRS)
	}

	clockRS := mustReadFile(t, filepath.Join(tempDir, "clock.rs"))
	consumeRS := mustReadFile(t, filepath.Join(tempDir, "consume.rs"))
	for name, code := range map[string]string{"clock.rs": clockRS, "consume.rs": consumeRS} {
		if !strings.Contains(code, "use crate::*;") {
			t.Fatalf("%s should import crate-root helpers, got:\n%s", name, code)
		}
		if strings.Contains(code, "struct GoTime") {
			t.Fatalf("%s should not define a file-local GoTime, got:\n%s", name, code)
		}
	}
}

func TestTranspiledExternalMultiFilePackageHelpersUseSharedCrateRootInclude(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22

require example.com/dep v0.0.0

replace example.com/dep => ./dep
`)
	writeTestFile(t, filepath.Join(tempDir, "dep", "go.mod"), `module example.com/dep

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "dep", "clock.go"), `package dep

import "time"

func Made() time.Time {
	return time.Unix(1, 0)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "dep", "consume.go"), `package dep

import "time"

func UnixSecond(t time.Time) int64 {
	return t.Unix()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func main() {
	println(dep.UnixSecond(dep.Made()))
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	depDir := filepath.Join(tempDir, "vendor", "example_com_dep")
	depLib := mustReadFile(t, filepath.Join(depDir, "lib.rs"))
	if !strings.Contains(depLib, `include!("__go2rust_helpers.rs");`) {
		t.Fatalf("external package lib.rs should include package-scoped helpers, got:\n%s", depLib)
	}

	helpersRS := mustReadFile(t, filepath.Join(depDir, packageHelperIncludeFile))
	if !strings.Contains(helpersRS, "struct GoTime") {
		t.Fatalf("external package helper include should define GoTime once, got:\n%s", helpersRS)
	}

	clockRS := mustReadFile(t, filepath.Join(depDir, "clock.rs"))
	consumeRS := mustReadFile(t, filepath.Join(depDir, "consume.rs"))
	for name, code := range map[string]string{"clock.rs": clockRS, "consume.rs": consumeRS} {
		if !strings.Contains(code, "use crate::*;") {
			t.Fatalf("%s should import crate-root helpers, got:\n%s", name, code)
		}
		if strings.Contains(code, "struct GoTime") {
			t.Fatalf("%s should not define a file-local GoTime, got:\n%s", name, code)
		}
	}
}

func TestStructWithImportedFieldDoesNotDeriveDebug(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22

require (
	example.com/dep v0.0.0
	example.com/label v0.0.0
)

replace example.com/dep => ./dep
replace example.com/label => ./label
`)
	writeTestFile(t, filepath.Join(tempDir, "label", "go.mod"), `module example.com/label

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "label", "label.go"), `package label

type Key interface {
	Name() string
}

type Label struct {
	Key Key
}
`)
	writeTestFile(t, filepath.Join(tempDir, "dep", "go.mod"), `module example.com/dep

go 1.22

require example.com/label v0.0.0

replace example.com/label => ../label
`)
	writeTestFile(t, filepath.Join(tempDir, "dep", "event.go"), `package dep

import "example.com/label"

type Event struct {
	Static [1]label.Label
}

func New() Event {
	return Event{}
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func main() {
	_ = dep.New()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	eventRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "event.rs"))
	if strings.Contains(eventRS, "#[derive(Debug, Clone, Default)]\npub struct Event") {
		t.Fatalf("struct with imported field should not derive Debug, got:\n%s", eventRS)
	}
	if !strings.Contains(eventRS, "#[derive(Clone, Default)]\npub struct Event") {
		t.Fatalf("struct with imported field should still derive Clone and Default, got:\n%s", eventRS)
	}
}

func TestGenerateCargoTomlIsDeterministic(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "main.go"), "package main\nfunc main() {}\n")

	pg := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	pg.projectImports = NewImportTracker()
	pg.packageMapping = map[string]string{
		"example.com/zeta":    "zeta",
		"example.com/alpha":   "alpha",
		"example.com/mu":      "mu",
		"example.com/beta":    "beta",
		"example.com/epsilon": "epsilon",
	}

	seen := map[string]bool{}
	for range 50 {
		if err := pg.generateCargoToml(); err != nil {
			t.Fatalf("generateCargoToml() error = %v", err)
		}
		seen[mustReadFile(t, filepath.Join(tempDir, "Cargo.toml"))] = true
	}

	if len(seen) != 1 {
		t.Fatalf("generateCargoToml() should be deterministic, saw %d distinct outputs", len(seen))
	}
}

func TestPackageDependencyCratesAreDeterministic(t *testing.T) {
	imports := map[string]*packages.Package{
		"example.com/zeta":  {},
		"fmt":               {},
		"example.com/alpha": {},
		"example.com/self":  {},
	}
	mapping := map[string]string{
		"example.com/zeta":  "zeta",
		"example.com/alpha": "alpha",
		"example.com/self":  "self",
	}

	got := packageDependencyCrates(imports, "self", mapping)
	want := []string{"alpha", "zeta"}
	if strings.Join(got, ",") != strings.Join(want, ",") {
		t.Fatalf("packageDependencyCrates() = %#v, want %#v", got, want)
	}
}

func TestTranspileWithMappingReturnsPerFileExternalPackages(t *testing.T) {
	fset := token.NewFileSet()
	fileA, err := parser.ParseFile(fset, "a.go", `package main

import "example.com/a"

func A() {}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(a.go) error = %v", err)
	}
	fileB, err := parser.ParseFile(fset, "b.go", `package main

import "example.com/b"

func B() {}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(b.go) error = %v", err)
	}

	_, _, pkgsA := TranspileWithMapping(fileA, fset, nil, nil)
	_, _, pkgsB := TranspileWithMapping(fileB, fset, nil, nil)

	if len(pkgsA) != 1 || !pkgsA["example.com/a"] {
		t.Fatalf("first file should report only example.com/a, got %#v", pkgsA)
	}
	if len(pkgsB) != 1 || !pkgsB["example.com/b"] {
		t.Fatalf("second file should report only example.com/b, got %#v", pkgsB)
	}
}

func TestPackageLoaderOrderedPackagePaths(t *testing.T) {
	loader := &PackageLoader{
		allPackages: map[string]*packages.Package{
			"github.com/zeta/lib":  {},
			"github.com/alpha/lib": {},
			"main":                 {},
			"fmt":                  {},
		},
	}

	got := loader.orderedPackagePaths()
	want := []string{"github.com/alpha/lib", "github.com/zeta/lib"}
	if strings.Join(got, ",") != strings.Join(want, ",") {
		t.Fatalf("orderedPackagePaths() = %v, want %v", got, want)
	}
}

func TestCollectGoFilesSkipsTestFilesForDirectoryInput(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "main.go"), "package main\nfunc main() {}\n")
	writeTestFile(t, filepath.Join(tempDir, "main_test.go"), "package main\n")
	writeTestFile(t, filepath.Join(tempDir, "helper.go"), "package main\n")

	files, err := collectGoFiles(tempDir)
	if err != nil {
		t.Fatalf("collectGoFiles() error = %v", err)
	}

	var basenames []string
	for _, file := range files {
		basenames = append(basenames, filepath.Base(file))
	}
	got := strings.Join(basenames, ",")
	want := "helper.go,main.go"
	if got != want {
		t.Fatalf("collectGoFiles() = %v, want %v", got, want)
	}
}

func writeTestFile(t *testing.T, path string, content string) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		t.Fatalf("MkdirAll(%q) error = %v", path, err)
	}
	if err := os.WriteFile(path, []byte(content), 0644); err != nil {
		t.Fatalf("WriteFile(%q) error = %v", path, err)
	}
}

func mustReadFile(t *testing.T, path string) string {
	t.Helper()
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("ReadFile(%q) error = %v", path, err)
	}
	return string(data)
}
