// var p *T = q, for a pointer field, a package-level pointer and a slice
// element: p shares q's pointee, as p := q does, so a write through one is
// seen through the other.
package main

import "fmt"

type Node struct {
	v    int
	next *Node
}

var head = &Node{v: 1, next: &Node{v: 2}}

func main() {
	n := head
	var p *Node = n.next
	q := n.next
	var r *Node = head
	p.v = 20
	r.v = 10
	fmt.Println(p.v, q.v, r.next.v, n.next.v, head.v)
	nodes := []*Node{p, r}
	var first *Node = nodes[0]
	first.v = 200
	var second *Node = (nodes[1])
	second.next = nil
	fmt.Println(p.v, q.v, head.next == nil, p == first, r == second)
	var cur *Node = r
	count := 0
	for cur != nil {
		count++
		cur = cur.next
	}
	fmt.Println(count)
}
