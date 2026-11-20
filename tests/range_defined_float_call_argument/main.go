// A range value of a defined float type, type Celsius float64, was passed
// to a function as *x, a dereference of the owned value, and did not
// compile.
package main

import "fmt"

type Celsius float64

func sumBy(xs []Celsius, f func(Celsius) Celsius) Celsius {
	var total Celsius
	for _, x := range xs {
		total += f(x)
	}
	return total
}

func main() {
	temps := []Celsius{20.5, 22, 19.5}
	fmt.Println(sumBy(temps, func(c Celsius) Celsius { return c * 2 }))
}
