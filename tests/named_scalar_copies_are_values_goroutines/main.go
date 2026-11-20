// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

type Counter int

func (c *Counter) Inc() { *c++ }

type Celsius float64

func (c *Celsius) Warm() { *c += 1.5 }

type Name string

func (n *Name) Shout() { *n += "!" }

type Flag bool

func (f *Flag) Toggle() { *f = !*f }

type Stats struct {
	Hits Counter
	Temp Celsius
}

func bump(c Counter) Counter {
	c.Inc()
	return c
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started

	a := Counter(1)
	b := a
	b.Inc()
	fmt.Println(a, b, bump(a), a)

	x := Celsius(1)
	y := x
	y.Warm()
	fmt.Println(x, y)

	n := Name("go")
	m := n
	m.Shout()
	fmt.Println(n, m)

	f := Flag(false)
	g := f
	g.Toggle()
	fmt.Println(f, g)

	s := Stats{Hits: 5, Temp: 20}
	t := s
	t.Hits.Inc()
	t.Temp.Warm()
	fmt.Println(s, t)

	counters := []Counter{1, 2, 3}
	first := counters[0]
	first.Inc()
	for _, c := range counters {
		c.Inc()
	}
	fmt.Println(counters, first)

	p := &a
	p.Inc()
	fmt.Println(a, *p)
	byName := map[string]Counter{"k": 10}
	v := byName["k"]
	v.Inc()
	fmt.Println(byName["k"], v)
}
