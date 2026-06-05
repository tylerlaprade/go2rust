package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
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
	initModules              []generatedInitModule
	typeInfo                 *TypeInfo
	projectImports           *ImportTracker // Collect imports across all files
	externalMode             ExternalPackageMode
	goImports                map[string][]string // package path -> list of imports
	packageMapping           map[string]string   // Go import path -> Rust crate name
	packageTypeModules       map[string]map[string]string
	goPtrParamFuncNames      map[string]map[int]string
	goPtrReturnFuncNames     map[string]map[int]goPtrResultInfo
	goPtrArrayFields         map[string]goPtrArrayFieldInfo
	generatedGoPtrFields     map[string]bool
	isVendorPackage          bool // True if this is a vendor package (no go.mod required)
	useSharedStdlibStubCrate bool // True when transpiled packages share one stdlib stub crate
	usePackageHelpers        bool // True when helper definitions must be shared across generated modules
}

const packageHelperIncludeFile = "__go2rust_helpers.rs"

type generatedRustModule struct {
	name     string
	path     string
	rustCode string
}

type generatedInitModule struct {
	moduleName       string
	initFunctionName string
	hasGlobals       bool
	hasInitFunctions bool
}

type generatedPackageInitOrderStep struct {
	moduleName   string
	functionName string
}

type generatedPackageInitPlan struct {
	zeroModules         []string
	orderSteps          []generatedPackageInitOrderStep
	initFunctionModules []string
}

type packageInitFeatures struct {
	hasGlobals       bool
	hasInitFunctions bool
}

