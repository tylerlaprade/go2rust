package main

import (
	"fmt"
	"github.com/stretchr/testify/assert"
)

func main() {
	// This test verifies that external package imports are detected
	fmt.Println("Testing external package imports")

	// This would normally use testify
	if assert.Equal(nil, 1, 1) {
		fmt.Println("Assert would work here")
	}
}
