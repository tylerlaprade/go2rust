package main

import "fmt"

func pick(v int) (res int) {
	if v < 0 {
		defer func() {}()
	}
	switch v {
	case 0:
		return 0
	default:
		return 1
	}
}

func main() {
	fmt.Println(pick(0))
	fmt.Println(pick(2))
}
