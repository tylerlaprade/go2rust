package main

import (
	"fmt"
	"go/token"
	"go/types"
)

func kind() types.BasicKind {
	return types.Int
}

func dir() types.ChanDir {
	return types.SendRecv
}

func zeroKind() types.BasicKind {
	return 0
}

func posFromInt(n int) token.Pos {
	return token.Pos(n)
}

func main() {
	if false {
		fmt.Println(uint32(kind()), uint32(dir()), uint32(zeroKind()), posFromInt(1))
	}
	fmt.Println("ok")
}
