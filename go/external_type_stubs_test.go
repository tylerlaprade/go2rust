package main

import (
	goast "go/ast"
	goconstant "go/constant"
	gotoken "go/token"
	gotypes "go/types"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestParserStubSurfaceRegistersEmptyStmt(t *testing.T) {
	prevContext := currentContext
	ctx := &TranspileContext{
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}
	SetTranspileContext(ctx)
	defer SetTranspileContext(prevContext)

	registerParserParseFileStubSurface()
	if !ctx.File.ExternalTypeStubs["ast_EmptyStmt"] {
		t.Fatalf("parser.ParseFile stub surface should register ast_EmptyStmt")
	}
}

func TestSourcePackageTypesDoNotRegisterExternalStubs(t *testing.T) {
	prevContext := currentContext
	ctx := &TranspileContext{
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}
	SetTranspileContext(ctx)
	defer SetTranspileContext(prevContext)

	RegisterExternalTypeStub("go_ast::ArrayType")
	RegisterExternalTypeStubInterface("go_ast::Expr")
	RegisterExternalIntegerTypeStub("go_token::Token", "i32")
	RegisterExternalTypeStubFieldByRustType("go_ast::ArrayType", "elt", "go_ast::Expr")

	if len(ctx.File.ExternalTypeStubs) != 0 {
		t.Fatalf("source-package Rust paths must not be emitted as external stubs: %#v", ctx.File.ExternalTypeStubs)
	}
	if len(ctx.File.ExternalTypeStubInterfaces) != 0 {
		t.Fatalf("source-package Rust paths must not be emitted as external interfaces: %#v", ctx.File.ExternalTypeStubInterfaces)
	}
	if len(ctx.File.ExternalTypeStubIntegerTypes) != 0 {
		t.Fatalf("source-package Rust paths must not be emitted as external integer types: %#v", ctx.File.ExternalTypeStubIntegerTypes)
	}
	if len(ctx.File.ExternalTypeStubFields) != 0 {
		t.Fatalf("source-package Rust paths must not get stub fields: %#v", ctx.File.ExternalTypeStubFields)
	}
}

func TestSourceMappedStdlibPackageIsNotStubBacked(t *testing.T) {
	prevContext := currentContext
	ctx := &TranspileContext{
		Package:        NewPackageState(),
		File:           NewFileState(NewImportTracker(), &HelperTracker{}, nil),
		PackageMapping: map[string]string{"go/types": "go_types"},
	}
	SetTranspileContext(ctx)
	defer SetTranspileContext(prevContext)

	if !isSourceMappedPackagePath("go/types") {
		t.Fatalf("go/types should be source-mapped")
	}
	if isStubBackedStdlibPackagePath("go/types") {
		t.Fatalf("source-mapped go/types must not be stub-backed")
	}
	if !isStubBackedStdlibPackagePath("go/ast") {
		t.Fatalf("unmapped go/ast should still be stub-backed")
	}
}

func TestGenericExternalPackageStubFunctionUsesSignatureTypeParams(t *testing.T) {
	prevContext := currentContext
	ctx := &TranspileContext{
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}
	SetTranspileContext(ctx)
	defer SetTranspileContext(prevContext)

	pkg := gotypes.NewPackage("slices", "slices")
	anyConstraint := gotypes.NewInterfaceType(nil, nil).Complete()
	elemName := gotypes.NewTypeName(gotoken.NoPos, pkg, "E", nil)
	elemParam := gotypes.NewTypeParam(elemName, anyConstraint)
	sliceConstraint := gotypes.NewInterfaceType(nil, []gotypes.Type{
		gotypes.NewUnion([]*gotypes.Term{
			gotypes.NewTerm(true, gotypes.NewSlice(elemParam)),
		}),
	}).Complete()
	sliceName := gotypes.NewTypeName(gotoken.NoPos, pkg, "S", nil)
	sliceParam := gotypes.NewTypeParam(sliceName, sliceConstraint)
	sig := gotypes.NewSignatureType(
		nil,
		nil,
		[]*gotypes.TypeParam{sliceParam, elemParam},
		gotypes.NewTuple(gotypes.NewVar(gotoken.NoPos, pkg, "s", sliceParam)),
		gotypes.NewTuple(gotypes.NewVar(gotoken.NoPos, pkg, "", sliceParam)),
		false,
	)

	RegisterExternalPackageStubFunction("slices", "clip", sig)
	got := GenerateExternalTypeStubs()
	for _, want := range []string{
		"pub fn clip<S, E>(",
		"_arg0: Rc<RefCell<Option<Vec<Rc<RefCell<Option<E>>>>>",
		"-> Rc<RefCell<Option<Vec<Rc<RefCell<Option<E>>>>>",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("generic external stub should preserve typed signature piece %q:\n%s", want, got)
		}
	}
	if strings.Contains(got, "pub fn clip<T0>") || strings.Contains(got, "pub fn clip<S, E, T0>") {
		t.Fatalf("generic external stub should not invent value-parameter generics:\n%s", got)
	}
}

