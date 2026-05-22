package main

import (
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
// See docs/rules/self-host-rules.md "Strategy: Transpile stdlib, don't bridge it".
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
