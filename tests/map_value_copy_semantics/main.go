package main

import "fmt"

// Go copies map values on assignment. Storing a value-typed loop variable
// must snapshot its value, not alias the variable's handle (which would make
// every entry track the variable's final value). Mirrors go/token's
// keywords[tokens[i]] = i.
type Tok int

func main() {
	m := map[int]int{}
	for i := 1; i <= 3; i++ {
		m[i] = i
	}
	fmt.Println(m[1], m[2], m[3])

	tm := map[int]Tok{}
	for t := Tok(1); t <= 3; t++ {
		tm[int(t)] = t
	}
	fmt.Println(int(tm[1]), int(tm[2]), int(tm[3]))
}
