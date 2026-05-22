package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
	"os"
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

func TestCapturedReceiverSelectorAssignmentUsesCloneName(t *testing.T) {
	prevReceiver := currentReceiver
	prevReceiverType := currentReceiverType
	prevRenames := currentCaptureRenames
	currentReceiver = "analysis"
	currentReceiverType = "transpileFileAnalysis"
	currentCaptureRenames = map[string]string{"analysis": "analysis_closure_clone"}
	defer func() {
		currentReceiver = prevReceiver
		currentReceiverType = prevReceiverType
		currentCaptureRenames = prevRenames
	}()

	var out strings.Builder
	writePointerHandleAssignmentTarget(&out, &ast.SelectorExpr{
		X:   ast.NewIdent("analysis"),
		Sel: ast.NewIdent("typeAssertExprs"),
	})

	if got, want := out.String(), "analysis_closure_clone.type_assert_exprs"; got != want {
		t.Fatalf("captured receiver selector target = %q, want %q", got, want)
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

func TestShortDeclSelfShadowingFunctionCallUsesTypeInfo(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func hash(key int) uint32 { return uint32(key) }

func lookup(key int, table map[uint32]int) int {
	hash := hash(key)
	return table[hash]
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

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "let __f_guard = hash.") {
		t.Fatalf("short declaration RHS should call the function, not the new local:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut hash = hash(") {
		t.Fatalf("short declaration RHS should emit a direct function call:\n%s", rust)
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

func TestNoTypeInfoContextCancelFuncCallUsesTupleResultType(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import "context"

func main() {
	_, cancel := context.WithCancel(context.Background())
	cancel()
}
`)

	if strings.Contains(rust, "*mut _") {
		t.Fatalf("context cancel call should use concrete tuple result function type:\n%s", rust)
	}
	if !strings.Contains(rust, "*mut GoCancelFunc") {
		t.Fatalf("context cancel call should use GoCancelFunc:\n%s", rust)
	}
}

func TestNoTypeInfoVariadicSliceArgsUseElementValue(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

func collect(groups ...[]string) []string {
	var out []string
	for _, group := range groups {
		for _, value := range group {
			out = append(out, value)
		}
	}
	return out
}

func main() {
	var missing []string
	_ = collect([]string{"go"}, missing, []string{"rust"})
}
`)

	if strings.Contains(rust, "vec![Rc::new(RefCell::new(Some(vec!") {
		t.Fatalf("variadic []string elements should be packed as raw Vec values:\n%s", rust)
	}
	if strings.Contains(rust, "(*missing.borrow().as_ref().unwrap())") {
		t.Fatalf("nil slice variadic argument should use clone-or-empty path:\n%s", rust)
	}
	if strings.Contains(rust, "type info required for range statement") {
		t.Fatalf("range over variadic parameter should use syntax-derived slice type:\n%s", rust)
	}
	if !strings.Contains(rust, "__slice_holder = missing.clone()") || !strings.Contains(rust, "unwrap_or_default()") {
		t.Fatalf("slice variadic argument should clone the inner slice or use empty default:\n%s", rust)
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

func TestNoTypeInfoPrintTrackedLocalSlice(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevCollections := localCollectionKinds
	defer func() {
		currentTypeInfo = prevTypeInfo
		localCollectionKinds = prevCollections
	}()
	SetTypeInfo(nil)
	localCollectionKinds = map[string]string{"nums": "slice"}

	var out strings.Builder
	transpilePrintArg(&out, ast.NewIdent("nums"))

	if got := out.String(); got != "format_slice(&nums)" {
		t.Fatalf("tracked local slice print arg = %q", got)
	}
}

func TestNoTypeInfoPrintfSelectorSliceFieldUsesSyntax(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import "fmt"

type Manager struct {
	Team []string
}

func (m Manager) Manage() {
	fmt.Printf("team: %v\n", m.Team)
}`)

	if strings.Contains(rust, "Type information not available for print argument") {
		t.Fatalf("selector slice print arg should use syntax fallback:\n%s", rust)
	}
	if !strings.Contains(rust, "format_slice(&self.team)") {
		t.Fatalf("selector slice print arg should format the field handle:\n%s", rust)
	}
}

func TestNoTypeInfoErrorFieldKeepsHandleAndErrorMethodUsesSyntax(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import (
	"errors"
	"fmt"
)

type holder struct {
	err error
}

func main() {
	h := holder{err: errors.New("boom")}
	fmt.Println(h.err.Error())
}`)

	if !strings.Contains(rust, "Self { err: self.err.clone() }") {
		t.Fatalf("error field value clone should preserve the error handle:\n%s", rust)
	}
	if strings.Contains(rust, ".error().borrow()") {
		t.Fatalf("error field Error method should not call a nonexistent boxed-error method:\n%s", rust)
	}
	if !strings.Contains(rust, "format!(\"{}\", (*h.borrow().as_ref().unwrap()).err.borrow().as_ref().unwrap())") {
		t.Fatalf("error field Error method should format the error handle:\n%s", rust)
	}
}

func TestNoTypeInfoTupleAssignToErrorSliceKeepsHandle(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import (
	"errors"
	"fmt"
)

func parse() (int, error) {
	return 7, errors.New("bad")
}

func main() {
	values := make([]int, 1)
	errs := make([]error, 1)
	values[0], errs[0] = parse()
	fmt.Println(values[0])
	if errs[0] != nil {
		fmt.Println(errs[0].Error())
	}
}`)

	if strings.Contains(rust, "__tmp_1.borrow_mut().take().unwrap_or_default()") {
		t.Fatalf("tuple assignment into []error should not move the boxed payload out of the error handle:\n%s", rust)
	}
	if !strings.Contains(rust, "(*errs.borrow_mut().as_mut().unwrap())[(0) as usize] = __tmp_1;") {
		t.Fatalf("tuple assignment into []error should store the returned error handle:\n%s", rust)
	}
	if strings.Contains(rust, ".error().borrow()") {
		t.Fatalf("error slice element Error method should not call a nonexistent boxed-error method:\n%s", rust)
	}
	if !strings.Contains(rust, "format!(\"{}\", (*errs.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap())") {
		t.Fatalf("error slice element Error method should format the element handle:\n%s", rust)
	}
}

func TestConcurrentTupleReturnAvoidsRelockingLocalResult(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "fmt"

var elems map[string]string

func elem(name string) (string, bool) {
	elemType := elems[name]
	if elemType == "" {
		return "", false
	}
	return elemType, true
}

func forceConcurrent() {
	go func() {}()
	fmt.Println(elem("x"))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	prevDetector := GetConcurrencyDetector()
	detector := NewConcurrencyDetector()
	detector.AnalyzeFile(file)
	SetConcurrencyDetector(detector)
	defer SetConcurrencyDetector(prevDetector)

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, "let __tmp_x = (*elemType.lock().unwrap().as_ref().unwrap()).clone()") {
		t.Fatalf("tuple return should not relock elemType in a later tuple element:\n%s", rust)
	}
	if !strings.Contains(rust, "Arc::new(Mutex::new(Some(true)))") {
		t.Fatalf("tuple return should use a literal true result after the empty check:\n%s", rust)
	}
}

func TestNoTypeInfoExternalExecLookPathRegistersStub(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import (
	"fmt"
	"os/exec"
)

func main() {
	_, err := exec.LookPath("__go2rust_missing_executable__")
	fmt.Println(err != nil)
}`)

	if !strings.Contains(rust, "pub mod exec") || !strings.Contains(rust, "pub fn look_path") {
		t.Fatalf("exec.LookPath should register an inline external stdlib stub without type info:\n%s", rust)
	}
	if !strings.Contains(rust, "exec::look_path") {
		t.Fatalf("exec.LookPath call should target the generated exec stub:\n%s", rust)
	}
}

func TestNoTypeInfoExternalStdlibVariadicRegistersStubs(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import (
	"crypto/md5"
	"fmt"
	"io"
)

func main() {
	io.MultiWriter(io.Discard, md5.New())
	fmt.Println("ok")
}`)

	for _, want := range []string{"pub mod io", "pub fn Discard()", "pub fn multi_writer", "pub mod md5", "pub fn new()"} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in external stdlib fallback output:\n%s", want, rust)
		}
	}
	if !strings.Contains(rust, "fn __go_next_external_interface_id()") {
		t.Fatalf("external stdlib interface fallback should emit the interface id helper:\n%s", rust)
	}
	if !strings.Contains(rust, "pub struct hash_Hash {\n    pub __go_id: usize,") {
		t.Fatalf("md5.New fallback should register hash.Hash as an interface stub:\n%s", rust)
	}
	if strings.Contains(rust, "io::discard") {
		t.Fatalf("io.Discard should call the generated package variable accessor:\n%s", rust)
	}
	if !strings.Contains(rust, "io::Discard()") {
		t.Fatalf("io.Discard should use the generated package variable accessor:\n%s", rust)
	}
}

func TestNoTypeInfoBytesNewBufferStdlibInterfaceField(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import (
	"bytes"
	"fmt"
	"io"
)

type holder struct {
	w io.Writer
}

func main() {
	h := holder{w: bytes.NewBuffer(nil)}
	_ = fmt.Errorf("%v", h.w)
}`)

	for _, want := range []string{
		"fn __go_next_external_interface_id()",
		"pub struct bytes_Buffer",
		"pub mod bytes",
		"pub fn new_buffer",
		"impl From<bytes_Buffer> for io_Writer",
		"let __arg = bytes::new_buffer(",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in bytes.NewBuffer interface-field fallback output:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "w: bytes::new_buffer(") {
		t.Fatalf("bytes.NewBuffer assigned to io.Writer field should be converted, not assigned directly:\n%s", rust)
	}
	if strings.Contains(rust, "Type information not available for print argument") {
		t.Fatalf("fmt.Errorf should format the syntax-known io.Writer field:\n%s", rust)
	}
}

func TestNoTypeInfoJsonMarshalCompositeMapFieldUsesStructSyntax(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import "encoding/json"

type OverlayJSON struct {
	Replace map[string]string `+"`json:\"replace,omitempty\"`"+`
}

func main() {
	overlays := map[string]string{"b.go": "tmp-b", "a.go": "tmp-a"}
	_, _ = json.Marshal(OverlayJSON{Replace: overlays})
	_, _ = json.Marshal(OverlayJSON{})
}`)

	for _, bad := range []string{
		"Type information required for json.Marshal",
		"json.Marshal currently supports struct values",
		"unimplemented!()",
	} {
		if strings.Contains(rust, bad) {
			t.Fatalf("json.Marshal should use struct syntax fallback without %q:\n%s", bad, rust)
		}
	}
	for _, want := range []string{
		"let __json_value = OverlayJSON { replace: overlays.clone(), ..Default::default() }",
		"let __json_value = OverlayJSON { replace: Rc::new(RefCell::new(Some(BTreeMap::new()))) }",
		"let __map_guard = __json_value.replace.borrow()",
		"go_json_escape(__k)",
		`format!("\"replace\":{{{}}}", __map_entries)`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in json.Marshal syntax fallback output:\n%s", want, rust)
		}
	}
}

func TestNoTypeInfoJsonMarshalTrackedStructLocalUsesStructSyntax(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import "encoding/json"

type User struct {
	Name string `+"`json:\"name\"`"+`
	Age  int    `+"`json:\"age\"`"+`
}

func main() {
	u := User{Name: "Alice", Age: 30}
	_, _ = json.Marshal(u)
}`)

	for _, bad := range []string{
		"Type information required for json.Marshal",
		"json.Marshal currently supports struct values",
		"unimplemented!()",
	} {
		if strings.Contains(rust, bad) {
			t.Fatalf("json.Marshal tracked struct local should use syntax fallback without %q:\n%s", bad, rust)
		}
	}
	for _, want := range []string{
		"let __json_value = (*u.borrow().as_ref().unwrap()).clone()",
		`format!("\"name\":\"{}\"", go_json_escape(`,
		`format!("\"age\":{}", *`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in json.Marshal tracked local output:\n%s", want, rust)
		}
	}
}

func TestNoTypeInfoJsonMarshalTupleResultStringConversionUsesByteSlice(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import (
	"encoding/json"
	"fmt"
)

type User struct {
	Name string `+"`json:\"name\"`"+`
}

func main() {
	data, _ := json.Marshal(User{Name: "Alice"})
	fmt.Println(string(data))
}`)

	if strings.Contains(rust, "(*data.borrow().as_ref().unwrap()).to_string()") {
		t.Fatalf("string(data) from json.Marshal should not call ToString on Vec<u8>:\n%s", rust)
	}
	if !strings.Contains(rust, "String::from_utf8((*data.borrow().as_ref().unwrap()).clone()).unwrap()") {
		t.Fatalf("string(data) from json.Marshal should use byte-slice string conversion:\n%s", rust)
	}
}

func TestJsonMarshalStructDefFallbackUsesStoredSyntaxFields(t *testing.T) {
	def := &StructDef{
		FieldTypes: map[string]ast.Expr{
			"Name":    ast.NewIdent("string"),
			"Age":     ast.NewIdent("int"),
			"Replace": &ast.MapType{Key: ast.NewIdent("string"), Value: ast.NewIdent("string")},
		},
		FieldTags: map[string]string{
			"Name":    `json:"name"`,
			"Age":     `json:"age"`,
			"Replace": `json:"replace,omitempty"`,
		},
		FieldOrder: []string{"Name", "Age", "Replace"},
	}

	fields, ok := jsonMarshalStructFieldsFromStructDef(def)
	if !ok {
		t.Fatal("jsonMarshalStructFieldsFromStructDef returned false")
	}
	if len(fields) != 3 {
		t.Fatalf("field count = %d, want 3: %#v", len(fields), fields)
	}
	if fields[0].jsonName != "name" || fields[0].kind != jsonMarshalBasicField || fields[0].basicKind != types.String || fields[0].basicName != "string" {
		t.Fatalf("first field = %#v, want string name field", fields[0])
	}
	if fields[1].jsonName != "age" || fields[1].kind != jsonMarshalBasicField || fields[1].basicName != "int" {
		t.Fatalf("second field = %#v, want int age field", fields[1])
	}
	if fields[2].jsonName != "replace" || fields[2].kind != jsonMarshalStringMapField || !fields[2].omitEmpty {
		t.Fatalf("third field = %#v, want omitempty string map field", fields[2])
	}
}

func TestNoTypeInfoStringSliceBoundsUseStringOutput(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

func trimParens(s string) string {
	return s[len("(") : len(s)-len(")")]
}`)

	if !strings.Contains(rust, "].to_string()") {
		t.Fatalf("string slice should produce a String under syntax fallback:\n%s", rust)
	}
	if strings.Contains(rust, "].to_vec()") {
		t.Fatalf("string slice should not use Vec output under syntax fallback:\n%s", rust)
	}
}

func TestNoTypeInfoMakeMapWithCapacityTracksMapSyntax(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import "fmt"

func main() {
	counts := make(map[string]int, 4)
	counts["go"]++
	counts["rust"] += 2
	fmt.Println(counts["go"], counts["rust"], len(counts))

	seen := make(map[int]bool, 3)
	seen[10] = true
	fmt.Println(seen[10], len(seen))
}`)

	for _, bad := range []string{
		"Cannot determine if map",
		"type info required for index expression",
		`as usize`,
	} {
		if strings.Contains(rust, bad) {
			t.Fatalf("make(map..., cap) should track map syntax without %q:\n%s", bad, rust)
		}
	}
	for _, want := range []string{
		`BTreeMap::<String, Rc<RefCell<Option<i32>>>>::new()`,
		`.entry("go".to_string())`,
		`.entry("rust".to_string())`,
		`.get(&"go".to_string())`,
		`BTreeMap::<i32, Rc<RefCell<Option<bool>>>>::new()`,
		`.insert(__map_key, __map_value)`,
		`.get(&10)`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in make-map syntax fallback output:\n%s", want, rust)
		}
	}
}

