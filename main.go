package main

import (
	"fmt"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "Usage: %s <go-file>\n", os.Args[0])
		os.Exit(1)
	}

	filename := os.Args[1]

	// Parse Go source
	file, err := ParseFile(filename)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Parse error: %v\n", err)
		os.Exit(1)
	}

	// Translate to Rust AST representation
	rustProgram, err := Translate(file)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Translation error: %v\n", err)
		os.Exit(1)
	}

	// Generate Rust code
	rustCode := Generate(rustProgram)
	fmt.Print(rustCode)
}
