package main

import "fmt"

type entry struct {
	value any
}

func get(e entry) any {
	return e.value
}

func main() {
	go func() {}()
	e := entry{value: "ok"}
	v := get(e)
	if _, ok := v.(string); ok {
		fmt.Println("ok")
	}
}
