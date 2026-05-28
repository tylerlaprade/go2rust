package main

import "fmt"

// Reproduces increment through a dereferenced pointer (`*p++`), as in
// sort's `*swaps++`. The pointer operand is itself the wrapper handle, so
// the mutation must lock the pointer once — not dereference to the bare
// scalar place and then re-lock it (which calls .lock()/.borrow() on a
// scalar: E0599).
func bump(p *int) {
	*p++
	*p += 2
}

func main() {
	x := 10
	bump(&x)
	fmt.Println(x)
}
