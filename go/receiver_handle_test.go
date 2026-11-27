package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
	"strings"
	"testing"
)

// A pointer method takes the handle it was called on when its receiver
// escapes, is reassigned, or is handed to such a method; one that only reads
// and writes its receiver's fields does not (DEV-165).
func TestReceiverHandleMethodsAreTheOnesWhoseReceiverEscapes(t *testing.T) {
	src := `package main

type T struct {
	m    map[string]int
	next *T
	n    int
}

var all []*T

func (t *T) fields()        { t.n++; t.m = nil; _ = *t; _ = t == nil }
func (t *T) self() *T       { return t }
func (t *T) keep()          { all = append(all, t) }
func (t *T) walk()          { for t.next != nil { t = t.next } }
func (t *T) later()         { defer t.fields() }
func (t *T) value() func()  { return t.fields }
func (t *T) viaSelf() *T    { t.n++; return t.self() }
func (t *T) viaFields()     { t.fields() }
func (t T) byValue() T      { return t }
`
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
	if err != nil {
		t.Fatal(err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatal(err)
	}
	named := typeInfo.pkg.Scope().Lookup("T").Type()
	want := map[string]bool{
		"fields": false, "self": true, "keep": true, "walk": true, "later": true,
		"value": true, "viaSelf": true, "viaFields": false, "byValue": false,
	}
	for name, takes := range want {
		obj, _, _ := types.LookupFieldOrMethod(types.NewPointer(named), true, typeInfo.pkg, name)
		fn, ok := obj.(*types.Func)
		if !ok {
			t.Fatalf("no method %s", name)
		}
		if got := methodTakesReceiverHandle(fn); got != takes {
			t.Errorf("%s takes its receiver's handle: got %v, want %v", name, got, takes)
		}
	}
}

// The method binds its receiver to the hinted handle, or runs on self
// without one, and a call on a pointer variable hints the handle and holds
// no borrow of it.
func TestReceiverHandleMethodBindsTheHintedHandle(t *testing.T) {
	src := `package main

import "fmt"

type Node struct {
	name string
	next *Node
}

func (n *Node) Link(name string) *Node {
	n.next = &Node{name: name}
	return n
}

func main() {
	p := &Node{name: "p"}
	p.Link("a").Link("b")
	fmt.Println(p.next.name)
}
`
	rust := transpileReceiverHandleSource(t, src)
	for _, want := range []string{
		"if let Some(mut n) = go_take_receiver_handle::<Self>(self as *const Self as usize) {",
		"} else {",
		"let __recv = p.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.link(",
		"fn go_take_receiver_handle<T: 'static>(address: usize)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
}

// An element of a map of pointers is the pointer's handle, whatever its key
// says: the choice reads the container's metadata, not the generated text.
func TestReceiverHandleMapElementGetsAHint(t *testing.T) {
	src := `package main

type T struct{ next *T }

var seen []*T

func (t *T) Visit() {
	t.next = &T{}
	seen = append(seen, t)
}

func main() {
	byName := map[string]*T{"GoPtr": {}}
	byName["GoPtr"].Visit()
}
`
	rust := transpileReceiverHandleSource(t, src)
	if !strings.Contains(rust, "go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.visit()") {
		t.Fatalf("the map element call should record a hint:\n%s", rust)
	}
}

// A local that shadows the receiver, `x := *x`, is a captured local in a
// closure: a call on it there names the closure's copy of its handle, so the
// closure stays Fn.
func TestReceiverHandleShadowingLocalInClosureUsesCaptureName(t *testing.T) {
	src := `package main

type T struct{ n int }

type Set struct{ items []int }

func (s *Set) each(f func(int) bool) bool {
	for _, v := range s.items {
		if !f(v) {
			return false
		}
	}
	return true
}

var seen []*T

func (x *T) fits(a, b *Set) bool {
	seen = append(seen, x)
	if a != nil && b != nil {
		x := *x // don't clobber the caller's T
		return a.each(func(i int) bool {
			x.n = i
			return b.each(func(j int) bool { return x.fits(nil, nil) && x.n+j > 0 })
		})
	}
	return x.n >= 0
}
`
	rust := transpileReceiverHandleSource(t, src)
	if !strings.Contains(rust, "let __recv = x_closure_clone_closure_clone.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv)") {
		t.Fatalf("the inner closure should call through its copy of x's handle:\n%s", rust)
	}
	if strings.Contains(rust, "let __recv = x.clone();") {
		t.Fatalf("the inner closure should not name the shadowing x itself:\n%s", rust)
	}
}

func transpileReceiverHandleSource(t *testing.T, src string) string {
	t.Helper()
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, parser.ParseComments)
	if err != nil {
		t.Fatal(err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatal(err)
	}
	return transpileParsedRegression(t, file, fset, typeInfo)
}

// unsafe.Pointer(t) in a method that took its handle is the address of the
// struct in the handle's cell, the address self has in a method that did
// not, so both methods give the same address for the same receiver.
func TestReceiverHandleUnsafePointerIsThePointeeAddress(t *testing.T) {
	src := `package main

import "unsafe"

type T struct{ n int }

var keep []*T

func (t *T) plain() uintptr { return uintptr(unsafe.Pointer(t)) }

func (t *T) escaping() uintptr {
	keep = append(keep, t)
	return uintptr(unsafe.Pointer(t))
}
`
	rust := transpileReceiverHandleSource(t, src)
	if !strings.Contains(rust, "self as *const _ as usize") {
		t.Fatalf("plain should take self's address:\n%s", rust)
	}
	if !strings.Contains(rust, "let __self_guard = t.borrow(); __self_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0)") {
		t.Fatalf("escaping should take the address of the struct in its receiver's cell:\n%s", rust)
	}
	if strings.Contains(rust, "Rc::as_ptr(&t)") {
		t.Fatalf("escaping should not take the address of the receiver's cell:\n%s", rust)
	}
}
