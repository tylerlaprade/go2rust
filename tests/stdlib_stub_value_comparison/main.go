package main

import (
	"fmt"
	"go/types"
)

func isBidirectional(ch *types.Chan) bool {
	return ch.Dir() == types.SendRecv
}

func main() {
	if false {
		fmt.Println(isBidirectional(nil))
	}
	fmt.Println("ok")
}
