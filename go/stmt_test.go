package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
	"strings"
	"testing"
)

func TestTypeSwitchDropsSubjectGuardBeforeCaseBody(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func isString(v interface{}) bool {
	switch v.(type) {
	case string:
		return true
	}
	return false
}

func classify(v interface{}) bool {
	switch v.(type) {
	case string:
		return isString(v)
	default:
		return false
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
	rust, _, _ := Transpile(file, fset, typeInfo)

	callIndex := strings.Index(rust, "return is_string(")
	if callIndex < 0 {
		t.Fatalf("generated Rust did not contain reentrant type switch call:\n%s", rust)
	}
	if dropIndex := strings.LastIndex(rust[:callIndex], "drop(_ts_guard);"); dropIndex < 0 {
		t.Fatalf("type switch case body should release subject guard before reusing subject:\n%s", rust)
	}
}

func TestTypeSwitchOnLocalInterfaceFieldUsesTraitAny(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Expr interface {
	isExpr()
}

type TagExpr struct {
	Tag string
}

func (*TagExpr) isExpr() {}

type NotExpr struct {
	X Expr
}

func (x *NotExpr) wrap() string {
	switch x.X.(type) {
	case *TagExpr:
		return "tag"
	}
	return ""
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
	if !strings.Contains(rust, "let _ts_owned = _ts_guard.as_ref().cloned();") ||
		!strings.Contains(rust, "let __any = __v.__go_as_any();") {
		t.Fatalf("type switch on local interface field should downcast through __go_as_any:\n%s", rust)
	}
	if strings.Contains(rust, "let _ts_val = _ts_guard.as_ref();") {
		t.Fatalf("type switch on local interface field should not treat trait objects as bare Any:\n%s", rust)
	}
}

func TestTypeSwitchOnLocalInterfaceParameterUsesTraitAny(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type decl interface {
	node()
}

type varDecl struct{}

func (varDecl) node() {}

type typeDecl struct{}

func (typeDecl) node() {}

func classify(d decl) bool {
	switch d.(type) {
	case varDecl:
		return true
	case typeDecl:
		return true
	}
	return false
}

func walk(d decl, f func(decl)) {
	f(d)
}

func classifyViaCallback(d decl) bool {
	ok := false
	walk(d, func(d decl) {
		switch d.(type) {
		case varDecl:
			ok = true
		case typeDecl:
			ok = true
		}
	})
	return ok
}
`)

	classifyIndex := strings.Index(rust, "pub fn classify")
	if classifyIndex < 0 {
		t.Fatalf("generated Rust did not contain classify function:\n%s", rust)
	}
	callbackIndex := strings.Index(rust, "pub fn classify_via_callback")
	if callbackIndex < 0 {
		t.Fatalf("generated Rust did not contain classify_via_callback function:\n%s", rust)
	}
	classifyRust := rust[classifyIndex:callbackIndex]
	if !strings.Contains(classifyRust, "let __any = __v.__go_as_any();") {
		t.Fatalf("type switch on local interface parameter should downcast through __go_as_any:\n%s", rust)
	}
	if strings.Contains(classifyRust, "__v.as_ref() as &dyn Any") {
		t.Fatalf("type switch on local interface parameter should not treat boxed trait objects as bare Any:\n%s", rust)
	}
	callbackRust := rust[callbackIndex:]
	closureIndex := strings.Index(callbackRust, "Box::new(move |d:")
	if closureIndex < 0 {
		t.Fatalf("generated Rust did not contain callback closure:\n%s", rust)
	}
	closureRust := callbackRust[closureIndex:]
	if closureEnd := strings.Index(closureRust, "}) as Box<dyn FnMut"); closureEnd >= 0 {
		closureRust = closureRust[:closureEnd]
	}
	if !strings.Contains(closureRust, "let __any = __v.__go_as_any();") {
		t.Fatalf("type switch on local interface callback parameter should downcast through __go_as_any:\n%s", rust)
	}
	if strings.Contains(closureRust, "__v.as_ref() as &dyn Any") {
		t.Fatalf("type switch on local interface callback parameter should not treat boxed trait objects as bare Any:\n%s", rust)
	}
}

func TestTypeSwitchLocalInterfaceCaseUsesConcreteCandidates(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Node interface {
	Pos() int
}

type Decl interface {
	Node
	declNode()
}

type Stmt interface {
	Node
	stmtNode()
}

type GenDecl struct{}

func (*GenDecl) Pos() int { return 0 }
func (*GenDecl) declNode() {}

type AssignStmt struct{}

func (*AssignStmt) Pos() int { return 0 }
func (*AssignStmt) stmtNode() {}

func important(q Node) bool {
	switch q.(type) {
	case Decl, Stmt:
		return true
	}
	return false
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
	fnIndex := strings.Index(rust, "pub fn important")
	if fnIndex < 0 {
		t.Fatalf("generated Rust did not contain important function:\n%s", rust)
	}
	fnRust := rust[fnIndex:]
	if strings.Contains(fnRust, "downcast_ref::<Decl>()") ||
		strings.Contains(fnRust, "downcast_ref::<Stmt>()") ||
		strings.Contains(fnRust, "downcast_ref::<Box<dyn Decl") ||
		strings.Contains(fnRust, "downcast_ref::<Box<dyn Stmt") {
		t.Fatalf("type switch case interfaces should not be emitted as concrete Rust downcasts:\n%s", rust)
	}
	if !strings.Contains(fnRust, "downcast_ref::<GenDeclPtr>()") ||
		!strings.Contains(fnRust, "downcast_ref::<AssignStmtPtr>()") {
		t.Fatalf("type switch case interfaces should check concrete implementors from go/types:\n%s", rust)
	}
}

func TestTypeSwitchPointerLocalInterfaceCaseUsesPointerWrapper(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Object interface {
	Name() string
}

type TypeName struct {
	name string
}

func (t *TypeName) Name() string { return t.name }

type Builtin struct {
	name string
}

func (b *Builtin) Name() string { return b.name }

func classify(obj Object) string {
	switch obj := obj.(type) {
	case *TypeName:
		return obj.Name()
	case *Builtin:
		return obj.Name()
	default:
		panic("unreachable")
	}
}

func assertTypeName(obj Object) string {
	if t, ok := obj.(*TypeName); ok {
		return t.Name()
	}
	return ""
}
`)
	classifyIndex := strings.Index(rust, "pub fn classify")
	assertIndex := strings.Index(rust, "pub fn assert_type_name")
	if classifyIndex < 0 || assertIndex < 0 {
		t.Fatalf("generated Rust did not contain expected functions:\n%s", rust)
	}
	classifyRust := rust[classifyIndex:assertIndex]
	assertRust := rust[assertIndex:]
	if !strings.Contains(classifyRust, "downcast_ref::<TypeNamePtr>()") ||
		!strings.Contains(classifyRust, "downcast_ref::<BuiltinPtr>()") {
		t.Fatalf("type switch on local interface pointer cases should downcast to pointer wrappers:\n%s", rust)
	}
	if strings.Contains(classifyRust, "downcast_ref::<TypeName>()") ||
		strings.Contains(classifyRust, "downcast_ref::<Builtin>()") {
		t.Fatalf("type switch on local interface pointer cases should not downcast to cloned pointee values:\n%s", rust)
	}
	if !strings.Contains(classifyRust, "unwrap().0.clone()") {
		t.Fatalf("type switch pointer case binding should preserve the original pointer handle:\n%s", rust)
	}
	if !strings.Contains(assertRust, "downcast_ref::<TypeNamePtr>()") {
		t.Fatalf("type assertion on local interface pointer should downcast to the pointer wrapper:\n%s", rust)
	}
	if strings.Contains(assertRust, "downcast_ref::<TypeName>()") {
		t.Fatalf("type assertion on local interface pointer should not downcast to the pointee value:\n%s", rust)
	}
	if !strings.Contains(assertRust, "typed_val.0.clone()") {
		t.Fatalf("type assertion on local interface pointer should return the original pointer handle:\n%s", rust)
	}
}

func TestSourceMappedImportedInterfaceTypeSwitchPointerCaseUsesWrapper(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/ast"

func classify(d ast.Decl) bool {
	switch d := d.(type) {
	case *ast.GenDecl:
		return d != nil
	default:
		return false
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

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/ast": "go_ast"})
	if !strings.Contains(rust, "downcast_ref::<go_ast::r#mod::GenDeclPtr>()") &&
		!strings.Contains(rust, "downcast_ref::<go_ast::GenDeclPtr>()") {
		t.Fatalf("type switch on source-mapped imported interface pointer case should downcast to pointer wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "let _ts_owned = _ts_guard.as_ref().cloned();") {
		t.Fatalf("type switch on source-mapped imported interface should drop the subject guard before evaluating cases:\n%s", rust)
	}
	if (!strings.Contains(rust, "downcast_ref::<Box<dyn go_ast::r#mod::Decl") &&
		!strings.Contains(rust, "downcast_ref::<Box<dyn go_ast::Decl")) ||
		!strings.Contains(rust, "__boxed.as_ref().__go_as_any()") {
		t.Fatalf("type switch on source-mapped imported interface should peel nested interface boxes:\n%s", rust)
	}
	if strings.Contains(rust, "let __any = __v.__go_as_any();") ||
		strings.Contains(rust, "__boxed.__go_as_any()") {
		t.Fatalf("type switch on source-mapped imported interface should inspect the dynamic trait object, not the Box wrapper:\n%s", rust)
	}
	if strings.Contains(rust, "downcast_ref::<go_ast::r#mod::GenDecl>()") ||
		strings.Contains(rust, "downcast_ref::<go_ast::GenDecl>()") {
		t.Fatalf("type switch on source-mapped imported interface pointer case should not downcast to pointee value:\n%s", rust)
	}
	if !strings.Contains(rust, "unwrap().0.clone()") {
		t.Fatalf("type switch on source-mapped imported interface pointer case should preserve the original pointer handle:\n%s", rust)
	}
}

func TestSourceMappedImportedInterfaceTypeSwitchSameInterfaceCaseRewrapsSubject(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/ast"

func classify(d ast.Decl) bool {
	switch d := d.(type) {
	case ast.Decl:
		return d != nil
	default:
		return false
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

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/ast": "go_ast"})
	if !strings.Contains(rust, "if _ts_val.is_some()") {
		t.Fatalf("type switch on same source-mapped interface case should match any non-nil subject:\n%s", rust)
	}
	if strings.Contains(rust, "downcast_ref::<go_ast::r#mod::GenDeclPtr>()") ||
		strings.Contains(rust, "downcast_ref::<go_ast::GenDeclPtr>()") {
		t.Fatalf("type switch on same source-mapped interface case should not search concrete implementors:\n%s", rust)
	}
	if !strings.Contains(rust, "let d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>>") &&
		!strings.Contains(rust, "let d: Arc<Mutex<Option<Box<dyn go_ast::Decl + Send + Sync>>>>") &&
		!strings.Contains(rust, "let d: Rc<RefCell<Option<Box<dyn go_ast::r#mod::Decl>>>>") &&
		!strings.Contains(rust, "let d: Rc<RefCell<Option<Box<dyn go_ast::Decl>>>>") {
		t.Fatalf("type switch case variable should keep the imported interface case type:\n%s", rust)
	}
	if !strings.Contains(rust, "let __inner: Box<dyn go_ast::r#mod::Decl + Send + Sync>") &&
		!strings.Contains(rust, "let __inner: Box<dyn go_ast::Decl + Send + Sync>") &&
		!strings.Contains(rust, "let __inner: Box<dyn go_ast::r#mod::Decl>") &&
		!strings.Contains(rust, "let __inner: Box<dyn go_ast::Decl>") {
		t.Fatalf("type switch same-interface case binding should rewrap the subject interface:\n%s", rust)
	}
}

func TestSourceMappedImportedInterfaceTypeSwitchEmbeddedInterfaceCaseRewrapsSubject(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/ast"

func classify(d ast.Decl) bool {
	switch n := d.(type) {
	case ast.Node:
		return n != nil
	default:
		return false
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

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/ast": "go_ast"})
	if !strings.Contains(rust, "if _ts_val.is_some()") {
		t.Fatalf("type switch on embedded source-mapped interface case should match any non-nil subject:\n%s", rust)
	}
	if strings.Contains(rust, "downcast_ref::<go_ast::r#mod::GenDeclPtr>()") ||
		strings.Contains(rust, "downcast_ref::<go_ast::GenDeclPtr>()") {
		t.Fatalf("type switch on embedded source-mapped interface case should not search concrete implementors:\n%s", rust)
	}
	if !strings.Contains(rust, "let n: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>") &&
		!strings.Contains(rust, "let n: Arc<Mutex<Option<Box<dyn go_ast::Node + Send + Sync>>>>") &&
		!strings.Contains(rust, "let n: Rc<RefCell<Option<Box<dyn go_ast::r#mod::Node>>>>") &&
		!strings.Contains(rust, "let n: Rc<RefCell<Option<Box<dyn go_ast::Node>>>>") {
		t.Fatalf("type switch embedded-interface case variable should keep the case type:\n%s", rust)
	}
	if !strings.Contains(rust, "let __inner: Box<dyn go_ast::r#mod::Node + Send + Sync>") &&
		!strings.Contains(rust, "let __inner: Box<dyn go_ast::Node + Send + Sync>") &&
		!strings.Contains(rust, "let __inner: Box<dyn go_ast::r#mod::Node>") &&
		!strings.Contains(rust, "let __inner: Box<dyn go_ast::Node>") {
		t.Fatalf("type switch embedded-interface case binding should rewrap the subject interface:\n%s", rust)
	}
}

func TestTypeSwitchOnGoErrorAnonymousInterfaceCasesUseConcreteErrorValues(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type multi struct {
	errs []error
}

func (m *multi) Error() string { return "multi" }
func (m *multi) Unwrap() []error { return m.errs }

func has(err error) bool {
	switch x := err.(type) {
	case interface{ Unwrap() error }:
		err = x.Unwrap()
		return err != nil
	case interface{ Unwrap() []error }:
		return len(x.Unwrap()) > 0
	default:
		return false
	}
}
`)

	if strings.Contains(rust, "downcast_ref::<multiPtr>()") {
		t.Fatalf("type switch on error should downcast to the boxed concrete error value, not a pointer wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<multi>()") {
		t.Fatalf("type switch on error should downcast to the boxed concrete error value:\n%s", rust)
	}
	if strings.Contains(rust, "let x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = unimplemented!(\"type info required: type switch on interface case with 0 concrete implementors") ||
		strings.Contains(rust, "let x: Rc<RefCell<Option<Box<dyn Any>>>> = unimplemented!(\"type info required: type switch on interface case with 0 concrete implementors") {
		t.Fatalf("unreachable anonymous interface case binding should still use its method-set trait type:\n%s", rust)
	}
	if strings.Contains(rust, "type switch on interface case with 0 concrete implementors needs a synthesized trait object") {
		t.Fatalf("unreachable anonymous interface case binding should use a typed nil handle, not unimplemented!:\n%s", rust)
	}
	if !strings.Contains(rust, "let x: Arc<Mutex<Option<Box<dyn GoAnonymousInterface") &&
		!strings.Contains(rust, "let x: Rc<RefCell<Option<Box<dyn GoAnonymousInterface") {
		t.Fatalf("anonymous interface type-switch binding should synthesize a method-set trait object:\n%s", rust)
	}
	if !strings.Contains(rust, "None::<Box<dyn GoAnonymousInterface") {
		t.Fatalf("anonymous interface type-switch binding should initialize unreachable cases with typed nil:\n%s", rust)
	}
}

func TestCurrentPackagePointerReceiverReturnToSourceMappedInterfaceBoxesWrapper(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "container/heap"

type IntHeap []int

func (h IntHeap) Len() int { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }
func (h *IntHeap) Push(x any) { *h = append(*h, x.(int)) }
func (h *IntHeap) Pop() any { return nil }
func (h *IntHeap) AsHeap() heap.Interface { return h }
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"container/heap": "container_heap",
		"sort":           "sort",
	})
	if strings.Contains(rust, "Rc::new(RefCell::new(Some(Box::new((*h.") ||
		strings.Contains(rust, "Rc::new(RefCell::new(Some(Box::new(self.clone()) as Box<dyn container_heap::Interface>)))") {
		t.Fatalf("pointer receiver returned to source-mapped interface should not box the pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(IntHeapPtr(") {
		t.Fatalf("pointer receiver returned to source-mapped interface should box the pointer wrapper:\n%s", rust)
	}
}

func TestTypeSwitchDefaultBindingKeepsInterfaceHandleWrapped(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type positioner interface {
	Pos() int
}

type Spec interface {
	Pos() int
	End() int
}

type ValueSpec struct{}

func (*ValueSpec) Pos() int { return 0 }
func (*ValueSpec) End() int { return 0 }

func report(p positioner) {}

func walk(specs []Spec) {
	for _, s := range specs {
		switch s := s.(type) {
		case *ValueSpec:
			_ = s
		default:
			report(s)
		}
	}
}
`)

	if strings.Contains(rust, "Box::new(s) as Box<dyn positioner") {
		t.Fatalf("type switch default binding should not box the interface handle itself:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner") &&
		!strings.Contains(rust, "Box::new({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner") {
		t.Fatalf("type switch default binding should rebox the source trait object through the adapter:\n%s", rust)
	}
}

func TestSourceMappedInterfaceIdentCallArgumentCloneIsScoped(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"go/ast"
	"go/token"
)

type positioner interface {
	Pos() token.Pos
}

func report(pos positioner, value any) {}

func walk(d ast.Decl) {
	report(d, d)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"go/ast":   "go_ast",
		"go/token": "go_token",
	})

	if !strings.Contains(rust, "let __arg_holder = d.clone(); let __arg_guard = __arg_holder.borrow();") {
		t.Fatalf("source-mapped interface ident call argument should scope the source guard before the next argument:\n%s", rust)
	}
	if strings.Contains(rust, "Box::new((*d.borrow().as_ref().unwrap()).clone()) as Box<dyn positioner") ||
		strings.Contains(rust, "Box::new((*d.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner") ||
		strings.Contains(rust, "Box::new((*d.borrow().as_ref().unwrap()).clone()) as Box<dyn Any") ||
		strings.Contains(rust, "Box::new((*d.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any") {
		t.Fatalf("source-mapped interface ident call argument should not keep the source guard alive in the outer call:\n%s", rust)
	}
}

func TestTypeSwitchStubBackedExternalInterfaceImplementsLocalInterfaceAsStubValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"go/ast"
	"go/token"
)

type positioner interface {
	Pos() token.Pos
}

func span(at positioner) token.Pos {
	switch x := at.(type) {
	case ast.Node:
		return x.Pos()
	default:
		return at.Pos()
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

	rust, _, _ := Transpile(file, fset, typeInfo)

	if strings.Contains(rust, "impl positioner for Box<dyn ast_Node") {
		t.Fatalf("stub-backed external interface should not be treated as a local trait object:\n%s", rust)
	}
	if !strings.Contains(rust, "impl positioner for ast_Node") {
		t.Fatalf("stub-backed external interface should implement the local interface as its stub value:\n%s", rust)
	}
}

func TestTypeSwitchAssignsConcreteToBareAnyRangeVar(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func rewrite(args []any) []any {
	for i, arg := range args {
		switch arg.(type) {
		case nil:
			arg = "<nil>"
		case string:
			arg = "string"
		}
		args[i] = arg
	}
	return args
}
`)

	if strings.Contains(rust, `let new_val = "<nil>"; arg = new_val`) {
		t.Fatalf("bare any range assignment should not store a raw string:\n%s", rust)
	}
	if !strings.Contains(rust, `Box::new("<nil>".to_string()) as Box<dyn Any`) {
		t.Fatalf("bare any range assignment should box the concrete string:\n%s", rust)
	}
	if strings.Contains(rust, "let _ts_ref = arg;") {
		t.Fatalf("type switch on bare any range value should not move the subject before later uses:\n%s", rust)
	}
	if !strings.Contains(rust, "let _ts_ref = &arg;") {
		t.Fatalf("type switch on bare any range value should borrow the subject for downcasts:\n%s", rust)
	}
}

func TestAnyRangeWritebackMovesSourceMappedInterfaceValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/ast"

func rewrite(args []any) []any {
	for i, arg := range args {
		switch a := arg.(type) {
		case ast.Decl:
			arg = a
		}
		args[i] = arg
	}
	return args
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/ast": "go_ast"})
	if !strings.Contains(rust, "std::mem::replace(&mut __range_guard.as_mut().unwrap()[__range_index]") {
		t.Fatalf("[]any range with indexed writeback should move elements instead of cloning dynamic payloads:\n%s", rust)
	}
	if strings.Contains(rust, "go_any_clone(__e.as_ref())") {
		t.Fatalf("[]any range with indexed writeback should not pre-clone dynamic payloads:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<Box<dyn go_ast::Decl") {
		t.Fatalf("source-mapped interface case should still downcast through the typed trait object:\n%s", rust)
	}
}

func TestTypeSwitchDefaultBareAnyRefPassedToVariadicAny(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func sink(args ...any) {}

func rewrite(args []any) {
	for _, arg := range args {
		switch n := arg.(type) {
		case int:
			_ = n
		default:
			sink(n)
		}
	}
}
`)

	if strings.Contains(rust, "let __any_holder = n.clone()") ||
		strings.Contains(rust, "__any_holder.borrow()") ||
		strings.Contains(rust, "__any_holder.lock().unwrap()") {
		t.Fatalf("type switch default any ref passed to ...any should not be treated as a wrapped handle:\n%s", rust)
	}
	if !strings.Contains(rust, "go_any_clone(n)") {
		t.Fatalf("type switch default any ref passed to ...any should clone the referenced boxed value:\n%s", rust)
	}
}

func TestConcurrentTypeSwitchDefaultBareAnyRefPassedToVariadicAny(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func sink(args ...any) {}

func rewrite(args []any) {
	go func() {}()
	for _, arg := range args {
		switch n := arg.(type) {
		case int:
			_ = n
		default:
			sink(n)
		}
	}
}
`)

	if strings.Contains(rust, "let _ts_val: Option<&dyn Any>") {
		t.Fatalf("concurrent type switch on bare any range value should keep Send+Sync on the borrowed trait object:\n%s", rust)
	}
	if !strings.Contains(rust, "let _ts_val: Option<&(dyn Any + Send + Sync)>") || !strings.Contains(rust, "go_any_clone(n)") {
		t.Fatalf("concurrent type switch default any ref passed to ...any should clone the Send+Sync boxed value:\n%s", rust)
	}
}

func TestConcurrentTypeSwitchOnAnyUnwrapsBoxedAnyPayload(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type marker struct{}

func classify(v any) string {
	done := make(chan bool)
	_ = done
	switch v.(type) {
	case marker:
		return "marker"
	default:
		return "other"
	}
}
`)

	for _, want := range []string{
		"let mut __any = __v.as_ref() as &dyn Any;",
		"while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>()",
		"__any = __boxed.as_ref() as &dyn Any;",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("concurrent type switch on any should unwrap boxed any payload fragment %q:\n%s", want, rust)
		}
	}
}

func TestTypeSwitchInterfaceCaseSynthesizesMultiCandidateBinding(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Positioner interface {
	Pos() int
}

type A struct{}
func (A) Pos() int { return 0 }

type B struct{}
func (B) Pos() int { return 0 }

func use(Positioner) {}

func visit(x any) {
	switch a := x.(type) {
	case Positioner:
		use(a)
	}
}
`)

	if strings.Contains(rust, `type switch on interface case with 2 concrete implementors needs a synthesized trait object`) {
		t.Fatalf("interface case binding should synthesize a trait object instead of panicking:\n%s", rust)
	}
	for _, want := range []string{
		`let a: Rc<RefCell<Option<Box<dyn Positioner>>>> = if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<A>())`,
		`} else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<B>())`,
		`Box::new(typed_val.clone()) as Box<dyn Positioner>`,
		`panic!("type switch interface case condition matched no concrete implementor")`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("interface case binding should contain %q:\n%s", want, rust)
		}
	}
}

func TestTypeSwitchInterfaceSingleCandidateBindingUsesCaseInterface(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Positioner interface {
	Pos() int
}

type A struct{}
func (A) Pos() int { return 0 }

func visit(x any) Positioner {
	switch a := x.(type) {
	case Positioner:
		a = nil
		return a
	}
	return nil
}
`)

	if strings.Contains(rust, `let mut a = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<A>()).unwrap().clone())))`) {
		t.Fatalf("single-candidate interface case binding should not narrow the case variable to the concrete type:\n%s", rust)
	}
	for _, want := range []string{
		`let mut a: Rc<RefCell<Option<Box<dyn Positioner>>>> = if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<A>())`,
		`Box::new(typed_val.clone()) as Box<dyn Positioner>`,
		`panic!("type switch interface case condition matched no concrete implementor")`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("single-candidate interface case binding should contain %q:\n%s", want, rust)
		}
	}
}

func TestLocalInterfaceReturnBoxesSelectorPointerWrapper(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node interface {
	Pos() int
}

type ImportSpec struct{}

func (*ImportSpec) Pos() int { return 0 }

type importDecl struct {
	spec *ImportSpec
}

func (d importDecl) node() Node {
	return d.spec
}
`)

	if strings.Contains(rust, "return self.spec.clone();") ||
		strings.Contains(rust, "return d.spec.clone();") {
		t.Fatalf("selector pointer returned as a local interface should not return the concrete handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(ImportSpecPtr(self.spec.clone())) as Box<dyn Node") {
		t.Fatalf("selector pointer returned as a local interface should box a pointer-identity wrapper:\n%s", rust)
	}
}

func TestLocalInterfaceReturnBoxesPackageGlobalPointerWrapper(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Object interface {
	Name() string
}

type TypeName struct{}

func (*TypeName) Name() string { return "" }

var global *TypeName

func lookup() Object {
	return global
}
`)

	if strings.Contains(rust, "Box::new((*global.borrow().as_ref().unwrap()).clone()) as Box<dyn Object") ||
		strings.Contains(rust, "Box::new((*global.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Object") {
		t.Fatalf("package-global pointer returned as local interface should not box the pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(TypeNamePtr({ let __arg_holder = global.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })) as Box<dyn Object") &&
		!strings.Contains(rust, "Box::new(TypeNamePtr({ let __arg_holder = global.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })) as Box<dyn Object") {
		t.Fatalf("package-global pointer returned as local interface should box a pointer-identity wrapper:\n%s", rust)
	}
}

func TestSourceMappedInterfaceMultiResultCallReturnBoxesConcretePointer(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "syscall"

type rawConn struct{}

func (*rawConn) Control(func(uintptr)) error { return nil }
func (*rawConn) Read(func(uintptr) bool) error { return nil }
func (*rawConn) Write(func(uintptr) bool) error { return nil }

func newRawConn() (*rawConn, error) {
	return &rawConn{}, nil
}

func syscallConn() (syscall.RawConn, error) {
	return newRawConn()
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"syscall": "syscall"})
	if strings.Contains(rust, "return new_raw_conn();") {
		t.Fatalf("multi-result call returned directly without source-mapped interface slot conversion:\n%s", rust)
	}
	if !strings.Contains(rust, "let (__return_tmp_0, __return_tmp_1) = new_raw_conn();") {
		t.Fatalf("multi-result call should be captured before slot conversion:\n%s", rust)
	}
	if !strings.Contains(rust, "let __return_slot_0 =") {
		t.Fatalf("converted multi-result slots should be bound before the final tuple:\n%s", rust)
	}
	if !strings.Contains(rust, "let __return_slot_0 = Rc::new(RefCell::new(Some(Box::new((*__return_tmp_0.borrow().as_ref().unwrap()).clone()) as Box<dyn syscall::RawConn") &&
		!strings.Contains(rust, "let __return_slot_0 = Arc::new(Mutex::new(Some(Box::new((*__return_tmp_0.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn syscall::RawConn") {
		t.Fatalf("concrete pointer result should be boxed into the source-mapped interface slot:\n%s", rust)
	}
	if !strings.Contains(rust, "(__return_slot_0, __return_tmp_1)") {
		t.Fatalf("final tuple should use the converted source-mapped interface slot:\n%s", rust)
	}
	if !strings.Contains(rust, "impl syscall::RawConn for rawConn") {
		t.Fatalf("source-mapped interface return should register the concrete trait impl:\n%s", rust)
	}
}

func TestPackageGlobalPointerCallArgumentBoxesPointerWrapper(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Object interface {
	Name() string
}

type TypeName struct{}

func (*TypeName) Name() string { return "" }

var global *TypeName

func def(obj Object) {}

func init() {
	def(global)
}
`)

	if strings.Contains(rust, "Box::new((*global.borrow().as_ref().unwrap()).clone()) as Box<dyn Object") ||
		strings.Contains(rust, "Box::new((*global.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Object") {
		t.Fatalf("package-global pointer argument should not box the pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(TypeNamePtr({ let __arg_holder = global.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })) as Box<dyn Object") &&
		!strings.Contains(rust, "Box::new(TypeNamePtr({ let __arg_holder = global.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })) as Box<dyn Object") {
		t.Fatalf("package-global pointer argument should box a pointer-identity wrapper:\n%s", rust)
	}
}

func TestPackageGlobalPointerFieldAssignmentMutatesPointee(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node struct {
	next *Node
}

var head *Node
var tail *Node

func link() {
	head = &Node{}
	head.next = tail
}
`)

	if strings.Contains(rust, "(*head.borrow_mut().as_mut().unwrap()).next = new_val") ||
		strings.Contains(rust, "(*head.lock().unwrap().as_mut().unwrap()).next = new_val") {
		t.Fatalf("package-global pointer field assignment should not mutate the global slot layer:\n%s", rust)
	}
	if !strings.Contains(rust, "(*(*head.borrow().as_ref().unwrap()).borrow_mut().as_mut().unwrap()).next = new_val") &&
		!strings.Contains(rust, "(*(*head.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_mut().unwrap()).next = new_val") {
		t.Fatalf("package-global pointer field assignment should unwrap the pointer handle before mutating the pointee:\n%s", rust)
	}
}

func TestAddressOfStringsBuilderPassedToStdlibWriterMethod(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"io"
	"strings"
)

type Scope struct{}

func (s *Scope) WriteTo(w io.Writer, n int, recurse bool) {}

func (s *Scope) String() string {
	var buf strings.Builder
	s.WriteTo(&buf, 0, false)
	return buf.String()
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"io":      "io",
		"strings": "strings",
	})

	if strings.Contains(rust, "write_to(buf.clone()") {
		t.Fatalf("strings.Builder passed as io.Writer should not use the raw builder handle:\n%s", rust)
	}
	if strings.Contains(rust, "io_Writer") {
		t.Fatalf("source-mapped strings.Builder as io.Writer should not use the external io_Writer bridge:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(strings::BuilderPtr(buf.clone().clone())) as Box<dyn io::Writer") &&
		!strings.Contains(rust, "Box::new(strings::r#mod::BuilderPtr(buf.clone().clone())) as Box<dyn io::r#mod::Writer") {
		t.Fatalf("source-mapped strings.Builder as io.Writer should box the pointer wrapper:\n%s", rust)
	}
}

func TestPackageGlobalSelectorReturnedAsExternalIoWriterIsLoud(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"io"
	"os"
)

func output() io.Writer {
	return os.Stderr
}
`)

	if strings.Contains(rust, "return os::Stderr().clone();") {
		t.Fatalf("package-global selector returned as stdlib interface should not return the concrete handle:\n%s", rust)
	}
	if strings.Contains(rust, "let __converted: Option<io_Writer>") ||
		strings.Contains(rust, "impl From<os_File> for io_Writer") {
		t.Fatalf("package-global os.File returned as external io.Writer should not synthesize an io_Writer bridge:\n%s", rust)
	}
	if !strings.Contains(rust, "unimplemented!") ||
		!strings.Contains(rust, "os.File to external io.Writer requires source-mapped io") {
		t.Fatalf("package-global os.File returned as external io.Writer should be loud:\n%s", rust)
	}
}

func TestAddressOfExternalStructLocalPassedToPointerParamWrapsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "go/types"

func setUsesCgo(conf *types.Config) {}

func use() {
	conf := types.Config{}
	setUsesCgo(&conf)
}
`)

	if strings.Contains(rust, "set_uses_cgo(conf.clone())") {
		t.Fatalf("address-of external struct local should not pass the bare struct clone to a pointer parameter:\n%s", rust)
	}
	if !strings.Contains(rust, "set_uses_cgo(Rc::new(RefCell::new(Some(conf.clone()))))") &&
		!strings.Contains(rust, "set_uses_cgo(Arc::new(Mutex::new(Some(conf.clone()))))") {
		t.Fatalf("address-of external struct local should wrap the cloned value in a pointer handle:\n%s", rust)
	}
}

func TestMultiResultCallReturnConvertsStdlibInterfaceSlot(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"io"
	"os"
)

func openFile(name string) (io.ReadCloser, error) {
	return os.Open(name)
}
`)

	if strings.Contains(rust, "return os::open(") && !strings.Contains(rust, "__return_slot_0") {
		t.Fatalf("multi-result return should not return the concrete os.File slot directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let (__return_tmp_0, __return_tmp_1) = os::open") ||
		!strings.Contains(rust, "let __return_slot_0") ||
		!strings.Contains(rust, "io_ReadCloser") ||
		!strings.Contains(rust, "into()") {
		t.Fatalf("multi-result return should convert the concrete first result into io.ReadCloser:\n%s", rust)
	}
}

func TestMultiResultCallReturnSourceMappedOsToExternalReadCloserIsLoud(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"io"
	"os"
)

func openFile(name string) (io.ReadCloser, error) {
	return os.Open(name)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"os": "os"})

	if strings.Contains(rust, "return os::open(") && !strings.Contains(rust, "__return_slot_0") {
		t.Fatalf("source-mapped multi-result return should not return the concrete os.File slot directly:\n%s", rust)
	}
	if strings.Contains(rust, "io_ReadCloser::__go_from") {
		t.Fatalf("source-mapped os.File returned as external io.ReadCloser should not synthesize a bridge conversion:\n%s", rust)
	}
	if !strings.Contains(rust, "let (__return_tmp_0, __return_tmp_1) = os::open") ||
		!strings.Contains(rust, "let __return_slot_0") ||
		!strings.Contains(rust, "unimplemented!") ||
		!strings.Contains(rust, "source-mapped os.File to external io.ReadCloser requires source-mapped io") {
		t.Fatalf("source-mapped os.File returned as external io.ReadCloser should be loud:\n%s", rust)
	}
}

func TestMultiResultCallReturnConvertsFullySourceMappedStdlibInterfaceSlot(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"io"
	"os"
)

func openFile(name string) (io.ReadCloser, error) {
	return os.Open(name)
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
		"io": "io",
		"os": "os",
	})

	if strings.Contains(rust, "io_ReadCloser") {
		t.Fatalf("fully source-mapped return should not use the external io_ReadCloser bridge:\n%s", rust)
	}
	if strings.Contains(rust, "Box::new((*__return_tmp_0.") {
		t.Fatalf("fully source-mapped return should not box the cloned os.File pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(os::FilePtr(__return_tmp_0.clone())) as Box<dyn io::ReadCloser") &&
		!strings.Contains(rust, "Box::new(os::file::FilePtr(__return_tmp_0.clone())) as Box<dyn io::r#mod::ReadCloser") {
		t.Fatalf("fully source-mapped return should box os.FilePtr into io.ReadCloser:\n%s", rust)
	}
}

func TestSourceMappedReadCloserAssignmentBoxesOsFilePointer(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"io"
	"os"
)

func use(file *os.File) {
	var rc io.ReadCloser
	rc = file
	_ = rc
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
		"io": "io",
		"os": "os",
	})

	if strings.Contains(rust, "io_ReadCloser") {
		t.Fatalf("source-mapped assignment should not use the external io_ReadCloser bridge:\n%s", rust)
	}
	if strings.Contains(rust, "Box::new((*file.") {
		t.Fatalf("source-mapped assignment should not box the cloned os.File pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(os::FilePtr(file.clone())) as Box<dyn io::ReadCloser") &&
		!strings.Contains(rust, "Box::new(os::file::FilePtr(file.clone())) as Box<dyn io::r#mod::ReadCloser") {
		t.Fatalf("source-mapped assignment should box os.FilePtr into io.ReadCloser:\n%s", rust)
	}
}

func TestSyncWaitGroupAddCastsLenArgument(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "sync"

func wait(files []string) {
	var wg sync.WaitGroup
	wg.Add(len(files))
}
`)

	if strings.Contains(rust, ".add((*files.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0))") ||
		strings.Contains(rust, ".add((*files.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0))") {
		t.Fatalf("sync.WaitGroup.Add should not pass len() as usize:\n%s", rust)
	}
	if !strings.Contains(rust, ".add((*files.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)") &&
		!strings.Contains(rust, ".add((*files.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)") {
		t.Fatalf("sync.WaitGroup.Add should cast len() to Go int:\n%s", rust)
	}
}

