package main

import (
	goast "go/ast"
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

// Registering types_Config alone (e.g. for a struct literal) must not pull in
// writeTypesBridgeSupport, which references ast_Expr, ast_File, types_Info,
// and other types that are only wired through the Check() method path.
func TestTypesBridgeSupportGatedOnConfigCheck(t *testing.T) {
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
	if !strings.Contains(withCheck, "GoTypesBridgeStringArg") {
		t.Fatalf("bridge support should appear when types.Config.Check is registered:\n%s", withCheck)
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

func TestTokenPosIsValidStubUsesPositionValue(t *testing.T) {
	var out strings.Builder
	writeTokenPosIsValidMethod(&out)
	got := out.String()
	if !strings.Contains(got, "Some(self.0 != 0)") {
		t.Fatalf("token.Pos IsValid stub should reflect the stored position:\n%s", got)
	}
}

func TestTypesStubsDoNotSilentlySynthesizeTypeInfo(t *testing.T) {
	var out strings.Builder
	writeTypesConfigCheckMethod(&out, externalTypeStubMethod{
		ParamCount: 4,
		ReturnTypes: []string{
			"Arc<Mutex<Option<types_Package>>>",
			"Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>",
		},
	})
	got := out.String()
	if !strings.Contains(got, "pub fn check<T0: GoTypesBridgeStringArg, T1, T3: GoTypesBridgeInfoArg>") {
		t.Fatalf("types.Config Check stub should use the generated type bridge signature:\n%s", got)
	}
	if !strings.Contains(got, "__go_types_config_check(_arg0, _arg2, _arg3)") {
		t.Fatalf("types.Config Check stub should call the go/types bridge:\n%s", got)
	}
	if strings.Contains(got, "go/types Config.Check is required for TypeInfo") {
		t.Fatalf("types.Config Check stub should no longer panic at the bridge boundary:\n%s", got)
	}

	out.Reset()
	writeTypesCheckerFilesMethod(&out, externalTypeStubMethod{
		ParamCount:  1,
		ReturnTypes: []string{"Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>"},
	})
	got = out.String()
	if !strings.Contains(got, "go/types Checker.Files is required for TypeInfo") {
		t.Fatalf("types.Checker Files stub should fail at the missing type checker boundary:\n%s", got)
	}
	if strings.Contains(got, "Default::default()") {
		t.Fatalf("types.Checker Files stub must not return default type info:\n%s", got)
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

func TestTypesConfigCheckBridgeRunsGoTypes(t *testing.T) {
	var out strings.Builder
	writeTypesBridgeSupport(&out)
	got := out.String()
	for _, want := range []string{
		"go/types",
		"impl GoTypesBridgeInfoArg for ()",
		"config.Check(req.Path, fset, files, info)",
		"types.Unalias(tv.Type).Underlying().(*types.Basic)",
		"types_map.insert(expr.clone()",
	} {
		if !strings.Contains(got, want) {
			t.Fatalf("types.Config Check bridge should contain %q:\n%s", want, got)
		}
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
