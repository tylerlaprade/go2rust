package main

import (
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
	"math/big"
	"sort"
	"strconv"
	"strings"
	"unicode/utf8"
)

func isConstIdent(ident *ast.Ident) bool {
	if ident == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.info != nil {
		obj := typeInfo.GetObject(ident)
		if _, ok := obj.(*types.Const); ok {
			return true
		}
		if obj != nil {
			return false
		}
	}
	if vt := GetVarTable(); vt != nil {
		if vt.Lookup(ident.Name) != nil {
			return false
		}
	}
	if _, ok := packageConstants[ident.Name]; ok {
		return true
	}
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	if typeInfo.pkg != nil && typeInfo.pkg.Scope() != nil {
		_, ok := typeInfo.pkg.Scope().Lookup(ident.Name).(*types.Const)
		return ok
	}
	return false
}

func rustConstNameBase(name string) string {
	return strings.ToUpper(strings.TrimPrefix(ToSnakeCase(name), "r#"))
}

func rustConstName(name string) string {
	if ctx := GetTranspileContext(); ctx != nil && ctx.Package != nil && ctx.Package.ConstantNameOverrides != nil {
		if rustName, ok := ctx.Package.ConstantNameOverrides[name]; ok {
			return rustName
		}
	}
	return rustConstNameBase(name)
}

type packageConstantName struct {
	goName   string
	rustName string
	pos      token.Pos
	exported bool
}

func assignPackageConstantNames(files []*ast.File) map[string]string {
	byRustName := make(map[string][]packageConstantName)
	seenGoNames := make(map[string]bool)
	for _, file := range files {
		for _, decl := range file.Decls {
			genDecl, ok := decl.(*ast.GenDecl)
			if !ok || genDecl.Tok != token.CONST {
				continue
			}
			for _, spec := range genDecl.Specs {
				valueSpec, ok := spec.(*ast.ValueSpec)
				if !ok {
					continue
				}
				for _, name := range valueSpec.Names {
					if name.Name == "_" || seenGoNames[name.Name] {
						continue
					}
					seenGoNames[name.Name] = true
					rustName := rustConstNameBase(name.Name)
					byRustName[rustName] = append(byRustName[rustName], packageConstantName{
						goName:   name.Name,
						rustName: rustName,
						pos:      name.Pos(),
						exported: ast.IsExported(name.Name),
					})
				}
			}
		}
	}

	overrides := make(map[string]string)
	for rustName, constants := range byRustName {
		if len(constants) <= 1 {
			continue
		}
		sort.Slice(constants, func(i, j int) bool {
			if constants[i].exported != constants[j].exported {
				return constants[i].exported
			}
			if constants[i].pos != constants[j].pos {
				return constants[i].pos < constants[j].pos
			}
			return constants[i].goName < constants[j].goName
		})
		for i, constantName := range constants {
			if i == 0 {
				continue
			}
			overrides[constantName.goName] = rustName + "_" + strconv.Itoa(i)
		}
	}
	return overrides
}

func registerPackageConstant(name string, constType string) {
	if packageConstants == nil {
		packageConstants = make(map[string]string)
	}
	packageConstants[name] = constType
	if currentContext != nil && currentContext.Package != nil {
		if currentContext.Package.PackageConstants == nil {
			currentContext.Package.PackageConstants = make(map[string]string)
		}
		currentContext.Package.PackageConstants[name] = constType
	}
}

func registerPackageConstantTypeName(name string, typeName string) {
	if typeName == "" {
		return
	}
	if packageConstantTypeNames == nil {
		packageConstantTypeNames = make(map[string]string)
	}
	packageConstantTypeNames[name] = typeName
	if currentContext != nil && currentContext.Package != nil {
		if currentContext.Package.PackageConstantTypeNames == nil {
			currentContext.Package.PackageConstantTypeNames = make(map[string]string)
		}
		currentContext.Package.PackageConstantTypeNames[name] = typeName
	}
}

func writeExpressionForExpectedType(out *strings.Builder, value ast.Expr, expected ast.Expr) bool {
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if writeConstExpressionForExpectedGoType(out, value, typeInfo.GetType(expected)) {
			return true
		}
	}
	expectedIdent, ok := expected.(*ast.Ident)
	if !ok {
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			return writeExpressionForExpectedTypesType(out, value, typeInfo.GetType(expected))
		}
		return false
	}
	if writeConstExpressionForExpectedTypeExpr(out, value, expectedIdent) {
		return true
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if writeExpressionForExpectedTypesType(out, value, typeInfo.GetType(expectedIdent)) {
			return true
		}
	}
	underlying, isTypeDef := LookupTypeDefinition(expectedIdent.Name)
	if !isTypeDef {
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			return writeExpressionForExpectedTypesType(out, value, typeInfo.GetType(expected))
		}
		return false
	}
	if underlying == "string" {
		writeStringTypeDefinitionConstructor(out, RustTypeNameForUse(expectedIdent.Name), value)
		return true
	}
	out.WriteString(RustTypeNameForUse(expectedIdent.Name))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	TranspileExpression(out, value)
	if rustType, ok := rustCastTypeForDefinedUnderlying(underlying); ok {
		out.WriteString(" as ")
		out.WriteString(rustType)
	}
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