func TestNoTypeInfoFixedArrayLocalIndexUsesSyntaxTracking(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

import "fmt"

func main() {
	var buf [128]byte
	fmt.Println(len(buf), buf[0])
}`)

	for _, bad := range []string{
		"Cannot determine if map or slice access",
		"type info required for index expression",
	} {
		if strings.Contains(rust, bad) {
			t.Fatalf("fixed array local should track indexability without %q:\n%s", bad, rust)
		}
	}
	if !strings.Contains(rust, "[(0) as usize].clone()") {
		t.Fatalf("fixed array local index should use direct sequence indexing:\n%s", rust)
	}
}

func TestNoTypeInfoLocalCollectionTrackingIsFunctionScoped(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "fmt"

func makeNums() []int {
	result := make([]int, 0)
	return result
}

func main() {
	result := func() int { return 1 }()
	fmt.Println(result)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, "format_slice(&result)") {
		t.Fatalf("slice tracking for makeNums.result leaked into main.result:\n%s", rust)
	}
	if !strings.Contains(rust, "Immediate") && !strings.Contains(rust, "(*result.borrow().as_ref().unwrap())") {
		t.Fatalf("main.result should print as a scalar wrapped value:\n%s", rust)
	}
}

func TestNoTypeInfoStringParamRangeUsesSyntaxType(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func upper(s string) string {
	result := ""
	for _, char := range s {
		result += string(char)
	}
	return result
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, "type info required for range statement") {
		t.Fatalf("string parameter range should use syntax-derived parameter type:\n%s", rust)
	}
	if !strings.Contains(rust, ".char_indices()") {
		t.Fatalf("string parameter range should iterate chars:\n%s", rust)
	}
	if !strings.Contains(rust, "to_string()") {
		t.Fatalf("string(char) over a range rune should use the bare char value:\n%s", rust)
	}
	if strings.Contains(rust, "guard.as_ref().unwrap() +") {
		t.Fatalf("string += should not use numeric compound assignment:\n%s", rust)
	}
}

func TestPartialTypeInfoStringConversionCompoundAssignUsesSyntax(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func upper(s string) string {
	result := ""
	for _, char := range s {
		result += string(char - 32)
	}
	return result
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, &TypeInfo{})
	if !strings.Contains(rust, "push_str") {
		t.Fatalf("partial type info should still use string append syntax fallback:\n%s", rust)
	}
	if strings.Contains(rust, "guard.as_ref().unwrap() +") {
		t.Fatalf("partial type info should not force numeric compound assignment:\n%s", rust)
	}
	if !strings.Contains(rust, "char as i32") {
		t.Fatalf("string(char - n) should cast the range char before arithmetic:\n%s", rust)
	}
}

func TestNoTypeInfoStringConcatUsesSyntaxStringOperand(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func join(prefix string, s string) string {
	return prefix + s
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if !strings.Contains(rust, "format!(\"{}{}\"") {
		t.Fatalf("string parameter concatenation should use syntax-derived string types:\n%s", rust)
	}
	if strings.Contains(rust, "__tmp_x + __tmp_y") {
		t.Fatalf("string parameter concatenation should not lower as numeric addition:\n%s", rust)
	}
}

func TestNoTypeInfoRuneSliceConversionTracksResult(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func reverse(s string) string {
	runes := []rune(s)
	return string(runes)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, "type info required") {
		t.Fatalf("[]rune/string conversion should not require go/types:\n%s", rust)
	}
	if !strings.Contains(rust, ".chars().map(|c| c as i32).collect::<Vec<_>>()") {
		t.Fatalf("[]rune(s) should lower through chars:\n%s", rust)
	}
	if !strings.Contains(rust, ".iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>()") {
		t.Fatalf("string(runes) should use the tracked rune slice element type:\n%s", rust)
	}
}

func TestNoTypeInfoFunctionFieldSyntaxValueAndCall(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "fmt"

type BinaryOp func(int, int) int

type Calculator struct {
	Multiply BinaryOp
}

func multiply(a, b int) int { return a * b }

func main() {
	calc := Calculator{Multiply: multiply}
	fmt.Println(calc.Multiply(3, 4))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, "multiply: multiply.clone()") {
		t.Fatalf("function field value should be boxed from the registered function signature:\n%s", rust)
	}
	if strings.Contains(rust, ".multiply(Rc::new") {
		t.Fatalf("function field call should not be lowered as a method call:\n%s", rust)
	}
	if !strings.Contains(rust, "let __f_holder = (*calc.borrow().as_ref().unwrap()).multiply.clone()") {
		t.Fatalf("function field call should invoke the field handle:\n%s", rust)
	}
}

func TestNoTypeInfoFunctionFieldCallInPrintfUsesSyntax(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "fmt"

type BinaryOp func(int, int) int

type Calculator struct {
	Add BinaryOp
}

func main() {
	calc := Calculator{Add: func(a, b int) int { return a + b }}
	fmt.Printf("%d\n", calc.Add(1, 2))
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, ".add(Rc::new") {
		t.Fatalf("Printf function field call should not be lowered as a method call:\n%s", rust)
	}
	if !strings.Contains(rust, "let __f_holder = (*calc.borrow().as_ref().unwrap()).add.clone()") {
		t.Fatalf("Printf function field call should invoke the field handle:\n%s", rust)
	}
}

func TestNoTypeInfoImmediateFuncLitCallUsesClosureType(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func main() {
	result := func(a, b int) int { return a + b }(10, 20)
	_ = result
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, "*mut _") {
		t.Fatalf("immediate function literal call should use a concrete closure type:\n%s", rust)
	}
	if !strings.Contains(rust, "*mut Box<dyn FnMut") {
		t.Fatalf("immediate function literal call should emit a concrete function box type:\n%s", rust)
	}
}

func TestFindStructFieldExprUsesRustCasedFallback(t *testing.T) {
	structType := &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
		{
			Names: []*ast.Ident{ast.NewIdent("Add")},
			Type:  ast.NewIdent("BinaryOp"),
		},
	}}}

	if got := findStructFieldExpr(structType, "add"); got == nil {
		t.Fatal("expected Rust-cased selector name to resolve to Add field")
	}
}

func TestFunctionValueSelectorSyntaxUsesUniqueStructFieldFallback(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevVarTable := currentVarTable
	prevRenames := currentCaptureRenames
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetVarTable(prevVarTable)
		currentCaptureRenames = prevRenames
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	currentCaptureRenames = nil
	functionTypeAliases = make(map[string]bool)
	functionTypeAliasBoxTypes = map[string]string{
		"BinaryOp": "Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>",
	}
	structDefs = map[string]*StructDef{
		"Calculator": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
				{
					Names: []*ast.Ident{ast.NewIdent("Add")},
					Type:  ast.NewIdent("BinaryOp"),
				},
			}}},
		},
	}

	call := &ast.CallExpr{
		Fun: &ast.SelectorExpr{
			X:   ast.NewIdent("calc"),
			Sel: ast.NewIdent("add"),
		},
		Args: []ast.Expr{
			&ast.BasicLit{Kind: token.INT, Value: "1"},
			&ast.BasicLit{Kind: token.INT, Value: "2"},
		},
	}

	var out strings.Builder
	TranspileExpression(&out, call)
	got := out.String()
	if strings.Contains(got, ".add(Rc::new") {
		t.Fatalf("function field selector fallback should not emit a method call:\n%s", got)
	}
	if !strings.Contains(got, "let __f_holder = (*calc.borrow().as_ref().unwrap()).add.clone()") {
		t.Fatalf("function field selector fallback should invoke the field handle:\n%s", got)
	}
}

func TestFunctionValueSelectorSyntaxDoesNotFallbackForClosureCloneMethod(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevVarTable := currentVarTable
	prevRenames := currentCaptureRenames
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetVarTable(prevVarTable)
		currentCaptureRenames = prevRenames
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	currentCaptureRenames = map[string]string{"ld": "ld_closure_clone"}
	functionTypeAliases = make(map[string]bool)
	functionTypeAliasBoxTypes = make(map[string]string)
	RegisterFunctionTypeAlias("ParseFunc")
	RegisterFunctionTypeAliasBox("ParseFunc", "Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<i32>>>>")
	structDefs = map[string]*StructDef{
		"Config": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
				{
					Names: []*ast.Ident{ast.NewIdent("ParseFile")},
					Type:  ast.NewIdent("ParseFunc"),
				},
			}}},
		},
	}

	call := &ast.CallExpr{
		Fun: &ast.SelectorExpr{
			X:   ast.NewIdent("ld"),
			Sel: ast.NewIdent("parse_file"),
		},
		Args: []ast.Expr{&ast.BasicLit{Kind: token.STRING, Value: `"x.go"`}},
	}

	var out strings.Builder
	TranspileExpression(&out, call)
	got := out.String()
	if strings.Contains(got, "let __f_holder = ld_closure_clone.parse_file.clone()") {
		t.Fatalf("closure clone method call should not use unrelated function-field fallback:\n%s", got)
	}
	if strings.Contains(got, "let __f_holder =") {
		t.Fatalf("captured receiver method call should not use function-field fallback:\n%s", got)
	}
}

func TestFunctionValueSelectorSyntaxDoesNotOverrideTypedMethod(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevInterfaces := interfaceTypes
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		interfaceTypes = prevInterfaces
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetVarTable(prevVarTable)
	}()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Key interface {
	Name() string
}

type Label struct {
	key Key
}

func (t Label) Key() Key { return t.key }

func use(t Label) {
	_ = t.Key()
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

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "let __f_holder = t.key.clone()") || strings.Contains(rust, "*mut Key") {
		t.Fatalf("typed method call should not use function-field syntax fallback:\n%s", rust)
	}
}

func TestFunctionValueSelectorSyntaxDoesNotUseUnrelatedFieldForKnownReceiver(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetVarTable(prevVarTable)
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	GetVarTable().Register("r", &VarInfo{
		WrapLevel: WrapFull,
		RustType:  "Decoder",
		Source:    SourceLocal,
	})
	functionTypeAliases = map[string]bool{"Hook": true}
	functionTypeAliasBoxTypes = map[string]string{
		"Hook": "Box<dyn FnMut(Rc<RefCell<Option<i32>>>)>",
	}
	structDefs = map[string]*StructDef{
		"PkgDecoder": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
				{
					Names: []*ast.Ident{ast.NewIdent("sync")},
					Type:  ast.NewIdent("Hook"),
				},
			}}},
		},
		"Decoder": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{}},
		},
	}

	sel := &ast.SelectorExpr{
		X:   ast.NewIdent("r"),
		Sel: ast.NewIdent("Sync"),
	}
	if isFunctionValueSelectorSyntax(sel) {
		t.Fatal("known Decoder receiver must not use an unrelated PkgDecoder.sync function field")
	}
}

func TestSelectorFieldTypeExprUsesRegisteredFieldTypeMap(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		SetVarTable(prevVarTable)
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	GetVarTable().Register("calc", &VarInfo{
		WrapLevel: WrapFull,
		RustType:  "Calculator",
		Source:    SourceLocal,
	})
	structDefs = map[string]*StructDef{
		"Calculator": {
			Fields: map[string]string{"Add": "regular"},
			FieldTypes: map[string]ast.Expr{
				"Add": ast.NewIdent("BinaryOp"),
			},
			ASTType: &ast.StructType{Fields: &ast.FieldList{}},
		},
	}

	fieldExpr, ok := selectorFieldTypeExpr(&ast.SelectorExpr{
		X:   ast.NewIdent("calc"),
		Sel: ast.NewIdent("add"),
	})
	if !ok {
		t.Fatal("selector field type should use registered FieldTypes when AST lookup has no names")
	}
	ident, ok := fieldExpr.(*ast.Ident)
	if !ok || ident.Name != "BinaryOp" {
		t.Fatalf("field type = %#v, want BinaryOp ident", fieldExpr)
	}
}

func TestFunctionValueSelectorSyntaxAllowsPartialTypeInfoFallback(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
	}()

	sel := &ast.SelectorExpr{
		X:   ast.NewIdent("calc"),
		Sel: ast.NewIdent("add"),
	}
	SetTypeInfo(&TypeInfo{info: &types.Info{
		Uses: map[*ast.Ident]types.Object{
			sel.Sel: types.NewFunc(token.NoPos, nil, "Add", nil),
		},
	}})
	functionTypeAliases = make(map[string]bool)
	functionTypeAliasBoxTypes = map[string]string{
		"BinaryOp": "Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>",
	}
	structDefs = map[string]*StructDef{
		"Calculator": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
				{
					Names: []*ast.Ident{ast.NewIdent("Add")},
					Type:  ast.NewIdent("BinaryOp"),
				},
			}}},
		},
	}

	if !isFunctionValueSelectorSyntax(sel) {
		t.Fatal("partial selector object info without selector type should not block syntax fallback")
	}
}

func TestFunctionValueSelectorSyntaxUsesUniqueFieldWhenTypedObjectIsMisclassified(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetVarTable(prevVarTable)
	}()

	sel := &ast.SelectorExpr{
		X:   ast.NewIdent("calc"),
		Sel: ast.NewIdent("add"),
	}
	SetTypeInfo(&TypeInfo{info: &types.Info{
		Types: map[ast.Expr]types.TypeAndValue{
			sel: {Type: types.Typ[types.Int]},
		},
		Uses: map[*ast.Ident]types.Object{
			sel.Sel: types.NewFunc(token.NoPos, nil, "Add", nil),
		},
	}})
	SetVarTable(NewVarTable())
	functionTypeAliases = map[string]bool{"BinaryOp": true}
	functionTypeAliasBoxTypes = map[string]string{
		"BinaryOp": "Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>",
	}
	structDefs = map[string]*StructDef{
		"Calculator": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
				{
					Names: []*ast.Ident{ast.NewIdent("Add")},
					Type:  ast.NewIdent("BinaryOp"),
				},
			}}},
		},
	}

	if !isFunctionValueSelectorSyntax(sel) {
		t.Fatal("syntax-proven function field should survive a misclassified selector object")
	}
}

func TestFunctionFieldCallUsesBoxWhenFuncAliasIsAlsoTypeDefinition(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevTypeDefs := typeDefinitions
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		typeDefinitions = prevTypeDefs
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetVarTable(prevVarTable)
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	typeDefinitions = map[string]string{"BinaryOp": "func"}
	functionTypeAliases = make(map[string]bool)
	functionTypeAliasBoxTypes = map[string]string{
		"BinaryOp": "Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>",
	}
	structDefs = map[string]*StructDef{
		"Calculator": {
			ASTType: &ast.StructType{Fields: &ast.FieldList{List: []*ast.Field{
				{
					Names: []*ast.Ident{ast.NewIdent("Add")},
					Type:  ast.NewIdent("BinaryOp"),
				},
			}}},
		},
	}
	GetVarTable().Register("calc", &VarInfo{
		WrapLevel: WrapFull,
		RustType:  "Calculator",
		Source:    SourceLocal,
	})

	call := &ast.CallExpr{
		Fun: &ast.SelectorExpr{
			X:   ast.NewIdent("calc"),
			Sel: ast.NewIdent("add"),
		},
		Args: []ast.Expr{
			&ast.BasicLit{Kind: token.INT, Value: "1"},
			&ast.BasicLit{Kind: token.INT, Value: "2"},
		},
	}

	var out strings.Builder
	TranspileExpression(&out, call)
	got := out.String()
	if strings.Contains(got, ".add(Rc::new") {
		t.Fatalf("function field call should not be lowered as a method call:\n%s", got)
	}
	if !strings.Contains(got, "let __f_holder = (*calc.borrow().as_ref().unwrap()).add.clone()") {
		t.Fatalf("function field call should invoke the field handle:\n%s", got)
	}
	if !strings.Contains(got, "*mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>") {
		t.Fatalf("function field call should use the stored function box type:\n%s", got)
	}
}

func TestFunctionMapValueUsesSyntaxAliasWithoutTypeInfo(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevAliases := functionTypeAliases
	prevSignatures := functionSignatures
	defer func() {
		currentTypeInfo = prevTypeInfo
		functionTypeAliases = prevAliases
		functionSignatures = prevSignatures
	}()

	SetTypeInfo(nil)
	functionTypeAliases = map[string]bool{"handler": true}
	functionSignatures = map[string]*FunctionSignature{
		"inc": {
			Params: []*ast.Field{
				{Type: ast.NewIdent("int")},
			},
			Results: []*ast.Field{
				{Type: ast.NewIdent("int")},
			},
		},
	}

	var out strings.Builder
	writeWrappedMapValue(&out, ast.NewIdent("inc"), ast.NewIdent("handler"), nil)

	got := out.String()
	if !strings.Contains(got, "Box::new(move |__arg0: Rc<RefCell<Option<i32>>>| { inc(__arg0) })") ||
		!strings.Contains(got, "as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>") {
		t.Fatalf("function map value should box named function from syntax alias:\n%s", got)
	}
	if strings.Contains(got, "inc.borrow") || strings.Contains(got, "Some(inc)") {
		t.Fatalf("function map value used generic wrapped expression path:\n%s", got)
	}
}

func TestNoTypeInfoMethodFunctionParameterPassesHandle(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type recorder struct{}

func (recorder) Use(record func(string)) {}

func relay(record func(string)) {
	var r recorder
	r.Use(record)
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if !strings.Contains(rust, ".r#use(record.clone())") {
		t.Fatalf("method function parameter should pass the existing handle:\n%s", rust)
	}
	if strings.Contains(rust, "record.borrow().as_ref().unwrap()") || strings.Contains(rust, "Rc::new(RefCell::new(Some((*record") {
		t.Fatalf("method function parameter used generic wrapping path:\n%s", rust)
	}
}

func TestTypedConstMethodInterfaceArgumentUsesNamedValueFromTypeInfo(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Code interface {
	Value() int
}

type CodeVal int

func (c CodeVal) Value() int {
	return int(c)
}

const (
	ValBool CodeVal = iota
	ValString
)

type Writer struct{}

func (Writer) Code(c Code) int {
	return c.Value()
}

func main() {
	var w Writer
	w.Code(ValBool)
	w.Code(ValString)
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

	if strings.Contains(rust, ".code(Rc::new(RefCell::new(Some(VAL_BOOL))))") ||
		strings.Contains(rust, ".code(Rc::new(RefCell::new(Some(VAL_STRING))))") {
		t.Fatalf("typed constants should not be passed as wrapped raw ints to interface params:\n%s", rust)
	}
	if !strings.Contains(rust, ".code(&CodeVal(Rc::new(RefCell::new(Some(VAL_BOOL as i32)))))") {
		t.Fatalf("typed constant should be constructed as its named value for interface params:\n%s", rust)
	}
	if !strings.Contains(rust, ".code(&CodeVal(Rc::new(RefCell::new(Some(VAL_STRING as i32)))))") {
		t.Fatalf("implicit typed constant should reuse the previous named type for interface params:\n%s", rust)
	}
}

func TestNoTypeInfoTrackedSliceIndexDoesNotUseStringPath(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "fmt"

func main() {
	values := []string{"alpha"}
	fmt.Println(values[0])
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if strings.Contains(rust, ".as_bytes()[") {
		t.Fatalf("tracked slice index should not use string indexing path:\n%s", rust)
	}
	if !strings.Contains(rust, "values.borrow().as_ref().unwrap())[") {
		t.Fatalf("tracked slice index should use slice indexing path:\n%s", rust)
	}
}

func TestNoTypeInfoRangeUsesTrackedMap(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevCollections := localCollectionKinds
	prevMapKeys := localMapKeyRustTypes
	prevMapValues := localMapValueRustTypes
	prevRangeVars := rangeLoopVars
	defer func() {
		currentTypeInfo = prevTypeInfo
		localCollectionKinds = prevCollections
		localMapKeyRustTypes = prevMapKeys
		localMapValueRustTypes = prevMapValues
		rangeLoopVars = prevRangeVars
	}()
	SetTypeInfo(nil)
	localCollectionKinds = map[string]string{"ages": "map"}
	localMapKeyRustTypes = map[string]string{"ages": "String"}
	localMapValueRustTypes = map[string]string{"ages": "Rc<RefCell<Option<i32>>>"}
	rangeLoopVars = make(map[string]string)

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
func f() {
	for _, age := range ages { fmt.Println(age) }
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	fn := file.Decls[0].(*ast.FuncDecl)
	stmt := fn.Body.List[0]
	var out strings.Builder
	TranspileStatementSimple(&out, stmt, nil, token.NewFileSet())

	got := out.String()
	if !strings.Contains(got, "for (_, age) in { let __range_holder = ages.clone();") {
		t.Fatalf("tracked map range did not use map iteration:\n%s", got)
	}
	if strings.Contains(got, "type info required") || strings.Contains(got, "0..") {
		t.Fatalf("tracked map range fell back to non-map lowering:\n%s", got)
	}
}

func TestNoTypeInfoSliceParamRangeUsesCopiedElement(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	defer func() { currentTypeInfo = prevTypeInfo }()
	SetTypeInfo(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func keep(numbers []int, pred func(int) bool) []int {
	var result []int
	for _, num := range numbers {
		if pred(num) {
			result = append(result, num)
		}
	}
	return result
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, nil)
	if !strings.Contains(rust, "for num in __range_values.iter().copied()") {
		t.Fatalf("[]int parameter range should copy scalar elements:\n%s", rust)
	}
	if !strings.Contains(rust, "push(num)") {
		t.Fatalf("append should store the copied scalar range element:\n%s", rust)
	}
}

func TestNoTypeInfoRangeUsesTrackedChannelParam(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevCollections := localCollectionKinds
	prevRangeElemTypes := localRangeElemRustTypes
	prevRangeVars := rangeLoopVars
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		localCollectionKinds = prevCollections
		localRangeElemRustTypes = prevRangeElemTypes
		rangeLoopVars = prevRangeVars
		SetVarTable(prevVarTable)
	}()
	SetTypeInfo(nil)
	localCollectionKinds = make(map[string]string)
	localRangeElemRustTypes = make(map[string]string)
	rangeLoopVars = make(map[string]string)

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
func f(ch chan struct{ name string }) {
	for event := range ch { println(event.name) }
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	fn := file.Decls[0].(*ast.FuncDecl)
	registerTypeExprCollectionInfo("ch", fn.Type.Params.List[0].Type)
	vt := NewVarTable()
	vt.Register("ch", &VarInfo{
		WrapLevel: WrapNone,
		Source:    SourceParam,
	})
	SetVarTable(vt)

	var out strings.Builder
	TranspileStatementSimple(&out, fn.Body.List[0], nil, token.NewFileSet())

	got := out.String()
	if !strings.Contains(got, "for event in ch.clone()") {
		t.Fatalf("tracked channel range did not use channel iteration:\n%s", got)
	}
	if strings.Contains(got, "type info required") || strings.Contains(got, "0..") {
		t.Fatalf("tracked channel range fell back to non-channel lowering:\n%s", got)
	}
}

func TestNoTypeInfoMakeChannelShortDeclIsBare(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevCollections := localCollectionKinds
	prevRangeElemTypes := localRangeElemRustTypes
	prevRangeVars := rangeLoopVars
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		localCollectionKinds = prevCollections
		localRangeElemRustTypes = prevRangeElemTypes
		rangeLoopVars = prevRangeVars
		SetVarTable(prevVarTable)
	}()
	SetTypeInfo(nil)
	localCollectionKinds = make(map[string]string)
	localRangeElemRustTypes = make(map[string]string)
	rangeLoopVars = make(map[string]string)
	vt := NewVarTable()
	SetVarTable(vt)

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
func f() {
	ch := make(chan int)
	for n := range ch { println(n) }
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	fn := file.Decls[0].(*ast.FuncDecl)

	var assignOut strings.Builder
	TranspileStatementSimple(&assignOut, fn.Body.List[0], nil, token.NewFileSet())
	assignRust := assignOut.String()
	if !strings.Contains(assignRust, "let mut ch = GoChannel::<i32>::new()") {
		t.Fatalf("make channel short decl should emit a bare GoChannel local:\n%s", assignRust)
	}
	if !isVarBare("ch") {
		t.Fatalf("make channel short decl did not register ch as bare; info=%#v\n%s", vt.Lookup("ch"), assignRust)
	}
	if localCollectionKinds["ch"] != "channel" {
		t.Fatalf("local collection kind for ch = %q, want channel", localCollectionKinds["ch"])
	}

	var rangeOut strings.Builder
	TranspileStatementSimple(&rangeOut, fn.Body.List[1], nil, token.NewFileSet())
	rangeRust := rangeOut.String()
	if !strings.Contains(rangeRust, "for n in ch.clone()") {
		t.Fatalf("tracked make channel range did not use channel iteration:\n%s", rangeRust)
	}
	if strings.Contains(rangeRust, ".lock()") || strings.Contains(rangeRust, "type info required") {
		t.Fatalf("tracked make channel range treated channel as wrapped:\n%s", rangeRust)
	}
}

func TestNoTypeInfoAnySelectorReturnKeepsFieldHandle(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevStructDefs := structDefs
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		structDefs = prevStructDefs
		SetVarTable(prevVarTable)
	}()
	SetTypeInfo(nil)

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
type entry struct { value any }
func get(e entry) any { return e.value }`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeSpec := file.Decls[0].(*ast.GenDecl).Specs[0].(*ast.TypeSpec)
	structType := typeSpec.Type.(*ast.StructType)
	structDefs = map[string]*StructDef{
		"entry": {
			Fields:  map[string]string{"value": "regular"},
			ASTType: structType,
		},
	}

	fn := file.Decls[1].(*ast.FuncDecl)
	vt := NewVarTable()
	vt.Register("e", &VarInfo{
		WrapLevel: WrapFull,
		RustType:  "entry",
		Source:    SourceParam,
	})
	SetVarTable(vt)

	var out strings.Builder
	TranspileStatementSimple(&out, fn.Body.List[0], fn.Type, token.NewFileSet())

	got := out.String()
	if !strings.Contains(got, ".value.clone()") {
		t.Fatalf("any selector return should clone the field handle:\n%s", got)
	}
	if strings.Contains(got, "Box<dyn Any") || strings.Contains(got, ".as_ref().unwrap()))") {
		t.Fatalf("any selector return should not unwrap and rewrap the Box payload:\n%s", got)
	}
}

