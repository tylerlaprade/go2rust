package main

import (
	"fmt"

	"example.com/package-selector-readonly-pointer-method/helper"
)

var local = helper.NewCounter()

func main() {
	fmt.Println(local.Total())
}
