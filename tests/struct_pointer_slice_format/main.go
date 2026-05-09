package main

import "fmt"

type item struct {
	n int
}

type holder struct {
	items []*item
}

func main() {
	h := holder{items: []*item{{n: 1}, {n: 2}}}
	_ = fmt.Sprintf("%v", h)
	fmt.Println("ok")
}
