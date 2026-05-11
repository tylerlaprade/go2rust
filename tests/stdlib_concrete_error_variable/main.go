package main

import (
	"fmt"
	"go/types"
)

func accept(err error) {
	if err != nil {
		fmt.Println("ok")
	}
}

func main() {
	err := types.Error{Msg: "boom"}
	accept(err)
}
