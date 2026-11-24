// Defined array and byte-slice types, [256] tables and [][]byte elements,
// as strings.Replacer uses them: ranging over a defined array, a nil
// element of a slice table, len stored into an int table, and a defined
// []byte that appends strings and converts to string.
package main

import "fmt"

type byteTable [256]byte

type appendWriter []byte

func (w *appendWriter) WriteString(s string) (int, error) {
	*w = append(*w, s...)
	return len(s), nil
}

func (w *appendWriter) Write(p []byte) (int, error) {
	*w = append(*w, p...)
	return len(p), nil
}

type replacer struct {
	mapping      [256]byte
	replacements [256][]byte
	skip         [256]int
}

func newTable() *byteTable {
	r := new(byteTable)
	for i := range r {
		r[i] = byte(i)
	}
	return r
}

func main() {
	t := newTable()
	t['a'] = 'A'
	fmt.Println(t['a'], t['b'], len(t))

	local := byteTable{}
	for i := range local {
		local[i] = byte(255 - i)
	}
	fmt.Println(local[0], local[255])

	var rep replacer
	for i := range rep.mapping {
		rep.mapping[i] = byte(i)
	}
	rep.replacements['x'] = []byte("XX")
	rep.replacements['y'] = []byte("")
	n := 0
	for _, b := range rep.mapping {
		if b != 0 {
			n++
		}
	}
	nilCount := 0
	for i := range rep.replacements {
		if rep.replacements[i] == nil {
			nilCount++
		}
	}
	fmt.Println(n, nilCount, rep.replacements['x'] != nil, rep.replacements['y'] != nil, len(rep.replacements['y']))

	table := make([][]int, 3)
	table[1] = []int{}
	table[2] = append(table[2], 7)
	fmt.Println(table[0] == nil, table[1] == nil, table[2] == nil, len(table[2]))

	pattern := "abc"
	for i := range rep.skip {
		rep.skip[i] = len(pattern)
	}
	last := len(pattern) - 1
	for i := 0; i < last; i++ {
		rep.skip[pattern[i]] = last - i
	}
	fmt.Println(rep.skip['a'], rep.skip['b'], rep.skip['z'])

	counts := []int{0, 0}
	counts[1] = len(pattern) + cap(counts)
	fmt.Println(counts)

	buf := make(appendWriter, 0, 8)
	buf.WriteString("hello")
	buf.Write([]byte(", world"))
	fmt.Println(string(buf), len(buf))
}
