package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestFormatSliceImportNamesIncludeWrappedFormatter(t *testing.T) {
	ht := &HelperTracker{needsFormatSlice: true}
	names := ht.ImportNames()
	seen := make(map[string]bool)
	for _, name := range names {
		seen[name] = true
	}

	for _, name := range []string{"format_slice", "format_slice_values", "format_slice_wrapped"} {
		if !seen[name] {
			t.Fatalf("ImportNames() missing %q in %v", name, names)
		}
	}
}

func TestGoRWMutexHelperExportsLockMethods(t *testing.T) {
	ht := &HelperTracker{needsGoRWMutex: true}
	helper := ht.GenerateHelperModule()

	for _, want := range []string{"pub struct GoRWMutex", "pub fn r_lock(&self)", "pub fn r_unlock(&self)"} {
		if !strings.Contains(helper, want) {
			t.Fatalf("GoRWMutex helper should export %q for cross-crate ForkLock users:\n%s", want, helper)
		}
	}
}

func TestGoPtrKeyHelperModuleImportsWrapperTypes(t *testing.T) {
	prevConcurrencyDetector := GetConcurrencyDetector()
	t.Cleanup(func() {
		SetConcurrencyDetector(prevConcurrencyDetector)
	})

	SetConcurrencyDetector(nil)
	helper := (&HelperTracker{needsGoPtrKey: true}).GenerateHelperModule()
	for _, want := range []string{"use std::rc::{Rc};", "use std::cell::{RefCell};"} {
		if !strings.Contains(helper, want) {
			t.Fatalf("single-threaded GoLocalPtrKey helper should import %q:\n%s", want, helper)
		}
	}

	cd := NewConcurrencyDetector()
	cd.hasChannels = true
	SetConcurrencyDetector(cd)
	helper = (&HelperTracker{needsGoPtrKey: true}).GenerateHelperModule()
	if !strings.Contains(helper, "use std::sync::{Arc, Mutex};") {
		t.Fatalf("concurrent GoLocalPtrKey helper should import Arc and Mutex:\n%s", helper)
	}
}

func TestWrapperMutexImportAliasesWhenLocalTypeUsesMutex(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Mutex struct {
	state int
}

func forceConcurrent() {
	go func() {}()
}
`)

	if !strings.Contains(rust, "use std::sync::{Arc, Mutex as StdMutex};") {
		t.Fatalf("wrapper mutex import should be aliased around local Mutex type:\n%s", rust)
	}
	if !strings.Contains(rust, "pub struct Mutex") {
		t.Fatalf("local Mutex type should keep its Rust type name:\n%s", rust)
	}
	if strings.Contains(rust, "Arc<Mutex<Option<") || strings.Contains(rust, "Arc::new(Mutex::new") {
		t.Fatalf("wrapper output should use the aliased StdMutex name:\n%s", rust)
	}
	if !strings.Contains(rust, "state: Arc<StdMutex<Option<i32>>>") {
		t.Fatalf("struct field wrapper should use StdMutex:\n%s", rust)
	}
	if !strings.Contains(rust, "state: Arc::new(StdMutex::new(Some(0)))") {
		t.Fatalf("struct default wrapper should construct through StdMutex:\n%s", rust)
	}
}

func TestWrapperMutexImportAliasesWhenSiblingModuleExportsMutex(t *testing.T) {
	fset := token.NewFileSet()
	mutexFile, err := parser.ParseFile(fset, "mutex.go", `package internal_sync

type Mutex struct{}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(mutex.go) error = %v", err)
	}
	hashFile, err := parser.ParseFile(fset, "hashtriemap.go", `package internal_sync

import "unsafe"

type Holder struct {
	mu Mutex
}

func fromPointer(p unsafe.Pointer) *Holder {
	return (*Holder)(p)
}

func forceConcurrent() {
	go func() {}()
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(hashtriemap.go) error = %v", err)
	}
	files := []*ast.File{mutexFile, hashFile}
	typeInfo, err := NewTypeInfo(files, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo error = %v", err)
	}
	cd := NewConcurrencyDetector()
	cd.AnalyzeProject(files)
	prevCD := GetConcurrencyDetector()
	SetConcurrencyDetector(cd)
	defer SetConcurrencyDetector(prevCD)

	packageState := NewPackageState()
	packageState.TypeModuleNames["Mutex"] = "mutex"
	packageState.TypeModuleNames["Holder"] = "hashtriemap"
	SetTranspileContext(&TranspileContext{
		Session:           NewTranspileSession(typeInfo, nil),
		Package:           packageState,
		CurrentModuleName: "hashtriemap",
	})
	defer SetTranspileContext(nil)

	rust, _, _ := Transpile(hashFile, fset, typeInfo)

	if !strings.Contains(rust, "use std::sync::{Arc, Mutex as StdMutex};") {
		t.Fatalf("wrapper mutex import should be aliased around sibling Mutex type:\n%s", rust)
	}
	if strings.Contains(rust, "Arc<Mutex<Option<Mutex>>") || strings.Contains(rust, "Arc::new(Mutex::new") {
		t.Fatalf("wrapper output should use StdMutex when sibling module exports Mutex:\n%s", rust)
	}
	if !strings.Contains(rust, "pub mu: Arc<StdMutex<Option<Mutex>>>") {
		t.Fatalf("same-package Mutex field should keep inner Mutex and use StdMutex wrapper:\n%s", rust)
	}
}
