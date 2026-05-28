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

func TestConstDeclLowersTypeConversionUnaryConstExpression(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const PtrSize = 4 << (^uintptr(0) >> 63)
const Int64Align = PtrSize
`)

	if strings.Contains(rust, "Arc::new") || strings.Contains(rust, "Mutex::new") {
		t.Fatalf("const expressions must not use runtime wrappers:\n%s", rust)
	}
	if !strings.Contains(rust, "pub const PTR_SIZE: i32 = 4 << (!(0 as usize) >> 63);") {
		t.Fatalf("uintptr conversion and unary xor const expression not lowered as a Rust const:\n%s", rust)
	}
	if !strings.Contains(rust, "pub const INT64_ALIGN: i32 = PTR_SIZE;") {
		t.Fatalf("const identifier initialized from PtrSize should keep the const path and type:\n%s", rust)
	}
}

func TestConstDeclUnsafeSizeofEmitsBareSizeof(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

type ctrlGroup uint64

const ctrlGroupsSize = unsafe.Sizeof(ctrlGroup(0))
`)

	if !strings.Contains(rust, "pub(crate) const CTRL_GROUPS_SIZE: usize = std::mem::size_of::<ctrlGroup>();") {
		t.Fatalf("unsafe.Sizeof const initializer should emit a bare Rust const expression:\n%s", rust)
	}
	if strings.Contains(rust, "CTRL_GROUPS_SIZE: usize = Arc::new") ||
		strings.Contains(rust, "CTRL_GROUPS_SIZE: usize = Rc::new") {
		t.Fatalf("unsafe.Sizeof const initializer must not emit runtime wrappers:\n%s", rust)
	}
}

func TestConstDeclUnsafeOffsetofEmitsBareOffsetof(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unsafe"

var cpu struct {
	HasSSE42 bool
	Count uint64
}

const offsetCount = unsafe.Offsetof(cpu.Count)
`)

	if !strings.Contains(rust, "pub(crate) const OFFSET_COUNT: usize = std::mem::offset_of!(") {
		t.Fatalf("unsafe.Offsetof const initializer should emit a bare Rust const expression:\n%s", rust)
	}
	if strings.Contains(rust, "OFFSET_COUNT: usize = Arc::new") ||
		strings.Contains(rust, "OFFSET_COUNT: usize = Rc::new") {
		t.Fatalf("unsafe.Offsetof const initializer must not emit runtime wrappers:\n%s", rust)
	}
}

func TestConstDeclUsesUntypedBoolTypeInfo(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const (
	IsArmbe = 0
	IsArm64be = 0
	IsMips = 0
	IsMips64 = 0
	IsPpc = 0
	IsPpc64 = 0
	IsS390 = 0
	IsS390x = 0
	IsSparc = 0
	IsSparc64 = 0
	BigEndian = IsArmbe|IsArm64be|IsMips|IsMips64|IsPpc|IsPpc64|IsS390|IsS390x|IsSparc|IsSparc64 == 1
)
`)

	if !strings.Contains(rust, "pub const BIG_ENDIAN: bool =") {
		t.Fatalf("untyped boolean const should emit bool type:\n%s", rust)
	}
	if strings.Contains(rust, "pub const BIG_ENDIAN: i32 =") {
		t.Fatalf("untyped boolean const must not emit integer type:\n%s", rust)
	}
}

func TestTypedVarInitializerCastsUntypedConstPeer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const PrimeRK = 16777619

