package main

import "fmt"

func choose(ok bool) (int, string) {
	for {
		if ok {
			return 1, "ok"
		}
		return 0, "no"
	}
}

func main() {
	n, label := choose(true)
	fmt.Println(n, label)
}
