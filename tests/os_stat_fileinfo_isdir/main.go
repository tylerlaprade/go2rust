package main

import (
	"fmt"
	"os"
)

func main() {
	if false {
		info, err := os.Stat(".")
		if err == nil {
			fmt.Println(info.IsDir())
		}
	}
}
