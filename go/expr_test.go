package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
	"os"
	"strings"
	"testing"
)

type exprTestImporter map[string]*types.Package

func (imp exprTestImporter) Import(path string) (*types.Package, error) {
	pkg, ok := imp[path]
	if !ok {
		return nil, fmt.Errorf("unknown import %s", path)
	}
	return pkg, nil
}

func eventKeyEqualityImporter() types.Importer {
	labelPkg := types.NewPackage("golang.org/x/tools/internal/event/label", "label")
	keyTypeName := types.NewTypeName(token.NoPos, labelPkg, "Key", nil)
	nameMethod := types.NewFunc(token.NoPos, labelPkg, "Name", types.NewSignatureType(
		nil,
		nil,
		nil,
		nil,
		types.NewTuple(types.NewVar(token.NoPos, nil, "", types.Typ[types.String])),
		false,
	))
	keyIface := types.NewInterfaceType([]*types.Func{nameMethod}, nil)
	keyIface.Complete()
	types.NewNamed(keyTypeName, keyIface, nil)
	labelPkg.Scope().Insert(keyTypeName)
	labelPkg.MarkComplete()

	keysPkg := types.NewPackage("golang.org/x/tools/internal/event/keys", "keys")
	stringTypeName := types.NewTypeName(token.NoPos, keysPkg, "String", nil)
	stringNamed := types.NewNamed(stringTypeName, types.NewStruct(nil, nil), nil)
	stringPtr := types.NewPointer(stringNamed)
	stringNamed.AddMethod(types.NewFunc(token.NoPos, keysPkg, "Name", types.NewSignatureType(
		types.NewVar(token.NoPos, keysPkg, "k", stringPtr),
		nil,
		nil,
		nil,
		types.NewTuple(types.NewVar(token.NoPos, nil, "", types.Typ[types.String])),
		false,
	)))
	keysPkg.Scope().Insert(stringTypeName)
	keysPkg.Scope().Insert(types.NewVar(token.NoPos, keysPkg, "Msg", stringPtr))
	keysPkg.MarkComplete()

	return exprTestImporter{
		"golang.org/x/tools/internal/event/keys":  keysPkg,
		"golang.org/x/tools/internal/event/label": labelPkg,
	}
}

func goTypesConfigImporter() types.Importer {
	goTypesPkg := types.NewPackage("go/types", "types")
	importerTypeName := types.NewTypeName(token.NoPos, goTypesPkg, "Importer", nil)
	importMethod := types.NewFunc(token.NoPos, goTypesPkg, "Import", types.NewSignatureType(
		nil,
		nil,
		nil,
		types.NewTuple(types.NewVar(token.NoPos, nil, "path", types.Typ[types.String])),
		nil,
		false,
	))
	importerIface := types.NewInterfaceType([]*types.Func{importMethod}, nil)
	importerIface.Complete()
	importerNamed := types.NewNamed(importerTypeName, importerIface, nil)
	goTypesPkg.Scope().Insert(importerTypeName)

	configTypeName := types.NewTypeName(token.NoPos, goTypesPkg, "Config", nil)
	configStruct := types.NewStruct(
		[]*types.Var{types.NewVar(token.NoPos, goTypesPkg, "Importer", importerNamed)},
		nil,
	)
	types.NewNamed(configTypeName, configStruct, nil)
	goTypesPkg.Scope().Insert(configTypeName)
	goTypesPkg.MarkComplete()

	return exprTestImporter{"go/types": goTypesPkg}
}

func TestUnknownPositionalStructLiteralFallbackParses(t *testing.T) {
	var out strings.Builder
	prevStructDefs := structDefs
	structDefs = make(map[string]*StructDef)
	defer func() {
		structDefs = prevStructDefs
	}()

	TranspileExpression(&out, &ast.CompositeLit{
		Type: ast.NewIdent("External"),
		Elts: []ast.Expr{ast.NewIdent("value")},
	})

	got := out.String()
	if strings.Contains(got, "*/, ..Default::default()") {
		t.Fatalf("fallback comment must not create an empty struct field before default:\n%s", got)
	}
	if !strings.Contains(got, "External { /* ERROR: Type information required for positional struct literal */ ..Default::default() }") {
		t.Fatalf("unexpected fallback for unknown positional struct literal:\n%s", got)
	}
}

func TestUppercaseLocalVariableReferenceUsesVarTable(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func sum(s float64, h float64) float64 {
	R := s + h
	return s * (h + R)
}
`)

	if strings.Contains(rust, "let __tmp_y = R;") {
		t.Fatalf("uppercase local variable should not be emitted as a bare const-like identifier:\n%s", rust)
	}
	if !strings.Contains(rust, "(*R.borrow().as_ref().unwrap())") &&
		!strings.Contains(rust, "(*R.lock().unwrap().as_ref().unwrap())") {
		t.Fatalf("uppercase local variable reference should unwrap the tracked local variable:\n%s", rust)
	}
}

func TestSelectorStructCompositeLiteralUsesTypeInfo(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/types"

func main() {
	_ = &types.Info{}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	var composite *ast.CompositeLit
	ast.Inspect(file, func(n ast.Node) bool {
		if lit, ok := n.(*ast.CompositeLit); ok {
			if _, ok := lit.Type.(*ast.SelectorExpr); ok {
				composite = lit
				return false
			}
		}
		return true
	})
	if composite == nil {
		t.Fatal("did not find selector-qualified composite literal")
	}

	var out strings.Builder
	TranspileExpression(&out, composite)

	got := out.String()
	if !strings.Contains(got, "types_Info {") {
		t.Fatalf("selector-qualified struct literal should use package-qualified Rust type:\n%s", got)
	}
	if strings.Contains(got, "Some()") || strings.Contains(got, "(*.borrow") {
		t.Fatalf("selector-qualified struct literal emitted missing expression:\n%s", got)
	}
}

func TestRepeatedWrappedValueCallArgumentsUseShortGuard(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func add(a int, b int) int {
	return a + b
}

func main() {
	x := 2
	ch := make(chan int, 1)
	_ = add(x, x)
	ch <- x
}
`)

	bad := "Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()).clone())))"
	if strings.Contains(rust, bad) {
		t.Fatalf("repeated wrapped call args should not keep mutex guards alive for the full call:\n%s", rust)
	}
	want := "{ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }"
	if strings.Count(rust, want) < 2 {
		t.Fatalf("repeated wrapped call args should clone through short guard blocks:\n%s", rust)
	}
}

func TestEmptyStructLiteralInitializesEmbeddedFields(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type flag uintptr

type Value struct {
	typ *int
	ptr uintptr
	flag
}

func zero() Value {
	return Value{}
}
`)

	body := rust
	if idx := strings.Index(rust, "pub fn zero"); idx >= 0 {
		body = rust[idx:]
	}
	if !strings.Contains(body, "flag:") {
		t.Fatalf("empty struct literal should initialize embedded fields:\n%s", rust)
	}
}

func TestEmptyStructLiteralUsesNilForPointerAndErrorFields(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "time"

type ctxResult struct {
	err error
	timer *time.Timer
}

func zero() ctxResult {
	return ctxResult{}
}
`)

	if strings.Contains(rust, "Some(Arc::new") || strings.Contains(rust, "Some(Rc::new") {
		t.Fatalf("empty struct literal should not double-wrap nil error fields:\n%s", rust)
	}
	if strings.Contains(rust, "timer: Rc::new(RefCell::new(Some(Default::default())))") ||
		strings.Contains(rust, "timer: Arc::new(Mutex::new(Some(Default::default())))") {
		t.Fatalf("empty struct literal should not initialize pointer fields with a default pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "err: Default::default()") || !strings.Contains(rust, "timer: Default::default()") {
		t.Fatalf("empty struct literal should use nil handle defaults for error and pointer fields:\n%s", rust)
	}
}

func TestPositionalStructLiteralWrapsBareScalarLocalField(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type seq struct {
	stackBytes uintptr
}

type desc struct {
	call seq
	retOffset uintptr
}

func align(x uintptr) uintptr { return x }

func makeDesc(in seq) desc {
	retOffset := align(in.stackBytes)
	return desc{in, retOffset}
}
`)

	if strings.Contains(rust, "ret_offset: retOffset.clone()") {
		t.Fatalf("bare scalar local field should not be treated as an existing handle:\n%s", rust)
	}
	if !strings.Contains(rust, "ret_offset: Rc::new(RefCell::new(Some(retOffset)))") &&
		!strings.Contains(rust, "ret_offset: Arc::new(Mutex::new(Some(retOffset)))") {
		t.Fatalf("bare scalar local field should be wrapped for the struct field:\n%s", rust)
	}
}

func TestEmptyInterfaceEqualityUsesAnyEqHelper(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func boxed(v int) interface{} {
	return v
}

func same(x interface{}, y interface{}) bool {
	if x == nil || y == nil {
		return x == y
	}
	return boxed(1) == boxed(2)
}
`)

	if strings.Contains(rust, "__tmp_x == __tmp_y") && strings.Contains(rust, "Box<dyn Any") {
		t.Fatalf("empty-interface equality should not compare Box<dyn Any> values directly:\n%s", rust)
	}
	if !strings.Contains(rust, "go_any_eq(&x, &y)") {
		t.Fatalf("empty-interface variable equality should use go_any_eq:\n%s", rust)
	}
	if !strings.Contains(rust, "go_any_eq(&boxed(") {
		t.Fatalf("empty-interface call equality should use go_any_eq:\n%s", rust)
	}
}

func TestEmptyInterfaceConcreteEqualityBoxesConcreteSide(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Mutex struct{}

type P *struct{}

func same(x any) bool {
	go func() {}()
	return x == P(nil)
}
`)

	if strings.Contains(rust, "__tmp_x == __tmp_y") && strings.Contains(rust, "Box<dyn Any") {
		t.Fatalf("empty-interface concrete equality should not compare Box<dyn Any> values directly:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(P(Arc::new(StdMutex::new(None::<AnonymousStruct1>)))) as Box<dyn Any + Send + Sync>") {
		t.Fatalf("empty-interface concrete equality should box the concrete side:\n%s", rust)
	}
	if !strings.Contains(rust, "go_any_eq(&x, &__right_holder)") {
		t.Fatalf("empty-interface concrete equality should use go_any_eq:\n%s", rust)
	}
}

func TestLocalInterfaceReferenceCallArgumentUsesCurrentReceiver(t *testing.T) {
	prevReceiver := currentReceiver
	currentReceiver = "k"
	defer func() { currentReceiver = prevReceiver }()

	var out strings.Builder
	if !writeLocalInterfaceReferenceCallArgument(&out, ast.NewIdent("k"), nil) {
		t.Fatal("writeLocalInterfaceReferenceCallArgument returned false")
	}
	if got, want := out.String(), "self"; got != want {
		t.Fatalf("receiver argument = %q, want %q", got, want)
	}
}

func TestLocalInterfaceEqualityWithCurrentPointerReceiverWrapsSelf(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	typeNode()
}

type Named struct{}

func (*Named) typeNode() {}

func id(t Type) Type {
	return t
}

func (t *Named) same(orig Type) bool {
	rbase := id(orig)
	return rbase == t
}
`)

	if strings.Contains(rust, "let __right_holder = self.clone(); let __right_guard = __right_holder.borrow()") ||
		strings.Contains(rust, "let __right_holder = self.clone(); let __right_guard = __right_holder.lock()") {
		t.Fatalf("interface equality against current pointer receiver should not treat self as an existing handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __right_holder = Rc::new(RefCell::new(Some(self.clone())))") &&
		!strings.Contains(rust, "let __right_holder = Arc::new(Mutex::new(Some(self.clone())))") &&
		!strings.Contains(rust, "let __right_wrapper = NamedPtr(Rc::new(RefCell::new(Some(self.clone()))))") &&
		!strings.Contains(rust, "let __right_wrapper = NamedPtr(Arc::new(Mutex::new(Some(self.clone()))))") {
		t.Fatalf("interface equality against current pointer receiver should wrap self as a pointer handle:\n%s", rust)
	}
}

func TestLocalInterfaceEqualityChecksNilBeforeTraitDispatch(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Object interface {
	Name() string
}

type item struct{}

func (*item) Name() string { return "" }

func same(obj Object, alias Object) bool {
	return obj == alias
}
`)

	if strings.Contains(rust, "__left_guard.as_ref().unwrap().as_ref()") ||
		strings.Contains(rust, "__right_guard.as_ref().unwrap().as_ref()") {
		t.Fatalf("local interface equality should not unwrap nil slots before comparing:\n%s", rust)
	}
	if !strings.Contains(rust, "match (__left_opt, __right_opt)") {
		t.Fatalf("local interface equality should match on optional trait references:\n%s", rust)
	}
	if !strings.Contains(rust, "(None, None) => true") {
		t.Fatalf("local interface equality should treat two nil interfaces as equal:\n%s", rust)
	}
	if !strings.Contains(rust, "_ => false") {
		t.Fatalf("local interface equality should treat one nil interface as unequal:\n%s", rust)
	}
	if !strings.Contains(rust, "(Some(__left), Some(__right)) => __left.__go_eq_object(__right)") {
		t.Fatalf("local interface equality should still dispatch through the trait equality helper:\n%s", rust)
	}
}

func TestLocalInterfaceSupersetArgumentUsesTraitObjectAdapter(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type positioner interface {
	Pos() int
}

type object interface {
	Pos() int
	Name() string
}

func add(p positioner) {}

func declare(obj object) {
	add(obj)
}
`)

	if strings.Contains(rust, "let __inner: Box<dyn positioner") {
		t.Fatalf("interface argument with a structural method superset should not use a Rust trait upcast:\n%s", rust)
	}
	if !strings.Contains(rust, "impl positioner for Box<dyn object") {
		t.Fatalf("structural interface argument should have a trait-object adapter:\n%s", rust)
	}
	if strings.Contains(rust, "(**self).__go_eq_positioner(other)") {
		t.Fatalf("structural interface adapter should not delegate equality to a non-embedded helper:\n%s", rust)
	}
	if !strings.Contains(rust, `panic!("interface equality for structurally adapted object as positioner")`) {
		t.Fatalf("structural interface adapter equality should be loud when it cannot preserve Go semantics:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new((*obj.borrow().as_ref().unwrap()).clone()) as Box<dyn positioner") &&
		!strings.Contains(rust, "Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner") {
		t.Fatalf("structural interface argument should box the source trait object through the adapter:\n%s", rust)
	}
}

func TestStubBackedInterfaceIndexArgumentBoxesBareValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"go/ast"
	"go/token"
)

type positioner interface {
	Pos() token.Pos
}

func report(pos positioner) {}

func walk(args []ast.Expr) {
	report(args[1])
}
`)

	if strings.Contains(rust, "].clone().borrow()") || strings.Contains(rust, "].clone().lock()") {
		t.Fatalf("indexed stub-backed interface argument should box the bare indexed value, not dereference it as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new((*args.borrow().as_ref().unwrap())[(1) as usize].clone()) as Box<dyn positioner") {
		t.Fatalf("indexed stub-backed interface argument should box the indexed interface value:\n%s", rust)
	}
}

func TestLocalInterfaceAdaptersIgnoreInterfaceTypedPackageVars(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	String() string
}

var universeBool Type
`)

	if strings.Contains(rust, "impl universeBool for Box<dyn Type") {
		t.Fatalf("interface adapter generation should ignore interface-typed package vars:\n%s", rust)
	}
}

func TestLocalInterfaceCompositeLiteralArgumentBoxesBareValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type decl interface {
	node() int
}

type importDecl struct {
	n int
}

func (d importDecl) node() int {
	return d.n
}

func walk(f func(decl)) {
	f(importDecl{n: 1})
}
`)

	if strings.Contains(rust, "importDecl {") && (strings.Contains(rust, "}.borrow()") || strings.Contains(rust, "}.lock().unwrap()")) {
		t.Fatalf("local interface composite literal argument should not be treated as a wrapper handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(importDecl {") || !strings.Contains(rust, "as Box<dyn decl") {
		t.Fatalf("local interface composite literal argument should box the bare struct value:\n%s", rust)
	}
}

func TestLocalInterfaceConcreteAssertionUsesTraitAny(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Expr interface {
	isExpr()
}

type TagExpr struct {
	Tag string
}

func (*TagExpr) isExpr() {}

func tagName(x Expr) string {
	if tag, ok := x.(*TagExpr); ok {
		return tag.Tag
	}
	return ""
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, "__go_as_any(any_val.as_ref()).downcast_ref::<TagExpr>()") &&
		!strings.Contains(rust, "__go_as_any(any_val.as_ref()).downcast_ref::<TagExprPtr>()") &&
		!strings.Contains(rust, "any_val.__go_as_any().downcast_ref::<TagExpr>()") &&
		!strings.Contains(rust, "any_val.__go_as_any().downcast_ref::<TagExprPtr>()") &&
		(!strings.Contains(rust, "let any_val = x.__go_as_any();") || (!strings.Contains(rust, "any_val.downcast_ref::<TagExpr>()") && !strings.Contains(rust, "any_val.downcast_ref::<TagExprPtr>()"))) {
		t.Fatalf("local interface concrete assertion should downcast through __go_as_any:\n%s", rust)
	}
	if strings.Contains(rust, "typed_val) = val.downcast_ref::<TagExpr>()") {
		t.Fatalf("local interface concrete assertion should not treat the trait object as bare Any:\n%s", rust)
	}
}

func TestAnonymousInterfaceAssertionWithMultipleImplementorsSynthesizesTrait(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type reader interface {
	read() int
}

type file struct{}

func (file) read() int {
	return 1
}

func (file) name() string {
	return "file"
}

type buffer struct{}

func (buffer) read() int {
	return 2
}

func (buffer) name() string {
	return "buffer"
}

func label(rc reader) string {
	if n, ok := rc.(interface{ name() string }); ok {
		return n.name()
	}
	return ""
}
`)

	if strings.Contains(rust, "unimplemented!(\"type info required: comma-ok assertion to anonymous interface") {
		t.Fatalf("anonymous interface assertion should not fall back to an unimplemented multi-candidate path:\n%s", rust)
	}
	if !strings.Contains(rust, "pub trait GoAnonymousInterface1") {
		t.Fatalf("anonymous interface assertion should synthesize a Rust trait:\n%s", rust)
	}
	if !strings.Contains(rust, "impl GoAnonymousInterface1 for buffer") ||
		!strings.Contains(rust, "impl GoAnonymousInterface1 for file") {
		t.Fatalf("anonymous interface assertion should implement the synthetic trait for every typed candidate:\n%s", rust)
	}
	if !strings.Contains(rust, "as Box<dyn GoAnonymousInterface1") {
		t.Fatalf("anonymous interface assertion should box successful candidates as the synthetic trait object:\n%s", rust)
	}
}

func TestAnyAssertionToPackageInterfaceUsesConcreteCandidates(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Expr interface {
	isExpr()
}

type TagExpr struct{}

func (TagExpr) isExpr() {}

func asExpr(x any) Expr {
	return x.(Expr)
}
`)

	if strings.Contains(rust, "downcast_ref::<Expr>()") {
		t.Fatalf("assertion to package interface should not downcast to the trait type:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<TagExpr>()") ||
		!strings.Contains(rust, "Box::new(typed_val.clone()) as Box<dyn Expr") {
		t.Fatalf("assertion to package interface should downcast concrete candidates and box the trait object:\n%s", rust)
	}
}

func TestConcreteAssertionReturnBoxesLocalInterface(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	Name() string
}

type rtype struct{}

func (*rtype) Name() string {
	return "rtype"
}

func cached(x any) Type {
	return x.(*rtype)
}
`)

	if strings.Contains(rust, "Some(({") && strings.Contains(rust, "downcast_ref::<rtype>()") {
		t.Fatalf("concrete assertion returned as local interface should not wrap an untyped expression directly:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(rtypePtr(") || !strings.Contains(rust, "downcast_ref::<rtype>()") || !strings.Contains(rust, "as Box<dyn Type") {
		t.Fatalf("concrete assertion returned as local interface should box a pointer-identity wrapper:\n%s", rust)
	}
}

func TestAssignedLocalInterfaceParamUsesWrappedShadow(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Expr interface {
	isExpr()
}

type TagExpr struct{}

func (*TagExpr) isExpr() {}

func rewrite(x Expr) Expr {
	x = &TagExpr{}
	return x
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, "x: Rc<RefCell<Option<Box<dyn Expr>>>>") &&
		!strings.Contains(rust, "x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>>") {
		t.Fatalf("local interface parameter should use the wrapped nilable handle shape:\n%s", rust)
	}
	if strings.Contains(rust, "let mut x: Rc<RefCell<Option<Box<dyn Expr>>>> = x.clone()") ||
		strings.Contains(rust, "let mut x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>> = x.clone()") {
		t.Fatalf("assigned local interface parameter must not shadow by cloning the caller's handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut x: Rc<RefCell<Option<Box<dyn Expr>>>> = Rc::new(RefCell::new(x.borrow().as_ref().map(|__v| Expr::__go_clone_box_expr(__v.as_ref()))))") &&
		!strings.Contains(rust, "let mut x: Arc<Mutex<Option<Box<dyn Expr + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Expr::__go_clone_box_expr(__v.as_ref()))))") {
		t.Fatalf("assigned local interface parameter should be shadowed with a freshly cloned wrapped value:\n%s", rust)
	}
}

func TestLocalInterfaceFieldConcreteAssertionUsesFieldHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Expr interface {
	isExpr()
}

type TagExpr struct{}

func (*TagExpr) isExpr() {}

type NotExpr struct {
	X Expr
}

func isTag(n *NotExpr) bool {
	_, ok := n.X.(*TagExpr)
	return ok
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "let val = (*{ let __field =") {
		t.Fatalf("type assertion on local interface field should not unwrap the field handle before assertion:\n%s", rust)
	}
	if !strings.Contains(rust, ".x.clone()") {
		t.Fatalf("type assertion on local interface field should clone the field handle:\n%s", rust)
	}
}

func TestStringConversionFromNamedStringAssertionUsesBareValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Text interface {
	text()
}

type Plain string

func (Plain) text() {}

func plain(x Text) string {
	return string(x.(Plain))
}
`)

	if strings.Contains(rust, "}).borrow()") || strings.Contains(rust, "}).lock()") {
		t.Fatalf("string conversion from named string assertion should not borrow the bare assertion result:\n%s", rust)
	}
	if !strings.Contains(rust, ".to_string()") {
		t.Fatalf("string conversion from named string assertion should stringify the bare value:\n%s", rust)
	}
}

func TestStringFieldCompositeLiteralUsesInnerStringValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Error struct {
	Expr string
}

func makeError(s string, n int) Error {
	return Error{Expr: s[:n]}
}
`)

	if strings.Contains(rust, "expr: Rc::new(RefCell::new(Some(Rc::new") ||
		strings.Contains(rust, "expr: Arc::new(Mutex::new(Some(Arc::new") {
		t.Fatalf("string field composite value should store the inner string, not a wrapped handle:\n%s", rust)
	}
}

func TestStructCompositeLiteralValueFieldCopiesWrappedIdent(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type span struct {
	start int
	end int
}

func collect() []span {
	start := 0
	spans := []span{}
	spans = append(spans, span{start, 3})
	start = 9
	return spans
}
`)

	if strings.Contains(rust, "span { start: start.clone()") {
		t.Fatalf("value field composite literal should not share the source local handle:\n%s", rust)
	}
	if !strings.Contains(rust, "start: Rc::new(RefCell::new(Some({ let __arg_holder = start.clone();") &&
		!strings.Contains(rust, "start: Arc::new(Mutex::new(Some({ let __arg_holder = start.clone();") {
		t.Fatalf("value field composite literal should copy the source local's current value:\n%s", rust)
	}
}

func TestSliceFieldCompositeLiteralUsesSliceExpressionHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Message struct {
	Data []byte
}

func makeMessage(b []byte, n int) Message {
	return Message{Data: b[1:n]}
}
`)

	if strings.Contains(rust, "data: Rc::new(RefCell::new(Some(Rc::new") ||
		strings.Contains(rust, "data: Arc::new(Mutex::new(Some(Arc::new") {
		t.Fatalf("slice field composite value should store the slice expression handle, not wrap it again:\n%s", rust)
	}
}

func TestFunctionValueFieldUsesRenamedFunction(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type mapper struct {
	mmap func() int
}

func Mmap() int {
	return 1
}

func mmap() int {
	return 2
}

func makeMapper() mapper {
	return mapper{mmap: mmap}
}
`)

	if strings.Contains(rust, "{ mmap() }) as Box<dyn FnMut() -> i32>") {
		t.Fatalf("function value should not call the colliding base Rust name:\n%s", rust)
	}
	if !strings.Contains(rust, "{ mmap_1() }) as Box<dyn FnMut() -> i32>") {
		t.Fatalf("function value should use the renamed Rust function target:\n%s", rust)
	}
}

func TestPointerAssertionToStructAliasUsesUnderlyingPointee(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Expr interface {
	expr()
}

type Ident struct {
	Name string
}

func (*Ident) expr() {}

type identType = Ident

func identName(n *identType) string {
	return n.Name
}

func firstName(values []Expr) string {
	for _, value := range values {
		if ident, ok := value.(*identType); ok {
			return identName(ident)
		}
	}
	return ""
}
`)

	if strings.Contains(rust, "downcast_ref::<identType>()") ||
		strings.Contains(rust, "None::<identType>") {
		t.Fatalf("pointer assertion to a struct alias should assert the underlying pointee type:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<Ident>()") {
		t.Fatalf("pointer assertion to a struct alias should downcast to the underlying struct:\n%s", rust)
	}
}

func TestEmbeddedLocalInterfaceFieldConcreteAssertionQualifiesAsAny(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Object interface {
	Name() string
}

type dependency interface {
	Object
	isDependency()
}

type Var struct{}

func (*Var) Name() string { return "v" }
func (*Var) isDependency() {}

type node struct {
	obj dependency
}

func asVar(n *node) (*Var, bool) {
	v, ok := n.obj.(*Var)
	return v, ok
}
`)

	if strings.Contains(rust, "any_val.__go_as_any().downcast_ref::<VarPtr>()") {
		t.Fatalf("embedded local interface assertion should not use ambiguous __go_as_any method lookup:\n%s", rust)
	}
	if !strings.Contains(rust, "<dyn dependency>::__go_as_any(any_val.as_ref()).downcast_ref::<VarPtr>()") {
		t.Fatalf("embedded local interface assertion should use trait-qualified __go_as_any:\n%s", rust)
	}
}

func TestAppendExpansionFromBareNestedSliceIndex(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Expr interface {
	isExpr()
}

type TagExpr struct{}

func (*TagExpr) isExpr() {}

func flatten(split [][][]Expr) []Expr {
	var lits []Expr
	for _, or := range split {
		lits = append(lits, or[0]...)
	}
	return lits
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "let __slice_holder = or[(0) as usize].clone().clone()") {
		t.Fatalf("append expansion from a bare nested slice index should not treat the source as wrapped:\n%s", rust)
	}
	if !strings.Contains(rust, "or[(0) as usize].clone().iter().cloned()") {
		t.Fatalf("append expansion from a bare nested slice index should extend from the bare Vec:\n%s", rust)
	}
}

func TestAppendStringConcatExpansionUsesBareString(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func appendName(name string) []byte {
	var repr []byte
	repr = append(repr, (" " + name)...)
	return repr
}
`)

	if strings.Contains(rust, ")).borrow().as_ref().unwrap()).clone().as_bytes()") ||
		strings.Contains(rust, ")).lock().unwrap().as_ref().unwrap()).clone().as_bytes()") {
		t.Fatalf("append string expansion should not treat string concatenation as wrapped:\n%s", rust)
	}
	if !strings.Contains(rust, "format!(\"{}{}\", \" \".to_string(),") ||
		!strings.Contains(rust, ".as_bytes().iter().cloned()") {
		t.Fatalf("append string expansion should extend from the bare formatted string:\n%s", rust)
	}
}

func TestTypedLongStringConcatUsesLinearBuilder(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func render(name string) string {
	return "a" + name + "b" + name + "c" + name + "d"
}
`)

	if strings.Contains(rust, `format!("{}{}", format!("{}{}"`) {
		t.Fatalf("long string concatenation should not lower to nested format macros:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut __s = String::new()") ||
		!strings.Contains(rust, `__s.push_str(&format!("{}",`) {
		t.Fatalf("long string concatenation should use a linear string builder shape:\n%s", rust)
	}
}

func TestAppendStringSliceToStringSliceUsesBareString(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func substrings(s string, beg int, end int) []string {
	var out []string
	out = append(out, s[beg:end])
	return out
}
`)

	if strings.Contains(rust, ".push(Rc::new(") || strings.Contains(rust, ".push(Arc::new(") {
		t.Fatalf("append string slice to []string should not push a wrapped String handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".push({ let __s = &(") || !strings.Contains(rust, "].to_string() }") {
		t.Fatalf("append string slice to []string should push the bare String value:\n%s", rust)
	}
}

func TestAppendConcreteLocalInterfaceAssertionBoxesValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Expr interface {
	isExpr()
}

type AndExpr struct {
	X Expr
}

func (*AndExpr) isExpr() {}

func collect(list []Expr, x Expr) []Expr {
	if x, ok := x.(*AndExpr); ok {
		return append(list, x)
	}
	return list
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, ".push(x.clone())") {
		t.Fatalf("append of concrete pointer assertion to local interface slice should not push the raw pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(AndExprPtr(x.clone())) as Box<dyn Expr>") &&
		!strings.Contains(rust, "Box::new(AndExprPtr(x.clone())) as Box<dyn Expr + Send + Sync>") {
		t.Fatalf("append of concrete pointer assertion to local interface slice should box the pointer wrapper:\n%s", rust)
	}
}

func TestLocalInterfaceStructFieldBoxesSelectorPointerValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	String() string
}

type Signature struct{}

func (*Signature) String() string { return "" }

type target struct {
	sig *Signature
}

type operand struct {
	typ Type
}

func record(t *target) operand {
	return operand{typ: t.sig}
}
`)

	if !strings.Contains(rust, "typ: Rc::new(RefCell::new(Some(Box::new(SignaturePtr((*t.borrow().as_ref().unwrap()).sig.clone())) as Box<dyn Type>)))") &&
		!strings.Contains(rust, "typ: Arc::new(Mutex::new(Some(Box::new(SignaturePtr((*t.lock().unwrap().as_ref().unwrap()).sig.clone())) as Box<dyn Type + Send + Sync>)))") {
		t.Fatalf("local interface struct field should box the selector pointer wrapper:\n%s", rust)
	}
}

func TestLocalInterfaceStructFieldSelectorClonesDropOwnerLock(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Expr interface {
	isExpr()
}

type Holder struct {
	first Expr
	second Expr
}

type Pair struct {
	first Expr
	second Expr
}

func record(h *Holder) Pair {
	return Pair{first: h.first, second: h.second}
}
`)

	if strings.Contains(rust, "first: (*h.borrow().as_ref().unwrap()).first.clone()") ||
		strings.Contains(rust, "first: (*h.lock().unwrap().as_ref().unwrap()).first.clone()") {
		t.Fatalf("local interface selector field should not hold the owner lock in the struct literal:\n%s", rust)
	}
	if !strings.Contains(rust, "first: { let __field = (*h.borrow().as_ref().unwrap()).first.clone(); __field }") &&
		!strings.Contains(rust, "first: { let __field = (*h.lock().unwrap().as_ref().unwrap()).first.clone(); __field }") {
		t.Fatalf("local interface selector field should clone inside a guard-dropping block:\n%s", rust)
	}
	if !strings.Contains(rust, "second: { let __field = (*h.borrow().as_ref().unwrap()).second.clone(); __field }") &&
		!strings.Contains(rust, "second: { let __field = (*h.lock().unwrap().as_ref().unwrap()).second.clone(); __field }") {
		t.Fatalf("second local interface selector field should clone inside a guard-dropping block:\n%s", rust)
	}
}

func TestLocalInterfaceSelectorNilComparisonDropsOwnerLock(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Expr interface {
	isExpr()
}

type Spec struct {
	Type Expr
	Values []int
}

func hasInit(s *Spec) bool {
	return s.Type != nil || len(s.Values) > 0
}
`)

	if strings.Contains(rust, "(*(*s.borrow().as_ref().unwrap()).r#type.borrow()") ||
		strings.Contains(rust, "(*(*s.lock().unwrap().as_ref().unwrap()).r#type.lock()") {
		t.Fatalf("local interface selector nil comparison should not hold the owner lock while inspecting the field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __iface_handle = { let __field = (*s.borrow().as_ref().unwrap()).r#type.clone(); __field };") &&
		!strings.Contains(rust, "let __iface_handle = { let __field = (*s.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field };") {
		t.Fatalf("local interface selector nil comparison should clone the field handle before inspection:\n%s", rust)
	}
	if !strings.Contains(rust, "let __len_target = { let __field = (*s.borrow().as_ref().unwrap()).values.clone(); __field };") &&
		!strings.Contains(rust, "let __len_target = { let __field = (*s.lock().unwrap().as_ref().unwrap()).values.clone(); __field };") {
		t.Fatalf("len over selector should clone the field handle before inspection:\n%s", rust)
	}
}

func TestLocalInterfaceStructFieldBoxesIndexedPointerValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	String() string
}

type Basic struct{}

func (*Basic) String() string { return "" }

var Typ = []*Basic{{}}

type operand struct {
	typ Type
}

func record() operand {
	return operand{typ: Typ[0]}
}
`)

	boxedPackageIndex := strings.Contains(rust, "Box::new(BasicPtr((*Typ.borrow().as_ref().unwrap())[(0) as usize].clone().clone())) as Box<dyn Type") ||
		strings.Contains(rust, "Box::new(BasicPtr((*Typ.lock().unwrap().as_ref().unwrap())[(0) as usize].clone().clone())) as Box<dyn Type + Send + Sync>")
	if !boxedPackageIndex {
		t.Fatalf("local interface struct field should box the indexed pointer wrapper:\n%s", rust)
	}
}

func TestErrorStructFieldBoxesConcreteNamedIntegerValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Errno uintptr

func (e Errno) Error() string { return "" }

type PathError struct {
	Err error
}

func wrap(errno Errno) *PathError {
	return &PathError{Err: errno}
}
`)

	if strings.Contains(rust, "err: errno.clone()") {
		t.Fatalf("error struct field should not clone the concrete error handle:\n%s", rust)
	}
	if !strings.Contains(rust, "err: Rc::new(RefCell::new(Some(Box::new((*errno.borrow().as_ref().unwrap()).clone()) as Box<dyn StdError>)))") &&
		!strings.Contains(rust, "err: Arc::new(Mutex::new(Some(Box::new((*errno.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>)))") {
		t.Fatalf("error struct field should box the concrete error value:\n%s", rust)
	}
}

func TestErrorStructFieldClonesInterfaceHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "io/fs"

type PathError = fs.PathError

func wrap(err error) *PathError {
	return &PathError{Err: err}
}
`)

	if strings.Contains(rust, "err.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("error interface struct field should not clone the boxed trait object:\n%s", rust)
	}
	if !strings.Contains(rust, "err: err.clone()") {
		t.Fatalf("error interface struct field should clone the error handle:\n%s", rust)
	}
}

