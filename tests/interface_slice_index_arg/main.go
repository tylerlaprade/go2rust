package main

import "fmt"

type Spec interface {
	Label() string
}

type ImportSpec struct {
	name string
}

func (s ImportSpec) Label() string { return s.name }

func pairOK(prev, next Spec) bool {
	return prev.Label() != next.Label()
}

func main() {
	specs := []Spec{ImportSpec{name: "a"}, ImportSpec{name: "b"}, ImportSpec{name: "b"}}
	for i, s := range specs {
		if i == len(specs)-1 {
			continue
		}
		fmt.Println(pairOK(s, specs[i+1]))
	}
}
