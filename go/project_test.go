package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"golang.org/x/tools/go/packages"
)

func TestPrefixDotImportedCrateUsesEmitsGlobUse(t *testing.T) {
	fset := token.NewFileSet()
	mapping := map[string]string{"internal/types/errors": "internal_types_errors"}

	// A dot import of a mapped (source-transpiled) package brings its names into
	// scope bare in Go (e.g. go/types referencing InvalidSyntaxTree); the Rust
	// output needs a glob use of the dependency crate so those bare names resolve.
	dotFile, err := parser.ParseFile(fset, "dot.go", `package p
import . "internal/types/errors"
var _ = InvalidSyntaxTree
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("parse dot.go: %v", err)
	}
	out := prefixDotImportedCrateUses("// body\n", dotFile, mapping)
	if !strings.Contains(out, "use internal_types_errors::*;") {
		t.Fatalf("dot import of a mapped package should emit a glob use:\n%s", out)
	}

	// A regular (non-dot) import qualifies references with the crate path, so no
	// glob use is emitted.
	plainFile, err := parser.ParseFile(fset, "plain.go", `package p
import "internal/types/errors"
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("parse plain.go: %v", err)
	}
	if got := prefixDotImportedCrateUses("// body\n", plainFile, mapping); strings.Contains(got, "use internal_types_errors") {
		t.Fatalf("non-dot import should not emit a glob use:\n%s", got)
	}

	// A dot import of an unmapped (bridged) package gets no glob use.
	unmappedFile, err := parser.ParseFile(fset, "unmapped.go", `package p
import . "internal/types/errors"
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("parse unmapped.go: %v", err)
	}
	if got := prefixDotImportedCrateUses("// body\n", unmappedFile, map[string]string{}); strings.Contains(got, "use ") {
		t.Fatalf("dot import of an unmapped package should not emit a glob use:\n%s", got)
	}
}

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

func TestCrossFileInterfaceImplEmittedOnlyWithTypeDeclaration(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "type.go"), `package main

type I interface {
	A() int
	B() int
}

type T struct{}

func (T) A() int { return 1 }
`)
	writeTestFile(t, filepath.Join(tempDir, "extra.go"), `package main

func (T) B() int { return 2 }

func Use() int {
	var i I = T{}
	return i.B()
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "type.go"),
		filepath.Join(tempDir, "extra.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	typeRS := mustReadFile(t, filepath.Join(tempDir, "type.rs"))
	extraRS := mustReadFile(t, filepath.Join(tempDir, "extra.rs"))
	implCount := strings.Count(typeRS, "impl I for") + strings.Count(extraRS, "impl I for")
	if implCount != 1 {
		t.Fatalf("cross-file methods should not emit duplicate trait impls, got %d\ntype.rs:\n%s\nextra.rs:\n%s", implCount, typeRS, extraRS)
	}
	if strings.Contains(extraRS, "impl I for") {
		t.Fatalf("trait impl should be emitted with the concrete type declaration, not each file adding methods:\n%s", extraRS)
	}
}

func TestExternalInterfaceCaseImplementsLocalInterfaceForTraitObject(t *testing.T) {
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

type Node interface {
	Pos() int
	End() int
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type positioner interface {
	Pos() int
}

func spanOf(at positioner) int {
	switch x := at.(type) {
	case dep.Node:
		return x.End()
	default:
		return at.Pos()
	}
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "main.go"),
	})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "impl positioner for example_com_dep::Node") {
		t.Fatalf("imported interface cases should not emit an impl for the trait type itself:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "impl positioner for Box<dyn example_com_dep::Node>") {
		t.Fatalf("imported interface cases should implement the local interface for the boxed trait object:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "(**self).pos()") {
		t.Fatalf("boxed imported interface impl should delegate methods through the inner trait object:\n%s", mainRS)
	}
}

func TestDefinedTypeOverImportedScalarEmitsDisplayForLocalInterface(t *testing.T) {
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

type Pos int
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type positioner interface {
	Pos() dep.Pos
}

type atPos dep.Pos

func (p atPos) Pos() dep.Pos {
	return dep.Pos(p)
}

var _ positioner = atPos(0)
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "main.go"),
	})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "impl Display for atPos") {
		t.Fatalf("defined type over imported displayable scalar should implement Display:\n%s", mainRS)
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

func TestTranspiledExternalPackageCargoIncludesComplexDependency(t *testing.T) {
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

func Nop() {}

func Accept(x complex128) {}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func main() {
	dep.Nop()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "mod.rs"))
	if !strings.Contains(depRS, "use num::Complex;") {
		t.Fatalf("complex signature should import num::Complex, got:\n%s", depRS)
	}

	depCargo := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "Cargo.toml"))
	if !strings.Contains(depCargo, `num = "0.4"`) {
		t.Fatalf("external package Cargo.toml should include num when generated code imports it, got:\n%s", depCargo)
	}
}

