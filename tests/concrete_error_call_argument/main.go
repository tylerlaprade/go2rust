package main

import "fmt"

type customError struct {
	msg string
}

func (e customError) Error() string {
	return e.msg
}

func accept(err error) {
	fmt.Println(err.Error())
}

func main() {
	accept(customError{msg: "boom"})
}
