package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
	"strings"
	"testing"
)

func firstEquality(t *testing.T, file *ast.File) *ast.BinaryExpr {
	t.Helper()

	var comparison *ast.BinaryExpr
	ast.Inspect(file, func(node ast.Node) bool {
		if expr, ok := node.(*ast.BinaryExpr); ok && expr.Op == token.EQL {
			comparison = expr
			return false
		}
		return true
	})
	if comparison == nil {
		t.Fatal("comparison not found")
	}
	return comparison
}

func TestConstExpressionForByteBinaryPeer(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

const opType = '.'

func main() {
	var path []byte
	_ = path[0] == opType
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	comparison := firstEquality(t, file)

	var out strings.Builder
	if !writeConstExpressionForBinaryPeer(&out, comparison.Y, comparison.X) {
		t.Fatal("const expression was not converted for byte peer")
	}
	if got, want := out.String(), "OP_TYPE as u8"; got != want {
		t.Fatalf("converted const = %q, want %q", got, want)
	}
}

func TestConstExpressionForNamedBytePeerUsesNamedConversion(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type VarKind uint8

const LocalVar VarKind = 1

func main() {
	var kind VarKind
	_ = kind == LocalVar
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}

	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	SetTypeInfo(typeInfo)
	defer SetTypeInfo(nil)

	comparison := firstEquality(t, file)

	var out strings.Builder
	if !writeConstExpressionForBinaryPeer(&out, comparison.Y, comparison.X) {
		t.Fatal("const expression was not converted for named byte-like peer")
	}
	if got, want := out.String(), "VarKind(Rc::new(RefCell::new(Some(LOCAL_VAR as u8))))"; got != want {
		t.Fatalf("converted const = %q, want %q", got, want)
	}
}

func TestNoTypeInfoPackageConstDoesNotUseGlobalPath(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevPackageConstants := packageConstants
	prevPackageGlobals := packageGlobalNames
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageConstants = prevPackageConstants
		packageGlobalNames = prevPackageGlobals
		SetVarTable(prevVarTable)
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	packageConstants = map[string]string{"Field": "i8"}
	packageGlobalNames = map[string]bool{"Field": true}

	ident := ast.NewIdent("Field")
	if !isConstIdent(ident) {
		t.Fatal("package constant should be recognized without type info")
	}
	if isPackageGlobalIdent(ident) {
		t.Fatal("package constant should not be treated as a package global")
	}

	var out strings.Builder
	TranspileExpressionContext(&out, ident, RValue)
	if got, want := out.String(), "FIELD"; got != want {
		t.Fatalf("package constant expression = %q, want %q", got, want)
	}
}

func TestPackageConstDeclUpdatesCurrentContext(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevPackageConstants := packageConstants
	prevContext := GetTranspileContext()
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageConstants = prevPackageConstants
		SetTranspileContext(prevContext)
	}()

	SetTypeInfo(nil)
	packageConstants = make(map[string]string)
	ctx := &TranspileContext{Package: NewPackageState()}
	SetTranspileContext(ctx)

	var out strings.Builder
	TranspileConstDecl(&out, &ast.GenDecl{
		Tok: token.CONST,
		Specs: []ast.Spec{&ast.ValueSpec{
			Names:  []*ast.Ident{ast.NewIdent("Field")},
			Type:   ast.NewIdent("int8"),
			Values: []ast.Expr{&ast.BasicLit{Kind: token.INT, Value: "1"}},
		}},
	})

	if got, want := packageConstants["Field"], "i8"; got != want {
		t.Fatalf("packageConstants[Field] = %q, want %q", got, want)
	}
	if got, want := ctx.Package.PackageConstants["Field"], "i8"; got != want {
		t.Fatalf("context package constant = %q, want %q", got, want)
	}
}

func TestCollectPackageGlobalsIgnoresConstDecl(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevGlobals := packageGlobalNames
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageGlobalNames = prevGlobals
	}()

	SetTypeInfo(nil)
	packageGlobalNames = make(map[string]bool)
	collectPackageGlobals([]*ast.GenDecl{{
		Tok: token.CONST,
		Specs: []ast.Spec{&ast.ValueSpec{
			Names:  []*ast.Ident{ast.NewIdent("Field")},
			Type:   ast.NewIdent("int8"),
			Values: []ast.Expr{&ast.BasicLit{Kind: token.INT, Value: "1"}},
		}},
	}})

	if packageGlobalNames["Field"] {
		t.Fatal("const declaration should not be registered as a package global")
	}
}

