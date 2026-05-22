package main

import "fmt"

type info struct {
	name  string
	count int
}

func main() {
	go func() {}()

	infos := []info{{name: "foo", count: 1}, {name: "bar", count: 2}}

	alt := &infos[1]
	fmt.Println(alt.name)
	fmt.Println(alt.count)

	alt.count = 42
	fmt.Println(infos[1].count)
}
