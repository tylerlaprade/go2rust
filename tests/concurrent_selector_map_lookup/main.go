package main

import "fmt"

type holder struct {
	table map[int]string
}

func main() {
	go func() {}()
	h := &holder{table: map[int]string{1: "one"}}
	fmt.Println(h.table[1])
}
