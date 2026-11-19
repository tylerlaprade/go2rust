package main

import "fmt"

func equal(p, q *int) bool {
	switch p {
	case q:
		return true
	}
	return false
}

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done
	a, b := 3, 3
	p, q := &a, &b
	fmt.Println(equal(p, q), equal(p, p), equal(nil, nil), equal(p, nil), equal(nil, q))
}
