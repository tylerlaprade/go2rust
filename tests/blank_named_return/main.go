package main

import "fmt"

func f() (_ int, s string) {
	s = "ok"
	return
}

func main() {
	_, s := f()
	fmt.Println(s)
}
