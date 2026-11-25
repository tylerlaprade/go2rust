// Constants Go computes exactly and Rust would compute at run time, and the
// parts of a complex number:
//
//   - int(^uint32(0)) and int(^uint(0)>>1) were written as bare literals
//     where a conversion's value is a cell, and Println locked them.
//   - -math.MinInt64 == math.MinInt64 and int32(-1) >> 40 are constants;
//     written as run-time arithmetic, Rust's overflow lint rejected them.
//   - real(c) and imag(c) were cells that a float print wrote as floats.
package main

import (
	"fmt"
	"math"
)

const big = 1 << 62

func main() {
	fmt.Println(int(^uint(0)>>1), int(^uint32(0)), int64(big), uint8(255))
	fmt.Println(-math.MinInt64 == math.MinInt64, big > 1<<61, int32(-1)>>40, int64(-8)>>70, uint16(1)<<3)
	x := int(^uint32(0)) + 1
	fmt.Println(x, x == 1<<32)

	c := complex(1.5, -2)
	fmt.Println(real(c), imag(c))
	fmt.Printf("%v %.2f %T\n", real(c), imag(c), real(c))
	sum := real(c) + imag(c)
	parts := []float64{real(c), imag(c)}
	fmt.Println(sum, parts, math.Abs(imag(c)))
	var c64 complex64 = complex(3, 4)
	fmt.Println(real(c64), imag(c64), math.Hypot(float64(real(c64)), float64(imag(c64))))
}
