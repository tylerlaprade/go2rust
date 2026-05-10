package main

import (
	"fmt"

	"example.com/fnconvert/dep"
)

type Exporter func(int) int

func Use(e Exporter) int {
	return dep.Set(dep.Exporter(e))
}

func main() {
	fmt.Println(Use(func(v int) int {
		return v * 2
	}))
}
