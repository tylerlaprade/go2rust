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
		`l, ok = lists["a"]`:         "Some(v) => ({ let __held = v.borrow().clone(); Rc::new(RefCell::new(__held)) }, true), None => (Default::default(), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *l.borrow_mut() = __moved_tmp_0;",
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

func TestMapCommaOkAssignmentKeepsHandleValuesInTheMap(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type message string
func (m message) Error() string { return string(m) }

func main() {
	functions := map[string]func(string) string{"k": func(prefix string) string { return prefix + "function" }}
	function := func(prefix string) string { return prefix + "old" }
	functionPointer := &function
	var ok bool
	function, ok = functions["k"]

	interfaces := map[string]any{"k": "interface"}
	var iface any = "old"
	interfacePointer := &iface
	iface, ok = interfaces["k"]

	errorsByName := map[string]error{"k": message("error")}
	var err error = message("old")
	errorPointer := &err
	err, ok = errorsByName["k"]

	channels := map[string]chan int{"k": make(chan int, 1)}
	channel := make(chan int)
	channel, ok = channels["k"]
	still := channels["k"]
	_, _, _, _, _, _, _, _, _ = function, iface, err, channel, still, ok, functionPointer, interfacePointer, errorPointer
}
`)

	for what, want := range map[string]string{
		"function value copy":   "let __shared = Rc::new(RefCell::new(Some(__original))); let __map_callable = __shared.clone(); *v.borrow_mut() = Some(Box::new(move |__arg0|",
		"function target cell":  "let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *function.borrow_mut() = __moved_tmp_0;",
		"interface value copy":  "let __held = v.borrow().as_ref().map(|__value| go_any_clone(__value.as_ref())); Rc::new(RefCell::new(__held))",
		"interface target cell": "let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *iface.borrow_mut() = __moved_tmp_0;",
		"error value copy":      "let __held = v.borrow().clone(); Rc::new(RefCell::new(__held))",
		"error target cell":     "let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_0;",
		"channel value":         "Some(v) => (v.borrow().as_ref().unwrap().clone(), true)",
		"channel map read":      ".map(|__v| { let __guard = __v.borrow(); __guard.as_ref().cloned().unwrap_or_default() }).unwrap_or_else(|| Default::default())",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s: missing %q in:\n%s", what, want, rust)
		}
	}
	for _, rebound := range []string{
		"function = __tmp_0",
		"iface = __tmp_0",
		"err = __tmp_0",
	} {
		if strings.Contains(rust, rebound) {
			t.Errorf("assignment replaced an addressable variable handle through %q in:\n%s", rebound, rust)
		}
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

// "v, ok = m[k]" for a slice or map value must leave the value in the map
// whatever the target is. Every target, including a plain local, gets a fresh
// variable cell: rebinding the local's handle would leave an existing &local
// pointer observing the old cell.
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
		"a plain local slice":    held + " let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *local.borrow_mut() = __moved_tmp_0;",
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
