// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

type Weekday int

const (
	Sunday Weekday = iota
	Monday
	Tuesday
)

func (d Weekday) String() string { return [...]string{"Sun", "Mon", "Tue"}[d] }

type Level int

const High Level = 3

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started

	fmt.Println(Monday, Tuesday+0, Weekday(0), High)
	fmt.Printf("%v|%d|%s|%v\n", Monday, Monday, Tuesday, High)
	var x interface{} = Monday
	fmt.Println(x, fmt.Sprint(Tuesday), Monday.String())
	d := Monday
	fmt.Println(d, d+1, d*2%3, []Weekday{Sunday, Tuesday})
	fmt.Println(fmt.Sprintf("%v-%v", d, Sunday))
}
