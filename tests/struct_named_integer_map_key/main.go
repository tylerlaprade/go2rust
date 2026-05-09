package main

import "fmt"

func main() {
	done := make(chan bool, 1)
	go func() {
		done <- true
	}()
	<-done

	seen := map[Entry]uint32{}
	k := Kind(2)
	idx := Index(3)
	e := Entry{k, idx}
	seen[e] = 0
	if got, ok := seen[e]; ok {
		fmt.Println(got)
	}
}
