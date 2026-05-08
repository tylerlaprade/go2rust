package main

import "fmt"

func main() {
	var buf [128]byte
	fmt.Println(len(buf), buf[0])
}
