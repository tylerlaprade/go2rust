package main

type Value interface{ kind() string }

type intVal int64

func (i intVal) kind() string { return "int" }

type complexVal struct {
	re Value
	im Value
}

func (c complexVal) kind() string { return "complex" }

func makeComplex(n int64) Value {
	return complexVal{re: intVal(n), im: intVal(0)}
}

func main() {
	v := makeComplex(5)
	println(v.kind())
}
