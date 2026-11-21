package main

import "fmt"

type Small complex64
type Phasor complex128

func printDynamic(format string, args ...any) {
	fmt.Printf(format, args...)
}

func main() {
	p := Phasor(3 + 4i)
	var source complex64 = 2 - 5i
	s := Small(source)
	fromReal := Phasor(2)
	var boxed any = p

	fmt.Printf("direct: %v %.2f %T | %v %g %T | %v\n", p, p, p, s, s, s, fromReal)
	fmt.Printf("boxed: %v %.2f %T\n", boxed, boxed, boxed)
	printDynamic("spread: %v %.2f %T | %v %g %T\n", p, p, p, s, s, s)
}
