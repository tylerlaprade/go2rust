package main

import (
	"errors"
	"fmt"
)

func parse() (int, error) {
	return 7, errors.New("bad")
}

func main() {
	values := make([]int, 1)
	errs := make([]error, 1)
	values[0], errs[0] = parse()

	fmt.Println(values[0])
	if errs[0] != nil {
		fmt.Println(errs[0].Error())
	}
}
