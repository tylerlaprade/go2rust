package main

import (
	"go/ast"
	"strings"
	"testing"
)

const structCopyProgram = `package main

type Point struct{ X, Y int }

type Line struct{ A, B Point }

type Grid struct{ Cells [2]Point }

type Config struct{ Name string }

func (c Config) WithName(name string) Config {
	c.Name = name
	return c
}

type Counter struct{ n int }

func (c *Counter) Later() func() { return func() { c.n++ } }

func use(p Point) {}

func main() {
	a := Point{1, 2}
	b := a
	var c Point
	c = a
	use(a)
	points := []Point{a}
	e := points[0]
	for _, p := range points {
		p.X = 1
	}
	arr := [2]Point{a, a}
	arr2 := arr
	for _, q := range arr {
		q.X = 1
	}
	name := "x"
	other := name
	_, _, _, _, _ = b, c, e, arr2, other
}
`

// Wherever Go copies a struct, the generated code must call the struct's copy
// helper. Clone on a struct clones the handles of its field cells, so it is a
// second handle on the same struct; using it for "b := a" made b.X = 9 change
// a.
func TestGoCopyOfAStructUsesTheCopyHelper(t *testing.T) {
	rust := transpileTypedRegression(t, structCopyProgram)

	for what, want := range map[string]string{
		"b := a":                   "let mut b = { let __owned = a.borrow().as_ref().unwrap().__go_value_clone();",
		"c = a":                    "{ let new_val = a.borrow().as_ref().unwrap().__go_value_clone(); *c.borrow_mut() = Some(new_val); }",
		"use(a)":                   "(*__arg_guard.as_ref().unwrap()).__go_value_clone() }",
		"e := points[0]":           "[(0) as usize].clone().__go_value_clone()",
		"range over a slice":       "for p in __range_values.iter() {\n        let p = &p.__go_value_clone();",
		"range over an array":      "let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| __e.__go_value_clone()).collect::<Vec<_>>()).unwrap_or_default();",
		"arr2 := arr":              "arr.borrow().as_ref().unwrap().each_ref().map(|__go_elem| __go_elem.__go_value_clone())",
		"a struct inside a struct": "(*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone())",
		"an array inside a struct": "(*__guard).as_ref().map(|__go_field| __go_field.each_ref().map(|__go_elem| __go_elem.__go_value_clone()))",
		"a value receiver":         "let mut __self = self.__go_value_clone();",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s should be Go's copy of the struct, missing %q", what, want)
		}
	}
	if t.Failed() {
		t.Logf("generated:\n%s", rust)
	}
}

// The copy helper is only for a copy. A pointer receiver that a closure
// captures has to stay the same struct, and a string is copied by Clone as
// before.
func TestGoCopyLeavesHandlesAndPlainValuesAlone(t *testing.T) {
	rust := transpileTypedRegression(t, structCopyProgram)

	for what, want := range map[string]string{
		"a closure over a pointer receiver": "= (*self).clone();",
		"other := name":                     "let mut other = { let __owned = name.borrow().as_ref().unwrap().clone();",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s must not use the struct copy, missing %q in:\n%s", what, want, rust)
		}
	}
	if strings.Contains(rust, "(*self).__go_value_clone(); ") {
		t.Errorf("a captured pointer receiver was copied:\n%s", rust)
	}
}

func TestGoValueCopyCallForType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Point struct{ X int }

type Node struct {
	p    *Point
	tags []string
	m    map[string]int
	pos  Point
}

func main() {
	n := Node{}
	_ = n
}
`)
	// The copy of a Node copies pos and keeps the cells of the fields Go
	// shares: the pointer, the slice and the map.
	for _, want := range []string{
		"self.p.clone()",
		"self.tags.clone()",
		"self.m.clone()",
		"(*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone())",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
}

// A range over a slice reads each element when its turn comes, so the
// snapshot taken before the loop must stay a set of second handles: a deep
// copy there would hide a field that the loop writes to a later element.
func TestRangeOverASliceOfStructsReadsEachElementInTurn(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type P struct{ X int }

func main() {
	s := []P{{1}, {2}}
	for _, p := range s {
		s[1].X = 9
		println(p.X)
	}
}
`)
	if !strings.Contains(rust, "let __range_values = __range_guard.as_ref().cloned().unwrap_or_default();") {
		t.Errorf("the slice must not be copied before the loop:\n%s", rust)
	}
	if !strings.Contains(rust, "let p = &p.__go_value_clone();") {
		t.Errorf("the loop variable should be Go's copy of its element, taken in the loop:\n%s", rust)
	}
}

