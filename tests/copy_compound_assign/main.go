package main

import "fmt"

func main() {
	dst := make([]int, 6)
	src1 := []int{1, 2, 3}
	src2 := []int{4, 5, 6}

	i := 0
	i += copy(dst[i:], src1)
	i += copy(dst[i:], src2)

	fmt.Println(i, dst)
}
