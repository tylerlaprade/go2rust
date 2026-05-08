package main

import (
	"bytes"
	"fmt"
)

func main() {
	go func() {}()

	var buf bytes.Buffer
	fmt.Println(buf.String() == "")
}
