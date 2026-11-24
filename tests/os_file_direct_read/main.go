package main

import (
	"fmt"
	"io"
	"os"
)

func main() {
	if err := os.WriteFile("direct_read.txt", []byte("hello, file"), 0o644); err != nil {
		fmt.Println(err)
		return
	}
	defer os.Remove("direct_read.txt")
	f, err := os.Open("direct_read.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer f.Close()
	buf := make([]byte, 4)
	total := 0
	for {
		n, err := f.Read(buf)
		total += n
		fmt.Printf("%d %q %v\n", n, string(buf[:n]), err)
		if err == io.EOF {
			break
		}
		if err != nil {
			fmt.Println("error:", err)
			break
		}
	}
	fmt.Println("total", total)

	g, _ := os.Open("direct_read.txt")
	big := make([]byte, 64)
	n, err := g.Read(big)
	fmt.Println(n, err, string(big[:n]))
	n, err = g.Read(big)
	fmt.Println(n, err == io.EOF)
	at := make([]byte, 4)
	n, err = g.ReadAt(at, 7)
	fmt.Println(n, err, string(at[:n]))
	g.Close()
}
