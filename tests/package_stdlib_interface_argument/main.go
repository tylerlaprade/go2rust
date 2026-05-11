package main

import (
	"fmt"
	"os"

	"example.com/package-stdlib-interface-argument/helper"
)

func main() {
	f, err := os.Open(os.Args[0])
	if err != nil {
		panic(err)
	}
	defer f.Close()

	fmt.Println(helper.Use(f))
}
