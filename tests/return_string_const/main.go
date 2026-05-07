package main

import "fmt"

const Future = ""

func fallback(ok bool) string {
	if ok {
		return "ok"
	}
	return Future
}

func main() {
	fmt.Println(fallback(true))
	fmt.Println(fallback(false))
}
