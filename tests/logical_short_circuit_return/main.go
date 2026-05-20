package main

import "fmt"

type item struct {
	Name string
}

func hasName(it *item) bool {
	return it != nil && it.Name == "ready"
}

func missingOrReady(it *item) bool {
	return it == nil || it.Name == "ready"
}

func main() {
	fmt.Println(hasName(nil))
	fmt.Println(hasName(&item{Name: "ready"}))
	fmt.Println(missingOrReady(nil))
	fmt.Println(missingOrReady(&item{Name: "other"}))
}
