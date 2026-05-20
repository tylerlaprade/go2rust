package main

import (
	"fmt"

	"example.com/atomicdep/dep"
)

func main() {
	counter := dep.NewCounter()
	fmt.Println(counter.Add(1))
	fmt.Println(counter.Add(-1))
}
