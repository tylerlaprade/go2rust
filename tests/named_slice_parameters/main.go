package main

import "fmt"

type Numbers []int

func (ns Numbers) String() string {
	return fmt.Sprintf("Numbers(%d)", len(ns))
}

func total(ns Numbers) int {
	sum := 0
	for _, n := range ns {
		sum += n
	}
	return sum
}

func grow(ns Numbers) Numbers {
	ns = append(ns, 4)
	return ns
}

func merge(a, b Numbers) Numbers {
	return append(a, b...)
}

func main() {
	nums := Numbers{1, 2, 3}
	more := Numbers{5, 6}
	grown := grow(nums)
	merged := merge(nums, more)
	fmt.Println(total(nums), total(grown), len(merged), merged[4], nums.String())
}