func TestSourceMappedStringsBuilderUsesGeneratedMethods(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "strings"

func build(parts []string, sep string) string {
	var b strings.Builder
	b.WriteString(parts[0])
	b.WriteString(sep)
	return b.String()
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"strings": "strings"})

	if strings.Contains(rust, ".push_str(") {
		t.Fatalf("source-mapped strings.Builder should call generated WriteString, not Rust String::push_str:\n%s", rust)
	}
	if strings.Contains(rust, "let __builder = b.clone()") {
		t.Fatalf("source-mapped strings.Builder should call generated String, not clone the builder as a native string:\n%s", rust)
	}
	if !strings.Contains(rust, ".write_string(") {
		t.Fatalf("source-mapped strings.Builder should call the generated write_string method:\n%s", rust)
	}
	if !strings.Contains(rust, ".string()") {
		t.Fatalf("source-mapped strings.Builder should call the generated string method:\n%s", rust)
	}
}

func TestSourceMappedStringsBuilderCompositeLiteralUsesGeneratedType(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "strings"

func build() string {
	b := strings.Builder{}
	b.WriteString("x")
	return b.String()
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"strings": "strings"})

	if strings.Contains(rust, "let mut b = String::new()") {
		t.Fatalf("source-mapped strings.Builder composite literal should not use native String:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut b = strings::Builder") {
		t.Fatalf("source-mapped strings.Builder composite literal should use the generated Builder type:\n%s", rust)
	}
	if !strings.Contains(rust, ".write_string(") || !strings.Contains(rust, ".string()") {
		t.Fatalf("source-mapped strings.Builder composite literal should call generated methods:\n%s", rust)
	}
}

func TestForwardGotoPreservesPreGotoVariableScope(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func trim(s string) string {
	i := 0
	for ; i < len(s); i++ {
		if s[i] > 127 {
			goto slow
		}
	}
	return s

slow:
	return s[i:]
}
`)

	iDecl := strings.Index(rust, "let mut i =")
	label := strings.Index(rust, "'slow: {")
	if iDecl < 0 {
		t.Fatalf("forward goto fixture should declare i:\n%s", rust)
	}
	if label < 0 {
		t.Fatalf("forward goto fixture should emit the slow label block:\n%s", rust)
	}
	if iDecl > label {
		t.Fatalf("variable declared before a forward goto must stay in scope at the label:\n%s", rust)
	}
}

func TestMethodForwardGotoUsesLabelBlock(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type scanner struct {
	offset int
	src []byte
}

func (s *scanner) scan() string {
	offs := s.offset
	for i, b := range s.src[s.offset:] {
		if b == ' ' {
			s.offset += i
			goto exit
		}
	}
	s.offset = len(s.src)

exit:
	return string(s.src[offs:s.offset])
}
`)

	if strings.Contains(rust, "TODO: unsupported goto exit") {
		t.Fatalf("method body goto should be lowered through the goto planner:\n%s", rust)
	}
	if !strings.Contains(rust, "'exit: {") || !strings.Contains(rust, "break 'exit") {
		t.Fatalf("method body forward goto should lower to a labeled block:\n%s", rust)
	}
}

func TestMethodBackwardGotoAtBodyStartUsesLoopLabel(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type scanner struct {
	skip bool
	n int
}

func (s *scanner) scan() int {
scanAgain:
	if s.n < 0 {
		return 0
	}
	s.n++
	if s.skip {
		s.skip = false
		goto scanAgain
	}
	return s.n
}
`)

	if !strings.Contains(rust, "'scan_again: loop {") {
		t.Fatalf("method body backward label should emit a Rust loop label:\n%s", rust)
	}
	if !strings.Contains(rust, "continue 'scan_again") {
		t.Fatalf("method body backward goto should continue the emitted loop label:\n%s", rust)
	}
}

func TestBackwardGotoBeforeSyntheticLabelBreakIsTerminated(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func retry(ok bool) {
again:
	if ok {
		goto again
	}
}
`)

	if strings.Contains(rust, "continue 'again\n") {
		t.Fatalf("backward goto before generated label break should be terminated:\n%s", rust)
	}
	if !strings.Contains(rust, "continue 'again;") {
		t.Fatalf("backward goto should emit a terminated Rust continue:\n%s", rust)
	}
}

func TestBackwardGotoAtSyntheticLabelEndOmitsBreak(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func retry(ok bool) bool {
again:
	if ok {
		return true
	}
	goto again
}
`)

	if !strings.Contains(rust, "continue 'again;") {
		t.Fatalf("backward goto should continue the emitted loop label:\n%s", rust)
	}
	if strings.Contains(rust, "continue 'again;\n    break 'again;") ||
		strings.Contains(rust, "continue 'again;\n        break 'again;") {
		t.Fatalf("synthetic loop break should not be emitted after a terminal backward goto:\n%s", rust)
	}
}

func TestForwardGotoIntervalsNestAcrossLaterLabels(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func bits(zero, high bool) int {
	out := 0
	if zero {
		out = 1
		goto out
	}
	if high {
		goto overflow
	}
	out = 2
	goto out

overflow:
	out = 3

out:
	return out
}
`)

	outLabel := strings.Index(rust, "'out: {")
	overflowLabel := strings.Index(rust, "'overflow: {")
	breakOut := strings.Index(rust, "break 'out")
	if outLabel < 0 || overflowLabel < 0 {
		t.Fatalf("overlapping forward gotos should emit both label blocks:\n%s", rust)
	}
	if outLabel > overflowLabel {
		t.Fatalf("later out label should wrap the earlier overflow interval:\n%s", rust)
	}
	if breakOut >= 0 && outLabel > breakOut {
		t.Fatalf("break 'out should be inside an emitted out label block:\n%s", rust)
	}
}

func TestForwardGotoInsideNestedLoopUsesEnclosingLoopBodyLabel(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func find(names []string, want string) int {
	i := 0
	for {
		for _, name := range names {
			if name == want {
				goto found
			}
			i++
		}
		return -1

	found:
		return i
	}
}
`)

	if strings.Contains(rust, "TODO: unsupported goto found") {
		t.Fatalf("nested loop forward goto should be lowered by the enclosing body planner:\n%s", rust)
	}
	if !strings.Contains(rust, "'found: {") || !strings.Contains(rust, "break 'found;") {
		t.Fatalf("nested loop forward goto should break to the emitted label block:\n%s", rust)
	}
}

func TestLoopBodyGotoPlanPreservesOuterForwardGoto(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func scan(values []int, want int, stop bool) int {
	i := 0
	for i < len(values) {
		for j := 0; j < 2; j++ {
			if values[i] == want {
				goto next
			}
		}
		return i

	next:
		i++
		if stop {
			goto fallback
		}
	}
	return -1

fallback:
	return -2
}
`)

	if strings.Contains(rust, "TODO: unsupported goto next") ||
		strings.Contains(rust, "TODO: unsupported goto fallback") {
		t.Fatalf("loop body goto planner should preserve both local and enclosing labels:\n%s", rust)
	}
	if !strings.Contains(rust, "'next: {") || !strings.Contains(rust, "break 'next;") {
		t.Fatalf("loop-local forward goto should break to the emitted next label block:\n%s", rust)
	}
	if !strings.Contains(rust, "'fallback: {") || !strings.Contains(rust, "break 'fallback;") {
		t.Fatalf("enclosing forward goto should remain visible inside the loop body planner:\n%s", rust)
	}
}

func TestSwitchCaseBodyGotoPlanHandlesNestedGotos(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func scan(kind int, values []int) int {
	switch kind {
	case 1:
	reswitch:
		for _, v := range values {
			if v < 0 {
				goto reswitch
			}
			if v == 0 {
				goto done
			}
		}
		return 1

	done:
		return 2
	}
	return 3
}
`)

	if strings.Contains(rust, "TODO: unsupported goto reswitch") ||
		strings.Contains(rust, "TODO: unsupported goto done") {
		t.Fatalf("switch case body goto planner should lower nested gotos to case-local labels:\n%s", rust)
	}
	if !strings.Contains(rust, "'reswitch: loop {") || !strings.Contains(rust, "continue 'reswitch;") {
		t.Fatalf("case-local backward goto should continue the emitted reswitch loop label:\n%s", rust)
	}
	if !strings.Contains(rust, "'done: {") || !strings.Contains(rust, "break 'done;") {
		t.Fatalf("case-local forward goto should break to the emitted done label block:\n%s", rust)
	}
}

func TestMultiShortDeclLenCapWrapsGoInt(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func search(a []int) int {
	i, j := 0, len(a)
	for i < j {
		i++
	}
	return j
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
	if strings.Contains(rust, "let (mut i, mut j) = (Rc::new(RefCell::new(Some(0))), (*a.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0));") ||
		strings.Contains(rust, "let (mut i, mut j) = (Arc::new(Mutex::new(Some(0))), (*a.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0));") {
		t.Fatalf("multi-name short declaration should not leave len as bare usize:\n%s", rust)
	}
	if !strings.Contains(rust, "Rc::new(RefCell::new(Some((*a.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))") &&
		!strings.Contains(rust, "Arc::new(Mutex::new(Some((*a.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))") {
		t.Fatalf("multi-name short declaration should wrap len as Go int:\n%s", rust)
	}
}

func TestAssignedStringRangeVarSliceStaysBare(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func trim(words []string) {
	for _, lit := range words {
		lit = lit[1:]
		println(lit)
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

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "let new_val = Rc::new(RefCell::new(Some({ let __s =") ||
		strings.Contains(rust, "let new_val = Arc::new(Mutex::new(Some({ let __s =") {
		t.Fatalf("assigned string range variable should not receive a wrapped string slice:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = { let __s = lit") &&
		!strings.Contains(rust, "let new_val = { let __s = &(lit)") {
		t.Fatalf("assigned string range variable should receive a bare string slice:\n%s", rust)
	}
}

func TestStringRangeKeyOnlyUsesCharIndices(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func sumIndexes(s string) int {
	total := 0
	for i := range s {
		total += i
	}
	return total
}
`)

	if strings.Contains(rust, "for _ in") {
		t.Fatalf("key-only string range should not discard the index binding:\n%s", rust)
	}
	if !strings.Contains(rust, "for i in 0..") && !strings.Contains(rust, "for (i, _) in") {
		t.Fatalf("key-only string range should bind the byte index:\n%s", rust)
	}
	if !strings.Contains(rust, ".char_indices()") {
		t.Fatalf("key-only string range should iterate string byte indices:\n%s", rust)
	}
}

func TestAssignedStringRangeRuneUsesGoInt32Binding(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func scan(mapping func(rune) rune, s string) int {
	total := 0
	for i, c := range s {
		r := mapping(c)
		if r == c {
			total += i
		}
		if c == '\n' {
			c = ' '
		}
		c = mapping(c)
		total += int(c)
	}
	return total
}
`)

	if strings.Contains(rust, "for (i, mut c) in") {
		t.Fatalf("assigned string range rune should not bind the Rust char directly:\n%s", rust)
	}
	if !strings.Contains(rust, "for (i, __range_c) in") {
		t.Fatalf("assigned string range rune should use a temporary char binding:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut c = __range_c as i32;") {
		t.Fatalf("assigned string range rune should lower to a mutable Go int32 value:\n%s", rust)
	}
	if strings.Contains(rust, "let new_val = ' '; c = new_val;") {
		t.Fatalf("assigned string range rune should cast rune literals to Go int32:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = (' ' as i32); c = new_val;") {
		t.Fatalf("assigned string range rune should assign rune literals as i32:\n%s", rust)
	}
}

func TestStringRangeRuneAssignmentToRuneVariableCastsToGoRune(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func lastRune(s string) rune {
	var out rune
	for _, c := range s {
		out = c
	}
	return out
}
`)

	if strings.Contains(rust, "let new_val = c;") {
		t.Fatalf("assignment to rune variable should not store Rust char directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = c as i32;") {
		t.Fatalf("assignment to rune variable should cast string range char to Go rune:\n%s", rust)
	}
}

func TestStringRangeRuneAssignmentToRuneSliceElementCastsToGoRune(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func runes(s string) []rune {
	out := make([]rune, len(s))
	i := 0
	for _, c := range s {
		out[i] = c
		i++
	}
	return out
}
`)

	if strings.Contains(rust, "] = c") && !strings.Contains(rust, "] = c as i32") {
		t.Fatalf("assignment to []rune element should not store Rust char directly:\n%s", rust)
	}
	if !strings.Contains(rust, "] = c as i32") {
		t.Fatalf("assignment to []rune element should cast string range char to Go rune:\n%s", rust)
	}
}

func TestStringConstRangeUsesBareString(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func mask() int {
	const str = "abc"
	total := 0
	for i, c := range str {
		total += i + int(c)
	}
	return total
}
`)

	if strings.Contains(rust, "str.borrow()") || strings.Contains(rust, "str.lock()") {
		t.Fatalf("range over string const should not unwrap a string handle:\n%s", rust)
	}
	if !strings.Contains(rust, "str.char_indices()") {
		t.Fatalf("range over string const should use the bare string value:\n%s", rust)
	}
}

func TestIfInitShortDeclDoesNotLeakPastIf(t *testing.T) {
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
		list = append(list, x)
	}
	return append(list, x)
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
	if !strings.Contains(rust, "{\n        let (mut x, mut ok) =") {
		t.Fatalf("if init short declaration should be wrapped in its own Rust block:\n%s", rust)
	}
	if strings.Contains(rust, "return { let __append_target = list.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(Box::new((*x.borrow().as_ref().unwrap()).clone()) as Box<dyn Expr>)") ||
		strings.Contains(rust, "return { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Box::new((*x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Expr + Send + Sync>)") {
		t.Fatalf("if init short declaration should not leak the concrete x past the if block:\n%s", rust)
	}
}

func TestTypeSwitchWithoutTypeInfoEmitsUnimplemented(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func classify(v interface{}) string {
	switch v.(type) {
	default:
		return "other"
	case int:
		return "int"
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	rust, _, _ := Transpile(file, fset, nil)

	want := `unimplemented!("type info required for type switch case")`
	if !strings.Contains(rust, want) {
		t.Fatalf("type switch case without type info must emit %q per AGENTS.md \"Type Info Is Authoritative\":\n%s", want, rust)
	}
	if strings.Contains(rust, "downcast_ref::<i32>()") {
		t.Fatalf("type switch case must not synthesize an i32 downcast from syntax when type info is missing:\n%s", rust)
	}
}

func TestTypeSwitchAssignedCaseBindingIsMutable(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type BasicLit struct {
	Value string
}

func normalizedNumber(x *BasicLit) *BasicLit {
	return x
}

func print(arg any) {
	switch x := arg.(type) {
	case *BasicLit:
		x = normalizedNumber(x)
		_ = x
	}
}
`)

	if !strings.Contains(rust, "let mut x =") {
		t.Fatalf("assigned type-switch case binding should be mutable:\n%s", rust)
	}
}

func TestUnsafeSizeofWithoutTypeInfoEmitsUnimplemented(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "unsafe"

func size() uintptr {
	var ptr uintptr
	return unsafe.Sizeof(ptr)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	rust, _, _ := Transpile(file, fset, nil)

	want := `unimplemented!("type info required for unsafe.Sizeof")`
	if !strings.Contains(rust, want) {
		t.Fatalf("unsafe.Sizeof without type info must emit %q per AGENTS.md \"Type Info Is Authoritative\":\n%s", want, rust)
	}
}

func TestTrailingEmptyStmtAfterTerminatingSwitchSkipsFinalDefer(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func pick(v int) (res int) {
	if v < 0 {
		defer func() {}()
	}
	switch v {
	default:
		return 1
	case 0:
		return 0
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	fn := file.Decls[0].(*ast.FuncDecl)
	fn.Body.List = append(fn.Body.List, &ast.EmptyStmt{})

	rust, _, _ := Transpile(file, fset, nil)

	if strings.Contains(rust, "Unhandled statement type") {
		t.Fatalf("empty statement should not emit a TODO:\n%s", rust)
	}
	if got := strings.Count(rust, "while let Some(f) = __defer_stack.pop()"); got != 2 {
		t.Fatalf("terminating switch with trailing empty statement should not emit final defer drain, got %d drains:\n%s", got, rust)
	}
}

func TestExpressionSwitchOnNamedInterfaceUsesTraitEquality(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	Underlying() Type
}

type Basic struct{}

func (b *Basic) Underlying() Type {
	return b
}

var Invalid = &Basic{}

func classify(t Type) string {
	switch t {
	case nil, Invalid:
		return "invalid"
	default:
		return "other"
	}
}
`)

	if strings.Contains(rust, "_switch_val == (None)") {
		t.Fatalf("interface switch nil case should test the wrapped interface handle, not compare against None:\n%s", rust)
	}
	if strings.Contains(rust, "_switch_val == (") {
		t.Fatalf("interface switch cases should not use raw trait-object equality:\n%s", rust)
	}
	if !strings.Contains(rust, "(*_switch_val") || !strings.Contains(rust, ").is_none()") {
		t.Fatalf("interface switch nil case should test the handle none state:\n%s", rust)
	}
	if strings.Contains(rust, "Box::new((*Invalid.borrow().as_ref().unwrap()).clone())") {
		t.Fatalf("interface switch concrete case should compare the pointed-to value, not box the pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "__go_eq_type_(") {
		t.Fatalf("interface switch concrete case should use the interface equality helper:\n%s", rust)
	}
}

func TestExpressionSwitchOnEmptyInterfaceKeepsHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Code string

const Large Code = "large"

func parse() (err any) {
	defer func() {
		switch r := recover(); r {
		default:
			panic(r)
		case nil:
		case Large:
			err = r
		}
	}()
	done := make(chan bool)
	_ = done
	return nil
}
`)

	if strings.Contains(rust, "let _switch_val = (*r.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("empty-interface switch tag should keep the handle, not clone the Box<dyn Any> payload:\n%s", rust)
	}
	if !strings.Contains(rust, "let _switch_val = r.clone();") {
		t.Fatalf("empty-interface switch tag should clone the interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, "go_any_eq(&_switch_val, &__right_holder)") {
		t.Fatalf("empty-interface switch case should compare boxed payloads through go_any_eq:\n%s", rust)
	}
}

func TestExpressionSwitchOnErrorKeepsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "errors"

var ErrPermission = errors.New("permission denied")
var ErrOther = errors.New("permission denied")

func classify(target error) string {
	switch target {
	case ErrPermission:
		return "permission"
	case ErrOther:
		return "other"
	default:
		return "unknown"
	}
}
`)

	if strings.Contains(rust, "let _switch_val = (*target.borrow().as_ref().unwrap()).clone()") ||
		strings.Contains(rust, "let _switch_val = (*target.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("error switch tag should keep the handle, not clone Box<dyn Error>:\n%s", rust)
	}
	if !strings.Contains(rust, "let _switch_val = target.clone();") {
		t.Fatalf("error switch tag should clone the error handle:\n%s", rust)
	}
	if !strings.Contains(rust, "std::ptr::addr_eq(&**__left, &**__right)") {
		t.Fatalf("error switch case should compare boxed error identity:\n%s", rust)
	}
}

func TestExpressionSwitchOnPointerUsesHandleIdentity(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type mutex struct {
	key int
}

type schedt struct {
	lock mutex
}

var sched schedt

func prefer(l *mutex) bool {
	switch l {
	case &sched.lock:
		return true
	default:
		return false
	}
}
`)

	if strings.Contains(rust, "let _switch_val = (*l.borrow().as_ref().unwrap()).clone()") ||
		strings.Contains(rust, "let _switch_val = (*l.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("pointer switch tag should keep the pointer handle, not clone the pointee:\n%s", rust)
	}
	if strings.Contains(rust, "_switch_val == (") {
		t.Fatalf("pointer switch case should not use pointee value equality:\n%s", rust)
	}
	if !strings.Contains(rust, "let _switch_val = l.clone();") {
		t.Fatalf("pointer switch tag should clone the pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Rc::ptr_eq(&_switch_val, &__case)") &&
		!strings.Contains(rust, "Arc::ptr_eq(&_switch_val, &__case)") {
		t.Fatalf("pointer switch case should compare pointer handle identity:\n%s", rust)
	}
}

func TestNoTypeInfoConcurrentPointerMapCommaOkKeepsSliceHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"fmt"
	"go/types"
)

type scope struct {
	id int
}

func lookup(m map[*scope][]types.Object, s *scope) []types.Object {
	objs, ok := m[s]
	if !ok {
		objs = make([]types.Object, 1)
		m[s] = objs
	}
	return objs
}

func forceConcurrent() {
	go func() {}()
	if false {
		fmt.Println(lookup(nil, nil))
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}

	prevDetector := GetConcurrencyDetector()
	detector := NewConcurrencyDetector()
	detector.AnalyzeFile(file)
	SetConcurrencyDetector(detector)
	defer SetConcurrencyDetector(prevDetector)

	rust, _, _ := Transpile(file, fset, nil)

	if strings.Contains(rust, "type information required") || strings.Contains(rust, "Cannot determine if map") {
		t.Fatalf("map comma-ok should use syntax fallback without type info:\n%s", rust)
	}
	if !strings.Contains(rust, "match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(s.clone())))") {
		t.Fatalf("map comma-ok should use pointer-key syntax fallback:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(v) => (v.clone(),") || !strings.Contains(rust, "None => (Default::default(),") {
		t.Fatalf("map comma-ok should keep the stored slice handle:\n%s", rust)
	}
	if !strings.Contains(rust, "vec![Default::default(); (1) as usize]") {
		t.Fatalf("make([]types.Object, n) should use selector element zero value without type info:\n%s", rust)
	}
	if !strings.Contains(rust, "let __map_key = GoLocalPtrKey::new(s.clone()); let __map_value = objs.clone();") {
		t.Fatalf("map assignment should use pointer-key syntax fallback and store the slice handle:\n%s", rust)
	}
	if !strings.Contains(rust, "format_slice(&lookup(") {
		t.Fatalf("fmt.Println should use syntax-known slice return without type info:\n%s", rust)
	}
	if strings.Contains(rust, "s as usize") || strings.Contains(rust, "s) as usize") {
		t.Fatalf("map assignment fell through to slice indexing:\n%s", rust)
	}
}

func TestPointerToMapOperationsUseMapHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type node struct{}
type flag uint8

func addSpan(start, last *node, f flag, flags *map[*node]flag) {
	if *flags == nil {
		*flags = make(map[*node]flag)
	}
	(*flags)[start] = f
	(*flags)[last] |= 8
}

func main() {
	go func() {}()
}
`)

	if strings.Contains(rust, ".as_ref().unwrap()).lock()") ||
		strings.Contains(rust, ".as_mut().unwrap()).lock()") {
		t.Fatalf("pointer-to-map operations should use the map handle, not relock a raw BTreeMap:\n%s", rust)
	}
	if !strings.Contains(rust, "{ let __nil_result = (*flags.lock().unwrap()).is_none(); __nil_result }") ||
		!strings.Contains(rust, "flags.lock().unwrap().as_mut().unwrap()).insert") ||
		!strings.Contains(rust, "let mut __map_guard = flags.lock().unwrap()") {
		t.Fatalf("pointer-to-map operations should borrow through the pointer map handle:\n%s", rust)
	}
	if strings.Contains(rust, "__value.as_ref().unwrap() |") {
		t.Fatalf("map compound assignment should own the stored value before applying |:\n%s", rust)
	}
}

func TestMapNilCheckDropsGuardBeforeBodyMutation(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type node struct{}

type checker struct {
	types map[*node]int
}

func forceConcurrent() {
	go func() {}()
}

func (c *checker) record(x *node) {
	if m := c.types; m != nil {
		m[x] = 1
	}
}
`)

	if strings.Contains(rust, "if (*m.lock().unwrap()).is_some()") {
		t.Fatalf("map nil check should not keep the map guard live across body mutation:\n%s", rust)
	}
	if !strings.Contains(rust, "if { let __nil_result = (*m.lock().unwrap()).is_some(); __nil_result }") {
		t.Fatalf("map nil check should store the nil result before entering the body:\n%s", rust)
	}
	if !strings.Contains(rust, "(*m.lock().unwrap().as_mut().unwrap()).insert") {
		t.Fatalf("map assignment should still mutate the short-declared map handle:\n%s", rust)
	}
}

func TestInterfaceSelfAssignmentDropsSourceGuardBeforeTargetMutation(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Type interface {
	isType()
}

type Basic struct{}

func (*Basic) isType() {}

func Unalias(t Type) Type {
	return t
}

func forceConcurrent() {
	go func() {}()
}

func assign(T Type) Type {
	T = Unalias(T)
	return T
}
`)

	if strings.Contains(rust, "let __iface_guard = __iface_handle.lock().unwrap(); *T.lock().unwrap()") {
		t.Fatalf("interface assignment should not hold the source guard while mutating the target:\n%s", rust)
	}
	if !strings.Contains(rust, "let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *T.lock().unwrap() = __iface_value;") {
		t.Fatalf("interface assignment should clone the source value before locking the target:\n%s", rust)
	}
}

func TestStringMapAssignmentClonesRangeRefKey(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Value struct {
	Names []string
}

func collect(values []Value) map[string]bool {
	syms := map[string]bool{}
	for _, v := range values {
		for _, name := range v.Names {
			syms[name] = true
		}
	}
	return syms
}
`)

	if strings.Contains(rust, "let __map_key = name;") {
		t.Fatalf("string map assignment should clone a range ref key, not store &String:\n%s", rust)
	}
	if !strings.Contains(rust, "let __map_key = (*name).clone();") {
		t.Fatalf("string map assignment should clone the range ref key:\n%s", rust)
	}
}

func TestRangeValueShadowingPackageGlobalUsesLocalBinding(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

var env = map[string]int{}
var envs []string

func Environ() []string {
	out := make([]string, 0, len(envs))
	for _, env := range envs {
		if env != "" {
			out = append(out, env)
		}
	}
	return out
}
`)

	if strings.Contains(rust, "for env in ") {
		t.Fatalf("range value shadowing package global should not bind the package-global name:\n%s", rust)
	}
	if !strings.Contains(rust, "for env_local in ") {
		t.Fatalf("range value shadowing package global should bind a local name:\n%s", rust)
	}
	if !strings.Contains(rust, "(*env_local).clone()") {
		t.Fatalf("range value shadowing package global should read the local binding:\n%s", rust)
	}
}

func TestInterfaceMapRangeKeyUsesStoredInterfaceHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Node interface {
	Pos() int
}

type CommentMap map[Node]int

func First(cmap CommentMap) int {
	for node := range cmap {
		return cmap[node]
	}
	return 0
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

	if !strings.Contains(rust, "let node = __range_key.value();") {
		t.Fatalf("interface map range key should recover the stored interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".get(&GoLocalPtrKey::new(node.clone()))") {
		t.Fatalf("interface map lookup with range key should rebuild GoLocalPtrKey from the handle:\n%s", rust)
	}
	if strings.Contains(rust, "node.__go_clone_box_node()") {
		t.Fatalf("interface map range key should not treat GoLocalPtrKey itself as the interface value:\n%s", rust)
	}
}

func TestPackageGlobalNamedMapLiteralAssignmentStoresNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type buckets map[int]int

var global buckets

func init() {
	global = buckets{1: 2}
}
`)

	if strings.Contains(rust, "Some(buckets {") {
		t.Fatalf("named map literal should not be emitted as a struct literal:\n%s", rust)
	}
	if strings.Contains(rust, "__collection_holder = buckets(") {
		t.Fatalf("package-global named map assignment should store the named value, not borrow it as a map handle:\n%s", rust)
	}
	if strings.Contains(rust, "GoGlobal<BTreeMap<") {
		t.Fatalf("package-global named map slot should keep the named type:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(buckets(") {
		t.Fatalf("package-global named map assignment should store an optional named map value:\n%s", rust)
	}
}

func TestPointerReceiverNamedMapNilMakeAssignmentUsesNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node struct{}

type nodeSet map[*Node]bool

func (s *nodeSet) add(p *Node) {
	if *s == nil {
		*s = make(nodeSet)
	}
	(*s)[p] = true
}
`)

	if strings.Contains(rust, "(*(*self.0.borrow_mut().unwrap()).borrow()).is_none()") ||
		strings.Contains(rust, "(*(*self.0.lock().unwrap().as_mut().unwrap()).lock().unwrap()).is_none()") {
		t.Fatalf("named map nil comparison should inspect the named map handle, not the raw map value:\n%s", rust)
	}
	if !strings.Contains(rust, "let __map_holder = self.0.clone(); let __map_guard = __map_holder.borrow(); (*__map_guard).is_none()") &&
		!strings.Contains(rust, "let __map_holder = self.0.clone(); let __map_guard = __map_holder.lock().unwrap(); (*__map_guard).is_none()") {
		t.Fatalf("named map nil comparison should borrow the inner map handle:\n%s", rust)
	}
	if strings.Contains(rust, "*self = new_val;") {
		t.Fatalf("pointer receiver named map assignment should store the named value, not the wrapped slot:\n%s", rust)
	}
	if !strings.Contains(rust, "*self = new_val.borrow_mut().take().unwrap_or_default();") &&
		!strings.Contains(rust, "*self = new_val.lock().unwrap().take().unwrap_or_default();") {
		t.Fatalf("pointer receiver named map assignment should unwrap make(nodeSet) to a named map value:\n%s", rust)
	}
}

func TestPointerToLocalInterfaceNilComparisonChecksInterfaceSlot(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	Underlying() Type
}

func has(h *Type) bool {
	return *h != nil
}
`)

	if strings.Contains(rust, "as_mut().unwrap()).borrow") ||
		strings.Contains(rust, "as_mut().unwrap()).lock") {
		t.Fatalf("pointer-to-interface nil comparison should not borrow a bare trait object as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __iface_handle = h.clone(); let __iface_guard = __iface_handle.borrow(); (*__iface_guard).is_some()") &&
		!strings.Contains(rust, "let __iface_handle = h.clone(); let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some()") {
		t.Fatalf("pointer-to-interface nil comparison should inspect the pointed-to interface slot:\n%s", rust)
	}
}

func TestPointerToLocalInterfaceReturnClonesInterfaceHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	Underlying() Type
}

func get(h *Type) Type {
	return *h
}
`)

	if strings.Contains(rust, "(*h.borrow_mut().as_mut().unwrap()).clone()") ||
		strings.Contains(rust, "(*h.lock().unwrap().as_mut().unwrap()).clone()") {
		t.Fatalf("pointer-to-interface return should not return the bare trait object:\n%s", rust)
	}
	if !strings.Contains(rust, "h.clone()") {
		t.Fatalf("pointer-to-interface return should clone the interface handle:\n%s", rust)
	}
}

func TestPointerReceiverNamedMapAssignmentFromUnnamedMakeWrapsNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Object interface {
	Id() string
}

type objset map[string]Object

func (s *objset) init() {
	if *s == nil {
		*s = make(map[string]Object)
	}
}
`)

	if strings.Contains(rust, "*self = new_val.borrow_mut().take().unwrap_or_default();") ||
		strings.Contains(rust, "*self = new_val.lock().unwrap().take().unwrap_or_default();") {
		t.Fatalf("pointer receiver named map assignment from unnamed make should not store the raw map value:\n%s", rust)
	}
	if strings.Contains(rust, "*self = objset(new_val.borrow_mut().take().unwrap_or_default());") ||
		strings.Contains(rust, "*self = objset(new_val.lock().unwrap().take().unwrap_or_default());") {
		t.Fatalf("pointer receiver named map assignment from unnamed make should not pass the raw map into the named type:\n%s", rust)
	}
	if !strings.Contains(rust, "*self = objset(new_val);") {
		t.Fatalf("pointer receiver named map assignment from unnamed make should wrap the map handle in the named type:\n%s", rust)
	}
}

func TestValueReceiverNamedMapAssignmentUpdatesLocalCopy(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type methodSet map[string]int

func (s methodSet) addOne(key string) methodSet {
	return s
}

func (s methodSet) add(list []string) methodSet {
	for _, key := range list {
		s = s.addOne(key)
	}
	return s
}
`)

	if strings.Contains(rust, "__self.0 = new_val") {
		t.Fatalf("named map value receiver assignment should not store the returned handle in the receiver map field:\n%s", rust)
	}
	if !strings.Contains(rust, "__self = __moved_val") {
		t.Fatalf("named map value receiver assignment should replace the mutable receiver copy:\n%s", rust)
	}
}

func TestAppendToNamedMapValueUsesInnerMapHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type entry struct {
	Pos int
}

type valueMap map[int][]entry

