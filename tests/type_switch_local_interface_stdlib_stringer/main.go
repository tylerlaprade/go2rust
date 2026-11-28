package main

import (
	"fmt"
	"time"
)

type stringer interface{ String() string }

func stringify(v any) string {
	switch s := v.(type) {
	case stringer:
		return s.String()
	case string:
		return s
	case nil:
		return "<nil>"
	}
	return "?"
}

func main() {
	fmt.Println(stringify(time.Second), stringify("x"), stringify(nil), stringify(3))
}
