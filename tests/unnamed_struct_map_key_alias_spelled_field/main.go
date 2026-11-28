package main

import "fmt"

type Part struct{ A int }

var seen = map[Part]bool{}

func main() {
	keys := map[struct{ X int }]bool{}
	keys[origin()] = true
	keys[struct{ X int }{1}] = false
	seen[Part{1}] = true
	fmt.Println(len(keys), keys[origin()], len(seen))
}