func (plan generatedPackageInitPlan) usesPackageWideInit() bool {
	return len(plan.zeroModules) > 0 || len(plan.orderSteps) > 0 || len(plan.initFunctionModules) > 0
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
		file, err := parser.ParseFile(fileSet, filename, nil, parser.ImportsOnly|parser.SkipObjectResolution)
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
		file, err := parser.ParseFile(fileSet, filename, nil, parser.ParseComments|parser.SkipObjectResolution)
		if err != nil {
			return fmt.Errorf("parse error in %s: %v", filename, err)
		}
		astFiles = append(astFiles, file)
		astFilesByPath[normalizeFilePath(filename)] = file
	}

	// Check if dependency loading is needed. Non-stdlib imports always need the
	// package loader; explicitly source-transpiled stdlib packages use the same
	// path so compiler packages such as go/types are real crates, not semantic
	// stubs.
	needsPackageLoader := false
	for _, file := range astFiles {
		for _, imp := range file.Imports {
			path := strings.Trim(imp.Path.Value, `"`)
			if !isStdlibPackage(path) || shouldTranspileStdlibPackage(path) {
				needsPackageLoader = true
				break
			}
		}
		if needsPackageLoader {
			break
		}
	}

	// Use PackageLoader for projects with external dependencies
	if needsPackageLoader && pg.externalMode == ModeTranspile && !skipExternalHandling {
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
		pg.packageTypeModules = loader.GetPackageTypeModuleNames()
		pg.goPtrParamFuncNames = loader.GetGoPtrParamFuncNames()
		pg.goPtrReturnFuncNames = loader.GetGoPtrReturnFuncNames()
		pg.goPtrArrayFields = loader.GetGoPtrArrayFields()
		pg.generatedGoPtrFields = loader.GetGeneratedGoPtrFields()
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
	} else if needsPackageLoader && pg.externalMode == ModeStub && !skipExternalHandling {
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

	packageAnalysis := analyzeTranspileFiles(astFiles, pg.typeInfo)
	if packageLoader == nil {
		if len(pg.packageMapping) == 0 {
			resetPackageMethodReceiverMutability()
		}
		registerPackageMethodReceiverMutability("main", astFiles)
		if pg.typeInfo != nil && pg.typeInfo.pkg != nil {
			registerInterfaceMethodMutableReceivers([]*types.Package{pg.typeInfo.pkg})
		}
	}
	packageState := NewPackageState()
	packageState.MapKeyStructTypes = packageAnalysis.mapKeyStructTypes
	packageState.ComparableStructTypes = packageAnalysis.comparableStructTypes
	packageState.PointerComparablePointeeTypes = packageAnalysis.pointerComparablePointees
	pg.usePackageHelpers = len(astFiles) > 1
	session := NewTranspileSession(pg.typeInfo, pg.packageMapping)
	session.PackageTypeModuleNames = pg.packageTypeModules
	session.GoPtrParamFuncNames = pg.goPtrParamFuncNames
	session.GoPtrReturnFuncNames = pg.goPtrReturnFuncNames
	session.GoPtrArrayFields = pg.goPtrArrayFields
	session.GeneratedGoPtrFields = pg.generatedGoPtrFields
	runCtx := &TranspileContext{
		Session:                 session,
		Package:                 packageState,
		PackageMapping:          pg.packageMapping,
		UsePackageExternalStubs: pg.useSharedStdlibStubCrate,
		UsePackageHelpers:       pg.usePackageHelpers,
	}
	runCtx.Package.FunctionNameOverrides = assignPackageFunctionNames(astFiles)
	runCtx.Package.GlobalNameOverrides = assignPackageGlobalNameOverrides(astFiles, runCtx.Package.FunctionNameOverrides)
	runCtx.Package.MethodNameOverrides = assignPackageMethodNames(astFiles, pg.typeInfo)
	runCtx.Package.ConstantNameOverrides = assignPackageConstantNames(astFiles)
	runCtx.Package.MethodsByType = collectPackageMethods(astFiles)
	SetTranspileContext(runCtx)
	defer SetTranspileContext(nil)
	if packageLoader != nil {
		prevSourceStdlibReachable := sourceStdlibReachable
		SetSourceStdlibReachable(packageLoader.GetSourceStdlibReachable())
		defer SetSourceStdlibReachable(prevSourceStdlibReachable)
	}
	if packageImports != nil {
		SetPackageImports(packageImports)
	}
	packageState.ImportedInterfaceImpls = packageAnalysis.importedInterfaceImpls
	packageState.ImportedPointerInterfaceImpls = packageAnalysis.importedPointerInterfaceImpls
	packageState.ExternalLocalInterfaceImpls = packageAnalysis.externalLocalInterfaceImpls(collectPackageInterfaceDecls(astFiles))
	registerPackageTypeFactsFromFiles(astFiles)
	registerFunctionSignaturesFromFiles(astFiles)
	registerSliceElemPtrFactsFromFiles(astFiles)
	packageState.FunctionBoundKinds = genericFunctionBoundKinds(collectPackageFunctions(astFiles))
	packageState.LocalInterfaceGoValueClone = collectLocalInterfaceGoValueCloneTypes(astFiles, packageState.FunctionBoundKinds)
	packageState.LocalInterfaceGoComparable = collectLocalInterfaceGoComparableTypes(astFiles)

	nonMainModuleNames := pg.nonMainModuleNames(astFilesByPath)
	for _, filename := range pg.goFiles {
		file := astFilesByPath[normalizeFilePath(filename)]
		if file == nil {
			continue
		}
		baseName := strings.TrimSuffix(filepath.Base(filename), ".go")
		if baseName == "main" && file.Name.Name == "main" {
			continue
		}
		outputName := pg.moduleNameForBase(baseName)
		registerPackageTypeModuleNamesForFile(packageState, file, outputName)
	}

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

		baseName := strings.TrimSuffix(filepath.Base(filename), ".go")

		// Check if this is main.go
		if baseName == "main" && file.Name.Name == "main" {
			pg.hasMain = true
			// We'll handle main.go specially later
			continue
		}

		outputName := pg.moduleNameForBase(baseName)
		rustFilename := pg.moduleFilePath(filename, outputName)
		initFeatures := packageInitFeaturesForFile(file)

		var rustCode string
		var fileImports *ImportTracker
		var fileExternalPkgs map[string]bool

		runCtx.CurrentModuleName = outputName
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

		if moduleHasPackageInitAll(rustCode) {
			initFunctionName := moduleInitAllFunctionName(outputName)
			rustCode = renamePackageInitAllFunction(rustCode, initFunctionName)
			pg.initModules = append(pg.initModules, generatedInitModule{
				moduleName:       outputName,
				initFunctionName: initFunctionName,
				hasGlobals:       initFeatures.hasGlobals,
				hasInitFunctions: initFeatures.hasInitFunctions,
			})
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
		err := pg.generateLibRs(packageState, astFilesByPath)
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
		if err := WriteSharedStdlibStubCrate(pg.projectPath, states, pg.packageMapping); err != nil {
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
		outputName := pg.moduleNameForBase(baseName)
		moduleNames = append(moduleNames, outputName)
	}
	sort.Strings(moduleNames)
	return moduleNames
}

func (pg *ProjectGenerator) moduleNameForBase(baseName string) string {
	outputName := baseName
	if pg.hasMainFile() && strings.HasPrefix(baseName, "lib") && strings.TrimLeft(baseName[3:], "_") == "" {
		outputName = baseName + "_"
	}
	return SanitizeRustModuleName(outputName)
}

func (pg *ProjectGenerator) moduleFilePath(filename, moduleName string) string {
	return filepath.Join(filepath.Dir(filename), SanitizeRustModuleFileName(moduleName)+".rs")
}

// prefixDotImportedCrateUses emits a glob `use <crate>::*;` for every
// dot-imported (`import . "path"`) source-mapped package, so the bare names a
// dot import brings into Go scope resolve to the dependency crate's exports.
// go/types dot-imports internal/types/errors, referencing Code and the error
// codes (InvalidSyntaxTree, ...) bare; without the glob use those resolve to
// nothing.
func prefixDotImportedCrateUses(rustCode string, astFile *ast.File, packageMapping map[string]string) string {
	if astFile == nil || packageMapping == nil {
		return rustCode
	}
	var uses strings.Builder
	seen := map[string]bool{}
	for _, imp := range astFile.Imports {
		if imp.Name == nil || imp.Name.Name != "." || imp.Path == nil {
			continue
		}
		path := strings.Trim(imp.Path.Value, `"`)
		crate := packageMapping[path]
		if crate == "" || seen[crate] {
			continue
		}
		seen[crate] = true
		uses.WriteString("use ")
		uses.WriteString(crate)
		uses.WriteString("::*;\n")
	}
	if uses.Len() == 0 {
		return rustCode
	}
	uses.WriteString("\n")
	uses.WriteString(rustCode)
	return uses.String()
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
	if strings.TrimSpace(rustCode) == "" {
		return imports.String()
	}
	imports.WriteString("\n")
	imports.WriteString(rustCode)
	return imports.String()
}

func prefixPackageHelperImports(rustCode string, helpers *HelperTracker, omitSharedStdlibHelpers bool) string {
	var names []string
	if omitSharedStdlibHelpers {
		names = helpers.ImportNamesOmittingSharedStdlibHelpers()
	} else {
		names = helpers.ImportNames()
	}
	if len(names) == 0 {
		return rustCode
	}
	if strings.TrimSpace(rustCode) == "" {
		return "use crate::{" + strings.Join(names, ", ") + "};\n"
	}
	return "use crate::{" + strings.Join(names, ", ") + "};\n\n" + rustCode
}

func (pg *ProjectGenerator) prefixModuleImports(rustCode, selfModule string, moduleNames []string, helpers *HelperTracker) string {
	rustCode = prefixSiblingModuleImports(rustCode, selfModule, moduleNames)
	if helpers != nil && helpers.HasAny() {
		rustCode = prefixPackageHelperImports(rustCode, helpers, pg.useSharedStdlibStubCrate)
	}
	if pg.useSharedStdlibStubCrate {
		rustCode = prefixSharedStdlibStubImport(rustCode)
	}
	return rustCode
}

func (pg *ProjectGenerator) packageHelpersNeeded(packageState *PackageState) bool {
	if !pg.usePackageHelpers || packageState == nil || packageState.Helpers == nil {
		return false
	}
	if pg.useSharedStdlibStubCrate {
		return packageState.Helpers.HasAnyOmittingSharedStdlibHelpers()
	}
	return packageState.Helpers.HasAny()
}

func (pg *ProjectGenerator) packageHelperModuleCode(packageState *PackageState) string {
	if packageState == nil || packageState.Helpers == nil {
		return ""
	}
	if pg.useSharedStdlibStubCrate {
		return packageState.Helpers.GenerateHelperModuleOmittingSharedStdlibHelpers()
	}
	return packageState.Helpers.GenerateHelperModule()
}

func stripRustUseImportsProvidedByHelperModule(rustCode, helperCode string) string {
	provided := rustUseImportKeys(helperCode)
	if len(provided) == 0 {
		return rustCode
	}

	hasTrailingNewline := strings.HasSuffix(rustCode, "\n")
	lines := strings.Split(rustCode, "\n")
	out := make([]string, 0, len(lines))
	for _, line := range lines {
		rewritten, keep := stripRustUseLineProvidedBy(line, provided)
		if keep {
			out = append(out, rewritten)
		}
	}

	result := strings.Join(out, "\n")
	if hasTrailingNewline && !strings.HasSuffix(result, "\n") {
		result += "\n"
	}
	return result
}

func stripRustUseLineProvidedBy(line string, provided map[string]bool) (string, bool) {
	trimmed := strings.TrimSpace(line)
	if !strings.HasPrefix(trimmed, "use ") || !strings.HasSuffix(trimmed, ";") {
		return line, true
	}

	body := strings.TrimSuffix(strings.TrimPrefix(trimmed, "use "), ";")
	if prefix, names, ok := parseRustGroupedUse(body); ok {
		kept := make([]string, 0, len(names))
		for _, name := range names {
			if !provided[prefix+"::"+name] {
				kept = append(kept, name)
			}
		}
		if len(kept) == 0 {
			return "", false
		}
		return leadingWhitespace(line) + "use " + prefix + "::{" + strings.Join(kept, ", ") + "};", true
	}

	if provided[body] {
		return "", false
	}
	return line, true
}

func rustUseImportKeys(rustCode string) map[string]bool {
	keys := make(map[string]bool)
	for _, line := range strings.Split(rustCode, "\n") {
		trimmed := strings.TrimSpace(line)
		if !strings.HasPrefix(trimmed, "use ") || !strings.HasSuffix(trimmed, ";") {
			continue
		}

		body := strings.TrimSuffix(strings.TrimPrefix(trimmed, "use "), ";")
		if prefix, names, ok := parseRustGroupedUse(body); ok {
			for _, name := range names {
				keys[prefix+"::"+name] = true
			}
			continue
		}
		keys[body] = true
	}
	return keys
}

func parseRustGroupedUse(body string) (string, []string, bool) {
	open := strings.LastIndex(body, "::{")
	if open < 0 || !strings.HasSuffix(body, "}") {
		return "", nil, false
	}

	prefix := body[:open]
	rawNames := strings.Split(body[open+3:len(body)-1], ",")
	names := make([]string, 0, len(rawNames))
	for _, rawName := range rawNames {
		name := strings.TrimSpace(rawName)
		if name != "" {
			names = append(names, name)
		}
	}
	return prefix, names, true
}

func leadingWhitespace(line string) string {
	return line[:len(line)-len(strings.TrimLeft(line, " \t"))]
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

	initPlan := pg.packageInitPlan(astFilesByPath)
	if initPlan.usesPackageWideInit() {
		mainContent = replaceOrAppendMainPackageInitAll(mainContent, initPlan)
		mainContent = ensureMainCallsPackageInitAll(mainContent)
	} else {
		mainContent = injectModuleInitCalls(mainContent, pg.initModules)
	}
	if pg.externalMode == ModeTranspile && len(pg.packageMapping) > 0 {
		mainContent = injectExternalPackageInitCalls(mainContent, pg.sortedCrateNames())
	}

	var mainRust strings.Builder
	if pg.packageHelpersNeeded(packageState) {
		helperCode := pg.packageHelperModuleCode(packageState)
		mainContent = stripRustUseImportsProvidedByHelperModule(mainContent, helperCode)
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

func moduleHasPackageInitAll(rustCode string) bool {
	return packageInitAllFunctionLineIndex(rustCode) >= 0
}

func moduleInitAllFunctionName(moduleName string) string {
	return "__go_init_all_" + strings.TrimPrefix(SanitizeRustModuleName(moduleName), "r#")
}

func renamePackageInitAllFunction(rustCode, initFunctionName string) string {
	lines := strings.Split(rustCode, "\n")
	for i, line := range lines {
		if strings.TrimSpace(line) == "pub(crate) fn __go_init_all() {" {
			lines[i] = leadingWhitespace(line) + "pub(crate) fn " + initFunctionName + "() {"
			break
		}
	}
	return strings.Join(lines, "\n")
}

func packageInitAllFunctionLineIndex(rustCode string) int {
	for i, line := range strings.Split(rustCode, "\n") {
		if strings.TrimSpace(line) == "pub(crate) fn __go_init_all() {" {
			return i
		}
	}
	return -1
}

func injectModuleInitCalls(rustCode string, initModules []generatedInitModule) string {
	if len(initModules) == 0 {
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
	for _, initModule := range initModules {
		initCalls.WriteString("    ")
		initCalls.WriteString(initModule.moduleName)
		initCalls.WriteString("::")
		initCalls.WriteString(initModule.initFunctionName)
		initCalls.WriteString("();\n")
	}

	return rustCode[:insertAt] + initCalls.String() + rustCode[insertAt:]
}

func injectExternalPackageInitCalls(rustCode string, crateNames []string) string {
	if len(crateNames) == 0 {
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
	for _, crateName := range crateNames {
		if crateName == "" || crateName == sharedStdlibStubCrateName {
			continue
		}
		initCalls.WriteString("    ")
		initCalls.WriteString(crateName)
		initCalls.WriteString("::__go_init_all();\n")
	}

	return rustCode[:insertAt] + initCalls.String() + rustCode[insertAt:]
}

func packageInitFeaturesForFile(file *ast.File) packageInitFeatures {
	var features packageInitFeatures
	if file == nil {
		return features
	}
	for _, decl := range file.Decls {
		switch d := decl.(type) {
		case *ast.FuncDecl:
			if d.Recv == nil && d.Name != nil && d.Name.Name == "init" {
				features.hasInitFunctions = true
			}
		case *ast.GenDecl:
			if d.Tok != token.VAR {
				continue
			}
			for _, spec := range d.Specs {
				valueSpec, ok := spec.(*ast.ValueSpec)
				if !ok {
					continue
				}
				for _, name := range valueSpec.Names {
					if name.Name != "_" {
						features.hasGlobals = true
						break
					}
				}
				if features.hasGlobals {
					break
				}
			}
		}
	}
	return features
}

func (pg *ProjectGenerator) packageInitPlan(astFilesByPath map[string]*ast.File) generatedPackageInitPlan {
	astFiles := make([]*ast.File, 0, len(pg.goFiles))
	moduleNames := make([]string, 0, len(pg.goFiles))
	for _, filename := range pg.goFiles {
		file := astFilesByPath[normalizeFilePath(filename)]
		if file == nil {
			continue
		}
		moduleName := ""
		baseName := strings.TrimSuffix(filepath.Base(filename), ".go")
		if !(baseName == "main" && file.Name.Name == "main") {
			moduleName = pg.moduleNameForBase(baseName)
		}
		astFiles = append(astFiles, file)
		moduleNames = append(moduleNames, moduleName)
	}
	return packageInitPlanForModules(astFiles, moduleNames, pg.typeInfo)
}

func packageInitPlanForModules(astFiles []*ast.File, moduleNames []string, typeInfo *TypeInfo) generatedPackageInitPlan {
	if len(astFiles) <= 1 || typeInfo == nil || typeInfo.info == nil {
		return generatedPackageInitPlan{}
	}

	globalModuleByName := make(map[string]string)
	var zeroModules []string
	var initFunctionModules []string
	for i, file := range astFiles {
		if file == nil || i >= len(moduleNames) {
			continue
		}
		moduleName := moduleNames[i]
		features := packageInitFeaturesForFile(file)
		if features.hasGlobals {
			zeroModules = append(zeroModules, moduleName)
		}
		if features.hasInitFunctions {
			initFunctionModules = append(initFunctionModules, moduleName)
		}
		for _, decl := range file.Decls {
			genDecl, ok := decl.(*ast.GenDecl)
			if !ok || genDecl.Tok != token.VAR {
				continue
			}
			for _, spec := range genDecl.Specs {
				valueSpec, ok := spec.(*ast.ValueSpec)
				if !ok {
					continue
				}
				for _, name := range valueSpec.Names {
					if name.Name != "_" {
						globalModuleByName[name.Name] = moduleName
					}
				}
			}
		}
	}

	var orderSteps []generatedPackageInitOrderStep
	for i, init := range typeInfo.info.InitOrder {
		moduleName, ok := packageInitModuleForInitializer(init, globalModuleByName)
		if !ok {
			continue
		}
		orderSteps = append(orderSteps, generatedPackageInitOrderStep{
			moduleName:   moduleName,
			functionName: fmt.Sprintf("__go_init_order_%d", i),
		})
	}

	return generatedPackageInitPlan{
		zeroModules:         zeroModules,
		orderSteps:          orderSteps,
		initFunctionModules: initFunctionModules,
	}
}

func packageInitModuleForInitializer(init *types.Initializer, globalModuleByName map[string]string) (string, bool) {
	for _, lhs := range init.Lhs {
		if lhs == nil {
			continue
		}
		moduleName, ok := globalModuleByName[lhs.Name()]
		if ok {
			return moduleName, true
		}
	}
	return "", false
}

func replaceOrAppendMainPackageInitAll(rustCode string, plan generatedPackageInitPlan) string {
	replacement := renderMainPackageInitAll(plan)
	if updated, ok := replacePackageInitAllFunction(rustCode, replacement); ok {
		return updated
	}
	if strings.HasSuffix(rustCode, "\n") {
		return rustCode + "\n" + replacement
	}
	return rustCode + "\n\n" + replacement
}

func renderMainPackageInitAll(plan generatedPackageInitPlan) string {
	var out strings.Builder
	out.WriteString("pub(crate) fn __go_init_all() {\n")
	writePackageInitPlanCalls(&out, plan, "    ")
	out.WriteString("}\n")
	return out.String()
}

func replacePackageInitAllFunction(rustCode, replacement string) (string, bool) {
	const marker = "pub(crate) fn __go_init_all() {"
	start := strings.Index(rustCode, marker)
	if start < 0 {
		return rustCode, false
	}
	depth := 0
	for i := start; i < len(rustCode); i++ {
		switch rustCode[i] {
		case '{':
			depth++
		case '}':
			depth--
			if depth == 0 {
				end := i + 1
				if end < len(rustCode) && rustCode[end] == '\n' {
					end++
				}
				return rustCode[:start] + replacement + rustCode[end:], true
			}
		}
	}
	return rustCode, false
}

func ensureMainCallsPackageInitAll(rustCode string) string {
	if strings.Contains(rustCode, "__go_init_all();") {
		return rustCode
	}
	const marker = "fn main() {"
	insertAt := strings.Index(rustCode, marker)
	if insertAt < 0 {
		return rustCode
	}
	insertAt += len(marker)
	return rustCode[:insertAt] + "\n    __go_init_all();\n" + rustCode[insertAt:]
}

func writePackageInitPlanCalls(out *strings.Builder, plan generatedPackageInitPlan, indent string) {
	for _, moduleName := range plan.zeroModules {
		writePackageInitPlanCall(out, indent, moduleName, "__go_zero_globals")
	}
	for _, step := range plan.orderSteps {
		writePackageInitPlanCall(out, indent, step.moduleName, step.functionName)
	}
	for _, moduleName := range plan.initFunctionModules {
		writePackageInitPlanCall(out, indent, moduleName, "__go_init_functions")
	}
}

func writePackageInitPlanCall(out *strings.Builder, indent, moduleName, functionName string) {
	out.WriteString(indent)
	if moduleName == "" {
		out.WriteString("self::")
	} else {
		out.WriteString(moduleName)
		out.WriteString("::")
	}
	out.WriteString(functionName)
	out.WriteString("();\n")
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
	if pg.generatedRustContains("num::Complex") {
		needsNum = true
	}
	needsSerdeJSON := pg.generatedRustContains("serde_json::") || pg.generatedRustContains("pub use serde_json")
	needsGosyn := pg.generatedRustContains("gosyn::")

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
		// Sanitize package name to ensure it's valid for Cargo.
		packageName = SanitizeRustCrateName(packageName)

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
	if needsNum || needsSerdeJSON || needsGosyn || len(dependencyCrateNames) > 0 {
		cargoContent += "\n[dependencies]\n"
		if needsNum {
			cargoContent += "num = \"0.4\"\n"
		}
		if needsSerdeJSON {
			cargoContent += "serde_json = \"1\"\n"
		}
		if needsGosyn {
			cargoContent += "gosyn = \"0.2.9\"\n"
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

func (pg *ProjectGenerator) generatedRustContains(needle string) bool {
	entries, err := os.ReadDir(pg.projectPath)
	if err != nil {
		return false
	}
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".rs") {
			continue
		}
		content, err := os.ReadFile(filepath.Join(pg.projectPath, entry.Name()))
		if err != nil {
			continue
		}
		if strings.Contains(string(content), needle) {
			return true
		}
	}
	return false
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

func (pg *ProjectGenerator) generateLibRs(packageState *PackageState, astFilesByPath map[string]*ast.File) error {
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
	var initDependencyCrates []string
	if pg.externalMode == ModeTranspile && len(pg.packageMapping) > 0 {
		initDependencyCrates = pg.sortedCrateNames()
	}
	writeLibraryPackageInitAll(&libRust, initDependencyCrates, pg.initModules, pg.packageInitPlan(astFilesByPath))

	libRsPath := filepath.Join(pg.projectPath, "lib.rs")
	return os.WriteFile(libRsPath, []byte(libRust.String()), 0644)
}

func writeLibraryPackageInitAll(out *strings.Builder, dependencyCrates []string, initModules []generatedInitModule, initPlan generatedPackageInitPlan) {
	out.WriteString("\n\nstatic __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();\n\n")
	out.WriteString("pub fn __go_init_all() {\n")
	out.WriteString("    __GO_INIT_ONCE.call_once(|| {\n")
	for _, crateName := range dependencyCrates {
		if crateName == "" || crateName == sharedStdlibStubCrateName {
			continue
		}
		out.WriteString("        ")
		out.WriteString(crateName)
		out.WriteString("::__go_init_all();\n")
	}
	if initPlan.usesPackageWideInit() {
		writePackageInitPlanCalls(out, initPlan, "        ")
	} else {
		for _, initModule := range initModules {
			out.WriteString("        ")
			out.WriteString(initModule.moduleName)
			out.WriteString("::")
			out.WriteString(initModule.initFunctionName)
			out.WriteString("();\n")
		}
	}
	out.WriteString("    });\n")
	out.WriteString("}\n")
}

func (pg *ProjectGenerator) writePackageHelperFile(packageState *PackageState) error {
	helpers := pg.packageHelperModuleCode(packageState)
	helperPath := filepath.Join(pg.projectPath, packageHelperIncludeFile)
	return os.WriteFile(helperPath, []byte(helpers), 0644)
}
