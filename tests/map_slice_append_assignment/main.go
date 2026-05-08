package main

import "fmt"

type entry struct {
	key int
}

func main() {
	go func() {}()

	table := map[int][]entry{
		1: {{key: 1}},
	}
	bucket := table[1]
	table[1] = append(bucket, entry{key: 2})

	fmt.Println(len(table[1]))
	fmt.Println(table[1][1].key)
}
