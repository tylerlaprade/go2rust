// A sync.Pool gives what its New makes, or nil without New, and Put
// takes a value back; Go allows a pool to keep none of them.
package main

import (
	"bytes"
	"fmt"
	"sync"
)

var bufs = sync.Pool{New: func() any {
	b := new(bytes.Buffer)
	b.WriteString("made by New")
	return b
}}

var bare sync.Pool

func render(n int) string {
	b := bufs.Get().(*bytes.Buffer)
	b.Reset()
	defer bufs.Put(b)
	fmt.Fprintf(b, "item-%d", n)
	return b.String()
}

func main() {
	// The pool is empty, so the first Get is what New makes.
	fmt.Println(bufs.Get().(*bytes.Buffer).String())
	out := make([]string, 4)
	for i := range out {
		out[i] = render(i)
	}
	fmt.Println(out)
	got := bare.Get()
	v, ok := got.(*bytes.Buffer)
	fmt.Println(got == nil, v == nil, ok)
	bare.Put(new(bytes.Buffer))
}