// With no type for a value, a copy site must say so instead of choosing
// Clone, which for a struct would silently share its fields.
func TestGoCopyWithoutTypeInfoIsLoud(t *testing.T) {
	previous := GetTypeInfo()
	SetTypeInfo(nil)
	defer SetTypeInfo(previous)

	if call, known := goValueCopyCallForExpr(ast.NewIdent("a")); known || call != "" {
		t.Fatalf("goValueCopyCallForExpr without type info = %q, %v; want no call and not known", call, known)
	}
	if call, known := elementGoValueCopyCall(ast.NewIdent("s")); known || call != "" {
		t.Fatalf("elementGoValueCopyCall without type info = %q, %v; want no call and not known", call, known)
	}

	var out strings.Builder
	writeIdentValueClone(&out, ast.NewIdent("a"))
	if out.String() != goValueCopyNeedsTypeInfo {
		t.Fatalf("a copy without type info should be %q, got %q", goValueCopyNeedsTypeInfo, out.String())
	}
}

const structStoreProgram = `package main

type Point struct{ X, Y int }

type Shape interface{ Area() int }

type Rect struct{ W, H int }

func (r Rect) Area() int { return r.W * r.H }

func main() {
	p := Point{1, 1}
	s := []Point{{0, 0}}
	s[0] = p
	m := map[string]Point{}
	m["a"] = p
	got := m["a"]
	r := Rect{2, 3}
	var shape Shape = r
	q := &p
	c := *q
	var d Point
	d = *q
	var e Point
	e = s[0]
	spread := append([]Point{}, s...)
	copied := make([]Point, len(s))
	copy(copied, s)
	names := []string{"a"}
	moreNames := append([]string{}, names...)
	copy(moreNames, names)
	_, _, _, _, _, _, _ = got, shape, c, d, e, spread, moreNames
}
`

// Storing a struct in a slice, a map or an interface copies it, and so does
// reading one back out of a map, a slice or a pointer. "s[0] = p" did not
// compile before this (it moved p out of its cell), and the others shared the
// struct between both sides.
func TestStoringAndReadingAStructCopiesIt(t *testing.T) {
	rust := transpileTypedRegression(t, structStoreProgram)

	for what, want := range map[string]string{
		"s[0] = p":               "let __rhs = (*p.borrow().as_ref().unwrap()).__go_value_clone(); (*s.borrow_mut().as_mut().unwrap())[__idx] = __rhs;",
		`m["a"] = p`:             "let __map_value = { let __owned = p.borrow().as_ref().unwrap().__go_value_clone();",
		`got := m["a"]`:          ".map(|__v| __v.borrow().as_ref().unwrap().__go_value_clone())",
		"var shape Shape = r":    "(*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Shape>",
		"c := *q":                "let mut c = Rc::new(RefCell::new(Some({ let __v = (*q.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));",
		"d = *q":                 "{ let new_val = { let __v = (*q.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone(); *d.borrow_mut() = Some(new_val); }",
		"e = s[0]":               "{ let new_val = (*s.borrow().as_ref().unwrap())[(0) as usize].clone().__go_value_clone(); *e.borrow_mut() = Some(new_val); }",
		"append(dst, s...)":      ".iter().map(|__go_elem| __go_elem.__go_value_clone()).collect();",
		"copy(copied, s)":        "(*copied.borrow_mut().as_mut().unwrap())[_i] = _src[_i].__go_value_clone();",
		"append(dst, names...)":  ".unwrap_or_default() }.iter().cloned().collect();",
		"copy(moreNames, names)": "(*moreNames.borrow_mut().as_mut().unwrap())[_i] = _src[_i].clone();",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s: missing %q", what, want)
		}
	}
	if t.Failed() {
		t.Logf("generated Rust:\n%s", rust)
	}
}

