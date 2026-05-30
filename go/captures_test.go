package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestClosureCapturesUseTypeInfoScopes(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "encoding/binary"

func makeSeq(pkgs []string) func(func(string) bool) {
	return func(yield func(string) bool) {
		for _, pkg := range pkgs {
			var seen [2]byte
			var visit func(int) bool
			visit = func(i int) bool {
				bit := byte(1) << (i % 8)
				if seen[i/8]&bit == 0 {
					seen[i/8] |= bit
					var data []byte
					_, _ = binary.Uvarint(data)
					if i > 0 && !visit(i-1) {
						return false
					}
					if !yield(pkg) {
						return false
					}
				}
				return true
			}
			if !visit(0) {
				return
			}
		}
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

	var returnStmt *ast.ReturnStmt
	var rangeStmt *ast.RangeStmt
	var assignStmt *ast.AssignStmt
	ast.Inspect(file, func(n ast.Node) bool {
		switch node := n.(type) {
		case *ast.ReturnStmt:
			if returnStmt == nil {
				returnStmt = node
			}
		case *ast.RangeStmt:
			if rangeStmt == nil {
				rangeStmt = node
			}
		case *ast.AssignStmt:
			if len(node.Rhs) == 1 {
				if _, ok := node.Rhs[0].(*ast.FuncLit); ok {
					assignStmt = node
				}
			}
		}
		return true
	})
	if returnStmt == nil {
		t.Fatal("did not find return statement with outer closure")
	}
	if rangeStmt == nil {
		t.Fatal("did not find range statement containing inner closure")
	}
	if assignStmt == nil {
		t.Fatal("did not find assignment statement with inner closure")
	}

	sp := NewStatementPreprocessor(fset)
	outerInfo := sp.PreprocessStatement(returnStmt, nil)
	if outerInfo == nil {
		t.Fatal("outer closure should capture pkgs")
	}
	if _, ok := outerInfo.CapturedVars["pkgs"]; !ok {
		t.Fatalf("outer closure should capture pkgs, got %#v", outerInfo.CapturedVars)
	}
	for _, name := range []string{"binary", "byte", "seen", "uint64", "visit", "yield"} {
		if _, ok := outerInfo.CapturedVars[name]; ok {
			t.Fatalf("outer closure should not capture %q, got %#v", name, outerInfo.CapturedVars)
		}
	}

	rangeInfo := sp.PreprocessStatement(rangeStmt, nil)
	if rangeInfo != nil && len(rangeInfo.CapturedVars) > 0 {
		t.Fatalf("range statement should leave body closure captures to body statements, got %#v", rangeInfo.CapturedVars)
	}

	innerInfo := sp.PreprocessStatement(assignStmt, nil)
	if innerInfo == nil {
		t.Fatal("inner closure should capture outer closure locals")
	}
	for _, name := range []string{"pkg", "seen", "visit", "yield"} {
		if _, ok := innerInfo.CapturedVars[name]; !ok {
			t.Fatalf("inner closure should capture %q, got %#v", name, innerInfo.CapturedVars)
		}
	}
	for _, name := range []string{"binary", "byte"} {
		if _, ok := innerInfo.CapturedVars[name]; ok {
			t.Fatalf("inner closure should not capture %q, got %#v", name, innerInfo.CapturedVars)
		}
	}

	var clones strings.Builder
	sp.GenerateCloneStatements(&clones, innerInfo)
	got := clones.String()
	if strings.Contains(got, " = yield.clone()") {
		t.Fatalf("keyword parameter capture should be escaped, got %q", got)
	}
	if !strings.Contains(got, "r#yield.clone()") {
		t.Fatalf("keyword parameter capture should use raw identifier, got %q", got)
	}
}

func TestReceiverClosureCloneClonesReceiverValue(t *testing.T) {
	prevReceiver := currentReceiver
	prevRenames := currentCaptureRenames
	currentReceiver = "analysis"
	currentCaptureRenames = nil
	defer func() {
		currentReceiver = prevReceiver
		currentCaptureRenames = prevRenames
	}()

	info := &CaptureInfo{
		CapturedVars: map[string]string{
			"analysis": "analysis_closure_clone",
		},
	}
	var clones strings.Builder
	NewStatementPreprocessor(nil).GenerateCloneStatements(&clones, info)

	if got, want := clones.String(), "let mut analysis_closure_clone = (*self).clone(); "; got != want {
		t.Fatalf("receiver closure clone = %q, want %q", got, want)
	}
}

func TestClosureCapturesNamedMapReceiverClone(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Node interface {
	Pos() int
}

type CommentMap map[Node][]int

func Inspect(node Node, f func(Node) bool) {}

func (cmap CommentMap) Filter(node Node) CommentMap {
	umap := make(CommentMap)
	Inspect(node, func(n Node) bool {
		if g := cmap[n]; len(g) > 0 {
			umap[n] = g
		}
		return true
	})
	return umap
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
	fnIndex := strings.Index(rust, "pub fn filter")
	if fnIndex < 0 {
		t.Fatalf("generated Rust did not contain filter method:\n%s", rust)
	}
	fnRust := rust[fnIndex:]
	if !strings.Contains(fnRust, "cmap_closure_clone") {
		t.Fatalf("closure should capture the named map receiver clone:\n%s", rust)
	}
	if strings.Contains(fnRust, "self.0.clone()") {
		t.Fatalf("closure body should not capture self by reference after cloning the receiver:\n%s", rust)
	}
}

func TestIfConditionFuncLitCapturesReceiverClone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Checker struct{}

func under(fn func(int) bool) bool {
	return fn(1)
}

func (check *Checker) fail(n int) {}

func (check *Checker) builtin() bool {
	if under(func(u int) bool {
		check.fail(u)
		return false
	}) {
		return true
	}
	return false
}
`)

	if strings.Contains(rust, "check.fail") {
		t.Fatalf("if-condition closure should not emit the Go receiver name:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut check_closure_clone = (*self).clone();") {
		t.Fatalf("if-condition closure should clone the receiver inline:\n%s", rust)
	}
	if !strings.Contains(rust, "check_closure_clone.fail") {
		t.Fatalf("if-condition closure should call through the cloned receiver:\n%s", rust)
	}
}

func TestIfConditionFuncLitCapturesParameterClone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func under(fn func(int) bool) bool {
	return fn(1)
}

func builtin(id int) bool {
	if under(func(u int) bool {
		return u == id
	}) {
		return true
	}
	return id == 0
}
`)

	if strings.Contains(rust, "(*id.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y") ||
		strings.Contains(rust, "(*id.borrow().as_ref().unwrap()).clone(); let __tmp_y") {
		t.Fatalf("if-condition closure should not capture the original parameter by move:\n%s", rust)
	}
	if !strings.Contains(rust, "let id_closure_clone = id.clone();") {
		t.Fatalf("if-condition closure should clone the captured parameter inline:\n%s", rust)
	}
	if !strings.Contains(rust, "(*id_closure_clone.lock().unwrap().as_ref().unwrap())") &&
		!strings.Contains(rust, "(*id_closure_clone.borrow().as_ref().unwrap())") {
		t.Fatalf("if-condition closure should read through the captured parameter clone:\n%s", rust)
	}
}

func TestIfConditionFuncLitBoxesCapturedInterfaceArgThroughClone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type positioner interface {
	pos() int
}

type operand struct {
	n int
}

func (*operand) pos() int {
	return 0
}

func under(fn func(int) bool) bool {
	return fn(1)
}

func report(positioner) {}

func clear(x *operand) bool {
	if under(func(u int) bool {
		report(x)
		return true
	}) {
		return true
	}
	return false
}
`)

	if strings.Contains(rust, "Box::new((*x.borrow()") ||
		strings.Contains(rust, "Box::new((*x.lock()") {
		t.Fatalf("captured interface argument should not box through the outer variable:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new((*x_closure_clone.borrow()") &&
		!strings.Contains(rust, "Box::new((*x_closure_clone.lock()") {
		t.Fatalf("captured interface argument should box through the closure clone:\n%s", rust)
	}
}

func TestIfConditionFuncLitAssignedCaptureCloneIsMutable(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func under(fn func(int) bool) bool {
	return fn(1)
}

func builtin(key int) int {
	if under(func(u int) bool {
		key = u
		return true
	}) {
		return key
	}
	return key
}
`)

	if strings.Contains(rust, "let key_closure_clone = key.clone();") {
		t.Fatalf("assigned if-condition closure capture should not be immutable:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut key_closure_clone = key.clone();") {
		t.Fatalf("assigned if-condition closure capture should be mutable:\n%s", rust)
	}
}

