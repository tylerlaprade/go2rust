package main

import "fmt"

func main() {
	view := make(map[string]int)
	var visit func(map[string]int)
	visit = func(pkgs map[string]int) {
		for id := range pkgs {
			view[id] = len(pkgs)
		}
		if len(pkgs) > 0 {
			visit(map[string]int{})
		}
	}

	visit(map[string]int{"pkg": 1})
	fmt.Println(len(view))
	fmt.Println(view["pkg"])
}