// Without the type of the source slice there is no telling whether its
// elements are structs, so the generated code stops instead of sharing them.
func TestGoCopyOfElementsWithoutTypeInfoIsLoud(t *testing.T) {
	previous := GetTypeInfo()
	SetTypeInfo(nil)
	defer SetTypeInfo(previous)

	if got := goElementCopyIter(ast.NewIdent("s")); !strings.Contains(got, goValueCopyNeedsTypeInfo) {
		t.Errorf("goElementCopyIter without type info = %q; want the type-info stop", got)
	}
	if got := copySourceElementCopy(ast.NewIdent("s"), false); got != "" {
		t.Errorf("copySourceElementCopy without type info = %q; want no call", got)
	}
	if got := copySourceElementCopy(ast.NewIdent("s"), true); got != ".clone()" {
		t.Errorf("copySourceElementCopy of a string = %q; want .clone()", got)
	}
	var loop strings.Builder
	writeCopyLoop(&loop, "", "", func() { loop.WriteString("_dst[_i]") })
	if !strings.Contains(loop.String(), "_dst[_i] = "+goValueCopyNeedsTypeInfo) {
		t.Errorf("copy loop without an element copy = %q; want the type-info stop", loop.String())
	}
}

const structRangeAndMapReadProgram = `package main

type Point struct{ X int }

type Registry struct{ byName map[string]Point }

func main() {
	s := []Point{{1}}
	var p Point
	for _, p = range s {
		p.X = 2
	}
	m := map[string]Point{"a": {1}}
	for _, v := range m {
		v.X = 3
	}
	var w Point
	for _, w = range m {
		w.X = 4
	}
	q, ok := m["a"]
	var r Point
	r, ok = m["a"]
	counts := map[string]int{"a": 1}
	n, ok := counts["a"]
	lists := map[string][]int{"a": {1}}
	var l []int
	l, ok = lists["a"]
	ptrs := map[string]*Point{}
	pp, ok := ptrs["a"]
	reg := Registry{byName: m}
	f, ok := reg.byName["a"]
	_, _, _, _, _, _, _, _ = q, r, n, l, pp, ok, p, f
}
`

// A range loop that assigns to a variable declared before it, a range over a
// map, and the two-result map read all hand over a copy of the value. Each of
// them handed over the container's own storage before: "for _, p = range s"
// left p sharing the last element, "v, ok := m[k]; v = 5" changed the map for
// any value type, and "v, ok = m[k]" moved the value out of the map.
func TestRangeAndMapReadsCopyTheValue(t *testing.T) {
	rust := transpileTypedRegression(t, structRangeAndMapReadProgram)

	for what, want := range map[string]string{
		"for _, p = range s":         "{ let new_val = (*__range_p).__go_value_clone(); *p.borrow_mut() = Some(new_val); }",
		"for _, v := range m":        "let v = Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone())));",
		"for _, w = range m":         "{ let new_val = __range_w.borrow().as_ref().unwrap().__go_value_clone(); *w.borrow_mut() = Some(new_val); }",
		`q, ok := m["a"]`:            "let (mut q, mut ok) = { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&\"a\".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true)",
		`n, ok := counts["a"]`:       "Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().clone()))), true), None => (Rc::new(RefCell::new(Some(0))), false)",
		`l, ok = lists["a"]`:         "Some(v) => (v.clone(), true), None => (Default::default(), false) } }; l = __tmp_0;",
		`pp, ok := ptrs["a"]`:        "let (mut pp, mut ok) = { let __map_holder = ptrs.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&\"a\".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true)",
		`f, ok := reg.byName["a"]`:   ".clone().get(&\"a\".to_string()) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true)",
		"the map's cell is not kept": "*r.borrow_mut() = __moved_tmp_0;",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s: missing %q", what, want)
		}
	}
	if strings.Contains(rust, ".get(&\"a\".to_string())) { /* MAP_COMMA_OK */ Some(v) => (Rc::new(RefCell::new(Some(v.borrow().as_ref().unwrap().__go_value_clone()))), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) };\n    {") {
		t.Errorf("the two-result read of a map held by value closes one parenthesis too many")
	}
	if t.Failed() {
		t.Logf("generated Rust:\n%s", rust)
	}
}

