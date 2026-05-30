package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	gotypes "go/types"
	"strings"
	"testing"
)

func TestTranspileContextOwnsSessionPackageAndFileState(t *testing.T) {
	typeInfo := &TypeInfo{}
	ctx := &TranspileContext{
		Session: NewTranspileSession(typeInfo, map[string]string{"example.com/dep": "example_com_dep"}),
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}

	SetTranspileContext(ctx)
	defer SetTranspileContext(nil)

	SetTypeInfo(typeInfo)
	RegisterFunctionSignature("fn_name", &FunctionSignature{})
	RegisterErrorImplType("MyError")
	SetPackageImports(map[string]string{
		"dep": "example.com/dep",
		"fmt": "fmt",
	})
	TrackImport("Rc")
	NeedFormatAny()

	if GetTypeInfo() != typeInfo {
		t.Fatalf("GetTypeInfo() should return the session type info")
	}
	if ctx.Package.FunctionSignatures["fn_name"] == nil {
		t.Fatalf("function signature should be recorded in package state")
	}
	if !ctx.Package.ErrorImplTypes["MyError"] {
		t.Fatalf("error implementation type should be recorded in package state")
	}
	if ctx.Package.GoPackageImports["dep"] != "example.com/dep" {
		t.Fatalf("package imports should be stored in package state, got %#v", ctx.Package.GoPackageImports)
	}
	if !ctx.Package.ExternalPackages["example.com/dep"] {
		t.Fatalf("external package set should be stored in package state")
	}
	if !ctx.File.Imports.needs["Rc"] || !ctx.File.Imports.needs["Any"] {
		t.Fatalf("file imports should be tracked in file state, got %#v", ctx.File.Imports.needs)
	}
	if !ctx.File.Helpers.needsFormatAny {
		t.Fatalf("helper usage should be tracked in file state")
	}
}

func TestSetTranspileContextSyncsFileCompatibilityState(t *testing.T) {
	ctx := &TranspileContext{
		Session: NewTranspileSession(nil, nil),
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}

	SetTranspileContext(ctx)
	currentReceiver = "recv"
	currentReceiverType = "Thing"
	currentFunctionHasDefer = true
	pendingLoopLabel = "outer"
	hasInitFunction = true
	SetTranspileContext(nil)

	if ctx.File.CurrentReceiver != "recv" {
		t.Fatalf("CurrentReceiver = %q, want recv", ctx.File.CurrentReceiver)
	}
	if ctx.File.CurrentReceiverType != "Thing" {
		t.Fatalf("CurrentReceiverType = %q, want Thing", ctx.File.CurrentReceiverType)
	}
	if !ctx.File.CurrentFunctionHasDefer {
		t.Fatalf("CurrentFunctionHasDefer should sync back into file state")
	}
	if ctx.File.PendingLoopLabel != "outer" {
		t.Fatalf("PendingLoopLabel = %q, want outer", ctx.File.PendingLoopLabel)
	}
	if !ctx.File.HasInitFunction {
		t.Fatalf("HasInitFunction should sync back into file state")
	}
}

func TestTranspileContextScopesAnonymousStructsToPackage(t *testing.T) {
	savedCounter := anonymousStructCounter
	savedStructs := anonymousStructs
	savedTypeMap := anonymousStructTypeMap
	defer func() {
		anonymousStructCounter = savedCounter
		anonymousStructs = savedStructs
		anonymousStructTypeMap = savedTypeMap
		SetTranspileContext(nil)
	}()

	ctxA := &TranspileContext{
		Session: NewTranspileSession(nil, nil),
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}
	SetTranspileContext(ctxA)
	firstName := generateAnonymousStructType(&ast.StructType{Fields: &ast.FieldList{
		List: []*ast.Field{{Names: []*ast.Ident{ast.NewIdent("A")}, Type: ast.NewIdent("int")}},
	}})
	if firstName != "AnonymousStruct1" {
		t.Fatalf("first package anonymous struct = %q, want AnonymousStruct1", firstName)
	}
	SetTranspileContext(nil)

	ctxB := &TranspileContext{
		Session: NewTranspileSession(nil, nil),
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}
	SetTranspileContext(ctxB)
	if len(anonymousStructs) != 0 {
		t.Fatalf("new package should not see prior anonymous structs, got %#v", anonymousStructs)
	}
	secondName := generateAnonymousStructType(&ast.StructType{Fields: &ast.FieldList{
		List: []*ast.Field{{Names: []*ast.Ident{ast.NewIdent("B")}, Type: ast.NewIdent("int")}},
	}})
	if secondName != "AnonymousStruct1" {
		t.Fatalf("second package anonymous struct = %q, want AnonymousStruct1", secondName)
	}
}

