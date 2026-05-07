package main

import "fmt"

type entry struct {
	deps string
}

var entries = [...]entry{
	{deps: "abc"},
}

func main() {
	data := []byte(entries[0].deps)
	fmt.Println(len(data), data[1])
}
