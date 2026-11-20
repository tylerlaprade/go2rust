// Generic containers: a linked List[T] of *Node[T], a binary Tree[T] that
// calls its methods on nil subtrees, an Optional[T] with Some and None,
// and a type switch on any(v) in a generic function.
package main

import (
	"cmp"
	"fmt"
)

type Node[T any] struct {
	Val  T
	Next *Node[T]
}

type List[T any] struct {
	head *Node[T]
	size int
}

func (l *List[T]) Push(v T) {
	l.head = &Node[T]{Val: v, Next: l.head}
	l.size++
}

func (l *List[T]) Each(f func(T)) {
	for n := l.head; n != nil; n = n.Next {
		f(n.Val)
	}
}

func (l *List[T]) Slice() []T {
	var out []T
	l.Each(func(v T) { out = append(out, v) })
	return out
}

type Tree[T cmp.Ordered] struct {
	Left, Right *Tree[T]
	Val         T
}

func (t *Tree[T]) Insert(v T) *Tree[T] {
	if t == nil {
		return &Tree[T]{Val: v}
	}
	if v < t.Val {
		t.Left = t.Left.Insert(v)
	} else {
		t.Right = t.Right.Insert(v)
	}
	return t
}

func (t *Tree[T]) Walk(visit func(T)) {
	if t == nil {
		return
	}
	t.Left.Walk(visit)
	visit(t.Val)
	t.Right.Walk(visit)
}

type Optional[T any] struct {
	val T
	ok  bool
}

func Some[T any](v T) Optional[T] { return Optional[T]{v, true} }
func None[T any]() Optional[T]    { return Optional[T]{} }

func (o Optional[T]) Get() (T, bool) { return o.val, o.ok }
func (o Optional[T]) OrElse(d T) T {
	if o.ok {
		return o.val
	}
	return d
}

func Describe[T any](v T) string {
	switch x := any(v).(type) {
	case int:
		return fmt.Sprintf("int %d", x)
	case string:
		return fmt.Sprintf("string %q", x)
	default:
		return fmt.Sprintf("other %v", x)
	}
}

func main() {
	var l List[string]
	l.Push("c")
	l.Push("b")
	l.Push("a")
	fmt.Println(l.Slice(), l.size)
	nums := &List[int]{}
	for i := 0; i < 4; i++ {
		nums.Push(i * i)
	}
	sum := 0
	nums.Each(func(n int) { sum += n })
	fmt.Println(sum, nums.Slice())

	var root *Tree[int]
	for _, v := range []int{5, 3, 8, 1, 4} {
		root = root.Insert(v)
	}
	root.Walk(func(v int) { fmt.Print(v, " ") })
	fmt.Println()
	var words *Tree[string]
	for _, w := range []string{"pear", "apple", "fig"} {
		words = words.Insert(w)
	}
	words.Walk(func(w string) { fmt.Print(w, " ") })
	fmt.Println()

	a := Some(42)
	b := None[string]()
	v, ok := a.Get()
	fmt.Println(v, ok, b.OrElse("default"), a.OrElse(0))
	fmt.Println(Describe(7), Describe("hi"), Describe(2.5))
}
