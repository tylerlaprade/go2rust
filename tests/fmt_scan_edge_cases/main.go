package main

import (
	"errors"
	"fmt"
	"io"
)

type Celsius float64

type errReader struct {
	s string
	i int
}

func (r *errReader) ReadRune() (rune, int, error) {
	if r.i < len(r.s) {
		c := r.s[r.i]
		r.i++
		return rune(c), 1, nil
	}
	return 0, 0, errors.New("boom")
}

func (r *errReader) UnreadRune() error { r.i--; return nil }

func (r *errReader) Read(p []byte) (int, error) { return 0, errors.New("unused") }

func main() {
	var f float64
	n, err := fmt.Sscan("0x1.fp+2 -0x.8p1", &f, &f)
	fmt.Println(n, err, f)
	n, err = fmt.Sscanf("1p2147483648", "%b", &f)
	fmt.Println(n, err, f)
	n, err = fmt.Sscanf("1.5p3", "%g", &f)
	fmt.Println(n, err, f)
	var a, b int
	n, err = fmt.Sscanf("7", "%100000000d%d", &a, &b)
	fmt.Println(n, err, a)
	var u uint64
	n, err = fmt.Sscan("18446744073709551616_", &u)
	fmt.Println(n, err)
	var bs []byte
	n, err = fmt.Sscanf(`"\xff\101é"`, "%q", &bs)
	fmt.Println(n, err, bs)
	var s string
	n, err = fmt.Sscanf(`"\400"`, "%q", &s)
	fmt.Println(n, err)
	var r1, r2, r3, r4 rune
	n, err = fmt.Sscanf("\xF0\xC2AB", "%c%c%c%c", &r1, &r2, &r3, &r4)
	fmt.Println(n, err, r1, r2, string(r3), string(r4))
	m := map[string][]int{"row": {1, 2}}
	n, err = fmt.Sscan("9", &m["row"][1])
	fmt.Println(n, err, m["row"])
	n, err = fmt.Fscan(&errReader{s: "1 "}, &a, &b)
	fmt.Println(n, err, a)
	_, err = readPair("5")
	fmt.Println(err, errors.Is(err, io.EOF), err == io.EOF)
	if n < 0 {
		var c Celsius
		fmt.Sscan("1", &c)
	}
}
