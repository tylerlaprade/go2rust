package main

import (
	"go/ast"
	"go/importer"
	"go/parser"
	"go/token"
	"go/types"
	"path/filepath"
	"strings"
	"testing"
)

func TestTranspileFunctionWithoutBodyDoesNotPanic(t *testing.T) {
	var out strings.Builder
	fn := &ast.FuncDecl{
		Name: ast.NewIdent("externalFunc"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{},
			Results: &ast.FieldList{
				List: []*ast.Field{{Type: ast.NewIdent("int")}},
			},
		},
	}

	TranspileFunction(&out, fn, token.NewFileSet(), nil)

	got := out.String()
	if !strings.Contains(got, "pub fn external_func() -> Rc<RefCell<Option<i32>>>") {
		t.Fatalf("missing function signature in:\n%s", got)
	}
	if !strings.Contains(got, "unimplemented!(\"Go function declaration has no body\")") {
		t.Fatalf("missing bodyless function fallback in:\n%s", got)
	}
}

func TestCryptoBoringSigBodylessFunctionsEmitNoOps(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "sig.go", `package sig

func BoringCrypto()
func FIPSOnly()
func StandardCrypto()
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(sig.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("crypto/internal/boring/sig", []*ast.File{file}, fset, importer.Default())
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(crypto/internal/boring/sig) error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"crypto/internal/boring/sig": "crypto_internal_boring_sig"})
	if strings.Contains(rust, `Go function declaration has no body`) || strings.Contains(rust, "unimplemented!") {
		t.Fatalf("boring sig assembly no-op declarations should not use bodyless fallback:\n%s", rust)
	}
	for _, want := range []string{
		"pub fn boring_crypto() {\n}\n",
		"pub fn f_i_p_s_only() {\n}\n",
		"pub fn standard_crypto() {\n}\n",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing no-op %q in:\n%s", want, rust)
		}
	}
}

func TestInternalSyscallUnixARC4RandomUsesRuntimeBody(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	currentTypeInfo = &TypeInfo{pkg: types.NewPackage("internal/syscall/unix", "unix")}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})

	fset := token.NewFileSet()
	var out strings.Builder
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("ARC4Random"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{{
				Names: []*ast.Ident{ast.NewIdent("p")},
				Type:  &ast.ArrayType{Elt: ast.NewIdent("byte")},
			}}},
		},
		Body: &ast.BlockStmt{List: []ast.Stmt{&ast.ExprStmt{X: ast.NewIdent("sourceBodyShouldNotRun")}}},
	}, fset, nil)

	got := out.String()
	for _, bad := range []string{"sourceBodyShouldNotRun", "Go function declaration has no body", "syscall_syscall"} {
		if strings.Contains(got, bad) {
			t.Fatalf("internal/syscall/unix.ARC4Random should use the runtime body, found %q in:\n%s", bad, got)
		}
	}
	for _, bad := range []string{"wrapping_mul(31)", "wrapping_add(17)"} {
		if strings.Contains(got, bad) {
			t.Fatalf("internal/syscall/unix.ARC4Random should not synthesize deterministic bytes, found %q in:\n%s", bad, got)
		}
	}
	for _, want := range []string{
		`std::fs::File::open("/dev/urandom")`,
		"std::io::Read::read_exact(&mut __arc4_file, __arc4_bytes.as_mut_slice())",
		"internal/syscall/unix.ARC4Random failed to read /dev/urandom",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in ARC4Random runtime body:\n%s", want, got)
		}
	}
}

func TestTranspileSyscallRuntimeLinkedFunctionsUseHostEnv(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	currentTypeInfo = &TypeInfo{pkg: types.NewPackage("syscall", "syscall")}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})

	fset := token.NewFileSet()
	var out strings.Builder
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_envs"),
		Type: &ast.FuncType{
			Params:  &ast.FieldList{},
			Results: &ast.FieldList{List: []*ast.Field{{Type: &ast.ArrayType{Elt: ast.NewIdent("string")}}}},
		},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtimeSetenv"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{{
				Names: []*ast.Ident{ast.NewIdent("k"), ast.NewIdent("v")},
				Type:  ast.NewIdent("string"),
			}}},
		},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtimeUnsetenv"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{{
				Names: []*ast.Ident{ast.NewIdent("k")},
				Type:  ast.NewIdent("string"),
			}}},
		},
	}, fset, nil)

	got := out.String()
	if strings.Contains(got, "Go function declaration has no body") {
		t.Fatalf("syscall runtime-linked declarations should not use the generic bodyless fallback:\n%s", got)
	}
	for _, want := range []string{"std::env::vars()", "std::env::set_var(__key, __value)", "std::env::remove_var(__key)"} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestTranspileSyncRuntimeLinkedFunctionsUseLocalRuntimeBodies(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	currentTypeInfo = &TypeInfo{pkg: types.NewPackage("sync", "sync")}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})
	prevCD := GetConcurrencyDetector()
	cd := NewConcurrencyDetector()
	cd.hasGoroutines = true
	SetConcurrencyDetector(cd)
	t.Cleanup(func() {
		SetConcurrencyDetector(prevCD)
	})

	fset := token.NewFileSet()
	var out strings.Builder
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_registerPoolCleanup"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("cleanup")},
			Type:  &ast.FuncType{},
		}}}},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_procPin"),
		Type: &ast.FuncType{Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("int")}}}},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_procUnpin"),
		Type: &ast.FuncType{},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_LoadAcquintptr"),
		Type: &ast.FuncType{
			Params:  &ast.FieldList{List: []*ast.Field{{Names: []*ast.Ident{ast.NewIdent("ptr")}, Type: &ast.StarExpr{X: ast.NewIdent("uintptr")}}}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("uintptr")}}},
		},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_StoreReluintptr"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{
				{Names: []*ast.Ident{ast.NewIdent("ptr")}, Type: &ast.StarExpr{X: ast.NewIdent("uintptr")}},
				{Names: []*ast.Ident{ast.NewIdent("val")}, Type: ast.NewIdent("uintptr")},
			}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("uintptr")}}},
		},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_randn"),
		Type: &ast.FuncType{
			Params:  &ast.FieldList{List: []*ast.Field{{Names: []*ast.Ident{ast.NewIdent("n")}, Type: ast.NewIdent("uint32")}}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("uint32")}}},
		},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_Semacquire"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{{Names: []*ast.Ident{ast.NewIdent("s")}, Type: &ast.StarExpr{X: ast.NewIdent("uint32")}}}}},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_SemacquireWaitGroup"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{{Names: []*ast.Ident{ast.NewIdent("s")}, Type: &ast.StarExpr{X: ast.NewIdent("uint32")}}}}},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_SemacquireRWMutexR"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{
			{Names: []*ast.Ident{ast.NewIdent("s")}, Type: &ast.StarExpr{X: ast.NewIdent("uint32")}},
			{Names: []*ast.Ident{ast.NewIdent("lifo")}, Type: ast.NewIdent("bool")},
			{Names: []*ast.Ident{ast.NewIdent("skipframes")}, Type: ast.NewIdent("int")},
		}}},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_Semrelease"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{
			{Names: []*ast.Ident{ast.NewIdent("s")}, Type: &ast.StarExpr{X: ast.NewIdent("uint32")}},
			{Names: []*ast.Ident{ast.NewIdent("handoff")}, Type: ast.NewIdent("bool")},
			{Names: []*ast.Ident{ast.NewIdent("skipframes")}, Type: ast.NewIdent("int")},
		}}},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_notifyListAdd"),
		Type: &ast.FuncType{
			Params:  &ast.FieldList{List: []*ast.Field{{Names: []*ast.Ident{ast.NewIdent("l")}, Type: &ast.StarExpr{X: ast.NewIdent("notifyList")}}}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("uint32")}}},
		},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_notifyListWait"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{
			{Names: []*ast.Ident{ast.NewIdent("l")}, Type: &ast.StarExpr{X: ast.NewIdent("notifyList")}},
			{Names: []*ast.Ident{ast.NewIdent("t")}, Type: ast.NewIdent("uint32")},
		}}},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_notifyListNotifyAll"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{{Names: []*ast.Ident{ast.NewIdent("l")}, Type: &ast.StarExpr{X: ast.NewIdent("notifyList")}}}}},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_notifyListNotifyOne"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{{Names: []*ast.Ident{ast.NewIdent("l")}, Type: &ast.StarExpr{X: ast.NewIdent("notifyList")}}}}},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_notifyListCheck"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{{Names: []*ast.Ident{ast.NewIdent("size")}, Type: ast.NewIdent("uintptr")}}}},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("throw"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{{Names: []*ast.Ident{ast.NewIdent("message")}, Type: ast.NewIdent("string")}}}},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("fatal"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("string")}}}},
	}, fset, nil)

	got := out.String()
	if strings.Contains(got, "Go function declaration has no body") {
		t.Fatalf("sync runtime-linked declarations should not use the generic bodyless fallback:\n%s", got)
	}
	for _, want := range []string{
		"let _ = cleanup;",
		"0\n}",
		"let _ = n;",
		"let __value = (*ptr.lock().unwrap().as_ref().unwrap()).clone();",
		"*ptr.lock().unwrap().as_mut().unwrap() = __stored;",
		"*__sem -= 1;",
		"std::thread::yield_now();",
		"*__sem = __sem.saturating_add(1);",
		"let _ = l;",
		"let _ = t;",
		"let _ = size;",
		"let __message = { let __arg_holder = __arg0.clone();",
		"panic!(\"{}\", __message);",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestTranspileInternalSyncRuntimeRandUsesLocalRuntimeBody(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	currentTypeInfo = &TypeInfo{pkg: types.NewPackage("internal/sync", "sync")}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})

	var out strings.Builder
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("runtime_rand"),
		Type: &ast.FuncType{Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("uint64")}}}},
	}, token.NewFileSet(), nil)

	got := out.String()
	if strings.Contains(got, "Go function declaration has no body") {
		t.Fatalf("internal/sync runtime_rand should not use the generic bodyless fallback:\n%s", got)
	}
	if !strings.Contains(got, "1u64") {
		t.Fatalf("internal/sync runtime_rand should emit a deterministic local body:\n%s", got)
	}
}

func TestTranspileSyncAtomicRuntimeIntrinsicsUseWrappedAddressBodies(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	currentTypeInfo = &TypeInfo{pkg: types.NewPackage("sync/atomic", "atomic")}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})
	prevCD := GetConcurrencyDetector()
	cd := NewConcurrencyDetector()
	cd.hasGoroutines = true
	SetConcurrencyDetector(cd)
	t.Cleanup(func() {
		SetConcurrencyDetector(prevCD)
	})

	field := func(name string, typ ast.Expr) *ast.Field {
		return &ast.Field{Names: []*ast.Ident{ast.NewIdent(name)}, Type: typ}
	}
	ptrTo := func(name string) ast.Expr {
		return &ast.StarExpr{X: ast.NewIdent(name)}
	}
	result := func(name string) *ast.FieldList {
		return &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent(name)}}}
	}
	fn := func(name string, params []*ast.Field, results *ast.FieldList) *ast.FuncDecl {
		return &ast.FuncDecl{
			Name: ast.NewIdent(name),
			Type: &ast.FuncType{Params: &ast.FieldList{List: params}, Results: results},
		}
	}

	fset := token.NewFileSet()
	var out strings.Builder
	for _, decl := range []*ast.FuncDecl{
		fn("LoadUint32", []*ast.Field{field("addr", ptrTo("uint32"))}, result("uint32")),
		fn("StoreUint32", []*ast.Field{field("addr", ptrTo("uint32")), field("val", ast.NewIdent("uint32"))}, nil),
		fn("SwapUint32", []*ast.Field{field("addr", ptrTo("uint32")), field("new", ast.NewIdent("uint32"))}, result("uint32")),
		fn("CompareAndSwapUint32", []*ast.Field{field("addr", ptrTo("uint32")), field("old", ast.NewIdent("uint32")), field("new", ast.NewIdent("uint32"))}, result("bool")),
		fn("AddUint32", []*ast.Field{field("addr", ptrTo("uint32")), field("delta", ast.NewIdent("uint32"))}, result("uint32")),
		fn("AndUint32", []*ast.Field{field("addr", ptrTo("uint32")), field("mask", ast.NewIdent("uint32"))}, result("uint32")),
		fn("OrUint32", []*ast.Field{field("addr", ptrTo("uint32")), field("mask", ast.NewIdent("uint32"))}, result("uint32")),
		fn("LoadPointer", []*ast.Field{field("addr", ptrTo("uintptr"))}, result("uintptr")),
		fn("StorePointer", []*ast.Field{field("addr", ptrTo("uintptr")), field("val", ast.NewIdent("uintptr"))}, nil),
		fn("CompareAndSwapPointer", []*ast.Field{field("addr", ptrTo("uintptr")), field("old", ast.NewIdent("uintptr")), field("new", ast.NewIdent("uintptr"))}, result("bool")),
	} {
		TranspileFunction(&out, decl, fset, nil)
	}

	got := out.String()
	if strings.Contains(got, "Go function declaration has no body") {
		t.Fatalf("sync/atomic runtime intrinsics should not use the generic bodyless fallback:\n%s", got)
	}
	for _, want := range []string{
		"let mut __guard = addr.lock().unwrap();",
		"*__guard.as_mut().unwrap() = __new;",
		"__current.wrapping_add(__delta)",
		"*__guard.as_mut().unwrap() &= __mask;",
		"*__guard.as_mut().unwrap() |= __mask;",
		"if *__guard.as_ref().unwrap() == __old",
		"as_ref().copied().unwrap_or(0)",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestTranspileSyncAtomicPointerUsesTypedHandleSlot(t *testing.T) {
	const src = `package atomic

import "unsafe"

type noCopy struct{}

type Pointer[T any] struct {
	_ [0]*T
	_ noCopy
	v unsafe.Pointer
}

func LoadPointer(addr *unsafe.Pointer) (val unsafe.Pointer)
func StorePointer(addr *unsafe.Pointer, val unsafe.Pointer)
func SwapPointer(addr *unsafe.Pointer, new unsafe.Pointer) (old unsafe.Pointer)
func CompareAndSwapPointer(addr *unsafe.Pointer, old, new unsafe.Pointer) (swapped bool)

func (x *Pointer[T]) Load() *T { return (*T)(LoadPointer(&x.v)) }
func (x *Pointer[T]) Store(val *T) { StorePointer(&x.v, unsafe.Pointer(val)) }
func (x *Pointer[T]) Swap(new *T) (old *T) { return (*T)(SwapPointer(&x.v, unsafe.Pointer(new))) }
func (x *Pointer[T]) CompareAndSwap(old, new *T) (swapped bool) {
	return CompareAndSwapPointer(&x.v, unsafe.Pointer(old), unsafe.Pointer(new))
}
`
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "type.go", src, 0)
	if err != nil {
		t.Fatalf("ParseFile(type.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("sync/atomic", []*ast.File{file}, fset, importer.Default())
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(sync/atomic) error = %v", err)
	}
	prevCD := GetConcurrencyDetector()
	cd := NewConcurrencyDetector()
	cd.hasGoroutines = true
	SetConcurrencyDetector(cd)
	t.Cleanup(func() {
		SetConcurrencyDetector(prevCD)
	})

	rust := transpileParsedRegression(t, file, fset, typeInfo)

	for _, forbidden := range []string{
		"pub v: Arc<Mutex<Option<usize>>>",
		"pub v: Arc<Mutex<Option<Arc<Mutex<Option<T>>>>>>",
		"unsafe.Pointer conversion to T",
		"load_pointer(self.v.clone())",
		"store_pointer(self.v.clone()",
		"swap_pointer(self.v.clone()",
		"compare_and_swap_pointer(self.v.clone()",
		"val.lock().unwrap().is_some()",
		"new.lock().unwrap().is_some()",
		"Arc::ptr_eq(__current, &old)",
		"pub struct Pointer<T: Any + Clone + Send + Sync + 'static>",
		"pub fn load(&self) -> Arc<Mutex<Option<T>>>",
		"pub fn swap(&self, new: Arc<Mutex<Option<T>>>) -> Arc<Mutex<Option<T>>>",
		"pub fn compare_and_swap(&self, old: Arc<Mutex<Option<T>>>, new: Arc<Mutex<Option<T>>>) -> bool",
	} {
		if strings.Contains(rust, forbidden) {
			t.Fatalf("sync/atomic Pointer should use a typed pointer-handle slot, found %q:\n%s", forbidden, rust)
		}
	}
	for _, want := range []string{
		"pub struct Pointer<T: Any + Send + Sync + 'static>",
		"pub v: Arc<Mutex<Option<GoPtr<T>>>>",
		"v: Arc::new(Mutex::new(None))",
		"__guard.as_ref().cloned().unwrap_or_else(|| GoPtr::nil())",
		"let __stored = if val.is_nil() { None } else { Some(val.clone()) };",
		"let __stored = if new.is_nil() { None } else { Some(new.clone()) };",
		"GoPtr::ptr_eq(__current, &old)",
		"pub fn load(&self) -> GoPtr<T>",
		"pub fn swap(&self, new: GoPtr<T>) -> GoPtr<T>",
		"pub fn compare_and_swap(&self, old: GoPtr<T>, new: GoPtr<T>) -> bool",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in sync/atomic Pointer lowering:\n%s", want, rust)
		}
	}
}

func TestNamedTypeOverImportedNamedStructImplementsDisplay(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "sync/atomic"

type profileStateHolder atomic.Uint32

type goroutine struct {
	profiled profileStateHolder
}
`)

	if !strings.Contains(rust, "impl Display for profileStateHolder") &&
		!strings.Contains(rust, "impl std::fmt::Display for profileStateHolder") {
		t.Fatalf("defined type over an imported named struct should implement Display:\n%s", rust)
	}
	if !strings.Contains(rust, `write!(f, "{}", self.0`) {
		t.Fatalf("defined type over an imported named struct should delegate Display to the inner named value:\n%s", rust)
	}
}

