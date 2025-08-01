package main

import (
	"fmt"
	_ "image/png"      // register PNG decoder
	_ "net/http/pprof" // register pprof handlers
)

func init() {
	fmt.Println("main package init")
}

func main() {
	fmt.Println("main function")
}
