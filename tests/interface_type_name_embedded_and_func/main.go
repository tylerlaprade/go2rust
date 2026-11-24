// %T of an interface that embeds two others names the dynamic type, as does
// %T of a nil one; a defined func type implements an interface.
package main

import "fmt"

type A interface{ PA() int }
type B interface{ PB() int }
type Both interface {
	A
	B
}

type T struct{ n int }

func (t T) PA() int { return t.n }
func (t T) PB() int { return -t.n }

type Runner interface{ Run() string }
type F func() string

func (f F) Run() string { return f() }

func show(b Both) string { return fmt.Sprintf("%T %d", b, b.PA()) }

func main() {
	var b Both = T{3}
	fmt.Printf("%T %d %d\n", b, b.PA(), b.PB())
	fmt.Println(show(T{4}))
	var r Runner = F(func() string { return "ran" })
	fmt.Printf("%v\n", r.Run())
	for _, x := range []Both{T{1}, nil} {
		fmt.Printf("%T\n", x)
	}
}
