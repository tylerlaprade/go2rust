package main

import "fmt"

type item struct {
	flag bool
}

func (x *item) label() string {
	switch {
	case x.flag:
		return "on"
	default:
		return "off"
	}
}

func (x *item) either(y *item) bool {
	return x.flag || !y.flag
}

func (x *item) active() bool {
	if x.flag {
		return true
	}
	return false
}

func main() {
	on := &item{flag: true}
	off := &item{}
	fmt.Println(on.label())
	fmt.Println(off.label())
	fmt.Println(on.either(off))
	fmt.Println(off.active())
}
