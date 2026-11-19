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
	a, b := 3, 3
	p, q := &a, &b
	fmt.Println(equal(p, q), equal(p, p), equal(nil, nil), equal(p, nil), equal(nil, q))
}
