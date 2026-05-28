package main

import "fmt"

// GAP: a[i] = s[lo:hi:cap] where a is [][]byte. The element slot is a raw
// Vec<u8>, but the slice-expression RHS is emitted wrapped
// (Rc/Arc<...<Vec<u8>>>), so a wrapped value lands in a raw slot (E0308).
// Root cause: go/stmt.go writeArraySliceElementAssignmentValue only unwraps
// *ast.CallExpr RHS, not *ast.SliceExpr.
func main() {
	s := []byte("hello world")
	a := make([][]byte, 2)
	a[0] = s[0:5:5]
	a[1] = s[6:11:11]
	fmt.Println(string(a[0]), string(a[1]))
}