func add(seen valueMap, key int, pos int) {
	seen[key] = append(seen[key], entry{Pos: pos})
}
`)

	if strings.Contains(rust, "let __slice = { let __map_holder = seen.clone();") {
		t.Fatalf("append to a named map value should borrow the named map inner handle, not the named value slot:\n%s", rust)
	}
	if !strings.Contains(rust, "let __named_map = (*seen.borrow().as_ref().unwrap()).0.clone(); __named_map") &&
		!strings.Contains(rust, "let __named_map = (*seen.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map") {
		t.Fatalf("append to a named map value should unwrap the named map handle:\n%s", rust)
	}
}

func TestRangeOverNilNamedMapFieldDoesNotUnwrapNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type namedMap map[string]int

type holder struct {
	values namedMap
}

func count(h holder) int {
	total := 0
	for k := range h.values {
		_ = k
		total++
	}
	return total
}
`)

	if strings.Contains(rust, "let __named_map = (*") &&
		strings.Contains(rust, ".as_ref().unwrap()).0.clone(); __named_map") {
		t.Fatalf("range over a nil named-map field should not unwrap the named value before map range defaulting:\n%s", rust)
	}
	if !strings.Contains(rust, "__named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else") {
		t.Fatalf("range over a named-map field should tolerate a nil named value slot:\n%s", rust)
	}
	if !strings.Contains(rust, "Rc::new(RefCell::new(None))") &&
		!strings.Contains(rust, "Arc::new(Mutex::new(None))") {
		t.Fatalf("range over a nil named-map field should materialize a nil inner map handle:\n%s", rust)
	}
}

func TestMapPointerValueNilAssignmentStoresNilHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Selection struct{}
type methodSet map[string]*Selection

func assign(s methodSet, key string) methodSet {
	s[key] = nil
	return s
}
`)

	if strings.Contains(rust, "Some(None)") {
		t.Fatalf("nil assigned to a pointer map value should store a nil handle, not Some(None):\n%s", rust)
	}
	if !strings.Contains(rust, "let __map_value = Rc::new(RefCell::new(None))") &&
		!strings.Contains(rust, "let __map_value = Arc::new(Mutex::new(None))") {
		t.Fatalf("nil assigned to a pointer map value should create an empty handle:\n%s", rust)
	}
}

func TestLocalInterfaceAssignmentCopiesWrappedHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Node interface {
	Pos() int
}

func assignFromRange(nodes []Node) Node {
	var p Node
	for _, q := range nodes {
		p = q
	}
	return p
}

func assignFromIndex(nodes []Node) Node {
	var top Node
	top = nodes[0]
	return top
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

	if strings.Contains(rust, "*p.borrow_mut().as_mut().unwrap() = Some(new_val)") ||
		strings.Contains(rust, "*p.lock().unwrap() = Some(new_val)") ||
		strings.Contains(rust, "*top.borrow_mut().as_mut().unwrap() = Some(new_val)") ||
		strings.Contains(rust, "*top.lock().unwrap() = Some(new_val)") {
		t.Fatalf("assignment between local interface handles should replace the handle, not store a handle inside Some:\n%s", rust)
	}
	if !strings.Contains(rust, "let __iface_handle = q.clone(); let __iface_guard = __iface_handle.borrow(); *p.borrow_mut() = (*__iface_guard).clone()") &&
		!strings.Contains(rust, "let __iface_handle = q.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *p.lock().unwrap() = (*__iface_guard).clone()") {
		t.Fatalf("range assignment should copy the local interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __iface_handle = (*nodes.borrow().as_ref().unwrap())[(0) as usize].clone().clone(); let __iface_guard = __iface_handle.borrow(); *top.borrow_mut() = (*__iface_guard).clone()") &&
		!strings.Contains(rust, "let __iface_handle = (*nodes.lock().unwrap().as_ref().unwrap())[(0) as usize].clone().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *top.lock().unwrap() = (*__iface_guard).clone()") &&
		!strings.Contains(rust, "top = { let __seq =") {
		t.Fatalf("index assignment should replace the local interface handle from the slice element:\n%s", rust)
	}
}

func TestLocalInterfaceVarInitializerConvertsStructuralInterface(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type positioner interface {
	Pos() int
}

type Expr interface {
	Pos() int
	End() int
}

type Field struct {
	X Expr
}

func use(f *Field) positioner {
	var at positioner = f.X
	return at
}
`)

	if strings.Contains(rust, "Some((*{ let __field =") {
		t.Fatalf("structural interface var initializer should not store the source trait object directly:\n%s", rust)
	}
	if !strings.Contains(rust, "impl positioner for Box<dyn Expr") {
		t.Fatalf("structural interface var initializer should emit a boxed trait-object adapter:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new((*(*f.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()).clone()) as Box<dyn positioner") &&
		!strings.Contains(rust, "Box::new((*(*f.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner") {
		t.Fatalf("structural interface var initializer should convert through the adapter:\n%s", rust)
	}
}

func TestLocalInterfaceRangeVarAssignmentFromCallKeepsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Object interface {
	Name() string
}

func resolve(name string, obj Object) Object {
	return obj
}

func use(all map[string]Object) {
	for name, obj := range all {
		obj = resolve(name, obj)
		_ = obj
	}
}
`)

	if strings.Contains(rust, "let new_val = (*resolve(") {
		t.Fatalf("interface range var assignment from call should not unwrap the returned handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __iface_handle = resolve(") ||
		(!strings.Contains(rust, "*obj.borrow_mut() = (*__iface_guard).clone();") &&
			!strings.Contains(rust, "*obj.lock().unwrap() = (*__iface_guard).clone();")) {
		t.Fatalf("interface range var assignment from call should copy the returned interface value into the existing slot:\n%s", rust)
	}
}

func TestParallelLocalInterfaceAssignmentCopiesHandles(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	isType()
}

type Basic struct{}
func (*Basic) isType() {}

func assign(k Type, e Type) (Type, Type) {
	var key Type
	var elem Type
	key, elem = k, e
	return key, elem
}
`)

	if strings.Contains(rust, "let __tmp_0 = (*k.borrow().as_ref().unwrap()).clone()") ||
		strings.Contains(rust, "let __tmp_0 = (*k.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("parallel interface assignment temp should copy the interface handle, not the boxed value:\n%s", rust)
	}
	if !strings.Contains(rust, "let __tmp_0 = k.clone();") ||
		!strings.Contains(rust, "let __iface_handle = __tmp_0;") ||
		!strings.Contains(rust, "let __iface_handle = __tmp_1;") ||
		(!strings.Contains(rust, "*key.borrow_mut() = __iface_value;") &&
			!strings.Contains(rust, "*key.lock().unwrap() = __iface_value;")) ||
		(!strings.Contains(rust, "*elem.borrow_mut() = __iface_value;") &&
			!strings.Contains(rust, "*elem.lock().unwrap() = __iface_value;")) {
		t.Fatalf("parallel interface assignment should copy temp interface values into existing slots:\n%s", rust)
	}
}

func TestClosureCaptureSeesReboundInterfaceAssignment(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Expr interface {
	exprNode()
}

type Ident struct{}

func (*Ident) exprNode() {}

func grouped() int {
	var typ Expr
	var source Expr
	add := func() int {
		if typ == nil {
			return -1
		}
		return 1
	}
	source = &Ident{}
	typ = source
	return add()
}
`)

	if !strings.Contains(rust, "let typ_closure_clone = typ.clone();") {
		t.Fatalf("closure should capture the interface variable slot:\n%s", rust)
	}
	if strings.Contains(rust, "typ = source.clone()") ||
		strings.Contains(rust, "typ = Rc::new") {
		t.Fatalf("interface assignment after closure creation should not replace the captured slot:\n%s", rust)
	}
	if !strings.Contains(rust, "let __iface_handle = source.clone();") ||
		(!strings.Contains(rust, "*typ.borrow_mut() = (*__iface_guard).clone();") &&
			!strings.Contains(rust, "*typ.lock().unwrap() = (*__iface_guard).clone();")) {
		t.Fatalf("interface assignment after closure creation should copy into the existing slot:\n%s", rust)
	}
}

func TestParallelNilAssignmentClearsNilableHandles(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Object interface {
	object()
}

func reset() (obj Object, index []int, indirect bool) {
	obj, index, indirect = nil, nil, false
	return
}
`)

	if strings.Contains(rust, "*obj.borrow_mut() = Some(__tmp_0);") ||
		strings.Contains(rust, "*obj.lock().unwrap() = Some(__tmp_0);") {
		t.Fatalf("parallel nil assignment to interface result should clear the handle slot, not store Some(None):\n%s", rust)
	}
	if strings.Contains(rust, "*index.borrow_mut() = Some(__tmp_1);") ||
		strings.Contains(rust, "*index.lock().unwrap() = Some(__tmp_1);") {
		t.Fatalf("parallel nil assignment to slice result should clear the handle slot, not store Some(None):\n%s", rust)
	}
	if !strings.Contains(rust, "*obj.borrow_mut() = __tmp_0;") &&
		!strings.Contains(rust, "*obj.lock().unwrap() = __tmp_0;") {
		t.Fatalf("parallel nil assignment to interface result should move the nil temp directly into the slot:\n%s", rust)
	}
	if !strings.Contains(rust, "*index.borrow_mut() = __tmp_1;") &&
		!strings.Contains(rust, "*index.lock().unwrap() = __tmp_1;") {
		t.Fatalf("parallel nil assignment to slice result should move the nil temp directly into the slot:\n%s", rust)
	}
}

func TestParallelSlicePointerElementAssignmentUsesHandleTemps(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node struct {
	index int
}

type nodeQueue []*Node

func (a nodeQueue) Swap(i, j int) {
	x, y := a[i], a[j]
	a[i], a[j] = y, x
	x.index, y.index = j, i
}
`)

	if strings.Contains(rust, "let __tmp_0 = (*y.borrow().as_ref().unwrap()).clone()") ||
		strings.Contains(rust, "let __tmp_0 = (*y.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("parallel pointer slice assignment temp should copy the pointer handle, not the pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "let __tmp_0 = y.clone();") ||
		!strings.Contains(rust, "let __tmp_1 = x.clone();") {
		t.Fatalf("parallel pointer slice assignment should capture pointer handles:\n%s", rust)
	}
}

func TestParallelPointerSliceElementSwapKeepsHandles(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node struct {
	index int
}

type nodeQueue []*Node

func (a nodeQueue) Swap(i, j int) {
	a[i], a[j] = a[j], a[i]
}
`)

	if strings.Contains(rust, ".take().unwrap_or_default()") {
		t.Fatalf("parallel pointer slice element swap should keep pointer handles, not move pointees:\n%s", rust)
	}
	if !strings.Contains(rust, "] = __tmp_0;") || !strings.Contains(rust, "] = __tmp_1;") {
		t.Fatalf("parallel pointer slice element swap should assign pointer handles back into slice slots:\n%s", rust)
	}
}

func TestTupleSliceElementSliceAssignmentMovesInnerValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func merge(xs []int, ns []uint32) ([]int, []uint32) {
	return xs, ns
}

func reset(chunks [][]int, ns []uint32) {
	chunks[0], ns = merge(chunks[1], ns)
	_ = ns
}
`)

	if strings.Contains(rust, "] = new_val;") {
		t.Fatalf("slice element assignment should not store a wrapped slice handle in a raw Vec slot:\n%s", rust)
	}
	if !strings.Contains(rust, ".take().unwrap_or_default()") {
		t.Fatalf("slice element assignment should move the wrapped slice inner value into the raw Vec slot:\n%s", rust)
	}
}

func TestSliceElementSliceLiteralAssignmentMovesInnerValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func reset(chunks [][]int) {
	chunks[0] = []int{}
}
`)

	if strings.Contains(rust, "] = Rc::new(") || strings.Contains(rust, "] = Arc::new(") {
		t.Fatalf("slice element assignment should not store a wrapped slice literal in a raw Vec slot:\n%s", rust)
	}
	if !strings.Contains(rust, ".as_ref().unwrap()).clone()") {
		t.Fatalf("slice element assignment should unwrap the slice literal before storing it:\n%s", rust)
	}
}

func TestIndexedStructSliceFieldAssignmentMutatesElement(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type inst struct {
	rune []int
}

func apply(insts []inst, chunks [][]int) {
	for i := range insts {
		insts[i].rune = chunks[i]
	}
}
`)

	if strings.Contains(rust, ".clone() }.rune = new_val") {
		t.Fatalf("indexed struct field assignment should not assign to a cloned element:\n%s", rust)
	}
	if !strings.Contains(rust, "].rune.borrow_mut() = Some(new_val)") &&
		!strings.Contains(rust, "].rune.lock().unwrap() = Some(new_val)") {
		t.Fatalf("indexed struct field assignment should mutate the field handle on the indexed element:\n%s", rust)
	}
}

func TestIndexedStructSliceFieldNilAssignmentClearsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type inst struct {
	next []uint32
}

func clear(insts []inst, i int) {
	insts[i].next = nil
}
`)

	if strings.Contains(rust, "Some(None)") {
		t.Fatalf("nil slice field assignment should clear the field handle, not store Some(None):\n%s", rust)
	}
	if !strings.Contains(rust, "].next.borrow_mut() = None") &&
		!strings.Contains(rust, "].next.lock().unwrap() = None") {
		t.Fatalf("nil slice field assignment should clear the indexed element field handle:\n%s", rust)
	}
}

func TestIndexedPointerStructSliceFieldAssignmentMutatesPointee(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type inst struct {
	errors []string
	names []string
}

type response struct {
	items []*inst
}

func repair(r *response, path string) {
	r.items[0].errors = nil
	r.items[0].names = []string{path}
}
`)

	if strings.Contains(rust, "[(0) as usize].errors") || strings.Contains(rust, "[(0) as usize].names") {
		t.Fatalf("indexed pointer element field assignment should dereference the pointee before field access:\n%s", rust)
	}
	if !strings.Contains(rust, "[(0) as usize].borrow_mut().as_mut().unwrap()).errors.borrow_mut() = None") &&
		!strings.Contains(rust, "[(0) as usize].lock().unwrap().as_mut().unwrap()).errors.lock().unwrap() = None") {
		t.Fatalf("nil slice field assignment should clear the pointee field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "[(0) as usize].borrow_mut().as_mut().unwrap()).names.borrow_mut() = Some(new_val)") &&
		!strings.Contains(rust, "[(0) as usize].lock().unwrap().as_mut().unwrap()).names.lock().unwrap() = Some(new_val)") {
		t.Fatalf("slice field assignment should mutate the pointee field handle:\n%s", rust)
	}
}

func TestIndexedStructPromotedSliceFieldAssignmentMutatesElement(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type embedded struct {
	rune []int
}

type inst struct {
	embedded
}

func apply(insts []inst, chunks [][]int) {
	for i := range insts {
		insts[i].rune = chunks[i]
	}
}
`)

	if strings.Contains(rust, ".clone() }.rune = new_val") {
		t.Fatalf("promoted indexed struct field assignment should not assign to a cloned element:\n%s", rust)
	}
	if !strings.Contains(rust, ".embedded.borrow().as_ref().unwrap()).rune.borrow_mut() = Some(new_val)") &&
		!strings.Contains(rust, ".embedded.lock().unwrap().as_ref().unwrap()).rune.lock().unwrap() = Some(new_val)") {
		t.Fatalf("promoted indexed struct field assignment should mutate the embedded field handle:\n%s", rust)
	}
}

func TestMapInterfaceValueAssignmentKeepsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	typ()
}

type Expr interface {
	expr()
}

type basic struct{}
func (*basic) typ() {}

type ident struct{}
func (*ident) expr() {}

func remember(seen map[Type]Expr, typ Type, nodes []Expr) {
	for _, node := range nodes {
		seen[typ] = node
	}
}
`)

	if strings.Contains(rust, "Some((*node).clone())") {
		t.Fatalf("interface map value assignment should not wrap an existing interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __map_value = node.clone();") &&
		!strings.Contains(rust, "let __map_value = (*node).clone();") {
		t.Fatalf("interface map value assignment should store the existing interface handle:\n%s", rust)
	}
}

func TestReturnLocalInterfaceMapValueKeepsHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Object interface {
	Pos() int
}

type Ident struct {
	p int
}

func (i Ident) Pos() int {
	return i.p
}

type Info struct {
	Uses map[*Ident]Object
}

func (info *Info) ObjectOf(id *Ident) Object {
	return info.Uses[id]
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
	if strings.Contains(rust, "Some({ let __map =") {
		t.Fatalf("returning a local-interface map value should not wrap the handle inside Some:\n%s", rust)
	}
	if !strings.Contains(rust, "get(&GoLocalPtrKey::new(id.clone()))") &&
		!strings.Contains(rust, "get(&GoLocalPtrKey::new(id.clone().clone()))") {
		t.Fatalf("returning a local-interface map value should return the map value handle directly:\n%s", rust)
	}
}

func TestSourceMappedImportedMapFieldLookupQualifiesPointerKeyHelper(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"go/ast"
	"go/types"
)

func usedIdent(e ast.Expr) *ast.Ident {
	return nil
}

func Callee(info *types.Info, call *ast.CallExpr) types.Object {
	return info.Uses[usedIdent(call.Fun)]
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"go/ast":   "go_ast",
		"go/types": "go_types",
	})
	if strings.Contains(rust, ".get(&GoLocalPtrKey::new(") {
		t.Fatalf("source-mapped map field lookup should not use the current crate pointer-key helper:\n%s", rust)
	}
	if !strings.Contains(rust, ".get(&go_types::GoLocalPtrKey::new(used_ident(") {
		t.Fatalf("source-mapped map field lookup should use the owning package pointer-key helper:\n%s", rust)
	}
}

func TestSourceMappedImportedStructMapFieldLiteralQualifiesPointerKeyHelper(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"go/ast"
	"go/types"
)

func NewInfo() *types.Info {
	return &types.Info{
		Uses: map[*ast.Ident]types.Object{},
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

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"go/ast":   "go_ast",
		"go/types": "go_types",
	})
	if strings.Contains(rust, "BTreeMap::<GoLocalPtrKey<go_ast::Ident>") {
		t.Fatalf("source-mapped struct map field literal should not use the current crate pointer-key type:\n%s", rust)
	}
	if !strings.Contains(rust, "BTreeMap::<go_types::GoLocalPtrKey<go_ast::Ident>") {
		t.Fatalf("source-mapped struct map field literal should use the owning package pointer-key type:\n%s", rust)
	}
}

func TestSourceMappedImportedTypesInfoMakeFieldQualifiesPointerKeyHelpers(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"go/ast"
	"go/types"
)

func NewInfo() *types.Info {
	return &types.Info{
		Types:        make(map[ast.Expr]types.TypeAndValue),
		Defs:         make(map[*ast.Ident]types.Object),
		Uses:         make(map[*ast.Ident]types.Object),
		Implicits:    make(map[ast.Node]types.Object),
		Instances:    make(map[*ast.Ident]types.Instance),
		Scopes:       make(map[ast.Node]*types.Scope),
		Selections:   make(map[*ast.SelectorExpr]*types.Selection),
		FileVersions: make(map[*ast.File]string),
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

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"go/ast":   "go_ast",
		"go/types": "go_types",
	})
	if strings.Contains(rust, "BTreeMap::<GoLocalPtrKey<") {
		t.Fatalf("source-mapped types.Info make field values should not use current crate pointer-key types:\n%s", rust)
	}
	for _, want := range []string{
		"BTreeMap::<go_types::GoLocalPtrKey<Box<dyn go_ast::Expr",
		"BTreeMap::<go_types::GoLocalPtrKey<go_ast::Ident>",
		"BTreeMap::<go_types::GoLocalPtrKey<Box<dyn go_ast::Node",
		"BTreeMap::<go_types::GoLocalPtrKey<go_ast::SelectorExpr>",
		"BTreeMap::<go_types::GoLocalPtrKey<go_ast::File>",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("source-mapped types.Info make field value missing %q:\n%s", want, rust)
		}
	}
}

func TestStubBackedTypesInfoSourceMappedMapFieldsUseErasedPointerKey(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"go/ast"
	"go/types"
)

func Version(info *types.Info, file *ast.File, node ast.Node) string {
	_ = info.Implicits[node]
	return info.FileVersions[file]
}

func NewInfo(file *ast.File) *types.Info {
	return &types.Info{
		FileVersions: map[*ast.File]string{file: "go1.22"},
		Implicits: map[ast.Node]types.Object{},
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

	mapping := map[string]string{"go/ast": "go_ast"}
	packageState := NewPackageState()
	prevCtx := GetTranspileContext()
	SetTranspileContext(&TranspileContext{
		Session:                 NewTranspileSession(typeInfo, mapping),
		Package:                 packageState,
		PackageMapping:          mapping,
		UsePackageExternalStubs: true,
	})
	defer SetTranspileContext(prevCtx)

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, mapping)
	stubs := GeneratePackageExternalStubs(packageState)
	helpers := packageState.Helpers.GenerateSharedStdlibHelperModule()
	combined := rust + "\n" + stubs + "\n" + helpers

	if strings.Contains(combined, "GoLocalPtrKey<go_ast") || strings.Contains(combined, "GoLocalPtrKey<Box<dyn go_ast") {
		t.Fatalf("stub-backed source-mapped map fields should not embed source crate types in shared pointer keys:\n%s", combined)
	}
	for _, want := range []string{
		"BTreeMap::<GoAnyPtrKey,",
		"get(&GoAnyPtrKey::new(file.clone()))",
		"pub file_versions:",
		"BTreeMap<GoAnyPtrKey,",
		"pub struct GoAnyPtrKey",
	} {
		if !strings.Contains(combined, want) {
			t.Fatalf("stub-backed source-mapped map field output missing %q:\n%s", want, combined)
		}
	}
}

func TestSourceMappedInterfaceMapAssignmentBoxesConcreteKey(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/types"

func mark(value any) map[types.Object]bool {
	localObjects := make(map[types.Object]bool)
	if obj, ok := value.(*types.Var); ok {
		localObjects[obj] = true
	}
	return localObjects
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/types": "go_types"})
	if strings.Contains(rust, "let __map_key = GoLocalPtrKey::new(obj.clone());") {
		t.Fatalf("interface map assignment should not use concrete pointer key directly:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(go_types::VarPtr(obj.clone())) as Box<dyn go_types::Object") {
		t.Fatalf("interface map assignment should box a pointer-identity wrapper as the expected interface:\n%s", rust)
	}
}

func TestSourceMappedImportedInterfaceFieldFromFunctionTypeAliasUsesWrapper(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/types"

type importerFunc func(path string) (*types.Package, error)

func (f importerFunc) Import(path string) (*types.Package, error) {
	return f(path)
}

func NewConfig(importer importerFunc) *types.Config {
	return &types.Config{
		Importer: importer,
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

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/types": "go_types"})
	if strings.Contains(rust, "Box::new((*importer.") {
		t.Fatalf("function-type alias should not clone the inner boxed closure for imported interface fields:\n%s", rust)
	}
	if !strings.Contains(rust, "impl go_types::Importer for importerFuncAsgo_types_Importer") {
		t.Fatalf("function-type alias should emit a wrapper impl for the imported interface:\n%s", rust)
	}
	if strings.Contains(rust, "Option<Rc<RefCell<Option<go_types::Package>>>>") {
		t.Fatalf("function-type alias wrapper should not double-wrap imported interface pointer returns:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(importerFuncAsgo_types_Importer(importer.clone())) as Box<dyn go_types::Importer") {
		t.Fatalf("function-type alias field should box the wrapper around the function handle:\n%s", rust)
	}
}

func TestSourceMappedPointerCompositeAssignmentToAnyBoxesConcreteValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"go/ast"
	"go/printer"
)

func BoxCommented(file *ast.File, comments []*ast.CommentGroup) any {
	var node any
	node = &printer.CommentedNode{Node: file, Comments: comments}
	return node
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{
		"go/ast":     "go_ast",
		"go/printer": "go_printer",
	})
	if strings.Contains(rust, "Option<go_printer::CommentedNode>") {
		t.Fatalf("assignment to any should not store the concrete source-mapped struct slot directly:\n%s", rust)
	}
	if strings.Contains(rust, "__moved_val") {
		t.Fatalf("assignment to any should not move the concrete source-mapped struct option into the any slot:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(") || !strings.Contains(rust, "as Box<dyn Any") {
		t.Fatalf("assignment to any should box the source-mapped pointer composite value:\n%s", rust)
	}
}

func TestSourceMappedImportedFunctionTypeAliasParamUsesAliasHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/types"

func Identity(qual types.Qualifier) types.Qualifier {
	return qual
}

func Call(qual types.Qualifier, pkg *types.Package) string {
	return qual(pkg)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/types": "go_types"})
	if strings.Contains(rust, "Option<go_types::Qualifier>") {
		t.Fatalf("source-mapped function type alias should not be wrapped again:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn identity(qual: go_types::Qualifier) -> go_types::Qualifier") {
		t.Fatalf("source-mapped function type alias param/result should use the imported alias handle:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn call(qual: go_types::Qualifier, pkg: Rc<RefCell<Option<go_types::Package>>>)") {
		t.Fatalf("source-mapped function type alias param should use the imported alias handle:\n%s", rust)
	}
}

func TestSourceMappedImportedFunctionTypeAliasStructFieldUsesAliasHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/types"

type Printer struct {
	qual types.Qualifier
}

func NewPrinter(qual types.Qualifier) *Printer {
	return &Printer{
		qual: qual,
	}
}

func (p *Printer) Qualify(pkg *types.Package) string {
	return p.qual(pkg)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/types": "go_types"})
	if strings.Contains(rust, "Option<go_types::Qualifier>") {
		t.Fatalf("source-mapped function type alias field should not be wrapped again:\n%s", rust)
	}
	if !strings.Contains(rust, "pub qual: go_types::Qualifier,") {
		t.Fatalf("source-mapped function type alias field should use the imported alias handle:\n%s", rust)
	}
	if !strings.Contains(rust, "qual: qual.clone()") {
		t.Fatalf("source-mapped function type alias field initializer should clone the imported alias handle:\n%s", rust)
	}
}

func TestConcretePointerMapLookupKeyForLocalInterfaceKeyBoxesValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Node interface {
	Pos() int
}

type ImportSpec struct {
	p int
}

func (s ImportSpec) Pos() int {
	return s.p
}

type Info struct {
	Implicits map[Node]int
}

func (info *Info) Value(imp *ImportSpec) int {
	return info.Implicits[imp]
}

func (info *Info) Set(imp *ImportSpec) {
	info.Implicits[imp] = 7
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
	if strings.Contains(rust, "GoLocalPtrKey::new(imp.clone())") {
		t.Fatalf("concrete pointer key for local-interface map should not use the concrete handle directly:\n%s", rust)
	}
	if strings.Contains(rust, "GoPtrKey::new(imp.clone())") {
		t.Fatalf("concrete pointer key for local-interface map should not use a concrete pointer key:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(ImportSpecPtr(imp.clone())) as Box<dyn Node>") {
		t.Fatalf("concrete pointer key for local-interface map should box a pointer-identity wrapper as the interface:\n%s", rust)
	}
}

func TestSubinterfaceMapLookupKeyConvertsToExpectedInterface(t *testing.T) {
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

type Info struct {
	Objects map[Object]int
}

type declInfo struct {
	deps map[Object]bool
}

func lookup(info *Info, d dependency) int {
	return info.Objects[d]
}

func lookupParam(objMap map[Object]*declInfo, obj dependency) bool {
	return objMap[obj].deps[obj]
}
`)

	if strings.Contains(rust, "GoLocalPtrKey::new(d.clone())") {
		t.Fatalf("subinterface map lookup key should not pass the subinterface handle directly:\n%s", rust)
	}
	if strings.Contains(rust, "GoLocalPtrKey::new(obj.clone())") {
		t.Fatalf("subinterface parameter-map lookup key should not pass the subinterface handle directly:\n%s", rust)
	}
	if !strings.Contains(rust, "as Box<dyn Object") {
		t.Fatalf("subinterface map lookup key should convert to the expected interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __inner: Box<dyn Object") {
		t.Fatalf("subinterface map lookup key should use the typed embedded-interface upcast:\n%s", rust)
	}
}

func TestStructuralInterfaceMapLookupKeyUsesAdapter(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type positioner interface {
	Pos() int
}

type object interface {
	Pos() int
	Name() string
}

func lookup(m map[positioner]int, obj object) int {
	return m[obj]
}
`)

	if strings.Contains(rust, "let __inner: Box<dyn positioner") {
		t.Fatalf("structural interface map lookup should not use Rust trait upcasting:\n%s", rust)
	}
	if !strings.Contains(rust, "impl positioner for Box<dyn object") {
		t.Fatalf("structural interface map lookup should emit a boxed trait-object adapter:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner") &&
		!strings.Contains(rust, "Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner") {
		t.Fatalf("structural interface map lookup should convert through the adapter:\n%s", rust)
	}
}

func TestPointerAssignmentFromRangeStructScalarFieldKeepsBareValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type option struct {
	Feature *bool
	Enable  bool
}

func apply(options []option) {
	for _, o := range options {
		*o.Feature = o.Enable
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

	rust, _, _ := Transpile(file, fset, typeInfo)

	if strings.Contains(rust, "let __owned = (*__v") {
		t.Fatalf("pointer assignment from a range struct scalar field should use the bare field value:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = (*o.enable.borrow().as_ref().unwrap())") {
		t.Fatalf("pointer assignment should read the range struct field directly:\n%s", rust)
	}
}

func TestPointerDerefAssignmentFromWrappedStructCallStoresBareValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type header struct {
	Len int
}

func makeHeader() header {
	return header{}
}

func assign(p *header) {
	*p = makeHeader()
}
`)

	if strings.Contains(rust, "Some(new_val)") && strings.Contains(rust, "let new_val = make_header()") {
		t.Fatalf("pointer deref assignment should not store a wrapped call result inside Some:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = (*make_header().borrow().as_ref().unwrap()).clone()") {
		t.Fatalf("pointer deref assignment should unwrap the struct call result before storing:\n%s", rust)
	}
}

func TestParallelPointerDerefAssignmentWritesThroughPointerHandles(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Int struct {
	n int
}

func rotate(a, b, c *Int) {
	*a, *b, *c = *b, *c, *a
}
`)

	if strings.Contains(rust, "*(*a.borrow_mut().as_mut().unwrap()).borrow_mut()") ||
		strings.Contains(rust, "*(*a.lock().unwrap().as_mut().unwrap()).lock()") {
		t.Fatalf("parallel pointer deref assignment should not borrow the pointee as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "*a.borrow_mut() = Some(__tmp_0);") &&
		!strings.Contains(rust, "*a.lock().unwrap() = Some(__tmp_0);") {
		t.Fatalf("parallel pointer deref assignment should write through the pointer handle:\n%s", rust)
	}
}

func TestPointerToPointerParamWritesThroughPointerSlot(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type List struct{}

type Alias struct {
	tparams *List
}

func bind() *List {
	return &List{}
}

func collect(dst **List) {
	*dst = bind()
}

func use(alias *Alias) {
	collect(&alias.tparams)
}
`)

	if !strings.Contains(rust, "pub fn collect(dst: Rc<RefCell<Option<Rc<RefCell<Option<List>>>>>>)") &&
		!strings.Contains(rust, "pub fn collect(dst: Arc<Mutex<Option<Arc<Mutex<Option<List>>>>>>)") {
		t.Fatalf("pointer-to-pointer parameter should preserve the nested pointer slot shape:\n%s", rust)
	}
	if strings.Contains(rust, "*dst.borrow_mut() = Some(new_val);") ||
		strings.Contains(rust, "*dst.lock().unwrap() = Some(new_val);") {
		t.Fatalf("pointer-to-pointer assignment should not store a pointee value in the outer pointer slot:\n%s", rust)
	}
	if !strings.Contains(rust, "*__dst_guard.as_ref().unwrap().borrow_mut() = (*new_val.borrow()).clone();") &&
		!strings.Contains(rust, "*__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone();") {
		t.Fatalf("pointer-to-pointer assignment should write through the pointee handle stored in the slot:\n%s", rust)
	}
	if !strings.Contains(rust, "collect(Rc::new(RefCell::new(Some((*alias.borrow().as_ref().unwrap()).tparams.clone()))))") &&
		!strings.Contains(rust, "collect(Arc::new(Mutex::new(Some((*alias.lock().unwrap().as_ref().unwrap()).tparams.clone()))))") {
		t.Fatalf("address-of pointer field should pass a pointer slot containing the field handle:\n%s", rust)
	}
}

func TestCopyScalarReturnBoundariesUseBareRustTypes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type counter struct {
	n int
}

func year() int {
	return 2024
}

func (c counter) value() int {
	return c.n
}

func named() (x int) {
	x = year()
	return
}

func pair() (int, bool) {
	return year(), true
}

func main() {
	fmt.Println(year())
}
`)

	for _, want := range []string{
		"pub fn year() -> i32",
		"pub fn named() -> i32",
		"pub fn pair() -> (i32, bool)",
		"pub fn value(&self) -> i32",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("copy scalar return boundary should emit %q:\n%s", want, rust)
		}
	}
	for _, unwanted := range []string{
		"pub fn year() -> Rc<RefCell<Option<i32>>>",
		"pub fn named() -> Rc<RefCell<Option<i32>>>",
		"pub fn pair() -> (Rc<RefCell<Option<i32>>>",
		"pub fn value(&self) -> Rc<RefCell<Option<i32>>>",
	} {
		if strings.Contains(rust, unwanted) {
			t.Fatalf("copy scalar return boundary should not remain wrapped (%q):\n%s", unwanted, rust)
		}
	}
	if !strings.Contains(rust, "\n    2024\n}") {
		t.Fatalf("tail literal scalar return should emit a bare expression:\n%s", rust)
	}
	if strings.Contains(rust, "return 2024") {
		t.Fatalf("tail literal scalar return should not keep explicit return syntax:\n%s", rust)
	}
	if strings.Contains(rust, "return year().borrow()") || strings.Contains(rust, "return (*year()") {
		t.Fatalf("bare scalar-returning calls should not be unwrapped at return boundaries:\n%s", rust)
	}
	if strings.Contains(rust, "format!(\"{}\", (*year().borrow().as_ref().unwrap()))") {
		t.Fatalf("bare scalar-returning calls should not be unwrapped for fmt printing:\n%s", rust)
	}
}

func TestBareScalarTupleReturnConstBinaryUsesExpectedType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const pages = 512

func find(i int) (int, uint) {
	return i, pages - 1
}
`)

	if !strings.Contains(rust, "pub fn find(i: Rc<RefCell<Option<i32>>>) -> (i32, u64)") {
		t.Fatalf("tuple return signature should use a u64 slot for uint:\n%s", rust)
	}
	if strings.Contains(rust, "let __tmp_x = PAGES; let __tmp_y = 1; __tmp_x - __tmp_y") {
		t.Fatalf("constant binary expression in uint return slot should not stay as i32 arithmetic:\n%s", rust)
	}
	if !strings.Contains(rust, "as u64") {
		t.Fatalf("constant binary expression in uint return slot should use the expected u64 type:\n%s", rust)
	}
}

func TestBareScalarReturnConstIdentUsesExpectedType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const maxPackedValue = 1 << 21

func start(ok bool) uint {
	if ok {
		return maxPackedValue
	}
	return 0
}

