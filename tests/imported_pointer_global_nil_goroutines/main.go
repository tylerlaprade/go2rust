package main

import (
	"fmt"

	helper "example.com/importedpointer/dep"
)

func matchesCurrent(value helper.Reader) bool { return value == helper.Current }

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	original := helper.Current
	view := helper.View()
	fmt.Println(original == helper.Current, matchesCurrent(view))
	helper.Current = nil
	cleared := helper.Get()
	fmt.Println(helper.Current == nil, cleared == nil, original.Value)

	other := helper.Other
	count := 0
	helper.Other, count = (nil), 1
	fmt.Println(helper.Other == nil, other.Value, count)
}
