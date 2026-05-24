package main

import (
	"errors"
	"fmt"
)

type wrappedErr struct {
	err error
	tag any
}

func mayPanic(triggerPanic bool) {
	if triggerPanic {
		panic(wrappedErr{err: errors.New("boom"), tag: 7})
	}
	fmt.Println("no panic")
}

func main() {
	mayPanic(false)
}
