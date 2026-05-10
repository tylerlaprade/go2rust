package main

import "fmt"

type reader struct{}

func (r *reader) pick(v int) (res int) {
	if v < 0 {
		defer func() {}()
	}
	switch v {
	default:
		return 1
	case 0:
		return 0
	}
}

func main() {
	r := &reader{}
	fmt.Println(r.pick(0))
	fmt.Println(r.pick(2))
}
