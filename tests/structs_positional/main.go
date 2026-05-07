package main

import "fmt"

type Pair struct {
	Left  string
	Right int
}

func main() {
	p := Pair{"go", 2}
	fmt.Println(p.Left, p.Right)
}