func TestPackageConstTypeInfoOverridesGlobalRegistry(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevGlobals := packageGlobalNames
	prevPackageConstants := packageConstants
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageGlobalNames = prevGlobals
		packageConstants = prevPackageConstants
		SetVarTable(prevVarTable)
	}()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

const Field int8 = 1

func f() {
	_ = Field
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
	SetVarTable(NewVarTable())
	packageConstants = make(map[string]string)
	packageGlobalNames = map[string]bool{"Field": true}

	fn := file.Decls[1].(*ast.FuncDecl)
	assign := fn.Body.List[0].(*ast.AssignStmt)
	ident := assign.Rhs[0].(*ast.Ident)
	if !isConstIdent(ident) {
		t.Fatal("type info should identify Field as a constant")
	}
	if isPackageGlobalIdent(ident) {
		t.Fatal("type-info-proven constant should not be treated as a package global")
	}

	var out strings.Builder
	TranspileExpressionContext(&out, ident, RValue)
	if got, want := out.String(), "FIELD"; got != want {
		t.Fatalf("package constant expression = %q, want %q", got, want)
	}
}

func TestPackageConstTypeInfoOverridesStaleVarTable(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevGlobals := packageGlobalNames
	prevPackageConstants := packageConstants
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageGlobalNames = prevGlobals
		packageConstants = prevPackageConstants
		SetVarTable(prevVarTable)
	}()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

const Field int8 = 1

func f() {
	_ = Field
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	vt := NewVarTable()
	vt.Register("Field", &VarInfo{WrapLevel: WrapFull, Source: SourceLocal})
	SetTypeInfo(typeInfo)
	SetVarTable(vt)
	packageConstants = make(map[string]string)
	packageGlobalNames = make(map[string]bool)

	fn := file.Decls[1].(*ast.FuncDecl)
	assign := fn.Body.List[0].(*ast.AssignStmt)
	ident := assign.Rhs[0].(*ast.Ident)
	if !isConstIdent(ident) {
		t.Fatal("type-info-proven constant should not be hidden by stale VarTable data")
	}

	var out strings.Builder
	TranspileExpressionContext(&out, ident, RValue)
	if got, want := out.String(), "FIELD"; got != want {
		t.Fatalf("package constant expression = %q, want %q", got, want)
	}
}

func TestRegisteredExportedPackageGlobalKeepsGlobalPath(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevGlobals := packageGlobalNames
	prevPackageConstants := packageConstants
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageGlobalNames = prevGlobals
		packageConstants = prevPackageConstants
		SetVarTable(prevVarTable)
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	packageConstants = make(map[string]string)
	packageGlobalNames = map[string]bool{"Field": true}

	var out strings.Builder
	TranspileExpressionContext(&out, ast.NewIdent("Field"), RValue)
	if got := out.String(); !strings.Contains(got, "Field") || strings.Contains(got, "FIELD") {
		t.Fatalf("registered exported package global should keep global path, got %q", got)
	}
}

