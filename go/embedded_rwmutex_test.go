package main

import (
	"go/ast"
	"strings"
	"testing"
)

// A struct embedding sync.RWMutex gets forwarders of its methods that call
// the bare GoRWMutex, and a struct embedding that struct forwards them
// through a clone of the mutex taken without holding the embedded struct's
// guard, so a forwarder that waits for the lock holds no other lock.
func TestEmbeddedRWMutexMethodsAreForwarded(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type Table struct {
	sync.RWMutex
	m map[string]int
}

func (t *Table) Read(k string) int {
	t.RLock()
	defer t.RUnlock()
	return t.m[k]
}

type Registry struct {
	*Table
}

func main() {
	r := Registry{Table: &Table{m: map[string]int{}}}
	r.Lock()
	r.Unlock()
	_ = r.Read("a")
}
`)
	for _, want := range []string{
		"pub fn r_lock(&self) {\n        let embedded_ref = &self.r_w_mutex;\n        embedded_ref.r_lock()",
		"pub fn __go_rw_mutex(&self) -> GoRWMutex {\n        let embedded = self.table.clone();",
		"pub fn lock(&self) {\n        self.__go_rw_mutex().lock()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "self.r_w_mutex.clone(); let mut __promoted_guard") {
		t.Fatalf("the embedded RWMutex is held bare and is not locked as a cell:\n%s", rust)
	}
}

// A directly embedded sync.RWMutex is at depth zero and wins over the one a
// struct embedded beside it has, whatever the field order.
func TestDirectlyEmbeddedRWMutexWinsOverPromotedOne(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type Table struct {
	sync.RWMutex
}

type Both struct {
	*Table
	sync.RWMutex
}

func (b *Both) Hold() {
	b.Lock()
	b.Unlock()
}
`)
	start := strings.Index(rust, "impl Both {")
	if start < 0 {
		t.Fatalf("missing impl Both:\n%s", rust)
	}
	both := rust[start:]
	if !strings.Contains(both, "pub fn lock(&self) {\n        let embedded_ref = &self.r_w_mutex;\n        embedded_ref.lock()") {
		t.Fatalf("Both's lock should be its own mutex's:\n%s", both)
	}
	if strings.Contains(both, "fn lock(&self) {\n        self.__go_rw_mutex().lock()") {
		t.Fatalf("Both's lock should not go through Table:\n%s", both)
	}
}

// Go promotes a sync.RWMutex method from the shallowest embedded field, so
// with Outer{ *Middle; *Shallow } and Middle's mutex one level deeper than
// Shallow's, Outer's forwarders go through Shallow even though Middle comes
// first.
func TestRWMutexMethodsComeFromTheShallowestEmbeddedField(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type Deep struct {
	sync.RWMutex
}

type Middle struct {
	*Deep
}

type Shallow struct {
	sync.RWMutex
}

type Outer struct {
	*Middle
	*Shallow
}

func main() {
	o := Outer{Middle: &Middle{Deep: &Deep{}}, Shallow: &Shallow{}}
	o.Lock()
	o.Shallow.Unlock()
}
`)
	start := strings.Index(rust, "impl Outer {\n    pub fn __go_rw_mutex")
	if start < 0 {
		t.Fatalf("missing Outer's RWMutex forwarders:\n%s", rust)
	}
	outer := rust[start:]
	if end := strings.Index(outer[1:], "\nimpl "); end >= 0 {
		outer = outer[:end+1]
	}
	if !strings.Contains(outer, "let embedded = self.shallow.clone();") {
		t.Fatalf("Outer's mutex should be Shallow's:\n%s", outer)
	}
	if strings.Contains(outer, "self.middle.clone()") {
		t.Fatalf("Outer's mutex should not come through Middle:\n%s", outer)
	}
}

// r.Lock() on a struct variable whose type embeds sync.RWMutex takes the
// mutex out of r's cell in a block of its own and then waits on it, so a
// goroutine waiting for the lock holds nothing the owner needs to reach
// r.Unlock() through the same variable.
func TestForwardedRWMutexCallOnAVariableReleasesItsCell(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type Table struct {
	sync.RWMutex
	m map[string]int
}

type Registry struct {
	*Table
}

func main() {
	r := Registry{Table: &Table{m: map[string]int{}}}
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		r.Lock()
		r.m["a"] = 1
		r.Unlock()
	}()
	wg.Wait()
}
`)
	for _, want := range []string{
		".__go_rw_mutex(); __sync_value }.lock()",
		".__go_rw_mutex(); __sync_value }.unlock()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, ".as_mut().unwrap()).lock()") || strings.Contains(rust, ".as_ref().unwrap()).lock()") {
		t.Fatalf("Lock is called holding the variable's cell:\n%s", rust)
	}
}

