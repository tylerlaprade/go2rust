package main

import (
	"fmt"
	"os"
)

func main() {
	defer os.Remove("test.txt")

	file, err := os.Create("test.txt")
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	defer file.Close()

	file.WriteString("Hello, World!")
	fmt.Println("File written successfully")
}
