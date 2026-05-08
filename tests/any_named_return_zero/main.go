package main

import "fmt"

func zero() (v any) {
	return
}

func main() {
	go func() {}()
	if zero() == nil {
		fmt.Println("nil")
	}
}
