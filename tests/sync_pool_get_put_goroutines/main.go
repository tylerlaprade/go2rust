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
	var wg sync.WaitGroup
	out := make([]string, 4)
	for i := range out {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			out[i] = render(i)
		}(i)
	}
	wg.Wait()
	fmt.Println(out)
	v, ok := bare.Get().(*bytes.Buffer)
	fmt.Println(v == nil, ok)
	bare.Put(new(bytes.Buffer))
}