func unpack(ok bool) (uint, uint, uint) {
	if ok {
		return maxPackedValue, maxPackedValue, maxPackedValue
	}
	return 0, 0, 0
}
`)

	if !strings.Contains(rust, "pub fn start(ok: Rc<RefCell<Option<bool>>>) -> u64") {
		t.Fatalf("single return signature should use u64 for uint:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn unpack(ok: Rc<RefCell<Option<bool>>>) -> (u64, u64, u64)") {
		t.Fatalf("tuple return signature should use u64 slots for uint:\n%s", rust)
	}
	if strings.Contains(rust, "return MAX_PACKED_VALUE;") {
		t.Fatalf("constant identifier in uint return slot should not stay as i32:\n%s", rust)
	}
	if strings.Contains(rust, "return (MAX_PACKED_VALUE, MAX_PACKED_VALUE, MAX_PACKED_VALUE);") {
		t.Fatalf("constant identifiers in uint tuple return slots should not stay as i32:\n%s", rust)
	}
	if count := strings.Count(rust, "MAX_PACKED_VALUE as u64"); count < 4 {
		t.Fatalf("constant identifier returns should use expected u64 types, found %d casts:\n%s", count, rust)
	}
}

func TestTailReturnExpressionsOmitReturnKeyword(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func early(ok bool) int {
	if ok {
		return 1
	}
	return 2
}

func pair() (int, bool) {
	return 3, true
}

func closure() int {
	f := func() int {
		return 4
	}
	return f()
}
`)

	if !strings.Contains(rust, "return 1;") {
		t.Fatalf("non-tail return should keep explicit return syntax:\n%s", rust)
	}
	if strings.Contains(rust, "return 2;") || strings.Contains(rust, "return (3, true);") || strings.Contains(rust, "return 4;") {
		t.Fatalf("tail returns should omit explicit return syntax:\n%s", rust)
	}
	for _, want := range []string{
		"\n    2\n}",
		"\n    (3, true)\n}",
		"move || -> i32 {\n        4\n    }",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("tail return should emit %q:\n%s", want, rust)
		}
	}
}

func TestReturnFunctionIdentifierUsesTypedFunctionValueBox(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type value struct {
	n int
}

func cvtInt(v value) value {
	return v
}

func convertOp(ok bool) func(value) value {
	if ok {
		return cvtInt
	}
	return nil
}
`)

	if strings.Contains(rust, "cvtInt.clone()") {
		t.Fatalf("returning a function identifier should not emit an untranslated Go identifier clone:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(move |__arg0:") ||
		!strings.Contains(rust, "{ cvt_int(__arg0) }) as Box<dyn FnMut") {
		t.Fatalf("returning a function identifier should box a typed function value using the Rust function name:\n%s", rust)
	}
}

func TestPackageStringVarFromUntypedConstUsesOwnedString(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const defaultGO_LDSO = ""

var GO_LDSO = defaultGO_LDSO
`)

	if strings.Contains(rust, "Some(DEFAULT_G_O__L_D_S_O)") {
		t.Fatalf("string var initialized from an untyped string const should not store a borrowed const:\n%s", rust)
	}
	if !strings.Contains(rust, `Some("".to_string())`) {
		t.Fatalf("string var initialized from an untyped string const should store an owned String:\n%s", rust)
	}
}

func TestStringShortDeclFromUntypedConstUsesOwnedString(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const DefaultGOARM = "7"

func goarm() string {
	def := DefaultGOARM
	def = "6"
	return def
}
`)

	if strings.Contains(rust, "Some(DEFAULT_G_O_A_R_M)") {
		t.Fatalf("string short declaration from an untyped string const should not store a borrowed const:\n%s", rust)
	}
	if !strings.Contains(rust, `Some("7".to_string())`) {
		t.Fatalf("string short declaration from an untyped string const should store an owned String:\n%s", rust)
	}
}

func TestStrconvAtoiStringSliceUsesOwnedStringInput(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "strconv"

func year(v string, i int) int {
	year, _ := strconv.Atoi(v[:i])
	return year
}
`)

	if strings.Contains(rust, "let __atoi_input = Rc::new") || strings.Contains(rust, "let __atoi_input = Arc::new") {
		t.Fatalf("strconv.Atoi should parse an owned string value, not a wrapped string handle:\n%s", rust)
	}
	if !strings.Contains(rust, "match __atoi_input.parse::<i32>()") {
		t.Fatalf("strconv.Atoi should parse the prepared string input:\n%s", rust)
	}
}

func TestReflectMethodValueMapAssignmentUsesTypedClosure(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "reflect"

func use(field reflect.Value) {
	names := map[string]func(bool){}
	names["x"] = field.SetBool
}
`)

	if strings.Contains(rust, ".set_bool.clone()") {
		t.Fatalf("method value assigned into function map should lower as a typed closure, not a field clone:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(move |") || !strings.Contains(rust, ".set_bool(") {
		t.Fatalf("method value assigned into function map should emit a callable closure:\n%s", rust)
	}
}

func TestStructLiteralWrappingBareStructVarClonesFieldValues(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "go/ast"

type Holder struct {
	First ast.Field
	Second ast.Field
}

func makeHolder() Holder {
	baseline := ast.Field{}
	return Holder{First: baseline, Second: baseline}
}
`)

	if strings.Contains(rust, "Some(baseline))") {
		t.Fatalf("struct literal field wrappers should not move a bare struct local:\n%s", rust)
	}
	if strings.Count(rust, "Some(baseline.clone())") < 2 {
		t.Fatalf("struct literal should clone bare non-copy struct values for wrapped fields:\n%s", rust)
	}
}

func TestStructLiteralLenFieldUsesGoInt(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type span struct {
	start int
	end int
}

func last(s string) span {
	return span{0, len(s)}
}
`)

	if strings.Contains(rust, ".len())))") {
		t.Fatalf("len field should not leave a usize in a wrapped Go int field:\n%s", rust)
	}
	if !strings.Contains(rust, ".len() as i32") {
		t.Fatalf("len field should cast Rust usize to Go int:\n%s", rust)
	}
}

func TestFloatCompoundAssignUntypedIntegerConstantUsesFloatLiteral(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func scale(m float64) float64 {
	m *= 10
	m /= 10
	return m
}
`)

	if strings.Contains(rust, "__rhs = 10;") || strings.Contains(rust, "m * 10;") || strings.Contains(rust, "m / 10;") {
		t.Fatalf("float compound assignment should not use an integer RHS literal:\n%s", rust)
	}
	if !strings.Contains(rust, "__rhs = 10.0;") {
		t.Fatalf("float compound assignment should coerce untyped integer constants to float literals:\n%s", rust)
	}
}

func TestUint64CompoundAssignKeepsFloatConstantSubexpression(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

const retainExtraPercent = 10

func goal(gcPercentGoal uint64) uint64 {
	go func() {}()
	gcPercentGoal += gcPercentGoal / (1.0 / (retainExtraPercent / 100.0))
	return gcPercentGoal
}
`)

	if strings.Contains(rust, "1.0 as u64") ||
		strings.Contains(rust, "100.0 as u64") ||
		strings.Contains(rust, "RETAIN_EXTRA_PERCENT as u64") ||
		strings.Contains(rust, "retainExtraPercent as u64") {
		t.Fatalf("uint64 compound assignment should not cast float constant subexpressions to u64 before evaluating them:\n%s", rust)
	}
	if !strings.Contains(rust, "let __tmp_x = 1.0") || !strings.Contains(rust, "let __tmp_y = 0.1") {
		t.Fatalf("uint64 compound assignment should evaluate the denominator in float space before the final cast:\n%s", rust)
	}
}

func TestBareFloatIncDecUsesFloatLiteral(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func split(x float64) (float64, float64) { return x, x }

func adjust(x float64) float64 {
	yf, yi := split(x)
	yf--
	yi++
	return yf + yi
}
`)

	if strings.Contains(rust, "yf -= 1;") || strings.Contains(rust, "yi += 1;") {
		t.Fatalf("bare float inc/dec should not use integer literals:\n%s", rust)
	}
	if !strings.Contains(rust, "yf -= 1.0;") || !strings.Contains(rust, "yi += 1.0;") {
		t.Fatalf("bare float inc/dec should emit float literals:\n%s", rust)
	}
}

func TestWrappedFloatFieldIncDecUsesFloatLiteral(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type stats struct {
	count float64
	total int
}

func bump(s *stats) {
	s.count++
	s.count--
	s.total++
}
`)

	if strings.Contains(rust, ".count.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1);") ||
		strings.Contains(rust, ".count.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1);") ||
		strings.Contains(rust, ".count.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1);") ||
		strings.Contains(rust, ".count.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1);") {
		t.Fatalf("wrapped float field inc/dec should not use integer literals:\n%s", rust)
	}
	if !strings.Contains(rust, ".count.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1.0);") &&
		!strings.Contains(rust, ".count.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1.0);") {
		t.Fatalf("wrapped float field increment should use a float literal:\n%s", rust)
	}
	if !strings.Contains(rust, ".count.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1.0);") &&
		!strings.Contains(rust, ".count.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1.0);") {
		t.Fatalf("wrapped float field decrement should use a float literal:\n%s", rust)
	}
}

func TestUintConversionUsesGoUintWidthForBitmask(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const shift = 52
const mask = 0x7ff
const bias = 1023

func clear(x uint64) uint64 {
	e := uint(x>>shift)&mask - bias
	if e < 64-12 {
		x &^= 1<<(64-12-e) - 1
	}
	return x
}
`)

	if strings.Contains(rust, "as u32") {
		t.Fatalf("uint conversion in uint64 bitmask path should not narrow to u32 on this target:\n%s", rust)
	}
	if !strings.Contains(rust, "as u64") {
		t.Fatalf("uint conversion should use the Go uint width for this target:\n%s", rust)
	}
}

func TestParallelAssignmentToValueReceiverUsesReceiverAlias(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type nat []uint

func (z nat) swap(x nat) nat {
	var zz nat
	zz, z = z, zz
	return z
}
`)

	if strings.Contains(rust, "*z.borrow") || strings.Contains(rust, "*z.lock") {
		t.Fatalf("parallel assignment to reassigned value receiver should use the receiver alias, not the Go receiver name:\n%s", rust)
	}
	if !strings.Contains(rust, "__self =") {
		t.Fatalf("parallel assignment to value receiver should assign the receiver alias:\n%s", rust)
	}
}

func TestNamedScalarTypeSwitchAndInterfaceConversionsUseInnerValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Value interface{ marker() }
type int64Val int64
type boolVal bool

func (int64Val) marker() {}
func (boolVal) marker() {}

func asBool(v Value) bool {
	switch x := v.(type) {
	case boolVal:
		return bool(x)
	default:
		return false
	}
}

func makeValue() Value {
	return int64Val(0)
}

func makeAny() any {
	return int64Val(0)
}
`)

	if strings.Contains(rust, "(*x.borrow().as_ref().unwrap()).borrow") ||
		strings.Contains(rust, "(*x.lock().unwrap().as_ref().unwrap()).lock") {
		t.Fatalf("type switch named bool conversion should use the named bool inner value:\n%s", rust)
	}
	if !strings.Contains(rust, ").0.borrow") && !strings.Contains(rust, ").0.lock") {
		t.Fatalf("type switch named bool conversion should read the newtype field:\n%s", rust)
	}
	if strings.Contains(rust, "int64Val(Rc::new(RefCell::new(Some(0 as i64)))).borrow") ||
		strings.Contains(rust, "int64Val(Arc::new(Mutex::new(Some(0 as i64)))).lock") {
		t.Fatalf("named integer conversion boxed as an interface should not be unwrapped as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(int64Val(") {
		t.Fatalf("named integer conversion should be boxed directly as the interface value:\n%s", rust)
	}
}

func TestNamedBoolReceiverConversionUsesSelfInnerValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type boolVal bool

func (x boolVal) truth() bool {
	return bool(x)
}
`)

	if strings.Contains(rust, "x.lock") || strings.Contains(rust, "x.borrow") {
		t.Fatalf("named bool receiver conversion should use self, not the Go receiver name:\n%s", rust)
	}
	if !strings.Contains(rust, "self.0") {
		t.Fatalf("named bool receiver conversion should read the receiver newtype field:\n%s", rust)
	}
}

func TestNamedBoolPointerReceiverDerefConversionUsesSelfInnerValue(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type boolVal bool

func (x *boolVal) truth() bool {
	return bool(*x)
}
`)

	if strings.Contains(rust, "as_mut().unwrap()).lock") ||
		strings.Contains(rust, "as_mut().unwrap()).borrow") {
		t.Fatalf("named bool pointer receiver deref conversion should not treat the scalar pointee as another handle:\n%s", rust)
	}
	if !strings.Contains(rust, "self.0") {
		t.Fatalf("named bool pointer receiver deref conversion should read the receiver newtype field:\n%s", rust)
	}
}

func TestNamedFloatPointerReceiverDerefConversionUsesSelfInnerValue(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type floatVal float64

func (x *floatVal) value() float64 {
	return float64(*x)
}
`)

	if strings.Contains(rust, "(*self).clone() as f64") {
		t.Fatalf("named float pointer receiver deref conversion should not cast the receiver wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "self.0") {
		t.Fatalf("named float pointer receiver deref conversion should read the receiver newtype field:\n%s", rust)
	}
}

func TestNamedBoolLogicalOpsBoxInterfaceReturns(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Value interface{ marker() }
type boolVal bool

func (boolVal) marker() {}

func notValue(v Value) Value {
	switch y := v.(type) {
	case boolVal:
		return !y
	}
	return nil
}

func andValue(x, y boolVal) Value {
	return x && y
}

func orValue(x, y boolVal) Value {
	return x || y
}
`)

	if strings.Count(rust, "Box::new(boolVal(") < 3 {
		t.Fatalf("named bool logical results should be boxed as concrete interface values:\n%s", rust)
	}
	if strings.Contains(rust, "Some(!{ let __v =") || strings.Contains(rust, "Some({ let __v =") {
		t.Fatalf("named bool logical operations should use primitive bool operands before rewrapping:\n%s", rust)
	}
}

func TestNamedIntegerUnaryAndShiftInterfaceReturnsBoxConcreteValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Value interface{ marker() }
type int64Val int64

func (int64Val) marker() {}

func negValue(v Value) Value {
	switch y := v.(type) {
	case int64Val:
		return -y
	}
	return nil
}

func shiftValue(x int64Val, s uint) Value {
	return x >> s
}

func asUint(x int64Val) uint64 {
	return uint64(-x)
}
`)

	if !strings.Contains(rust, "impl std::ops::Neg for int64Val") {
		t.Fatalf("named signed integer type should implement unary minus:\n%s", rust)
	}
	if strings.Contains(rust, "(*-") {
		t.Fatalf("numeric conversion of a negated named integer should negate the primitive value, not dereference after unary minus:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(-") || !strings.Contains(rust, "Box::new((*x") {
		t.Fatalf("named integer unary and shift results should be boxed as concrete interface values:\n%s", rust)
	}
}

func TestExternalNamedIntegerRangeCompoundAssignMutatesBareValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "math/big"

func drain(words []big.Word) byte {
	var out byte
	for _, w := range words {
		out = byte(w)
		w >>= 8
	}
	return out
}
`)

	if strings.Contains(rust, "w.lock") || strings.Contains(rust, "w.borrow") {
		t.Fatalf("bare external named integer range variables should not be mutated as wrapped handles:\n%s", rust)
	}
	if !strings.Contains(rust, "w = big_Word(") {
		t.Fatalf("external named integer compound assignment should rewrap the mutated primitive value:\n%s", rust)
	}
}

func TestNamedIntegerRangeCompoundAssignMutatesBareValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func drain(words nat) byte {
	var out byte
	for _, w := range words {
		out = byte(w)
		w >>= 8
	}
	return out
}
`)

	if strings.Contains(rust, "w.lock") || strings.Contains(rust, "w.borrow") {
		t.Fatalf("bare named integer range variables should not be mutated as wrapped handles:\n%s", rust)
	}
	if !strings.Contains(rust, "w = w >> __rhs") {
		t.Fatalf("named integer compound assignment should mutate the bare range value:\n%s", rust)
	}
}

func TestNamedIntegerSliceElementUnaryNotStoresNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func invert(z nat) {
	for i := range z {
		z[i] = ^z[i]
	}
}
`)

	if strings.Contains(rust, "] = !(*") {
		t.Fatalf("named integer slice element unary-not assignment should not store a raw primitive:\n%s", rust)
	}
	if !strings.Contains(rust, "] = Word(") {
		t.Fatalf("named integer slice element unary-not assignment should store the named value:\n%s", rust)
	}
}

func TestNamedIntegerUnaryNotReturnWrapsNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type EmptyOp uint8

func impossible() EmptyOp {
	return ^EmptyOp(0)
}
`)

	if strings.Contains(rust, "pub fn impossible() -> Rc<RefCell<Option<EmptyOp>>> {\n    Rc::new(RefCell::new(Some(!0 as u8)))") {
		t.Fatalf("named integer unary-not return should not wrap the raw primitive:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(EmptyOp(") {
		t.Fatalf("named integer unary-not return should rebuild the named value:\n%s", rust)
	}
}

func TestUintptrUnaryNotReturnUsesBareConversionValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func invalid() uintptr {
	return ^(uintptr(0))
}
`)

	if strings.Contains(rust, "!(Rc::new") || strings.Contains(rust, "!(Arc::new") {
		t.Fatalf("uintptr unary-not return should not complement a wrapped conversion:\n%s", rust)
	}
	if !strings.Contains(rust, "!(0 as usize)") {
		t.Fatalf("uintptr unary-not return should complement the bare usize conversion:\n%s", rust)
	}
}

func TestNamedIntegerReturnCallAssignedToSliceElementPreservesCall(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func decode(buf []byte) Word {
	return Word(buf[0])
}

func store(z nat, buf []byte) {
	z[0] = decode(buf[:1])
}
`)

	if !strings.Contains(rust, "decode(Rc::new") && !strings.Contains(rust, "decode(Arc::new") {
		t.Fatalf("named integer return call assigned to slice element should remain a function call, not a conversion:\n%s", rust)
	}
}

func TestMutexGuardTransfersAcrossPointerAliasAssignment(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type node struct {
	mu sync.Mutex
}

func aliasUnlock(x, l *node) {
	l.mu.Lock()
	x = l
	x.mu.Unlock()
}
`)

	if strings.Contains(rust, ".mu.unlock();") {
		t.Fatalf("mutex unlock through an assigned alias should drop the tracked guard, not call a missing unlock method:\n%s", rust)
	}
	if !strings.Contains(rust, "drop(__mutex_guard") {
		t.Fatalf("mutex unlock through an assigned alias should drop the original lock guard:\n%s", rust)
	}
}

func TestPackageMutexLockUsesInnerGoMutexValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

var mu sync.Mutex

func lockPackageMutex() {
	mu.Lock()
	defer mu.Unlock()
}
`)

	if strings.Contains(rust, "= mu.clone(); let __mutex_guard") {
		t.Fatalf("package mutex guard source should clone the inner GoMutex value, not the wrapped handle:\n%s", rust)
	}
	if !strings.Contains(rust, "mu.borrow().as_ref().unwrap()).clone(); let __mutex_guard") &&
		!strings.Contains(rust, "mu.lock().unwrap().as_ref().unwrap()).clone(); let __mutex_guard") {
		t.Fatalf("package mutex guard source should dereference the wrapped mutex handle:\n%s", rust)
	}
}

func TestSourceMappedPackageMutexLockUsesGeneratedMethods(t *testing.T) {
	rust := transpileTypedConcurrentRegressionWithMapping(t, `package main

import "sync"

var mu sync.Mutex

func lockPackageMutex() {
	mu.Lock()
	defer mu.Unlock()
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, ".guard()") {
		t.Fatalf("source-mapped sync.Mutex should call the generated lock method, not the GoMutex guard helper:\n%s", rust)
	}
	if strings.Contains(rust, "// mu.Unlock() handled by RAII guard") {
		t.Fatalf("source-mapped sync.Mutex defer should not be suppressed as a GoMutex RAII guard:\n%s", rust)
	}
	if !strings.Contains(rust, ".lock();") || !strings.Contains(rust, ".unlock();") {
		t.Fatalf("source-mapped sync.Mutex should emit generated lock and unlock method calls:\n%s", rust)
	}
}

func TestSourceMappedMutexFieldUnlockUnwrapsGeneratedValue(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "sync", `package sync

type Mutex struct{}

func (m *Mutex) Lock() {}
func (m *Mutex) Unlock() {}

type Once struct {
	m Mutex
}

func (o *Once) slow() {
	go func() {}()
	o.m.Lock()
	defer o.m.Unlock()
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, ".m.unlock();") {
		t.Fatalf("source-mapped mutex field unlock should unwrap the generated Mutex value:\n%s", rust)
	}
	if !strings.Contains(rust, ".m.lock().unwrap().as_ref().unwrap()).lock();") ||
		!strings.Contains(rust, ".m.lock().unwrap().as_ref().unwrap()).unlock();") {
		t.Fatalf("source-mapped mutex field lock/unlock should call methods on the inner generated Mutex value:\n%s", rust)
	}
}

func TestSourceMappedMutexFieldTryLockUnwrapsGeneratedValue(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "sync", `package sync

type Mutex struct{}

func (m *Mutex) TryLock() bool { return true }

type RWMutex struct {
	w Mutex
}

func (rw *RWMutex) TryLock() bool {
	go func() {}()
	return rw.w.TryLock()
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, ".w.try_lock()") {
		t.Fatalf("source-mapped mutex field TryLock should not call std::sync::Mutex::try_lock on the wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, ".w.lock().unwrap().as_ref().unwrap()).try_lock()") {
		t.Fatalf("source-mapped mutex field TryLock should call the generated Mutex method on the inner value:\n%s", rust)
	}
}

func TestSourceMappedEmbeddedSyncMutexDeferUnlockUsesEmbeddedField(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "reflect", `package reflect

import "sync"

var cache struct {
	sync.Mutex
	n int
}

func use() {
	go func() {}()
	cache.Lock()
	defer cache.Unlock()
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, ".as_mut().unwrap()).unlock();") {
		t.Fatalf("deferred promoted source-mapped sync.Mutex unlock should not call Unlock on the containing anonymous struct:\n%s", rust)
	}
	if !strings.Contains(rust, ".mutex.unlock();") {
		t.Fatalf("deferred promoted source-mapped sync.Mutex unlock should call through the embedded mutex field:\n%s", rust)
	}
}

func TestSourceMappedEmbeddedSyncMutexDefaultUsesGeneratedDefault(t *testing.T) {
	rust := transpileTypedConcurrentPackageWithMapping(t, "reflect", `package reflect

import "sync"

var cache struct {
	sync.Mutex
	n int
}

func use() {
	go func() {}()
}
`, map[string]string{"sync": "sync"})

	if strings.Contains(rust, "sync::mutex::Mutex::new()") {
		t.Fatalf("source-mapped sync.Mutex field default should use the generated Default impl, not the helper new constructor:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_default_0_0 = Default::default();") ||
		!strings.Contains(rust, "mutex: __go_default_0_0") {
		t.Fatalf("source-mapped sync.Mutex field default should initialize the embedded field with Default::default():\n%s", rust)
	}
}

func TestMutexGuardNameStableAcrossFileSetBase(t *testing.T) {
	source := `package main

import "sync"

func lock(mu *sync.Mutex) {
	mu.Lock()
	defer mu.Unlock()
}
`
	rustA := transpileTypedRegressionWithPrefixFile(t, source, "")
	rustB := transpileTypedRegressionWithPrefixFile(t, source, `package other

var pad = "shift file set base"
`)

	guardA := firstMutexGuardName(t, rustA)
	guardB := firstMutexGuardName(t, rustB)
	if guardA != guardB {
		t.Fatalf("mutex guard names should be stable across token.FileSet base changes: %q != %q\nA:\n%s\nB:\n%s", guardA, guardB, rustA, rustB)
	}
}

func transpileTypedRegressionWithPrefixFile(t *testing.T, source string, prefixSource string) string {
	t.Helper()
	fset := token.NewFileSet()
	if prefixSource != "" {
		if _, err := parser.ParseFile(fset, "prefix.go", prefixSource, parser.ParseComments); err != nil {
			t.Fatalf("ParseFile(prefix) error = %v", err)
		}
	}
	file, err := parser.ParseFile(fset, "main.go", source, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(main) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo error = %v", err)
	}
	rust, _, _ := Transpile(file, fset, typeInfo)
	return rust
}

func transpileTypedConcurrentRegressionWithMapping(t *testing.T, src string, packageMapping map[string]string) string {
	t.Helper()

	return transpileTypedConcurrentPackageWithMapping(t, "", src, packageMapping)
}

func transpileTypedConcurrentPackageWithMapping(t *testing.T, packagePath string, src string, packageMapping map[string]string) string {
	t.Helper()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter(packagePath, []*ast.File{file}, fset, nil)
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
	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, packageMapping)
	return rust
}

func firstMutexGuardName(t *testing.T, rust string) string {
	t.Helper()
	const prefix = "__mutex_guard_"
	start := strings.Index(rust, prefix)
	if start < 0 {
		t.Fatalf("missing mutex guard in generated Rust:\n%s", rust)
	}
	end := start + len(prefix)
	for end < len(rust) && rust[end] >= '0' && rust[end] <= '9' {
		end++
	}
	return rust[start:end]
}

func TestPointerReceiverComparisonUsesHandleIdentity(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type node struct{}

func (x *node) different(y *node) bool {
	return y != x
}
`)

	if strings.Contains(rust, "return true") {
		t.Fatalf("pointer receiver comparison against another pointer should not collapse to a constant:\n%s", rust)
	}
	if !strings.Contains(rust, "__self_ptr") {
		t.Fatalf("pointer receiver comparison should compare the peer handle against the receiver address:\n%s", rust)
	}
}

func TestPointerReceiverAssignedToPointerVariableUsesHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type node struct {
	n int
}

func (x *node) current() *node {
	var out *node
	out = x
	return out
}
`)

	if strings.Contains(rust, "let new_val = self.clone(); out = new_val") {
		t.Fatalf("pointer receiver assignment should not assign the bare receiver value to a pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = Rc::new(RefCell::new(Some(self.clone())))") &&
		!strings.Contains(rust, "let new_val = Arc::new(Mutex::new(Some(self.clone())))") {
		t.Fatalf("pointer receiver assignment should wrap the receiver clone as a pointer handle:\n%s", rust)
	}
}

func TestPointerReceiverReassignmentUsesLocalReceiverCopy(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type node struct {
	n int
}

func makeNode() *node {
	return &node{n: 7}
}

func (x *node) reset() int {
	x = makeNode()
	return x.n
}
`)

	if strings.Contains(rust, "self = new_val") {
		t.Fatalf("pointer receiver reassignment should not assign a pointer handle to self:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut __self = self.clone();") {
		t.Fatalf("pointer receiver reassignment should introduce a local receiver copy:\n%s", rust)
	}
	if !strings.Contains(rust, "__self = __moved_val") {
		t.Fatalf("pointer receiver reassignment should move the pointed value into the local receiver copy:\n%s", rust)
	}
}

func TestNestedReturnsInsideTailControlFlowStayExplicit(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func choose(v int) int {
	switch v {
	case 0:
		return 1
	default:
		return 2
	}
}
`)

	for _, want := range []string{
		"return 1;",
		"return 2;",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("nested return inside tail control flow should keep explicit %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "\n            1\n") || strings.Contains(rust, "\n            2\n") {
		t.Fatalf("nested return inside tail control flow must not become a bare expression:\n%s", rust)
	}
}

func TestTailReturnStartingWithConcurrentBlockStaysExplicit(t *testing.T) {
	src := `package main

func both(a, b, c, d int) bool {
	go func() {}()
	return a == b && c == d
}
`
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
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

	rust := transpileParsedRegression(t, file, fset, typeInfo)

	if !strings.Contains(rust, "return { let __tmp_x =") || !strings.Contains(rust, "} && { let __tmp_x =") {
		t.Fatalf("tail return whose expression starts with a Rust block should stay explicit:\n%s", rust)
	}
}

func TestTailReturnStartingWithConcurrentIdentBlockStaysExplicit(t *testing.T) {
	src := `package main

func same(isRight bool, child, parent int) bool {
	go func() {}()
	if child < parent {
		return true
	}
	return isRight && child == parent
}
`
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
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

	rust := transpileParsedRegression(t, file, fset, typeInfo)

	if !strings.Contains(rust, "return { let __v = (*isRight") || !strings.Contains(rust, "} && { let __tmp_x =") {
		t.Fatalf("tail return whose leading ident emits a Rust block should stay explicit:\n%s", rust)
	}
}

func TestBareBoolCallConditionStaysBare(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func ok() bool {
	return true
}

func run() {
	if ok() {
		return
	}
}
`)

	if !strings.Contains(rust, "if ok() {") {
		t.Fatalf("bare bool-returning call should be used directly in conditions:\n%s", rust)
	}
	if strings.Contains(rust, "ok().borrow()") || strings.Contains(rust, "ok().lock()") {
		t.Fatalf("bare bool-returning call should not be unwrapped as a wrapped handle:\n%s", rust)
	}
}

func TestConcurrentIfBuildsLargeLogicalConditionWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type module struct {
	a uintptr
	b uintptr
	c uintptr
	d uintptr
	e uintptr
	f uintptr
	g uintptr
	h uintptr
}

var first module

func inRange(p uintptr) bool {
	go func() {}()
	datap := &first
	if datap.a <= p && p < datap.b ||
		datap.c <= p && p < datap.d ||
		datap.e <= p && p < datap.f ||
		datap.g <= p && p < datap.h {
		return true
	}
	return false
}
`)

	if strings.Contains(rust, "} && {") || strings.Contains(rust, "} || {") {
		t.Fatalf("large logical condition should not chain inline statement blocks:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("large logical condition should build condition operands with statements:\n%s", rust)
	}
}

func TestConcurrentIfBuildsComplexTwoPartLogicalConditionWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type object struct {
	off uint32
	size uint32
}

type buf struct {
	nobj int
	obj [64]object
}

type stack struct {
	lo uintptr
}

type state struct {
	stack stack
}

func overlaps(x *buf, s *state, addr uintptr) bool {
	go func() {}()
	if x.nobj > 0 && uint32(addr-s.stack.lo) < x.obj[x.nobj-1].off+x.obj[x.nobj-1].size {
		return true
	}
	return false
}
`)

	if strings.Contains(rust, "} && {") {
		t.Fatalf("complex two-part logical condition should not chain inline statement blocks:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("complex two-part logical condition should build condition operands with statements:\n%s", rust)
	}
	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "let __go_cond_1 =") && strings.Contains(line, "let __tmp_x =") {
			t.Fatalf("complex comparison condition temp should split its operands across lines:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "let __go_cond_1 = {\n") {
		t.Fatalf("complex comparison condition temp should use a multiline block:\n%s", rust)
	}
}

func TestConcurrentIfBuildsNestedSelectorConversionConditionWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type special struct {
	offset uint16
	kind byte
}

func find(s *special, offset uintptr, kind byte) bool {
	go func() {}()
	if offset < uintptr(s.offset) || (offset == uintptr(s.offset) && kind < s.kind) {
		return true
	}
	return false
}
`)

	if strings.Contains(rust, "} || ({") || strings.Contains(rust, "} && {") {
		t.Fatalf("selector conversion logical condition should not chain inline statement blocks:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("selector conversion logical condition should build condition operands with statements:\n%s", rust)
	}
}

func TestConcurrentLogicalShortDeclBuildsSequenceIndexConditionsWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func overflow(data []uint64, stk []uintptr) bool {
	go func() {}()
	isOverflow := len(stk) == 1 && data[2] == 0 && data[3] == 0 && data[4] == 0
	return isOverflow
}
`)

	if strings.Contains(rust, "} && {") {
		t.Fatalf("logical short declaration with repeated sequence indexes should not chain inline blocks:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("logical short declaration with repeated sequence indexes should build condition operands with statements:\n%s", rust)
	}
}

func TestConcurrentLogicalShortDeclBuildsPointerFieldConditionsWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type stackFrame struct {
	pc uintptr
	sp uintptr
}

type goroutine struct {
	syscallpc uintptr
	syscallsp uintptr
}

func check(frame *stackFrame, gp *goroutine, pc0 uintptr, sp0 uintptr) bool {
	go func() {}()
	isSyscall := frame.pc == pc0 && frame.sp == sp0 && pc0 == gp.syscallpc && sp0 == gp.syscallsp
	return isSyscall
}
`)

	if strings.Contains(rust, "} && {") {
		t.Fatalf("logical short declaration with pointer field comparisons should not chain inline blocks:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("logical short declaration with pointer field comparisons should build condition operands with statements:\n%s", rust)
	}
}

func TestConcurrentIfBuildsLogicalCallConditionWithSelectorArgs(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type module struct {
	data uintptr
	edata uintptr
	bss uintptr
	ebss uintptr
}

func inRange(p uintptr, start uintptr, end uintptr) bool {
	return start <= p && p < end
}

func isGoPointer(p uintptr, datap *module) bool {
	go func() {}()
	if inRange(p, datap.data, datap.edata) || inRange(p, datap.bss, datap.ebss) {
		return true
	}
	return false
}
`)

	if strings.Contains(rust, "} || {") {
		t.Fatalf("logical call condition with selector args should not chain inline statement blocks:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("logical call condition with selector args should build condition operands with statements:\n%s", rust)
	}
}

func TestConcurrentIfBuildsRepeatedLogicalCallConditionsWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func hasPrefix(name string, prefix string) bool {
	return false
}