func TestExternalPackageStubConstantsPreserveGoTypesValues(t *testing.T) {
	got := generateExternalStubs(
		nil,
		nil,
		nil,
		nil,
		nil,
		nil,
		nil,
		map[string]*externalPackageStub{
			"goarch": {
				Constants: map[string]string{
					"PTR_SIZE": "i32",
				},
				ConstantValues: map[string]goconstant.Value{
					"PTR_SIZE": goconstant.MakeInt64(8),
				},
			},
		},
	)

	if !strings.Contains(got, "pub const PTR_SIZE: i32 = 8;") {
		t.Fatalf("external package stub constants should preserve go/types values:\n%s", got)
	}
	if strings.Contains(got, "pub const PTR_SIZE: i32 = 0;") {
		t.Fatalf("external package stub constants must not silently default typed values:\n%s", got)
	}
}

func TestExternalPackageStubStringConstantsUseStaticStr(t *testing.T) {
	got := generateExternalStubs(
		nil,
		nil,
		nil,
		nil,
		nil,
		nil,
		nil,
		map[string]*externalPackageStub{
			"goos": {
				Constants: map[string]string{
					"G_O_O_S": "&'static str",
				},
				ConstantValues: map[string]goconstant.Value{
					"G_O_O_S": goconstant.MakeString("darwin"),
				},
			},
		},
	)

	if !strings.Contains(got, `pub const G_O_O_S: &'static str = "darwin";`) {
		t.Fatalf("external string constants should use a Rust const-compatible string type:\n%s", got)
	}
	if strings.Contains(got, `pub const G_O_O_S: String = "darwin";`) {
		t.Fatalf("external string constants must not emit non-const String values:\n%s", got)
	}
}

func TestRegisterExternalPackageSelectorPreservesConstValue(t *testing.T) {
	prevContext := currentContext
	prevTypeInfo := GetTypeInfo()
	prevImports := goPackageImports
	ctx := &TranspileContext{
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}
	SetTranspileContext(ctx)
	goPackageImports = map[string]string{"goarch": "internal/goarch"}
	defer func() {
		SetTranspileContext(prevContext)
		SetTypeInfo(prevTypeInfo)
		goPackageImports = prevImports
	}()

	pkg := gotypes.NewPackage("internal/goarch", "goarch")
	selIdent := goast.NewIdent("PtrSize")
	SetTypeInfo(&TypeInfo{
		info: &gotypes.Info{
			Uses: map[*goast.Ident]gotypes.Object{
				selIdent: gotypes.NewConst(gotoken.NoPos, pkg, "PtrSize", gotypes.Typ[gotypes.Int], goconstant.MakeInt64(8)),
			},
		},
	})

	RegisterExternalPackageSelector(&goast.SelectorExpr{X: goast.NewIdent("goarch"), Sel: selIdent})

	stub := ctx.File.ExternalPackageStubs["goarch"]
	if stub == nil {
		t.Fatalf("goarch selector should register an external package stub")
	}
	if got := stub.ConstantValues["PTR_SIZE"]; got == nil || got.String() != "8" {
		t.Fatalf("goarch PtrSize value was not preserved, got %#v", got)
	}
}

func TestRegisterExternalPackageSelectorPreservesStringConstType(t *testing.T) {
	prevContext := currentContext
	prevTypeInfo := GetTypeInfo()
	prevImports := goPackageImports
	ctx := &TranspileContext{
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}
	SetTranspileContext(ctx)
	goPackageImports = map[string]string{"goos": "internal/goos"}
	defer func() {
		SetTranspileContext(prevContext)
		SetTypeInfo(prevTypeInfo)
		goPackageImports = prevImports
	}()

	pkg := gotypes.NewPackage("internal/goos", "goos")
	selIdent := goast.NewIdent("GOOS")
	SetTypeInfo(&TypeInfo{
		info: &gotypes.Info{
			Uses: map[*goast.Ident]gotypes.Object{
				selIdent: gotypes.NewConst(gotoken.NoPos, pkg, "GOOS", gotypes.Typ[gotypes.String], goconstant.MakeString("darwin")),
			},
		},
	})

	RegisterExternalPackageSelector(&goast.SelectorExpr{X: goast.NewIdent("goos"), Sel: selIdent})

	stub := ctx.File.ExternalPackageStubs["goos"]
	if stub == nil {
		t.Fatalf("goos selector should register an external package stub")
	}
	if got := stub.Constants["G_O_O_S"]; got != "&'static str" {
		t.Fatalf("goos GOOS should register as a Rust const-compatible string, got %q", got)
	}
	if got := stub.ConstantValues["G_O_O_S"]; got == nil || goconstant.StringVal(got) != "darwin" {
		t.Fatalf("goos GOOS value was not preserved, got %#v", got)
	}
}

func TestPackageExternalStubsUseCanonicalMutexWrapperName(t *testing.T) {
	prevContext := currentContext
	prevCD := GetConcurrencyDetector()
	imports := NewImportTracker()
	imports.ReserveName("Mutex")
	ctx := &TranspileContext{
		Package:                 NewPackageState(),
		File:                    NewFileState(imports, &HelperTracker{}, nil),
		Imports:                 imports,
		UsePackageExternalStubs: true,
	}
	cd := NewConcurrencyDetector()
	cd.hasGoroutines = true
	SetConcurrencyDetector(cd)
	SetTranspileContext(ctx)
	defer func() {
		SetTranspileContext(prevContext)
		SetConcurrencyDetector(prevCD)
	}()

	got := wrappedExternalStubType("usize")
	if got != "Arc<Mutex<Option<usize>>>" {
		t.Fatalf("package external stubs should store canonical Mutex wrapper, got %q", got)
	}
	RegisterExternalTypeStubFieldByRustType("abi_Type", "equal", "Arc<StdMutex<Option<usize>>>")
	fieldType := ctx.Package.ExternalTypeStubFields["abi_Type"]["equal"]
	if fieldType != "Arc<Mutex<Option<usize>>>" {
		t.Fatalf("package external stub fields should canonicalize Mutex wrapper, got %q", fieldType)
	}
}