// An element sent on a channel is a copy of the element.
func TestSendingASliceElementCopiesIt(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Point struct{ X int }

func main() {
	s := []Point{{1}}
	ch := make(chan Point, 1)
	ch <- s[0]
}
`)
	if want := "ch.send((*s.borrow().as_ref().unwrap())[(0) as usize].clone().__go_value_clone());"; !strings.Contains(rust, want) {
		t.Fatalf("ch <- s[0] should send Go's copy of the element, missing %q:\n%s", want, rust)
	}
}

// With no type for the map's value, the two-result read stops instead of
// handing out the map's cell.
func TestMapCommaOkWithoutTypeInfoIsLoud(t *testing.T) {
	previous := GetTypeInfo()
	SetTypeInfo(nil)
	defer SetTypeInfo(previous)

	var out strings.Builder
	writeMapCommaOkFoundValue(&out, &ast.IndexExpr{X: ast.NewIdent("m"), Index: ast.NewIdent("k")}, false, false, false)
	if out.String() != goValueCopyNeedsTypeInfo {
		t.Fatalf("found value without type info = %q; want %q", out.String(), goValueCopyNeedsTypeInfo)
	}

	out.Reset()
	writeMapRangeValueIterationCopy(&out, nil, ast.NewIdent("v"))
	if !strings.Contains(out.String(), "let v = ") || !strings.Contains(out.String(), goValueCopyNeedsTypeInfo) {
		t.Fatalf("map range value without a type = %q; want the type-info stop", out.String())
	}
}

// A struct with type parameters gets the same Clone as every other struct:
// the handles of its field cells, so that a pointer method can hand on its
// receiver. Its Clone used to be Go's copy, which detached "c" in a closure
// such as "func() { c.N++ }" from the struct the method was called on.
func TestGenericStructCloneKeepsTheFieldCells(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type P struct{ X int }

type C[T any] struct {
	P P
	V T
}

func (c *C[T]) Later() func() { return func() { c.P.X++ } }

func main() {
	c := &C[int]{}
	c.Later()()
}
`)
	cloneStart := strings.Index(rust, "> Clone for C<T> {")
	if cloneStart < 0 {
		t.Fatalf("missing the Clone impl of the generic struct:\n%s", rust)
	}
	cloneImpl := rust[cloneStart:]
	if end := strings.Index(cloneImpl, "\n}\n"); end >= 0 {
		cloneImpl = cloneImpl[:end]
	}
	for _, want := range []string{"p: self.p.clone(),", "v: self.v.clone(),"} {
		if !strings.Contains(cloneImpl, want) {
			t.Errorf("Clone of a generic struct should clone each field cell, missing %q:\n%s", want, cloneImpl)
		}
	}
	if strings.Contains(cloneImpl, "__go_value_clone") {
		t.Errorf("Clone of a generic struct should not be Go's copy:\n%s", cloneImpl)
	}
	if !strings.Contains(rust, "pub fn __go_value_clone(&self) -> Self {") {
		t.Errorf("the generic struct should still have Go's copy as its own method:\n%s", rust)
	}
}

func TestGenericStructGoValueCloneCopiesDirectTypeParamFieldByGoSemantics(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type P struct{ X int }

type C[T any] struct { V T }

func main() {
	c := &C[P]{V: P{X: 1}}
	v := *c
	v.V.X = 2
}
`)

	for _, want := range []string{
		"impl<T: Any + GoTypeParamFieldClone + 'static> C<T> {\n    pub fn __go_value_clone(&self) -> Self {",
		"<T as GoTypeParamFieldClone>::go_type_param_field_clone(&self.v)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("generic struct field copy should use instantiated Go copy semantics, missing %q:\n%s", want, rust)
		}
	}
}

func TestGenericStructValueReceiverCarriesImplicitCopyBound(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type C[T any] struct { V T; N int }

func (c C[T]) Bump() { c.N++ }
`)

	if !strings.Contains(rust, "impl<T: Any + GoTypeParamFieldClone + 'static> C<T> {\n    pub fn bump(&self)") {
		t.Fatalf("generic value receiver should carry the bound required by its implicit receiver copy:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut __self = self.__go_value_clone();") {
		t.Fatalf("generic value receiver should still make its Go value copy:\n%s", rust)
	}
}

