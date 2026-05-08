package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"golang.org/x/tools/go/packages"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// PackageLoader loads Go packages with full type information
type PackageLoader struct {
	workDir             string
	mainPkg             *packages.Package
	allPackages         map[string]*packages.Package // import path -> package
	packageMapping      map[string]string            // Go import -> Rust crate name
	packageStates       map[string]*PackageState
	concurrencyDetector *ConcurrencyDetector
	fileSet             *token.FileSet
}

const sharedStdlibStubCrateName = "go2rust_stdlib_stubs"

// NewPackageLoader creates a new package loader
func NewPackageLoader(workDir string) *PackageLoader {
	return &PackageLoader{
		workDir:        workDir,
		allPackages:    make(map[string]*packages.Package),
		packageMapping: make(map[string]string),
		packageStates:  make(map[string]*PackageState),
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

	isMain := pkg == pl.mainPkg || pkg.PkgPath == "main"

	if !isMain && isStdlibPackage(pkg.PkgPath) {
		return
	}

	// Store the package
	pl.allPackages[pkg.PkgPath] = pkg

	// Generate Rust crate name for external packages
	if !isMain {
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
	pl.concurrencyDetector = pl.buildWorkspaceConcurrencyDetector()

	// Transpile external packages first
	for _, pkgPath := range pl.orderedPackagePaths() {
		pkg := pl.allPackages[pkgPath]
		fmt.Fprintf(os.Stderr, "Transpiling package %s...\n", pkgPath)
		if err := pl.transpilePackage(pkg); err != nil {
			fmt.Fprintf(os.Stderr, "Warning: Failed to transpile %s: %v\n", pkgPath, err)
		}
	}

	// Main package is transpiled by the regular ProjectGenerator flow
	// but now with full type information available

	return nil
}

func (pl *PackageLoader) buildWorkspaceConcurrencyDetector() *ConcurrencyDetector {
	workspaceDetector := NewConcurrencyDetector()
	for _, pkgPath := range pl.orderedAllPackagePaths() {
		pkg := pl.allPackages[pkgPath]
		if pkg == nil {
			continue
		}
		pkgDetector := NewConcurrencyDetector()
		pkgDetector.AnalyzeProject(pkg.Syntax)
		workspaceDetector.Merge(pkgDetector)
	}
	return workspaceDetector
}

func (pl *PackageLoader) orderedAllPackagePaths() []string {
	var paths []string
	for pkgPath := range pl.allPackages {
		if isStdlibPackage(pkgPath) {
			continue
		}
		paths = append(paths, pkgPath)
	}
	sort.Strings(paths)
	return paths
}

func (pl *PackageLoader) orderedPackagePaths() []string {
	mainPkgPath := ""
	if pl.mainPkg != nil {
		mainPkgPath = pl.mainPkg.PkgPath
	}
	var paths []string
	for pkgPath := range pl.allPackages {
		if pkgPath == "main" || pkgPath == mainPkgPath {
			continue
		}
		if isStdlibPackage(pkgPath) {
			continue
		}
		paths = append(paths, pkgPath)
	}
	sort.Strings(paths)
	return paths
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

	parentDetector := GetConcurrencyDetector()
	concurrencyDetector := pl.concurrencyDetector
	if concurrencyDetector == nil {
		concurrencyDetector = NewConcurrencyDetector()
		concurrencyDetector.AnalyzeProject(pkg.Syntax)
	}
	SetConcurrencyDetector(concurrencyDetector)
	defer SetConcurrencyDetector(parentDetector)

	parentCtx := GetTranspileContext()
	pkgState := NewPackageState()
	pkgState.FunctionNameOverrides = assignPackageFunctionNames(pkg.Syntax)
	SetTranspileContext(&TranspileContext{
		Session:                 NewTranspileSession(pkgTypeInfo, pl.packageMapping),
		Package:                 pkgState,
		PackageMapping:          pl.packageMapping,
		UsePackageExternalStubs: true,
	})
	pkgCtx := GetTranspileContext()
	defer SetTranspileContext(parentCtx)
	parentTypeInfo := GetTypeInfo()
	SetTypeInfo(pkgTypeInfo)
	defer SetTypeInfo(parentTypeInfo)

	// Generate lib.rs with all modules
	var libRs strings.Builder
	var modules []string
	moduleNamesByIndex := make([]string, len(pkg.Syntax))
	for i, astFile := range pkg.Syntax {
		if len(astFile.Decls) == 0 {
			continue
		}
		fileName := packageFileName(pkg, i)
		baseName := strings.TrimSuffix(fileName, ".go")
		if baseName == pkg.Name {
			baseName = "mod" // Avoid name collision
		}
		moduleName := SanitizeRustModuleName(baseName)
		moduleNamesByIndex[i] = moduleName
		modules = append(modules, moduleName)
	}
	usePackageHelpers := len(modules) > 1
	if pkgCtx != nil {
		pkgCtx.UsePackageHelpers = usePackageHelpers
	}
	pkgState.ImportedInterfaceImpls = collectImportedInterfaceImplsFromFiles(pkg.Syntax)

	var generatedModules []generatedRustModule

	// Process each file in the package
	for i, astFile := range pkg.Syntax {
		if len(astFile.Decls) == 0 {
			continue // Skip empty files
		}

		moduleName := moduleNamesByIndex[i]

		// Transpile with the package's type info and global package mapping
		rustCode, _, _ := TranspileWithMapping(astFile, pkg.Fset, pkgTypeInfo, pl.packageMapping)

		moduleFile := filepath.Join(outputDir, SanitizeRustModuleFileName(moduleName)+".rs")
		generatedModules = append(generatedModules, generatedRustModule{
			name:     moduleName,
			path:     moduleFile,
			rustCode: rustCode,
		})
	}

	helpersNeeded := usePackageHelpers && pkgState.Helpers.HasAny()

	// Generate lib.rs
	if helpersNeeded {
		libRs.WriteString(fmt.Sprintf("include!(\"%s\");\n", packageHelperIncludeFile))
	}
	libRs.WriteString(fmt.Sprintf("pub use %s::*;\n", sharedStdlibStubCrateName))
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

	for _, module := range generatedModules {
		var helpers *HelperTracker
		if helpersNeeded {
			helpers = pkgState.Helpers
		}
		rustCode := prefixExternalPackageModuleImports(module.rustCode, module.name, modules, helpers)
		if err := os.WriteFile(module.path, []byte(rustCode), 0644); err != nil {
			return fmt.Errorf("failed to write module %s: %v", module.name, err)
		}
	}

	if helpersNeeded {
		helperPath := filepath.Join(outputDir, packageHelperIncludeFile)
		helperCode := pkgState.Helpers.GenerateHelperModule()
		if err := os.WriteFile(helperPath, []byte(helperCode), 0644); err != nil {
			return fmt.Errorf("failed to write package helper module: %v", err)
		}
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
	dependencyCrates := packageDependencyCrates(pkg.Imports, crateName, pl.packageMapping)
	dependencyCrates = addSharedStdlibStubCrateDependency(dependencyCrates)
	if len(dependencyCrates) > 0 {
		cargoToml += "\n[dependencies]\n"
		for _, depCrate := range dependencyCrates {
			cargoToml += fmt.Sprintf("%s = { path = \"../%s\" }\n", depCrate, depCrate)
		}
	}

	cargoPath := filepath.Join(outputDir, "Cargo.toml")
	if err := os.WriteFile(cargoPath, []byte(cargoToml), 0644); err != nil {
		return fmt.Errorf("failed to write Cargo.toml: %v", err)
	}

	pl.packageStates[pkg.PkgPath] = pkgState
	return nil
}

func prefixExternalPackageModuleImports(rustCode, selfModule string, moduleNames []string, helpers *HelperTracker) string {
	rustCode = prefixSiblingModuleImports(rustCode, selfModule, moduleNames)
	if helpers != nil && helpers.HasAny() {
		rustCode = prefixPackageHelperImports(rustCode, helpers)
	}
	return prefixSharedStdlibStubImport(rustCode)
}

func prefixSharedStdlibStubImport(rustCode string) string {
	return fmt.Sprintf("use %s::*;\n\n%s", sharedStdlibStubCrateName, rustCode)
}

func addSharedStdlibStubCrateDependency(crateNames []string) []string {
	seen := make(map[string]bool, len(crateNames)+1)
	for _, crateName := range crateNames {
		if crateName == "" {
			continue
		}
		seen[crateName] = true
	}
	seen[sharedStdlibStubCrateName] = true
	result := make([]string, 0, len(seen))
	for crateName := range seen {
		result = append(result, crateName)
	}
	sort.Strings(result)
	return result
}

func packageFileName(pkg *packages.Package, index int) string {
	if index < len(pkg.CompiledGoFiles) {
		return filepath.Base(pkg.CompiledGoFiles[index])
	}
	return fmt.Sprintf("file%d.go", index)
}

func packageDependencyCrates(imports map[string]*packages.Package, currentCrate string, packageMapping map[string]string) []string {
	seen := make(map[string]bool)
	for importPath := range imports {
		if isStdlibPackage(importPath) {
			continue
		}
		crateName, ok := packageMapping[importPath]
		if !ok || crateName == "" || crateName == currentCrate {
			continue
		}
		seen[crateName] = true
	}
	crateNames := make([]string, 0, len(seen))
	for crateName := range seen {
		crateNames = append(crateNames, crateName)
	}
	sort.Strings(crateNames)
	return crateNames
}

// goPathToRustCrate converts a Go import path to a Rust-compatible crate name
func (pl *PackageLoader) goPathToRustCrate(goPath string) string {
	return RustCrateNameForGoImportPath(goPath)
}

// GetPackageMapping returns the package mapping
func (pl *PackageLoader) GetPackageMapping() map[string]string {
	return pl.packageMapping
}

func (pl *PackageLoader) GetPackageStates() []*PackageState {
	states := make([]*PackageState, 0, len(pl.packageStates))
	for _, pkgPath := range pl.orderedPackagePaths() {
		if state := pl.packageStates[pkgPath]; state != nil {
			states = append(states, state)
		}
	}
	return states
}

func (pl *PackageLoader) GetConcurrencyDetector() *ConcurrencyDetector {
	return pl.concurrencyDetector
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

// GetMainASTByPath returns the main package AST files keyed by normalized file path.
func (pl *PackageLoader) GetMainASTByPath() map[string]*ast.File {
	astByPath := make(map[string]*ast.File)
	if pl.mainPkg == nil {
		return astByPath
	}
	for i, astFile := range pl.mainPkg.Syntax {
		if i >= len(pl.mainPkg.CompiledGoFiles) {
			continue
		}
		astByPath[pl.normalizePackageFilePath(pl.mainPkg.CompiledGoFiles[i])] = astFile
	}
	return astByPath
}

func (pl *PackageLoader) normalizePackageFilePath(path string) string {
	if filepath.IsAbs(path) {
		return normalizeFilePath(path)
	}
	cleanPath := filepath.Clean(path)
	cleanWorkDir := filepath.Clean(pl.workDir)
	if cleanWorkDir == "." || cleanWorkDir == "" {
		return normalizeFilePath(cleanPath)
	}
	if cleanPath == cleanWorkDir || strings.HasPrefix(cleanPath, cleanWorkDir+string(filepath.Separator)) {
		return normalizeFilePath(cleanPath)
	}
	return normalizeFilePath(filepath.Join(cleanWorkDir, cleanPath))
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
