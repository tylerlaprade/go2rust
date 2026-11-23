package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println(strings.IndexByte("aézz", 'z'))
	fmt.Println(strings.IndexByte("aézz", 0xa9))
	fmt.Println(strings.IndexByte("aézz", 0))
	fmt.Println(strings.IndexByte("", 'a'))
	fmt.Println(strings.IndexByte("\x00a", 0))
}