// When go/types cannot say which embedded field a sync.RWMutex method is
// promoted through, its forwarder fails loudly instead of locking whichever
// mutex comes first.
func TestRWMutexForwarderWithoutSelectionIsUnimplemented(t *testing.T) {
	methods := rwMutexForwardersForSelection([]externalPromotedMethod{
		{RustMethodName: "lock", ThroughRWMutexAccessor: true},
		{RustMethodName: rwMutexAccessorName, RWMutexAccessor: true},
	}, false)
	var out strings.Builder
	for _, method := range methods {
		generateExternalPromotedMethod(&out, method)
	}
	rust := out.String()
	for _, want := range []string{
		"pub fn lock(&self) {\n        unimplemented!(\"type info required for promoting sync.RWMutex methods\")",
		"pub fn __go_rw_mutex(&self) -> GoRWMutex {\n        unimplemented!(",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if kept := rwMutexForwardersForSelection([]externalPromotedMethod{{RustMethodName: "lock"}}, true); kept[0].Unimplemented != "" {
		t.Fatalf("a known selection should keep its forwarder: %+v", kept[0])
	}
}

// When forwarders of methods named Lock, Unlock, RLock and RUnlock promoted
// from deeper in the struct have taken the names, a directly embedded
// sync.RWMutex, which Go selects, still gets the __go_rw_mutex accessor
// that a call on a struct variable reaches the mutex through.
func TestRWMutexAccessorIsWrittenWhenItsMethodNamesAreTaken(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type Custom struct{ n int }

func (c *Custom) Lock()    { c.n++ }
func (c *Custom) Unlock()  { c.n-- }
func (c *Custom) RLock()   {}
func (c *Custom) RUnlock() {}

type Deep struct {
	*Custom
}

type Outer struct {
	*Deep
	sync.RWMutex
}

func (o *Outer) Hold() {
	o.Lock()
	o.RWMutex.Unlock()
}

func main() {
	o := Outer{Deep: &Deep{Custom: &Custom{}}}
	o.Lock()
	o.Unlock()
	o.Hold()
}
`)
	if !strings.Contains(rust, "pub fn __go_rw_mutex(&self) -> GoRWMutex {\n        self.r_w_mutex.clone()") {
		t.Fatalf("Outer should reach its own mutex through __go_rw_mutex:\n%s", rust)
	}
	outer := rustImplOf(rust, "Outer")
	if !strings.Contains(outer, "pub fn lock(&self) {\n        let embedded_ref = &self.r_w_mutex;\n        embedded_ref.lock()") {
		t.Fatalf("Outer.Lock should forward to Outer's own mutex, which Go selects over Custom.Lock:\n%s", outer)
	}
	if strings.Count(outer, "pub fn lock(") != 1 {
		t.Fatalf("Outer should have one lock forwarder:\n%s", outer)
	}
}

// rustImplOf returns the inherent impl blocks of typeName in rust.
func rustImplOf(rust, typeName string) string {
	var blocks []string
	header := "impl " + typeName + " {"
	for rest := rust; ; {
		start := strings.Index(rest, header)
		if start < 0 {
			break
		}
		rest = rest[start:]
		end := strings.Index(rest, "\n}\n")
		if end < 0 {
			blocks = append(blocks, rest)
			break
		}
		blocks = append(blocks, rest[:end])
		rest = rest[end:]
	}
	return strings.Join(blocks, "\n}\n")
}

func TestRWMutexMethodValueKeepsTheMutexItWasMadeFrom(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type Table struct {
	sync.RWMutex
}

type Registry struct {
	*Table
}

func main() {
	held := &Table{}
	reg := &Registry{Table: held}
	held.Lock()
	unlock := reg.Unlock
	reg.Table = &Table{}
	unlock()
}
`)
	if !strings.Contains(rust, "let __recv = { let __sync_value = (*__recv") || !strings.Contains(rust, "__go_rw_mutex(); __sync_value }; Rc::new(move || { __recv.unlock() })") {
		t.Fatalf("reg.Unlock should take the mutex when the method value is made:\n%s", rust)
	}
}

