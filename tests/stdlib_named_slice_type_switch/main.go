package main

import (
	"fmt"
	"go/scanner"
)

func describe(err error) {
	switch err := err.(type) {
	case scanner.ErrorList:
		for _, item := range err {
			fmt.Println(item.Msg)
		}
	default:
		fmt.Println("other")
	}
}

func main() {
	var err error
	describe(err)
}
