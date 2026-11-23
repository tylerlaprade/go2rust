// Flags built with iota and shifts, float constants computed from integer
// shifts, package variables set before init functions run, variadic calls,
// a recursive closure, bytes and runes of a string, goto, and a struct type
// declared inside a function and compared with ==.
package main

import "fmt"

type Perm uint8

const (
	Read Perm = 1 << iota
	Write
	Exec
	All = Read | Write | Exec
)

func (p Perm) Has(q Perm) bool { return p&q == q }

func (p Perm) String() string {
	s := ""
	for _, x := range []struct {
		bit Perm
		ch  byte
	}{{Read, 'r'}, {Write, 'w'}, {Exec, 'x'}} {
		if p.Has(x.bit) {
			s += string(x.ch)
		} else {
			s += "-"
		}
	}
	return s
}

type Size float64

const (
	_       = iota
	KB Size = 1 << (10 * iota)
	MB
	GB
)

var registry = buildRegistry()
var order []string

func buildRegistry() map[string]int {
	order = append(order, "registry")
	return map[string]int{"a": 1}
}

func init() {
	order = append(order, "init1")
	registry["b"] = 2
}

func init() {
	order = append(order, "init2")
}

func sum(nums ...int) int {
	t := 0
	for _, n := range nums {
		t += n
	}
	return t
}

func apply(fs ...func(int) int) func(int) int {
	return func(x int) int {
		for _, f := range fs {
			x = f(x)
		}
		return x
	}
}

func main() {
	fmt.Println(Read, Write, Exec, All, Perm(5), All.Has(Write), Perm(4).Has(Read))
	fmt.Println(KB, MB/KB, GB/MB, int64(GB))
	fmt.Println(order, len(registry), registry["b"])
	xs := []int{1, 2, 3}
	fmt.Println(sum(), sum(1), sum(xs...), sum(append(xs, 4)...))
	inc := func(x int) int { return x + 1 }
	dbl := func(x int) int { return x * 2 }
	fmt.Println(apply(inc, dbl, inc)(5), apply()(7))
	var fact func(int) int
	fact = func(n int) int {
		if n <= 1 {
			return 1
		}
		return n * fact(n-1)
	}
	fmt.Println(fact(10))
	s := "héllo, 世界"
	bytes, runes := 0, 0
	for range s {
		runes++
	}
	for i := 0; i < len(s); i++ {
		bytes++
	}
	fmt.Println(bytes, runes, s[1], string(s[7:]), []rune(s)[1])
	for i, r := range "aé" {
		fmt.Print(i, ":", string(r), " ")
	}
	fmt.Println()
	i := 0
loop:
	if i < 3 {
		fmt.Print("g", i, " ")
		i++
		goto loop
	}
	fmt.Println()
	type local struct{ a, b int }
	l := local{1, 2}
	l2 := l
	l2.a = 5
	fmt.Println(l, l2, l == local{1, 2})
}
