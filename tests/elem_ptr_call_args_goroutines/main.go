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

func double(p *int) int {
	*p *= 2
	return *p
}

func main() {
	// A goroutine makes every variable a locked one.
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)
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
}