func hash() uint32 {
	var pow, sq uint32 = 1, PrimeRK
	return pow + sq
}
`)

	if !strings.Contains(rust, "Some(PRIME_R_K as u32)") {
		t.Fatalf("typed uint32 var initializer should cast untyped const to the LHS type:\n%s", rust)
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

func TestPackageConstNamedIntegerConversionUsesConstName(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type ctrlGroup uint64

const bitsetEmpty = 7

func (g *ctrlGroup) setEmpty() {
	*g = ctrlGroup(bitsetEmpty)
}
`)

	if !strings.Contains(rust, "BITSET_EMPTY") {
		t.Fatalf("package const conversion should use generated Rust const name:\n%s", rust)
	}
	if strings.Contains(rust, "bitsetEmpty.lock") {
		t.Fatalf("package const conversion should not treat the const as a wrapped variable:\n%s", rust)
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

func transpileTypedRegression(t *testing.T, src string) string {
	t.Helper()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	return transpileParsedRegression(t, file, fset, typeInfo)
}

func transpileRegression(t *testing.T, src string, typeInfo *TypeInfo) string {
	t.Helper()

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	return transpileParsedRegression(t, file, fset, typeInfo)
}

func transpileParsedRegression(t *testing.T, file *ast.File, fset *token.FileSet, typeInfo *TypeInfo) string {
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
	prevCurrentReceiverObject := currentReceiverObject
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
		currentReceiverObject = prevCurrentReceiverObject
		currentReceiverType = prevCurrentReceiverType
		currentTypeMethods = prevCurrentTypeMethods
		hasInitFunction = prevHasInitFunction
	})

	currentTypeInfo = typeInfo
	currentContext = nil
	SetVarTable(nil)

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

func TestSwitchOnImportedConstSelectorDoesNotUnwrapField(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "go/token"

func classify() string {
	switch token.ILLEGAL {
	case token.ILLEGAL:
		return "illegal"
	}
	return "other"
}
`)

	if strings.Contains(rust, "ILLEGAL; let __owned = (*__v") {
		t.Fatalf("switch tag package constant selector should not be treated as a wrapped field:\n%s", rust)
	}
}

func TestSwitchOnWrappedUintptrFieldUnwrapsFieldHandleOnce(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type arrayType struct {
	Len uintptr
}

func classify(a *arrayType) int {
	switch a.Len {
	case 0:
		return 1
	}
	return 2
}
`)

	if strings.Contains(rust, "let __v = (*{ let __field = (*a.borrow().as_ref().unwrap()).len.clone(); __field }.borrow().as_ref().unwrap())") {
		t.Fatalf("switch tag field should not unwrap the field before borrowing its handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __v = (*a.borrow().as_ref().unwrap()).len.clone()") {
		t.Fatalf("switch tag field should clone the wrapped field handle before borrowing:\n%s", rust)
	}
}

func TestNoTypeInfoLocalConstByteFieldAssignmentDoesNotSynthesizeCast(t *testing.T) {
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
	assertLocalConstByteFieldEmission(t, transpileNoTypeInfoRegression(t, src))
	assertLocalConstByteFieldEmission(t, transpileRegression(t, src, &TypeInfo{}))
}

