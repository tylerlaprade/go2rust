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
