package main

// A struct field that is a nested slice of pointers ([][]*Ident) gets an
// auto-generated Display impl. The innermost element lowers to a wrapped handle,
// which the plain nested-slice formatter cannot Display — it must unwrap each
// handle (format_nested_slice_wrapped). The struct's mere existence forces the
// Display impl to be generated and compiled.

type Ident struct {
	name string
}

type resolver struct {
	stack [][]*Ident
}

func main() {
	a := &Ident{name: "a"}
	b := &Ident{name: "b"}
	r := resolver{stack: [][]*Ident{{a, b}, {a}}}
	println(len(r.stack))
	println(len(r.stack[0]))
	println(r.stack[0][0].name)
}
