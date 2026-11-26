// A constant in a []any literal goes in as a value of its type, whether a
// package or a function declares it.
package main

import "fmt"

type Weekday int

const (
	Sunday Weekday = iota
	Monday
)

func (d Weekday) String() string { return [...]string{"Sun", "Mon"}[d] }

type Level int

const High Level = 3

type Color string

const Red Color = "red"

const Plain = 7

const Pi = 3.5

const Greeting = "hi"

const Ratio float64 = 0.25

const On = true

func show(v any) { fmt.Println(v) }

func main() {
	// A goroutine makes every variable a locked one.
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)
	const local Level = 9
	const word = "w"
	var x any = Monday
	vs := []any{Monday, High, Plain, Sunday + 1, Red, Pi, Greeting, Ratio, On, local, word}
	arr := [...]any{Red, High}
	show(Monday)
	show(Red)
	fmt.Println(x, vs, len(vs), arr[0], arr[1])
	fmt.Printf("%v %T %T %T\n", vs[4], vs[4], vs[1], vs[7])
	var y any = Red
	var z any = Greeting
	fmt.Println(y, z)
}
