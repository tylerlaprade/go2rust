package main

import (
	"fmt"
	"io"
)

func main() {
	out := io.MultiWriter(io.Discard)
	write := func(x uint32) {
		_, _ = out.Write([]byte{byte(x)})
	}
	write(7)
	fmt.Println("ok")
}
