package main

import (
	"fmt"
	"os"
)

func main() {
	read, write, err := os.Pipe()
	if err != nil {
		fmt.Println("pipe error")
		return
	}
	if err := read.Close(); err != nil {
		fmt.Println("read close error")
		return
	}
	if err := write.Close(); err != nil {
		fmt.Println("write close error")
		return
	}
	fmt.Println("closed")
}