func assertLocalConstByteFieldEmission(t *testing.T, rust string) {
	t.Helper()
	if !strings.Contains(rust, "color: Rc::new(RefCell::new(Some(white as u8)))") {
		t.Fatalf("struct field literal should cast local const to byte field type:\n%s", rust)
	}
	if !strings.Contains(rust, "== white as u8") {
		t.Fatalf("selector comparison should cast local const to byte field type:\n%s", rust)
	}
	// AGENTS.md "Type Info Is Authoritative": the selector assignment must not
	// synthesize an `as u8` cast from struct field syntax when type info is
	// missing. The previous writeByteConstAssignmentValue syntax fallback
	// produced `let new_val = grey as u8` by routing through structDefs;
	// that branch was added in commit 50541a50 inside the 470fcb0b..3e3d9fc3
	// fallback-incident range and is gone now.
	if strings.Contains(rust, "let new_val = grey as u8") || strings.Contains(rust, "let new_val = black as u8") {
		t.Fatalf("selector assignment must not synthesize as-u8 cast from struct field syntax when type info is missing:\n%s", rust)
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

func TestConstBinaryPeerCastsUntypedConstIdentifierToUint64(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const (
	ctrlEmpty = 0b10000000
	bitsetLSB = 0x0101010101010101
	ctrlWord = bitsetLSB * uint64(ctrlEmpty)
)
`)

	if strings.Contains(rust, "BITSET_L_S_B * (CTRL_EMPTY as u64)") {
		t.Fatalf("const binary expression should not multiply signed inferred const by uint64 conversion:\n%s", rust)
	}
	if !strings.Contains(rust, "BITSET_L_S_B as u64 * (CTRL_EMPTY as u64)") {
		t.Fatalf("const binary expression should cast untyped const identifier to uint64 peer:\n%s", rust)
	}
}

func TestUntypedHexConstAboveInt64UsesUint64(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const bitsetMSB = 0x8080808080808080
`)

	if strings.Contains(rust, "BITSET_M_S_B: i32") {
		t.Fatalf("large positive hex constant should not default to i32:\n%s", rust)
	}
	if !strings.Contains(rust, "BITSET_M_S_B: u64 = 0x8080808080808080") {
		t.Fatalf("large positive hex constant should use u64:\n%s", rust)
	}
}

func TestConstBinaryPeerKeepsNamedIntegerConstsPrimitive(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type ChanDir int

const (
	RecvDir ChanDir = 1 << iota
	SendDir
	BothDir = RecvDir | SendDir
)
`)

	if strings.Contains(rust, "BOTH_DIR: i32 = ChanDir(") {
		t.Fatalf("const binary expression should not construct named integer wrappers:\n%s", rust)
	}
	if !strings.Contains(rust, "BOTH_DIR: i32 = RECV_DIR as i32 | SEND_DIR as i32") {
		t.Fatalf("named integer const operands should be emitted as primitive const values:\n%s", rust)
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

func TestNamedIntegerShortDeclAndIncDecPreserveNamedValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Token int

const (
	keywordBeg Token = 1
	keywordEnd Token = 3
)

func fill(tokens []string, keywords map[string]Token) {
	for i := keywordBeg + 1; i < keywordEnd; i++ {
		keywords[tokens[i]] = i
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

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "let mut i = Rc::new(RefCell::new(Some({") ||
		strings.Contains(rust, "let mut i = Arc::new(Mutex::new(Some({") {
		t.Fatalf("short declaration from named integer arithmetic must store the named value, not the raw scalar:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut i = Rc::new(RefCell::new(Some(Token(") &&
		!strings.Contains(rust, "let mut i = Arc::new(Mutex::new(Some(Token(") {
		t.Fatalf("short declaration from named integer arithmetic did not wrap Token:\n%s", rust)
	}
	if strings.Contains(rust, "*guard = Some(guard.as_ref().unwrap() + 1)") ||
		strings.Contains(rust, "*guard = Some(guard.as_ref().unwrap() - 1)") {
		t.Fatalf("inc/dec on named integers must preserve the named value:\n%s", rust)
	}
	if !strings.Contains(rust, "*guard = Some(Token(") {
		t.Fatalf("inc/dec on named integers did not write back Token:\n%s", rust)
	}
}

func TestNamedIntegerShortDeclFromConstIdentPreservesNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type StepKind int

const (
	StepBad StepKind = iota
	StepPointer
)

func f(flag bool) StepKind {
	kind := StepBad
	if flag {
		kind = StepPointer
	}
	return kind
}
`)

	if strings.Contains(rust, "let mut kind = Rc::new(RefCell::new(Some(STEP_BAD)))") ||
		strings.Contains(rust, "let mut kind = Arc::new(Mutex::new(Some(STEP_BAD)))") {
		t.Fatalf("short declaration from named integer const ident must not store the raw const:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut kind = Rc::new(RefCell::new(Some(StepKind(") &&
		!strings.Contains(rust, "let mut kind = Arc::new(Mutex::new(Some(StepKind(") {
		t.Fatalf("short declaration from named integer const ident should wrap StepKind:\n%s", rust)
	}
}

func TestNamedIntegerMixedPrimitiveOpsReturnNamedType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type bitset uint64

func (b bitset) removeFirst() bitset {
	return b & (b - 1)
}
`)

	if strings.Contains(rust, "impl std::ops::Sub<u64> for bitset {\n    type Output = u64") {
		t.Fatalf("named integer minus primitive should preserve the named type:\n%s", rust)
	}
	if !strings.Contains(rust, "impl std::ops::Sub<u64> for bitset {\n    type Output = bitset") {
		t.Fatalf("named integer minus primitive should output bitset:\n%s", rust)
	}
	if strings.Contains(rust, "impl std::ops::BitAnd<u64> for bitset {\n    type Output = u64") {
		t.Fatalf("named integer bitwise primitive op should preserve the named type:\n%s", rust)
	}
	if !strings.Contains(rust, "impl std::ops::BitAnd<u64> for bitset {\n    type Output = bitset") {
		t.Fatalf("named integer bitwise primitive op should output bitset:\n%s", rust)
	}
}

func TestNamedIntegerReturnSameTypedValueDoesNotReconstruct(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type ctrl uint8

func read(p *ctrl) ctrl {
	return *p
}
`)

	if strings.Contains(rust, " as u8") {
		t.Fatalf("returning a value already typed as ctrl should not cast ctrl back to u8:\n%s", rust)
	}
}

