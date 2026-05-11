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

func main() {
	errs := map[string]error{
		"one": errors.New("one"),
		"nil": nil,
	}
	errs["two"] = customError{msg: "two"}
	errs["nil"] = errors.New("three")

	if err := errs["one"]; err != nil {
		accept(err)
	}
	accept(errs["two"])
	accept(errs["nil"])
}
