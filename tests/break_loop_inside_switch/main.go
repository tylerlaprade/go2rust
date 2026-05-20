package main

import "fmt"

func main() {
	switch true {
	case true:
		if false {
			break
		}
		for _, n := range []int{1, 2, 3} {
			if n == 2 {
				break
			}
			fmt.Println(n)
		}
		fmt.Println("after")
	}
	fmt.Println("done")
}
