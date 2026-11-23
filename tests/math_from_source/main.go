// math is translated from its Go source, with the generic Go code in place
// of the assembly-backed accelerators, and matches Go bit for bit.
package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println(math.Inf(1) > 1e308, math.IsInf(math.Inf(-1), -1), math.IsNaN(math.NaN()))
	fmt.Println(math.Floor(-2.5), math.Ceil(2.1), math.Trunc(-2.7), math.Round(2.5), math.Abs(-3))
	fmt.Println(math.Sqrt(2), math.Pow(2, 10), math.Exp(1), math.Log(math.E), math.Log2(8), math.Log10(1000))
	fmt.Println(math.Max(3, 7), math.Min(3, 7), math.Hypot(3, 4), math.Mod(7, 3))
	fmt.Println(math.Float64bits(1.5), math.Float64frombits(0x3ff8000000000000), math.Signbit(math.Copysign(0, -1)))
	fmt.Println(math.Sin(0), math.Cos(0), math.Atan2(1, 1)*4, math.MaxInt64, math.MaxFloat64 > 1)
	frac, exp := math.Frexp(8)
	fmt.Println(frac, exp, math.Ldexp(0.5, 4), math.Cbrt(27))
	i, f := math.Modf(3.75)
	fmt.Println(i, f)
}