func TestTranspileInternalABIFuncPCIntrinsicsUseFunctionTypeID(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	currentTypeInfo = &TypeInfo{pkg: types.NewPackage("internal/abi", "abi")}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})

	fset := token.NewFileSet()
	param := &ast.FieldList{List: []*ast.Field{{
		Names: []*ast.Ident{ast.NewIdent("f")},
		Type:  &ast.InterfaceType{Methods: &ast.FieldList{}},
	}}}
	result := &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("uintptr")}}}

	var out strings.Builder
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("FuncPCABI0"),
		Type: &ast.FuncType{Params: param, Results: result},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("FuncPCABIInternal"),
		Type: &ast.FuncType{Params: param, Results: result},
	}, fset, nil)

	got := out.String()
	if strings.Contains(got, "Go function declaration has no body") {
		t.Fatalf("internal/abi FuncPC intrinsics should not use the generic bodyless fallback:\n%s", got)
	}
	for _, want := range []string{
		"std::any::Any::type_id(__value.as_ref())",
		"std::hash::Hash::hash",
		"std::hash::Hasher::finish(&__hasher) as usize",
		"internal/abi.FuncPCABI0 requires a function value",
		"internal/abi.FuncPCABIInternal requires a function value",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestTranspileInternalABITypeOfUsesRuntimeTypeIntrinsic(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	currentTypeInfo = &TypeInfo{pkg: types.NewPackage("internal/abi", "abi")}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})

	fn := &ast.FuncDecl{
		Name: ast.NewIdent("TypeOf"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{{
				Names: []*ast.Ident{ast.NewIdent("a")},
				Type:  &ast.InterfaceType{Methods: &ast.FieldList{}},
			}}},
			Results: &ast.FieldList{List: []*ast.Field{{
				Type: &ast.StarExpr{X: ast.NewIdent("Type")},
			}}},
		},
		Body: &ast.BlockStmt{Lbrace: token.Pos(1), Rbrace: token.Pos(2)},
	}

	var out strings.Builder
	TranspileFunction(&out, fn, token.NewFileSet(), nil)
	got := out.String()
	if strings.Contains(got, "unsafe.Pointer conversion to EmptyInterface") ||
		strings.Contains(got, "unsafe.Pointer conversion to Type") {
		t.Fatalf("internal/abi.TypeOf should use a Rust runtime type intrinsic, not Go interface layout casts:\n%s", got)
	}
	for _, want := range []string{
		"<dyn std::any::Any>::is::<String>",
		"<dyn std::any::Any>::is::<char>",
		"<dyn std::any::Any>::is::<Vec<String>>",
		"<dyn std::any::Any>::is::<Vec<Box<dyn Any>>>",
		"<dyn std::any::Any>::is::<Box<dyn Any>>",
		"<dyn std::any::Any>::is::<Rc<RefCell<Option<Box<dyn Any>>>>",
		"<dyn std::any::Any>::is::<Rc<RefCell<Option<Vec<Box<dyn Any>>>>>",
		"std::mem::size_of_val(__value)",
		"std::any::type_name_of_val(__value)",
		"internal/abi.TypeOf unsupported Rust Any payload",
		"if let Some(__go_meta) = __go_any_metadata { if __go_meta.kind == \"pointer\"",
		"let mut __ptr_type = PtrType::default()",
		"if let Some(__go_elem_kind) = __go_meta.elem_kind",
		"go_register_embedded_owner(__embedded_key, __owner.clone())",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestGoPtrStructFieldUsesCurrentPackageModuleForCachedSamePathElem(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "iface.go", `package abi

type Type struct{}

type EmptyInterface struct {
	Type *Type
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("example.com/abi", []*ast.File{file}, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter() error = %v", err)
	}

	var emptySpec *ast.TypeSpec
	for _, decl := range file.Decls {
		gen, ok := decl.(*ast.GenDecl)
		if !ok || gen.Tok != token.TYPE {
			continue
		}
		for _, spec := range gen.Specs {
			typeSpec := spec.(*ast.TypeSpec)
			if typeSpec.Name.Name == "EmptyInterface" {
				emptySpec = typeSpec
			}
		}
	}
	if emptySpec == nil {
		t.Fatalf("test fixture missing EmptyInterface type")
	}

	typeObj := typeInfo.pkg.Scope().Lookup("Type").(*types.TypeName)
	emptyObj := typeInfo.pkg.Scope().Lookup("EmptyInterface").(*types.TypeName)
	key := sliceElemPtrFieldKey(emptyObj.Type().(*types.Named), "Type")

	prevTypeInfo := currentTypeInfo
	prevCtx := GetTranspileContext()
	defer func() {
		currentTypeInfo = prevTypeInfo
		SetTranspileContext(prevCtx)
	}()
	SetTranspileContext(&TranspileContext{
		CurrentModuleName: "iface",
		Package: &PackageState{
			TypeModuleNames: map[string]string{
				"Type":           "r#type",
				"EmptyInterface": "iface",
			},
			SliceElemPtrFields: map[string]sliceElemPtrFieldInfo{
				key: {
					elemRustType: "example_com_abi::r#type::Type",
					elemType:     typeObj.Type(),
				},
			},
		},
		Session: &TranspileSession{TypeInfo: typeInfo},
	})
	SetTypeInfo(typeInfo)

	var out strings.Builder
	emitStructTypeDeclBody(&out, emptySpec, emptySpec.Type.(*ast.StructType))
	got := out.String()
	if strings.Contains(got, "example_com_abi::") {
		t.Fatalf("GoPtr field should not reuse cached external crate type inside declaring package:\n%s", got)
	}
	if !strings.Contains(got, "pub r#type: GoPtr<crate::r#type::Type>") {
		t.Fatalf("GoPtr field should render the current package sibling module type:\n%s", got)
	}
}

func TestTranspileInternalBuildcfgExpListUsesTypedGoexperimentFields(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	currentPkg := types.NewPackage("internal/buildcfg", "buildcfg")
	flagsPkg := types.NewPackage("internal/goexperiment", "goexperiment")
	flagsStruct := types.NewStruct([]*types.Var{
		types.NewField(token.NoPos, flagsPkg, "FieldTrack", types.Typ[types.Bool], false),
		types.NewField(token.NoPos, flagsPkg, "RegabiArgs", types.Typ[types.Bool], false),
	}, nil)
	flagsNamed := types.NewNamed(types.NewTypeName(token.NoPos, flagsPkg, "Flags", nil), flagsStruct, nil)

	fnName := ast.NewIdent("expList")
	sig := types.NewSignatureType(nil, nil, nil,
		types.NewTuple(
			types.NewVar(token.NoPos, currentPkg, "exp", types.NewPointer(flagsNamed)),
			types.NewVar(token.NoPos, currentPkg, "base", types.NewPointer(flagsNamed)),
			types.NewVar(token.NoPos, currentPkg, "all", types.Typ[types.Bool]),
		),
		types.NewTuple(types.NewVar(token.NoPos, currentPkg, "", types.NewSlice(types.Typ[types.String]))),
		false,
	)
	currentTypeInfo = &TypeInfo{
		info: &types.Info{Defs: map[*ast.Ident]types.Object{
			fnName: types.NewFunc(token.NoPos, currentPkg, "expList", sig),
		}},
		pkg: currentPkg,
	}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})

	fn := &ast.FuncDecl{
		Name: fnName,
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{
				{Names: []*ast.Ident{ast.NewIdent("exp"), ast.NewIdent("base")}, Type: &ast.StarExpr{X: ast.NewIdent("Flags")}},
				{Names: []*ast.Ident{ast.NewIdent("all")}, Type: ast.NewIdent("bool")},
			}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: &ast.ArrayType{Elt: ast.NewIdent("string")}}}},
		},
		Body: &ast.BlockStmt{Lbrace: token.Pos(1), List: []ast.Stmt{
			&ast.ExprStmt{X: &ast.CallExpr{Fun: ast.NewIdent("panic"), Args: []ast.Expr{&ast.BasicLit{Kind: token.STRING, Value: `"body should be replaced"`}}}},
		}, Rbrace: token.Pos(2)},
	}

	var out strings.Builder
	TranspileFunction(&out, fn, token.NewFileSet(), nil)
	got := out.String()
	if strings.Contains(got, "reflect::value_of") || strings.Contains(got, "body should be replaced") {
		t.Fatalf("internal/buildcfg.expList should lower from go/types fields, not its reflect body:\n%s", got)
	}
	for _, want := range []string{
		".field_track",
		"\"fieldtrack\".to_string()",
		"\"noregabiargs\".to_string()",
		"internal/buildcfg.expList requires exp flags",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestTranspileInternalGodebugRuntimeLinknamesUseHostRuntime(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	currentTypeInfo = &TypeInfo{pkg: types.NewPackage("internal/godebug", "godebug")}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})

	stringIdent := ast.NewIdent("string")
	var out strings.Builder
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("setUpdate"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("update")},
			Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{
				{Type: stringIdent},
				{Type: stringIdent},
			}}},
		}}}},
	}, token.NewFileSet(), nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("setNewIncNonDefault"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("newIncNonDefault")},
			Type: &ast.FuncType{
				Params:  &ast.FieldList{List: []*ast.Field{{Type: stringIdent}}},
				Results: &ast.FieldList{List: []*ast.Field{{Type: &ast.FuncType{Params: &ast.FieldList{}}}}},
			},
		}}}},
	}, token.NewFileSet(), nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("registerMetric"),
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{
			{Names: []*ast.Ident{ast.NewIdent("name")}, Type: stringIdent},
			{Names: []*ast.Ident{ast.NewIdent("read")}, Type: &ast.FuncType{Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("uint64")}}}}},
		}}},
	}, token.NewFileSet(), nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("write"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{
				{Names: []*ast.Ident{ast.NewIdent("fd")}, Type: ast.NewIdent("uintptr")},
				{Names: []*ast.Ident{ast.NewIdent("p")}, Type: ast.NewIdent("uintptr")},
				{Names: []*ast.Ident{ast.NewIdent("n")}, Type: ast.NewIdent("int32")},
			}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("int32")}}},
		},
	}, token.NewFileSet(), nil)

	got := out.String()
	if strings.Contains(got, "Go function declaration has no body") {
		t.Fatalf("internal/godebug runtime linknames should not use the generic bodyless fallback:\n%s", got)
	}
	for _, want := range []string{
		"std::env::var(\"GODEBUG\")",
		"if !__env.is_empty()",
		"let _ = newIncNonDefault",
		"let _ = (name, read)",
		"std::io::stderr()",
		"std::slice::from_raw_parts",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestTranspileStringsBuilderRuntimeUnsafeIntrinsics(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	stringsPkg := types.NewPackage("strings", "strings")
	builderNamed := types.NewNamed(types.NewTypeName(token.NoPos, stringsPkg, "Builder", nil), types.NewStruct(nil, nil), nil)
	builderRecv := types.NewVar(token.NoPos, stringsPkg, "b", types.NewPointer(builderNamed))
	copyCheckName := ast.NewIdent("copyCheck")
	stringName := ast.NewIdent("String")
	currentTypeInfo = &TypeInfo{
		info: &types.Info{Defs: map[*ast.Ident]types.Object{
			copyCheckName: types.NewFunc(token.NoPos, stringsPkg, "copyCheck", types.NewSignatureType(builderRecv, nil, nil, nil, nil, false)),
			stringName:    types.NewFunc(token.NoPos, stringsPkg, "String", types.NewSignatureType(builderRecv, nil, nil, nil, types.NewTuple(types.NewVar(token.NoPos, stringsPkg, "", types.Typ[types.String])), false)),
		}},
		pkg: stringsPkg,
	}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})

	recv := &ast.FieldList{List: []*ast.Field{{
		Names: []*ast.Ident{ast.NewIdent("b")},
		Type:  &ast.StarExpr{X: ast.NewIdent("Builder")},
	}}}
	copyCheck := &ast.FuncDecl{
		Recv: recv,
		Name: copyCheckName,
		Type: &ast.FuncType{Params: &ast.FieldList{}},
		Body: &ast.BlockStmt{Lbrace: token.Pos(1), List: []ast.Stmt{
			&ast.AssignStmt{
				Lhs: []ast.Expr{&ast.SelectorExpr{X: ast.NewIdent("b"), Sel: ast.NewIdent("addr")}},
				Tok: token.ASSIGN,
				Rhs: []ast.Expr{ast.NewIdent("b")},
			},
		}, Rbrace: token.Pos(2)},
	}
	stringMethod := &ast.FuncDecl{
		Recv: recv,
		Name: stringName,
		Type: &ast.FuncType{
			Params:  &ast.FieldList{},
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("string")}}},
		},
		Body: &ast.BlockStmt{Lbrace: token.Pos(3), List: []ast.Stmt{
			&ast.ReturnStmt{Results: []ast.Expr{&ast.BasicLit{Kind: token.STRING, Value: `"body should be replaced"`}}},
		}, Rbrace: token.Pos(4)},
	}

	var out strings.Builder
	TranspileMethodImpl(&out, copyCheck, token.NewFileSet(), nil)
	TranspileMethodImpl(&out, stringMethod, token.NewFileSet(), nil)

	got := out.String()
	for _, bad := range []string{
		"unsafe.Pointer conversion to Builder",
		"unsafe.String requires unsafe intrinsic support",
		"body should be replaced",
	} {
		if strings.Contains(got, bad) {
			t.Fatalf("strings.Builder runtime intrinsics should replace unsafe Go bodies, found %q in:\n%s", bad, got)
		}
	}
	for _, want := range []string{
		"pub fn copy_check(&mut self)",
		"let _ = self;",
		"pub fn string(&self) -> Rc<RefCell<Option<String>>>",
		"String::from_utf8_lossy(__buf).to_string()",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestTranspileInternalBytealgCountIntrinsicsUseGenericBodies(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	currentTypeInfo = &TypeInfo{pkg: types.NewPackage("internal/bytealg", "bytealg")}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})

	fset := token.NewFileSet()
	var out strings.Builder
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("CountString"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{
				{Names: []*ast.Ident{ast.NewIdent("s")}, Type: ast.NewIdent("string")},
				{Names: []*ast.Ident{ast.NewIdent("c")}, Type: ast.NewIdent("byte")},
			}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("int")}}},
		},
	}, fset, nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("Count"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{
				{Names: []*ast.Ident{ast.NewIdent("b")}, Type: &ast.ArrayType{Elt: ast.NewIdent("byte")}},
				{Names: []*ast.Ident{ast.NewIdent("c")}, Type: ast.NewIdent("byte")},
			}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("int")}}},
		},
	}, fset, nil)

	got := out.String()
	if strings.Contains(got, "Go function declaration has no body") {
		t.Fatalf("internal/bytealg count intrinsics should not use the generic bodyless fallback:\n%s", got)
	}
	for _, want := range []string{
		".as_bytes().iter().filter",
		".iter().filter",
		".count()).unwrap_or(0) as i32",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestTranspileInternalBytealgMakeNoZeroUsesRuntimeAllocation(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	currentTypeInfo = &TypeInfo{pkg: types.NewPackage("internal/bytealg", "bytealg")}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})

	var out strings.Builder
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("MakeNoZero"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{{
				Names: []*ast.Ident{ast.NewIdent("n")},
				Type:  ast.NewIdent("int"),
			}}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: &ast.ArrayType{Elt: ast.NewIdent("byte")}}}},
		},
	}, token.NewFileSet(), nil)

	got := out.String()
	if strings.Contains(got, "Go function declaration has no body") {
		t.Fatalf("internal/bytealg.MakeNoZero should use a runtime allocation body:\n%s", got)
	}
	for _, want := range []string{
		"let __n = (*n",
		"internal/bytealg.MakeNoZero: negative length",
		"let __len = __n as usize;",
		"vec![0u8; __len]",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestTranspileInternalBytealgNativeStringIntrinsics(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	currentTypeInfo = &TypeInfo{pkg: types.NewPackage("internal/bytealg", "bytealg")}
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
	})

	stringIdent := ast.NewIdent("string")
	byteIdent := ast.NewIdent("byte")
	var out strings.Builder
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("IndexByteString"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{
				{Names: []*ast.Ident{ast.NewIdent("s")}, Type: stringIdent},
				{Names: []*ast.Ident{ast.NewIdent("c")}, Type: byteIdent},
			}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("int")}}},
		},
	}, token.NewFileSet(), nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("IndexString"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{
				{Names: []*ast.Ident{ast.NewIdent("a")}, Type: stringIdent},
				{Names: []*ast.Ident{ast.NewIdent("b")}, Type: stringIdent},
			}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("int")}}},
		},
	}, token.NewFileSet(), nil)
	TranspileFunction(&out, &ast.FuncDecl{
		Name: ast.NewIdent("abigen_runtime_cmpstring"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{
				{Names: []*ast.Ident{ast.NewIdent("a")}, Type: stringIdent},
				{Names: []*ast.Ident{ast.NewIdent("b")}, Type: stringIdent},
			}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("int")}}},
		},
	}, token.NewFileSet(), nil)

	got := out.String()
	if strings.Contains(got, "Go function declaration has no body") {
		t.Fatalf("internal/bytealg native string intrinsics should not use the generic bodyless fallback:\n%s", got)
	}
	for _, want := range []string{
		"__s.as_bytes().iter().position",
		"__a.find(&__b).map",
		"match __a.cmp(&__b)",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestTranspileFunctionWithoutBodyNamesUnnamedParams(t *testing.T) {
	var out strings.Builder
	fn := &ast.FuncDecl{
		Name: ast.NewIdent("externalFunc"),
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{
				{Type: ast.NewIdent("int")},
				{Type: ast.NewIdent("string")},
			}},
			Results: &ast.FieldList{
				List: []*ast.Field{{Type: ast.NewIdent("bool")}},
			},
		},
	}

	TranspileFunction(&out, fn, token.NewFileSet(), nil)

	got := out.String()
	if strings.Contains(got, "(,") || strings.Contains(got, ", )") {
		t.Fatalf("unnamed parameters should not leave empty Rust signature slots:\n%s", got)
	}
	if !strings.Contains(got, "__arg0: Rc<RefCell<Option<i32>>>") {
		t.Fatalf("first unnamed parameter should get a synthetic Rust name:\n%s", got)
	}
	if !strings.Contains(got, "__arg1: Rc<RefCell<Option<String>>>") {
		t.Fatalf("second unnamed parameter should get a synthetic Rust name:\n%s", got)
	}
}

func TestStructWithSourceMappedStdlibFieldDoesNotDeriveDebug(t *testing.T) {
	prevContext := GetTranspileContext()
	prevTypeInfo := currentTypeInfo
	prevImports := goPackageImports
	defer func() {
		SetTranspileContext(prevContext)
		currentTypeInfo = prevTypeInfo
		goPackageImports = prevImports
	}()

	selector := &ast.SelectorExpr{X: ast.NewIdent("abi"), Sel: ast.NewIdent("SwissMapType")}
	fieldType := &ast.StarExpr{X: selector}
	structType := &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{{
		Names: []*ast.Ident{ast.NewIdent("typ")},
		Type:  fieldType,
	}}}}

	abiPkg := types.NewPackage("internal/abi", "abi")
	currentPkg := types.NewPackage("internal/runtime/maps", "maps")
	named := types.NewNamed(types.NewTypeName(token.NoPos, abiPkg, "SwissMapType", nil), types.NewStruct(nil, nil), nil)
	SetTypeInfo(&TypeInfo{
		info: &types.Info{Types: map[ast.Expr]types.TypeAndValue{
			selector:  {Type: named},
			fieldType: {Type: types.NewPointer(named)},
		}},
		pkg: currentPkg,
	})
	SetTranspileContext(&TranspileContext{PackageMapping: map[string]string{"internal/abi": "internal_abi"}})
	goPackageImports = map[string]string{"abi": "internal/abi"}

	var out strings.Builder
	writeStructDerive(&out, "Iter", structType, true)
	got := out.String()
	if strings.Contains(got, "Debug") {
		t.Fatalf("struct with source-mapped stdlib field should not derive Debug:\n%s", got)
	}
	if !strings.Contains(got, "Clone") {
		t.Fatalf("struct with source-mapped stdlib field should still derive Clone:\n%s", got)
	}
}

func TestStructWithForwardNamedNonDebugFieldDoesNotDeriveDebug(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Union struct {
	terms []*Term
}

type Term term

type Type interface {
	Underlying() Type
}

type term struct {
	typ Type
}
`)

	if strings.Contains(rust, "#[derive(Debug, Clone, Default)]\npub struct Union") {
		t.Fatalf("struct with forward named non-Debug field should not derive Debug:\n%s", rust)
	}
	if !strings.Contains(rust, "#[derive(Clone, Default)]\npub struct Union") {
		t.Fatalf("struct with forward named non-Debug field should still derive Clone and Default:\n%s", rust)
	}
}