func TestGenericStructFunctionTypeArgumentUsesCloneableFieldProxy(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type C[T any] struct { V T }

func main() {
	original := &C[func() int]{V: func() int { return 1 }}
	copy := *original
	_ = copy.V()
}
`)

	for _, want := range []string{
		"impl<R: 'static> GoTypeParamFieldClone for Box<dyn FnMut() -> R>",
		"let original = field.borrow_mut().take();",
		"<T as GoTypeParamFieldClone>::go_type_param_field_clone(&self.v)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("function-valued type arguments should copy through an independent forwarding field, missing %q:\n%s", want, rust)
		}
	}
}

// "v, ok = m[k]" for a slice or map value must leave the value in the map
// whatever the target is. A plain local slice takes over the entry's handle.
// A named slice, a field of one, a package variable and an indexed target
// are filled by moving a value into them, so they get a clone of the entry's
// value in a cell of its own to move it from; they used to be given the
// entry's own cell, and the move emptied the map entry.
func TestMapCommaOkAssignmentLeavesSliceValuesInTheMap(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Numbers []int

type Holder struct{ nums Numbers }

var global []int

func main() {
	named := map[string]Numbers{}
	lists := map[string][]int{}
	var numbers Numbers
	var ok bool
	numbers, ok = named["a"]
	var h Holder
	h.nums, ok = named["a"]
	global, ok = lists["a"]
	dst := [][]int{{9}}
	dst[0], ok = lists["a"]
	var local []int
	local, ok = lists["a"]
	_, _, _ = numbers, ok, local
}
`)
	const held = "Some(v) => ({ let __held = v.borrow().clone(); Rc::new(RefCell::new(__held)) }, true), None => (Default::default(), false) } };"
	const namedHeld = "Some(v) => ({ let __held = v.borrow().clone(); Rc::new(RefCell::new(__held)) }, true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };"
	for what, want := range map[string]string{
		"a named slice variable": namedHeld + " let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *numbers.borrow_mut() = __moved_tmp_0;",
		"a named slice field":    namedHeld + " let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *(*h.borrow().as_ref().unwrap()).nums.borrow_mut() = __moved_tmp_0;",
		"a package variable":     held + " let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *global.borrow_mut() = __moved_tmp_0;",
		"an indexed target":      held + " { let __idx = (0) as usize; (*dst.borrow_mut().as_mut().unwrap())[__idx] = __tmp_0.borrow_mut().take().unwrap_or_default(); }",
		"a plain local slice":    "Some(v) => (v.clone(), true), None => (Default::default(), false) } }; local = __tmp_0;",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s: missing %q", what, want)
		}
	}
	if t.Failed() {
		t.Logf("generated Rust:\n%s", rust)
	}
}

// A named array type is a newtype around a cell, and its derived Clone is
// the same array again. It needs its own copy helper, and the copy sites
// must call it: "b := a; b[0] = 9" changed a for every named array type,
// arrays of ints included.
func TestNamedArrayIsCopiedWithItsOwnHelper(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type P struct{ X int }

type Cells [1]P

type Nums [2]int

type Grid [2]Cells

func use(c Cells) {}

func main() {
	var a Cells
	b := a
	var n Nums
	m := n
	var g Grid
	h := g
	use(a)
	_, _, _ = b, m, h
}
`)
	for what, want := range map[string]string{
		"the helper of an array of structs":      "impl Cells {\n    pub fn __go_value_clone(&self) -> Self {\n        Cells(Rc::new(RefCell::new(self.0.borrow().as_ref().map(|__go_array| __go_array.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))))",
		"the helper of an array of ints":         "impl Nums {\n    pub fn __go_value_clone(&self) -> Self {\n        Nums(Rc::new(RefCell::new(self.0.borrow().as_ref().map(|__go_array| __go_array.clone()))))",
		"the helper of an array of named arrays": "impl Grid {\n    pub fn __go_value_clone(&self) -> Self {\n        Grid(Rc::new(RefCell::new(self.0.borrow().as_ref().map(|__go_array| __go_array.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))))",
		"b := a":                                 "let mut b = { let __owned = a.borrow().as_ref().unwrap().__go_value_clone();",
		"m := n":                                 "let mut m = { let __owned = n.borrow().as_ref().unwrap().__go_value_clone();",
		"h := g":                                 "let mut h = { let __owned = g.borrow().as_ref().unwrap().__go_value_clone();",
		"use(a)":                                 "(*__arg_guard.as_ref().unwrap()).__go_value_clone() }",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s: missing %q", what, want)
		}
	}
	if t.Failed() {
		t.Logf("generated Rust:\n%s", rust)
	}
}

// A map or slice taken out of an interface{} shares its data with the one
// that was put in. A new variable takes the handle the interface value holds,
// in a short declaration and in a var declaration, and so does a use that
// needs a handle, such as len(x.(map[string]int)) or printing x.([]int).
func TestAssertedMapOrSliceSharesTheInterfaceValuesHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	var counts any = map[string]int{"a": 1}
	m := counts.(map[string]int)
	m["b"] = 2
	var c = counts.(map[string]int)
	c["x"] = 7
	fmt.Println(len(counts.(map[string]int)))
	var list any = []int{1}
	fmt.Println(list.([]int))
}
`)

	const mapHandle = "Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>"
	shared := "({ let val = counts.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<" + mapHandle + ">().expect(\"type assertion failed\").clone() } else { panic!(\"type assertion on nil interface\") } })"
	for what, want := range map[string]string{
		"a short declaration": "let mut m = " + shared + ";",
		"a var declaration":   "let mut c = " + shared + ";",
		"len of an assertion": shared,
		"printing a slice":    "format_slice(&({ let val = list.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>().expect(\"type assertion failed\").clone() }",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s: missing %q", what, want)
		}
	}
	if t.Failed() {
		t.Logf("generated Rust:\n%s", rust)
	}
}

