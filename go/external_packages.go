package main

import (
	"bufio"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

// ExternalPackageHandler handles transpilation of external packages
type ExternalPackageHandler struct {
	workDir           string
	processedPackages map[string]bool   // Track already processed packages to avoid cycles
	packageMapping    map[string]string // Maps Go import path to Rust crate name
	vendorDir         string
	modulePath        string // The module path from go.mod
}

// NewExternalPackageHandler creates a new handler for external packages
func NewExternalPackageHandler(workDir string) *ExternalPackageHandler {
	return &ExternalPackageHandler{
		workDir:           workDir,
		processedPackages: make(map[string]bool),
		packageMapping:    make(map[string]string),
		vendorDir:         filepath.Join(workDir, "vendor"),
	}
}

// HandleExternalPackages processes all external packages found during transpilation
func (h *ExternalPackageHandler) HandleExternalPackages(packages map[string]bool) error {
	// First, ensure we have a go.mod file for the main project
	goModPath := filepath.Join(h.workDir, "go.mod")
	if _, err := os.Stat(goModPath); os.IsNotExist(err) {
		return fmt.Errorf("go.mod not found in %s. External packages require a Go module", h.workDir)
	}

	// Parse go.mod to get the module path
	if err := h.parseGoMod(goModPath); err != nil {
		return fmt.Errorf("failed to parse go.mod: %v", err)
	}

	// Run go mod vendor to download all dependencies
	fmt.Fprintf(os.Stderr, "Downloading dependencies with 'go mod vendor'...\n")
	if err := h.runGoModVendor(); err != nil {
		return fmt.Errorf("failed to vendor dependencies: %v", err)
	}

	// Process each external package
	for pkg := range packages {
		if err := h.transpilePackage(pkg); err != nil {
			// Log error but continue with other packages
			fmt.Fprintf(os.Stderr, "Warning: Failed to transpile package %s: %v\n", pkg, err)
		}
	}

	// Update Cargo.toml to be a workspace after processing packages (now we have the mapping)
	// This will be done after the main Cargo.toml is generated
	// For now, we'll store the mapping for the ProjectGenerator to use

	return nil
}

// parseGoMod extracts the module path from go.mod
func (h *ExternalPackageHandler) parseGoMod(goModPath string) error {
	file, err := os.Open(goModPath)
	if err != nil {
		return err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if strings.HasPrefix(line, "module ") {
			h.modulePath = strings.TrimSpace(strings.TrimPrefix(line, "module"))
			return nil
		}
	}

	return fmt.Errorf("module directive not found in go.mod")
}

// runGoModVendor runs 'go mod vendor' to download dependencies
func (h *ExternalPackageHandler) runGoModVendor() error {
	cmd := exec.Command("go", "mod", "vendor")
	cmd.Dir = h.workDir
	cmd.Stdout = os.Stderr
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

// createRustWorkspace updates the Cargo.toml to be a workspace
func (h *ExternalPackageHandler) updateCargoWorkspace() error {
	// The Cargo.toml should already exist from the main project generation
	cargoPath := filepath.Join(h.workDir, "Cargo.toml")
	existingBytes, err := os.ReadFile(cargoPath)
	if err != nil {
		// If it doesn't exist, we'll need to wait for it to be created
		return nil
	}
	existingContent := string(existingBytes)

	// Create workspace configuration
	workspaceSection := "[workspace]\nmembers = [\n    \".\",\n"

	// Add external packages as workspace members
	for pkg := range h.packageMapping {
		crateName := h.packageMapping[pkg]
		workspaceSection += fmt.Sprintf("    \"vendor/%s\",\n", crateName)
	}
	workspaceSection += "]\n\n"

	// Add dependencies section for external packages
	depsSection := "[dependencies]\n"
	for pkg := range h.packageMapping {
		crateName := h.packageMapping[pkg]
		depsSection += fmt.Sprintf("%s = { path = \"vendor/%s\" }\n", crateName, crateName)
	}

	// Combine workspace section with existing package configuration
	updatedContent := workspaceSection + depsSection + "\n" + existingContent

	// Write updated Cargo.toml
	return os.WriteFile(cargoPath, []byte(updatedContent), 0644)
}

// transpilePackage recursively transpiles an external package
func (h *ExternalPackageHandler) transpilePackage(pkgPath string) error {
	// Skip stdlib packages
	if isStdlibPackage(pkgPath) {
		return nil
	}

	// Check if already processed
	if h.processedPackages[pkgPath] {
		return nil
	}
	h.processedPackages[pkgPath] = true

	fmt.Fprintf(os.Stderr, "Transpiling external package: %s\n", pkgPath)

	// Find the package in vendor directory
	vendorPkgPath := filepath.Join(h.vendorDir, pkgPath)
	if _, err := os.Stat(vendorPkgPath); os.IsNotExist(err) {
		return fmt.Errorf("package %s not found in vendor directory", pkgPath)
	}

	// Generate a Rust-compatible crate name
	crateName := h.goPathToRustCrate(pkgPath)
	h.packageMapping[pkgPath] = crateName

	// Create output directory for the transpiled package
	outputDir := filepath.Join(h.workDir, "vendor", crateName)
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		return fmt.Errorf("failed to create output directory: %v", err)
	}

	// Find all Go files in the package
	goFiles, err := h.findGoFiles(vendorPkgPath)
	if err != nil {
		return fmt.Errorf("failed to find Go files: %v", err)
	}

	if len(goFiles) == 0 {
		return fmt.Errorf("no Go files found in package %s", pkgPath)
	}

	// Create a new project generator for this package
	generator := NewProjectGenerator(goFiles)
	if generator == nil {
		return fmt.Errorf("failed to create project generator for %s", pkgPath)
	}

	// Set to none mode for vendor packages (they shouldn't have external deps in vendor)
	generator.SetExternalPackageMode(ModeNone)

	// Override the project path to output to vendor directory
	generator.projectPath = outputDir

	// For vendor packages, we don't require go.mod
	generator.isVendorPackage = true

	// Generate the transpiled package
	if err := generator.GeneratePackage(); err != nil {
		return fmt.Errorf("failed to transpile package %s: %v", pkgPath, err)
	}

	// Process any new external packages found during transpilation
	if generator.hasExternalPackages() {
		for _, imports := range generator.goImports {
			for _, importPath := range imports {
				if !h.processedPackages[importPath] {
					if err := h.transpilePackage(importPath); err != nil {
						fmt.Fprintf(os.Stderr, "Warning: Failed to transpile dependency %s: %v\n", importPath, err)
					}
				}
			}
		}
	}

	return nil
}

// findGoFiles finds all Go files in a directory (non-recursive)
func (h *ExternalPackageHandler) findGoFiles(dir string) ([]string, error) {
	var goFiles []string

	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}

	for _, entry := range entries {
		if entry.IsDir() {
			continue
		}
		if strings.HasSuffix(entry.Name(), ".go") && !strings.HasSuffix(entry.Name(), "_test.go") {
			goFiles = append(goFiles, filepath.Join(dir, entry.Name()))
		}
	}

	return goFiles, nil
}

// goPathToRustCrate converts a Go import path to a Rust-compatible crate name
func (h *ExternalPackageHandler) goPathToRustCrate(goPath string) string {
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

// GetPackageMapping returns the mapping from Go import paths to Rust crate names
func (h *ExternalPackageHandler) GetPackageMapping() map[string]string {
	return h.packageMapping
}

// UpdateImportPaths updates import statements in transpiled code to use Rust crate names
func (h *ExternalPackageHandler) UpdateImportPaths(rustCode string) string {
	result := rustCode

	// Replace Go import paths with Rust crate names in use statements
	for goPath, crateName := range h.packageMapping {
		// This is a simplified approach - in reality we'd need to parse the Rust AST
		// For now, we'll do simple string replacement
		oldImport := fmt.Sprintf("use %s", goPath)
		newImport := fmt.Sprintf("use %s", crateName)
		result = strings.ReplaceAll(result, oldImport, newImport)
	}

	return result
}
