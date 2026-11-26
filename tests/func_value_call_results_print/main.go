// The result of calling a func value that a call, a map or a slice gives
// prints, compares and passes like any other string.
package main

import "fmt"

func swapIn(f func() string) func() string { return f }

func makeAdder(n int) func(int) int { return func(x int) int { return x + n } }

type pair struct{ a, b string }

func pairer() func() pair { return func() pair { return pair{"x", "y"} } }

func main() {
	g := func() string { return "orig" }
	fmt.Println(swapIn(g)())
	hooks := map[string]func() string{"a": g}
	fmt.Println(hooks["a"]())
	list := []func() string{g}
	fmt.Println(list[0](), len(list[0]()))
	fmt.Println(makeAdder(2)(3))
	fmt.Printf("%s|%v\n", swapIn(g)(), pairer()())
	s := swapIn(g)() + "!"
	fmt.Println(s)
	if swapIn(g)() == "orig" && hooks["a"]() != "" {
		fmt.Println("equal")
	}
	t := list[0]()
	var u string = hooks["a"]()
	fmt.Println(t, u, pairer()().a, func() string { return "lit" }())
	fmt.Println(greet(swapIn(g)()), echo(hooks["a"]))
	parts := []string{swapIn(g)(), hooks["a"]()}
	fmt.Println(parts, map[string]string{"k": list[0]()})
	var x any = swapIn(g)()
	fmt.Println(x)
}

func greet(s string) string { return "hi " + s }

func echo(f func() string) string { return f() }