func TestSyntaxClosurePredeclaredConversionsAreNotCaptured(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func process(fn func(string) string) {}

func main() {
	process(func(s string) string {
		runes := []rune(s)
		return string(runes)
	})
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	var funcLit *ast.FuncLit
	ast.Inspect(file, func(n ast.Node) bool {
		if funcLit != nil {
			return false
		}
		lit, ok := n.(*ast.FuncLit)
		if ok {
			funcLit = lit
			return false
		}
		return true
	})
	if funcLit == nil {
		t.Fatal("did not find function literal")
	}

	captured := NewStatementPreprocessor(fset).CapturedVarsForFuncLit(funcLit)
	for _, name := range []string{"rune", "string"} {
		if captured[name] {
			t.Fatalf("predeclared conversion %q should not be captured, got %#v", name, captured)
		}
	}
}

func TestClosureParameterDoesNotCaptureOuterName(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func use(T any) func(any) any {
	visit := func(T any) any {
		return T
	}
	return visit
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

	var assignStmt *ast.AssignStmt
	ast.Inspect(file, func(n ast.Node) bool {
		if assignStmt != nil {
			return false
		}
		assign, ok := n.(*ast.AssignStmt)
		if !ok || len(assign.Rhs) != 1 {
			return true
		}
		if _, ok := assign.Rhs[0].(*ast.FuncLit); ok {
			assignStmt = assign
			return false
		}
		return true
	})
	if assignStmt == nil {
		t.Fatal("did not find assignment statement with closure")
	}

	sp := NewStatementPreprocessor(fset)
	info := sp.PreprocessStatement(assignStmt, nil)
	if info != nil {
		if _, ok := info.CapturedVars["T"]; ok {
			t.Fatalf("closure parameter T should not capture outer T, got %#v", info.CapturedVars)
		}
	}
}

func TestClosureTypeSwitchCaseVarIsLocal(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func makeVisitor(prefix string) func(any) string {
	return func(value any) string {
		switch node := value.(type) {
		case string:
			return prefix + node
		case int:
			return prefix
		}
		return prefix
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

	var returnStmt *ast.ReturnStmt
	ast.Inspect(file, func(n ast.Node) bool {
		if returnStmt != nil {
			return false
		}
		node, ok := n.(*ast.ReturnStmt)
		if !ok || len(node.Results) != 1 {
			return true
		}
		if _, ok := node.Results[0].(*ast.FuncLit); ok {
			returnStmt = node
			return false
		}
		return true
	})
	if returnStmt == nil {
		t.Fatal("did not find return statement with closure")
	}

	sp := NewStatementPreprocessor(fset)
	info := sp.PreprocessStatement(returnStmt, nil)
	if info == nil {
		t.Fatal("closure should capture prefix")
	}
	if _, ok := info.CapturedVars["prefix"]; !ok {
		t.Fatalf("closure should capture prefix, got %#v", info.CapturedVars)
	}
	for _, name := range []string{"node", "value", "string", "int"} {
		if _, ok := info.CapturedVars[name]; ok {
			t.Fatalf("closure should not capture %q, got %#v", name, info.CapturedVars)
		}
	}
}

func TestCachedFuncLitCapturesReturnCopy(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func outer(prefix string) func(string) string {
	return func(name string) string {
		return prefix + name
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

	var funcLit *ast.FuncLit
	ast.Inspect(file, func(n ast.Node) bool {
		if funcLit != nil {
			return false
		}
		if node, ok := n.(*ast.FuncLit); ok {
			funcLit = node
			return false
		}
		return true
	})
	if funcLit == nil {
		t.Fatal("did not find closure")
	}

	sp := NewStatementPreprocessor(fset)
	captured := sp.CapturedVarsForFuncLit(funcLit)
	if _, ok := captured["prefix"]; !ok {
		t.Fatalf("closure should capture prefix, got %#v", captured)
	}
	captured["name"] = true
	captured["extra"] = true

	again := sp.CapturedVarsForFuncLit(funcLit)
	if _, ok := again["prefix"]; !ok {
		t.Fatalf("cached closure should still capture prefix, got %#v", again)
	}
	for _, name := range []string{"name", "extra"} {
		if _, ok := again[name]; ok {
			t.Fatalf("cached closure captures should not include caller mutation %q, got %#v", name, again)
		}
	}
	if len(sp.funcLitCaptures) != 1 {
		t.Fatalf("expected one cached closure capture entry, got %d", len(sp.funcLitCaptures))
	}
}

func TestClosureStructLiteralFieldKeysAreNotCaptured(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Config struct {
	Mode string
	BuildFlags string
}

type Request struct {
	Mode string
	BuildFlags string
}

func makeRequest(prefix string) func(Config) Request {
	return func(cfg Config) Request {
		return Request{
			Mode: cfg.Mode,
			BuildFlags: prefix + cfg.BuildFlags,
		}
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

	var returnStmt *ast.ReturnStmt
	ast.Inspect(file, func(n ast.Node) bool {
		if returnStmt != nil {
			return false
		}
		node, ok := n.(*ast.ReturnStmt)
		if !ok || len(node.Results) != 1 {
			return true
		}
		if _, ok := node.Results[0].(*ast.FuncLit); ok {
			returnStmt = node
			return false
		}
		return true
	})
	if returnStmt == nil {
		t.Fatal("did not find return statement with closure")
	}

	sp := NewStatementPreprocessor(fset)
	info := sp.PreprocessStatement(returnStmt, nil)
	if info == nil {
		t.Fatal("closure should capture prefix")
	}
	if _, ok := info.CapturedVars["prefix"]; !ok {
		t.Fatalf("closure should capture prefix, got %#v", info.CapturedVars)
	}
	for _, name := range []string{"Mode", "BuildFlags", "cfg"} {
		if _, ok := info.CapturedVars[name]; ok {
			t.Fatalf("closure should not capture %q, got %#v", name, info.CapturedVars)
		}
	}
}

func TestSyntaxClosureCompositeLiteralTypeIsNotCaptured(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Config struct {
	Mode string
}

type Request struct {
	Mode string
}

func makeRequest(prefix string) func(Config) Request {
	return func(cfg Config) Request {
		return Request{
			Mode: prefix + cfg.Mode,
		}
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	var funcLit *ast.FuncLit
	ast.Inspect(file, func(n ast.Node) bool {
		if funcLit != nil {
			return false
		}
		returnStmt, ok := n.(*ast.ReturnStmt)
		if !ok || len(returnStmt.Results) != 1 {
			return true
		}
		if lit, ok := returnStmt.Results[0].(*ast.FuncLit); ok {
			funcLit = lit
			return false
		}
		return true
	})
	if funcLit == nil {
		t.Fatal("did not find returned closure")
	}

	prevTypeInfo := GetTypeInfo()
	SetTypeInfo(nil)
	defer SetTypeInfo(prevTypeInfo)

	captured := findCapturedVarsSyntaxFallback(funcLit)
	if _, ok := captured["prefix"]; !ok {
		t.Fatalf("closure should capture prefix, got %#v", captured)
	}
	for _, name := range []string{"Request", "Mode", "cfg"} {
		if _, ok := captured[name]; ok {
			t.Fatalf("syntax capture should not capture %q, got %#v", name, captured)
		}
	}
}