func TestErrorsIsSelectorErrorArgumentKeepsHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"errors"
	"io"
)

func check(err error) bool {
	return errors.Is(err, io.EOF)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	prevConcurrencyDetector := globalConcurrencyDetector
	cd := NewConcurrencyDetector()
	cd.AnalyzeProject([]*ast.File{file})
	SetConcurrencyDetector(cd)
	t.Cleanup(func() {
		SetConcurrencyDetector(prevConcurrencyDetector)
	})

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"io": "io"})

	if strings.Contains(rust, "io::EOF.lock().unwrap().as_ref().unwrap()).clone()") ||
		strings.Contains(rust, "io::EOF.borrow().as_ref().unwrap()).clone()") {
		t.Fatalf("errors.Is selector error argument should not clone the boxed trait object:\n%s", rust)
	}
	if !strings.Contains(rust, "errors::is(err.clone(), { let __field = io::EOF.clone(); __field })") {
		t.Fatalf("errors.Is selector error argument should pass the error handle:\n%s", rust)
	}
}

func TestAppendCurrentReceiverToInterfaceSliceBoxesSelf(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node interface {
	ID() int
}

type item struct{}

func (i *item) ID() int {
	return 1
}

func (i *item) Nodes() []Node {
	var nodes []Node
	nodes = append(nodes, i)
	return nodes
}
`)

	if strings.Contains(rust, "self.borrow()") || strings.Contains(rust, "self.lock()") {
		t.Fatalf("current receiver boxed into an interface should not be treated as a wrapped handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(self.clone()) as Box<dyn Node>") &&
		!strings.Contains(rust, "Box::new(self.clone()) as Box<dyn Node + Send + Sync>") {
		t.Fatalf("current receiver boxed into an interface should clone self into the trait object:\n%s", rust)
	}
}

func TestAppendCurrentPointerReceiverToPointerSliceWrapsSelfHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Pool struct {
	id int
}

var allPools []*Pool

func (p *Pool) add() {
	allPools = append(allPools, p)
}
`)

	if strings.Contains(rust, ".push(self.clone())") {
		t.Fatalf("current pointer receiver appended to pointer slice should not push the bare receiver value:\n%s", rust)
	}
	if !strings.Contains(rust, ".push(Rc::new(RefCell::new(Some(self.clone()))))") &&
		!strings.Contains(rust, ".push(Arc::new(Mutex::new(Some(self.clone()))))") {
		t.Fatalf("current pointer receiver appended to pointer slice should wrap self in a pointer handle:\n%s", rust)
	}
}

func TestAppendInterfaceCallResultKeepsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node interface {
	ID() int
}

type item struct{}

func (item) ID() int {
	return 1
}

func makeNode() Node {
	return item{}
}

func Nodes() []Node {
	var nodes []Node
	nodes = append(nodes, makeNode())
	return nodes
}
`)

	if strings.Contains(rust, "(*make_node()") {
		t.Fatalf("append of local interface call result should not unwrap the interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".push(make_node())") && !strings.Contains(rust, ".push(make_node().clone())") {
		t.Fatalf("append of local interface call result should store the returned handle:\n%s", rust)
	}
}

func TestAppendConcreteCallResultToInterfaceSliceBoxesValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node interface {
	ID() int
}

type item struct{}

func (*item) ID() int {
	return 1
}

func makeItem() *item {
	return &item{}
}

func Nodes() []Node {
	var nodes []Node
	nodes = append(nodes, makeItem())
	return nodes
}
`)

	if strings.Contains(rust, ".push(make_item())") || strings.Contains(rust, ".push(make_item().clone())") {
		t.Fatalf("append of concrete call result to interface slice should not store the concrete handle directly:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(itemPtr(make_item().clone())) as Box<dyn Node>") &&
		!strings.Contains(rust, "Box::new(itemPtr(make_item().clone())) as Box<dyn Node + Send + Sync>") {
		t.Fatalf("append of concrete call result to interface slice should box the pointer wrapper:\n%s", rust)
	}
}

func TestMapLiteralConcreteValueToLocalInterfaceBoxesValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	Underlying() Type
}

type TypeParam struct{}

func (*TypeParam) Underlying() Type {
	return nil
}

func literal(tp *TypeParam) map[string]Type {
	return map[string]Type{"T": tp}
}
`)

	if strings.Contains(rust, `"T".to_string(), tp.clone()`) ||
		strings.Contains(rust, `"T".to_string(), Rc::new(RefCell::new(Some((*tp`) ||
		strings.Contains(rust, `"T".to_string(), Arc::new(Mutex::new(Some((*tp`) {
		t.Fatalf("map literal should not store concrete pointer handle directly in interface slot:\n%s", rust)
	}
	if !strings.Contains(rust, `Box::new(TypeParamPtr(tp.clone())) as Box<dyn Type`) &&
		!strings.Contains(rust, `Box::new(TypeParamPtr(tp.clone())) as Box<dyn Type + Send + Sync`) {
		t.Fatalf("map literal should box concrete pointer wrapper into local interface value:\n%s", rust)
	}
}

func TestAppendConcreteValueToEmptyInterfaceSliceBoxesBareAny(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func add(list []any, n int) []any {
	return append(list, n)
}
`)

	if strings.Contains(rust, ".push((*n.borrow().as_ref().unwrap()).clone())") ||
		strings.Contains(rust, ".push((*n.lock().unwrap().as_ref().unwrap()).clone())") {
		t.Fatalf("append of concrete value to []any should not push the concrete value directly:\n%s", rust)
	}
	if !strings.Contains(rust, ".push(Box::new((*n.borrow().as_ref().unwrap()).clone()) as Box<dyn Any") &&
		!strings.Contains(rust, ".push(Box::new((*n.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any") {
		t.Fatalf("append of concrete value to []any should box the concrete value:\n%s", rust)
	}
}

func TestConcurrentRealImagUseComplexHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func parts(x complex128) (float64, float64) {
	go func() {}()
	return real(x), imag(x)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
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
	if strings.Contains(rust, "__v }.lock()") {
		t.Fatalf("real/imag should not borrow a raw cloned complex value as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "(*x.lock().unwrap().as_ref().unwrap()).re") {
		t.Fatalf("real should read the complex component through the original handle:\n%s", rust)
	}
	if !strings.Contains(rust, "(*x.lock().unwrap().as_ref().unwrap()).im") {
		t.Fatalf("imag should read the complex component through the original handle:\n%s", rust)
	}
}

func TestComplexBuiltinUsesBareTupleScalarArgs(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func floats() (float64, float64) {
	return 1.25, 2.5
}

func keyVal() interface{} {
	r, i := floats()
	return complex(r, i)
}
`)

	if strings.Contains(rust, "r.borrow()") || strings.Contains(rust, "r.lock()") ||
		strings.Contains(rust, "i.borrow()") || strings.Contains(rust, "i.lock()") {
		t.Fatalf("complex builtin should use bare tuple scalar arguments directly:\n%s", rust)
	}
	if !strings.Contains(rust, "num::Complex::new(r as f64, i as f64)") {
		t.Fatalf("complex builtin should build a complex value from bare scalar arguments:\n%s", rust)
	}
}

func TestComplex64ToComplex128ConversionUsesComponents(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func widen(x complex64) complex128 {
	return complex128(x)
}
`)

	if strings.Contains(rust, "as f64, 0.0") {
		t.Fatalf("complex64 to complex128 conversion should preserve real and imaginary components:\n%s", rust)
	}
	if !strings.Contains(rust, "num::Complex::<f64>::new(__z.re as f64, __z.im as f64)") {
		t.Fatalf("complex64 to complex128 conversion should widen both components:\n%s", rust)
	}
}

func TestConcurrentComplexComparisonCastsZeroConstant(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

var done chan int

func isZero(z complex128) bool {
	return z == 0
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
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

	if strings.Contains(rust, "let __tmp_y = 0;") {
		t.Fatalf("complex comparison should not leave an integer zero peer:\n%s", rust)
	}
	if !strings.Contains(rust, "let __tmp_y = num::Complex::<f64>::new(0.0, 0.0)") {
		t.Fatalf("complex comparison should cast zero to the peer complex type:\n%s", rust)
	}
}

func TestAppendLenToIntSliceCastsToGoInt(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func add(valueStart []int, steps []string) []int {
	return append(valueStart, len(steps))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, ".push((*steps.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)") {
		t.Fatalf("append len() to []int should cast usize length to Go int:\n%s", rust)
	}
}

func TestIntSliceLiteralCastsLenElementToGoInt(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func seq(words []string) []int {
	return []int{0, len(words)}
}
`)

	if !strings.Contains(rust, " as i32") {
		t.Fatalf("[]int literal len element should cast Rust usize to Go int:\n%s", rust)
	}
	if strings.Contains(rust, "len()).unwrap_or(0)]") {
		t.Fatalf("[]int literal len element should not keep Rust usize inference:\n%s", rust)
	}
}

func TestAppendRangeIndexToIntSliceCastsToGoInt(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func indexes(values []string) []int {
	var out []int
	for i := range values {
		out = append(out, i)
	}
	return out
}
`)

	if strings.Contains(rust, ".push(i)") {
		t.Fatalf("append range index to []int should not push usize directly:\n%s", rust)
	}
	if !strings.Contains(rust, ".push(i as i32)") {
		t.Fatalf("append range index to []int should cast usize to Go int:\n%s", rust)
	}
}

func TestAppendStringRangeRuneToRuneSliceCastsToGoRune(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func runes(s string) []rune {
	var out []rune
	for _, c := range s {
		out = append(out, c)
	}
	return out
}
`)

	if strings.Contains(rust, ".push(c)") {
		t.Fatalf("append string range rune to []rune should not push Rust char directly:\n%s", rust)
	}
	if !strings.Contains(rust, ".push(c as i32)") {
		t.Fatalf("append string range rune to []rune should cast char to Go rune:\n%s", rust)
	}
}

func TestStringRangeRuneComparisonWithRuneVariableCastsToGoRune(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func hasQuote(s string) bool {
	var quote rune
	for _, c := range s {
		if c == quote {
			return true
		}
	}
	return false
}
`)

	if strings.Contains(rust, "let __tmp_x = c;") || strings.Contains(rust, "if c == (*quote") {
		t.Fatalf("comparison with rune variable should not compare Rust char directly:\n%s", rust)
	}
	if !strings.Contains(rust, "c as i32") {
		t.Fatalf("comparison with rune variable should cast string range char to Go rune:\n%s", rust)
	}
}

func TestStringRangeRuneComparisonWithRuneLiteralStaysChar(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func validTag(word string) bool {
	for _, c := range word {
		if c != '_' && c != '.' {
			return false
		}
	}
	return true
}
`)

	if strings.Contains(rust, "c as i32") {
		t.Fatalf("comparison with rune literal should keep the Rust char value:\n%s", rust)
	}
	if !strings.Contains(rust, "c != '_'") || !strings.Contains(rust, "c != '.'") {
		t.Fatalf("comparison with rune literal should emit char literals:\n%s", rust)
	}
}

func TestAppendNamedIntegerConstToNamedSliceWrapsElement(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func push(z nat) nat {
	return append(z, 0)
}
`)

	if strings.Contains(rust, "__values.push(0)") {
		t.Fatalf("append to named-integer slice should not push raw untyped constants:\n%s", rust)
	}
	if !strings.Contains(rust, "__values.push(Word(") {
		t.Fatalf("append to named-integer slice should wrap the element as the named type:\n%s", rust)
	}
}

func TestNamedSliceConversionFromFunctionResultUsesSliceHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node struct{}
type nodeQueue []*Node

func dependencyGraph() []*Node {
	return nil
}

func use() nodeQueue {
	return nodeQueue(dependencyGraph())
}
`)

	if strings.Contains(rust, "nodeQueue(Rc::new(RefCell::new(Some(dependency_graph())))") ||
		strings.Contains(rust, "nodeQueue(Arc::new(Mutex::new(Some(dependency_graph())))") {
		t.Fatalf("named slice conversion should not wrap an existing slice handle:\n%s", rust)
	}
	if !strings.Contains(rust, "nodeQueue(dependency_graph())") {
		t.Fatalf("named slice conversion should pass the slice handle into the newtype constructor:\n%s", rust)
	}
}

func TestNamedSliceConversionFromSliceParameterUsesSliceHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func (z nat) norm() nat {
	return z
}

func setBits(abs []Word) nat {
	return nat(abs).norm()
}
`)

	if strings.Contains(rust, "nat({ let __v = (*abs.borrow().as_ref().unwrap()).clone(); __v })") ||
		strings.Contains(rust, "nat({ let __v = (*abs.lock().unwrap().as_ref().unwrap()).clone(); __v })") {
		t.Fatalf("named slice conversion should not unwrap a slice parameter to a bare Vec:\n%s", rust)
	}
	if !strings.Contains(rust, "nat(abs.clone()).norm()") {
		t.Fatalf("named slice conversion should clone the slice handle for a slice parameter:\n%s", rust)
	}
}

func TestUnnamedSliceConversionFromNamedSliceReceiverUsesInnerHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type CaseRange struct{}
type SpecialCase []CaseRange

var ch chan int

func use(ranges []CaseRange) int {
	return len(ranges)
}

func (special SpecialCase) score() int {
	return use([]CaseRange(special))
}
`)

	if strings.Contains(rust, "use((*self.0.lock().unwrap().as_ref().unwrap()))") ||
		strings.Contains(rust, "use((*self.0.borrow().as_ref().unwrap()))") {
		t.Fatalf("unnamed slice conversion from named-slice receiver should not unwrap to a bare Vec:\n%s", rust)
	}
	if !strings.Contains(rust, "use(self.0.clone())") {
		t.Fatalf("unnamed slice conversion from named-slice receiver should pass the inner slice handle:\n%s", rust)
	}
}

func TestNamedArrayFieldIndexUsesInnerHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type d [3]int

type CaseRange struct {
	Delta d
}

var ch chan int

func convert(cr *CaseRange, i int) int {
	return cr.Delta[i]
}
`)

	if strings.Contains(rust, "let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[") {
		t.Fatalf("named array field index should not index the named wrapper value:\n%s", rust)
	}
	if !strings.Contains(rust, "let __named_array = (*") ||
		!strings.Contains(rust, ".delta") ||
		!strings.Contains(rust, ".0.clone(); __named_array }") {
		t.Fatalf("named array field index should use the inner array handle:\n%s", rust)
	}
}

func TestNamedArrayOverNamedArrayFieldIndexUsesInnermostHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type words [3]int
type flags words

type Holder struct {
	Bits flags
}

var ch chan int

func pick(h *Holder, i int) int {
	return h.Bits[i]
}
`)

	if strings.Contains(rust, "let __seq = __seq_guard.as_ref().unwrap(); __seq[") {
		t.Fatalf("named array over named array field index should not index the intermediate named wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "let __seq_inner_holder_0 = __seq.0.clone()") {
		t.Fatalf("named array over named array field index should peel the nested named array handle:\n%s", rust)
	}
}

func TestNestedIndexThroughPointerArrayElementBorrowsReturnedHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type arena struct {
	mark int
}

type heap struct {
	arenas [2]*[3]*arena
}

var h heap

func pick(i, j int) *arena {
	go func() {}()
	return h.arenas[i][j]
}
`)

	if strings.Contains(rust, "}[(j) as usize]") ||
		strings.Contains(rust, "}[(j.lock().unwrap().as_ref().unwrap()) as usize]") {
		t.Fatalf("nested index through pointer-to-array element should not index the returned handle directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let __seq_holder = { let __seq =") ||
		!strings.Contains(rust, "let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone()") {
		t.Fatalf("nested index through pointer-to-array element should borrow the returned array handle before indexing:\n%s", rust)
	}
}

func TestNestedIndexThroughArrayOfSlicesUsesBareInnerSlice(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type holder struct {
	summary [2][]uint64
}

var ch chan int

func pick(h *holder, i int) uint64 {
	return h.summary[1][i]
}
`)

	if strings.Contains(rust, "let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default()") {
		t.Fatalf("nested index through array-of-slices should not borrow a bare inner Vec as a wrapped slice handle:\n%s", rust)
	}
	if !strings.Contains(rust, "}; __seq[(1) as usize].clone() }[") {
		t.Fatalf("nested index through array-of-slices should index the bare inner Vec directly:\n%s", rust)
	}
}

func TestNamedScalarMethodOnNestedIndexUsesGeneratedMethod(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type score uint64

func (s score) max() uint64 {
	return uint64(s)
}

type holder struct {
	summary [2][]score
}

var ch chan int

func pick(h *holder, i int) uint64 {
	return h.summary[1][i].max()
}
`)

	if strings.Contains(rust, "].clone().max()") {
		t.Fatalf("named scalar method on nested index should not dispatch to Rust Ord::max:\n%s", rust)
	}
	if !strings.Contains(rust, "score::max(&") {
		t.Fatalf("named scalar method on nested index should use the generated method via explicit receiver type:\n%s", rust)
	}
}

func TestNamedScalarMethodOnWrappedReceiverUsesGeneratedMethod(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type score uint64

func (s score) max() uint64 {
	return uint64(s)
}

func pick(s score) uint64 {
	return s.max()
}
`)

	if strings.Contains(rust, ".as_ref().unwrap()).max()") {
		t.Fatalf("named scalar method on wrapped receiver should not dispatch to Rust Ord::max:\n%s", rust)
	}
	if !strings.Contains(rust, "score::max(&") {
		t.Fatalf("named scalar method on wrapped receiver should use the generated method via explicit receiver type:\n%s", rust)
	}
}

func TestNamedScalarReceiverMethodCallKeepsNamedReceiver(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type headTailIndex uint64

func (h headTailIndex) head() uint32 {
	return uint32(uint64(h) >> 32)
}

func (h headTailIndex) tail() uint32 {
	return uint32(h)
}

func (h headTailIndex) split() (uint32, uint32) {
	return h.head(), h.tail()
}
`)

	if strings.Contains(rust, "headTailIndex::head(&(*self.0.lock().unwrap().as_ref().unwrap()))") ||
		strings.Contains(rust, "headTailIndex::tail(&(*self.0.lock().unwrap().as_ref().unwrap()))") ||
		strings.Contains(rust, "headTailIndex::head(&(*self.0.borrow().as_ref().unwrap()))") ||
		strings.Contains(rust, "headTailIndex::tail(&(*self.0.borrow().as_ref().unwrap()))") {
		t.Fatalf("named scalar receiver method call should keep the named receiver, not unwrap to the scalar field:\n%s", rust)
	}
	if !strings.Contains(rust, "headTailIndex::head(self)") ||
		!strings.Contains(rust, "headTailIndex::tail(self)") {
		t.Fatalf("named scalar receiver method call should pass self as the generated method receiver:\n%s", rust)
	}
}

func TestAppendLocalInterfaceHandleKeepsWrappedValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Node interface {
	Pos() int
}

type nodeStack []Node

func push(s *nodeStack, n Node) {
	*s = append(*s, n)
}

func withNil(nodes []Node) []Node {
	return append(nodes, nil)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, ".push((*n.borrow().as_ref().unwrap()).clone())") ||
		strings.Contains(rust, ".push((*n.lock().unwrap().as_ref().unwrap()).clone())") {
		t.Fatalf("append of local interface handle should not clone the boxed trait object out of the handle:\n%s", rust)
	}
	if !strings.Contains(rust, "__values.push(n.clone())") {
		t.Fatalf("named slice append should keep the existing local interface handle:\n%s", rust)
	}
	if strings.Contains(rust, ".push(None)") {
		t.Fatalf("append nil to local interface slice should push a wrapped None handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".push(Rc::new(RefCell::new(None)))") &&
		!strings.Contains(rust, ".push(Arc::new(Mutex::new(None)))") {
		t.Fatalf("append nil to local interface slice should emit a wrapped None handle:\n%s", rust)
	}
}

func TestAppendNilToPointerSliceUsesWrappedNone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Ident struct{}

func collect(names []*Ident) []*Ident {
	return append(names, nil)
}
`)

	if strings.Contains(rust, ".push(None)") {
		t.Fatalf("append nil to pointer slice should push a wrapped None handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".push(Rc::new(RefCell::new(None)))") &&
		!strings.Contains(rust, ".push(Arc::new(Mutex::new(None)))") {
		t.Fatalf("append nil to pointer slice should emit a wrapped None handle:\n%s", rust)
	}
}

func TestAppendThroughPointerToSliceFieldUsesHandleTarget(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Pos int

type Interface struct {
	embedPos *[]Pos
}

func addEmbedded(ityp *Interface, pos Pos) {
	if ityp.embedPos == nil {
		ityp.embedPos = new([]Pos)
	}
	*ityp.embedPos = append(*ityp.embedPos, pos)
}
`)

	if strings.Contains(rust, "let __append_target = { let __v = (*") {
		t.Fatalf("append through pointer-to-slice field should not unwrap the target to a bare Vec:\n%s", rust)
	}
	if !strings.Contains(rust, ".embed_pos.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push") &&
		!strings.Contains(rust, ".embed_pos.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push") {
		t.Fatalf("append through pointer-to-slice field should mutate the slice handle:\n%s", rust)
	}
}

func TestAppendFromGenericSliceResultConvertsToConcreteSlice(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func Clip[S ~[]E, E any](s S) S {
	return s
}

type Config struct {
	Env []string
	Dir string
}

func buildEnv(cfg *Config) []string {
	return append(Clip(cfg.Env), "PWD="+cfg.Dir)
}
`)

	if strings.Contains(rust, "let __append_target = clip::<Vec<String>, String>({ let __field = (*cfg") {
		t.Fatalf("append target from generic slice result should convert back to concrete slice representation:\n%s", rust)
	}
	if !strings.Contains(rust, "let __result = clip::<Vec<String>, String>") {
		t.Fatalf("append target should call the generic slice function before concrete conversion:\n%s", rust)
	}
	if !strings.Contains(rust, ".iter().cloned().map(|__elem| (*__elem") {
		t.Fatalf("generic slice result should unwrap type-param element handles before append:\n%s", rust)
	}
}

func TestLocalInterfaceSliceLiteralBoxesConcretePointerElements(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Expr interface{ exprNode() }

type UnaryExpr struct{ Op string }

func (*UnaryExpr) exprNode() {}

type Ident struct{ Name string }

func (*Ident) exprNode() {}

func main() {
	_ = []Expr{&UnaryExpr{Op: "-"}, &Ident{Name: "x"}}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "Box::new(Rc::new(RefCell::new(Some(UnaryExpr") ||
		strings.Contains(rust, "Box::new(Rc::new(RefCell::new(Some(Ident") {
		t.Fatalf("interface slice literal should not box pointer handles as trait objects:\n%s", rust)
	}
	if !strings.Contains(rust, "Rc::new(RefCell::new(Some(Box::new(UnaryExpr") ||
		!strings.Contains(rust, "Rc::new(RefCell::new(Some(Box::new(Ident") {
		t.Fatalf("interface slice literal should box concrete values inside wrapped interface handles:\n%s", rust)
	}
}

func TestPointerSliceCompositeLiteralKeepsSelectorHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Package struct{}

type Info struct {
	pkg *Package
}

func use(pkgs []*Package) {}

func call(info *Info) {
	use([]*Package{info.pkg})
}
`)

	if strings.Contains(rust, "vec![(*") {
		t.Fatalf("pointer slice literal should keep selector pointer handles, not pointee values:\n%s", rust)
	}
	if !strings.Contains(rust, "pkg.clone()") {
		t.Fatalf("pointer slice literal should clone the selector pointer handle:\n%s", rust)
	}
}

func TestVariadicFixedLocalInterfaceArgumentBoxesIndexedPointer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface{ Underlying() Type }

type Basic struct{}

func (*Basic) Underlying() Type { return nil }

var Typ = []*Basic{{}}

func makeSig(res Type, args ...Type) {}

func use(x Type) {
	makeSig(Typ[0], x)
}
`)

	if strings.Contains(rust, "make_sig(Rc::new(RefCell::new(Some({") ||
		strings.Contains(rust, "make_sig(Arc::new(Mutex::new(Some({") ||
		strings.Contains(rust, "Some((*Typ.borrow().as_ref().unwrap())[(0) as usize].clone())") {
		t.Fatalf("fixed local-interface argument should not wrap indexed pointer handles directly:\n%s", rust)
	}
	if !strings.Contains(rust, "make_sig(Rc::new(RefCell::new(Some(Box::new(BasicPtr((*Typ.borrow().as_ref().unwrap())[(0) as usize].clone().clone())) as Box<dyn Type>)))") &&
		!strings.Contains(rust, "make_sig(Arc::new(Mutex::new(Some(Box::new(BasicPtr((*Typ.lock().unwrap().as_ref().unwrap())[(0) as usize].clone().clone())) as Box<dyn Type + Send + Sync>)))") {
		t.Fatalf("fixed local-interface argument should box the indexed pointer wrapper:\n%s", rust)
	}
}

func TestVariadicPointerCallElementKeepsReturnedHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Var struct{}
type Tuple struct{}

func NewVar() *Var { return &Var{} }
func NewTuple(vars ...*Var) *Tuple { return &Tuple{} }

func makeTuple() *Tuple {
	return NewTuple(NewVar())
}
`)

	if strings.Contains(rust, "(*new_var().borrow().as_ref().unwrap())") ||
		strings.Contains(rust, "(*new_var().lock().unwrap().as_ref().unwrap())") {
		t.Fatalf("pointer-returning variadic element should not be unwrapped to the pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "new_tuple(Rc::new(RefCell::new(Some(vec![new_var()])))") &&
		!strings.Contains(rust, "new_tuple(Arc::new(Mutex::new(Some(vec![new_var()])))") {
		t.Fatalf("pointer-returning variadic element should keep the returned handle:\n%s", rust)
	}
}

func TestVariadicErrorArgumentsKeepHandles(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "errors"

func join(errs ...error) error { return nil }

func use() {
	_ = join(errors.New("one"), nil, errors.New("two"))
}
`)

	if strings.Contains(rust, `Box<dyn std::error::Error`) &&
		strings.Contains(rust, `.as_ref().unwrap()).clone()`) {
		t.Fatalf("variadic error elements should not unwrap returned error handles to boxed payloads:\n%s", rust)
	}
	if !strings.Contains(rust, `join(Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("one".to_string())))), Rc::new(RefCell::new(None)), Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("two".to_string()))))])))`) &&
		!strings.Contains(rust, `join(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("one".to_string())))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("two".to_string()))))])))`) {
		t.Fatalf("variadic error elements should keep returned error handles:\n%s", rust)
	}
	if !strings.Contains(rust, `Rc::new(RefCell::new(None))`) &&
		!strings.Contains(rust, `Arc::new(Mutex::new(None))`) {
		t.Fatalf("nil variadic error element should remain an empty error handle:\n%s", rust)
	}
}

func TestVariadicStdlibInterfaceRangeValueClonesElement(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "go/ast"

type checker struct{}

func (c *checker) use(args ...ast.Expr) {}

func (c *checker) walk(values []ast.Expr) {
	for _, expr := range values {
		c.use(expr)
	}
}
`)

	if strings.Contains(rust, "vec![expr]") {
		t.Fatalf("stdlib interface range value should not be packed by reference:\n%s", rust)
	}
	if !strings.Contains(rust, "vec![(*expr).clone()]") {
		t.Fatalf("stdlib interface range value should be cloned as an owned variadic element:\n%s", rust)
	}
}

func TestStdlibInterfaceIndexedValueConversionUsesRawValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "go/ast"

func end(n ast.Node) {}

func use(values []ast.Expr, i int) {
	end(values[i])
}
`)

	if strings.Contains(rust, "__arg.borrow()") || strings.Contains(rust, "__arg.lock()") {
		t.Fatalf("indexed stdlib interface value should convert as a raw value, not a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "__arg.into()") {
		t.Fatalf("indexed stdlib interface value should convert into the target interface value:\n%s", rust)
	}
}

func TestStdlibInterfaceRangeValueConversionUsesRawValue(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "go/types", `package types

import "go/ast"

func end(n ast.Node) {}

func use(values []ast.Expr) {
	for _, e := range values {
		end(e)
	}
}
`, map[string]string{"go/types": "go_types"})

	if strings.Contains(rust, "__arg.borrow()") || strings.Contains(rust, "__arg.lock()") {
		t.Fatalf("range stdlib interface value should convert as a raw value, not a handle:\n%s", rust)
	}
	if strings.Contains(rust, "let __arg = e;") {
		t.Fatalf("range stdlib interface value should clone the referenced range element:\n%s", rust)
	}
	if !strings.Contains(rust, "__arg.into()") {
		t.Fatalf("range stdlib interface value should convert into the target interface value:\n%s", rust)
	}
}

func TestStdlibInterfaceTypeConversionToAnyBoxesBareValue(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "go/types", `package types

import "go/ast"

func report(args ...any) {}

