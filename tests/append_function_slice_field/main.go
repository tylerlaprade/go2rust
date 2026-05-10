package main

import "fmt"

type queue struct {
	later []func()
}

func (q *queue) add(fn func()) {
	q.later = append(q.later, fn)
}

func main() {
	go func() {}()

	q := queue{later: []func(){}}
	q.add(func() {
		fmt.Println("later")
	})
	f := q.later[0]
	f()
}
