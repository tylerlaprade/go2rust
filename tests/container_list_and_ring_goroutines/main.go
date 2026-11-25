// In a program with goroutines: container/list and container/ring from
// source, where comparing with the receiver must not lock its own handle.
package main

import (
	"container/list"
	"container/ring"
	"fmt"
)

func printList(l *list.List) {
	for e := l.Front(); e != nil; e = e.Next() {
		fmt.Print(e.Value, " ")
	}
	fmt.Println("| len", l.Len())
}

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done
	l := list.New()
	e4 := l.PushBack(4)
	e1 := l.PushFront(1)
	l.InsertBefore(3, e4)
	l.InsertAfter(2, e1)
	printList(l)
	l.MoveToBack(e1)
	l.MoveBefore(e4, l.Front())
	printList(l)
	l.Remove(e4)
	other := list.New()
	other.PushBack(9)
	l.PushBackList(other)
	printList(l)
	for e := l.Back(); e != nil; e = e.Prev() {
		fmt.Print(e.Value, " ")
	}
	fmt.Println()

	r := ring.New(4)
	for i := 0; i < r.Len(); i++ {
		r.Value = i * i
		r = r.Next()
	}
	r.Do(func(v any) { fmt.Print(v, " ") })
	fmt.Println(r.Len())
}
