// In a program with goroutines every read of a value locks it, and the
// locks a print! argument takes last until the print is done. So Printf
// with two arguments that both read p, p.In.V and p.Age, waited forever
// for the first argument's lock. Each argument is formatted on its own now.
// Sprintf too.
package main

import "fmt"

type P struct {
	Name string
	Age  int
	In   *Inner
}

type Inner struct{ V int }

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done
	p := &P{"ann", 3, &Inner{4}}
	fmt.Printf("%s %d\n", p.Name, p.Age)
	fmt.Printf("%d %d\n", p.In.V, p.Age)
	q := P{"bob", 5, &Inner{6}}
	fmt.Printf("%s %d %d\n", q.Name, q.Age, q.In.V)
	line := fmt.Sprintf("%d/%d %5.1f", p.In.V, p.Age, float64(q.In.V)/4)
	fmt.Println(line)
}
