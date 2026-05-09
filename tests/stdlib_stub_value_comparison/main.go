package main

import (
	"fmt"
	"go/types"
)

func isBidirectional(ch *types.Chan) bool {
	return ch.Dir() == types.SendRecv
}

func hasDirection(ch *types.Chan) bool {
	return ch.Dir()&types.SendRecv != 0
}

func main() {
	if false {
		fmt.Println(isBidirectional(nil))
		fmt.Println(hasDirection(nil))
	}
	fmt.Println("ok")
}
