package main

import (
	"fmt"
	"go/types"
)

func main() {
	_ = types.Config{
		Error: func(err error) {},
	}
	fmt.Println("ok")
}
