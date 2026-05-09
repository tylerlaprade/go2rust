package main

import "fmt"

type Index int

type Holder struct {
	Values map[string]Index
}

func main() {
	done := make(chan bool, 1)
	go func() {
		done <- true
	}()
	<-done

	h := Holder{Values: map[string]Index{}}
	var idx Index = 7
	h.Values["answer"] = idx

	missing, ok := h.Values["missing"]
	fmt.Println(missing, ok)
	fmt.Println(h.Values["answer"])
}
