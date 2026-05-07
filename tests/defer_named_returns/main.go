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

type counter struct{}

func (counter) method() (result int) {
	defer func() {
		result += 3
	}()
	return 4
}

func main() {
	fmt.Println(compute())
	fmt.Println(decorate())
	var c counter
	fmt.Println(c.method())
}
