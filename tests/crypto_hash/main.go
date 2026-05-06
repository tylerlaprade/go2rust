package main

import (
	"crypto/sha256"
	"fmt"
)

func main() {
	data := "Hello, World!"
	hash := sha256.Sum256([]byte(data))
	fmt.Printf("SHA256: %x\n", hash)
}
