package main

import "fmt"

func run(fn func()) {
	fn()
}

func main() {
	var v any = 7

	f := func() {
		run(func() {
			_, ok := v.(int)
			fmt.Println(ok)
		})
		_, ok := v.(int)
		fmt.Println(ok)
	}

	f()
}
