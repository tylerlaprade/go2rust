package main

import "fmt"

type C[T any] struct {
	P     P
	Items [2]P
	V     T
	N     int
}

type Outer[T any] struct{ Box C[T] }

func CopyC[T any](value C[T]) C[T] { return value }

func (c *C[T]) LaterNested() func() { return func() { c.P.X++ } }

func (c *C[T]) LaterDirect() func() { return func() { c.N++ } }

func (c *C[T]) LaterItems() func() { return func() { c.Items[1].X += 5 } }

func (c *C[T]) Self() *C[T] { return c }

// A pointer method of a struct with type parameters can hand on its
// receiver: a closure over it and a returned receiver are the same struct.
func main() {
	c := &C[int]{}
	f := c.LaterNested()
	f()
	f()
	fmt.Println("nested field through a closure", c.P.X)

	g := c.LaterDirect()
	g()
	fmt.Println("direct field through a closure", c.N)

	k := c.LaterItems()
	k()
	fmt.Println("array of structs through a closure", c.Items[1].X)

	d := c.Self()
	d.N = 50
	d.P.X = 60
	fmt.Println("returned receiver", c.N, c.P.X)

	v := *c
	v.N = 7
	v.P.X = 8
	v.Items[1].X = 9
	fmt.Println("copy", c.N, c.P.X, c.Items[1].X, v.N, v.P.X, v.Items[1].X)

	// The instantiated type parameter is a field value too. Copying the
	// generic struct must copy a concrete struct value rather than sharing its
	// generated field cells.
	originalValuePtr := &C[P]{V: P{X: 11}}
	originalValue := *originalValuePtr
	copiedValue := originalValue
	copiedValue.V.X = 12
	fmt.Println("type parameter struct copy", originalValue.V.X, copiedValue.V.X)

	// A slice is itself a handle in Go: the generic field cell is independent,
	// while the slice's backing storage remains shared by the copied header.
	originalSlicePtr := &C[[]int]{V: []int{13}}
	originalSlice := *originalSlicePtr
	copiedSlice := originalSlice
	copiedSlice.V[0] = 14
	fmt.Println("type parameter slice elements", originalSlice.V[0], copiedSlice.V[0])
	copiedSlice.V = []int{15}
	fmt.Println("type parameter slice field", originalSlice.V[0], copiedSlice.V[0])

	// Arrays remain Go values when they instantiate the type parameter.
	originalArrayPtr := &C[[2]int]{V: [2]int{16, 17}}
	originalArray := *originalArrayPtr
	copiedArray := originalArray
	copiedArray.V[0] = 18
	fmt.Println("type parameter array copy", originalArray.V[0], copiedArray.V[0])

	// Named scalar instantiations implement the generated copy traits, and a
	// generic function returning C[T] propagates the stronger bound.
	originalNumberPtr := &C[Number]{V: 19}
	originalNumber := *originalNumberPtr
	copiedNumber := CopyC(originalNumber)
	copiedNumber.V = 20
	fmt.Println("type parameter named scalar", originalNumber.V, copiedNumber.V)

	// A nested generic struct propagates the field-copy bound through C[T].
	originalOuterPtr := &Outer[P]{Box: C[P]{V: P{X: 21}}}
	originalOuter := *originalOuterPtr
	copiedOuter := originalOuter
	copiedOuter.Box.V.X = 22
	fmt.Println("nested generic struct copy", originalOuter.Box.V.X, copiedOuter.Box.V.X)

	// An array containing a slice copies the array value but retains the slice
	// backing storage, as Go does.
	shared := []int{23}
	originalSliceArrayPtr := &C[[1][]int]{V: [1][]int{shared}}
	originalSliceArray := *originalSliceArrayPtr
	copiedSliceArray := originalSliceArray
	copiedSliceArray.V[0][0] = 24
	fmt.Println("type parameter array slice backing", originalSliceArray.V[0][0], copiedSliceArray.V[0][0])
}
