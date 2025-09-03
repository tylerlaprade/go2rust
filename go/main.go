package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

var (
	externalPackagesFlag = flag.String("external-packages", "transpile", "How to handle external packages: transpile, ffi, or none")
	helpFlag             = flag.Bool("help", false, "Show help message")
)

func main() {
	flag.Parse()

	if *helpFlag {
		showHelp()
		os.Exit(0)
	}

	externalMode, err := ParseExternalPackageMode(*externalPackagesFlag)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}

	args := flag.Args()
	if len(args) == 0 {
		fmt.Fprintf(os.Stderr, "Usage: %s [options] <go-file-or-directory>...\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "Run '%s -help' for more information\n", os.Args[0])
		os.Exit(1)
	}

	var goFiles []string
	for _, arg := range args {
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

	// Set the external package mode
	generator.SetExternalPackageMode(externalMode)

	genErr := generator.Generate()
	if genErr != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", genErr)
		os.Exit(1)
	}
}

func showHelp() {
	fmt.Printf("go2rust - Go to Rust transpiler\n\n")
	fmt.Printf("Usage: %s [options] <go-file-or-directory>...\n\n", os.Args[0])
	fmt.Printf("Options:\n")
	fmt.Printf("  -external-packages <mode>  How to handle external packages (default: transpile)\n")
	fmt.Printf("                             Modes:\n")
	fmt.Printf("                               transpile - Recursively transpile all dependencies\n")
	fmt.Printf("                               ffi       - Generate FFI bridge to Go libraries\n")
	fmt.Printf("                               stub      - Generate stub implementations for manual completion\n")
	fmt.Printf("                               none      - Error on external imports\n")
	fmt.Printf("  -help                      Show this help message\n\n")
	fmt.Printf("Examples:\n")
	fmt.Printf("  %s main.go                                    # Transpile with default settings\n", os.Args[0])
	fmt.Printf("  %s -external-packages=ffi ./cmd/myapp        # Use FFI for external packages\n", os.Args[0])
	fmt.Printf("  %s -external-packages=stub mycode.go         # Generate stubs for external deps\n", os.Args[0])
	fmt.Printf("  %s -external-packages=none simple.go         # Fail on external imports\n", os.Args[0])
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
