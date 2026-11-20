// A named integer converted from a negative constant. go/types leaves the
// operand of a constant expression untyped, and `I(-2)` came out as `- as i32`.
package main

import "fmt"

type I8 int8
type Level int
type U8 uint8
type Big int64

const step = 5

func main() {
	a := I8(-2)
	b := Level(-3)
	var c any = Level(-4)
	levels := []Level{Level(-5), -6}
	fmt.Println(a, b, c, levels)

	fmt.Println(Level(-3+1), I8(-128), Level(^0), Level(-step), Big(-(1 << 40)), Level(+7), U8(^uint8(0)))

	n := 4
	fmt.Println(Level(-n), Level(-n+1))

	fmt.Printf("%v %d %T\n", b, a, a)
	vals := []any{I8(-2), Level(-9), Big(-1)}
	fmt.Println(vals...)
	for _, v := range vals {
		fmt.Printf("%v|", v)
	}
	fmt.Println()

	if b < Level(-2) {
		fmt.Println("below -2")
	}
}