func use(e *ast.SelectorExpr) {
	report(ast.Expr(e))
}
`, map[string]string{"go/types": "go_types"})

	if strings.Contains(rust, "let __owned = (*__v.lock().unwrap()") ||
		strings.Contains(rust, "let __owned = (*__v.borrow()") {
		t.Fatalf("stdlib interface conversion boxed as any should not unwrap the converted bare value as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new({ let __arg = e.clone();") ||
		!strings.Contains(rust, "map(|__v| (*__v).clone().into())") {
		t.Fatalf("stdlib interface conversion boxed as any should box the converted bare value:\n%s", rust)
	}
}

func TestVariadicEllipsisSelectorSliceKeepsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Expr interface {
	Pos() int
}

type checker struct{}

func (c *checker) consume(args ...Expr) {}

type indexed struct {
	indices []Expr
}

func call(c *checker, ix *indexed) {
	c.consume(ix.indices...)
}
`)

	if strings.Contains(rust, ".consume((*{ let __field = (*ix.borrow().as_ref().unwrap()).indices.clone(); __field }.borrow().as_ref().unwrap()).clone())") ||
		strings.Contains(rust, ".consume((*{ let __field = (*ix.lock().unwrap().as_ref().unwrap()).indices.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())") {
		t.Fatalf("ellipsis selector slice should not unwrap to a raw Vec:\n%s", rust)
	}
	if !strings.Contains(rust, ".consume((*ix.borrow().as_ref().unwrap()).indices.clone())") &&
		!strings.Contains(rust, ".consume((*ix.lock().unwrap().as_ref().unwrap()).indices.clone())") &&
		!strings.Contains(rust, ".consume({ let __field = (*ix.borrow().as_ref().unwrap()).indices.clone(); __field })") &&
		!strings.Contains(rust, ".consume({ let __field = (*ix.lock().unwrap().as_ref().unwrap()).indices.clone(); __field })") {
		t.Fatalf("ellipsis selector slice should pass the existing slice handle:\n%s", rust)
	}
}

func TestWrappedStringCallArgumentUsesShortGuardBlock(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Expr interface {
	isExpr()
}

type TagExpr struct {
	Tag string
}

func (*TagExpr) isExpr() {}

func tag(tok string) Expr {
	return &TagExpr{tok}
}

func atom(tok string) Expr {
	defer func() {}()
	return tag(tok)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "Some((*tok.borrow().as_ref().unwrap()).clone())") ||
		strings.Contains(rust, "Some((*tok.lock().unwrap().as_ref().unwrap()).clone())") {
		t.Fatalf("wrapped string call argument should not borrow inline in return expression:\n%s", rust)
	}
	if !strings.Contains(rust, "let __arg_holder = tok.clone()") {
		t.Fatalf("wrapped string call argument should clone through a short guard block:\n%s", rust)
	}
}

func TestIndexedStructStringFieldCallArgumentWrapsBareFieldValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type entry struct {
	name string
}

func consume(s string) {}

func use(table []entry, i int) {
	consume(table[i].name)
}
`)

	if strings.Contains(rust, "consume({ let __field =") {
		t.Fatalf("indexed struct field call argument should not pass the cloned field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "consume(Rc::new(RefCell::new(Some({ let __seq =") &&
		!strings.Contains(rust, "consume(Rc::new(RefCell::new(Some({ let __selector_holder =") &&
		!strings.Contains(rust, "consume(Arc::new(Mutex::new(Some({ let __seq =") &&
		!strings.Contains(rust, "consume(Arc::new(Mutex::new(Some({ let __selector_holder =") {
		t.Fatalf("indexed struct field call argument should wrap the bare field value:\n%s", rust)
	}
}

func TestNamedStringConversionWrapsBareIndexedStructFieldValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type entry struct {
	name string
}

type errorString string

func use(table []entry, i int) errorString {
	return errorString(table[i].name)
}
`)

	if strings.Contains(rust, ".name.clone().borrow()") ||
		strings.Contains(rust, ".name.clone().lock()") ||
		strings.Contains(rust, ".clone().borrow().as_ref().unwrap()).clone()") ||
		strings.Contains(rust, ".clone().lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("named string conversion should not borrow a cloned bare field value:\n%s", rust)
	}
	if !strings.Contains(rust, "errorString(Rc::new(RefCell::new(Some(") &&
		!strings.Contains(rust, "errorString(Arc::new(Mutex::new(Some(") {
		t.Fatalf("named string conversion should wrap a string value:\n%s", rust)
	}
}

func TestPromotedFieldSelectorOnWrappedCallResultUnwrapsBase(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type inner struct {
	value int
}

type outer struct {
	*inner
}

func makeOuter() outer {
	return outer{inner: &inner{value: 3}}
}

func use() int {
	return makeOuter().value
}
`)

	if strings.Contains(rust, "make_outer().inner") {
		t.Fatalf("promoted field selector on wrapped call result should not access the returned handle directly:\n%s", rust)
	}
	if !strings.Contains(rust, "(*make_outer().borrow().as_ref().unwrap()).inner") &&
		!strings.Contains(rust, "(*make_outer().lock().unwrap().as_ref().unwrap()).inner") {
		t.Fatalf("promoted field selector on wrapped call result should unwrap the call result before following the embedded field:\n%s", rust)
	}
}

func TestLocalInterfaceSelectorArgumentPassesHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Expr interface {
	isExpr()
}

type TagExpr struct{}

func (*TagExpr) isExpr() {}

type NotExpr struct {
	X Expr
}

func label(x Expr) string {
	return "x"
}

func (n *NotExpr) String() string {
	return label(n.X)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, ".as_ref().unwrap().as_ref()") {
		t.Fatalf("local interface selector argument should not unwrap to a bare trait object:\n%s", rust)
	}
	if !strings.Contains(rust, "label(self.x.clone())") {
		t.Fatalf("local interface selector argument should pass the wrapper handle:\n%s", rust)
	}
}

func TestFunctionValueLocalInterfaceSelectorArgumentPassesHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Stmt interface {
	stmtNode()
}

type LabeledStmt struct {
	Stmt Stmt
}

func walk(fn func(Stmt), s *LabeledStmt) {
	fn(s.Stmt)
}
`)

	if strings.Contains(rust, ".as_ref().unwrap().as_ref()") {
		t.Fatalf("function value local-interface selector argument should not unwrap to a bare trait object:\n%s", rust)
	}
	if !strings.Contains(rust, ".stmt.clone())") {
		t.Fatalf("function value local-interface selector argument should pass the wrapper handle:\n%s", rust)
	}
}

func TestFunctionVariableSourceMappedInterfaceSelectorArgumentPassesHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "labels.go", `package main

import "go/ast"

func walk(s *ast.LabeledStmt) {
	var stmtBranches func(ast.Stmt)
	stmtBranches = func(x ast.Stmt) {}
	stmtBranches(s.Stmt)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(labels.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/ast": "go_ast"})
	if strings.Contains(rust, ".as_ref().unwrap().as_ref()") {
		t.Fatalf("function variable source-mapped interface selector argument should not unwrap to a bare trait object:\n%s", rust)
	}
	if !strings.Contains(rust, ".stmt.clone())") {
		t.Fatalf("function variable source-mapped interface selector argument should pass the wrapper handle:\n%s", rust)
	}
}

func TestSliceSelectorArgumentDefaultsNilSliceToEmpty(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Field struct{}

type FieldList struct {
	List []*Field
}

func walkList[N any](list []N) {
	for range list {
	}
}

func walk(n *FieldList) {
	walkList(n.List)
}
`)

	if !strings.Contains(rust, "__selector_guard.as_ref().cloned().unwrap_or_default()") {
		t.Fatalf("slice selector argument should clone nil slices as empty values:\n%s", rust)
	}
	if strings.Contains(rust, "(*__selector_guard.as_ref().unwrap()).clone()") {
		t.Fatalf("slice selector argument should not unwrap nil slices:\n%s", rust)
	}
}

func TestSourceMappedInterfacePointerArgumentBoxesPointerWrapper(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "walk.go", `package main

import "go/ast"

func walk(node ast.Node) {}

func use() {
	expr := &ast.CallExpr{}
	walk(expr)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(walk.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/ast": "go_ast"})
	if strings.Contains(rust, "Box::new((*expr.borrow().as_ref().unwrap()).clone())") ||
		strings.Contains(rust, "Box::new((*expr.lock().unwrap().as_ref().unwrap()).clone())") {
		t.Fatalf("pointer to source-mapped interface should not box the pointee value:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(go_ast::r#mod::CallExprPtr(expr.clone())) as Box<dyn go_ast::r#mod::Node") &&
		!strings.Contains(rust, "Box::new(go_ast::CallExprPtr(expr.clone())) as Box<dyn go_ast::Node") {
		t.Fatalf("pointer to source-mapped interface should box the generated pointer wrapper:\n%s", rust)
	}
}

func TestLocalPointerSourceMappedInterfaceArgumentBoxesPointerWrapper(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "heap.go", `package main

import "container/heap"

type IntHeap []int

func (h IntHeap) Len() int { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }
func (h *IntHeap) Push(x any) { *h = append(*h, x.(int)) }
func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func use() {
	h := &IntHeap{3, 1, 2}
	heap.Push(h, 0)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(heap.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"container/heap": "container_heap",
		"sort":           "sort",
	})
	if strings.Contains(rust, "Box::new({ let __arg_holder = h.clone();") {
		t.Fatalf("local pointer passed to source-mapped interface should not box a cloned pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(IntHeapPtr(h.clone())) as Box<dyn container_heap::r#mod::Interface") &&
		!strings.Contains(rust, "Box::new(IntHeapPtr(h.clone())) as Box<dyn container_heap::Interface") {
		t.Fatalf("local pointer source-mapped interface argument should box the pointer wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "impl container_heap::r#mod::Interface for IntHeapPtr") &&
		!strings.Contains(rust, "impl container_heap::Interface for IntHeapPtr") {
		t.Fatalf("local pointer source-mapped interface wrapper should implement heap.Interface:\n%s", rust)
	}
	if strings.Contains(rust, "impl container_heap::r#mod::Interface for IntHeap {") ||
		strings.Contains(rust, "impl container_heap::Interface for IntHeap {") {
		t.Fatalf("pointer-only source-mapped interface should not emit a value impl for the pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "impl sort::r#mod::Interface for IntHeapPtr") &&
		!strings.Contains(rust, "impl sort::Interface for IntHeapPtr") {
		t.Fatalf("local pointer source-mapped interface wrapper should implement embedded sort.Interface:\n%s", rust)
	}
	if strings.Contains(rust, "write!(f, \"{}\", __v)") {
		t.Fatalf("pointer wrapper Display should not require the pointee to implement Display:\n%s", rust)
	}
	if !strings.Contains(rust, "write!(f, \"{:p}\", __v as *const _)") {
		t.Fatalf("pointer wrapper Display should format pointer identity without opening a pointee Display bound:\n%s", rust)
	}
}

func TestSourceMappedPackageGlobalPointerInterfaceEqualityBoxesWrapper(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "event.go", `package event

import (
	"golang.org/x/tools/internal/event/keys"
	"golang.org/x/tools/internal/event/label"
)

func isMsg(key label.Key) bool {
	return key == keys.Msg
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(event.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("golang.org/x/tools/internal/event", []*ast.File{file}, fset, eventKeyEqualityImporter())
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(event) error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"golang.org/x/tools/internal/event/keys":  "golang_org_x_tools_internal_event_keys",
		"golang.org/x/tools/internal/event/label": "golang_org_x_tools_internal_event_label",
	})
	if strings.Contains(rust, "__right_guard.as_ref().map(|__v| __v as &(dyn golang_org_x_tools_internal_event_label") {
		t.Fatalf("source-mapped package global pointer equality should not cast the pointee to the interface:\n%s", rust)
	}
	usesNamedWrapper := strings.Contains(rust, "let __right_wrapper = golang_org_x_tools_internal_event_keys::String_Ptr(") &&
		(strings.Contains(rust, "Some(&__right_wrapper as &dyn golang_org_x_tools_internal_event_label") ||
			strings.Contains(rust, "Some(&__right_wrapper as &(dyn golang_org_x_tools_internal_event_label"))
	usesScopedWrapperHandle := strings.Contains(rust, "let __right_holder = { let __arg_holder = golang_org_x_tools_internal_event_keys::Msg.clone();") &&
		strings.Contains(rust, "__right_opt: Option<&dyn golang_org_x_tools_internal_event_label::Key>")
	if !usesNamedWrapper && !usesScopedWrapperHandle {
		t.Fatalf("source-mapped package global pointer equality should compare through the pointer wrapper:\n%s", rust)
	}
}

func TestCurrentPackagePointerSourceMappedInterfaceStructFieldBoxesWrapper(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "srcimporter.go", `package srcimporter

import "go/types"

type Importer struct{}

func (p *Importer) Import(path string) {}

func (p *Importer) Config() types.Config {
	return types.Config{Importer: p}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(srcimporter.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("go/internal/srcimporter", []*ast.File{file}, fset, goTypesConfigImporter())
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(srcimporter) error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/types": "go_types"})
	if strings.Contains(rust, "importer: Rc::new(RefCell::new(Some(Box::new(self.clone()) as Box<dyn go_types::Importer") ||
		strings.Contains(rust, "importer: Arc::new(Mutex::new(Some(Box::new(self.clone()) as Box<dyn go_types::Importer") ||
		strings.Contains(rust, "Box::new((*p.borrow().as_ref().unwrap()).clone()) as Box<dyn go_types::Importer") ||
		strings.Contains(rust, "Box::new((*p.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_types::Importer") {
		t.Fatalf("current-package pointer struct field should not box the pointee for a source-mapped interface:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(ImporterPtr(") {
		t.Fatalf("current-package pointer struct field should box the pointer wrapper:\n%s", rust)
	}
}

func TestTranspiledInterfaceMethodCallArgumentScopesReceiverLock(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "heap.go", `package main

import "container/heap"

func down(h heap.Interface, i int, n int) bool { return false }

func use(h heap.Interface, i int) {
	if !down(h, i, h.Len()) {
		down(h, i, i)
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(heap.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"container/heap": "container_heap",
		"sort":           "sort",
	})
	if strings.Contains(rust, "Some((*h.borrow().as_ref().unwrap()).len())") ||
		strings.Contains(rust, "Some((*h.lock().unwrap().as_ref().unwrap()).len())") {
		t.Fatalf("interface method call argument should not hold the receiver lock through the callee:\n%s", rust)
	}
	if !strings.Contains(rust, "Some({ let __arg_value = (*h.borrow().as_ref().unwrap()).len(); __arg_value })") &&
		!strings.Contains(rust, "Some({ let __arg_value = (*h.lock().unwrap().as_ref().unwrap()).len(); __arg_value })") {
		t.Fatalf("interface method call argument should scope the receiver lock before wrapping:\n%s", rust)
	}
}

func TestSourceMappedInterfaceFieldCallResultBoxesPointerWrapper(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "walk.go", `package main

import "go/ast"

func use() {
	_ = ast.CallExpr{Fun: ast.NewIdent("println")}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(walk.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/ast": "go_ast"})
	if strings.Contains(rust, "Box::new((*go_ast::new_ident") {
		t.Fatalf("pointer-returning call stored in source-mapped interface field should not box the pointee value:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(go_ast::IdentPtr(go_ast::new_ident(") &&
		!strings.Contains(rust, "Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(") {
		t.Fatalf("pointer-returning call stored in source-mapped interface field should box the generated pointer wrapper:\n%s", rust)
	}
}

func TestSourceMappedFunctionValueBoxUsesWrappedInterfaceParam(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"go/ast"
	"strings"
)

func emit(out *strings.Builder, expr ast.Expr) {
	_ = out
	_ = expr
}

func use(fn func(*strings.Builder, ast.Expr)) {
	_ = fn
}

func run() {
	use(emit)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"go/ast":  "go_ast",
		"strings": "strings",
	})

	if strings.Contains(rust, "FnMut(Rc<RefCell<Option<strings::Builder>>>, &dyn go_ast::Expr") {
		t.Fatalf("source-mapped function value box should not cast ast.Expr to a bare trait reference:\n%s", rust)
	}
	if !strings.Contains(rust, "FnMut(Rc<RefCell<Option<strings::Builder>>>, Rc<RefCell<Option<Box<dyn go_ast::Expr>>>>)") {
		t.Fatalf("source-mapped function value box should use the wrapped ast.Expr parameter shape:\n%s", rust)
	}
}

func TestSourceMappedInterfaceAssertionCommaOkFalseBranchIsNil(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/ast"

func asNode(node any) (ast.Node, bool) {
	n, ok := node.(ast.Node)
	return n, ok
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/ast": "go_ast"})
	if strings.Contains(rust, "Some(Default::default())") {
		t.Fatalf("comma-ok assertion to source-mapped interface should use nil on the false branch:\n%s", rust)
	}
	if !strings.Contains(rust, "None::<Box<dyn go_ast::Node") {
		t.Fatalf("comma-ok assertion to source-mapped interface should type the nil false branch:\n%s", rust)
	}
}

func TestLocalInterfaceWrappedIdentArgumentPassesHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Expr interface {
	isExpr()
}

type TagExpr struct{}

func (*TagExpr) isExpr() {}

func label(x Expr) string {
	return "x"
}

func use(x Expr) string {
	y := x
	return label(y)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, ".as_ref().unwrap().as_ref()") {
		t.Fatalf("wrapped local interface argument should not unwrap to a bare trait object:\n%s", rust)
	}
	if !strings.Contains(rust, "label(y.clone())") {
		t.Fatalf("wrapped local interface argument should pass the wrapper handle:\n%s", rust)
	}
}

func TestLocalInterfaceNamedReturnInitializesNil(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Expr interface {
	isExpr()
}

type TagExpr struct{}

func (*TagExpr) isExpr() {}

func parse() (x Expr) {
	return
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, "let mut x: Rc<RefCell<Option<Box<dyn Expr>>>> = Rc::new(RefCell::new(None));") {
		t.Fatalf("local interface named return should initialize to nil:\n%s", rust)
	}
	if strings.Contains(rust, "Option<Box<dyn Expr>>>> = Rc::new(RefCell::new(Some(Default::default())))") {
		t.Fatalf("local interface named return should not require a default concrete value:\n%s", rust)
	}
}

func TestScalarSelectorShortDeclCopiesInnerValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type File struct {
	size int
}

func (f *File) valid(offset int) bool {
	size := f.size
	return size <= offset
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, "let __selector_holder = self.size.clone()") {
		t.Fatalf("scalar selector short declaration should copy the inner field value:\n%s", rust)
	}
	if strings.Contains(rust, "Some(self.size.clone())") {
		t.Fatalf("scalar selector short declaration should not store the field handle inside the new wrapper:\n%s", rust)
	}
}

func TestScalarSelectorAssignmentCopiesInnerValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Parser struct {
	i   int
	pos int
}

func (p *Parser) mark() {
	p.pos = p.i
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, "let __selector_holder = self.i.clone()") {
		t.Fatalf("scalar selector assignment should copy the inner field value:\n%s", rust)
	}
	if strings.Contains(rust, "let new_val = self.i.clone();") {
		t.Fatalf("scalar selector assignment should not store the field handle:\n%s", rust)
	}
}

func TestLocalConstNameDoesNotShadowPointerSelectorAssignment(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Pos int

type Ident struct {
	NamePos Pos
	Name    string
}

func trace() {
	const dots = ". . . "
	const n = len(dots)
	_ = n
}

func NewIdent(name string) *Ident {
	return &Ident{Name: name}
}

func assign(pos Pos) *Ident {
	n := NewIdent("_")
	n.NamePos = pos
	return n
}
`)

	if strings.Contains(rust, "*n.name_pos") {
		t.Fatalf("pointer selector assignment should not use stale local-const name state:\n%s", rust)
	}
	if !strings.Contains(rust, "*(*n.borrow().as_ref().unwrap()).name_pos.borrow_mut() = Some(new_val);") &&
		!strings.Contains(rust, "*(*n.lock().unwrap().as_ref().unwrap()).name_pos.lock().unwrap() = Some(new_val);") {
		t.Fatalf("pointer selector assignment should unwrap the pointer handle before field access:\n%s", rust)
	}
}

func TestCapturedReceiverSelectorAssignmentUsesCloneName(t *testing.T) {
	prevReceiver := currentReceiver
	prevReceiverType := currentReceiverType
	prevRenames := currentCaptureRenames
	currentReceiver = "analysis"
	currentReceiverType = "transpileFileAnalysis"
	currentCaptureRenames = map[string]string{"analysis": "analysis_closure_clone"}
	defer func() {
		currentReceiver = prevReceiver
		currentReceiverType = prevReceiverType
		currentCaptureRenames = prevRenames
	}()

	var out strings.Builder
	writePointerHandleAssignmentTarget(&out, &ast.SelectorExpr{
		X:   ast.NewIdent("analysis"),
		Sel: ast.NewIdent("typeAssertExprs"),
	})

	if got, want := out.String(), "analysis_closure_clone.type_assert_exprs"; got != want {
		t.Fatalf("captured receiver selector target = %q, want %q", got, want)
	}
}

func TestIsFunctionNameUsesRegisteredSignatureWithoutTypeInfo(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevContext := currentContext
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		SetTranspileContext(prevContext)
		SetVarTable(prevVarTable)
	}()

	SetTranspileContext(&TranspileContext{
		Session: NewTranspileSession(nil, nil),
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	})
	SetTypeInfo(nil)
	RegisterFunctionSignature("hasName", &FunctionSignature{})

	if !isFunctionName(ast.NewIdent("hasName")) {
		t.Fatal("registered package function should be recognized without go/types")
	}

	vt := NewVarTable()
	vt.Register("hasName", &VarInfo{WrapLevel: WrapFull, Source: SourceLocal})
	SetVarTable(vt)
	if isFunctionName(ast.NewIdent("hasName")) {
		t.Fatal("local variable should shadow registered package function")
	}
}

func TestShortDeclSelfShadowingFunctionCallUsesTypeInfo(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func hash(key int) uint32 { return uint32(key) }

func lookup(key int, table map[uint32]int) int {
	hash := hash(key)
	return table[hash]
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "let __f_guard = hash.") {
		t.Fatalf("short declaration RHS should call the function, not the new local:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut hash = hash(") {
		t.Fatalf("short declaration RHS should emit a direct function call:\n%s", rust)
	}
}

func TestFunctionBoxTypeUsesVarTableWithoutTypeInfo(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		SetVarTable(prevVarTable)
	}()

	SetTypeInfo(nil)
	vt := NewVarTable()
	vt.Register("processData", &VarInfo{
		WrapLevel: WrapFull,
		RustType:  "Box<dyn FnMut(Rc<RefCell<Option<Vec<i32>>>>) -> Rc<RefCell<Option<Box<dyn StdError>>>>>",
		Source:    SourceLocal,
	})
	SetVarTable(vt)

	got := functionBoxTypeForCallTarget(ast.NewIdent("processData"))
	if !strings.HasPrefix(got, "Box<dyn FnMut(") {
		t.Fatalf("function box type = %q", got)
	}
}

func TestNoTypeInfoContextCancelFuncCallUsesTupleResultType(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import "context"

func main() {
	_, cancel := context.WithCancel(context.Background())
	cancel()
}
`)

	if strings.Contains(rust, "*mut _") {
		t.Fatalf("context cancel call should use concrete tuple result function type:\n%s", rust)
	}
	if !strings.Contains(rust, "*mut GoCancelFunc") {
		t.Fatalf("context cancel call should use GoCancelFunc:\n%s", rust)
	}
}

func TestNoTypeInfoVariadicSliceArgsUseElementValue(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

func collect(groups ...[]string) []string {
	var out []string
	for _, group := range groups {
		for _, value := range group {
			out = append(out, value)
		}
	}
	return out
}

func main() {
	var missing []string
	_ = collect([]string{"go"}, missing, []string{"rust"})
}
`)

	if strings.Contains(rust, "vec![Rc::new(RefCell::new(Some(vec!") {
		t.Fatalf("variadic []string elements should be packed as raw Vec values:\n%s", rust)
	}
	if strings.Contains(rust, "(*missing.borrow().as_ref().unwrap())") {
		t.Fatalf("nil slice variadic argument should use clone-or-empty path:\n%s", rust)
	}
	if strings.Contains(rust, "type info required for range statement") {
		t.Fatalf("range over variadic parameter should use syntax-derived slice type:\n%s", rust)
	}
	if !strings.Contains(rust, "__slice_holder = missing.clone()") || !strings.Contains(rust, "unwrap_or_default()") {
		t.Fatalf("slice variadic argument should clone the inner slice or use empty default:\n%s", rust)
	}
}

func TestVariadicByteArgumentsUseElementType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func fnv(seed uint32, bytes ...byte) uint32 {
	return seed
}

func use(hash uint32) uint32 {
	return fnv(hash, 'm', byte(hash>>24))
}
`)

	if strings.Contains(rust, "vec![('m' as i32),") {
		t.Fatalf("variadic byte rune literal should be cast to u8:\n%s", rust)
	}
	if strings.Contains(rust, "Rc<RefCell<Option<u8>>>") {
		t.Fatalf("variadic byte conversion should emit raw u8 elements, not wrapped handles:\n%s", rust)
	}
	if !strings.Contains(rust, "vec![('m' as i32) as u8, (*") {
		t.Fatalf("variadic byte arguments should use raw u8 vector elements:\n%s", rust)
	}
}

func TestVariadicStringCallResultClonesWrappedString(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func sink(args ...string) {}

func label(n uint32) string {
	return ""
}

func use(n uint32) {
	sink("value: ", label(n))
}
`)

	if strings.Contains(rust, ".as_ref().unwrap())]") {
		t.Fatalf("variadic string call result should clone the wrapped String before packing:\n%s", rust)
	}
	if !strings.Contains(rust, ".as_ref().unwrap()).clone()") {
		t.Fatalf("variadic string call result should clone the wrapped String before packing:\n%s", rust)
	}
}

func TestVariadicStringArgumentsCloneWrappedValues(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Context struct {
	GOROOT string
}

func (c *Context) joinPath(elem ...string) string {
	return ""
}

func (c *Context) srcDirs(paths []string) {
	_ = c.joinPath(c.GOROOT, "src")
	for _, p := range paths {
		_ = c.joinPath(p, "src")
	}
}
`)

	if strings.Contains(rust, "vec![self.g_o_r_o_o_t.clone(),") {
		t.Fatalf("variadic string field should be cloned as a raw String, not packed as a handle:\n%s", rust)
	}
	if strings.Contains(rust, "vec![p,") {
		t.Fatalf("variadic string range ref should be cloned as a raw String, not packed as a reference:\n%s", rust)
	}
	if !strings.Contains(rust, "(*p).clone()") {
		t.Fatalf("variadic string range ref should clone the referenced String:\n%s", rust)
	}
}

func TestVariadicStringSliceExprUsesRawElement(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func join(elem ...string) string {
	return ""
}

func caller(srcDir string, args []string) string {
	var out string
	for _, arg := range args {
		out = join(srcDir, arg[2:])
	}
	return out
}
`)

	if strings.Contains(rust, "Rc::new(RefCell::new(Some({ let __s = &(arg);") ||
		strings.Contains(rust, "Arc::new(Mutex::new(Some({ let __s = &(arg);") {
		t.Fatalf("variadic string slice expression should be packed as a raw String element:\n%s", rust)
	}
	if !strings.Contains(rust, "vec![(*srcDir") || !strings.Contains(rust, "{ let __s = &(arg);") {
		t.Fatalf("variadic string slice expression should stay inside the raw string vector:\n%s", rust)
	}
}

func TestExternalExecCommandLongVariadicArgsPackSlice(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "os/exec"

func run(goCmd, compiler, tags, suffix, path string) {
	_ = exec.Command(goCmd, "list", "-e", "-compiler="+compiler, "-tags="+tags, "-installsuffix="+suffix, "-f=x", "--", path)
}
`)

	if strings.Contains(rust, "exec::command(goCmd.clone(), (") {
		t.Fatalf("long exec.Command variadic args should not emit an overlarge tuple:\n%s", rust)
	}
	if !strings.Contains(rust, ", Rc::new(RefCell::new(Some(vec![") &&
		!strings.Contains(rust, ", Arc::new(Mutex::new(Some(vec![") {
		t.Fatalf("long exec.Command variadic args should be packed as a wrapped string slice:\n%s", rust)
	}
	if strings.Contains(rust, "\"--\".to_string(), path.clone()") {
		t.Fatalf("exec.Command variadic string ident should be cloned as a raw String in the packed slice:\n%s", rust)
	}
}

func TestVariadicAnyArgumentFromExistingAnyClonesDynamicValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func sink(args ...any) int { return len(args) }

func call(value any) int {
	return sink(value)
}
`)

	if strings.Contains(rust, "Box::new((*value.") && strings.Contains(rust, "clone()) as Box<dyn Any") {
		t.Fatalf("existing any passed to variadic any should not clone Box<dyn Any> directly:\n%s", rust)
	}
	if !strings.Contains(rust, "go_any_clone(") {
		t.Fatalf("existing any passed to variadic any should clone through go_any_clone:\n%s", rust)
	}
}

func TestVariadicAnyArgumentFromAnySliceClonesElements(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func sink(args ...any) int { return len(args) }

func call(values []any) int {
	return sink(values)
}
`)

	if strings.Contains(rust, "Box::new((*values.borrow().as_ref().unwrap()).clone()) as Box<dyn Any") ||
		strings.Contains(rust, "Box::new((*values.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any") {
		t.Fatalf("[]any passed as one variadic any argument should not clone the Vec<Box<dyn Any>> directly:\n%s", rust)
	}
	if !strings.Contains(rust, ".iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()") {
		t.Fatalf("[]any passed as one variadic any argument should clone elements through go_any_clone:\n%s", rust)
	}
}

func TestAnySliceElementTypeAssertionUsesBareBox(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func last(values []any) int {
	return values[len(values)-1].(int)
}
`)

	if strings.Contains(rust, "let __cloned = (*__seq_guard.as_ref().unwrap()).clone()") ||
		strings.Contains(rust, "let guard = val.borrow()") ||
		strings.Contains(rust, "let guard = val.lock().unwrap()") {
		t.Fatalf("[]any element type assertion should not clone the Vec or treat the element as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "[__idx].as_ref()") || !strings.Contains(rust, "downcast_ref::<i32>()") {
		t.Fatalf("[]any element type assertion should downcast the boxed element directly:\n%s", rust)
	}
}

func TestReferenceRangeComparisonDereferencesWithoutTypeInfo(t *testing.T) {
	expr, err := parser.ParseExpr("num > 6")
	if err != nil {
		t.Fatalf("ParseExpr() error = %v", err)
	}

	prevTypeInfo := currentTypeInfo
	prevRangeLoopVars := rangeLoopVars
	defer func() {
		currentTypeInfo = prevTypeInfo
		rangeLoopVars = prevRangeLoopVars
	}()
	SetTypeInfo(nil)
	rangeLoopVars = map[string]string{"num": "ref_value"}

	var out strings.Builder
	TranspileExpression(&out, expr)

	got := out.String()
	if !strings.Contains(got, "(*num).clone() > 6") {
		t.Fatalf("reference range comparison should own the range value, got:\n%s", got)
	}
	if strings.Contains(got, "num > 6") {
		t.Fatalf("reference range comparison used borrowed range value:\n%s", got)
	}
}

func TestElidedNestedSliceLiteralUsesOuterSyntaxWithoutTypeInfo(t *testing.T) {
	expr, err := parser.ParseExpr(`[][]string{{"a", "b"}, {}}`)
	if err != nil {
		t.Fatalf("ParseExpr() error = %v", err)
	}

	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	var out strings.Builder
	TranspileExpression(&out, expr)

	got := out.String()
	if strings.Contains(got, "CompositeLit with nil Type") || strings.Contains(got, "unimplemented!()") {
		t.Fatalf("elided nested slice literal should use outer syntax type, got:\n%s", got)
	}
	if !strings.Contains(got, `vec!["a".to_string(), "b".to_string()]`) {
		t.Fatalf("elided nested string slice literal did not emit owned strings:\n%s", got)
	}
	if !strings.Contains(got, "Vec::<String>::new()") {
		t.Fatalf("empty elided nested string slice literal needs explicit Vec type:\n%s", got)
	}
}

func TestTrackedRangeSlicePrintArgWithoutTypeInfo(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevRangeLoopVars := rangeLoopVars
	prevRangeElemTypes := localRangeElemRustTypes
	defer func() {
		currentTypeInfo = prevTypeInfo
		rangeLoopVars = prevRangeLoopVars
		localRangeElemRustTypes = prevRangeElemTypes
	}()
	SetTypeInfo(nil)
	localRangeElemRustTypes = make(map[string]string)

	expr, err := parser.ParseExpr(`[][]int{{1, 2}}`)
	if err != nil {
		t.Fatalf("ParseExpr() error = %v", err)
	}
	registerCompositeLiteralRangeElemType(ast.NewIdent("testData"), expr.(*ast.CompositeLit))
	elemRustType, ok := trackedRangeElemRustType(ast.NewIdent("testData"))
	if !ok || elemRustType != "Vec<i32>" {
		t.Fatalf("tracked range elem type = %q, %v; want Vec<i32>, true", elemRustType, ok)
	}

	rangeLoopVars = map[string]string{"data": rangeValueTypeFromTrackedRustElem(elemRustType)}
	var out strings.Builder
	transpilePrintArg(&out, ast.NewIdent("data"))

	got := out.String()
	if got != "format_slice_values(data)" {
		t.Fatalf("tracked range slice print arg = %q", got)
	}
}

func TestNoTypeInfoPrintTrackedLocalSlice(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevCollections := localCollectionKinds
	defer func() {
		currentTypeInfo = prevTypeInfo
		localCollectionKinds = prevCollections
	}()
	SetTypeInfo(nil)
	localCollectionKinds = map[string]string{"nums": "slice"}

	var out strings.Builder
	transpilePrintArg(&out, ast.NewIdent("nums"))

	if got := out.String(); got != "format_slice(&nums)" {
		t.Fatalf("tracked local slice print arg = %q", got)
	}
}

func TestNoTypeInfoPrintfSelectorSliceFieldUsesSyntax(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import "fmt"

type Manager struct {
	Team []string
}

func (m Manager) Manage() {
	fmt.Printf("team: %v\n", m.Team)
}`)

	if strings.Contains(rust, "Type information not available for print argument") {
		t.Fatalf("selector slice print arg should use syntax fallback:\n%s", rust)
	}
	if !strings.Contains(rust, "format_slice(&self.team)") {
		t.Fatalf("selector slice print arg should format the field handle:\n%s", rust)
	}
}