func writeConstExpressionForExpectedTypeExpr(out *strings.Builder, value ast.Expr, expected ast.Expr) bool {
	ident, ok := expected.(*ast.Ident)
	if !ok || !isSyntaxConstantExpression(value) {
		return false
	}
	rustType, ok := rustCastTypeForDefinedUnderlying(ident.Name)
	if !ok {
		return false
	}
	TranspileExpression(out, value)
	out.WriteString(" as ")
	out.WriteString(rustType)
	return true
}

func isSyntaxConstantExpression(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.BasicLit:
		return true
	case *ast.Ident:
		if e.Name == "true" || e.Name == "false" {
			return true
		}
		if _, ok := localConstants[e.Name]; ok {
			return true
		}
		if _, ok := packageConstants[e.Name]; ok {
			return true
		}
		return isConstIdent(e)
	case *ast.BinaryExpr:
		return isSyntaxConstantExpression(e.X) && isSyntaxConstantExpression(e.Y)
	case *ast.UnaryExpr:
		return isSyntaxConstantExpression(e.X)
	case *ast.ParenExpr:
		return isSyntaxConstantExpression(e.X)
	default:
		return false
	}
}

func writeConstExpressionForSyntaxPeer(out *strings.Builder, expr ast.Expr, other ast.Expr) bool {
	if !isSyntaxConstantExpression(expr) {
		return false
	}
	if lit, ok := expr.(*ast.BasicLit); ok && lit.Kind == token.CHAR {
		return false
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.GetType(other) != nil && !syntaxPeerCanUseCompleteTypeInfo(other) {
		return false
	}
	rustType, ok := syntaxIntegerRustType(other)
	if !ok {
		return false
	}
	TranspileExpression(out, expr)
	out.WriteString(" as ")
	out.WriteString(rustType)
	return true
}

func syntaxPeerCanUseCompleteTypeInfo(expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	fieldExpr, ok := selectorFieldTypeExpr(sel)
	if !ok {
		return false
	}
	ident, ok := fieldExpr.(*ast.Ident)
	if !ok {
		return false
	}
	if _, isTypeDef := LookupTypeDefinition(ident.Name); isTypeDef {
		return false
	}
	_, ok = rustCastTypeForDefinedUnderlying(ident.Name)
	return ok
}

func syntaxIntegerRustType(expr ast.Expr) (string, bool) {
	switch e := expr.(type) {
	case *ast.Ident:
		if rangeLoopVars[e.Name] == "usize" {
			return "i32", true
		}
		if info := lookupVarInfo(e.Name); info != nil && info.RustType != "" {
			return rustIntegerCastTypeFromRustType(info.RustType)
		}
		if typeName := packageConstantTypeNames[e.Name]; typeName != "" {
			return rustIntegerCastTypeFromRustType(typeName)
		}
		if constType := packageConstants[e.Name]; constType != "" {
			return rustIntegerCastTypeFromRustType(constType)
		}
		if localType := localConstants[e.Name]; localType != "" {
			return rustIntegerCastTypeFromRustType(localType)
		}
	case *ast.SelectorExpr:
		if fieldExpr, ok := selectorFieldTypeExpr(e); ok {
			return syntaxIntegerRustTypeFromTypeExpr(fieldExpr)
		}
	case *ast.BinaryExpr:
		switch e.Op {
		case token.AND, token.OR, token.XOR, token.SHL, token.SHR:
			if rustType, ok := syntaxIntegerRustType(e.X); ok {
				return rustType, true
			}
			return syntaxIntegerRustType(e.Y)
		}
	case *ast.CallExpr:
		if ident, ok := e.Fun.(*ast.Ident); ok {
			return rustIntegerCastTypeFromRustType(ident.Name)
		}
	}
	return "", false
}

func syntaxIntegerRustTypeFromTypeExpr(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return "", false
	}
	return rustIntegerCastTypeFromRustType(ident.Name)
}

func rustIntegerCastTypeFromRustType(rustType string) (string, bool) {
	rustType = unwrapStoredRustType(rustType)
	if underlying, isTypeDef := LookupTypeDefinition(rustType); isTypeDef {
		return rustCastTypeForDefinedUnderlying(underlying)
	}
	if rustType == "usize" {
		return rustType, true
	}
	switch rustType {
	case "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128":
		return rustType, true
	default:
		return rustCastTypeForDefinedUnderlying(rustType)
	}
}

