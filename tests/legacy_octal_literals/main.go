package main

import "fmt"

const (
	legacy      = 0640
	underscored = 0_640
	modern      = 0o640
	upperModern = 0o640
	hexUpper    = 0x1A
	binaryUpper = 0b1010
)

func returned() int {
	return 0777
}

func floatReturned() float64 {
	return 0640
}

func main() {
	converted := uint32(0640)
	values := []int{0640, 0_640, 0o640, 0o640}

	fmt.Println(legacy == 416)
	fmt.Println(underscored == 416)
	fmt.Println(modern == 416 && upperModern == 416)
	fmt.Println(hexUpper == 26 && binaryUpper == 10)
	fmt.Println(returned() == 511)
	fmt.Println(floatReturned() == 416)
	fmt.Println(converted == 416)
	fmt.Println(values[0] == 416 && values[1] == 416 && values[2] == 416 && values[3] == 416)
}
