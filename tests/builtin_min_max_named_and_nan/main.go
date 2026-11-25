// The builtin min and max of named scalars, which are newtypes over a
// cell, and of floats: a NaN argument gives NaN, and of a negative and a
// positive zero, min gives the negative one and max the positive one.
package main

import "fmt"

type Celsius float64

type Level int

type Name string

func main() {
	a, b := Celsius(21.5), Celsius(-3)
	m := a
	m = max(m, b)
	fmt.Println(max(a, b), min(a, b, 0), m)
	l := Level(3)
	fmt.Println(max(l, 7), min(l, Level(1)))
	fmt.Println(max(Name("bo"), Name("al")), min(Name("bo"), "cy"))
	zero := 0.0
	nan := zero / zero
	negZero := -zero
	fmt.Println(max(1.0, nan), min(nan, 2.0), max(negZero, 0.0), min(0.0, negZero), 1/min(0.0, negZero), 1/max(negZero, 0.0))
	fmt.Println(max(Celsius(nan), a), min(float32(2), 1.5), max(3, 7, 1), min(2.5, 1.5))
}