func TestExternalPackageTimeAfterFuncStubEmitsGoTimerHelper(t *testing.T) {
	got := generateExternalStubs(
		nil,
		nil,
		nil,
		nil,
		nil,
		nil,
		nil,
		map[string]*externalPackageStub{
			"time": {
				Functions: map[string]externalPackageStubFunction{
					"after_func": {
						ParamCount:  2,
						ReturnTypes: []string{"GoTimer"},
					},
				},
			},
		},
	)

	if !strings.Contains(got, "struct GoTimer") {
		t.Fatalf("time.AfterFunc stub returning GoTimer should emit the helper type:\n%s", got)
	}
	if !strings.Contains(got, "pub fn after_func") {
		t.Fatalf("time.AfterFunc package stub was not emitted:\n%s", got)
	}
}

func TestFilepathPureFunctionsStayOffExternalBridge(t *testing.T) {
	got := generateExternalStubs(
		nil,
		nil,
		nil,
		nil,
		nil,
		nil,
		nil,
		map[string]*externalPackageStub{
			"filepath": {
				Functions: map[string]externalPackageStubFunction{
					"abs": {
						ParamCount:  1,
						ReturnTypes: []string{wrappedExternalStubType("String"), wrappedExternalStubType("Box<dyn std::error::Error>")},
					},
					"base": {
						ParamCount:  1,
						ReturnTypes: []string{wrappedExternalStubType("String")},
					},
					"clean": {
						ParamCount:  1,
						ReturnTypes: []string{wrappedExternalStubType("String")},
					},
					"dir": {
						ParamCount:  1,
						ReturnTypes: []string{wrappedExternalStubType("String")},
					},
					"is_abs": {
						ParamCount:  1,
						ReturnTypes: []string{"bool"},
					},
					"join": {
						ParamCount:  1,
						ReturnTypes: []string{wrappedExternalStubType("String")},
					},
				},
			},
		},
	)

	if !strings.Contains(got, "pub fn abs") {
		t.Fatalf("OS-tied filepath.Abs bridge should remain available:\n%s", got)
	}
	for _, unwanted := range []string{
		"pub fn base",
		"pub fn clean",
		"pub fn dir",
		"pub fn is_abs",
		"pub fn join",
		"pub trait GoPathJoinArgs",
	} {
		if strings.Contains(got, unwanted) {
			t.Fatalf("pure path/filepath function %q should come from source-transpiled path/filepath, not the external bridge:\n%s", unwanted, got)
		}
	}
}

func TestJsonSupportHelpersDecodeUnsignedAndFixedArrays(t *testing.T) {
	var out strings.Builder
	writeJsonSupportHelpers(&out, false)
	got := out.String()

	for _, want := range []string{
		"impl GoJsonDecode for u16",
		"impl GoJsonDecode for usize",
		"impl<T, const N: usize> GoJsonDecode for [T; N]",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("JSON support helpers should include %q:\n%s", want, got)
		}
	}
}

func TestJsonDecoderMoreMatchesBareBoolSignature(t *testing.T) {
	var out strings.Builder
	writeJsonDecoderStub(&out)
	got := out.String()

	if strings.Contains(got, "pub fn more(&self) -> Arc<") ||
		strings.Contains(got, "Some::<bool>(has_more)") {
		t.Fatalf("json.Decoder.More shim should return a bare bool for the generated scalar ABI:\n%s", got)
	}
	if !strings.Contains(got, "pub fn more(&self) -> bool") ||
		!strings.Contains(got, "        has_more\n") {
		t.Fatalf("json.Decoder.More shim should expose the computed bool directly:\n%s", got)
	}
}

func TestSourceMappedStdlibCompositeLiteralDoesNotRegisterStubFields(t *testing.T) {
	prevContext := currentContext
	ctx := &TranspileContext{
		Package:        NewPackageState(),
		File:           NewFileState(NewImportTracker(), &HelperTracker{}, nil),
		PackageMapping: map[string]string{"go/types": "go_types"},
	}
	SetTranspileContext(ctx)
	defer SetTranspileContext(prevContext)

	named, structType := testExternalTypesBasicStruct()
	registerExternalStructCompositeLiteralFields(named, structType, []goast.Expr{
		&goast.KeyValueExpr{Key: goast.NewIdent("name"), Value: &goast.BasicLit{Kind: gotoken.STRING, Value: `"x"`}},
	})

	if len(ctx.File.ExternalTypeStubs) != 0 {
		t.Fatalf("source-mapped stdlib type should not register a stub: %#v", ctx.File.ExternalTypeStubs)
	}
	if len(ctx.File.ExternalTypeStubFields) != 0 {
		t.Fatalf("source-mapped stdlib type should not register stub fields: %#v", ctx.File.ExternalTypeStubFields)
	}
}

