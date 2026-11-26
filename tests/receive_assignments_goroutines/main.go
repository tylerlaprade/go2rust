// A select case or a channel range that assigns, case x = <-ch or
// for x = range ch, stores the received value into an existing variable,
// field, element or map entry.
package main

import (
	"errors"
	"fmt"
)

type holder struct {
	n   int
	s   string
	fn  func() string
	err error
}

type shape interface{ area() int }

type sq struct{ side int }

func (q sq) area() int { return q.side * q.side }

func main() {
	// A goroutine makes every variable a locked one.
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)

	ch := make(chan int, 4)
	ch <- 1
	ch <- 2
	ch <- 3
	x := 0
	select {
	case x = <-ch:
	default:
	}
	fmt.Println(x)
	var ok bool
	select {
	case x, ok = <-ch:
		fmt.Println("in case:", x, ok)
	}
	fmt.Println(x, ok)
	h := &holder{}
	arr := []int{0, 0}
	m := map[string]int{}
	ch <- 7
	ch <- 8
	ch <- 9
	select {
	case h.n = <-ch:
	}
	select {
	case arr[1] = <-ch:
	}
	select {
	case m["k"] = <-ch:
	}
	fmt.Println(h.n, arr, m)
	close(ch)
	_, ok = <-ch
	select {
	case x, ok = <-ch:
	}
	fmt.Println(x, ok)

	sc := make(chan string, 2)
	sc <- "a"
	sc <- "b"
	close(sc)
	last := ""
	for last = range sc {
	}
	fmt.Println(last)
	nums := make(chan int, 2)
	nums <- 4
	nums <- 5
	close(nums)
	for h.n = range nums {
		fmt.Println("range field:", h.n)
	}
	fmt.Println(h.n)

	ec := make(chan error, 1)
	ec <- errors.New("boom")
	select {
	case h.err = <-ec:
	}
	var err error
	ec <- errors.New("bang")
	select {
	case err = <-ec:
	}
	fmt.Println(h.err, err)

	shc := make(chan shape, 1)
	shc <- sq{3}
	var sh shape
	select {
	case sh = <-shc:
	}
	fmt.Println(sh.area())

	// Both targets are evaluated before either is stored.
	idx := make(chan int, 1)
	idx <- 1
	i := 0
	flags := []bool{false, false}
	select {
	case i, flags[i] = <-idx:
	}
	fmt.Println(i, flags)

	// A variable named like the temporaries keeps its value.
	__go_recv_value_1 := "kept"
	idx <- 2
	select {
	case i = <-idx:
	}
	fmt.Println(i, __go_recv_value_1)

	pc := make(chan holder, 1)
	pc <- holder{n: 42}
	var hv holder
	select {
	case hv = <-pc:
	}
	fmt.Println(hv.n)
}
