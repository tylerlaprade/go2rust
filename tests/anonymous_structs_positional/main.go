package main

import "fmt"

func main() {
	item := struct {
		Name  string
		Count int
	}{"go", 2}
	fmt.Println(item.Name, item.Count)
}