func TestNamedIntegerConstReturnWrapsLocalPackageConst(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type ChanDir int

const (
	RecvDir    ChanDir = 1 << iota
	SendDir
	BothDir            = RecvDir | SendDir
	InvalidDir ChanDir = 0
)

type Type struct{}

func (t *Type) ChanDir() ChanDir {
	return InvalidDir
}
`)

	if strings.Contains(rust, "Some(INVALID_DIR)") {
		t.Fatalf("named integer const returned through wrapped result must construct the named type:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(ChanDir(") {
		t.Fatalf("named integer const returned through wrapped result should be wrapped as ChanDir:\n%s", rust)
	}
}

func TestNamedIntegerBitClearConversionOperandStaysBare(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type bitset uint64

func clearBelow(b bitset, mask uint64) bitset {
	return b &^ bitset(mask)
}
`)

	if strings.Contains(rust, "(*bitset(") {
		t.Fatalf("named integer conversion in binary return should not be borrowed as a wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "impl std::ops::Not for bitset") {
		t.Fatalf("named integer bit-clear should have a Not impl for the named type:\n%s", rust)
	}
}

func TestNamedIntegerBitwiseConstShiftOperandStaysPrimitive(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type bitset uint64

func start() {
	go func() {}()
}

func lowestSet(b bitset) bool {
	return b&(1<<7) != 0
}
`)

	if strings.Contains(rust, "(1 << 7).0") {
		t.Fatalf("constant shift operand should stay primitive inside named integer bitwise op:\n%s", rust)
	}
}

func TestNamedIntegerShiftOpsPreserveNamedType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type bitset uint64

func shiftConst(b bitset) bitset {
	return b >> 8
}

func shiftCount(b bitset, n uint) bitset {
	return b << n
}
`)

	for _, want := range []string{
		"impl std::ops::Shr<i32> for bitset {\n    type Output = bitset",
		"impl std::ops::Shl<u32> for bitset {\n    type Output = bitset",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("named integer shift op should preserve named type, missing %q:\n%s", want, rust)
		}
	}
}

func TestNamedIntegerShiftAssignClonesCurrentValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type ctrlGroup uint64

func shift(ctrls ctrlGroup) ctrlGroup {
	ctrls >>= 8
	return ctrls
}
`)

	if strings.Contains(rust, "guard.as_ref().unwrap() >> __rhs") {
		t.Fatalf("named integer shift assignment should shift the owned named value, not a reference:\n%s", rust)
	}
	if !strings.Contains(rust, "guard.as_ref().unwrap().clone() >> __rhs") {
		t.Fatalf("named integer shift assignment should clone the current named value:\n%s", rust)
	}
}

func TestShiftLeftUntypedConstantDoesNotUseCountType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type table struct {
	localDepth uint8
}

type Map struct {
	globalDepth uint8
}

func (m *Map) entries(nt *table) int {
	entries := 1 << (m.globalDepth - nt.localDepth)
	return entries
}
`)

	if strings.Contains(rust, "1 as u8 <<") {
		t.Fatalf("left operand of shift should not be cast to the shift count type:\n%s", rust)
	}
}

