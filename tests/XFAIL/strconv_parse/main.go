package main

import (
	"fmt"
	"strconv"
)

func main() {
	str := "42"
	num, err := strconv.Atoi(str)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	fmt.Println("Parsed number:", num)
}