func isByteLikeGoType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	basic, ok := types.Unalias(typ).(*types.Basic)
	return ok && basic.Kind() == types.Uint8
}

func constExpressionInt64Value(expr ast.Expr) (int64, bool) {
	value, ok := constExpressionValue(expr)
	if !ok {
		return 0, false
	}
	return constant.Int64Val(value)
}

func constExpressionValue(expr ast.Expr) (constant.Value, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	if tv, ok := typeInfo.info.Types[expr]; ok && tv.Value != nil {
		return tv.Value, true
	}
	if ident, ok := expr.(*ast.Ident); ok {
		if obj, ok := typeInfo.GetObject(ident).(*types.Const); ok && obj.Val() != nil {
			return obj.Val(), true
		}
		if typeInfo.pkg != nil && typeInfo.pkg.Scope() != nil {
			if obj, ok := typeInfo.pkg.Scope().Lookup(ident.Name).(*types.Const); ok && obj.Val() != nil {
				return obj.Val(), true
			}
		}
	}
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if obj, ok := typeInfo.GetObject(sel.Sel).(*types.Const); ok && obj.Val() != nil {
			return obj.Val(), true
		}
	}
	if paren, ok := expr.(*ast.ParenExpr); ok {
		return constExpressionValue(paren.X)
	}
	if binary, ok := expr.(*ast.BinaryExpr); ok {
		left, lok := constExpressionValue(binary.X)
		right, rok := constExpressionValue(binary.Y)
		if !lok || !rok {
			return nil, false
		}
		switch binary.Op {
		case token.SHL, token.SHR:
			shift, exact := constant.Uint64Val(right)
			if !exact {
				return nil, false
			}
			return constant.Shift(left, binary.Op, uint(shift)), true
		case token.ADD, token.SUB, token.MUL, token.QUO, token.REM,
			token.AND, token.OR, token.XOR, token.AND_NOT:
			return constant.BinaryOp(left, binary.Op, right), true
		default:
			return nil, false
		}
	}
	if unary, ok := expr.(*ast.UnaryExpr); ok {
		value, ok := constExpressionValue(unary.X)
		if !ok {
			return nil, false
		}
		switch unary.Op {
		case token.ADD, token.SUB, token.XOR:
			return constant.UnaryOp(unary.Op, value, 0), true
		default:
			return nil, false
		}
	}
	return nil, false
}

func writeConstExpressionForExpectedGoType(out *strings.Builder, value ast.Expr, expected types.Type) bool {
	if writeConstZeroExpressionForExpectedComplex(out, value, expected) {
		return true
	}
	if writeConstExpressionForExpectedNamedInteger(out, value, expected) {
		return true
	}
	if writeConstExpressionForExpectedString(out, value, expected) {
		return true
	}
	if writeConstExpressionForExpectedFloat(out, value, expected) {
		return true
	}
	if writeConstExpressionForExpectedInteger(out, value, expected) {
		return true
	}
	if !isByteLikeGoType(expected) {
		return false
	}
	intValue, ok := constExpressionInt64Value(value)
	if ok {
		if intValue < 0 || intValue > 255 {
			return false
		}
	} else {
		ident, isIdent := value.(*ast.Ident)
		if !isIdent {
			return false
		}
		if _, isPackageConst := packageConstants[ident.Name]; !isPackageConst {
			return false
		}
	}
	TranspileExpression(out, value)
	out.WriteString(" as u8")
	return true
}

func constStringLiteral(expr ast.Expr) (string, bool) {
	value, ok := constExpressionValue(expr)
	if !ok || value.Kind() != constant.String {
		return "", false
	}
	return RustStringLiteral(strconv.Quote(constant.StringVal(value))), true
}

func writeConstStringLiteralValue(out *strings.Builder, expr ast.Expr) bool {
	lit, ok := constStringLiteral(expr)
	if !ok {
		return false
	}
	out.WriteString(lit)
	return true
}

func constStringValue(expr ast.Expr) (string, bool) {
	value, ok := constExpressionValue(expr)
	if !ok || value.Kind() != constant.String {
		return "", false
	}
	return constant.StringVal(value), true
}

func constStringValueNeedsByteSlice(value constant.Value) bool {
	return value != nil && value.Kind() == constant.String && !utf8.ValidString(constant.StringVal(value))
}

func constStringNeedsByteSlice(expr ast.Expr) bool {
	value, ok := constStringValue(expr)
	return ok && !utf8.ValidString(value)
}

