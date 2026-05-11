package main

import "fmt"

func main() {
	chunks := [][]string{}
	patterns := []string{"a", "bb", "ccc"}
	start := 0

	chunks = append(chunks, patterns[start:2])
	chunks = append(chunks, patterns[2:])

	fmt.Println(len(chunks), len(chunks[0]), chunks[0][0], chunks[0][1], len(chunks[1]), chunks[1][0])
}
