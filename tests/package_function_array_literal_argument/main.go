package main

import (
	"fmt"

	"example.com/arrayarg/dep"
)

func main() {
	labels := []dep.Item{dep.Of(4), dep.Of(5)}
	fmt.Println(dep.Make([3]dep.Item{dep.Of(1), dep.Of(2), dep.Of(3)}, labels))
}