func TestNoTypeInfoErrorFieldKeepsHandleAndErrorMethodUsesSyntax(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import (
	"errors"
	"fmt"
)

type holder struct {
	err error
}

func main() {
	h := holder{err: errors.New("boom")}
	fmt.Println(h.err.Error())
}`)

	if !strings.Contains(rust, "Self { err: self.err.clone() }") {
		t.Fatalf("error field value clone should preserve the error handle:\n%s", rust)
	}
	if strings.Contains(rust, ".error().borrow()") {
		t.Fatalf("error field Error method should not call a nonexistent boxed-error method:\n%s", rust)
	}
	if !strings.Contains(rust, "format!(\"{}\", (*h.borrow().as_ref().unwrap()).err.borrow().as_ref().unwrap())") {
		t.Fatalf("error field Error method should format the error handle:\n%s", rust)
	}
}

func TestTupleAssignToErrorSliceKeepsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"errors"
	"fmt"
)

func parse() (int, error) {
	return 7, errors.New("bad")
}

func main() {
	values := make([]int, 1)
	errs := make([]error, 1)
	values[0], errs[0] = parse()
	fmt.Println(values[0])
	if errs[0] != nil {
		fmt.Println(errs[0].Error())
	}
}`)

	if strings.Contains(rust, "__tmp_1.borrow_mut().take().unwrap_or_default()") {
		t.Fatalf("tuple assignment into []error should not move the boxed payload out of the error handle:\n%s", rust)
	}
	if !strings.Contains(rust, "(*errs.borrow_mut().as_mut().unwrap())[(0) as usize] = __tmp_1;") {
		t.Fatalf("tuple assignment into []error should store the returned error handle:\n%s", rust)
	}
	if strings.Contains(rust, ".error().borrow()") {
		t.Fatalf("error slice element Error method should not call a nonexistent boxed-error method:\n%s", rust)
	}
	if !strings.Contains(rust, "format!(\"{}\", (*errs.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap())") {
		t.Fatalf("error slice element Error method should format the element handle:\n%s", rust)
	}
}

func TestErrorRangeVarErrorMethodFormatsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func collect(errs []error) []string {
	out := []string{}
	for _, err := range errs {
		out = append(out, err.Error())
	}
	return out
}
`)

	if strings.Contains(rust, `format!("{}", err)`) {
		t.Fatalf("error range variable Error method should not format the handle itself:\n%s", rust)
	}
	if !strings.Contains(rust, `format!("{}", (*err.borrow().as_ref().unwrap()))`) {
		t.Fatalf("error range variable Error method should format the boxed error in the handle:\n%s", rust)
	}
}

func TestConcurrentTupleReturnAvoidsRelockingLocalResult(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "fmt"

var elems map[string]string

func elem(name string) (string, bool) {
	elemType := elems[name]
	if elemType == "" {
		return "", false
	}
	return elemType, true
}

func forceConcurrent() {
	go func() {}()
	fmt.Println(elem("x"))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	prevDetector := GetConcurrencyDetector()
	detector := NewConcurrencyDetector()
	detector.AnalyzeFile(file)
	SetConcurrencyDetector(detector)
	defer SetConcurrencyDetector(prevDetector)

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, "let __tmp_x = (*elemType.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("tuple return should not relock elemType in a later tuple element:\n%s", rust)
	}
	if !strings.Contains(rust, "Arc::new(Mutex::new(Some(true)))") {
		t.Fatalf("tuple return should use a literal true result after the empty check:\n%s", rust)
	}
}

func TestNamedScalarReceiverFunctionArgumentUsesSelf(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type bitset uint64

func (b bitset) first() uintptr {
	return bitsetFirst(b)
}

func bitsetFirst(b bitset) uintptr {
	return uintptr(b)
}
`)

	if strings.Contains(rust, "bitset_first(Rc::new(RefCell::new(Some((*b.borrow()") ||
		strings.Contains(rust, "bitset_first(Arc::new(Mutex::new(Some((*b.lock()") {
		t.Fatalf("named scalar receiver used the Go receiver name instead of self in call argument:\n%s", rust)
	}
	if !strings.Contains(rust, "bitset_first(Rc::new(RefCell::new(Some(self.clone()))))") &&
		!strings.Contains(rust, "bitset_first(Arc::new(Mutex::new(Some(self.clone()))))") {
		t.Fatalf("named scalar receiver should pass self.clone() as the named value argument:\n%s", rust)
	}
}

func TestPointerReceiverPassedAsPointerArgumentUsesHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Map struct {
	seed int
}

type table struct{}

func (t *table) Put(m *Map) int {
	return m.seed
}

func (m *Map) Use(t *table) int {
	return t.Put(m)
}
`)

	if strings.Contains(rust, ".put(self.clone())") {
		t.Fatalf("pointer receiver passed as pointer argument should not pass the bare receiver value:\n%s", rust)
	}
	if !strings.Contains(rust, ".put(Rc::new(RefCell::new(Some(self.clone()))))") &&
		!strings.Contains(rust, ".put(Arc::new(Mutex::new(Some(self.clone()))))") {
		t.Fatalf("pointer receiver passed as pointer argument should be wrapped as a pointer handle:\n%s", rust)
	}
}

func TestNamedIntegerConversionCallArgumentWrapsExpectedNamedParam(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type ctrl uint8

type group struct{}

func h2(hash uintptr) uintptr {
	return hash >> 7
}

func (g *group) set(c ctrl) {}

func use(g *group, hash uintptr) {
	g.set(ctrl(h2(hash)))
}
`)

	if strings.Contains(rust, ".set(ctrl(") {
		t.Fatalf("named integer conversion call argument should not bypass parameter wrapping:\n%s", rust)
	}
	if !strings.Contains(rust, ".set(Rc::new(RefCell::new(Some(ctrl(") &&
		!strings.Contains(rust, ".set(Arc::new(Mutex::new(Some(ctrl(") {
		t.Fatalf("named integer conversion call argument should wrap the named value for the parameter:\n%s", rust)
	}
}

func TestConstBinaryFunctionArgumentUsesExpectedIntegerType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const slots = 8

func newTable(capacity uint64) {}

func use() {
	newTable(2 * slots)
}
`)

	if strings.Contains(rust, "Some({ let __tmp_x = 2; let __tmp_y = SLOTS; __tmp_x * __tmp_y })") {
		t.Fatalf("const binary function argument should not stay at the default integer type:\n%s", rust)
	}
	if !strings.Contains(rust, " as u64") {
		t.Fatalf("const binary function argument should cast to the expected uint64 parameter:\n%s", rust)
	}
}

func TestConstShiftExternalStubArgumentUsesExpectedIntegerType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "io"

func use(r io.ReaderAt) {
	_ = io.NewSectionReader(r, 0, 1<<63-1)
}
`)

	if strings.Contains(rust, "let __tmp_x = 1; let __tmp_y = 63; __tmp_x << __tmp_y") {
		t.Fatalf("const shift external stub argument should not stay at the default integer type:\n%s", rust)
	}
	if !strings.Contains(rust, "(1 as u64) << (63 as u64)") {
		t.Fatalf("const shift external stub argument should use a wide intermediate before the int64 cast:\n%s", rust)
	}
}

func TestNoTypeInfoExternalExecLookPathRegistersStub(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import (
	"fmt"
	"os/exec"
)

func main() {
	_, err := exec.LookPath("__go2rust_missing_executable__")
	fmt.Println(err != nil)
}`)

	if !strings.Contains(rust, "pub mod exec") || !strings.Contains(rust, "pub fn look_path") {
		t.Fatalf("exec.LookPath should register an inline external stdlib stub without type info:\n%s", rust)
	}
	if !strings.Contains(rust, "exec::look_path") {
		t.Fatalf("exec.LookPath call should target the generated exec stub:\n%s", rust)
	}
}

func TestNoTypeInfoExternalStdlibVariadicRegistersStubs(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import (
	"crypto/md5"
	"fmt"
	"io"
)

func main() {
	io.MultiWriter(io.Discard, md5.New())
	fmt.Println("ok")
}`)

	for _, want := range []string{"pub mod io", "pub fn Discard()", "pub fn multi_writer", "pub mod md5", "pub fn new()"} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in external stdlib fallback output:\n%s", want, rust)
		}
	}
	if !strings.Contains(rust, "fn __go_next_external_interface_id()") {
		t.Fatalf("external stdlib interface fallback should emit the interface id helper:\n%s", rust)
	}
	if !strings.Contains(rust, "pub struct hash_Hash {\n    pub __go_id: usize,") {
		t.Fatalf("md5.New fallback should register hash.Hash as an interface stub:\n%s", rust)
	}
	if strings.Contains(rust, "io::discard") {
		t.Fatalf("io.Discard should call the generated package variable accessor:\n%s", rust)
	}
	if !strings.Contains(rust, "io::Discard()") {
		t.Fatalf("io.Discard should use the generated package variable accessor:\n%s", rust)
	}
}

func TestNoTypeInfoBytesNewBufferStdlibInterfaceFieldDoesNotSynthesizeConversion(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import (
	"bytes"
	"fmt"
	"io"
)

type holder struct {
	w io.Writer
}

func main() {
	h := holder{w: bytes.NewBuffer(nil)}
	_ = fmt.Errorf("%v", h.w)
}`)

	// AGENTS.md "Type Info Is Authoritative": with typeInfo=nil, the
	// transpiler must not synthesize an io.Writer ← bytes.Buffer conversion
	// from the struct field syntax. The previous
	// writeStdlibInterfaceFieldValueFromSyntax peer path registered the
	// conversion stubs and emitted the impl-From lowering by reading the
	// field type expression — that branch was added in commit 50ecb15d
	// inside the 470fcb0b..3e3d9fc3 fallback-incident range and is gone.
	// The real fixture tests/fmt_errorf_interface_field still covers this
	// case via the typeInfo path (writeStdlibInterfaceCallArgumentConversion).
	if strings.Contains(rust, "impl From<bytes_Buffer> for io_Writer") {
		t.Fatalf("Mode 1 must not synthesize impl From<bytes_Buffer> for io_Writer from field syntax:\n%s", rust)
	}
	if strings.Contains(rust, "let __arg = bytes::new_buffer(") {
		t.Fatalf("Mode 1 must not emit the stdlib interface conversion __arg lowering without type info:\n%s", rust)
	}
}

func TestNoTypeInfoJsonMarshalEmitsUnimplemented(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import "encoding/json"

type User struct {
	Name string `+"`json:\"name\"`"+`
	Age  int    `+"`json:\"age\"`"+`
}

func main() {
	u := User{Name: "Alice", Age: 30}
	_, _ = json.Marshal(u)
}`)

	want := `unimplemented!("type info required for json.Marshal")`
	if !strings.Contains(rust, want) {
		t.Fatalf("json.Marshal without type info must emit %q per AGENTS.md \"Type Info Is Authoritative\":\n%s", want, rust)
	}
}

func TestNoTypeInfoStringSliceBoundsUseStringOutput(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

func trimParens(s string) string {
	return s[len("(") : len(s)-len(")")]
}`)

	if !strings.Contains(rust, "].to_string()") {
		t.Fatalf("string slice should produce a String under syntax fallback:\n%s", rust)
	}
	if strings.Contains(rust, "].to_vec()") {
		t.Fatalf("string slice should not use Vec output under syntax fallback:\n%s", rust)
	}
}

func TestStringSliceComplexBoundsUseLocalTemps(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Accuracy int8

const name = "BelowExactAbove"

var index = [...]uint8{0, 5, 10, 15}

func (i Accuracy) String() string {
	i -= -1
	return name[index[i]:index[i+1]]
}
`)

	if strings.Contains(rust, "__s[({ let __seq") {
		t.Fatalf("string slice should not borrow complex bounds inside the slice range:\n%s", rust)
	}
	if !strings.Contains(rust, "let __low =") || !strings.Contains(rust, "let __high =") ||
		!strings.Contains(rust, "__s[__low..__high].to_string()") {
		t.Fatalf("string slice should bind complex bounds before slicing:\n%s", rust)
	}
}

func TestNoTypeInfoMakeMapWithCapacityTracksMapSyntax(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import "fmt"

func main() {
	counts := make(map[string]int, 4)
	counts["go"]++
	counts["rust"] += 2
	fmt.Println(counts["go"], counts["rust"], len(counts))

	seen := make(map[int]bool, 3)
	seen[10] = true
	fmt.Println(seen[10], len(seen))
}`)

	for _, bad := range []string{
		"Cannot determine if map",
		"type info required for index expression",
		`as usize`,
	} {
		if strings.Contains(rust, bad) {
			t.Fatalf("make(map..., cap) should track map syntax without %q:\n%s", bad, rust)
		}
	}
	for _, want := range []string{
		`BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new()`,
		`.entry("go".to_string())`,
		`.entry("rust".to_string())`,
		`.get(&"go".to_string())`,
		`BTreeMap::<i32, Rc<RefCell<Option<bool>>>>::new()`,
		`.insert(__map_key, __map_value)`,
		`.get(&10)`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in make-map syntax fallback output:\n%s", want, rust)
		}
	}
}

func TestNoTypeInfoFixedArrayLocalIndexUsesSyntaxTracking(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import "fmt"

func main() {
	var buf [128]byte
	fmt.Println(len(buf), buf[0])
}`)

	for _, bad := range []string{
		"Cannot determine if map or slice access",
		"type info required for index expression",
	} {
		if strings.Contains(rust, bad) {
			t.Fatalf("fixed array local should track indexability without %q:\n%s", bad, rust)
		}
	}
	if !strings.Contains(rust, "[(0) as usize].clone()") {
		t.Fatalf("fixed array local index should use direct sequence indexing:\n%s", rust)
	}
}

func TestNoTypeInfoLocalCollectionTrackingIsFunctionScoped(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "fmt"

func makeNums() []int {
	result := make([]int, 0)
	return result
}

func main() {
	result := func() int { return 1 }()
	fmt.Println(result)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, "format_slice(&result)") {
		t.Fatalf("slice tracking for makeNums.result leaked into main.result:\n%s", rust)
	}
	if !strings.Contains(rust, "Immediate") && !strings.Contains(rust, "(*result.borrow().as_ref().unwrap())") {
		t.Fatalf("main.result should print as a scalar wrapped value:\n%s", rust)
	}
}

func TestNoTypeInfoStringParamRangeUsesSyntaxType(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func upper(s string) string {
	result := ""
	for _, char := range s {
		result += string(char)
	}
	return result
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, "type info required for range statement") {
		t.Fatalf("string parameter range should use syntax-derived parameter type:\n%s", rust)
	}
	if !strings.Contains(rust, ".char_indices()") {
		t.Fatalf("string parameter range should iterate chars:\n%s", rust)
	}
	if !strings.Contains(rust, "to_string()") {
		t.Fatalf("string(char) over a range rune should use the bare char value:\n%s", rust)
	}
	if strings.Contains(rust, "guard.as_ref().unwrap() +") {
		t.Fatalf("string += should not use numeric compound assignment:\n%s", rust)
	}
}

func TestPartialTypeInfoStringConversionCompoundAssignUsesSyntax(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func upper(s string) string {
	result := ""
	for _, char := range s {
		result += string(char - 32)
	}
	return result
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, &TypeInfo{})
	if !strings.Contains(rust, "push_str") {
		t.Fatalf("partial type info should still use string append syntax fallback:\n%s", rust)
	}
	if strings.Contains(rust, "guard.as_ref().unwrap() +") {
		t.Fatalf("partial type info should not force numeric compound assignment:\n%s", rust)
	}
	if !strings.Contains(rust, "char as i32") {
		t.Fatalf("string(char - n) should cast the range char before arithmetic:\n%s", rust)
	}
}

func TestStringCompoundAssignFromSliceUsesBareStringSlice(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func add(path string, start int, end int) string {
	dest := ""
	dest += path[start:end]
	return dest
}
`)

	if strings.Contains(rust, "push_str(&Rc::new") || strings.Contains(rust, "push_str(&Arc::new") {
		t.Fatalf("string += string slice should pass a bare String to push_str:\n%s", rust)
	}
	if !strings.Contains(rust, "__s[__low..__high].to_string()") {
		t.Fatalf("string += string slice should use the raw string-slice helper:\n%s", rust)
	}
}

func TestNoTypeInfoStringConcatUsesSyntaxStringOperand(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func join(prefix string, s string) string {
	return prefix + s
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if !strings.Contains(rust, "format!(\"{}{}\"") {
		t.Fatalf("string parameter concatenation should use syntax-derived string types:\n%s", rust)
	}
	if strings.Contains(rust, "__tmp_x + __tmp_y") {
		t.Fatalf("string parameter concatenation should not lower as numeric addition:\n%s", rust)
	}
}

func TestNoTypeInfoRuneSliceConversionTracksResult(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func reverse(s string) string {
	runes := []rune(s)
	return string(runes)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, "type info required") {
		t.Fatalf("[]rune/string conversion should not require go/types:\n%s", rust)
	}
	if !strings.Contains(rust, ".chars().map(|c| c as i32).collect::<Vec<_>>()") {
		t.Fatalf("[]rune(s) should lower through chars:\n%s", rust)
	}
	if !strings.Contains(rust, ".iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>()") {
		t.Fatalf("string(runes) should use the tracked rune slice element type:\n%s", rust)
	}
}

func TestSourceMappedReflectStructTagConversionUsesLocalType(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "type.go", `package reflect

type StructTag string

type StructField struct {
	Tag StructTag
}

func assign(tag string) StructField {
	var f StructField
	if tag != "" {
		f.Tag = StructTag(tag)
	}
	return f
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(type.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("reflect", []*ast.File{file}, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(reflect) error = %v", err)
	}

	rust := transpileParsedRegression(t, file, fset, typeInfo)
	if strings.Contains(rust, "GoReflectStructTag") {
		t.Fatalf("source-mapped reflect package should not use external reflect StructTag helper:\n%s", rust)
	}
	if !strings.Contains(rust, "StructTag(") {
		t.Fatalf("source-mapped reflect package should construct its local StructTag type:\n%s", rust)
	}
}

func TestSourceMappedNamedStringSelectorConversionUsesFieldHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "type.go", `package reflect

type StructTag string

type StructField struct {
	Tag StructTag
}

func tagString(field StructField) string {
	return string(field.Tag)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(type.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("reflect", []*ast.File{file}, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(reflect) error = %v", err)
	}

	rust := transpileParsedRegression(t, file, fset, typeInfo)
	if strings.Contains(rust, ".clone().lock()") || strings.Contains(rust, ".clone().borrow()") {
		t.Fatalf("string conversion from a named string selector should borrow the field handle, not lock the cloned value:\n%s", rust)
	}
	if !strings.Contains(rust, ".tag") || !strings.Contains(rust, ".to_string()") {
		t.Fatalf("string conversion from a named string selector should emit a field string conversion:\n%s", rust)
	}
}

func TestSourceMappedStdlibTimeDurationUsesNamedIntegerStub(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "flag", `package flag

import "time"

type durationValue struct {
	value time.Duration
}

type durationAlias time.Duration

func get(v durationValue) any {
	return v.value
}

func (v durationAlias) get() any {
	return time.Duration(v)
}

func text(v time.Duration) string {
	return v.String()
}
`, map[string]string{"flag": "flag"})

	if strings.Contains(rust, "std::time::Duration") {
		t.Fatalf("source-mapped stdlib package should not lower time.Duration through Rust std duration:\n%s", rust)
	}
	if !strings.Contains(rust, "time_Duration") {
		t.Fatalf("source-mapped stdlib package should keep stub-backed time.Duration as a named integer stub:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn string") {
		t.Fatalf("time.Duration selector call should register a loud external method stub:\n%s", rust)
	}
	if strings.Contains(rust, "time_Duration((*") && strings.Contains(rust, " as i64") {
		t.Fatalf("same-typed external named integer value should be cloned, not reconstructed through a cast:\n%s", rust)
	}
}

func TestRuneSliceSelectorStringConversionBorrowsFieldHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type inst struct {
	Rune []rune
}

func quote(i *inst) string {
	return string(i.Rune)
}

func main() {
	go func() {}()
}
`)

	if strings.Contains(rust, ".clone().lock().unwrap().as_ref().unwrap()).iter()") {
		t.Fatalf("string conversion from a rune slice selector should not relock a cloned Vec:\n%s", rust)
	}
	if !strings.Contains(rust, ".iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>()") {
		t.Fatalf("string conversion from a rune slice selector should iterate rune elements:\n%s", rust)
	}
}

func TestPointerFieldStructLiteralUsesPackageGlobalPointerHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type struct{}

type Value struct {
	typ *Type
}

func rtypeOf() *Type {
	return &Type{}
}

var uint8Type = rtypeOf()

func Make() Value {
	return Value{uint8Type}
}
`)

	if strings.Contains(rust, "typ: uint8Type.clone()") {
		t.Fatalf("pointer field literal should not store the package-global slot handle:\n%s", rust)
	}
	if !strings.Contains(rust, "typ: (*uint8Type.borrow().as_ref().unwrap()).clone()") {
		t.Fatalf("pointer field literal should clone the pointer handle stored in the package global:\n%s", rust)
	}
}

func TestAddressOfParenthesizedSelectorKeepsAddressContext(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Type struct{}

type rtype struct {
	t Type
}

type arrayType struct {
	slice *Type
}

func assign(rt *rtype) arrayType {
	var array arrayType
	array.slice = &(rt.t)
	return array
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust := transpileParsedRegression(t, file, fset, typeInfo)
	if strings.Contains(rust, ".t.borrow().as_ref().unwrap()).clone()).clone()") ||
		strings.Contains(rust, ".t.lock().unwrap().as_ref().unwrap()).clone()).clone()") {
		t.Fatalf("address-of parenthesized selector should keep the field handle, not clone the selected value:\n%s", rust)
	}
	if !strings.Contains(rust, ".t).clone()") && !strings.Contains(rust, ".t.clone()") {
		t.Fatalf("address-of parenthesized selector should clone the field handle:\n%s", rust)
	}
}

func TestUnsafePointerArrayLiteralZeroUsesTypedNil(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "unsafe"

func makeAny() any {
	var iarray any = [1]unsafe.Pointer{}
	return iarray
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("main", []*ast.File{file}, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(main) error = %v", err)
	}

	rust := transpileParsedRegression(t, file, fset, typeInfo)
	if strings.Contains(rust, "[Default::default()]") {
		t.Fatalf("unsafe.Pointer array literal zero should not emit trait-level Default::default():\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new([0])") {
		t.Fatalf("unsafe.Pointer array literal zero should emit a typed nil pointer value:\n%s", rust)
	}
}

func TestUnsafePointerStructFieldNilUsesTypedDefault(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

type Value struct {
	ptr unsafe.Pointer
}

func zero() Value {
	return Value{nil}
}
`)

	if strings.Contains(rust, "ptr: Rc::new(RefCell::new(Some(None)))") ||
		strings.Contains(rust, "ptr: Arc::new(Mutex::new(Some(None)))") {
		t.Fatalf("unsafe.Pointer struct field nil should not double-wrap None:\n%s", rust)
	}
	if !strings.Contains(rust, "ptr: Default::default()") {
		t.Fatalf("unsafe.Pointer struct field nil should use the typed handle default:\n%s", rust)
	}
}

func TestImportedInterfaceStructFieldNilUsesEmptyHandle(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "go/types", `package types

import "go/constant"

type operand struct {
	val constant.Value
}

func record(x *operand) {}

func use() {
	record(&operand{nil})
}
`, map[string]string{"go/constant": "go_constant", "go/types": "go_types"})
	if strings.Contains(rust, "val: Rc::new(RefCell::new(Some(None)))") ||
		strings.Contains(rust, "val: Arc::new(Mutex::new(Some(None)))") {
		t.Fatalf("nil imported-interface struct field should not double-wrap nil:\n%s", rust)
	}
	if !strings.Contains(rust, "val: Rc::new(RefCell::new(None))") &&
		!strings.Contains(rust, "val: Arc::new(Mutex::new(None))") &&
		!strings.Contains(rust, "val: Default::default()") {
		t.Fatalf("nil imported-interface struct field should clear the field handle:\n%s", rust)
	}
}

func TestNamedPointerNilConversionConstructsNamedHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Mutex struct{}

type P *struct{}

func box() any {
	go func() {}()
	var x any
	x = P(nil)
	return x
}
`)

	if strings.Contains(rust, "Some(None)") {
		t.Fatalf("named pointer nil conversion should not double-wrap nil:\n%s", rust)
	}
	if strings.Contains(rust, "P(Arc::new(Mutex::new") {
		t.Fatalf("named pointer nil conversion should use the aliased wrapper mutex:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(P(Arc::new(StdMutex::new(None::<AnonymousStruct1>)))) as Box<dyn Any + Send + Sync>") {
		t.Fatalf("named pointer nil conversion should construct the named pointer handle before boxing as any:\n%s", rust)
	}
	if !strings.Contains(rust, `go_register_any_type_with_elem::<P>("pointer", true, "struct", true)`) {
		t.Fatalf("named pointer nil conversion boxed as any should register Go pointer metadata:\n%s", rust)
	}
}

func TestErrorPointerNilConversionBoxedAsAnyRegistersPointerMetadata(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func box() any {
	go func() {}()
	return (*error)(nil)
}
`)

	if !strings.Contains(rust, "Box::new(Arc::new(Mutex::new(None::<Option<Box<dyn StdError + Send + Sync>>>))) as Box<dyn Any + Send + Sync>") {
		t.Fatalf("error pointer nil conversion should preserve the typed nil pointer payload:\n%s", rust)
	}
	if !strings.Contains(rust, `go_register_any_type_with_elem::<Arc<Mutex<Option<Option<Box<dyn StdError + Send + Sync>>>>>>("pointer", true, "interface", true)`) {
		t.Fatalf("error pointer nil conversion boxed as any should register Go pointer metadata:\n%s", rust)
	}
}

