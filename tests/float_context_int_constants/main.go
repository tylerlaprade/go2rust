// Integer constants that Go converts to float64 or float32 keep their values:
// negated literals in slice, map and struct literals, a named untyped
// constant, and a shifted constant.
package main

import "fmt"

type P struct{ X, Y float64 }

func scale(f float64) float64 { return f * 2 }

func main() {
	fs := []float64{2.5, -1, 3}
	var a float64 = -2
	b := []float32{-1, +2}
	m := map[string]float64{"x": -3}
	p := P{-1, -2}
	fmt.Println(fs, a, b, m, p, scale(-4), -1.5+a)
	const k = -7
	var c float64 = k
	fmt.Println(c, float64(-k)/2)
	weights := [3]float32{1 << 3, -2 * 3, (4)}
	fmt.Println(weights, weights[0]/weights[1])
	var r float64 = 'a'
	fmt.Println(r / 2)
}
