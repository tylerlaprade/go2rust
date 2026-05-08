package main

import "fmt"

type termSet struct {
	complete bool
}

func compute() bool {
	seen := make(map[string]*termSet)
	tset := &termSet{}
	defer func() {
		tset.complete = true
	}()
	seen["x"] = tset
	return false
}

func main() {
	fmt.Println(compute())
}
