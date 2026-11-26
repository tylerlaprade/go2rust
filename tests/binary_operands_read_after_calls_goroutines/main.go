package main

import "fmt"

// Go calls the functions of an expression before it reads the plain
// operands, so an operand that a call changes is read after the call.

var counter int

func bumpCounter() int { counter += 5; return 1 }

func bump(s []int) int { s[0]++; return 1 }

func fill(s []int, v int) { s[1] = v }

type box struct{ n int }

func (b *box) inc() int { b.n++; return b.n }

func (b *box) total() int { return b.n + b.inc() }

func tag(p *string) string { *p += "!"; return "t" }

func addOne(m map[string]int) int { m["a"]++; return 0 }

func plain(i int) int { return i * 2 }

func twice(s string) string { return s + s }

func main() {
	// A goroutine makes every variable a locked one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

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
	fmt.Println("len of a call:", len(words[0])+len(twice(words[1])))

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