func TestUnmappedStdlibCompositeLiteralStillRegistersStubFields(t *testing.T) {
	prevContext := currentContext
	ctx := &TranspileContext{
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}
	SetTranspileContext(ctx)
	defer SetTranspileContext(prevContext)

	named, structType := testExternalTypesBasicStruct()
	registerExternalStructCompositeLiteralFields(named, structType, []goast.Expr{
		&goast.KeyValueExpr{Key: goast.NewIdent("name"), Value: &goast.BasicLit{Kind: gotoken.STRING, Value: `"x"`}},
	})

	if !ctx.File.ExternalTypeStubs["types_Basic"] {
		t.Fatalf("unmapped stdlib type should still register a stub: %#v", ctx.File.ExternalTypeStubs)
	}
	fields := ctx.File.ExternalTypeStubFields["types_Basic"]
	if fields == nil || fields["name"] == "" {
		t.Fatalf("unmapped stdlib type should register requested field: %#v", ctx.File.ExternalTypeStubFields)
	}
}

func testExternalTypesBasicStruct() (*gotypes.Named, *gotypes.Struct) {
	pkg := gotypes.NewPackage("go/types", "types")
	field := gotypes.NewField(gotoken.NoPos, pkg, "name", gotypes.Typ[gotypes.String], false)
	structType := gotypes.NewStruct([]*gotypes.Var{field}, nil)
	named := gotypes.NewNamed(gotypes.NewTypeName(gotoken.NoPos, pkg, "Basic", nil), structType, nil)
	return named, structType
}

// writeTypesBasicStub emits a struct whose field types are types_BasicKind and
// types_BasicInfo. Registering types_Basic without also registering those
// integer stubs leaves the generated Rust referring to undefined types.
func TestRegisterTypesBasicAlsoRegistersFieldTypes(t *testing.T) {
	prevContext := currentContext
	ctx := &TranspileContext{
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	}
	SetTranspileContext(ctx)
	defer SetTranspileContext(prevContext)

	RegisterExternalTypeStub("types_Basic")
	if !ctx.File.ExternalTypeStubs["types_BasicKind"] {
		t.Fatalf("registering types_Basic must also register types_BasicKind: %#v", ctx.File.ExternalTypeStubs)
	}
	if !ctx.File.ExternalTypeStubs["types_BasicInfo"] {
		t.Fatalf("registering types_Basic must also register types_BasicInfo: %#v", ctx.File.ExternalTypeStubs)
	}
	if ctx.File.ExternalTypeStubIntegerTypes["types_BasicKind"] != "i32" {
		t.Fatalf("types_BasicKind must register as i32 integer stub: %#v", ctx.File.ExternalTypeStubIntegerTypes)
	}
	if ctx.File.ExternalTypeStubIntegerTypes["types_BasicInfo"] != "i32" {
		t.Fatalf("types_BasicInfo must register as i32 integer stub: %#v", ctx.File.ExternalTypeStubIntegerTypes)
	}
}

// Registering types_Config.Check must not pull in the retired subprocess
// bridge. Source-transpiled go/types owns Config.Check behavior now.
func TestTypesConfigCheckStubIsRetired(t *testing.T) {
	noCheck := generateExternalStubs(
		map[string]bool{"types_Config": true},
		nil, nil, nil, nil,
		map[string]map[string]externalTypeStubMethod{},
		nil, nil,
	)
	if strings.Contains(noCheck, "GoTypesBridgeStringArg") {
		t.Fatalf("bridge support must not appear without types.Config.Check usage:\n%s", noCheck)
	}

	withCheck := generateExternalStubs(
		map[string]bool{"types_Config": true},
		nil, nil, nil, nil,
		map[string]map[string]externalTypeStubMethod{
			"types_Config": {"check": externalTypeStubMethod{}},
		},
		nil, nil,
	)
	for _, unwanted := range []string{
		"GoTypesBridgeStringArg",
		"__go_types_config_check",
		"pub fn check",
	} {
		if strings.Contains(withCheck, unwanted) {
			t.Fatalf("types.Config.Check external stub must be retired; found %q:\n%s", unwanted, withCheck)
		}
	}
}

func TestAtomicUint64StubSupportsStoreAndCompareAndSwap(t *testing.T) {
	got := generateExternalStubs(
		map[string]bool{"atomic_Uint64": true},
		nil, nil, nil, nil, nil, nil, nil,
	)

	for _, want := range []string{
		"pub fn store<T0: 'static>(&self, arg0: T0)",
		"pub fn compare_and_swap<T0: 'static, T1: 'static>(&self, old: T0, new: T1) -> bool",
		"self.__go_value.compare_exchange(old, new, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst).is_ok()",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("atomic.Uint64 helper should include %q:\n%s", want, got)
		}
	}
}

func TestParserStubUsesGoAstShapesForCalls(t *testing.T) {
	var out strings.Builder
	writeParserParseFileFunction(&out, externalPackageStubFunction{
		ReturnTypes: []string{"Arc<Mutex<Option<ast_File>>>", "Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>"},
	})
	got := out.String()
	if !strings.Contains(got, "ellipsis: call.dots.map(go_parser_pos).unwrap_or_else(go_parser_no_pos)") {
		t.Fatalf("parser stub should store absent call ellipsis as token.Pos zero:\n%s", got)
	}
	if !strings.Contains(got, "None if token == token::M_U_L => ast_Expr::__go_from_with_pos(ast_StarExpr") {
		t.Fatalf("parser stub should lower unary star to ast_StarExpr:\n%s", got)
	}
	if !strings.Contains(got, "gosyn::ast::Statement::Empty(_) => ast_Stmt::__go_from(ast_EmptyStmt)") {
		t.Fatalf("parser stub should preserve empty statements as ast.EmptyStmt:\n%s", got)
	}
}

