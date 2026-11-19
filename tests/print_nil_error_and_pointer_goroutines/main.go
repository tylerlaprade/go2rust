package main

import (
	"errors"
	"fmt"
)

type Point struct{ X, Y int }

// Status has a String method on the value, so *Status has it too.
type Status struct{ Code int }

func (s Status) String() string { return fmt.Sprintf("status %d", s.Code) }

// Gauge has a String method on the pointer only.
type Gauge struct{ Level int }

func (g *Gauge) String() string { return fmt.Sprintf("gauge %d", g.Level) }

// Broken is an error through its pointer.
type Broken struct{ Why string }

func (b *Broken) Error() string { return "broken: " + b.Why }

// Timeout is an error through its value.
type Timeout struct{ Seconds int }

func (t Timeout) Error() string { return fmt.Sprintf("timeout after %ds", t.Seconds) }

func find(ok bool) error {
	if ok {
		return nil
	}
	return errors.New("missing")
}

func lookup(ok bool) *Point {
	if ok {
		return &Point{1, 2}
	}
	return nil
}

func main() {
	// A goroutine makes every shared value a locked one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	// A nil error prints as <nil>, whichever print call it reaches and
	// whether it comes from a variable or straight from a call.
	err := find(true)
	fmt.Println(err)
	fmt.Println("result:", err)
	fmt.Printf("%v\n", err)
	fmt.Printf("%v and %v\n", err, 3)
	fmt.Println(find(true))
	fmt.Println(fmt.Sprintf("%v", err))
	fmt.Println(fmt.Sprintf("got %v", find(true)))

	// An error that is not nil prints its message, as before.
	err = find(false)
	fmt.Println(err)
	fmt.Println("result:", err)
	fmt.Printf("%v %s\n", err, err)
	fmt.Println(find(false))

	// A nil pointer prints as <nil>.
	var p *Point
	fmt.Println(p)
	fmt.Println("point:", p)
	fmt.Printf("%v\n", p)
	fmt.Println(lookup(false))
	p = lookup(true)
	fmt.Println(p)
	fmt.Println(lookup(true))

	// A pointer whose type has a String or Error method prints through the
	// method, with no & in front. A nil one prints as <nil>.
	s := &Status{7}
	fmt.Println(s)
	fmt.Printf("%v %s\n", s, s)
	g := &Gauge{3}
	fmt.Println(g)
	fmt.Println(fmt.Sprintf("%v", g))
	b := &Broken{"z"}
	fmt.Println(b)
	fmt.Println("gauge:", g, "broken:", b)
	var none *Status
	fmt.Println(none)
	var empty *Gauge
	fmt.Println(empty)

	// A struct or a pointer that implements error can start an error
	// variable, and prints as the error it is.
	var declared error = &Broken{"declared"}
	fmt.Println(declared)
	var plain error = Timeout{3}
	fmt.Println(plain)
	fmt.Println(declared != nil, plain != nil)
	var unset error
	fmt.Println(unset, unset == nil)
}
