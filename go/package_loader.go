package main

import (
	"fmt"
	"go/ast"
	"go/importer"
	"go/parser"
	"go/token"
	"go/types"
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
const sourceStdlibPackagesEnv = "GO2RUST_SOURCE_STDLIB_PACKAGES"

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
	if err := pl.loadLocalModuleFallbacks(); err != nil {
		fmt.Fprintf(os.Stderr, "Package loading warning: %v\n", err)
	}

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

	if !isMain && isStdlibPackage(pkg.PkgPath) && !shouldTranspileStdlibPackage(pkg.PkgPath) {
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
	for importPath, imp := range pkg.Imports {
		if imp != nil && imp.PkgPath == "" && importPath != "" {
			imp.PkgPath = importPath
		}
		pl.collectAllPackages(imp)
	}
}

func (pl *PackageLoader) loadLocalModuleFallbacks() error {
	if pl.mainPkg == nil {
		return nil
	}
	modulePath, err := pl.readModulePath()
	if err == nil && modulePath != "" {
		if err := pl.loadLocalImportsForPackage(pl.mainPkg, modulePath, make(map[string]bool)); err != nil {
			return err
		}
	}
	inferredModulePath := pl.inferModulePathFromLocalImports()
	if inferredModulePath != "" && inferredModulePath != modulePath {
		if err := pl.loadLocalImportsForPackage(pl.mainPkg, inferredModulePath, make(map[string]bool)); err != nil {
			return err
		}
	}
	// Type-check every parsed-but-not-yet-checked local-module fallback using
	// the project importer so siblings can resolve each other. Without this
	// pass, any local module that no other local module imports would keep
	// pkg.TypesInfo == nil, and the transpiler would later hit nil-typeInfo
	// branches — which AGENTS.md classifies as a loader bug.
	imp := pl.projectImporter()
	keys := make([]string, 0, len(pl.allPackages))
	for path := range pl.allPackages {
		keys = append(keys, path)
	}
	sort.Strings(keys)
	for _, path := range keys {
		pkg := pl.allPackages[path]
		if pkg == nil || pkg.Types != nil || len(pkg.Syntax) == 0 {
			continue
		}
		if err := pl.typeCheckLocalPackage(pkg, imp); err != nil {
			fmt.Fprintf(os.Stderr, "Package loading warning: type-check for %s failed: %v\n", path, err)
		}
	}
	return nil
}

func (pl *PackageLoader) readModulePath() (string, error) {
	data, err := os.ReadFile(filepath.Join(pl.workDir, "go.mod"))
	if err != nil {
		return "", err
	}
	for _, line := range strings.Split(string(data), "\n") {
		fields := strings.Fields(line)
		if len(fields) >= 2 && fields[0] == "module" {
			return fields[1], nil
		}
	}
	return "", nil
}

func (pl *PackageLoader) loadLocalImportsForPackage(pkg *packages.Package, modulePath string, visited map[string]bool) error {
	if pkg == nil {
		return nil
	}
	importPaths := make(map[string]bool)
	for _, file := range pkg.Syntax {
		for _, imp := range file.Imports {
			if imp == nil || imp.Path == nil {
				continue
			}
			importPath := strings.Trim(imp.Path.Value, `"`)
			importPaths[importPath] = true
		}
	}
	for importPath := range pkg.Imports {
		importPaths[importPath] = true
	}
	paths := make([]string, 0, len(importPaths))
	for importPath := range importPaths {
		paths = append(paths, importPath)
	}
	sort.Strings(paths)
	for _, importPath := range paths {
		if !localModuleImportPath(importPath, modulePath) {
			continue
		}
		if pkg.Imports == nil {
			pkg.Imports = make(map[string]*packages.Package)
		}
		if existing := pkg.Imports[importPath]; existing != nil && existing.PkgPath != "" && len(existing.Syntax) > 0 {
			continue
		}
		localPkg, err := pl.loadLocalModulePackage(importPath, modulePath)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Package loading warning: local fallback for %s failed: %v\n", importPath, err)
			continue
		}
		pkg.Imports[importPath] = localPkg
		pl.allPackages[importPath] = localPkg
		if _, exists := pl.packageMapping[importPath]; !exists {
			crateName := pl.goPathToRustCrate(importPath)
			pl.packageMapping[importPath] = crateName
			fmt.Fprintf(os.Stderr, "Found package: %s -> %s\n", importPath, crateName)
		}
		if !visited[importPath] {
			visited[importPath] = true
			if err := pl.loadLocalImportsForPackage(localPkg, modulePath, visited); err != nil {
				return err
			}
		}
	}
	return nil
}

