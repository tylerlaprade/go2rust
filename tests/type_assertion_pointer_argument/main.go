package main

import "fmt"

type info struct {
	name string
}

func accept(ptr *info) string {
	return ptr.name
}

func box(value any) any {
	return value
}

func main() {
	value := box(&info{name: "alpha"})
	fmt.Println(accept(value.(*info)))
}
