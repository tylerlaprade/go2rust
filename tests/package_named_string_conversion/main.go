package main

import (
	"fmt"

	"example.com/package-named-string-conversion/helper"
)

func main() {
	p := helper.Path(helper.Text())
	fmt.Println(p == "")
	fmt.Println(helper.Object(p))
}
