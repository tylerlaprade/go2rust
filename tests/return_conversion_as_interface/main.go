package main

type Value interface{ kind() string }

type boolVal bool

func (b boolVal) kind() string { return "bool" }

func makeBool(b bool) Value {
	return boolVal(b)
}

func main() {
	v := makeBool(true)
	println(v.kind())
}
