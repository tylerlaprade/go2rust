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
	results := make(chan bool, 8)
	go func() {
		converted := uint32(0640)
		values := []int{0640, 0_640, 0o640, 0o640}

		results <- legacy == 416
		results <- underscored == 416
		results <- modern == 416 && upperModern == 416
		results <- hexUpper == 26 && binaryUpper == 10
		results <- returned() == 511
		results <- floatReturned() == 416
		results <- converted == 416
		results <- values[0] == 416 && values[1] == 416 && values[2] == 416 && values[3] == 416
	}()
	for i := 0; i < 8; i++ {
		fmt.Println(<-results)
	}
}
