package main

import "fmt"

type box struct {
	items []int
}

func (b *box) Print(n int) {
	fmt.Println(n)
}

func (b *box) Add(x int) int {
	b.items = append(b.items, x)
	return len(b.items)
}

func (b *box) Flush() {
	b.Print(len(b.items))
	b.Print(b.Add(len(b.items)))
}

func main() {
	b := &box{items: []int{1, 2, 3}}
	b.Flush()
	fmt.Println(len(b.items))
}
