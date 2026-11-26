// A defer in the body of a range over a function belongs to the function
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

func main() {
	f()
	fmt.Println("g:", g())
}