func TestPackageFunctionNameOverridesDisambiguateCaseCollisions(t *testing.T) {
	fset := token.NewFileSet()
	exportedFile, err := parser.ParseFile(fset, "versions.go", `package versions

func Compare(x string) string { return compare(x) }
`, 0)
	if err != nil {
		t.Fatalf("ParseFile exported: %v", err)
	}
	privateFile, err := parser.ParseFile(fset, "gover.go", `package versions

func compare(x string) string { return x }
`, 0)
	if err != nil {
		t.Fatalf("ParseFile private: %v", err)
	}

	overrides := assignPackageFunctionNames([]*ast.File{privateFile, exportedFile})
	if overrides["Compare"] != "" {
		t.Fatalf("Compare should keep the base Rust name, got override %q", overrides["Compare"])
	}
	if overrides["compare"] != "compare_1" {
		t.Fatalf("compare override = %q, want compare_1", overrides["compare"])
	}

	savedOverrides := packageFunctionNameOverrides
	packageFunctionNameOverrides = overrides
	defer func() { packageFunctionNameOverrides = savedOverrides }()

	if got := rustFunctionNameForUse("Compare"); got != "compare" {
		t.Fatalf("rustFunctionNameForUse(Compare) = %q, want compare", got)
	}
	if got := rustFunctionNameForUse("compare"); got != "compare_1" {
		t.Fatalf("rustFunctionNameForUse(compare) = %q, want compare_1", got)
	}
}

func TestPackageMethodNameOverridesAreReceiverScoped(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "runner.go", `package runner

type Runner struct{}
type Other struct{}

func (r *Runner) RunPiped() string { return r.runPiped() }
func (r *Runner) runPiped() string { return "runner" }
func (o *Other) runPiped() string { return "other" }
`, 0)
	if err != nil {
		t.Fatalf("ParseFile: %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo: %v", err)
	}

	overrides := assignPackageMethodNames([]*ast.File{file}, typeInfo)
	methodOverride := func(receiver, name string) string {
		for _, decl := range file.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Recv == nil || fn.Name.Name != name || getReceiverType(fn.Recv.List[0].Type) != receiver {
				continue
			}
			return overrides[methodOverrideKey(methodFuncForDecl(fn, typeInfo))]
		}
		t.Fatalf("method %s.%s not found", receiver, name)
		return ""
	}

	if got := methodOverride("Runner", "RunPiped"); got != "" {
		t.Fatalf("Runner.RunPiped should keep the base Rust name, got override %q", got)
	}
	if got := methodOverride("Runner", "runPiped"); got != "run_piped_1" {
		t.Fatalf("Runner.runPiped override = %q, want run_piped_1", got)
	}
	if got := methodOverride("Other", "runPiped"); got != "" {
		t.Fatalf("Other.runPiped should not inherit Runner override, got %q", got)
	}
}

func TestExternalPackageTypesUseMappedCratePaths(t *testing.T) {
	savedTypeInfo := currentTypeInfo
	savedGoImports := goPackageImports
	savedExternalPackages := externalPackages
	defer func() {
		currentTypeInfo = savedTypeInfo
		goPackageImports = savedGoImports
		externalPackages = savedExternalPackages
		SetTranspileContext(nil)
	}()

	mainPkg := gotypes.NewPackage("example.com/main", "main")
	depPkg := gotypes.NewPackage("example.com/dep", "dep")
	typeName := gotypes.NewTypeName(token.NoPos, depPkg, "Thing", nil)
	named := gotypes.NewNamed(typeName, gotypes.NewStruct(nil, nil), nil)

	SetTranspileContext(&TranspileContext{
		Session: NewTranspileSession(&TypeInfo{pkg: mainPkg}, map[string]string{
			"example.com/dep": "example_com_dep",
		}),
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	})
	SetPackageImports(map[string]string{"dep": "example.com/dep"})

	if got := goTypesNamedTypeToRust(named); got != "example_com_dep::Thing" {
		t.Fatalf("goTypesNamedTypeToRust() = %q, want mapped crate type", got)
	}
	got := goTypeToRustBase(&ast.SelectorExpr{
		X:   ast.NewIdent("dep"),
		Sel: ast.NewIdent("Thing"),
	})
	if got != "example_com_dep::Thing" {
		t.Fatalf("goTypeToRustBase(selector) = %q, want mapped crate type", got)
	}
}

