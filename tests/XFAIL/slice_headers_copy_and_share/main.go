package main

import "fmt"

// A Go slice is a header (offset, length, capacity) over a backing array.
// Assigning or passing a slice copies the header: writes to the elements are
// shared, but append, reslicing and nil change only the copy. The default
// lowering keeps one Vec per slice value, so each half of that is right in
// some operations and wrong in others (DEV-36). GO2RUST_GOSLICE=1 makes all
// of it right; this fixture passes there.

func grow(xs []int) {
	xs = append(xs, 9)
	xs[0] = 100
}

func reset(xs []int) {
	xs = nil
	_ = xs
}

func push(xs []int, v int) []int {
	xs = append(xs, v)
	return xs
}

func reverse(xs []int) {
	for i, j := 0, len(xs)-1; i < j; i, j = i+1, j-1 {
		xs[i], xs[j] = xs[j], xs[i]
	}
}

type bag struct{ items []int }

func (b *bag) all() []int { return b.items }

func describe(v interface{}) {
	if s, ok := v.([]int); ok {
		s[0] = -s[0]
		s = append(s, 7)
		fmt.Println("asserted:", s)
	}
}

func main() {
	qs := make([]int, 1, 4)
	grow(qs)
	fmt.Println("callee append:", len(qs), qs[0], qs[:2])

	zs := []int{1, 2}
	reset(zs)
	fmt.Println("callee nil:", len(zs))

	base := []int{1, 2, 3}
	a := push(base[:1], 7)
	fmt.Println("append within capacity:", base, a)

	ys := base
	ys = append(ys, 4)
	ys[0] = -1
	fmt.Println("alias then append:", base, ys)

	view := base[1:]
	view[0] = 99
	fmt.Println("sub-slice write:", base, view)

	b2 := make([]int, 2, 4)
	b2[0], b2[1] = 1, 2
	v2 := b2[1:2]
	v2[0] = 9
	grown := append(v2, 5)
	fmt.Println("lengths:", len(b2), len(v2), len(grown), b2[1], b2[:3][2])
	detached := append(b2[:2:2], 7)
	detached[0] = 8
	fmt.Println("full slice expression:", b2[0], detached[0])

	nums := []int{1, 2, 3, 4, 5, 6}
	reverse(nums[1:5])
	fmt.Println("in place on a sub-slice:", nums)

	grid := [][]int{{1, 2}, {3, 4}}
	row := grid[0]
	row[1] = 9
	fmt.Println("row of a grid:", grid)

	var log []string
	add := func(s string) { log = append(log, s) }
	add("a")
	add("b")
	fmt.Println("closure append:", log)

	bg := bag{items: base}
	base = append(base, 5)
	base = nil
	it := bg.all()
	it = append(it, 6)
	fmt.Println("struct field copy:", bg.items, it, base == nil)

	m := map[string][]int{}
	ms := []int{7}
	m["k"] = ms
	ms = nil
	fmt.Println("map value copy:", m, ms == nil)

	src := []int{5, 6}
	describe(src)
	fmt.Println("after assertion:", src)
}
