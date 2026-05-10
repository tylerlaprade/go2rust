package main

import "fmt"

type queue struct {
	later []func()
}

func main() {
	q := queue{later: []func(){}}
	fmt.Println(q)
}