func TestRegisterPackageGlobalNamesBeforeEmission(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevGlobals := packageGlobalNames
	prevPackageConstants := packageConstants
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageGlobalNames = prevGlobals
		packageConstants = prevPackageConstants
		SetVarTable(prevVarTable)
	}()

	SetTypeInfo(nil)
	SetVarTable(NewVarTable())
	packageConstants = make(map[string]string)
	packageGlobalNames = make(map[string]bool)

	file, err := parser.ParseFile(token.NewFileSet(), "main.go", `package main

var PackageSymbols map[string]int

func f() {
	_ = PackageSymbols
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	globalDecl := file.Decls[0].(*ast.GenDecl)
	registerPackageGlobalNames([]*ast.GenDecl{globalDecl})

	fn := file.Decls[1].(*ast.FuncDecl)
	assign := fn.Body.List[0].(*ast.AssignStmt)
	var out strings.Builder
	TranspileExpressionContext(&out, assign.Rhs[0], RValue)
	if got := out.String(); strings.Contains(got, "PACKAGE_SYMBOLS") || !strings.Contains(got, "PackageSymbols") {
		t.Fatalf("registered exported package global should not use const fallback, got %q", got)
	}
}

func TestCrossFilePackageGlobalUsesTypeInfoBeforeConstFallback(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevGlobals := packageGlobalNames
	prevPackageConstants := packageConstants
	prevVarTable := currentVarTable
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageGlobalNames = prevGlobals
		packageConstants = prevPackageConstants
		SetVarTable(prevVarTable)
	}()

	fset := token.NewFileSet()
	manifestFile, err := parser.ParseFile(fset, "manifest.go", `package stdlib

var PackageSymbols map[string]int
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(manifest.go) error = %v", err)
	}
	stdlibFile, err := parser.ParseFile(fset, "stdlib.go", `package stdlib

func HasPackage(path string) bool {
	_, ok := PackageSymbols[path]
	return ok
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(stdlib.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{manifestFile, stdlibFile}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	SetTypeInfo(typeInfo)
	SetVarTable(NewVarTable())
	packageConstants = make(map[string]string)
	packageGlobalNames = make(map[string]bool)

	fn := stdlibFile.Decls[0].(*ast.FuncDecl)
	assign := fn.Body.List[0].(*ast.AssignStmt)
	index := assign.Rhs[0].(*ast.IndexExpr)
	var out strings.Builder
	TranspileExpressionContext(&out, index.X, RValue)
	if got := out.String(); strings.Contains(got, "PACKAGE_SYMBOLS") || !strings.Contains(got, "PackageSymbols") {
		t.Fatalf("cross-file package global should not use const fallback, got %q", got)
	}
}

func transpileNoTypeInfoRegression(t *testing.T, src string) string {
	return transpileRegression(t, src, nil)
}

func transpileRegression(t *testing.T, src string, typeInfo *TypeInfo) string {
	t.Helper()

	prevTypeInfo := currentTypeInfo
	prevContext := currentContext
	prevVarTable := currentVarTable
	prevPackageConstants := packageConstants
	prevPackageConstantTypeNames := packageConstantTypeNames
	prevPackageGlobals := packageGlobalNames
	prevTypeDefinitions := typeDefinitions
	prevStructDefs := structDefs
	prevFunctionSignatures := functionSignatures
	prevFunctionNameOverrides := functionNameOverrides
	prevFunctionNameOverridesByGoName := functionNameOverridesByGoName
	prevPackageFunctionNameOverrides := packageFunctionNameOverrides
	prevPackageMethodNameOverrides := packageMethodNameOverrides
	prevLocalConstants := localConstants
	prevRangeLoopVars := rangeLoopVars
	prevCurrentReceiver := currentReceiver
	prevCurrentReceiverType := currentReceiverType
	prevCurrentTypeMethods := currentTypeMethods
	prevHasInitFunction := hasInitFunction
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
		currentContext = prevContext
		SetVarTable(prevVarTable)
		packageConstants = prevPackageConstants
		packageConstantTypeNames = prevPackageConstantTypeNames
		packageGlobalNames = prevPackageGlobals
		typeDefinitions = prevTypeDefinitions
		structDefs = prevStructDefs
		functionSignatures = prevFunctionSignatures
		functionNameOverrides = prevFunctionNameOverrides
		functionNameOverridesByGoName = prevFunctionNameOverridesByGoName
		packageFunctionNameOverrides = prevPackageFunctionNameOverrides
		packageMethodNameOverrides = prevPackageMethodNameOverrides
		localConstants = prevLocalConstants
		rangeLoopVars = prevRangeLoopVars
		currentReceiver = prevCurrentReceiver
		currentReceiverType = prevCurrentReceiverType
		currentTypeMethods = prevCurrentTypeMethods
		hasInitFunction = prevHasInitFunction
	})

	currentTypeInfo = typeInfo
	currentContext = nil
	SetVarTable(nil)

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	rust, _, _ := Transpile(file, fset, typeInfo)
	return rust
}

func TestNoTypeInfoNamedConstComparisonUsesConstPath(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

type Kind int8

const Field Kind = 1

type Symbol struct {
	Kind Kind
}

func (s Symbol) isField() bool {
	return s.Kind == Field
}
`)

	if strings.Contains(rust, "Field.") {
		t.Fatalf("named constant comparison should not use wrapped global path:\n%s", rust)
	}
	if !strings.Contains(rust, "let __tmp_y = FIELD") {
		t.Fatalf("named constant comparison should use the generated const:\n%s", rust)
	}
	if !strings.Contains(rust, "let __selector_holder = self.kind.clone()") {
		t.Fatalf("named selector operand should be cloned before comparison:\n%s", rust)
	}
}