func TestDefinedStructTypeUsedAsGoValueCloneGenericArgImplementsTrait(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Term term

type Type interface {
	Underlying() Type
}

type term struct {
	typ Type
}

func first[T any](values []T) T {
	return values[0]
}

func use(terms []Term) Term {
	return first(terms)
}
`)

	if !strings.Contains(rust, "pub fn first<T: Any + GoValueClone + 'static>") {
		t.Fatalf("generic helper returning a direct type-param element should require GoValueClone:\n%s", rust)
	}
	want := "impl GoValueClone for Term {\n    fn go_value_clone(&self) -> Self {\n        self.__go_value_clone()\n    }\n}"
	if !strings.Contains(rust, want) {
		t.Fatalf("defined struct type used as a GoValueClone generic argument should implement the trait, missing %q:\n%s", want, rust)
	}
}

func TestStructValueCloneBuildsLargeStructWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type node struct {
	a int
	b string
	next *node
	values []int
	flags [2]bool
}

var a int

func use(n node) node {
	go func() {}()
	return n
}
`)

	if strings.Contains(rust, "Self { a: { let __guard = self.a.lock().unwrap();") {
		t.Fatalf("struct value clone should not emit every field clone in one struct literal expression:\n%s", rust)
	}
	if strings.Contains(rust, "let a =") {
		t.Fatalf("struct value clone local should not shadow same-named package globals:\n%s", rust)
	}
	for _, want := range []string{
		"let __go_clone_0_0 = { let __guard = self.a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };",
		"let __go_clone_1_0 = { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };",
		"let __go_clone_2_0 = self.next.clone();",
		"let __go_clone_3_0 = self.values.clone();",
		"let __go_clone_4_0 = { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };",
		"Self {\n            a: __go_clone_0_0,\n            b: __go_clone_1_0,\n            next: __go_clone_2_0,\n            values: __go_clone_3_0,\n            flags: __go_clone_4_0,\n        }",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("struct value clone should build fields with statements, missing %q:\n%s", want, rust)
		}
	}
}

func TestStructDisplayBuildsFieldStringsWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type displayNode struct {
	a int
	b string
	next *displayNode
	values []int
	flags [2]bool
}

func use(n displayNode) {
	go func() {}()
	_ = n
}
`)

	if strings.Contains(rust, "write!(f, \"{{{} {} {} {} {}}}\", (*self.a.lock().unwrap().as_ref().unwrap())") {
		t.Fatalf("struct Display should not embed every field expression in one write! call:\n%s", rust)
	}
	for _, want := range []string{
		"let __go_fmt_0 = format!(\"{}\", (*self.a.lock().unwrap().as_ref().unwrap()));",
		"let __go_fmt_1 = format!(\"{}\", (*self.b.lock().unwrap().as_ref().unwrap()));",
		"let __go_fmt_2 = format!(\"{}\", { let __guard = self.next.lock().unwrap(); match __guard.as_ref()",
		"let __go_fmt_3 = format!(\"{}\", format_slice(&self.values));",
		"let __go_fmt_4 = format!(\"{}\", format_slice(&self.flags));",
		"write!(f, \"{{{} {} {} {} {}}}\", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("struct Display should build formatted fields with statements, missing %q:\n%s", want, rust)
		}
	}
}

func TestStructAliasUsedAsGoValueCloneGenericArgDoesNotEmitAliasTraitImpl(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Alias = Target

type Target struct {
	name string
}

func first[T any](values []T) T {
	return values[0]
}

func use(values []Alias) Alias {
	return first(values)
}
`)

	if strings.Contains(rust, "impl GoValueClone for Alias") {
		t.Fatalf("struct alias should not get its own GoValueClone impl because it does not emit a local Rust struct:\n%s", rust)
	}
}

func TestStructDefaultUsesBareDefaultForChannelField(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type worker struct {
	done chan int
	name string
}

func zero() worker {
	return worker{}
}
`)

	if strings.Contains(rust, "done: Rc::new(RefCell::new(None))") ||
		strings.Contains(rust, "done: Arc::new(Mutex::new(None))") {
		t.Fatalf("struct Default should not wrap channel fields in Option handles:\n%s", rust)
	}
	if !strings.Contains(rust, "done: Default::default()") {
		t.Fatalf("struct Default should use the channel default directly:\n%s", rust)
	}
}

func TestConcurrentMapKeyStructWithInterfaceFieldUsesTraitEquality(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), `module example.com/mainmod

go 1.22
`)
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type Node interface {
	String() string
}

type visit struct {
	addr uintptr
	typ  Node
}

func seen(t Node) bool {
	go func() {}()
	visited := map[visit]bool{}
	v := visit{0, t}
	return visited[v]
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	rust := mustReadFile(t, filepath.Join(tempDir, "main.rs"))

	bad := "self.typ.lock().unwrap(); let __right = other.typ.lock().unwrap(); __left.as_ref() == __right.as_ref()"
	if strings.Contains(rust, bad) {
		t.Fatalf("interface field equality should use the interface equality helper, not trait object ==:\n%s", rust)
	}
	if !strings.Contains(rust, "__left.as_ref().__go_eq_node(__right.as_ref())") {
		t.Fatalf("interface field equality should call the typed interface equality helper:\n%s", rust)
	}
	if !strings.Contains(rust, "format!(\"{}\", __left.as_ref()).cmp(&format!(\"{}\", __right.as_ref()))") {
		t.Fatalf("interface field ordering should use an orderable trait-object key:\n%s", rust)
	}
}

func TestConcurrentComparableStructWithInterfaceFieldGetsCustomPartialEq(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Type interface {
	String() string
}

type Pointer struct {
	base Type
}

func used(p *Pointer) bool {
	go func() {}()
	return *p != (Pointer{})
}
`)

	if !strings.Contains(rust, "impl PartialEq for Pointer") {
		t.Fatalf("comparable struct with interface field should get custom PartialEq:\n%s", rust)
	}
	if strings.Contains(rust, "#[derive(Clone, Default, PartialEq)]") {
		t.Fatalf("comparable struct with interface field should not derive PartialEq over trait fields:\n%s", rust)
	}
	if !strings.Contains(rust, "__left.as_ref().__go_eq_type_(__right.as_ref())") {
		t.Fatalf("comparable struct with interface field should use interface equality helper:\n%s", rust)
	}
}

func TestTranspileGenericInterfaceConstrainedFunctionEmitsRustTypeParam(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Node interface {
	Pos() int
}

func Use(node Node) {}

func VisitAll[N Node](list []N) {
	for _, node := range list {
		Use(node)
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
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)

	if !strings.Contains(rust, "pub fn visit_all<N: Node + Clone") {
		t.Fatalf("generic interface-constrained function should emit a Rust type parameter bound:\n%s", rust)
	}
	if !strings.Contains(rust, "Vec<Rc<RefCell<Option<N>>>>") {
		t.Fatalf("slice of interface-constrained type parameter should use wrapped elements:\n%s", rust)
	}
	if strings.Contains(rust, "Vec<N>") {
		t.Fatalf("slice of interface-constrained type parameter should not emit unwrapped Vec<N>:\n%s", rust)
	}
}

func TestGenericComparableSliceFunctionUsesWrappedElements(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func substList[T comparable](in []T, subst func(T) T) []T {
	for i, t := range in {
		if u := subst(t); u != t {
			out := make([]T, len(in))
			copy(out, in)
			out[i] = u
			return out
		}
	}
	return nil
}
`)

	if !strings.Contains(rust, "Vec<Rc<RefCell<Option<T>>>>") {
		t.Fatalf("generic []T should store wrapped element handles:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn subst_list<T: Any + GoComparable + GoValueClone + 'static>") ||
		strings.Contains(rust, "PartialEq") {
		t.Fatalf("generic comparable type parameter should not require raw Rust PartialEq:\n%s", rust)
	}
	if !strings.Contains(rust, "GoComparable::go_eq(__left_value, __right_value)") {
		t.Fatalf("generic comparable handle values should use GoComparable value comparison:\n%s", rust)
	}
	if strings.Contains(rust, "Vec<T>") {
		t.Fatalf("generic []T should not store bare type parameters:\n%s", rust)
	}
	if strings.Contains(rust, "Some(t)") || strings.Contains(rust, "Some(t.clone())") {
		t.Fatalf("range value passed to func(T) should not be wrapped again:\n%s", rust)
	}
	if !strings.Contains(rust, "(*__f)(t.clone())") &&
		!strings.Contains(rust, "(*__f)((*t).clone())") {
		t.Fatalf("range value passed to func(T) should pass the element handle:\n%s", rust)
	}
	if strings.Contains(rust, "Box::new((*u.borrow().as_ref().unwrap()).clone()) as Box<dyn Any") {
		t.Fatalf("generic []T assignment should not box the replacement value as Any:\n%s", rust)
	}
}

func TestGenericComparablePointerInstantiationEmitsPointerIdentityComparable(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Var struct {
	name string
}

func substList[T comparable](in []T, subst func(T) T) []T {
	for i, t := range in {
		if u := subst(t); u != t {
			out := make([]T, len(in))
			copy(out, in)
			out[i] = u
			return out
		}
	}
	return nil
}

func use(vars []*Var, subst func(*Var) *Var) []*Var {
	return substList(vars, subst)
}
`)

	if !strings.Contains(rust, "subst_list::<Var>") {
		t.Fatalf("generic comparable pointer instantiation should keep the existing raw pointee ABI:\n%s", rust)
	}
	if !strings.Contains(rust, "impl GoComparable for Var") ||
		!strings.Contains(rust, "std::ptr::eq(self, other)") {
		t.Fatalf("generic comparable pointer instantiation should give the pointee pointer-identity GoComparable semantics:\n%s", rust)
	}
}

