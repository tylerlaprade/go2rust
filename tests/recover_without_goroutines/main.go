// recover() in a program without goroutines. It returned nil and the panic
// stopped the program; with a goroutine in the program it already worked.
package main

import (
	"errors"
	"fmt"
)

func boom() { panic("in named function") }

func safe() {
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("recovered:", r)
		}
	}()
	boom()
}

func safeLine() {
	defer func() {
		r := recover()
		if r != nil {
			fmt.Println("recovered too:", r)
		}
	}()
	panic("same function")
}

func divide(a, b int) (result int, err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("caught: %v", r)
		}
	}()
	return a / b, nil
}

func element(xs []int, i int) (v int, ok bool) {
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("element:", r)
		}
	}()
	return xs[i], true
}

func withError() (n int) {
	defer func() {
		if r := recover(); r != nil {
			n = -1
		}
	}()
	panic(errors.New("an error value"))
}

// A deferred function that panics replaces the panic, and the deferred
// functions after it still run.
func replaced() {
	defer func() {
		fmt.Println("last:", recover())
	}()
	defer func() {
		fmt.Println("still runs")
	}()
	defer func() {
		panic("second")
	}()
	panic("first")
}

// A function literal recovers its own panic and panics again.
func nested() {
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("outer:", r)
		}
	}()
	func() {
		defer func() {
			if r := recover(); r != nil {
				fmt.Println("inner:", r)
				panic("again")
			}
		}()
		panic("original")
	}()
}

func noPanic() int {
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("never printed")
		}
	}()
	return 7
}

func main() {
	safe()
	safeLine()
	q, err := divide(10, 2)
	fmt.Println(q, err)
	q, err = divide(1, 0)
	fmt.Println(q, err)
	v, ok := element([]int{1, 2, 3}, 5)
	fmt.Println(v, ok)
	fmt.Println(withError())
	replaced()
	nested()
	fmt.Println(noPanic())
	defer fmt.Println("deferred in main")
	fmt.Println("done")
}
