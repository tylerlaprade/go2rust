package main

import (
	"errors"
	"fmt"
)

func describe(err error) string {
	if err == nil {
		return "nil"
	}
	return err.Error()
}

func main() {
	fmt.Println(describe(errors.New("boom")))
}