func TestIncompleteTypeInfoReturnExtractionClonesSyntaxNamedSelector(t *testing.T) {
	rust := transpileRegression(t, `package main

type Kind int8

const Field Kind = 1

type Symbol struct {
	Kind Kind
}

func (s Symbol) isField() bool {
	return s.Kind == Field
}
`, &TypeInfo{})

	if strings.Contains(rust, "let __tmp_x = (*self.kind.") {
		t.Fatalf("return extraction should not move a named selector field out of a shared borrow:\n%s", rust)
	}
	if !strings.Contains(rust, "let __selector_holder = self.kind.clone()") {
		t.Fatalf("return extraction should use the syntax clone for named selector fields:\n%s", rust)
	}
}

func TestNoTypeInfoNamedConstSwitchCaseUsesConstPath(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

type Kind int8

const (
	Field Kind = 1
	Method Kind = 2
)

type Symbol struct {
	Kind Kind
}

func (s Symbol) kindName() string {
	switch s.Kind {
	case Field:
		return "field"
	case Method:
		return "method"
	}
	return "invalid"
}
`)

	if strings.Contains(rust, "Field.") || strings.Contains(rust, "Method.") {
		t.Fatalf("named constant switch cases should not use wrapped global path:\n%s", rust)
	}
	if !strings.Contains(rust, "_switch_val = { let __selector_holder = self.kind.clone()") {
		t.Fatalf("named selector switch tag should be cloned before comparison:\n%s", rust)
	}
	if !strings.Contains(rust, "_switch_val == (FIELD)") || !strings.Contains(rust, "_switch_val == (METHOD)") {
		t.Fatalf("named constant switch cases should use generated consts:\n%s", rust)
	}
}

func TestNoTypeInfoLocalConstUsesSyntaxByteFieldContext(t *testing.T) {
	src := `package main

type node struct {
	color uint8
}

func main() {
	const (
		white = 0
		grey = 1
		black = 2
	)
	n := node{color: white}
	if n.color == white {
		n.color = grey
	}
	n.color = black
}
`
	assertLocalConstUsesSyntaxByteFieldContext(t, transpileNoTypeInfoRegression(t, src))
	assertLocalConstUsesSyntaxByteFieldContext(t, transpileRegression(t, src, &TypeInfo{}))
}

func assertLocalConstUsesSyntaxByteFieldContext(t *testing.T, rust string) {
	t.Helper()
	if !strings.Contains(rust, "color: Rc::new(RefCell::new(Some(white as u8)))") {
		t.Fatalf("struct field literal should cast local const to byte field type:\n%s", rust)
	}
	if !strings.Contains(rust, "== white as u8") {
		t.Fatalf("selector comparison should cast local const to byte field type:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = grey as u8") {
		t.Fatalf("selector assignment should cast local const to byte field type:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = black as u8") {
		t.Fatalf("selector assignment should cast later local const to byte field type:\n%s", rust)
	}
}

func TestNoTypeInfoNamedUintConstPeersUseSyntax(t *testing.T) {
	src := `package main

type Version uint32

const (
	V0 Version = iota
	V1
	V2
	numVersions = iota
)

const (
	flagSyncMarkers = 1 << iota
)

type Header struct {
	version Version
}

func (v Version) Has(f Version) bool {
	return V0 <= v && (v < V2 || f == V0)
}

func decode(ver uint32, flags uint32) bool {
	var h Header
	h.version = Version(ver)
	if h.version >= numVersions {
		return false
	}
	return h.version.Has(V1) && flags&flagSyncMarkers != 0
}
`
	assertNamedUintConstPeersUseSyntax(t, transpileNoTypeInfoRegression(t, src))
	assertNamedUintConstPeersUseSyntax(t, transpileRegression(t, src, &TypeInfo{}))
	assertNamedUintConstPeersUseSyntax(t, transpileRegression(t, src, &TypeInfo{info: &types.Info{
		Types:      map[ast.Expr]types.TypeAndValue{},
		Defs:       map[*ast.Ident]types.Object{},
		Uses:       map[*ast.Ident]types.Object{},
		Selections: map[*ast.SelectorExpr]*types.Selection{},
	}}))
}

func assertNamedUintConstPeersUseSyntax(t *testing.T, rust string) {
	t.Helper()
	if strings.Contains(rust, "new_val.borrow_mut()") || strings.Contains(rust, "new_val.lock().unwrap()") {
		t.Fatalf("named integer conversion assignment should store the converted value, not move an inner wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = Version(") || !strings.Contains(rust, ".version.borrow_mut() = Some(new_val)") {
		t.Fatalf("named integer conversion assignment should assign the named value into the field wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "NUM_VERSIONS as u32") {
		t.Fatalf("comparison with named uint field should cast untyped peer constant to u32:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(Version(") || !strings.Contains(rust, "V1 as u32") {
		t.Fatalf("method argument should wrap typed named const V1 as Version:\n%s", rust)
	}
	if !strings.Contains(rust, "FLAG_SYNC_MARKERS as u32") || !strings.Contains(rust, "0 as u32") {
		t.Fatalf("bitwise uint32 expression should cast untyped constants to u32:\n%s", rust)
	}
}

func TestNoTypeInfoNamedIntegerBitwiseClonesSelectorField(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

type Kind int8

const (
	Invalid Kind = 0
	Field Kind = 1
	Method Kind = 2
)

type Symbol struct {
	Kind Kind
}

func (s Symbol) hasFieldFlag() bool {
	return s.Kind&Field != Invalid && s.Kind|Method != Invalid
}
`)

	if strings.Contains(rust, "let __tmp_x = (*self.kind.") {
		t.Fatalf("named integer selector operand should not move from a shared field borrow:\n%s", rust)
	}
	if !strings.Contains(rust, "let __selector_holder = self.kind.clone()") {
		t.Fatalf("named integer selector operand should be cloned before bitwise operations:\n%s", rust)
	}
}

