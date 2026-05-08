package main

import "fmt"

func key() uint {
	return 1
}

func main() {
	go func() {}()
	m := map[uint]string{1: "one"}
	fmt.Println(m[key()])
}
