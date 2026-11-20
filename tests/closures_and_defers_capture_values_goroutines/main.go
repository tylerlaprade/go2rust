// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

func deferred() {
	x := 1
	defer fmt.Println("deferred x:", x)
	x = 2
	for i := 3; i > 0; i-- {
		defer fmt.Print(i, " ")
	}
	name := "first"
	defer func(n string) { fmt.Println("arg:", n) }(name)
	name = "second"
	defer func() { fmt.Println("closure sees:", name) }()
	fmt.Println("x is", x)
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started

	var funcs []func() int
	for i := 0; i < 3; i++ {
		funcs = append(funcs, func() int { return i * i })
	}
	for _, f := range funcs {
		fmt.Print(f(), " ")
	}
	fmt.Println()

	var printers []func()
	for i, s := 0, "a"; i < 3; i, s = i+1, s+"a" {
		printers = append(printers, func() { fmt.Print(i, s, " ") })
	}
	for _, p := range printers {
		p()
	}
	fmt.Println()

	total := 0
	for i := 0; i < 4; i++ {
		add := func() { total += i }
		add()
	}
	fmt.Println("total:", total)

	count := 0
	for i := 0; i < 10; {
		step := func() { i += 3 }
		step()
		count++
	}
	fmt.Println("steps:", count)
	deferred()
	fmt.Println()
}