func isRuntimeName(name string) bool {
	go func() {}()
	if hasPrefix(name, "runtime.") ||
		hasPrefix(name, "runtime/internal/") ||
		hasPrefix(name, "internal/runtime/") ||
		hasPrefix(name, "reflect.") {
		return true
	}
	return false
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Count(line, "has_prefix(") > 1 {
			t.Fatalf("repeated logical call operands should not stay on one condition line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("repeated logical call operands should build condition operands with statements:\n%s", rust)
	}
}

func TestConcurrentIfBuildsFourPartLogicalConditionWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func compareAndSwap(state int32, old int32, new int32) bool {
	return true
}

func spin(awoke bool, old int32) bool {
	go func() {}()
	if !awoke && old&2 == 0 && old>>3 != 0 && compareAndSwap(old, old, old|2) {
		return true
	}
	return false
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "if !") && strings.Contains(line, "compare_and_swap(") {
			t.Fatalf("four-part logical condition should not stay on one if line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("four-part logical condition should build condition operands with statements:\n%s", rust)
	}
}

func TestConcurrentIfBuildsNestedStringIndexLogicalConditionWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func fractional(layout string, i int) bool {
	go func() {}()
	if i+1 < len(layout) && (layout[i+1] == '0' || layout[i+1] == '9') {
		return true
	}
	return false
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "if ") && strings.Contains(line, "as_bytes()") && strings.Contains(line, "&&") {
			t.Fatalf("nested string-index logical condition should not stay on one if line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("nested string-index logical condition should build condition operands with statements:\n%s", rust)
	}
}

func TestConcurrentIfBuildsNegatedNestedLogicalConditionWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func visible(i uintptr, off uintptr) bool {
	go func() {}()
	if !(i < 128*8 || off-16*8 < i && i < off+16*8) {
		return false
	}
	return true
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "if !") && strings.Contains(line, "||") && strings.Contains(line, "&&") {
			t.Fatalf("negated nested logical condition should not stay on one if line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "if !({\n") {
		t.Fatalf("negated nested logical condition should wrap a statement-built condition:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("negated nested logical condition should build condition operands with statements:\n%s", rust)
	}
}

func TestConcurrentIfBuildsThreeFieldBitmaskConditionsWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type timer struct {
	blocked uint32
	state uint8
}

const (
	timerHeaped uint8 = 1
	timerZombie uint8 = 2
)

func unblock(t *timer) bool {
	go func() {}()
	if t.blocked == 0 && t.state&timerHeaped != 0 && t.state&timerZombie == 0 {
		return true
	}
	return false
}
`)

	if strings.Contains(rust, "} && {") {
		t.Fatalf("three field bitmask conditions should not chain inline statement blocks:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("three field bitmask conditions should build condition operands with statements:\n%s", rust)
	}
}

func TestConcurrentIfBuildsGoErrorComparisonChainWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type errno int

func (e errno) Error() string {
	return ""
}

const (
	errA errno = 1
	errB errno = 2
	errC errno = 3
	errD errno = 4
)

func check(err error) bool {
	go func() {}()
	if err != errA && err != errB && err != errC && err != errD {
		return true
	}
	return false
}
`)

	if strings.Contains(rust, "} && {") {
		t.Fatalf("go error comparison chain should not keep inline downcast blocks:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("go error comparison chain should build condition operands with statements:\n%s", rust)
	}
}

func TestConcurrentLogicalValueBuildsComplexOperandsWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type funcID uint8

const (
	sigpanic funcID = 1
	asyncPreempt funcID = 2
	debugCall funcID = 3
)

type fn struct {
	funcID funcID
}

func injected(f *fn) bool {
	go func() {}()
	injectedCall := f.funcID == sigpanic || f.funcID == asyncPreempt || f.funcID == debugCall
	return injectedCall
}
`)

	if strings.Contains(rust, "} || {") {
		t.Fatalf("complex logical value should not chain inline statement blocks:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_cond_") {
		t.Fatalf("complex logical value should build operands with statements:\n%s", rust)
	}
}

func TestConcurrentMethodCallBreaksComplexArgumentsAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type log struct{}

func (l *log) write(a any, n int64, hdr []uint64, stk []uint64) {}

type profiler struct {
	log log
	extra []uint64
}

func (p *profiler) addExtra(i int) {
	go func() {}()
	hdr := [1]uint64{1}
	p.log.write(nil, 0, hdr[:], p.extra[i+1:i+int(p.extra[i])])
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, ".write(") && strings.Contains(line, "let __seq_holder") {
			t.Fatalf("complex method-call arguments should not stay on the opening call line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, ".write(\n") {
		t.Fatalf("complex method-call arguments should use multiline call syntax:\n%s", rust)
	}
}

func TestConcurrentMethodCallBreaksManyArgumentsAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type writer struct{}

func (w writer) write(a int, b int, c int, d int, e int) writer {
	return w
}

type sample struct {
	a int
	b int
	c int
	d int
	e int
}

func update(w writer, s *sample) writer {
	go func() {}()
	w = w.write(s.a, s.b, s.c, s.d, s.e)
	return w
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, ".write(") && strings.Contains(line, "s.lock()") {
			t.Fatalf("many method-call arguments should not stay on the opening call line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, ".write(\n") {
		t.Fatalf("many method-call arguments should use multiline call syntax:\n%s", rust)
	}
}

func TestConcurrentPointerReceiverMethodCallBreaksFunctionValueArgumentAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type timer struct{}
type g struct{}

func ready(arg any, seq uintptr, delay int64) {}

func (t *timer) init(f func(any, uintptr, int64), arg any) {}

func use(t *timer, gp *g) {
	go func() {}()
	t.init(ready, gp)
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "let __recv =") && strings.Contains(line, "Box::new(move |") {
			t.Fatalf("pointer receiver method call with function-value argument should not stay on one line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, ".init(\n") {
		t.Fatalf("pointer receiver method call with function-value argument should use multiline call syntax:\n%s", rust)
	}
}

func TestConcurrentFunctionCallBreaksComplexArgumentsAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func writeFD(fd int, p []byte) (int, error) {
	return 0, nil
}

func retry(fn func(int, []byte) (int, error), fd int, p []byte) (int, error) {
	return fn(fd, p)
}

func writeAll(fd int, p []byte, nn int, max int) int {
	go func() {}()
	n, _ := retry(writeFD, fd, p[nn:max])
	return n
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "retry(") && strings.Contains(line, "let __seq_holder") {
			t.Fatalf("complex function-call arguments should not stay on the opening call line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "retry(\n") {
		t.Fatalf("complex function-call arguments should use multiline call syntax:\n%s", rust)
	}
}

func TestConcurrentFunctionCallBreaksTwoComplexArgumentsAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func hash(left []byte, right []byte) int {
	return 0
}

func call(left []byte, right []byte, i int, j int) int {
	go func() {}()
	return hash(left[i:j], right[i:j])
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "hash(") && strings.Contains(line, "let __seq_holder") {
			t.Fatalf("two complex function-call arguments should not stay on the opening call line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "hash(\n") {
		t.Fatalf("two complex function-call arguments should use multiline call syntax:\n%s", rust)
	}
}

func TestConcurrentFunctionCallBreaksSingleFunctionValueArgumentAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type timers struct{}

func (ts *timers) run(now int64) int64 {
	return now
}

func pc(fn func(*timers, int64) int64) uintptr {
	return 1
}

func start(v uintptr) uintptr {
	return v
}

const quantum uintptr = 4

func use() uintptr {
	go func() {}()
	return start(pc((*timers).run) + quantum)
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "start(") && strings.Contains(line, "Box::new(move |") {
			t.Fatalf("single function-value call argument should not stay on the opening call line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "start(\n") {
		t.Fatalf("single function-value call argument should use multiline call syntax:\n%s", rust)
	}
}

func TestConcurrentFunctionCallBuildsFunctionValueBinaryArgumentWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type timers struct{}

func (ts *timers) run(now int64) int64 {
	return now
}

func pc(fn func(*timers, int64) int64) uintptr {
	return 1
}

func start(v uintptr) uintptr {
	return v
}

const quantum uintptr = 4

func use() uintptr {
	go func() {}()
	return start(pc((*timers).run) + quantum)
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "Arc::new(Mutex::new(Some({ let __tmp_x = pc(") {
			t.Fatalf("function-value binary call argument should not keep the wrapper body on one line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "let __go_binary_") {
		t.Fatalf("function-value binary call argument should build binary operands with statements:\n%s", rust)
	}
}

func TestConcurrentFunctionValueAnyArgumentBuildsMethodExpressionWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type timers struct{}

func (ts *timers) run(now int64) int64 {
	return now
}

func pc(fn any) uintptr {
	return 1
}

func start(v uintptr) uintptr {
	return v
}

const quantum uintptr = 4

func use() uintptr {
	go func() {}()
	return start(pc((*timers).run) + quantum)
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "let __go_binary_") && strings.Contains(line, "Box::new(move |") {
			t.Fatalf("method-expression function value boxed into any should not stay on one line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "let __func_value =") {
		t.Fatalf("method-expression function value boxed into any should use a temporary handle:\n%s", rust)
	}
}

func TestConcurrentBinaryOperandBreaksComplexCallArgumentsAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func hash(left []byte, right []byte) int {
	return 0
}

func call(left []byte, right []byte, i int, j int, mask int) int {
	go func() {}()
	return hash(left[i:j], right[i:j]) & mask
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "let __tmp_x = hash(") {
			t.Fatalf("complex call operand should not stay on the binary temp assignment line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "let __tmp_x =\n") || !strings.Contains(rust, "hash(\n") {
		t.Fatalf("complex call operand should use multiline binary temp and call syntax:\n%s", rust)
	}
}

func TestConcurrentMultiReturnBreaksComplexResultsAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func read(data []uint64, tags []uintptr, di int, ti int) ([]uint64, []uintptr, bool) {
	go func() {}()
	return data[:di], tags[:ti], false
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "__seq_holder = data.clone()") &&
			strings.Contains(line, "__seq_holder = tags.clone()") {
			t.Fatalf("complex multi-result return values should not stay on one tuple line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "(\n") {
		t.Fatalf("complex multi-result return should use multiline tuple syntax:\n%s", rust)
	}
}

func TestConcurrentFixedArrayCallArgumentBreaksComplexElementsAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func word(buf []byte) uint64 {
	return uint64(buf[0])
}

type State struct{}

func (s *State) Init64(seed [4]uint64) {}

func (s *State) Init(seed [32]byte) {
	go func() {}()
	s.Init64([4]uint64{
		word(seed[0*8:]),
		word(seed[1*8:]),
		word(seed[2*8:]),
		word(seed[3*8:]),
	})
}
`)

	if strings.Contains(rust, "Some([word(") {
		t.Fatalf("complex fixed-array call argument should not inline every element in one array expression:\n%s", rust)
	}
	if !strings.Contains(rust, "Some([\n") {
		t.Fatalf("complex fixed-array call argument should break array elements across lines:\n%s", rust)
	}
}

func TestConcurrentIndexedSwapBreaksComplexParallelAssignmentAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type G struct{}

type P struct {
	runq [8]*G
	runqtail uint32
}

func shuffle(pp *P, n uint32) {
	go func() {}()
	off := func(o uint32) uint32 {
		return (pp.runqtail + o) % uint32(len(pp.runq))
	}
	for i := uint32(1); i < n; i++ {
		j := i - 1
		pp.runq[off(i)], pp.runq[off(j)] = pp.runq[off(j)], pp.runq[off(i)]
	}
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "let __tmp_0 =") && strings.Contains(line, "let __tmp_1 =") {
			t.Fatalf("complex parallel assignment temps should be emitted across lines:\n%s", rust)
		}
		if strings.Contains(line, "let __tmp_1 =") {
			return
		}
	}
	t.Fatalf("complex parallel assignment should keep the second temp on its own line:\n%s", rust)
}

func TestTupleAssignmentFromBareScalarReturnSlots(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func pair() (int, bool) {
	return 1, true
}

func assign() int {
	var x int
	var ok bool
	x, ok = pair()
	if ok {
		return x
	}
	return 0
}

func short() int {
	x, ok := pair()
	if ok {
		return x
	}
	return 0
}
`)

	if !strings.Contains(rust, "pub fn pair() -> (i32, bool)") {
		t.Fatalf("tuple return signature should use bare scalar slots:\n%s", rust)
	}
	if strings.Contains(rust, "__tmp_0.borrow()") || strings.Contains(rust, "__tmp_0.lock()") ||
		strings.Contains(rust, "__tmp_1.borrow()") || strings.Contains(rust, "__tmp_1.lock()") {
		t.Fatalf("tuple reassignment from bare scalar slots should not borrow temporaries:\n%s", rust)
	}
	if !strings.Contains(rust, "= Some(__tmp_0)") || !strings.Contains(rust, "= Some(__tmp_1)") {
		t.Fatalf("tuple reassignment should store bare scalar temps into existing wrapped locals:\n%s", rust)
	}
	if !strings.Contains(rust, "let (mut x, mut ok) = pair();") {
		t.Fatalf("tuple short declaration should bind bare scalar results directly:\n%s", rust)
	}
	if !strings.Contains(rust, "if ok {") || !strings.Contains(rust, "return x;") {
		t.Fatalf("tuple short-declared bare scalar locals should be used directly:\n%s", rust)
	}
}

func TestConcurrentTupleReassignmentCallArgumentsBreakAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type info struct {
	pcfile uint32
	pcln uint32
}

func pair(f info, off uint32, target uint32, strict bool) (int32, uint32) {
	if strict {
		return int32(off + target), f.pcfile
	}
	return int32(off), f.pcln
}

func assign(f *info, target uint32, strict bool) int32 {
	go func() {}()
	var line int32
	line, _ = pair(*f, f.pcln, target, strict)
	return line
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "{ let (__tmp_0, __tmp_1) = pair(") && strings.Contains(line, "__arg_holder") {
			t.Fatalf("tuple reassignment call arguments should not stay on one line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "{ let (__tmp_0, __tmp_1) = pair(\n") {
		t.Fatalf("tuple reassignment call should break arguments across lines:\n%s", rust)
	}
}

func TestConcurrentTupleReassignmentMethodFieldReceiverBreaksAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type errString string

func (e errString) Error() string { return string(e) }

type pfd struct{}

func (p *pfd) seek(offset int64, whence int) (int64, error) {
	return offset, errString("x")
}

type file struct {
	pfd *pfd
}

func (f *file) Seek(offset int64, whence int) (ret int64, err error) {
	go func() {}()
	ret, err = f.pfd.seek(offset, whence)
	return
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "let (__tmp_0, __tmp_1) =") && strings.Contains(line, "*ret") {
			t.Fatalf("tuple reassignment method call should not keep assignments on the tuple line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "\n            let (__tmp_0, __tmp_1) =") {
		t.Fatalf("tuple reassignment method call should use a multiline block:\n%s", rust)
	}
}

func TestTupleBareScalarShortDeclWrapsForGeneratedCallArgument(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func pair() (int, bool) {
	return 1, true
}

func use(v int) int {
	return v
}

func caller() int {
	x, ok := pair()
	if ok {
		return use(x)
	}
	return 0
}
`)

	if strings.Contains(rust, "use(x.clone())") {
		t.Fatalf("bare scalar local must be wrapped when passed to a generated wrapped parameter:\n%s", rust)
	}
	if !strings.Contains(rust, "use(Rc::new(RefCell::new(Some(x))))") &&
		!strings.Contains(rust, "use(Arc::new(Mutex::new(Some(x))))") {
		t.Fatalf("bare scalar local should be wrapped for generated call argument:\n%s", rust)
	}
}

func TestMixedTupleShortDeclKeepsExistingWrappedLocal(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func round(n uint64) (uint64, bool) {
	return n, true
}

func caller(seed uint64) uint64 {
	size := seed + 1
	size, ok := round(size)
	if ok {
		return size
	}
	return 0
}
`)

	if strings.Contains(rust, "round(Rc::new(RefCell::new(Some(size))))") ||
		strings.Contains(rust, "round(Arc::new(Mutex::new(Some(size))))") {
		t.Fatalf("mixed tuple short declaration must evaluate RHS with the existing wrapped local:\n%s", rust)
	}
	if strings.Contains(rust, "let (mut size, mut ok) = round") {
		t.Fatalf("mixed tuple short declaration must not redeclare the existing local:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(__tmp_0)") || !strings.Contains(rust, "mut ok") {
		t.Fatalf("mixed tuple short declaration should assign the existing local from a temp and declare the new name:\n%s", rust)
	}
	if !strings.Contains(rust, "; *size.borrow_mut() = Some(__tmp_0);") &&
		!strings.Contains(rust, "; *size.lock().unwrap() = Some(__tmp_0);") {
		t.Fatalf("mixed tuple short declaration should terminate the tuple binding before temp assignment:\n%s", rust)
	}
}

func TestMixedMultiRhsShortDeclKeepsExistingBareScalar(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func split(n uint32) (uint32, int, bool) {
	return n, 0, true
}

func caller(seed uint32, extra uint) uint64 {
	di, _, _ := split(seed)
	mask := uint32(7)
	di, dfrac := di>>extra, di&mask
	if di&1 == 1 {
		return uint64(di + dfrac)
	}
	return uint64(di)
}
`)

	if strings.Contains(rust, "let (mut di, mut dfrac)") {
		t.Fatalf("mixed multi-RHS short declaration must not redeclare the existing bare scalar:\n%s", rust)
	}
	if !strings.Contains(rust, "let (__tmp_0, mut dfrac) =") {
		t.Fatalf("mixed multi-RHS short declaration should keep a temp for the existing bare scalar:\n%s", rust)
	}
	if !strings.Contains(rust, "di = __tmp_0;") {
		t.Fatalf("mixed multi-RHS short declaration should assign the existing bare scalar directly:\n%s", rust)
	}
	if strings.Contains(rust, "di.borrow()") || strings.Contains(rust, "di.lock()") {
		t.Fatalf("bare scalar should remain bare after mixed multi-RHS short declaration:\n%s", rust)
	}
	if !strings.Contains(rust, "if di & 1 as u32 == 1 as u32 {") {
		t.Fatalf("later bitwise use of existing scalar should stay bare:\n%s", rust)
	}
	if !strings.Contains(rust, "di as u64") {
		t.Fatalf("later numeric conversion of existing scalar should stay bare:\n%s", rust)
	}
}

func TestTupleShortDeclReregistersNewBareBoolAfterWrappedShadow(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type iter struct{}

func (it *iter) pair(seed int) (int, int, bool) {
	return seed, seed + 1, true
}

func caller(it *iter, seed int) int {
	if seed > 0 {
		var ok bool
		a, b, ok := it.pair(seed)
		_ = a
		_ = b
		if ok {
			return 1
		}
	}
	x, _, ok := it.pair(seed)
	if !ok {
		return 0
	}
	return x
}
`)

	if !strings.Contains(rust, "let (mut x, _, mut ok) = it.pair") &&
		!strings.Contains(rust, "let (mut x, _, mut ok) = (*it") {
		t.Fatalf("later tuple short declaration should bind a new bare ok result:\n%s", rust)
	}
	if !strings.Contains(rust, "if !ok {") {
		t.Fatalf("new tuple bool result should be used as a bare bool, not through a stale wrapped ok:\n%s", rust)
	}
}

func TestStringsCutExistingBoolTupleAssignmentUsesBareOkTemp(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "strings"

func caller(s string) bool {
	var ok bool
	var after string
	_, after, ok = strings.Cut(s, ".")
	return ok && after != ""
}
`)

	if strings.Contains(rust, "__tmp_2.lock()") {
		t.Fatalf("strings.Cut bool tuple slot should be assigned as a bare temp:\n%s", rust)
	}
	if !strings.Contains(rust, "*ok.lock().unwrap() = Some(__tmp_2);") &&
		!strings.Contains(rust, "*ok.borrow_mut() = Some(__tmp_2);") {
		t.Fatalf("existing bool assignment from strings.Cut should store the bare bool in the wrapped local:\n%s", rust)
	}
}

func TestTupleStringResultCallArgumentUsesWrappedLocal(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "strings"

func caller(s string) []string {
	line, _, _ := strings.Cut(s, ":")
	return strings.Fields(line)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"strings": "strings"})

	if strings.Contains(rust, "Some(line.clone())") {
		t.Fatalf("string tuple result should not wrap the returned handle as a String value:\n%s", rust)
	}
	if !strings.Contains(rust, "__arg_holder = line.clone()") {
		t.Fatalf("string tuple result should be treated as a wrapped local call argument:\n%s", rust)
	}
}

func TestSingleMultiResultCallExpandsIntoFunctionArguments(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func pair() (int, error) {
	return 1, nil
}

func fix(n int, err error) (int, error) {
	return n, err
}

func caller() (int, error) {
	return fix(pair())
}
`)

	if strings.Contains(rust, "fix(pair())") {
		t.Fatalf("single multi-result call should not be passed as one Rust argument:\n%s", rust)
	}
	if !strings.Contains(rust, "let (__multi_arg_0, __multi_arg_1) = pair()") {
		t.Fatalf("single multi-result call should be bound once before argument expansion:\n%s", rust)
	}
	if !strings.Contains(rust, "fix(Rc::new(RefCell::new(Some(__multi_arg_0))), __multi_arg_1)") &&
		!strings.Contains(rust, "fix(Arc::new(Mutex::new(Some(__multi_arg_0))), __multi_arg_1)") {
		t.Fatalf("expanded scalar result slot should be wrapped for the matching parameter:\n%s", rust)
	}
}

func TestSingleResultCallArgumentDoesNotUseMultiResultExpansion(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func inner() int {
	return 1
}

func outer(n int) int {
	return n
}

func caller() int {
	return outer(inner())
}
`)

	if strings.Contains(rust, "__multi_arg_0") {
		t.Fatalf("single-result call argument should not use multi-result expansion:\n%s", rust)
	}
}

func TestFunctionValueCallWithCallArgumentUsesClosureHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func inner() int {
	return 1
}

func walk(yield func(int) bool) {
	if !yield(inner()) {
		return
	}
}
`)

	if strings.Contains(rust, "yield(__multi_arg_0)") {
		t.Fatalf("function value call should not be lowered as a direct function call:\n%s", rust)
	}
	if !strings.Contains(rust, "*mut Box<dyn FnMut") {
		t.Fatalf("function value call should invoke the closure handle:\n%s", rust)
	}
}

func TestNewFunctionValueInitializesPointerToNil(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func alloc() *func() {
	go func() {}()
	return new(func())
}
`)

	if strings.Contains(rust, "Box::<dyn FnMut() -> () + Send + Sync>::default()") ||
		strings.Contains(rust, "Some(Default::default())") {
		t.Fatalf("new(func()) should allocate a nil function slot, not default a boxed function:\n%s", rust)
	}
	if !strings.Contains(rust, "Arc::new(Mutex::new(None") {
		t.Fatalf("new(func()) should initialize the function slot to None:\n%s", rust)
	}
}

func TestFunctionTypedFieldAssignmentUsesMutableStructTarget(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type group struct {
	wait func(int) bool
}

func install(gp *group, fn func(int) bool) {
	gp.wait = fn
}
`)

	if strings.Contains(rust, ".as_ref().unwrap()).wait = new_val") {
		t.Fatalf("function-typed field assignment through a wrapped pointer should not use an immutable struct borrow:\n%s", rust)
	}
	if !strings.Contains(rust, "(*gp.borrow_mut().as_mut().unwrap()).wait = new_val;") &&
		!strings.Contains(rust, "(*gp.lock().unwrap().as_mut().unwrap()).wait = new_val;") {
		t.Fatalf("function-typed field assignment through a wrapped pointer should mutate the struct field handle:\n%s", rust)
	}
}

func TestPointerFunctionAssignmentFromCallMovesReturnedSlot(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func makeFunc(name string) func() {
	return func() {}
}

func use(slot *func(), name string) {
	go func() {}()
	*slot = makeFunc(name)
}
`)

	if strings.Contains(rust, ").lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("pointer function assignment from call should not clone the inner function box:\n%s", rust)
	}
	if !strings.Contains(rust, "let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }") {
		t.Fatalf("pointer function assignment from call should move the returned function slot:\n%s", rust)
	}
}

func TestPointerFunctionCallUsesSlotHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func use(slot *func(string) func(), name string) func() {
	go func() {}()
	return (*slot)(name)
}
`)

	if strings.Contains(rust, "((*slot.lock().unwrap().as_ref().unwrap()))") {
		t.Fatalf("pointer function call should not clone the inner function box as the call target:\n%s", rust)
	}
	if !strings.Contains(rust, "let __f_holder = slot.clone();") {
		t.Fatalf("pointer function call should use the function slot handle:\n%s", rust)
	}
}

func TestTupleStringResultRegistersWrappedSlot(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "strings"

func caller(s string) bool {
	line, _, ok := strings.Cut(s, ":")
	return ok && line != ""
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	fn := file.Decls[len(file.Decls)-1].(*ast.FuncDecl)
	assign := fn.Body.List[0].(*ast.AssignStmt)
	call := assign.Rhs[0].(*ast.CallExpr)

	prevTypeInfo := currentTypeInfo
	prevVarTable := currentVarTable
	SetTypeInfo(typeInfo)
	SetVarTable(NewVarTable())
	defer func() {
		SetTypeInfo(prevTypeInfo)
		SetVarTable(prevVarTable)
	}()

	registerCallTupleResultSyntaxInfo(assign.Lhs, call)

	lineInfo := lookupVarInfo("line")
	if lineInfo == nil || lineInfo.WrapLevel != WrapFull || lineInfo.RustType != "String" {
		t.Fatalf("line tuple slot should be registered as wrapped String, got %#v", lineInfo)
	}
	okInfo := lookupVarInfo("ok")
	if okInfo == nil || okInfo.WrapLevel != WrapNone {
		t.Fatalf("ok tuple slot should stay registered as bare bool, got %#v", okInfo)
	}
}

func TestTupleStringResultShadowingBareRangeVarRegistersWrappedLocal(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "strings"

func caller(s string) []string {
	var out []string
	for _, line := range strings.Split(s, "\n") {
		line, _, _ := strings.Cut(line, ":")
		out = strings.Fields(line)
	}
	return out
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"strings": "strings"})

	if strings.Contains(rust, "strings::fields(Rc::new(RefCell::new(Some(line.clone()))") ||
		strings.Contains(rust, "strings::fields(Arc::new(Mutex::new(Some(line.clone()))") {
		t.Fatalf("shadowed string tuple result should override the bare range-var metadata:\n%s", rust)
	}
	if !strings.Contains(rust, "__arg_holder = line.clone()") {
		t.Fatalf("shadowed string tuple result should be treated as the wrapped tuple local:\n%s", rust)
	}
}

func TestTupleResultRegistrationOverridesRangeVarInfo(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "strings"

func caller(s string) {
	for _, line := range strings.Split(s, "\n") {
		line, _, _ := strings.Cut(line, ":")
		_ = line
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
	fn := file.Decls[len(file.Decls)-1].(*ast.FuncDecl)
	rangeStmt := fn.Body.List[0].(*ast.RangeStmt)
	assign := rangeStmt.Body.List[0].(*ast.AssignStmt)
	call := assign.Rhs[0].(*ast.CallExpr)

	prevTypeInfo := currentTypeInfo
	prevVarTable := currentVarTable
	SetTypeInfo(typeInfo)
	vt := NewVarTable()
	vt.Register("line", &VarInfo{WrapLevel: WrapNone, RustType: "String", Source: SourceRangeVal})
	SetVarTable(vt)
	defer func() {
		SetTypeInfo(prevTypeInfo)
		SetVarTable(prevVarTable)
	}()

	registerCallTupleResultSyntaxInfo(assign.Lhs, call)

	lineInfo := lookupVarInfo("line")
	if lineInfo == nil || lineInfo.WrapLevel != WrapFull || lineInfo.Source != SourceLocal {
		t.Fatalf("tuple result registration should override the shadowed range var, got %#v", lineInfo)
	}
}

func TestTupleAssignmentToBareRangeStringUnwrapsWrappedTemp(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func expand(s string) (string, bool) {
	return s, true
}

func caller(args []string) []string {
	ok := true
	for i, arg := range args {
		arg, ok = expand(arg)
		if !ok {
			return args
		}
		args[i] = arg
	}
	return args
}
`)

	if strings.Contains(rust, "arg = __tmp_0;") {
		t.Fatalf("tuple assignment to a bare range string should not store the wrapped temp handle:\n%s", rust)
	}
	if !strings.Contains(rust, "arg = { let __tmp_holder = __tmp_0.clone();") {
		t.Fatalf("tuple assignment to a bare range string should unwrap the temp into the bare local:\n%s", rust)
	}
}

func TestShortDeclFromBareRangeStringClonesValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func caller(lines []string) string {
	var out string
	for _, line := range lines {
		orig := line
		line = line + "x"
		out = orig
	}
	return out
}
`)

	if strings.Contains(rust, "let mut orig = Rc::new(RefCell::new(Some(line)))") ||
		strings.Contains(rust, "let mut orig = Arc::new(Mutex::new(Some(line)))") {
		t.Fatalf("short declaration from a bare range string should not move the range value:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut orig = Rc::new(RefCell::new(Some(line.clone())))") &&
		!strings.Contains(rust, "let mut orig = Arc::new(Mutex::new(Some(line.clone())))") {
		t.Fatalf("short declaration from a bare range string should clone into the wrapper:\n%s", rust)
	}
}

func TestBareScalarAssignmentFromWrappedLocalUnwrapsRHS(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func pair() (int, bool) {
	return 1, true
}

func caller() int {
	x, _ := pair()
	var y int
	y = 2
	x = y
	return x
}
`)

	if strings.Contains(rust, "*x.borrow_mut()") || strings.Contains(rust, "*x.lock().unwrap()") {
		t.Fatalf("assignment to a bare scalar local should not use wrapper mutation:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = (*y.borrow().as_ref().unwrap())") &&
		!strings.Contains(rust, "let new_val = (*y.lock().unwrap().as_ref().unwrap())") {
		t.Fatalf("assignment to a bare scalar local should unwrap the wrapped RHS:\n%s", rust)
	}
	if !strings.Contains(rust, "x = new_val;") {
		t.Fatalf("assignment to a bare scalar local should assign the Rust local directly:\n%s", rust)
	}
}

func TestBareScalarAssignmentFromSelectorUnwrapsField(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type checker struct {
	nextID uint64
}

func nextID() uint64 {
	return 1
}

func (check *checker) newTypeParam() uint64 {
	id := nextID()
	check.nextID++
	id = check.nextID
	return id
}
`)

	if strings.Contains(rust, "id = new_val;") && strings.Contains(rust, "let new_val = self.next_i_d.clone();") {
		t.Fatalf("bare scalar assignment from selector should not store the field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __v = self.next_i_d.clone(); let __owned = (*__v") {
		t.Fatalf("bare scalar assignment from selector should copy the field value:\n%s", rust)
	}
}

func TestShortDeclShadowingBareTupleResultRegistersWrappedLocal(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type box struct {
	flag bool
}

func source() (int, bool) {
	return 0, true
}

func flip(b *box) bool {
	_, flag := source()
	if flag {
		flag := true
		flag = !flag
		b.flag = flag
		return flag
	}
	return flag
}
`)

	if strings.Contains(rust, "let new_val = !flag; flag = new_val;") {
		t.Fatalf("shadowed wrapped bool local should not inherit the outer bare tuple slot:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut flag = ") ||
		(!strings.Contains(rust, "Rc::new(RefCell::new(Some(true)))") &&
			!strings.Contains(rust, "Arc::new(Mutex::new(Some(true)))")) {
		t.Fatalf("inner bool short declaration should remain a wrapped local:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = !(*flag.borrow().as_ref().unwrap())") &&
		!strings.Contains(rust, "let new_val = !(*flag.lock().unwrap().as_ref().unwrap())") {
		t.Fatalf("shadowed wrapped bool assignment should unwrap the inner local:\n%s", rust)
	}
}

func TestSliceTypeAssertionAssignmentStoresAssertedSliceInHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type struct{}

func load(v any) []*Type {
	var rts []*Type
	if v != nil {
		rts = v.([]*Type)
	}
	return rts
}
`)

	if !strings.Contains(rust, "downcast_ref::<Vec<Rc<RefCell<Option<Type>>>>>()") &&
		!strings.Contains(rust, "downcast_ref::<Vec<Arc<Mutex<Option<Type>>>>>()") {
		t.Fatalf("slice type assertion should assert the bare slice value:\n%s", rust)
	}
	if strings.Contains(rust, "rts = new_val;") {
		t.Fatalf("slice type assertion assignment should not replace the slice handle with a bare Vec:\n%s", rust)
	}
	if !strings.Contains(rust, "*rts.borrow_mut() = Some(new_val);") &&
		!strings.Contains(rust, "*rts.lock().unwrap() = Some(new_val);") {
		t.Fatalf("slice type assertion assignment should store the asserted Vec in the existing handle:\n%s", rust)
	}
}

func TestTypeAssertionCommaOkBindsRawBool(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func reassert(v any) bool {
	_, ok := v.(*int)
	if !ok {
		return false
	}
	_, ok = v.(*string)
	return ok
}
`)

	if strings.Contains(rust, "Rc::new(RefCell::new(Some(true)))") ||
		strings.Contains(rust, "Arc::new(Mutex::new(Some(true)))") {
		t.Fatalf("type assertion comma-ok result should be a raw bool:\n%s", rust)
	}
	if !strings.Contains(rust, "let (_, mut ok) =") || !strings.Contains(rust, "let (__tmp_0, __tmp_1) =") {
		t.Fatalf("type assertion short declaration and reassignment should keep tuple shape:\n%s", rust)
	}
}

func TestPointerTupleAssignmentRebindsLocalHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Alias struct{}
type Other struct{}

func reassign(v any) bool {
	a0 := &Alias{}
	a := a0
	for a != nil {
		v = &Other{}
		a, _ = v.(*Alias)
	}
	return a0 != nil
}
`)

	if strings.Contains(rust, "*a.borrow_mut() = __moved_tmp_0") ||
		strings.Contains(rust, "*a.lock().unwrap() = __moved_tmp_0") {
		t.Fatalf("tuple assignment to a pointer local should not mutate the previous pointed handle:\n%s", rust)
	}
	if !strings.Contains(rust, "a = __tmp_0.clone();") {
		t.Fatalf("tuple assignment to a pointer local should rebind the local handle from the tuple temp:\n%s", rust)
	}
}

func TestMapCommaOkBindsRawBool(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func relookup(m map[string]int) bool {
	_, ok := m["first"]
	if !ok {
		return false
	}
	_, ok = m["second"]
	return ok
}
`)

	if strings.Contains(rust, "Rc::new(RefCell::new(Some(true)))") ||
		strings.Contains(rust, "Arc::new(Mutex::new(Some(true)))") {
		t.Fatalf("map comma-ok result should be a raw bool:\n%s", rust)
	}
	if !strings.Contains(rust, "/* MAP_COMMA_OK */ Some(v) => (v.clone(), true)") ||
		!strings.Contains(rust, "let (__tmp_0, __tmp_1) = { let __map_holder") {
		t.Fatalf("map comma-ok short declaration and reassignment should keep tuple shape:\n%s", rust)
	}
}

func TestMapAssignmentWrapsBareBoolValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func decide() bool {
	return true
}

func store(m map[string]bool, key string) {
	readOnly := decide()
	m[key] = readOnly
}
`)

	if strings.Contains(rust, "readOnly.borrow") || strings.Contains(rust, "readOnly.lock()") {
		t.Fatalf("map assignment should wrap bare bool values without borrowing:\n%s", rust)
	}
	if !strings.Contains(rust, "let __map_value = Rc::new(RefCell::new(Some(readOnly)))") &&
		!strings.Contains(rust, "let __map_value = Arc::new(Mutex::new(Some(readOnly)))") {
		t.Fatalf("map assignment should store the bare bool inside a fresh handle:\n%s", rust)
	}
}

func TestMapAssignmentCastsBareRangeIndexValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func positions(items []string) map[string]int {
	result := map[string]int{}
	for i, item := range items {
		result[item] = i
	}
	return result
}
`)

	if !strings.Contains(rust, "let __map_value = Rc::new(RefCell::new(Some(i as i32)))") &&
		!strings.Contains(rust, "let __map_value = Arc::new(Mutex::new(Some(i as i32)))") {
		t.Fatalf("map assignment should cast bare range indexes to Go int values:\n%s", rust)
	}
}

func TestMapAssignmentKeepsRangeMapValueHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func copyValues(src map[int]string) map[int]string {
	dst := map[int]string{}
	for k, v := range src {
		dst[k] = v
	}
	return dst
}
`)

	if strings.Contains(rust, "Some(v.clone())") {
		t.Fatalf("map assignment from a range map value should not double-wrap the value handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __map_value = Rc::new(RefCell::new(Some((*v.borrow().as_ref().unwrap()).clone())))") &&
		!strings.Contains(rust, "let __map_value = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()).clone())))") {
		t.Fatalf("map assignment from a range map value should copy the inner value into a fresh handle:\n%s", rust)
	}
}

func TestMapAssignmentClonesWrappedStringKey(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func store(m map[string]int, key string, value int) {
	m[key] = value
}
`)

	if strings.Contains(rust, "let __map_key = (*key.borrow().as_ref().unwrap());") ||
		strings.Contains(rust, "let __map_key = (*key.lock().unwrap().as_ref().unwrap());") {
		t.Fatalf("map assignment should not move a wrapped string key out of its handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __map_key = (*key.borrow().as_ref().unwrap()).clone();") &&
		!strings.Contains(rust, "let __map_key = (*key.lock().unwrap().as_ref().unwrap()).clone();") {
		t.Fatalf("map assignment should clone a wrapped string key:\n%s", rust)
	}
}

func TestMapLookupCastsBareRangeIndexKey(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func firstHit(items []string, table map[int]string) int {
	for i := range items {
		if _, ok := table[i]; ok {
			return i
		}
	}
	return -1
}
`)

	if !strings.Contains(rust, ".get(&(i as i32))") {
		t.Fatalf("map lookup should cast bare range indexes to Go int keys:\n%s", rust)
	}
}

func TestSliceAssignmentFromPointerDerefClonesPointeeHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Iovec struct{}

type file struct {
	iovecs *[]Iovec
}

func use(f *file) {
	var iovecs []Iovec
	if f.iovecs != nil {
		iovecs = *f.iovecs
	}
	_ = iovecs
}
`)

	if strings.Contains(rust, "let __v = (*") && strings.Contains(rust, "iovecs = new_val") {
		t.Fatalf("slice assignment from pointer dereference should not assign a raw Vec into the slice handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".iovecs.clone(); iovecs = new_val") {
		t.Fatalf("slice assignment from pointer dereference should clone the pointee slice handle:\n%s", rust)
	}
}

func TestPointerToSliceDerefAssignmentFromLocalSliceClonesHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Iovec struct{}

type file struct {
	iovecs *[]Iovec
}

func store(f *file, iovecs []Iovec) {
	if f.iovecs == nil {
		f.iovecs = new([]Iovec)
	}
	*f.iovecs = iovecs
}
`)

	if strings.Contains(rust, "let __v = (*iovecs.") && strings.Contains(rust, "new_val.borrow") {
		t.Fatalf("pointer-to-slice assignment from local slice should not unwrap the RHS handle before copying:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = iovecs.clone(); let __cloned_val") {
		t.Fatalf("pointer-to-slice assignment from local slice should clone the RHS slice handle:\n%s", rust)
	}
}

func TestNamedSliceFieldSliceAssignmentStoresNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

type Float struct {
	mant nat
}

func trim(z *Float, n uint32) {
	z.mant = z.mant[:n]
}
`)

	if !strings.Contains(rust, "let new_val = nat(") {
		t.Fatalf("named-slice slice assignment should construct the named value:\n%s", rust)
	}
	if strings.Contains(rust, "new_val.borrow_mut()") || strings.Contains(rust, "new_val.lock().unwrap()") {
		t.Fatalf("named-slice slice assignment should not treat the named value as a wrapper handle:\n%s", rust)
	}
	if strings.Contains(rust, "__moved_val") {
		t.Fatalf("named-slice slice assignment should store the named value, not move from a wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, " = Some(new_val); }") {
		t.Fatalf("named-slice slice assignment should store the named value in the existing slot:\n%s", rust)
	}
}

func TestPackageGlobalNamedSliceSliceAssignmentStoresNamedValue(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Word uint32
type mask []Word

var idle mask

func trim(n int) {
	idle = idle[:n]
}

func main() {
	go func() {}()
}
`)

	if strings.Contains(rust, "__collection_holder = mask(") ||
		strings.Contains(rust, "__collection_holder.lock()") {
		t.Fatalf("package-global named-slice slice assignment should not treat the named value as a wrapper handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = Some(mask(") {
		t.Fatalf("package-global named-slice slice assignment should construct the named value in the global slot:\n%s", rust)
	}
}

func TestPackageGlobalNamedSliceAssignmentFromUnnamedSliceWrapsNamedValue(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Word uint32
type mask []Word

var idle mask

func grow(n int) {
	next := make([]Word, n)
	idle = next
}

func main() {
	go func() {}()
}
`)

	if strings.Contains(rust, "*idle.lock().unwrap() = new_val") &&
		strings.Contains(rust, "let __collection_holder = next.clone()") {
		t.Fatalf("package-global named-slice assignment from unnamed slice should not store the raw slice option:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = Some(mask(next.clone()))") {
		t.Fatalf("package-global named-slice assignment from unnamed slice should wrap the slice handle in the named value:\n%s", rust)
	}
}

func TestUnsafePointerDerefAssignmentSkipsInvalidRHS(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "unsafe"

func store(slot *unsafe.Pointer, tagPtr *unsafe.Pointer) {
	if tagPtr != nil {
		*(*uintptr)(unsafe.Pointer(slot)) = uintptr(*tagPtr)
	}
}

func main() {
	go func() {}()
}
`)

	if strings.Contains(rust, "let _ =") && strings.Contains(rust, "unsafe.Pointer dereference assignment") {
		t.Fatalf("unsupported unsafe pointer assignment should not lower the RHS before panicking:\n%s", rust)
	}
	if !strings.Contains(rust, "unimplemented!(\"unsafe.Pointer dereference assignment\")") {
		t.Fatalf("unsupported unsafe pointer assignment should panic loudly:\n%s", rust)
	}
}

func TestUnsafePointerSliceElementNilAssignmentUsesZero(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "unsafe"

func clear(tags []unsafe.Pointer, i int) {
	tags[i] = nil
}

func main() {
	go func() {}()
}
`)

	if strings.Contains(rust, "] = None") {
		t.Fatalf("unsafe.Pointer slice element nil assignment should not store Option::None in a usize slot:\n%s", rust)
	}
	if !strings.Contains(rust, "] = 0") {
		t.Fatalf("unsafe.Pointer slice element nil assignment should store the zero pointer value:\n%s", rust)
	}
}

func TestNamedSliceSelectorShortDeclCopiesNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

type Float struct {
	mant nat
}

func (x *Float) bits() int {
	m := x.mant
	return len(m)
}
`)

	if strings.Contains(rust, "Some(self.mant.clone())") {
		t.Fatalf("named-slice selector short declaration should not store the field handle inside the new wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "let __selector_holder = self.mant.clone()") {
		t.Fatalf("named-slice selector short declaration should clone the named field value:\n%s", rust)
	}
}

func TestTypeParamSliceElementShortDeclUsesHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func pick[S ~[]E, E any](values S, i int, use func(E) bool) bool {
	v := values[i]
	return use(v)
}
`)

	if strings.Contains(rust, "Some({ let __seq") {
		t.Fatalf("type-parameter slice element short declaration should not double-wrap the element handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut v = (*values") {
		t.Fatalf("type-parameter slice element short declaration should bind the existing element handle:\n%s", rust)
	}
}

func TestTypeParamZeroValueUsesNilHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func miss[T any]() (T, bool) {
	return *new(T), false
}

func store[T any](ok bool, value T) (T, bool) {
	if ok {
		return value, true
	}
	var zero T
	return zero, false
}

func storeNamedWithDefer[T any]() (previous T, loaded bool) {
	defer func() {}()
	var zero T
	return zero, false
}

func named[T any]() (value T, ok bool) {
	return
}

type holder[T any] struct{}

func (h *holder[T]) use(value T) {}

func call[T any](h *holder[T]) {
	h.use(*new(T))
}
`)

	for _, forbidden := range []string{
		"T::default()",
		"Some(Default::default())",
		"Some(Rc::new(RefCell::new(None)))",
		"Some(Arc::new(Mutex::new(None)))",
	} {
		if strings.Contains(rust, forbidden) {
			t.Fatalf("unconstrained type-parameter zero value should not require Rust Default via %q:\n%s", forbidden, rust)
		}
	}
	if !strings.Contains(rust, "(Rc::new(RefCell::new(None)), false)") &&
		!strings.Contains(rust, "(Arc::new(Mutex::new(None)), false)") {
		t.Fatalf("*new(T) zero return should emit a nil generic value handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut zero: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));") &&
		!strings.Contains(rust, "let mut zero: Arc<Mutex<Option<T>>> = Arc::new(Mutex::new(None));") {
		t.Fatalf("var zero T should initialize the generic value handle to None:\n%s", rust)
	}
	if strings.Contains(rust, "zero.borrow().as_ref().unwrap()") ||
		strings.Contains(rust, "zero.lock().unwrap().as_ref().unwrap()") {
		t.Fatalf("returning var zero T through named results should not unwrap a nil generic handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut value: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));") &&
		!strings.Contains(rust, "let mut value: Arc<Mutex<Option<T>>> = Arc::new(Mutex::new(None));") {
		t.Fatalf("named result T should initialize the generic value handle to None:\n%s", rust)
	}
}

func TestTypeParamPointerNamedResultUsesNilHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func swap[T any]() (old *T) {
	return
}
`)

	if strings.Contains(rust, "Some(Default::default())") {
		t.Fatalf("named *T result should not require Rust Default:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut old: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));") &&
		!strings.Contains(rust, "let mut old: Arc<Mutex<Option<T>>> = Arc::new(Mutex::new(None));") {
		t.Fatalf("named *T result should initialize to a nil handle:\n%s", rust)
	}
}

func TestNamedSliceSelectorReturnAsUnnamedSliceUsesInnerHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

type Int struct {
	abs nat
}

func (x *Int) Bits() []Word {
	return x.abs
}
`)

	if strings.Contains(rust, "return self.abs.clone();") {
		t.Fatalf("named-slice field returned as unnamed slice should not return the named wrapper handle:\n%s", rust)
	}
	if !strings.Contains(rust, "return { let __named_slice = (*self.abs.borrow().as_ref().unwrap()).0.clone(); __named_slice };") &&
		!strings.Contains(rust, "return { let __named_slice = (*self.abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice };") {
		t.Fatalf("named-slice field returned as unnamed slice should return the inner slice handle:\n%s", rust)
	}
}

func TestNamedSliceSliceExprReturnWrapsNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func (z nat) norm(i int) nat {
	return z[:i]
}
`)

	if strings.Contains(rust, "return nat(") || strings.Contains(rust, "{\n        nat(") {
		t.Fatalf("named-slice slice return should not return the bare named value:\n%s", rust)
	}
	if !strings.Contains(rust, "Rc::new(RefCell::new(Some(nat(") &&
		!strings.Contains(rust, "Arc::new(Mutex::new(Some(nat(") {
		t.Fatalf("named-slice slice return should wrap the named slice value:\n%s", rust)
	}
}

func TestNamedSliceSliceExprCallArgumentUsesInnerHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func addVV(z, x, y []Word) Word {
	return 0
}

func (z nat) add(x, y nat, n int) Word {
	return addVV(z[:n], x, y)
}
`)

	if strings.Contains(rust, "nat(Rc::new(RefCell::new(Some(") &&
		strings.Contains(rust, ")).borrow().as_ref().unwrap()).0.clone()") {
		t.Fatalf("named-slice slice call argument should not borrow from bare named value:\n%s", rust)
	}
	if strings.Contains(rust, "nat(Arc::new(Mutex::new(Some(") &&
		strings.Contains(rust, ")).lock().unwrap().as_ref().unwrap()).0.clone()") {
		t.Fatalf("named-slice slice call argument should not lock from bare named value:\n%s", rust)
	}
}

func TestNamedSliceSliceExprCallArgumentWrapsForNamedParameter(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func take(z nat) {}

func call(z nat, n int) {
	take(z[n:])
}
`)

	if strings.Contains(rust, "take(nat(") {
		t.Fatalf("named-slice slice call argument for named parameter should not pass a bare named value:\n%s", rust)
	}
	if !strings.Contains(rust, "take(Rc::new(RefCell::new(Some(nat(") &&
		!strings.Contains(rust, "take(Arc::new(Mutex::new(Some(nat(") {
		t.Fatalf("named-slice slice call argument for named parameter should wrap the named value:\n%s", rust)
	}
}

func TestNamedSliceSliceExprShortDeclWrapsNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func karatsuba(z, x, y nat) {
}

func mul(z, x, y nat, n int) {
	x0 := x[:n]
	y0 := y[:n]
	karatsuba(z, x0, y0)
}
`)

	if strings.Contains(rust, "let mut x0 = nat(") || strings.Contains(rust, "let mut y0 = nat(") {
		t.Fatalf("named-slice slice short declaration should wrap the named value:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut x0 = Rc::new(RefCell::new(Some(nat(") &&
		!strings.Contains(rust, "let mut x0 = Arc::new(Mutex::new(Some(nat(") {
		t.Fatalf("named-slice slice short declaration should initialize a wrapped local:\n%s", rust)
	}
}

func TestNamedSliceTupleAssignmentTargetsUseInnerHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func pair(x Word) (Word, Word) {
	return x, x
}

func set(z nat, x Word) {
	z[1], z[0] = pair(x)
}
`)

	if strings.Contains(rust, "(*z.borrow_mut().as_mut().unwrap())") ||
		strings.Contains(rust, "(*z.lock().unwrap().as_mut().unwrap())") {
		t.Fatalf("tuple assignment into named-slice elements should not index the named wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "__named_slice = (*z.borrow().as_ref().unwrap()).0.clone()") &&
		!strings.Contains(rust, "__named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone()") {
		t.Fatalf("tuple assignment into named-slice elements should use the inner slice handle:\n%s", rust)
	}
}

func TestPointerToNamedSliceElementAssignmentUsesInnerHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func set(z *nat) {
	(*z)[0] = 0xfedcb
}
`)

	if strings.Contains(rust, "(*(*z.borrow().as_mut().unwrap()).borrow()") ||
		strings.Contains(rust, "(*(*z.lock().unwrap().as_mut().unwrap()).lock()") {
		t.Fatalf("pointer-to-named-slice element assignment should not lock the dereferenced named value:\n%s", rust)
	}
	if !strings.Contains(rust, "__named_slice = (*z.borrow().as_ref().unwrap()).0.clone()") &&
		!strings.Contains(rust, "__named_slice = (*z.lock().unwrap().as_ref().unwrap()).0.clone()") {
		t.Fatalf("pointer-to-named-slice element assignment should use the named slice inner handle:\n%s", rust)
	}
}

func TestPointerToSliceElementAssignmentUsesPointeeHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

var _ sync.Mutex

func trim(v *[][]byte, n int) {
	(*v)[0] = (*v)[0][n:]
	(*v)[0] = nil
}
`)

	if strings.Contains(rust, "(*((*v.lock().unwrap().as_mut().unwrap())).lock()") ||
		strings.Contains(rust, "(*((*v.borrow_mut().as_mut().unwrap())).borrow_mut()") {
		t.Fatalf("pointer-to-slice element assignment should not lock the dereferenced Vec as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __slice_holder = v.clone()") {
		t.Fatalf("pointer-to-slice element assignment should mutate through the pointer's slice handle:\n%s", rust)
	}
	if strings.Contains(rust, "] = None") {
		t.Fatalf("nil assigned into a bare nested slice slot should use the slice zero value:\n%s", rust)
	}
}

func TestNestedArrayElementAssignmentMutatesOuterHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type holder struct {
	slots [2][3]uint64
}

func set(h *holder, gen uintptr, exp int) {
	h.slots[gen%2][exp] = 7
}
`)

	if strings.Contains(rust, "].clone() }.lock().unwrap().as_mut().unwrap())") ||
		strings.Contains(rust, "].clone().borrow_mut().as_mut().unwrap())") {
		t.Fatalf("nested array element assignment should not borrow a cloned inner array as a wrapped handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".slots.borrow_mut().as_mut().unwrap())[") ||
		!strings.Contains(rust, "][") {
		t.Fatalf("nested array element assignment should mutate through the outer array handle:\n%s", rust)
	}
}

func TestRangeOverPointerToNamedArrayUsesInnerHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type callers [4]uintptr

func sum(frames *callers) uintptr {
	var total uintptr
	for _, pc := range frames {
		if pc == 0 {
			break
		}
		total += pc
	}
	return total
}
`)

	if strings.Contains(rust, "let __range_holder = frames.clone()") {
		t.Fatalf("range over pointer to named array should not iterate the named wrapper value:\n%s", rust)
	}
	if !strings.Contains(rust, ".0.clone(); __named_array }; let __range_guard = __range_holder") {
		t.Fatalf("range over pointer to named array should materialize the inner array handle:\n%s", rust)
	}
}

func TestPointerToSliceDerefAssignmentCopiesSliceValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

var _ sync.Mutex

func trim(v *[][]byte) {
	*v = (*v)[1:]
}
`)

	if strings.Contains(rust, "((*v.lock().unwrap().as_mut().unwrap())).lock()") ||
		strings.Contains(rust, "((*v.borrow_mut().as_mut().unwrap())).borrow_mut()") {
		t.Fatalf("pointer-to-slice deref assignment should not lock the dereferenced Vec as a handle:\n%s", rust)
	}
	if strings.Contains(rust, "= Some(new_val)") {
		t.Fatalf("pointer-to-slice deref assignment should store the RHS slice option, not a wrapped handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __slice_holder = v.clone()") {
		t.Fatalf("pointer-to-slice deref assignment should slice through the pointer's handle:\n%s", rust)
	}
}

func TestParallelNamedSliceFieldAssignmentStoresValues(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

type Int struct {
	abs nat
}

type Rat struct {
	a, b Int
}

func swap(z *Rat) {
	z.a.abs, z.b.abs = z.b.abs, z.a.abs
}
`)

	if strings.Contains(rust, "__tmp_0.borrow_mut().take()") ||
		strings.Contains(rust, "__tmp_1.borrow_mut().take()") ||
		strings.Contains(rust, "__tmp_0.lock().unwrap().take()") ||
		strings.Contains(rust, "__tmp_1.lock().unwrap().take()") {
		t.Fatalf("parallel named-slice field assignment should not treat value temps as handles:\n%s", rust)
	}
	if !strings.Contains(rust, "= Some(__tmp_0);") || !strings.Contains(rust, "= Some(__tmp_1);") {
		t.Fatalf("parallel named-slice field assignment should store captured named-slice values:\n%s", rust)
	}
}

func TestParallelNamedSliceReceiverAssignmentUsesNamedTemps(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func (z nat) swap(zz nat) nat {
	zz, z = z, zz
	return z
}
`)

	if strings.Contains(rust, "let __tmp_0 = __self.0.clone();") ||
		strings.Contains(rust, "let __tmp_1 = __self.0.clone();") {
		t.Fatalf("parallel named-slice receiver assignment should not capture the inner slice handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __tmp_0 = Rc::new(RefCell::new(Some(__self.clone())));") &&
		!strings.Contains(rust, "let __tmp_0 = Arc::new(Mutex::new(Some(__self.clone())));") {
		t.Fatalf("parallel named-slice receiver assignment should capture the named receiver value:\n%s", rust)
	}
}

func TestNamedIntegerSliceRangeUsesClonedValues(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func forceConcurrent() {
	go func() {}()
}

func (x nat) sticky() bool {
	for _, x := range x[:1] {
		if x != 0 {
			return true
		}
	}
	return false
}
`)

	if strings.Contains(rust, ".iter().copied()") {
		t.Fatalf("range over named integer slice should clone wrapper values, not require Copy:\n%s", rust)
	}
	if !strings.Contains(rust, ".iter().cloned()") {
		t.Fatalf("range over named integer slice should use cloned values:\n%s", rust)
	}
	if strings.Contains(rust, "if *x !=") || strings.Contains(rust, "let __tmp_x = *x") {
		t.Fatalf("owned named integer range value should not be dereferenced in comparison:\n%s", rust)
	}
}

func TestStructLiteralNamedSliceFieldFromUnnamedSliceConstructsNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

type Int struct {
	abs nat
}

func makeInt(abs []Word) *Int {
	return &Int{abs: abs}
}
`)

	if strings.Contains(rust, "abs: abs.clone()") {
		t.Fatalf("named-slice field should not receive the unnamed slice handle directly:\n%s", rust)
	}
	if !strings.Contains(rust, "abs: Rc::new(RefCell::new(Some(nat(abs.clone()))))") &&
		!strings.Contains(rust, "abs: Arc::new(Mutex::new(Some(nat(abs.clone()))))") {
		t.Fatalf("named-slice field should construct the named slice inside the field wrapper:\n%s", rust)
	}
}

func TestVarInitializerFromBareScalarCallRegistersBareLocal(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func fnv1(x uint32, list ...byte) uint32 {
	return x
}

func use() uint32 {
	var (
		hash = fnv1(0, []byte("struct {")...)
	)
	name := "field"
	hash = fnv1(hash, []byte(name)...)
	return hash
}
`)

	if strings.Contains(rust, "hash.borrow") || strings.Contains(rust, "hash.lock") {
		t.Fatalf("var initialized from a bare-scalar call should stay a bare local:\n%s", rust)
	}
	if !strings.Contains(rust, "hash = new_val;") {
		t.Fatalf("bare scalar var reassignment should assign the Rust local directly:\n%s", rust)
	}
}

func TestUnsafePointerDerefNilAssignmentIsLoudUnsupported(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

func clear(slot unsafe.Pointer) {
	*(*unsafe.Pointer)(slot) = nil
}
`)

	if strings.Contains(rust, "let _ = None") {
		t.Fatalf("unsafe pointer nil assignment should not emit type-ambiguous None:\n%s", rust)
	}
	if !strings.Contains(rust, `unimplemented!("unsafe.Pointer dereference assignment")`) {
		t.Fatalf("unsafe pointer dereference assignment should fail loudly:\n%s", rust)
	}
}

func TestUnsafePointerFunctionDerefAssignmentIsLoudUnsupported(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

func assign(raw unsafe.Pointer) {
	var cleanup func()
	cleanup = *(*func())(raw)
	cleanup()
}
`)

	if strings.Contains(rust, "let new_val = { let __v =") && strings.Contains(rust, " as usize") {
		t.Fatalf("unsafe pointer to function dereference should not become a uintptr value:\n%s", rust)
	}
	if !strings.Contains(rust, `let new_val = unimplemented!("unsafe.Pointer conversion to function value")`) {
		t.Fatalf("unsafe pointer to function dereference should fail loudly with the target function type:\n%s", rust)
	}
}

func TestNamedIntegerBinaryIndexParenthesizesAsCast(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Code int

const Invalid Code = -1

func main() {
	var x [1]struct{}
	_ = x[Invalid - -1]
}
`)

	if strings.Contains(rust, "INVALID as i32 - -1 as i32 as usize") {
		t.Fatalf("named-integer binary index must parenthesize as-cast operands before as-usize:\n%s", rust)
	}
	if !strings.Contains(rust, "(INVALID as i32 - -1 as i32) as usize") &&
		!strings.Contains(rust, "(INVALID as i32 - -1) as usize") {
		t.Fatalf("expected named-integer binary index emission to wrap operands in parens:\n%s", rust)
	}
}

func TestShiftLHSWithAsCastIsParenthesized(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func byteBit(i int) byte {
	return byte(1) << (i % 8)
}

func main() {
	_ = byteBit(3)
}
`)

	if strings.Contains(rust, "as u8 <<") {
		t.Fatalf("shift LHS with as-cast must be parenthesized to avoid Rust precedence collision:\n%s", rust)
	}
	if !strings.Contains(rust, "as u8) <<") {
		t.Fatalf("expected shift LHS with as-cast to be wrapped in parens:\n%s", rust)
	}
}

func TestStrconvAtoiTupleSlotEmitsBareScalarFirstResult(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"strconv"
)

func main() {
	num, err := strconv.Atoi("42")
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	fmt.Println("Parsed number:", num)
}
`)

	if strings.Contains(rust, "Ok(n) => (Rc::new(") || strings.Contains(rust, "Ok(n) => (Arc::new(") {
		t.Fatalf("strconv.Atoi Ok branch should keep the bare scalar in the first tuple slot:\n%s", rust)
	}
	if !strings.Contains(rust, "Ok(n) => (n,") {
		t.Fatalf("strconv.Atoi Ok branch should yield bare scalar n directly:\n%s", rust)
	}
	if strings.Contains(rust, "Err(e) => (Rc::new(RefCell::new(Some(0)))") ||
		strings.Contains(rust, "Err(e) => (Arc::new(Mutex::new(Some(0)))") {
		t.Fatalf("strconv.Atoi Err branch should emit bare scalar zero in the first tuple slot:\n%s", rust)
	}
	if !strings.Contains(rust, "Err(_) => (0 as i32,") {
		t.Fatalf("strconv.Atoi Err branch should emit bare zero (0 as i32) in the first tuple slot:\n%s", rust)
	}
	if !strings.Contains(rust, "(mut num, mut err)") {
		t.Fatalf("expected num/err short-decl destructuring:\n%s", rust)
	}
	if strings.Contains(rust, "format!(\"{}\", (*num.borrow()") || strings.Contains(rust, "format!(\"{}\", (*num.lock()") {
		t.Fatalf("bare scalar num must not be unwrapped through .borrow()/lock() in fmt printing:\n%s", rust)
	}
}

func TestStrconvAtoiExistingIntTupleAssignmentUsesBareNumTemp(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "strconv"

func caller(s string) int {
	var n int
	n, _ = strconv.Atoi(s)
	return n
}
`)

	if strings.Contains(rust, "__tmp_0.lock()") {
		t.Fatalf("strconv.Atoi int tuple slot should be assigned as a bare temp:\n%s", rust)
	}
	if !strings.Contains(rust, "*n.lock().unwrap() = Some(__tmp_0);") &&
		!strings.Contains(rust, "*n.borrow_mut() = Some(__tmp_0);") {
		t.Fatalf("existing int assignment from strconv.Atoi should store the bare scalar in the wrapped local:\n%s", rust)
	}
}

func TestConcreteErrorPointerCallReturnBoxesPointee(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type NumError struct{}

func (*NumError) Error() string { return "bad" }

func syntaxError() *NumError { return &NumError{} }

func parse() (bool, error) {
	return false, syntaxError()
}
`)

	if strings.Contains(rust, "Box::new(syntax_error()) as Box<dyn StdError") {
		t.Fatalf("pointer error call result should not box the pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new((*syntax_error()") ||
		!strings.Contains(rust, ".as_ref().unwrap()).clone()) as Box<dyn StdError") {
		t.Fatalf("pointer error call result should box the pointee clone:\n%s", rust)
	}
}

func TestTailReturnFallsBackToExplicitForBareScalarNamedReturns(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func calc(a, b int) (sum, product int) {
	sum = a + b
	product = a * b
	return
}

func paramsOnly(a, b int) (int, int) {
	return a + b, a * b
}
`)

	if !strings.Contains(rust, "return ((*sum.borrow().as_ref().unwrap()), (*product.borrow().as_ref().unwrap()));") {
		t.Fatalf("naked return through bare scalar named-returns should keep explicit return for borrow lifetime safety:\n%s", rust)
	}
	if !strings.Contains(rust, "((*a.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()) * (*b.borrow().as_ref().unwrap()))\n}") {
		t.Fatalf("returns that only borrow parameters should still tail-return:\n%s", rust)
	}
}

func TestBareScalarCharReturnUsesRustEscapes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func escaped() (rune, rune, rune) {
	return '\a', '\f', '\v'
}
`)

	for _, invalid := range []string{`'\a'`, `'\f'`, `'\v'`} {
		if strings.Contains(rust, invalid) {
			t.Fatalf("bare scalar rune return should translate Go-only char escape %s:\n%s", invalid, rust)
		}
	}
	for _, want := range []string{`'\u{7}'`, `'\u{c}'`, `'\u{b}'`} {
		if !strings.Contains(rust, want) {
			t.Fatalf("bare scalar rune return should include %s:\n%s", want, rust)
		}
	}
}

func TestTailReturnFallsBackForShortDeclWrappedLocals(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func explicitFromLocals(a, b int) (int, int) {
	x := a + b
	y := a * b
	return x, y
}
`)

	if !strings.Contains(rust, "return ((*x.borrow().as_ref().unwrap()), (*y.borrow().as_ref().unwrap()));") {
		t.Fatalf("explicit return reading short-decl wrapped locals must stay an explicit return statement:\n%s", rust)
	}
}

func TestDeferReturnEmitsTrailingSemicolonForBareScalarReturn(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func run() (result int) {
	defer func() {
		result = 7
		return
	}()
	return 3
}

func main() {
	fmt.Println(run())
}
`)

	if !strings.Contains(rust, "return (*result.borrow().as_ref().unwrap());") {
		t.Fatalf("defer-block return reading a wrapped bare-scalar local must end with `;` so its temporaries drop before the local:\n%s", rust)
	}
}

func TestShortDeclFromBareScalarCallRegistersBareLocal(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func indexByte(s string, c byte) int {
	return 0
}

func parse(env string) {
	i := indexByte(env, ',')
	if i < 0 {
		return
	}
	i = indexByte(env, '=')
	_ = env[:i]
}
`)

	if strings.Contains(rust, "i.borrow()") || strings.Contains(rust, "i.lock()") {
		t.Fatalf("short-declared bare scalar call result should stay bare on later uses:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut i = index_byte") {
		t.Fatalf("short declaration from bare scalar call should initialize a bare local:\n%s", rust)
	}
}

func TestBareScalarCallResultIncDecAndCompoundAssignStayBare(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func scalar() int {
	return 3
}

func adjust() {
	n := scalar()
	n--
	frameSize := scalar()
	frameSize += scalar()
	_, _ = n, frameSize
}
`)

	if strings.Contains(rust, "n.borrow()") || strings.Contains(rust, "n.borrow_mut()") || strings.Contains(rust, "n.lock()") ||
		strings.Contains(rust, "frameSize.borrow()") || strings.Contains(rust, "frameSize.borrow_mut()") || strings.Contains(rust, "frameSize.lock()") {
		t.Fatalf("bare scalar call results should mutate as raw locals, not wrapper handles:\n%s", rust)
	}
}

func TestBareScalarCallResultNumericConversionStaysBare(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func scalar() uintptr {
	return 3
}

func use(v uint32) {}

func adjust() {
	frameSize := scalar()
	use(uint32(frameSize))
}
`)

	if strings.Contains(rust, "frameSize.borrow()") || strings.Contains(rust, "frameSize.lock()") {
		t.Fatalf("numeric conversion of a bare scalar local should use the raw local:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(frameSize as u32)") {
		t.Fatalf("numeric conversion of a bare scalar local should cast the raw local:\n%s", rust)
	}
}

func TestUnsafePointerAnyDerefAssignmentMovesTemporaryInterface(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

func load(ptr unsafe.Pointer) any {
	var eface any
	eface = *(*any)(ptr)
	return eface
}
`)

	if strings.Contains(rust, "as_mut().unwrap()).clone()") {
		t.Fatalf("assignment from temporary unsafe any dereference should not clone Box<dyn Any>:\n%s", rust)
	}
	if !strings.Contains(rust, "eface = Rc::new(RefCell::new({") &&
		!strings.Contains(rust, "eface = Arc::new(Mutex::new({") {
		t.Fatalf("assignment from temporary unsafe any dereference should move the temporary interface handle:\n%s", rust)
	}
}

func TestShortDeclFromSelectorNamedArrayCompositeLiteralRegistersBareLocal(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "hash/crc32"

func f() {
	table := crc32.Table{}
	_ = table
}
`)

	if strings.Contains(rust, "table.borrow()") || strings.Contains(rust, "table.lock()") {
		t.Fatalf("selector named-array composite literal should register a bare local:\n%s", rust)
	}
}

func TestNamedArrayIndexedCompoundAssignMutatesInnerArray(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type asciiSet [8]uint32

func add(as asciiSet, c byte) asciiSet {
	as[c/32] |= 1 << (c % 32)
	return as
}
`)

	if strings.Contains(rust, "let mut __seq_guard = as.borrow_mut()") ||
		strings.Contains(rust, "let mut __seq_guard = as.lock().unwrap()") {
		t.Fatalf("named array indexed compound assignment should not index the wrapper struct:\n%s", rust)
	}
	if !strings.Contains(rust, ").0.clone()") {
		t.Fatalf("named array indexed compound assignment should mutate the inner array handle:\n%s", rust)
	}
}

func TestPointerNamedArrayFieldIndexAssignmentMutatesInnerArray(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type callers [4]uintptr

type machine struct {
	callers *callers
}

func clear(mp *machine) {
	mp.callers[0] = 0
}
`)

	if strings.Contains(rust, ".callers.lock().unwrap().as_mut().unwrap())[(0) as usize]") {
		t.Fatalf("pointer-to-named-array field assignment should not index the named wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, ".callers") ||
		!strings.Contains(rust, ".0.clone(); __named_array }") ||
		!strings.Contains(rust, ")[(0) as usize] = 0 as usize;") {
		t.Fatalf("pointer-to-named-array field assignment should mutate the inner array handle:\n%s", rust)
	}
}

func TestParallelLenCapAssignmentToIntFieldsCastsTemporaries(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type sliceHeader struct {
	len int
	cap int
}

func set(x []byte, bb *sliceHeader) {
	bb.len, bb.cap = len(x), cap(x)
}
`)

	if strings.Contains(rust, "Some(__tmp_0);") || strings.Contains(rust, "Some(__tmp_1);") {
		t.Fatalf("parallel len/cap assignment to int fields should not store usize temporaries directly:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(__tmp_0 as i32);") ||
		!strings.Contains(rust, "Some(__tmp_1 as i32);") {
		t.Fatalf("parallel len/cap assignment to int fields should cast temporary values to Go int:\n%s", rust)
	}
}

func TestNamedIntegerReceiverCompoundAssignUsesUnderlyingRHS(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Accuracy int8

func (i Accuracy) String() string {
	i -= -1
	return ""
}
`)

	if strings.Contains(rust, "let __rhs = Accuracy(") {
		t.Fatalf("named integer receiver compound assignment should use underlying RHS:\n%s", rust)
	}
	if !strings.Contains(rust, "let __rhs = -1") || !strings.Contains(rust, " as i8") {
		t.Fatalf("named integer receiver compound assignment should cast RHS to int8:\n%s", rust)
	}
}

