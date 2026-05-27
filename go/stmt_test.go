package main

import (
	"go/ast"
	"go/parser"
	"go/token"
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
	if !strings.Contains(rust, "_ts_guard.as_ref().map(|__v| __v.__go_as_any())") {
		t.Fatalf("type switch on local interface field should downcast through __go_as_any:\n%s", rust)
	}
	if strings.Contains(rust, "let _ts_val = _ts_guard.as_ref();") {
		t.Fatalf("type switch on local interface field should not treat trait objects as bare Any:\n%s", rust)
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
	if !strings.Contains(fnRust, "downcast_ref::<GenDecl>()") ||
		!strings.Contains(fnRust, "downcast_ref::<AssignStmt>()") {
		t.Fatalf("type switch case interfaces should check concrete implementors from go/types:\n%s", rust)
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
	if !strings.Contains(rust, "match __map.get(&GoLocalPtrKey::new(s.clone()))") {
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
	if !strings.Contains(rust, "p = (*q).clone()") &&
		!strings.Contains(rust, "p = q.clone()") {
		t.Fatalf("range assignment should copy the local interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, "top = (*nodes.borrow().as_ref().unwrap())[(0) as usize].clone()") &&
		!strings.Contains(rust, "top = { let __seq =") {
		t.Fatalf("index assignment should replace the local interface handle from the slice element:\n%s", rust)
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
		(!strings.Contains(rust, "v = { let __recv = v.clone(); let __result = (*__recv.borrow().as_ref().unwrap()).visit") &&
			!strings.Contains(rust, "v = { let __recv = v.clone(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).visit")) ||
		!strings.Contains(rust, "__result }") {
		t.Fatalf("assignment from a method call on the same interface handle should clone the receiver before assignment:\n%s", rust)
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
