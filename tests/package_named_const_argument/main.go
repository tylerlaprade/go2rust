package main

import (
	"fmt"

	"example.com/package-named-const-argument/helper"
)

func main() {
	v := helper.NewVar()
	helper.SetKind(v, helper.PackageVar)
	fmt.Println("ok")
}