func TestNamedIntegerReceiverCompoundAssignUnwrapsNamedSelectorRHS(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Mode int

type item struct {
	mode Mode
}

func (mode Mode) String() string {
	for _, item := range []item{{mode: 1}} {
		mode ^= item.mode
	}
	return ""
}
`)

	if strings.Contains(rust, "let __rhs = (*item.mode") {
		t.Fatalf("named integer receiver compound assignment should not keep selector RHS as a named value:\n%s", rust)
	}
	if !strings.Contains(rust, "((*item.mode.borrow().as_ref().unwrap()).clone()).0.borrow().as_ref().unwrap()).clone()") &&
		!strings.Contains(rust, "((*item.mode.lock().unwrap().as_ref().unwrap()).clone()).0.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("named integer receiver compound assignment should use the selector RHS underlying value:\n%s", rust)
	}
}

func TestNamedIntegerPointerReceiverBitwiseConstCompoundAssignUsesUnderlyingRHS(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Flags uint8

const hasFree Flags = 1 << iota

func (f *Flags) clear() {
	*f &^= hasFree
}

func (f *Flags) set() {
	*f |= hasFree
}
`)

	if strings.Contains(rust, "let __rhs = Flags(") {
		t.Fatalf("named integer pointer receiver bitwise const compound assignment should not keep the named RHS value:\n%s", rust)
	}
	if !strings.Contains(rust, "let __rhs = HAS_FREE as u8") {
		t.Fatalf("named integer pointer receiver bitwise const compound assignment should use the underlying RHS value:\n%s", rust)
	}
}

func TestRangeIndexReturnedFromBareScalarTupleSlotCastsToI32(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func findInSlice(slice []int, target int) (int, bool) {
	for i, val := range slice {
		if val == target {
			return i, true
		}
	}
	return -1, false
}
`)

	if !strings.Contains(rust, "pub fn find_in_slice") {
		t.Fatalf("expected find_in_slice declaration:\n%s", rust)
	}
	if !strings.Contains(rust, "return (i as i32, true);") {
		t.Fatalf("range index returned through bare i32 tuple slot must cast usize -> i32:\n%s", rust)
	}
	if strings.Contains(rust, "return (i, true);") {
		t.Fatalf("range index emit should not leak usize into a bare i32 return slot:\n%s", rust)
	}
}

func TestAssignedRangeIndexNamedReturnKeepsWrappedResult(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func write(data []byte) (n int, err error) {
	var b byte
	for n, b = range data {
		if b == 0 {
			return n, err
		}
	}
	return
}
`)

	if strings.Contains(rust, "for (n, b) in") {
		t.Fatalf("range assignment should not shadow existing result and local handles:\n%s", rust)
	}
	if !strings.Contains(rust, "for (__range_n, __range_b) in") {
		t.Fatalf("range assignment should bind loop temporaries before assigning existing variables:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = __range_n as i32; *n.borrow_mut() = Some(new_val);") {
		t.Fatalf("range assignment should store the usize key into the named result handle as Go int:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = __range_b; *b.borrow_mut() = Some(new_val);") {
		t.Fatalf("range assignment should store the byte value into the existing local handle:\n%s", rust)
	}
	if strings.Contains(rust, "return (n as i32") {
		t.Fatalf("explicit return should read the named result handle, not a shadowing usize range key:\n%s", rust)
	}
}

func TestAssignedPointerRangeValueReplacesHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Func struct {
	name string
}

func pick(methods []*Func) *Func {
	var m *Func
	for _, m = range methods {
		return m
	}
	return nil
}
`)

	if strings.Contains(rust, "*m.borrow_mut() = Some(new_val);") {
		t.Fatalf("pointer range assignment should replace the handle, not store the handle inside the pointee slot:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = (*__range_m).clone(); m = new_val;") {
		t.Fatalf("pointer range assignment should clone the range pointer handle into the existing variable:\n%s", rust)
	}
}

func TestRangeIndexComparedWithBareScalarCallCastsToI32(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func limit(values []int) int {
	return len(values)
}

func countUntilLimit(values []int) int {
	count := 0
	for i := range values {
		if i >= limit(values) {
			break
		}
		count++
	}
	return count
}
`)

	if !strings.Contains(rust, "i as i32 >= limit(values.clone())") {
		t.Fatalf("range index usize must cast to i32 when compared against a bare-scalar int call:\n%s", rust)
	}
}

func TestIntegerRangeOverSelectorUnwrapsLimit(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Map struct {
	dirLen int
}

func (m *Map) grow() {
	for i := range m.dirLen {
		_ = i
	}
}
`)

	if strings.Contains(rust, "0..(self.dir_len.clone())") {
		t.Fatalf("integer range over selector should unwrap the field limit, not iterate over the handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __range_value = { let __range_guard = __range_limit") {
		t.Fatalf("integer range over selector should borrow the cloned field handle in an inner block:\n%s", rust)
	}
	if !strings.Contains(rust, "}; __range_value }") {
		t.Fatalf("integer range over selector should return an owned range limit after the borrow ends:\n%s", rust)
	}
	if !strings.Contains(rust, "self.dir_len.clone()") {
		t.Fatalf("integer range over selector should clone the field handle before borrowing:\n%s", rust)
	}
}

func TestIntegerRangeOverSelectorConstUsesBareLimit(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unicode/utf8"

func scan() {
	for i := range utf8.UTFMax {
		_ = i
	}
}
`)

	if strings.Contains(rust, "__range_guard") || strings.Contains(rust, ".lock().unwrap()") {
		t.Fatalf("integer range over selector const should not borrow a wrapped limit:\n%s", rust)
	}
	if !strings.Contains(rust, "0..(") {
		t.Fatalf("expected integer range emission:\n%s", rust)
	}
}

func TestRangeKeyOnlyOverSlicedArrayExpressionWrapsLengthBlock(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func sum(n int) int {
	var stk [4]int
	total := 0
	for i := range stk[:n] {
		total += i
	}
	return total
}
`)

	if strings.Contains(rust, " in 0..{ let __seq =") {
		t.Fatalf("key-only range over sliced array expression should parenthesize the length block:\n%s", rust)
	}
	if !strings.Contains(rust, " in 0..({ let __seq =") {
		t.Fatalf("expected parenthesized range length block:\n%s", rust)
	}
}

func TestConcurrentRangeOverComplexSliceLiteralBreaksVecAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func trace(workProcs, mark, sweep, end, assist, dedicated, fractional, idle, term int64) int64 {
	go func() {}()
	total := int64(0)
	for i, ns := range []int64{
		workProcs * (mark - sweep),
		assist,
		dedicated + fractional,
		idle,
		workProcs * (end - term),
	} {
		total += int64(i) + ns
	}
	return total
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "for (i, ns) in vec![") && strings.Contains(line, ".iter().copied().enumerate()") {
			t.Fatalf("complex range slice literal should not emit the whole vec on one line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "for (i, ns) in vec![\n") {
		t.Fatalf("complex range slice literal should break vec elements across lines:\n%s", rust)
	}
}

func TestConcurrentErrorStructLiteralReturnBreaksFieldsAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type parseError struct {
	layout string
	value string
	layoutElem string
	valueElem string
	message string
}

func (e *parseError) Error() string {
	return e.message
}

func parse(b []byte) (int, error) {
	go func() {}()
	return 0, &parseError{"2006-01-02T15:04:05Z07:00", string(b), "15", string(b[len("2006-01-02T"):][:1]), ""}
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "layout:") && strings.Contains(line, "value_elem:") {
			t.Fatalf("large error struct literal should split fields across lines:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "parseError {\n") {
		t.Fatalf("large error struct literal should use multiline struct syntax:\n%s", rust)
	}
	if strings.Contains(rust, "..Default::default(),") {
		t.Fatalf("struct update syntax must not keep a comma after the base expression:\n%s", rust)
	}
}

func TestConcurrentLocalStructLiteralWithWrappedLocalFieldsBreaksFieldsAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type ParseError struct {
	layout string
	value string
	layoutElem string
	valueElem string
	message string
}

func clone(s string) string {
	return s
}

func newParseError(layout, value, layoutElem, valueElem, message string) *ParseError {
	go func() {}()
	valueCopy := clone(value)
	valueElemCopy := clone(valueElem)
	return &ParseError{layout, valueCopy, layoutElem, valueElemCopy, message}
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "ParseError {") && strings.Contains(line, "value_elem:") {
			t.Fatalf("struct literal with wrapped local fields should split fields across lines:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "ParseError {\n") {
		t.Fatalf("struct literal with wrapped local fields should use multiline struct syntax:\n%s", rust)
	}
}

func TestConcurrentStructLiteralAssignmentBreaksFieldsAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "unsafe"

type node byte

type nodeSlice struct {
	array *node
	len int
	cap int
}

func build(base, size, offset uintptr, elems int) nodeSlice {
	go func() {}()
	var sl nodeSlice
	sl = nodeSlice{(*node)(unsafe.Pointer(base + size - offset)), elems, elems}
	return sl
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "let new_val = nodeSlice {") && strings.Contains(line, "array:") && strings.Contains(line, "cap:") {
			t.Fatalf("complex struct literal assignment should split fields across lines:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "let new_val = nodeSlice {\n") {
		t.Fatalf("complex struct literal assignment should use multiline struct syntax:\n%s", rust)
	}
}

func TestConcurrentSelectorAssignmentFromMethodExpressionCallBreaksAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type timers struct {
	raceCtx uintptr
}

func (ts *timers) run(now int64) int64 {
	return now
}

func pc(fn func(*timers, int64) int64) uintptr {
	return 1
}

func start(v uintptr) uintptr {
	return v
}

const quantum uintptr = 4

func use(ts *timers) {
	go func() {}()
	if ts.raceCtx == 0 {
		ts.raceCtx = start(pc((*timers).run) + quantum)
	}
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "let new_val = start(") && strings.Contains(line, "= Some(new_val); }") {
			t.Fatalf("complex selector assignment should split the value and target writes across lines:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "let new_val = start(") {
		t.Fatalf("test setup should lower the assignment through a start call:\n%s", rust)
	}
}

func TestConcurrentSelectorAssignmentFromNestedSelectorCloneBreaksAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type timers struct {
	raceCtx uintptr
}

type proc struct {
	timers timers
}

func (p *proc) ptr() *proc {
	return p
}

type machine struct {
	p *proc
}

type g struct {
	m *machine
	racectx uintptr
}

func use(gp *g) {
	go func() {}()
	gp.racectx = gp.m.p.ptr().timers.raceCtx
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "let new_val = { let __selector_holder") && strings.Contains(line, " = Some(new_val); }") {
			t.Fatalf("selector clone assignment should not keep value clone and target write on one line:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "let new_val = { let __selector_holder") {
		t.Fatalf("test setup should clone a selector value through a temporary holder:\n%s", rust)
	}
}

func TestWrappedUint64AssignmentFromSelectorConstCastsValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unicode/utf8"

func clamp(capacity uint64) uint64 {
	if capacity < utf8.UTFMax {
		capacity = utf8.UTFMax
	}
	return capacity
}
`)

	if !strings.Contains(rust, "let new_val = utf8::") || !strings.Contains(rust, "as u64; *capacity") {
		t.Fatalf("selector const assigned to uint64 should be cast to the target type:\n%s", rust)
	}
}

func TestArrayElementAssignmentFromNamedScalarSelectorUsesValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Pos int

type parser struct {
	pos Pos
}

func (p *parser) assign(i int) [2]Pos {
	var colons [2]Pos
	colons[i] = p.pos
	return colons
}
`)

	if strings.Contains(rust, "] = self.pos.clone();") {
		t.Fatalf("array element assignment should not store a selector field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __v = self.pos.clone(); let __owned = (*__v") {
		t.Fatalf("array element assignment should copy the selector field value:\n%s", rust)
	}
}

func TestIndexedSelectorIncDecMutatesSequenceElement(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type decimal struct {
	mant []byte
}

func (x *decimal) bump(n int) {
	x.mant[n-1]++
}
`)

	if strings.Contains(rust, ".clone() }.lock().unwrap()") {
		t.Fatalf("indexed selector inc/dec should not lock a cloned scalar element:\n%s", rust)
	}
	if !strings.Contains(rust, "let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + 1;") {
		t.Fatalf("indexed selector inc/dec should mutate the underlying sequence element:\n%s", rust)
	}
}

func TestNamedSliceElementCompoundAssignUsesStableHandleAndClonesElement(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func round(z nat, n uint32, lsb Word) {
	const msb = 1 << 63
	z[n-1] |= msb
	z[0] &^= lsb - 1
}
`)

	if strings.Contains(rust, "}.borrow_mut()") || strings.Contains(rust, "}.lock().unwrap()") {
		t.Fatalf("named slice compound assignment should bind the slice handle before borrowing it:\n%s", rust)
	}
	if !strings.Contains(rust, "let __seq_holder = { let __named_slice =") {
		t.Fatalf("named slice compound assignment should keep a stable slice handle binding:\n%s", rust)
	}
	if !strings.Contains(rust, "__seq[__idx].clone() | __rhs") ||
		!strings.Contains(rust, "__seq[__idx].clone() & ! __rhs") {
		t.Fatalf("named slice compound assignment should clone non-Copy elements before applying the operator:\n%s", rust)
	}
}

func TestRangeNamedIntegerComparisonClonesValueBeforeReuse(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Word uint
type nat []Word

func start() {
	go func() {}()
}

func keep(values nat) Word {
	var out Word
	for _, d := range values {
		if d != 0 {
			out = d
		}
	}
	return out
}
`)

	if strings.Contains(rust, "let __tmp_x = d;") {
		t.Fatalf("comparison should not move a non-Copy range value that is reused later:\n%s", rust)
	}
	if !strings.Contains(rust, "let __tmp_x = d.clone();") {
		t.Fatalf("comparison should clone a non-Copy range value before comparing:\n%s", rust)
	}
}

func TestGoFuncLiteralArgumentParameterEscapesRustKeyword(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func run(fn func() error) {
	go func(fn func() error) {
		_ = fn()
	}(fn)
}
`)

	if strings.Contains(rust, "|fn:") {
		t.Fatalf("goroutine function literal parameter should escape Rust keywords:\n%s", rust)
	}
	if !strings.Contains(rust, "|r#fn:") {
		t.Fatalf("goroutine function literal parameter should preserve the Go name with Rust escaping:\n%s", rust)
	}
	if strings.Contains(rust, "|r#fn: Box<dyn FnMut") {
		t.Fatalf("goroutine function literal parameter should use the wrapped function parameter shape:\n%s", rust)
	}
	if !strings.Contains(rust, "|r#fn: Arc<Mutex<Option<Box<dyn FnMut") {
		t.Fatalf("goroutine function literal parameter should accept the wrapped function handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_arg_0 = r#fn.clone();") || !strings.Contains(rust, "__closure(__go_arg_0)") {
		t.Fatalf("goroutine function literal call should capture and pass the escaped function handle:\n%s", rust)
	}
}

func TestGoFuncLiteralRangeFunctionArgumentPassesHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func run(funcs []func() error) {
	for _, fn := range funcs {
		go func(fn func() error) {
			_ = fn()
		}(fn)
	}
}
`)

	if strings.Contains(rust, "Some(r#fn)") {
		t.Fatalf("range function value argument should not be wrapped as an inner function box:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_arg_0 = r#fn.clone();") || !strings.Contains(rust, "__closure(__go_arg_0)") {
		t.Fatalf("range function value argument should capture and pass the function handle clone:\n%s", rust)
	}
}

func TestGoFuncLiteralCapturesLocalFunctionValueHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type holder struct {
	open func(string) int
}

func run(h holder, names []string) {
	open := h.open
	for _, name := range names {
		go func(path string) {
			_ = open(path)
		}(name)
	}
}
`)

	if strings.Contains(rust, "Some((*open.") {
		t.Fatalf("goroutine should not clone the inner function box when capturing a local function value:\n%s", rust)
	}
	if !strings.Contains(rust, "let open_thread = open.clone();") {
		t.Fatalf("goroutine should capture the local function value handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut __closure = move |") {
		t.Fatalf("goroutine closure that invokes a function value should be mutable:\n%s", rust)
	}
}

func TestGoFuncLiteralRangeIndexArgumentCastsAndShadowsOuterRange(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func run(values []string, counts []int) {
	for i, value := range values {
		go func(i int, text string) {
			counts[i] = len(text)
		}(i, value)
	}
}
`)

	if strings.Contains(rust, "Some(i))") {
		t.Fatalf("goroutine range index argument should cast usize to Go int before wrapping:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(i as i32)") {
		t.Fatalf("goroutine range index argument should cast usize to i32:\n%s", rust)
	}
	if strings.Contains(rust, "[(i) as usize]") {
		t.Fatalf("function literal int parameter should shadow the outer range index:\n%s", rust)
	}
}

func TestGoFuncLiteralWrappedCallArgumentPassesHandle(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func join(parts []string) string {
	return parts[0] + parts[1]
}

func run(dir string, names []string) {
	for _, name := range names {
		go func(path string) {
			_ = path
		}(join([]string{dir, name}))
	}
}
`)

	if strings.Contains(rust, "Some(join(") {
		t.Fatalf("goroutine argument should not double-wrap a call result that already returns a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_arg_0 = join(") {
		t.Fatalf("goroutine argument should capture the wrapped call result before spawning:\n%s", rust)
	}
}

func TestTripleSlashStatementCommentEmitsRegularRustComment(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func f() {
	/// may be other situations.
	x := 1
	_ = x
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust := transpileParsedRegression(t, file, fset, typeInfo)
	if strings.Contains(rust, "/// may be other situations.") {
		t.Fatalf("statement comment starting with /// should not emit a Rust doc comment:\n%s", rust)
	}
	if !strings.Contains(rust, "// may be other situations.") {
		t.Fatalf("statement comment starting with /// should stay as a regular comment:\n%s", rust)
	}
}

func TestReturnBitClearUsesRustOperator(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type bitset uint64

func clear(b, mask bitset) bitset {
	return b &^ mask
}
`)

	if strings.Contains(rust, "&^") {
		t.Fatalf("Go bit clear operator must not leak into generated Rust:\n%s", rust)
	}
	if !strings.Contains(rust, "& !") {
		t.Fatalf("Go bit clear return should lower to Rust '& !':\n%s", rust)
	}
}

func TestReturnLocalInterfaceSliceIndexKeepsElementHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	Name() string
}

var types []Type

func get(i int) Type {
	if types[i] != nil {
		return types[i]
	}
	return types[i]
}
`)

	if strings.Contains(rust, "Some({ let __seq") || strings.Contains(rust, "Some((*types.borrow") {
		t.Fatalf("returning a local-interface slice element should not wrap the existing handle:\n%s", rust)
	}
	if !strings.Contains(rust, "return (*types.borrow().as_ref().unwrap())") &&
		!strings.Contains(rust, "return { let __seq") {
		t.Fatalf("returning a local-interface slice element should return the cloned handle directly:\n%s", rust)
	}
}

func TestNamedDeferReturnBoxesConcreteLocalInterface(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	Underlying() Type
}

type Slice struct{}

func (*Slice) Underlying() Type { return nil }

func newType() (T Type) {
	defer func() {}()
	typ := new(Slice)
	return typ
}
`)

	if strings.Contains(rust, "*T.borrow_mut() = Some(new_val)") ||
		strings.Contains(rust, "*T.lock().unwrap() = Some(new_val)") {
		t.Fatalf("named local-interface return should not assign a concrete value into the interface slot:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(SlicePtr(typ.clone())) as Box<dyn Type") {
		t.Fatalf("named local-interface return should box a pointer-identity wrapper:\n%s", rust)
	}
}

func TestNamedDeferReturnBoxesConcreteError(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type parseError struct {
	msg string
}

func (*parseError) Error() string { return "" }

func parse() (ok bool, err error) {
	defer func() {}()
	return false, &parseError{msg: "bad"}
}
`)

	if strings.Contains(rust, "*err.borrow_mut() = __moved_val") ||
		strings.Contains(rust, "*err.lock().unwrap() = __moved_val") {
		t.Fatalf("named error return should not assign a concrete error option into the error slot:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(parseError") ||
		!strings.Contains(rust, "as Box<dyn StdError") {
		t.Fatalf("named error return should box the concrete error value:\n%s", rust)
	}
}

func TestNamedIntegerErrorConstReturnConstructsNamedError(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Errno uintptr

const EINVAL = Errno(22)

func (e Errno) Error() string { return "" }

func setenv() error {
	return EINVAL
}
`)

	if strings.Contains(rust, "Box::new(EINVAL)") {
		t.Fatalf("typed error const return should not box the raw const:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(Errno(") {
		t.Fatalf("typed error const return should construct the named error before boxing:\n%s", rust)
	}
}

func TestReturnErrorChannelReceiveDoesNotDoubleWrapOption(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func recv(ch <-chan error) error {
	return <-ch
}
`)

	if strings.Contains(rust, "Some(ch.recv().unwrap_or_default())") {
		t.Fatalf("error channel receive return should not double-wrap the optional error:\n%s", rust)
	}
	if !strings.Contains(rust, "Arc::new(Mutex::new(ch.recv().unwrap_or_default()))") &&
		!strings.Contains(rust, "Rc::new(RefCell::new(ch.recv().unwrap_or_default()))") {
		t.Fatalf("error channel receive return should wrap the received optional error directly:\n%s", rust)
	}
}

func TestChannelFieldAssignmentClonesReusableLocal(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type holder struct {
	ch chan int
}

func (h *holder) assign() {
	ch := make(chan int, 1)
	h.ch = ch
	ch <- 1
}
`)

	if strings.Contains(rust, "self.ch = ch;") {
		t.Fatalf("channel field assignment should not move a reusable channel local:\n%s", rust)
	}
	if !strings.Contains(rust, "self.ch = ch.clone();") {
		t.Fatalf("channel field assignment should clone the channel handle:\n%s", rust)
	}
}

func TestChannelFieldAssignmentUsesMutableStructTarget(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type sleeper struct {
	wake chan struct{}
}

func install(s *sleeper) {
	s.wake = make(chan struct{}, 1)
	s.wake = nil
}
`)

	if strings.Contains(rust, ".as_ref().unwrap()).wake =") {
		t.Fatalf("channel field assignment through a wrapped pointer should not use an immutable struct borrow:\n%s", rust)
	}
	if !strings.Contains(rust, "(*s.borrow_mut().as_mut().unwrap()).wake = GoChannel") &&
		!strings.Contains(rust, "(*s.lock().unwrap().as_mut().unwrap()).wake = GoChannel") {
		t.Fatalf("channel field assignment through a wrapped pointer should mutate the struct field:\n%s", rust)
	}
	if !strings.Contains(rust, ".wake = Default::default()") {
		t.Fatalf("nil channel field assignment should still lower to the channel default value:\n%s", rust)
	}
}

func TestSelectWithReturningCasesEmitsUnreachableTail(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func wait(a <-chan error, b <-chan error) error {
	select {
	case err := <-a:
		return err
	case err := <-b:
		return err
	}
}
`)

	if !strings.Contains(rust, "unreachable!()") {
		t.Fatalf("select with all returning cases should mark the fallthrough unreachable:\n%s", rust)
	}
}

func TestTerminatingFallthroughSwitchEmitsUnreachableTail(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func decide(state int) bool {
	switch state {
	case 0:
		return true
	case 1:
		return false
	case 2:
		fallthrough
	default:
		return false
	}
}
`)

	if !strings.Contains(rust, "_fallthrough = true;") {
		t.Fatalf("test did not exercise fallthrough switch lowering:\n%s", rust)
	}
	if !strings.Contains(rust, "unreachable!()") {
		t.Fatalf("terminating fallthrough switch should provide a diverging tail expression:\n%s", rust)
	}
}

func TestAddressOfConcreteReturnBoxesLocalInterface(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Expr interface {
	exprNode()
}

type BinaryExpr struct {
	X Expr
}

func (*BinaryExpr) exprNode() {}

func cloneExpr(x Expr) Expr {
	switch x := x.(type) {
	case *BinaryExpr:
		op := *x
		return &op
	}
	return nil
}
`)

	if strings.Contains(rust, "return op.clone()") {
		t.Fatalf("address-of concrete returned as interface should not return the local pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(BinaryExprPtr(op.clone()") || !strings.Contains(rust, "as Box<dyn Expr") {
		t.Fatalf("address-of concrete returned as interface should box a pointer-identity wrapper:\n%s", rust)
	}
}

func TestLabeledContinueBeforeLoopPostTerminatesRustStatement(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func scan(limit int) {
outer:
	for i := 0; i < limit; i++ {
		for j := 0; j < limit; j++ {
			continue outer
		}
	}
}
`)

	if !strings.Contains(rust, "continue 'outer;") {
		t.Fatalf("labeled continue followed by a loop post statement should be terminated:\n%s", rust)
	}
}

func TestLabeledBreakToSwitchEmitsLabeledBlock(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func scan(x int) int {
Switch:
	switch x {
	case 1:
		for {
			break Switch
		}
		return 10
	}
	return 20
}
`)

	if !strings.Contains(rust, "'switch: {") {
		t.Fatalf("labeled switch should emit a Rust labeled block:\n%s", rust)
	}
	if !strings.Contains(rust, "break 'switch") {
		t.Fatalf("break to switch label should target the emitted block:\n%s", rust)
	}
}

func TestLocalInterfaceAssignmentFromOwnMethodCallClonesReceiver(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Node interface {
	Pos() int
}

type Visitor interface {
	Visit(Node) Visitor
}

func Walk(v Visitor, node Node) {
	v = v.Visit(node)
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
	if strings.Contains(rust, "v = (*v.borrow().as_ref().unwrap()).visit") ||
		strings.Contains(rust, "v = (*v.lock().unwrap().as_ref().unwrap()).visit") {
		t.Fatalf("assignment from a method call on the same interface handle should not borrow the assigned variable directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let __recv = v.clone();") ||
		(!strings.Contains(rust, "let __iface_handle = { let __recv = v.clone(); let __result = (*__recv.borrow().as_ref().unwrap()).visit") &&
			!strings.Contains(rust, "let __iface_handle = { let __recv = v.clone(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).visit")) ||
		!strings.Contains(rust, "__result }") ||
		(!strings.Contains(rust, "*v.borrow_mut() = (*__iface_guard).clone();") &&
			!strings.Contains(rust, "*v.lock().unwrap() = (*__iface_guard).clone();")) {
		t.Fatalf("assignment from a method call on the same interface handle should clone the receiver before assignment:\n%s", rust)
	}
}

func TestCapturedValueReceiverAssignmentUpdatesBareClone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Value struct {
	n int
}

func (v Value) Elem() Value {
	return Value{n: v.n + 1}
}

func (v Value) Seq() func(func(Value) bool) {
	return func(yield func(Value) bool) {
		v = v.Elem()
		yield(v)
	}
}
`)

	if strings.Contains(rust, "*v_closure_clone.borrow_mut()") || strings.Contains(rust, "*v_closure_clone.lock().unwrap()") {
		t.Fatalf("captured value receiver assignment should update the bare closure clone, not a wrapper slot:\n%s", rust)
	}
	if !strings.Contains(rust, "v_closure_clone = __moved_val") {
		t.Fatalf("captured value receiver assignment should move the wrapped return value into the bare clone:\n%s", rust)
	}
	if strings.Contains(rust, "Some(self.clone())") {
		t.Fatalf("captured value receiver argument should use the closure clone, not self:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(v_closure_clone.clone())") {
		t.Fatalf("captured value receiver argument should wrap the closure clone:\n%s", rust)
	}
}

func TestRangeValueShadowingCapturedReceiverKeepsRangeBinding(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Value struct {
	n int
}

func ValueOf(x interface{}) Value {
	return Value{}
}

func (v Value) String() string {
	return "ab"
}

func (v Value) Seq2() func(func(Value, Value) bool) {
	return func(yield func(Value, Value) bool) {
		for i, v := range v.String() {
			if !yield(ValueOf(i), ValueOf(v)) {
				return
			}
		}
	}
}
`)

	if strings.Contains(rust, "Box::new(v_closure_clone)") {
		t.Fatalf("range value shadowing a captured receiver should not use the receiver capture rename:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(v) as Box<dyn Any") {
		t.Fatalf("range value shadowing a captured receiver should box the range binding:\n%s", rust)
	}
}

func TestLocalMapKeyRustTypeReportsTrackedPointerKey(t *testing.T) {
	prevCollections := localCollectionKinds
	prevMapKeys := localMapKeyRustTypes
	defer func() {
		localCollectionKinds = prevCollections
		localMapKeyRustTypes = prevMapKeys
	}()

	localCollectionKinds = map[string]string{"m": "map"}
	localMapKeyRustTypes = map[string]string{"m": "GoLocalPtrKey<scope>"}

	keyType, ok := localMapKeyRustType(ast.NewIdent("m"))
	if !ok || keyType != "GoLocalPtrKey<scope>" {
		t.Fatalf("localMapKeyRustType() = (%q, %v), want tracked pointer key", keyType, ok)
	}
}

func TestNestedReturnInsideFuncLitStaysExplicit(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func produce() func() {
	return func() {
		if true {
			return
		}
	}
}

func main() {
	produce()()
}
`)

	if strings.Contains(rust, "if true {\n        ()\n    }") {
		t.Fatalf("nested return inside func lit must not become trailing `()`:\n%s", rust)
	}
	if !strings.Contains(rust, "if true {\n        return;") &&
		!strings.Contains(rust, "if true {\n            return;") {
		t.Fatalf("nested return inside func lit should emit explicit return;\n%s", rust)
	}
}

func TestNamedIntegerConstBinaryUsesPrimitiveOperands(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type LoadMode int

const (
	NeedName LoadMode = 1 << iota
	NeedFiles
	NeedTypes
)

type Config struct {
	Mode LoadMode
}

func main() {
	_ = Config{Mode: NeedName | NeedFiles | NeedTypes}
}
`)

	mainIdx := strings.Index(rust, "fn main()")
	if mainIdx < 0 {
		t.Fatalf("expected fn main in generated output:\n%s", rust)
	}
	mainRust := rust[mainIdx:]
	if strings.Count(mainRust, "LoadMode(") > 1 {
		t.Fatalf("nested named-integer const operands must not each be wrapped in LoadMode(...) — expected a single outer wrap inside main():\n%s", mainRust)
	}
	if !strings.Contains(mainRust, "NEED_NAME as i32 | NEED_FILES as i32") {
		t.Fatalf("named-integer const bitwise OR should emit bare i32 operands:\n%s", mainRust)
	}
}

func TestConcurrentShortDeclBuildsComplexBinaryInitializerWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func decode(buf []byte) int64 {
	go func() {}()
	sec := int64(buf[7]) | int64(buf[6])<<8 | int64(buf[5])<<16 | int64(buf[4])<<24 |
		int64(buf[3])<<32 | int64(buf[2])<<40 | int64(buf[1])<<48 | int64(buf[0])<<56
	return sec
}
`)

	if strings.Contains(rust, "let mut sec = Arc::new(Mutex::new(Some({ let __tmp_x =") {
		t.Fatalf("complex binary short declaration should not inline the whole binary tree into the wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_binary_") {
		t.Fatalf("complex binary short declaration should build the initializer through statement locals:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut sec = Arc::new(Mutex::new(Some(__go_binary_") {
		t.Fatalf("short declaration should wrap the final binary initializer local:\n%s", rust)
	}
}

func TestConcurrentBareScalarReturnBuildsComplexBinaryWithStatements(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func decode(buf []byte) uint64 {
	go func() {}()
	return uint64(buf[0]) | uint64(buf[1])<<8 | uint64(buf[2])<<16 | uint64(buf[3])<<24 |
		uint64(buf[4])<<32 | uint64(buf[5])<<40 | uint64(buf[6])<<48 | uint64(buf[7])<<56
}
`)

	if strings.Contains(rust, "return { let __tmp_x =") {
		t.Fatalf("complex bare-scalar return should not inline the whole binary tree into the return expression:\n%s", rust)
	}
	if !strings.Contains(rust, "return {\n") || !strings.Contains(rust, "let __go_binary_") {
		t.Fatalf("complex bare-scalar return should build the return value through statement locals:\n%s", rust)
	}
}

func TestConcurrentPointerReturnBuildsComplexSequenceIndexAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type node struct {
	value int
}

type arena struct {
	spans [8]*node
}

type heap struct {
	arenas [][]*arena
}

