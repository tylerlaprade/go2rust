package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
	"strings"
	"testing"
)

func TestGoTypesTypeToRustMapsUnsafePointer(t *testing.T) {
	if got, want := goTypesTypeToRust(types.Typ[types.UnsafePointer]), "usize"; got != want {
		t.Fatalf("goTypesTypeToRust(unsafe.Pointer) = %q, want %q", got, want)
	}
}

func TestGoTypesTypeToRustMapsComplexBasics(t *testing.T) {
	for _, tc := range []struct {
		name string
		typ  types.Type
		want string
	}{
		{"complex64", types.Typ[types.Complex64], "num::Complex<f32>"},
		{"complex128", types.Typ[types.Complex128], "num::Complex<f64>"},
		{"untyped complex", types.Typ[types.UntypedComplex], "num::Complex<f64>"},
	} {
		if got := goTypesTypeToRust(tc.typ); got != tc.want {
			t.Fatalf("goTypesTypeToRust(%s) = %q, want %q", tc.name, got, tc.want)
		}
		if got := goTypesReturnTypeToRust(tc.typ); strings.Contains(got, "unknown basic type") {
			t.Fatalf("goTypesReturnTypeToRust(%s) should not emit an unknown basic type: %q", tc.name, got)
		}
	}
}

func TestGoTypesTypeToRustUsesAnyForUnnamedInterfaces(t *testing.T) {
	method := types.NewFunc(
		token.NoPos,
		nil,
		"Read",
		types.NewSignatureType(
			nil,
			nil,
			nil,
			types.NewTuple(),
			types.NewTuple(types.NewVar(token.NoPos, nil, "n", types.Typ[types.Int])),
			false,
		),
	)
	iface := types.NewInterfaceType([]*types.Func{method}, nil).Complete()

	got := goTypesTypeToRustWrapped(iface)
	want := "Rc<RefCell<Option<Box<dyn Any>>>>"
	if got != want {
		t.Fatalf("goTypesTypeToRustWrapped(non-empty interface) = %q, want %q", got, want)
	}
}

func TestGoTypesReturnTypeToRustWrapsCurrentStdlibInterfaceTraitObject(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()

	method := types.NewFunc(
		token.NoPos,
		nil,
		"Name",
		types.NewSignatureType(
			nil,
			nil,
			nil,
			types.NewTuple(),
			types.NewTuple(types.NewVar(token.NoPos, nil, "", types.Typ[types.String])),
			false,
		),
	)
	iface := types.NewInterfaceType([]*types.Func{method}, nil).Complete()
	reflectPkg := types.NewPackage("reflect", "reflect")
	named := types.NewNamed(types.NewTypeName(token.NoPos, reflectPkg, "Type", nil), iface, nil)

	SetTypeInfo(&TypeInfo{pkg: reflectPkg})
	got := goTypesReturnTypeToRust(named)
	if strings.Contains(got, "Option<Type>") {
		t.Fatalf("current stdlib named interface return used bare trait type: %q", got)
	}
	if !strings.Contains(got, "Option<Box<dyn Type") {
		t.Fatalf("current stdlib named interface return should wrap a trait object, got %q", got)
	}
}

func TestGoTypesNamedTypeToRustUsesCurrentPackageModuleForSamePathPackage(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevCtx := GetTranspileContext()
	defer func() {
		currentTypeInfo = prevTypeInfo
		SetTranspileContext(prevCtx)
	}()

	currentPkg := types.NewPackage("example.com/abi", "abi")
	declPkg := types.NewPackage("example.com/abi", "abi")
	named := types.NewNamed(
		types.NewTypeName(token.NoPos, declPkg, "Type", nil),
		types.NewStruct(nil, nil),
		nil,
	)

	typeInfo := &TypeInfo{pkg: currentPkg}
	SetTypeInfo(typeInfo)
	SetTranspileContext(&TranspileContext{
		CurrentModuleName: "iface",
		Package: &PackageState{
			TypeModuleNames: map[string]string{"Type": "r#type"},
		},
		PackageMapping: map[string]string{"example.com/abi": "example_com_abi"},
		Session: &TranspileSession{
			TypeInfo:       typeInfo,
			PackageMapping: map[string]string{"example.com/abi": "example_com_abi"},
			PackageTypeModuleNames: map[string]map[string]string{
				"example.com/abi": {"Type": "r#type"},
			},
		},
	})

	got := goTypesTypeToRust(named)
	if strings.Contains(got, "example_com_abi::") {
		t.Fatalf("same-package named type should not use the package's external crate path: %q", got)
	}
	if got != "crate::r#type::Type" {
		t.Fatalf("same-package named type = %q, want sibling module path", got)
	}
}

