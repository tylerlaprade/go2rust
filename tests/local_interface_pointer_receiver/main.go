package main

import "fmt"

type List interface {
	Valid(index int) bool
	Label(index int) int
}

type list struct {
	labels []int
}

func (l *list) Valid(index int) bool {
	return index >= 0 && index < len(l.labels)
}

func (l *list) Label(index int) int {
	return l.labels[index]
}

func main() {
	l := &list{labels: []int{3}}
	valid := l.Valid(0)
	label := l.Label(0)
	fmt.Println(valid, label)
}