func TestFuncLiteralReturningAnyBoxesUnsafePointerCall(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "unsafe"

type Type struct{}

type Pool struct {
	New func() any
}

func alloc(t *Type) unsafe.Pointer {
	return nil
}

func makePool(t *Type) Pool {
	return Pool{New: func() any {
		return alloc(t)
	}}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("main", []*ast.File{file}, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter(main) error = %v", err)
	}

	rust := transpileParsedRegression(t, file, fset, typeInfo)
	if strings.Contains(rust, "return alloc(") {
		t.Fatalf("func literal returning any should not return the unsafe pointer handle directly:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new({ let __v = alloc(") {
		t.Fatalf("func literal returning any should box the unsafe pointer call result:\n%s", rust)
	}
}

func TestParenthesizedNumericConversionTargetWrapsBinaryOperand(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

var done chan int

func extractBits(data uint64, start, end uint) uint {
	return (uint)(data>>start) & ((1 << (end - start + 1)) - 1)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
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

	if strings.Contains(rust, "}.borrow().as_ref().unwrap()") ||
		strings.Contains(rust, "}.lock().unwrap().as_ref().unwrap()") ||
		strings.Contains(rust, ").borrow().as_ref().unwrap().borrow().as_ref().unwrap()") ||
		strings.Contains(rust, ").lock().unwrap().as_ref().unwrap().lock().unwrap().as_ref().unwrap()") {
		t.Fatalf("parenthesized numeric conversion target should not leave a bare block for binary unwrapping:\n%s", rust)
	}
	if !strings.Contains(rust, " as "+rustUintType()) {
		t.Fatalf("parenthesized uint conversion target should still cast to the Rust uint representation:\n%s", rust)
	}
}

func TestNumericConversionCastsConstantShiftOperandsBeforeEvaluation(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func maxWork() int64 {
	checkWork := int64(1<<63 - 1)
	return checkWork
}

func decHead() int64 {
	delta := int64(-(1 << 32))
	return delta
}

func add(delta int64) int64 {
	return delta
}

func useAdd() int64 {
	return add(-(1 << 32))
}

type pallocSum uint64

func pack(max uint) pallocSum {
	if max == 0 {
		return pallocSum(uint64(1 << 63))
	}
	return 0
}
`)

	if strings.Contains(rust, "let __tmp_x = 1; let __tmp_y = 63; __tmp_x << __tmp_y") {
		t.Fatalf("numeric conversion of a constant shift should not evaluate the shift before widening:\n%s", rust)
	}
	if !strings.Contains(rust, "1 as u64") || !strings.Contains(rust, "63 as u64") {
		t.Fatalf("numeric conversion of a constant shift should widen operands before evaluating the shift:\n%s", rust)
	}
	if strings.Contains(rust, "-(1 << 32) as i64") {
		t.Fatalf("numeric conversion of a negated constant shift should not evaluate the shift before widening:\n%s", rust)
	}
	if !strings.Contains(rust, "1 as i64") || !strings.Contains(rust, "32 as i64") {
		t.Fatalf("numeric conversion of a negated constant shift should widen operands before applying unary negation:\n%s", rust)
	}
	if strings.Contains(rust, "Some(-((1 << 32)) as i64)") ||
		strings.Contains(rust, "Some(-(1 << 32) as i64)") {
		t.Fatalf("constant shift call argument should widen operands before applying unary negation:\n%s", rust)
	}
	if strings.Contains(rust, "((1 << 63) as u64)") {
		t.Fatalf("nested constant conversion should widen shift operands before casting:\n%s", rust)
	}
}

func TestFloatBinaryExpressionCastsIntegerConstantIdentifierOperand(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func fastlog2(x float64) float64 {
	return x
}

func sample(q uint32) float64 {
	go func() {}()
	const randomBitCount = 26
	qlog := fastlog2(float64(q)) - randomBitCount
	return qlog
}
`)

	if strings.Contains(rust, "let __tmp_y = randomBitCount; __tmp_x - __tmp_y") ||
		strings.Contains(rust, "let __tmp_y = RANDOM_BIT_COUNT; __tmp_x - __tmp_y") {
		t.Fatalf("float binary expression should not preserve an integer-typed constant operand without a cast:\n%s", rust)
	}
	if !strings.Contains(rust, "randomBitCount as f64") &&
		!strings.Contains(rust, "RANDOM_BIT_COUNT as f64") {
		t.Fatalf("float binary expression should cast the constant identifier operand to f64:\n%s", rust)
	}
}

func TestUnsafePointerConversionUsesCurrentReceiver(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "unsafe"

type Type struct{}

func (t *Type) addr() uintptr {
	return uintptr(unsafe.Pointer(t))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)

	if strings.Contains(rust, "Arc::as_ptr(&t)") || strings.Contains(rust, "Rc::as_ptr(&t)") {
		t.Fatalf("unsafe.Pointer conversion for current receiver should use the lowered receiver, not the Go receiver name:\n%s", rust)
	}
	if !strings.Contains(rust, "self as *const _ as usize") {
		t.Fatalf("unsafe.Pointer conversion should take the address of the lowered receiver:\n%s", rust)
	}
}

func TestUnsafePointerAddressOfArrayElementAvoidsSliceElemPtr(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

type regs struct {
	ints [4]uintptr
}

func (r *regs) addr(i int) uintptr {
	return uintptr(unsafe.Pointer(&r.ints[i]))
}
`)

	if strings.Contains(rust, "GoSliceElemPtr::new") {
		t.Fatalf("unsafe.Pointer(&array[index]) should not use the slice element pointer helper:\n%s", rust)
	}
	if !strings.Contains(rust, "as *const _ as usize") {
		t.Fatalf("unsafe.Pointer(&array[index]) should lower through an element address:\n%s", rust)
	}
}

func TestUnsafePointerAddressOfPointerToArrayElementBorrowsPointerArray(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

func addr(values *[4]uint64, i int) uintptr {
	return uintptr(unsafe.Pointer(&(*values)[i]))
}
`)

	if strings.Contains(rust, "let __seq_holder = ({ let __v = (*values") {
		t.Fatalf("unsafe.Pointer(&(*arrayPtr)[index]) should not clone the array before borrowing its element:\n%s", rust)
	}
	if !strings.Contains(rust, "let __seq_holder = values.clone(); let __seq_guard = __seq_holder.borrow(); &__seq_guard.as_ref().unwrap()[") {
		t.Fatalf("unsafe.Pointer(&(*arrayPtr)[index]) should borrow the pointer's array payload:\n%s", rust)
	}
}

func TestUnsafePointerAddressOfNestedSequenceElementBorrowsOuterSequence(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

type holder struct {
	summary [2][]uint64
}

func (h *holder) addr(level int) uintptr {
	return uintptr(unsafe.Pointer(&h.summary[level][0]))
}
`)

	if strings.Contains(rust, "let __seq_holder = { let __seq =") {
		t.Fatalf("unsafe.Pointer(&outer[index][index]) should not clone the inner sequence value as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __outer_holder = self.summary.clone();") ||
		!strings.Contains(rust, "let __inner_seq = &__outer_guard.as_ref().unwrap()[") ||
		!strings.Contains(rust, "&__inner_seq[(0) as usize] as *const _ as usize") {
		t.Fatalf("unsafe.Pointer(&outer[index][index]) should borrow the outer sequence and address the inner element:\n%s", rust)
	}
}

func TestUnsafePointerAddressOfBareUintptrLocalUsesRawAddress(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

func code() uintptr {
	return 1
}

func ptr() unsafe.Pointer {
	code := code()
	return unsafe.Pointer(&code)
}
`)

	if strings.Contains(rust, "Arc::as_ptr(&code.clone())") ||
		strings.Contains(rust, "Rc::as_ptr(&code.clone())") {
		t.Fatalf("unsafe.Pointer(&bareUintptrLocal) should not call wrapper as_ptr on a raw uintptr:\n%s", rust)
	}
	if !strings.Contains(rust, "&code as *const _ as usize") {
		t.Fatalf("unsafe.Pointer(&bareUintptrLocal) should use the raw local address:\n%s", rust)
	}
}

func TestUnsafePointerToNamedStructConversionDoesNotExposeRawPointerToSelector(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

type Header struct {
	Len uintptr
}

type Type struct{}

func (t *Type) Len() int {
	return int((*Header)(unsafe.Pointer(t)).Len)
}
`)

	if strings.Contains(rust, "as usize))).lock().unwrap().as_ref().unwrap()).len") {
		t.Fatalf("unsafe.Pointer to named struct conversion should not expose a raw usize to field selection:\n%s", rust)
	}
	if !strings.Contains(rust, "unimplemented!(\"unsafe.Pointer conversion to Header\")") {
		t.Fatalf("unsafe.Pointer to named struct conversion should emit a typed loud unsupported path:\n%s", rust)
	}
}

func TestPointerToStructAliasUsesUnderlyingPointee(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Target struct {
	Elem *int
}

type Alias = Target

func elem(p *Alias) *int {
	return p.Elem
}
`)

	if strings.Contains(rust, "Option<Alias>") {
		t.Fatalf("pointer to struct alias should not point at the full alias handle:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn elem(p: Rc<RefCell<Option<Target>>>)") {
		t.Fatalf("pointer to struct alias should use the underlying pointee type:\n%s", rust)
	}
}

func TestUnsafePointerToStructAliasConversionUsesUnderlyingPointee(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

type Target struct {
	Elem *int
}

type Alias = Target

type Type struct{}

func elem(t *Type) *int {
	return (*Alias)(unsafe.Pointer(t)).Elem
}
`)

	if strings.Contains(rust, "Some::<Alias>") {
		t.Fatalf("unsafe.Pointer conversion to struct alias should not use the full alias handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Some::<Target>(unimplemented!(\"unsafe.Pointer conversion to Target\"))") {
		t.Fatalf("unsafe.Pointer conversion to struct alias should use the underlying pointee type:\n%s", rust)
	}
}

func TestUnsafePointerToArrayConversionForSliceDoesNotIndexRawPointer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

type Method struct{}

func methods(p unsafe.Pointer, n int) []Method {
	return (*[1 << 4]Method)(p)[:n:n]
}
`)

	if strings.Contains(rust, "let __seq_holder = p.clone()") {
		t.Fatalf("unsafe.Pointer to array conversion should not expose the raw pointer handle to slicing:\n%s", rust)
	}
	if !strings.Contains(rust, "Some::<[Method; 16]>(unimplemented!(\"unsafe.Pointer conversion to [Method; 16]\"))") {
		t.Fatalf("unsafe.Pointer to array conversion should emit a typed unsupported array pointee:\n%s", rust)
	}
}

func TestUintptrConversionFromUnsafePointerArrayIndexKeepsIndexedScalarBare(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "unsafe"

func addr(x any) uintptr {
	go func() {}()
	return uintptr((*[2]unsafe.Pointer)(unsafe.Pointer(&x))[1])
}
`)

	if strings.Contains(rust, "].clone() }.borrow()") ||
		strings.Contains(rust, "].clone() }.lock()") {
		t.Fatalf("uintptr conversion from unsafe pointer array index should not borrow the indexed scalar:\n%s", rust)
	}
	if !strings.Contains(rust, "].clone()") {
		t.Fatalf("uintptr conversion from unsafe pointer array index should keep the indexed scalar value:\n%s", rust)
	}
}

func TestUnsafePointerToPointerConversionUsesTypedUnsupportedPointee(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

type table struct{}

func directoryAt(dir unsafe.Pointer, i uintptr) *table {
	return *(**table)(unsafe.Pointer(uintptr(dir) + i))
}
`)

	if strings.Contains(rust, "as usize))).borrow().as_ref().unwrap()).clone()") {
		t.Fatalf("unsafe.Pointer to pointer conversion should not expose raw uintptr to dereference:\n%s", rust)
	}
	if !strings.Contains(rust, "Some::<Rc<RefCell<Option<table>>>>(unimplemented!(\"unsafe.Pointer conversion to Rc<RefCell<Option<table>>>\")") {
		t.Fatalf("unsafe.Pointer to pointer conversion should emit a typed unsupported pointee:\n%s", rust)
	}
}

func TestUnsafePointerToPointerConversionKeepsSourceHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

func load(slot unsafe.Pointer) unsafe.Pointer {
	return *((*unsafe.Pointer)(slot))
}
`)

	if strings.Contains(rust, "let __ptr = { let __v = (*slot.borrow().as_ref().unwrap()).clone(); __v }; let __ptr_guard = __ptr.borrow()") {
		t.Fatalf("unsafe.Pointer to pointer conversion should not unwrap the source handle before nil check:\n%s", rust)
	}
	if !strings.Contains(rust, "let __ptr = slot.clone(); let __ptr_guard = __ptr.borrow()") {
		t.Fatalf("unsafe.Pointer to pointer conversion should borrow the wrapped source handle:\n%s", rust)
	}
}

func TestUnsafePointerToPointerConversionWrapsUnsafePointerDerefValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

type Type struct{}

func load(data [1]unsafe.Pointer) *Type {
	return (*Type)(*(*unsafe.Pointer)(unsafe.Pointer(&data[0])))
}
`)

	if strings.Contains(rust, "let __ptr = (*Rc::new(RefCell::new({") ||
		strings.Contains(rust, "let __ptr = (*Arc::new(Mutex::new({") {
		t.Fatalf("unsafe.Pointer deref source should not expose a raw usize to the outer pointer conversion:\n%s", rust)
	}
	if !strings.Contains(rust, " -> GoPtr<Type>") || !strings.Contains(rust, "GoPtr::raw(") {
		t.Fatalf("unsafe.Pointer deref source returned as a pointer should preserve raw pointer identity through GoPtr:\n%s", rust)
	}
	if !strings.Contains(rust, "unimplemented!(\"unsafe.Pointer conversion to usize\")") {
		t.Fatalf("unsafe.Pointer deref source should still fail loudly at the unsupported loaded-pointer boundary:\n%s", rust)
	}
}

func TestUnsafePointerAnyAddressToInternalABIEmptyInterfaceUsesRuntimeMirror(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "internal/reflectlite", `package reflectlite

import (
	"internal/abi"
	"unsafe"
)

func typOf(i any) *abi.Type {
	e := (*abi.EmptyInterface)(unsafe.Pointer(&i))
	return e.Type
}
`, map[string]string{"internal/abi": "internal_abi"})

	if strings.Contains(rust, "unsafe.Pointer conversion to internal_abi::iface::EmptyInterface") {
		t.Fatalf("unsafe.Pointer(&any) to *internal/abi.EmptyInterface should use the runtime interface mirror:\n%s", rust)
	}
	if !strings.Contains(rust, "internal_abi::type_of(__iface_value.clone())") {
		t.Fatalf("runtime interface mirror should derive Type through internal/abi.TypeOf lowering:\n%s", rust)
	}
	if !strings.Contains(rust, "r#type:") || !strings.Contains(rust, "data:") {
		t.Fatalf("runtime interface mirror should initialize EmptyInterface layout fields:\n%s", rust)
	}
}

func TestReflectliteTypeOfErrorPointerElemUsesReflectliteCall(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "errors", `package errors

import "internal/reflectlite"

var errorType = reflectlite.TypeOf((*error)(nil)).Elem()
`, nil)

	if strings.Contains(rust, `reflectlite_Type::__go_from("error".to_string())`) {
		t.Fatalf("reflectlite.TypeOf((*error)(nil)).Elem should not emit an opaque error type token:\n%s", rust)
	}
	if !strings.Contains(rust, `reflectlite::type_of`) || !strings.Contains(rust, `.elem()`) {
		t.Fatalf("reflectlite.TypeOf((*error)(nil)).Elem should keep the generated reflectlite call:\n%s", rust)
	}
}

func TestSourceMappedReflectliteTypeOfErrorPointerElemUsesSourceCall(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "errors", `package errors

import "internal/reflectlite"

var errorType = reflectlite.TypeOf((*error)(nil)).Elem()
`, map[string]string{"internal/reflectlite": "internal_reflectlite"})

	if strings.Contains(rust, `reflectlite_Type::__go_from("error".to_string())`) {
		t.Fatalf("source-mapped reflectlite should not use the external opaque type token:\n%s", rust)
	}
	if !strings.Contains(rust, `internal_reflectlite::type_of`) {
		t.Fatalf("source-mapped reflectlite should keep the generated source call:\n%s", rust)
	}
}

func TestUnsafePointerToAnonymousInterfaceDerefIsLoudUnsupported(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

func load(slot unsafe.Pointer) any {
	return *(*interface{ M() })(slot)
}
`)

	if strings.Contains(rust, ".as_ref().unwrap()).clone().borrow()") ||
		strings.Contains(rust, ".as_ref().unwrap()).clone().lock()") {
		t.Fatalf("unsafe.Pointer interface dereference should not treat the pointer payload as a wrapped handle:\n%s", rust)
	}
	if !strings.Contains(rust, `unimplemented!("unsafe.Pointer conversion to Box<dyn Any`) {
		t.Fatalf("unsafe.Pointer interface dereference should fail loudly at the typed boundary:\n%s", rust)
	}
}

func TestUnsafePointerConversionFromSliceElementPointerIsLoudUnsupported(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "unsafe"

type eface struct {
	typ, val unsafe.Pointer
}

func load(vals []eface, i int) any {
	go func() {}()
	slot := &vals[i]
	return *(*any)(unsafe.Pointer(slot))
}
`)

	if strings.Contains(rust, "Arc::as_ptr(&slot)") {
		t.Fatalf("unsafe.Pointer conversion from slice element pointer should not treat the helper as an Arc handle:\n%s", rust)
	}
	if !strings.Contains(rust, `unimplemented!("unsafe.Pointer conversion from slice element pointer`) {
		t.Fatalf("unsafe.Pointer conversion from slice element pointer should fail loudly at the typed boundary:\n%s", rust)
	}
	if !strings.Contains(rust, "let __unsupported: usize = unimplemented!") {
		t.Fatalf("unsafe.Pointer conversion from slice element pointer should anchor the unsupported pointer value as usize:\n%s", rust)
	}
}

func TestUnsafePointerConversionFromEmbeddedFirstFieldUsesOwnerRegistry(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "unsafe"

type node struct {
	isEntry bool
}

type entry struct {
	node
	value int
}

func newEntry(value int) *entry {
	return &entry{node: node{isEntry: true}, value: value}
}

func (n *node) entry() *entry {
	if !n.isEntry {
		panic("not entry")
	}
	return (*entry)(unsafe.Pointer(n))
}

func use() int {
	e := newEntry(7)
	return e.node.entry().value
}
`)

	if strings.Contains(rust, `unimplemented!("unsafe.Pointer conversion to entry")`) {
		t.Fatalf("embedded first-field unsafe conversion should not use the generic unsupported path:\n%s", rust)
	}
	if !strings.Contains(rust, "go_register_embedded_owner(__embedded_key, __owner.clone())") {
		t.Fatalf("pointer composite literal should register its embedded field owner:\n%s", rust)
	}
	if !strings.Contains(rust, `go_lookup_embedded_owner::<entry>(*__ptr_guard.as_ref().unwrap(), "entry")`) {
		t.Fatalf("embedded first-field unsafe conversion should use typed owner lookup:\n%s", rust)
	}
}

func TestUnsafePointerDerefNilComparisonUsesPointerValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

func nilWord(slot unsafe.Pointer) bool {
	return *(*unsafe.Pointer)(slot) == nil
}
`)

	if strings.Contains(rust, ".as_mut().unwrap()).borrow()") ||
		strings.Contains(rust, ".as_mut().unwrap()).lock()") {
		t.Fatalf("unsafe.Pointer deref nil comparison should not borrow after extracting the pointer value:\n%s", rust)
	}
	if !strings.Contains(rust, "== 0") {
		t.Fatalf("unsafe.Pointer deref nil comparison should compare the loaded pointer value to zero:\n%s", rust)
	}
}

func TestUnsafePointerSelectorFieldUsesPointerHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

type Type struct{}

type EmptyInterface struct {
	Type *Type
}

func addr(e EmptyInterface) uintptr {
	return uintptr(unsafe.Pointer(e.Type))
}
`)

	if strings.Contains(rust, ".r#type.borrow().as_ref().unwrap())) as usize") {
		t.Fatalf("unsafe.Pointer selector conversion should take the address of the pointer handle, not the pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "as_ptr(&") || !strings.Contains(rust, ".r#type.clone()") {
		t.Fatalf("unsafe.Pointer selector conversion should use the cloned pointer field handle:\n%s", rust)
	}
}

func TestGenericValueCallArgumentBoxesForAnyParameter(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func typeOf(a any) {}

func typeFor[T any]() {
	var v T
	typeOf(v)
}
`)

	if strings.Contains(rust, "type_of((*v.") {
		t.Fatalf("generic value passed to any parameter should not be emitted as raw T:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }) as Box<dyn Any") {
		t.Fatalf("generic value passed to any parameter should be boxed as Any:\n%s", rust)
	}
}

func TestGenericValueAnyConversionBoxesTypeParam(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func box[T any]() any {
	var v T
	return any(v)
}
`)

	if strings.Contains(rust, "return v.clone()") {
		t.Fatalf("generic value converted to any should not be treated as an existing any handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new((*v.") || !strings.Contains(rust, "as Box<dyn Any") {
		t.Fatalf("generic value converted to any should be boxed as Any:\n%s", rust)
	}
}

func TestGenericNilPointerCallArgumentBoxesTypedNone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func typeOf(a any) {}

func typeFor[T any]() {
	typeOf((*T)(nil))
}
`)

	if strings.Contains(rust, "RefCell::new(None))") || strings.Contains(rust, "Mutex::new(None))") {
		t.Fatalf("generic nil pointer boxed as any should not emit untyped None:\n%s", rust)
	}
	if !strings.Contains(rust, "None::<T>") || !strings.Contains(rust, "as Box<dyn Any") {
		t.Fatalf("generic nil pointer boxed as any should include typed None inside Any box:\n%s", rust)
	}
}

func TestTypedNilMapAnyVarInitializerBoxesTypedNone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func boxMap() any {
	var x any = (map[int]int)(nil)
	return x
}
`)

	if strings.Contains(rust, "let mut x: Rc<RefCell<Option<Box<dyn Any>>>> = None;") ||
		strings.Contains(rust, "Box::new(None)") {
		t.Fatalf("typed nil map in any initializer should not emit a nil interface or untyped None:\n%s", rust)
	}
	if !strings.Contains(rust, "None::<BTreeMap<i32,") || !strings.Contains(rust, "as Box<dyn Any") {
		t.Fatalf("typed nil map in any initializer should preserve the typed nil map inside Any:\n%s", rust)
	}
}

func TestMapCallArgumentToAnyBoxesMapHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func typeOf(a any) {}

func use[K comparable, V any](m map[K]V) {
	typeOf(m)
}
`)

	if strings.Contains(rust, "Box::new((*m.borrow().as_ref().unwrap()).clone()) as Box<dyn Any") ||
		strings.Contains(rust, "Box::new((*m.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any") {
		t.Fatalf("map value passed to any should box the map handle, not clone the BTreeMap:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(m.clone()) as Box<dyn Any") {
		t.Fatalf("map value passed to any should clone the map handle into Any:\n%s", rust)
	}
}

func TestInternalABITypeOfMapTypeCallUsesTypedMapIntrinsic(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevContext := GetTranspileContext()
	prevCD := GetConcurrencyDetector()
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
		SetTranspileContext(prevContext)
		SetConcurrencyDetector(prevCD)
	})

	cd := NewConcurrencyDetector()
	cd.hasGoroutines = true
	SetConcurrencyDetector(cd)

	abiIdent := ast.NewIdent("abi")
	mIdent := ast.NewIdent("m")
	call := &ast.CallExpr{Fun: &ast.SelectorExpr{
		X:   &ast.CallExpr{Fun: &ast.SelectorExpr{X: abiIdent, Sel: ast.NewIdent("TypeOf")}, Args: []ast.Expr{mIdent}},
		Sel: ast.NewIdent("MapType"),
	}}

	abiPkg := types.NewPackage("internal/abi", "abi")
	currentPkg := types.NewPackage("internal/sync", "sync")
	mapType := types.NewMap(types.Typ[types.String], types.Typ[types.Int])
	currentTypeInfo = &TypeInfo{
		info: &types.Info{
			Uses: map[*ast.Ident]types.Object{
				abiIdent: types.NewPkgName(token.NoPos, currentPkg, "abi", abiPkg),
				mIdent:   types.NewVar(token.NoPos, currentPkg, "m", mapType),
			},
			Types: map[ast.Expr]types.TypeAndValue{
				mIdent: {Type: mapType},
			},
		},
		pkg: currentPkg,
	}
	SetTranspileContext(&TranspileContext{PackageMapping: map[string]string{"internal/abi": "internal_abi"}})

	var out strings.Builder
	TranspileExpression(&out, call)
	got := out.String()

	for _, forbidden := range []string{"type_of(", ".map_type()", "unsupported Rust Any payload"} {
		if strings.Contains(got, forbidden) {
			t.Fatalf("internal/abi.TypeOf(map).MapType should not route through erased Any runtime path %q:\n%s", forbidden, got)
		}
	}
	for _, forbidden := range []string{"wrapping_mul(2654435761usize)", "unwrap_or(0); __key_value"} {
		if strings.Contains(got, forbidden) {
			t.Fatalf("internal/abi.TypeOf(map).MapType should not hash erased pointer values via %q:\n%s", forbidden, got)
		}
	}
	for _, want := range []string{
		"internal_abi::SwissMapType::default()",
		"internal_abi::Kind(Arc::new(Mutex::new(Some(internal_abi::MAP as u8))))",
		"let __hasher: Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> usize + Send + Sync>",
		"as *const Mutex<Option<String>>",
		"GoComparable::go_hash(__key_value, __seed_value)",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestInternalABITypeOfMapTypeGoPtrValueUsesLocalHandle(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevContext := GetTranspileContext()
	prevCD := GetConcurrencyDetector()
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
		SetTranspileContext(prevContext)
		SetConcurrencyDetector(prevCD)
	})

	cd := NewConcurrencyDetector()
	cd.hasGoroutines = true
	SetConcurrencyDetector(cd)

	abiIdent := ast.NewIdent("abi")
	mIdent := ast.NewIdent("m")
	call := &ast.CallExpr{Fun: &ast.SelectorExpr{
		X:   &ast.CallExpr{Fun: &ast.SelectorExpr{X: abiIdent, Sel: ast.NewIdent("TypeOf")}, Args: []ast.Expr{mIdent}},
		Sel: ast.NewIdent("MapType"),
	}}

	abiPkg := types.NewPackage("internal/abi", "abi")
	currentPkg := types.NewPackage("internal/sync", "sync")
	mapType := types.NewMap(types.Typ[types.String], types.Typ[types.Int])
	swissMapType := types.NewNamed(types.NewTypeName(token.NoPos, abiPkg, "SwissMapType", nil), types.NewStruct(nil, nil), nil)
	currentTypeInfo = &TypeInfo{
		info: &types.Info{
			Uses: map[*ast.Ident]types.Object{
				abiIdent: types.NewPkgName(token.NoPos, currentPkg, "abi", abiPkg),
				mIdent:   types.NewVar(token.NoPos, currentPkg, "m", mapType),
			},
			Types: map[ast.Expr]types.TypeAndValue{
				mIdent: {Type: mapType},
			},
		},
		pkg: currentPkg,
	}
	SetTranspileContext(&TranspileContext{PackageMapping: map[string]string{"internal/abi": "internal_abi"}})

	var out strings.Builder
	info := goPtrResultInfo{elemRustType: "internal_abi::map_swiss::SwissMapType", elemType: swissMapType}
	if !writeGoPtrCallArgumentWithQualifierForInfo(&out, call, info, "") {
		t.Fatalf("internal/abi.TypeOf(map).MapType should lower as a GoPtr-compatible value")
	}
	got := out.String()

	for _, forbidden := range []string{"match __go_ptr", "internal_abi::GoPtr::Nil", "internal_abi::GoPtr::Local"} {
		if strings.Contains(got, forbidden) {
			t.Fatalf("typed map intrinsic should not convert an ordinary pointer handle as a GoPtr via %q:\n%s", forbidden, got)
		}
	}
	for _, want := range []string{
		"GoPtr::local(Arc::new(Mutex::new(Some({ let mut __type = internal_abi::Type::default()",
		"internal_abi::SwissMapType::default()",
		"GoComparable::go_hash(__key_value, __seed_value)",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestTypedNilChannelAnyVarInitializerBoxesTypedNilChannel(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func boxChan() any {
	var x any = (chan int)(nil)
	return x
}
`)

	if strings.Contains(rust, "let __v = None") || strings.Contains(rust, "Box::new(None)") {
		t.Fatalf("typed nil channel in any initializer should not emit untyped None:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(GoChannel::<i32>::default()) as Box<dyn Any") {
		t.Fatalf("typed nil channel in any initializer should preserve the channel type inside Any:\n%s", rust)
	}
}

func TestTypedNilFunctionAnyVarInitializerBoxesTypedNone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func boxFunc() any {
	var x any = (func())(nil)
	return x
}
`)

	if strings.Contains(rust, "let __v = None") || strings.Contains(rust, "Box::new(None)") {
		t.Fatalf("typed nil function in any initializer should not emit untyped None:\n%s", rust)
	}
	if !strings.Contains(rust, "None::<Box<dyn FnMut() -> ()>") || !strings.Contains(rust, "as Box<dyn Any") {
		t.Fatalf("typed nil function in any initializer should preserve the function type inside Any:\n%s", rust)
	}
}

func TestExplicitGenericFunctionCallUsesRustTypeArgs(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func identity[T any](value T) T {
	return value
}

func use() int {
	return identity[int](3)
}
`)

	if strings.Contains(rust, "identity.clone()") || strings.Contains(rust, "int.borrow") || strings.Contains(rust, "int.lock") {
		t.Fatalf("explicit generic function call should not lower the type argument as an index expression:\n%s", rust)
	}
	if !strings.Contains(rust, "identity::<i32>(") {
		t.Fatalf("explicit generic function call should emit Rust type arguments:\n%s", rust)
	}
}

func TestMakeZeroLengthSliceIncludesElementType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func makeRunes() []rune {
	return make([]rune, 0)
}
`)

	if !strings.Contains(rust, "Vec::<i32>::with_capacity(0)") {
		t.Fatalf("make([]rune, 0) should emit a typed empty Vec:\n%s", rust)
	}
}

func TestMakeZeroLengthInterfaceSliceUsesWrappedElementType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	Name() string
}

type named struct{}

func (named) Name() string { return "" }

func makeTypes(n int) []Type {
	values := make([]Type, 0, n)
	values = append(values, named{})
	return values
}
`)

	if !(strings.Contains(rust, "Vec::<Rc<RefCell<Option<Box<dyn Type") && strings.Contains(rust, ">::with_capacity")) &&
		!(strings.Contains(rust, "Vec::<Arc<Mutex<Option<Box<dyn Type + Send + Sync") && strings.Contains(rust, ">::with_capacity")) {
		t.Fatalf("make([]Type, 0, n) should emit the wrapped interface slice element type:\n%s", rust)
	}
}

func TestThreeIndexSliceCapacityUnwrapsSelectorBounds(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Header struct {
	Count uint16
}

func take(values []int, h *Header) []int {
	return values[:h.Count:h.Count]
}
`)

	if strings.Contains(rust, "h.count.clone()) - 0") {
		t.Fatalf("three-index slice capacity should unwrap selector bounds before arithmetic:\n%s", rust)
	}
	if !strings.Contains(rust, "let __low = 0;") ||
		!strings.Contains(rust, "let __max = ((*") ||
		!strings.Contains(rust, " as usize;") ||
		!strings.Contains(rust, "Vec::with_capacity((__max - __low) as usize)") {
		t.Fatalf("three-index slice capacity should use usize bounds:\n%s", rust)
	}
}

func TestTwoIndexSliceReslicePreservesSourceCapacity(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func grow() []byte {
	s := make([]byte, 2, 4)
	s = s[:3]
	return s[1:3]
}
`)

	if !strings.Contains(rust, "let __source_cap = ") {
		t.Fatalf("two-index slice should capture source capacity before cloning:\n%s", rust)
	}
	if !strings.Contains(rust, "if __seq.len() < __high { __seq.resize_with(__high, Default::default); }") {
		t.Fatalf("two-index slice should materialize zero values when reslicing within capacity:\n%s", rust)
	}
	if !strings.Contains(rust, "Vec::with_capacity((__max - __low) as usize)") {
		t.Fatalf("two-index slice should preserve Go cap(source)-low capacity:\n%s", rust)
	}
}

func TestArraySliceDoesNotUseVecCapacityHelpers(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func cut() []byte {
	var digits [65]byte
	return digits[1:3]
}
`)

	if strings.Contains(rust, ".capacity()") || strings.Contains(rust, "resize_with") {
		t.Fatalf("array slicing should use array length, not Vec capacity helpers:\n%s", rust)
	}
	if !strings.Contains(rust, "let __max = __source_cap;") ||
		!strings.Contains(rust, "Vec::with_capacity((__max - __low) as usize)") {
		t.Fatalf("array slicing should preserve Go len(array)-low capacity:\n%s", rust)
	}
}

func TestUintptrConversionFromUnsafePointerIdentifierUsesHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

func addr(p unsafe.Pointer) uintptr {
	return uintptr(p)
}
`)

	if strings.Contains(rust, "__v }.borrow()") || strings.Contains(rust, "__v }.lock()") {
		t.Fatalf("uintptr(unsafe.Pointer identifier) should not borrow after unwrapping to a raw integer:\n%s", rust)
	}
	if !strings.Contains(rust, "(*p.borrow().as_ref().unwrap()) as usize") {
		t.Fatalf("uintptr(unsafe.Pointer identifier) should borrow the pointer handle directly:\n%s", rust)
	}
}

func TestUnsafeRuntimeIntrinsicsEmitTypedUnimplemented(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "unsafe"

func unsafeValues(p *byte, n int, b []byte, s string) (string, []byte, unsafe.Pointer, unsafe.Pointer, unsafe.Pointer) {
	return unsafe.String(p, n), unsafe.Slice(p, n), unsafe.Add(unsafe.Pointer(p), n), unsafe.Pointer(unsafe.SliceData(b)), unsafe.Pointer(unsafe.StringData(s))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)

	if strings.Contains(rust, "unsafe::") {
		t.Fatalf("unsafe compiler intrinsics should not be emitted as Rust module calls:\n%s", rust)
	}
	for _, name := range []string{"unsafe.String", "unsafe.Slice", "unsafe.Add", "unsafe.StringData"} {
		if !strings.Contains(rust, `unimplemented!("`+name+` requires unsafe intrinsic support")`) {
			t.Fatalf("%s should emit a loud unsupported intrinsic marker:\n%s", name, rust)
		}
	}
	if strings.Contains(rust, `unimplemented!("unsafe.SliceData requires unsafe intrinsic support")`) {
		t.Fatalf("unsafe.Pointer(unsafe.SliceData(slice)) should lower through the typed slice-data pointer path:\n%s", rust)
	}
}

func TestUnsupportedUnsafePointerIntrinsicKeepsPointerHandleShape(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "unsafe"

func sink(p *byte) {}

func call(s string) {
	go func() {}()
	sink(unsafe.StringData(s))
}
`)

	if strings.Contains(rust, "Some({ let __go_unsafe_result: Arc<Mutex<Option<u8>>>") {
		t.Fatalf("unsafe.StringData pointer result should not be wrapped inside another Option slot:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_unsafe_result: Arc<Mutex<Option<u8>>> = unimplemented!(\"unsafe.StringData requires unsafe intrinsic support\")") {
		t.Fatalf("unsafe.StringData should keep the typed pointer handle shape:\n%s", rust)
	}
}

func TestUnsafeStringFromByteSliceAddressUsesSliceBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

