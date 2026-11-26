// A pointer to a slice or array element passed to a function that reads
// and writes through it: the write is seen in the element.
package main

import "fmt"

type P struct{ N int }

func tag(p *string) { *p += "!" }

func bump(p *int) { *p++ }

func inc(p *P) { p.N++ }

func set(p *int, v int) {
	if p != nil {
		*p = v
	}
}

func swap(a, b *int) { *a, *b = *b, *a }

// reset replaces the whole struct, so only the write-back shows it.
func reset(p *P) { *p = P{N: 9} }

// both writes through two pointers that can be one element.
func both(a, b *int) {
	*a = 5
	*b += 1
}

var calls []string

func first() int { calls = append(calls, "first"); return 0 }

func at(i int) int { calls = append(calls, "at"); return i }

func add(x int, p *int) { *p += x + 10 }

func double(p *int) int {
	*p *= 2
	return *p
}

func main() {
	words := []string{"ab", "cd"}
	tag(&words[0])
	nums := []int{1, 2}
	bump(&nums[1])
	set(&nums[0], 7)
	swap(&nums[0], &nums[1])
	ps := []P{{1}, {2}}
	inc(&ps[1])
	arr := [2]int{5, 6}
	bump(&arr[0])
	i := 1
	fmt.Println(double(&arr[i]), arr)
	fmt.Println(words, nums, ps, arr)
	reset(&ps[0])
	one := []int{1, 2}
	both(&one[0], &one[0])
	both(&one[1], &one[0])
	add(first(), &one[at(1)])
	fmt.Println(ps, one, calls)
}