func TestNoTypeInfoAnyHandleReuseUsesSyntax(t *testing.T) {
	src := `package main
import "fmt"
type entry struct { value any }
func assign(e *entry, value any) { e.value = value }
func callAssign(e *entry, value any) { assign(e, value) }
func each(e *entry, f func(any)) { f(e.value) }
func printAny(value any) { fmt.Println(value) }
func callEach(e *entry) { each(e, func(v any) { fmt.Println(v) }) }`

	assertAnyHandleReuseUsesSyntax(t, transpileNoTypeInfoRegression(t, src))
	assertAnyHandleReuseUsesSyntax(t, transpileRegression(t, src, &TypeInfo{}))
}

func assertAnyHandleReuseUsesSyntax(t *testing.T, rust string) {
	t.Helper()

	if !strings.Contains(rust, "let new_val = value.clone();") {
		t.Fatalf("any field assignment should clone the existing interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".as_mut().unwrap()).value = new_val") ||
		strings.Contains(rust, ".as_ref().unwrap()).value = new_val") {
		t.Fatalf("any field assignment should mutate the owning struct slot:\n%s", rust)
	}
	if !strings.Contains(rust, "assign(e.clone(), value.clone())") {
		t.Fatalf("any function argument should pass the existing interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".value.clone())") {
		t.Fatalf("any selector closure argument should pass the field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "format_any(value") {
		t.Fatalf("fmt.Println(any) should use format_any under syntax fallback:\n%s", rust)
	}
	if !strings.Contains(rust, "format_any(v") {
		t.Fatalf("fmt.Println on function-literal any parameter should use format_any:\n%s", rust)
	}
	if strings.Contains(rust, "value.borrow().as_ref().unwrap().clone()") ||
		strings.Contains(rust, "value.lock().unwrap().as_ref().unwrap().clone()") ||
		strings.Contains(rust, "Some((*value") ||
		strings.Contains(rust, "format!(\"{}\", (*value") ||
		strings.Contains(rust, "format!(\"{}\", (*v") {
		t.Fatalf("any handle reuse should not clone or rewrap the Box payload:\n%s", rust)
	}
}

func TestNoTypeInfoPackageGlobalIdentUsesGlobalName(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevGlobals := packageGlobalNames
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageGlobalNames = prevGlobals
		SetVarTable(prevVarTable)
	}()
	SetTypeInfo(nil)
	packageGlobalNames = map[string]bool{"n": true}
	SetVarTable(NewVarTable())

	var exprOut strings.Builder
	TranspileExpression(&exprOut, ast.NewIdent("n"))
	if got := exprOut.String(); strings.Contains(got, "n_local") || !strings.Contains(got, "(*n") {
		t.Fatalf("package global expression = %q, want global n access", got)
	}

	stmt, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
func f() { n++ }`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	fn := stmt.Decls[0].(*ast.FuncDecl)
	var stmtOut strings.Builder
	TranspileStatementSimple(&stmtOut, fn.Body.List[0], nil, token.NewFileSet())
	if got := stmtOut.String(); strings.Contains(got, "n_local") || !strings.Contains(got, "n.borrow_mut()") {
		t.Fatalf("package global increment = %q, want global n mutation", got)
	}
}

func TestNoTypeInfoPackageGlobalMapIndexUsesSyntaxKind(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevGlobals := packageGlobalNames
	prevCollections := localCollectionKinds
	prevMapKeys := localMapKeyRustTypes
	prevMapValues := localMapValueRustTypes
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageGlobalNames = prevGlobals
		localCollectionKinds = prevCollections
		localMapKeyRustTypes = prevMapKeys
		localMapValueRustTypes = prevMapValues
	}()
	SetTypeInfo(nil)
	packageGlobalNames = make(map[string]bool)
	localCollectionKinds = make(map[string]string)
	localMapKeyRustTypes = make(map[string]string)
	localMapValueRustTypes = make(map[string]string)

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
var counts map[string]int
func f() { _ = counts["x"] }`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	globalDecl := file.Decls[0].(*ast.GenDecl)
	collectPackageGlobals([]*ast.GenDecl{globalDecl})
	if got := localCollectionKinds["counts"]; got != "map" {
		t.Fatalf("package global map kind = %q, want map", got)
	}

	restore := pushFunctionLocalSyntaxInfo()
	defer restore()
	if got := localCollectionKinds["counts"]; got != "map" {
		t.Fatalf("function-local syntax scope dropped package global map kind = %q, want map", got)
	}

	fn := file.Decls[1].(*ast.FuncDecl)
	assign := fn.Body.List[0].(*ast.AssignStmt)
	index := assign.Rhs[0]
	var out strings.Builder
	TranspileExpression(&out, index)

	got := out.String()
	if strings.Contains(got, "type info required") {
		t.Fatalf("package global map index should use syntax collection kind:\n%s", got)
	}
	if !strings.Contains(got, "(*counts.borrow().as_ref().unwrap()).get(&\"x\".to_string())") {
		t.Fatalf("package global map index should read from global map:\n%s", got)
	}
}

func TestNoTypeInfoPackageGlobalFunctionMapIndexKeepsHandle(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevGlobals := packageGlobalNames
	prevCollections := localCollectionKinds
	prevMapKeys := localMapKeyRustTypes
	prevMapValues := localMapValueRustTypes
	prevAliases := functionTypeAliases
	prevAliasBoxes := functionTypeAliasBoxTypes
	prevContext := currentContext
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageGlobalNames = prevGlobals
		localCollectionKinds = prevCollections
		localMapKeyRustTypes = prevMapKeys
		localMapValueRustTypes = prevMapValues
		functionTypeAliases = prevAliases
		functionTypeAliasBoxTypes = prevAliasBoxes
		SetTranspileContext(prevContext)
	}()
	SetTranspileContext(nil)
	SetTypeInfo(nil)
	packageGlobalNames = make(map[string]bool)
	localCollectionKinds = make(map[string]string)
	localMapKeyRustTypes = make(map[string]string)
	localMapValueRustTypes = make(map[string]string)
	functionTypeAliases = map[string]bool{"handler": true}
	functionTypeAliasBoxTypes = map[string]string{
		"handler": "Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>",
	}

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
type handler func(int) int
var handlers map[string]handler
func f() { _ = handlers["inc"] }`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	globalDecl := file.Decls[1].(*ast.GenDecl)
	collectPackageGlobals([]*ast.GenDecl{globalDecl})
	restore := pushFunctionLocalSyntaxInfo()
	defer restore()
	if got := localMapValueRustTypes["handlers"]; got == "" {
		t.Fatal("package global function map value type was not tracked")
	} else if !rustMapValueTypeKeepsHandle(got) {
		t.Fatalf("tracked function map value type %q should keep handle", got)
	}

	fn := file.Decls[2].(*ast.FuncDecl)
	assign := fn.Body.List[0].(*ast.AssignStmt)
	index := assign.Rhs[0]
	var out strings.Builder
	TranspileExpression(&out, index)

	if got, want := functionBoxTypeForCallTarget(index), "Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>"; got != want {
		t.Fatalf("function map index box type = %q, want %q", got, want)
	}

	got := out.String()
	if !strings.Contains(got, ".map(|__v| __v.clone()).unwrap_or_else(|| Default::default())") {
		t.Fatalf("function map index should clone the stored handle:\n%s", got)
	}
	if strings.Contains(got, ".borrow().as_ref().unwrap().clone()") {
		t.Fatalf("function map index should not unwrap the stored function handle:\n%s", got)
	}
}

func TestNoTypeInfoAssignedNestedStringRangeUsesBareValue(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevRangeVars := rangeLoopVars
	defer func() {
		currentTypeInfo = prevTypeInfo
		rangeLoopVars = prevRangeVars
	}()
	SetTypeInfo(nil)
	rangeLoopVars = map[string]string{"files": "&Vec<String>"}

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main
func f() {
	for _, file := range files {
		if file == "a.go" {
			file = "src/" + file
		}
		res = append(res, file)
	}
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	fn := file.Decls[0].(*ast.FuncDecl)
	stmt := fn.Body.List[0]
	var out strings.Builder
	TranspileStatementSimple(&out, stmt, nil, token.NewFileSet())

	got := out.String()
	if !strings.Contains(got, "for mut file in files.iter().cloned()") {
		t.Fatalf("assigned nested string range should iterate owned values:\n%s", got)
	}
	if strings.Contains(got, "file.lock()") || strings.Contains(got, "(*file).clone()") {
		t.Fatalf("assigned nested string range should treat file as bare String:\n%s", got)
	}
	if !strings.Contains(got, "file = new_val") {
		t.Fatalf("assigned nested string range should assign the bare binding:\n%s", got)
	}
	if !strings.Contains(got, ".push(file.clone())") {
		t.Fatalf("assigned nested string range append should clone the bare binding:\n%s", got)
	}
}

func TestStructRangeSelectorUsesRangeBinding(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import (
	"fmt"
	"sort"
)

type packageFunctionName struct {
	goName string
	pos int
	exported bool
}

func f() map[string]string {
	byRustName := make(map[string][]packageFunctionName)
	overrides := make(map[string]string)
	for rustName, functions := range byRustName {
		sort.Slice(functions, func(i, j int) bool {
			if functions[i].exported != functions[j].exported {
				return functions[i].exported
			}
			if functions[i].pos != functions[j].pos {
				return functions[i].pos < functions[j].pos
			}
			return functions[i].goName < functions[j].goName
		})
		for i, fn := range functions {
			if i == 0 {
				continue
			}
			overrides[fn.goName] = fmt.Sprintf("%s_%d", rustName, i)
		}
	}
	return overrides
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "fn.borrow()") || strings.Contains(rust, "fn.lock()") || strings.Contains(rust, "r#fn.borrow()") || strings.Contains(rust, "r#fn.lock()") {
		t.Fatalf("struct range selector should use the range binding directly:\n%s", rust)
	}
	if !strings.Contains(rust, "r#fn.go_name") {
		t.Fatalf("struct range selector should access the field on the range binding:\n%s", rust)
	}
}

func TestPackageGlobalsRangeSelectorUsesRangeBinding(t *testing.T) {
	fset := token.NewFileSet()
	entries, err := os.ReadDir(".")
	if err != nil {
		t.Fatalf("ReadDir(.) error = %v", err)
	}
	var files []*ast.File
	var target *ast.File
	for _, entry := range entries {
		name := entry.Name()
		if entry.IsDir() || !strings.HasSuffix(name, ".go") || strings.HasSuffix(name, "_test.go") {
			continue
		}
		file, err := parser.ParseFile(fset, name, nil, 0)
		if err != nil {
			t.Fatalf("ParseFile(%s) error = %v", name, err)
		}
		files = append(files, file)
		if name == "package_globals.go" {
			target = file
		}
	}
	if target == nil {
		t.Fatal("package_globals.go was not parsed")
	}
	// The transpiler's own files import non-stdlib packages such as
	// golang.org/x/tools/go/packages that the default importer cannot resolve.
	// NewTypeInfoWithImporter now returns those partial-info errors instead of
	// silently dropping them, but the regression being pinned here only needs
	// the in-package type information.
	typeInfo, err := NewTypeInfo(files, fset)
	if typeInfo == nil {
		t.Fatalf("NewTypeInfo() returned no TypeInfo: %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(target, fset, typeInfo)
	for _, bad := range []string{
		"r#fn.lock().unwrap().as_ref().unwrap()).go_name",
		"r#fn.borrow().as_ref().unwrap()).go_name",
	} {
		idx := strings.Index(rust, bad)
		if idx < 0 {
			continue
		}
		start := max(0, idx-200)
		end := min(len(rust), idx+300)
		t.Fatalf("package_globals range selector should use the range binding directly:\n%s", rust[start:end])
	}
}

func TestMethodPointerArgsPreserveHandles(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/types"

type term struct{}
type termlist []*term

func (x *term) union(y *term) (*term, *term) { return x, y }
func (x *term) includes(t types.Type) bool { return false }

func (xl termlist) norm(t types.Type) {
	for i, xi := range xl {
		xj := xl[i]
		xi.union(xj)
	}
	for _, x := range xl {
		x.includes(t)
	}
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "union((*xj") || strings.Contains(rust, "intersect((*y") {
		t.Fatalf("pointer method argument should preserve the pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".union(xj.clone())") {
		t.Fatalf("pointer method argument should clone the pointer handle:\n%s", rust)
	}
	if strings.Contains(rust, "includes({ let __v = (*t") {
		t.Fatalf("stdlib interface method argument should preserve the interface handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".includes(t.clone())") {
		t.Fatalf("stdlib interface method argument should clone the interface handle:\n%s", rust)
	}
}

func TestNilPointerFunctionArgumentUsesNilHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Context struct{}

func read(ctxt *Context) {}

func f() {
	read(nil)
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "nil.clone()") {
		t.Fatalf("nil pointer argument should emit a nil handle:\n%s", rust)
	}
	if !strings.Contains(rust, "read(Rc::new(RefCell::new(None)))") {
		t.Fatalf("nil pointer argument should pass a wrapped nil handle:\n%s", rust)
	}
}

func TestReturnStringParameterCopiesWrappedValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func echo(value string) string {
	return value
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "return value.clone();") {
		t.Fatalf("returning a string parameter should not alias the wrapper handle:\n%s", rust)
	}
	if !strings.Contains(rust, "return Rc::new(RefCell::new(Some(value.borrow().as_ref().unwrap().clone())));") {
		t.Fatalf("returning a string parameter should clone the wrapped value into a new handle:\n%s", rust)
	}
}

func TestForInitShortDeclShadowsOuterRangeIndex(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type List struct{}

func (l *List) Len() int { return 0 }
func (l *List) At(i int) int { return i }

func f(lists []*List) {
	go func() {}()
	for i, list := range lists {
		_ = i
		for i := 0; i < list.Len(); i++ {
			_ = list.At(i)
		}
	}
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "let __tmp_x = i;") {
		t.Fatalf("inner for index should shadow outer bare range index in comparisons:\n%s", rust)
	}
	if strings.Contains(rust, ".at(i);") {
		t.Fatalf("inner for index should be passed as a wrapped handle, not the outer bare range index:\n%s", rust)
	}
	if strings.Contains(rust, ".at(i.clone())") {
		t.Fatalf("inner for index should not inherit bare range-index call argument handling:\n%s", rust)
	}
	if !strings.Contains(rust, "while (*i.borrow().as_ref().unwrap()) <") {
		t.Fatalf("inner for index should be unwrapped from its own local wrapper:\n%s", rust)
	}
}

func TestRangeIndexUntypedConstPeersUseGoInt(t *testing.T) {
	rust := transpileRegression(t, `package main

func f(xs []int) int {
	go func() {}()
	total := 0
	for i := range xs {
		if i > 0 {
			total += i
		}
		j := i + 1
		total += j
	}
	return total
}`, &TypeInfo{})

	if strings.Contains(rust, "0 as usize") || strings.Contains(rust, "1 as usize") {
		t.Fatalf("range index untyped integer peers should use Go int, not usize:\n%s", rust)
	}
	hasComparison := strings.Contains(rust, "i as i32 > 0 as i32") ||
		strings.Contains(rust, "let __tmp_x = i as i32; let __tmp_y = 0 as i32")
	if !hasComparison {
		t.Fatalf("range index comparison should cast the range index and constant to Go int:\n%s", rust)
	}
	hasArithmetic := strings.Contains(rust, "i as i32 + 1 as i32") ||
		strings.Contains(rust, "let __tmp_x = i as i32; let __tmp_y = 1 as i32")
	if !hasArithmetic {
		t.Fatalf("range index arithmetic should cast the range index and constant to Go int:\n%s", rust)
	}
}

func TestIncompleteTypeInfoPromotedEmbeddedFieldUsesSyntax(t *testing.T) {
	rust := transpileRegression(t, `package main

type Person struct {
	Name string
}

type Employee struct {
	Person
	ID int
}

func f() string {
	emp := Employee{Person: Person{Name: "Alice"}, ID: 1}
	return emp.Name
}`, &TypeInfo{})

	if !strings.Contains(rust, ".person.borrow().as_ref().unwrap()).name") &&
		!strings.Contains(rust, ".person.lock().unwrap().as_ref().unwrap()).name") {
		t.Fatalf("promoted embedded field should traverse the embedded Person field:\n%s", rust)
	}
}

func TestReturnStructSliceRangeValueClonesReference(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Label struct {
	name string
}

type listMap struct {
	labels []Label
}

func (lm listMap) Find(name string) Label {
	for _, l := range lm.labels {
		if l.name == name {
			return l
		}
	}
	return Label{}
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "Some(l)") {
		t.Fatalf("returning a struct range value should not wrap the reference directly:\n%s", rust)
	}
	if !strings.Contains(rust, "Some((*l).clone())") {
		t.Fatalf("returning a struct range value should clone the referenced value:\n%s", rust)
	}
}

func TestRangeStringFunctionArgumentWrapsOwnedClone(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

func find(pkg string) (int, bool) { return 0, true }

func imports(pkgs []string) {
	for _, pkg := range pkgs {
		find(pkg)
	}
}`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "find(pkg.clone())") {
		t.Fatalf("range string argument should not be passed as a bare clone:\n%s", rust)
	}
	if !strings.Contains(rust, "find(Rc::new(RefCell::new(Some((*pkg).clone()))))") {
		t.Fatalf("range string argument should be cloned into a wrapped Go string:\n%s", rust)
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