func text(b []byte) string {
	return unsafe.String(&b[0], len(b))
}
`)

	if strings.Contains(rust, `unimplemented!("unsafe.String requires unsafe intrinsic support")`) {
		t.Fatalf("unsafe.String(&bytes[0], len(bytes)) should lower through the typed byte-slice path:\n%s", rust)
	}
	if !strings.Contains(rust, `let __bytes_holder = b.clone()`) {
		t.Fatalf("unsafe.String byte-slice path should borrow the slice handle:\n%s", rust)
	}
	if strings.Contains(rust, `let __len = ((*b.borrow()`) ||
		strings.Contains(rust, `let __len = ((*b.lock()`) {
		t.Fatalf("unsafe.String byte-slice path should not relock the borrowed slice for len(b):\n%s", rust)
	}
	if !strings.Contains(rust, `let __len = __bytes.len()`) {
		t.Fatalf("unsafe.String byte-slice path should reuse the borrowed slice length:\n%s", rust)
	}
	if !strings.Contains(rust, `String::from_utf8(__bytes[__start..__end].to_vec()).unwrap()`) {
		t.Fatalf("unsafe.String byte-slice path should build a String from the selected bytes:\n%s", rust)
	}
}

func TestUnsafePointerSliceDataConversionUsesSliceDataPointer(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "unsafe"

func ptr(b []byte) uintptr {
	if len(b) == 0 {
		return 0
	}
	return uintptr(unsafe.Pointer(unsafe.SliceData(b)))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "unsafe.SliceData requires unsafe intrinsic support") {
		t.Fatalf("unsafe.Pointer(unsafe.SliceData(slice)) should not use unsupported fallback:\n%s", rust)
	}
	if !strings.Contains(rust, "__v.as_mut_ptr() as usize") {
		t.Fatalf("unsafe.Pointer(unsafe.SliceData(slice)) should use the slice data pointer:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut __slice_guard = __slice_holder") {
		t.Fatalf("unsafe.Pointer(unsafe.SliceData(slice)) should borrow the slice mutably before as_mut_ptr:\n%s", rust)
	}
}

func TestNoTypeInfoFunctionFieldDoesNotSynthesizeBoxFromRegistry(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "fmt"

type BinaryOp func(int, int) int

type Calculator struct {
	Multiply BinaryOp
}

func multiply(a, b int) int { return a * b }

func main() {
	calc := Calculator{Multiply: multiply}
	fmt.Println(calc.Multiply(3, 4))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	// AGENTS.md "Type Info Is Authoritative": with typeInfo=nil, the
	// transpiler must not synthesize a Box::new(move |...|) closure for
	// `multiply` from the FunctionSignature registry. The previous
	// writeFunctionValueBoxFromSyntax helper was added in commit 50ecb15d
	// (May 2026) inside the 470fcb0b..3e3d9fc3 fallback-incident range
	// and is gone. The function-call lowering of calc.Multiply(...) is
	// unrelated to this bucket and may still go through its own paths.
	if strings.Contains(rust, "Box::new(move |__arg0: ") {
		t.Fatalf("Mode 1 must not synthesize Box::new closure for function value from FunctionSignature registry:\n%s", rust)
	}
}

func TestNoTypeInfoFunctionFieldCallInPrintfUsesSyntax(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "fmt"

type BinaryOp func(int, int) int

type Calculator struct {
	Add BinaryOp
}

func main() {
	calc := Calculator{Add: func(a, b int) int { return a + b }}
	fmt.Printf("%d\n", calc.Add(1, 2))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, ".add(Rc::new") {
		t.Fatalf("Printf function field call should not be lowered as a method call:\n%s", rust)
	}
	if !strings.Contains(rust, "let __f_holder = (*calc.borrow().as_ref().unwrap()).add.clone()") {
		t.Fatalf("Printf function field call should invoke the field handle:\n%s", rust)
	}
}

func TestMethodExpressionFunctionArgumentUsesClosure(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "slices"

type Node struct {
	Subs []*Node
}

func (n *Node) Equal(other *Node) bool {
	return n == other
}

func same(a, b []*Node) bool {
	return slices.EqualFunc(a, b, (*Node).Equal)
}
`)

	if strings.Contains(rust, "NODE") {
		t.Fatalf("method expression should not be lowered through an uppercase type value:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(move |__arg0:") || !strings.Contains(rust, ".equal(__arg1)") {
		t.Fatalf("method expression should lower to a function-value closure:\n%s", rust)
	}
}

func TestConcurrentMethodExpressionReceiverUsesPointerTemp(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "slices"

type Node struct {
	Subs []*Node
}

func (n *Node) Equal(other *Node) bool {
	return n == other
}

func same(a, b []*Node) bool {
	return slices.EqualFunc(a, b, (*Node).Equal)
}

func main() {
	go func() {}()
}
`)

	if strings.Contains(rust, "(*__recv.lock().unwrap().as_ref().unwrap()).equal") {
		t.Fatalf("concurrent method expression receiver should not borrow through the method call:\n%s", rust)
	}
	if !strings.Contains(rust, "let __recv_ptr: *const Node") ||
		!strings.Contains(rust, "unsafe { &*__recv_ptr }.equal") {
		t.Fatalf("concurrent method expression receiver should use a pointer temp:\n%s", rust)
	}
}

func TestTypeParamSliceSelectorArgumentClonesFieldHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type re struct {
	Rune []rune
}

func (x *re) Equal(y *re) bool {
	_ = x.Rune
	_ = y.Rune
	return true
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	var receiverSelector *ast.SelectorExpr
	ast.Inspect(file, func(n ast.Node) bool {
		sel, ok := n.(*ast.SelectorExpr)
		if !ok || sel.Sel.Name != "Rune" {
			return true
		}
		if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "x" {
			receiverSelector = sel
			return false
		}
		return true
	})
	if receiverSelector == nil {
		t.Fatal("did not find x.Rune selector")
	}

	prevTypeInfo := currentTypeInfo
	prevReceiver := currentReceiver
	prevReceiverType := currentReceiverType
	defer func() {
		currentTypeInfo = prevTypeInfo
		currentReceiver = prevReceiver
		currentReceiverType = prevReceiverType
	}()
	SetTypeInfo(typeInfo)
	currentReceiver = "x"
	currentReceiverType = "re"

	var out strings.Builder
	writeConcreteSliceAsTypeParamSliceArgument(&out, receiverSelector)
	got := out.String()
	if strings.Contains(got, "let __slice_holder = self.rune;") {
		t.Fatalf("type-param slice selector argument should clone the field handle before borrowing:\n%s", got)
	}
	if !strings.Contains(got, "self.rune.clone()") {
		t.Fatalf("type-param slice selector argument should clone receiver field handle:\n%s", got)
	}
}

func TestMutatingTypeParamSliceCallWritesConcreteSliceBack(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func Mutate[S ~[]E, E any](x S, cmp func(a, b E) int) {
	x[0], x[1] = x[1], x[0]
}

func compare(a, b string) int {
	return 0
}

func sort(list []string) {
	Mutate(list, compare)
}
`)

	if strings.Contains(rust, "mutate::<Vec<String>, String>(list.clone()") {
		t.Fatalf("mutating type-param slice call should not pass the concrete slice directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let __slice_holder_0 = list.clone()") {
		t.Fatalf("mutating type-param slice call should keep the original slice handle:\n%s", rust)
	}
	if !strings.Contains(rust, "mutate::<Vec<String>, String>(__slice_arg_0.clone()") {
		t.Fatalf("mutating type-param slice call should pass the converted element handles:\n%s", rust)
	}
	if !strings.Contains(rust, "*__slice_holder_0.borrow_mut() = __converted_values_0") {
		t.Fatalf("mutating type-param slice call should copy converted elements back:\n%s", rust)
	}
}

func TestMutatingOrderedTypeParamSliceCallKeepsRawSlice(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Ordered interface {
	~string
}

func Mutate[S ~[]E, E Ordered](x S) {
	x[0] = x[0]
}

func sort(list []string) {
	Mutate(list)
}
`)

	if strings.Contains(rust, "__slice_arg_0") {
		t.Fatalf("mutating ordered type-param slice call should not convert raw elements:\n%s", rust)
	}
	if !strings.Contains(rust, "mutate::<Vec<String>, String>(list.clone())") {
		t.Fatalf("mutating ordered type-param slice call should pass the raw slice handle:\n%s", rust)
	}
}

func TestNoTypeInfoImmediateFuncLitCallUsesClosureType(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func main() {
	result := func(a, b int) int { return a + b }(10, 20)
	_ = result
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, "*mut _") {
		t.Fatalf("immediate function literal call should use a concrete closure type:\n%s", rust)
	}
	if !strings.Contains(rust, "*mut Box<dyn FnMut") {
		t.Fatalf("immediate function literal call should emit a concrete function box type:\n%s", rust)
	}
}

func TestLocalStructAliasFuncLiteralUsesBareParamsReturnsAndArgs(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func wrap(words []int) int64 {
	type score struct {
		hi int64
		lo int64
	}
	add := func(s, t score) score {
		return score{s.hi + t.hi, s.lo + t.lo}
	}
	f := []score{{0, 0}}
	g := func(i int) score {
		return add(f[i], score{1, 2})
	}
	return g(0).hi
}
`)

	if strings.Contains(rust, "s.borrow()") || strings.Contains(rust, "s.lock()") {
		t.Fatalf("local struct alias closure params should be bare struct values:\n%s", rust)
	}
	if strings.Contains(rust, "Rc::new(RefCell::new(Some(score") ||
		strings.Contains(rust, "Arc::new(Mutex::new(Some(score") {
		t.Fatalf("local struct alias closure returns should not wrap bare score values:\n%s", rust)
	}
	if strings.Contains(rust, "Rc::new(RefCell::new(Some({ let __seq") ||
		strings.Contains(rust, "Arc::new(Mutex::new(Some({ let __seq") {
		t.Fatalf("local struct alias function arguments should pass indexed score values bare:\n%s", rust)
	}
}

func TestAppendLocalStructAliasCallResultUsesBareValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func wrap() int64 {
	type score struct {
		hi int64
		lo int64
	}
	f := []score{{0, 0}}
	g := func(i int) score {
		return score{int64(i), 0}
	}
	f = append(f, g(1))
	return f[1].hi
}
`)

	if strings.Contains(rust, ".borrow().as_ref().unwrap()).clone())") ||
		strings.Contains(rust, ".lock().unwrap().as_ref().unwrap()).clone())") {
		t.Fatalf("append of local struct alias call result should push the bare value:\n%s", rust)
	}
}

func TestNamedStructAliasReturnKeepsWrappedAliasHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type inner struct {
	n int
}

type alias = inner

func makeAlias() (alias, string) {
	v := alias{n: 1}
	return v, ""
}
`)

	if strings.Contains(rust, "return v,") {
		t.Fatalf("named struct alias return should not treat the alias as a bare anonymous struct:\n%s", rust)
	}
	if !strings.Contains(rust, "v.clone()") && !strings.Contains(rust, "__owned") {
		t.Fatalf("named struct alias return should keep a wrapped handle/value copy:\n%s", rust)
	}
}

func TestFindStructFieldExprUsesRustCasedFallback(t *testing.T) {
	structType := &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
		{
			Names: []*ast.Ident{ast.NewIdent("Add")},
			Type:  ast.NewIdent("BinaryOp"),
		},
	}}}

	if got := findStructFieldExpr(structType, "add"); got == nil {
		t.Fatal("expected Rust-cased selector name to resolve to Add field")
	}
}

func TestFunctionValueSelectorSyntaxUsesUniqueStructFieldFallback(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevVarTable := currentVarTable
	prevRenames := currentCaptureRenames
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetVarTable(prevVarTable)
		currentCaptureRenames = prevRenames
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	currentCaptureRenames = nil
	functionTypeAliases = make(map[string]bool)
	functionTypeAliasBoxTypes = map[string]string{
		"BinaryOp": "Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>",
	}
	structDefs = map[string]*StructDef{
		"Calculator": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
				{
					Names: []*ast.Ident{ast.NewIdent("Add")},
					Type:  ast.NewIdent("BinaryOp"),
				},
			}}},
		},
	}

	call := &ast.CallExpr{
		Fun: &ast.SelectorExpr{
			X:   ast.NewIdent("calc"),
			Sel: ast.NewIdent("add"),
		},
		Args: []ast.Expr{
			&ast.BasicLit{Kind: token.INT, Value: "1"},
			&ast.BasicLit{Kind: token.INT, Value: "2"},
		},
	}

	var out strings.Builder
	TranspileExpression(&out, call)
	got := out.String()
	if strings.Contains(got, ".add(Rc::new") {
		t.Fatalf("function field selector fallback should not emit a method call:\n%s", got)
	}
	if !strings.Contains(got, "let __f_holder = (*calc.borrow().as_ref().unwrap()).add.clone()") {
		t.Fatalf("function field selector fallback should invoke the field handle:\n%s", got)
	}
}

func TestFunctionValueSelectorSyntaxDoesNotFallbackForClosureCloneMethod(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevVarTable := currentVarTable
	prevRenames := currentCaptureRenames
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetVarTable(prevVarTable)
		currentCaptureRenames = prevRenames
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	currentCaptureRenames = map[string]string{"ld": "ld_closure_clone"}
	functionTypeAliases = make(map[string]bool)
	functionTypeAliasBoxTypes = make(map[string]string)
	RegisterFunctionTypeAlias("ParseFunc")
	RegisterFunctionTypeAliasBox("ParseFunc", "Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<i32>>>>")
	structDefs = map[string]*StructDef{
		"Config": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
				{
					Names: []*ast.Ident{ast.NewIdent("ParseFile")},
					Type:  ast.NewIdent("ParseFunc"),
				},
			}}},
		},
	}

	call := &ast.CallExpr{
		Fun: &ast.SelectorExpr{
			X:   ast.NewIdent("ld"),
			Sel: ast.NewIdent("parse_file"),
		},
		Args: []ast.Expr{&ast.BasicLit{Kind: token.STRING, Value: `"x.go"`}},
	}

	var out strings.Builder
	TranspileExpression(&out, call)
	got := out.String()
	if strings.Contains(got, "let __f_holder = ld_closure_clone.parse_file.clone()") {
		t.Fatalf("closure clone method call should not use unrelated function-field fallback:\n%s", got)
	}
	if strings.Contains(got, "let __f_holder =") {
		t.Fatalf("captured receiver method call should not use function-field fallback:\n%s", got)
	}
}

func TestFunctionValueSelectorSyntaxDoesNotOverrideTypedMethod(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevInterfaces := interfaceTypes
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		interfaceTypes = prevInterfaces
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetVarTable(prevVarTable)
	}()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Key interface {
	Name() string
}

type Label struct {
	key Key
}

func (t Label) Key() Key { return t.key }

func use(t Label) {
	_ = t.Key()
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "let __f_holder = t.key.clone()") || strings.Contains(rust, "*mut Key") {
		t.Fatalf("typed method call should not use function-field syntax fallback:\n%s", rust)
	}
}

func TestFunctionValueSelectorSyntaxDoesNotOverrideTypedMethodNameCollision(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type FileInfo interface {
	Name() string
}

type DirEntry interface {
	Name() string
	Type() int
}

func fileInfoToDirEntry(info FileInfo) DirEntry {
	return nil
}

type Context struct {
	ReadDir func(string) ([]FileInfo, error)
}

func (c *Context) readDir(path string) ([]DirEntry, error) {
	if f := c.ReadDir; f != nil {
		fis, err := f(path)
		if err != nil {
			return nil, err
		}
		des := make([]DirEntry, len(fis))
		for i, fi := range fis {
			des[i] = fileInfoToDirEntry(fi)
		}
		return des, nil
	}
	return nil, nil
}

func (c *Context) scan() {
	dirs, err := c.readDir("")
	_ = err
	for _, d := range dirs {
		_ = d.Type()
	}
}
`)

	if strings.Contains(rust, "let __f_holder = self.read_dir.clone()") {
		t.Fatalf("typed method call should not use same-Rust-name function field:\n%s", rust)
	}
	if !strings.Contains(rust, "self.read_dir(Rc::new") && !strings.Contains(rust, "self.read_dir(Arc::new") {
		t.Fatalf("typed method call should lower through the method receiver:\n%s", rust)
	}
}

func TestFunctionValueSelectorSyntaxDoesNotUseUnrelatedFieldForKnownReceiver(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetVarTable(prevVarTable)
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	GetVarTable().Register("r", &VarInfo{
		WrapLevel: WrapFull,
		RustType:  "Decoder",
		Source:    SourceLocal,
	})
	functionTypeAliases = map[string]bool{"Hook": true}
	functionTypeAliasBoxTypes = map[string]string{
		"Hook": "Box<dyn FnMut(Rc<RefCell<Option<i32>>>)>",
	}
	structDefs = map[string]*StructDef{
		"PkgDecoder": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
				{
					Names: []*ast.Ident{ast.NewIdent("sync")},
					Type:  ast.NewIdent("Hook"),
				},
			}}},
		},
		"Decoder": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{}},
		},
	}

	sel := &ast.SelectorExpr{
		X:   ast.NewIdent("r"),
		Sel: ast.NewIdent("Sync"),
	}
	if isFunctionValueSelectorSyntax(sel) {
		t.Fatal("known Decoder receiver must not use an unrelated PkgDecoder.sync function field")
	}
}

func TestSelectorFieldTypeExprUsesRegisteredFieldTypeMap(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		SetVarTable(prevVarTable)
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	GetVarTable().Register("calc", &VarInfo{
		WrapLevel: WrapFull,
		RustType:  "Calculator",
		Source:    SourceLocal,
	})
	structDefs = map[string]*StructDef{
		"Calculator": {
			Fields: map[string]string{"Add": "regular"},
			FieldTypes: map[string]ast.Expr{
				"Add": ast.NewIdent("BinaryOp"),
			},
			ASTType: &ast.StructType{Fields: &ast.FieldList{}},
		},
	}

	fieldExpr, ok := selectorFieldTypeExpr(&ast.SelectorExpr{
		X:   ast.NewIdent("calc"),
		Sel: ast.NewIdent("add"),
	})
	if !ok {
		t.Fatal("selector field type should use registered FieldTypes when AST lookup has no names")
	}
	ident, ok := fieldExpr.(*ast.Ident)
	if !ok || ident.Name != "BinaryOp" {
		t.Fatalf("field type = %#v, want BinaryOp ident", fieldExpr)
	}
}

func TestFunctionValueSelectorSyntaxAllowsPartialTypeInfoFallback(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
	}()

	sel := &ast.SelectorExpr{
		X:   ast.NewIdent("calc"),
		Sel: ast.NewIdent("add"),
	}
	SetTypeInfo(&TypeInfo{info: &types.Info{
		Uses: map[*ast.Ident]types.Object{
			sel.Sel: types.NewFunc(token.NoPos, nil, "Add", nil),
		},
	}})
	functionTypeAliases = make(map[string]bool)
	functionTypeAliasBoxTypes = map[string]string{
		"BinaryOp": "Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>",
	}
	structDefs = map[string]*StructDef{
		"Calculator": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
				{
					Names: []*ast.Ident{ast.NewIdent("Add")},
					Type:  ast.NewIdent("BinaryOp"),
				},
			}}},
		},
	}

	if !isFunctionValueSelectorSyntax(sel) {
		t.Fatal("partial selector object info without selector type should not block syntax fallback")
	}
}

func TestFunctionValueSelectorSyntaxUsesUniqueFieldWhenTypedObjectIsMisclassified(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetVarTable(prevVarTable)
	}()

	sel := &ast.SelectorExpr{
		X:   ast.NewIdent("calc"),
		Sel: ast.NewIdent("add"),
	}
	SetTypeInfo(&TypeInfo{info: &types.Info{
		Types: map[ast.Expr]types.TypeAndValue{
			sel: {Type: types.Typ[types.Int]},
		},
		Uses: map[*ast.Ident]types.Object{
			sel.Sel: types.NewFunc(token.NoPos, nil, "Add", nil),
		},
	}})
	SetVarTable(NewVarTable())
	functionTypeAliases = map[string]bool{"BinaryOp": true}
	functionTypeAliasBoxTypes = map[string]string{
		"BinaryOp": "Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>",
	}
	structDefs = map[string]*StructDef{
		"Calculator": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
				{
					Names: []*ast.Ident{ast.NewIdent("Add")},
					Type:  ast.NewIdent("BinaryOp"),
				},
			}}},
		},
	}

	if !isFunctionValueSelectorSyntax(sel) {
		t.Fatal("syntax-proven function field should survive a misclassified selector object")
	}
}

func TestFunctionFieldCallUsesBoxWhenFuncAliasIsAlsoTypeDefinition(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevTypeDefs := typeDefinitions
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		typeDefinitions = prevTypeDefs
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetVarTable(prevVarTable)
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	typeDefinitions = map[string]string{"BinaryOp": "func"}
	functionTypeAliases = make(map[string]bool)
	functionTypeAliasBoxTypes = map[string]string{
		"BinaryOp": "Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>",
	}
	structDefs = map[string]*StructDef{
		"Calculator": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
				{
					Names: []*ast.Ident{ast.NewIdent("Add")},
					Type:  ast.NewIdent("BinaryOp"),
				},
			}}},
		},
	}
	GetVarTable().Register("calc", &VarInfo{
		WrapLevel: WrapFull,
		RustType:  "Calculator",
		Source:    SourceLocal,
	})

	call := &ast.CallExpr{
		Fun: &ast.SelectorExpr{
			X:   ast.NewIdent("calc"),
			Sel: ast.NewIdent("add"),
		},
		Args: []ast.Expr{
			&ast.BasicLit{Kind: token.INT, Value: "1"},
			&ast.BasicLit{Kind: token.INT, Value: "2"},
		},
	}

	var out strings.Builder
	TranspileExpression(&out, call)
	got := out.String()
	if strings.Contains(got, ".add(Rc::new") {
		t.Fatalf("function field call should not be lowered as a method call:\n%s", got)
	}
	if !strings.Contains(got, "let __f_holder = (*calc.borrow().as_ref().unwrap()).add.clone()") {
		t.Fatalf("function field call should invoke the field handle:\n%s", got)
	}
	if !strings.Contains(got, "*mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>") {
		t.Fatalf("function field call should use the stored function box type:\n%s", got)
	}
}

func TestFunctionMapValueWithoutTypeInfoDoesNotSynthesizeBoxFromRegistry(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevAliases := functionTypeAliases
	prevSignatures := functionSignatures
	defer func() {
		currentTypeInfo = prevTypeInfo
		functionTypeAliases = prevAliases
		functionSignatures = prevSignatures
	}()

	SetTypeInfo(nil)
	functionTypeAliases = map[string]bool{"handler": true}
	functionSignatures = map[string]*FunctionSignature{
		"inc": {
			Params: []*ast.Field{
				{Type: ast.NewIdent("int")},
			},
			Results: []*ast.Field{
				{Type: ast.NewIdent("int")},
			},
		},
	}

	var out strings.Builder
	writeWrappedMapValue(&out, ast.NewIdent("inc"), ast.NewIdent("handler"), nil)

	got := out.String()
	// AGENTS.md "Type Info Is Authoritative": with typeInfo=nil and only an
	// AST-derived FunctionSignature in the registry, the transpiler must
	// not synthesize a Box::new closure for the map value. The previous
	// writeFunctionValueBoxFromSyntax helper routed through GetFunctionSignature
	// to produce `Box::new(move |__arg0: ...| { inc(__arg0) })` and the
	// matching `as Box<dyn FnMut(...) -> ...>` cast; that branch was added
	// in commit 50ecb15d inside the 470fcb0b..3e3d9fc3 fallback-incident
	// range and is gone.
	if strings.Contains(got, "Box::new(move |__arg0:") {
		t.Fatalf("Mode 1 map value must not synthesize Box::new closure from FunctionSignature registry:\n%s", got)
	}
	if strings.Contains(got, "as Box<dyn FnMut(") {
		t.Fatalf("Mode 1 map value must not synthesize FnMut box-type cast from AST alias:\n%s", got)
	}
}

func TestSyncOnceDoPromotedFieldMethodValueUsesTypedReceiverAndClosure(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type outer struct {
	*inner
}

type inner struct {
	once sync.Once
	n int
}

func (o *outer) init() {
	o.once.Do(o.mark)
}

func (o *outer) mark() {
	o.n++
}
`)

	if strings.Contains(rust, "self.once.r#do") {
		t.Fatalf("promoted sync.Once field should not be emitted as a direct field on the outer receiver:\n%s", rust)
	}
	if strings.Contains(rust, "self.mark.clone()") {
		t.Fatalf("method value argument to sync.Once.Do should lower to a callable closure, not a field clone:\n%s", rust)
	}
	if !strings.Contains(rust, "let __once =") || !strings.Contains(rust, "__once.r#do(") {
		t.Fatalf("sync.Once.Do should clone the typed receiver before invoking Do:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(move |") || !strings.Contains(rust, ".mark()") {
		t.Fatalf("method value argument should lower to a boxed method-value closure:\n%s", rust)
	}
}

func TestPackageGlobalSyncOnceDoUnwrapsGlobal(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

var once sync.Once
var count int

func initOnce() {
	once.Do(func() {
		count++
	})
}
`)

	if strings.Contains(rust, "let __once = once.clone(); __once.r#do") {
		t.Fatalf("package global sync.Once should not call Do on the wrapper handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __once = (*once.borrow().as_ref().unwrap()).clone(); __once.r#do") {
		t.Fatalf("package global sync.Once should clone the stored GoOnce value before Do:\n%s", rust)
	}
}

func TestStructLiteralFunctionFieldUsesMethodValueClosure(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Parser struct {
	LookupPackage func(string) (string, bool)
	LookupSym     func(string, string) bool
}

type Package struct {
	Name string
}

func (p *Package) lookupPackage(name string) (string, bool) {
	return p.Name, true
}

func (p *Package) lookupSym(recv, name string) bool {
	return recv == "" && name == p.Name
}

func (p *Package) Parser() *Parser {
	return &Parser{
		LookupPackage: p.lookupPackage,
		LookupSym:     p.lookupSym,
	}
}
`)

	if strings.Contains(rust, "lookup_package: Rc::new(RefCell::new(Some(self.lookup_package.clone())))") ||
		strings.Contains(rust, "lookup_package: Arc::new(Mutex::new(Some(self.lookup_package.clone())))") {
		t.Fatalf("method value field should not be lowered as a selector field clone:\n%s", rust)
	}
	if !strings.Contains(rust, "lookup_package: Rc::new(RefCell::new(Some({ let mut __recv = self.clone(); Box::new(move |") &&
		!strings.Contains(rust, "lookup_package: Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |") {
		t.Fatalf("method value field should lower to a boxed receiver closure:\n%s", rust)
	}
}

func TestSyncOnceDoPromotedAnonymousStructMethodUsesEmbeddedOnce(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "sync"

var holder struct {
	sync.Once
	n int
}

func initHolder() {
	holder.Do(func() {
		holder.n++
	})
}
`)

	if strings.Contains(rust, ".as_mut().unwrap()).r#do") {
		t.Fatalf("promoted sync.Once.Do should not call Do on the outer anonymous struct:\n%s", rust)
	}
	if !strings.Contains(rust, ".once.clone(); __once.r#do") {
		t.Fatalf("promoted sync.Once.Do should invoke the embedded once field:\n%s", rust)
	}
}

func TestSliceElemPtrPromotedLocalEmbeddedMethodUsesEmbeddedField(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type reader struct {
	n int
}

func (r *reader) peek() int {
	r.n++
	return r.n
}

func run() int {
	type state struct {
		reader
		next int
	}
	values := []state{{}}
	s := &values[0]
	s.next = s.peek()
	return s.next
}
`)

	if strings.Contains(rust, ".as_mut().unwrap()).peek()") {
		t.Fatalf("promoted method on slice-element pointer should not call the outer struct:\n%s", rust)
	}
	if !strings.Contains(rust, ".reader.clone()") || !strings.Contains(rust, "__promoted_ref.peek()") {
		t.Fatalf("promoted method on slice-element pointer should call through the embedded field:\n%s", rust)
	}
}

func TestSourceMappedSyncOnceDoFuncLitWrapsSourceOnceArgument(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "internal/godebug", `package godebug

import "sync"

type Setting struct {
	once sync.Once
	n int
}

func (s *Setting) Value() {
	go func() {}()
	s.once.Do(func() {
		s.n++
	})
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, "__once.r#do(||") {
		t.Fatalf("source-mapped sync.Once.Do should not pass a raw closure to source sync::Once:\n%s", rust)
	}
	if !strings.Contains(rust, "__once.r#do(Arc::new(Mutex::new(Some(") ||
		!strings.Contains(rust, "Box::new(move ||") {
		t.Fatalf("source-mapped sync.Once.Do should wrap function literal arguments for source sync::Once:\n%s", rust)
	}
}

func TestSourceMappedSyncOnceDoMethodValueWrapsSourceOnceArgument(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "internal/godebug", `package godebug

import "sync"

type Setting struct {
	nonDefaultOnce sync.Once
	n int
}

func (s *Setting) IncNonDefault() {
	go func() {}()
	s.nonDefaultOnce.Do(s.register)
}

func (s *Setting) register() {
	s.n++
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, "__once.r#do({ let mut __recv = self.clone(); Box::new") {
		t.Fatalf("source-mapped sync.Once.Do should not pass a raw method-value closure to source sync::Once:\n%s", rust)
	}
	if !strings.Contains(rust, "__once.r#do(Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new") {
		t.Fatalf("source-mapped sync.Once.Do should wrap method-value arguments for source sync::Once:\n%s", rust)
	}
}

func TestSourceMappedPackageLocalSyncOnceDoUsesHelperCallable(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "golang.org/x/tools/internal/gcimporter", `package gcimporter

import "sync"

func lookup(run func()) func() {
	var listOnce sync.Once
	return func() {
		listOnce.Do(func() {
			run()
		})
	}
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, "__once.r#do(Arc::new(Mutex::new(Some(") {
		t.Fatalf("local sync.Once variables lower to GoOnce and should pass a callable, not a wrapped func() handle:\n%s", rust)
	}
	if !strings.Contains(rust, "__once.r#do(||") {
		t.Fatalf("local sync.Once variables should pass a callable to GoOnce::do:\n%s", rust)
	}
}

func TestSourceMappedPromotedSyncOnceDoWrapsSourceOnceArgument(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "math/big", `package big

import "sync"

var threeOnce struct {
	sync.Once
	n int
}

func three() {
	threeOnce.Do(func() {
		threeOnce.n++
	})
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, "__once.r#do(||") {
		t.Fatalf("promoted source sync.Once should not pass a raw closure to source sync::Once:\n%s", rust)
	}
	hasWrappedOnce := strings.Contains(rust, "__once.r#do(Arc::new(Mutex::new(Some(") ||
		strings.Contains(rust, "__once.r#do(Rc::new(RefCell::new(Some(")
	if !hasWrappedOnce || !strings.Contains(rust, "Box::new(move ||") {
		t.Fatalf("promoted source sync.Once should wrap function literal arguments for source sync::Once:\n%s", rust)
	}
}

