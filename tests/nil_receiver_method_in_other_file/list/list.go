package list

// List is a linked list whose methods accept a nil *List, as an empty list.
type List struct {
	Val  int
	Next *List
}

// Len is written for a nil receiver.
func (l *List) Len() int {
	if l == nil {
		return 0
	}
	return 1 + l.Next.Len()
}

// Or gives l, or def when l is nil.
func (l *List) Or(def *List) *List {
	if l == nil {
		return def
	}
	return l
}
