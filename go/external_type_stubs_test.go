package main

import (
	"strings"
	"testing"
)

func TestParserStubUsesGoAstShapesForCalls(t *testing.T) {
	var out strings.Builder
	writeParserParseFileFunction(&out, externalPackageStubFunction{
		ReturnTypes: []string{"Arc<Mutex<Option<ast_File>>>", "Arc<Mutex<Option<Box<dyn std::error::Error + Send + Sync>>>>"},
	})
	got := out.String()
	if !strings.Contains(got, "ellipsis: call.dots.map(go_parser_pos).unwrap_or_else(|| go_parser_pos(0))") {
		t.Fatalf("parser stub should store absent call ellipsis as token.Pos zero:\n%s", got)
	}
	if !strings.Contains(got, "None if token == token::M_U_L => ast_Expr::__go_from(ast_StarExpr") {
		t.Fatalf("parser stub should lower unary star to ast_StarExpr:\n%s", got)
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
