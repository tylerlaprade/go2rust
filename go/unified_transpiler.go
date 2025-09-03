package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

// PackageInfo holds information about a single Go package
type PackageInfo struct {
	ImportPath   string
	GoFiles      []string
	ASTFiles     []*ast.File
	OutputPath   string
	CrateName    string
	Dependencies []string
	IsMain       bool
	PackageName  string
}

// UnifiedTranspiler handles transpilation of main package and all dependencies together
type UnifiedTranspiler struct {
	workDir        string
	vendorDir      string
	mainPackage    *PackageInfo
	vendorPackages map[string]*PackageInfo // import path -> package info
	globalTypeInfo *TypeInfo               // Shared across ALL packages
	packageMapping map[string]string       // Go import -> Rust crate name
	fileSet        *token.FileSet
	allASTFiles    []*ast.File // All AST files from all packages
	rustOutputDir  string      // Temporary directory for Rust output
}

// NewUnifiedTranspiler creates a new unified transpiler
func NewUnifiedTranspiler(workDir string, mainFiles []string) *UnifiedTranspiler {
	return &UnifiedTranspiler{
		workDir:        workDir,
		vendorDir:      filepath.Join(workDir, "vendor"),
		vendorPackages: make(map[string]*PackageInfo),
		packageMapping: make(map[string]string),
		fileSet:        token.NewFileSet(),
		allASTFiles:    []*ast.File{},
		rustOutputDir:  filepath.Join(workDir, ".rust_vendor_output"),
	}
}

// TranspileWithDependencies performs unified transpilation of main package and all dependencies
func (ut *UnifiedTranspiler) TranspileWithDependencies(mainFiles []string, externalPackages map[string]bool) error {
	// Step 1: Set up main package
	if err := ut.setupMainPackage(mainFiles); err != nil {
		return fmt.Errorf("failed to setup main package: %v", err)
	}

	// Step 2: Vendor dependencies
	if len(externalPackages) > 0 {
		fmt.Fprintf(os.Stderr, "Downloading dependencies with 'go mod vendor'...\n")
		if err := ut.runGoModVendor(); err != nil {
			return fmt.Errorf("failed to vendor dependencies: %v", err)
		}

		// Step 3: Discover all packages in vendor
		if err := ut.discoverVendorPackages(externalPackages); err != nil {
			return fmt.Errorf("failed to discover vendor packages: %v", err)
		}
	}

	// Step 4: Parse all files from all packages
	if err := ut.parseAllPackages(); err != nil {
		return fmt.Errorf("failed to parse packages: %v", err)
	}

	// Step 5: Unified type checking
	fmt.Fprintf(os.Stderr, "Performing unified type checking across all packages...\n")
	if err := ut.typeCheckAll(); err != nil {
		// Log warning but continue
		fmt.Fprintf(os.Stderr, "Warning: Type checking incomplete: %v\n", err)
	}

	// Step 6: Transpile all packages with shared type info
	if err := ut.transpileAll(); err != nil {
		return fmt.Errorf("failed to transpile packages: %v", err)
	}

	// Step 7: Clean up vendor directory (remove Go sources, keep only Rust)
	if len(externalPackages) > 0 {
		if err := ut.cleanupVendor(); err != nil {
			return fmt.Errorf("failed to cleanup vendor: %v", err)
		}
	}

	return nil
}

// setupMainPackage initializes the main package info
func (ut *UnifiedTranspiler) setupMainPackage(mainFiles []string) error {
	ut.mainPackage = &PackageInfo{
		ImportPath: "main",
		GoFiles:    mainFiles,
		IsMain:     true,
	}
	return nil
}

// runGoModVendor runs 'go mod vendor' to download dependencies
func (ut *UnifiedTranspiler) runGoModVendor() error {
	cmd := exec.Command("go", "mod", "vendor")
	cmd.Dir = ut.workDir
	cmd.Stdout = os.Stderr
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

// discoverVendorPackages walks the vendor directory to find all packages
func (ut *UnifiedTranspiler) discoverVendorPackages(requestedPackages map[string]bool) error {
	// Process each requested external package
	for pkgPath := range requestedPackages {
		if isStdlibPackage(pkgPath) {
			continue // Skip stdlib packages
		}

		vendorPkgPath := filepath.Join(ut.vendorDir, pkgPath)
		if _, err := os.Stat(vendorPkgPath); os.IsNotExist(err) {
			fmt.Fprintf(os.Stderr, "Warning: Package %s not found in vendor\n", pkgPath)
			continue
		}

		// Find all Go files in the package (non-recursive, no test files)
		goFiles, err := ut.findGoFiles(vendorPkgPath)
		if err != nil {
			return fmt.Errorf("failed to find Go files for %s: %v", pkgPath, err)
		}

		if len(goFiles) == 0 {
			fmt.Fprintf(os.Stderr, "Warning: No Go files found in package %s\n", pkgPath)
			continue
		}

		// Create package info
		crateName := ut.goPathToRustCrate(pkgPath)
		ut.packageMapping[pkgPath] = crateName

		pkg := &PackageInfo{
			ImportPath: pkgPath,
			GoFiles:    goFiles,
			CrateName:  crateName,
			OutputPath: filepath.Join(ut.rustOutputDir, crateName),
			IsMain:     false,
		}

		ut.vendorPackages[pkgPath] = pkg
		fmt.Fprintf(os.Stderr, "Discovered package: %s -> %s\n", pkgPath, crateName)
	}

	return nil
}

// findGoFiles finds all Go files in a directory (non-recursive, excluding test files)
func (ut *UnifiedTranspiler) findGoFiles(dir string) ([]string, error) {
	var goFiles []string

	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}

	for _, entry := range entries {
		if entry.IsDir() {
			continue
		}
		name := entry.Name()
		if strings.HasSuffix(name, ".go") && !strings.HasSuffix(name, "_test.go") {
			goFiles = append(goFiles, filepath.Join(dir, name))
		}
	}

	return goFiles, nil
}

