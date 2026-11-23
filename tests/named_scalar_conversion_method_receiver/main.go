package main

import (
	"fmt"
	"os"
)

func main() {
	rawDir := uint32(os.ModeDir | 0755)
	converted := os.FileMode(0644)

	fmt.Println(!os.FileMode(0644).IsDir())
	fmt.Println(os.FileMode(os.ModeDir | 0755).IsDir())
	fmt.Println(os.FileMode(rawDir).IsDir())
	fmt.Println(!converted.IsDir())
}
