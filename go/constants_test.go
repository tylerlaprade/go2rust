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

func TestStringConstAliasUsesConstantValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const defaultGOEXPERIMENT = ""
const DefaultGOEXPERIMENT = defaultGOEXPERIMENT
`)

	if strings.Contains(rust, "pub const DEFAULT_G_O_E_X_P_E_R_I_M_E_N_T: &'static str = DEFAULT_G_O_E_X_P_E_R_I_M_E_N_T;") {
		t.Fatalf("string const alias should not self-reference after Rust name normalization:\n%s", rust)
	}
	if !strings.Contains(rust, `pub const DEFAULT_G_O_E_X_P_E_R_I_M_E_N_T: &'static str = "";`) {
		t.Fatalf("string const alias should use the go/types constant value:\n%s", rust)
	}
}

func TestConstShiftCountNamedIntegerUsesUnderlyingValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "math/big"

const (
	_m = ^big.Word(0)
	_log = _m>>8&1 + _m>>16&1 + _m>>32&1
	wordSize = 1 << _log
)
`)

	if strings.Contains(rust, "1 << _LOG") && !strings.Contains(rust, "1 << _LOG.0") {
		t.Fatalf("shift count with named integer type should use the underlying scalar value:\n%s", rust)
	}
	if strings.Contains(rust, "_M >> 8 as u64 & 1 as u64 as u64 + _M >>") {
		t.Fatalf("bitwise terms summed in const expressions must stay parenthesized for Rust precedence:\n%s", rust)
	}
	if !strings.Contains(rust, "WORD_SIZE") {
		t.Fatalf("test fixture should emit wordSize const:\n%s", rust)
	}
}

func TestConstBitwiseTermsAreParenthesizedBeforeAddition(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint

const (
	_m = ^Word(0)
	_log = _m>>8&1 + _m>>16&1 + _m>>32&1
)
`)

	if strings.Contains(rust, "_M >> 8 as u64 & 1 as u64 as u64 + _M >>") {
		t.Fatalf("bitwise terms summed in const expressions must stay parenthesized for Rust precedence:\n%s", rust)
	}
}

func TestUntypedShiftConstAtUintWidthUsesWideRustStorage(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint

const (
	W = 64
	B = 1 << W
	M = B - 1
)

func mask() Word {
	return Word(M)
}
`)

	if strings.Contains(rust, "const B: i64 = 1 << 64") {
		t.Fatalf("untyped shift const at Go uint width should not use overflowing i64 storage:\n%s", rust)
	}
	if strings.Contains(rust, "const M: i32 = B - 1") || strings.Contains(rust, "const M: u64 = B - 1") {
		t.Fatalf("derived untyped integer const should use storage compatible with its wide source expression:\n%s", rust)
	}
	if !strings.Contains(rust, "const B: u128 = 1 << W") {
		t.Fatalf("wide untyped shift const should use u128 storage:\n%s", rust)
	}
	if !strings.Contains(rust, "const M: u128 = B - 1") {
		t.Fatalf("derived wide untyped const should preserve the source expression in compatible storage:\n%s", rust)
	}
}

func TestConstNameCollisionDisambiguatesPackageConstants(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const uintSize = 32 << (^uint(0) >> 63)
const UintSize = uintSize
`)

	if count := strings.Count(rust, "const UINT_SIZE:"); count != 1 {
		t.Fatalf("Rust const name collision should leave only one UINT_SIZE declaration, got %d:\n%s", count, rust)
	}
	if !strings.Contains(rust, "pub(crate) const UINT_SIZE_1:") {
		t.Fatalf("unexported colliding constant should receive a deterministic suffix:\n%s", rust)
	}
	if strings.Contains(rust, "pub(crate) const UINT_SIZE_1: u64") {
		t.Fatalf("typed uint shift-count intermediates should not force the final untyped const to u64 storage:\n%s", rust)
	}
	if !strings.Contains(rust, "pub const UINT_SIZE: i32 = UINT_SIZE_1;") {
		t.Fatalf("exported const alias should refer to the renamed unexported const:\n%s", rust)
	}
}

