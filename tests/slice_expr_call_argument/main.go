package main

import "fmt"

func count(values []byte) int {
	return len(values)
}

func main() {
	var buf [128]byte
	fmt.Println(count(buf[:0]))
}
