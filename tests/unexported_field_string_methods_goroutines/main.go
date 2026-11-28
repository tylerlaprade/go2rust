package main

import "fmt"

type level int

func (l level) String() string { return fmt.Sprint("L", int(l)) }

type name string

func (n name) String() string { return "N(" + string(n) + ")" }

type ratio float64

func (r ratio) String() string { return "R" }

type toggle bool

func (f toggle) String() string { return "F" }

type code int

func (c code) Error() string { return "E" }

// inner prints through its String method when the field is exported.
type inner struct{ v level }

func (i inner) String() string { return "I" }

// pair's unexported fields print their underlying values: fmt cannot call
// a method of a value it reached through an unexported field.
type pair struct {
	hidden level
	Shown  level
	n      name
	Label  name
	r      ratio
	f      toggle
	c      code
	Out    inner
	levels []level
	Ranks  []level
	grid   [2]ratio
}

type wrapper struct {
	Inside pair
	mode   level
}

// holder's unexported field has an Error method; it prints as its value.
type holder struct {
	c    code
	Code code
}

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	var on toggle = true
	fmt.Println(pair{1, 2, "a", "b", 1.5, on, 7, inner{4}, nil, nil, [2]ratio{}})
	fmt.Printf("%v %+v\n", pair{hidden: 3, Shown: 4}, pair{hidden: 5, Shown: 6})
	p := pair{levels: []level{1, 2}, Ranks: []level{3}, grid: [2]ratio{0.5, 2}}
	fmt.Println(p)
	fmt.Printf("%+v\n", p)
	fmt.Println(p.hidden, p.levels, p.grid)
	w := wrapper{Inside: p, mode: 2}
	fmt.Println(w.mode, w.Inside.hidden)
	fmt.Printf("%v|%s\n", holder{3, 4}, fmt.Sprint(holder{5, 6}))
}
