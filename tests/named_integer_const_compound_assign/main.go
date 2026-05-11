package main

import "fmt"

type LoadMode int

const (
	NeedName LoadMode = 1 << iota
	NeedFiles
	NeedImports
)

func main() {
	var mode LoadMode
	mode |= NeedImports
	mode |= 8
	mode |= NeedFiles | NeedName
	fmt.Println(int(mode))
}