func TestComparableGenericStructMethodKeepsGoComparableBound(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type HashTrieMap[K comparable, V any] struct {
	seed uintptr
}

func (ht *HashTrieMap[K, V]) load(key K) (value V) {
	var m map[K]V
	_ = m
	return value
}
`)

	if !strings.Contains(rust, "pub struct HashTrieMap<K: Any + GoComparable + 'static, V: Any + 'static>") {
		t.Fatalf("comparable generic struct should require GoComparable for K:\n%s", rust)
	}
	if !strings.Contains(rust, "impl<K: Any + GoComparable + GoValueClone + 'static, V: Any + GoValueClone + 'static> HashTrieMap<K, V>") {
		t.Fatalf("method impl with direct K/V values should keep GoComparable plus GoValueClone bounds:\n%s", rust)
	}
}

func TestInterfaceMethodSignatureNamesUnnamedParams(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node interface {
	Method(int) int
}
`)

	if strings.Contains(rust, "fn method(&self)") {
		t.Fatalf("interface method with unnamed parameter should not omit the parameter:\n%s", rust)
	}
	if !strings.Contains(rust, "fn method(&self, __arg0: Rc<RefCell<Option<i32>>>)") {
		t.Fatalf("interface method with unnamed parameter should get a synthetic Rust name:\n%s", rust)
	}
}

func TestGenericAnyTypeParamReturnUsesTypeParam(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func Identity[T any](x T) T {
	return x
}
`)

	want := "pub fn identity<T: Any + GoValueClone + 'static>(x: Rc<RefCell<Option<T>>>) -> Rc<RefCell<Option<T>>>"
	if !strings.Contains(rust, want) {
		t.Fatalf("generic any return should preserve the type parameter in the result signature, want %q:\n%s", want, rust)
	}
	if strings.Contains(rust, "Rc<RefCell<Option<Box<dyn Any") {
		t.Fatalf("generic any value/result slots should not lower to the any constraint object:\n%s", rust)
	}
}

func TestGenericFunctionValueTypeParamReturnKeepsGenericWrapperABI(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func retry[T any](fn func() (T, error)) (T, error) {
	return fn()
}

func call() (int, error) {
	v, err := retry(func() (int, error) {
		return 1, nil
	})
	return v, err
}
`)

	if !strings.Contains(rust, "Box<dyn FnMut() -> (Rc<RefCell<Option<T>>>, Rc<RefCell<Option<Box<dyn StdError>>>>)") {
		t.Fatalf("generic func parameter should keep the generic wrapped T return ABI:\n%s", rust)
	}
	if !strings.Contains(rust, "Box<dyn FnMut() -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>)") {
		t.Fatalf("instantiated func literal should be coerced to the wrapped generic return ABI:\n%s", rust)
	}
	if strings.Contains(rust, "Box<dyn FnMut() -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>)") {
		t.Fatalf("func literal passed to generic func parameter should not keep a bare scalar return ABI:\n%s", rust)
	}
	if strings.Contains(rust, "let (mut v, mut err) = retry::<i32>") {
		t.Fatalf("scalar result from wrapped generic return should be unpacked through a temp before binding:\n%s", rust)
	}
	if !strings.Contains(rust, "let (__tmp_0, mut err) = retry::<i32>") {
		t.Fatalf("wrapped generic return should bind scalar result through a temp:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut v = { let __tmp_holder = __tmp_0.clone(); let __tmp_guard = __tmp_holder.borrow(); (*__tmp_guard.as_ref().unwrap()).clone() };") {
		t.Fatalf("wrapped generic scalar result temp should be unwrapped into the bare scalar local:\n%s", rust)
	}
}

func TestGenericFunctionValueTypeParamReturnWrapsMultiResultCall(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func pair() (int, error) {
	return 1, nil
}

func retry[T any](fn func() (T, error)) (T, error) {
	return fn()
}

func call() (int, error) {
	return retry(func() (int, error) {
		return pair()
	})
}
`)

	if strings.Contains(rust, "return pair();") || strings.Contains(rust, "\n        pair()\n") {
		t.Fatalf("multi-result call in wrapped generic func literal should not bypass slot conversion:\n%s", rust)
	}
	if !strings.Contains(rust, "let (__return_tmp_0, __return_tmp_1) = pair();") {
		t.Fatalf("multi-result call should be captured before generic return-slot conversion:\n%s", rust)
	}
	if !strings.Contains(rust, "let __return_slot_0 = Rc::new(RefCell::new(Some(__return_tmp_0)));") {
		t.Fatalf("bare scalar multi-result slot should be bound before the final tuple:\n%s", rust)
	}
	if !strings.Contains(rust, "(__return_slot_0, __return_tmp_1)") {
		t.Fatalf("bare scalar multi-result slot should be wrapped for the generic T return ABI:\n%s", rust)
	}
}

func TestGenericUnionTypeParamParameterUsesTypeParam(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func Keep[N int64 | uint64](num N) N {
	return num
}
`)

	want := "pub fn keep<N: GoInteger + Clone + 'static>(num: N) -> Rc<RefCell<Option<N>>>"
	if !strings.Contains(rust, want) {
		t.Fatalf("generic union-constrained parameter should preserve the type parameter in the signature, want %q:\n%s", want, rust)
	}
	if strings.Contains(rust, "num: Rc<RefCell<Option<i64>>>") {
		t.Fatalf("generic union-constrained parameter should not lower to the first constraint term:\n%s", rust)
	}
	if !strings.Contains(rust, "Rc::new(RefCell::new(Some(num)))") {
		t.Fatalf("generic union-constrained bare parameter should be wrapped for the generic return slot:\n%s", rust)
	}
}

func TestGenericNamedOrderedConstraintAddsPartialOrdBound(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Ordered interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 |
		~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 | ~uintptr |
		~float32 | ~float64 |
		~string
}

func Less[T Ordered](x, y T) bool {
	return x < y
}
`)

	if !strings.Contains(rust, "pub fn less<T: Ordered + Clone") || !strings.Contains(rust, "PartialOrd") {
		t.Fatalf("named ordered constraint should emit clone and ordering bounds:\n%s", rust)
	}
	if !strings.Contains(rust, "impl Ordered for i32") {
		t.Fatalf("named ordered constraint should be implemented for primitive ordered Rust types:\n%s", rust)
	}
}

func TestGenericFunctionPropagatesGoValueCloneBoundFromCalledFunction(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func touch[E any](data []E) {
	var value E = data[0]
	_ = value
}

func caller[E any](data []E) {
	touch(data)
}
`)

	if !strings.Contains(rust, "pub fn touch<E: Any + GoValueClone + 'static>") {
		t.Fatalf("helper with direct type-param element use should require GoValueClone:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn caller<E: Any + GoValueClone + 'static>") {
		t.Fatalf("caller should propagate GoValueClone required by the called helper:\n%s", rust)
	}
}

func TestStructUsedAsGoValueCloneGenericArgImplementsTrait(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type File struct {
	name string
}

func first[E any](values []E) E {
	return values[0]
}

func use(files []File) File {
	return first(files)
}
`)

	if !strings.Contains(rust, "pub fn first<E: Any + GoValueClone + 'static>") {
		t.Fatalf("generic helper returning a direct type-param element should require GoValueClone:\n%s", rust)
	}
	want := "impl GoValueClone for File {\n    fn go_value_clone(&self) -> Self {\n        self.__go_value_clone()\n    }\n}"
	if !strings.Contains(rust, want) {
		t.Fatalf("struct used as a GoValueClone generic argument should implement the trait, missing %q:\n%s", want, rust)
	}
}

func TestLocalInterfaceUsedAsGoValueCloneGenericArgImplementsTrait(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Spec interface {
	specNode()
}

type ImportSpec struct{}

func (*ImportSpec) specNode() {}

func first[E any](values []E) E {
	return values[0]
}

func use(specs []Spec) Spec {
	return first(specs)
}
`)

	if !strings.Contains(rust, "pub fn first<E: Any + GoValueClone + 'static>") {
		t.Fatalf("generic helper returning a direct type-param element should require GoValueClone:\n%s", rust)
	}
	want := "impl GoValueClone for Box<dyn Spec> {\n    fn go_value_clone(&self) -> Self {\n        Spec::__go_clone_box_spec(self.as_ref())\n    }\n}"
	if !strings.Contains(rust, want) {
		t.Fatalf("local interface used as a GoValueClone generic argument should implement the trait, missing %q:\n%s", want, rust)
	}
}

func TestGenericOrderedFunctionPropagatesCloneBoundFromCalledFunction(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Ordered interface {
	~int | ~string
}

func order2[E Ordered](data []E) {
	var value E = data[0]
	_ = value
}

func median[E Ordered](data []E) {
	order2(data)
}
`)

	if !strings.Contains(rust, "pub fn order2<E: Ordered + Clone + PartialOrd + 'static>") {
		t.Fatalf("ordered helper with direct type-param element use should require Clone:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn median<E: Ordered + Clone + PartialOrd + 'static>") {
		t.Fatalf("ordered caller should propagate Clone required by the called helper:\n%s", rust)
	}
	if strings.Contains(rust, "pub fn median<E: Ordered + GoValueClone") {
		t.Fatalf("ordered caller should not emit GoValueClone for ordered constraints:\n%s", rust)
	}
}

func TestGenericStructDeclCarriesTypeParamsIntoImpls(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Holder[T any] struct {
	value T
	next *Holder[T]
}

func (h *Holder[T]) Get() T {
	return h.value
}
`)

	wantDecl := "pub struct Holder<T: Any + 'static> {"
	if !strings.Contains(rust, wantDecl) {
		t.Fatalf("generic struct declaration should carry type parameters, want %q:\n%s", wantDecl, rust)
	}
	if !strings.Contains(rust, "pub value: Rc<RefCell<Option<T>>>") {
		t.Fatalf("generic struct field should use the type parameter in its wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "pub next: Rc<RefCell<Option<Holder<T>>>>") {
		t.Fatalf("instantiated generic field should keep type arguments:\n%s", rust)
	}
	for _, want := range []string{
		"impl<T: Any + 'static> Holder<T> {",
		"impl<T: Any + GoValueClone + 'static> Holder<T> {\n    pub fn get",
		"impl<T: Any + 'static> std::fmt::Display for Holder<T> where T: std::fmt::Display {",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("generic struct impl should carry type parameters, missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "impl Unknown") || strings.Contains(rust, "impl Holder {\n    pub fn get") {
		t.Fatalf("generic receiver method should lower under Holder<T>, not Unknown or bare Holder:\n%s", rust)
	}
}

func TestGenericStructAnyInstantiationDoesNotRequireCloneBound(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Bucket[T any] struct {
	next *Bucket[T]
}

type Holder struct {
	bucket Bucket[any]
}
`)

	if strings.Contains(rust, "pub struct Bucket<T: Any + Clone") {
		t.Fatalf("generic struct declaration should not require Clone for any instantiations:\n%s", rust)
	}
	if !strings.Contains(rust, "impl<T: Any + 'static> Clone for Bucket<T>") {
		t.Fatalf("generic struct should use a clone impl with declaration-level bounds:\n%s", rust)
	}
	if !strings.Contains(rust, "pub bucket: Rc<RefCell<Option<Bucket<Box<dyn Any>>>>>") {
		t.Fatalf("struct field should still instantiate Bucket[any] with the any representation:\n%s", rust)
	}
}

func TestGenericStructAnyMethodWithoutTypeParamUseDoesNotRequireCloneBound(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Bucket[T any] struct {
	next *Bucket[T]
}

func (b *Bucket[T]) Clear() {
	b.next = nil
}

type Holder struct {
	bucket Bucket[any]
}

func Use(h *Holder) {
	h.bucket.Clear()
}
`)

	if strings.Contains(rust, "impl<T: Any + Clone + 'static> Bucket<T> {\n    pub fn clear") {
		t.Fatalf("generic method that does not use T should not require Clone for Bucket[any]:\n%s", rust)
	}
	if !strings.Contains(rust, "impl<T: Any + 'static> Bucket<T> {\n    pub fn clear") {
		t.Fatalf("generic method that does not use T should use declaration-level bounds:\n%s", rust)
	}
}

func TestGenericStructAnyMethodValueCopyUsesGoValueCloneBound(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Cell[T any] struct {
	value T
}

func (c *Cell[T]) Store(value T) {
	c.value = value
}

func (c *Cell[T]) Load() T {
	return c.value
}

func Use(c *Cell[any]) {
	c.Store("value")
	fmt.Println(c.Load())
}
`)

	if strings.Contains(rust, "impl<T: Any + Clone + 'static> Cell<T> {\n    pub fn store") {
		t.Fatalf("generic method copying T should not require Rust Clone for Cell[any]:\n%s", rust)
	}
	if !strings.Contains(rust, "impl<T: Any + GoValueClone + 'static> Cell<T> {\n    pub fn store") {
		t.Fatalf("generic method copying T should require GoValueClone bounds:\n%s", rust)
	}
	for _, want := range []string{
		"trait GoValueClone",
		"impl GoValueClone for Box<dyn Any>",
		"(*value.borrow().as_ref().unwrap()).go_value_clone()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("generic any method copy should emit %q:\n%s", want, rust)
		}
	}
}

func TestGenericStructMethodCallingTypeParamHelperKeepsCloneBound(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node[T any] struct{}

func newNode[T any]() Node[T] {
	return Node[T]{}
}

type Bucket[T any] struct {
	node Node[T]
}

func (b *Bucket[T]) Reset() {
	b.node = newNode[T]()
}

func (b *Bucket[T]) Clear() {
}
`)

	if strings.Contains(rust, "impl<T: Any + Clone + 'static> Bucket<T> {\n    pub fn reset") {
		t.Fatalf("generic method calling a helper that does not copy T should not require Clone bounds:\n%s", rust)
	}
	if !strings.Contains(rust, "impl<T: Any + 'static> Bucket<T> {\n    pub fn reset") {
		t.Fatalf("generic method calling a helper that does not copy T should stay in declaration-bound impl:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn new_node<T: Any + 'static>()") {
		t.Fatalf("generic helper that only constructs handles should not require Clone bounds:\n%s", rust)
	}
	declImplStart := strings.Index(rust, "impl<T: Any + 'static> Bucket<T> {\n    pub fn reset")
	if declImplStart < 0 || !strings.Contains(rust[declImplStart:], "pub fn clear") {
		t.Fatalf("generic methods that do not copy T should share declaration-bound impl:\n%s", rust)
	}
}

func TestGenericStructMethodCallingCloneBoundMethodKeepsCloneBound(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node[T any] struct{}

func newNode[T any]() Node[T] {
	return Node[T]{}
}

type Bucket[T any] struct {
	node Node[T]
}

func (b *Bucket[T]) resetSlow() {
	b.node = newNode[T]()
}

func (b *Bucket[T]) Reset() {
	b.resetSlow()
}

func (b *Bucket[T]) Clear() {
}
`)

	if strings.Contains(rust, "impl<T: Any + Clone + 'static> Bucket<T> {\n    pub fn reset_slow") {
		t.Fatalf("helper-instantiating method should not require Clone when the helper does not copy T:\n%s", rust)
	}
	declImplStart := strings.Index(rust, "impl<T: Any + 'static> Bucket<T> {\n    pub fn reset_slow")
	if declImplStart < 0 ||
		!strings.Contains(rust[declImplStart:], "pub fn reset(&mut self)") ||
		!strings.Contains(rust[declImplStart:], "pub fn clear") {
		t.Fatalf("methods that only call declaration-bound helpers should share declaration-bound impl:\n%s", rust)
	}
}

func TestGenericStructWithUnusedTypeParamsAddsPhantomData(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Header[K any, V any] struct {
	isEntry bool
}

func MakeHeader[K any, V any]() Header[K, V] {
	return Header[K, V]{isEntry: true}
}
`)

	if !strings.Contains(rust, "pub __go_phantom: std::marker::PhantomData<(K, V)>") {
		t.Fatalf("generic struct should retain unused type parameters with PhantomData:\n%s", rust)
	}
	if !strings.Contains(rust, "__go_phantom: std::marker::PhantomData") {
		t.Fatalf("generic struct literals and defaults should initialize PhantomData:\n%s", rust)
	}
}

