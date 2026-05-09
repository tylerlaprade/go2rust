package main

import (
	"fmt"
	"strconv"
)

func check(err error) {
	if err != nil {
		fmt.Println("bad")
	}
}

func main() {
	_, err := strconv.Atoi("x")
	check(err)
	fmt.Println("ok")
}