func rustByteSliceLiteralForStringValue(value string) string {
	var out strings.Builder
	out.WriteString("&[")
	for i, b := range []byte(value) {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString("0x")
		out.WriteString(strconv.FormatUint(uint64(b), 16))
		out.WriteString("u8")
	}
	out.WriteString("]")
	return out.String()
}

func writeConstByteSliceLiteralValue(out *strings.Builder, expr ast.Expr) bool {
	value, ok := constStringValue(expr)
	if !ok || utf8.ValidString(value) {
		return false
	}
	out.WriteString(rustByteSliceLiteralForStringValue(value))
	return true
}

func formatRustFloat64(value float64) (string, bool) {
	text := strconv.FormatFloat(value, 'g', -1, 64)
	if strings.Contains(text, "Inf") || strings.Contains(text, "NaN") {
		return "", false
	}
	if !strings.ContainsAny(text, ".eE") {
		text += ".0"
	}
	return text, true
}

func rustFloatLiteral(lit *ast.BasicLit) (string, bool) {
	if lit == nil || lit.Kind != token.FLOAT {
		return "", false
	}
	text := strings.ReplaceAll(lit.Value, "_", "")
	if strings.ContainsAny(text, "pP") {
		if value, ok := constExpressionValue(lit); ok && value.Kind() == constant.Float {
			f, _ := constant.Float64Val(value)
			if formatted, ok := formatRustFloat64(f); ok {
				return formatted, true
			}
		}
		if f, err := strconv.ParseFloat(text, 64); err == nil {
			if formatted, ok := formatRustFloat64(f); ok {
				return formatted, true
			}
		}
		return "", false
	}
	if strings.HasPrefix(text, ".") {
		text = "0" + text
	}
	if strings.HasSuffix(text, ".") {
		text += "0"
	}
	return text, true
}

func writeConstIntegerLiteralValue(out *strings.Builder, expr ast.Expr) bool {
	value, ok := constExpressionValue(expr)
	if !ok || value.Kind() != constant.Int {
		return false
	}
	out.WriteString(value.String())
	return true
}

func writeConstShiftCountValue(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isNamedIntegerType(typeInfo.GetType(expr)) {
		return false
	}
	return writeConstIntegerLiteralValue(out, expr)
}

func rustConstTypeForUntypedIntegerValue(expr ast.Expr, value constant.Value) (string, bool) {
	min, max, ok := typedIntegerConstBounds(expr)
	if valueMin, valueMax, valueOK := integerConstValueBounds(value); valueOK {
		if ok {
			if valueMin.Cmp(min) < 0 {
				min = valueMin
			}
			if valueMax.Cmp(max) > 0 {
				max = valueMax
			}
		} else {
			min, max, ok = valueMin, valueMax, true
		}
	}
	if !ok {
		return "", false
	}
	return rustIntegerTypeForConstBounds(min, max)
}

func typedIntegerConstBounds(expr ast.Expr) (*big.Int, *big.Int, bool) {
	if expr == nil {
		return nil, nil, false
	}
	var min, max *big.Int
	ast.Inspect(expr, func(node ast.Node) bool {
		expr, ok := node.(ast.Expr)
		if !ok {
			return true
		}
		value, ok := typedIntegerConstValue(expr)
		if !ok {
			return true
		}
		n, ok := bigIntForConstInteger(value)
		if !ok {
			return true
		}
		if min == nil || n.Cmp(min) < 0 {
			min = n
		}
		if max == nil || n.Cmp(max) > 0 {
			max = n
		}
		return true
	})
	return min, max, min != nil && max != nil
}

func typedIntegerConstValue(expr ast.Expr) (constant.Value, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || expr == nil {
		return nil, false
	}
	if tv, ok := typeInfo.info.Types[expr]; ok && tv.Value != nil && tv.Value.Kind() == constant.Int && isUntypedIntegerConstType(tv.Type) {
		return tv.Value, true
	}
	switch e := expr.(type) {
	case *ast.Ident:
		if obj, ok := typeInfo.GetObject(e).(*types.Const); ok && obj.Val() != nil && obj.Val().Kind() == constant.Int && isUntypedIntegerConstType(obj.Type()) {
			return obj.Val(), true
		}
	case *ast.SelectorExpr:
		if obj, ok := typeInfo.GetObject(e.Sel).(*types.Const); ok && obj.Val() != nil && obj.Val().Kind() == constant.Int && isUntypedIntegerConstType(obj.Type()) {
			return obj.Val(), true
		}
	}
	return nil, false
}

func isUntypedIntegerConstType(typ types.Type) bool {
	basic, ok := types.Unalias(typ).(*types.Basic)
	return ok && (basic.Kind() == types.UntypedInt || basic.Kind() == types.UntypedRune)
}