func TestGeneratedStringTableConstConcatenationStaysStr(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const ntz8tab = "" +
	"\x08\x00\x01\x00" +
	"\x02\x00\x01\x00"
`)

	if !strings.Contains(rust, "pub(crate) const NTZ8TAB: &'static str =") {
		t.Fatalf("string table const should be emitted as &'static str:\n%s", rust)
	}
	if strings.Contains(rust, ".to_string() +") || strings.Contains(rust, "String::from") {
		t.Fatalf("string table const concatenation must be folded to a string literal, not runtime String concatenation:\n%s", rust)
	}
}

func TestInvalidStringTableConstUsesByteSlice(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const rev8tab = "\x00\x80\xff"

func lookup(i int) byte {
	return rev8tab[i]
}
`)

	if !strings.Contains(rust, "pub(crate) const REV8TAB: &'static [u8] = &[0x0u8, 0x80u8, 0xffu8];") {
		t.Fatalf("non-UTF-8 string table should be emitted as a byte slice const:\n%s", rust)
	}
	if strings.Contains(rust, "String::from_utf8_lossy") {
		t.Fatalf("non-UTF-8 string table must preserve bytes instead of using lossy conversion:\n%s", rust)
	}
	if strings.Contains(rust, "REV8TAB); __s.as_bytes()[") {
		t.Fatalf("byte-slice string table index should not call as_bytes on the generated byte slice:\n%s", rust)
	}
}

func TestLocalExportedConstReferenceKeepsLocalName(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func normalize(x float64) bool {
	const SmallestNormal = 2.2250738585072014e-308
	return x < SmallestNormal
}
`)

	if strings.Contains(rust, "SMALLEST_NORMAL") {
		t.Fatalf("local const reference should not use package const naming:\n%s", rust)
	}
	if !strings.Contains(rust, "const SmallestNormal: f64 =") {
		t.Fatalf("local const declaration should keep the Go local const name:\n%s", rust)
	}
	if !strings.Contains(rust, "< SmallestNormal") && !strings.Contains(rust, "let __tmp_y = SmallestNormal") {
		t.Fatalf("local const reference should use the local declaration name:\n%s", rust)
	}
}

func TestUntypedFloatConstObjectUsesF64(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const (
	Ln2 = 0.693147180559945309417232121458176568075500134360255254120680009
	Log2E = 1 / Ln2
	Underflow = -7.45133219101941108420e+02
)
`)

	if !strings.Contains(rust, "pub const LOG2_E: f64 = 1.0 / LN2;") {
		t.Fatalf("untyped float const expression should infer f64:\n%s", rust)
	}
	if !strings.Contains(rust, "pub const UNDERFLOW: f64 = -7.45133219101941108420e+02;") {
		t.Fatalf("unary untyped float const should infer f64:\n%s", rust)
	}
	if strings.Contains(rust, "LOG2_E: i32") || strings.Contains(rust, "UNDERFLOW: i32") {
		t.Fatalf("untyped float constants must not fall back to i32:\n%s", rust)
	}
}

