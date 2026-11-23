package main

import (
	"fmt"
)

type wrappedError struct {
	error
}

type codeError struct{ code int }

func (codeError) Error() string { return "code" }

func build() error {
	return wrappedError{fmt.Errorf("wrapped %s", "boom")}
}

func main() {
	err := build()
	fmt.Println(err)
	a := wrappedError{codeError{1}}
	b := wrappedError{codeError{1}}
	c := wrappedError{codeError{2}}
	var empty wrappedError
	fmt.Println(a == b, a == c, a == empty, empty == wrappedError{})
}
