package main

import "fmt"

func main() {
	ch := make(chan struct{})
	close(ch)
	<-ch
	fmt.Println("closed")
}
