package main

import (
	"fmt"
	"go/parser"
	"go/token"
	"os"

	go2rust "go2rust/go"
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

	rustCode := go2rust.Transpile(file)
	fmt.Print(rustCode)
}