func localModuleImportPath(importPath, modulePath string) bool {
	return importPath == modulePath || strings.HasPrefix(importPath, modulePath+"/")
}

func (pl *PackageLoader) inferModulePathFromLocalImports() string {
	if pl.mainPkg == nil {
		return ""
	}
	importPaths := make(map[string]bool)
	for _, file := range pl.mainPkg.Syntax {
		for _, imp := range file.Imports {
			if imp == nil || imp.Path == nil {
				continue
			}
			importPath := strings.Trim(imp.Path.Value, `"`)
			importPaths[importPath] = true
		}
	}
	for importPath := range pl.mainPkg.Imports {
		importPaths[importPath] = true
	}
	paths := make([]string, 0, len(importPaths))
	for importPath := range importPaths {
		paths = append(paths, importPath)
	}
	sort.Strings(paths)
	for _, importPath := range paths {
		if isStdlibPackage(importPath) {
			continue
		}
		parts := strings.Split(importPath, "/")
		for i := 1; i < len(parts); i++ {
			rel := strings.Join(parts[i:], "/")
			if directoryHasGoFiles(filepath.Join(pl.workDir, rel)) {
				return strings.Join(parts[:i], "/")
			}
		}
	}
	return ""
}

func directoryHasGoFiles(dir string) bool {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return false
	}
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".go") {
			return true
		}
	}
	return false
}

func shouldTranspileStdlibPackage(importPath string) bool {
	if !isStdlibPackage(importPath) {
		return false
	}
	return sourceStdlibPackagePatternMatches(importPath, os.Getenv(sourceStdlibPackagesEnv))
}

func sourceStdlibPackagePatternMatches(importPath, patterns string) bool {
	for _, pattern := range strings.FieldsFunc(patterns, func(r rune) bool {
		return r == ',' || r == ';' || r == ':' || r == ' ' || r == '\t' || r == '\n'
	}) {
		pattern = strings.TrimSpace(pattern)
		if pattern == "" {
			continue
		}
		if pattern == "all" || pattern == "std" {
			return true
		}
		if strings.HasSuffix(pattern, "/...") {
			prefix := strings.TrimSuffix(pattern, "/...")
			if importPath == prefix || strings.HasPrefix(importPath, prefix+"/") {
				return true
			}
			continue
		}
		if importPath == pattern {
			return true
		}
	}
	return false
}

// projectImporter resolves Go import paths first against the packages already
// loaded into pl.allPackages (including transpiler-managed local module
// fallbacks) and falls back to importer.Default() for stdlib paths. This is
// what makes local modules type-check fully: without it, types.Config.Check
// silently leaves info.Types / info.Uses with holes for every cross-package
// reference, and the transpiler then visits AST nodes whose go/types data is
// nil — which is the bug class AGENTS.md explicitly forbids working around
// with syntax heuristics.
type projectImporter struct {
	pl       *PackageLoader
	fallback types.Importer
}

func (pi *projectImporter) Import(path string) (*types.Package, error) {
	if pi.pl != nil {
		if pkg, ok := pi.pl.allPackages[path]; ok && pkg != nil && pkg.Types != nil {
			return pkg.Types, nil
		}
		// Trigger lazy type-checking of pending local modules. Parse-only
		// fallback packages have Syntax populated but Types nil; resolve them
		// on demand so siblings can see each other.
		if pkg, ok := pi.pl.allPackages[path]; ok && pkg != nil && pkg.Types == nil && len(pkg.Syntax) > 0 {
			if err := pi.pl.typeCheckLocalPackage(pkg, pi); err != nil {
				return nil, err
			}
			if pkg.Types != nil {
				return pkg.Types, nil
			}
		}
	}
	if pi.fallback == nil {
		return nil, fmt.Errorf("package %q not found", path)
	}
	return pi.fallback.Import(path)
}

