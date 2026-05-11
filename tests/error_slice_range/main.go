package main

import (
	"errors"
	"fmt"
)

type customError struct {
	msg string
}

func (e customError) Error() string {
	return e.msg
}

func accept(err error) {
	if err == nil {
		fmt.Println("nil")
		return
	}
	fmt.Println(err.Error())
}

func collect() []error {
	errs := []error{errors.New("boom"), nil}
	errs[1] = errors.New("two")
	errs = append(errs, customError{msg: "custom"})
	return errs
}

func main() {
	for _, err := range collect() {
		accept(err)
	}
}
