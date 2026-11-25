// In a program with goroutines: a slice of an array shares its elements.
package main

import (
	"fmt"
	"strings"
)

type rec struct {
	buf [8]byte
	n   int
}

func (r *rec) put(s string) {
	r.n = copy(r.buf[r.n:], s)
}

func main() {
	done := make(chan bool, 1)
	go func() { done <- true }()
	<-done
	var arr [5]int
	s := arr[:0]
	s = append(s, 1, 2)
	fmt.Println(arr, len(s), cap(s))
	t := arr[3:]
	t = append(t, 9, 9, 9) // past capacity: copies
	t[0] = 7
	fmt.Println(arr, t)

	var r rec
	r.put("hey")
	fmt.Println(string(r.buf[:r.n]), r.buf[1])

	var buf [4]byte
	rd := strings.NewReader("abcdef")
	n, _ := rd.Read(buf[:])
	fmt.Println(n, string(buf[:n]))

	sum := 0
	for _, v := range arr[:] {
		sum += v
	}
	fmt.Println(sum)

	arr = [5]int{}
	fmt.Println(s[0], arr[0])
	s[1] = 42
	fmt.Println(arr[1])

	rows := [][3]int{{1, 2, 3}}
	q := rows[0][:2]
	fmt.Println(q)

	p := &arr
	ps := p[1:3]
	ps[0] = 5
	fmt.Println(arr[1], len(ps), cap(ps))
}
