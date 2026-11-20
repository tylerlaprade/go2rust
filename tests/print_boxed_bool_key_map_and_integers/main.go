// A map with boolean keys held in an interface{} prints like any other map,
// false before true. Integers of every size print as numbers. These used to
// print "<unknown>".
package main

import "fmt"

func show(v any) { fmt.Println(v) }

func main() {
	format := "%v\n"
	fmt.Printf(format, map[bool]int{true: 1, false: 0})
	var x any = map[bool]string{true: "yes"}
	fmt.Println(x)
	show(map[bool]bool{false: true})

	var u8 any = uint8(7)
	var i8 any = int8(-8)
	var i16 any = int16(-3)
	var u16 any = uint16(65535)
	var u32 any = uint32(1 << 31)
	var u64 any = uint64(1 << 40)
	var u any = uint(9)
	fmt.Println(u8, i8, i16, u16, u32, u64, u)
	fmt.Printf(format, uint(9))
	fmt.Printf("%v %d\n", u8, u64)
	show(int8(1))
	show(uint16(2))
}