func TestGenericStructDisplayImplBoundsFormattedTypeParams(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Entry[K any, V any] struct {
	key K
	value V
}
`)

	want := "impl<K: Any + 'static, V: Any + 'static> std::fmt::Display for Entry<K, V> where K: std::fmt::Display, V: std::fmt::Display {"
	if !strings.Contains(rust, want) {
		t.Fatalf("generic struct Display impl should bound formatted type parameters, missing %q:\n%s", want, rust)
	}
}

func TestGenericStructDisplayZeroLengthPointerArrayNeedsNoDisplayBound(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Pointer[T any] struct {
	_ [0]*T
	v int
}
`)

	for _, forbidden := range []string{
		"where T: std::fmt::Display",
		"format_slice_wrapped(&self.__blank_0_0)",
	} {
		if strings.Contains(rust, forbidden) {
			t.Fatalf("zero-length pointer-array phantom field should not require Display via %q:\n%s", forbidden, rust)
		}
	}
	if !strings.Contains(rust, `let __go_fmt_0 = format!("{}", "[]");`) ||
		!strings.Contains(rust, `write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)`) {
		t.Fatalf("zero-length pointer-array field should format as an empty array literal:\n%s", rust)
	}
}

func TestGenericStructDisplayDoesNotLeakBoundsThroughOpaqueNamedField(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node[K any, V any] struct {
	isEntry bool
}

type Indirect[K any, V any] struct {
	node Node[K, V]
}

type Entry[K any, V any] struct {
	key K
	value V
}

type Holder[K any, V any] struct {
	entry Entry[K, V]
}
`)

	if strings.Contains(rust, "impl<K: Any + 'static, V: Any + 'static> std::fmt::Display for Indirect<K, V> where K: std::fmt::Display") {
		t.Fatalf("Display bounds from Node should not leak through Indirect's named field:\n%s", rust)
	}
	want := "impl<K: Any + 'static, V: Any + 'static> std::fmt::Display for Holder<K, V> where K: std::fmt::Display, V: std::fmt::Display {"
	if !strings.Contains(rust, want) {
		t.Fatalf("Display bounds from Entry should still propagate through Holder, missing %q:\n%s", want, rust)
	}
}

func TestGenericStructDisplayRequiresTypeInfoForBounds(t *testing.T) {
	rust := transpileRegression(t, `package main

type Entry[K any] struct {
	key K
}
`, nil)

	if !strings.Contains(rust, `unimplemented!("type info required for generic struct Display bounds")`) {
		t.Fatalf("generic struct Display without type info should fail loudly:\n%s", rust)
	}
}

func TestGenericSliceConstrainedParameterUsesSliceRepresentation(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func Len[S ~[]E, E any](s S) int {
	return len(s)
}
`)

	if strings.Contains(rust, "s: Rc<RefCell<Option<S>>>") ||
		strings.Contains(rust, "s: Arc<Mutex<Option<S>>>") {
		t.Fatalf("slice-constrained type parameter should not stay opaque in value parameter slots:\n%s", rust)
	}
	if !strings.Contains(rust, "Vec<Rc<RefCell<Option<E>>>>") &&
		!strings.Contains(rust, "Vec<Arc<Mutex<Option<E>>>>") {
		t.Fatalf("slice-constrained type parameter should use the slice representation:\n%s", rust)
	}
}

func TestGenericOrderedSliceConstrainedParameterUsesBareElements(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Ordered interface {
	~int | ~string
}

func LessAt[S ~[]E, E Ordered](s S, i int, j int) bool {
	return s[i] < s[j]
}
`)

	if strings.Contains(rust, "Vec<Rc<RefCell<Option<E>>>>") ||
		strings.Contains(rust, "Vec<Arc<Mutex<Option<E>>>>") {
		t.Fatalf("ordered slice-constrained type parameter should not wrap ordered elements:\n%s", rust)
	}
	if !strings.Contains(rust, "Vec<E>") {
		t.Fatalf("ordered slice-constrained type parameter should use raw ordered elements:\n%s", rust)
	}
}

func TestGenericOrderedSliceParameterUsesBareElements(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Ordered interface {
	~int | ~string
}

func Sort[E Ordered](data []E) {
	_ = data[0]
}
`)

	if strings.Contains(rust, "Vec<Rc<RefCell<Option<E>>>>") ||
		strings.Contains(rust, "Vec<Arc<Mutex<Option<E>>>>") {
		t.Fatalf("ordered slice parameter should not wrap ordered elements:\n%s", rust)
	}
	if !strings.Contains(rust, "Vec<E>") {
		t.Fatalf("ordered slice parameter should use raw ordered elements:\n%s", rust)
	}
}

func TestGenericStringTypeParamAddsToStringBound(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func Join[S ~[]T, T ~string](s S) []string {
	strs := make([]string, 0, len(s))
	for _, v := range s {
		strs = append(strs, string(v))
	}
	return strs
}
`)

	if !strings.Contains(rust, "T: Clone + PartialOrd + ToString") {
		t.Fatalf("string-constrained type parameter should include ToString for string conversion:\n%s", rust)
	}
	if !strings.Contains(rust, "v.to_string()") {
		t.Fatalf("string-constrained type parameter conversion should use ToString:\n%s", rust)
	}
}

func TestGenericOrderedTypeParamParameterUsesBareValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Ordered interface {
	~int | ~string
}

func Less[T Ordered](x, y T) bool {
	return x < y
}
`)

	if strings.Contains(rust, "x: Rc<RefCell<Option<T>>>") ||
		strings.Contains(rust, "x: Arc<Mutex<Option<T>>>") ||
		strings.Contains(rust, "y: Rc<RefCell<Option<T>>>") ||
		strings.Contains(rust, "y: Arc<Mutex<Option<T>>>") {
		t.Fatalf("ordered type parameter values should be raw parameters:\n%s", rust)
	}
	if !strings.Contains(rust, "x: T") || !strings.Contains(rust, "y: T") {
		t.Fatalf("ordered type parameter values should use raw Rust type parameters:\n%s", rust)
	}
}

func TestGenericPointerConstraintTypeParamUsesCloneBound(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func clone[P *T, T any](p P) P {
	c := *p
	return &c
}
`)

	want := "pub fn clone<P: Clone + 'static, T: Any + GoValueClone + 'static>(p: Rc<RefCell<Option<P>>>) -> Rc<RefCell<Option<P>>>"
	if !strings.Contains(rust, want) {
		t.Fatalf("generic pointer-constrained type parameter should get a clone bound, want %q:\n%s", want, rust)
	}
	if strings.Contains(rust, "let mut c = (*p") {
		t.Fatalf("pointer-constrained dereference short declaration should not bind a bare type-parameter value:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut c = Rc::new(RefCell::new(Some({ let __v = (*p") &&
		!strings.Contains(rust, "let mut c = Arc::new(Mutex::new(Some({ let __v = (*p") {
		t.Fatalf("pointer-constrained dereference short declaration should wrap the cloned pointee in a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "return c.clone();") {
		t.Fatalf("address of the cloned pointee should return the local handle:\n%s", rust)
	}
}

func TestStructDefaultWrapsNamedArrayFieldZeroValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Bitmap [2]uint8

type RegArgs struct {
	ReturnIsPtr Bitmap
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

	if !strings.Contains(rust, "return_is_ptr: Rc::new(RefCell::new(Some(Bitmap(") &&
		!strings.Contains(rust, "return_is_ptr: Arc::new(Mutex::new(Some(Bitmap(") {
		t.Fatalf("named array field default should construct the named type:\n%s", rust)
	}
	if !strings.Contains(rust, "impl Display for Bitmap") || !strings.Contains(rust, "format_slice(&self.0)") {
		t.Fatalf("named array type should implement Display through the slice formatter:\n%s", rust)
	}
}

func TestNamedArrayDefaultInitializesInnerArray(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Bitmap [2]uint8

func mark() Bitmap {
	var b Bitmap
	b[1] |= 3
	return b
}
`)

	if strings.Contains(rust, "#[derive(Debug, Clone, Default)]\npub struct Bitmap") {
		t.Fatalf("named array type should not derive Default for an Option-backed array handle:\n%s", rust)
	}
	if !strings.Contains(rust, "impl Default for Bitmap") ||
		!strings.Contains(rust, "Some(std::array::from_fn(|_| 0))") {
		t.Fatalf("named array default should initialize the inner array handle:\n%s", rust)
	}
}

func TestNamedArrayOverNamedArrayDefaultUsesInnerNamedArray(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type PageBits [2]uint64
type PallocBits PageBits

func mark() PallocBits {
	var p PallocBits
	return p
}
`)

	if !strings.Contains(rust, "pub struct PallocBits(pub Rc<RefCell<Option<PageBits>>>);") &&
		!strings.Contains(rust, "pub struct PallocBits(pub Arc<Mutex<Option<PageBits>>>);") {
		t.Fatalf("named array over named array should store the inner named array type:\n%s", rust)
	}
	if strings.Contains(rust, "PallocBits(Rc::new(RefCell::new(Some(std::array::from_fn(|_| 0)))))") ||
		strings.Contains(rust, "PallocBits(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))") {
		t.Fatalf("named array over named array default must not initialize a raw array in the inner named slot:\n%s", rust)
	}
	if !strings.Contains(rust, "PallocBits(Rc::new(RefCell::new(Some(PageBits(") &&
		!strings.Contains(rust, "PallocBits(Arc::new(Mutex::new(Some(PageBits(") {
		t.Fatalf("named array over named array default should initialize the inner named array value:\n%s", rust)
	}

	displayStart := strings.Index(rust, "impl Display for PallocBits")
	if displayStart < 0 {
		t.Fatalf("named array over named array should implement Display:\n%s", rust)
	}
	displayEnd := strings.Index(rust[displayStart:], "\n}\n")
	if displayEnd < 0 {
		t.Fatalf("could not isolate PallocBits Display impl:\n%s", rust)
	}
	displayImpl := rust[displayStart : displayStart+displayEnd]
	if strings.Contains(displayImpl, "format_slice(&self.0)") {
		t.Fatalf("named array over named array Display must not format the outer inner-named slot as a raw slice:\n%s", rust)
	}
	if !strings.Contains(displayImpl, "__inner_guard.as_ref().unwrap()") {
		t.Fatalf("named array over named array Display should format through the inner named value:\n%s", rust)
	}
}

func TestNamedMapTypeDefinitionUsesFormatMapDisplay(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node struct{}

type nodeSet map[*Node]bool

type graphNode struct {
	pred nodeSet
}
`)

	if !strings.Contains(rust, "impl Display for nodeSet") ||
		!strings.Contains(rust, "format_map(&self.0)") {
		t.Fatalf("displayable named map definitions should implement Display through format_map:\n%s", rust)
	}
}

func TestNamedMapStructFieldDefaultConstructsNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node struct{}

type nodeSet map[*Node]bool

type graphNode struct {
	succ nodeSet
}
`)

	if strings.Contains(rust, "#[derive(Debug, Clone, Default)]\npub struct graphNode") {
		t.Fatalf("named map field should require a custom struct default:\n%s", rust)
	}
	if !strings.Contains(rust, "impl Default for graphNode") {
		t.Fatalf("named map field default should emit a custom Default impl:\n%s", rust)
	}
	if !strings.Contains(rust, "succ: Rc::new(RefCell::new(Some(nodeSet::default())))") &&
		!strings.Contains(rust, "succ: Arc::new(Mutex::new(Some(nodeSet::default())))") {
		t.Fatalf("named map field default should construct the named map value:\n%s", rust)
	}
}

func TestStructDisplayPointerToSliceFieldUsesSliceFormatter(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type ranges struct {
	p *[]rune
}
`)

	if strings.Contains(rust, "(*self.p") {
		t.Fatalf("pointer-to-slice struct display should not format the raw Vec with Display:\n%s", rust)
	}
	if !strings.Contains(rust, "format_slice(&self.p)") {
		t.Fatalf("pointer-to-slice struct display should use the slice formatter:\n%s", rust)
	}
}

func TestStructDisplayNestedPointerArrayUsesNestedPointerWrappedFormatter(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type arena struct {
	id int
}

type heap struct {
	arenas [2]*[4]*arena
}
`)

	if strings.Contains(rust, "format_slice_wrapped(&self.arenas)") {
		t.Fatalf("nested pointer-array struct display should not use the single-level wrapped formatter:\n%s", rust)
	}
	if !strings.Contains(rust, "format_nested_pointer_slice_wrapped(&self.arenas)") {
		t.Fatalf("nested pointer-array struct display should use the nested pointer wrapped formatter:\n%s", rust)
	}
}

func TestStructDisplayNestedPointerRawArrayUsesNestedPointerFormatter(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type data struct {
	id int
}

type page struct {
	chunks [2]*[4]data
}
`)

	if strings.Contains(rust, "format_slice_wrapped(&self.chunks)") {
		t.Fatalf("nested pointer-array struct display should not use the single-level wrapped formatter:\n%s", rust)
	}
	if !strings.Contains(rust, "format_nested_pointer_slice(&self.chunks)") {
		t.Fatalf("nested pointer-array struct display should use the nested pointer formatter:\n%s", rust)
	}
}

