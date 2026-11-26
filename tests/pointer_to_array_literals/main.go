// A pointer to an array, from &[N]T{...}, new([N]T) or &arr: indexing,
// len and range go through it, a function, a field and a return value pass
// it on, and *p copies the array.
package main

import "fmt"

// A defined array type keeps its newtype behind a pointer.
type A [2]int

type Big [40]int

func set(p *A) { p[1] = 9 }

type Grid struct {
	cells *[4]int
}

func fill(p *[4]int, v int) {
	for i := range p {
		p[i] = v + i
	}
}

func makeArr() *[4]int {
	return &[4]int{9, 8, 7, 6}
}

func sum(p *[4]int) int {
	t := 0
	for _, v := range p {
		t += v
	}
	return t
}

func main() {
	a := &[4]int{}
	fill(a, 10)
	fmt.Println(*a, len(a), sum(a))
	b := makeArr()
	b[0] = 1
	fmt.Println(b[0], *b, sum(b))
	g := Grid{cells: &[4]int{1, 1, 1, 1}}
	g.cells[2] = 5
	fmt.Println(*g.cells, sum(g.cells))
	c := new([4]int)
	c[3] = 3
	fmt.Println(*c)
	var d *[4]int
	fmt.Println(d == nil)
	d = a
	d[0] = 100
	fmt.Println(a[0], d[0])
	e := *a
	e[1] = -1
	fmt.Println(a[1], e[1])
	da := &A{1, 2}
	set(da)
	fmt.Println(da[0], da[1], *da)
	na := new(A)
	na[0] = 3
	big := new(Big)
	big[39] = 1
	wide := new([40]int)
	wide[0] = 2
	fmt.Println(*na, big[39], wide[0], len(wide))
}
