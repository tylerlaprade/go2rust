// In a program with goroutines: a defer in the body of a range over a function belongs to the function
// the loop is in: it runs when that function returns, after the code that
// follows the loop, last first, with the arguments it had at the defer.
package main

import "fmt"

func seq3(yield func(int) bool) {
	for i := 0; i < 3; i++ {
		if !yield(i) {
			return
		}
	}
}

func pairs(yield func(string, int) bool) {
	_ = yield("a", 1) && yield("b", 2)
}

func f() {
	defer fmt.Println("first defer")
	for i := range seq3 {
		defer fmt.Println("deferred", i)
		if i == 1 {
			defer func() { fmt.Println("closure at", i) }()
		}
	}
	defer fmt.Println("last defer")
	fmt.Println("after loop")
}

func g() (n int) {
	for k, v := range pairs {
		defer func(s string) { fmt.Println("pair", s, n) }(k)
		for i := range seq3 {
			if i == 2 {
				defer fmt.Println("inner", k, i)
			}
		}
		if v == 2 {
			return v * 10
		}
	}
	return -1
}

func pair() (string, int) { return "p", 7 }

func record(s string, n int) { fmt.Println("record", s, n) }

func show(prefix string, xs ...int) { fmt.Println(prefix, xs) }

func picker(name string) func(int) {
	fmt.Println("pick", name)
	return func(i int) { fmt.Println("picked", name, i) }
}

// h defers calls whose function and arguments are evaluated at the defer:
// a call with two results as the arguments, a slice passed on with ...,
// and a function that a call returns. A deferred panic does not keep the
// calls deferred before it from running.
func h() {
	defer func() { fmt.Println("recovered", recover()) }()
	act := func(i int) { fmt.Println("act", i) }
	xs := []int{1, 2}
	for i := range seq3 {
		defer record(pair())
		defer show("xs", xs...)
		defer act(i)
		defer picker("at")(i)
		if i == 0 {
			defer fmt.Println("cleanup")
		}
		if i == 2 {
			defer panic("boom")
		}
	}
	xs[0] = 9
	fmt.Println("h after loop")
}

func main() {
	h()
	f()
	fmt.Println("g:", g())
	done := make(chan int)
	go func() {
		defer close(done)
		for i := range seq3 {
			defer func() { done <- i }()
		}
	}()
	for v := range done {
		fmt.Println("from goroutine", v)
	}
}