func TestGoFloatLiteralSyntaxLowersToRustFloatLiteral(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const MaxFloat64 = 0x1p1023 * (1 + (1 - 0x1p-52))

func threshold(x float64) bool {
	return x < .5
}
`)

	if strings.Contains(rust, "0x1p") || strings.Contains(rust, "0x1P") {
		t.Fatalf("Go hexadecimal float literals should not be emitted as raw Rust literals:\n%s", rust)
	}
	if strings.Contains(rust, " .5") || strings.Contains(rust, "= .5") {
		t.Fatalf("Go float literals without integer part should be normalized for Rust:\n%s", rust)
	}
	if !strings.Contains(rust, "pub const MAX_FLOAT64: f64 =") {
		t.Fatalf("hexadecimal float const should infer f64:\n%s", rust)
	}
	if !strings.Contains(rust, "< 0.5") && !strings.Contains(rust, "let __tmp_y = 0.5") {
		t.Fatalf("decimal float literal without integer part should become 0.5:\n%s", rust)
	}
}

func TestUntypedIntegerConstExpressionCastsForFloatPeer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func scale(x float64) float64 {
	return x * (1 << 52)
}

func isNegOne(x float64) bool {
	return x == -1
}
`)

	if strings.Contains(rust, "* ({ let __tmp_x = 1; let __tmp_y = 52; __tmp_x << __tmp_y })") {
		t.Fatalf("integer const expression used with float peer should not stay integer-typed:\n%s", rust)
	}
	if !strings.Contains(rust, "* 4503599627370496.0") && !strings.Contains(rust, "* 4.503599627370496e+15") {
		t.Fatalf("integer const expression used with float peer should lower to a float literal:\n%s", rust)
	}
	if !strings.Contains(rust, "let __tmp_y = -1.0") && !strings.Contains(rust, "== -1.0") {
		t.Fatalf("integer const comparison peer should lower to a float literal:\n%s", rust)
	}
}

func TestUntypedFloatLiteralConstUsesIntegerValueForIntPeer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func overflow(n int) bool {
	return n >= 1e8
}
`)

	if strings.Contains(rust, "1e8") {
		t.Fatalf("untyped numeric constant used with int peer should not remain a Rust float literal:\n%s", rust)
	}
	if !strings.Contains(rust, "100000000") {
		t.Fatalf("untyped numeric constant used with int peer should emit its exact integer value:\n%s", rust)
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

func TestIntegerTypedFloatLiteralConstEmitsIntegerLiteral(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

const maxNestLev int = 1e5
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	prevTypeInfo := currentTypeInfo
	prevPackageConstants := packageConstants
	defer func() {
		currentTypeInfo = prevTypeInfo
		packageConstants = prevPackageConstants
	}()

	SetTypeInfo(typeInfo)
	packageConstants = make(map[string]string)

	genDecl := file.Decls[0].(*ast.GenDecl)
	var out strings.Builder
	TranspileConstDecl(&out, genDecl)

	rust := out.String()
	if !strings.Contains(rust, "MAX_NEST_LEV: i32 = 100000;") {
		t.Fatalf("integer-typed float literal const should emit an integer literal, got:\n%s", rust)
	}
	if strings.Contains(rust, "1e5") || strings.Contains(rust, "100000.0") {
		t.Fatalf("integer-typed float literal const should not emit a Rust float literal, got:\n%s", rust)
	}
}

func TestUntypedConstLenStringEmitsGoIntCast(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const dots = ". . . "
const n = len(dots)
`)

	if !strings.Contains(rust, "const N: i32 = DOTS.len() as i32;") {
		t.Fatalf("untyped len const should cast Rust usize length to Go int, got:\n%s", rust)
	}
	if strings.Contains(rust, "const N: i32 = DOTS.len();") {
		t.Fatalf("untyped len const should not assign usize length directly to i32:\n%s", rust)
	}
}

func TestLenOfStringConstUsesConstValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const repeated = "xxxxxxxx"

func within(n int) bool {
	return n <= len(repeated)
}
`)

	if strings.Contains(rust, "REPEATED.borrow()") || strings.Contains(rust, "REPEATED.lock()") {
		t.Fatalf("len(string const) should not treat the const as a wrapped handle:\n%s", rust)
	}
	if !strings.Contains(rust, "REPEATED.len() as i32") {
		t.Fatalf("len(string const) should use the Rust const string length as Go int:\n%s", rust)
	}
}

func TestConstStringEqualityUsesPatternMatch(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const GOOS = "darwin"
const sizeofPtr = 8
const darwin64Bit = (GOOS == "darwin" || GOOS == "ios") && sizeofPtr == 8
`)

	if strings.Contains(rust, `G_O_O_S == "darwin"`) ||
		strings.Contains(rust, `G_O_O_S == "ios"`) {
		t.Fatalf("const string equality should not use non-const str equality:\n%s", rust)
	}
	if !strings.Contains(rust, `matches!(G_O_O_S, "darwin")`) ||
		!strings.Contains(rust, `matches!(G_O_O_S, "ios")`) {
		t.Fatalf("const string equality should use pattern matches:\n%s", rust)
	}
}