func (pl *PackageLoader) projectImporter() *projectImporter {
	return &projectImporter{pl: pl, fallback: importer.Default()}
}

// loadLocalModulePackage parses the Go files of a local module without
// type-checking. Type-checking is deferred to typeCheckLocalPackage so the
// shared projectImporter can resolve sibling local modules. Caller must call
// typeCheckLocalPackage (directly or via lazy projectImporter.Import) before
// the resulting *packages.Package is used for transpilation.
func (pl *PackageLoader) loadLocalModulePackage(importPath, modulePath string) (*packages.Package, error) {
	rel := strings.TrimPrefix(importPath, modulePath)
	rel = strings.TrimPrefix(rel, "/")
	dir := pl.workDir
	if rel != "" {
		dir = filepath.Join(pl.workDir, rel)
	}
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}
	fileSet := pl.fileSet
	if fileSet == nil {
		fileSet = token.NewFileSet()
		pl.fileSet = fileSet
	}
	var files []*ast.File
	var filenames []string
	for _, entry := range entries {
		if entry.IsDir() {
			continue
		}
		name := entry.Name()
		if !strings.HasSuffix(name, ".go") || strings.HasSuffix(name, "_test.go") {
			continue
		}
		filename := filepath.Join(dir, name)
		file, err := parser.ParseFile(fileSet, filename, nil, parser.ParseComments)
		if err != nil {
			return nil, err
		}
		files = append(files, file)
		filenames = append(filenames, filename)
	}
	if len(files) == 0 {
		return nil, fmt.Errorf("no Go files found in %s", dir)
	}
	pkgName := files[0].Name.Name
	return &packages.Package{
		Name:            pkgName,
		PkgPath:         importPath,
		Fset:            fileSet,
		GoFiles:         filenames,
		CompiledGoFiles: filenames,
		Syntax:          files,
		Imports:         make(map[string]*packages.Package),
	}, nil
}

