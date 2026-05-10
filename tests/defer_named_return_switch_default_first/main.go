package main

import "fmt"

func pick(v int) (res int) {
	if v < 0 {
		defer func() {}()
	}
	switch v {
	default:
		return 1
	case 0:
		return 0
	}
}

func main() {
	fmt.Println(pick(0))
	fmt.Println(pick(2))
}