func TestParserStubPreservesFileAndNodeMetadataForTypeInfoBridge(t *testing.T) {
	var out strings.Builder
	writeParserParseFileFunction(&out, externalPackageStubFunction{
		ReturnTypes: []string{"Arc<Mutex<Option<ast_File>>>", "Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>"},
	})
	got := out.String()
	for _, want := range []string{
		"__go_filename: go_parser_some(filename.to_string())",
		"__go_source: go_parser_some(source.to_string())",
		"pos as i32 + 1",
		"assign: if spec.alias { go_parser_pos(1) } else { go_parser_no_pos() }",
		"ast_Ident { __go_pos: go_parser_pos_value(id.pos)",
		"ast_Expr::__go_from_with_pos(go_parser_ident_struct(id), go_parser_pos_value(pos))",
		"ast_Expr::__go_from_with_pos(ast_BasicLit",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("parser stub should preserve metadata %q for the type-info bridge:\n%s", want, got)
		}
	}
}

func TestRuntimeGOMAXPROCSStubMatchesBareReturnSignature(t *testing.T) {
	var out strings.Builder
	writeRuntimeGOMAXPROCSStub(&out, externalPackageStubFunction{
		ParamCount:  1,
		ReturnTypes: []string{"i32"},
	})
	got := out.String()
	if strings.Contains(got, "Some::<i32>") || strings.Contains(got, "RefCell::new") || strings.Contains(got, "Mutex::new") {
		t.Fatalf("runtime.GOMAXPROCS stub should return bare i32 for bare signatures:\n%s", got)
	}
	if !strings.Contains(got, "pub fn g_o_m_a_x_p_r_o_c_s<T0>(_arg0: T0) -> i32") ||
		!strings.Contains(got, "std::thread::available_parallelism()") {
		t.Fatalf("runtime.GOMAXPROCS stub should preserve the runtime fallback body:\n%s", got)
	}
}

func TestRuntimeGOROOTStubUsesHostGoRoot(t *testing.T) {
	var out strings.Builder
	writeRuntimeGOROOTStub(&out, externalPackageStubFunction{
		ReturnTypes: []string{wrappedExternalStubType("String")},
	})
	got := out.String()
	if strings.Contains(got, "generic stub function body has no implementation") {
		t.Fatalf("runtime.GOROOT should not use the generic stub panic:\n%s", got)
	}
	for _, want := range []string{
		"pub fn g_o_r_o_o_t() -> " + wrappedExternalStubType("String"),
		"std::sync::OnceLock<String>",
		"std::env::var(\"GOROOT\")",
		"std::process::Command::new(\"go\")",
		".args([\"env\", \"GOROOT\"])",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("missing %q in:\n%s", want, got)
		}
	}
}

func TestFsDirEntryStubImplementsTypeMethod(t *testing.T) {
	var out strings.Builder
	writeFsDirEntryStub(&out, "fs_DirEntry", map[string]externalTypeStubMethod{
		"r#type": {ReturnTypes: []string{wrappedExternalStubType("fs_FileMode")}},
	})
	got := out.String()
	for _, want := range []string{
		"pub fn r#type(&self) -> " + wrappedExternalStubType("fs_FileMode"),
		"fs_FileMode(1u32 << 31)",
		"fs_FileMode(0)",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("fs.DirEntry Type shim should include %q:\n%s", want, got)
		}
	}
}

func TestFsFileInfoStubImplementsModeMethod(t *testing.T) {
	var out strings.Builder
	writeFsFileInfoStub(&out, "fs_FileInfo", map[string]externalTypeStubMethod{
		"mode": {ReturnTypes: []string{wrappedExternalStubType("fs_FileMode")}},
	})
	got := out.String()
	if strings.Contains(got, "generic stub method body has no implementation") {
		t.Fatalf("fs.FileInfo Mode shim should not use the generic method panic:\n%s", got)
	}
	for _, want := range []string{
		"pub fn mode(&self) -> " + wrappedExternalStubType("fs_FileMode"),
		"fs_FileMode(1u32 << 31)",
		"fs_FileMode(0)",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("fs.FileInfo Mode shim should include %q:\n%s", want, got)
		}
	}
}

func TestFsFileModeStubImplementsIsDirMethod(t *testing.T) {
	got := generateExternalStubs(
		map[string]bool{"fs_FileMode": true},
		nil,
		map[string]string{"fs_FileMode": "u32"},
		nil,
		nil,
		map[string]map[string]externalTypeStubMethod{
			"fs_FileMode": {
				"is_dir": {ReturnTypes: []string{"bool"}},
			},
		},
		nil,
		nil,
	)
	if strings.Contains(got, "generic stub method body has no implementation") {
		t.Fatalf("fs.FileMode IsDir shim should not use the generic method panic:\n%s", got)
	}
	for _, want := range []string{
		"pub fn is_dir(&self) -> bool",
		"(self.0 & (1u32 << 31)) != 0",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("fs.FileMode IsDir shim should include %q:\n%s", want, got)
		}
	}
}

