package main

import (
	"go/ast"
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