func TestSourceMappedPackageGlobalSyncOnceDoWrapsSourceOnceArgument(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "golang.org/x/tools/internal/gcimporter", `package gcimporter

import "sync"

var fakeLinesOnce sync.Once

func initFakeLines() {
	fakeLinesOnce.Do(func() {})
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, "__once.r#do(||") {
		t.Fatalf("source-mapped package-global sync.Once should not pass a raw closure to source sync::Once:\n%s", rust)
	}
	hasWrappedOnce := strings.Contains(rust, "__once.r#do(Arc::new(Mutex::new(Some(") ||
		strings.Contains(rust, "__once.r#do(Rc::new(RefCell::new(Some(")
	if !hasWrappedOnce || !strings.Contains(rust, "Box::new(move ||") {
		t.Fatalf("source-mapped package-global sync.Once should wrap function literal arguments for source sync::Once:\n%s", rust)
	}
}

func TestSourceMappedMethodCallWrapsPackageConstArgument(t *testing.T) {
	fset := token.NewFileSet()
	arena, err := parser.ParseFile(fset, "arena.go", `package runtime

const userArenaChunkPages = 4

func split(s *mspan, base uintptr) {
	s.init(base, userArenaChunkPages)
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(arena.go) error = %v", err)
	}
	mheap, err := parser.ParseFile(fset, "mheap.go", `package runtime

type mspan struct{}

func (s *mspan) init(base, npages uintptr) {}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(mheap.go) error = %v", err)
	}
	files := []*ast.File{arena, mheap}
	typeInfo, err := NewTypeInfoWithImporter("runtime", files, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter() error = %v", err)
	}
	prevConcurrencyDetector := globalConcurrencyDetector
	cd := NewConcurrencyDetector()
	cd.AnalyzeProject(files)
	SetConcurrencyDetector(cd)
	t.Cleanup(func() {
		SetConcurrencyDetector(prevConcurrencyDetector)
	})

	prevContext := currentContext
	packageState := NewPackageState()
	packageState.ConstantNameOverrides = assignPackageConstantNames(files)
	packageState.MethodsByType = collectPackageMethods(files)
	SetTranspileContext(&TranspileContext{
		Session:        NewTranspileSession(typeInfo, map[string]string{"runtime": "runtime"}),
		Package:        packageState,
		PackageMapping: map[string]string{"runtime": "runtime"},
	})
	t.Cleanup(func() {
		SetTranspileContext(prevContext)
	})

	rust, _, _ := TranspileWithMapping(arena, fset, typeInfo, map[string]string{"runtime": "runtime"})

	if strings.Contains(rust, "USER_ARENA_CHUNK_PAGES.clone()") {
		t.Fatalf("source-mapped package const should not be cloned as a wrapped value:\n%s", rust)
	}
	if !strings.Contains(rust, "Arc::new(Mutex::new(Some(USER_ARENA_CHUNK_PAGES as usize)))") &&
		!strings.Contains(rust, "Rc::new(RefCell::new(Some(USER_ARENA_CHUNK_PAGES as usize)))") {
		t.Fatalf("source-mapped package const should be wrapped for the generated method parameter:\n%s", rust)
	}
}

func TestSourceMappedSyncOnceDoReceiverFieldAssignmentUsesOriginalReceiver(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "internal/godebug", `package godebug

import "sync"

type Setting struct {
	once sync.Once
	setting *setting
}

type setting struct {
	value int
}

func lookup() *setting {
	return &setting{value: 7}
}

func (s *Setting) Value() int {
	s.once.Do(func() {
		s.setting = lookup()
	})
	return s.setting.value
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, "s_closure_clone.setting = new_val") {
		t.Fatalf("source sync.Once.Do closure should not assign receiver fields on a cloned receiver:\n%s", rust)
	}
	if !strings.Contains(rust, "let __recv_ptr = self as *mut Setting as usize") ||
		!strings.Contains(rust, "let __recv_ref: &mut Setting = unsafe { &mut *(__recv_ptr as *mut Setting) }") ||
		!strings.Contains(rust, "__recv_ref.setting = new_val") {
		t.Fatalf("source sync.Once.Do closure should assign receiver fields through the original pointer receiver:\n%s", rust)
	}
}

func TestSourceMappedSyncWaitGroupFieldAddWrapsSourceArgument(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "golang.org/x/sync/errgroup", `package errgroup

import "sync"

type Group struct {
	wg sync.WaitGroup
}

func (g *Group) Go() {
	go func() {}()
	g.wg.Add(1)
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, ".wg.add(1)") {
		t.Fatalf("source-mapped sync.WaitGroup field should not pass a raw helper argument:\n%s", rust)
	}
	if !strings.Contains(rust, ".wg.add(Arc::new(Mutex::new(Some(1))))") {
		t.Fatalf("source-mapped sync.WaitGroup field should wrap Add arguments for source sync::WaitGroup:\n%s", rust)
	}
}

func TestSourceMappedLocalSyncWaitGroupAddUsesHelperArgument(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "golang.org/x/tools/internal/gcimporter", `package gcimporter

import "sync"

func wait() {
	var wg sync.WaitGroup
	wg.Add(1)
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, "wg.add(Arc::new(Mutex::new(Some(1))))") {
		t.Fatalf("local sync.WaitGroup variables lower to the helper and should not wrap Add arguments:\n%s", rust)
	}
	if !strings.Contains(rust, "wg.add(1)") {
		t.Fatalf("local sync.WaitGroup variables should pass raw helper arguments:\n%s", rust)
	}
}

func TestNoTypeInfoMethodFunctionParameterPassesHandle(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type recorder struct{}

func (recorder) Use(record func(string)) {}

func relay(record func(string)) {
	var r recorder
	r.Use(record)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if !strings.Contains(rust, ".r#use(record.clone())") {
		t.Fatalf("method function parameter should pass the existing handle:\n%s", rust)
	}
	if strings.Contains(rust, "record.borrow().as_ref().unwrap()") || strings.Contains(rust, "Rc::new(RefCell::new(Some((*record") {
		t.Fatalf("method function parameter used generic wrapping path:\n%s", rust)
	}
}

func TestTypedConstMethodInterfaceArgumentUsesNamedValueFromTypeInfo(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Code interface {
	Value() int
}

type CodeVal int

func (c CodeVal) Value() int {
	return int(c)
}

const (
	ValBool CodeVal = iota
	ValString
)

type Writer struct{}

func (Writer) Code(c Code) int {
	return c.Value()
}

func main() {
	var w Writer
	w.Code(ValBool)
	w.Code(ValString)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)

	if strings.Contains(rust, ".code(Rc::new(RefCell::new(Some(VAL_BOOL))))") ||
		strings.Contains(rust, ".code(Rc::new(RefCell::new(Some(VAL_STRING))))") {
		t.Fatalf("typed constants should not be passed as wrapped raw ints to interface params:\n%s", rust)
	}
	if !strings.Contains(rust, ".code(Rc::new(RefCell::new(Some(Box::new(CodeVal(Rc::new(RefCell::new(Some(VAL_BOOL as i32))))) as Box<dyn Code>))))") {
		t.Fatalf("typed constant should be constructed as its named value for interface params:\n%s", rust)
	}
	if !strings.Contains(rust, ".code(Rc::new(RefCell::new(Some(Box::new(CodeVal(Rc::new(RefCell::new(Some(VAL_STRING as i32))))) as Box<dyn Code>))))") {
		t.Fatalf("implicit typed constant should reuse the previous named type for interface params:\n%s", rust)
	}
}

func TestErrorComparedWithNamedIntegerConstDowncastsConcreteError(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Errno uintptr

const EINTR = Errno(4)

func (e Errno) Error() string { return "" }

func retry(err error) bool {
	return err != EINTR
}
`)

	if strings.Contains(rust, "!= E_I_N_T_R") {
		t.Fatalf("error comparison should not compare boxed error directly with raw const:\n%s", rust)
	}
	if !strings.Contains(rust, ".downcast_ref::<Errno>()") {
		t.Fatalf("error comparison should downcast to the concrete named error type:\n%s", rust)
	}
	if !strings.Contains(rust, "!__matched") {
		t.Fatalf("not-equal error comparison should negate the downcast match:\n%s", rust)
	}
}

func TestNoTypeInfoTrackedSliceIndexDoesNotUseStringPath(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "fmt"

func main() {
	values := []string{"alpha"}
	fmt.Println(values[0])
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, ".as_bytes()[") {
		t.Fatalf("tracked slice index should not use string indexing path:\n%s", rust)
	}
	if !strings.Contains(rust, "values.borrow().as_ref().unwrap())[") {
		t.Fatalf("tracked slice index should use slice indexing path:\n%s", rust)
	}
}

func TestStringConstIndexUsesBareStringBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const table = "" + "\x00\x01\x02"

func lookup(i uint8) int {
	return int(table[i])
}
`)

	if strings.Contains(rust, "TABLE.borrow()") || strings.Contains(rust, "TABLE.lock()") {
		t.Fatalf("string constant index should not borrow or lock the constant as a wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, ".as_bytes()[") {
		t.Fatalf("string constant index should read from bare string bytes:\n%s", rust)
	}
}

func TestRuneConstantBinaryWithUintPeerCastsToPeerType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func digit(val uint) byte {
	q := val / 10
	return byte('0' + val - q*10)
}
`)

	if !strings.Contains(rust, "('0' as u64)") {
		t.Fatalf("rune constant in uint arithmetic should cast to the typed uint peer:\n%s", rust)
	}
	if strings.Contains(rust, "('0' as i32)") {
		t.Fatalf("rune constant in uint arithmetic must not default to i32:\n%s", rust)
	}
}

func TestStringOrByteSliceTypeParamIndexUsesByteSequenceTrait(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func hash[T string | []byte](sep T) uint32 {
	return uint32(sep[0])
}
`)

	if !strings.Contains(rust, ".go_byte(") {
		t.Fatalf("string/byte-slice type parameter index should use byte-sequence trait byte access:\n%s", rust)
	}
	body := rust[strings.Index(rust, "pub fn hash"):]
	if strings.Contains(body, ".as_bytes()[") || strings.Contains(body, "__seq[") {
		t.Fatalf("string/byte-slice type parameter index should not use a concrete String or slice path:\n%s", rust)
	}
}

func TestStringOrByteSliceTypeParamUsesByteSequenceTrait(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func fingerprint[T string | []byte](s T, sep T) (uint32, string) {
	hash := uint32(len(sep))
	hash += uint32(sep[0])
	return hash, string(s[:len(sep)])
}
`)

	if !strings.Contains(rust, "pub fn fingerprint<T: GoByteSequence + Clone") {
		t.Fatalf("string/byte-slice type parameter should use a byte-sequence trait bound:\n%s", rust)
	}
	if !strings.Contains(rust, ".go_len()") {
		t.Fatalf("len on string/byte-slice type parameter should use byte-sequence trait length:\n%s", rust)
	}
	if !strings.Contains(rust, ".go_byte(") {
		t.Fatalf("indexing string/byte-slice type parameter should use byte-sequence trait byte access:\n%s", rust)
	}
	if !strings.Contains(rust, ".go_slice_to_string(") {
		t.Fatalf("string conversion of a string/byte-slice type-parameter slice should use byte-sequence trait conversion:\n%s", rust)
	}
	body := rust[strings.Index(rust, "pub fn fingerprint"):]
	if strings.Contains(body, ".as_bytes()[") {
		t.Fatalf("string/byte-slice type parameter should not be lowered as concrete String bytes:\n%s", rust)
	}
}

func TestTypeParamCallArgumentIsNotLoweredAsInterfaceReference(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func hash[T string | []byte](sep T) uint32 {
	return uint32(sep[0])
}

func use[T string | []byte](sep T) uint32 {
	return hash(sep)
}
`)

	if strings.Contains(rust, "hash(&*") {
		t.Fatalf("type parameter argument should not be lowered as an interface reference:\n%s", rust)
	}
	if !strings.Contains(rust, "hash::<T>(") {
		t.Fatalf("inferred type parameter call should emit explicit Rust type arguments:\n%s", rust)
	}
	if !strings.Contains(rust, "hash::<T>(sep.clone())") {
		t.Fatalf("type parameter value argument should be passed through the wrapped value path:\n%s", rust)
	}
}

func TestTypeParamMethodArgumentClonesWrappedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type box[T any] struct{}

func (b *box[T]) take(value T) {}

func use[T any](b *box[T], value T) {
	b.take(value)
}
`)

	if strings.Contains(rust, "Some((*value.borrow().as_ref().unwrap()))") ||
		strings.Contains(rust, "Some((*value.lock().unwrap().as_ref().unwrap()))") {
		t.Fatalf("type-parameter method argument should not move from a shared value borrow:\n%s", rust)
	}
	if !strings.Contains(rust, "let __arg_holder = value.clone()") ||
		!strings.Contains(rust, "(*__arg_guard.as_ref().unwrap()).go_value_clone()") {
		t.Fatalf("type-parameter method argument should clone the inner value into a fresh handle:\n%s", rust)
	}
}

func TestTypeParamAnyCallArgumentUsesScopedClone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func sink(value any) {}

func use[T any](value T) {
	sink(value)
}
`)

	if !strings.Contains(rust, "pub fn r#use<T: Any + GoValueClone + 'static>") {
		t.Fatalf("type-parameter any argument should require GoValueClone for Go value semantics:\n%s", rust)
	}
	if !strings.Contains(rust, "let __arg_holder = value.clone(); let __arg_guard = __arg_holder.borrow();") ||
		!strings.Contains(rust, "(*__arg_guard.as_ref().unwrap()).go_value_clone()") {
		t.Fatalf("type-parameter any argument should scope a GoValueClone inner clone:\n%s", rust)
	}
}

func TestComparableTypeParamSliceElementEqualityUsesGoComparison(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func same[S ~[]E, E comparable](left, right S, i int) bool {
	return left[i] != right[i]
}
`)

	if strings.Contains(rust, "__tmp_x != __tmp_y") || strings.Contains(rust, "::ptr_eq(&__left, &__right)") {
		t.Fatalf("type-parameter slice element equality should not compare raw generic values:\n%s", rust)
	}
	if !strings.Contains(rust, "GoComparable::go_eq(__left_value, __right_value)") {
		t.Fatalf("type-parameter slice element equality should use Go comparable value comparison:\n%s", rust)
	}
}

func TestTypeParamSelectorHandleUsesClone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type entry[T comparable] struct {
	key T
}

func same[T comparable](e *entry[T], key T, yield func(T) bool) bool {
	if e.key == key {
		return yield(e.key)
	}
	return false
}
`)

	if strings.Contains(rust, "let __left = (*e.borrow().as_ref().unwrap()).key;") ||
		strings.Contains(rust, "let __left = (*e.lock().unwrap().as_ref().unwrap()).key;") {
		t.Fatalf("type-parameter selector equality should not move the field handle:\n%s", rust)
	}
	if strings.Contains(rust, "(*__f)((*e.borrow().as_ref().unwrap()).key)") ||
		strings.Contains(rust, "(*__f)((*e.lock().unwrap().as_ref().unwrap()).key)") {
		t.Fatalf("type-parameter selector function argument should not move the field handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".key.clone()") {
		t.Fatalf("type-parameter selector handle should be cloned before use:\n%s", rust)
	}
}

func TestTypeParamClosureCallUsesHandleArguments(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func same[S1 ~[]E1, S2 ~[]E2, E1, E2 any](left S1, right S2, eq func(E1, E2) bool) bool {
	for i, v := range left {
		if !eq(v, right[i]) {
			return false
		}
	}
	return true
}
`)

	if strings.Contains(rust, "(*__f)(Rc::new(RefCell::new(Some(v)))") {
		t.Fatalf("type-parameter range value should be passed as its existing handle:\n%s", rust)
	}
	if strings.Contains(rust, "Some({ let __seq") {
		t.Fatalf("type-parameter slice element should be passed as its existing handle:\n%s", rust)
	}
	if !strings.Contains(rust, "(*v).clone()") {
		t.Fatalf("type-parameter range value should clone the range handle:\n%s", rust)
	}
}

func TestTypeParamGenericCallUsesHandleArguments(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func less[E int | string](a, b E) bool {
	return a < b
}

func check[S ~[]E, E int | string](data S, i int) bool {
	return less(data[i], data[i-1])
}
`)

	if strings.Contains(rust, "Some({ let __seq") {
		t.Fatalf("type-parameter slice elements should not be double-wrapped as generic call arguments:\n%s", rust)
	}
	if !strings.Contains(rust, "less::<E>(") {
		t.Fatalf("generic helper call should retain inferred type arguments:\n%s", rust)
	}
}

func TestGenericFuncLiteralPointerResultKeepsPointerHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Var struct{}

func apply[T comparable](value T, f func(T) T) T {
	return f(value)
}

func use(a *Var) *Var {
	return apply(a, func(*Var) *Var { return a })
}
`)

	if strings.Contains(rust, "Rc<RefCell<Option<Rc<RefCell<Option<Var>>>>>>") ||
		strings.Contains(rust, "Arc<Mutex<Option<Arc<Mutex<Option<Var>>>>>>") {
		t.Fatalf("function literal returning a pointer for generic T should not nest the pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box<dyn FnMut(Rc<RefCell<Option<Var>>>) -> Rc<RefCell<Option<Var>>>>") &&
		!strings.Contains(rust, "Box<dyn FnMut(Arc<Mutex<Option<Var>>>) -> Arc<Mutex<Option<Var>>>>") {
		t.Fatalf("function literal returning a pointer for generic T should keep the pointer handle ABI:\n%s", rust)
	}
}

func TestSliceConstrainedTypeParamCompositeLiteralUsesSliceRepresentation(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func empty[S ~[]E, E any]() S {
	return S{}
}
`)

	if strings.Contains(rust, "S::default()") {
		t.Fatalf("slice-constrained type parameter composite literal should not use a Rust type-parameter default:\n%s", rust)
	}
	if !strings.Contains(rust, "Vec::<Rc<RefCell<Option<E>>>>::new()") &&
		!strings.Contains(rust, "Vec::<Arc<Mutex<Option<E>>>>::new()") {
		t.Fatalf("slice-constrained type parameter composite literal should emit an empty slice representation:\n%s", rust)
	}
}

func TestSliceConstrainedTypeParamMakeEmitsSliceRepresentation(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func grow[S ~[]E, E any](s S, i int, n int) S {
	return append(s[:i], make(S, n-i)...)
}
`)

	if strings.Contains(rust, "__slice_holder = .clone()") {
		t.Fatalf("make(S, n) used as append expansion should emit a slice value:\n%s", rust)
	}
	if !strings.Contains(rust, "vec![Default::default();") {
		t.Fatalf("make(S, n) should use the slice-constrained element representation:\n%s", rust)
	}
}

func TestConcreteByteSliceAssignedFromGenericSliceResultConvertsRepresentation(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func Insert[S ~[]E, E any](s S, i int, v ...E) S {
	return append(s[:i], v...)
}

type lazybuf struct {
	buf []byte
}

func (b *lazybuf) prepend(prefix ...byte) {
	b.buf = Insert(b.buf, 0, prefix...)
}
`)

	if strings.Contains(rust, "insert::<Vec<u8>, u8>({ let __field = self.buf.clone(); __field }") {
		t.Fatalf("concrete []byte should be converted before calling a generic slice function:\n%s", rust)
	}
	if strings.Contains(rust, "prefix.clone())") {
		t.Fatalf("variadic []byte expansion should be converted before calling a generic slice function:\n%s", rust)
	}
	if !strings.Contains(rust, ".map(|__elem|") {
		t.Fatalf("generic slice result should be converted back to the concrete []byte representation:\n%s", rust)
	}
	if strings.Contains(rust, "collect::<Vec<_>()") {
		t.Fatalf("generic slice conversion should emit valid Rust turbofish syntax:\n%s", rust)
	}
}

func TestNumericTypeParamConversionForLoopUsesConsistentWrapperShape(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func count[T int8 | int16 | int32 | int64 | int, N int64 | uint64](num N) int {
	total := 0
	for i := T(0); i < T(num); i++ {
		total += int(i)
	}
	return total
}
`)

	if strings.Contains(rust, "let mut i = 0;") {
		t.Fatalf("type-parameter conversion short declaration should not create a bare local later used as wrapped:\n%s", rust)
	}
	if strings.Contains(rust, "(*(*num.borrow().as_ref().unwrap()).borrow()") ||
		strings.Contains(rust, "(*(*num.lock().unwrap().as_ref().unwrap()).lock()") {
		t.Fatalf("type-parameter conversion in comparison should not double-unwrap the converted value:\n%s", rust)
	}
	if strings.Contains(rust, "go_integer_cast::<T, _>(") &&
		(strings.Contains(rust, "go_integer_cast::<T, _>((*num.borrow().as_ref().unwrap())).borrow()") ||
			strings.Contains(rust, "go_integer_cast::<T, _>((*num.lock().unwrap().as_ref().unwrap())).lock()")) {
		t.Fatalf("type-parameter conversion in comparison should be used as a raw converted value:\n%s", rust)
	}
}

func TestNumericTypeParamConversionForLoopUsesIntegerTrait(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func count[T int8 | int16 | int32 | int64 | int, N int64 | uint64](num N) int {
	total := 0
	for i := T(0); i < T(num); i++ {
		total += int(i)
	}
	return total
}
`)

	for _, want := range []string{
		"T: GoInteger + Clone",
		"N: GoInteger + Clone",
		"go_integer_from_i128::<T>(0",
		"go_integer_cast::<T, _>",
		"go_integer_add_one",
		"go_integer_cast::<i32, _>",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("numeric type-parameter loop should use integer trait helper %q:\n%s", want, rust)
		}
	}
}

func TestIntegerTypeParamConversionInClosureUsesRawCapture(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func rangeNum[T int8 | int16 | int32 | int64 | int, N int64 | uint64](num N) func() bool {
	return func() bool {
		return T(0) < T(num)
	}
}
`)

	if strings.Contains(rust, "num.borrow()") || strings.Contains(rust, "num.lock()") {
		t.Fatalf("integer type-parameter conversion should not unwrap raw type-param values:\n%s", rust)
	}
	if strings.Contains(rust, "num_closure_clone.borrow()") || strings.Contains(rust, "num_closure_clone.lock()") {
		t.Fatalf("integer type-parameter conversion should not unwrap captured raw type-param values:\n%s", rust)
	}
	if !strings.Contains(rust, "go_integer_cast::<T, _>(num_closure_clone.clone())") {
		t.Fatalf("integer type-parameter conversion should use the raw closure capture:\n%s", rust)
	}
}

func TestOrderedTypeParamComparisonUsesRawOperands(t *testing.T) {
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

	if strings.Contains(rust, "x.borrow()") || strings.Contains(rust, "x.lock()") ||
		strings.Contains(rust, "y.borrow()") || strings.Contains(rust, "y.lock()") {
		t.Fatalf("ordered type-parameter comparison should use raw operands, not wrapped handles:\n%s", rust)
	}
	if !strings.Contains(rust, "x < y") {
		t.Fatalf("ordered type-parameter comparison should compare raw operands:\n%s", rust)
	}
}

func TestOrderedTypeParamConcurrentComparisonClonesRawTemps(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Ordered interface {
	~int | ~string
}

func Less[T Ordered](x, y T) bool {
	go func() {}()
	return x < y || x != x
}
`)

	if strings.Contains(rust, "let __tmp_x = x;") || strings.Contains(rust, "let __tmp_y = y;") {
		t.Fatalf("concurrent ordered type-parameter comparison should not move raw operands into temps:\n%s", rust)
	}
	if !strings.Contains(rust, "let __tmp_x = x.clone(); let __tmp_y = y.clone(); __tmp_x < __tmp_y") {
		t.Fatalf("concurrent ordered type-parameter comparison should clone raw temp operands:\n%s", rust)
	}
}

func TestOrderedTypeParamCallArgumentClonesRawValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Ordered interface {
	~int | ~string
}

func isNaN[T Ordered](x T) bool {
	return x != x
}

func Less[T Ordered](x, y T) bool {
	return isNaN(x) || x < y
}
`)

	if strings.Contains(rust, "is_na_n::<T>(x)") {
		t.Fatalf("ordered type-parameter call argument should not move a raw value that can be reused:\n%s", rust)
	}
	if !strings.Contains(rust, "is_na_n::<T>(x.clone())") {
		t.Fatalf("ordered type-parameter call argument should clone raw values:\n%s", rust)
	}
}

func TestExternalStdlibScalarCallStaysBareInComparison(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "runtime"

func ok() bool {
	return runtime.GOMAXPROCS(0) > 0
}
`)

	if strings.Contains(rust, "g_o_m_a_x_p_r_o_c_s(0).borrow()") ||
		strings.Contains(rust, "g_o_m_a_x_p_r_o_c_s(0).lock()") {
		t.Fatalf("external stdlib scalar call should not be unwrapped as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "runtime::g_o_m_a_x_p_r_o_c_s(0) > 0") {
		t.Fatalf("external stdlib scalar call should stay bare in comparison:\n%s", rust)
	}
}

