// Go evaluates constant expressions exactly and rounds once, so 0.1 + 0.2
// is exactly the float64 nearest 0.3. Adding the two float64 values at run
// time would give 0.30000000000000004.
package main

import "fmt"

const third = 1.0 / 3

func main() {
	x := 0.1 + 0.2
	var y float64 = 0.1 + 0.2
	z := []float64{0.1 + 0.2, 1.1 * 1.1}
	fmt.Println(x, y, z, third*3, x == 0.3)
	fmt.Println(float32(0.1+0.2), 2.0/3*3 == 2)
	fmt.Printf("%.20f\n", 0.1+0.2)
}
