package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println("Program name:", os.Args[0])
	fmt.Println("Arguments:", os.Args[1:])
	fmt.Println("Total args:", len(os.Args))
}