func TestStructDisplayPointerFieldDoesNotRequirePointeeDisplay(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "time"

type ctxResult struct {
	timer *time.Timer
}
`)

	if strings.Contains(rust, "(*self.timer") {
		t.Fatalf("pointer struct display should not dereference and require pointee Display:\n%s", rust)
	}
	if !strings.Contains(rust, "format!(\"{:p}\", __v as *const _)") {
		t.Fatalf("pointer struct display should format the pointer value:\n%s", rust)
	}
}

func TestGoPtrStructFieldCustomDefaultAndDisplayUseGoPtr(t *testing.T) {
	fset := token.NewFileSet()
	mheapFile, err := parser.ParseFile(fset, "mheap.go", `package main

type mspan struct{}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(mheap.go) error = %v", err)
	}
	arenaFile, err := parser.ParseFile(fset, "arena.go", `package main

import "unsafe"

type liveUserArenaChunk struct {
	*mspan
	x uintptr
}

func raw(addr uintptr) *mspan {
	return (*mspan)(unsafe.Pointer(addr))
}

func assign(c *liveUserArenaChunk, addr uintptr) {
	c.mspan = raw(addr)
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(arena.go) error = %v", err)
	}
	files := []*ast.File{mheapFile, arenaFile}
	typeInfo, err := NewTypeInfo(files, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	prevTypeInfo := currentTypeInfo
	prevContext := currentContext
	prevVarTable := currentVarTable
	t.Cleanup(func() {
		SetTypeInfo(prevTypeInfo)
		SetTranspileContext(prevContext)
		SetVarTable(prevVarTable)
	})

	SetTypeInfo(typeInfo)
	ctx := &TranspileContext{
		Session: NewTranspileSession(typeInfo, nil),
		Package: NewPackageState(),
	}
	SetTranspileContext(ctx)
	registerSliceElemPtrFactsFromFiles(files)

	rust, _, _ := TranspileWithMapping(arenaFile, fset, typeInfo, nil)

	if !strings.Contains(rust, "pub mspan: GoPtr<mspan>") {
		t.Fatalf("pointer field assigned a GoPtr value should emit GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "Self { mspan: GoPtr::nil(), x:") {
		t.Fatalf("custom Default should initialize generated GoPtr fields with GoPtr::nil():\n%s", rust)
	}
	if strings.Contains(rust, "self.mspan.borrow()") || strings.Contains(rust, "self.mspan.lock()") {
		t.Fatalf("Display should not format generated GoPtr fields as wrapped pointer handles:\n%s", rust)
	}
	if !strings.Contains(rust, "self.mspan.is_nil()") {
		t.Fatalf("Display should format generated GoPtr fields through GoPtr::is_nil:\n%s", rust)
	}
}

func TestGoPtrStructFieldCustomPartialEqUsesGoPtrIdentity(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type item struct {
	n int
}

type bitvector struct {
	n        int
	bytedata *item
}

func start() {
	go func() {}()
}

func used(items []item) bool {
	v := bitvector{n: 1, bytedata: &items[0]}
	return v != bitvector{}
}
`)

	if !strings.Contains(rust, "pub bytedata: GoPtr<item>") {
		t.Fatalf("test setup should promote pointer field to GoPtr storage:\n%s", rust)
	}
	if strings.Contains(rust, "self.bytedata.lock()") ||
		strings.Contains(rust, "other.bytedata.lock()") ||
		strings.Contains(rust, "Arc::ptr_eq(&self.bytedata") ||
		strings.Contains(rust, "Rc::ptr_eq(&self.bytedata") {
		t.Fatalf("custom PartialEq should not treat generated GoPtr fields as old pointer wrappers:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::ptr_eq(&self.bytedata, &other.bytedata)") {
		t.Fatalf("custom PartialEq should compare generated GoPtr field handles by identity:\n%s", rust)
	}
}

func TestNamedMapWithInterfaceValueDefinitionUsesFormatMapDisplay(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type TypeParam struct{}

type Type interface {
	String() string
}

type substMap map[*TypeParam]Type

type subster struct {
	smap substMap
}
`)

	if !strings.Contains(rust, "impl Display for substMap") ||
		!strings.Contains(rust, "format_map(&self.0)") {
		t.Fatalf("named map definitions with interface values should implement Display through format_map:\n%s", rust)
	}
}

func TestNamedSliceTypeDefinitionOverNonDebugStructDoesNotDeriveDebug(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type dependency interface {
	Name() string
}

type graphNode struct {
	obj dependency
}

type nodeQueue []*graphNode
`)

	if strings.Contains(rust, "#[derive(Debug, Clone, Default)]\npub struct nodeQueue") {
		t.Fatalf("named slice over non-Debug element should not derive Debug:\n%s", rust)
	}
	if !strings.Contains(rust, "#[derive(Clone, Default)]\npub struct nodeQueue") {
		t.Fatalf("named slice over non-Debug element should still derive Clone and Default:\n%s", rust)
	}
}

func TestNamedPointerSliceTypeDefinitionOverStringerImplementsDisplay(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type sortable interface {
	Len() int
	Less(i, j int) bool
	Swap(i, j int)
}

type TypeParam struct{}

func (*TypeParam) String() string { return "T" }

type typeParamsById []*TypeParam

func (t typeParamsById) Len() int           { return len(t) }
func (t typeParamsById) Less(i, j int) bool { return false }
func (t typeParamsById) Swap(i, j int)      {}

func use(s sortable) {}

func call(t typeParamsById) {
	use(t)
}
`)

	if !strings.Contains(rust, "impl Display for typeParamsById") {
		t.Fatalf("named pointer slice over Stringer should implement Display for interface objects:\n%s", rust)
	}
	if !strings.Contains(rust, "format_slice_wrapped(&self.0)") {
		t.Fatalf("named pointer slice Display should use wrapped slice formatting:\n%s", rust)
	}
}

func TestNamedPointerSliceTypeDefinitionOverDisplayStructImplementsDisplay(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type sortable interface {
	Len() int
	Less(i, j int) bool
	Swap(i, j int)
}

type graphNode struct {
	name string
}

type nodeQueue []*graphNode

func (t nodeQueue) Len() int           { return len(t) }
func (t nodeQueue) Less(i, j int) bool { return false }
func (t nodeQueue) Swap(i, j int)      {}

func use(s sortable) {}

func call(t nodeQueue) {
	use(t)
}
`)

	if !strings.Contains(rust, "impl Display for nodeQueue") {
		t.Fatalf("named pointer slice over generated-Display struct should implement Display for interface objects:\n%s", rust)
	}
	if !strings.Contains(rust, "format_slice_wrapped(&self.0)") {
		t.Fatalf("named pointer slice over generated-Display struct should use wrapped slice formatting:\n%s", rust)
	}
}

func TestNamedSliceErrorTypeDefinitionWithoutDeriveDebugImplementsDebug(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type detail interface {
	String() string
}

type item struct {
	d detail
}

type itemList []*item

func (p itemList) Error() string {
	return "items"
}
`)

	if strings.Contains(rust, "#[derive(Debug, Clone, Default)]\npub struct itemList") {
		t.Fatalf("named error slice over non-Debug element should not derive Debug:\n%s", rust)
	}
	if !strings.Contains(rust, "impl std::fmt::Debug for itemList") {
		t.Fatalf("named error slice without derived Debug should implement Debug manually:\n%s", rust)
	}
	if !strings.Contains(rust, "impl StdError for itemList") {
		t.Fatalf("named error slice should still implement StdError:\n%s", rust)
	}
}

func TestSyncMapStructFieldUsesWrappedHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type cache struct {
	m sync.Map
}
`)

	if strings.Contains(rust, "pub m: sync_Map,") {
		t.Fatalf("sync.Map struct fields should not be bare when struct helpers treat fields as handles:\n%s", rust)
	}
	if !strings.Contains(rust, "pub m: Rc<RefCell<Option<sync_Map>>>") &&
		!strings.Contains(rust, "pub m: Arc<Mutex<Option<sync_Map>>>") {
		t.Fatalf("sync.Map struct fields should use the normal wrapped field representation:\n%s", rust)
	}
}

func TestDisplayImplForMutableStringMethodUsesReceiverClone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type setting struct {
	once sync.Once
	n int
}

func (s *setting) String() string {
	return s.Value()
}

func (s *setting) Value() string {
	s.once.Do(func() {
		s.n++
	})
	return "setting"
}
`)

	if strings.Contains(rust, "write!(f, \"{}\", (*self.string()") {
		t.Fatalf("Display impl should not call a mutable String method through &self:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut __self = self.clone();") || !strings.Contains(rust, "(*__self.string()") {
		t.Fatalf("Display impl should call mutable String through a receiver clone:\n%s", rust)
	}
}

func TestEmbeddedInterfaceTraitObjectImplementsSupertrait(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Node interface {
	Pos() int
}

type Expr interface {
	Node
	ExprNode()
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

	if !strings.Contains(rust, "impl Node for Box<dyn Expr>") {
		t.Fatalf("boxed Expr trait object should implement embedded Node trait:\n%s", rust)
	}
	if !strings.Contains(rust, "fn pos(&self) -> i32") {
		t.Fatalf("Node method should be delegated on boxed Expr trait object:\n%s", rust)
	}
}

func TestEmbeddedImportedSameNameInterfaceCloneUsesQualifiedHelper(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "sort"

type Interface interface {
	sort.Interface
	Push(x any)
	Pop() any
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
	if !strings.Contains(rust, "pub trait Interface: sort::Interface +") {
		t.Fatalf("same-name embedded imported interface should be emitted as a supertrait:\n%s", rust)
	}
	if !strings.Contains(rust, "Interface::__go_clone_box_interface(self.as_ref())") {
		t.Fatalf("Clone impl should qualify the local trait clone helper:\n%s", rust)
	}
	if strings.Contains(rust, "self.__go_clone_box_interface()") {
		t.Fatalf("Clone impl should not use ambiguous method-call syntax:\n%s", rust)
	}
}

func TestFunctionTypeInterfaceWrapperUsesMutableTraitReceiver(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node interface {
	node()
}

type Visitor interface {
	Visit(node Node) Visitor
}

type inspector func(Node) bool

func (f inspector) Visit(node Node) Visitor {
	if f(node) {
		return f
	}
	return nil
}

type counter struct {
	n int
}

func (c *counter) Visit(node Node) Visitor {
	c.n = c.n + 1
	return c
}

func Walk(v Visitor, node Node) {
	v = v.Visit(node)
}

func Inspect(node Node, f func(Node) bool) {
	Walk(inspector(f), node)
}
`)

	if strings.Contains(rust, "impl Visitor for inspectorAsVisitor {\n    fn visit(&self") {
		t.Fatalf("function-type interface wrapper should match mutable trait receiver:\n%s", rust)
	}
	if !strings.Contains(rust, "impl Visitor for inspectorAsVisitor {\n    fn visit(&mut self") {
		t.Fatalf("function-type interface wrapper should use &mut self for mutable trait method:\n%s", rust)
	}
}

func TestFunctionTypeWrapperForEmbeddedInterfaceImplementsSupertrait(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Value interface {
	Set(string) error
	String() string
}

type BoolFlag interface {
	Value
	IsBoolFlag() bool
}

type boolFunc func(string) error

func (f boolFunc) Set(s string) error { return f(s) }
func (f boolFunc) String() string { return "" }
func (f boolFunc) IsBoolFlag() bool { return true }

func bind(v BoolFlag) {}

func use(f func(string) error) {
	bind(boolFunc(f))
}
`)

	if !strings.Contains(rust, "impl Value for boolFuncAsBoolFlag") {
		t.Fatalf("function-type wrapper for embedded interface should implement the supertrait on the same wrapper:\n%s", rust)
	}

	implHeader := "impl BoolFlag for boolFuncAsBoolFlag {"
	start := strings.Index(rust, implHeader)
	if start < 0 {
		t.Fatalf("missing BoolFlag impl for function-type wrapper:\n%s", rust)
	}
	boolFlagImpl := rust[start:]
	if next := strings.Index(boolFlagImpl[len(implHeader):], "\nimpl "); next >= 0 {
		boolFlagImpl = boolFlagImpl[:len(implHeader)+next]
	}

	if strings.Contains(boolFlagImpl, "fn set(") || strings.Contains(boolFlagImpl, "fn string(") {
		t.Fatalf("embedded Value methods should live in the Value impl, not BoolFlag impl:\n%s", boolFlagImpl)
	}
	if strings.Contains(boolFlagImpl, "fn __go_as_any(") {
		t.Fatalf("embedded interface support method should be inherited from the supertrait impl:\n%s", boolFlagImpl)
	}
	if !strings.Contains(boolFlagImpl, "fn is_bool_flag(") {
		t.Fatalf("BoolFlag impl should still emit its direct method:\n%s", boolFlagImpl)
	}
}

func TestFunctionTypeInterfaceWrapperQualifiesSameSignatureMethodTrait(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Value interface {
	Set(string) error
	String() string
}

type BoolFlag interface {
	Value
	IsBoolFlag() bool
}

type funcValue func(string) error

func (f funcValue) Set(s string) error { return f(s) }
func (f funcValue) String() string { return "" }

type boolFuncValue func(string) error

func (f boolFuncValue) Set(s string) error { return f(s) }
func (f boolFuncValue) String() string { return "" }
func (f boolFuncValue) IsBoolFlag() bool { return true }

func bindValue(v Value) {}
func bindBool(v BoolFlag) {}

func use(f func(string) error) {
	bindValue(funcValue(f))
	bindBool(boolFuncValue(f))
}
`)

	for _, want := range []string{
		"funcValueMethods::set(&self.0",
		"funcValueMethods::string(&self.0",
		"boolFuncValueMethods::set(&self.0",
		"boolFuncValueMethods::string(&self.0",
		"boolFuncValueMethods::is_bool_flag(&self.0",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("function-type interface wrapper should qualify same-signature method trait %q:\n%s", want, rust)
		}
	}
}

func TestEmbeddedInterfaceTraitObjectAdapterUsesMutableReceiver(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Importer interface {
	Import(path string)
}

type ImporterFrom interface {
	Importer
	ImportFrom(path string)
}

type loader struct {
	n int
}

func (l *loader) Import(path string) {
	l.n = l.n + 1
}

func (l *loader) ImportFrom(path string) {}
`)

	if strings.Contains(rust, "fn import(&self, path: Rc<RefCell<Option<String>>>)") {
		t.Fatalf("embedded interface trait-object adapter should not use &self for mutable methods:\n%s", rust)
	}
	if !strings.Contains(rust, "impl Importer for Box<dyn ImporterFrom> {\n") ||
		!strings.Contains(rust, "fn import(&mut self, path: Rc<RefCell<Option<String>>>)") {
		t.Fatalf("embedded interface trait-object adapter should use &mut self for mutable methods:\n%s", rust)
	}
}

func TestInterfaceMethodMutableReceiverMatchesEquivalentMethodObject(t *testing.T) {
	first := interfaceMethodFromSource(t, `package main

type Importer interface {
	Import(path string)
}
`, "Importer", "Import")
	second := interfaceMethodFromSource(t, `package main

type Importer interface {
	Import(path string)
}
`, "Importer", "Import")
	if first == second {
		t.Fatal("test requires distinct method objects")
	}

	prevInterfaceMethodMutableReceiver := interfaceMethodMutableReceiver
	prevInterfaceMethodMutableReceiverByName := interfaceMethodMutableReceiverByName
	prevInterfaceMethodMutableReceiverByTrait := interfaceMethodMutableReceiverByTrait
	interfaceMethodMutableReceiver = map[*types.Func]bool{first: true}
	interfaceMethodMutableReceiverByName = make(map[string]bool)
	interfaceMethodMutableReceiverByTrait = map[string]bool{
		interfaceMethodMutableReceiverTraitKey("main::Importer", first.Name()): true,
	}
	t.Cleanup(func() {
		interfaceMethodMutableReceiver = prevInterfaceMethodMutableReceiver
		interfaceMethodMutableReceiverByName = prevInterfaceMethodMutableReceiverByName
		interfaceMethodMutableReceiverByTrait = prevInterfaceMethodMutableReceiverByTrait
	})

	if !interfaceMethodRequiresMutableReceiver(second) {
		t.Fatalf("equivalent interface method object should keep mutable receiver decision")
	}
	if !interfaceTraitMethodRequiresMutableReceiver("main::Importer", second.Name(), second) {
		t.Fatalf("equivalent imported trait method should keep mutable receiver decision")
	}
	if !interfaceTraitMethodRequiresMutableReceiver("main::Importer", second.Name(), nil) {
		t.Fatalf("trait method name should keep mutable receiver decision without a method object")
	}
}

func interfaceMethodFromSource(t *testing.T, src string, ifaceName string, methodName string) *types.Func {
	t.Helper()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	obj, ok := typeInfo.pkg.Scope().Lookup(ifaceName).(*types.TypeName)
	if !ok {
		t.Fatalf("interface %s not found", ifaceName)
	}
	iface, ok := types.Unalias(obj.Type()).Underlying().(*types.Interface)
	if !ok {
		t.Fatalf("%s is not an interface", ifaceName)
	}
	iface.Complete()
	for i := 0; i < iface.NumMethods(); i++ {
		method := iface.Method(i)
		if method.Name() == methodName {
			return method
		}
	}
	t.Fatalf("method %s not found on %s", methodName, ifaceName)
	return nil
}

func TestInterfaceKeywordNameUsesIdentifierSafeHelperSuffix(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	String() string
}
`)

	if strings.Contains(rust, "__go_clone_box_r#type") || strings.Contains(rust, "__go_eq_r#type") {
		t.Fatalf("interface helper suffix should not use raw identifiers inside larger names:\n%s", rust)
	}
	if !strings.Contains(rust, "fn __go_clone_box_type_(&self)") {
		t.Fatalf("keyword-derived interface helper suffix should be identifier-safe:\n%s", rust)
	}
	if !strings.Contains(rust, "fn __go_eq_type_(&self, other: &dyn Type)") {
		t.Fatalf("keyword-derived equality helper suffix should be identifier-safe:\n%s", rust)
	}
}

