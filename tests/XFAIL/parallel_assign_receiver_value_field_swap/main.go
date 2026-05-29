package main

// Companion to parallel_assign_value_field_swap: when a value-typed field is
// read directly off the current receiver (x.a) in a swap, the capture keeps the
// field's wrapped handle (unlike the same field read through a non-receiver
// pointer variable, which unwraps to a bare value). The target consume must
// move the value out of that handle rather than re-wrap a bare value.

type box struct {
	a int
	b int
}

func (x *box) rotate(v int) int {
	old := x.a
	x.a, x.b, v = x.b, v, x.a
	return old + x.a + x.b + v
}

func main() {
	x := &box{a: 1, b: 2}
	r := x.rotate(9)
	println(r)
	println(x.a)
	println(x.b)
}