// o.Deep.Lock() through pointer fields takes the mutex out in a block of its
// own, so the call waits holding none of the guards on the way.
func TestForwardedRWMutexCallThroughPointerFieldsReleasesTheirGuards(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type Deep struct {
	sync.RWMutex
}

type Middle struct {
	*Deep
}

type Outer struct {
	*Middle
}

func main() {
	o := Outer{Middle: &Middle{Deep: &Deep{}}}
	o.Deep.Lock()
	o.Deep.Unlock()
}
`)
	if !strings.Contains(rust, ".deep.borrow().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();") {
		t.Fatalf("o.Deep.Lock() should take the mutex out before waiting:\n%s", rust)
	}
}

// defer r.Unlock() takes the mutex when the defer runs, as Go follows the
// embedded fields then.
func TestDeferredRWMutexUnlockTakesTheMutexWhenDeferred(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type Table struct {
	sync.RWMutex
}

type Registry struct {
	*Table
}

func (r *Registry) Swap() {
	r.Lock()
	defer r.Unlock()
	r.Table = &Table{}
}

func main() {
	reg := &Registry{Table: &Table{}}
	reg.Swap()
}
`)
	if !strings.Contains(rust, "let __defer_mutex = { let __recv = ") || !strings.Contains(rust, "__defer_mutex.unlock();\n    }));") {
		t.Fatalf("defer r.Unlock() should capture the mutex when deferred:\n%s", rust)
	}
}

// Without type info for the owner struct, the forwarders of its promoted
// methods do not guess which one Go selects.
func TestPromotedMethodsWithoutTypeInfoAreUnimplemented(t *testing.T) {
	oldContext := currentContext
	currentContext = nil
	SetTypeInfo(nil)
	defer func() { currentContext = oldContext }()
	method := &ast.FuncDecl{
		Name: ast.NewIdent("Hello"),
		Recv: &ast.FieldList{List: []*ast.Field{{
			Names: []*ast.Ident{ast.NewIdent("b")},
			Type:  &ast.StarExpr{X: ast.NewIdent("Base")},
		}}},
		Type: &ast.FuncType{Params: &ast.FieldList{}},
		Body: &ast.BlockStmt{},
	}
	structDef := &StructDef{
		EmbeddedTypes: []string{"Base"},
		ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
			{Type: ast.NewIdent("Base")},
		}}},
	}
	var out strings.Builder
	count := 0
	writePromotedMethodForwarders(&out, "Outer", structDef, map[string][]*ast.FuncDecl{"Base": {method}}, nil, nil, &count)
	got := out.String()
	if !strings.Contains(got, "pub fn hello(") || !strings.Contains(got, `unimplemented!("type info required for selecting a promoted method")`) {
		t.Fatalf("the forwarder should be unimplemented without type info:\n%s", got)
	}
}
