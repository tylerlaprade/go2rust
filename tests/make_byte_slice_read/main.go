// A slice made with make has its element type even when nothing else in the
// program pins it down: f.Read fills a make([]byte, n) buffer.
package main

import (
	"fmt"
	"os"
)

func main() {
	buf := make([]byte, 8)
	f, err := os.Open("main.go")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()
	n, err := f.Read(buf)
	fmt.Printf("%d %q %v\n", n, buf[:n], err)

	small := make([]float32, 2, 4)
	big := make([]uint64, 3)
	fmt.Println(len(small), cap(small), small, big)
}
