package main

import (
	"fmt"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "Usage: %s <go-file-or-directory>...\n", os.Args[0])
		os.Exit(1)
	}

	// Collect all Go files from arguments
	var goFiles []string
	for _, arg := range os.Args[1:] {
		files, err := collectGoFiles(arg)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error processing %s: %v\n", arg, err)
			os.Exit(1)
		}
		goFiles = append(goFiles, files...)
	}

	if len(goFiles) == 0 {
		fmt.Fprintf(os.Stderr, "No Go files found\n")
		os.Exit(1)
	}

	// Sort files to ensure consistent output (main.go last if present)
	sort.Slice(goFiles, func(i, j int) bool {
		// Put main.go last
		if filepath.Base(goFiles[i]) == "main.go" {
			return false
		}
		if filepath.Base(goFiles[j]) == "main.go" {
			return true
		}
		return goFiles[i] < goFiles[j]
	})

	// Transpile all files and concatenate
	var rustCode strings.Builder
	file_set := token.NewFileSet()

	for i, filename := range goFiles {
		file, err := parser.ParseFile(file_set, filename, nil, parser.ParseComments)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Parse error in %s: %v\n", filename, err)
			os.Exit(1)
		}

		fileRust := Transpile(file)
		rustCode.WriteString(fileRust)

		// Add newline between files (except after the last one)
		if i < len(goFiles)-1 {
			rustCode.WriteString("\n")
		}
	}

	fmt.Print(rustCode.String())
}

// collectGoFiles returns all Go files from the given path (file or directory)
func collectGoFiles(path string) ([]string, error) {
	info, err := os.Stat(path)
	if err != nil {
		return nil, err
	}

	if !info.IsDir() {
		// Single file
		if !strings.HasSuffix(path, ".go") {
			return nil, fmt.Errorf("not a Go file: %s", path)
		}
		return []string{path}, nil
	}

	// Directory - find all .go files
	var goFiles []string
	entries, err := os.ReadDir(path)
	if err != nil {
		return nil, err
	}

	for _, entry := range entries {
		if entry.IsDir() {
			continue // Skip subdirectories for now
		}
		if strings.HasSuffix(entry.Name(), ".go") {
			goFiles = append(goFiles, filepath.Join(path, entry.Name()))
		}
	}

	return goFiles, nil
}
