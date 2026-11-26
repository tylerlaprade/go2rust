// A channel of funcs carries func values: nil can be sent, a closed
// channel gives nil, and each receive gives a copy of the value sent.
package main

import "fmt"

func hello() string { return "named" }

type counter struct{ n int }

func (c *counter) inc() int { c.n++; return c.n }

type worker struct {
	jobs chan func() int
}

func producer(out chan<- func() int, f func() int) { out <- f }

func consume(in <-chan func() int) int { return (<-in)() }

func main() {
	// A goroutine makes every variable a locked one.
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)

	ch := make(chan func() string, 4)
	ch <- func() string { return "literal" }
	ch <- hello
	greet := func() string { return "variable" }
	ch <- greet
	greet = func() string { return "changed" }
	ch <- nil
	f := <-ch
	fmt.Println(f())
	var g func() string
	g = <-ch
	fmt.Println(g(), (<-ch)())
	h, ok := <-ch
	fmt.Println(h == nil, ok, greet())
	close(ch)
	k, ok := <-ch
	fmt.Println(k == nil, ok)

	nums := make(chan func(int) int, 2)
	nums <- func(n int) int { return n * 2 }
	nums <- func(n int) int { return n + 1 }
	close(nums)
	total := 0
	for op := range nums {
		total += op(10)
	}
	fmt.Println(total)

	done := make(chan func())
	go func() { done <- func() { fmt.Println("from a goroutine") } }()
	(<-done)()

	c := &counter{}
	jobs := make(chan func() int, 3)
	jobs <- c.inc
	producer(jobs, func() int { return 100 })
	fmt.Println(len(jobs), consume(jobs), consume(jobs), c.n)

	w := worker{jobs: make(chan func() int, 1)}
	w.jobs <- func() int { return 7 }
	got := <-w.jobs
	fmt.Println(got())

	sel := make(chan func() string, 1)
	sel <- func() string { return "selected" }
	select {
	case f := <-sel:
		fmt.Println(f())
	default:
		fmt.Println("none")
	}
	close(sel)
	select {
	case f, ok := <-sel:
		fmt.Println(f == nil, ok)
	}

	table := map[string]func() int{}
	pass := make(chan func() int, 1)
	pass <- func() int { return 5 }
	table["five"] = <-pass
	fmt.Println(table["five"]())

	results := make(chan func(string) string)
	go func() {
		for i := 0; i < 2; i++ {
			prefix := fmt.Sprint(i)
			results <- func(s string) string { return prefix + s }
		}
		close(results)
	}()
	for f := range results {
		fmt.Println(f("x"))
	}
}
