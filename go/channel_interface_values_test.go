package main

import (
	"strings"
	"testing"
)

func requireAll(t *testing.T, rust string, wants ...string) {
	t.Helper()
	for _, want := range wants {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}

func TestInterfaceChannelCarriesTheHandleOption(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Shape interface{ Area() int }

type Rect struct{ W, H int }

func (r Rect) Area() int { return r.W * r.H }

func main() {
	ch := make(chan Shape, 4)
	var cur Shape = Rect{1, 2}
	ch <- Rect{2, 3}
	ch <- nil
	ch <- cur
	first := <-ch
	var second Shape
	second = <-ch
	third, ok := <-ch
	close(ch)
	for shape := range ch {
		_ = shape.Area()
	}
	_, _, _, _ = first, second, third, ok
}
`)
	requireAll(t, rust,
		// nil can be sent, and a closed channel gives nil.
		"GoChannel::<Option<Box<dyn Shape>>>::new_buffered(",
		"ch.send(None::<Box<dyn Shape>>);",
		// What is sent is the value the variable holds, out of its handle.
		"let __iface_value = (*__iface_handle.borrow()).clone(); __iface_value });",
		// The receiver gets a handle of its own.
		"let mut first = Rc::new(RefCell::new(ch.recv().unwrap_or_default()));",
		"{ let new_val = ch.recv().unwrap_or_default(); *second.borrow_mut() = new_val; }",
		"None => (Rc::new(RefCell::new(None::<Box<dyn Shape>>)),",
		"for __iface_value in ch.clone() {",
		"let mut shape = Rc::new(RefCell::new(__iface_value));",
	)
}

func TestChannelInAFieldIsReadOutBeforeItIsUsed(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Pool struct{ results chan int }

func main() {
	p := &Pool{results: make(chan int)}
	go func() { p.results <- 1 }()
	_ = <-p.results
	lanes := []chan int{make(chan int)}
	go func() { lanes[0] <- 2 }()
	_ = <-lanes[0]
}
`)
	// The send and the receive may block while the other goroutine locks p
	// to reach the channel, so p's lock ends first.
	requireAll(t, rust,
		".results.clone(); __channel }.send(",
		".results.clone(); __channel }.recv()",
		".get_cloned((0) as usize).clone(); __channel }.recv()",
	)
	if strings.Contains(rust, "unwrap()).results.send(") || strings.Contains(rust, "unwrap()).results.recv(") || strings.Contains(rust, "unwrap())[(0) as usize].send(") {
		t.Errorf("a channel field is used while its struct is locked:\n%s", rust)
	}
}

func TestAppendingAChannelKeepsTheVariable(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func main() {
	ch := make(chan int, 1)
	var chans []chan int
	chans = append(chans, ch)
	ch <- 1
	_ = chans
}
`)
	requireAll(t, rust, "let __append_value = ch.clone();")
}

func TestAnySliceElementIsReadIntoAHandleOfItsOwn(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	mixed := []any{1, "two"}
	v := mixed[0]
	fmt.Println(v, mixed[1])
	fmt.Printf("%T\n", mixed[1])
}
`)
	// A Box<dyn Any> has no Clone; go_any_clone copies it.
	requireAll(t, rust,
		"let __seq_guard = __seq_holder.borrow(); let __slice_guard = __seq_guard.as_ref().unwrap().as_slice(); go_any_clone(__slice_guard[__idx].as_ref()) }",
		"__go_type_name(&**(Rc::new(RefCell::new(Some({ let __idx = (1) as usize;",
	)
	if strings.Contains(rust, "[(0) as usize].clone()") {
		t.Errorf("an []any element is cloned as a Box:\n%s", rust)
	}
}
