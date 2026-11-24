// Values of any type sent on a chan any (or chan interface{}) arrive with
// their dynamic type: nil, strings, numbers, structs, pointers, slices,
// errors and interface values, through sends, receives, range, select,
// comma-ok and directional channels.
package main

import (
	"errors"
	"fmt"
	"time"
)

type point struct{ x, y int }

type shape interface{ area() int }

type square struct{ side int }

func (s square) area() int { return s.side * s.side }

type box struct{ in chan any }

func produce(out chan<- any, v any) { out <- v }

func next(in <-chan any) any { return <-in }

func main() {
	firstHalf()
	secondHalf()
}

func firstHalf() {
	ch := make(chan any, 8)
	ch <- "old"
	fmt.Println((<-ch).(string))

	ch <- 42
	ch <- 3.5
	ch <- true
	ch <- point{1, 2}
	ch <- []int{1, 2}
	ch <- nil
	ch <- errors.New("boom")
	ch <- square{3}
	for i := 0; i < 8; i++ {
		v := <-ch
		switch x := v.(type) {
		case int:
			fmt.Println("int", x)
		case float64:
			fmt.Println("float", x)
		case bool:
			fmt.Println("bool", x)
		case point:
			fmt.Println("point", x.x, x.y)
		case []int:
			fmt.Println("slice", len(x))
		case nil:
			fmt.Println("nil")
		case error:
			fmt.Println("error", x)
		case shape:
			fmt.Println("shape", x.area())
		}
	}

	s := "kept"
	ch <- s
	fmt.Println(s, <-ch)

	var anyVal any = 7
	ch <- anyVal
	got, ok := <-ch
	fmt.Println(got, ok)

	results := make(chan interface{})
	go func() {
		results <- "from goroutine"
		results <- 99
		close(results)
	}()
	for r := range results {
		fmt.Println(r)
	}
	v, ok := <-results
	fmt.Println(v, ok)
}

func secondHalf() {
	ch := make(chan any, 4)
	p := &point{1, 2}
	ch <- p
	got := (<-ch).(*point)
	got.x = 10
	fmt.Println(p.x)

	b := box{in: make(chan any, 1)}
	b.in <- "field"
	fmt.Println(<-b.in)

	produce(ch, 5)
	fmt.Println(next(ch))

	var v any = "start"
	ch <- 6
	v = <-ch
	fmt.Println(v)

	m := map[string]any{}
	ch <- []string{"a", "b"}
	m["k"] = <-ch
	fmt.Println(m["k"])

	sel := make(chan any, 1)
	select {
	case sel <- "selected":
	default:
		fmt.Println("full")
	}
	select {
	case x := <-sel:
		fmt.Println("got", x)
	case <-time.After(time.Second):
		fmt.Println("timeout")
	}

	done := make(chan struct{})
	work := make(chan interface{})
	go func() {
		defer close(done)
		for w := range work {
			if n, ok := w.(int); ok {
				fmt.Println("n", n*2)
			} else {
				fmt.Println("other", w)
			}
		}
	}()
	work <- 21
	work <- "text"
	work <- nil
	close(work)
	<-done
}
