// A linked list whose nodes live in a slice or an array: s[i].Next = &s[j]
// writes the element in place, and a loop pointer starts at &s[0] and
// follows Next.
package main

import "fmt"

type Node struct {
	Val  int
	Next *Node
}

type List struct {
	nodes []Node
	head  *Node
}

func (l *List) Link() {
	for i := range l.nodes {
		if i+1 < len(l.nodes) {
			l.nodes[i].Next = &l.nodes[i+1]
		}
	}
	l.head = &l.nodes[0]
}

func (l *List) Sum() int {
	total := 0
	for n := l.head; n != nil; n = n.Next {
		total += n.Val
	}
	return total
}

func main() {
	nodes := []Node{{}, {}, {}}
	for i := range nodes {
		nodes[i].Val = i * 10
		if i > 0 {
			nodes[i-1].Next = &nodes[i]
		}
	}
	nodes[2].Val = 99
	for n := &nodes[0]; n != nil; n = n.Next {
		fmt.Print(n.Val, " ")
	}
	fmt.Println()
	fmt.Println(nodes[2].Next == nil, nodes[0].Next == &nodes[1])

	var arr [3]Node
	for i := 0; i < 2; i++ {
		arr[i].Next = &arr[i+1]
	}
	arr[2].Val = 7
	fmt.Println(arr[0].Next.Next.Val)

	l := &List{nodes: []Node{{Val: 1}, {Val: 2}, {Val: 3}}}
	l.Link()
	l.nodes[1].Val = 20
	fmt.Println(l.Sum())
	l.nodes[len(l.nodes)-1].Next = &l.nodes[0]
	fmt.Println(l.nodes[2].Next.Val)
	orderAndRecover()
}

// The index on the left is evaluated before the value on the right, and an
// index out of range panics without leaving the slice unusable.
func orderAndRecover() {
	nodes := []Node{{Val: 1}, {Val: 2}}
	var order []string
	at := func(name string, i int) int {
		order = append(order, name)
		return i
	}
	nodes[at("lhs", 0)].Next = &nodes[at("rhs", 1)]
	fmt.Println(order, nodes[0].Next.Val)
	func() {
		defer func() { fmt.Println("recovered:", recover()) }()
		nodes[at("bad", 5)].Next = &nodes[0]
	}()
	nodes[1].Next = &nodes[0]
	fmt.Println(order, nodes[1].Next.Val, nodes[0].Next.Next.Val)
}
