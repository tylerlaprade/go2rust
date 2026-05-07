package main

import "fmt"

func label() string {
	return "x"
}

func main() {
	err := fmt.Errorf("bad type %T", label())
	fmt.Println(err)
	value := label()
	fmt.Println(fmt.Errorf("stored type %T", value))
}
