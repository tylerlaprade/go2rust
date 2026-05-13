package main

type handler func(int) int

type item struct {
	value int
}

type ptrHandler func(*item) int

func inc(x int) int {
	return x + 1
}

func twice(x int) int {
	return x * 2
}

func read(p *item) int {
	return p.value
}

var handlers map[string]handler
var ptrHandlers map[string]ptrHandler

func init() {
	handlers = map[string]handler{
		"inc":   inc,
		"twice": twice,
	}
	ptrHandlers = map[string]ptrHandler{
		"read": read,
	}
}

func main() {
	a := handlers["inc"](4)
	b := handlers["twice"](5)
	c := ptrHandlers["read"](&item{value: 7})
	println(a)
	println(b)
	println(c)
}
