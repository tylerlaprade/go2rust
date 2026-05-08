package main

import "fmt"

type entry struct {
	value any
}

func main() {
	go func() {}()
	e := entry{}
	if e.value == nil {
		fmt.Println("nil")
	}
}
