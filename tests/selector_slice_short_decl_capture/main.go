package main

import "fmt"

type dict struct {
	tparams []int
}

type reader struct {
	dict  *dict
	later []func()
}

func (r *reader) collect(vals []int) {
	tparams := r.dict.tparams
	r.later = append(r.later, func() {
		for i, val := range vals {
			tparams[i] = val
		}
	})
}

func main() {
	r := &reader{
		dict:  &dict{tparams: []int{1, 2}},
		later: []func(){},
	}
	vals := []int{3, 4}
	r.collect(vals)
	r.later[0]()
	fmt.Println(r.dict.tparams[0], r.dict.tparams[1])
}
