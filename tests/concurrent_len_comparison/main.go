package main

import "fmt"

func consumedAll(v string) bool {
	i := 0
	for i < len(v) {
		i++
	}
	return i == len(v)
}

func main() {
	ch := make(chan int, 1)
	ch <- 1
	fmt.Println(<-ch)
	fmt.Println(consumedAll("abc"))
}