func TestTranspiledExternalPackageCargoIncludesComplexDependencyForExpressions(t *testing.T) {
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

func Nop() {}

func Value() any {
	return complex(1, 2)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func main() {
	dep.Nop()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "mod.rs"))
	if !strings.Contains(depRS, "num::Complex::new") {
		t.Fatalf("complex expression should use num::Complex, got:\n%s", depRS)
	}

	depCargo := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "Cargo.toml"))
	if !strings.Contains(depCargo, `num = "0.4"`) {
		t.Fatalf("external package Cargo.toml should include num when generated expressions reference it, got:\n%s", depCargo)
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

func TestTranspiledExternalPackageConstSelectorUsesConstName(t *testing.T) {
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

const Func = 2
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func value() int {
	return dep.Func
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "mod.rs"))
	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(depRS, "pub const FUNC") {
		t.Fatalf("external package should emit exported const with const naming, got:\n%s", depRS)
	}
	if !strings.Contains(mainRS, "example_com_dep::FUNC") {
		t.Fatalf("external package const selector should use the generated const name, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "example_com_dep::func") {
		t.Fatalf("external package const selector should not use snake-case function naming, got:\n%s", mainRS)
	}
}

func TestTranspiledExternalPackageConstSelectorCastsToInferredConstType(t *testing.T) {
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

const MaxInt32 = 1<<31 - 1
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

const MaxExp = dep.MaxInt32
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "pub const MAX_EXP: i32 = example_com_dep::MAX_INT32;") {
		t.Fatalf("external package const selector should be cast to the inferred const storage type:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "pub const MAX_EXP: i32 = example_com_dep::MAX_INT32 as i32;") {
		t.Fatalf("external package const selector should cast to i32:\n%s", mainRS)
	}
}

func TestTranspiledExternalPackageConstSelectorBareScalarReturn(t *testing.T) {
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

const (
	MaxUint64 = 1<<64 - 1
	Small32 = 0x1p-126 * 0x1p-23
)
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func limits() (uint64, float32) {
	return dep.MaxUint64, dep.Small32
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "example_com_dep::MAX_UINT64.lock()") || strings.Contains(mainRS, "example_com_dep::SMALL32.lock()") {
		t.Fatalf("external const selectors returned through bare scalar slots should not be unwrapped as handles:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "example_com_dep::MAX_UINT64 as u64") {
		t.Fatalf("external integer const selector should cast to the bare return slot type:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "example_com_dep::SMALL32 as f32") {
		t.Fatalf("external float const selector should cast to the bare return slot type:\n%s", mainRS)
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

func TestAtomicPointerExternalPackageHelperKeepsLocalElementTypeOutOfSharedStubs(t *testing.T) {
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

import "sync/atomic"

type File struct {
	Base int
}

type Set struct {
	last atomic.Pointer[File]
}

func (s *Set) Last() *File {
	return s.last.Load()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func main() {
	var s dep.Set
	_ = s.Last()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	depDir := filepath.Join(tempDir, "vendor", "example_com_dep")
	depRS := mustReadFile(t, filepath.Join(depDir, "mod.rs"))
	stubsRS := mustReadFile(t, filepath.Join(tempDir, "vendor", sharedStdlibStubCrateName, "lib.rs"))

	if !strings.Contains(depRS, "GoAtomicPointer<File>") {
		t.Fatalf("atomic.Pointer[File] should use the package-local generic helper, got:\n%s", depRS)
	}
	if !strings.Contains(depRS, "struct GoAtomicPointer<T>") {
		t.Fatalf("external package should emit GoAtomicPointer helper, got:\n%s", depRS)
	}
	if strings.Contains(stubsRS, "Option<File>") || strings.Contains(stubsRS, "atomic_Pointer") {
		t.Fatalf("shared stdlib stubs must not capture package-local atomic.Pointer element types, got:\n%s", stubsRS)
	}
}

func TestAtomicUint64MethodValueUsesSharedAtomicHelper(t *testing.T) {
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

import "sync/atomic"

type Counter struct {
	n atomic.Uint64
}

func use(read func() uint64) {}

func (c *Counter) Register() {
	use(c.n.Load)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func main() {
	var c dep.Counter
	c.Register()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	stubsRS := mustReadFile(t, filepath.Join(tempDir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	if !strings.Contains(stubsRS, "pub struct atomic_Uint64") || !strings.Contains(stubsRS, "AtomicU64") {
		t.Fatalf("atomic.Uint64 should use the real Rust atomic helper, got:\n%s", stubsRS)
	}
	if !strings.Contains(stubsRS, "pub fn load(&self) -> u64") {
		t.Fatalf("atomic.Uint64 helper should provide Load for method values, got:\n%s", stubsRS)
	}
}

func TestSyncRWMutexFieldUsesBareHelper(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "sync"

type Set struct {
	mu sync.RWMutex
}

func (s *Set) Use() {
	s.mu.RLock()
	defer s.mu.RUnlock()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "struct GoRWMutex") {
		t.Fatalf("sync.RWMutex should emit the bare helper, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "pub mu: GoRWMutex") {
		t.Fatalf("sync.RWMutex struct field should be bare, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "mu: Arc<Mutex<Option<GoRWMutex>>>") || strings.Contains(mainRS, "mu.lock().unwrap()") {
		t.Fatalf("sync.RWMutex field should not be treated as a wrapped field, got:\n%s", mainRS)
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

func TestStructFieldUsesImportedInterfaceTraitObject(t *testing.T) {
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

type Node interface {
	Pos() int
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type Holder struct {
	Node dep.Node
	List []dep.Node
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "pub node: Rc<RefCell<Option<Box<dyn example_com_dep::Node>>>>") {
		t.Fatalf("imported interface field should use a boxed trait object, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "Option<example_com_dep::Node>") {
		t.Fatalf("imported interface field should not use a bare trait name as a type, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Vec<Rc<RefCell<Option<Box<dyn example_com_dep::Node>>>>>") {
		t.Fatalf("imported interface slices should wrap boxed trait-object elements, got:\n%s", mainRS)
	}
}

func TestImportedInterfaceValueBoundariesBoxConcreteValues(t *testing.T) {
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

type Node interface {
	Pos() int
}

type Ident struct{}

func (Ident) Pos() int { return 1 }
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type Holder struct {
	Node dep.Node
}

func keep(node dep.Node) Holder {
	return Holder{Node: node}
}

func asNode(ident dep.Ident) dep.Node {
	return ident
}

func collect(ident dep.Ident) []dep.Node {
	var nodes []dep.Node
	nodes = append(nodes, ident)
	return nodes
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "return ident.clone();") {
		t.Fatalf("concrete imported values returned as an imported interface should be boxed, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Box::new((*ident.borrow().as_ref().unwrap()).clone()) as Box<dyn example_com_dep::Node>") {
		t.Fatalf("concrete imported values should be boxed at imported interface boundaries, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "node: Rc::new(RefCell::new(Some(Box::new((*node.borrow().as_ref().unwrap()).clone())") {
		t.Fatalf("existing imported interface handles should not be cloned into a nested box, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "node: node.clone()") {
		t.Fatalf("existing imported interface handles should be preserved at struct fields, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, ".push((*ident.borrow().as_ref().unwrap()).clone())") {
		t.Fatalf("append to imported interface slice should not push the concrete value directly, got:\n%s", mainRS)
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
	if !strings.Contains(mainRS, "Box<dyn FnMut(Rc<RefCell<Option<Event>>>, &dyn example_com_dep::Mapper)") {
		t.Fatalf("function type alias should keep imported interface params as trait refs, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "(*__f)(ev.clone(), ev.borrow().as_ref().unwrap())") {
		t.Fatalf("function type call should pass concrete values as imported interface refs, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "use std::any::Any;") {
		t.Fatalf("imported interface impl support should import Any, got:\n%s", mainRS)
	}
}

func TestFunctionTypeAliasFuncLiteralInitializerUsesInnerBox(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type Node interface {
	Name() string
}

type inspector func(Node) bool

func NewInspector() inspector {
	var insp inspector = func(n Node) bool {
		return n.Name() == "alpha"
	}
	return insp
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "Some(Rc::new(RefCell::new(Some(Box::new(move") {
		t.Fatalf("named function literal initializer should store the closure box, not a nested function handle, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "let mut insp: inspector = Rc::new(RefCell::new(Some(Box::new(move |n: Rc<RefCell<Option<Box<dyn Node>>>>|") {
		t.Fatalf("named function literal initializer should wrap one closure box in the alias handle, got:\n%s", mainRS)
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

func TestUnsafePointerToNamedFunctionTypeConversionEmitsUnsupportedTypedPointer(t *testing.T) {
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
	if !strings.Contains(mainRS, "if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Exporter>(unimplemented!(\"unsafe.Pointer conversion to Exporter\")) }") {
		t.Fatalf("pointer conversion from unsafe pointer should preserve nil before emitting a typed unsupported path, got:\n%s", mainRS)
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
	if !strings.Contains(mainRS, "__left.__go_eq_key(__right)") {
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
	if !strings.Contains(mainRS, "let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard);") {
		t.Fatalf("range over wrapped array fields should clone values before the loop and drop the guard, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "fn format_slice<T, C>") || !strings.Contains(mainRS, "C: AsRef<[T]>") {
		t.Fatalf("format_slice should accept arrays and slices through AsRef, got:\n%s", mainRS)
	}
}

func TestNonLiteralFixedArrayLengthUsesTypeInfo(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

var deps = [...]string{"a", "b", "c"}

func Use() byte {
	var seen [1 + len(deps)/8]byte
	seen[0] = 1
	return seen[0]
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "Option<Vec<u8>>") {
		t.Fatalf("non-literal fixed array length should not be translated as a Vec, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Option<[u8; 1]>") {
		t.Fatalf("non-literal fixed array length should use go/types length, got:\n%s", mainRS)
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

func TestStdlibConstStubUsesConstName(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "go/parser"

func mode() parser.Mode {
	return parser.SkipObjectResolution
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "pub const SKIP_OBJECT_RESOLUTION") {
		t.Fatalf("stdlib const stub should use Rust const naming, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "parser::SKIP_OBJECT_RESOLUTION") {
		t.Fatalf("stdlib const selector should use the generated const name, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "parser::skip_object_resolution") {
		t.Fatalf("stdlib const selector should not use snake-case function naming, got:\n%s", mainRS)
	}
}

func TestStdlibVariableStubUsesGlobalName(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "go/types"

func invalid() types.Type {
	return types.Typ[types.Invalid]
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "pub fn Typ()") {
		t.Fatalf("stdlib variable stub should keep the generated global name, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "types::Typ()") {
		t.Fatalf("stdlib variable selector should use the generated global accessor, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "types::typ()") {
		t.Fatalf("stdlib variable selector should not use snake-case function naming, got:\n%s", mainRS)
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
		if !strings.Contains(code, "use crate::{GoTime, go_time_civil_from_days};") {
			t.Fatalf("%s should import crate-root helpers, got:\n%s", name, code)
		}
		if strings.Contains(code, "use crate::*;") {
			t.Fatalf("%s should not glob-import the crate root for helpers, got:\n%s", name, code)
		}
		if strings.Contains(code, "struct GoTime") {
			t.Fatalf("%s should not define a file-local GoTime, got:\n%s", name, code)
		}
	}
}

func TestMainPackageHelperIncludeDeduplicatesRootImports(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "helper.go"), `package main

import "fmt"

func PrintMap(m map[string]int) {
	go func() {}()
	fmt.Println(m)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type Item struct {
	Value int
}

func main() {}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "helper.go"),
		filepath.Join(tempDir, "main.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, `include!("__go2rust_helpers.rs");`) {
		t.Fatalf("main.rs should include package-scoped helpers, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "use std::sync::{Arc, Mutex};") {
		t.Fatalf("main.rs should rely on helper include for Arc/Mutex imports, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "use std::fmt::{Display") {
		t.Fatalf("main.rs should rely on helper include for Display import, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "use std::fmt::{Formatter};") {
		t.Fatalf("main.rs should keep non-helper fmt imports, got:\n%s", mainRS)
	}

	helpersRS := mustReadFile(t, filepath.Join(tempDir, packageHelperIncludeFile))
	if !strings.Contains(helpersRS, "use std::sync::{Arc, Mutex};") {
		t.Fatalf("helper include should import Arc/Mutex for shared helpers, got:\n%s", helpersRS)
	}
	if !strings.Contains(helpersRS, "use std::fmt::{Display};") {
		t.Fatalf("helper include should import Display for shared helpers, got:\n%s", helpersRS)
	}
}

func TestMultiFileVariadicAnyPrintlnImportsCrateRootFormatter(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "printer.go"), `package main

import "fmt"

func Print(values ...any) {
	fmt.Println(values...)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "printer.go"),
		filepath.Join(tempDir, "main.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	printerRS := mustReadFile(t, filepath.Join(tempDir, "printer.rs"))
	if !strings.Contains(printerRS, "format_any_variadic(&values)") {
		t.Fatalf("fmt.Println(values...) should call the variadic any formatter, got:\n%s", printerRS)
	}
	if !strings.Contains(printerRS, "use crate::{format_any, format_any_slice, format_any_variadic};") {
		t.Fatalf("multi-file module should import the crate-root variadic any formatter, got:\n%s", printerRS)
	}

	helpersRS := mustReadFile(t, filepath.Join(tempDir, packageHelperIncludeFile))
	if !strings.Contains(helpersRS, "fn format_any_variadic") {
		t.Fatalf("package helper include should define format_any_variadic, got:\n%s", helpersRS)
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
		if !strings.Contains(code, "use crate::{GoTime, go_time_civil_from_days};") {
			t.Fatalf("%s should import crate-root helpers, got:\n%s", name, code)
		}
		if strings.Contains(code, "use crate::*;") {
			t.Fatalf("%s should not glob-import the crate root for helpers, got:\n%s", name, code)
		}
		if strings.Contains(code, "struct GoTime") {
			t.Fatalf("%s should not define a file-local GoTime, got:\n%s", name, code)
		}
	}
}

func TestPackageHelperImportsAvoidSiblingModuleTypeCollisions(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "termlist.go"), `package main

type termlist struct{}
`)
	writeTestFile(t, filepath.Join(tempDir, "normalize.go"), `package main

import "fmt"

type termSet struct {
	terms termlist
}

func TypeName(v any) string {
	return fmt.Sprintf("%T", v)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {
	println(TypeName(termSet{}))
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "termlist.go"),
		filepath.Join(tempDir, "normalize.go"),
		filepath.Join(tempDir, "main.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	normalizeRS := mustReadFile(t, filepath.Join(tempDir, "normalize.rs"))
	if strings.Contains(normalizeRS, "use crate::*;") {
		t.Fatalf("module should not glob-import crate root and shadow termlist type with termlist module, got:\n%s", normalizeRS)
	}
	if !strings.Contains(normalizeRS, "use crate::{__go_type_name};") {
		t.Fatalf("module should import only the needed crate-root helper, got:\n%s", normalizeRS)
	}
	if !strings.Contains(normalizeRS, "pub terms: Rc<RefCell<Option<termlist>>>") {
		t.Fatalf("sibling termlist type should remain usable through sibling module imports, got:\n%s", normalizeRS)
	}

	helpersRS := mustReadFile(t, filepath.Join(tempDir, packageHelperIncludeFile))
	if !strings.Contains(helpersRS, "use std::any::Any;") {
		t.Fatalf("helper include should import Any for type-name helper, got:\n%s", helpersRS)
	}
}

func TestTypeNameHelperDoesNotCollideWithPackageGoTypeNameFunction(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "typexpr.go"), `package main

import "fmt"

func goTypeName(v any) string {
	return fmt.Sprintf("%T", v)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {
	println(goTypeName(1))
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "typexpr.go"),
		filepath.Join(tempDir, "main.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	typexprRS := mustReadFile(t, filepath.Join(tempDir, "typexpr.rs"))
	if strings.Contains(typexprRS, "use crate::{go_type_name};") {
		t.Fatalf("module should not import a helper name that collides with its own function, got:\n%s", typexprRS)
	}
	if !strings.Contains(typexprRS, "use crate::{__go_type_name};") {
		t.Fatalf("module should import the internal type-name helper, got:\n%s", typexprRS)
	}
	if !strings.Contains(typexprRS, "pub fn go_type_name(") {
		t.Fatalf("source function should still lower to its normal Rust name, got:\n%s", typexprRS)
	}
	if !strings.Contains(typexprRS, "__go_type_name(") {
		t.Fatalf("%%T lowering should call the internal helper, got:\n%s", typexprRS)
	}
}

func TestPackageAnyCloneHelperQualifiesSiblingModuleTypes(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "termlist.go"), `package main

type termlist struct{}
`)
	writeTestFile(t, filepath.Join(tempDir, "use.go"), `package main

func sink(args ...any) {}

func call(x termlist, y any) {
	sink(x)
	sink(y)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "termlist.go"),
		filepath.Join(tempDir, "use.go"),
		filepath.Join(tempDir, "main.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	helpersRS := mustReadFile(t, filepath.Join(tempDir, packageHelperIncludeFile))
	if strings.Contains(helpersRS, "downcast_ref::<termlist>()") {
		t.Fatalf("package helper should not refer to a sibling module type without qualification, got:\n%s", helpersRS)
	}
	if !strings.Contains(helpersRS, "downcast_ref::<crate::termlist::termlist>()") {
		t.Fatalf("package helper should qualify sibling module types, got:\n%s", helpersRS)
	}
}

func TestCrossFileNamedMapZeroValueImportsBTreeMap(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "defs.go"), `package main

type objset map[string]int
`)
	writeTestFile(t, filepath.Join(tempDir, "use.go"), `package main

func collect() {
	var m objset
	_ = m
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "defs.go"),
		filepath.Join(tempDir, "use.go"),
		filepath.Join(tempDir, "main.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	useRS := mustReadFile(t, filepath.Join(tempDir, "use.rs"))
	wantInitializer := "Some(objset(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new())))))"
	if !strings.Contains(useRS, wantInitializer) {
		t.Fatalf("cross-file named map zero value should construct the named map wrapper, got:\n%s", useRS)
	}
	if !strings.Contains(useRS, "use std::collections::BTreeMap;") {
		t.Fatalf("cross-file named map zero value should import BTreeMap at the use site, got:\n%s", useRS)
	}
}

func TestCrossFileMethodImplQualifiesReceiverType(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "defs.go"), `package main

type flag uintptr
`)
	writeTestFile(t, filepath.Join(tempDir, "methods.go"), `package main

func (f flag) PanicNotMap() {
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "defs.go"),
		filepath.Join(tempDir, "methods.go"),
		filepath.Join(tempDir, "main.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	methodsRS := mustReadFile(t, filepath.Join(tempDir, "methods.rs"))
	if strings.Contains(methodsRS, "\nimpl flag {") {
		t.Fatalf("cross-file method impl should not use the unqualified receiver type:\n%s", methodsRS)
	}
	if !strings.Contains(methodsRS, "impl crate::defs::flag {") {
		t.Fatalf("cross-file method impl should qualify the receiver type's defining module:\n%s", methodsRS)
	}
}

func TestCrossFilePromotedMethodUsesPackageMethodSet(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "defs.go"), `package main

type flag uintptr

type Value struct {
	flag
}
`)
	writeTestFile(t, filepath.Join(tempDir, "methods.go"), `package main

func (f flag) PanicNotMap() {
}
`)
	writeTestFile(t, filepath.Join(tempDir, "use.go"), `package main

func (v Value) Use() {
	v.PanicNotMap()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "defs.go"),
		filepath.Join(tempDir, "methods.go"),
		filepath.Join(tempDir, "use.go"),
		filepath.Join(tempDir, "main.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	defsRS := mustReadFile(t, filepath.Join(tempDir, "defs.rs"))
	if !strings.Contains(defsRS, "pub fn panic_not_map(&self)") {
		t.Fatalf("outer type should forward promoted methods declared in sibling files:\n%s", defsRS)
	}
	if !strings.Contains(defsRS, "embedded_ref.panic_not_map()") {
		t.Fatalf("promoted method forwarder should delegate through the embedded field:\n%s", defsRS)
	}
}

func TestCrossFileInterfaceImplUsesPackageMethodSet(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "defs.go"), `package main

type Node interface {
	A() int
	B() int
}

type item struct{}

func (item) A() int {
	return 1
}

func Box(v item) Node {
	return v
}
`)
	writeTestFile(t, filepath.Join(tempDir, "methods.go"), `package main

func (item) B() int {
	return 2
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "defs.go"),
		filepath.Join(tempDir, "methods.go"),
		filepath.Join(tempDir, "main.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	defsRS := mustReadFile(t, filepath.Join(tempDir, "defs.rs"))
	if !strings.Contains(defsRS, "impl Node for item") {
		t.Fatalf("interface impl should use package-wide methods, including sibling-file methods:\n%s", defsRS)
	}
	if !strings.Contains(defsRS, "fn b(&self) -> i32") {
		t.Fatalf("interface impl should include sibling-file method in the trait impl:\n%s", defsRS)
	}
	if !strings.Contains(defsRS, "self.b()") {
		t.Fatalf("interface impl should delegate to the inherent sibling-file method:\n%s", defsRS)
	}
}

func TestAppendPointerReturnKeepsHandle(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type Item struct {
	Value int
}

func NewItem() *Item {
	return &Item{Value: 1}
}

func Use() []*Item {
	var items []*Item
	items = append(items, NewItem())
	return items
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "push((*new_item()") {
		t.Fatalf("append of pointer-returning call should keep the pointer handle, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "push(new_item())") {
		t.Fatalf("append of pointer-returning call should push the returned handle, got:\n%s", mainRS)
	}
}

func TestConcurrentComparableStructUsesCustomPartialEq(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type version struct {
	major string
	minor string
}

func start() {
	go func() {}()
}

func valid(x string) bool {
	return version{major: x} != version{}
}

func lang(v version) bool {
	return v.minor == ""
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "#[derive(Debug, Clone, Default, PartialEq)]") {
		t.Fatalf("concurrent comparable struct should not derive PartialEq over Mutex fields, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "impl PartialEq for version") {
		t.Fatalf("concurrent comparable struct should get custom PartialEq, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "__left.as_ref() == __right.as_ref()") {
		t.Fatalf("custom PartialEq should compare locked field values by reference, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "let __tmp_x = (*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).minor.clone(); __field }.lock().unwrap().as_ref().unwrap());") {
		t.Fatalf("string selector comparison should not move String out of a shared reference, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "let __cloned = (*__selector_guard.as_ref().unwrap()).clone();") {
		t.Fatalf("string selector comparison should clone the selected String value, got:\n%s", mainRS)
	}
}

func TestTypeDefinitionMethodReceiverUsesSelfValue(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "fmt"

type Code int

func start() {
	go func() {}()
}

func (c Code) String() string {
	i := int(c)
	return fmt.Sprintf("Code(%d)", c) + fmt.Sprintf("%d", i)
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "c.lock().unwrap()") {
		t.Fatalf("type-definition method body should translate receiver uses through self, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "(*self.0.lock().unwrap().as_ref().unwrap()) as i32") {
		t.Fatalf("type-definition receiver conversion should use self.0, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "(*self.0.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("type-definition receiver format argument should clone from self.0, got:\n%s", mainRS)
	}
}

func TestNamedSliceMethodReceiverUsesSelfValue(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type Items []int

func start() {
	go func() {}()
}

func (xs Items) equal(ys Items) bool {
	return xs.subset(ys) && ys.subset(xs)
}

func (xs Items) subset(ys Items) bool {
	return len(xs) <= len(ys)
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "xs.lock().unwrap()") {
		t.Fatalf("named-slice method body should translate receiver uses through self, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "Some((*self.0.lock().unwrap().as_ref().unwrap()).clone())") {
		t.Fatalf("named-slice receiver argument should pass the named slice value, not its inner Vec, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Some(self.clone())") {
		t.Fatalf("named-slice receiver argument should pass self.clone(), got:\n%s", mainRS)
	}
}

func TestNamedSliceNilConversionMethodReceiverUsesNamedValue(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type Items []int

func start() {
	go func() {}()
}

func (z Items) shift(xs Items, s uint) Items {
	return z
}

func use(xs Items, s uint) Items {
	return Items(nil).shift(xs, s)
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "None::<Items>") {
		t.Fatalf("named-slice nil conversion receiver should not be a missing Items handle, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "Arc::new(Mutex::new(None::<Items>)).shift") {
		t.Fatalf("named-slice nil conversion receiver should call the method on an Items value, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Items(Arc::new(Mutex::new(None::<Vec<i32>>))).shift") {
		t.Fatalf("named-slice nil conversion receiver should construct the named slice value, got:\n%s", mainRS)
	}
}

func TestUnaryMinusWrappedIntFieldUsesInnerValue(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type decimal struct {
	mant []byte
	exp  int
}

func start() {
	go func() {}()
}

func appendZeros(buf []byte, n int) []byte {
	return buf
}

func (d *decimal) String() []byte {
	var buf []byte
	if d.exp <= 0 {
		buf = make([]byte, 0, 2+(-d.exp)+len(d.mant))
		buf = appendZeros(buf, -d.exp)
	}
	return buf
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "-self.exp.clone()") {
		t.Fatalf("unary minus on wrapped int field should unwrap the field value, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "-({ let __selector_holder = self.exp.clone();") {
		t.Fatalf("unary minus on wrapped int field should negate the inner value, got:\n%s", mainRS)
	}
}

func TestNumericTypeDefinitionOps(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type Code int

func start() {
	go func() {}()
}

func (c Code) Check() bool {
	return 1 <= c && c <= 3 && c+1 == 4 && 1+c == 4 && c-1 == 2 && c-c == 0
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	for _, want := range []string{
		"impl PartialEq<i32> for Code",
		"impl PartialOrd<i32> for Code",
		"impl PartialEq<Code> for i32",
		"impl PartialOrd<Code> for i32",
		"impl std::ops::Add<i32> for Code",
		"impl std::ops::Add<Code> for i32",
		"impl std::ops::Sub for Code",
	} {
		if !strings.Contains(mainRS, want) {
			t.Fatalf("numeric type definition should generate %q, got:\n%s", want, mainRS)
		}
	}
}

func TestCrossFileNamedIntegerShiftShortDeclStoresNamedValue(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "arith.go"), `package main

type Word uint
type nat []Word
`)
	writeTestFile(t, filepath.Join(tempDir, "round.go"), `package main

func start() {
	go func() {}()
}

func rounded(z nat, ntz uint32) bool {
	lsb := Word(1) << ntz
	return z[0]&lsb != 0
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "arith.go"),
		filepath.Join(tempDir, "round.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	roundRS := mustReadFile(t, filepath.Join(tempDir, "round.rs"))
	if strings.Contains(roundRS, "let mut lsb = Arc::new(Mutex::new(Some({") {
		t.Fatalf("cross-file named integer shift short declaration should store Word, not the raw scalar:\n%s", roundRS)
	}
	if !strings.Contains(roundRS, "let mut lsb = Arc::new(Mutex::new(Some(Word(") {
		t.Fatalf("cross-file named integer shift short declaration should wrap Word:\n%s", roundRS)
	}
}

func TestCrossFileNamedSliceCallArgumentUsesInnerHandleForUnnamedSliceParam(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "arith.go"), `package main

type Word uint
type nat []Word

func addVW(z, x []Word, y Word) Word {
	return y
}
`)
	writeTestFile(t, filepath.Join(tempDir, "round.go"), `package main

func start() {
	go func() {}()
}

type Float struct {
	mant nat
}

func rounded(z *Float, lsb Word) Word {
	return addVW(z.mant, z.mant, lsb)
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "arith.go"),
		filepath.Join(tempDir, "round.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	roundRS := mustReadFile(t, filepath.Join(tempDir, "round.rs"))
	if strings.Contains(roundRS, "let __field = (*z.lock().unwrap().as_ref().unwrap()).mant.clone(); __field") {
		t.Fatalf("named-slice field passed to []Word parameter should not pass the nat handle:\n%s", roundRS)
	}
	if strings.Count(roundRS, ".mant.lock().unwrap().as_ref().unwrap()).0.clone()") != 2 {
		t.Fatalf("named-slice field passed to []Word parameter should pass inner slice handles:\n%s", roundRS)
	}
}

func TestCrossFileNamedSliceFieldCompoundAssignUsesInnerHandle(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "arith.go"), `package main

type Word uint
type nat []Word
`)
	writeTestFile(t, filepath.Join(tempDir, "round.go"), `package main

func start() {
	go func() {}()
}

type Float struct {
	mant nat
}

func rounded(z *Float, n int, lsb Word) {
	const msb Word = 1 << 31
	z.mant[n-1] |= msb
	z.mant[0] &^= lsb - 1
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "arith.go"),
		filepath.Join(tempDir, "round.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	roundRS := mustReadFile(t, filepath.Join(tempDir, "round.rs"))
	if strings.Contains(roundRS, ".mant.lock().unwrap().as_mut().unwrap(); __seq[__idx]") {
		t.Fatalf("named-slice field compound assignment should not index the named wrapper:\n%s", roundRS)
	}
	if strings.Count(roundRS, ".mant.lock().unwrap().as_ref().unwrap()).0.clone()") != 2 {
		t.Fatalf("named-slice field compound assignment should mutate inner slice handles:\n%s", roundRS)
	}
}

func TestGlobalInitDoesNotCallDuplicateNameOverrides(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

var names = []string{"one"}

func UsedIdent() int {
	return 1
}

func usedIdent() int {
	return 2
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "__go_init_globals();") {
		t.Fatalf("package globals should still be initialized, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "__go_init_0();") {
		t.Fatalf("duplicate function name override should not be treated as an init function, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "pub fn used_ident_1()") {
		t.Fatalf("duplicate Rust function name should still get an override, got:\n%s", mainRS)
	}
}

func TestPackageInitAllQualifiesLocalInitHelpers(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

var count = 1

func init() {
	count = 2
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	for _, want := range []string{
		"fn __go_init_globals()",
		"fn __go_init_0()",
		"self::__go_init_globals();",
		"self::__go_init_0();",
	} {
		if !strings.Contains(mainRS, want) {
			t.Fatalf("package init helper should be qualified as %q, got:\n%s", want, mainRS)
		}
	}
	if strings.Contains(mainRS, "\n    __go_init_globals();") || strings.Contains(mainRS, "\n    __go_init_0();") {
		t.Fatalf("package init helper calls should not be unqualified, got:\n%s", mainRS)
	}
}

func TestSiblingModuleInitAllNamesAreUnique(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

var rootValue = 1

func main() {}
`)
	writeTestFile(t, filepath.Join(tempDir, "alpha.go"), `package main

const alphaMarker = "pub(crate) fn __go_init_all()"

var alphaValue = 2
`)
	writeTestFile(t, filepath.Join(tempDir, "beta.go"), `package main

var betaValue = 3
`)
	writeTestFile(t, filepath.Join(tempDir, "gamma.go"), `package main

var gammaValue = 4
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "alpha.go"),
		filepath.Join(tempDir, "beta.go"),
		filepath.Join(tempDir, "gamma.go"),
		filepath.Join(tempDir, "main.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	for _, want := range []string{
		"alpha::__go_init_all_alpha();",
		"beta::__go_init_all_beta();",
		"gamma::__go_init_all_gamma();",
	} {
		if !strings.Contains(mainRS, want) {
			t.Fatalf("main should call module-specific init helper %q, got:\n%s", want, mainRS)
		}
	}
	if strings.Contains(mainRS, "alpha::__go_init_all();") || strings.Contains(mainRS, "beta::__go_init_all();") || strings.Contains(mainRS, "gamma::__go_init_all();") {
		t.Fatalf("main should not call ambiguous module init helper names, got:\n%s", mainRS)
	}

	alphaRS := mustReadFile(t, filepath.Join(tempDir, "alpha.rs"))
	if !strings.Contains(alphaRS, "pub(crate) fn __go_init_all_alpha()") {
		t.Fatalf("alpha module should rename its init helper, got:\n%s", alphaRS)
	}
	if strings.Contains(alphaRS, "pub(crate) fn __go_init_all() {") {
		t.Fatalf("alpha module should not keep the shared init helper name, got:\n%s", alphaRS)
	}
	if !strings.Contains(alphaRS, "pub(crate) fn __go_init_all()") {
		t.Fatalf("alpha module should preserve string literals that mention init helper names, got:\n%s", alphaRS)
	}
}

func TestBlankPackageVarDoesNotRequireGlobalInit(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type Named interface {
	Name() string
}

type thing struct{}

func (*thing) Name() string { return "thing" }

var _ Named = (*thing)(nil)
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "__go_init_globals") {
		t.Fatalf("blank package var should not require a missing global init helper, got:\n%s", mainRS)
	}
}

func TestLocalAnonymousStructTypeEmitsDefinition(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func check() {
	var x [1]struct{}
	_ = x[0]
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "struct AnonymousStruct1") {
		t.Fatalf("local anonymous struct type should emit a struct definition, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "[AnonymousStruct1; 1]") {
		t.Fatalf("local anonymous struct array should use the generated type, got:\n%s", mainRS)
	}
}

func TestExportedAnonymousStructGlobalFieldsArePublic(t *testing.T) {
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

type CacheLinePad struct{}

var ARM64 struct {
	_ CacheLinePad
	HasDIT bool
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

var Supported = dep.ARM64.HasDIT
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "mod.rs"))
	if strings.Contains(depRS, "pub(crate) struct AnonymousStruct1") || strings.Contains(depRS, "pub(crate) has_d_i_t:") {
		t.Fatalf("exported anonymous struct global fields must be public across package crates, got:\n%s", depRS)
	}
	if !strings.Contains(depRS, "pub struct AnonymousStruct1") || !strings.Contains(depRS, "pub has_d_i_t:") {
		t.Fatalf("exported anonymous struct global field should be public, got:\n%s", depRS)
	}
}

func TestUnsafeOffsetofImportedAnonymousStructGlobalUsesTypeAlias(t *testing.T) {
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

var CPU struct {
	Flag bool
	Count uint64
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import (
	"unsafe"

	"example.com/dep"
)

const OffsetCount = unsafe.Offsetof(dep.CPU.Count)

func main() {
	println(OffsetCount)
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "mod.rs"))
	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(depRS, "pub type CPU = AnonymousStruct1;") {
		t.Fatalf("external anonymous struct global should expose a type alias, got:\n%s", depRS)
	}
	if !strings.Contains(mainRS, "std::mem::offset_of!(example_com_dep::CPU, count)") {
		t.Fatalf("unsafe.Offsetof should use the imported package global type alias, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "/* unknown struct */") {
		t.Fatalf("unsafe.Offsetof should not emit an unknown struct placeholder, got:\n%s", mainRS)
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
	if !strings.Contains(eventRS, "#[derive(Clone)]\npub struct Event") {
		t.Fatalf("struct with imported field should still derive Clone, got:\n%s", eventRS)
	}
	if !strings.Contains(eventRS, "impl Default for Event") {
		t.Fatalf("struct with imported array field should get an explicit Go zero-value Default, got:\n%s", eventRS)
	}
	if !strings.Contains(eventRS, "pub fn __go_value_clone(&self) -> Self") {
		t.Fatalf("struct with imported field should get a Go value-copy helper, got:\n%s", eventRS)
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

func TestSourceStdlibPackagePatterns(t *testing.T) {
	for _, tt := range []struct {
		patterns string
		path     string
		want     bool
	}{
		{patterns: "go/token", path: "go/token", want: true},
		{patterns: "go/...", path: "go/types", want: true},
		{patterns: "go/...", path: "go", want: true},
		{patterns: "internal/types/...", path: "internal/types/errors", want: true},
		{patterns: "go/token", path: "go/types", want: false},
	} {
		got := sourceStdlibPackagePatternMatches(tt.path, tt.patterns)
		if got != tt.want {
			t.Fatalf("sourceStdlibPackagePatternMatches(%q, %q) = %v, want %v", tt.path, tt.patterns, got, tt.want)
		}
	}
}

func TestPackageLoaderIncludesMappedStdlibSourcePackage(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "go/token")
	loader := &PackageLoader{
		mainPkg: &packages.Package{PkgPath: "main"},
		allPackages: map[string]*packages.Package{
			"main":     {PkgPath: "main"},
			"go/token": {PkgPath: "go/token"},
			"fmt":      {PkgPath: "fmt"},
		},
		packageMapping: map[string]string{
			"go/token": "go_token",
		},
	}

	got := loader.orderedPackagePaths()
	want := []string{"go/token"}
	if strings.Join(got, ",") != strings.Join(want, ",") {
		t.Fatalf("orderedPackagePaths() = %v, want %v", got, want)
	}

	deps := packageDependencyCrates(map[string]*packages.Package{
		"go/token": {PkgPath: "go/token"},
		"fmt":      {PkgPath: "fmt"},
	}, "go_types", loader.packageMapping)
	if strings.Join(deps, ",") != "go_token" {
		t.Fatalf("packageDependencyCrates() = %v, want [go_token]", deps)
	}
}

func TestSourceStdlibPackageUsesMappedCrate(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "go/token")
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/source-stdlib

go 1.24
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "go/token"

func main() {
	var pos token.Pos = token.NoPos
	println(int(pos))
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "go_token::") {
		t.Fatalf("source stdlib package selectors should use the mapped crate, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, " token::") || strings.Contains(mainRS, "\ntoken::") || strings.Contains(mainRS, "(token::") {
		t.Fatalf("source stdlib package should not use the stub-style package module, got:\n%s", mainRS)
	}
	if _, err := os.Stat(filepath.Join(tempDir, "vendor", "go_token", "lib.rs")); err != nil {
		t.Fatalf("source stdlib package should generate vendor/go_token/lib.rs: %v", err)
	}
}

func TestSourceStdlibPackageLocalInterfacesDoNotRegisterExternalStubs(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "go/build/constraint")
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/source-constraint

go 1.24
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "go/build/constraint"

func main() {
	x, err := constraint.Parse("+build linux")
	if err != nil {
		println(err.Error())
	}
	_ = x
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	stubsRS := mustReadFile(t, filepath.Join(tempDir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	exprRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "go_build_constraint", "expr.rs"))
	if strings.Contains(stubsRS, "pub struct Expr;") {
		t.Fatalf("source stdlib local interface Expr should not be emitted in shared stubs:\n%s", stubsRS)
	}
	if !strings.Contains(exprRS, "pub trait Expr") {
		t.Fatalf("source stdlib Expr should be emitted as a local trait, got:\n%s", exprRS)
	}
	if !strings.Contains(exprRS, "let any_val = x.__go_as_any();") {
		t.Fatalf("source stdlib interface assertion should use the generated local trait object, got:\n%s", exprRS)
	}
	if strings.Contains(exprRS, "typed_val) = val.downcast_ref::<") {
		t.Fatalf("source stdlib interface assertion should not use stub-style bare downcast handling, got:\n%s", exprRS)
	}
}

func TestSourceStdlibImportedInterfaceImplEmitsTraitImpl(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "parser.go", `package parser

import "go/ast"

type resolver struct{}

func (r resolver) Visit(node ast.Node) ast.Visitor {
	return r
}

func Use() {
	ast.Walk(resolver{}, nil)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(parser.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/ast": "go_ast"})
	if !strings.Contains(rust, "impl go_ast::Visitor for resolver") {
		t.Fatalf("source-mapped stdlib interface impl should be emitted with the concrete type, got:\n%s", rust)
	}
}

func TestSourceStdlibReachabilityPrunesUnusedDeclarations(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "go/token")
	prevCtx := GetTranspileContext()
	SetTranspileContext(&TranspileContext{PackageMapping: map[string]string{"go/token": "go_token"}})
	defer SetTranspileContext(prevCtx)
	prevReachable := sourceStdlibReachable
	defer SetSourceStdlibReachable(prevReachable)

	fset := token.NewFileSet()
	tokenPkg := parsePackageForReachabilityTest(t, fset, "go/token", "token.go", `package token

type File struct{}
func (f *File) Position() int { return helper() }

type Box[T any] struct{}
func (b Box[T]) Touch() int { return helper() }

func helper() int { return 1 }
func unused() int { return 0 }

type Unused struct{}
func (Unused) Drop() {}
`)
	mainPkg := parsePackageForReachabilityTest(t, fset, "main", "main.go", `package main

import "go/token"

func main() {
	var f *token.File
	_ = f.Position()
	var _ token.Box[int]
}
`)
	loader := &PackageLoader{
		fileSet: fset,
		mainPkg: mainPkg,
		allPackages: map[string]*packages.Package{
			"main":     mainPkg,
			"go/token": tokenPkg,
		},
		packageMapping: map[string]string{"go/token": "go_token"},
	}
	if err := loader.typeCheckLocalPackage(tokenPkg, loader.projectImporter()); err != nil {
		t.Fatalf("typeCheckLocalPackage(go/token) error = %v", err)
	}
	if err := loader.typeCheckLocalPackage(mainPkg, loader.projectImporter()); err != nil {
		t.Fatalf("typeCheckLocalPackage(main) error = %v", err)
	}

	SetSourceStdlibReachable(loader.computeSourceStdlibReachable())
	for _, name := range []string{"File", "Position", "Box", "Touch", "helper"} {
		if sourceMappedDeclIsPruned(definitionByName(t, tokenPkg, name)) {
			t.Fatalf("%s should be reachable in source stdlib package", name)
		}
	}
	for _, name := range []string{"unused", "Unused", "Drop"} {
		if !sourceMappedDeclIsPruned(definitionByName(t, tokenPkg, name)) {
			t.Fatalf("%s should be pruned from source stdlib package", name)
		}
	}
}

func TestSourceStdlibPruningSkipsUnreachableDeclarationsInOutput(t *testing.T) {
	prevCtx := GetTranspileContext()
	SetTranspileContext(&TranspileContext{PackageMapping: map[string]string{"go/token": "go_token"}})
	defer SetTranspileContext(prevCtx)
	prevReachable := sourceStdlibReachable
	defer SetSourceStdlibReachable(prevReachable)

	fset := token.NewFileSet()
	pkg := parsePackageForReachabilityTest(t, fset, "go/token", "token.go", `package token

// Live docs.
type Live struct{}
func (Live) Keep() {}

// Dead docs.
type Dead struct{}
func (Dead) Drop() {}

func live() {}
func dead() {}
`)
	typeInfo, err := NewTypeInfoWithImporter("go/token", pkg.Syntax, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(go/token) error = %v", err)
	}
	pkg.Types = typeInfo.pkg
	pkg.TypesInfo = typeInfo.info
	SetSourceStdlibReachable(map[types.Object]bool{
		definitionByName(t, pkg, "Live"): true,
		definitionByName(t, pkg, "live"): true,
	})

	rust, _, _ := TranspileWithMapping(pkg.Syntax[0], fset, typeInfo, map[string]string{"go/token": "go_token"})
	for _, want := range []string{"pub struct Live", "pub fn live"} {
		if !strings.Contains(rust, want) {
			t.Fatalf("reachable declaration %q missing from output:\n%s", want, rust)
		}
	}
	for _, unwanted := range []string{"pub struct Dead", "fn dead", "Dead docs", "impl Dead"} {
		if strings.Contains(rust, unwanted) {
			t.Fatalf("unreachable declaration %q should be pruned from output:\n%s", unwanted, rust)
		}
	}
}

// TestSourceStdlibPruningSkipsCrossFileImplForUnreachableType covers the case
// where a pruned type's methods live in a different file from its type decl -
// exactly token.FileSet (declared in position.go) with Read/Write methods in
// serialize.go. The per-file prunedTypeNames gate only sees types declared in
// the file being emitted, so the methods file would emit `impl Dead { ... }`
// for a type that was never declared in any emitted file. The receiver-type
// reachability gate must skip the whole impl block.
func TestSourceStdlibPruningSkipsCrossFileImplForUnreachableType(t *testing.T) {
	prevCtx := GetTranspileContext()
	SetTranspileContext(&TranspileContext{PackageMapping: map[string]string{"go/token": "go_token"}})
	defer SetTranspileContext(prevCtx)
	prevReachable := sourceStdlibReachable
	defer SetSourceStdlibReachable(prevReachable)

	fset := token.NewFileSet()
	posFile, err := parser.ParseFile(fset, "position.go", `package token

type Live struct{}
type Dead struct{}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(position.go) error = %v", err)
	}
	serFile, err := parser.ParseFile(fset, "serialize.go", `package token

func (Live) Keep() {}
func (Dead) Drop() {}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(serialize.go) error = %v", err)
	}

	typeInfo, err := NewTypeInfoWithImporter("go/token", []*ast.File{posFile, serFile}, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(go/token) error = %v", err)
	}
	pkg := &packages.Package{
		Name:      "token",
		PkgPath:   "go/token",
		Fset:      fset,
		Syntax:    []*ast.File{posFile, serFile},
		Types:     typeInfo.pkg,
		TypesInfo: typeInfo.info,
	}
	SetSourceStdlibReachable(map[types.Object]bool{
		definitionByName(t, pkg, "Live"): true,
		definitionByName(t, pkg, "Keep"): true,
	})

	// Emit the methods file. Dead is unreachable and declared elsewhere, so its
	// impl block must be skipped; Live's reachable impl must remain.
	rust, _, _ := TranspileWithMapping(serFile, fset, typeInfo, map[string]string{"go/token": "go_token"})
	if strings.Contains(rust, "impl Dead") {
		t.Fatalf("pruned cross-file type Dead should not get an impl block:\n%s", rust)
	}
	if !strings.Contains(rust, "impl Live") {
		t.Fatalf("reachable cross-file impl for Live missing from output:\n%s", rust)
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

func TestImportedAliasStructLiteralConstructsAliasedStruct(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "abi", "abi.go"), `package abi

type Name string

type Type struct{}

type StructField struct {
	Name Name
	Typ *Type
	Offset uintptr
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/mainmod/abi"

type structField = abi.StructField

func makeField(n abi.Name, t *abi.Type) structField {
	return structField{Name: n, Typ: t, Offset: 0}
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "structField {") {
		t.Fatalf("alias struct literal must not construct the alias wrapper name:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "example_com_mainmod_abi::StructField {") {
		t.Fatalf("alias struct literal should construct the aliased package struct:\n%s", mainRS)
	}
}

func TestImportedConstSelectorCastsToUint8StructField(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "dep", "dep.go"), `package dep

const PtrSize = 8

type Type struct {
	Align_ uint8
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/mainmod/dep"

func build() dep.Type {
	return dep.Type{Align_: dep.PtrSize}
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "Some(example_com_mainmod_dep::PTR_SIZE))") {
		t.Fatalf("imported const selector assigned to uint8 field should be cast before wrapping:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Some(example_com_mainmod_dep::PTR_SIZE as u8)") {
		t.Fatalf("imported const selector assigned to uint8 field should cast to u8:\n%s", mainRS)
	}
}

func parsePackageForReachabilityTest(t *testing.T, fset *token.FileSet, pkgPath, filename, source string) *packages.Package {
	t.Helper()
	file, err := parser.ParseFile(fset, filename, source, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(%s) error = %v", filename, err)
	}
	return &packages.Package{
		Name:            file.Name.Name,
		PkgPath:         pkgPath,
		Fset:            fset,
		GoFiles:         []string{filename},
		CompiledGoFiles: []string{filename},
		Syntax:          []*ast.File{file},
		Imports:         make(map[string]*packages.Package),
	}
}

func definitionByName(t *testing.T, pkg *packages.Package, name string) types.Object {
	t.Helper()
	if pkg == nil || pkg.TypesInfo == nil {
		t.Fatalf("package %s has no type info", name)
	}
	for ident, obj := range pkg.TypesInfo.Defs {
		if ident != nil && ident.Name == name && obj != nil {
			return obj
		}
	}
	t.Fatalf("definition %s not found", name)
	return nil
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