func integerConstValueBounds(value constant.Value) (*big.Int, *big.Int, bool) {
	n, ok := bigIntForConstInteger(value)
	if !ok {
		return nil, nil, false
	}
	return n, new(big.Int).Set(n), true
}

func bigIntForConstInteger(value constant.Value) (*big.Int, bool) {
	if value == nil || value.Kind() != constant.Int {
		return nil, false
	}
	var n big.Int
	if _, ok := n.SetString(value.ExactString(), 0); !ok {
		return nil, false
	}
	return &n, true
}

func rustIntegerTypeForConstBounds(min *big.Int, max *big.Int) (string, bool) {
	if min == nil || max == nil {
		return "", false
	}
	if min.Sign() < 0 {
		if min.Cmp(minSignedBigInt(32)) >= 0 && max.Cmp(maxSignedBigInt(32)) <= 0 {
			return "i32", true
		}
		if min.Cmp(minSignedBigInt(64)) >= 0 && max.Cmp(maxSignedBigInt(64)) <= 0 {
			return "i64", true
		}
		if min.Cmp(minSignedBigInt(128)) >= 0 && max.Cmp(maxSignedBigInt(128)) <= 0 {
			return "i128", true
		}
		return "", false
	}
	if max.Cmp(maxSignedBigInt(32)) <= 0 {
		return "i32", true
	}
	if max.Cmp(maxSignedBigInt(64)) <= 0 {
		return "i64", true
	}
	if max.Cmp(maxUnsignedBigInt(64)) <= 0 {
		return "u64", true
	}
	if max.Cmp(maxUnsignedBigInt(128)) <= 0 {
		return "u128", true
	}
	return "", false
}

func minSignedBigInt(bits uint) *big.Int {
	return new(big.Int).Neg(new(big.Int).Lsh(big.NewInt(1), bits-1))
}

func maxSignedBigInt(bits uint) *big.Int {
	return new(big.Int).Sub(new(big.Int).Lsh(big.NewInt(1), bits-1), big.NewInt(1))
}

func maxUnsignedBigInt(bits uint) *big.Int {
	return new(big.Int).Sub(new(big.Int).Lsh(big.NewInt(1), bits), big.NewInt(1))
}

func isBasicStringGoType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	basic, ok := types.Unalias(typ).(*types.Basic)
	return ok && basic.Kind() == types.String
}

func writeConstExpressionForExpectedString(out *strings.Builder, value ast.Expr, expected types.Type) bool {
	if !isBasicStringGoType(expected) {
		return false
	}
	if !writeConstStringLiteralValue(out, value) {
		return false
	}
	out.WriteString(".to_string()")
	return true
}

func rustFloatTypeForGoType(typ types.Type) (string, bool) {
	basic, ok := types.Unalias(typ).(*types.Basic)
	if !ok {
		return "", false
	}
	switch basic.Kind() {
	case types.Float32:
		return "f32", true
	case types.Float64, types.UntypedFloat:
		return "f64", true
	default:
		return "", false
	}
}

func constFloatLiteralForRustType(value constant.Value, rustType string) (string, bool) {
	floatValue := constant.ToFloat(value)
	if floatValue.Kind() != constant.Float {
		return "", false
	}
	f, _ := constant.Float64Val(floatValue)
	bits := 64
	if rustType == "f32" {
		bits = 32
	}
	text := strconv.FormatFloat(f, 'g', -1, bits)
	if strings.Contains(text, "Inf") || strings.Contains(text, "NaN") {
		return "", false
	}
	if !strings.ContainsAny(text, ".eE") {
		text += ".0"
	}
	return text, true
}

func writeConstExpressionForExpectedFloat(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	rustType, ok := rustFloatTypeForGoType(expected)
	if !ok {
		return false
	}
	value, ok := constExpressionValue(expr)
	if !ok || (value.Kind() != constant.Int && value.Kind() != constant.Float) {
		return false
	}
	if value.Kind() == constant.Float {
		switch expr.(type) {
		case *ast.Ident, *ast.SelectorExpr:
			return false
		}
	}
	lit, ok := constFloatLiteralForRustType(value, rustType)
	if !ok {
		return false
	}
	out.WriteString(lit)
	return true
}

func writeConstZeroExpressionForExpectedComplex(out *strings.Builder, value ast.Expr, expected types.Type) bool {
	if !isConstantExpression(value) || expected == nil {
		return false
	}
	basic, ok := types.Unalias(expected).(*types.Basic)
	if !ok {
		return false
	}
	componentRust := ""
	switch basic.Kind() {
	case types.Complex64:
		componentRust = "f32"
	case types.Complex128:
		componentRust = "f64"
	default:
		return false
	}
	constValue, ok := constExpressionValue(value)
	if !ok || constant.Sign(constValue) != 0 {
		return false
	}
	TrackImport("num::Complex")
	out.WriteString("num::Complex::<")
	out.WriteString(componentRust)
	out.WriteString(">::new(0.0, 0.0)")
	return true
}

