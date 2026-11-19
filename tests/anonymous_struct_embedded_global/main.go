package main

import "fmt"

type Payload struct {
	N int
}

var holder struct {
	Payload
	Version int
}

func main() {
	fmt.Println(holder.N, holder.Version)
	holder.N = 7
	holder.Version = 2
	fmt.Println(holder.Payload.N, holder.Version)
}
