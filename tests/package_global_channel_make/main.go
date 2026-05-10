package main

import "fmt"

type token struct{}

func size() int {
	return 2
}

var sem = make(chan token, size())

func main() {
	sem <- token{}
	fmt.Println(len(sem))
	<-sem
	fmt.Println(len(sem))
}
