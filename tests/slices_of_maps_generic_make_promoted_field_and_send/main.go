package main

import "fmt"

type Inner struct{ M map[string]int }

type Row struct {
	Inner
	N int
}

func alloc[S ~[]map[string]int]() S { return make(S, 2) }

type Tables []map[string]int

func main() {
	rows := []Row{{}}
	m := map[string]int{"k": 1}
	rows[0].M = m
	m["x"] = 2
	fmt.Println(len(rows[0].M))
	rows[0].M = nil
	fmt.Println(rows[0].M == nil, len(m))

	t := alloc[Tables]()
	fmt.Println(len(t), t[0] == nil)
	u := alloc[[]map[string]int]()
	u[1] = map[string]int{"a": 1}
	fmt.Println(len(u[1]))

	ch := make(chan map[string]int, 2)
	s := []map[string]int{{"a": 1}, {"b": 2}}
	for _, v := range s {
		ch <- v
	}
	fmt.Println(len(<-ch), len(<-ch))
	mm := map[string]map[string]int{"o": {"i": 1}}
	for _, v := range mm {
		ch <- v
	}
	fmt.Println(len(<-ch))
}
