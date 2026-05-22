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
	want := "&dyn example_com_label::Map"
	if got != want {
		t.Fatalf("GoTypeToRustParam(imported interface) = %q, want %q", got, want)
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

	SetTypeInfo(&TypeInfo{
		info: &types.Info{
			Uses: map[*ast.Ident]types.Object{
				sel.Sel: of64,
				arg:     types.NewVar(token.NoPos, keysPkg, "k", valuePtr),
			},
		},
		pkg: keysPkg,
	})
	defer SetTypeInfo(nil)

	impls := collectImportedInterfaceImpls(file)
	if _, ok := impls["Value"]["example_com_label::Key"]; !ok {
		t.Fatalf("collectImportedInterfaceImpls() = %#v, want Value to implement imported Key", impls)
	}
}
