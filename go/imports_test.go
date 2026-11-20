package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"os/exec"
	"path/filepath"
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

func TestEmbeddedOwnerRegistryLivesInSharedStdlibHelpers(t *testing.T) {
	ht := &HelperTracker{needsEmbeddedOwnerRegistry: true}

	local := ht.GenerateHelperModuleOmittingSharedStdlibHelpers()
	if strings.Contains(local, "go_embedded_owner_registry") ||
		strings.Contains(local, "go_register_embedded_owner") ||
		strings.Contains(local, "go_lookup_embedded_owner") {
		t.Fatalf("package-local helper module should omit shared embedded-owner registry:\n%s", local)
	}

	shared := ht.GenerateSharedStdlibHelperModule()
	for _, want := range []string{
		"GO_EMBEDDED_OWNER_REGISTRY",
		"pub fn go_register_embedded_owner",
		"pub fn go_lookup_embedded_owner",
	} {
		if !strings.Contains(shared, want) {
			t.Fatalf("shared stdlib helper module missing %q:\n%s", want, shared)
		}
	}

	names := ht.ImportNamesOmittingSharedStdlibHelpers()
	for _, name := range names {
		if name == "go_register_embedded_owner" || name == "go_lookup_embedded_owner" {
			t.Fatalf("package-local helper imports should omit shared embedded-owner registry names: %v", names)
		}
	}

	merged := &HelperTracker{}
	mergeHelperTracker(merged, ht)
	if !strings.Contains(merged.GenerateSharedStdlibHelperModule(), "pub fn go_lookup_embedded_owner") {
		t.Fatalf("merged shared helper state should retain embedded-owner registry need")
	}
}

func TestPanicRecoverRuntimeLivesInSharedStdlibHelpers(t *testing.T) {
	ht := &HelperTracker{needsPanicRecover: true}

	local := ht.GenerateHelperModuleOmittingSharedStdlibHelpers()
	if strings.Contains(local, "fn go_recover") || strings.Contains(local, "fn go_store_panic_payload") {
		t.Fatalf("package-local helper module should omit the shared panic runtime:\n%s", local)
	}

	shared := ht.GenerateSharedStdlibHelperModule()
	for _, want := range []string{
		"pub struct GoFunctionDepthGuard",
		"pub fn go_enter_function",
		"pub struct GoPanicHookSuppression",
		"pub fn go_suppress_panic_hook",
		"pub fn go_panic_any_local",
		"pub fn go_recover",
		"pub fn go_replace_panic_payload",
		"pub fn go_store_panic_payload",
		"pub fn go_resume_unrecovered_panic",
	} {
		if !strings.Contains(shared, want) {
			t.Fatalf("shared stdlib helper module missing %q:\n%s", want, shared)
		}
	}

	names := ht.ImportNames()
	for _, want := range []string{"GoFunctionDepthGuard", "go_enter_function", "GoPanicHookSuppression", "go_suppress_panic_hook"} {
		found := false
		for _, name := range names {
			if name == want {
				found = true
				break
			}
		}
		if !found {
			t.Fatalf("panic runtime imports missing %q in %v", want, names)
		}
	}
	if !strings.Contains(shared, "static __GO_PANIC_HOOK_INIT: std::sync::Once") ||
		!strings.Contains(shared, "static __GO_PANIC_HOOK_SUPPRESSION_DEPTH: std::cell::Cell<usize>") ||
		strings.Count(shared, "std::panic::take_hook()") != 1 {
		t.Fatalf("shared panic runtime should install one forwarding hook with thread-local suppression:\n%s", shared)
	}
}