// parseAllPackages parses all Go files from all packages
func (ut *UnifiedTranspiler) parseAllPackages() error {
	// Parse main package files
	fmt.Fprintf(os.Stderr, "Parsing main package files...\n")
	for _, file := range ut.mainPackage.GoFiles {
		ast, err := parser.ParseFile(ut.fileSet, file, nil, parser.ParseComments)
		if err != nil {
			return fmt.Errorf("failed to parse %s: %v", file, err)
		}
		ut.mainPackage.ASTFiles = append(ut.mainPackage.ASTFiles, ast)
		ut.allASTFiles = append(ut.allASTFiles, ast)

		// Get package name from first file
		if ut.mainPackage.PackageName == "" {
			ut.mainPackage.PackageName = ast.Name.Name
		}
	}

	// Parse vendor package files
	for pkgPath, pkg := range ut.vendorPackages {
		fmt.Fprintf(os.Stderr, "Parsing package %s...\n", pkgPath)
		for _, file := range pkg.GoFiles {
			ast, err := parser.ParseFile(ut.fileSet, file, nil, parser.ParseComments)
			if err != nil {
				fmt.Fprintf(os.Stderr, "Warning: Failed to parse %s: %v\n", file, err)
				continue
			}
			pkg.ASTFiles = append(pkg.ASTFiles, ast)
			ut.allASTFiles = append(ut.allASTFiles, ast)

			// Get package name from first file
			if pkg.PackageName == "" {
				pkg.PackageName = ast.Name.Name
			}
		}
	}

	fmt.Fprintf(os.Stderr, "Parsed %d files from %d packages\n",
		len(ut.allASTFiles), len(ut.vendorPackages)+1)
	return nil
}

// typeCheckAll performs unified type checking on all packages together
func (ut *UnifiedTranspiler) typeCheckAll() error {
	// Create unified type info for ALL files
	var err error
	ut.globalTypeInfo, err = NewTypeInfo(ut.allASTFiles, ut.fileSet)
	if err != nil {
		return err
	}

	// Set the global type info so all transpilation functions can use it
	SetTypeInfo(ut.globalTypeInfo)

	// Initialize the goPackageImports map with discovered packages
	ut.initializePackageImports()

	return nil
}

// initializePackageImports populates the global goPackageImports map
func (ut *UnifiedTranspiler) initializePackageImports() {
	// Initialize the global maps
	goPackageImports = make(map[string]string)
	externalPackages = make(map[string]bool)

	// Add all vendor packages to the import map
	for pkgPath, pkg := range ut.vendorPackages {
		// Use the package name as the import alias
		goPackageImports[pkg.PackageName] = pkgPath
		externalPackages[pkgPath] = true
	}

	// Also track imports from main package files
	for _, astFile := range ut.mainPackage.ASTFiles {
		for _, imp := range astFile.Imports {
			// Extract the import path (remove quotes)
			importPath := strings.Trim(imp.Path.Value, `"`)

			// Determine the package name
			var pkgName string
			if imp.Name != nil {
				// Explicit alias
				pkgName = imp.Name.Name
			} else {
				// Use the last segment of the import path as package name
				segments := strings.Split(importPath, "/")
				pkgName = segments[len(segments)-1]

				// Check if we have a vendor package with this path
				if vendorPkg, ok := ut.vendorPackages[importPath]; ok {
					pkgName = vendorPkg.PackageName
				}
			}

			goPackageImports[pkgName] = importPath
			if !isStdlibPackage(importPath) {
				externalPackages[importPath] = true
			}
		}
	}

	fmt.Fprintf(os.Stderr, "Initialized package imports: %v\n", goPackageImports)
}

