package main

import "fmt"

func main() {
	pointer := &splitError{code: 7}
	var err error = pointer
	asserted, ok := err.(*splitError)
	fmt.Println(err == pointer, ok, asserted == pointer, err.Error())
}