func writeConstExpressionForExpectedInteger(out *strings.Builder, value ast.Expr, expected types.Type) bool {
	if !isConstantExpression(value) {
		return false
	}
	if hasStdlibSelectorMapping(value) {
		return false
	}
	if lit, ok := value.(*ast.BasicLit); ok && lit.Kind == token.CHAR {
		return false
	}
	rustType, ok := rustIntegerCastTypeForExpected(expected)
	if !ok {
		return false
	}
	writeConstExpressionCastValue(out, value)
	out.WriteString(" as ")
	out.WriteString(rustType)
	return true
}

func hasStdlibSelectorMapping(expr ast.Expr) bool {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	ident, ok := sel.X.(*ast.Ident)
	if !ok {
		return false
	}
	return GetStdlibSelectorMapping(resolveStdlibPackageName(ident.Name), sel.Sel.Name) != ""
}

func writeConstExpressionCastValue(out *strings.Builder, value ast.Expr) {
	typeInfo := GetTypeInfo()
	call, isCall := value.(*ast.CallExpr)
	if typeInfo != nil && isCall && typeInfo.ReturnsWrappedValue(call) && !isBareBuiltinReturn(call) && (!typeInfo.IsTypeConversion(call) || typeConversionEmitsWrappedValue(call)) {
		out.WriteString("(*")
		TranspileExpression(out, value)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
		return
	}
	// A constant named-integer binary expression (e.g. `dep.NeedName |
	// dep.NeedFiles`) must stay in its primitive Rust form here so the
	// surrounding ` as i32` cast applies to bare i32, not to a wrapped
	// newtype value. `TranspileExpression` would dispatch to the binary
	// handler that emits a newtype, leaving us with `LoadMode(...) as i32`
	// which Rust rejects.
	if _, isBin := value.(*ast.BinaryExpr); isBin && isConstantExpression(value) {
		if typeInfo != nil {
			if named, ok := types.Unalias(typeInfo.GetType(value)).(*types.Named); ok && isNamedIntegerType(named) {
				TranspileConstExpr(out, value, 0)
				return
			}
		}
	}
	if typeInfo != nil {
		if call, ok := value.(*ast.CallExpr); ok && typeInfo.IsTypeConversion(call) {
			if named, ok := types.Unalias(typeInfo.GetType(call)).(*types.Named); ok && isNamedIntegerType(named) {
				writeNumericConversionValue(out, call)
				return
			}
		}
	}
	if _, ok := value.(*ast.UnaryExpr); ok && isConstantExpression(value) {
		TranspileConstExpr(out, value, 0)
		return
	}
	TranspileExpression(out, value)
}

func rustIntegerCastTypeForExpected(expected types.Type) (string, bool) {
	if expected == nil {
		return "", false
	}
	if named, ok := types.Unalias(expected).(*types.Named); ok && isNamedIntegerType(named) {
		basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
		if !ok {
			return "", false
		}
		return rustCastTypeForDefinedUnderlying(basic.Name())
	}
	basic, ok := types.Unalias(expected).(*types.Basic)
	if !ok || !isIntegerBasicKind(basic.Kind()) {
		return "", false
	}
	if basic.Kind() == types.UntypedInt || basic.Kind() == types.UntypedRune {
		return "", false
	}
	if basic.Kind() == types.Int {
		return "", false
	}
	return rustCastTypeForDefinedUnderlying(basic.Name())
}

func writeConstExpressionForExpectedNamedInteger(out *strings.Builder, value ast.Expr, expected types.Type) bool {
	if !isConstantExpression(value) {
		return false
	}
	named, ok := constNamedIntegerTargetType(value, expected)
	if !ok {
		return false
	}
	return writeExpressionForExpectedTypesType(out, value, named)
}

func constNamedIntegerTargetType(value ast.Expr, expected types.Type) (*types.Named, bool) {
	named, ok := types.Unalias(expected).(*types.Named)
	if !ok && expected == nil {
		typeInfo := GetTypeInfo()
		if typeInfo == nil {
			return nil, false
		}
		named, ok = types.Unalias(typeInfo.GetType(value)).(*types.Named)
	}
	if !ok || named.Obj() == nil || !isNamedIntegerType(named) {
		return nil, false
	}
	if _, isTypeDef := LookupTypeDefinition(named.Obj().Name()); !isTypeDef {
		typeInfo := GetTypeInfo()
		isKnownStdlibHelper := named.Obj().Pkg() != nil && isKnownStdlibHelperType(named.Obj().Pkg().Path(), named.Obj().Name())
		if typeInfo != nil && !isKnownStdlibHelper {
			if valueNamed, ok := types.Unalias(typeInfo.GetType(value)).(*types.Named); ok && sameNamedTypeDefinition(valueNamed, named) {
				return named, true
			}
		}
		isCurrentPackageType := typeInfo != nil && typeInfo.pkg != nil && named.Obj().Pkg() == typeInfo.pkg
		_, isExternalInteger := externalIntegerRustTypeForNamed(named)
		isExternalInteger = isExternalInteger && !isKnownStdlibHelper
		if !isCurrentPackageType && !isExternalInteger {
			return nil, false
		}
	}
	return named, true
}

