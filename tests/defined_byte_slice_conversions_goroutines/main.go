// In a program with goroutines: a defined []byte or []rune type is built
// from a string, a []byte literal, a declaration with the type, an
// assignment, an argument and a struct field, and converts back to a string.
package main

import (
	"fmt"
	"sync"
)

type Bytes []byte
type Runes []rune
type Text string

func (b Bytes) Upper() Bytes {
	out := make(Bytes, len(b))
	for i, c := range b {
		if c >= 'a' && c <= 'z' {
			c -= 32
		}
		out[i] = c
	}
	return out
}

func size(b Bytes) int { return len(b) }

type Msg struct{ Body Bytes }

func main() {

	b := Bytes("xyz")
	var c Bytes = []byte("abc")
	d := Bytes([]byte{'h', 'i'})
	fmt.Println(string(b), string(c), string(d), len(b))
	fmt.Println(string(c.Upper()), string(b[1:]))
	fmt.Printf("%s %v\n", b, c)
	r := Runes("héllo")
	fmt.Println(len(r), string(r))
	c = []byte("reassigned")
	fmt.Println(string(c), size([]byte("four")))
	m := Msg{Body: []byte("body")}
	fmt.Println(string(m.Body))
	// u shares raw's array until the append, which needs a new one.
	raw := make([]byte, 3)
	copy(raw, "raw")
	var u Bytes = raw
	u[1] = 'A'
	u = append(u, '!')
	u[0] = 'R'
	fmt.Println(string(raw), string(u))
	// A defined string, a nil []byte, an assigned variable and a [][]byte
	// row go into the defined type; a nil or invalid []rune converts.
	var name Text = "hey"
	fmt.Println(string(Bytes(name)), len(Runes(name)))
	var none []byte
	var empty Bytes = none
	fmt.Println(empty == nil, len(empty), string(Runes(nil)) == "")
	one := []byte{1}
	u = one
	u[0] = 2
	rows := [][]byte{{7}}
	var row Bytes = rows[0]
	row[0] = 8
	bad := Runes("x")
	bad[0] = -1
	fmt.Println(one[0], rows[0][0], string(bad) == "\uFFFD")
	var wg sync.WaitGroup
	out := make([]Bytes, 2)
	for i := range out {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			out[i] = Bytes(fmt.Sprint("g", i)).Upper()
		}(i)
	}
	wg.Wait()
	fmt.Println(string(out[0]), string(out[1]))
}