func TestPanicHookSuppressionIsThreadLocalAndRestoresDiagnostics(t *testing.T) {
	if _, err := exec.LookPath("rustc"); err != nil {
		t.Skip("rustc is not available")
	}

	helper := (&HelperTracker{needsPanicRecover: true}).GenerateHelperModule()
	harness := `
static REPORTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn main() {
    std::panic::set_hook(Box::new(|_| {
        REPORTS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }));

    let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
    let first_barrier = barrier.clone();
    let first = std::thread::spawn(move || {
        let _guard = go_suppress_panic_hook();
        first_barrier.wait();
        let _ = std::panic::catch_unwind(|| panic!("first recovered panic"));
    });
    let second_barrier = barrier.clone();
    let second = std::thread::spawn(move || {
        let _guard = go_suppress_panic_hook();
        second_barrier.wait();
        let _ = std::panic::catch_unwind(|| panic!("second recovered panic"));
    });

    barrier.wait();
    first.join().unwrap();
    second.join().unwrap();
    assert_eq!(REPORTS.load(std::sync::atomic::Ordering::SeqCst), 0);

    let _ = std::panic::catch_unwind(|| panic!("later uncaught-style panic"));
    assert_eq!(REPORTS.load(std::sync::atomic::Ordering::SeqCst), 1);
}
`

	dir := t.TempDir()
	source := filepath.Join(dir, "main.rs")
	if err := os.WriteFile(source, []byte(helper+harness), 0o600); err != nil {
		t.Fatal(err)
	}
	binary := filepath.Join(dir, "panic-hook-test")
	if output, err := exec.Command("rustc", "--edition=2021", source, "-o", binary).CombinedOutput(); err != nil {
		t.Fatalf("rustc failed: %v\n%s", err, output)
	}
	if output, err := exec.Command(binary).CombinedOutput(); err != nil {
		t.Fatalf("panic-hook harness failed: %v\n%s", err, output)
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

func TestGoPtrHelperCloneDoesNotRequireCloneablePointee(t *testing.T) {
	prevConcurrencyDetector := GetConcurrencyDetector()
	t.Cleanup(func() {
		SetConcurrencyDetector(prevConcurrencyDetector)
	})

	SetConcurrencyDetector(nil)
	helper := (&HelperTracker{needsSliceElemPtr: true, needsGoAtomicPointer: true}).GenerateHelperModule()
	for _, forbidden := range []string{
		"pub enum GoPtr<T: Clone + 'static>",
		"impl<T: Clone + 'static> Clone for GoPtr<T>",
		"struct GoAtomicPointer<T: Clone + 'static>",
	} {
		if strings.Contains(helper, forbidden) {
			t.Fatalf("single-threaded GoPtr helper should clone pointer handles without requiring cloned pointees, found %q:\n%s", forbidden, helper)
		}
	}
	for _, want := range []string{
		"pub enum GoPtr<T: 'static>",
		"impl<T: 'static> Clone for GoPtr<T>",
		"impl<T: Clone + 'static> GoPtr<T>",
		"struct GoAtomicPointer<T: 'static>",
		"pub struct GoForeignArrayElemPtrDyn<T: 'static>",
		"Rc<dyn Fn() -> Option<T>>",
		"pub fn array_elem_foreign(",
		"GoPtr::ArrayElem(value) => value.identity_dyn().0.is_null()",
	} {
		if !strings.Contains(helper, want) {
			t.Fatalf("single-threaded GoPtr helper missing %q:\n%s", want, helper)
		}
	}

	cd := NewConcurrencyDetector()
	cd.hasChannels = true
	SetConcurrencyDetector(cd)
	helper = (&HelperTracker{needsSliceElemPtr: true, needsGoAtomicPointer: true}).GenerateHelperModule()
	for _, forbidden := range []string{
		"pub enum GoPtr<T: Clone + Send + Sync + 'static>",
		"impl<T: Clone + Send + Sync + 'static> Clone for GoPtr<T>",
		"struct GoAtomicPointer<T: Clone + Send + Sync + 'static>",
	} {
		if strings.Contains(helper, forbidden) {
			t.Fatalf("concurrent GoPtr helper should clone pointer handles without requiring cloned pointees, found %q:\n%s", forbidden, helper)
		}
	}
	for _, want := range []string{
		"pub enum GoPtr<T: Send + Sync + 'static>",
		"impl<T: Send + Sync + 'static> Clone for GoPtr<T>",
		"impl<T: Clone + Send + Sync + 'static> GoPtr<T>",
		"struct GoAtomicPointer<T: Send + Sync + 'static>",
		"pub struct GoForeignArrayElemPtrDyn<T: Send + Sync + 'static>",
		"Arc<dyn Fn() -> Option<T> + Send + Sync>",
		"pub fn array_elem_foreign(",
		"GoPtr::ArrayElem(value) => value.identity_dyn().0.is_null()",
	} {
		if !strings.Contains(helper, want) {
			t.Fatalf("concurrent GoPtr helper missing %q:\n%s", want, helper)
		}
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
	if !strings.Contains(rust, "state: Arc<StdMutex<Option<i64>>>") {
		t.Fatalf("struct field wrapper should use StdMutex:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_default_0_0 = Arc::new(StdMutex::new(Some(0)));") ||
		!strings.Contains(rust, "state: __go_default_0_0") {
		t.Fatalf("struct default wrapper should construct through StdMutex:\n%s", rust)
	}
}

func TestAtomicLoadPointerUsesAliasedWrapperMutexWhenLocalTypeUsesMutex(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import (
	"sync/atomic"
	"unsafe"
)

type Mutex struct{}

type slot struct {
	typ unsafe.Pointer
}

func load(s *slot) unsafe.Pointer {
	return atomic.LoadPointer(&s.typ)
}

func forceConcurrent() {
	go func() {}()
}
`)

	if !strings.Contains(rust, "use std::sync::{Arc, Mutex as StdMutex};") {
		t.Fatalf("wrapper mutex import should be aliased around local Mutex type:\n%s", rust)
	}
	if strings.Contains(rust, "Arc::new(Mutex::new((*__guard).clone()))") {
		t.Fatalf("atomic.LoadPointer should use the aliased StdMutex wrapper constructor:\n%s", rust)
	}
	if !strings.Contains(rust, "Arc::new(StdMutex::new((*__guard).clone()))") {
		t.Fatalf("atomic.LoadPointer should construct the wrapper through StdMutex:\n%s", rust)
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
