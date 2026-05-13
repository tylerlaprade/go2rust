package main

import (
	"fmt"
	"strings"
)

func main() {
	builder := strings.Builder{}
	builder.WriteString("a")
	builder.WriteByte('b')
	builder.WriteRune('c')
	result := builder.String()
	fmt.Println(result, builder.Len())
}