func TestExecCmdStubImplementsEnvironMethod(t *testing.T) {
	var out strings.Builder
	writeExecCmdTypeStub(&out, nil, nil)
	got := out.String()
	for _, want := range []string{
		"pub fn environ(&self) -> ",
		"std::env::vars().map(|(__k, __v)| format!(\"{}={}\", __k, __v)).collect()",
		"env.extend(cmd_env.iter().cloned())",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("exec.Cmd Environ shim should include %q:\n%s", want, got)
		}
	}
}

func TestExecCmdStubImplementsStderrPipeMethod(t *testing.T) {
	var out strings.Builder
	writeExecCmdTypeStub(&out, nil, map[string]externalTypeStubMethod{
		"stderr_pipe": {
			ReturnTypes: []string{
				wrappedExternalStubType("io_ReadCloser"),
				wrappedExternalStubType(externalStubErrorInnerType()),
			},
		},
	})
	got := out.String()
	for _, want := range []string{
		"pub fn stderr_pipe(&mut self) -> ",
		"let file = os_File::default();",
		"io_ReadCloser::__go_from(file)",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("exec.Cmd StderrPipe shim should include %q:\n%s", want, got)
		}
	}

	got = generateExternalStubs(
		map[string]bool{"exec_Cmd": true},
		nil, nil, nil, nil,
		map[string]map[string]externalTypeStubMethod{
			"exec_Cmd": {
				"stderr_pipe": {
					ReturnTypes: []string{
						wrappedExternalStubType("io_ReadCloser"),
						wrappedExternalStubType(externalStubErrorInnerType()),
					},
				},
			},
		},
		nil, nil,
	)
	if !strings.Contains(got, "pub struct os_File") {
		t.Fatalf("exec.Cmd StderrPipe shim should emit its os.File backing type:\n%s", got)
	}
}

func TestOsFileStubImplementsReadAtMethod(t *testing.T) {
	var out strings.Builder
	writeOsFileStub(&out)
	got := out.String()
	for _, want := range []string{
		"pub fn read_at<T0: 'static, T1: 'static>",
		"let offset =",
		"target[..count].copy_from_slice(&data[start..start + count]);",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("os.File ReadAt shim should include %q:\n%s", want, got)
		}
	}
}

func TestOsGetenvStubUsesHostEnv(t *testing.T) {
	var out strings.Builder
	writeOsPackageStub(&out, &externalPackageStub{
		Functions: map[string]externalPackageStubFunction{
			"getenv": {
				ParamCount:  1,
				ReturnTypes: []string{wrappedExternalStubType("String")},
			},
		},
	}, nil)
	got := out.String()
	if strings.Contains(got, "generic stub function body has no implementation") {
		t.Fatalf("os.Getenv should not use the generic stub panic:\n%s", got)
	}
	for _, want := range []string{
		"std::env::var(key).unwrap_or_default()",
		"downcast_ref::<" + wrappedExternalStubType("String") + ">",
		externalStubBorrowExpr("v") + ".as_ref().cloned().unwrap_or_default()",
		"os.Getenv bridge: expected string argument",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("os.Getenv shim should include %q:\n%s", want, got)
		}
	}
}

func TestOsGetwdStubUsesCurrentDir(t *testing.T) {
	var out strings.Builder
	writeOsPackageStub(&out, &externalPackageStub{
		Functions: map[string]externalPackageStubFunction{
			"getwd": {
				ReturnTypes: []string{
					wrappedExternalStubType("String"),
					wrappedExternalStubType(externalStubErrorInnerType()),
				},
			},
		},
	}, nil)
	got := out.String()
	if strings.Contains(got, "generic stub function body has no implementation") {
		t.Fatalf("os.Getwd should not use the generic stub panic:\n%s", got)
	}
	for _, want := range []string{
		"std::env::current_dir()",
		"path.to_string_lossy().into_owned()",
		"String::new()",
		"io_error(err)",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("os.Getwd shim should include %q:\n%s", want, got)
		}
	}
}

func TestOsIsPathSeparatorStubUsesHostPathRules(t *testing.T) {
	var out strings.Builder
	writeOsPackageStub(&out, &externalPackageStub{
		Functions: map[string]externalPackageStubFunction{
			"is_path_separator": {
				ParamCount:  1,
				ReturnTypes: []string{"bool"},
			},
		},
	}, nil)
	got := out.String()
	if strings.Contains(got, "generic stub function body has no implementation") {
		t.Fatalf("os.IsPathSeparator should not use the generic stub panic:\n%s", got)
	}
	for _, want := range []string{
		"downcast_ref::<u8>()",
		"downcast_ref::<" + wrappedExternalStubType("u8") + ">",
		"#[cfg(windows)]",
		"c == b'/'",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("os.IsPathSeparator shim should include %q:\n%s", want, got)
		}
	}
}

func TestOsLstatStubUsesSymlinkMetadata(t *testing.T) {
	var out strings.Builder
	writeOsPackageStub(&out, &externalPackageStub{
		Functions: map[string]externalPackageStubFunction{
			"lstat": {
				ParamCount: 1,
				ReturnTypes: []string{
					wrappedExternalStubType("fs_FileInfo"),
					wrappedExternalStubType(externalStubErrorInnerType()),
				},
			},
		},
	}, nil)
	got := out.String()
	if strings.Contains(got, "generic stub function body has no implementation") {
		t.Fatalf("os.Lstat should not use the generic stub panic:\n%s", got)
	}
	for _, want := range []string{
		"std::fs::symlink_metadata(&path)",
		"fs_FileInfo { name, is_dir: metadata.is_dir(), size: metadata.len() as i64 }",
		"fs_FileInfo::default()",
		"io_error(err)",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("os.Lstat shim should include %q:\n%s", want, got)
		}
	}
}

