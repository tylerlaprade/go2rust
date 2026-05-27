package main

import (
	"flag"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

var (
	externalPackagesFlag    = flag.String("external-packages", "transpile", "How to handle external packages: transpile, ffi, or none")
	sourceStdlibPackagesArg = flag.String("source-stdlib-packages", "", "Comma-separated stdlib import paths to transpile from source instead of using the bridge (e.g., 'path/filepath,go/token'). Also accepts 'all' or 'std' to opt in everything; suffix '/...' for a subtree. Same effect as setting GO2RUST_SOURCE_STDLIB_PACKAGES.")
	rawFlag                 = flag.String("raw", "", "Transpile a raw Go source string and write Rust to stdout. Use '-' to read source from stdin. Skips disk I/O and dependency resolution; intended for single-file snippets.")
	helpFlag                = flag.Bool("help", false, "Show help message")
)

func main() {
	flag.Parse()

	if *helpFlag {
		showHelp()
		os.Exit(0)
	}

	// CLI flag overrides env var if explicitly set, otherwise leaves the env
	// var alone so existing callers and Go unit tests keep working.
	if *sourceStdlibPackagesArg != "" {
		os.Setenv("GO2RUST_SOURCE_STDLIB_PACKAGES", *sourceStdlibPackagesArg)
	}

	externalMode, err := ParseExternalPackageMode(*externalPackagesFlag)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}

	if *rawFlag != "" {
		source := *rawFlag
		if source == "-" {
			data, err := io.ReadAll(os.Stdin)
			if err != nil {
				fmt.Fprintf(os.Stderr, "Error reading stdin: %v\n", err)
				os.Exit(1)
			}
			source = string(data)
		}
		rust, err := TranspileSource(source)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
		fmt.Print(rust)
		return
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
	fmt.Printf("  --external-packages <mode>      How to handle external packages (default: transpile)\n")
	fmt.Printf("                                  Modes:\n")
	fmt.Printf("                                    transpile - Recursively transpile all dependencies\n")
	fmt.Printf("                                    ffi       - Generate FFI bridge to Go libraries\n")
	fmt.Printf("                                    stub      - Generate stub implementations for manual completion\n")
	fmt.Printf("                                    none      - Error on external imports\n")
	fmt.Printf("  --source-stdlib-packages <list> Comma-separated stdlib import paths to transpile from source\n")
	fmt.Printf("                                  instead of using the bridge. Supports 'all', 'std', and\n")
	fmt.Printf("                                  '<prefix>/...' subtree patterns. Mirrors the\n")
	fmt.Printf("                                  GO2RUST_SOURCE_STDLIB_PACKAGES env var.\n")
	fmt.Printf("  --raw <source>                  Transpile a raw Go source string and write Rust to stdout.\n")
	fmt.Printf("                                  Use '-' to read source from stdin. Skips disk I/O and\n")
	fmt.Printf("                                  dependency resolution; intended for single-file snippets.\n")
	fmt.Printf("  --help                          Show this help message\n\n")
	fmt.Printf("Examples:\n")
	fmt.Printf("  %s main.go                                                 # Transpile with default settings\n", os.Args[0])
	fmt.Printf("  %s --external-packages=ffi ./cmd/myapp                    # Use FFI for external packages\n", os.Args[0])
	fmt.Printf("  %s --external-packages=stub mycode.go                     # Generate stubs for external deps\n", os.Args[0])
	fmt.Printf("  %s --external-packages=none simple.go                     # Fail on external imports\n", os.Args[0])
	fmt.Printf("  %s --source-stdlib-packages=path/filepath ./tests/foo     # Transpile path/filepath from source\n", os.Args[0])
	fmt.Printf("  %s --raw 'package main\\nfunc main(){println(\"hi\")}'     # Transpile a Go snippet to stdout\n", os.Args[0])
	fmt.Printf("  echo 'package main\\nfunc main(){}' | %s --raw -          # Read Go source from stdin\n", os.Args[0])
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
		if strings.HasSuffix(entry.Name(), ".go") && !strings.HasSuffix(entry.Name(), "_test.go") {
			goFiles = append(goFiles, filepath.Join(path, entry.Name()))
		}
	}

	return goFiles, nil
}