func TestNoTypeInfoRangeUsesTrackedMap(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevCollections := localCollectionKinds
	prevMapKeys := localMapKeyRustTypes
	prevMapValues := localMapValueRustTypes
	prevRangeVars := rangeLoopVars
	defer func() {
		currentTypeInfo = prevTypeInfo
		localCollectionKinds = prevCollections
		localMapKeyRustTypes = prevMapKeys
		localMapValueRustTypes = prevMapValues
		rangeLoopVars = prevRangeVars
	}()
	SetTypeInfo(nil)
	localCollectionKinds = map[string]string{"ages": "map"}
	localMapKeyRustTypes = map[string]string{"ages": "String"}
	localMapValueRustTypes = map[string]string{"ages": "Rc<RefCell<Option<i32>>>"}
	rangeLoopVars = make(map[string]string)

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
func f() {
	for _, age := range ages { fmt.Println(age) }
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	fn := file.Decls[0].(*ast.FuncDecl)
	stmt := fn.Body.List[0]
	var out strings.Builder
	TranspileStatementSimple(&out, stmt, nil, token.NewFileSet())

	got := out.String()
	if !strings.Contains(got, "for (_, age) in { let __range_holder = ages.clone();") {
		t.Fatalf("tracked map range did not use map iteration:\n%s", got)
	}
	if strings.Contains(got, "type info required") || strings.Contains(got, "0..") {
		t.Fatalf("tracked map range fell back to non-map lowering:\n%s", got)
	}
}

func TestNoTypeInfoSliceParamRangeUsesCopiedElement(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func keep(numbers []int, pred func(int) bool) []int {
	var result []int
	for _, num := range numbers {
		if pred(num) {
			result = append(result, num)
		}
	}
	return result
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if !strings.Contains(rust, "for num in __range_values.iter().copied()") {
		t.Fatalf("[]int parameter range should copy scalar elements:\n%s", rust)
	}
	if !strings.Contains(rust, "push(num)") {
		t.Fatalf("append should store the copied scalar range element:\n%s", rust)
	}
}

func TestNoTypeInfoRangeUsesTrackedChannelParam(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevCollections := localCollectionKinds
	prevRangeElemTypes := localRangeElemRustTypes
	prevRangeVars := rangeLoopVars
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		localCollectionKinds = prevCollections
		localRangeElemRustTypes = prevRangeElemTypes
		rangeLoopVars = prevRangeVars
		SetVarTable(prevVarTable)
	}()
	SetTypeInfo(nil)
	localCollectionKinds = make(map[string]string)
	localRangeElemRustTypes = make(map[string]string)
	rangeLoopVars = make(map[string]string)

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
func f(ch chan struct{ name string }) {
	for event := range ch { println(event.name) }
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	fn := file.Decls[0].(*ast.FuncDecl)
	registerTypeExprCollectionInfo("ch", fn.Type.Params.List[0].Type)
	vt := NewVarTable()
	vt.Register("ch", &VarInfo{
		WrapLevel: WrapNone,
		Source:    SourceParam,
	})
	SetVarTable(vt)

	var out strings.Builder
	TranspileStatementSimple(&out, fn.Body.List[0], nil, token.NewFileSet())

	got := out.String()
	if !strings.Contains(got, "for event in ch.clone()") {
		t.Fatalf("tracked channel range did not use channel iteration:\n%s", got)
	}
	if strings.Contains(got, "type info required") || strings.Contains(got, "0..") {
		t.Fatalf("tracked channel range fell back to non-channel lowering:\n%s", got)
	}
}

func TestNoTypeInfoMakeChannelShortDeclIsBare(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevCollections := localCollectionKinds
	prevRangeElemTypes := localRangeElemRustTypes
	prevRangeVars := rangeLoopVars
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		localCollectionKinds = prevCollections
		localRangeElemRustTypes = prevRangeElemTypes
		rangeLoopVars = prevRangeVars
		SetVarTable(prevVarTable)
	}()
	SetTypeInfo(nil)
	localCollectionKinds = make(map[string]string)
	localRangeElemRustTypes = make(map[string]string)
	rangeLoopVars = make(map[string]string)
	vt := NewVarTable()
	SetVarTable(vt)

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
func f() {
	ch := make(chan int)
	for n := range ch { println(n) }
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	fn := file.Decls[0].(*ast.FuncDecl)

	var assignOut strings.Builder
	TranspileStatementSimple(&assignOut, fn.Body.List[0], nil, token.NewFileSet())
	assignRust := assignOut.String()
	if !strings.Contains(assignRust, "let mut ch = GoChannel::<i32>::new()") {
		t.Fatalf("make channel short decl should emit a bare GoChannel local:\n%s", assignRust)
	}
	if !isVarBare("ch") {
		t.Fatalf("make channel short decl did not register ch as bare; info=%#v\n%s", vt.Lookup("ch"), assignRust)
	}
	if localCollectionKinds["ch"] != "channel" {
		t.Fatalf("local collection kind for ch = %q, want channel", localCollectionKinds["ch"])
	}

	var rangeOut strings.Builder
	TranspileStatementSimple(&rangeOut, fn.Body.List[1], nil, token.NewFileSet())
	rangeRust := rangeOut.String()
	if !strings.Contains(rangeRust, "for n in ch.clone()") {
		t.Fatalf("tracked make channel range did not use channel iteration:\n%s", rangeRust)
	}
	if strings.Contains(rangeRust, ".lock()") || strings.Contains(rangeRust, "type info required") {
		t.Fatalf("tracked make channel range treated channel as wrapped:\n%s", rangeRust)
	}
}

func TestNoTypeInfoAnySelectorReturnKeepsFieldHandle(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		SetVarTable(prevVarTable)
	}()
	SetTypeInfo(nil)

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
type entry struct { value any }
func get(e entry) any { return e.value }`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeSpec := file.Decls[0].(*ast.GenDecl).Specs[0].(*ast.TypeSpec)
	structType := typeSpec.Type.(*ast.StructType)
	structDefs = map[string]*StructDef{
		"entry": {
			Fields:  map[string]string{"value": "regular"},
			ASTType: structType,
		},
	}

	fn := file.Decls[1].(*ast.FuncDecl)
	vt := NewVarTable()
	vt.Register("e", &VarInfo{
		WrapLevel: WrapFull,
		RustType:  "entry",
		Source:    SourceParam,
	})
	SetVarTable(vt)

	var out strings.Builder
	TranspileStatementSimple(&out, fn.Body.List[0], fn.Type, token.NewFileSet())

	got := out.String()
	if !strings.Contains(got, ".value.clone()") {
		t.Fatalf("any selector return should clone the field handle:\n%s", got)
	}
	if strings.Contains(got, "Box<dyn Any") || strings.Contains(got, ".as_ref().unwrap()))") {
		t.Fatalf("any selector return should not unwrap and rewrap the Box payload:\n%s", got)
	}
}

func TestAnyReturnPointerSelectorBoxesHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type node struct {
	n int
}

type holder struct {
	ptr *node
}

func start() {
	go func() {}()
}

func get(h holder) any {
	return h.ptr
}
`)

	if strings.Contains(rust, "Box::new((*{ let __field = (*h.lock().unwrap().as_ref().unwrap()).ptr.clone(); __field }.lock().unwrap().as_ref().unwrap()))") {
		t.Fatalf("pointer selector returned as any should box the pointer handle, not unwrap the pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new((*h.lock().unwrap().as_ref().unwrap()).ptr.clone()) as Box<dyn Any + Send + Sync>") {
		t.Fatalf("pointer selector returned as any should clone the pointer handle into the box:\n%s", rust)
	}
}

func TestNoTypeInfoAnyHandleReuseUsesSyntax(t *testing.T) {
	src := `package main
import "fmt"
type entry struct { value any }
func assign(e *entry, value any) { e.value = value }
func callAssign(e *entry, value any) { assign(e, value) }
func each(e *entry, f func(any)) { f(e.value) }
func printAny(value any) { fmt.Println(value) }
func callEach(e *entry) { each(e, func(v any) { fmt.Println(v) }) }`

	assertAnyHandleReuseUsesSyntax(t, transpileNoTypeInfoRegression(t, src))
	assertAnyHandleReuseUsesSyntax(t, transpileRegression(t, src, &TypeInfo{}))
}

func assertAnyHandleReuseUsesSyntax(t *testing.T, rust string) {
	t.Helper()

	if !strings.Contains(rust, "let new_val = value.clone();") {
		t.Fatalf("any field assignment should clone the existing interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".as_mut().unwrap()).value = new_val") ||
		strings.Contains(rust, ".as_ref().unwrap()).value = new_val") {
		t.Fatalf("any field assignment should mutate the owning struct slot:\n%s", rust)
	}
	if !strings.Contains(rust, "assign(e.clone(), value.clone())") {
		t.Fatalf("any function argument should pass the existing interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".value.clone())") {
		t.Fatalf("any selector closure argument should pass the field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "format_any(value") {
		t.Fatalf("fmt.Println(any) should use format_any under syntax fallback:\n%s", rust)
	}
	if !strings.Contains(rust, "format_any(v") {
		t.Fatalf("fmt.Println on function-literal any parameter should use format_any:\n%s", rust)
	}
	if strings.Contains(rust, "value.borrow().as_ref().unwrap().clone()") ||
		strings.Contains(rust, "value.lock().unwrap().as_ref().unwrap().clone()") ||
		strings.Contains(rust, "Some((*value") ||
		strings.Contains(rust, "format!(\"{}\", (*value") ||
		strings.Contains(rust, "format!(\"{}\", (*v") {
		t.Fatalf("any handle reuse should not clone or rewrap the Box payload:\n%s", rust)
	}
}

func TestNoTypeInfoPackageGlobalIdentUsesGlobalName(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevGlobals := packageGlobalNames
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageGlobalNames = prevGlobals
		SetVarTable(prevVarTable)
	}()
	SetTypeInfo(nil)
	packageGlobalNames = map[string]bool{"n": true}
	SetVarTable(NewVarTable())

	var exprOut strings.Builder
	TranspileExpression(&exprOut, ast.NewIdent("n"))
	if got := exprOut.String(); strings.Contains(got, "n_local") || !strings.Contains(got, "(*n") {
		t.Fatalf("package global expression = %q, want global n access", got)
	}

	stmt, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
func f() { n++ }`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	fn := stmt.Decls[0].(*ast.FuncDecl)
	var stmtOut strings.Builder
	TranspileStatementSimple(&stmtOut, fn.Body.List[0], nil, token.NewFileSet())
	if got := stmtOut.String(); strings.Contains(got, "n_local") || !strings.Contains(got, "n.borrow_mut()") {
		t.Fatalf("package global increment = %q, want global n mutation", got)
	}
}

func TestPackageGlobalPointerEqualityUsesStoredHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type struct{}

type MapType struct {
	Key *Type
}

func makeType() *Type {
	return &Type{}
}

var stringType = makeType()

func same(tt *MapType) bool {
	return tt.Key == stringType
}
`)

	if strings.Contains(rust, "let __right = stringType.clone()") {
		t.Fatalf("package-global pointer equality should not compare against the global slot handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __right = (*stringType.borrow().as_ref().unwrap()).clone()") {
		t.Fatalf("package-global pointer equality should clone the pointer stored in the global:\n%s", rust)
	}
}

func TestNoTypeInfoPackageGlobalMapIndexUsesSyntaxKind(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevGlobals := packageGlobalNames
	prevCollections := localCollectionKinds
	prevMapKeys := localMapKeyRustTypes
	prevMapValues := localMapValueRustTypes
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageGlobalNames = prevGlobals
		localCollectionKinds = prevCollections
		localMapKeyRustTypes = prevMapKeys
		localMapValueRustTypes = prevMapValues
	}()
	SetTypeInfo(nil)
	packageGlobalNames = make(map[string]bool)
	localCollectionKinds = make(map[string]string)
	localMapKeyRustTypes = make(map[string]string)
	localMapValueRustTypes = make(map[string]string)

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
var counts map[string]int
func f() { _ = counts["x"] }`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	globalDecl := file.Decls[0].(*ast.GenDecl)
	collectPackageGlobals([]*ast.GenDecl{globalDecl})
	if got := localCollectionKinds["counts"]; got != "map" {
		t.Fatalf("package global map kind = %q, want map", got)
	}

	restore := pushFunctionLocalSyntaxInfo()
	defer restore()
	if got := localCollectionKinds["counts"]; got != "map" {
		t.Fatalf("function-local syntax scope dropped package global map kind = %q, want map", got)
	}

	fn := file.Decls[1].(*ast.FuncDecl)
	assign := fn.Body.List[0].(*ast.AssignStmt)
	index := assign.Rhs[0]
	var out strings.Builder
	TranspileExpression(&out, index)

	got := out.String()
	if strings.Contains(got, "type info required") {
		t.Fatalf("package global map index should use syntax collection kind:\n%s", got)
	}
	if !strings.Contains(got, "counts.clone()") ||
		!strings.Contains(got, ".as_ref().and_then(|__map| __map.get(&\"x\".to_string()))") {
		t.Fatalf("package global map index should read from global map:\n%s", got)
	}
}

func TestNoTypeInfoPackageGlobalFunctionMapIndexKeepsHandle(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevGlobals := packageGlobalNames
	prevCollections := localCollectionKinds
	prevMapKeys := localMapKeyRustTypes
	prevMapValues := localMapValueRustTypes
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevContext := currentContext
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageGlobalNames = prevGlobals
		localCollectionKinds = prevCollections
		localMapKeyRustTypes = prevMapKeys
		localMapValueRustTypes = prevMapValues
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetTranspileContext(prevContext)
	}()
	SetTranspileContext(nil)
	SetTypeInfo(nil)
	packageGlobalNames = make(map[string]bool)
	localCollectionKinds = make(map[string]string)
	localMapKeyRustTypes = make(map[string]string)
	localMapValueRustTypes = make(map[string]string)
	functionTypeAliases = map[string]bool{"handler": true}
	functionTypeAliasBoxTypes = map[string]string{
		"handler": "Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>",
	}

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
type handler func(int) int
var handlers map[string]handler
func f() { _ = handlers["inc"] }`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	globalDecl := file.Decls[1].(*ast.GenDecl)
	collectPackageGlobals([]*ast.GenDecl{globalDecl})
	restore := pushFunctionLocalSyntaxInfo()
	defer restore()
	if got := localMapValueRustTypes["handlers"]; got == "" {
		t.Fatal("package global function map value type was not tracked")
	} else if !rustMapValueTypeKeepsHandle(got) {
		t.Fatalf("tracked function map value type %q should keep handle", got)
	}

	fn := file.Decls[2].(*ast.FuncDecl)
	assign := fn.Body.List[0].(*ast.AssignStmt)
	index := assign.Rhs[0]
	var out strings.Builder
	TranspileExpression(&out, index)

	if got, want := functionBoxTypeForCallTarget(index), "Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>"; got != want {
		t.Fatalf("function map index box type = %q, want %q", got, want)
	}

	got := out.String()
	if !strings.Contains(got, ".map(|__v| __v.clone()).unwrap_or_else(|| Default::default())") {
		t.Fatalf("function map index should clone the stored handle:\n%s", got)
	}
	if strings.Contains(got, ".borrow().as_ref().unwrap().clone()") {
		t.Fatalf("function map index should not unwrap the stored function handle:\n%s", got)
	}
}

func TestSelectorNamedIntegerMapValueConversionUnwrapsValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "go/token"

type atPos token.Pos

func lookup(m map[string]token.Pos, key string) atPos {
	return atPos(m[key])
}
`)

	if strings.Contains(rust, ".map(|__v| __v.clone()).unwrap_or_else(|| Default::default())") {
		t.Fatalf("selector-named integer map value should not keep the map value handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".map(|__v| __v.borrow().as_ref().unwrap().clone())") &&
		!strings.Contains(rust, ".map(|__v| __v.lock().unwrap().as_ref().unwrap().clone())") {
		t.Fatalf("selector-named integer map value should unwrap the stored value:\n%s", rust)
	}
}

func TestMapLookupOnNilMapFieldUsesZeroValue(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Object interface {
	Name() string
}

type item struct{}

func (*item) Name() string { return "" }

type Scope struct {
	elems map[string]Object
}

func forceConcurrent() {
	go func() {}()
}

func (s *Scope) Lookup(name string) Object {
	return s.elems[name]
}
`)

	if strings.Contains(rust, ".as_ref().unwrap()).get(") {
		t.Fatalf("map lookup should not unwrap a nil map before reading:\n%s", rust)
	}
	if !strings.Contains(rust, ".as_ref().and_then(|__map| __map.get(") {
		t.Fatalf("map lookup should treat a nil map as empty:\n%s", rust)
	}
	if !strings.Contains(rust, ".map(|__v| __v.clone()).unwrap_or_else(|| Default::default())") {
		t.Fatalf("map lookup should still return the element zero value when missing:\n%s", rust)
	}
}

func TestStringSliceMapLookupKeyUsesOwnedString(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type group struct {
	sign int
}

var groups = map[string]group{
	"ab": {sign: 1},
}

func lookup(s string) group {
	return groups[s[0:2]]
}
`)

	if strings.Contains(rust, ".get(&Rc::new(RefCell::new(Some(") ||
		strings.Contains(rust, ".get(&Arc::new(Mutex::new(Some(") {
		t.Fatalf("string map lookup key should not pass a wrapped string handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".get(&{ let __") {
		t.Fatalf("string map lookup key should pass an owned string value:\n%s", rust)
	}
}

func TestStringTypeAssertionMapLookupKeyStaysBare(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func lookup(did map[string]bool, name any) bool {
	return did[name.(string)]
}
`)

	if strings.Contains(rust, "__map_key_holder.lock()") ||
		strings.Contains(rust, "__map_key_holder.borrow()") {
		t.Fatalf("string type assertion map key should stay bare:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<String>()") {
		t.Fatalf("string type assertion map key should use the asserted string value:\n%s", rust)
	}
}

func TestPointerToSliceIndexUsesDerefValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

var _ sync.Mutex

type Pos int

type iface struct {
	embedPos *[]Pos
}

func pick(ityp *iface, i int) Pos {
	var pos Pos
	if ityp.embedPos != nil {
		pos = (*ityp.embedPos)[i]
	}
	return pos
}
`)

	if strings.Contains(rust, "__seq_guard") && strings.Contains(rust, "embed_pos") {
		t.Fatalf("pointer-to-slice index should not lock the cloned Vec as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __seq =") || !strings.Contains(rust, "__seq[") {
		t.Fatalf("pointer-to-slice index should index the dereferenced slice value:\n%s", rust)
	}
}

func TestNilFunctionFieldCompositeLiteralUsesEmptyHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Package struct{}

type Qualifier func(*Package) string

type writer struct {
	qf Qualifier
}

func newWriter() *writer {
	return &writer{qf: nil}
}
`)

	if strings.Contains(rust, "qf: Rc::new(RefCell::new(Some(None)))") ||
		strings.Contains(rust, "qf: Arc::new(Mutex::new(Some(None)))") {
		t.Fatalf("nil function field should clear the function handle, not store Some(None):\n%s", rust)
	}
	if !strings.Contains(rust, "qf: Default::default()") {
		t.Fatalf("nil function field should use an empty function handle:\n%s", rust)
	}
}

func TestNoTypeInfoAssignedNestedStringRangeUsesBareValue(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevRangeVars := rangeLoopVars
	defer func() {
		currentTypeInfo = prevTypeInfo
		rangeLoopVars = prevRangeVars
	}()
	SetTypeInfo(nil)
	rangeLoopVars = map[string]string{"files": "&Vec<String>"}

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
func f() {
	for _, file := range files {
		if file == "a.go" {
			file = "src/" + file
		}
		res = append(res, file)
	}
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	fn := file.Decls[0].(*ast.FuncDecl)
	stmt := fn.Body.List[0]
	var out strings.Builder
	TranspileStatementSimple(&out, stmt, nil, token.NewFileSet())

	got := out.String()
	if !strings.Contains(got, "for mut file in files.iter().cloned()") {
		t.Fatalf("assigned nested string range should iterate owned values:\n%s", got)
	}
	if strings.Contains(got, "file.lock()") || strings.Contains(got, "(*file).clone()") {
		t.Fatalf("assigned nested string range should treat file as bare String:\n%s", got)
	}
	if !strings.Contains(got, "file = new_val") {
		t.Fatalf("assigned nested string range should assign the bare binding:\n%s", got)
	}
	if !strings.Contains(got, ".push(file.clone())") {
		t.Fatalf("assigned nested string range append should clone the bare binding:\n%s", got)
	}
}

func TestStructRangeSelectorUsesRangeBinding(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"fmt"
	"sort"
)

type packageFunctionName struct {
	goName string
	pos int
	exported bool
}

func f() map[string]string {
	byRustName := make(map[string][]packageFunctionName)
	overrides := make(map[string]string)
	for rustName, functions := range byRustName {
		sort.Slice(functions, func(i, j int) bool {
			if functions[i].exported != functions[j].exported {
				return functions[i].exported
			}
			if functions[i].pos != functions[j].pos {
				return functions[i].pos < functions[j].pos
			}
			return functions[i].goName < functions[j].goName
		})
		for i, fn := range functions {
			if i == 0 {
				continue
			}
			overrides[fn.goName] = fmt.Sprintf("%s_%d", rustName, i)
		}
	}
	return overrides
}`, 0)
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
	if strings.Contains(rust, "fn.borrow()") || strings.Contains(rust, "fn.lock()") || strings.Contains(rust, "r#fn.borrow()") || strings.Contains(rust, "r#fn.lock()") {
		t.Fatalf("struct range selector should use the range binding directly:\n%s", rust)
	}
	if !strings.Contains(rust, "r#fn.go_name") {
		t.Fatalf("struct range selector should access the field on the range binding:\n%s", rust)
	}
}

func TestPackageGlobalsRangeSelectorUsesRangeBinding(t *testing.T) {
	fset := token.NewFileSet()
	entries, err := os.ReadDir(".")
	if err != nil {
		t.Fatalf("ReadDir(.) error = %v", err)
	}
	var files []*ast.File
	var target *ast.File
	for _, entry := range entries {
		name := entry.Name()
		if entry.IsDir() || !strings.HasSuffix(name, ".go") || strings.HasSuffix(name, "_test.go") {
			continue
		}
		file, err := parser.ParseFile(fset, name, nil, 0)
		if err != nil {
			t.Fatalf("ParseFile(%s) error = %v", name, err)
		}
		files = append(files, file)
		if name == "package_globals.go" {
			target = file
		}
	}
	if target == nil {
		t.Fatal("package_globals.go was not parsed")
	}
	// The transpiler's own files import non-stdlib packages such as
	// golang.org/x/tools/go/packages that the default importer cannot resolve.
	// NewTypeInfoWithImporter now returns those partial-info errors instead of
	// silently dropping them, but the regression being pinned here only needs
	// the in-package type information.
	typeInfo, err := NewTypeInfo(files, fset)
	if typeInfo == nil {
		t.Fatalf("NewTypeInfo() returned no TypeInfo: %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(target, fset, typeInfo)
	for _, bad := range []string{
		"r#fn.lock().unwrap().as_ref().unwrap()).go_name",
		"r#fn.borrow().as_ref().unwrap()).go_name",
	} {
		idx := strings.Index(rust, bad)
		if idx < 0 {
			continue
		}
		start := max(0, idx-200)
		end := min(len(rust), idx+300)
		t.Fatalf("package_globals range selector should use the range binding directly:\n%s", rust[start:end])
	}
}

func TestMethodPointerArgsPreserveHandles(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/types"

type term struct{}
type termlist []*term

func (x *term) union(y *term) (*term, *term) { return x, y }
func (x *term) includes(t types.Type) bool { return false }

func (xl termlist) norm(t types.Type) {
	for i, xi := range xl {
		xj := xl[i]
		xi.union(xj)
	}
	for _, x := range xl {
		x.includes(t)
	}
}`, 0)
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
	if strings.Contains(rust, "union((*xj") || strings.Contains(rust, "intersect((*y") {
		t.Fatalf("pointer method argument should preserve the pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".union(xj.clone())") {
		t.Fatalf("pointer method argument should clone the pointer handle:\n%s", rust)
	}
	if strings.Contains(rust, "includes({ let __v = (*t") {
		t.Fatalf("stdlib interface method argument should preserve the interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".includes(t.clone())") {
		t.Fatalf("stdlib interface method argument should clone the interface handle:\n%s", rust)
	}
}

func TestBareReceiverMethodCallSelectorFieldArgumentKeepsHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Step struct{ I int }
type Bitmap [2]byte

func (b *Bitmap) Set(i int) {}

func f(st Step, bitmap Bitmap) {
	bitmap.Set(st.I)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	prevTypeInfo := currentTypeInfo
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		SetVarTable(prevVarTable)
	}()
	SetTypeInfo(typeInfo)
	vt := NewVarTable()
	vt.Register("bitmap", &VarInfo{WrapLevel: WrapNone, Source: SourceLocal})
	vt.Register("st", &VarInfo{WrapLevel: WrapNone, Source: SourceLocal})
	SetVarTable(vt)

	var call *ast.CallExpr
	ast.Inspect(file, func(node ast.Node) bool {
		if call != nil {
			return false
		}
		candidate, ok := node.(*ast.CallExpr)
		if !ok {
			return true
		}
		sel, ok := candidate.Fun.(*ast.SelectorExpr)
		if ok && sel.Sel.Name == "Set" {
			call = candidate
			return false
		}
		return true
	})
	if call == nil {
		t.Fatal("method call not found")
	}

	var out strings.Builder
	TranspileExpression(&out, call)
	got := out.String()
	if strings.Contains(got, ".set((*st.i") || strings.Contains(got, "st.i.borrow") || strings.Contains(got, "st.i.lock") {
		t.Fatalf("bare receiver method argument should preserve the selector field handle:\n%s", got)
	}
	if !strings.Contains(got, ".set({ let __field = st.i.clone(); __field })") {
		t.Fatalf("bare receiver method argument should clone the selector field handle:\n%s", got)
	}
}

func TestNilPointerFunctionArgumentUsesNilHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Context struct{}

func read(ctxt *Context) {}

func f() {
	read(nil)
}`, 0)
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
	if strings.Contains(rust, "nil.clone()") {
		t.Fatalf("nil pointer argument should emit a nil handle:\n%s", rust)
	}
	if !strings.Contains(rust, "read(Rc::new(RefCell::new(None)))") {
		t.Fatalf("nil pointer argument should pass a wrapped nil handle:\n%s", rust)
	}
}

func TestAnonymousInterfaceAssertionMethodCallUsesSynthesizedTraitObject(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Pos int

type Decl struct{}
func (*Decl) Pos() Pos { return 1 }

type Other struct{}
func (*Other) Pos() Pos { return 2 }

func f(x any) Pos {
	pos := x.(interface{ Pos() Pos }).Pos()
	return pos
}
`)

	for _, want := range []string{
		`pub trait GoAnonymousInterface`,
		`if let Some(typed_val) = any_val.downcast_ref::<DeclPtr>()`,
		`Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface`,
		`if let Some(typed_val) = any_val.downcast_ref::<OtherPtr>()`,
		`let __result = (*__recv.borrow().as_ref().unwrap()).pos(); __result`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("anonymous-interface assertion method call should use a synthesized trait object; missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, `unimplemented!("type info required: assertion method call on anonymous interface`) {
		t.Fatalf("anonymous-interface assertion method call should not stay on the old unsupported path:\n%s", rust)
	}
}

func TestReturnStringParameterCopiesWrappedValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func echo(value string) string {
	return value
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
	if strings.Contains(rust, "return value.clone();") {
		t.Fatalf("returning a string parameter should not alias the wrapper handle:\n%s", rust)
	}
	if !strings.Contains(rust, "{ let __owned = value.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }") {
		t.Fatalf("returning a string parameter should clone the wrapped value into a new handle:\n%s", rust)
	}
}

func TestForInitShortDeclShadowsOuterRangeIndex(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type List struct{}

func (l *List) Len() int { return 0 }
func (l *List) At(i int) int { return i }

func f(lists []*List) {
	go func() {}()
	for i, list := range lists {
		_ = i
		for i := 0; i < list.Len(); i++ {
			_ = list.At(i)
		}
	}
}`, 0)
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
	if strings.Contains(rust, "let __tmp_x = i;") {
		t.Fatalf("inner for index should shadow outer bare range index in comparisons:\n%s", rust)
	}
	if strings.Contains(rust, ".at(i);") {
		t.Fatalf("inner for index should be passed as a wrapped handle, not the outer bare range index:\n%s", rust)
	}
	if strings.Contains(rust, ".at(i.clone())") {
		t.Fatalf("inner for index should not inherit bare range-index call argument handling:\n%s", rust)
	}
	if !strings.Contains(rust, "while (*i.borrow().as_ref().unwrap()) <") {
		t.Fatalf("inner for index should be unwrapped from its own local wrapper:\n%s", rust)
	}
}

func TestRangeIndexUntypedConstPeersUseGoInt(t *testing.T) {
	rust := transpileRegression(t, `package main

func f(xs []int) int {
	go func() {}()
	total := 0
	for i := range xs {
		if i > 0 {
			total += i
		}
		j := i + 1
		total += j
	}
	return total
}`, &TypeInfo{})

	if strings.Contains(rust, "0 as usize") || strings.Contains(rust, "1 as usize") {
		t.Fatalf("range index untyped integer peers should use Go int, not usize:\n%s", rust)
	}
	hasComparison := strings.Contains(rust, "i as i32 > 0 as i32") ||
		strings.Contains(rust, "let __tmp_x = i as i32; let __tmp_y = 0 as i32")
	if !hasComparison {
		t.Fatalf("range index comparison should cast the range index and constant to Go int:\n%s", rust)
	}
	hasArithmetic := strings.Contains(rust, "i as i32 + 1 as i32") ||
		strings.Contains(rust, "let __tmp_x = i as i32; let __tmp_y = 1 as i32")
	if !hasArithmetic {
		t.Fatalf("range index arithmetic should cast the range index and constant to Go int:\n%s", rust)
	}
}

func TestPromotedImportedEmbeddedFieldUsesTypedSelectionPath(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "go/token"

type wrapped struct {
	token.Position
}

func line(w *wrapped) int {
	return w.Line
}`)

	returnIndex := strings.Index(rust, "return ")
	if returnIndex < 0 {
		t.Fatalf("missing return in generated Rust:\n%s", rust)
	}
	returnExpr := rust[returnIndex:]
	positionIndex := strings.Index(returnExpr, ".position.")
	lineIndex := strings.Index(returnExpr, ".line")
	if positionIndex < 0 || lineIndex < 0 || positionIndex > lineIndex {
		t.Fatalf("promoted imported embedded field should traverse the typed embedded field path:\n%s", rust)
	}
}

func TestPromotedNamedIntegerMethodUsesEmbeddedFieldReceiver(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type flag uintptr

func (f flag) kind() int {
	return int(f)
}

func (f flag) mustBeExported() {}

type Value struct {
	flag
}

func use(x Value) int {
	x.mustBeExported()
	return x.kind()
}`)

	if strings.Contains(rust, "flag::must_be_exported(&(*x.borrow().as_ref().unwrap()))") ||
		strings.Contains(rust, "flag::kind(&(*x.borrow().as_ref().unwrap()))") {
		t.Fatalf("promoted named-integer method should not use the outer struct as the receiver:\n%s", rust)
	}
	if !strings.Contains(rust, ".flag.clone()") ||
		!strings.Contains(rust, ".must_be_exported()") ||
		!strings.Contains(rust, ".kind()") {
		t.Fatalf("promoted named-integer method should dispatch through the embedded field:\n%s", rust)
	}
}

func TestPromotedPointerFieldDoesNotUseReceiverMethodName(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type File struct {
	*file
}

type file struct {
	name string
}

func (f *File) Name() string {
	return f.name
}`)

	methodIndex := strings.Index(rust, "pub fn name(&self)")
	if methodIndex < 0 {
		t.Fatalf("missing generated Name method:\n%s", rust)
	}
	methodBody := rust[methodIndex:]
	if strings.Contains(methodBody, "self.name") {
		t.Fatalf("promoted field should not lower to receiver method-name field access:\n%s", rust)
	}
	if !strings.Contains(methodBody, "self.file") || !strings.Contains(methodBody, ".name") {
		t.Fatalf("promoted field should traverse the embedded pointer field:\n%s", rust)
	}
}

func TestFieldAccessInfoFromSelectionUsesPromotedPointerPath(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type File struct {
	*file
}

type file struct {
	name string
}

func (f *File) Name() string {
	return f.name
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	var selector *ast.SelectorExpr
	ast.Inspect(file, func(n ast.Node) bool {
		if sel, ok := n.(*ast.SelectorExpr); ok && sel.Sel.Name == "name" {
			selector = sel
			return false
		}
		return true
	})
	if selector == nil {
		t.Fatal("missing f.name selector")
	}
	fieldInfo, ok := fieldAccessInfoFromSelection(selector, typeInfo)
	if !ok {
		t.Fatal("fieldAccessInfoFromSelection did not resolve f.name")
	}
	if !fieldInfo.IsPromoted || len(fieldInfo.EmbeddedPath) != 1 || fieldInfo.EmbeddedPath[0] != "file" || fieldInfo.FieldName != "name" {
		t.Fatalf("field info = %+v, want promoted file.name path", fieldInfo)
	}
}

func TestReturnStructSliceRangeValueClonesReference(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Label struct {
	name string
}

type listMap struct {
	labels []Label
}

func (lm listMap) Find(name string) Label {
	for _, l := range lm.labels {
		if l.name == name {
			return l
		}
	}
	return Label{}
}`, 0)
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
	if strings.Contains(rust, "Some(l)") {
		t.Fatalf("returning a struct range value should not wrap the reference directly:\n%s", rust)
	}
	if !strings.Contains(rust, "Some((*l).clone())") {
		t.Fatalf("returning a struct range value should clone the referenced value:\n%s", rust)
	}
}

func TestRangeStringFunctionArgumentWrapsOwnedClone(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func find(pkg string) (int, bool) { return 0, true }

func imports(pkgs []string) {
	for _, pkg := range pkgs {
		find(pkg)
	}
}`, 0)
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
	if strings.Contains(rust, "find(pkg.clone())") {
		t.Fatalf("range string argument should not be passed as a bare clone:\n%s", rust)
	}
	if !strings.Contains(rust, "find(Rc::new(RefCell::new(Some((*pkg).clone()))))") {
		t.Fatalf("range string argument should be cloned into a wrapped Go string:\n%s", rust)
	}
}

func TestCapturedReferenceRangeValueUsesCapturedClone(t *testing.T) {
	prevRangeLoopVars := rangeLoopVars
	prevCaptureRenames := currentCaptureRenames
	defer func() {
		rangeLoopVars = prevRangeLoopVars
		currentCaptureRenames = prevCaptureRenames
	}()

	rangeLoopVars = map[string]string{"chunk": "&Vec<String>"}
	currentCaptureRenames = map[string]string{"chunk": "chunk_closure_clone"}

	var out strings.Builder
	if !writeOwnedRangeValue(&out, ast.NewIdent("chunk")) {
		t.Fatalf("writeOwnedRangeValue returned false")
	}
	if got := out.String(); got != "chunk_closure_clone.clone()" {
		t.Fatalf("captured range clone = %q", got)
	}
}

func TestCapturedWrappedStructFieldValueUsesClosureClone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Ident struct{}

type Field struct {
	Names []*Ident
}

func makeField(names []*Ident) func() *Field {
	return func() *Field {
		return &Field{Names: names}
	}
}
`)

	if strings.Contains(rust, "names: names.clone()") {
		t.Fatalf("captured struct field value should not move the outer binding into the closure:\n%s", rust)
	}
	if !strings.Contains(rust, "let names_closure_clone = names.clone();") ||
		!strings.Contains(rust, "names: names_closure_clone.clone()") {
		t.Fatalf("captured struct field value should use the closure clone:\n%s", rust)
	}
}

func TestTrackedRangeElemFallbackFillsGenericValueType(t *testing.T) {
	prevRangeElemTypes := localRangeElemRustTypes
	defer func() {
		localRangeElemRustTypes = prevRangeElemTypes
	}()
	localRangeElemRustTypes = map[string]string{"testData": "Vec<i32>"}

	valueType, needsCopied, ok := trackedRangeElemValueType(ast.NewIdent("testData"))
	if !ok {
		t.Fatalf("trackedRangeElemValueType ok = false")
	}
	if valueType != "&Vec<i32>" {
		t.Fatalf("valueType = %q, want &Vec<i32>", valueType)
	}
	if needsCopied {
		t.Fatalf("needsCopied = true, want false for Vec element")
	}
}

func TestExternalStubCallClonesMapRangeStringKey(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "path/filepath"

func f(overlay map[string]string) {
	overlays := make(map[string]string)
	for k, v := range overlay {
		_ = filepath.Base(k)
		overlays[k] = v
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, "filepath::base(k.clone())") {
		t.Fatalf("external stub call should clone map range key before later reuse:\n%s", rust)
	}
	if strings.Contains(rust, "filepath::base(k)") {
		t.Fatalf("external stub call moved map range key:\n%s", rust)
	}
}

func TestStdlibInterfaceSelectorFieldArgumentUsesFieldHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/ast"

func accept(n ast.Node) {}

func visit(kv *ast.KeyValueExpr) {
	accept(kv.Value)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, "let __arg = { let __field = (*kv.borrow().as_ref().unwrap()).value.clone(); __field }; let __converted") {
		t.Fatalf("stdlib interface selector field argument did not clone the field handle:\n%s", rust)
	}
}

func TestStdlibInterfaceConstSelectorConversionUsesRawValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "rand.go", `package rand

import (
	"crypto/internal/boring"
	"io"
)

var Reader io.Reader

func init() {
	if boring.Enabled {
		Reader = boring.RandReader
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(rand.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("crypto/rand", []*ast.File{file}, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "boring::RAND_READER.lock()") {
		t.Fatalf("stdlib interface conversion from a const selector must not borrow it as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __arg = boring::RAND_READER; __arg.into()") {
		t.Fatalf("stdlib interface conversion from a const selector should convert the raw value:\n%s", rust)
	}
}

func TestSourceMappedPointerToStdlibInterfaceConversionKeepsHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "rand.go", `package rand

import "io"

type reader struct{}

func (r *reader) Read(b []byte) (int, error) {
	return len(b), nil
}

var Reader io.Reader

func init() {
	Reader = &reader{}
}

func set(r *reader) {
	Reader = r
}

func use() bool {
	_, ok := Reader.(*reader)
	return ok
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(rand.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("crypto/rand", []*ast.File{file}, fset, nil)
	if err != nil {
		t.Fatalf("NewTypeInfoWithImporter() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"crypto/rand": "crypto_rand"})
	if strings.Contains(rust, "io_Reader::default()") {
		t.Fatalf("source concrete pointer assigned to stdlib interface must not become a default interface:\n%s", rust)
	}
	if !strings.Contains(rust, "io_Reader::__go_from(") {
		t.Fatalf("source concrete pointer assigned to stdlib interface should use __go_from:\n%s", rust)
	}
	if strings.Contains(rust, "io_Reader::__go_from((*r.borrow().as_ref().unwrap())") ||
		strings.Contains(rust, "io_Reader::__go_from((*r.lock().unwrap().as_ref().unwrap())") {
		t.Fatalf("source concrete pointer ident assigned to stdlib interface should pass the handle:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<Rc<RefCell<Option<reader>>>>") &&
		!strings.Contains(rust, "downcast_ref::<Arc<Mutex<Option<reader>>>>") {
		t.Fatalf("stdlib interface pointer assertion should recover the stored pointer handle:\n%s", rust)
	}
}

func TestLocalVariableShadowsImportedPackageSelector(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/ast"

func packageName(file *ast.File) string {
	ast := file
	return ast.Name.Name
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "ast::name") {
		t.Fatalf("local variable named ast should not be emitted as package selector:\n%s", rust)
	}
	if !strings.Contains(rust, ".name") {
		t.Fatalf("selector chain should still access the Name fields:\n%s", rust)
	}
}
