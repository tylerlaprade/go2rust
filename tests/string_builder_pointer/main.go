package main

import (
	"fmt"
	"strings"
)

func appendParts(out *strings.Builder, suffix string) {
	out.WriteString("go")
	out.WriteByte('2')
	out.WriteRune('r')
	out.WriteString(suffix)
}

func read(out *strings.Builder) string {
	return out.String()
}

func size(out *strings.Builder) int {
	return out.Len()
}

func main() {
	var builder strings.Builder
	appendParts(&builder, "ust")
	fmt.Println(read(&builder), size(&builder))
}