func writeConstExpressionForBinaryPeer(out *strings.Builder, expr ast.Expr, other ast.Expr) bool {
	if !isConstantExpression(expr) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	expected := typeInfo.GetType(other)
	if expectedNamed, ok := types.Unalias(expected).(*types.Named); ok && isNamedIntegerType(expectedNamed) {
		if exprNamed, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Named); ok && sameNamedTypeDefinition(exprNamed, expectedNamed) {
			if _, ok := externalIntegerRustTypeForNamed(expectedNamed); ok {
				return writeExpressionForExpectedTypesType(out, expr, expectedNamed)
			}
			if call, ok := expr.(*ast.CallExpr); ok && typeInfo.IsTypeConversion(call) {
				return writeNamedIntegerConversionConstForBinaryPeer(out, call, expectedNamed)
			}
			return writeNamedIntegerConstForExpected(out, expr, expectedNamed)
		}
	}
	if writeConstExpressionForExpectedGoType(out, expr, expected) {
		return true
	}
	return writeConstExpressionForExpectedInteger(out, expr, expected)
}

func writePrimitiveConstExpressionForBinaryPeer(out *strings.Builder, expr ast.Expr, other ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	expected := typeInfo.GetType(other)
	if writeConstExpressionForExpectedFloat(out, expr, expected) {
		return true
	}
	return writeConstExpressionForExpectedInteger(out, expr, expected)
}

func writeSwitchCaseValueForTag(out *strings.Builder, expr ast.Expr, tag ast.Expr) {
	if lit, ok := expr.(*ast.BasicLit); ok && lit.Kind == token.CHAR && rangeVarRustType(tag) == "char" {
		out.WriteString(RustCharLiteral(lit.Value))
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && writeConstExpressionForExpectedGoType(out, expr, typeInfo.GetType(tag)) {
		return
	}
	writeSwitchCaseValue(out, expr)
}

func expectsStringType(expected ast.Expr) bool {
	expectedIdent, ok := expected.(*ast.Ident)
	return ok && expectedIdent.Name == "string"
}

func rustCastTypeForDefinedUnderlying(underlying string) (string, bool) {
	switch underlying {
	case "int", "int8", "int16", "int32", "int64", "rune",
		"uint", "uint8", "uint16", "uint32", "uint64", "uintptr", "byte",
		"float32", "float64":
		return goTypeToRustBase(ast.NewIdent(underlying)), true
	default:
		return "", false
	}
}

func isBitwiseDefinedUnderlying(underlying string) bool {
	switch underlying {
	case "int", "int8", "int16", "int32", "int64", "rune",
		"uint", "uint8", "uint16", "uint32", "uint64", "uintptr", "byte":
		return true
	default:
		return false
	}
}

func rustConstTypeForDefinedUnderlying(underlying string) (string, bool) {
	switch underlying {
	case "string":
		return "&'static str", true
	case "bool":
		return "bool", true
	default:
		return rustCastTypeForDefinedUnderlying(underlying)
	}
}

func rustConstTypeForGoTypesType(typ types.Type) (string, bool) {
	switch t := typ.(type) {
	case *types.Named:
		if _, ok := externalIntegerRustTypeForNamed(t); ok && t.Obj() != nil && t.Obj().Pkg() != nil && !isKnownStdlibHelperType(t.Obj().Pkg().Path(), t.Obj().Name()) {
			return goTypesNamedTypeToRust(t), true
		}
		return rustConstTypeForGoTypesType(t.Underlying())
	case *types.Basic:
		switch t.Kind() {
		case types.Bool, types.UntypedBool:
			return "bool", true
		case types.String, types.UntypedString:
			return "&'static str", true
		case types.Float32:
			return "f32", true
		case types.Float64, types.UntypedFloat:
			return "f64", true
		}
		return rustConstTypeForDefinedUnderlying(t.Name())
	default:
		return "", false
	}
}

func rustConstTypeForTypeExpr(expr ast.Expr) string {
	if ident, ok := expr.(*ast.Ident); ok {
		if ident.Name == "string" {
			return "&'static str"
		}
		if underlying, isTypeDef := LookupTypeDefinition(ident.Name); isTypeDef {
			if rustType, ok := rustConstTypeForDefinedUnderlying(underlying); ok {
				return rustType
			}
		}
	}

	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if rustType, ok := rustConstTypeForGoTypesType(typeInfo.GetType(expr)); ok {
			return rustType
		}
	}

	return goTypeToRustBase(expr)
}

