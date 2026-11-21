package main

import "fmt"

func fill(dst []byte, done chan bool) {
	dst[0] = 9
	done <- true
}

func main() {
	a := [3]byte{1, 2, 3}
	done := make(chan bool)
	go fill(a[1:], done)
	<-done
	fmt.Println(a)
}
