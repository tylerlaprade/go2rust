package main

import "fmt"

type Kind int32

type Entry struct {
	Kind Kind
}

func asInt(k Kind) int {
	return int(k)
}

func asUint64(k Kind) uint64 {
	return uint64(k)
}

func fieldAsInt(e Entry) int {
	return int(e.Kind)
}

func plusAsInt(k Kind) int {
	return int(k + 1)
}

func (k Kind) methodInt() int {
	return int(k)
}

func (k Kind) methodPlus() int {
	return int(k + 1)
}

func main() {
	var k Kind = 3
	entry := Entry{Kind: k}
	fmt.Println(asInt(k))
	fmt.Println(asUint64(k))
	fmt.Println(fieldAsInt(entry))
	fmt.Println(plusAsInt(k))
	fmt.Println(k.methodInt())
	fmt.Println(k.methodPlus())
}
