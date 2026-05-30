package main

import (
	"go/ast"
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
	writeStructDerive(&out, "Iter", structType)
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
	if !strings.Contains(rust, "pub fn subst_list<T: Any + Clone + 'static>") ||
		strings.Contains(rust, "PartialEq") {
		t.Fatalf("generic comparable type parameter should not require raw Rust PartialEq:\n%s", rust)
	}
	if !strings.Contains(rust, "::ptr_eq(&__left, &__right)") {
		t.Fatalf("generic comparable handle values should compare handles:\n%s", rust)
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

	want := "pub fn identity<T: Any + Clone + 'static>(x: Rc<RefCell<Option<T>>>) -> Rc<RefCell<Option<T>>>"
	if !strings.Contains(rust, want) {
		t.Fatalf("generic any return should preserve the type parameter in the result signature, want %q:\n%s", want, rust)
	}
	if strings.Contains(rust, "Box<dyn Any") {
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

	want := "pub fn keep<N: GoInteger + Clone + 'static>(num: Rc<RefCell<Option<N>>>) -> Rc<RefCell<Option<N>>>"
	if !strings.Contains(rust, want) {
		t.Fatalf("generic union-constrained parameter should preserve the type parameter in the signature, want %q:\n%s", want, rust)
	}
	if strings.Contains(rust, "num: Rc<RefCell<Option<i64>>>") {
		t.Fatalf("generic union-constrained parameter should not lower to the first constraint term:\n%s", rust)
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

	want := "pub fn clone<P: Clone + 'static, T: Any + Clone + 'static>(p: Rc<RefCell<Option<P>>>) -> Rc<RefCell<Option<P>>>"
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
	generatePromotedMethod(&out, method, "Package")

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
	generatePromotedMethod(&out, method, "Package")

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
	generatePromotedMethod(&out, method, "Package")

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
	generatePromotedMethod(&out, method, "File")

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
