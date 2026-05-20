package main

import "fmt"

type pair struct {
	left  string
	right string
}

func both(p *pair) (string, string) {
	return p.left, p.right
}

func main() {
	done := make(chan bool)
	go func() {
		done <- true
	}()
	<-done

	p := &pair{left: "left", right: "right"}
	left, right := both(p)
	fmt.Println(left, right)
}
