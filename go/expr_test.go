package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestUnknownPositionalStructLiteralFallbackParses(t *testing.T) {
	var out strings.Builder
	prevStructDefs := structDefs
	structDefs = make(map[string]*StructDef)
	defer func() {
		structDefs = prevStructDefs
	}()

	TranspileExpression(&out, &ast.CompositeLit{
		Type: ast.NewIdent("External"),
		Elts: []ast.Expr{ast.NewIdent("value")},
	})

	got := out.String()
	if strings.Contains(got, "*/, ..Default::default()") {
		t.Fatalf("fallback comment must not create an empty struct field before default:\n%s", got)
	}
	if !strings.Contains(got, "External { /* ERROR: Type information required for positional struct literal */ ..Default::default() }") {
		t.Fatalf("unexpected fallback for unknown positional struct literal:\n%s", got)
	}
}

func TestSelectorStructCompositeLiteralUsesTypeInfo(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/types"

func main() {
	_ = &types.Info{}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	var composite *ast.CompositeLit
	ast.Inspect(file, func(n ast.Node) bool {
		if lit, ok := n.(*ast.CompositeLit); ok {
			if _, ok := lit.Type.(*ast.SelectorExpr); ok {
				composite = lit
				return false
			}
		}
		return true
	})
	if composite == nil {
		t.Fatal("did not find selector-qualified composite literal")
	}

	var out strings.Builder
	TranspileExpression(&out, composite)

	got := out.String()
	if !strings.Contains(got, "types_Info {") {
		t.Fatalf("selector-qualified struct literal should use package-qualified Rust type:\n%s", got)
	}
	if strings.Contains(got, "Some()") || strings.Contains(got, "(*.borrow") {
		t.Fatalf("selector-qualified struct literal emitted missing expression:\n%s", got)
	}
}

func TestLocalInterfaceReferenceCallArgumentUsesCurrentReceiver(t *testing.T) {
	prevReceiver := currentReceiver
	currentReceiver = "k"
	defer func() { currentReceiver = prevReceiver }()

	var out strings.Builder
	if !writeLocalInterfaceReferenceCallArgument(&out, ast.NewIdent("k"), nil) {
		t.Fatal("writeLocalInterfaceReferenceCallArgument returned false")
	}
	if got, want := out.String(), "self"; got != want {
		t.Fatalf("receiver argument = %q, want %q", got, want)
	}
}

func TestIsFunctionNameUsesRegisteredSignatureWithoutTypeInfo(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevContext := currentContext
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		SetTranspileContext(prevContext)
		SetVarTable(prevVarTable)
	}()

	SetTranspileContext(&TranspileContext{
		Session: NewTranspileSession(nil, nil),
		Package: NewPackageState(),
		File:    NewFileState(NewImportTracker(), &HelperTracker{}, nil),
	})
	SetTypeInfo(nil)
	RegisterFunctionSignature("hasName", &FunctionSignature{})

	if !isFunctionName(ast.NewIdent("hasName")) {
		t.Fatal("registered package function should be recognized without go/types")
	}

	vt := NewVarTable()
	vt.Register("hasName", &VarInfo{WrapLevel: WrapFull, Source: SourceLocal})
	SetVarTable(vt)
	if isFunctionName(ast.NewIdent("hasName")) {
		t.Fatal("local variable should shadow registered package function")
	}
}

func TestFunctionBoxTypeUsesVarTableWithoutTypeInfo(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		SetVarTable(prevVarTable)
	}()

	SetTypeInfo(nil)
	vt := NewVarTable()
	vt.Register("processData", &VarInfo{
		WrapLevel: WrapFull,
		RustType:  "Box<dyn FnMut(Rc<RefCell<Option<Vec<i32>>>>) -> Rc<RefCell<Option<Box<dyn StdError>>>>>",
		Source:    SourceLocal,
	})
	SetVarTable(vt)

	got := functionBoxTypeForCallTarget(ast.NewIdent("processData"))
	if !strings.HasPrefix(got, "Box<dyn FnMut(") {
		t.Fatalf("function box type = %q", got)
	}
}

func TestReferenceRangeComparisonDereferencesWithoutTypeInfo(t *testing.T) {
	expr, err := parser.ParseExpr("num > 6")
	if err != nil {
		t.Fatalf("ParseExpr() error = %v", err)
	}

	prevTypeInfo := currentTypeInfo
	prevRangeLoopVars := rangeLoopVars
	defer func() {
		currentTypeInfo = prevTypeInfo
		rangeLoopVars = prevRangeLoopVars
	}()
	SetTypeInfo(nil)
	rangeLoopVars = map[string]string{"num": "ref_value"}

	var out strings.Builder
	TranspileExpression(&out, expr)

	got := out.String()
	if !strings.Contains(got, "(*num).clone() > 6") {
		t.Fatalf("reference range comparison should own the range value, got:\n%s", got)
	}
	if strings.Contains(got, "num > 6") {
		t.Fatalf("reference range comparison used borrowed range value:\n%s", got)
	}
}

