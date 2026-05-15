package main

import "fmt"

type recorder struct{}

func (recorder) Use(record func(string)) {
	record("method")
}

func relay(record func(string)) {
	var r recorder
	r.Use(record)
}

func main() {
	out := ""
	relay(func(s string) {
		out = s
	})
	fmt.Println(out)
}
