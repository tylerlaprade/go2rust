package main

import (
	"fmt"
	"go/types"
)

func kind() types.BasicKind {
	return types.Int
}

func dir() types.ChanDir {
	return types.SendRecv
}

func main() {
	if false {
		fmt.Println(uint32(kind()), uint32(dir()))
	}
	fmt.Println("ok")
}
