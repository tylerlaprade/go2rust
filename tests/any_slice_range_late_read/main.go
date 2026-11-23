package main

import "fmt"

func main() {
	values := []any{7, "before", 9}
	for i, value := range values {
		if i == 0 {
			values[1] = "after"
		}
		fmt.Println(value)
	}
}