func TestStubBackedStdlibStringConstEqualityUsesTypeInfoValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "runtime"

const hostDarwin = runtime.GOOS == "darwin"
`)

	if strings.Contains(rust, `matches!(runtime::G_O_O_S`) ||
		strings.Contains(rust, `runtime::G_O_O_S ==`) {
		t.Fatalf("stub-backed stdlib string const equality should not use the unusable generated selector:\n%s", rust)
	}
	if !strings.Contains(rust, "const HOST_DARWIN: bool = true;") &&
		!strings.Contains(rust, "const HOST_DARWIN: bool = false;") {
		t.Fatalf("stub-backed stdlib string const equality should use the go/types value:\n%s", rust)
	}
}

func TestConstTypeConversionCastsWholeBinaryExpression(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const n = uint64(2 - 1)
`)

	if strings.Contains(rust, "2 - 1 as u64") {
		t.Fatalf("const type conversion should not cast only the binary RHS:\n%s", rust)
	}
	if !strings.Contains(rust, "const N: u64 = ((2 - 1) as u64);") {
		t.Fatalf("const type conversion should cast the whole binary expression:\n%s", rust)
	}
}

func TestConstLenArrayVariableUsesTypedLength(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

var table = [3]uint64{1, 5, 25}
const m = uint64(len(table) - 1)
`)

	if strings.Contains(rust, "TABLE.borrow") || strings.Contains(rust, "TABLE.lock") {
		t.Fatalf("const len(array variable) should use the go/types array length, not runtime wrapper access:\n%s", rust)
	}
	if strings.Contains(rust, "len() - 1 as u64") {
		t.Fatalf("const len(array variable) conversion should not cast only the binary RHS:\n%s", rust)
	}
	if !strings.Contains(rust, "const M: u64 = ((3 - 1) as u64);") {
		t.Fatalf("const len(array variable) conversion should cast the typed length expression:\n%s", rust)
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

func transpileTypedConcurrentRegression(t *testing.T, src string) string {
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
	prevConcurrencyDetector := globalConcurrencyDetector
	cd := NewConcurrencyDetector()
	cd.AnalyzeProject([]*ast.File{file})
	SetConcurrencyDetector(cd)
	t.Cleanup(func() {
		SetConcurrencyDetector(prevConcurrencyDetector)
	})
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
	prevPackageGlobalNameOverrides := packageGlobalNameOverrides
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
		packageGlobalNameOverrides = prevPackageGlobalNameOverrides
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

func TestConstIotaBinaryWithNamedIntegerPeer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Op uint8

const (
	opPseudo Op = 128 + iota
	opLeftParen = opPseudo + iota
	opVerticalBar
)
`)

	if strings.Contains(rust, "iota") {
		t.Fatalf("const expressions should substitute iota through named integer peers:\n%s", rust)
	}
	if !strings.Contains(rust, "OP_LEFT_PAREN: u8 = OP_PSEUDO as u8 + 1 as u8") {
		t.Fatalf("opLeftParen should use the const block iota value:\n%s", rust)
	}
	if !strings.Contains(rust, "OP_VERTICAL_BAR: u8 = OP_PSEUDO as u8 + 2 as u8") {
		t.Fatalf("opVerticalBar should reuse the const expression with its own iota value:\n%s", rust)
	}
}

