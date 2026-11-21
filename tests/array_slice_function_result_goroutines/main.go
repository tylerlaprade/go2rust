package main

import "fmt"

func makeArray() [3]int {
	return [3]int{1, 2, 3}
}

func update(view []int, done chan bool) {
	view[1] = 9
	done <- true
}

func main() {
	a := makeArray()
	done := make(chan bool)
	go update(a[:], done)
	<-done
	fmt.Println(a)
}
