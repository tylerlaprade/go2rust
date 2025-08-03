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
	goFiles     []string
	moduleNames []string
	hasMain     bool
	projectPath string
	packageName string
	isLibrary   bool
	typeInfo    *TypeInfo
}

func NewProjectGenerator(goFiles []string) *ProjectGenerator {
	if len(goFiles) == 0 {
		return nil
	}

	return &ProjectGenerator{
		goFiles:     goFiles,
		projectPath: filepath.Dir(goFiles[0]),
	}
}

func (pg *ProjectGenerator) Generate() error {
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

	// First pass: transpile all files and detect structure
	for i, filename := range pg.goFiles {
		// Use the already parsed file from astFiles
		file := astFiles[i]

		// Detect package name from first file
		if i == 0 {
			pg.packageName = file.Name.Name
			pg.isLibrary = pg.packageName != "main"
		}

		rustCode := Transpile(file, fileSet, pg.typeInfo)

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

	mainContent := Transpile(file, fileSet, pg.typeInfo)
	mainRust.WriteString(mainContent)

	mainRsPath := filepath.Join(pg.projectPath, "main.rs")
	return os.WriteFile(mainRsPath, []byte(mainRust.String()), 0644)
}

func (pg *ProjectGenerator) generateCargoToml() error {
	cargoPath := filepath.Join(pg.projectPath, "Cargo.toml")

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
		cargoContent = `[package]
name = "transpiled"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "transpiled"
path = "main.rs"
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