func TestNamedStringConstCompositeLiteralUsesConstName(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type ErrorCode string

type Error struct {
	Code ErrorCode
}

const ErrInvalid ErrorCode = "invalid"

func makeError() Error {
	return Error{Code: ErrInvalid}
}
`)

	if strings.Contains(rust, "ErrInvalid.borrow()") || strings.Contains(rust, "ErrInvalid.lock()") {
		t.Fatalf("named string const composite value should not be treated as a wrapped local:\n%s", rust)
	}
	if !strings.Contains(rust, "ERR_INVALID.to_string()") {
		t.Fatalf("named string const composite value should use the registered Rust const:\n%s", rust)
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
	if !strings.Contains(rust, "*guard = Some(Token(") &&
		!strings.Contains(rust, "*guard = Some(guard.as_ref().unwrap().clone() + 1 as i32)") {
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

func TestNamedIntegerMulDivRemAndCharConstPreserveNamedType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint

func digit(n, ch Word) Word {
	return n*10 + ch - '0'
}

func split(n, ch Word) Word {
	return n/ch + n%ch
}

func emit(d Word) byte {
	return byte(d + '0')
}
`)

	for _, trait := range []string{"Mul", "Div", "Rem"} {
		want := "impl std::ops::" + trait + " for Word {\n    type Output = Word"
		if !strings.Contains(rust, want) {
			t.Fatalf("named integer %s op should preserve named type, missing %q:\n%s", trait, want, rust)
		}
	}
	if strings.Contains(rust, "+ '0'") || strings.Contains(rust, "- '0'") {
		t.Fatalf("named integer arithmetic should cast char constants to the named type's primitive peer:\n%s", rust)
	}
	if !strings.Contains(rust, "('0' as u64)") {
		t.Fatalf("named integer char constant should use the underlying primitive type:\n%s", rust)
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

func TestNamedIntegerPrimitiveConversionUsesInnerValue(t *testing.T) {
	src := `package main

type Word uint

func widen(w Word) uint64 {
	return uint64(w)
}

func mask(s uint) uint {
	return uint(Word(1)<<s - 1)
}

func digit(n Word, s uint) byte {
	for n>>s == 0 {
		n *= 10
	}
	mask := Word(1)<<s - 1
	d := n >> s
	n &= mask
	return byte(d + '0')
}

func forceConcurrent() {
	go func() {}()
}
`

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	prevDetector := GetConcurrencyDetector()
	detector := NewConcurrencyDetector()
	detector.AnalyzeFile(file)
	SetConcurrencyDetector(detector)
	defer SetConcurrencyDetector(prevDetector)

	rust := transpileParsedRegression(t, file, fset, typeInfo)
	if strings.Contains(rust, "Word(Arc::new(Mutex::new(Some(1 as u64)))) as u64") ||
		strings.Contains(rust, "Word(Rc::new(RefCell::new(Some(1 as u64)))) as u64") {
		t.Fatalf("primitive conversion of named integer values must cast the inner scalar, not the named wrapper:\n%s", rust)
	}
}

func TestNamedIntegerConversionSliceElementAssignmentStoresNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func set(z nat, x uint64) {
	z[1] = Word(x >> 32)
	z[0] = Word(x)
}
`)

	if strings.Contains(rust, "Word(Rc::new(RefCell::new(Some(") &&
		strings.Contains(rust, ")).borrow().as_ref().unwrap()") {
		t.Fatalf("named integer conversion assigned into slice element should not be borrowed as a wrapper:\n%s", rust)
	}
	if strings.Contains(rust, "Word(Arc::new(Mutex::new(Some(") &&
		strings.Contains(rust, ")).lock().unwrap().as_ref().unwrap()") {
		t.Fatalf("named integer conversion assigned into slice element should not be locked as a wrapper:\n%s", rust)
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

func TestNamedIntegerConstBinaryAssignmentCastsWholeExpression(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint

const MaxBase = 10 + ('z' - 'a' + 1)

func set() Word {
	var d Word
	d = MaxBase + 1
	return d
}
`)

	if strings.Contains(rust, "MAX_BASE + 1 as u64") {
		t.Fatalf("named integer const assignment should not cast only the binary RHS:\n%s", rust)
	}
	if !strings.Contains(rust, "((MAX_BASE + 1) as u64)") {
		t.Fatalf("named integer const assignment should cast the whole expression:\n%s", rust)
	}
}

