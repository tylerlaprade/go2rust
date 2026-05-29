package main

import "fmt"

type Expr interface{ kind() string }

type lit struct{ v string }

func (l *lit) kind() string { return l.v }

type checker struct{ n int }

func (c *checker) use(args ...Expr) {
	for _, a := range args {
		c.n += len(a.kind())
	}
}

func (c *checker) run(rhs Expr) {
	c.use(rhs) // variadic of interface; rhs is an Expr value
}

func main() {
	c := &checker{}
	c.run(&lit{v: "abc"})
	fmt.Println(c.n)
}