// typeCheckLocalPackage runs go/types over a previously parsed local-module
// package. The supplied importer must resolve any sibling local modules
// (typically the shared projectImporter). On success the package's Types and
// TypesInfo fields are populated and the result is registered as a TypeInfo
// for downstream lookups; on failure the caller will see incomplete type info
// and the transpiler will hit nil-typeInfo branches — which AGENTS.md says
// must be treated as a loader bug, not patched over.
func (pl *PackageLoader) typeCheckLocalPackage(pkg *packages.Package, imp types.Importer) error {
	if pkg == nil {
		return fmt.Errorf("typeCheckLocalPackage: nil package")
	}
	if pkg.Types != nil {
		return nil // already type-checked
	}
	if len(pkg.Syntax) == 0 {
		return fmt.Errorf("typeCheckLocalPackage: no syntax for %s", pkg.PkgPath)
	}
	fileSet := pkg.Fset
	if fileSet == nil {
		fileSet = pl.fileSet
	}
	if imp == nil {
		imp = pl.projectImporter()
	}
	ti, err := NewTypeInfoWithImporter(pkg.PkgPath, pkg.Syntax, fileSet, imp)
	if err != nil {
		return fmt.Errorf("type-check %s: %w", pkg.PkgPath, err)
	}
	pkg.Types = ti.pkg
	pkg.TypesInfo = ti.info
	return nil
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

	// When any stdlib package is transpiled from source, prune unreachable
	// funcs/methods/types in those packages so peripheral declarations pulling
	// in heavy deps (go/ast's reflect printer, filepath's os-based Glob) don't
	// block compilation of the subset the program actually uses.
	SetSourceStdlibReachable(pl.computeSourceStdlibReachable())

	resetPackageMethodReceiverMutability()
	var allPackageTypes []*types.Package
	for _, pkgPath := range pl.orderedAllPackagePaths() {
		pkg := pl.allPackages[pkgPath]
		if pkg != nil {
			registerPackageMethodReceiverMutability(pkg.PkgPath, pkg.Syntax)
			if pkg.Types != nil {
				allPackageTypes = append(allPackageTypes, pkg.Types)
			}
		}
	}
	// With every package's concrete-method mutability registered, decide which
	// interface methods lower to `&mut self` (any implementor mutates through
	// them). Trait defs, impls, and dispatch call sites all consult this.
	registerInterfaceMethodMutableReceivers(allPackageTypes)

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
		if isStdlibPackage(pkgPath) && pl.packageMapping[pkgPath] == "" {
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
		if isStdlibPackage(pkgPath) && pl.packageMapping[pkgPath] == "" {
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
	pkgState.MethodNameOverrides = assignPackageMethodNames(pkg.Syntax, pkgTypeInfo)
	pkgState.ConstantNameOverrides = assignPackageConstantNames(pkg.Syntax)
	pkgState.MethodsByType = collectPackageMethods(pkg.Syntax)
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
	packageAnalysis := analyzeTranspileFiles(pkg.Syntax, pkgTypeInfo)
	pkgState.MapKeyStructTypes = packageAnalysis.mapKeyStructTypes

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
	pkgState.ImportedInterfaceImpls = packageAnalysis.importedInterfaceImpls
	registerPackageTypeModuleNames(pkgState, pkg.Syntax, moduleNamesByIndex)
	registerPackageTypeFactsFromFiles(pkg.Syntax)
	registerFunctionSignaturesFromFiles(pkg.Syntax)

	var generatedModules []generatedRustModule
	var initModules []generatedInitModule
	packageImports := NewImportTracker()

	// Process each file in the package
	for i, astFile := range pkg.Syntax {
		if len(astFile.Decls) == 0 {
			continue // Skip empty files
		}

		moduleName := moduleNamesByIndex[i]

		// Transpile with the package's type info and global package mapping
		if pkgCtx != nil {
			pkgCtx.CurrentModuleName = moduleName
		}
		rustCode, fileImports, _ := TranspileWithMapping(astFile, pkg.Fset, pkgTypeInfo, pl.packageMapping)
		rustCode = prefixDotImportedCrateUses(rustCode, astFile, pl.packageMapping)
		if fileImports != nil {
			for imp := range fileImports.needs {
				packageImports.Add(imp)
			}
		}
		if moduleHasPackageInitAll(rustCode) {
			initModules = append(initModules, generatedInitModule{
				moduleName:       moduleName,
				initFunctionName: "__go_init_all",
			})
		}

		moduleFile := filepath.Join(outputDir, SanitizeRustModuleFileName(moduleName)+".rs")
		generatedModules = append(generatedModules, generatedRustModule{
			name:     moduleName,
			path:     moduleFile,
			rustCode: rustCode,
		})
	}

	helpersNeeded := usePackageHelpers && pkgState.Helpers.HasAnyOmittingSharedStdlibHelpers()
	dependencyCrates := packageDependencyCrates(pkg.Imports, crateName, pl.packageMapping)

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
	writeLibraryPackageInitAll(&libRs, dependencyCrates, initModules)

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
		helperCode := pkgState.Helpers.GenerateHelperModuleOmittingSharedStdlibHelpers()
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
	dependencyCrates = addSharedStdlibStubCrateDependency(dependencyCrates)
	needsNum := packageImports.needs["num::Complex"]
	needsSerdeJSON := generatedRustModulesContain(generatedModules, "serde_json::") || generatedRustModulesContain(generatedModules, "pub use serde_json")
	needsGosyn := generatedRustModulesContain(generatedModules, "gosyn::")
	if needsNum || needsSerdeJSON || needsGosyn || len(dependencyCrates) > 0 {
		cargoToml += "\n[dependencies]\n"
		if needsNum {
			cargoToml += "num = \"0.4\"\n"
		}
		if needsSerdeJSON {
			cargoToml += "serde_json = \"1\"\n"
		}
		if needsGosyn {
			cargoToml += "gosyn = \"0.2.9\"\n"
		}
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

func generatedRustModulesContain(modules []generatedRustModule, needle string) bool {
	for _, module := range modules {
		if strings.Contains(module.rustCode, needle) {
			return true
		}
	}
	return false
}

func prefixExternalPackageModuleImports(rustCode, selfModule string, moduleNames []string, helpers *HelperTracker) string {
	rustCode = prefixSiblingModuleImports(rustCode, selfModule, moduleNames)
	if helpers != nil && helpers.HasAny() {
		rustCode = prefixPackageHelperImports(rustCode, helpers, true)
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