func TestNoTypeInfoChannelSendUnwrapsMethodBoolResult(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

type Symbol struct{}

func (s Symbol) isField() bool {
	return true
}

func run() {
	done := make(chan bool, 1)
	sym := Symbol{}
	go func() {
		done <- sym.isField()
	}()
}
`)

	unwrappedWithLock := strings.Contains(rust, ".is_field().lock().unwrap().as_ref().unwrap()")
	unwrappedWithBorrow := strings.Contains(rust, ".is_field().borrow().as_ref().unwrap()")
	if !strings.Contains(rust, ".send((*") || (!unwrappedWithLock && !unwrappedWithBorrow) {
		t.Fatalf("channel send should unwrap the wrapped bool returned by a method call:\n%s", rust)
	}
}

func TestNoTypeInfoFloatStructFieldsAndBinaryConstantsUseSyntax(t *testing.T) {
	src := `package main

type Rect struct {
	Width, Height float64
}

func (r Rect) Perimeter() float64 {
	return 2 * (r.Width + r.Height)
}

func main() {
	_ = Rect{Width: 10, Height: 5}
}
`

	assertNoTypeInfoFloatStructFieldsAndBinaryConstantsUseSyntax(t, transpileNoTypeInfoRegression(t, src))
	assertNoTypeInfoFloatStructFieldsAndBinaryConstantsUseSyntax(t, transpileRegression(t, src, &TypeInfo{}))
}

func assertNoTypeInfoFloatStructFieldsAndBinaryConstantsUseSyntax(t *testing.T, rust string) {
	t.Helper()
	if strings.Contains(rust, "Some(2 *") {
		t.Fatalf("float binary expression should not keep an untyped integer lhs:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(2.0 *") {
		t.Fatalf("float binary expression should cast the untyped integer lhs:\n%s", rust)
	}
	if strings.Contains(rust, "height: Rc::new(RefCell::new(Some(5)))") {
		t.Fatalf("float struct field should not keep an untyped integer literal:\n%s", rust)
	}
	if !strings.Contains(rust, "height: Rc::new(RefCell::new(Some(5 as f64)))") &&
		!strings.Contains(rust, "height: Rc::new(RefCell::new(Some(5.0 as f64)))") {
		t.Fatalf("float struct field should cast the untyped integer literal:\n%s", rust)
	}
}
