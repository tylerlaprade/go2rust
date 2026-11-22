package main

import (
	"fmt"
	"os"
)

func main() {
	wd, err := os.Getwd()
	fmt.Println(err == nil)
	b := []byte(wd)
	fmt.Println(b[len(b)-2:])
}
