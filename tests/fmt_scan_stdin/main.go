package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var n int
	fmt.Scan(&n)
	nums := make([]int, n)
	for i := range nums {
		fmt.Scan(&nums[i])
	}
	var name string
	var score float64
	cnt, err := fmt.Scanln(&name, &score)
	fmt.Println(n, nums, name, score, cnt, err)
	var a, b int
	k, err := fmt.Scanf("%d,%d\n", &a, &b)
	fmt.Println(a+b, k, err)
	in := bufio.NewReader(os.Stdin)
	var w1, w2 string
	fmt.Fscan(in, &w1, &w2)
	var x int
	k, err = fmt.Fscan(in, &x)
	fmt.Println(w1, w2, x, k, err)
	k, err = fmt.Fscan(os.Stdin, &x)
	fmt.Println(k, err)
}
