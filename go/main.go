package main

import (
	"fmt"
	"go/parser"
	"go/token"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "Usage: %s <go-file>\n", os.Args[0])
		os.Exit(1)
	}

	filename := os.Args[1]
	file_set := token.NewFileSet()
	file, err := parser.ParseFile(file_set, filename, nil, parser.ParseComments)

	if err != nil {
		fmt.Fprintf(os.Stderr, "Parse error: %v\n", err)
		os.Exit(1)
	}

	rustCode := Transpile(file)
	fmt.Print(rustCode)
}
