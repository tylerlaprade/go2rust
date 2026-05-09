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

func intValue() int {
	return 1
}

func kindName() string {
	switch kind() {
	case types.Int:
		return "int"
	default:
		return "other"
	}
}

func intName() string {
	switch intValue() {
	case 1:
		return "one"
	default:
		return "other"
	}
}

func main() {
	if false {
		fmt.Println(uint32(kind()), uint32(dir()), uint32(zeroKind()), posFromInt(1), kindName(), intName())
	}
	fmt.Println("ok")
}
