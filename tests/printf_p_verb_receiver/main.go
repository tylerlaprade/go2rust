package main

import (
	"fmt"
	"strings"
)

type Scope struct {
	name string
}

func (s *Scope) String() string {
	var buf strings.Builder
	fmt.Fprintf(&buf, "scope %p {", s)
	fmt.Fprintf(&buf, "name=%s}", s.name)
	return buf.String()
}

func main() {
	s := &Scope{name: "outer"}
	out := s.String()
	// Strip the pointer (varies by run); just check the structural prefix/suffix.
	if !strings.HasPrefix(out, "scope ") || !strings.Contains(out, " {name=outer}") {
		fmt.Println("FAIL:", out)
		return
	}
	fmt.Println("OK")
}
