package main

import (
	"bytes"
	"fmt"
)

func main() {
	var buf bytes.Buffer
	fmt.Println(buf.String() == "")
	buf.WriteString("go")
	buf.WriteByte('2')
	buf.WriteString("rust")
	fmt.Println(buf.Len(), buf.String())
}
