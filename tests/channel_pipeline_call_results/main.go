// A function that returns a channel returns the bare GoChannel, and its
// result passed on to another function, fanIn(gen(5), gen(5)), was taken for
// a cell and locked. Also a declared channel, var ch chan int, which is the
// nil channel.
package main

import (
	"fmt"
	"sync"
)

func gen(n int) <-chan int {
	ch := make(chan int)
	go func() {
		defer close(ch)
		for i := 0; i < n; i++ {
			ch <- i
		}
	}()
	return ch
}

func square(in <-chan int) <-chan int {
	out := make(chan int)
	go func() {
		defer close(out)
		for v := range in {
			out <- v * v
		}
	}()
	return out
}

func fanIn(chs ...<-chan int) <-chan int {
	out := make(chan int)
	var wg sync.WaitGroup
	for _, c := range chs {
		wg.Add(1)
		go func(c <-chan int) {
			defer wg.Done()
			for v := range c {
				out <- v
			}
		}(c)
	}
	go func() { wg.Wait(); close(out) }()
	return out
}

func sum(ch <-chan int) int {
	total := 0
	for v := range ch {
		total += v
	}
	return total
}

func main() {
	fmt.Println(sum(fanIn(gen(5), gen(5))))
	fmt.Println(sum(square(gen(4))))
	fmt.Println(sum(fanIn(square(gen(3)), gen(3), square(square(gen(3))))))
	mk := func() <-chan int { return gen(2) }
	fmt.Println(sum(mk()))

	var ch chan int
	fmt.Println(ch == nil, len(ch), cap(ch))
	ch = make(chan int, 2)
	ch <- 7
	fmt.Println(<-ch, ch != nil, cap(ch))
}