func TestOsOpenStubReadsFileBytes(t *testing.T) {
	var out strings.Builder
	writeOsPackageStub(&out, &externalPackageStub{
		Functions: map[string]externalPackageStubFunction{
			"open": {
				ParamCount: 1,
				ReturnTypes: []string{
					wrappedExternalStubType("os_File"),
					wrappedExternalStubType(externalStubErrorInnerType()),
				},
			},
		},
	}, nil)
	got := out.String()
	if strings.Contains(got, "generic stub function body has no implementation") {
		t.Fatalf("os.Open should not use the generic stub panic:\n%s", got)
	}
	for _, want := range []string{
		"std::fs::read(&path)",
		"os_File { __go_data: std::sync::Arc::new(std::sync::Mutex::new(data))",
		"None::<os_File>",
		"io_error(err)",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("os.Open shim should include %q:\n%s", want, got)
		}
	}
}

func TestIoReadCloserCloseDispatchesToOsFile(t *testing.T) {
	var out strings.Builder
	writeExternalInterfaceStub(&out, "io_ReadCloser", map[string]externalTypeStubMethod{
		"close": {
			ReturnTypes: []string{
				wrappedExternalStubType(externalStubErrorInnerType()),
			},
		},
	})
	got := out.String()
	if strings.Contains(got, "generic stub method body has no implementation") {
		t.Fatalf("io.ReadCloser Close should not use the generic stub panic:\n%s", got)
	}
	for _, want := range []string{
		"if let Some(file) = self.downcast_ref::<os_File>()",
		"return file.close();",
		"unsupported concrete receiver",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("io.ReadCloser Close shim should include %q:\n%s", want, got)
		}
	}
}

func TestIoCopyStubMatchesBareCountReturnSignature(t *testing.T) {
	var out strings.Builder
	writeIoCopyStub(&out, externalPackageStubFunction{
		ParamCount: 2,
		ReturnTypes: []string{
			"i64",
			"Rc<RefCell<Option<Box<dyn StdError>>>>",
		},
	}, nil)
	got := out.String()
	if strings.Contains(got, "Some::<i64>") {
		t.Fatalf("io.Copy stub should not wrap a bare byte-count return:\n%s", got)
	}
	if !strings.Contains(got, "(data.len() as i64, Rc::new(RefCell::new(None::<Box<dyn StdError>>)))") {
		t.Fatalf("io.Copy stub should return a bare byte count and wrapped error:\n%s", got)
	}
}

func TestAstInterfacesCarrySourcePositions(t *testing.T) {
	var out strings.Builder
	writeExternalInterfaceStub(&out, "ast_Expr", map[string]externalTypeStubMethod{
		"pos": {ReturnTypes: []string{"Arc<Mutex<Option<token_Pos>>>"}},
	})
	got := out.String()
	for _, want := range []string{
		"pub __go_pos: i32",
		"pub fn __go_from_with_pos",
		"__go_pos: pos",
		"Arc::new(Mutex::new(Some(token_Pos(self.__go_pos))))",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("ast interface stub should carry source position %q:\n%s", want, got)
		}
	}
}

func TestAstInspectStubTraversesSyntaxTree(t *testing.T) {
	var out strings.Builder
	writeAstInspectFunction(&out)
	got := out.String()
	if !strings.Contains(got, "pub fn inspect<T0: InspectRoot>") {
		t.Fatalf("ast inspect stub should expose the generated inspect function:\n%s", got)
	}
	if !strings.Contains(got, "impl InspectRoot for ast_Expr") {
		t.Fatalf("ast inspect stub should accept bare ast.Expr values:\n%s", got)
	}
	if !strings.Contains(got, "visit_decl_list(callback, &value.decls);") {
		t.Fatalf("ast inspect stub should walk file declarations:\n%s", got)
	}
	if !strings.Contains(got, "visit_opt_expr(callback, &value.x);") {
		t.Fatalf("ast inspect stub should walk expression children:\n%s", got)
	}
	if !strings.Contains(got, "visit_opt_block(callback, &value.body);") {
		t.Fatalf("ast inspect stub should walk statement bodies:\n%s", got)
	}
}

func TestTokenPosIsValidStubIsRetired(t *testing.T) {
	got := generateExternalStubs(
		map[string]bool{"token_Pos": true},
		nil, nil, nil, nil,
		map[string]map[string]externalTypeStubMethod{
			"token_Pos": {
				"is_valid": {
					ReturnTypes: []string{"Arc<Mutex<Option<bool>>>"},
				},
			},
		},
		nil, nil,
	)
	for _, unwanted := range []string{
		"pub fn is_valid",
		"Some(self.0 != 0)",
	} {
		if strings.Contains(got, unwanted) {
			t.Fatalf("token.Pos.IsValid external stub must be retired; found %q:\n%s", unwanted, got)
		}
	}
}

