// A constant of a type with a String method, Monday of type Weekday, printed
// as its integer and went into an interface as an int. It prints and boxes
// as a Weekday now, and arithmetic on a Weekday is a Weekday.
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
	fmt.Println(Monday, Tuesday+0, Weekday(0), High)
	fmt.Printf("%v|%d|%s|%v\n", Monday, Monday, Tuesday, High)
	var x interface{} = Monday
	fmt.Println(x, fmt.Sprint(Tuesday), Monday.String())
	d := Monday
	fmt.Println(d, d+1, d*2%3, []Weekday{Sunday, Tuesday})
	fmt.Println(fmt.Sprintf("%v-%v", d, Sunday))
}
