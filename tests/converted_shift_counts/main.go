package main

import "fmt"

type mode uint32

func hasBit(m mode, i int) bool {
	return m&(1<<uint(31-i)) != 0
}

func shiftRight(m mode, i int) mode {
	return m >> uint(i)
}

func main() {
	m := mode(0x80000010)
	fmt.Println(hasBit(m, 0), hasBit(m, 1), shiftRight(m, 4) == 0x08000001)
}
