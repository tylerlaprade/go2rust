package main

import "fmt"

func byteBit(i int) byte {
	return byte(1) << (i % 8)
}

func uint64Mask(i uint) uint64 {
	return uint64(1) << i
}

func byteFromExpr(v byte) byte {
	return byte(v + '0')
}

func main() {
	fmt.Println("byte literal bit:", byteBit(3))
	fmt.Println("uint64 literal mask:", uint64Mask(5))
	fmt.Println("byte expression:", byteFromExpr(4))
}
