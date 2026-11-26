// gc runs every call among a print's operands before it reads the plain
// operands and formats them, so an operand that a call changes prints as
// the call left it. The Go specification leaves this order unspecified;
// these cases pin what gc does.
package main

import (
	"fmt"
	"os"
)

// del removes s[i:j] the way slices.Delete does: it moves the tail down and
// clears what is left at the end of the backing array.
func del(s []int, i, j int) []int {
	n := copy(s[i:], s[j:])
	for k := i + n; k < len(s); k++ {
		s[k] = 0
	}
	return s[:i+n]
}

type box struct{ n int }

func (b *box) inc() int { b.n++; return b.n }

func main() {
	// A goroutine makes every variable a locked one.
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)
	shapes()
	ins := []int{1, 2, 3, 4}
	fmt.Println(ins, del(ins, 0, 2))
	b := &box{}
	fmt.Println(b.n, b.inc(), b.n, b.inc())
	fmt.Printf("%d %d %v\n", b.n, b.inc(), *b)
	s := fmt.Sprintf("%d-%d", b.n, b.inc())
	fmt.Println(s)
	err := fmt.Errorf("at %d: %d", b.n, b.inc())
	fmt.Println(err)
	fmt.Fprintln(os.Stdout, b.n, b.inc())
	t := fmt.Sprint(b.n, " ", b.inc())
	fmt.Println(t, fmt.Sprintln(b.n, b.inc()) == "9 9\n")
	m := map[string]int{"a": 1}
	grow := func() int { m["b"] = 2; return len(m) }
	fmt.Println(m, grow())
	ch := make(chan int, 1)
	ch <- 5
	n := 1
	pn := &n
	fmt.Println(n, <-ch, func() int { *pn = 2; return 0 }())
}

type pt struct{ x int }

func shapes() {
	x := 1
	px := &x
	f := func() int { *px = 9; return 0 }
	fmt.Println(x, f())
	s := []int{1, 2}
	g := func() int { s[0] = 7; return 0 }
	fmt.Println(s, s[0], g())
	p := pt{1}
	pp := &p
	h := func() int { pp.x = 5; return 0 }
	fmt.Println(p, p.x, h())
	m := map[string]int{"a": 1}
	k := func() int { m["a"] = 3; return 0 }
	fmt.Println(m, m["a"], k())
	str := "a"
	ps := &str
	j := func() string { *ps = "b"; return "c" }
	fmt.Println(str, j(), str+"!")
	fmt.Printf("%d %d\n", x, f())
	y := fmt.Sprint(x, f())
	fmt.Println(y)
}
