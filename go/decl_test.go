package main

import (
	"go/ast"
	"go/parser"
	"go/token"
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
	if !strings.Contains(rust, "fn pos(&self) -> Rc<RefCell<Option<i32>>>") {
		t.Fatalf("Node method should be delegated on boxed Expr trait object:\n%s", rust)
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
	if !strings.Contains(rust, "pub(crate) struct AnonymousStruct1") {
		t.Fatalf("anonymous package-global struct type should be visible across generated modules:\n%s", rust)
	}
	if !strings.Contains(rust, "pub(crate) enabled:") {
		t.Fatalf("anonymous package-global struct fields should be visible across generated modules:\n%s", rust)
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