func TestSourceMappedStdlibTypesUseMappedCratePaths(t *testing.T) {
	savedTypeInfo := currentTypeInfo
	savedGoImports := goPackageImports
	savedExternalPackages := externalPackages
	defer func() {
		currentTypeInfo = savedTypeInfo
		goPackageImports = savedGoImports
		externalPackages = savedExternalPackages
		SetTranspileContext(nil)
	}()

	mainPkg := gotypes.NewPackage("example.com/main", "main")
	stringsPkg := gotypes.NewPackage("strings", "strings")
	typeName := gotypes.NewTypeName(token.NoPos, stringsPkg, "Builder", nil)
	named := gotypes.NewNamed(typeName, gotypes.NewStruct(nil, nil), nil)

	SetTranspileContext(&TranspileContext{
		Session: NewTranspileSession(&TypeInfo{pkg: mainPkg}, map[string]string{
			"strings": "strings",
		}),
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	})
	SetPackageImports(map[string]string{"strings": "strings"})

	if got := goTypesNamedTypeToRust(named); got != "strings::Builder" {
		t.Fatalf("goTypesNamedTypeToRust(source-mapped strings.Builder) = %q, want strings::Builder", got)
	}
	selector := &ast.SelectorExpr{
		X:   ast.NewIdent("strings"),
		Sel: ast.NewIdent("Builder"),
	}
	if got := goTypeToRustBase(selector); got != "strings::Builder" {
		t.Fatalf("goTypeToRustBase(source-mapped strings.Builder) = %q, want strings::Builder", got)
	}
	if got := zeroValueForGoType(selector); got != "Default::default()" {
		t.Fatalf("zeroValueForGoType(source-mapped strings.Builder) = %q, want Default::default()", got)
	}
}

func TestPackageTypeMetadataPrefersContextState(t *testing.T) {
	savedInterfaceTypes := interfaceTypes
	savedTypeAliases := typeAliases
	savedTypeDefinitions := typeDefinitions
	defer func() {
		interfaceTypes = savedInterfaceTypes
		typeAliases = savedTypeAliases
		typeDefinitions = savedTypeDefinitions
	}()

	ctx := &TranspileContext{
		Session: NewTranspileSession(nil, nil),
		Package: &PackageState{
			InterfaceTypes: map[string]bool{"LocalIface": true},
			TypeAliases:    map[string]bool{"LocalAlias": true},
			TypeDefinitions: map[string]string{
				"LocalInt": "int",
			},
		},
		File: NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}

	SetTranspileContext(ctx)
	defer SetTranspileContext(nil)

	interfaceTypes = map[string]bool{"GlobalIface": true}
	typeAliases = map[string]bool{}
	typeDefinitions = map[string]string{}

	localIfaceIdent := ast.NewIdent("LocalIface")
	currentPkg := gotypes.NewPackage("example.com/current", "current")
	readMethod := gotypes.NewFunc(
		token.NoPos,
		currentPkg,
		"Read",
		gotypes.NewSignatureType(nil, nil, nil, gotypes.NewTuple(), gotypes.NewTuple(), false),
	)
	localIfaceType := gotypes.NewNamed(
		gotypes.NewTypeName(token.NoPos, currentPkg, "LocalIface", nil),
		gotypes.NewInterfaceType([]*gotypes.Func{readMethod}, nil).Complete(),
		nil,
	)
	SetTypeInfo(&TypeInfo{
		info: &gotypes.Info{
			Types: map[ast.Expr]gotypes.TypeAndValue{
				localIfaceIdent: {Type: localIfaceType},
			},
		},
		pkg: currentPkg,
	})

	if got := GoTypeToRustParam(localIfaceIdent); got != "Rc<RefCell<Option<Box<dyn LocalIface>>>>" {
		t.Fatalf("GoTypeToRustParam() = %q, want type info to prove context-owned interface", got)
	}
	if got := GoTypeToRust(ast.NewIdent("LocalAlias")); got != "LocalAlias" {
		t.Fatalf("GoTypeToRust() = %q, want type alias package state to win", got)
	}
	if IsParamValueType(&FunctionSignature{Params: []*ast.Field{{Type: ast.NewIdent("LocalIface")}}}, 0) {
		t.Fatalf("IsParamValueType() should treat context-owned interface types as non-value types")
	}

	var out strings.Builder
	TranspileTypeConversion(&out, &ast.CallExpr{
		Fun:  ast.NewIdent("LocalInt"),
		Args: []ast.Expr{&ast.BasicLit{Kind: token.INT, Value: "1"}},
	})
	if !strings.Contains(out.String(), "LocalInt(") {
		t.Fatalf("TranspileTypeConversion() = %q, want type definition package state to win", out.String())
	}
}
