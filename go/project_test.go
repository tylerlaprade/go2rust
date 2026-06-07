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

func TestModuleImportPrefixesDoNotAddBlankLineForEmptyBody(t *testing.T) {
	sibling := prefixSiblingModuleImports("", "compiletype", []string{"compiletype", "escape"})
	if sibling != "use crate::escape::*;\n" {
		t.Fatalf("empty module sibling imports should not add a trailing blank line: %q", sibling)
	}

	helpers := &HelperTracker{needsFormatSlice: true}
	helperImports := prefixPackageHelperImports("", helpers, false)
	if helperImports != "use crate::{format_slice, format_slice_values, format_slice_wrapped};\n" {
		t.Fatalf("empty module helper imports should not add a trailing blank line: %q", helperImports)
	}

	shared := prefixSharedStdlibStubImport("")
	if shared != "use go2rust_stdlib_stubs::*;\n" {
		t.Fatalf("empty module shared stub import should not add a trailing blank line: %q", shared)
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

func TestProductionParseFileCallsSkipObjectResolution(t *testing.T) {
	for _, filename := range []string{"project.go", "package_loader.go", "transpile.go", "unified_transpiler.go"} {
		t.Run(filename, func(t *testing.T) {
			fset := token.NewFileSet()
			source, err := os.ReadFile(filename)
			if err != nil {
				t.Fatalf("ReadFile(%s) error = %v", filename, err)
			}
			file, err := parser.ParseFile(fset, filename, source, parser.ParseComments|parser.SkipObjectResolution)
			if err != nil {
				t.Fatalf("ParseFile(%s) error = %v", filename, err)
			}

			ast.Inspect(file, func(n ast.Node) bool {
				call, ok := n.(*ast.CallExpr)
				if !ok || !isParserParseFileCall(call) {
					return true
				}
				if len(call.Args) < 4 {
					t.Fatalf("%s: parser.ParseFile call has %d args", fset.Position(call.Pos()), len(call.Args))
				}
				if !parseFileModeIncludesSkipObjectResolution(call.Args[3]) {
					t.Fatalf("%s: production parser.ParseFile call must include parser.SkipObjectResolution", fset.Position(call.Pos()))
				}
				return true
			})
		})
	}
}

func isParserParseFileCall(call *ast.CallExpr) bool {
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "ParseFile" {
		return false
	}
	pkg, ok := sel.X.(*ast.Ident)
	return ok && pkg.Name == "parser"
}

func parseFileModeIncludesSkipObjectResolution(expr ast.Expr) bool {
	if isParserSkipObjectResolution(expr) {
		return true
	}
	binary, ok := expr.(*ast.BinaryExpr)
	if !ok || binary.Op != token.OR {
		return false
	}
	return parseFileModeIncludesSkipObjectResolution(binary.X) ||
		parseFileModeIncludesSkipObjectResolution(binary.Y)
}

func isParserSkipObjectResolution(expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "SkipObjectResolution" {
		return false
	}
	pkg, ok := sel.X.(*ast.Ident)
	return ok && pkg.Name == "parser"
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
	implCount := strings.Count(typeRS, "impl I for T {") + strings.Count(extraRS, "impl I for T {")
	if implCount != 1 {
		t.Fatalf("cross-file methods should not emit duplicate trait impls, got %d\ntype.rs:\n%s\nextra.rs:\n%s", implCount, typeRS, extraRS)
	}
	if strings.Contains(extraRS, "impl I for") {
		t.Fatalf("trait impl should be emitted with the concrete type declaration, not each file adding methods:\n%s", extraRS)
	}
}

func TestImportedGenericSelectorCallEmitsInferredTypeArgs(t *testing.T) {
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

func Search[S ~[]E, E any, T any](x S, target T, cmp func(E, T) int) (int, bool) {
	if len(x) > 0 {
		return cmp(x[0], target), false
	}
	return 0, false
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type item struct {
	offset int
}

func use(items []item, x int) int {
	i, _ := dep.Search(items, x, func(a item, x int) int {
		return a.offset - x
	})
	return i
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "example_com_dep::search::<Vec<item>, item, i32>(") {
		t.Fatalf("imported generic selector call should emit inferred Rust type arguments:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "example_com_dep::search::<Vec<item>, item, i32>(items.clone(),") {
		t.Fatalf("imported generic selector call should not pass a bare concrete slice to a slice-type-param slot:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, ".iter().cloned().map(|__elem| Rc::new(RefCell::new(Some(__elem))))") &&
		!strings.Contains(mainRS, ".iter().cloned().map(|__elem| Arc::new(Mutex::new(Some(__elem))))") {
		t.Fatalf("imported generic selector call should adapt concrete slice elements to type-param handles:\n%s", mainRS)
	}
}

func TestSharedGoValueCloneTraitCrossesTranspiledCrates(t *testing.T) {
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

func First[E any](values []E) E {
	return values[0]
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type item struct {
	offset int
}

func use(items []item) item {
	return dep.First(items)
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "use go2rust_stdlib_stubs::*;") {
		t.Fatalf("root package should import shared stdlib helpers, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "impl GoValueClone for item") {
		t.Fatalf("root struct passed to imported generic helper should implement shared GoValueClone, got:\n%s", mainRS)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "mod.rs"))
	if !strings.Contains(depRS, "pub fn first<E: Any + GoValueClone + 'static>") {
		t.Fatalf("dependency generic helper should require shared GoValueClone, got:\n%s", depRS)
	}
	if strings.Contains(depRS, "pub trait GoValueClone") {
		t.Fatalf("dependency crate should use the shared GoValueClone helper, not define its own, got:\n%s", depRS)
	}

	sharedLib := mustReadFile(t, filepath.Join(tempDir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	if !strings.Contains(sharedLib, "pub trait GoValueClone") {
		t.Fatalf("shared stdlib helper crate should define GoValueClone, got:\n%s", sharedLib)
	}
}

func TestSharedGoValueCloneKeepsLocalAnyCloneArms(t *testing.T) {
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

func Touch() {}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type marker struct{}

func first[E any](values []E) E {
	return values[0]
}

func use(values []marker) marker {
	dep.Touch()
	return first(values)
}

func fail() {
	done := make(chan bool)
	_ = done
	panic(marker{})
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "use go2rust_stdlib_stubs::*;") {
		t.Fatalf("root package should still import shared helpers, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "impl GoValueClone for marker") {
		t.Fatalf("root local type should still implement shared GoValueClone, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "fn go_any_clone(value: &(dyn Any + Send + Sync))") ||
		!strings.Contains(mainRS, "value.downcast_ref::<marker>()") {
		t.Fatalf("root package should keep local any clone arms for local dynamic panic payloads, got:\n%s", mainRS)
	}
}

func TestSharedGoComparableTraitCrossesTranspiledCrates(t *testing.T) {
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

func Contains[E comparable](values []E, target E) bool {
	for _, value := range values {
		if value == target {
			return true
		}
	}
	return false
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type item struct {
	offset int
}

func use(items []*item, target *item) bool {
	return dep.Contains(items, target)
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "use go2rust_stdlib_stubs::*;") {
		t.Fatalf("root package should import shared stdlib helpers, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "impl GoComparable for item") {
		t.Fatalf("root pointer-comparable type should implement shared GoComparable, got:\n%s", mainRS)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "mod.rs"))
	if !strings.Contains(depRS, "pub fn contains<E: Any + GoComparable + GoValueClone + 'static>") {
		t.Fatalf("dependency generic helper should require shared GoComparable, got:\n%s", depRS)
	}
	if strings.Contains(depRS, "pub trait GoComparable") {
		t.Fatalf("dependency crate should use the shared GoComparable helper, not define its own, got:\n%s", depRS)
	}

	sharedLib := mustReadFile(t, filepath.Join(tempDir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	if !strings.Contains(sharedLib, "pub trait GoComparable") {
		t.Fatalf("shared stdlib helper crate should define GoComparable, got:\n%s", sharedLib)
	}
}

func TestImportedGenericGoValueCloneHelperAcceptsLocalInterface(t *testing.T) {
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

func First[E any](values []E) E {
	return values[0]
}
`)
	writeTestFile(t, filepath.Join(tempDir, "types.go"), `package main

type Spec interface {
	specNode()
}

type ImportSpec struct{}

func (*ImportSpec) specNode() {}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func use(specs []Spec) Spec {
	return dep.First(specs)
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "types.go"),
		filepath.Join(tempDir, "main.go"),
	})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	typesRS := mustReadFile(t, filepath.Join(tempDir, "types.rs"))
	want := "impl GoValueClone for Box<dyn Spec> {\n    fn go_value_clone(&self) -> Self {\n        Spec::__go_clone_box_spec(self.as_ref())\n    }\n}"
	if !strings.Contains(typesRS, want) {
		t.Fatalf("local interface used with imported GoValueClone generic helper should implement the trait, missing %q:\n%s", want, typesRS)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "mod.rs"))
	if !strings.Contains(depRS, "pub fn first<E: Any + GoValueClone + 'static>") {
		t.Fatalf("dependency generic helper should require shared GoValueClone, got:\n%s", depRS)
	}
}

func TestImportedGenericGoComparableHelperAcceptsLocalInterface(t *testing.T) {
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

func Contains[E comparable](values []E, target E) bool {
	for _, value := range values {
		if value == target {
			return true
		}
	}
	return false
}
`)
	writeTestFile(t, filepath.Join(tempDir, "types.go"), `package main

type Spec interface {
	specNode()
}

type Item struct {
	id int
}

func (Item) specNode() {}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func use(specs []Spec, target Spec) bool {
	return dep.Contains(specs, target)
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "types.go"),
		filepath.Join(tempDir, "main.go"),
	})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	typesRS := mustReadFile(t, filepath.Join(tempDir, "types.rs"))
	want := "impl GoComparable for Box<dyn Spec> {\n    fn go_eq(&self, other: &Self) -> bool {\n        self.__go_eq_spec(other.as_ref())\n    }"
	if !strings.Contains(typesRS, want) {
		t.Fatalf("local interface used as a GoComparable generic argument should implement the trait, missing %q:\n%s", want, typesRS)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_dep", "mod.rs"))
	if !strings.Contains(depRS, "pub fn contains<E: Any + GoComparable + GoValueClone + 'static>") {
		t.Fatalf("dependency generic helper should require shared GoComparable, got:\n%s", depRS)
	}
	if strings.Contains(depRS, "pub trait GoComparable") {
		t.Fatalf("dependency crate should use the shared GoComparable helper, not define its own, got:\n%s", depRS)
	}
}

func TestImportedGenericSelectorCallWrapsFunctionIdentifierArgument(t *testing.T) {
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

func Search[S ~[]E, E any, T any](x S, target T, cmp func(E, T) int) (int, bool) {
	if len(x) > 0 {
		return cmp(x[0], target), false
	}
	return 0, false
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type item struct {
	offset int
}

func compare(a item, x int) int {
	return a.offset - x
}

func use(items []item, x int) int {
	i, _ := dep.Search(items, x, compare)
	return i
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "Some(compare)") {
		t.Fatalf("function identifier argument should be boxed as a function value, not stored as a raw function item:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Box::new(move |__arg0:") || !strings.Contains(mainRS, "compare(__arg0, __arg1)") {
		t.Fatalf("function identifier argument should lower through a boxed closure:\n%s", mainRS)
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
	if strings.Contains(mainRS, "impl positioner for example_com_dep::Node") || strings.Contains(mainRS, "impl positioner for example_com_dep::r#mod::Node") {
		t.Fatalf("imported interface cases should not emit an impl for the trait type itself:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "impl positioner for Box<dyn example_com_dep::Node>") &&
		!strings.Contains(mainRS, "impl positioner for Box<dyn example_com_dep::r#mod::Node>") {
		t.Fatalf("imported interface cases should implement the local interface for the boxed trait object:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "(**self).pos()") {
		t.Fatalf("boxed imported interface impl should delegate methods through the inner trait object:\n%s", mainRS)
	}
}

func TestExternalInterfaceCallArgumentImplementsLocalInterfaceForTraitObject(t *testing.T) {
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

type Expr interface {
	Node
	exprNode()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type positioner interface {
	Pos() int
}

func report(at positioner) int {
	return at.Pos()
}

func use(expr dep.Expr) int {
	return report(expr)
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
	if !strings.Contains(mainRS, "impl positioner for Box<dyn example_com_dep::Expr>") &&
		!strings.Contains(mainRS, "impl positioner for Box<dyn example_com_dep::r#mod::Expr>") {
		t.Fatalf("imported interface call arguments should implement the local interface for the boxed trait object:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "let __inner: Box<dyn positioner> = (*expr.borrow().as_ref().unwrap()).clone()") {
		t.Fatalf("imported interface call argument should not use local subtrait upcast shape:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Box::new((*expr.borrow().as_ref().unwrap()).clone()) as Box<dyn positioner>") &&
		!strings.Contains(mainRS, "Box::new({ let __arg_holder = expr.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner>") {
		t.Fatalf("imported interface call argument should box the imported trait object as the local interface:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "(**self).pos()") {
		t.Fatalf("boxed imported interface impl should delegate methods through the inner trait object:\n%s", mainRS)
	}
}

func TestImportedEmbeddedInterfaceImplEmitsSupertraitImpl(t *testing.T) {
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

type Importer interface {
	Import(path string) int
}

type ImporterFrom interface {
	Importer
	ImportFrom(path string) int
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type importer struct{}

func (importer) Import(path string) int {
	return 1
}

func (importer) ImportFrom(path string) int {
	return 2
}

func NewImporter() dep.ImporterFrom {
	return importer{}
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
	if !strings.Contains(mainRS, "impl example_com_dep::Importer for importer") {
		t.Fatalf("concrete type implementing an imported embedded interface should implement the supertrait separately:\n%s", mainRS)
	}
	marker := "impl example_com_dep::ImporterFrom for importer"
	start := strings.Index(mainRS, marker)
	if start == -1 {
		t.Fatalf("missing imported subtrait impl:\n%s", mainRS)
	}
	importerFromBlock := mainRS[start:]
	if end := strings.Index(importerFromBlock, "\n\n"); end != -1 {
		importerFromBlock = importerFromBlock[:end]
	}
	if strings.Contains(importerFromBlock, "fn import(") {
		t.Fatalf("subtrait impl should not redeclare inherited methods:\n%s", importerFromBlock)
	}
	if !strings.Contains(importerFromBlock, "fn import_from(") {
		t.Fatalf("subtrait impl should include directly declared methods:\n%s", importerFromBlock)
	}
}

func TestExternalInterfaceFieldAssignmentBoxesTraitObjectForLocalInterface(t *testing.T) {
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

type Expr interface {
	Node
	exprNode()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type positioner interface {
	Pos() int
}

type operand struct {
	expr dep.Expr
}

func use(args []operand) positioner {
	var at positioner
	at = args[0].expr
	return at
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
	if strings.Contains(mainRS, ".expr.clone();") {
		t.Fatalf("imported interface field assignment should not copy the source interface handle into the local interface slot:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, ".expr.borrow().as_ref().unwrap()).clone()) as Box<dyn positioner>") {
		t.Fatalf("imported interface field assignment should box the imported trait object as the local interface:\n%s", mainRS)
	}
}

func TestExternalInterfaceConversionVariadicAnyBoxesWrappedInterface(t *testing.T) {
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

type Expr interface {
	Node
	exprNode()
}

type SelectorExpr struct{}

func (s *SelectorExpr) Pos() int { return 0 }
func (s *SelectorExpr) End() int { return 0 }
func (s *SelectorExpr) exprNode() {}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func sink(args ...any) {}

func use(expr *dep.SelectorExpr) {
	sink(dep.Expr(expr))
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
	if strings.Contains(mainRS, "let __v = (*expr.borrow().as_ref().unwrap()); let __owned = (*__v.borrow().as_ref().unwrap()).clone()") {
		t.Fatalf("interface conversion passed to variadic any should not treat the concrete pointer value as a wrapped handle:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Box::new((*expr.borrow().as_ref().unwrap()).clone()) as Box<dyn example_com_dep::Expr>") &&
		!strings.Contains(mainRS, "Box::new(example_com_dep::r#mod::SelectorExprPtr(expr.clone())) as Box<dyn example_com_dep::r#mod::Expr>") {
		t.Fatalf("interface conversion passed to variadic any should build the converted interface handle before boxing as any:\n%s", mainRS)
	}
}

func TestExternalInterfaceCallArgumentImplEmitsWithInterfaceFileOnce(t *testing.T) {
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

type Expr interface {
	Node
	exprNode()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "iface.go"), `package main

type positioner interface {
	Pos() int
}

func report(at positioner) int {
	return at.Pos()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "a.go"), `package main

import "example.com/dep"

func useA(expr dep.Expr) int {
	return report(expr)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "b.go"), `package main

import "example.com/dep"

func useB(expr dep.Expr) int {
	return report(expr)
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "iface.go"),
		filepath.Join(tempDir, "a.go"),
		filepath.Join(tempDir, "b.go"),
	})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	ifaceRS := mustReadFile(t, filepath.Join(tempDir, "iface.rs"))
	aRS := mustReadFile(t, filepath.Join(tempDir, "a.rs"))
	bRS := mustReadFile(t, filepath.Join(tempDir, "b.rs"))
	impl := "impl positioner for Box<dyn example_com_dep::Expr>"
	implMod := "impl positioner for Box<dyn example_com_dep::r#mod::Expr>"
	implCount := strings.Count(ifaceRS, impl) + strings.Count(aRS, impl) + strings.Count(bRS, impl) +
		strings.Count(ifaceRS, implMod) + strings.Count(aRS, implMod) + strings.Count(bRS, implMod)
	if implCount != 1 {
		t.Fatalf("external interface adapter should be emitted exactly once with the local interface, got %d\niface.rs:\n%s\na.rs:\n%s\nb.rs:\n%s", implCount, ifaceRS, aRS, bRS)
	}
	if !strings.Contains(ifaceRS, impl) && !strings.Contains(ifaceRS, implMod) {
		t.Fatalf("external interface adapter should be emitted in the file that defines the local interface:\n%s", ifaceRS)
	}
	if !strings.Contains(ifaceRS, "(**self).pos()") {
		t.Fatalf("boxed external interface adapter should delegate the local interface method:\n%s", ifaceRS)
	}
}

func TestExternalInterfaceSliceIndexCallArgumentBoxesElementValue(t *testing.T) {
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

type Expr interface {
	Node
	exprNode()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func end(node dep.Node) int {
	return node.End()
}

func use(rhs []dep.Expr) int {
	return end(rhs[len(rhs)-1])
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
	if strings.Contains(mainRS, "Box::new({ let __seq =") {
		t.Fatalf("indexed imported interface argument should not box the wrapped element handle:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, ".borrow().as_ref().unwrap()).clone()) as Box<dyn example_com_dep::Node>") &&
		!strings.Contains(mainRS, ".borrow().as_ref().unwrap()).clone()) as Box<dyn example_com_dep::r#mod::Node>") {
		t.Fatalf("indexed imported interface argument should unwrap and box the element value as the expected interface:\n%s", mainRS)
	}
}

func TestSelectorInterfaceStructFieldBoxesConcretePointerValue(t *testing.T) {
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

type Expr interface {
	Pos() int
}

type SelectorExpr struct{}

func (*SelectorExpr) Pos() int { return 0 }
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

type operand struct {
	expr dep.Expr
}

func record(selx *dep.SelectorExpr) operand {
	return operand{expr: selx}
}

func recordPositional(selx *dep.SelectorExpr) operand {
	return operand{selx}
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
	if strings.Contains(mainRS, "expr: selx.clone()") {
		t.Fatalf("selector-qualified interface struct field should not clone the concrete pointer handle:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "expr: Rc::new(RefCell::new(Some(Box::new((*selx.borrow().as_ref().unwrap()).clone()) as Box<dyn example_com_dep::Expr>)))") &&
		!strings.Contains(mainRS, "expr: Arc::new(Mutex::new(Some(Box::new((*selx.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn example_com_dep::Expr + Send + Sync>)))") &&
		!strings.Contains(mainRS, "expr: Rc::new(RefCell::new(Some(Box::new(example_com_dep::r#mod::SelectorExprPtr(selx.clone())) as Box<dyn example_com_dep::r#mod::Expr>)))") {
		t.Fatalf("selector-qualified interface struct field should box the concrete pointee:\n%s", mainRS)
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

func TestDefinedTypeOverImportedNamedIntegerConversionKeepsNamedValue(t *testing.T) {
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

func wrap(pos dep.Pos) positioner {
	return atPos(pos)
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
	if strings.Contains(mainRS, "atPos(Rc::new(RefCell::new(Some((*(*pos.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32") ||
		strings.Contains(mainRS, "atPos(Arc::new(Mutex::new(Some((*(*pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as i32") {
		t.Fatalf("conversion to defined type over imported named integer should keep the named value:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "atPos(Rc::new(RefCell::new(Some((*pos.borrow().as_ref().unwrap()).clone()))") &&
		!strings.Contains(mainRS, "atPos(Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()).clone()))") {
		t.Fatalf("conversion to defined type over imported named integer should wrap a cloned named value:\n%s", mainRS)
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

func TestTranspiledExternalPackageCargoIncludesGeneratedIndirectTypeDependency(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "fspkg", "fs.go"), `package fspkg

type FileMode uint32
`)
	writeTestFile(t, filepath.Join(tempDir, "oslike", "oslike.go"), `package oslike

import "example.com/mainmod/fspkg"

func WriteMode(mode fspkg.FileMode) {}
`)
	writeTestFile(t, filepath.Join(tempDir, "caller", "caller.go"), `package caller

import "example.com/mainmod/oslike"

func Nop() {}

func Use() {
	oslike.WriteMode(0666)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/mainmod/caller"

func main() {
	caller.Nop()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	callerRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_mainmod_caller", "mod.rs"))
	if !strings.Contains(callerRS, "example_com_mainmod_fspkg::FileMode") &&
		!strings.Contains(callerRS, "example_com_mainmod_fspkg::fs::FileMode") {
		t.Fatalf("caller should reference the indirect type from the callee signature, got:\n%s", callerRS)
	}

	callerCargo := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_mainmod_caller", "Cargo.toml"))
	if !strings.Contains(callerCargo, `example_com_mainmod_fspkg = { path = "../example_com_mainmod_fspkg" }`) {
		t.Fatalf("caller Cargo.toml should include generated indirect type dependency, got:\n%s", callerCargo)
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
	if !strings.Contains(mainRS, "(*example_com_dep::Public.lock().unwrap().as_ref().unwrap()).clone()") &&
		!strings.Contains(mainRS, "(*example_com_dep::Public.borrow().as_ref().unwrap()).clone()") {
		t.Fatalf("external package global selector should read the stored value, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "example_com_dep::public") {
		t.Fatalf("external package global selector should not be snake-cased, got:\n%s", mainRS)
	}
}

func TestTranspiledExternalPackageGlobalPromotedFieldUnwrapsStoredValue(t *testing.T) {
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

type Flags struct {
	AliasTypeParams bool
}

type ExperimentFlags struct {
	Flags
}

var Experiment ExperimentFlags
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/dep"

func enabled() bool {
	return dep.Experiment.AliasTypeParams
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)

	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "example_com_dep::Experiment.flags") {
		t.Fatalf("external package global promoted field should not access fields on the global handle:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "example_com_dep::Experiment.borrow().as_ref().unwrap()).flags") &&
		!strings.Contains(mainRS, "example_com_dep::Experiment.lock().unwrap().as_ref().unwrap()).flags") {
		t.Fatalf("external package global promoted field should unwrap the stored struct before field access:\n%s", mainRS)
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

func (s *Set) Store(file *File) {
	s.last.Store(file)
}

func (s *Set) Clear(file *File) bool {
	return s.last.CompareAndSwap(file, nil)
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
	if !strings.Contains(depRS, "struct GoAtomicPointer<") {
		t.Fatalf("external package should emit GoAtomicPointer helper, got:\n%s", depRS)
	}
	for _, forbidden := range []string{
		"fn load(&self) -> Arc<Mutex<Option<T>>>",
		"fn store(&self, value: Arc<Mutex<Option<T>>>)",
		"fn compare_and_swap(&self, old: Arc<Mutex<Option<T>>>, new: Arc<Mutex<Option<T>>>) -> bool",
	} {
		if strings.Contains(depRS, forbidden) {
			t.Fatalf("atomic.Pointer helper should use GoPtr handles, found %q:\n%s", forbidden, depRS)
		}
	}
	for _, want := range []string{
		"fn load(&self) -> GoPtr<T>",
		"fn store(&self, value: GoPtr<T>)",
		"fn compare_and_swap(&self, old: GoPtr<T>, new: GoPtr<T>) -> bool",
	} {
		if !strings.Contains(depRS, want) {
			t.Fatalf("atomic.Pointer helper should include %q:\n%s", want, depRS)
		}
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

func TestSharedStdlibStubVariableReturningRWMutexIncludesHelper(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22

require example.com/lockdep v0.0.0

replace example.com/lockdep => ./lockdep
`)
	writeTestFile(t, filepath.Join(tempDir, "lockdep", "go.mod"), `module example.com/lockdep

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "lockdep", "lockdep.go"), `package lockdep

import "syscall"

func Lock() {
	syscall.ForkLock.RLock()
	syscall.ForkLock.RUnlock()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/lockdep"

func main() {
	lockdep.Lock()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	stubsRS := mustReadFile(t, filepath.Join(tempDir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	if !strings.Contains(stubsRS, "pub struct GoRWMutex") {
		t.Fatalf("shared stdlib stubs should export GoRWMutex for RWMutex-returning vars, got:\n%s", stubsRS)
	}
	if !strings.Contains(stubsRS, "pub fn ForkLock() -> Rc<RefCell<Option<GoRWMutex>>>") &&
		!strings.Contains(stubsRS, "pub fn ForkLock() -> Arc<Mutex<Option<GoRWMutex>>>") {
		t.Fatalf("shared stdlib stubs should preserve the RWMutex variable type, got:\n%s", stubsRS)
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
	if !strings.Contains(mainRS, "&dyn example_com_dep::Mapper") &&
		!strings.Contains(mainRS, "&dyn example_com_dep::r#mod::Mapper") {
		t.Fatalf("function type alias should use a trait object for imported interface parameters, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "Option<Box<dyn example_com_dep::Mapper") ||
		strings.Contains(mainRS, "Option<Box<dyn example_com_dep::r#mod::Mapper") {
		t.Fatalf("function type alias should not wrap imported interface parameters, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "Option<example_com_dep::Mapper>") ||
		strings.Contains(mainRS, "Option<example_com_dep::r#mod::Mapper>") {
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
	if !strings.Contains(mainRS, "pub node: Rc<RefCell<Option<Box<dyn example_com_dep::Node>>>>") &&
		!strings.Contains(mainRS, "pub node: Rc<RefCell<Option<Box<dyn example_com_dep::r#mod::Node>>>>") {
		t.Fatalf("imported interface field should use a boxed trait object, got:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "Option<example_com_dep::Node>") ||
		strings.Contains(mainRS, "Option<example_com_dep::r#mod::Node>") {
		t.Fatalf("imported interface field should not use a bare trait name as a type, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Vec<Rc<RefCell<Option<Box<dyn example_com_dep::Node>>>>>") &&
		!strings.Contains(mainRS, "Vec<Rc<RefCell<Option<Box<dyn example_com_dep::r#mod::Node>>>>>") {
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
	if !strings.Contains(mainRS, "Box::new((*ident.borrow().as_ref().unwrap()).clone()) as Box<dyn example_com_dep::Node>") &&
		!strings.Contains(mainRS, "Box::new((*ident.borrow().as_ref().unwrap()).clone()) as Box<dyn example_com_dep::r#mod::Node>") {
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
	if !strings.Contains(mainRS, "Box<dyn FnMut(Rc<RefCell<Option<Event>>>, &dyn example_com_dep::Mapper)") &&
		!strings.Contains(mainRS, "Box<dyn FnMut(Rc<RefCell<Option<Event>>>, &dyn example_com_dep::r#mod::Mapper)") {
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

func TestImportedInterfaceImplUsesSiblingFileMethods(t *testing.T) {
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
`)
	writeTestFile(t, filepath.Join(tempDir, "methods.go"), `package main

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
		filepath.Join(tempDir, "methods.go"),
		filepath.Join(tempDir, "export.go"),
		filepath.Join(tempDir, "main.go"),
	})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	eventRS := mustReadFile(t, filepath.Join(tempDir, "event.rs"))
	if !strings.Contains(eventRS, "impl example_com_dep::Mapper for Event") {
		t.Fatalf("imported interface impl should use methods declared in sibling files, got:\n%s", eventRS)
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
	if !strings.Contains(manifestRS, "kind: Rc::new(RefCell::new(Some(Kind(") &&
		!strings.Contains(manifestRS, "kind: Rc::new(RefCell::new(Some(crate::defs::Kind(") {
		t.Fatalf("typed constant field should convert through Kind, got:\n%s", manifestRS)
	}
	if !strings.Contains(manifestRS, "version: Rc::new(RefCell::new(Some(Version(") &&
		!strings.Contains(manifestRS, "version: Rc::new(RefCell::new(Some(crate::defs::Version(") {
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
	if strings.Contains(depLib, `include!("__go2rust_helpers.rs");`) {
		t.Fatalf("external package should use shared GoTime instead of a package-scoped helper include, got:\n%s", depLib)
	}
	if !strings.Contains(depLib, "pub use go2rust_stdlib_stubs::*;") {
		t.Fatalf("external package should re-export shared stdlib helpers, got:\n%s", depLib)
	}
	sharedLib := mustReadFile(t, filepath.Join(tempDir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	if !strings.Contains(sharedLib, "pub struct GoTime") {
		t.Fatalf("shared stdlib helper crate should define exported GoTime once, got:\n%s", sharedLib)
	}

	clockRS := mustReadFile(t, filepath.Join(depDir, "clock.rs"))
	consumeRS := mustReadFile(t, filepath.Join(depDir, "consume.rs"))
	for name, code := range map[string]string{"clock.rs": clockRS, "consume.rs": consumeRS} {
		if !strings.Contains(code, "use go2rust_stdlib_stubs::*;") {
			t.Fatalf("%s should import shared stdlib helpers, got:\n%s", name, code)
		}
		if strings.Contains(code, "use crate::*;") {
			t.Fatalf("%s should not glob-import the crate root for helpers, got:\n%s", name, code)
		}
		if strings.Contains(code, "struct GoTime") {
			t.Fatalf("%s should not define a file-local GoTime, got:\n%s", name, code)
		}
	}
}

func TestSharedStdlibHelperGoTimeCrossesTranspiledCrates(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "clock", "clock.go"), `package clock

import "time"

func Made() time.Time {
	return time.Unix(1, 0)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "consume", "consume.go"), `package consume

import "time"

func UnixSecond(t time.Time) int64 {
	return t.Unix()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import (
	"example.com/mainmod/clock"
	"example.com/mainmod/consume"
)

func main() {
	println(consume.UnixSecond(clock.Made()))
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	sharedLib := mustReadFile(t, filepath.Join(tempDir, "vendor", sharedStdlibStubCrateName, "lib.rs"))
	if !strings.Contains(sharedLib, "pub struct GoTime") {
		t.Fatalf("shared stdlib helper crate should define exported GoTime once, got:\n%s", sharedLib)
	}

	for _, importPath := range []string{"example.com/mainmod/clock", "example.com/mainmod/consume"} {
		crateName := RustCrateNameForGoImportPath(importPath)
		libRS := mustReadFile(t, filepath.Join(tempDir, "vendor", crateName, "lib.rs"))
		if strings.Contains(libRS, `include!("__go2rust_helpers.rs");`) {
			t.Fatalf("%s should use shared GoTime instead of a package-local helper include, got:\n%s", crateName, libRS)
		}
		if !strings.Contains(libRS, "pub use go2rust_stdlib_stubs::*;") {
			t.Fatalf("%s should re-export shared stdlib helpers, got:\n%s", crateName, libRS)
		}
		entries, err := os.ReadDir(filepath.Join(tempDir, "vendor", crateName))
		if err != nil {
			t.Fatalf("ReadDir(%s) error = %v", crateName, err)
		}
		for _, entry := range entries {
			if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".rs") {
				continue
			}
			code := mustReadFile(t, filepath.Join(tempDir, "vendor", crateName, entry.Name()))
			if strings.Contains(code, "struct GoTime") {
				t.Fatalf("%s/%s should not define a crate-local GoTime, got:\n%s", crateName, entry.Name(), code)
			}
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

func TestStdGoFileModuleDoesNotShadowRustStd(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/stdfile

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "std.go"), `package stdfile

const value = 1
`)
	writeTestFile(t, filepath.Join(tempDir, "use.go"), `package stdfile

func Use() int {
	return value
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "std.go"),
		filepath.Join(tempDir, "use.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	libRS := mustReadFile(t, filepath.Join(tempDir, "lib.rs"))
	if strings.Contains(libRS, "pub mod std;") || strings.Contains(libRS, "pub use std::*;") {
		t.Fatalf("std.go should not generate a crate-root module named std:\n%s", libRS)
	}
	if !strings.Contains(libRS, "pub mod std_;") || !strings.Contains(libRS, "pub use std_::*;") {
		t.Fatalf("std.go should generate a non-shadowing std_ module:\n%s", libRS)
	}
	if _, err := os.Stat(filepath.Join(tempDir, "std_.rs")); err != nil {
		t.Fatalf("std.go should write std_.rs: %v", err)
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
	wantInitializers := []string{
		"Some(objset(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new())))))",
		"Some(crate::defs::objset(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new())))))",
	}
	foundInitializer := false
	for _, want := range wantInitializers {
		if strings.Contains(useRS, want) {
			foundInitializer = true
			break
		}
	}
	if !foundInitializer {
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
	if !strings.Contains(defsRS, "self.b()") && !strings.Contains(defsRS, "item::b(self)") {
		t.Fatalf("interface impl should delegate to the inherent sibling-file method:\n%s", defsRS)
	}
}

func TestCrossFileInterfaceImplEmitsWhenDeclarationFileHasNoMethods(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "defs.go"), `package main

type Node interface {
	A() int
}

type item struct{}

func Box(v *item) Node {
	return v
}
`)
	writeTestFile(t, filepath.Join(tempDir, "methods.go"), `package main

func (i *item) A() int {
	return 1
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
		t.Fatalf("interface impl should be emitted with the type declaration even when methods are in sibling files:\n%s", defsRS)
	}
	if !strings.Contains(defsRS, "self.a()") && !strings.Contains(defsRS, "item::a(self)") {
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

func TestConcurrentComparableStructPointerFieldUsesPointerIdentity(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type scope struct {
	values []int
}

type key struct {
	scope *scope
	name  string
}

var seen map[key]int

func start() {
	go func() {}()
}

func lookup(s *scope) int {
	return seen[key{scope: s, name: "x"}]
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "self.scope.lock().unwrap().as_ref().cloned()") {
		t.Fatalf("pointer field ordering should not clone or compare pointee values, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Arc::ptr_eq(&self.scope, &other.scope)") {
		t.Fatalf("pointer field equality should compare pointer handles by identity, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "Arc::as_ptr(&self.scope)") || !strings.Contains(mainRS, "Arc::as_ptr(&other.scope)") {
		t.Fatalf("pointer field ordering should order pointer handles by address, got:\n%s", mainRS)
	}
}

func TestExternalComparableStructUsedByEqualityGetsPartialEq(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "dep", "dep.go"), `package dep

type Type interface {
	String() string
}

type Pointer struct {
	Base Type
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/mainmod/dep"

func used(p *dep.Pointer) bool {
	go func() {}()
	return *p != (dep.Pointer{})
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	depRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_mainmod_dep", "mod.rs"))
	if !strings.Contains(depRS, "impl PartialEq for Pointer") {
		t.Fatalf("external comparable struct used by equality should get PartialEq in its defining crate, got:\n%s", depRS)
	}
	if !strings.Contains(depRS, "__left.as_ref().__go_eq_type_(__right.as_ref())") {
		t.Fatalf("external comparable struct interface field should use interface equality helper, got:\n%s", depRS)
	}
}

func TestConcurrentStructDisplayAnySliceUsesAnyFormatter(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type desc struct {
	args []any
}

func start() {
	go func() {}()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "format_slice(&self.args)") {
		t.Fatalf("[]any struct display should not use Display-bound slice formatter, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "format_any_slice(&self.args)") {
		t.Fatalf("[]any struct display should use format_any_slice, got:\n%s", mainRS)
	}
}

func TestConcurrentNamedTypeOverNamedScalarGlobalZeroPreservesInnerName(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type Pos int
type atPos Pos

var p atPos

func start() {
	go func() {}()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "atPos(Arc::new(Mutex::new(Some(0))))") {
		t.Fatalf("named type over named scalar zero value should not erase inner named type, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "atPos(Arc::new(Mutex::new(Some(Pos(") {
		t.Fatalf("named type over named scalar zero value should wrap the inner named zero value, got:\n%s", mainRS)
	}
}

func TestConcurrentPointerToErrorAssignmentStoresErrorValue(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type holder struct {
	firstErr error
}

func (h *holder) handle(err *error) {
	*err = h.firstErr
}

func start() {
	go func() {}()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "let new_val = self.first_err.clone(); *err.lock().unwrap() = Some(new_val);") {
		t.Fatalf("*error assignment should store the error option value, not the handle, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "__err_guard.take()") {
		t.Fatalf("*error assignment should extract an error option from the RHS handle, got:\n%s", mainRS)
	}
}

func TestConcurrentAddressOfErrorPassesErrorHandleToPointerParam(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type holder struct{}

func (h *holder) handle(err *error) {}

func (h *holder) run() (err error) {
	h.handle(&err)
	return
}

func start() {
	go func() {}()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "Option<Option<Box<dyn StdError") {
		t.Fatalf("*error should use the same error handle shape as an error variable, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "pub fn handle(&self, err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>)") {
		t.Fatalf("*error parameter should accept the error handle shape, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "self.handle(err.clone())") {
		t.Fatalf("&error argument should pass the existing error handle, got:\n%s", mainRS)
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

func TestConcurrentNumericTypeDefinitionComparisonCopiesBeforeComparing(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type Code uint8

func start() {
	go func() {}()
}

func same(c Code) bool {
	return c == c
}

func ordered(c Code) bool {
	return c <= c
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "self.0.lock().unwrap().as_ref().unwrap() == other.0.lock().unwrap().as_ref().unwrap()") {
		t.Fatalf("named scalar PartialEq should not hold self while locking other:\n%s", mainRS)
	}
	if strings.Contains(mainRS, "self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other.0.lock().unwrap().as_ref().unwrap())") {
		t.Fatalf("named scalar PartialOrd should not hold self while locking other:\n%s", mainRS)
	}
	for _, want := range []string{
		"let __left = { self.0.lock().unwrap().as_ref().cloned() };",
		"let __right = { other.0.lock().unwrap().as_ref().cloned() };",
		"__left == __right",
		"__left.partial_cmp(&__right)",
	} {
		if !strings.Contains(mainRS, want) {
			t.Fatalf("named scalar comparison should copy values before comparing, missing %q:\n%s", want, mainRS)
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
	if !strings.Contains(roundRS, "let mut lsb = Arc::new(Mutex::new(Some(Word(") &&
		!strings.Contains(roundRS, "let mut lsb = Arc::new(Mutex::new(Some(crate::arith::Word(") {
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

func TestCrossFileGoPtrFieldAssignmentPromotesLocal(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "types.go"), `package main

import "unsafe"

type node struct{}

type list struct {
	first *node
}

func raw(n *node) *node {
	return (*node)(unsafe.Pointer(n))
}

func fill(l *list, n *node) {
	l.first = raw(n)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "use.go"), `package main

func ordinary(n *node) *node {
	return n
}

func use(l *list, n *node, flag bool) *node {
	var p *node
	p = l.first
	if flag {
		p = ordinary(n)
	}
	return p
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "use.go"),
		filepath.Join(tempDir, "types.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	typesRS := mustReadFile(t, filepath.Join(tempDir, "types.rs"))
	if !strings.Contains(typesRS, "pub first: GoPtr<node>") {
		t.Fatalf("cross-file field assigned a GoPtr value should use GoPtr storage:\n%s", typesRS)
	}
	useRS := mustReadFile(t, filepath.Join(tempDir, "use.rs"))
	if !strings.Contains(useRS, "let mut p: GoPtr<") || !strings.Contains(useRS, "node> = GoPtr::nil();") {
		t.Fatalf("cross-file local assigned from a GoPtr field should use GoPtr storage:\n%s", useRS)
	}
	if strings.Contains(useRS, "let mut p: Rc<RefCell<Option<node>>>") ||
		strings.Contains(useRS, "let mut p: Arc<Mutex<Option<node>>>") {
		t.Fatalf("cross-file local assigned from a GoPtr field should not keep ordinary pointer wrapper storage:\n%s", useRS)
	}
	if strings.Contains(useRS, "p = GoPtr::local((*l") || strings.Contains(useRS, "p = GoPtr::local({ let __field") {
		t.Fatalf("cross-file local assignment from a generated GoPtr field should clone the field handle, not rewrap it:\n%s", useRS)
	}
	if !strings.Contains(useRS, ".first.clone(); p = new_val;") && !strings.Contains(useRS, "p = (*l.lock().unwrap().as_ref().unwrap()).first.clone();") {
		t.Fatalf("cross-file local assignment from a generated GoPtr field should use the field handle directly:\n%s", useRS)
	}
}

func TestCrossFileGoPtrMethodReturnFactPropagatesToCaller(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "heap.go"), `package main

type node struct{}

type heap struct {
	current *node
}

func initHeap(h *heap, items []node) {
	h.current = &items[0]
}

func (h *heap) alloc() *node {
	p := h.current
	return p
}
`)
	writeTestFile(t, filepath.Join(tempDir, "use.go"), `package main

func use(h *heap, items []node) *node {
	var p *node
	p = h.current
	p = h.alloc()
	return p
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "use.go"),
		filepath.Join(tempDir, "heap.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	heapRS := mustReadFile(t, filepath.Join(tempDir, "heap.rs"))
	if !strings.Contains(heapRS, "pub current: GoPtr<node>") {
		t.Fatalf("field assigned a slice element pointer should use GoPtr storage:\n%s", heapRS)
	}
	if !strings.Contains(heapRS, "pub fn alloc(&self") || !strings.Contains(heapRS, " -> GoPtr<node>") {
		t.Fatalf("cross-file method returning a GoPtr field local should emit a GoPtr result:\n%s", heapRS)
	}
	useRS := mustReadFile(t, filepath.Join(tempDir, "use.rs"))
	if !strings.Contains(useRS, "let mut p: GoPtr<") || !strings.Contains(useRS, "node> = GoPtr::nil();") {
		t.Fatalf("caller local assigned from a GoPtr field should use GoPtr storage:\n%s", useRS)
	}
	if strings.Contains(useRS, "p = (*h.alloc(") || strings.Contains(useRS, "p = GoPtr::local(h.alloc(") {
		t.Fatalf("caller assignment from a GoPtr-returning method should not rewrap the returned handle:\n%s", useRS)
	}
	if !strings.Contains(useRS, ".alloc();") {
		t.Fatalf("caller assignment should store the GoPtr-returning method handle directly:\n%s", useRS)
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
		"alpha::__go_zero_globals();",
		"beta::__go_zero_globals();",
		"gamma::__go_zero_globals();",
		"alpha::__go_init_order_0();",
		"beta::__go_init_order_1();",
		"gamma::__go_init_order_2();",
	} {
		if !strings.Contains(mainRS, want) {
			t.Fatalf("main should call package-wide init helper %q, got:\n%s", want, mainRS)
		}
	}
	if strings.Contains(mainRS, "alpha::__go_init_all();") || strings.Contains(mainRS, "beta::__go_init_all();") || strings.Contains(mainRS, "gamma::__go_init_all();") ||
		strings.Contains(mainRS, "alpha::__go_init_all_alpha();") || strings.Contains(mainRS, "beta::__go_init_all_beta();") || strings.Contains(mainRS, "gamma::__go_init_all_gamma();") {
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

func TestCrossFilePackageGlobalInitUsesGoTypesInitOrder(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "aaa.go"), `package main

var A = 1
var C = B + 1
`)
	writeTestFile(t, filepath.Join(tempDir, "bbb.go"), `package main

var B = A + 1
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

func main() {
	println(A, B, C)
}
`)

	generator := NewProjectGenerator([]string{
		filepath.Join(tempDir, "aaa.go"),
		filepath.Join(tempDir, "bbb.go"),
		filepath.Join(tempDir, "main.go"),
	})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	wants := []string{
		"aaa::__go_zero_globals();",
		"bbb::__go_zero_globals();",
		"aaa::__go_init_order_0();",
		"bbb::__go_init_order_1();",
		"aaa::__go_init_order_2();",
	}
	last := -1
	for _, want := range wants {
		idx := strings.Index(mainRS, want)
		if idx < 0 {
			t.Fatalf("package init order missing %q, got:\n%s", want, mainRS)
		}
		if idx <= last {
			t.Fatalf("package init order put %q out of order, got:\n%s", want, mainRS)
		}
		last = idx
	}
	if strings.Contains(mainRS, "aaa::__go_init_all_aaa();") || strings.Contains(mainRS, "bbb::__go_init_all_bbb();") {
		t.Fatalf("package-wide init should not call module-at-a-time global initializers, got:\n%s", mainRS)
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
		{patterns: "go/types+deps", path: "go/types", want: true},
		{patterns: "go/token", path: "go/types", want: false},
		{patterns: "go/types+deps", path: "go/token", want: false},
	} {
		got := sourceStdlibPackagePatternMatches(tt.path, tt.patterns)
		if got != tt.want {
			t.Fatalf("sourceStdlibPackagePatternMatches(%q, %q) = %v, want %v", tt.path, tt.patterns, got, tt.want)
		}
	}
	for _, tt := range []struct {
		patterns string
		path     string
		want     bool
	}{
		{patterns: "go/types+deps", path: "go/types", want: true},
		{patterns: "go/types", path: "go/types", want: false},
		{patterns: "go/types+deps", path: "go/token", want: false},
		{patterns: "go/...+deps", path: "go/token", want: true},
	} {
		got := sourceStdlibPackagePatternExpandsDeps(tt.path, tt.patterns)
		if got != tt.want {
			t.Fatalf("sourceStdlibPackagePatternExpandsDeps(%q, %q) = %v, want %v", tt.path, tt.patterns, got, tt.want)
		}
	}
}

func TestRetiredFilepathBridgeSourceMapsByDefault(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "")

	for _, path := range []string{
		"path/filepath",
		"internal/filepathlite",
		"internal/stringslite",
	} {
		if !shouldTranspileStdlibPackage(path) {
			t.Fatalf("%s should source-map by default after its pure bridge shims retire", path)
		}
		if sourceStdlibPackagePatternExpandsDeps(path, "") {
			t.Fatalf("%s should not expand all transitive stdlib deps by default", path)
		}
	}
	if shouldTranspileStdlibPackage("os") {
		t.Fatalf("OS-tied filepath dependencies should not be source-mapped by the filepath default")
	}
}

func TestExternalNamedIntegerConversionSourceMapsTokenExplicitly(t *testing.T) {
	path := filepath.Join("..", "tests", "external_named_integer_conversion", ".go2rust.toml")
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("read %s: %v", path, err)
	}
	if !strings.Contains(string(data), `source_stdlib_packages = "go/token"`) {
		t.Fatalf("%s must source-map go/token explicitly so the fixture cannot regenerate through token_Pos stubs", path)
	}
}

func TestPackageLoaderSourceStdlibDepsPatternIncludesTransitiveStdlibImports(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "go/types+deps")
	tokenPkg := &packages.Package{Name: "token", PkgPath: "go/token", Imports: make(map[string]*packages.Package)}
	unsafePkg := &packages.Package{Name: "unsafe", PkgPath: "unsafe", Imports: make(map[string]*packages.Package)}
	typesPkg := &packages.Package{Name: "types", PkgPath: "go/types", Imports: map[string]*packages.Package{
		"go/token": tokenPkg,
		"unsafe":   unsafePkg,
	}}
	mainPkg := &packages.Package{Name: "main", PkgPath: "main", Imports: map[string]*packages.Package{
		"go/types": typesPkg,
	}}
	loader := NewPackageLoader(t.TempDir())
	loader.mainPkg = mainPkg

	loader.collectAllPackages(mainPkg)

	for _, path := range []string{"go/types", "go/token"} {
		if loader.allPackages[path] == nil {
			t.Fatalf("collectAllPackages() should include source stdlib dependency %s", path)
		}
		if !loader.sourceStdlibPackages[path] {
			t.Fatalf("collectAllPackages() should mark %s as source stdlib", path)
		}
		if loader.packageMapping[path] == "" {
			t.Fatalf("collectAllPackages() should map %s to a Rust crate", path)
		}
	}
	if loader.allPackages["unsafe"] != nil || loader.packageMapping["unsafe"] != "" || loader.sourceStdlibPackages["unsafe"] {
		t.Fatalf("collectAllPackages() should keep unsafe on the compiler-intrinsic path, not source-map it")
	}
}

func TestUnsafeStdlibPackageIsCompilerIntrinsic(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "unsafe,all")
	if shouldTranspileStdlibPackage("unsafe") {
		t.Fatalf("unsafe should not be source-transpiled as a normal stdlib package")
	}
	if (&PackageLoader{sourceStdlibPackages: map[string]bool{"unsafe": true}}).isSourceStdlibPackage("unsafe") {
		t.Fatalf("unsafe should not be treated as a source stdlib package from loader state")
	}
}

func TestPackageLoaderSourceStdlibExactPatternDoesNotIncludeTransitiveStdlibImports(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "go/types")
	tokenPkg := &packages.Package{Name: "token", PkgPath: "go/token", Imports: make(map[string]*packages.Package)}
	typesPkg := &packages.Package{Name: "types", PkgPath: "go/types", Imports: map[string]*packages.Package{
		"go/token": tokenPkg,
	}}
	mainPkg := &packages.Package{Name: "main", PkgPath: "main", Imports: map[string]*packages.Package{
		"go/types": typesPkg,
	}}
	loader := NewPackageLoader(t.TempDir())
	loader.mainPkg = mainPkg

	loader.collectAllPackages(mainPkg)

	if loader.allPackages["go/types"] == nil {
		t.Fatalf("collectAllPackages() should include directly selected go/types")
	}
	if loader.allPackages["go/token"] != nil {
		t.Fatalf("collectAllPackages() should not include transitive stdlib dependency without +deps")
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

func TestSourceStdlibReachabilityAppliesDuringMainGeneration(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "errors")
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/source-stdlib-main-reachability

go 1.24
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "errors"

func sink(v any) {}

func main() {
	err := errors.New("x")
	sink(err)
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if strings.Contains(mainRS, "joinError") {
		t.Fatalf("main-package error-to-any lowering should not reference pruned source-stdlib error candidates:\n%s", mainRS)
	}
}

func TestSourceStdlibRerunRemovesStaleGeneratedVendorCrates(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "go/token")
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/source-stdlib-stale

go 1.24
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "go/token"

func main() {
	var pos token.Pos = token.NoPos
	println(int(pos))
}
`)

	staleCrateDir := filepath.Join(tempDir, "vendor", "runtime")
	writeTestFile(t, filepath.Join(staleCrateDir, "Cargo.toml"), `[package]
name = "runtime"
version = "0.1.0"
edition = "2021"

[lib]
name = "runtime"
path = "lib.rs"

[dependencies]
go2rust_stdlib_stubs = { path = "../go2rust_stdlib_stubs" }
`)
	writeTestFile(t, filepath.Join(staleCrateDir, "lib.rs"), `pub use go2rust_stdlib_stubs::*;
`)
	writeTestFile(t, filepath.Join(tempDir, "vendor", "go_token", "stale.rs"), `pub fn stale() {}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	if _, err := os.Stat(staleCrateDir); !os.IsNotExist(err) {
		t.Fatalf("stale generated crate should be removed, stat err = %v", err)
	}
	if _, err := os.Stat(filepath.Join(tempDir, "vendor", "go_token", "stale.rs")); !os.IsNotExist(err) {
		t.Fatalf("stale file inside regenerated source stdlib crate should be removed, stat err = %v", err)
	}
	if _, err := os.Stat(filepath.Join(tempDir, "vendor", "go_token", "lib.rs")); err != nil {
		t.Fatalf("current source stdlib crate should still be generated: %v", err)
	}
}

func TestSourceStdlibSyncPoolRaceAddrGeneratedIndexScalarIsBare(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "sync,internal/sync")
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/source-stdlib-sync

go 1.24
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "sync"

func main() {
	var m sync.Map
	m.Store("key", "value")
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	poolRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "sync", "pool.rs"))
	if strings.Contains(poolRS, "].clone() }.borrow()") ||
		strings.Contains(poolRS, "].clone() }.lock()") {
		snippet := poolRS
		if start := strings.Index(poolRS, "pub fn pool_race_addr"); start >= 0 {
			end := start + 1200
			if end > len(poolRS) {
				end = len(poolRS)
			}
			snippet = poolRS[start:end]
		}
		t.Fatalf("source stdlib poolRaceAddr uintptr index should stay bare, got:\n%s", snippet)
	}
}

func TestSourceStdlibRuntimeMspanInitWrapsPackageConstArgument(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "go/types+deps")
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/source-stdlib-runtime

go 1.24
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import (
	"go/token"
	"go/types"
)

func main() {
	name := types.NewTypeName(token.NoPos, nil, "T", nil)
	_ = types.NewTypeParam(name, nil)
}
`)

	loader := NewPackageLoader(tempDir)
	if err := loader.LoadWithDependencies([]string{"."}); err != nil {
		t.Fatalf("LoadWithDependencies() error = %v", err)
	}
	runtimePkg := loader.allPackages["runtime"]
	if runtimePkg == nil {
		t.Fatalf("go/types+deps should source-load runtime; loaded packages: %v", loader.orderedAllPackagePaths())
	}
	typeInfo := &TypeInfo{info: runtimePkg.TypesInfo, pkg: runtimePkg.Types}
	packageState := NewPackageState()
	packageState.ConstantNameOverrides = assignPackageConstantNames(runtimePkg.Syntax)
	packageState.MethodsByType = collectPackageMethods(runtimePkg.Syntax)
	session := NewTranspileSession(typeInfo, loader.packageMapping)
	prevContext := GetTranspileContext()
	prevTypeInfo := GetTypeInfo()
	prevConcurrencyDetector := globalConcurrencyDetector
	cd := NewConcurrencyDetector()
	cd.AnalyzeProject(runtimePkg.Syntax)
	SetConcurrencyDetector(cd)
	SetTranspileContext(&TranspileContext{
		Session:                 session,
		Package:                 packageState,
		PackageMapping:          loader.packageMapping,
		UsePackageExternalStubs: true,
	})
	SetTypeInfo(typeInfo)
	t.Cleanup(func() {
		SetConcurrencyDetector(prevConcurrencyDetector)
		SetTranspileContext(prevContext)
		SetTypeInfo(prevTypeInfo)
	})

	var targetCall *ast.CallExpr
	for i, file := range runtimePkg.Syntax {
		if filepath.Base(packageFileName(runtimePkg, i)) != "arena.go" {
			continue
		}
		ast.Inspect(file, func(node ast.Node) bool {
			call, ok := node.(*ast.CallExpr)
			if !ok || len(call.Args) != 2 {
				return true
			}
			sel, ok := call.Fun.(*ast.SelectorExpr)
			if !ok || sel.Sel.Name != "init" {
				return true
			}
			arg, ok := call.Args[1].(*ast.Ident)
			if !ok || arg.Name != "userArenaChunkPages" {
				return true
			}
			targetCall = call
			return false
		})
		break
	}
	if targetCall == nil {
		t.Fatal("could not find runtime arena.go s.init(..., userArenaChunkPages) call")
	}
	sel := targetCall.Fun.(*ast.SelectorExpr)
	if IsExternalStdlibSelectorMethod(sel) {
		t.Fatalf("source-mapped runtime method should not use external stub method handling")
	}
	if methodCallUsesBareArguments(sel) {
		t.Fatalf("runtime mspan.init should use generated wrapped arguments, not bare helper arguments")
	}
	arg := targetCall.Args[1].(*ast.Ident)
	if !isConstIdent(arg) {
		t.Fatalf("userArenaChunkPages should be recognized as a package constant")
	}
	if expected := selectedMethodParamType(sel, 1); expected == nil || expected.String() != "uintptr" {
		t.Fatalf("go/types should identify mspan.init npages as uintptr, got %v", expected)
	}
	var out strings.Builder
	writeRegularMethodCallArgument(&out, sel, targetCall, arg, 1)
	got := out.String()
	if strings.Contains(got, "USER_ARENA_CHUNK_PAGES.clone()") {
		t.Fatalf("source-mapped package const should not be cloned as a wrapped value: %s", got)
	}
	if !strings.Contains(got, "Arc::new(Mutex::new(Some(USER_ARENA_CHUNK_PAGES as usize)))") {
		t.Fatalf("source-mapped package const should be wrapped for the generated method parameter: %s", got)
	}
}

func TestMappedImportedTypeUsesDeclaringModulePath(t *testing.T) {
	fset := token.NewFileSet()
	mutexPkg := parsePackageForReachabilityTest(t, fset, "example.com/internal/sync", "mutex.go", `package sync

type Mutex struct{}
`)
	syncPkg := parsePackageForReachabilityTest(t, fset, "example.com/sync", "sync.go", `package sync

import isync "example.com/internal/sync"

type Holder struct {
	mu isync.Mutex
}
`)
	loader := &PackageLoader{
		fileSet: fset,
		mainPkg: syncPkg,
		allPackages: map[string]*packages.Package{
			"example.com/internal/sync": mutexPkg,
			"example.com/sync":          syncPkg,
		},
		packageMapping: map[string]string{"example.com/internal/sync": "internal_sync"},
	}
	if err := loader.typeCheckLocalPackage(mutexPkg, loader.projectImporter()); err != nil {
		t.Fatalf("typeCheckLocalPackage(example.com/internal/sync) error = %v", err)
	}
	if err := loader.typeCheckLocalPackage(syncPkg, loader.projectImporter()); err != nil {
		t.Fatalf("typeCheckLocalPackage(example.com/sync) error = %v", err)
	}

	typeInfo := &TypeInfo{info: syncPkg.TypesInfo, pkg: syncPkg.Types}
	session := NewTranspileSession(typeInfo, loader.packageMapping)
	session.PackageTypeModuleNames = map[string]map[string]string{
		"example.com/internal/sync": {"Mutex": "mutex"},
	}
	prevCtx := GetTranspileContext()
	SetTranspileContext(&TranspileContext{
		Session:        session,
		Package:        NewPackageState(),
		PackageMapping: loader.packageMapping,
	})
	defer SetTranspileContext(prevCtx)

	rust, _, _ := TranspileWithMapping(syncPkg.Syntax[0], fset, typeInfo, loader.packageMapping)
	if strings.Contains(rust, "internal_sync::Mutex") {
		t.Fatalf("mapped imported type should not use crate-root path that can be shadowed:\n%s", rust)
	}
	if !strings.Contains(rust, "internal_sync::mutex::Mutex") {
		t.Fatalf("mapped imported type should use declaring module path:\n%s", rust)
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
	if !strings.Contains(exprRS, "let __any = __v.__go_as_any();") {
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

func TestSourceMappedPointerWrapperImplementsLocalInterface(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "types.go", `package types

import (
	"go/ast"
	"go/token"
)

type positioner interface {
	Pos() token.Pos
}

func report(call *ast.CallExpr) positioner {
	return call
}

func use(call *ast.CallExpr) positioner {
	return report(call)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(types.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"go/ast":   "go_ast",
		"go/token": "go_token",
	})
	if strings.Contains(rust, "impl positioner for go_ast::CallExpr {") {
		t.Fatalf("source-mapped pointer return should not implement the local interface for the pointee value:\n%s", rust)
	}
	if !strings.Contains(rust, "impl positioner for go_ast::CallExprPtr") {
		t.Fatalf("source-mapped pointer return should implement the local interface for the pointer wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "go_ast::CallExpr::pos(__recv)") {
		t.Fatalf("source-mapped pointer local-interface impl should delegate through the pointee:\n%s", rust)
	}
	if strings.Contains(rust, "Box::new((*call.") {
		t.Fatalf("source-mapped pointer local-interface argument should not box the cloned pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(go_ast::CallExprPtr(call.clone())) as Box<dyn positioner") {
		t.Fatalf("source-mapped pointer local-interface argument should box the pointer wrapper:\n%s", rust)
	}
}

func TestSourceMappedPointerReturnBoxesWrapperForLocalInterface(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "typesinternal.go", `package typesinternal

import "go/types"

type NamedOrAlias interface {
	types.Type
	Obj() *types.TypeName
	TypeArgs() *types.TypeList
	TypeParams() *types.TypeParamList
	SetTypeParams(tparams []*types.TypeParam)
}

func aliasOrigin(alias *types.Alias) *types.Alias {
	return alias.Origin()
}

func fromAliasCall(alias *types.Alias) NamedOrAlias {
	return aliasOrigin(alias)
}

func fromNamedMethod(named *types.Named) NamedOrAlias {
	return named.Origin()
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(typesinternal.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/types": "go_types"})
	for _, bad := range []string{
		"Box::new((*alias_origin(alias.clone())",
		"Box::new((*{ let __recv = named.clone()",
	} {
		if strings.Contains(rust, bad) {
			t.Fatalf("source-mapped pointer return should not box the cloned pointee, found %q:\n%s", bad, rust)
		}
	}
	for _, want := range []string{
		"Box::new(go_types::AliasPtr(",
		"Box::new(go_types::NamedPtr(",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("source-mapped pointer return should box the pointer wrapper, missing %q:\n%s", want, rust)
		}
	}
}

func TestSourceMappedPointerAssertionCandidatesUseWrappers(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "objectpath.go", `package objectpath

import "go/types"

type hasElem interface {
	Elem() types.Type
}

type hasTypeParams interface {
	TypeParams() *types.TypeParamList
}

func elem(t types.Type) (hasElem, bool) {
	v, ok := t.(hasElem)
	return v, ok
}

func typeParams(t types.Type) (hasTypeParams, bool) {
	v, ok := t.(hasTypeParams)
	return v, ok
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(objectpath.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/types": "go_types"})
	for _, bad := range []string{
		"downcast_ref::<go_types::Array>()",
		"downcast_ref::<go_types::Named>()",
	} {
		if strings.Contains(rust, bad) {
			t.Fatalf("source-mapped pointer assertion should not downcast to the pointee, found %q:\n%s", bad, rust)
		}
	}
	for _, want := range []string{
		"downcast_ref::<go_types::ArrayPtr>()",
		"downcast_ref::<go_types::NamedPtr>()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("source-mapped pointer assertion should downcast to pointer wrappers, missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "let __recv_guard = self.0.borrow();\n        let __recv = __recv_guard.as_ref().unwrap();\n        go_types::Named::type_params(__recv)") {
		t.Fatalf("source-mapped pointer local-interface impl should mutably borrow methods that require mutable receivers:\n%s", rust)
	}
}

func TestSourceMappedPointerAssertionCandidateWithoutSourceInterfaceUsesPointee(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "parser.go", `package parser

import (
	"go/ast"
	"go/token"
)

type hasPos interface {
	Pos() token.Pos
}

func asHasPos(x any) (hasPos, bool) {
	v, ok := x.(hasPos)
	return v, ok
}

func objectAsAny(obj *ast.Object) any {
	return obj
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(parser.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"go/ast":   "go_ast",
		"go/token": "go_token",
	})
	if strings.Contains(rust, "ObjectPtr") {
		t.Fatalf("source-mapped pointer candidate without a source-package wrapper should not use ObjectPtr:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<go_ast::Object>()") {
		t.Fatalf("source-mapped pointer candidate without a source-package wrapper should downcast the pointee:\n%s", rust)
	}
}

func TestSourceMappedPointerInterfaceReturnWithoutSourceWrapperBoxesPointee(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "importer.go", `package importer

import (
	"go/build"
	"go/internal/srcimporter"
	"go/token"
	"go/types"
)

func sourceImporter(fset *token.FileSet) types.Importer {
	return srcimporter.New(&build.Default, fset, make(map[string]*types.Package))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(importer.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"go/build":                "go_build",
		"go/internal/srcimporter": "go_internal_srcimporter",
		"go/token":                "go_token",
		"go/types":                "go_types",
	})
	if strings.Contains(rust, "ImporterPtr") {
		t.Fatalf("source-mapped pointer return without a source-package wrapper should not use ImporterPtr:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new((*go_internal_srcimporter::new(") {
		t.Fatalf("source-mapped pointer return without a source-package wrapper should box the pointee:\n%s", rust)
	}
}

func TestProjectGeneratorPreservesLoaderInterfaceMutabilityForMainImpl(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.24
`)
	writeTestFile(t, filepath.Join(tempDir, "api", "api.go"), `package api

type Importer interface {
	Import(path string)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "mut", "mut.go"), `package mut

import "example.com/mainmod/api"

type Other struct {
	seen string
}

func (o *Other) Import(path string) {
	o.seen = path
}

var _ api.Importer = (*Other)(nil)
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import (
	"example.com/mainmod/api"
	_ "example.com/mainmod/mut"
)

type projectImporter struct{}

func (pi *projectImporter) Import(path string) {}

var _ api.Importer = (*projectImporter)(nil)

func accept(importer api.Importer) {}

func main() {
	accept(&projectImporter{})
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "impl example_com_mainmod_api::Importer for projectImporter") {
		t.Fatalf("main concrete type should implement imported interface, got:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "fn import(&mut self") {
		t.Fatalf("main imported-interface impl should preserve loader-wide mutable receiver decision:\n%s", mainRS)
	}
}

func TestSourceStdlibImportedOrderedConstraintImplForNamedScalar(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "token.go", `package token

import "cmp"

type Pos int

func ComparePos(a, b Pos) int {
	return cmp.Compare(a, b)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(token.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)
	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"cmp": "cmp"})
	if !strings.Contains(rust, "impl cmp::Ordered for Pos") {
		t.Fatalf("source-mapped ordered constraint should be implemented by named scalar types, got:\n%s", rust)
	}
}

func TestSourceStdlibOrderedSliceCallUsesRawElements(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "sort.go", `package slices

import "cmp"

func LessAt[E cmp.Ordered](data []E, i int) bool {
	return cmp.Less(data[i], data[0])
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(sort.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"cmp": "cmp"})
	if strings.Contains(rust, "Vec<Rc<RefCell<Option<E>>>>") ||
		strings.Contains(rust, "Vec<Arc<Mutex<Option<E>>>>") {
		t.Fatalf("source-mapped ordered slice should store raw elements:\n%s", rust)
	}
	if strings.Contains(rust, "cmp::less::<E>(Rc::new") ||
		strings.Contains(rust, "cmp::less::<E>(Arc::new") {
		t.Fatalf("source-mapped ordered call should pass raw elements, not wrappers:\n%s", rust)
	}
	if !strings.Contains(rust, "Vec<E>") || !strings.Contains(rust, "cmp::less::<E>(") {
		t.Fatalf("source-mapped ordered call should use raw Vec<E> and cmp::less:\n%s", rust)
	}
}

func TestSourceStdlibOrderedBinarySearchUsesRawSlice(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "cmp"

func Search[S ~[]E, E cmp.Ordered](x S, target E) (int, bool) {
	return 0, false
}

var stdPkgs = []string{"bytes", "fmt"}

func isStdPkg(path string) bool {
	_, ok := Search(stdPkgs, path)
	return ok
}
`)
	if strings.Contains(rust, "map(|__elem| Rc::new(RefCell::new(Some(__elem))))") ||
		strings.Contains(rust, "map(|__elem| Arc::new(Mutex::new(Some(__elem))))") {
		t.Fatalf("source-mapped ordered binary search should not wrap raw string slice elements:\n%s", rust)
	}
	if !strings.Contains(rust, "search::<Vec<String>, String>") {
		t.Fatalf("source-mapped ordered binary search should use raw String slice elements:\n%s", rust)
	}
}

func TestSourceStdlibComparablePointerIndexEmitsPointerIdentityComparable(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "lookup.go", `package types

import "slices"

type TypeParam struct {
	name string
}

func indexOf(tparams []*TypeParam, t *TypeParam) int {
	return slices.Index(tparams, t)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(lookup.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"slices": "slices"})
	if !strings.Contains(rust, "slices::index::<Vec<Rc<RefCell<Option<TypeParam>>>>, TypeParam>") &&
		!strings.Contains(rust, "slices::index::<Vec<Arc<Mutex<Option<TypeParam>>>>, TypeParam>") {
		t.Fatalf("source-mapped slices.Index over pointer slices should keep the existing raw pointee ABI:\n%s", rust)
	}
	if !strings.Contains(rust, "impl GoComparable for TypeParam") ||
		!strings.Contains(rust, "std::ptr::eq(self, other)") {
		t.Fatalf("source-mapped slices.Index over pointer slices should give the pointee pointer-identity GoComparable semantics:\n%s", rust)
	}
}

func TestSourceStdlibOrderedCallUnwrapsSelectorField(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "position.go", `package token

import (
	"cmp"
	"slices"
)

type lineInfo struct {
	Offset int
}

func searchLineInfos(a []lineInfo, x int) int {
	i, _ := slices.BinarySearchFunc(a, x, func(a lineInfo, x int) int {
		return cmp.Compare(a.Offset, x)
	})
	return i
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(position.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"cmp": "cmp", "slices": "slices"})
	if strings.Contains(rust, "cmp::compare::<i32>({ let __field =") {
		t.Fatalf("source-mapped ordered selector argument should pass the raw field value, not the field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "cmp::compare::<i32>({ let __selector_holder =") {
		t.Fatalf("source-mapped ordered selector argument should unwrap through an owned selector value:\n%s", rust)
	}
	compareStart := strings.Index(rust, "cmp::compare::<i32>")
	if compareStart < 0 {
		t.Fatalf("source-mapped ordered call should emit cmp::compare:\n%s", rust)
	}
	compareEnd := compareStart + 500
	if compareEnd > len(rust) {
		compareEnd = len(rust)
	}
	compareCall := rust[compareStart:compareEnd]
	if strings.Contains(compareCall, ", Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap()).clone())))") ||
		strings.Contains(compareCall, ", Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()).clone())))") {
		t.Fatalf("source-mapped ordered scalar argument should pass the raw value, not a wrapper:\n%s", rust)
	}
}

func TestSourceStdlibOrderedCallUnwrapsCallResult(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "comment.go", `package ast

import "cmp"

type Pos int

type CommentGroup struct {
	Slash Pos
}

func (g *CommentGroup) Pos() Pos {
	return g.Slash
}

func CompareGroups(a, b *CommentGroup) int {
	return cmp.Compare(a.Pos(), b.Pos())
}

func importComment(g *CommentGroup) string {
	return ""
}

func CompareComments(a, b *CommentGroup) int {
	return cmp.Compare(importComment(a), importComment(b))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(comment.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"cmp": "cmp"})
	if strings.Contains(rust, "cmp::compare::<Pos>({ let __recv = a.clone()") {
		t.Fatalf("source-mapped ordered call should pass the raw method result, not the result handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".pos().borrow().as_ref().unwrap()).clone()") &&
		!strings.Contains(rust, ".pos().lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("source-mapped ordered call should unwrap method results before comparing:\n%s", rust)
	}
	if strings.Contains(rust, "cmp::compare::<String>(import_comment(a.clone()),") {
		t.Fatalf("source-mapped ordered call should pass the raw function result, not the result handle:\n%s", rust)
	}
	if !strings.Contains(rust, "cmp::compare::<String>((*import_comment(a.clone()).borrow().as_ref().unwrap()).clone()") &&
		!strings.Contains(rust, "cmp::compare::<String>((*import_comment(a.clone()).lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("source-mapped ordered call should unwrap function results before comparing:\n%s", rust)
	}
}

func TestSourceStdlibImportedInterfaceTypeExprUsesTraitObject(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "parser.go", `package parser

import "go/ast"

type field struct {
	typ ast.Expr
}

func parseExprList() []ast.Expr {
	var list []ast.Expr
	return list
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

	if strings.Contains(rust, "Option<go_ast::Expr>") {
		t.Fatalf("source-mapped interface type expression should not be emitted as a bare trait type:\n%s", rust)
	}
	if !strings.Contains(rust, "Option<Box<dyn go_ast::Expr") {
		t.Fatalf("source-mapped interface type expression should use a trait object:\n%s", rust)
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

func TestSourceStdlibPruningFiltersInterfaceAssertionCandidates(t *testing.T) {
	prevCtx := GetTranspileContext()
	SetTranspileContext(&TranspileContext{PackageMapping: map[string]string{"go/types": "go_types"}})
	defer SetTranspileContext(prevCtx)
	prevReachable := sourceStdlibReachable
	defer SetSourceStdlibReachable(prevReachable)

	fset := token.NewFileSet()
	pkg := parsePackageForReachabilityTest(t, fset, "go/types", "api.go", `package types

type LiveError struct{}
func (LiveError) Error() string { return "live" }

type DeadError struct{}
func (DeadError) Error() string { return "dead" }

func take(v any) {}

func live(err error) {
	take(err)
}
`)
	typeInfo, err := NewTypeInfoWithImporter("go/types", pkg.Syntax, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(go/types) error = %v", err)
	}
	pkg.Types = typeInfo.pkg
	pkg.TypesInfo = typeInfo.info
	SetSourceStdlibReachable(map[types.Object]bool{
		definitionByName(t, pkg, "LiveError"): true,
		definitionByName(t, pkg, "Error"):     true,
		definitionByName(t, pkg, "live"):      true,
		definitionByName(t, pkg, "take"):      true,
	})

	rust, _, _ := TranspileWithMapping(pkg.Syntax[0], fset, typeInfo, map[string]string{"go/types": "go_types"})
	if strings.Contains(rust, "downcast_ref::<DeadError>()") {
		t.Fatalf("error-to-any lowering should not reference pruned source-mapped candidates:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<LiveError>()") {
		t.Fatalf("error-to-any lowering should keep reachable source-mapped candidates:\n%s", rust)
	}
}

func TestSourceStdlibPruningFiltersImportedInterfaceAssertionCandidatesByStableKey(t *testing.T) {
	prevCtx := GetTranspileContext()
	SetTranspileContext(&TranspileContext{PackageMapping: map[string]string{"go/types": "go_types"}})
	defer SetTranspileContext(prevCtx)
	prevReachable := sourceStdlibReachable
	defer SetSourceStdlibReachable(prevReachable)

	fset := token.NewFileSet()
	sourcePkg := parsePackageForReachabilityTest(t, fset, "go/types", "api.go", `package types

type LiveError struct{}
func (LiveError) Error() string { return "live" }

type DeadError struct{}
func (DeadError) Error() string { return "dead" }
`)
	sourceTypeInfo, err := NewTypeInfoWithImporter("go/types", sourcePkg.Syntax, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(source go/types) error = %v", err)
	}
	sourcePkg.Types = sourceTypeInfo.pkg
	sourcePkg.TypesInfo = sourceTypeInfo.info

	importedPkg := parsePackageForReachabilityTest(t, fset, "go/types", "api_imported.go", `package types

type LiveError struct{}
func (LiveError) Error() string { return "live" }

type DeadError struct{}
func (DeadError) Error() string { return "dead" }
`)
	importedTypeInfo, err := NewTypeInfoWithImporter("go/types", importedPkg.Syntax, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(imported go/types) error = %v", err)
	}
	importedPkg.Types = importedTypeInfo.pkg
	importedPkg.TypesInfo = importedTypeInfo.info

	mainPkg := parsePackageForReachabilityTest(t, fset, "main", "main.go", `package main

import gotypes "go/types"

func take(v any) {}

func use(err error, _ gotypes.LiveError) {
	take(err)
}
`)
	mainTypeInfo, err := NewTypeInfoWithImporter("main", mainPkg.Syntax, fset, exprTestImporter{
		"go/types": importedPkg.Types,
	})
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(main) error = %v", err)
	}
	mainPkg.Types = mainTypeInfo.pkg
	mainPkg.TypesInfo = mainTypeInfo.info

	SetSourceStdlibReachable(map[types.Object]bool{
		definitionByName(t, sourcePkg, "LiveError"): true,
	})

	rust, _, _ := TranspileWithMapping(mainPkg.Syntax[0], fset, mainTypeInfo, map[string]string{"go/types": "go_types"})
	if strings.Contains(rust, "go_types::DeadError") {
		t.Fatalf("error-to-any lowering should not reference pruned imported source-mapped candidates with different object identity:\n%s", rust)
	}
	if !strings.Contains(rust, "go_types::LiveError") {
		t.Fatalf("error-to-any lowering should keep reachable imported source-mapped candidates by stable key:\n%s", rust)
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

func TestSourceStdlibPruningSkipsImportedInterfaceImplForPrunedInterface(t *testing.T) {
	prevReachable := sourceStdlibReachable
	defer SetSourceStdlibReachable(prevReachable)

	fset := token.NewFileSet()
	fsPkg := parsePackageForReachabilityTest(t, fset, "example.com/fs", "fs.go", `package fs

type File interface {
	Close() error
}
`)
	osPkg := parsePackageForReachabilityTest(t, fset, "example.com/os", "os.go", `package os

import "example.com/fs"

type File struct{}

func (File) Close() error { return nil }

func use(sink func(fs.File), f File) {
	sink(f)
}
`)
	loader := &PackageLoader{
		fileSet: fset,
		mainPkg: osPkg,
		allPackages: map[string]*packages.Package{
			"example.com/fs": fsPkg,
			"example.com/os": osPkg,
		},
		packageMapping: map[string]string{"example.com/fs": "example_com_fs"},
	}
	if err := loader.typeCheckLocalPackage(fsPkg, loader.projectImporter()); err != nil {
		t.Fatalf("typeCheckLocalPackage(example.com/fs) error = %v", err)
	}
	if err := loader.typeCheckLocalPackage(osPkg, loader.projectImporter()); err != nil {
		t.Fatalf("typeCheckLocalPackage(example.com/os) error = %v", err)
	}
	SetSourceStdlibReachable(map[types.Object]bool{})

	typeInfo := &TypeInfo{info: osPkg.TypesInfo, pkg: osPkg.Types}
	rust, _, _ := TranspileWithMapping(osPkg.Syntax[0], fset, typeInfo, map[string]string{"example.com/fs": "example_com_fs"})
	if strings.Contains(rust, "impl example_com_fs::File for File") {
		t.Fatalf("pruned imported interface should not get an impl block:\n%s", rust)
	}
}

func TestSourceStdlibPruningSkipsCrossFileLocalInterfaceImplForPrunedInterface(t *testing.T) {
	t.Setenv(sourceStdlibPackagesEnv, "sync")
	prevCtx := GetTranspileContext()
	SetTranspileContext(&TranspileContext{PackageMapping: map[string]string{"sync": "sync"}})
	defer SetTranspileContext(prevCtx)
	prevReachable := sourceStdlibReachable
	defer SetSourceStdlibReachable(prevReachable)

	fset := token.NewFileSet()
	mutexFile, err := parser.ParseFile(fset, "mutex.go", `package sync

type Locker interface {
	Lock()
	Unlock()
}

type Mutex struct {
	_ noCopy
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(mutex.go) error = %v", err)
	}
	condFile, err := parser.ParseFile(fset, "cond.go", `package sync

type noCopy struct{}

func (noCopy) Lock() {}
func (noCopy) Unlock() {}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(cond.go) error = %v", err)
	}
	mainPkg := parsePackageForReachabilityTest(t, fset, "main", "main.go", `package main

import "sync"

var _ sync.Mutex
`)
	syncPkg := &packages.Package{
		Name:            "sync",
		PkgPath:         "sync",
		Fset:            fset,
		GoFiles:         []string{"mutex.go", "cond.go"},
		CompiledGoFiles: []string{"mutex.go", "cond.go"},
		Syntax:          []*ast.File{mutexFile, condFile},
		Imports:         make(map[string]*packages.Package),
	}
	loader := &PackageLoader{
		fileSet: fset,
		mainPkg: mainPkg,
		allPackages: map[string]*packages.Package{
			"main": mainPkg,
			"sync": syncPkg,
		},
		packageMapping: map[string]string{"sync": "sync"},
	}
	if err := loader.typeCheckLocalPackage(syncPkg, loader.projectImporter()); err != nil {
		t.Fatalf("typeCheckLocalPackage(sync) error = %v", err)
	}
	if err := loader.typeCheckLocalPackage(mainPkg, loader.projectImporter()); err != nil {
		t.Fatalf("typeCheckLocalPackage(main) error = %v", err)
	}
	SetSourceStdlibReachable(loader.computeSourceStdlibReachable())

	rust, _, _ := TranspileWithMapping(condFile, fset, &TypeInfo{info: syncPkg.TypesInfo, pkg: syncPkg.Types}, map[string]string{"sync": "sync"})
	if strings.Contains(rust, "impl Locker for noCopy") {
		t.Fatalf("pruned cross-file local interface should not get an impl block:\n%s", rust)
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
	if !strings.Contains(mainRS, "example_com_mainmod_abi::StructField {") &&
		!strings.Contains(mainRS, "example_com_mainmod_abi::r#mod::StructField {") {
		t.Fatalf("alias struct literal should construct the aliased package struct:\n%s", mainRS)
	}
}

func TestImportedNamedScalarAliasMapKeyUsesAliasedValueType(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "abi", "abi.go"), `package abi

type TypeOff int32
type Type struct{}
`)
	writeTestFile(t, filepath.Join(tempDir, "rt", "rt.go"), `package rt

import "example.com/mainmod/abi"

type typeOff = abi.TypeOff

type moduledata struct {
	typemap map[typeOff]*abi.Type
}

func New() moduledata {
	return moduledata{}
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/mainmod/rt"

func main() {
	_ = rt.New()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	rtRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_mainmod_rt", "mod.rs"))
	if strings.Contains(rtRS, "BTreeMap<typeOff,") {
		t.Fatalf("map key using imported named scalar alias should not use the alias handle type:\n%s", rtRS)
	}
	if !strings.Contains(rtRS, "BTreeMap<example_com_mainmod_abi::TypeOff, Arc<Mutex<Option<example_com_mainmod_abi::Type>>>") &&
		!strings.Contains(rtRS, "BTreeMap<example_com_mainmod_abi::r#mod::TypeOff, Arc<Mutex<Option<example_com_mainmod_abi::r#mod::Type>>>") &&
		!strings.Contains(rtRS, "BTreeMap<example_com_mainmod_abi::TypeOff, Rc<RefCell<Option<example_com_mainmod_abi::Type>>>") &&
		!strings.Contains(rtRS, "BTreeMap<example_com_mainmod_abi::r#mod::TypeOff, Rc<RefCell<Option<example_com_mainmod_abi::r#mod::Type>>>") {
		t.Fatalf("map key using imported named scalar alias should use the unaliased value type:\n%s", rtRS)
	}
}

func TestCrossPackageGoPtrParamFactPropagatesToCalleeSignature(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "leaf", "leaf.go"), `package leaf

//go:noescape
func Touch(p *byte)
`)
	writeTestFile(t, filepath.Join(tempDir, "worker", "worker.go"), `package worker

import "example.com/mainmod/leaf"

type holder struct {
	value byte
}

func mid(p *byte) {
	leaf.Touch(p)
}

func Use(buf []byte) {
	mid(&buf[0])
}

func UseDirect(buf []byte) {
	leaf.Touch(&buf[0])
}

func UseLocal(h *holder) {
	leaf.Touch(&h.value)
}

func UseNil() {
	leaf.Touch(nil)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/mainmod/worker"

func main() {
	buf := []byte{1}
	worker.Use(buf)
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	leafRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_mainmod_leaf", "mod.rs"))
	if !strings.Contains(leafRS, "pub fn touch(p: GoPtr<u8>)") {
		t.Fatalf("cross-package noescape callee should receive the propagated GoPtr param fact:\n%s", leafRS)
	}
	if strings.Contains(leafRS, "pub fn touch(p: Rc<RefCell<Option<u8>>>)") ||
		strings.Contains(leafRS, "pub fn touch(p: Arc<Mutex<Option<u8>>>)") {
		t.Fatalf("cross-package noescape callee should not keep the local pointer wrapper signature:\n%s", leafRS)
	}

	workerRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_mainmod_worker", "mod.rs"))
	if strings.Contains(workerRS, `unimplemented!("GoPtr parameter argument requires pointer-compatible value")`) {
		t.Fatalf("cross-package GoPtr-param calls should not fall back to an unimplemented argument path:\n%s", workerRS)
	}
	if strings.Contains(workerRS, "leaf::touch(__arg0.clone())") {
		t.Fatalf("direct package selector noescape element-pointer call should not use the wrapper temp adapter:\n%s", workerRS)
	}
	if !strings.Contains(workerRS, "leaf::touch(example_com_mainmod_leaf::GoPtr::slice_elem(") {
		t.Fatalf("direct package selector noescape element-pointer call should use the callee crate GoPtr constructor:\n%s", workerRS)
	}
	if !strings.Contains(workerRS, "leaf::touch(example_com_mainmod_leaf::GoPtr::local(") {
		t.Fatalf("package selector call to a GoPtr-param function should wrap local pointer handles with GoPtr::local:\n%s", workerRS)
	}
	if !strings.Contains(workerRS, "leaf::touch(example_com_mainmod_leaf::GoPtr::nil())") {
		t.Fatalf("package selector call to a GoPtr-param function should call the nil constructor:\n%s", workerRS)
	}
}

func TestCrossPackageGoPtrMethodParamUsesCalleeHelperType(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "slot", "slot.go"), `package slot

type Pointer[T any] struct {
	value *T
}

func (p *Pointer[T]) Store(value *T) {
	p.value = value
}
`)
	writeTestFile(t, filepath.Join(tempDir, "worker", "worker.go"), `package worker

import (
	"unsafe"

	"example.com/mainmod/slot"
)

type entry[T any] struct {
	value T
}

type node[T any] struct {
	value T
}

func (n *node[T]) entry() *entry[T] {
	return (*entry[T])(unsafe.Pointer(n))
}

func Use[T any](p *slot.Pointer[entry[T]], n *node[T]) {
	var old *entry[T]
	if n != nil {
		old = n.entry()
	}
	p.Store(old)
}

func UseDirect(p *slot.Pointer[entry[int]], values []entry[int]) {
	p.Store(&values[0])
}

func Run() {
	var p slot.Pointer[entry[int]]
	var n node[int]
	Use(&p, &n)
	values := []entry[int]{{}}
	UseDirect(&p, values)
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/mainmod/worker"

func main() {
	worker.Run()
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	slotRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_mainmod_slot", "mod.rs"))
	if !strings.Contains(slotRS, "pub fn store(&mut self, value: GoPtr<T>)") {
		t.Fatalf("imported generic method should receive the propagated GoPtr param fact:\n%s", slotRS)
	}
	if strings.Contains(slotRS, "pub fn store(&mut self, value: Rc<RefCell<Option<T>>>)") ||
		strings.Contains(slotRS, "pub fn store(&mut self, value: Arc<Mutex<Option<T>>>)") {
		t.Fatalf("imported generic method should not keep the ordinary pointer wrapper signature:\n%s", slotRS)
	}

	workerRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_mainmod_worker", "mod.rs"))
	if strings.Contains(workerRS, ".store(old.clone())") {
		t.Fatalf("cross-package method call should not pass the caller crate GoPtr directly:\n%s", workerRS)
	}
	for _, want := range []string{
		"match __go_ptr",
		"example_com_mainmod_slot::GoPtr::nil()",
		"example_com_mainmod_slot::GoPtr::local(",
		"example_com_mainmod_slot::GoPtr::slice_elem(",
	} {
		if !strings.Contains(workerRS, want) {
			t.Fatalf("cross-package method GoPtr argument should include %q:\n%s", want, workerRS)
		}
	}
}

func TestCrossPackageGoPtrUnsafeReceiverIdentityMethodCallUsesOriginalPointee(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "abi", "abi.go"), `package abi

import "unsafe"

type Node struct {
	IsEntry bool
}

type Entry struct {
	Node
	Value int
}

func Pick(addr uintptr) (*Node, int) {
	return (*Node)(unsafe.Pointer(addr)), 0
}

func (n *Node) ValueOrZero() int {
	if !n.IsEntry {
		return 0
	}
	return (*Entry)(unsafe.Pointer(n)).Value
}
`)
	writeTestFile(t, filepath.Join(tempDir, "worker", "worker.go"), `package worker

import "example.com/mainmod/abi"

func Use(addr uintptr) int {
	n, _ := abi.Pick(addr)
	return n.ValueOrZero()
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import "example.com/mainmod/worker"

func main() {
	_ = worker.Use(0)
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	workerRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_mainmod_worker", "mod.rs"))
	if strings.Contains(workerRS, "let __recv_value = n.borrow(); let __result = (*__recv_value.as_ref().unwrap()).value_or_zero(") {
		t.Fatalf("cross-package unsafe receiver-identity method call should not call through a cloned pointee:\n%s", workerRS)
	}
	if !strings.Contains(workerRS, "n.with_mut(|__recv_value| __recv_value.value_or_zero(") {
		t.Fatalf("cross-package unsafe receiver-identity method call should use the original pointee:\n%s", workerRS)
	}
}

func TestCrossPackageGoPtrFieldReturnFactPropagatesToImporterSignature(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "abi", "abi.go"), `package abi

type Type struct {
	Data *byte
}

func Set(t *Type, buf []byte) {
	t.Data = &buf[0]
}
`)
	writeTestFile(t, filepath.Join(tempDir, "rt", "rt.go"), `package rt

import "example.com/mainmod/abi"

type _type = abi.Type

func Get(t *_type) *byte {
	return t.Data
}
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import (
	"example.com/mainmod/abi"
	"example.com/mainmod/rt"
)

func main() {
	var t abi.Type
	buf := []byte{1}
	abi.Set(&t, buf)
	_ = rt.Get(&t)
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	generator.SetExternalPackageMode(ModeTranspile)
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	abiRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_mainmod_abi", "mod.rs"))
	if !strings.Contains(abiRS, "pub data: GoPtr<u8>") {
		t.Fatalf("defining package should emit slice-element pointer field as GoPtr:\n%s", abiRS)
	}

	rtRS := mustReadFile(t, filepath.Join(tempDir, "vendor", "example_com_mainmod_rt", "mod.rs"))
	if !strings.Contains(rtRS, "pub fn get(") || !strings.Contains(rtRS, " -> GoPtr<u8>") {
		t.Fatalf("importing package returning a GoPtr field should use GoPtr result type:\n%s", rtRS)
	}
	if strings.Contains(rtRS, " -> Arc<Mutex<Option<u8>>>") ||
		strings.Contains(rtRS, " -> Rc<RefCell<Option<u8>>>") {
		t.Fatalf("importing package should not keep the old pointer wrapper result for a GoPtr field:\n%s", rtRS)
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