func TestNamedIntegerBinaryConstIdentOperandCastsToPrimitivePeer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint

const M = 1<<64 - 1

func max(b Word) bool {
	max := M / b
	return max >= b
}
`)

	if strings.Contains(rust, "M / (*") {
		t.Fatalf("named integer binary const identifier should not keep its inferred const type:\n%s", rust)
	}
	if !strings.Contains(rust, "M as u64 / (*") {
		t.Fatalf("named integer binary const identifier should cast to the named primitive peer:\n%s", rust)
	}
}

func TestPrimitiveShiftCountNamedIntegerUsesInnerValue(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Word uint

const primeBitMask uint64 = 1<<2 | 1<<3

func start() {
	go func() {}()
}

func small(w Word) bool {
	return primeBitMask&(1<<w) != 0
}
`)

	if strings.Contains(rust, "__tmp_y = { let __v = (*w.borrow().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y") ||
		strings.Contains(rust, "__tmp_y = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y") {
		t.Fatalf("primitive shift count should use the named integer inner value, not the wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "__tmp_y = (*{ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()); __tmp_x << __tmp_y") {
		t.Fatalf("primitive shift count should unwrap the named integer inner value:\n%s", rust)
	}
}

func TestNamedIntegerBitwiseConstExpressionCastsOperands(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint

const (
	primesA = 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23 * 37
	primesB = 29 * 31 * 41 * 43 * 47 * 53
	M = 1<<64 - 1
)

func use(w Word) {}

func masked() {
	use((primesA * primesB) & M)
}
`)

	if strings.Contains(rust, "(PRIMES_A * PRIMES_B) & M") {
		t.Fatalf("named integer bitwise const expression should not combine standalone const storage types:\n%s", rust)
	}
	if !strings.Contains(rust, "(PRIMES_A as u64) * (PRIMES_B as u64)") || !strings.Contains(rust, "(M as u64)") {
		t.Fatalf("named integer bitwise const expression should cast operands to the named primitive peer:\n%s", rust)
	}
}

func TestExpectedUint64ConstBinaryCastsLocalConstOperand(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "math"

func invalid(ln uint32) bool {
	const j = 1 + 4
	return uint64(ln) > math.MaxInt-j
}
`)

	if strings.Contains(rust, "math::MAX_INT; let __tmp_y = j; __tmp_x - __tmp_y") ||
		strings.Contains(rust, "math::MAX_INT - j") {
		t.Fatalf("uint64 comparison const expression should not subtract an i32 local const from MaxInt:\n%s", rust)
	}
	if !strings.Contains(rust, "(j as u64)") {
		t.Fatalf("uint64 comparison const expression should cast the local const operand:\n%s", rust)
	}
}

