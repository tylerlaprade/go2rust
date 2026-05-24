package main

import "fmt"

// Architectural blocker (2026-05-24): function-type aliases cannot impl
// local interface traits because the trait carries `std::fmt::Display`
// as a supertrait. `counter` is `Rc<RefCell<Option<Box<dyn FnMut()...>>>>`
// — entirely foreign types — so `impl Display for counter` violates the
// orphan rule. Resolving requires one of:
//   (a) emit function-type definitions as newtype structs (Deref shim
//       preserves existing call sites)
//   (b) drop the Display supertrait from interfaces and route
//       fmt.Println(iface_val) through a separate downcast helper
//
// Mirrors go/ast/walk.go's inspector pattern: a function-type alias
// declares a method that satisfies an interface, then values of the alias
// are passed where the interface is expected.

type Speaker interface {
	Speak() int
}

type counter func() int

func (c counter) Speak() int {
	return c()
}

func runSpeaker(s Speaker) int {
	return s.Speak()
}

func makeCounter() counter {
	x := 41
	return func() int { x++; return x }
}

func main() {
	c := makeCounter()
	fmt.Println(runSpeaker(c))
}
