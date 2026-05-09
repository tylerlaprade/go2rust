package main

import "fmt"

type Holder struct {
	elems [2][]string
}

func main() {
	done := make(chan bool, 1)
	go func() {
		done <- true
	}()
	<-done

	h := Holder{elems: [2][]string{}}
	h.elems[0] = append(h.elems[0], "alpha")
	h.elems[0] = append(h.elems[0], "beta")
	h.elems[1] = append(h.elems[1], "gamma")

	fmt.Println(len(h.elems[0]), h.elems[0][0], h.elems[0][1])
	fmt.Println(len(h.elems[1]), h.elems[1][0])
}