func TestTypeInfoUsesCoreTypeForTypeParameterRanges(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main
func Join[S ~[]T, T ~string](s S) {
	for _, v := range s { _ = string(v) }
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	fn := file.Decls[0].(*ast.FuncDecl)
	rangeStmt := fn.Body.List[0].(*ast.RangeStmt)

	if !typeInfo.IsSlice(rangeStmt.X) {
		t.Fatalf("type parameter S ~[]T should be treated as a slice")
	}
	elem := typeInfo.GetArrayOrSliceElemType(rangeStmt.X)
	if elem != types.Typ[types.String] {
		t.Fatalf("range elem type = %v, want string", elem)
	}
	preservedElem := typeInfo.GetArrayOrSliceElemTypePreservingTypeParam(rangeStmt.X)
	if _, ok := types.Unalias(preservedElem).(*types.TypeParam); !ok {
		t.Fatalf("preserved range elem type = %v, want type parameter", preservedElem)
	}
}

func TestTypeInfoPreservesDirectSliceTypeParamElem(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main
func Each[T comparable](in []T) {
	for _, v := range in { _ = v }
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	fn := file.Decls[0].(*ast.FuncDecl)
	rangeStmt := fn.Body.List[0].(*ast.RangeStmt)

	elem := typeInfo.GetArrayOrSliceElemTypePreservingTypeParam(rangeStmt.X)
	if _, ok := types.Unalias(elem).(*types.TypeParam); !ok {
		t.Fatalf("direct []T elem type = %v, want type parameter", elem)
	}
}

// Bug: types.Config.Check used to be invoked as `pkg, _ := config.Check(...)`,
// and a later refactor swallowed partial errors onto stderr without ever
// returning them. Downstream code-gen then ran on partial type info as if it
// were complete, which is the original source of the "syntax fallback"
// incident documented in AGENTS.md. These tests pin the contract: every
// type-check error must reach the caller via the returned error.
func TestNewTypeInfoWithImporterPropagatesPartialErrors(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main
func F() { _ = doesNotExist + 1 }`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("main", []*ast.File{file}, fset, nil)
	if err == nil {
		t.Fatalf("NewTypeInfoWithImporter: expected error for undefined identifier, got nil")
	}
	if typeInfo == nil {
		t.Fatalf("NewTypeInfoWithImporter: expected partial TypeInfo to be returned alongside error, got nil")
	}
	if !strings.Contains(err.Error(), "doesNotExist") {
		t.Fatalf("NewTypeInfoWithImporter: error should mention the failing identifier; got %v", err)
	}
	if !strings.Contains(err.Error(), "main") {
		t.Fatalf("NewTypeInfoWithImporter: error should mention the package label; got %v", err)
	}
}

func TestNewTypeInfoWithImporterJoinsAllCheckErrors(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main
func F() {
	_ = missingA
	_ = missingB
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("main", []*ast.File{file}, fset, nil)
	if err == nil {
		t.Fatalf("expected joined error, got nil")
	}
	if typeInfo == nil {
		t.Fatalf("expected partial TypeInfo alongside joined error, got nil")
	}
	for _, want := range []string{"missingA", "missingB"} {
		if !strings.Contains(err.Error(), want) {
			t.Fatalf("joined error should mention %q; got %v", want, err)
		}
	}
}

func TestNewTypeInfoWithImporterCleanInputHasNoError(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main
func F() int { return 1 + 2 }`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfoWithImporter("main", []*ast.File{file}, fset, nil)
	if err != nil {
		t.Fatalf("clean input should not produce a type-check error, got %v", err)
	}
	if typeInfo == nil {
		t.Fatalf("clean input should produce TypeInfo")
	}
}

func TestExternalStubDefaultValueUsesNoneForAnyTraitObjects(t *testing.T) {
	var out strings.Builder
	writeExternalStubDefaultValue(&out, "Rc<RefCell<Option<Box<dyn Any>>>>")

	got := out.String()
	want := "Rc::new(RefCell::new(None::<Box<dyn Any>>))"
	if got != want {
		t.Fatalf("writeExternalStubDefaultValue(Box<dyn Any>) = %q, want %q", got, want)
	}
}

func TestGoTypeToRustParamUsesTypeInfoForImportedInterfaces(t *testing.T) {
	paramType := types.Typ[types.Int]
	method := types.NewFunc(
		token.NoPos,
		nil,
		"Find",
		types.NewSignatureType(
			nil,
			nil,
			nil,
			types.NewTuple(types.NewVar(token.NoPos, nil, "key", paramType)),
			types.NewTuple(types.NewVar(token.NoPos, nil, "", paramType)),
			false,
		),
	)
	iface := types.NewInterfaceType([]*types.Func{method}, nil).Complete()
	labelPkg := types.NewPackage("example.com/label", "label")
	named := types.NewNamed(types.NewTypeName(token.NoPos, labelPkg, "Map", nil), iface, nil)
	expr := &ast.SelectorExpr{X: ast.NewIdent("label"), Sel: ast.NewIdent("Map")}

	SetTypeInfo(&TypeInfo{
		info: &types.Info{
			Types: map[ast.Expr]types.TypeAndValue{
				expr: {Type: named},
			},
		},
		pkg: types.NewPackage("example.com/main", "main"),
	})
	defer SetTypeInfo(nil)

	got := GoTypeToRustParam(expr)
	want := "Rc<RefCell<Option<Box<dyn example_com_label::Map>>>>"
	if got != want {
		t.Fatalf("GoTypeToRustParam(imported interface) = %q, want %q", got, want)
	}
}

func TestGoTypesFunctionParamUsesBareImportedInterfaceReference(t *testing.T) {
	paramType := types.Typ[types.Int]
	method := types.NewFunc(
		token.NoPos,
		nil,
		"Find",
		types.NewSignatureType(
			nil,
			nil,
			nil,
			types.NewTuple(types.NewVar(token.NoPos, nil, "key", paramType)),
			types.NewTuple(types.NewVar(token.NoPos, nil, "", paramType)),
			false,
		),
	)
	iface := types.NewInterfaceType([]*types.Func{method}, nil).Complete()
	labelPkg := types.NewPackage("example.com/label", "label")
	named := types.NewNamed(types.NewTypeName(token.NoPos, labelPkg, "Map", nil), iface, nil)

	if got, want := goTypesParamTypeToRust(named), "Rc<RefCell<Option<Box<dyn example_com_label::Map>>>>"; got != want {
		t.Fatalf("goTypesParamTypeToRust(imported interface) = %q, want %q", got, want)
	}
	if got, want := goTypesFunctionParamTypeToRust(named), "&dyn example_com_label::Map"; got != want {
		t.Fatalf("goTypesFunctionParamTypeToRust(imported interface) = %q, want %q", got, want)
	}
}

func TestGoTypesFunctionParamUsesWrappedLocalInterfaceHandle(t *testing.T) {
	paramType := types.Typ[types.Int]
	method := types.NewFunc(
		token.NoPos,
		nil,
		"Find",
		types.NewSignatureType(
			nil,
			nil,
			nil,
			types.NewTuple(types.NewVar(token.NoPos, nil, "key", paramType)),
			types.NewTuple(types.NewVar(token.NoPos, nil, "", paramType)),
			false,
		),
	)
	iface := types.NewInterfaceType([]*types.Func{method}, nil).Complete()
	pkg := types.NewPackage("example.com/mainmod", "main")
	named := types.NewNamed(types.NewTypeName(token.NoPos, pkg, "Map", nil), iface, nil)

	SetTypeInfo(&TypeInfo{pkg: pkg})
	defer SetTypeInfo(nil)

	if got, want := goTypesFunctionParamTypeToRust(named), "Rc<RefCell<Option<Box<dyn Map>>>>"; got != want {
		t.Fatalf("goTypesFunctionParamTypeToRust(local interface) = %q, want %q", got, want)
	}
}

func TestGoTypesNamedFunctionTypeSkipsStubBackedStdlibPackage(t *testing.T) {
	yieldSig := types.NewSignatureType(
		nil,
		nil,
		nil,
		types.NewTuple(types.NewVar(token.NoPos, nil, "v", types.Typ[types.Int])),
		types.NewTuple(types.NewVar(token.NoPos, nil, "", types.Typ[types.Bool])),
		false,
	)
	seqSig := types.NewSignatureType(
		nil,
		nil,
		nil,
		types.NewTuple(types.NewVar(token.NoPos, nil, "yield", yieldSig)),
		nil,
		false,
	)
	iterPkg := types.NewPackage("iter", "iter")
	named := types.NewNamed(types.NewTypeName(token.NoPos, iterPkg, "Seq", nil), seqSig, nil)

	SetTypeInfo(&TypeInfo{pkg: types.NewPackage("example.com/main", "main")})
	defer SetTypeInfo(nil)

	if got, ok := goTypesNamedFunctionTypeToRust(named); ok {
		t.Fatalf("stub-backed stdlib function type should not use external stub name, got %q", got)
	}
	if got := goTypesReturnTypeToRust(named); got == "iter_Seq" {
		t.Fatalf("stub-backed stdlib function return should keep the underlying function handle, got %q", got)
	}
}

func TestGoTypeToRustParamDoesNotTrustInterfaceRegistryWithoutTypeInfo(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevContext := currentContext
	prevInterfaces := interfaceTypes
	prevDetector := globalConcurrencyDetector
	defer func() {
		currentTypeInfo = prevTypeInfo
		SetTranspileContext(prevContext)
		interfaceTypes = prevInterfaces
		globalConcurrencyDetector = prevDetector
	}()

	SetTranspileContext(nil)
	SetTypeInfo(nil)
	interfaceTypes = map[string]bool{"entry": true}
	globalConcurrencyDetector = nil

	got := GoTypeToRustParam(ast.NewIdent("entry"))
	want := "Rc<RefCell<Option<entry>>>"
	if got != want {
		t.Fatalf("GoTypeToRustParam without type info = %q, want %q", got, want)
	}
}

func TestCallParamTypeFromTypeInfoUsesPackageSelectorObject(t *testing.T) {
	pkg := types.NewPackage("example.com/label", "label")
	paramType := types.Typ[types.Int]
	fn := types.NewFunc(
		token.NoPos,
		pkg,
		"Of64",
		types.NewSignatureType(
			nil,
			nil,
			nil,
			types.NewTuple(types.NewVar(token.NoPos, nil, "key", paramType)),
			types.NewTuple(),
			false,
		),
	)
	sel := &ast.SelectorExpr{X: ast.NewIdent("label"), Sel: ast.NewIdent("Of64")}
	call := &ast.CallExpr{Fun: sel, Args: []ast.Expr{ast.NewIdent("k")}}

	SetTypeInfo(&TypeInfo{
		info: &types.Info{
			Uses: map[*ast.Ident]types.Object{
				sel.Sel: fn,
			},
		},
		pkg: types.NewPackage("example.com/main", "main"),
	})
	defer SetTypeInfo(nil)

	if got := callParamTypeFromTypeInfo(call, 0); got != paramType {
		t.Fatalf("callParamTypeFromTypeInfo(package selector) = %v, want %v", got, paramType)
	}
}

func TestCollectImportedInterfaceImplsRecordsCurrentConcreteArgs(t *testing.T) {
	labelPkg := types.NewPackage("example.com/label", "label")
	keysPkg := types.NewPackage("example.com/keys", "keys")
	stringType := types.Typ[types.String]

	nameMethod := types.NewFunc(
		token.NoPos,
		labelPkg,
		"Name",
		types.NewSignatureType(
			nil,
			nil,
			nil,
			types.NewTuple(),
			types.NewTuple(types.NewVar(token.NoPos, nil, "", stringType)),
			false,
		),
	)
	keyIface := types.NewInterfaceType([]*types.Func{nameMethod}, nil).Complete()
	keyNamed := types.NewNamed(types.NewTypeName(token.NoPos, labelPkg, "Key", nil), keyIface, nil)

	valueNamed := types.NewNamed(types.NewTypeName(token.NoPos, keysPkg, "Value", nil), types.NewStruct(nil, nil), nil)
	valuePtr := types.NewPointer(valueNamed)
	valueNameMethod := types.NewFunc(
		token.NoPos,
		keysPkg,
		"Name",
		types.NewSignatureType(
			types.NewVar(token.NoPos, keysPkg, "", valuePtr),
			nil,
			nil,
			types.NewTuple(),
			types.NewTuple(types.NewVar(token.NoPos, nil, "", stringType)),
			false,
		),
	)
	valueNamed.AddMethod(valueNameMethod)

	of64 := types.NewFunc(
		token.NoPos,
		labelPkg,
		"Of64",
		types.NewSignatureType(
			nil,
			nil,
			nil,
			types.NewTuple(types.NewVar(token.NoPos, nil, "key", keyNamed)),
			types.NewTuple(),
			false,
		),
	)

	arg := ast.NewIdent("k")
	sel := &ast.SelectorExpr{X: ast.NewIdent("label"), Sel: ast.NewIdent("Of64")}
	file := &ast.File{
		Name: ast.NewIdent("keys"),
		Decls: []ast.Decl{
			&ast.FuncDecl{
				Name: ast.NewIdent("useKey"),
				Type: &ast.FuncType{},
				Body: &ast.BlockStmt{
					List: []ast.Stmt{
						&ast.ExprStmt{X: &ast.CallExpr{Fun: sel, Args: []ast.Expr{arg}}},
					},
				},
			},
		},
	}

	typeInfo := &TypeInfo{
		info: &types.Info{
			Uses: map[*ast.Ident]types.Object{
				sel.Sel: of64,
				arg:     types.NewVar(token.NoPos, keysPkg, "k", valuePtr),
			},
		},
		pkg: keysPkg,
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	analysis := analyzeTranspileFile(file, typeInfo)
	if _, ok := analysis.importedInterfaceImpls["Value"]["example_com_label::Key"]; ok {
		t.Fatalf("imported interface value impls = %#v, did not want Value to implement imported Key", analysis.importedInterfaceImpls)
	}
	if _, ok := analysis.importedPointerInterfaceImpls["Value"]["example_com_label::Key"]; !ok {
		t.Fatalf("imported interface pointer impls = %#v, want *Value to implement imported Key", analysis.importedPointerInterfaceImpls)
	}
}

func TestCollectImportedInterfaceImplsFromPackageImports(t *testing.T) {
	ioPkg := types.NewPackage("io", "io")
	bytesPkg := types.NewPackage("bytes", "bytes")
	bytesPkg.SetImports([]*types.Package{ioPkg})
	byteSlice := types.NewSlice(types.Typ[types.Byte])
	errorType := types.Universe.Lookup("error").Type()

	writeMethod := types.NewFunc(
		token.NoPos,
		ioPkg,
		"Write",
		types.NewSignatureType(
			nil,
			nil,
			nil,
			types.NewTuple(types.NewVar(token.NoPos, nil, "p", byteSlice)),
			types.NewTuple(
				types.NewVar(token.NoPos, nil, "n", types.Typ[types.Int]),
				types.NewVar(token.NoPos, nil, "err", errorType),
			),
			false,
		),
	)
	writerIface := types.NewInterfaceType([]*types.Func{writeMethod}, nil).Complete()
	writerName := types.NewTypeName(token.NoPos, ioPkg, "Writer", nil)
	writerNamed := types.NewNamed(writerName, writerIface, nil)
	ioPkg.Scope().Insert(writerName)

	bufferName := types.NewTypeName(token.NoPos, bytesPkg, "Buffer", nil)
	bufferNamed := types.NewNamed(bufferName, types.NewStruct(nil, nil), nil)
	bytesPkg.Scope().Insert(bufferName)
	bufferPtr := types.NewPointer(bufferNamed)
	bufferNamed.AddMethod(types.NewFunc(
		token.NoPos,
		bytesPkg,
		"Write",
		types.NewSignatureType(
			types.NewVar(token.NoPos, bytesPkg, "", bufferPtr),
			nil,
			nil,
			types.NewTuple(types.NewVar(token.NoPos, nil, "p", byteSlice)),
			types.NewTuple(
				types.NewVar(token.NoPos, nil, "n", types.Typ[types.Int]),
				types.NewVar(token.NoPos, nil, "err", errorType),
			),
			false,
		),
	))

	typeInfo := &TypeInfo{pkg: bytesPkg, info: &types.Info{}}
	SetTypeInfo(typeInfo)
	SetTranspileContext(&TranspileContext{PackageMapping: map[string]string{
		"bytes": "bytes",
		"io":    "io",
	}})
	defer SetTypeInfo(nil)
	defer SetTranspileContext(nil)

	analysis := analyzeTranspileFiles(nil, typeInfo)
	if _, ok := analysis.importedInterfaceImpls["Buffer"]["io::Writer"]; ok {
		t.Fatalf("imported interface value impls = %#v, did not want Buffer value to implement io.Writer", analysis.importedInterfaceImpls)
	}
	if _, ok := analysis.importedPointerInterfaceImpls["Buffer"]["io::Writer"]; !ok {
		t.Fatalf("imported interface pointer impls = %#v, want *Buffer to implement io.Writer", analysis.importedPointerInterfaceImpls)
	}
	if !sourceMappedPointerWrapperAvailableForInterface(bufferNamed, writerNamed) {
		t.Fatalf("source-mapped pointer wrapper should be available for bytes.Buffer as io.Writer")
	}
}

func TestCollectImportedInterfaceImplsRecordsStructLiteralInterfaceFields(t *testing.T) {
	depPkg := types.NewPackage("example.com/dep", "dep")
	mainPkg := types.NewPackage("example.com/main", "main")
	stringType := types.Typ[types.String]

	importMethod := types.NewFunc(
		token.NoPos,
		depPkg,
		"Import",
		types.NewSignatureType(
			nil,
			nil,
			nil,
			types.NewTuple(types.NewVar(token.NoPos, nil, "path", stringType)),
			types.NewTuple(types.NewVar(token.NoPos, nil, "", types.Typ[types.Int])),
			false,
		),
	)
	importerIface := types.NewInterfaceType([]*types.Func{importMethod}, nil).Complete()
	importerNamed := types.NewNamed(types.NewTypeName(token.NoPos, depPkg, "Importer", nil), importerIface, nil)
	configNamed := types.NewNamed(
		types.NewTypeName(token.NoPos, depPkg, "Config", nil),
		types.NewStruct([]*types.Var{
			types.NewField(token.NoPos, depPkg, "Importer", importerNamed, false),
		}, nil),
		nil,
	)

	concreteNamed := types.NewNamed(types.NewTypeName(token.NoPos, mainPkg, "Importer", nil), types.NewStruct(nil, nil), nil)
	concretePtr := types.NewPointer(concreteNamed)
	concreteNamed.AddMethod(types.NewFunc(
		token.NoPos,
		mainPkg,
		"Import",
		types.NewSignatureType(
			types.NewVar(token.NoPos, mainPkg, "", concretePtr),
			nil,
			nil,
			types.NewTuple(types.NewVar(token.NoPos, nil, "path", stringType)),
			types.NewTuple(types.NewVar(token.NoPos, nil, "", types.Typ[types.Int])),
			false,
		),
	))

	value := ast.NewIdent("p")
	lit := &ast.CompositeLit{
		Elts: []ast.Expr{
			&ast.KeyValueExpr{Key: ast.NewIdent("Importer"), Value: value},
		},
	}
	file := &ast.File{
		Name: ast.NewIdent("main"),
		Decls: []ast.Decl{
			&ast.FuncDecl{
				Name: ast.NewIdent("use"),
				Type: &ast.FuncType{},
				Body: &ast.BlockStmt{List: []ast.Stmt{&ast.ExprStmt{X: lit}}},
			},
		},
	}

	typeInfo := &TypeInfo{
		info: &types.Info{
			Types: map[ast.Expr]types.TypeAndValue{
				lit:   {Type: configNamed},
				value: {Type: concretePtr},
			},
			Uses: map[*ast.Ident]types.Object{
				lit.Elts[0].(*ast.KeyValueExpr).Key.(*ast.Ident): configNamed.Underlying().(*types.Struct).Field(0),
			},
		},
		pkg: mainPkg,
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	analysis := analyzeTranspileFile(file, typeInfo)
	if _, ok := analysis.importedInterfaceImpls["Importer"]["example_com_dep::Importer"]; ok {
		t.Fatalf("imported interface value impls = %#v, did not want Importer value to implement imported struct field interface", analysis.importedInterfaceImpls)
	}
	if _, ok := analysis.importedPointerInterfaceImpls["Importer"]["example_com_dep::Importer"]; !ok {
		t.Fatalf("imported interface pointer impls = %#v, want *Importer to implement imported struct field interface", analysis.importedPointerInterfaceImpls)
	}
}
