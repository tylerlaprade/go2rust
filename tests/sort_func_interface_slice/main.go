package main

import (
	"fmt"
	"slices"
)

type Spec interface {
	Name() string
}

type Impl struct {
	name string
}

func (i Impl) Name() string { return i.name }

func getName(s Spec) string {
	return s.Name()
}

func cmpStrings(a, b string) int {
	if a < b {
		return -1
	}
	if a > b {
		return 1
	}
	return 0
}

func main() {
	specs := []Spec{Impl{name: "banana"}, Impl{name: "apple"}, Impl{name: "cherry"}}
	slices.SortFunc(specs, func(a, b Spec) int {
		return cmpStrings(getName(a), getName(b))
	})
	for _, s := range specs {
		fmt.Println(s.Name())
	}
}
