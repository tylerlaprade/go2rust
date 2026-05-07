package main

import "fmt"

type symbol struct {
	name string
	kind int
}

var symbols = map[string][]symbol{
	"fmt": {
		{name: "Println", kind: 1},
		{name: "Printf", kind: 1},
	},
	"strings": {
		{name: "Builder", kind: 2},
		{name: "TrimSpace", kind: 1},
	},
	"bytes": {
		{name: "Buffer", kind: 2},
	},
}

func main() {
	fmt.Println(len(symbols), len(symbols["fmt"]), symbols["strings"][0].name, symbols["bytes"][0].kind)
}
