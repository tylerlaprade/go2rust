package main

import (
	"fmt"
	"os"
)

func main() {
	const path = "go2rust-file-handle.txt"

	file, err := os.Create(path)
	if err != nil {
		fmt.Println(false)
		return
	}
	_, err = file.WriteString("first")
	fmt.Println(err == nil && file.Close() == nil)

	file, err = os.Open(path)
	fmt.Println(err == nil && file.Close() == nil)

	file, err = os.OpenFile(path, os.O_APPEND|os.O_WRONLY, 0600)
	if err != nil {
		fmt.Println(false)
		return
	}
	_, err = file.WriteString("-second")
	fmt.Println(err == nil && file.Close() == nil)

	contents, err := os.ReadFile(path)
	fmt.Println(string(contents), err == nil)
	fmt.Println(os.Remove(path) == nil)
}
