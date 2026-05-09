package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

type ProjectGenerator struct {
	goFiles                  []string
	projectPath              string
	packageName              string
	isLibrary                bool
	hasMain                  bool
	moduleNames              []string
	initModuleNames          []string
	typeInfo                 *TypeInfo
	projectImports           *ImportTracker // Collect imports across all files
	externalMode             ExternalPackageMode
	goImports                map[string][]string // package path -> list of imports
	packageMapping           map[string]string   // Go import path -> Rust crate name
	isVendorPackage          bool                // True if this is a vendor package (no go.mod required)
	useSharedStdlibStubCrate bool                // True when transpiled packages share one stdlib stub crate
	usePackageHelpers        bool                // True when helper definitions must be shared across generated modules
}

const packageHelperIncludeFile = "__go2rust_helpers.rs"

type generatedRustModule struct {
	name     string
	path     string
	rustCode string
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

// GeneratePackage generates a package without handling external dependencies (for vendor packages)
func (pg *ProjectGenerator) GeneratePackage() error {
	// Skip external package checks and handling for vendor packages
	return pg.generateInternal(true)
}

func (pg *ProjectGenerator) Generate() error {
	// Check for external packages first if mode is 'none'
	if pg.externalMode == ModeNone {
		if err := pg.checkForExternalPackages(); err != nil {
			return err
		}
	}
	return pg.generateInternal(false)
}

func (pg *ProjectGenerator) generateInternal(skipExternalHandling bool) error {
	fileSet := token.NewFileSet()
	var packageImports map[string]string
	var packageLoader *PackageLoader
	var workspaceConcurrencyDetector *ConcurrencyDetector

	// Parse all files first for type checking
	var astFiles []*ast.File
	astFilesByPath := make(map[string]*ast.File, len(pg.goFiles))
	for _, filename := range pg.goFiles {
		file, err := parser.ParseFile(fileSet, filename, nil, parser.ParseComments)
		if err != nil {
			return fmt.Errorf("parse error in %s: %v", filename, err)
		}
		astFiles = append(astFiles, file)
		astFilesByPath[normalizeFilePath(filename)] = file
	}

	// Check if we have external packages
	hasExternal := false
	for _, file := range astFiles {
		for _, imp := range file.Imports {
			path := strings.Trim(imp.Path.Value, `"`)
			if !isStdlibPackage(path) {
				hasExternal = true
				break
			}
		}
		if hasExternal {
			break
		}
	}

	// Use PackageLoader for projects with external dependencies
	if hasExternal && pg.externalMode == ModeTranspile && !skipExternalHandling {
		fmt.Fprintf(os.Stderr, "Loading packages with dependencies...\n")

		// Use PackageLoader to get full type information
		loader := NewPackageLoader(pg.projectPath)

		// Load with the current directory pattern
		if err := loader.LoadWithDependencies([]string{"."}); err != nil {
			return fmt.Errorf("failed to load packages: %v", err)
		}

		// Transpile external packages
		if err := loader.TranspileAll(); err != nil {
			return fmt.Errorf("failed to transpile dependencies: %v", err)
		}
		packageLoader = loader
		workspaceConcurrencyDetector = loader.GetConcurrencyDetector()

		// Get type info and package mapping
		pg.typeInfo = loader.GetTypeInfo()
		pg.packageMapping = loader.GetPackageMapping()
		pg.useSharedStdlibStubCrate = len(pg.packageMapping) > 0

		// CRITICAL: Replace our AST files with the ones from PackageLoader
		// which have the proper type information
		astFiles = loader.GetMainAST()
		astFilesByPath = loader.GetMainASTByPath()
		if len(astFilesByPath) == 0 {
			return fmt.Errorf("no AST files from package loader")
		}

		// Set up imports for the main package once the package-scoped context exists.
		packageImports = loader.GetMainImports()

		// The main package will use this type info
		SetTypeInfo(pg.typeInfo)

		// Skip duplicate handling
		skipExternalHandling = true
	} else if hasExternal && pg.externalMode == ModeStub && !skipExternalHandling {
		fmt.Fprintf(os.Stderr, "Generating stubs for external packages...\n")

		// Generate stub implementations
		stubGen := NewStubGenerator(pg.projectPath)
		if err := stubGen.GenerateStubsFromImports(astFiles); err != nil {
			return fmt.Errorf("failed to generate stubs: %v", err)
		}

		// Use the package mapping from stub generator
		pg.packageMapping = stubGen.GetPackageMapping()

		// Set up package imports for proper transpilation
		imports := make(map[string]string)
		for _, file := range astFiles {
			for _, imp := range file.Imports {
				importPath := strings.Trim(imp.Path.Value, `"`)
				var pkgName string
				if imp.Name != nil {
					pkgName = imp.Name.Name
				} else {
					segments := strings.Split(importPath, "/")
					pkgName = segments[len(segments)-1]
				}
				imports[pkgName] = importPath
			}
		}
		packageImports = imports

		// Regular type checking (will have missing types for external packages)
		typeInfo, err := NewTypeInfo(astFiles, fileSet)
		if err != nil {
			// This is expected with stubs - external types won't be available
			fmt.Fprintf(os.Stderr, "Note: Type checking incomplete (external packages are stubs): %v\n", err)
			fmt.Fprintf(os.Stderr, "You will need to implement the stub packages in external_stubs/\n")
		}
		pg.typeInfo = typeInfo
		SetTypeInfo(typeInfo)

		// Skip duplicate handling
		skipExternalHandling = true
	} else {
		// Regular type checking for projects without external dependencies
		typeInfo, err := NewTypeInfo(astFiles, fileSet)
		if err != nil {
			// Log warning but continue - we'll handle missing type info in individual functions
			fmt.Fprintf(os.Stderr, "Warning: Type checking incomplete: %v\n", err)
			fmt.Fprintf(os.Stderr, "Generated code may contain errors where type information is required\n")
		}
		pg.typeInfo = typeInfo

		// Set the global type info once for the entire project
		SetTypeInfo(typeInfo)
	}

	// Detect concurrency in the project
	concurrencyDetector := workspaceConcurrencyDetector
	if concurrencyDetector == nil {
		concurrencyDetector = NewConcurrencyDetector()
		concurrencyDetector.AnalyzeProject(astFiles)
	}
	SetConcurrencyDetector(concurrencyDetector)
	defer SetConcurrencyDetector(nil) // Clear when done

	packageState := NewPackageState()
	pg.usePackageHelpers = len(astFiles) > 1
	runCtx := &TranspileContext{
		Session:                 NewTranspileSession(pg.typeInfo, pg.packageMapping),
		Package:                 packageState,
		PackageMapping:          pg.packageMapping,
		UsePackageExternalStubs: pg.useSharedStdlibStubCrate,
		UsePackageHelpers:       pg.usePackageHelpers,
	}
	runCtx.Package.FunctionNameOverrides = assignPackageFunctionNames(astFiles)
	SetTranspileContext(runCtx)
	defer SetTranspileContext(nil)
	if packageImports != nil {
		SetPackageImports(packageImports)
	}
	packageState.ImportedInterfaceImpls = collectImportedInterfaceImplsFromFiles(astFiles)
	registerFunctionSignaturesFromFiles(astFiles)

	nonMainModuleNames := pg.nonMainModuleNames(astFilesByPath)

	// Ensure we clean up TypeInfo when done
	defer SetTypeInfo(nil)

	var generatedModules []generatedRustModule

	// First pass: transpile all files and detect structure
	for i, filename := range pg.goFiles {
		// Use the AST matched to this filename to avoid relying on go/packages order.
		normalizedFilename := normalizeFilePath(filename)
		file := astFilesByPath[normalizedFilename]
		if file == nil {
			return fmt.Errorf("no AST found for %s (normalized %s; available %s)", filename, normalizedFilename, sampleASTPathKeys(astFilesByPath, 5))
		}

		// Detect package name from first file
		if i == 0 {
			pg.packageName = file.Name.Name
			pg.isLibrary = pg.packageName != "main"
		}

		var rustCode string
		var fileImports *ImportTracker
		var fileExternalPkgs map[string]bool

		if pg.packageMapping != nil {
			rustCode, fileImports, fileExternalPkgs = TranspileWithMapping(file, fileSet, pg.typeInfo, pg.packageMapping)
		} else {
			rustCode, fileImports, fileExternalPkgs = Transpile(file, fileSet, pg.typeInfo)
		}

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

		if strings.Contains(rustCode, "__go_init_all") {
			pg.initModuleNames = append(pg.initModuleNames, outputName)
		}

		generatedModules = append(generatedModules, generatedRustModule{
			name:     outputName,
			path:     rustFilename,
			rustCode: rustCode,
		})
		pg.moduleNames = append(pg.moduleNames, outputName)
	}

	// Handle external packages based on mode (skip for vendor packages)
	// Note: ModeTranspile is already handled above with unified transpilation
	if !skipExternalHandling && len(pg.goImports) > 0 && pg.hasExternalPackages() {
		switch pg.externalMode {
		case ModeTranspile:
			// Already handled above with unified transpilation
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
		err := pg.generateMainRs(fileSet, astFilesByPath, packageState)
		if err != nil {
			return err
		}
	} else if pg.isLibrary {
		err := pg.generateLibRs(packageState)
		if err != nil {
			return err
		}
	}

	helpersNeeded := pg.packageHelpersNeeded(packageState)
	for _, module := range generatedModules {
		var helpers *HelperTracker
		if helpersNeeded {
			helpers = packageState.Helpers
		}
		rustCode := pg.prefixModuleImports(module.rustCode, module.name, nonMainModuleNames, helpers)
		if err := os.WriteFile(module.path, []byte(rustCode), 0644); err != nil {
			return fmt.Errorf("error writing %s: %v", module.path, err)
		}
	}

	if pg.useSharedStdlibStubCrate {
		var states []*PackageState
		if packageLoader != nil {
			states = append(states, packageLoader.GetPackageStates()...)
		}
		states = append(states, packageState)
		if err := WriteSharedStdlibStubCrate(pg.projectPath, states); err != nil {
			return err
		}
	}

	if helpersNeeded {
		if err := pg.writePackageHelperFile(packageState); err != nil {
			return err
		}
	}

	return pg.generateCargoToml()
}

func sampleASTPathKeys(astFilesByPath map[string]*ast.File, limit int) string {
	if len(astFilesByPath) == 0 {
		return "[]"
	}
	keys := make([]string, 0, len(astFilesByPath))
	for key := range astFilesByPath {
		keys = append(keys, key)
	}
	sort.Strings(keys)
	if limit > 0 && len(keys) > limit {
		keys = keys[:limit]
	}
	return "[" + strings.Join(keys, ", ") + "]"
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

func (pg *ProjectGenerator) nonMainModuleNames(astFilesByPath map[string]*ast.File) []string {
	var moduleNames []string
	for _, filename := range pg.goFiles {
		file := astFilesByPath[normalizeFilePath(filename)]
		if file == nil {
			continue
		}
		baseName := strings.TrimSuffix(filepath.Base(filename), ".go")
		if baseName == "main" && file.Name.Name == "main" {
			continue
		}
		outputName := baseName
		if pg.hasMainFile() && strings.HasPrefix(baseName, "lib") && strings.TrimLeft(baseName[3:], "_") == "" {
			outputName = baseName + "_"
		}
		moduleNames = append(moduleNames, outputName)
	}
	sort.Strings(moduleNames)
	return moduleNames
}

func prefixSiblingModuleImports(rustCode, selfModule string, moduleNames []string) string {
	var imports strings.Builder
	for _, modName := range moduleNames {
		if modName == selfModule {
			continue
		}
		imports.WriteString("use crate::")
		imports.WriteString(modName)
		imports.WriteString("::*;\n")
	}
	if imports.Len() == 0 {
		return rustCode
	}
	imports.WriteString("\n")
	imports.WriteString(rustCode)
	return imports.String()
}

func prefixPackageHelperImports(rustCode string, helpers *HelperTracker) string {
	names := helpers.ImportNames()
	if len(names) == 0 {
		return rustCode
	}
	return "use crate::{" + strings.Join(names, ", ") + "};\n\n" + rustCode
}

func (pg *ProjectGenerator) prefixModuleImports(rustCode, selfModule string, moduleNames []string, helpers *HelperTracker) string {
	rustCode = prefixSiblingModuleImports(rustCode, selfModule, moduleNames)
	if helpers != nil && helpers.HasAny() {
		rustCode = prefixPackageHelperImports(rustCode, helpers)
	}
	if pg.useSharedStdlibStubCrate {
		rustCode = prefixSharedStdlibStubImport(rustCode)
	}
	return rustCode
}

func (pg *ProjectGenerator) packageHelpersNeeded(packageState *PackageState) bool {
	return pg.usePackageHelpers && packageState != nil && packageState.Helpers.HasAny()
}

func (pg *ProjectGenerator) hasMainFile() bool {
	for _, file := range pg.goFiles {
		if filepath.Base(file) == "main.go" {
			return true
		}
	}
	return false
}

func (pg *ProjectGenerator) generateMainRs(fileSet *token.FileSet, astFilesByPath map[string]*ast.File, packageState *PackageState) error {
	var (
		mainGoFile *ast.File
		mainPath   string
	)
	for _, filename := range pg.goFiles {
		if filepath.Base(filename) == "main.go" {
			mainPath = filename
			mainGoFile = astFilesByPath[normalizeFilePath(filename)]
			break
		}
	}

	if mainGoFile == nil {
		return fmt.Errorf("main.go not found")
	}

	file := mainGoFile

	var mainContent string
	var mainImports *ImportTracker
	var mainExternalPkgs map[string]bool

	if pg.packageMapping != nil {
		mainContent, mainImports, mainExternalPkgs = TranspileWithMapping(file, fileSet, pg.typeInfo, pg.packageMapping)
	} else {
		mainContent, mainImports, mainExternalPkgs = Transpile(file, fileSet, pg.typeInfo)
	}

	// Track external packages from main
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

	mainContent = injectModuleInitCalls(mainContent, pg.initModuleNames)

	var mainRust strings.Builder
	if pg.packageHelpersNeeded(packageState) {
		mainRust.WriteString(fmt.Sprintf("include!(\"%s\");\n", packageHelperIncludeFile))
	}
	if pg.useSharedStdlibStubCrate {
		mainRust.WriteString(fmt.Sprintf("use %s::*;\n", sharedStdlibStubCrateName))
	}
	for _, modName := range pg.moduleNames {
		mainRust.WriteString(fmt.Sprintf("mod %s;\n", modName))
	}
	if len(pg.moduleNames) > 0 {
		for _, modName := range pg.moduleNames {
			mainRust.WriteString(fmt.Sprintf("use %s::*;\n", modName))
		}
		mainRust.WriteString("\n")
	}
	mainRust.WriteString(mainContent)

	mainRsPath := filepath.Join(pg.projectPath, "main.rs")
	return os.WriteFile(mainRsPath, []byte(mainRust.String()), 0644)
}

func injectModuleInitCalls(rustCode string, moduleNames []string) string {
	if len(moduleNames) == 0 {
		return rustCode
	}
	const marker = "fn main() {"
	insertAt := strings.Index(rustCode, marker)
	if insertAt < 0 {
		return rustCode
	}
	insertAt += len(marker)

	var initCalls strings.Builder
	initCalls.WriteString("\n")
	for _, modName := range moduleNames {
		initCalls.WriteString("    ")
		initCalls.WriteString(modName)
		initCalls.WriteString("::__go_init_all();\n")
	}

	return rustCode[:insertAt] + initCalls.String() + rustCode[insertAt:]
}

func (pg *ProjectGenerator) generateCargoToml() error {
	cargoPath := filepath.Join(pg.projectPath, "Cargo.toml")
	crateNames := pg.sortedCrateNames()
	dependencyCrateNames := crateNames
	if pg.useSharedStdlibStubCrate {
		dependencyCrateNames = addSharedStdlibStubCrateDependency(dependencyCrateNames)
	}

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

	// Add workspace configuration if we have external packages
	if len(dependencyCrateNames) > 0 {
		depDir := "external_stubs"
		if pg.externalMode == ModeTranspile {
			depDir = "vendor"
		}
		workspaceSection := "\n[workspace]\nmembers = [\n    \".\",\n"
		for _, crateName := range dependencyCrateNames {
			workspaceSection += fmt.Sprintf("    \"%s/%s\",\n", depDir, crateName)
		}
		workspaceSection += "]\n"
		cargoContent = workspaceSection + "\n" + cargoContent
	}

	// Add dependencies section
	if needsNum || len(dependencyCrateNames) > 0 {
		cargoContent += "\n[dependencies]\n"
		if needsNum {
			cargoContent += "num = \"0.4\"\n"
		}
		// Add external package dependencies
		for _, crateName := range dependencyCrateNames {
			depDir := "external_stubs"
			if pg.externalMode == ModeTranspile {
				depDir = "vendor"
			}
			cargoContent += fmt.Sprintf("%s = { path = \"%s/%s\" }\n", crateName, depDir, crateName)
		}
	}

	return os.WriteFile(cargoPath, []byte(cargoContent), 0644)
}

func (pg *ProjectGenerator) sortedCrateNames() []string {
	crateNames := make([]string, 0, len(pg.packageMapping))
	for _, crateName := range pg.packageMapping {
		crateNames = append(crateNames, crateName)
	}
	sort.Strings(crateNames)
	return crateNames
}

func normalizeFilePath(path string) string {
	absPath, err := filepath.Abs(path)
	if err != nil {
		return filepath.Clean(path)
	}
	resolvedPath, err := filepath.EvalSymlinks(absPath)
	if err == nil {
		return resolvedPath
	}
	return filepath.Clean(absPath)
}

func (pg *ProjectGenerator) generateLibRs(packageState *PackageState) error {
	var libRust strings.Builder

	if pg.packageHelpersNeeded(packageState) {
		libRust.WriteString(fmt.Sprintf("include!(\"%s\");\n", packageHelperIncludeFile))
	}
	if pg.useSharedStdlibStubCrate {
		libRust.WriteString(fmt.Sprintf("pub use %s::*;\n", sharedStdlibStubCrateName))
	}
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

func (pg *ProjectGenerator) writePackageHelperFile(packageState *PackageState) error {
	helpers := ""
	if packageState != nil && packageState.Helpers != nil {
		helpers = packageState.Helpers.GenerateHelperModule()
	}
	helperPath := filepath.Join(pg.projectPath, packageHelperIncludeFile)
	return os.WriteFile(helperPath, []byte(helpers), 0644)
}
