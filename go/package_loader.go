package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"golang.org/x/tools/go/packages"
	"os"
	"path/filepath"
	"strings"
)

// PackageLoader loads Go packages with full type information
type PackageLoader struct {
	workDir        string
	mainPkg        *packages.Package
	allPackages    map[string]*packages.Package // import path -> package
	packageMapping map[string]string            // Go import -> Rust crate name
	fileSet        *token.FileSet
}

// NewPackageLoader creates a new package loader
func NewPackageLoader(workDir string) *PackageLoader {
	return &PackageLoader{
		workDir:        workDir,
		allPackages:    make(map[string]*packages.Package),
		packageMapping: make(map[string]string),
	}
}

// LoadWithDependencies loads the main package and all its dependencies with full type info
func (pl *PackageLoader) LoadWithDependencies(patterns []string) error {
	// Configure package loading to get everything we need
	cfg := &packages.Config{
		Mode: packages.NeedName |
			packages.NeedFiles |
			packages.NeedCompiledGoFiles |
			packages.NeedImports |
			packages.NeedDeps |
			packages.NeedTypes |
			packages.NeedSyntax |
			packages.NeedTypesInfo,
		Dir:   pl.workDir,
		Tests: false,
	}

	// Load the packages
	pkgs, err := packages.Load(cfg, patterns...)
	if err != nil {
		return fmt.Errorf("failed to load packages: %v", err)
	}

	// Check for errors but continue
	for _, pkg := range pkgs {
		if len(pkg.Errors) > 0 {
			for _, err := range pkg.Errors {
				fmt.Fprintf(os.Stderr, "Package loading warning: %v\n", err)
			}
		}
	}

	if len(pkgs) == 0 {
		return fmt.Errorf("no packages found")
	}

	// Log what we found
	for _, pkg := range pkgs {
		fmt.Fprintf(os.Stderr, "Loaded package: %s\n", pkg.PkgPath)
	}

	// Store the main package
	pl.mainPkg = pkgs[0]
	pl.fileSet = pl.mainPkg.Fset

	fmt.Fprintf(os.Stderr, "Main package has %d imports\n", len(pl.mainPkg.Imports))
	for path, pkg := range pl.mainPkg.Imports {
		fmt.Fprintf(os.Stderr, "  Import: %s -> %s\n", path, pkg.PkgPath)
	}

	// Collect all packages (including transitive dependencies)
	pl.collectAllPackages(pl.mainPkg)

	fmt.Fprintf(os.Stderr, "Loaded %d packages with full type information\n", len(pl.allPackages))
	return nil
}

// collectAllPackages recursively collects all packages
func (pl *PackageLoader) collectAllPackages(pkg *packages.Package) {
	if pkg == nil {
		return
	}

	// Handle main package specially
	if pkg.PkgPath == "" || pkg.PkgPath == "simpletest" || pkg.PkgPath == "testpkg" {
		pkg.PkgPath = "main"
	}

	// Skip if already processed
	if _, exists := pl.allPackages[pkg.PkgPath]; exists {
		return
	}

	// Skip standard library packages (we don't transpile those)
	// External packages have dots in their path (e.g., github.com/...)
	isExternal := strings.Contains(pkg.PkgPath, ".")
	isMain := pkg.PkgPath == "main"

	if !isExternal && !isMain {
		// Standard library package - skip
		return
	}

	// Store the package
	pl.allPackages[pkg.PkgPath] = pkg

	// Generate Rust crate name for external packages
	if pkg.PkgPath != "main" && strings.Contains(pkg.PkgPath, ".") {
		crateName := pl.goPathToRustCrate(pkg.PkgPath)
		pl.packageMapping[pkg.PkgPath] = crateName
		fmt.Fprintf(os.Stderr, "Found package: %s -> %s\n", pkg.PkgPath, crateName)
	}

	// Recursively process imports
	for _, imp := range pkg.Imports {
		pl.collectAllPackages(imp)
	}
}

// TranspileAll transpiles all loaded packages
func (pl *PackageLoader) TranspileAll() error {
	// Create output directory for external packages
	vendorDir := filepath.Join(pl.workDir, "vendor")
	if len(pl.packageMapping) > 0 {
		if err := os.MkdirAll(vendorDir, 0755); err != nil {
			return fmt.Errorf("failed to create vendor directory: %v", err)
		}
	}

	// Set the global type info from the main package
	// This gives us access to all type information including imports
	globalTypeInfo := &TypeInfo{
		info: pl.mainPkg.TypesInfo,
		pkg:  pl.mainPkg.Types,
	}
	SetTypeInfo(globalTypeInfo)

	// Transpile external packages first
	for pkgPath, pkg := range pl.allPackages {
		if pkgPath == "main" || pkgPath == pl.mainPkg.PkgPath {
			continue // Skip main package (handled separately)
		}

		// Skip standard library
		if !strings.Contains(pkgPath, ".") {
			continue
		}

		fmt.Fprintf(os.Stderr, "Transpiling package %s...\n", pkgPath)
		if err := pl.transpilePackage(pkg); err != nil {
			fmt.Fprintf(os.Stderr, "Warning: Failed to transpile %s: %v\n", pkgPath, err)
		}
	}

	// Main package is transpiled by the regular ProjectGenerator flow
	// but now with full type information available

	return nil
}

