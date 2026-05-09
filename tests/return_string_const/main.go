package main

import "fmt"

const Future = ""
const Greeting = "hello"

func fallback(ok bool) string {
	if ok {
		return "ok"
	}
	return Future
}

func middle() string {
	return Greeting[1:4]
}

func main() {
	fmt.Println(fallback(true))
	fmt.Println(fallback(false))
	fmt.Println(middle())
}
