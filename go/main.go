package main

import (
	"fmt"
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
		if filepath.Base(goFiles[i]) == "main.go" {
			return false
		}
		if filepath.Base(goFiles[j]) == "main.go" {
			return true
		}
		return goFiles[i] < goFiles[j]
	})

	generator := NewProjectGenerator(goFiles)
	if generator == nil {
		fmt.Fprintf(os.Stderr, "No files to process\n")
		os.Exit(1)
	}

	err := generator.Generate()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func collectGoFiles(path string) ([]string, error) {
	info, err := os.Stat(path)
	if err != nil {
		return nil, err
	}

	if !info.IsDir() {
		if !strings.HasSuffix(path, ".go") {
			return nil, fmt.Errorf("not a Go file: %s", path)
		}
		return []string{path}, nil
	}

	var goFiles []string
	entries, err := os.ReadDir(path)
	if err != nil {
		return nil, err
	}

	for _, entry := range entries {
		if entry.IsDir() {
			// Skip subdirectories for now
			continue
		}
		if strings.HasSuffix(entry.Name(), ".go") {
			goFiles = append(goFiles, filepath.Join(path, entry.Name()))
		}
	}

	return goFiles, nil
}
