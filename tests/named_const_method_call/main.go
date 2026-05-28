package main

import "fmt"

// GAP: calling a method on a named-integer typed constant. The const lowers
// to a raw scalar (pub const Add: i32 = 12), but the method-call receiver was
// treated as a wrapped handle -> .borrow()/.lock() on i32 (E0599). Mirrors
// token.ADD.String() on a source-transpiled go/token.
type Tok int

const Add Tok = 12

func (t Tok) Name() string {
	if t == Add {
		return "add"
	}
	return "other"
}

func main() {
	fmt.Println(Add.Name())
}
