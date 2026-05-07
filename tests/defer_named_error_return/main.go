package main

import (
	"errors"
	"fmt"
)

func compute() (result int, err error) {
	defer func() {
		result++
	}()
	err = errors.New("boom")
	return 2, err
}

func main() {
	result, err := compute()
	fmt.Println(result)
	fmt.Println(err)
}
