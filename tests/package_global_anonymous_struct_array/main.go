package main

import "fmt"

var modes = [...]struct {
	value int
	name  string
}{
	{1, "one"},
	{2, "two"},
}

func main() {
	for _, item := range modes {
		fmt.Println(item.name, item.value)
	}
}
