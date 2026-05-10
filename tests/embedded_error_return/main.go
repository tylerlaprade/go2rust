package main

import (
	"fmt"
)

type wrappedError struct {
	error
}

func build() error {
	return wrappedError{fmt.Errorf("wrapped %s", "boom")}
}

func main() {
	err := build()
	fmt.Println(err)
}
