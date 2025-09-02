package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"strings"
)

type ProjectGenerator struct {
	goFiles        []string
	projectPath    string
	packageName    string
	isLibrary      bool
	hasMain        bool
	moduleNames    []string
	typeInfo       *TypeInfo
	projectImports *ImportTracker // Collect imports across all files
	externalMode   ExternalPackageMode
	goImports      map[string][]string // package path -> list of imports
}

func NewProjectGenerator(goFiles []string) *ProjectGenerator {
	if len(goFiles) == 0 {
		return nil
	}
	return &ProjectGenerator{
		goFiles:        goFiles,
		projectPath:    filepath.Dir(goFiles[0]),
		projectImports: NewImportTracker(),
		externalMode:   ModeTranspile, // Default to transpile mode
		goImports:      make(map[string][]string),
	}
}

// SetExternalPackageMode sets how external packages should be handled
func (pg *ProjectGenerator) SetExternalPackageMode(mode ExternalPackageMode) {
	pg.externalMode = mode
}

// checkForExternalPackages scans for external package imports when mode is 'none'
func (pg *ProjectGenerator) checkForExternalPackages() error {
	fileSet := token.NewFileSet()

	for _, filename := range pg.goFiles {
		file, err := parser.ParseFile(fileSet, filename, nil, parser.ImportsOnly)
		if err != nil {
			continue // Skip files with parse errors
		}

		for _, imp := range file.Imports {
			path := strings.Trim(imp.Path.Value, `"`)
			if !isStdlibPackage(path) {
				return fmt.Errorf("external package import not allowed with --external-packages=none: %s in %s", path, filename)
			}
		}
	}

	return nil
}

func (pg *ProjectGenerator) Generate() error {
	// Check for external packages first if mode is 'none'
	if pg.externalMode == ModeNone {
		if err := pg.checkForExternalPackages(); err != nil {
			return err
		}
	}
	fileSet := token.NewFileSet()

	// Parse all files first for type checking
	var astFiles []*ast.File
	for _, filename := range pg.goFiles {
		file, err := parser.ParseFile(fileSet, filename, nil, parser.ParseComments)
		if err != nil {
			return fmt.Errorf("parse error in %s: %v", filename, err)
		}
		astFiles = append(astFiles, file)
	}

	// Perform type checking
	typeInfo, err := NewTypeInfo(astFiles, fileSet)
	if err != nil {
		// Log warning but continue - we'll handle missing type info in individual functions
		fmt.Fprintf(os.Stderr, "Warning: Type checking incomplete: %v\n", err)
		fmt.Fprintf(os.Stderr, "Generated code may contain errors where type information is required\n")
	}
	pg.typeInfo = typeInfo

	// Set the global type info once for the entire project
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil) // Clear when done

	// Detect concurrency in the project
	concurrencyDetector := NewConcurrencyDetector()
	concurrencyDetector.AnalyzeProject(astFiles)
	SetConcurrencyDetector(concurrencyDetector)
	defer SetConcurrencyDetector(nil) // Clear when done

	// First pass: transpile all files and detect structure
	for i, filename := range pg.goFiles {
		// Use the already parsed file from astFiles
		file := astFiles[i]

		// Detect package name from first file
		if i == 0 {
			pg.packageName = file.Name.Name
			pg.isLibrary = pg.packageName != "main"
		}

		rustCode, fileImports, fileExternalPkgs := Transpile(file, fileSet, pg.typeInfo)

		// Track external packages found
		for pkg := range fileExternalPkgs {
			if pg.goImports[filename] == nil {
				pg.goImports[filename] = []string{}
			}
			pg.goImports[filename] = append(pg.goImports[filename], pkg)
		}

		// Merge file imports into project imports
		if fileImports != nil {
			for imp := range fileImports.needs {
				pg.projectImports.Add(imp)
			}
		}

		baseName := strings.TrimSuffix(filepath.Base(filename), ".go")
		rustFilename := strings.TrimSuffix(filename, ".go") + ".rs"

		// Check if this is main.go
		if baseName == "main" && file.Name.Name == "main" {
			pg.hasMain = true
			// We'll handle main.go specially later
			continue
		}

		// For lib.go in a binary crate, rename to avoid Rust warnings
		outputName := baseName
		if pg.hasMainFile() && strings.HasPrefix(baseName, "lib") && strings.TrimLeft(baseName[3:], "_") == "" {
			outputName = baseName + "_"
			rustFilename = strings.TrimSuffix(filename, ".go") + "_.rs"
		}

		// Write module file
		err = os.WriteFile(rustFilename, []byte(rustCode), 0644)
		if err != nil {
			return fmt.Errorf("error writing %s: %v", rustFilename, err)
		}

		pg.moduleNames = append(pg.moduleNames, outputName)
	}

	// Handle external packages based on mode
	if len(pg.goImports) > 0 && pg.hasExternalPackages() {
		switch pg.externalMode {
		case ModeTranspile:
			// TODO: Implement recursive transpilation
			fmt.Fprintf(os.Stderr, "Warning: Recursive transpilation of external packages not yet implemented\n")
			fmt.Fprintf(os.Stderr, "External packages found:\n")
			for _, imports := range pg.goImports {
				for _, pkg := range imports {
					fmt.Fprintf(os.Stderr, "  - %s\n", pkg)
				}
			}
		case ModeFfi:
			// TODO: Implement FFI bridge generation
			fmt.Fprintf(os.Stderr, "Warning: FFI bridge generation not yet implemented\n")
			fmt.Fprintf(os.Stderr, "External packages found:\n")
			for _, imports := range pg.goImports {
				for _, pkg := range imports {
					fmt.Fprintf(os.Stderr, "  - %s\n", pkg)
				}
			}
		case ModeNone:
			// This should have been caught earlier, but double-check
			return fmt.Errorf("external packages found but mode is 'none'")
		}
	}

	// Second pass: generate main.rs or lib.rs with module declarations
	if pg.hasMain {
		err := pg.generateMainRs(fileSet, astFiles)
		if err != nil {
			return err
		}
	} else if pg.isLibrary {
		err := pg.generateLibRs()
		if err != nil {
			return err
		}
	}

	return pg.generateCargoToml()
}

