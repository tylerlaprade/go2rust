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
