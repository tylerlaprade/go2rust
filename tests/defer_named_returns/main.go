package main

import "fmt"

func compute() (result int) {
	defer func() {
		result += 10
	}()
	result = 5
	return
}

func decorate() (msg string) {
	defer func() {
		msg = "[" + msg + "]"
	}()
	msg = "ok"
	return
}

func main() {
	fmt.Println(compute())
	fmt.Println(decorate())
}
