package main

import (
	"fmt"
	"strconv"
)

func main() {
	value, err := strconv.Unquote("\"newline\\n\"")
	if err != nil {
		fmt.Println("error")
		return
	}
	fmt.Print(value)
	fmt.Println("ok")
}
