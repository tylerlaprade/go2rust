// fmt prints a float as %v: strconv's shortest %g, which switches to an
// exponent when the exponent is below -4 or at least 6. Rust's Display never
// does. This holds for Println and Sprint operands, struct fields, slice and
// map elements, a type defined as a float, and a float inside an interface.
package main

import "fmt"

type Celsius float64

type Point struct {
	X, Y float64
	Name string
	Z    float32
}

func main() {
	for _, v := range []float64{1e10, 1e20, 1e21, 123456.0, 1234567.0, 0.0001, 0.00001, 3.0, 2.5e-7, 100000, 1e6, -4.5e15, 123456789.125} {
		fmt.Println(v)
	}
	zero := 0.0
	fmt.Println(1/zero, -1/zero, zero/zero, -zero, float32(0.1), float32(16777216.0), 1e-7)
	fmt.Println([]float64{1e10, 0.5, 1234567})
	fmt.Println(Point{1e9, -2.5e-8, "p", 3e7})
	fmt.Println(map[string]float64{"a": 1e21, "b": 2})
	var c Celsius = 1e7
	fmt.Println(c, Celsius(21.5))
	s := fmt.Sprint(1e10, " ", 2.0)
	fmt.Println(s, fmt.Sprintln(3e-9))
	fmt.Print(4e12, "\n")
	fmt.Printf("%v %v %g\n", []float64{1e10}, Point{1e9, 1, "q", 0}, 1e10)
	var arr [2]float32
	arr[0] = 1e8
	fmt.Println(arr)
	p := &Point{X: 1e6}
	fmt.Println(*p)
	var x any = 1e10
	var y any = float32(2e-6)
	fmt.Println(x, y)
}