func TestElidedNestedSliceLiteralUsesOuterSyntaxWithoutTypeInfo(t *testing.T) {
	expr, err := parser.ParseExpr(`[][]string{{"a", "b"}, {}}`)
	if err != nil {
		t.Fatalf("ParseExpr() error = %v", err)
	}

	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	var out strings.Builder
	TranspileExpression(&out, expr)

	got := out.String()
	if strings.Contains(got, "CompositeLit with nil Type") || strings.Contains(got, "unimplemented!()") {
		t.Fatalf("elided nested slice literal should use outer syntax type, got:\n%s", got)
	}
	if !strings.Contains(got, `vec!["a".to_string(), "b".to_string()]`) {
		t.Fatalf("elided nested string slice literal did not emit owned strings:\n%s", got)
	}
	if !strings.Contains(got, "Vec::<String>::new()") {
		t.Fatalf("empty elided nested string slice literal needs explicit Vec type:\n%s", got)
	}
}

func TestTrackedRangeSlicePrintArgWithoutTypeInfo(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevRangeLoopVars := rangeLoopVars
	prevRangeElemTypes := localRangeElemRustTypes
	defer func() {
		currentTypeInfo = prevTypeInfo
		rangeLoopVars = prevRangeLoopVars
		localRangeElemRustTypes = prevRangeElemTypes
	}()
	SetTypeInfo(nil)
	localRangeElemRustTypes = make(map[string]string)

	expr, err := parser.ParseExpr(`[][]int{{1, 2}}`)
	if err != nil {
		t.Fatalf("ParseExpr() error = %v", err)
	}
	registerCompositeLiteralRangeElemType(ast.NewIdent("testData"), expr.(*ast.CompositeLit))
	elemRustType, ok := trackedRangeElemRustType(ast.NewIdent("testData"))
	if !ok || elemRustType != "Vec<i32>" {
		t.Fatalf("tracked range elem type = %q, %v; want Vec<i32>, true", elemRustType, ok)
	}

	rangeLoopVars = map[string]string{"data": rangeValueTypeFromTrackedRustElem(elemRustType)}
	var out strings.Builder
	transpilePrintArg(&out, ast.NewIdent("data"))

	got := out.String()
	if got != "format_slice_values(data)" {
		t.Fatalf("tracked range slice print arg = %q", got)
	}
}

func TestCapturedReferenceRangeValueUsesCapturedClone(t *testing.T) {
	prevRangeLoopVars := rangeLoopVars
	prevCaptureRenames := currentCaptureRenames
	defer func() {
		rangeLoopVars = prevRangeLoopVars
		currentCaptureRenames = prevCaptureRenames
	}()

	rangeLoopVars = map[string]string{"chunk": "&Vec<String>"}
	currentCaptureRenames = map[string]string{"chunk": "chunk_closure_clone"}

	var out strings.Builder
	if !writeOwnedRangeValue(&out, ast.NewIdent("chunk")) {
		t.Fatalf("writeOwnedRangeValue returned false")
	}
	if got := out.String(); got != "chunk_closure_clone.clone()" {
		t.Fatalf("captured range clone = %q", got)
	}
}

func TestTrackedRangeElemFallbackFillsGenericValueType(t *testing.T) {
	prevRangeElemTypes := localRangeElemRustTypes
	defer func() {
		localRangeElemRustTypes = prevRangeElemTypes
	}()
	localRangeElemRustTypes = map[string]string{"testData": "Vec<i32>"}

	valueType, needsCopied, ok := trackedRangeElemValueType(ast.NewIdent("testData"))
	if !ok {
		t.Fatalf("trackedRangeElemValueType ok = false")
	}
	if valueType != "&Vec<i32>" {
		t.Fatalf("valueType = %q, want &Vec<i32>", valueType)
	}
	if needsCopied {
		t.Fatalf("needsCopied = true, want false for Vec element")
	}
}

func TestExternalStubCallClonesMapRangeStringKey(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "path/filepath"

func f(overlay map[string]string) {
	overlays := make(map[string]string)
	for k, v := range overlay {
		_ = filepath.Base(k)
		overlays[k] = v
	}
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, "filepath::base(k.clone())") {
		t.Fatalf("external stub call should clone map range key before later reuse:\n%s", rust)
	}
	if strings.Contains(rust, "filepath::base(k)") {
		t.Fatalf("external stub call moved map range key:\n%s", rust)
	}
}

func TestStdlibInterfaceSelectorFieldArgumentUsesFieldHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/ast"

func accept(n ast.Node) {}

func visit(kv *ast.KeyValueExpr) {
	accept(kv.Value)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if !strings.Contains(rust, "let __arg = { let __field = (*kv.borrow().as_ref().unwrap()).value.clone(); __field }; let __converted") {
		t.Fatalf("stdlib interface selector field argument did not clone the field handle:\n%s", rust)
	}
}

func TestLocalVariableShadowsImportedPackageSelector(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/ast"

func packageName(file *ast.File) string {
	ast := file
	return ast.Name.Name
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "ast::name") {
		t.Fatalf("local variable named ast should not be emitted as package selector:\n%s", rust)
	}
	if !strings.Contains(rust, ".name") {
		t.Fatalf("selector chain should still access the Name fields:\n%s", rust)
	}
}