// "var n Numbers = x.([]int)" declares a Numbers, not the asserted []int:
// the initializer must build the Numbers value, not hand over the []int
// handle the interface holds.
func TestAssertedSliceIntoVarOfAnotherSliceTypeBuildsThatType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Numbers []int

func main() {
	var x any = []int{1}
	var n Numbers = x.([]int)
	var s []int = x.([]int)
	_, _ = n, s
}
`)
	if !strings.Contains(rust, "let mut n: Rc<RefCell<Option<Numbers>>> = Rc::new(RefCell::new(Some(Numbers(") {
		t.Fatalf("var n Numbers should be built as a Numbers value:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut s: Rc<RefCell<Option<Vec<i32>>>> = ({ let val = x.clone();") {
		t.Fatalf("var s []int should keep the interface's slice handle:\n%s", rust)
	}
}

// "any(a.([]int))" puts the handle the assertion reads into the new box, so
// both interface values hold the same slice, as in Go. Wrapping that handle
// in a new cell gave a box the next assertion did not recognize.
func TestReboxedAssertedSliceSharesTheHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func main() {
	a := any([]int{1, 2})
	b := any(a.([]int))
	_ = b
}
`)
	if !strings.Contains(rust, "Box::new(({ let val = a.clone();") {
		t.Fatalf("the new box should hold the asserted handle itself:\n%s", rust)
	}
	if strings.Contains(rust, "Box::new(Rc::new(RefCell::new(Some(({ let val = a.clone();") {
		t.Fatalf("the asserted handle must not be wrapped in a new cell:\n%s", rust)
	}
}

// A function value is boxed into an interface{} as its handle, so every form
// of assertion that keeps it takes that handle: "g := x.(func())", the
// comma-ok form, "g = x.(func())" and an element of a []any. They used to
// downcast to the closure box itself, which is not what the box holds, and
// clone a reference to it.
func TestFunctionValueTakenOutOfInterfaceIsItsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func main() {
	var x any = func() {}
	g := x.(func())
	h, ok := x.(func())
	var k func()
	k = x.(func())
	items := []any{func() {}}
	e := items[0].(func())
	f, ok2 := items[0].(func())
	_, _, _, _, _, _, _ = g, h, ok, k, e, f, ok2
}
`)
	handle := "Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>"
	for _, want := range []string{
		"let mut g = ({ let val = x.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<" + handle + ">()",
		"if let Some(typed_val) = any_val.downcast_ref::<" + handle + ">() {\n                (typed_val.clone(), true)",
		"{ let new_val = ({ let val = x.clone();",
		"any_val.downcast_ref::<" + handle + ">().expect(\"type assertion failed\").clone()",
		"match __seq_guard.as_ref().expect(\"nil []any in type assertion\")[__idx].downcast_ref::<" + handle + ">() {",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("expected %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "downcast_ref::<Box<dyn FnMut() -> ()>>()") {
		t.Fatalf("no assertion should downcast to the closure box:\n%s", rust)
	}
}

// The value of a range over a []interface{} is a &Box<dyn Any>. Put into a
// new interface value, it is copied out of the box with go_any_clone, and
// printed, it is the box's payload. Cloning the reference did not compile,
// and passing the reference itself to go_any_clone would copy the Box, not
// what it holds.
func TestRangeValueOverAnySliceIsCopiedOutOfItsBox(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func show(v interface{}) { fmt.Println(v) }

func main() {
	values := []interface{}{"a", 1}
	for _, val := range values {
		show(val)
		var copy any = val
		fmt.Println(copy, val)
	}
}
`)
	for _, want := range []string{
		"show(Rc::new(RefCell::new(Some(go_any_clone(val.as_ref())))));",
		"Rc::new(RefCell::new(Some(go_any_clone(val.as_ref()))));",
		"format_any(val.as_ref())",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("expected %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "show(val.clone())") {
		t.Fatalf("the range value must not be passed as a cloned reference:\n%s", rust)
	}
}
