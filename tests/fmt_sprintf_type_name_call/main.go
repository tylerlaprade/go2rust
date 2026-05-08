package main

import "fmt"

func label() string {
	return "x"
}

func main() {
	go func() {}()

	fmt.Println(fmt.Sprintf("type %T", label()))
}
