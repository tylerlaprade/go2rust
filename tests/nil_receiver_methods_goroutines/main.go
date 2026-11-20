// The same program with a goroutine, so every value is behind a lock.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	Value int
	Next  *Node
}

func (n *Node) Len() int {
	if n == nil {
		return 0
	}
	return 1 + n.Next.Len()
}

func (n *Node) Sum() int {
	if n == nil {
		return 0
	}
	return n.Value + n.Next.Sum()
}

type Tree struct {
	Left, Right *Tree
	Val         int
}

func (t *Tree) Insert(v int) *Tree {
	if t == nil {
		return &Tree{Val: v}
	}
	if v < t.Val {
		t.Left = t.Left.Insert(v)
	} else {
		t.Right = t.Right.Insert(v)
	}
	return t
}

func (t *Tree) InOrder(visit func(int)) {
	if t == nil {
		return
	}
	t.Left.InOrder(visit)
	visit(t.Val)
	t.Right.InOrder(visit)
}

func (t *Tree) Height() int {
	if t == nil {
		return 0
	}
	l, r := t.Left.Height(), t.Right.Height()
	if l > r {
		return l + 1
	}
	return r + 1
}

func (t *Tree) Contains(v int) bool {
	for t != nil {
		if v == t.Val {
			return true
		}
		if v < t.Val {
			t = t.Left
		} else {
			t = t.Right
		}
	}
	return false
}

type List struct {
	Head *Elem
	size int
}

type Elem struct {
	Val  string
	Next *Elem
}

func (e *Elem) String() string {
	if e == nil {
		return "nil"
	}
	return e.Val + "->" + e.Next.String()
}

func (e *Elem) Append(v string) *Elem {
	if e == nil {
		return &Elem{Val: v}
	}
	e.Next = e.Next.Append(v)
	return e
}

func (l *List) Push(v string) {
	l.Head = l.Head.Append(v)
	l.size++
}

type Stringer interface{ String() string }

type Counter struct{ hits int }

func (c *Counter) Hit() int {
	if c == nil {
		return -1
	}
	c.hits++
	return c.hits
}

func describe(s Stringer) string { return "[" + s.String() + "]" }

func more() {
	var l List
	for _, w := range strings.Fields("a b c") {
		l.Push(w)
	}
	fmt.Println(l.Head.String(), l.size)
	var none *Elem
	fmt.Println(none.String(), describe(l.Head))
	c := &Counter{}
	c.Hit()
	c.Hit()
	var missing *Counter
	fmt.Println(c.Hit(), c.hits, missing.Hit())
	cs := []*Counter{c, nil, {}}
	for _, x := range cs {
		fmt.Print(x.Hit(), " ")
	}
	fmt.Println()
	byName := map[string]*Counter{"a": c}
	fmt.Println(byName["a"].Hit(), byName["zz"].Hit())
	done := make(chan int)
	go func() { done <- c.Hit() }()
	fmt.Println(<-done)
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started

	list := &Node{1, &Node{2, &Node{3, nil}}}
	fmt.Println(list.Len(), list.Sum())
	var empty *Node
	fmt.Println(empty.Len(), empty.Sum())

	var root *Tree
	for _, v := range []int{5, 3, 8, 1, 4, 9} {
		root = root.Insert(v)
	}
	var out []int
	root.InOrder(func(v int) { out = append(out, v) })
	fmt.Println(out, root.Height(), root.Contains(4), root.Contains(7))
	more()
}