func TestTypedGoIntMaxConstUsesRustIntModel(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const maxInt = int(^uint(0) >> 1)

func over(n int) bool {
	return n > maxInt-1
}
`)

	if !strings.Contains(rust, "const MAX_INT: i32 = i32::MAX;") {
		t.Fatalf("typed Go int max constant should use the Rust Go-int model:\n%s", rust)
	}
	if strings.Contains(rust, "9223372036854775807") {
		t.Fatalf("typed Go int max constant should not inline host-width literal values:\n%s", rust)
	}
}

func TestExpectedUint64ConstBinaryWithNegativeOperandUsesSignedIntermediate(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

const (
	shift = 52
	bias = 1023
)

func setExponent(x uint64) uint64 {
	x |= (-1 + bias) << shift
	return x
}
`)

	if strings.Contains(rust, "-1 as u64") {
		t.Fatalf("unsigned const expression should not cast a negative operand to u64 before addition:\n%s", rust)
	}
	if !strings.Contains(rust, "-1 as i128") || !strings.Contains(rust, "BIAS as i128") {
		t.Fatalf("unsigned const expression with negative operand should use a signed intermediate:\n%s", rust)
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

func TestNamedIntegerShiftShortDeclStoresNamedValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Word uint
type nat []Word

func rounded(z nat, ntz uint32) bool {
	lsb := Word(1) << ntz
	return z[0]&lsb != 0
}
`)

	if strings.Contains(rust, "let mut lsb = Rc::new(RefCell::new(Some({") ||
		strings.Contains(rust, "let mut lsb = Arc::new(Mutex::new(Some({") {
		t.Fatalf("short declaration from named integer shift must store the named value, not the raw scalar:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut lsb = Rc::new(RefCell::new(Some(Word(") &&
		!strings.Contains(rust, "let mut lsb = Arc::new(Mutex::new(Some(Word(") {
		t.Fatalf("short declaration from named integer shift did not wrap Word:\n%s", rust)
	}
	if strings.Contains(rust, "1 as u64 as u32") {
		t.Fatalf("named integer shift left operand should not be cast to the count type:\n%s", rust)
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

func TestNamedIntegerPointerReceiverXorShiftAssignStoresUnderlyingValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type xorshift uint64

func (r *xorshift) Next() uint64 {
	*r ^= *r << 13
	return uint64(*r)
}
`)

	if strings.Contains(rust, "let __rhs = (*self).clone() <<") {
		t.Fatalf("named integer pointer receiver compound assignment should not keep the named RHS value:\n%s", rust)
	}
	if !strings.Contains(rust, "let __rhs = (*((*self).clone() <<") {
		t.Fatalf("named integer pointer receiver compound assignment should unwrap the named RHS value:\n%s", rust)
	}
}

func TestNamedIntegerCompoundAssignMethodCallRHSUsesWrappedResult(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type flag uintptr

func (f flag) ro() flag { return f }

type Value struct {
	flag flag
}

func set(dst *Value, src Value) {
	dst.flag |= src.flag.ro()
}
`)

	if strings.Contains(rust, ".ro()).0") || strings.Contains(rust, ".ro().0") {
		t.Fatalf("named integer method-call RHS already returns a wrapped value and should not be unwrapped as bare:\n%s", rust)
	}
	if !strings.Contains(rust, ".ro()") {
		t.Fatalf("named integer compound assignment should retain the method call RHS:\n%s", rust)
	}
}

func TestByteCompoundAssignCastsConstantExpression(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func upper(s string) byte {
	c := s[0]
	c -= 'a' - 'A'
	return c
}
`)

	if strings.Contains(rust, "let __rhs = { let __tmp_x = ('a' as i32); let __tmp_y = ('A' as i32); __tmp_x - __tmp_y }") {
		t.Fatalf("byte compound assignment should not use an i32 RHS constant expression:\n%s", rust)
	}
	if !strings.Contains(rust, "let __rhs = ") || !strings.Contains(rust, " as u8") {
		t.Fatalf("byte compound assignment should cast the RHS constant expression to u8:\n%s", rust)
	}
}

func TestShiftLeftUntypedConstantDoesNotUseCountType(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func start() {
	go func() {}()
}

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

	if strings.Contains(rust, "1 as u8") {
		t.Fatalf("left operand of shift should not be cast to the shift count type:\n%s", rust)
	}
}

