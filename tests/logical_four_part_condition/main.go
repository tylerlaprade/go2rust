package main

import "fmt"

func compareAndSwap(state int32, old int32, next int32) bool {
	return state == 4 && old == 4 && next == 6
}

func spin(awoke bool, old int32) bool {
	go func() {}()
	if !awoke && old&2 == 0 && old>>1 != 0 && compareAndSwap(old, old, old|2) {
		return true
	}
	return false
}

func main() {
	fmt.Println(spin(false, 4))
	fmt.Println(spin(true, 4))
	fmt.Println(spin(false, 2))
}
