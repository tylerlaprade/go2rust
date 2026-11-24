// Storing a channel somewhere new copies the handle: a struct literal
// field, a slice, array or map literal element, a value sent on a channel
// of channels. The original variable still refers to the same channel.
package main

import "fmt"

type holder struct {
	ch   chan int
	name string
}

type pair struct {
	in, out chan string
}

var global = make(chan int, 1)

func send(h holder, v int) { h.ch <- v }

func main() {
	c := make(chan int, 3)
	h := holder{ch: c, name: "keyed"}
	h.ch <- 4
	fmt.Println(<-c, h.name)

	anon := struct{ ch chan int }{ch: c}
	anon.ch <- 5
	fmt.Println(<-c)

	pos := holder{c, "positional"}
	pos.ch <- 6
	fmt.Println(<-c, pos.name)

	ptr := &holder{ch: c}
	ptr.ch <- 7
	fmt.Println(<-c)

	send(holder{ch: c}, 8)
	fmt.Println(<-c)

	in := make(chan string, 1)
	p := pair{in: in, out: in}
	p.in <- "through"
	fmt.Println(<-p.out)
	in <- "again"
	fmt.Println(<-p.in)

	list := []holder{{ch: c}, {ch: c}}
	list[0].ch <- 9
	fmt.Println(<-list[1].ch)

	chans := []chan int{c, c}
	chans[0] <- 10
	fmt.Println(<-c)
	chans = append(chans, c)
	chans[2] <- 11
	fmt.Println(<-chans[1])

	arr := [2]chan int{c, c}
	arr[1] <- 12
	fmt.Println(<-c)

	byName := map[string]chan int{"c": c}
	byName["c"] <- 13
	fmt.Println(<-c)
	c <- 14
	fmt.Println(<-byName["c"])
	held := map[string]holder{"a": {ch: c}}
	held["a"].ch <- 15
	fmt.Println(<-c)

	cc := make(chan chan int, 1)
	cc <- c
	inner := <-cc
	inner <- 16
	fmt.Println(<-c)

	g := holder{ch: global}
	g.ch <- 17
	fmt.Println(<-global)
	g2 := holder{ch: g.ch}
	g2.ch <- 18
	fmt.Println(<-g.ch)

	done := make(chan bool)
	w := holder{ch: c, name: "worker"}
	go func() {
		w.ch <- 19
		done <- true
	}()
	<-done
	fmt.Println(<-c, w.name)

	close(c)
	v, ok := <-h.ch
	fmt.Println(v, ok)
	_, ok = <-list[0].ch
	fmt.Println(ok)
	_, open := <-chans[2]
	fmt.Println(open)
}
