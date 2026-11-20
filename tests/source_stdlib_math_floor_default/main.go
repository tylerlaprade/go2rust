package main

import "math"

func main() {
	if math.Floor(2.6) != 2 {
		panic("math.Floor mismatch")
	}
	println(1)
}