func TestShiftLeftConvertedConstantUsesResultTypeInConcurrentTemp(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

func start() {
	go func() {}()
}

func localDepthMask(localDepth uint8) uintptr {
	return uintptr(1) << (64 - localDepth)
}
`)

	if strings.Contains(rust, "let __tmp_x = (*Arc::new(Mutex::new(Some(1 as usize))).lock().unwrap().as_ref().unwrap()) as u8") {
		t.Fatalf("converted shift left operand should not be cast to the shift count type:\n%s", rust)
	}
	if !strings.Contains(rust, "as usize") {
		t.Fatalf("converted shift left operand should use the uintptr result type:\n%s", rust)
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

func TestNamedIntegerAliasUnaryNotConversionUsesPrimitiveValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type fileMode uint32
type FileMode = fileMode

func unknown() FileMode {
	return ^FileMode(0)
}
`)

	if strings.Contains(rust, "!fileMode(") {
		t.Fatalf("unary-not of named integer alias conversion should not apply ! to the named wrapper:\n%s", rust)
	}
	if strings.Contains(rust, " as u32) as u32") {
		t.Fatalf("unary-not of named integer alias conversion should not cast a wrapped named value:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(!0 as u32)") {
		t.Fatalf("unary-not of named integer alias conversion should operate on the primitive value:\n%s", rust)
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

func TestTimeDurationConversionToInt64UsesNanoseconds(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "time"

func nanos(t time.Time) int64 {
	return int64(time.Until(t))
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
	if strings.Contains(rust, ".0 as i64") {
		t.Fatalf("conversion from time.Duration to int64 should not use tuple fields:\n%s", rust)
	}
	if !strings.Contains(rust, ".as_nanos() as i64") {
		t.Fatalf("conversion from time.Duration to int64 should use nanoseconds:\n%s", rust)
	}
}

func TestTimeDurationConversionFromIntegerUsesFromNanos(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "time"

func duration(n int64) time.Duration {
	return time.Duration(n) * time.Nanosecond
}
`)

	if strings.Contains(rust, "std::time::Duration(") {
		t.Fatalf("time.Duration conversion should not call std::time::Duration as a tuple struct:\n%s", rust)
	}
	if !strings.Contains(rust, "std::time::Duration::from_nanos(") {
		t.Fatalf("time.Duration conversion from integer should use from_nanos:\n%s", rust)
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

func TestNamedIntegerSelectorAndNotAssignUsesWrappedMutation(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type kind uint8

const direct kind = 1 << 5

type typ struct {
	kind_ kind
}

func clear(t *typ) {
	t.kind_ &^= direct
}
`)

	if !strings.Contains(rust, "let __rhs = kind(") {
		t.Fatalf("named integer &^= should construct a named RHS value:\n%s", rust)
	}
	if !strings.Contains(rust, "& ! __rhs") {
		t.Fatalf("named integer &^= should lower to Rust bit-clear on the wrapped slot:\n%s", rust)
	}
	if strings.Contains(rust, ".kind_ = DIRECT") {
		t.Fatalf("named integer &^= must not become a direct assignment:\n%s", rust)
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

// Regression: when a Go package has two functions whose snake-case names
// collide (e.g. `Walk` vs `walk`, both → `walk`), the per-package rename
// (`assignPackageFunctionNames`) and the per-file rename
// (`assignFunctionNames`) must not both produce `_1` suffixes for different
// functions. Before the fix, `walk` (lowercase) became `walk_1` from the
// package layer, and `Walk` (uppercase) became `walk_1` from the file
// layer — colliding.
func TestSnakeCaseCollidingFunctionsDoNotDoubleSuffix(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func Walk() int    { return 0 }
func walk() int    { return 0 }
func WalkDir() int { return 0 }
func walkDir() int { return 0 }

func main() {
	_ = Walk()
	_ = walk()
	_ = WalkDir()
	_ = walkDir()
}
`)

	walkOccurrences := strings.Count(rust, "pub fn walk_1(")
	if walkOccurrences > 1 {
		t.Fatalf("snake-case name collisions should each get a unique suffix; `walk_1` appears %d times:\n%s", walkOccurrences, rust)
	}
	walkDirOccurrences := strings.Count(rust, "pub fn walk_dir_1(")
	if walkDirOccurrences > 1 {
		t.Fatalf("snake-case name collisions should each get a unique suffix; `walk_dir_1` appears %d times:\n%s", walkDirOccurrences, rust)
	}
}
