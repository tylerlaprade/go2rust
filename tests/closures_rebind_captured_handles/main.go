// A pointer, map or channel variable that a closure assigns, or that is
// assigned after a closure that uses it is made. The variable is its handle,
// and the closure held a copy of it: an assignment inside the closure did
// not reach the variable, and the closure did not see a later one. Each such
// variable is now given a cell that the closures share.
package main

import (
	"errors"
	"fmt"
	"iter"
	"sort"
)

type Node struct {
	V    int
	Name string
	Next *Node
	Kids []*Node
}

func walk(n *Node, f func(*Node)) {
	f(n)
	for _, k := range n.Kids {
		walk(k, f)
	}
}

// find records the node a callback sees.
func find(root *Node, name string) (*Node, error) {
	var found *Node
	walk(root, func(n *Node) {
		if n.Name == name {
			found = n
		}
	})
	if found == nil {
		return nil, errors.New("not found: " + name)
	}
	return found, nil
}

// lazy builds its map on the first call only.
func lazy() func() map[string]int {
	var cache map[string]int
	return func() map[string]int {
		if cache == nil {
			fmt.Println("building")
			cache = map[string]int{"a": 1}
		}
		return cache
	}
}

// recovered sets a pointer from a deferred closure.
func recovered() (n *Node) {
	var last *Node
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("recovered", r, last.V)
		}
	}()
	last = &Node{V: 41}
	panic("boom")
}

func nodes(vs ...int) iter.Seq[*Node] {
	return func(yield func(*Node) bool) {
		for _, v := range vs {
			if !yield(&Node{V: v}) {
				return
			}
		}
	}
}

func maps() iter.Seq[map[string]int] {
	return func(yield func(map[string]int) bool) {
		if !yield(map[string]int{"a": 1}) {
			return
		}
		yield(map[string]int{"b": 2})
	}
}

// firstMap returns nil from the body of a range over a function.
func firstMap(stop bool) map[string]int {
	for m := range maps() {
		if stop {
			return nil
		}
		return m
	}
	return map[string]int{}
}

func main() {
	var head *Node
	push := func(v int) { head = &Node{V: v, Next: head} }
	push(1)
	push(2)
	push(3)
	for n := head; n != nil; n = n.Next {
		fmt.Print(n.V, " ")
	}
	fmt.Println()

	var m map[string]int
	initM := func() { m = map[string]int{"a": 1} }
	initM()
	m["b"] = 2
	fmt.Println(m, len(m))

	var p *int
	x := 5
	func() { p = &x }()
	*p = 6
	fmt.Println(x, *p)

	var ch chan int
	fmt.Println(ch == nil)
	func() { ch = make(chan int, 1) }()
	ch <- 1
	fmt.Println(<-ch, ch == nil)

	root := &Node{Name: "root", Kids: []*Node{{Name: "a"}, {Name: "b", Kids: []*Node{{Name: "c", V: 7}}}}}
	c, err := find(root, "c")
	fmt.Println(c.V, err)
	_, err = find(root, "zz")
	fmt.Println(err)

	get := lazy()
	fmt.Println(get()["a"], len(get()))

	list := []*Node{{V: 3}, {V: 9}, {V: 1}}
	var best *Node
	sort.Slice(list, func(i, j int) bool {
		if best == nil || list[i].V > best.V {
			best = list[i]
		}
		return list[i].V < list[j].V
	})
	fmt.Println(best.V, list[0].V)

	// The closures share cur, which the loop assigns after each is made.
	var cur *Node
	var fs []func() int
	for i := 0; i < 3; i++ {
		cur = &Node{V: i * 10}
		fs = append(fs, func() int { return cur.V })
	}
	for _, f := range fs {
		fmt.Print(f(), " ")
	}
	fmt.Println()

	fmt.Println(recovered() == nil)

	// A range over a function: its body is the yield function.
	var q *Node
	for q = range nodes(1, 2, 3) {
		q = nil
		break
	}
	fmt.Println(q == nil)
	var last *Node
	for n := range nodes(7, 8, 9) {
		if n.V > 7 {
			last = n
		}
	}
	fmt.Println(last.V)
	var lm map[string]int
	for lm = range maps() {
	}
	fmt.Println(lm)
	fmt.Println(firstMap(true) == nil, firstMap(false))

	var done *Node
	defer func() { fmt.Println("done", done.V) }()
	done = &Node{V: 42}
}
