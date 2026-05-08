package main

import "fmt"

type Free struct {
	seen map[string]bool
}

func (f *Free) Has(key string) (res bool) {
	if f.seen == nil {
		f.seen = make(map[string]bool)
	}
	defer func() {
		f.seen[key] = res
	}()
	res = true
	return
}

func main() {
	var f Free
	has := f.Has("x")
	seen := f.seen["x"]
	fmt.Println(has, seen)
}
