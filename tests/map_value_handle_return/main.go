package main

import "fmt"

type Method struct {
	Name string
}

var fallback []*Method

func lookup(methods map[string][]*Method, receiver string) []*Method {
	if receiver != "" && len(methods[receiver]) > 0 {
		return methods[receiver]
	}
	return fallback
}

func main() {
	first := &Method{Name: "first"}
	second := &Method{Name: "second"}
	methods := map[string][]*Method{
		"Thing": {first, second},
	}
	fallback = []*Method{{Name: "fallback"}}

	got := lookup(methods, "Thing")
	emptyReceiver := lookup(methods, "")
	fmt.Println(len(got), len(emptyReceiver))
}
