package list

// Sum is in another file than the methods it calls on a nil *List.
func Sum(l *List) int {
	total := 0
	for n := l.Len(); n > 0; n-- {
		total += l.Val
		l = l.Next
	}
	return total + l.Or(&List{Val: 100}).Val
}

// Push puts v in front of l, which may be nil.
func Push(l *List, v int) *List {
	return &List{Val: v, Next: l}
}
