package main

import "fmt"

func consume(any) {}

func main() {
	defer func() {
		if recover() != nil {
			fmt.Println("recovered bounds panic")
		}
	}()
	var values []any
	consume(values[0])
}
