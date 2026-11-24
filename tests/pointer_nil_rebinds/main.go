// Assigning nil to a pointer variable, range variable, parameter or field
// changes only that pointer: the value it pointed to, and every other
// pointer to it, are left alone.
package main

import (
	"fmt"
	"sync"
)

type node struct {
	val  int
	next *node
}

type list struct{ head *node }

func (l *list) clear() { l.head = nil }

func drop(p *int) {
	p = nil
	_ = p
}

func main() {
	locals()
	linked()
}

func locals() {
	value := 1
	items := []*int{&value}
	for _, item := range items {
		item = nil
		fmt.Println(item == nil)
	}
	fmt.Println(items[0] == nil, *items[0])

	other := 2
	for i, item := range items {
		item = &other
		*item = 20
		fmt.Println(i, *item)
	}
	fmt.Println(*items[0], other)

	nodes := []*node{{val: 1}, {val: 2}}
	for _, n := range nodes {
		n.val *= 10
		n = &node{val: 99}
		n.val++
	}
	fmt.Println(nodes[0].val, nodes[1].val)

	m := map[string]*int{"a": &value}
	for k, p := range m {
		p = nil
		fmt.Println(k, p == nil)
	}
	fmt.Println(m["a"] == nil)

	var p *int = &value
	q := p
	q = nil
	fmt.Println(p == nil, q == nil)
}

func linked() {
	third := &node{val: 3}
	second := &node{val: 2, next: third}
	first := &node{val: 1, next: second}
	saved := first.next
	first.next = nil
	fmt.Println(first.next == nil, saved.val, second.next.val)

	l := &list{head: first}
	keep := l.head
	l.clear()
	fmt.Println(l.head == nil, keep.val)

	x := 5
	px := &x
	drop(px)
	fmt.Println(*px)

	var wg sync.WaitGroup
	shared := &node{val: 7}
	ptrs := []*node{shared, shared}
	wg.Add(1)
	go func() {
		defer wg.Done()
		for _, p := range ptrs {
			p = nil
			_ = p
		}
	}()
	wg.Wait()
	fmt.Println(ptrs[0].val, ptrs[1] == nil, shared.val)

	cur := first
	for cur != nil {
		fmt.Println("visit", cur.val)
		cur = cur.next
	}
	cur = second
	n := 0
	for cur != nil {
		n++
		cur = cur.next
	}
	fmt.Println(n, second.val)

	var pp *int
	pp = &x
	pp = nil
	fmt.Println(pp == nil, x)
}
