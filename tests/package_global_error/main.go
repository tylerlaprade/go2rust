package main

import (
	"errors"
	"fmt"
)

var ErrUnset error
var ErrBoom = errors.New("boom")

func main() {
	if ErrUnset == nil {
		fmt.Println("unset")
	}
	if ErrBoom != nil {
		fmt.Println(ErrBoom)
	}
}
