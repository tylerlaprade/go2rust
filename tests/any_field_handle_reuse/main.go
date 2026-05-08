package main

import "fmt"

type entry struct {
	value any
}

func assign(e *entry, value any) {
	e.value = value
}

func each(e *entry, f func(any)) {
	f(e.value)
}

func main() {
	go func() {}()

	var value any = "ok"
	e := &entry{}
	assign(e, value)
	each(e, func(v any) {
		fmt.Println(v)
	})
}