func TestTypesTermBridgeIsRetired(t *testing.T) {
	got := generateExternalStubs(
		map[string]bool{"types_Term": true},
		nil, nil, nil, nil,
		map[string]map[string]externalTypeStubMethod{
			"types_Term": {
				"r#type": {
					ReturnTypes: []string{"Arc<Mutex<Option<types_Type>>>"},
				},
				"tilde": {
					ReturnTypes: []string{"Arc<Mutex<Option<bool>>>"},
				},
			},
		},
		nil,
		map[string]*externalPackageStub{
			"types": {
				Functions: map[string]externalPackageStubFunction{
					"new_term": {
						ParamCount:  2,
						ReturnTypes: []string{"Arc<Mutex<Option<types_Term>>>"},
					},
				},
			},
		},
	)
	for _, unwanted := range []string{
		"types_Term",
		"GoTypesTypeArg",
		"__go_into_types_type_arg",
		"pub fn new_term",
	} {
		if strings.Contains(got, unwanted) {
			t.Fatalf("go/types.Term external bridge must be retired; found %q:\n%s", unwanted, got)
		}
	}
}

func TestTypesCheckerFilesStubIsRetired(t *testing.T) {
	got := generateExternalStubs(
		map[string]bool{"types_Checker": true},
		nil, nil, nil, nil,
		map[string]map[string]externalTypeStubMethod{
			"types_Checker": {
				"files": {
					ParamCount:  1,
					ReturnTypes: []string{"Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>"},
				},
			},
		},
		nil, nil,
	)
	for _, unwanted := range []string{"pub fn files", "__go_types_checker_files"} {
		if strings.Contains(got, unwanted) {
			t.Fatalf("types.Checker Files should use source-transpiled go/types, not an external stub method; found %q:\n%s", unwanted, got)
		}
	}
}

// Bridge methods for go/types must panic loudly on unsupported kinds rather
// than returning Default/empty values. Returning soft defaults here is
// structurally the same bug as the 2026 syntax-fallback incident: callers
// receive plausible-but-wrong type information and the failure shows up as a
// downstream code-gen mystery instead of an obvious crash at the boundary.
// See AGENTS.md "Strategy: Transpile stdlib, don't bridge it".
func TestTypesTypeBridgeMethodsPanicOnUnsupportedKinds(t *testing.T) {
	var out strings.Builder
	writeTypesTypeStringMethod(&out)
	got := out.String()
	if strings.Contains(got, "String::new()") {
		t.Fatalf("types.Type.String() bridge must not return an empty String for unsupported kinds:\n%s", got)
	}
	if !strings.Contains(got, "panic!(") {
		t.Fatalf("types.Type.String() bridge must panic on unsupported kinds:\n%s", got)
	}
	if !strings.Contains(got, "transpile go/types") {
		t.Fatalf("types.Type.String() panic should point at the transpile-not-bridge strategy:\n%s", got)
	}

	out.Reset()
	writeTypesTypeUnderlyingMethod(&out)
	got = out.String()
	if strings.Contains(got, "types_Type::default()") {
		t.Fatalf("types.Type.Underlying() bridge must not return a default Type for unsupported kinds:\n%s", got)
	}
	if !strings.Contains(got, "panic!(") {
		t.Fatalf("types.Type.Underlying() bridge must panic on unsupported kinds:\n%s", got)
	}
}

// TestBridgeDebtRegistryCoversAllShims is the tripwire that enforces
// AGENTS.md → "Strategy: Transpile stdlib, don't bridge it". Every
// `// TEMPORARY:` comment in external_type_stubs.go must have a matching
// row in docs/bridge_debt.md. Adding a shim without a row, or removing a
// row without removing the shim, fails this test.
//
// To make this test pass when you legitimately need to add a shim, edit
// docs/bridge_debt.md *first* and add a row that names the Go symbol, the
// transpiler gap, and a fixture. The shim code goes in after. See AGENTS.md
// for the full checklist.
func TestBridgeDebtRegistryCoversAllShims(t *testing.T) {
	stubsPath := "external_type_stubs.go"
	registryPath := filepath.Join("..", "docs", "bridge_debt.md")

	stubsBytes, err := os.ReadFile(stubsPath)
	if err != nil {
		t.Fatalf("read %s: %v", stubsPath, err)
	}
	registryBytes, err := os.ReadFile(registryPath)
	if err != nil {
		t.Fatalf("read %s: %v", registryPath, err)
	}

	shimLines := []int{}
	for i, line := range strings.Split(string(stubsBytes), "\n") {
		if strings.HasPrefix(strings.TrimSpace(line), "// TEMPORARY:") {
			shimLines = append(shimLines, i+1)
		}
	}

	registry := string(registryBytes)
	shimsHeader := "## Shims"
	idx := strings.Index(registry, shimsHeader)
	if idx < 0 {
		t.Fatalf("%s is missing the %q section that holds shim rows", registryPath, shimsHeader)
	}
	registryRowCount := 0
	for _, line := range strings.Split(registry[idx+len(shimsHeader):], "\n") {
		if strings.HasPrefix(line, "### ") {
			registryRowCount++
		}
	}

	if len(shimLines) != registryRowCount {
		t.Fatalf(`bridge debt drift detected.

%s has %d `+"`// TEMPORARY:`"+` shim comments (lines: %v).
%s has %d level-3 rows under %q.

Every shim must have a registry row. See AGENTS.md → "Strategy: Transpile
stdlib, don't bridge it". To resolve:

- If you added a shim: add a row to %s first, then re-run this test.
- If you deleted a shim: also delete its row.
- If you moved a shim: update its row's Location field.
`,
			stubsPath, len(shimLines), shimLines,
			registryPath, registryRowCount, shimsHeader,
			registryPath)
	}
}
