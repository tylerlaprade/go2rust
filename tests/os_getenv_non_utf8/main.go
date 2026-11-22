package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println([]byte(os.Getenv("GO2RUST_BAD")))
	fmt.Println([]byte(os.Getenv("GO2RUST_MISSING")))
}
