package main

import "fmt"

type entry struct {
	key int
}

func main() {
	go func() {}()

	table := map[int][]entry{
		1: {{key: 1}, {key: 2}},
	}

	for _, bucket := range table {
		for _, e := range bucket {
			fmt.Println(e.key)
		}
	}
}
