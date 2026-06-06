package main

import (
	"fmt"
	"go/types"
)

func makeType() (res types.Type) {
	defer func() {}()
	return nil
}

func main() {
	fmt.Println(makeType() == nil)
}
