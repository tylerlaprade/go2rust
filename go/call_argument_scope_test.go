package main

import (
	"strings"
	"testing"
)

func TestCallArgumentThatReadsTheCalleesSliceIsReadFirst(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func fill(s []int, n int) {
	for i := range s {
		s[i] = n
	}
}

func main() {
	s := []int{1, 2, 3}
	fill(s, s[2]+1)
	inc := func(v int) { s[0] += v }
	inc(len(s))
	fill(s, 4)
}
`)
	for _, want := range []string{
		"fill(s.clone(), Rc::new(RefCell::new(Some({ let __arg_value = (*s.borrow().as_ref().unwrap())[(2) as usize].clone() + 1; __arg_value }))));",
		"fill(s.clone(), Rc::new(RefCell::new(Some(4))));",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}

func TestScopeCallArgumentGuards(t *testing.T) {
	var prefix strings.Builder
	WriteWrapperPrefix(&prefix)
	for _, tc := range []struct{ in, want string }{
		// A value that borrows a cell is read in a block.
		{prefix.String() + "(*s.borrow().as_ref().unwrap())[0].clone()" + ")))",
			prefix.String() + "{ let __arg_value = (*s.borrow().as_ref().unwrap())[0].clone(); __arg_value }" + ")))"},
		// A value that borrows nothing, a block, and a handle are left alone.
		{prefix.String() + "4" + ")))", prefix.String() + "4" + ")))"},
		{prefix.String() + "{ let __v = (*s.borrow()).len(); __v }" + ")))", prefix.String() + "{ let __v = (*s.borrow()).len(); __v }" + ")))"},
		{"s.clone()", "s.clone()"},
		// The trailing ))) does not close the cell: the text is not one cell.
		{prefix.String() + "a))).f(g(h(s.borrow()" + ")))", prefix.String() + "a))).f(g(h(s.borrow()" + ")))"},
		// Parentheses in a string literal do not count.
		{prefix.String() + `format!("(", x.borrow())` + ")))", prefix.String() + `{ let __arg_value = format!("(", x.borrow()); __arg_value }` + ")))"},
	} {
		if got := scopeCallArgumentGuards(tc.in); got != tc.want {
			t.Errorf("scopeCallArgumentGuards(%q)\n got %q\nwant %q", tc.in, got, tc.want)
		}
	}
}

func TestCallArgumentStartingWithABlockIsStillScoped(t *testing.T) {
	var prefix strings.Builder
	WriteWrapperPrefix(&prefix)
	in := prefix.String() + "{ let __v = 1; __v } + (*s.borrow().as_ref().unwrap())[0]" + ")))"
	want := prefix.String() + "{ let __arg_value = { let __v = 1; __v } + (*s.borrow().as_ref().unwrap())[0]; __arg_value }" + ")))"
	if got := scopeCallArgumentGuards(in); got != want {
		t.Errorf("a value that only starts with a block should be scoped:\n got %q\nwant %q", got, want)
	}
	for text, want := range map[string]bool{
		"{ a }":         true,
		"{ a } + { b }": false,
		"{ \"}\" }":     true,
		"{ '}' }":       true,
		"x":             false,
		"{ { a } }":     true,
		"{ a }.clone()": false,
	} {
		if got := rustTextIsOneBlock(text); got != want {
			t.Errorf("rustTextIsOneBlock(%q) = %v, want %v", text, got, want)
		}
	}
}

func TestPointerMethodArgumentsAreReadBeforeTheReceiverWithGoroutines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Grid struct{ Cells []int }

func (g *Grid) Put(i, v int) { g.Cells[i] = v }

func main() {
	g := &Grid{Cells: []int{5, 6}}
	g.Put(0, g.Cells[1]+1)
	done := make(chan bool)
	go func() { done <- true }()
	<-done
}
`)
	at := strings.Index(rust, "let __method_args = (")
	recv := strings.Index(rust, "let __recv_ptr")
	if at < 0 || recv < 0 || at > recv {
		t.Fatalf("the arguments should be read before the receiver's pointer is made:\n%s", rust)
	}
	if !strings.Contains(rust, ".put(__method_args.0, __method_args.1)") {
		t.Fatalf("the call should take the arguments read before it:\n%s", rust)
	}
}

func TestPointerMethodArgumentsAreReadBeforeTheReceiverOfATypeNamedOutlet(t *testing.T) {
	// The let in "*mut Outlet =" is the end of a name, not a binding.
	rust := transpileTypedConcurrentRegression(t, `package main

type Outlet struct{ Cells []int }

func (o *Outlet) Put(i, v int) { o.Cells[i] = v }

func main() {
	o := &Outlet{Cells: []int{5, 6}}
	o.Put(0, o.Cells[1]+1)
	done := make(chan bool)
	go func() { done <- true }()
	<-done
}
`)
	at := strings.Index(rust, "let __method_args = (")
	recv := strings.Index(rust, "let __recv_ptr")
	if at < 0 || recv < 0 || at > recv {
		t.Fatalf("the arguments should be read before the receiver's pointer is made:\n%s", rust)
	}
	if !rustPreludeBindsOnlyReceiver("{ let __recv = o.clone(); let __recv_ptr: *mut Outlet = { 1 }; let __result = ") {
		t.Error("the let at the end of Outlet was taken for a binding")
	}
	if rustPreludeBindsOnlyReceiver("{ let __recv = o.clone(); let x = 1; ") {
		t.Error("a binding of another name was not seen")
	}
}

func TestVariadicArgumentsAreReadBeforeTheCall(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func fill(s []int, ns ...int) {
	for i := range s {
		s[i] = ns[0]
	}
}

func main() {
	s := []int{1, 2, 3}
	fill(s, s[0]+1, s[1])
}
`)
	if !strings.Contains(rust, "{ let __arg_value = (*s.borrow().as_ref().unwrap())[(0) as usize].clone() + 1; __arg_value }") {
		t.Errorf("a packed variadic argument that reads s should be read in a block of its own:\n%s", rust)
	}
}

func TestBlockArgumentWhoseTailBorrowsIsScoped(t *testing.T) {
	for _, tc := range []struct{ in, want string }{
		// The tail's Ref lives to the end of the call's statement.
		{"{ let a = 1; (*x.borrow()).clone() + a }", "{ let __arg_value = { let a = 1; (*x.borrow()).clone() + a }; __arg_value }"},
		// A block whose guards end in its statements is left alone.
		{"{ let g = x.borrow(); let v = g.clone(); drop(g); v }", "{ let g = x.borrow(); let v = g.clone(); drop(g); v }"},
		// A ; inside a nested block is not the outer block's.
		{"{ f({ let a = 1; a }) }", "{ f({ let a = 1; a }) }"},
	} {
		var out strings.Builder
		writeCallArgumentValueScoped(&out, tc.in)
		if out.String() != tc.want {
			t.Errorf("writeCallArgumentValueScoped(%q)\n got %q\nwant %q", tc.in, out.String(), tc.want)
		}
	}
	if got := rustBlockTail("{ let a = 1; { b }; c.borrow() }"); got != " c.borrow() " {
		t.Errorf("rustBlockTail = %q", got)
	}
}