func TestBlankStructFieldsUseGeneratedRustFieldNames(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type CacheLinePad struct{ _ [8]byte }

var Features struct {
	_ CacheLinePad
	Enabled bool
	_ CacheLinePad
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
	for _, invalid := range []string{"pub _:", " _:", "self._."} {
		if strings.Contains(rust, invalid) {
			t.Fatalf("blank struct fields should not emit invalid Rust field %q:\n%s", invalid, rust)
		}
	}
	for _, want := range []string{"pub __blank_0_0", "__blank_0_0:", "__blank_2_0:"} {
		if !strings.Contains(rust, want) {
			t.Fatalf("blank struct field should use generated field name %q:\n%s", want, rust)
		}
	}
}

func TestAnonymousStructFieldsArePackageVisible(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

var Features struct {
	Enabled bool
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
	if !strings.Contains(rust, "pub struct AnonymousStruct1") {
		t.Fatalf("anonymous package-global struct type should be visible across generated modules:\n%s", rust)
	}
	if !strings.Contains(rust, "pub enabled:") {
		t.Fatalf("anonymous package-global struct fields should be visible across generated modules:\n%s", rust)
	}
}

func TestAnonymousStructEmbeddedFieldsAreDeclared(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type PtrType struct {
	N int
}

var Holder struct {
	PtrType
	U int
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
	if strings.Contains(rust, "WARNING: embedded field in anonymous struct") {
		t.Fatalf("anonymous embedded struct field should be declared, not warned:\n%s", rust)
	}
	if strings.Contains(rust, "/* unknown struct */") {
		t.Fatalf("anonymous embedded struct package global should use the registered anonymous struct type:\n%s", rust)
	}
	if !strings.Contains(rust, "pub ptr_type: Rc<RefCell<Option<PtrType>>>") {
		t.Fatalf("anonymous embedded struct field should be emitted with the generated field name:\n%s", rust)
	}
	if !strings.Contains(rust, "Self { ptr_type:") {
		t.Fatalf("anonymous embedded struct clone/default paths should reference declared field:\n%s", rust)
	}
}

func TestAnonymousStructEmbeddedLocalMethodsAreForwarded(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type List struct {
	head int
}

func (l *List) empty() bool {
	return l.head == 0
}

func (l *List) pop() int {
	v := l.head
	l.head = 0
	return v
}

var holder struct {
	List
	n int
}

func use() int {
	if holder.empty() {
		return 0
	}
	return holder.pop()
}

func main() {
	go func() {}()
}
`)

	promotedImplStart := strings.Index(rust, "impl AnonymousStruct1 {\n    pub fn empty(&self) -> bool")
	if promotedImplStart < 0 {
		t.Fatalf("anonymous embedded struct should forward read-only promoted methods:\n%s", rust)
	}
	promotedImpl := rust[promotedImplStart:]
	if !strings.Contains(promotedImpl, "    pub fn pop(&mut self) -> i32") {
		t.Fatalf("anonymous embedded struct should forward mutating promoted methods:\n%s", rust)
	}
	if !strings.Contains(promotedImpl, "let embedded = self.list.clone();") {
		t.Fatalf("anonymous promoted method should delegate through the embedded field:\n%s", rust)
	}
}

func TestGenericEmbeddedFieldUsesBaseTypeName(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type node[K comparable, V any] struct {
	isEntry bool
}

type entry[K comparable, V any] struct {
	node[K, V]
	value V
}

func newEntry[K comparable, V any](value V) *entry[K, V] {
	return &entry[K, V]{
		node:  node[K, V]{isEntry: true},
		value: value,
	}
}

func header[K comparable, V any](e *entry[K, V]) *node[K, V] {
	return &e.node
}
`)

	if strings.Contains(rust, "pub embedded:") || strings.Contains(rust, ".embedded") {
		t.Fatalf("generic embedded field should not use fallback embedded name:\n%s", rust)
	}
	if !strings.Contains(rust, "pub node: Rc<RefCell<Option<node<K, V>>>>") &&
		!strings.Contains(rust, "pub node: Arc<Mutex<Option<node<K, V>>>>") {
		t.Fatalf("generic embedded field should use the base type name:\n%s", rust)
	}
	if !strings.Contains(rust, ".node") {
		t.Fatalf("selector for generic embedded field should use the declared field name:\n%s", rust)
	}
	if strings.Contains(rust, "Some())") {
		t.Fatalf("generic struct composite literal should emit the struct value inside Some:\n%s", rust)
	}
	if strings.Contains(rust, "Some(entry<K, V> {") || strings.Contains(rust, "Some(node<K, V> {") {
		t.Fatalf("generic struct literal should use Rust turbofish path syntax:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(entry::<K, V> {") || !strings.Contains(rust, "Some(node::<K, V> {") {
		t.Fatalf("generic struct literal should include Rust turbofish path syntax:\n%s", rust)
	}
}

func TestPackageGlobalNewAnonymousStructRegistersType(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Type interface {
	String() string
}

var reserved = new(struct{ Type })
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "/* unknown struct */") {
		t.Fatalf("new(struct{ ... }) package global should register the anonymous struct type:\n%s", rust)
	}
	if !strings.Contains(rust, "AnonymousStruct1") {
		t.Fatalf("new(struct{ ... }) package global should use the generated anonymous struct type:\n%s", rust)
	}
	if !strings.Contains(rust, "impl Type for AnonymousStruct1") {
		t.Fatalf("anonymous struct embedding an interface should implement that interface:\n%s", rust)
	}
	if !strings.Contains(rust, "embedded_ref.string()") {
		t.Fatalf("anonymous struct interface impl should delegate to the embedded interface field:\n%s", rust)
	}
}

func TestLocalInterfaceForwardMethodUsesInherentCall(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	Underlying() Type
}

type Interface struct{}

func (t *Interface) Underlying() Type { return t }

type Named struct {
	typ Type
}

func (t *Named) Underlying() Type {
	t.typ = t
	return t.typ
}
`)

	if strings.Contains(rust, "fn underlying(&mut self)") && strings.Contains(rust, "self.underlying()") {
		t.Fatalf("mutable trait forwarder must not recursively dispatch through the trait method:\n%s", rust)
	}
	if !strings.Contains(rust, "Interface::underlying(self)") {
		t.Fatalf("mutable trait forwarder should call the inherent method explicitly:\n%s", rust)
	}
}

func TestTypedNilPointerReceiverMethodCallPreservesNilReceiver(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Checker struct {
	cleaned bool
}

type Alias struct{}

func NewAlias() *Alias {
	return (*Checker)(nil).newAlias()
}

func (check *Checker) newAlias() *Alias {
	a := &Alias{}
	if check != nil {
		check.cleaned = true
	}
	return a
}
`)

	if strings.Contains(rust, "None::<Checker>") && strings.Contains(rust, ".as_mut().unwrap()).new_alias(") {
		t.Fatalf("typed nil pointer receiver call must not unwrap the nil receiver before dispatch:\n%s", rust)
	}
	if !strings.Contains(rust, "__go_nil_recv_checker_new_alias") {
		t.Fatalf("typed nil pointer receiver call should dispatch through an explicit receiver helper:\n%s", rust)
	}
	if !strings.Contains(rust, "if (*check.borrow()).is_some()") {
		t.Fatalf("typed nil pointer receiver helper should preserve the receiver nil check:\n%s", rust)
	}
}

func TestPackageGlobalOsArgsUsesSharedHelper(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"os"
	"strings"
)

var inTest = len(os.Args) > 0 && strings.HasSuffix(strings.TrimSuffix(os.Args[0], ".exe"), ".test")

func main() {
	_ = inTest
}
`)

	if !strings.Contains(rust, "fn go_os_args()") {
		t.Fatalf("package global os.Args should use a shared helper:\n%s", rust)
	}
	if strings.Contains(rust, "__go_os_args") {
		t.Fatalf("package global os.Args should not reference a function-local binding:\n%s", rust)
	}
}

func TestAnonymousStructEmbeddedMutexPromotedLockUsesMutexField(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

var cache struct {
	sync.Mutex
	n int
}

func lockCache() {
	cache.Lock()
	defer cache.Unlock()
}
`)

	if strings.Contains(rust, ".as_mut().unwrap()).lock()") ||
		strings.Contains(rust, ".as_mut().unwrap()).unlock()") {
		t.Fatalf("promoted sync.Mutex methods on anonymous structs should not call methods on the outer struct:\n%s", rust)
	}
	if !strings.Contains(rust, ".mutex.clone(); let __mutex_guard_") {
		t.Fatalf("promoted sync.Mutex Lock should acquire the embedded mutex field:\n%s", rust)
	}
	if !strings.Contains(rust, "// mu.Unlock() handled by RAII guard") {
		t.Fatalf("promoted sync.Mutex deferred Unlock should be handled by the active guard:\n%s", rust)
	}
}

func TestMethodReceiverEmbeddedMutexPromotedLockUsesMutexField(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "sync"

type mmapper struct {
	sync.Mutex
	active int
}

func (m *mmapper) lock() {
	m.Lock()
	defer m.Unlock()
	m.active = 1
}
`)

	if strings.Contains(rust, "self.lock().unwrap()") ||
		strings.Contains(rust, "self.borrow().unwrap()") {
		t.Fatalf("promoted sync.Mutex methods on method receivers should not unwrap self as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "self.mutex.clone(); let __mutex_guard_") {
		t.Fatalf("promoted sync.Mutex Lock should acquire the receiver's embedded mutex field:\n%s", rust)
	}
	if !strings.Contains(rust, "// mu.Unlock() handled by RAII guard") {
		t.Fatalf("promoted sync.Mutex deferred Unlock should be handled by the active guard:\n%s", rust)
	}
}

func TestTranspileConstDeclUsesPackageVisibility(t *testing.T) {
	var out strings.Builder
	TranspileConstDecl(&out, &ast.GenDecl{
		Tok: token.CONST,
		Specs: []ast.Spec{&ast.ValueSpec{
			Names: []*ast.Ident{ast.NewIdent("Future"), ast.NewIdent("privateValue")},
			Values: []ast.Expr{
				&ast.BasicLit{Kind: token.STRING, Value: `""`},
				&ast.BasicLit{Kind: token.INT, Value: "1"},
			},
		}},
	})

	got := out.String()
	if !strings.Contains(got, `pub const FUTURE: &'static str = "";`) {
		t.Fatalf("exported package const should be public, got:\n%s", got)
	}
	if !strings.Contains(got, `pub(crate) const PRIVATE_VALUE: i32 = 1;`) {
		t.Fatalf("private package const should be crate-visible, got:\n%s", got)
	}
}

func TestTranspileTypedIntegerConstDeclCastsBinaryInitializer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const heapAddrBits = 48
const is64 = 1
const maxAlloc int64 = (1 << heapAddrBits) - (1 - is64) * 1
const arenaBaseOffset uint64 = 0xffff800000000000*is64 + 1

func use() {
	_, _ = maxAlloc, arenaBaseOffset
}
`)

	if strings.Contains(rust, "pub(crate) const MAX_ALLOC: i64 = (1 << HEAP_ADDR_BITS)") {
		t.Fatalf("typed i64 const binary initializer should not stay at default integer width:\n%s", rust)
	}
	if !strings.Contains(rust, "HEAP_ADDR_BITS as i64") || !strings.Contains(rust, "IS64 as i64") {
		t.Fatalf("typed i64 const binary initializer should cast operands to the declared width:\n%s", rust)
	}
	if !strings.Contains(rust, "IS64 as u64") {
		t.Fatalf("typed u64 const binary initializer should cast named const operands to the declared width:\n%s", rust)
	}
}

func TestTranspileTypedIntegerConstDeclCastsImportedSelectorOperands(t *testing.T) {
	rust := transpileTypedRegression(t, `package runtime

import (
	"internal/goarch"
	"internal/goos"
)

const arenaBaseOffset uint64 = 0xffff800000000000*goarch.IsAmd64 + 0x0a00000000000000*goos.IsAix

func use() {
	_ = arenaBaseOffset
}
`)

	if strings.Contains(rust, "0xffff800000000000 * internal_goarch::IS_AMD64 +") ||
		strings.Contains(rust, "0x0a00000000000000 * internal_goos::IS_AIX") {
		t.Fatalf("typed u64 const binary initializer should not leave imported selector operands at their source width:\n%s", rust)
	}
	if strings.Count(rust, "as u64") < 4 {
		t.Fatalf("typed u64 const binary initializer should cast imported selector operands and literals to u64:\n%s", rust)
	}
}

func TestTranspileTypedIntegerConstDeclKeepsIotaSubstitution(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const (
	state0 uint32 = iota
	state1 uint32 = 1 << (iota - 1)
	state2
)

func use() {
	_, _, _ = state0, state1, state2
}
`)

	if strings.Contains(rust, "iota") {
		t.Fatalf("typed const binary initializer should preserve iota substitution:\n%s", rust)
	}
	if strings.Contains(rust, "(0 as u32) - (1 as u32)") {
		t.Fatalf("typed const binary initializer should not cast iota subtraction into unsigned underflow:\n%s", rust)
	}
}

func TestTranspileTypedIntegerConstConversionCastsShiftOperands(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const marker = uintptr(0xdeaddead | 0xdeaddead<<32)

func use() uintptr {
	return marker
}
`)

	if strings.Contains(rust, "0xdeaddead << 32") {
		t.Fatalf("integer const conversion should not leave shift operands at default integer width:\n%s", rust)
	}
	if !strings.Contains(rust, "0xdeaddead as usize") || !strings.Contains(rust, "32 as usize") {
		t.Fatalf("integer const conversion should cast shift operands to the conversion width:\n%s", rust)
	}
}

func TestTranspileNestedIntegerConstConversionCastsShiftOperands(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type pallocSum uint64

const pallocChunkPages = 1 << 9
const logMaxPackedValue = 21
const freeChunkSum = pallocSum(uint64(pallocChunkPages) |
	uint64(pallocChunkPages<<logMaxPackedValue) |
	uint64(pallocChunkPages<<(2*logMaxPackedValue)))

func use() pallocSum {
	return freeChunkSum
}
`)

	if strings.Contains(rust, "PALLOC_CHUNK_PAGES << LOG_MAX_PACKED_VALUE") ||
		strings.Contains(rust, "PALLOC_CHUNK_PAGES << (2 * LOG_MAX_PACKED_VALUE)") {
		t.Fatalf("nested integer const conversion should not leave shift operands at default integer width:\n%s", rust)
	}
	if !strings.Contains(rust, "PALLOC_CHUNK_PAGES as u64") ||
		!strings.Contains(rust, "LOG_MAX_PACKED_VALUE as u64") ||
		!strings.Contains(rust, "2 as u64") {
		t.Fatalf("nested integer const conversion should cast shift operands to the conversion width:\n%s", rust)
	}
}