// hasExternalPackages checks if any external packages were found
func (pg *ProjectGenerator) hasExternalPackages() bool {
	for _, imports := range pg.goImports {
		if len(imports) > 0 {
			return true
		}
	}
	return false
}

func (pg *ProjectGenerator) hasMainFile() bool {
	for _, file := range pg.goFiles {
		if filepath.Base(file) == "main.go" {
			return true
		}
	}
	return false
}

func (pg *ProjectGenerator) generateMainRs(fileSet *token.FileSet, astFiles []*ast.File) error {
	var mainGoFile *ast.File
	for i, filename := range pg.goFiles {
		if filepath.Base(filename) == "main.go" {
			mainGoFile = astFiles[i]
			break
		}
	}

	if mainGoFile == nil {
		return fmt.Errorf("main.go not found")
	}

	file := mainGoFile

	var mainRust strings.Builder

	for _, modName := range pg.moduleNames {
		mainRust.WriteString(fmt.Sprintf("mod %s;\n", modName))
	}
	if len(pg.moduleNames) > 0 {
		for _, modName := range pg.moduleNames {
			mainRust.WriteString(fmt.Sprintf("use %s::*;\n", modName))
		}
		mainRust.WriteString("\n")
	}

	mainContent, mainImports, mainExternalPkgs := Transpile(file, fileSet, pg.typeInfo)

	// Track external packages from main
	mainPath := ""
	for _, fname := range pg.goFiles {
		if filepath.Base(fname) == "main.go" {
			mainPath = fname
			break
		}
	}
	if mainPath != "" {
		for pkg := range mainExternalPkgs {
			if pg.goImports[mainPath] == nil {
				pg.goImports[mainPath] = []string{}
			}
			pg.goImports[mainPath] = append(pg.goImports[mainPath], pkg)
		}
	}

	// Merge main imports into project imports
	if mainImports != nil {
		for imp := range mainImports.needs {
			pg.projectImports.Add(imp)
		}
	}

	mainRust.WriteString(mainContent)

	mainRsPath := filepath.Join(pg.projectPath, "main.rs")
	return os.WriteFile(mainRsPath, []byte(mainRust.String()), 0644)
}

func (pg *ProjectGenerator) generateCargoToml() error {
	cargoPath := filepath.Join(pg.projectPath, "Cargo.toml")

	// Check if we need the num crate from project imports
	needsNum := false
	if pg.projectImports != nil && pg.projectImports.needs["num::Complex"] {
		needsNum = true
	}

	var cargoContent string
	if pg.isLibrary {
		cargoContent = fmt.Sprintf(`[package]
name = "%s"
version = "0.1.0"
edition = "2021"

[lib]
name = "%s"
path = "lib.rs"
`, pg.packageName, pg.packageName)
	} else {
		// Use directory name as package name for better test isolation
		packageName := filepath.Base(pg.projectPath)
		if packageName == "." {
			// If running in current directory, use "transpiled" as default name
			packageName = "transpiled"
		}
		// Sanitize package name to ensure it's valid for Cargo
		packageName = strings.ReplaceAll(packageName, "-", "_")
		packageName = strings.ReplaceAll(packageName, " ", "_")

		cargoContent = fmt.Sprintf(`[package]
name = "%s"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "%s"
path = "main.rs"
`, packageName, packageName)
	}

	// Add dependencies if needed
	if needsNum {
		cargoContent += `
[dependencies]
num = "0.4"
`
	}

	return os.WriteFile(cargoPath, []byte(cargoContent), 0644)
}

func (pg *ProjectGenerator) generateLibRs() error {
	var libRust strings.Builder

	for _, modName := range pg.moduleNames {
		libRust.WriteString(fmt.Sprintf("pub mod %s;\n", modName))
	}

	// Re-export everything from modules
	if len(pg.moduleNames) > 0 {
		libRust.WriteString("\n")
		for _, modName := range pg.moduleNames {
			libRust.WriteString(fmt.Sprintf("pub use %s::*;\n", modName))
		}
	}

	libRsPath := filepath.Join(pg.projectPath, "lib.rs")
	return os.WriteFile(libRsPath, []byte(libRust.String()), 0644)
}
