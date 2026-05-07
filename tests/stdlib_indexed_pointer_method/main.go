package main

import (
	"fmt"
	"go/types"
)

func main() {
	terms := []*types.Term{types.NewTerm(false, nil)}
	terms[0].Type()
	for _, term := range terms {
		term.Type()
	}
	fmt.Println("ok")
}
