package main

import (
	"fmt"
	"sort"
)

func main() {
	index := map[string]map[string]uint64{
		"dep": {"beta": 2, "alpha": 1},
	}

	names := make([]string, 0, len(index["dep"]))
	for name := range index["dep"] {
		names = append(names, name)
	}
	sort.Strings(names)
	fmt.Println(names[0], names[1])
}
