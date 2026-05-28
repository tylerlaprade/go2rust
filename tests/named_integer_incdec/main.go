package main

import "fmt"

// Reproduces a named-integer-type increment inside a loop, mirroring
// go/token's `for i := keyword_beg + 1; i < keyword_end; i++`. The named
// type lowers to a wrapped newtype with Add<scalar>/Sub<scalar> impls, so
// `i++` must not re-wrap the already-newtype value in another newtype
// constructor (which puts a Token where the inner Option expects i32).
type Token int

const (
	tokBeg Token = iota
	tokAdd
	tokSub
	tokMul
	tokEnd
)

func main() {
	count := 0
	for i := tokBeg + 1; i < tokEnd; i++ {
		count++
	}
	fmt.Println(count)
}
