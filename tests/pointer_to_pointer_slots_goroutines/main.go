package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

// insert walks to the nil link where v belongs and rebinds it.
func insert(pp **Node, v int) {
	for *pp != nil {
		if v < (*pp).Val {
			pp = &(*pp).Left
		} else {
			pp = &(*pp).Right
		}
	}
	*pp = &Node{Val: v}
}

func inorder(n *Node, out *[]int) {
	if n == nil {
		return
	}
	inorder(n.Left, out)
	*out = append(*out, n.Val)
	inorder(n.Right, out)
}

// minLink returns the link that holds the smallest node.
func minLink(pp **Node) **Node {
	for (*pp).Left != nil {
		pp = &(*pp).Left
	}
	return pp
}

type Tree struct{ root *Node }

type LNode struct {
	v    int
	next *LNode
}

type List struct{ head *LNode }

// remove unlinks the first node holding v.
func remove(l *List, v int) bool {
	for pp := &l.head; *pp != nil; pp = &(*pp).next {
		if (*pp).v == v {
			*pp = (*pp).next
			return true
		}
	}
	return false
}

// push puts a node in front: the new node points at the old head.
func push(pp **LNode, v int) { *pp = &LNode{v, *pp} }

func (l *List) String() string {
	s := "["
	for n := l.head; n != nil; n = n.next {
		if n != l.head {
			s += " "
		}
		s += fmt.Sprint(n.v)
	}
	return s + "]"
}

type Holder struct {
	slot **LNode
}

func addCount(m *map[string]int, k string) {
	if *m == nil {
		*m = map[string]int{}
	}
	(*m)[k]++
}

func main() {
	// A goroutine makes the program use Arc and Mutex.
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	var root *Node
	for _, v := range []int{5, 3, 8, 1, 4} {
		insert(&root, v)
	}
	var vals []int
	inorder(root, &vals)
	fmt.Println(vals)
	low := minLink(&root)
	fmt.Println("min", (*low).Val)
	*low = (*low).Right
	vals = nil
	inorder(root, &vals)
	fmt.Println("without min", vals)

	t := &Tree{}
	for _, v := range []int{2, 1, 3} {
		insert(&t.root, v)
	}
	vals = nil
	inorder(t.root, &vals)
	fmt.Println(vals, t.root.Val)

	l := &List{}
	for i := 1; i <= 4; i++ {
		push(&l.head, i)
	}
	fmt.Println(l)
	fmt.Println(remove(l, 3), remove(l, 4), remove(l, 9), l)

	var head *LNode
	push(&head, 1)
	push(&head, 2)
	fmt.Println(&List{head})

	arr := [2]*LNode{}
	push(&arr[0], 7)
	push(&arr[0], 8)
	fmt.Println(&List{arr[0]}, arr[1] == nil)

	links := []*LNode{nil, nil}
	push(&links[1], 6)
	fmt.Println(links[0] == nil, links[1].v)
	// The slot of &links[0] keeps the element it was taken from, after
	// links is given another slice.
	first := &links[0]
	old := links
	links = []*LNode{{v: 2}, nil}
	push(first, 5)
	fmt.Println(old[0].v, links[0].v)

	h := Holder{slot: &head}
	push(h.slot, 3)
	fmt.Println(&List{head}, h.slot != nil)

	var empty Holder
	fmt.Println(empty.slot == nil)

	pp := new(*LNode)
	push(pp, 9)
	fmt.Println((*pp).v, (*pp).next == nil)

	var counts map[string]int
	addCount(&counts, "a")
	addCount(&counts, "a")
	addCount(&counts, "b")
	fmt.Println(counts["a"], counts["b"], len(counts))
}
