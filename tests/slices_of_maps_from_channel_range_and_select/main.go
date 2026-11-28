package main

import "fmt"

func main() {
	ch := make(chan map[string]int, 2)
	ch <- map[string]int{"a": 1}
	ch <- map[string]int{"b": 2, "c": 3}
	close(ch)
	var all []map[string]int
	for m := range ch {
		s := []map[string]int{m}
		all = append(all, m)
		fmt.Println(len(s[0]))
	}
	other := make(chan map[string]int, 1)
	other <- map[string]int{"z": 9}
	select {
	case v := <-other:
		arr := [1]map[string]int{}
		arr[0] = v
		fmt.Println(arr[0]["z"])
	}
	fmt.Println(len(all), len(all[1]))
}
