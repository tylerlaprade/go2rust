package main

import "fmt"

// The Go specification leaves the order of a plain operand's read and a call
// in the same expression unspecified; these cases pin what gc does. gc runs
// the calls of an expression before it reads the plain operands, so an
// operand that a call changes is read after the call. A builtin call such as
// len or min, and x && b, are evaluated in order with the calls.

var counter int

func bumpCounter() int { counter += 5; return 1 }

var label = "a"

func relabel() string { label = "b"; return "c" }

type record struct {
	n int
	s string
}

func newRecord() record {
	counter += 100
	label = "d"
	return record{n: 3, s: "e"}
}

func (r record) twice() int { return r.n * 2 }

func bump(s []int) int { s[0]++; return 1 }

func fill(s []int, v int) { s[1] = v }

type box struct{ n int }

func (b *box) inc() int { b.n++; return b.n }

func (b *box) total() int { return b.n + b.inc() }

func tag(p *string) string { *p += "!"; return "t" }

func addOne(m map[string]int) int { m["a"]++; return 0 }

func plain(i int) int { return i * 2 }

func twice(s string) string { return s + s }

var flag bool

func raise() bool { flag = true; return true }

var text = "a"

func grow() int { text = "abcd"; return 0 }

var spare = make([]int, 1, 4)

func regrow() int { spare = make([]int, 1, 9); return 0 }

func main() {
	s := []int{1, 2}
	x := s[0] + bump(s)
	fmt.Println("slice element:", x, s)
	fill(s, s[0]+bump(s))
	fmt.Println("in an argument:", s)
	y := s[0]*10 + bump(s) - s[0]
	fmt.Println("several operands:", y, s)
	fmt.Println("comparison:", s[0] == bump(s), s)

	b := &box{n: 5}
	z := b.n + b.inc()
	fmt.Println("field and pointer method:", z, b.n)
	fmt.Println("in a method:", b.total(), b.n)

	n := 3
	pn := &n
	w := n + func() int { *pn = 10; return 1 }()
	fmt.Println("address taken:", w, n)

	c := 0
	next := func() int { c++; return c }
	fmt.Println("captured by a closure:", c+next(), c)

	fmt.Println("package variable:", counter+bumpCounter(), counter)
	fmt.Println("parenthesized:", label+(relabel()), label)
	fmt.Println("field of a call:", counter+newRecord().n)
	fmt.Println("string field of a call:", label+newRecord().s)
	fmt.Println("method of a call's result:", counter+newRecord().twice())

	str := "a"
	t := str + tag(&str)
	fmt.Println("string:", t, str)

	m := map[string]int{"a": 1}
	fmt.Println("map element:", m["a"]+addOne(m), m["a"])

	f := 1.5
	pf := &f
	g := f * func() float64 { *pf = 2; return 3 }()
	fmt.Println("float:", g, f)

	words := []string{"ab", "cde"}
	fmt.Println("len of a call:", counter+len(twice(words[1])))

	yes := true
	fmt.Println("&& on the left:", (yes && flag) == raise())
	fmt.Println("len and cap on the left:", len(text)+grow(), cap(spare)+regrow())
	low := 2
	pl := &low
	fmt.Println("min on the left:", min(low, 5)+func() int { *pl = 0; return 0 }())
	fmt.Println("negation on the left:", -low+func() int { *pl = 7; return 0 }())
	dst := []int{1}
	fmt.Println("copy on the right:", dst[0]+copy(dst, []int{5}), dst)

	total := 0
	for i := 0; i < 3; i++ {
		total = total + plain(i)
	}
	fmt.Println("plain local:", total)

	if s[0] > bump(s) {
		fmt.Println("condition: before", s)
	} else {
		fmt.Println("condition: after", s)
	}
}