func TestMethodMutatesReceiverDetectsReceiverAssignments(t *testing.T) {
	readOnly := &ast.FuncDecl{
		Body: &ast.BlockStmt{List: []ast.Stmt{
			&ast.ReturnStmt{Results: []ast.Expr{
				&ast.SelectorExpr{X: ast.NewIdent("k"), Sel: ast.NewIdent("name")},
			}},
		}},
	}
	if methodMutatesReceiver(readOnly, "k") {
		t.Fatalf("read-only receiver field access should not require &mut self")
	}

	mutating := &ast.FuncDecl{
		Body: &ast.BlockStmt{List: []ast.Stmt{
			&ast.AssignStmt{
				Lhs: []ast.Expr{&ast.SelectorExpr{X: ast.NewIdent("k"), Sel: ast.NewIdent("name")}},
				Tok: token.ASSIGN,
				Rhs: []ast.Expr{&ast.BasicLit{Kind: token.STRING, Value: `"updated"`}},
			},
		}},
	}
	if !methodMutatesReceiver(mutating, "k") {
		t.Fatalf("receiver field assignment should require &mut self")
	}

	caller := &ast.FuncDecl{
		Name: ast.NewIdent("Update"),
		Recv: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("k")},
			Type:  &ast.StarExpr{X: ast.NewIdent("Thing")},
		}}},
		Body: &ast.BlockStmt{List: []ast.Stmt{
			&ast.ExprStmt{X: &ast.CallExpr{
				Fun: &ast.SelectorExpr{X: ast.NewIdent("k"), Sel: ast.NewIdent("Set")},
			}},
		}},
	}
	mutating.Name = ast.NewIdent("Set")
	mutating.Recv = &ast.FieldList{List: []*ast.Field{{
		Names: []*ast.Ident{ast.NewIdent("k")},
		Type:  &ast.StarExpr{X: ast.NewIdent("Thing")},
	}}}

	previousTypeMethods := currentTypeMethods
	currentTypeMethods = []*ast.FuncDecl{caller, mutating}
	defer func() { currentTypeMethods = previousTypeMethods }()

	if !methodMutatesReceiver(caller, "k") {
		t.Fatalf("receiver call to mutating receiver method should require &mut self")
	}
}

func TestValueReceiverCallingMutatingMethodUsesLocalReceiverCopy(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type writer struct {
	n int
}

func (w *writer) byte(v int) {
	w.n = v
}

func (w writer) refill() writer {
	w.byte(1)
	return w
}
`)

	if !strings.Contains(rust, "pub fn refill(&self)") {
		t.Fatalf("value receiver method should keep a shared Rust receiver:\n%s", rust)
	}
	if strings.Contains(rust, "pub fn refill(&mut self)") {
		t.Fatalf("value receiver method should not require mutable access to the caller's receiver:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut __self = self.clone();") {
		t.Fatalf("mutating value receiver method should create a mutable receiver copy:\n%s", rust)
	}
	if !strings.Contains(rust, "__self.byte(") {
		t.Fatalf("mutating value receiver method should call through the receiver copy:\n%s", rust)
	}
	if strings.Contains(rust, "\n        self.byte(") {
		t.Fatalf("mutating value receiver method should not call mutating methods through shared self:\n%s", rust)
	}
}

func TestMethodMutatesReceiverUsesPackageMethods(t *testing.T) {
	caller := &ast.FuncDecl{
		Name: ast.NewIdent("Read"),
		Recv: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("k")},
			Type:  &ast.StarExpr{X: ast.NewIdent("Thing")},
		}}},
		Body: &ast.BlockStmt{List: []ast.Stmt{
			&ast.ExprStmt{X: &ast.CallExpr{
				Fun: &ast.SelectorExpr{X: ast.NewIdent("k"), Sel: ast.NewIdent("Set")},
			}},
		}},
	}
	mutating := &ast.FuncDecl{
		Name: ast.NewIdent("Set"),
		Recv: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("k")},
			Type:  &ast.StarExpr{X: ast.NewIdent("Thing")},
		}}},
		Body: &ast.BlockStmt{List: []ast.Stmt{
			&ast.AssignStmt{
				Lhs: []ast.Expr{&ast.SelectorExpr{X: ast.NewIdent("k"), Sel: ast.NewIdent("name")}},
				Tok: token.ASSIGN,
				Rhs: []ast.Expr{&ast.BasicLit{Kind: token.STRING, Value: `"updated"`}},
			},
		}},
	}

	previousTypeMethods := currentTypeMethods
	currentTypeMethods = []*ast.FuncDecl{caller}
	defer func() { currentTypeMethods = previousTypeMethods }()

	previousCtx := GetTranspileContext()
	ctx := &TranspileContext{Package: NewPackageState()}
	ctx.Package.MethodsByType = map[string][]*ast.FuncDecl{"Thing": {caller, mutating}}
	SetTranspileContext(ctx)
	defer SetTranspileContext(previousCtx)

	if !methodMutatesReceiver(caller, "k") {
		t.Fatalf("receiver calls to mutating methods from another file should require &mut self")
	}
}

func TestGeneratePromotedMethodKeepsReadOnlyPointerReceiverShared(t *testing.T) {
	method := &ast.FuncDecl{
		Name: ast.NewIdent("String"),
		Recv: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("p")},
			Type:  &ast.StarExpr{X: ast.NewIdent("Package")},
		}}},
		Type: &ast.FuncType{
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("string")}}},
		},
		Body: &ast.BlockStmt{List: []ast.Stmt{
			&ast.ReturnStmt{Results: []ast.Expr{
				&ast.SelectorExpr{X: ast.NewIdent("p"), Sel: ast.NewIdent("ID")},
			}},
		}},
	}

	var out strings.Builder
	generatePromotedMethod(&out, method, "", "Package")

	got := out.String()
	if !strings.Contains(got, "pub fn string(&self)") {
		t.Fatalf("read-only promoted pointer method should use &self, got:\n%s", got)
	}
	if strings.Contains(got, "&mut self") {
		t.Fatalf("read-only promoted pointer method should not require &mut self, got:\n%s", got)
	}
	if !strings.Contains(got, "let guard = embedded") || !strings.Contains(got, "guard.as_ref().unwrap()") {
		t.Fatalf("read-only promoted pointer method should borrow embedded value immutably, got:\n%s", got)
	}
	if strings.Contains(got, "as_mut().unwrap()") {
		t.Fatalf("read-only promoted pointer method should not mutably borrow embedded value, got:\n%s", got)
	}
}

func TestPromotedMethodRustNameCollisionDoesNotDuplicateImpl(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Inner struct{}

func (Inner) Common() int { return 1 }

type Outer struct {
	Inner
}

func (Outer) common() int { return 2 }
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	implIndex := strings.LastIndex(rust, "impl Outer {")
	if implIndex < 0 {
		t.Fatalf("generated Rust did not contain impl Outer:\n%s", rust)
	}
	outerImpl := rust[implIndex:]
	if nextImpl := strings.Index(outerImpl[len("impl Outer {"):], "\nimpl "); nextImpl >= 0 {
		outerImpl = outerImpl[:len("impl Outer {")+nextImpl]
	}
	if count := strings.Count(outerImpl, "pub fn common(&self)"); count != 1 {
		t.Fatalf("promoted method Rust-name collision should not duplicate common in impl Outer, got %d:\n%s", count, rust)
	}
}

func TestCurrentReceiverPromotedMethodRustNameCollisionDelegatesToEmbedded(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type flag uintptr

func (f flag) kind() int {
	return int(f)
}

type Value struct {
	flag
}

func (v Value) Kind() int {
	return v.kind()
}
`)

	implIndex := strings.LastIndex(rust, "impl Value {")
	if implIndex < 0 {
		t.Fatalf("generated Rust did not contain impl Value:\n%s", rust)
	}
	valueImpl := rust[implIndex:]
	if nextImpl := strings.Index(valueImpl[len("impl Value {"):], "\nimpl "); nextImpl >= 0 {
		valueImpl = valueImpl[:len("impl Value {")+nextImpl]
	}
	if strings.Contains(valueImpl, "self.kind()") {
		t.Fatalf("current receiver promoted method call should not recurse through same Rust method name:\n%s", rust)
	}
	if !strings.Contains(valueImpl, "let __promoted_recv = self.flag.clone()") ||
		!strings.Contains(valueImpl, "__promoted_ref.kind()") {
		t.Fatalf("current receiver promoted method call should delegate through embedded field:\n%s", rust)
	}
}

func TestCurrentReceiverNestedPromotedMethodRustNameCollisionDelegatesThroughPath(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type struct{}

func (t *Type) Uncommon() int {
	return 1
}

type InterfaceType struct {
	Type
}

type interfaceType struct {
	InterfaceType
}

func (t *interfaceType) uncommon() int {
	return t.Uncommon()
}
`)

	implIndex := strings.LastIndex(rust, "impl interfaceType {")
	if implIndex < 0 {
		t.Fatalf("generated Rust did not contain impl interfaceType:\n%s", rust)
	}
	interfaceImpl := rust[implIndex:]
	if nextImpl := strings.Index(interfaceImpl[len("impl interfaceType {"):], "\nimpl "); nextImpl >= 0 {
		interfaceImpl = interfaceImpl[:len("impl interfaceType {")+nextImpl]
	}
	if strings.Contains(interfaceImpl, "self.uncommon()") {
		t.Fatalf("nested promoted method call should not recurse through same Rust method name:\n%s", rust)
	}
	if !strings.Contains(interfaceImpl, "let __promoted_recv_0 = self.interface_type.clone()") ||
		!strings.Contains(interfaceImpl, "let __promoted_recv_1 = __promoted_ref_0.r#type.clone()") ||
		!strings.Contains(interfaceImpl, "__promoted_ref_1.uncommon()") {
		t.Fatalf("nested promoted method call should delegate through embedded field path:\n%s", rust)
	}
}

func TestImportedTypeAliasDoesNotGetLocalImplBlock(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "io/fs"

type timeout interface {
	Timeout() bool
}

type PathError = fs.PathError
`)

	if strings.Contains(rust, "impl PathError {") {
		t.Fatalf("imported type alias should not get an inherent impl block:\n%s", rust)
	}
	if strings.Contains(rust, "impl timeout for PathError") {
		t.Fatalf("imported type alias should not get a local trait impl under the alias name:\n%s", rust)
	}
}

func TestGeneratePromotedMethodKeepsMutatingPointerReceiverMutable(t *testing.T) {
	method := &ast.FuncDecl{
		Name: ast.NewIdent("Set"),
		Recv: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("p")},
			Type:  &ast.StarExpr{X: ast.NewIdent("Package")},
		}}},
		Type: &ast.FuncType{},
		Body: &ast.BlockStmt{List: []ast.Stmt{
			&ast.AssignStmt{
				Lhs: []ast.Expr{&ast.SelectorExpr{X: ast.NewIdent("p"), Sel: ast.NewIdent("ID")}},
				Tok: token.ASSIGN,
				Rhs: []ast.Expr{&ast.BasicLit{Kind: token.STRING, Value: `"updated"`}},
			},
		}},
	}

	var out strings.Builder
	generatePromotedMethod(&out, method, "", "Package")

	got := out.String()
	if !strings.Contains(got, "pub fn set(&mut self)") {
		t.Fatalf("mutating promoted pointer method should use &mut self, got:\n%s", got)
	}
	if !strings.Contains(got, "let mut guard = embedded") || !strings.Contains(got, "guard.as_mut().unwrap()") {
		t.Fatalf("mutating promoted pointer method should borrow embedded value mutably, got:\n%s", got)
	}
}

func TestGeneratePromotedMethodEscapesKeywordParams(t *testing.T) {
	method := &ast.FuncDecl{
		Name: ast.NewIdent("Count"),
		Recv: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("p")},
			Type:  &ast.StarExpr{X: ast.NewIdent("Package")},
		}}},
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{{
				Names: []*ast.Ident{ast.NewIdent("match")},
				Type:  ast.NewIdent("int"),
			}}},
			Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent("int")}}},
		},
		Body: &ast.BlockStmt{},
	}

	var out strings.Builder
	generatePromotedMethod(&out, method, "", "Package")

	got := out.String()
	if strings.Contains(got, " match:") || strings.Contains(got, "(match)") {
		t.Fatalf("promoted method should not emit unescaped Rust keyword parameter:\n%s", got)
	}
	if !strings.Contains(got, "r#match:") || !strings.Contains(got, "count(r#match)") {
		t.Fatalf("promoted method should escape keyword parameter consistently:\n%s", got)
	}
}

func TestGeneratePromotedMethodNamesAndForwardsUnnamedParams(t *testing.T) {
	method := &ast.FuncDecl{
		Name: ast.NewIdent("ReadFrom"),
		Recv: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("n")},
			Type:  ast.NewIdent("noReadFrom"),
		}}},
		Type: &ast.FuncType{
			Params: &ast.FieldList{List: []*ast.Field{{
				Type: ast.NewIdent("Reader"),
			}}},
			Results: &ast.FieldList{List: []*ast.Field{
				{Type: ast.NewIdent("int64")},
				{Type: ast.NewIdent("error")},
			}},
		},
		Body: &ast.BlockStmt{},
	}

	var out strings.Builder
	generatePromotedMethod(&out, method, "", "File")

	got := out.String()
	if strings.Contains(got, "read_from()") {
		t.Fatalf("promoted method with unnamed parameter should not drop the forwarded arg:\n%s", got)
	}
	if !strings.Contains(got, "__arg0: Rc<RefCell<Option<Reader>>>") {
		t.Fatalf("promoted method with unnamed parameter should synthesize an argument name:\n%s", got)
	}
	if !strings.Contains(got, "embedded_ref.read_from(__arg0)") {
		t.Fatalf("promoted method should forward synthesized unnamed parameter:\n%s", got)
	}
}

func TestExternalEmbeddedInterfacePromotesMethods(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type byteReader struct {
	fmt.ScanState
}

func (r byteReader) ReadByte() (byte, error) {
	ch, _, err := r.ReadRune()
	return byte(ch), err
}
`)

	if !strings.Contains(rust, "pub fn read_rune(&self)") {
		t.Fatalf("embedded external interface should promote its methods onto the outer type:\n%s", rust)
	}
	if !strings.Contains(rust, "embedded_ref.read_rune()") {
		t.Fatalf("promoted external interface method should delegate through the embedded field:\n%s", rust)
	}
}

func TestSourceMappedEmbeddedRawFieldPromotedMethodsUseFieldValue(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "math/big", `package big

import "sync"

var divisors struct {
	sync.Mutex
	n int
}

func use() {
	go func() {}()
	divisors.Lock()
	divisors.Unlock()
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, "let mut guard = embedded.lock().unwrap()") ||
		strings.Contains(rust, "let guard = embedded.lock().unwrap()") {
		t.Fatalf("source-mapped raw embedded field should not be unwrapped as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn lock(&mut self)") {
		t.Fatalf("mutable promoted source-mapped method should require a mutable receiver:\n%s", rust)
	}
	if !strings.Contains(rust, "let embedded_ref = &mut self.mutex;") {
		t.Fatalf("promoted source-mapped sync.Mutex method should borrow the raw field directly:\n%s", rust)
	}
	if !strings.Contains(rust, "embedded_ref.lock()") ||
		!strings.Contains(rust, "embedded_ref.unlock()") {
		t.Fatalf("promoted source-mapped sync.Mutex methods should delegate to the generated field value:\n%s", rust)
	}
}
