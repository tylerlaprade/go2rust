package main

import "fmt"

type entry struct {
	key   int
	value string
}

func main() {
	go func() {}()

	bucket := []entry{{key: 1, value: "old"}}
	var hole *entry

	for i, e := range bucket {
		if e.key == 1 {
			hole = &bucket[i]
		}
	}

	if hole != nil {
		*hole = entry{key: 1, value: "new"}
	}

	fmt.Println(bucket[0].value)
}