func TestNamedIntegerConversionShiftLeftOperandIsParenthesized(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type flag uintptr

const (
	flagMethod      flag = 1 << 9
	flagMethodShift      = 10
)

func methodFlag(i int) flag {
	var fl flag
	fl |= flag(i)<<flagMethodShift | flagMethod
	return fl
}
`)

	if strings.Contains(rust, "as usize <<") {
		t.Fatalf("named integer conversion used as shift left operand must be parenthesized:\n%s", rust)
	}
	if !strings.Contains(rust, ") as usize) << FLAG_METHOD_SHIFT") {
		t.Fatalf("named integer conversion shift left operand should keep the cast grouped:\n%s", rust)
	}
}

func TestNamedIntegerAssignmentUsesRawNamedValue(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "time"

func main() {
	delay := 0 * time.Nanosecond
	delay = 30 * time.Second
	_ = delay
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := Transpile(file, fset, typeInfo)
	if strings.Contains(rust, "let new_val = Rc::new(RefCell::new(Some(std::time::Duration::from_secs(30))))") ||
		strings.Contains(rust, "let new_val = Arc::new(Mutex::new(Some(std::time::Duration::from_secs(30))))") {
		t.Fatalf("assignment to named integer must write the raw named value, not a nested wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = std::time::Duration::from_secs(30)") {
		t.Fatalf("assignment to time.Duration did not emit a raw duration value:\n%s", rust)
	}
}

func TestCompoundAssignSelectorConstCastsToExpectedUintptr(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "unicode/utf8"

func f() uintptr {
	spill := uintptr(0)
	spill += utf8.RuneSelf
	return spill
}
`)

	if !strings.Contains(rust, "RUNE_SELF as usize") {
		t.Fatalf("selector constant compound assignment should cast to uintptr/usize:\n%s", rust)
	}
}

func TestCompoundAssignBareScalarLocalUsesRawValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type seq struct {
	stackBytes uintptr
}

func align(x uintptr) uintptr { return x }

func adjust(out *seq, in seq) {
	retOffset := align(in.stackBytes)
	out.stackBytes = retOffset
	out.stackBytes -= retOffset
}
`)

	if strings.Contains(rust, "retOffset.borrow()") || strings.Contains(rust, "retOffset.lock()") {
		t.Fatalf("bare scalar compound assignment RHS should not be unwrapped as a handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __rhs = retOffset") {
		t.Fatalf("bare scalar compound assignment RHS should use the raw local:\n%s", rust)
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

// Coverage matrix for the named-type-const short-circuit at
// `writeExpressionForExpectedTypesType` (`go/expr.go`). Each case differs in
// where the named-type constant comes from; the four variants together pin
// the three iterations of behavior tweaks (commits 36b814b5, ce74f893,
// d9405f00) so future tweaks fail loudly.

func TestNamedTypeConstReturnLocalConst(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Kind int

const Field Kind = 1

func get() Kind {
	return Field
}
`)

	// Local typed constants are emitted as bare i32. The return slot is
	// `Rc<RefCell<Option<Kind>>>`, so the const must be wrapped in
	// `Kind(...)` newtype to match.
	if strings.Contains(rust, "Some(FIELD)") || strings.Contains(rust, "Some(Field)") {
		t.Fatalf("local typed const return must construct the named type, not assign bare:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(Kind(") {
		t.Fatalf("local typed const return should wrap into the named newtype:\n%s", rust)
	}
}

// Cross-package coverage lives in the `tests/package_named_const_argument/`
// fixture, which exercises a user-package constant arg through the full
// transpile pipeline. The in-memory helper here can't load
// `example.com/<pkg>` paths, so we rely on the fixture for that variant.

func TestNamedTypeConstReturnStdlibStubConst(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "go/types"

func chanDir() types.ChanDir {
	return types.SendRecv
}
`)

	// Stdlib-stub-qualified constants of the same named type are emitted
	// by the external stub generator with the named Rust type
	// (`types::SEND_RECV: types_ChanDir`). They don't need the constructor
	// + as-int rewrap.
	chanDirIdx := strings.Index(rust, "pub fn chan_dir")
	if chanDirIdx < 0 {
		t.Fatalf("expected chan_dir function in output:\n%s", rust)
	}
	chanDirRust := rust[chanDirIdx:]
	if strings.Contains(chanDirRust, "Some(types_ChanDir(") {
		t.Fatalf("stdlib stub const already typed as the named type should not be re-wrapped inside chan_dir:\n%s", chanDirRust)
	}
	if !strings.Contains(chanDirRust, "Some(types::SEND_RECV)") {
		t.Fatalf("stdlib stub const should be assigned directly into the wrapped slot:\n%s", chanDirRust)
	}
}
