// Parentheses around a call do not change it: (f()).s selects the field of
// the call's result, as f().s does, and (f()).m() calls its method.
package main

import "fmt"

type rec struct {
	n int
	s string
}

func f() rec { return rec{n: 2, s: "c"} }

func (r rec) twice() int { return r.n * 2 }

func (r *rec) bump() int { r.n++; return r.n }

func p() *rec { return &rec{n: 5, s: "p"} }

func main() {
	// A goroutine makes every variable a locked one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	w := (f()).s
	v := ((f())).n
	fmt.Println(w, v, (f()).twice(), ((f())).s)
	fmt.Println((p()).n, (p()).bump(), (p()).s)
	q := &rec{n: 1}
	fmt.Println((*q).n, (q).n)
	(p()).n = 9
}