func isDisplayableDefinedUnderlying(underlying string) bool {
	switch underlying {
	case "string", "bool", "int", "int8", "int16", "int32", "int64", "rune",
		"uint", "uint8", "uint16", "uint32", "uint64", "uintptr", "byte",
		"float32", "float64":
		return true
	default:
		return false
	}
}

func isEqualityComparableDefinedUnderlying(underlying string) bool {
	return isDisplayableDefinedUnderlying(underlying)
}

func namedTypeDefinitionFromType(typ types.Type) (*types.Named, bool) {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil {
		return nil, false
	}
	if _, isTypeDef := LookupTypeDefinition(named.Obj().Name()); !isTypeDef {
		return nil, false
	}
	return named, true
}

func sameNamedTypeDefinition(left *types.Named, right *types.Named) bool {
	if left == nil || right == nil || left.Obj() == nil || right.Obj() == nil {
		return false
	}
	if left.Obj() == right.Obj() {
		return true
	}
	leftPkg := ""
	if left.Obj().Pkg() != nil {
		leftPkg = left.Obj().Pkg().Path()
	}
	rightPkg := ""
	if right.Obj().Pkg() != nil {
		rightPkg = right.Obj().Pkg().Path()
	}
	return left.Obj().Name() == right.Obj().Name() && leftPkg == rightPkg
}

func writeNamedConstForBinaryPeer(out *strings.Builder, expr ast.Expr, other ast.Expr) bool {
	if !isConstantExpression(expr) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	exprNamed, ok := namedTypeDefinitionFromType(typeInfo.GetType(expr))
	if !ok {
		return false
	}
	otherNamed, ok := namedTypeDefinitionFromType(typeInfo.GetType(other))
	if !ok || !sameNamedTypeDefinition(exprNamed, otherNamed) {
		return false
	}
	if call, ok := expr.(*ast.CallExpr); ok && typeInfo.IsTypeConversion(call) {
		return writeNamedIntegerConversionConstForBinaryPeer(out, call, otherNamed)
	}
	writeExpressionForExpectedTypesType(out, expr, otherNamed)
	return true
}

func writeNamedIntegerConversionConstForBinaryPeer(out *strings.Builder, call *ast.CallExpr, named *types.Named) bool {
	if len(call.Args) != 1 || named == nil {
		return false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok || !isIntegerBasicKind(basic.Kind()) {
		return false
	}
	rustType, ok := rustCastTypeForDefinedUnderlying(basic.Name())
	if !ok {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	writeNumericConversionValue(out, call.Args[0])
	out.WriteString(" as ")
	out.WriteString(rustType)
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

func writeNamedIntegerConstForExpected(out *strings.Builder, value ast.Expr, named *types.Named) bool {
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok || !isIntegerBasicKind(basic.Kind()) {
		return false
	}
	rustType, ok := rustCastTypeForDefinedUnderlying(basic.Name())
	if !ok {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	if isConstantExpression(value) && !isNamedIntegerConversionCall(value) {
		TranspileConstExpr(out, value, 0)
	} else {
		writeNumericConversionValue(out, value)
	}
	out.WriteString(" as ")
	out.WriteString(rustType)
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

func isNamedIntegerConversionCall(value ast.Expr) bool {
	call, ok := value.(*ast.CallExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsTypeConversion(call) {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(call)).(*types.Named)
	return ok && isNamedIntegerType(named)
}

func writeWrappedExpressionForExpectedType(out *strings.Builder, value ast.Expr, expected ast.Expr) {
	WriteWrapperPrefix(out)
	if expectsStringType(expected) && isStringConstExpr(value) {
		TranspileExpression(out, value)
		out.WriteString(".to_string()")
	} else if !writeExpressionForExpectedType(out, value, expected) {
		TranspileExpression(out, value)
	}
	WriteWrapperSuffix(out)
}

func isNamedTypeDefinitionValue(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil {
		return false
	}
	_, isTypeDef := LookupTypeDefinition(named.Obj().Name())
	return isTypeDef
}

func writeNamedTypeInnerExpression(out *strings.Builder, expr ast.Expr) bool {
	if !isNamedTypeDefinitionValue(expr) {
		return false
	}
	out.WriteString("(*")
	TranspileExpression(out, expr)
	out.WriteString(".0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
	return true
}