// transpileAll transpiles all packages using the shared type information
func (ut *UnifiedTranspiler) transpileAll() error {
	// Create output directory for Rust vendor packages
	if len(ut.vendorPackages) > 0 {
		if err := os.MkdirAll(ut.rustOutputDir, 0755); err != nil {
			return fmt.Errorf("failed to create Rust output directory: %v", err)
		}
	}

	// Transpile vendor packages first (dependencies before dependents)
	for pkgPath, pkg := range ut.vendorPackages {
		fmt.Fprintf(os.Stderr, "Transpiling package %s...\n", pkgPath)
		if err := ut.transpilePackage(pkg); err != nil {
			fmt.Fprintf(os.Stderr, "Warning: Failed to transpile %s: %v\n", pkgPath, err)
		}
	}

	// Main package is transpiled by the regular flow, but it now has:
	// - Full type information including vendor packages
	// - Package mapping for import resolution

	return nil
}

// transpilePackage transpiles a single package with shared type info
func (ut *UnifiedTranspiler) transpilePackage(pkg *PackageInfo) error {
	// Create output directory
	if err := os.MkdirAll(pkg.OutputPath, 0755); err != nil {
		return fmt.Errorf("failed to create output directory: %v", err)
	}

	// Generate lib.rs with all modules
	var libRs strings.Builder
	var modules []string

	// Process each file in the package
	for i, astFile := range pkg.ASTFiles {
		// Get base name for module
		baseName := strings.TrimSuffix(filepath.Base(pkg.GoFiles[i]), ".go")
		if baseName == pkg.PackageName {
			baseName = "mod" // Avoid module name collision with package name
		}
		moduleName := SanitizeRustModuleName(baseName)

		// Skip empty files or doc-only files
		if len(astFile.Decls) == 0 {
			continue
		}

		// Transpile the file with shared type info and package mapping
		rustCode, _, _ := TranspileWithMapping(astFile, ut.fileSet, ut.globalTypeInfo, ut.packageMapping)

		// Write the module file
		moduleFile := filepath.Join(pkg.OutputPath, moduleName+".rs")
		if err := os.WriteFile(moduleFile, []byte(rustCode), 0644); err != nil {
			return fmt.Errorf("failed to write module %s: %v", moduleName, err)
		}

		modules = append(modules, moduleName)
	}

	// Generate lib.rs
	for _, mod := range modules {
		libRs.WriteString(fmt.Sprintf("pub mod %s;\n", mod))
	}
	if len(modules) > 0 {
		libRs.WriteString("\n")
		for _, mod := range modules {
			libRs.WriteString(fmt.Sprintf("pub use %s::*;\n", mod))
		}
	}

	// Write lib.rs
	libRsPath := filepath.Join(pkg.OutputPath, "lib.rs")
	if err := os.WriteFile(libRsPath, []byte(libRs.String()), 0644); err != nil {
		return fmt.Errorf("failed to write lib.rs: %v", err)
	}

	// Generate Cargo.toml for the package
	cargoToml := fmt.Sprintf(`[package]
name = "%s"
version = "0.1.0"
edition = "2021"

[lib]
name = "%s"
path = "lib.rs"
`, pkg.CrateName, pkg.CrateName)

	cargoPath := filepath.Join(pkg.OutputPath, "Cargo.toml")
	if err := os.WriteFile(cargoPath, []byte(cargoToml), 0644); err != nil {
		return fmt.Errorf("failed to write Cargo.toml: %v", err)
	}

	return nil
}

// cleanupVendor removes Go source files and keeps only transpiled Rust code
func (ut *UnifiedTranspiler) cleanupVendor() error {
	fmt.Fprintf(os.Stderr, "Cleaning up vendor directory...\n")

	// Remove the entire vendor directory (with Go sources)
	if err := os.RemoveAll(ut.vendorDir); err != nil {
		return fmt.Errorf("failed to remove vendor directory: %v", err)
	}

	// Move our clean Rust output to vendor location
	if err := os.Rename(ut.rustOutputDir, ut.vendorDir); err != nil {
		return fmt.Errorf("failed to move Rust output to vendor: %v", err)
	}

	fmt.Fprintf(os.Stderr, "Vendor directory cleaned - only Rust code remains\n")
	return nil
}

// goPathToRustCrate converts a Go import path to a Rust-compatible crate name
func (ut *UnifiedTranspiler) goPathToRustCrate(goPath string) string {
	// Replace special characters with underscores
	crate := strings.ReplaceAll(goPath, "/", "_")
	crate = strings.ReplaceAll(crate, ".", "_")
	crate = strings.ReplaceAll(crate, "-", "_")

	// Ensure it starts with a letter
	if len(crate) > 0 && (crate[0] >= '0' && crate[0] <= '9') {
		crate = "pkg_" + crate
	}

	return crate
}

// GetPackageMapping returns the Go import path to Rust crate name mapping
func (ut *UnifiedTranspiler) GetPackageMapping() map[string]string {
	return ut.packageMapping
}
