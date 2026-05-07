package main

import "fmt"

type Item struct {
	Name string
}

type Holder struct {
	Item   Item
	Values []int
}

func getItem(h Holder) Item {
	return h.Item
}

func getValues(h Holder) []int {
	return h.Values
}

func main() {
	h := Holder{
		Item:   Item{Name: "go"},
		Values: []int{2, 3},
	}

	item := getItem(h)
	values := getValues(h)

	fmt.Println(item.Name)
	fmt.Println(len(values), values[0], values[1])
}
