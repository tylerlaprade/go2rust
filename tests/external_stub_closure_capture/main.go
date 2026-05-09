package main

import (
	"encoding/binary"
	"fmt"
	"io"
)

func main() {
	out := io.MultiWriter(io.Discard)
	write := func(x uint32) {
		_ = binary.Write(out, binary.LittleEndian, x)
	}
	write(7)
	fmt.Println("ok")
}