// transpilePackage transpiles a single package
func (pl *PackageLoader) transpilePackage(pkg *packages.Package) error {
	if len(pkg.Syntax) == 0 {
		return fmt.Errorf("no syntax trees for package %s", pkg.PkgPath)
	}

	crateName := pl.packageMapping[pkg.PkgPath]
	if crateName == "" {
		return fmt.Errorf("no crate name for package %s", pkg.PkgPath)
	}

	outputDir := filepath.Join(pl.workDir, "vendor", crateName)
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		return fmt.Errorf("failed to create output directory: %v", err)
	}

	// Create TypeInfo for this package
	pkgTypeInfo := &TypeInfo{
		info: pkg.TypesInfo,
		pkg:  pkg.Types,
	}

	// Generate lib.rs with all modules
	var libRs strings.Builder
	var modules []string

	// Process each file in the package
	for i, astFile := range pkg.Syntax {
		if len(astFile.Decls) == 0 {
			continue // Skip empty files
		}

		// Get the file name
		var fileName string
		if i < len(pkg.CompiledGoFiles) {
			fileName = filepath.Base(pkg.CompiledGoFiles[i])
		} else {
			fileName = fmt.Sprintf("file%d.go", i)
		}

		baseName := strings.TrimSuffix(fileName, ".go")
		if baseName == pkg.Name {
			baseName = "mod" // Avoid name collision
		}
		moduleName := SanitizeRustModuleName(baseName)

		// Transpile with the package's type info and global package mapping
		rustCode, _, _ := TranspileWithMapping(astFile, pkg.Fset, pkgTypeInfo, pl.packageMapping)

		// Write the module file
		moduleFile := filepath.Join(outputDir, moduleName+".rs")
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
	libRsPath := filepath.Join(outputDir, "lib.rs")
	if err := os.WriteFile(libRsPath, []byte(libRs.String()), 0644); err != nil {
		return fmt.Errorf("failed to write lib.rs: %v", err)
	}

	// Generate Cargo.toml
	cargoToml := fmt.Sprintf(`[package]
name = "%s"
version = "0.1.0"
edition = "2021"

[lib]
name = "%s"
path = "lib.rs"
`, crateName, crateName)

	cargoPath := filepath.Join(outputDir, "Cargo.toml")
	if err := os.WriteFile(cargoPath, []byte(cargoToml), 0644); err != nil {
		return fmt.Errorf("failed to write Cargo.toml: %v", err)
	}

	return nil
}

// goPathToRustCrate converts a Go import path to a Rust-compatible crate name
func (pl *PackageLoader) goPathToRustCrate(goPath string) string {
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

// GetPackageMapping returns the package mapping
func (pl *PackageLoader) GetPackageMapping() map[string]string {
	return pl.packageMapping
}

// GetMainPackage returns the main package with all type info
func (pl *PackageLoader) GetMainPackage() *packages.Package {
	return pl.mainPkg
}

// GetMainAST returns the AST files from the main package
func (pl *PackageLoader) GetMainAST() []*ast.File {
	if pl.mainPkg == nil {
		return nil
	}
	return pl.mainPkg.Syntax
}

// GetMainImports returns the import mapping for the main package
func (pl *PackageLoader) GetMainImports() map[string]string {
	imports := make(map[string]string)
	if pl.mainPkg == nil {
		return imports
	}

	// Build import map from package imports
	for path, pkg := range pl.mainPkg.Imports {
		if pkg == nil {
			continue
		}
		// Use the package name as the key
		name := pkg.Name
		if name == "" {
			// Default to last component of path
			parts := strings.Split(path, "/")
			name = parts[len(parts)-1]
		}
		imports[name] = path
	}

	return imports
}

// GetTypeInfo returns the global type info from the main package
func (pl *PackageLoader) GetTypeInfo() *TypeInfo {
	if pl.mainPkg == nil {
		return nil
	}
	return &TypeInfo{
		info: pl.mainPkg.TypesInfo,
		pkg:  pl.mainPkg.Types,
	}
}
