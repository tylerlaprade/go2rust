package main

import "fmt"

type Items []string

var All = Items{"alpha", "beta"}

func first(xs Items) string {
	return xs[0]
}

func grow(xs Items) Items {
	return append(xs, "gamma")
}

func main() {
	grown := grow(All)
	fmt.Println(len(All), first(All), len(grown), grown[2])
}
