package main

import "fmt"

type item struct {
	name string
}

var items = [...]item{
	{name: "first"},
	{name: "second"},
}

func main() {
	fmt.Println(items[0].name, items[1].name, len(items))
}
