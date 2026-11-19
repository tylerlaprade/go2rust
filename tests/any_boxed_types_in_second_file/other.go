package main

import "fmt"

// The helpers that clone and name boxed values are generated into their own
// file. They have to import what these payload types are spelled with, even
// though main.go uses none of it.
func typeName(v interface{}) string {
	return fmt.Sprintf("%T", v)
}

func report() {
	counts := map[string]int{"a": 1}
	failed := func() error { return nil }
	items := []interface{}{counts, failed, 3}
	for _, item := range items {
		fmt.Printf("%T\n", item)
	}
	fmt.Println(typeName(counts))
}