func pick(h *heap, i int, j int, k int) *node {
	go func() {}()
	return h.arenas[i][j].spans[k]
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "; __seq[({ let __v = (*k") {
			t.Fatalf("complex pointer return sequence index should split sequence setup across lines:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "{\n        let __seq =") || !strings.Contains(rust, "\n        __seq[") {
		t.Fatalf("complex pointer return sequence index should build the sequence in a return block:\n%s", rust)
	}
}

func TestConcurrentStructLiteralWithMultipleComplexFieldsBreaksAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type fd struct {
	sysfd int
	isStream bool
	zeroReadIsEOF bool
}

type file struct {
	pfd fd
	name string
	stdoutOrErr bool
}

func build(raw int, name string) file {
	go func() {}()
	return file{
		pfd: fd{sysfd: raw, isStream: true, zeroReadIsEOF: true},
		name: name,
		stdoutOrErr: raw == 1 || raw == 2,
	}
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "file {") && strings.Contains(line, "stdout_or_err:") {
			t.Fatalf("struct literal with multiple complex fields should split fields across lines:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "file {\n") || !strings.Contains(rust, "\n        stdout_or_err:") {
		t.Fatalf("struct literal with multiple complex fields should use multiline struct syntax:\n%s", rust)
	}
}

func TestMethodReceiverShadowShortDeclUsesLocalIdent(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type thing struct {
	n int
}

func (t *thing) value(items []*thing) int {
	for _, scan := range items {
		t := scan
		return t.n
	}
	return 0
}
`)

	if strings.Contains(rust, "let mut self") {
		t.Fatalf("short declaration shadowing the receiver should not bind Rust self:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut t") {
		t.Fatalf("short declaration shadowing the receiver should keep the local identifier:\n%s", rust)
	}
}

func TestClosureCaptureOfReceiverShadowShortDeclUsesLocalHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type operand struct {
	n int
}

type term struct{}

func (t *term) is(fn func(*term) bool) bool {
	return fn(t)
}

func (x *operand) convertible(t *term) bool {
	if t != nil {
		x := *x
		return t.is(func(_ *term) bool {
			x.n = 1
			return true
		})
	}
	return false
}
`)

	if strings.Contains(rust, "x_closure_clone = (*self).clone()") {
		t.Fatalf("closure capture of receiver-shadowing local should not clone the receiver:\n%s", rust)
	}
	if !strings.Contains(rust, "x_closure_clone = x.clone()") {
		t.Fatalf("closure capture of receiver-shadowing local should clone the local handle:\n%s", rust)
	}
}

func TestMethodReceiverStructLiteralFieldUsesSelf(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Value struct {
	n int
}

type Iter struct {
	m Value
}

func (v Value) MapRange() *Iter {
	return &Iter{m: v}
}
`)

	if strings.Contains(rust, "m: v.clone()") {
		t.Fatalf("receiver used as a struct literal field should lower through self, not the Go receiver name:\n%s", rust)
	}
	if !strings.Contains(rust, "m: Rc::new(RefCell::new(Some(self.clone())))") {
		t.Fatalf("receiver used as a struct literal field should wrap a cloned self value:\n%s", rust)
	}
}

func TestMethodReceiverAssignmentAndCallArgumentUseSelf(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Value struct {
	n int
}

func accept(v Value) {
}

func (v Value) call() {
	var rcvr Value
	rcvr = v
	accept(v)
	_ = rcvr
}
`)

	if strings.Contains(rust, "v.borrow") || strings.Contains(rust, "v.clone()") {
		t.Fatalf("receiver assignment and call arguments should lower through self, not the Go receiver name:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = self.clone()") {
		t.Fatalf("receiver assignment should clone self into the wrapped target:\n%s", rust)
	}
	if !strings.Contains(rust, "accept(Rc::new(RefCell::new(Some(self.clone()))))") {
		t.Fatalf("receiver call argument should wrap a cloned self value:\n%s", rust)
	}
}

func TestValueReceiverReassignmentMovesWrappedReturnIntoLocalCopy(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Value struct {
	n int
}

func (v Value) Elem() Value {
	return v
}

func (v Value) Field(i int) Value {
	return v
}

func (v Value) Walk(index []int) Value {
	v = Value{n: 1}
	for _, x := range index {
		v = v.Elem()
		v = v.Field(x)
	}
	return v
}
`)

	if strings.Contains(rust, "*self.borrow_mut()") || strings.Contains(rust, "*self.lock().unwrap()") {
		t.Fatalf("value receiver reassignment should not treat self as a wrapped slot:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut __self = self.clone();") {
		t.Fatalf("value receiver reassignment should introduce a mutable local receiver copy:\n%s", rust)
	}
	if strings.Count(rust, "__self = __moved_val") < 2 {
		t.Fatalf("value receiver reassignment should move wrapped returns into the mutable receiver copy:\n%s", rust)
	}
	if !strings.Contains(rust, "Rc::new(RefCell::new(Some(__self.clone())))") {
		t.Fatalf("value receiver return should use the reassigned local receiver copy:\n%s", rust)
	}
}

func TestValueReceiverTupleAssignmentMovesReturnIntoLocalCopy(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type nat []int

func (z nat) div(x, y nat) (nat, nat) {
	return z, x
}

func (q nat) convert(r nat) nat {
	q, r = q.div(r, q)
	return q
}
`)

	if strings.Contains(rust, "*q.borrow") || strings.Contains(rust, "*q.lock") {
		t.Fatalf("value receiver tuple assignment should not write through an undefined receiver handle:\n%s", rust)
	}
	if !strings.Contains(rust, "__self = __moved_val") {
		t.Fatalf("value receiver tuple assignment should move the returned value into the local receiver copy:\n%s", rust)
	}
}

func TestNamedStringValueReceiverSliceReassignmentWrapsLocalCopy(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type StructTag string

func (tag StructTag) Trim(i int) StructTag {
	tag = tag[i:]
	return tag
}
`)

	if !strings.Contains(rust, "let mut __self = self.clone();") {
		t.Fatalf("named string receiver reassignment should introduce a mutable local receiver copy:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = StructTag(") || !strings.Contains(rust, "__self = new_val") {
		t.Fatalf("named string receiver reassignment should wrap the sliced string in the named type:\n%s", rust)
	}
	if strings.Contains(rust, "__self = __moved_val") {
		t.Fatalf("named string receiver reassignment should not move a bare string into the named receiver copy:\n%s", rust)
	}
}

func TestNamedStringValueReceiverComparisonUsesInnerString(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type StructTag string

func (tag StructTag) HasSuffix(i int) bool {
	tag = tag[i:]
	return tag != ""
}
`)

	if strings.Contains(rust, "__self.0.borrow().as_ref().unwrap()).0.borrow()") ||
		strings.Contains(rust, "__self.0.lock().unwrap().as_ref().unwrap()).0.lock()") {
		t.Fatalf("named string receiver comparison should not treat the raw string handle as another named value:\n%s", rust)
	}
	if !strings.Contains(rust, "(*__self.0.borrow().as_ref().unwrap()).clone()") &&
		!strings.Contains(rust, "(*__self.0.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("named string receiver comparison should clone the receiver's inner string:\n%s", rust)
	}
}

func TestNamedScalarReceiverCallArgumentKeepsNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type bitset uint64

func bitsetFirst(b bitset) uintptr {
	return uintptr(b)
}

func (b bitset) first() uintptr {
	return bitsetFirst(b)
}
`)

	if strings.Contains(rust, "(*self.0.borrow().as_ref().unwrap()).clone()") {
		t.Fatalf("named scalar receiver call argument should pass the named value, not the raw scalar:\n%s", rust)
	}
	if !strings.Contains(rust, "bitset_first(Rc::new(RefCell::new(Some(self.clone()))))") {
		t.Fatalf("named scalar receiver call argument should wrap a cloned self value:\n%s", rust)
	}
}

func TestLocalInterfaceFieldAssignmentMutablyBorrowsStruct(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	Name() string
}

type Method struct {
	Type Type
}

func assign(m Method, typ Type) Method {
	m.Type = typ
	return m
}
`)

	if strings.Contains(rust, ".borrow().as_ref().unwrap()).r#type =") ||
		strings.Contains(rust, ".lock().unwrap().as_ref().unwrap()).r#type =") {
		t.Fatalf("local interface field assignment should not replace the field through an immutable struct borrow:\n%s", rust)
	}
	if !strings.Contains(rust, "*(*m.borrow_mut().as_mut().unwrap()).r#type.borrow_mut() =") &&
		!strings.Contains(rust, "*(*m.lock().unwrap().as_mut().unwrap()).r#type.lock().unwrap() =") {
		t.Fatalf("local interface field assignment should borrow the struct mutably and update the field slot:\n%s", rust)
	}
}

func TestWrappedStructReturnClonesBeforeWrapping(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Method struct {
	Name string
}

func method() Method {
	var m Method
	return m
}
`)

	if strings.Contains(rust, "Rc::new(RefCell::new(Some(m.borrow().as_ref().unwrap().clone())))") ||
		strings.Contains(rust, "Arc::new(Mutex::new(Some(m.lock().unwrap().as_ref().unwrap().clone())))") {
		t.Fatalf("wrapped struct return should not wrap a borrow temporary directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let __owned = m.borrow().as_ref().unwrap().clone();") &&
		!strings.Contains(rust, "let __owned = m.lock().unwrap().as_ref().unwrap().clone();") {
		t.Fatalf("wrapped struct return should clone before constructing the wrapper:\n%s", rust)
	}
}

func TestParallelShortDeclPointerIdentClonesHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

type RegArgs struct{}

func assign(frame unsafe.Pointer, regs *RegArgs) (unsafe.Pointer, *RegArgs) {
	valueFrame, valueRegs := frame, regs
	return valueFrame, valueRegs
}
`)

	if strings.Contains(rust, "Some((*regs.borrow().as_ref().unwrap()))") ||
		strings.Contains(rust, "Some((*regs.lock().unwrap().as_ref().unwrap()))") {
		t.Fatalf("parallel short declaration should not move the pointee out of a pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "regs.clone()") {
		t.Fatalf("parallel short declaration should clone pointer handles:\n%s", rust)
	}
}

func TestUnsafePointerShadowShortDeclUsesOuterHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

func add(p unsafe.Pointer, x uintptr) unsafe.Pointer {
	return unsafe.Pointer(uintptr(p) + x)
}

func store(base uintptr, offset uintptr, wide bool) {
	dst := unsafe.Pointer(base)
	if wide {
		dst0 := (*uintptr)(add(dst, offset))
		*dst0 = 1
	} else {
		dst := (*uintptr)(add(dst, offset))
		*dst = 2
	}
}
`)

	if strings.Contains(rust, "Some(dst)") {
		t.Fatalf("shadowing unsafe pointer short declaration should not wrap the new local as the RHS argument:\n%s", rust)
	}
	if strings.Count(rust, "let __arg_holder = dst.clone();") < 2 {
		t.Fatalf("shadowing unsafe pointer short declaration should read the outer pointer handle on the RHS:\n%s", rust)
	}
}

func TestScalarConversionShadowShortDeclUsesOuterValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func div(sec int64) uint64 {
	if sec > 0 {
		sec := uint64(sec)
		return sec
	}
	return 0
}
`)

	if strings.Contains(rust, "Some(sec as u64)") {
		t.Fatalf("shadowing scalar conversion short declaration should unwrap the outer value before casting:\n%s", rust)
	}
	if !strings.Contains(rust, "(*sec.borrow().as_ref().unwrap()) as u64") {
		t.Fatalf("shadowing scalar conversion short declaration should cast the outer wrapped value:\n%s", rust)
	}
}

func TestFuncLitSelectorAssignmentClonesCapturedTarget(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type table struct {
	Equal func(int) bool
	Values []int
}

func assign(typ *table, comparable bool) {
	typ.Equal = nil
	if comparable {
		typ.Equal = func(v int) bool {
			return len(typ.Values) == v
		}
	}
}
`)

	targetClone := strings.Index(rust, "let __func_lit_target = ")
	if targetClone == -1 {
		t.Fatalf("function literal assignment should clone selector target before the move closure:\n%s", rust)
	}
	newValue := strings.Index(rust, "let new_val = Box::new(move |")
	if newValue == -1 {
		t.Fatalf("function literal assignment should lower through a boxed move closure:\n%s", rust)
	}
	if targetClone > newValue {
		t.Fatalf("function literal target clone should be emitted before the move closure:\n%s", rust)
	}
	if !strings.Contains(rust, "; *__func_lit_target") {
		t.Fatalf("function literal assignment should write through the cloned target handle:\n%s", rust)
	}
}

func TestParallelAssignmentBareScalarsMutatesBareLocals(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func triple() (int, int, bool) {
	return 1, 2, true
}

func swap() int {
	i, _, _ := triple()
	i2, _, _ := triple()
	i, i2 = i2, i
	return i
}
`)

	if strings.Contains(rust, "*i.borrow") || strings.Contains(rust, "*i.lock") ||
		strings.Contains(rust, "*i2.borrow") || strings.Contains(rust, "*i2.lock") {
		t.Fatalf("parallel assignment of bare scalars should not treat locals as wrapper handles:\n%s", rust)
	}
	if !strings.Contains(rust, "i = __tmp_0;") || !strings.Contains(rust, "i2 = __tmp_1;") {
		t.Fatalf("parallel assignment of bare scalars should assign temporaries directly:\n%s", rust)
	}
}

func TestParallelAssignmentCurrentReceiverValueFieldsSnapshotValues(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type box struct {
	a int
	b int
}

func (x *box) rotate(v int) int {
	old := x.a
	x.a, x.b, v = x.b, v, x.a
	return old + x.a + x.b + v
}
`)

	if strings.Contains(rust, "let __tmp_0 = self.b.clone()") ||
		strings.Contains(rust, "let __tmp_2 = self.a.clone()") {
		t.Fatalf("parallel assignment should snapshot current receiver value fields, not their handles:\n%s", rust)
	}
	if !strings.Contains(rust, "let __tmp_0 = { let __selector_holder = self.b.clone()") ||
		!strings.Contains(rust, "let __tmp_2 = { let __selector_holder = self.a.clone()") {
		t.Fatalf("parallel assignment should clone current receiver field values through selector holders:\n%s", rust)
	}
	if strings.Contains(rust, "*v.borrow_mut() = __tmp_2.borrow_mut().take()") ||
		strings.Contains(rust, "*v.lock().unwrap() = __tmp_2.lock().unwrap().take()") {
		t.Fatalf("parallel assignment should not move a receiver field handle into a scalar local:\n%s", rust)
	}
	if !strings.Contains(rust, "*v.borrow_mut() = Some(__tmp_2);") &&
		!strings.Contains(rust, "*v.lock().unwrap() = Some(__tmp_2);") {
		t.Fatalf("parallel assignment should store the receiver field value snapshot into the scalar local:\n%s", rust)
	}
}

func TestGenericComparableMapRangeKeyStaysTypeParamValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func MapKeys[K comparable, V any](m map[K]V) []K {
	r := make([]K, 0, len(m))
	for k := range m {
		r = append(r, k)
	}
	return r
}
`)

	if strings.Contains(rust, "GoLocalPtrKey<K>") || strings.Contains(rust, "__range_key.value()") {
		t.Fatalf("generic comparable map keys should range as K values, not pointer-key wrappers:\n%s", rust)
	}
	if !strings.Contains(rust, "BTreeMap<K, Rc<RefCell<Option<V>>>>") &&
		!strings.Contains(rust, "BTreeMap<K, Arc<Mutex<Option<V>>>>") {
		t.Fatalf("generic comparable map should use K directly as the Rust key type:\n%s", rust)
	}
}

func TestCompoundAssignRangeIndexCastsToIntField(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type scanner struct {
	rdOffset int
	src []byte
}

func (s *scanner) advance() {
	for rdOffset := range s.src[s.rdOffset:] {
		s.rdOffset += rdOffset
	}
}
`)

	if strings.Contains(rust, "let __rhs = rdOffset;") {
		t.Fatalf("compound assignment to int field should not use usize range index without a cast:\n%s", rust)
	}
	if !strings.Contains(rust, "let __rhs = (rdOffset as i32);") {
		t.Fatalf("compound assignment to int field should cast usize range index to i32:\n%s", rust)
	}
}

func TestRangeOverSliceExpressionBindsFieldBoundBeforeIterator(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type scanner struct {
	rdOffset int
	src []byte
}

func (s *scanner) advance() {
	for rdOffset, b := range s.src[s.rdOffset:] {
		if b == ' ' {
			s.rdOffset += rdOffset
			break
		}
	}
}
`)

	if strings.Contains(rust, "__seq[(*self.rd_offset.clone().lock().unwrap().as_ref().unwrap()) as usize..]") ||
		strings.Contains(rust, "__seq[(*self.rd_offset.clone().borrow().as_ref().unwrap()) as usize..]") {
		t.Fatalf("slice range should not keep the field-bound borrow in the iterator expression:\n%s", rust)
	}
	if !strings.Contains(rust, "let __low = (*self.rd_offset.clone()") ||
		!strings.Contains(rust, "__seq[__low..].to_vec()") {
		t.Fatalf("slice range should bind the field bound before slicing:\n%s", rust)
	}
}

func TestDeferClosureCompoundAssignUsesCapturedIdentRename(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type parser struct {
	nestLev int
}

func (p *parser) parse() {
	var n int
	defer func() { p.nestLev -= n }()
	for n = 1; n < 3; n++ {
		p.nestLev++
	}
}
`)

	if !strings.Contains(rust, "let n_defer_captured = n.clone();") {
		t.Fatalf("deferred closure should clone the captured local:\n%s", rust)
	}
	if strings.Contains(rust, "let __rhs = n as i32") ||
		strings.Contains(rust, "let __rhs = (*n.borrow") ||
		strings.Contains(rust, "let __rhs = (*n.lock") {
		t.Fatalf("deferred closure compound assignment should not read the outer binding directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let __rhs = (*n_defer_captured.borrow") &&
		!strings.Contains(rust, "let __rhs = (*n_defer_captured.lock") {
		t.Fatalf("deferred closure compound assignment should read the captured clone:\n%s", rust)
	}
}

func TestDeferClosureFmtErrorfUsesCapturedErrorIdent(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func parse(filename string) (err error) {
	defer func() {
		if err != nil {
			err = fmt.Errorf("reading %s: %v", filename, err)
		}
	}()
	return fmt.Errorf("boom")
}
`)

	if !strings.Contains(rust, "err_defer_captured") {
		t.Fatalf("deferred closure should clone the named error result:\n%s", rust)
	}
	if strings.Contains(rust, "format!(\"{}\", (*err.borrow") ||
		strings.Contains(rust, "format!(\"{}\", (*err.lock") {
		t.Fatalf("fmt.Errorf inside deferred closure should not format the outer err binding:\n%s", rust)
	}
	if !strings.Contains(rust, "format!(\"{}\", (*err_defer_captured.borrow") &&
		!strings.Contains(rust, "format!(\"{}\", (*err_defer_captured.lock") {
		t.Fatalf("fmt.Errorf inside deferred closure should format the captured err clone:\n%s", rust)
	}
}

func TestDeferClosureAssignedCapturedResultIsMutable(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type File struct{}

func parse() (f *File) {
	defer func() {
		if f == nil {
			f = &File{}
		}
	}()
	return
}
`)

	if !strings.Contains(rust, "let mut f_defer_captured = f.clone();") {
		t.Fatalf("deferred closure should make directly assigned captured result handles mutable:\n%s", rust)
	}
}

func TestDeferFuncLiteralSelectorArgumentCapturesValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type env struct {
	hasCallOrRecv bool
}

type checker struct {
	env
}

func (check *checker) builtin() {
	defer func(b bool) {
		check.hasCallOrRecv = b
	}(check.hasCallOrRecv)
	check.hasCallOrRecv = false
}
`)

	if strings.Contains(rust, "has_call_or_recv.clone())))") ||
		strings.Contains(rust, "has_call_or_recv.clone(); __field })") {
		t.Fatalf("deferred selector argument should capture the selector value, not the field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone()") &&
		!strings.Contains(rust, "let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone()") {
		t.Fatalf("deferred selector argument should unwrap and clone the field value:\n%s", rust)
	}
}

func TestDeferFuncLiteralPointerSelectorArgumentKeepsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type scope struct{}

type checker struct {
	scope *scope
}

func (check *checker) stmt() {
	defer func(scope *scope) {
		if scope == check.scope {
		}
	}(check.scope)
	check.scope = nil
}
`)

	if strings.Contains(rust, "Some((*check_defer_captured") && strings.Contains(rust, ".scope.clone())") {
		t.Fatalf("deferred pointer selector argument should not wrap the pointer handle inside Some:\n%s", rust)
	}
	if !strings.Contains(rust, "let __defer_arg_0 = (*check_defer_captured.borrow().as_ref().unwrap()).scope.clone();") &&
		!strings.Contains(rust, "let __defer_arg_0 = (*check_defer_captured.lock().unwrap().as_ref().unwrap()).scope.clone();") &&
		!strings.Contains(rust, "let __defer_arg_0 = check_defer_captured.scope.clone();") {
		t.Fatalf("deferred pointer selector argument should capture the pointer handle:\n%s", rust)
	}
}

func TestTrailingInfiniteForWithDeferSuppressesFinalDeferDrain(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type parser struct {
	nestLev int
}

func (p *parser) parse() int {
	var n int
	defer func() { p.nestLev -= n }()
	for n = 1; ; n++ {
		if n > 2 {
			return p.nestLev
		}
		p.nestLev++
	}
}
`)

	if got := strings.Count(rust, "// Execute deferred functions"); got != 1 {
		t.Fatalf("infinite trailing for loop should not emit a second final defer drain, got %d:\n%s", got, rust)
	}
}

func TestImportedTranspiledInterfaceAssignmentCopiesHandle(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevContext := currentContext
	defer func() {
		currentTypeInfo = prevTypeInfo
		currentContext = prevContext
	}()

	astPkg := types.NewPackage("go/ast", "ast")
	posMethod := types.NewFunc(
		token.NoPos,
		astPkg,
		"Pos",
		types.NewSignatureType(
			nil,
			nil,
			nil,
			types.NewTuple(),
			types.NewTuple(types.NewVar(token.NoPos, nil, "", types.Typ[types.Int])),
			false,
		),
	)
	exprType := types.NewNamed(
		types.NewTypeName(token.NoPos, astPkg, "Expr", nil),
		types.NewInterfaceType([]*types.Func{posMethod}, nil).Complete(),
		nil,
	)
	currentPkg := types.NewPackage("go/parser", "parser")
	lhs := ast.NewIdent("key")
	rhs := ast.NewIdent("other")

	SetTypeInfo(&TypeInfo{
		info: &types.Info{
			Types: map[ast.Expr]types.TypeAndValue{
				lhs: {Type: exprType},
				rhs: {Type: exprType},
			},
		},
		pkg: currentPkg,
	})
	SetTranspileContext(&TranspileContext{
		PackageMapping: map[string]string{"go/ast": "go_ast"},
	})

	var out strings.Builder
	if !writeLocalInterfaceHandleAssignment(&out, lhs, rhs) {
		t.Fatal("imported source-mapped interface assignment should copy the interface handle")
	}
	rust := out.String()
	if strings.Contains(rust, "Some(other") {
		t.Fatalf("interface handle assignment should not wrap an existing handle inside the slot:\n%s", rust)
	}
	if !strings.Contains(rust, "let __iface_handle = other.clone()") ||
		(!strings.Contains(rust, "*key.borrow_mut() = (*__iface_guard).clone();") &&
			!strings.Contains(rust, "*key.lock().unwrap() = (*__iface_guard).clone();")) {
		t.Fatalf("interface handle assignment should copy the RHS interface value into the existing slot:\n%s", rust)
	}

	out.Reset()
	writeParallelAssignmentTarget(&out, lhs, "__tmp_0", rhs)
	rust = out.String()
	if strings.Contains(rust, "Some(__tmp_0") {
		t.Fatalf("parallel interface handle assignment should not wrap an existing handle inside the slot:\n%s", rust)
	}
	if !strings.Contains(rust, "let __iface_handle = __tmp_0") ||
		(!strings.Contains(rust, "*key.borrow_mut() = (*__iface_guard).clone();") &&
			!strings.Contains(rust, "*key.lock().unwrap() = (*__iface_guard).clone();")) {
		t.Fatalf("parallel interface handle assignment should copy the captured RHS interface value into the existing slot:\n%s", rust)
	}
}

func TestSourceMappedInterfaceSelectorShortDeclKeepsHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/constant"

type env struct {
	iota constant.Value
}

func take(v constant.Value) {}

func use(e *env) {
	iota := e.iota
	take(iota)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/constant": "go_constant"})
	if strings.Contains(rust, "Some((*") && strings.Contains(rust, ".iota.clone()") {
		t.Fatalf("source-mapped interface selector short declaration should not wrap the field handle inside a new handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut iota =") || !strings.Contains(rust, ".iota.clone();") {
		t.Fatalf("source-mapped interface selector short declaration should clone the existing field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "take(iota.clone())") {
		t.Fatalf("source-mapped interface selector short declaration should pass the cloned handle:\n%s", rust)
	}
}

func TestInterfaceAssertionShortDeclKeepsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Type interface {
	typeNode()
}

type genericType interface {
	Type
	TypeParams() int
}

type Alias struct{}

func (Alias) typeNode() {}
func (Alias) TypeParams() int { return 1 }

func use(orig Type) int {
	orig_ := orig.(genericType)
	return orig_.TypeParams()
}

func useReturn(orig Type) genericType {
	return orig.(genericType)
}
`)

	if !strings.Contains(rust, "Box::new(typed_val.clone()) as Box<dyn genericType") {
		t.Fatalf("interface assertion should box the asserted concrete value as the target interface:\n%s", rust)
	}
	rcHandle := "Rc::new(RefCell::new(Some(Box::new(typed_val.clone()) as Box<dyn genericType"
	arcHandle := "Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn genericType"
	if !strings.Contains(rust, rcHandle) && !strings.Contains(rust, arcHandle) {
		t.Fatalf("interface assertion short declaration should keep the normal interface handle:\n%s", rust)
	}
	if strings.Contains(rust, "return Rc::new(RefCell::new(Some(({") ||
		strings.Contains(rust, "return Arc::new(Mutex::new(Some(({") {
		t.Fatalf("interface assertion return should not wrap an existing interface handle again:\n%s", rust)
	}
}

func TestFunctionTypeInterfaceAssertionBoxesAdapterWrapper(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type value interface {
	Set(string)
	String() string
}

type boolFlag interface {
	value
	IsBoolFlag() bool
}

type boolFuncValue func(string)

func (f boolFuncValue) Set(s string) {
	f(s)
}

func (f boolFuncValue) String() string {
	return ""
}

func (f boolFuncValue) IsBoolFlag() bool {
	return true
}

type flag struct {
	value value
}

func use(f flag) bool {
	v, ok := f.value.(boolFlag)
	return ok && v.IsBoolFlag()
}

func useAny(x any) string {
	v := x.(value)
	return v.String()
}

func useAnyDirect(x any) string {
	return x.(value).String()
}
`)

	if strings.Contains(rust, "Box::new(typed_val.clone()) as Box<dyn boolFlag") {
		t.Fatalf("function type assertion should not box the raw function handle as the target interface:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(boolFuncValueAsboolFlag(typed_val.clone())) as Box<dyn boolFlag") {
		t.Fatalf("function type assertion should box the generated target-interface adapter:\n%s", rust)
	}
	if strings.Contains(rust, "Box::new(typed_val.clone()) as Box<dyn value") {
		t.Fatalf("function type assertion should not box the raw function handle as the base interface:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new(boolFuncValueAsvalue(typed_val.clone())) as Box<dyn value") {
		t.Fatalf("function type assertion should box the generated base-interface adapter:\n%s", rust)
	}
	if strings.Contains(rust, "}).string()") {
		t.Fatalf("method call on named-interface assertion should not dispatch on the wrapper handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __recv = ({") || !strings.Contains(rust, "let __result = (*__recv.borrow().as_ref().unwrap()).string(); __result }") {
		t.Fatalf("method call on named-interface assertion should unwrap the asserted trait object receiver:\n%s", rust)
	}
	if !strings.Contains(rust, "fn __go_as_any(&self) -> &dyn std::any::Any {\n        &self.0\n    }") {
		t.Fatalf("function type interface adapter should expose the original function value as the Go dynamic type:\n%s", rust)
	}
}

func TestAnonymousInterfaceAssertionPointerMethodCallUsesMutableBorrow(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Alias struct {
	params []int
}

func (a *Alias) SetTypeParams(params []int) {
	a.params = params
}

func setTypeParams(alias *Alias, params []int) {
	if alias, ok := any(alias).(interface {
		SetTypeParams(params []int)
	}); ok {
		alias.SetTypeParams(params)
	}
}
`)

	if strings.Contains(rust, ".as_ref().unwrap()).set_type_params") {
		t.Fatalf("anonymous interface assertion method call should not borrow pointer receiver immutably:\n%s", rust)
	}
	if !strings.Contains(rust, ".as_mut().unwrap()).set_type_params") {
		t.Fatalf("anonymous interface assertion method call should borrow the asserted concrete handle mutably:\n%s", rust)
	}
}

func TestExternalLocalInterfaceMethodUsesMutableTraitReceiver(t *testing.T) {
	method := types.NewFunc(token.NoPos, nil, "SetTypeParams", types.NewSignatureType(nil, nil, nil, nil, nil, false))
	prevInterfaceMethodMutableReceiver := interfaceMethodMutableReceiver
	interfaceMethodMutableReceiver = map[*types.Func]bool{method: true}
	t.Cleanup(func() {
		interfaceMethodMutableReceiver = prevInterfaceMethodMutableReceiver
	})

	var out strings.Builder
	writeExternalLocalInterfaceMethod(&out, "Alias", "SetTypeParams", &ast.FuncType{}, externalLocalInterfaceImpl{}, method)
	if !strings.Contains(out.String(), "fn set_type_params(&mut self)") {
		t.Fatalf("external local-interface impl should match a mutable trait receiver:\n%s", out.String())
	}
}

func TestExternalLocalInterfaceMethodUsesMutableTraitReceiverByName(t *testing.T) {
	prevInterfaceMethodMutableReceiverByTrait := interfaceMethodMutableReceiverByTrait
	interfaceMethodMutableReceiverByTrait = map[string]bool{
		interfaceMethodMutableReceiverTraitKey("go_types::Importer", "Import"): true,
	}
	t.Cleanup(func() {
		interfaceMethodMutableReceiverByTrait = prevInterfaceMethodMutableReceiverByTrait
	})

	var out strings.Builder
	writeExternalLocalInterfaceMethod(&out, "go_types::Importer", "Import", &ast.FuncType{}, externalLocalInterfaceImpl{}, nil)
	if !strings.Contains(out.String(), "fn import(&mut self)") {
		t.Fatalf("external local-interface impl should match a mutable trait receiver by method name:\n%s", out.String())
	}
}

func TestConcreteErrorAssertionMethodCallUnwrapsReceiver(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Errno uintptr

func (e Errno) Error() string { return "" }
func (e Errno) Is(target error) bool { return false }

type syscallErrorType = Errno

func underlyingErrorIs(err, target error) bool {
	e, ok := err.(syscallErrorType)
	return ok && e.Is(target)
}
`)

	if strings.Contains(rust, "e.is(target.clone())") {
		t.Fatalf("asserted concrete error method call should not call through the wrapper handle:\n%s", rust)
	}
	if strings.Contains(rust, "downcast_ref::<syscallErrorType>()") {
		t.Fatalf("type assertion to an error alias should downcast to the underlying concrete value, not the alias handle:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<Errno>()") {
		t.Fatalf("type assertion to an error alias should downcast to the underlying concrete error:\n%s", rust)
	}
	if !strings.Contains(rust, "Errno::is(&(*e.borrow().as_ref().unwrap()), target.clone())") &&
		!strings.Contains(rust, "Errno::is(&(*e.lock().unwrap().as_ref().unwrap()), target.clone())") &&
		!strings.Contains(rust, "(*e.borrow().as_ref().unwrap()).is(target.clone())") &&
		!strings.Contains(rust, "(*e.lock().unwrap().as_ref().unwrap()).is(target.clone())") {
		t.Fatalf("asserted concrete error method call should unwrap the receiver:\n%s", rust)
	}
}

func TestPointerErrorAssertionDowncastsConcretePayload(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type parseError struct {
	code int
}

func (e *parseError) Error() string { return "" }

func makeErr() error {
	return &parseError{code: 7}
}

func code() int {
	err := makeErr()
	if err != nil && err.(*parseError).code != 0 {
		return err.(*parseError).code
	}
	return 0
}
`)

	if strings.Contains(rust, "downcast_ref::<Arc<Mutex<Option<parseError>>>>") ||
		strings.Contains(rust, "downcast_ref::<Rc<RefCell<Option<parseError>>>>") {
		t.Fatalf("error assertion to *parseError should not downcast to the wrapper handle:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<parseError>()") {
		t.Fatalf("error assertion to *parseError should downcast the concrete error payload:\n%s", rust)
	}
	if !strings.Contains(rust, "Arc::new(Mutex::new(Some(typed_val.clone())))") &&
		!strings.Contains(rust, "Rc::new(RefCell::new(Some(typed_val.clone())))") {
		t.Fatalf("error assertion to *parseError should rebuild the pointer handle from the payload:\n%s", rust)
	}
}

func TestAnyAssignedErrorThenAssertedErrorUsesAnyBox(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "errors"

func normalize(err any) error {
	if s, ok := err.(string); ok {
		err = errors.New(s)
	}
	return err.(error)
}
`)

	if strings.Contains(rust, "downcast_ref::<error>()") {
		t.Fatalf("error assertion from any should not use the Go identifier as a Rust type:\n%s", rust)
	}
	if strings.Contains(rust, "err = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error") ||
		strings.Contains(rust, "err = Rc::new(RefCell::new(Some(Box::<dyn std::error::Error") {
		t.Fatalf("assigning error to any should store a Box<dyn Any>, not an error handle:\n%s", rust)
	}
	if !strings.Contains(rust, "as Box<dyn Any") {
		t.Fatalf("assigning error to any should box the dynamic value as any:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::<dyn StdError") {
		t.Fatalf("asserting any to error should rebuild an error interface value:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(({") || !strings.Contains(rust, "let typed_val = any_val.downcast_ref::<std::string::String>()") {
		t.Fatalf("error assertion used as an error result should be stored in a wrapped error handle:\n%s", rust)
	}
}

func TestErrorPassedToAnyPreservesDynamicErrorValue(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type myErr struct{}

func (myErr) Error() string { return "mine" }

func take(v any) {}

func pass(err error) {
	take(err)
}
`)

	if strings.Contains(rust, `format!("{}",`) {
		t.Fatalf("error-to-any lowering should not preserve only formatted error text:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<myErr>()") {
		t.Fatalf("error-to-any lowering should downcast the dynamic concrete error through go/types candidates:\n%s", rust)
	}
	if !strings.Contains(rust, `go_box_any_with_metadata(typed_val.clone(), "struct", true)`) {
		t.Fatalf("error-to-any lowering should box the dynamic concrete error with go/types metadata:\n%s", rust)
	}
}

func TestConcurrentPanicErrorPayloadBreaksDowncastChainAcrossLines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type alphaErr struct{}
func (alphaErr) Error() string { return "alpha" }

type betaErr struct{}
func (betaErr) Error() string { return "beta" }

type gammaErr struct{}
func (gammaErr) Error() string { return "gamma" }

type deltaErr struct{}
func (deltaErr) Error() string { return "delta" }

func fail(err error) {
	go func() {}()
	panic(err)
}
`)

	for _, line := range strings.Split(rust, "\n") {
		if strings.Contains(line, "match __err_guard.as_ref()") && strings.Contains(line, "downcast_ref::<") {
			t.Fatalf("error-to-any downcast chain should split across lines:\n%s", rust)
		}
	}
	if !strings.Contains(rust, "match __err_guard.as_ref() {\n") {
		t.Fatalf("error-to-any downcast chain should use multiline match syntax:\n%s", rust)
	}
	if !strings.Contains(rust, "downcast_ref::<alphaErr>()") ||
		!strings.Contains(rust, "downcast_ref::<betaErr>()") ||
		!strings.Contains(rust, "downcast_ref::<gammaErr>()") ||
		!strings.Contains(rust, "downcast_ref::<deltaErr>()") {
		t.Fatalf("error-to-any downcast chain should preserve concrete error candidates:\n%s", rust)
	}
}

func TestPointerAssertionCallArgumentKeepsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Object interface {
	object()
}

type TypeName struct{}

func (*TypeName) object() {}

type Checker struct{}

func (c *Checker) collectMethods(obj *TypeName) {}

func (c *Checker) packageObjects(objects []Object) {
	for _, obj := range objects {
		if obj, _ := obj.(*TypeName); obj != nil {
			c.collectMethods(obj)
		}
	}
}
`)

	if strings.Contains(rust, "collect_methods((*obj).clone())") ||
		strings.Contains(rust, "collect_methods((*obj.borrow") ||
		strings.Contains(rust, "collect_methods((*obj.lock") {
		t.Fatalf("type-asserted pointer call argument should pass the handle, not clone the pointee slot:\n%s", rust)
	}
	if !strings.Contains(rust, "collect_methods(obj.clone())") {
		t.Fatalf("type-asserted pointer call argument should clone the pointer handle:\n%s", rust)
	}
}

func TestMakeAnonymousStructSliceShortDeclRegistersElementType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func count() int {
	structs := make([]struct {
		name string
	}, 0)
	return len(structs)
}
`)

	if strings.Contains(rust, "/* unknown struct */") {
		t.Fatalf("make([]struct{...}) short declaration should use the generated anonymous struct type:\n%s", rust)
	}
	if !strings.Contains(rust, "Vec<AnonymousStruct1>") || !strings.Contains(rust, "Vec::<AnonymousStruct1>::with_capacity") {
		t.Fatalf("make([]struct{...}) short declaration should annotate and initialize with the anonymous struct type:\n%s", rust)
	}
}
