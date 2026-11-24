package main

import (
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
	"strconv"
	"strings"
)

// Go defines every shift count: shifting an integer left by its width or
// more gives 0, and shifting it right gives 0, or -1 for a negative signed
// value. A negative count panics. Rust masks the count instead when overflow
// checks are off, as they are in a translated crate, so `1u64 << 64` is 1
// and strconv.ParseUint's `uint64(1)<<uint(bitSize) - 1` came out 0.
//
// A shift whose count is not a constant smaller than the width is written as
// a block that checks the count first. Shifts by such a constant keep the
// plain Rust operator.

// shiftNeedsGoCountCheck reports whether a shift of a value of type operand
// by count can reach a count Rust would mask.
func shiftNeedsGoCountCheck(operand types.Type, count ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || operand == nil {
		return false
	}
	basic, ok := types.Unalias(operand).Underlying().(*types.Basic)
	if !ok || basic.Info()&types.IsInteger == 0 || basic.Info()&types.IsUntyped != 0 {
		return false
	}
	if tv, ok := typeInfo.info.Types[count]; ok && tv.Value != nil {
		if n, exact := constant.Uint64Val(constant.ToInt(tv.Value)); exact && n < uint64(basicIntegerBits(basic)) {
			return false
		}
	}
	return true
}

// writeGoShiftBlock writes `x op count` with Go's count rules. writeValue
// writes the shifted value, writeCount the count; both as primitives.
func writeGoShiftBlock(out *strings.Builder, op token.Token, writeValue func(*strings.Builder), writeCount func(*strings.Builder)) {
	out.WriteString("{ let __shift_x = ")
	writeValue(out)
	out.WriteString("; let __shift_c = (")
	writeCount(out)
	out.WriteString(") as i128; if __shift_c < 0 { panic!(\"runtime error: negative shift amount\") } ")
	out.WriteString("let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; ")
	out.WriteString("if __shift_c >= __shift_bits as i128 { ")
	if op == token.SHR || op == token.SHR_ASSIGN {
		// The sign bit, spread over every bit: -1 for a negative signed
		// value, 0 otherwise. For an unsigned value it is 0.
		out.WriteString("(__shift_x >> (__shift_bits - 1)) >> 1")
	} else {
		out.WriteString("0")
	}
	out.WriteString(" } else { __shift_x ")
	if op == token.SHR || op == token.SHR_ASSIGN {
		out.WriteString(">>")
	} else {
		out.WriteString("<<")
	}
	out.WriteString(" (__shift_c as u32) } }")
}

// writeGoShiftCount writes a shift count as a primitive integer.
func writeGoShiftCount(out *strings.Builder, count ast.Expr) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.info != nil {
		if tv, ok := typeInfo.info.Types[count]; ok && tv.Value != nil {
			out.WriteString(constant.ToInt(tv.Value).ExactString())
			out.WriteString("i128")
			return
		}
	}
	writeNumericConversionValue(out, count)
}

// writeGoSemanticsShift writes a shift with a basic integer result. A shift
// whose result is a named type or a constant is left to the other paths.
func writeGoSemanticsShift(out *strings.Builder, e *ast.BinaryExpr) bool {
	if e == nil || (e.Op != token.SHL && e.Op != token.SHR) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	if tv, ok := typeInfo.info.Types[e]; ok && tv.Value != nil {
		return false
	}
	resultType := typeInfo.GetType(e)
	if resultType == nil {
		return false
	}
	if _, isNamed := types.Unalias(resultType).(*types.Named); isNamed {
		return false
	}
	basic, ok := types.Unalias(resultType).(*types.Basic)
	if !ok || !shiftNeedsGoCountCheck(basic, e.Y) {
		return false
	}
	rustType := rustCastForExpectedBasic(basic)
	if rustType == "" {
		return false
	}
	writeGoShiftBlock(out, e.Op, func(out *strings.Builder) {
		if tv, ok := typeInfo.info.Types[e.X]; ok && tv.Value != nil {
			out.WriteString(constant.ToInt(tv.Value).ExactString())
			out.WriteString(rustType)
			return
		}
		out.WriteString("(")
		writeNumericConversionValue(out, e.X)
		out.WriteString(") as ")
		out.WriteString(rustType)
	}, func(out *strings.Builder) {
		writeGoShiftCount(out, e.Y)
	})
	return true
}

// goShiftCompoundAsAssignment rewrites `x <<= n` and `x >>= n` as
// `x = x << n` when the count can reach the width, so the shift goes
// through writeGoSemanticsShift. The target is read twice, so only a target
// without calls or receives is rewritten.
func goShiftCompoundAsAssignment(s *ast.AssignStmt) (*ast.AssignStmt, bool) {
	if s == nil || len(s.Lhs) != 1 || len(s.Rhs) != 1 || (s.Tok != token.SHL_ASSIGN && s.Tok != token.SHR_ASSIGN) {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	lhsType := typeInfo.GetType(s.Lhs[0])
	if lhsType == nil || !shiftNeedsGoCountCheck(lhsType, s.Rhs[0]) {
		return nil, false
	}
	if _, isNamed := types.Unalias(lhsType).(*types.Named); isNamed && !isNamedIntegerType(lhsType) {
		return nil, false
	}
	if !shiftTargetIsPure(s.Lhs[0]) {
		return nil, false
	}
	op := token.SHL
	if s.Tok == token.SHR_ASSIGN {
		op = token.SHR
	}
	shift := &ast.BinaryExpr{X: s.Lhs[0], OpPos: s.TokPos, Op: op, Y: s.Rhs[0]}
	typeInfo.info.Types[shift] = types.TypeAndValue{Type: lhsType}
	return &ast.AssignStmt{Lhs: s.Lhs, TokPos: s.TokPos, Tok: token.ASSIGN, Rhs: []ast.Expr{shift}}, true
}

func shiftTargetIsPure(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.Ident:
		return e.Name != "_"
	case *ast.ParenExpr:
		return shiftTargetIsPure(e.X)
	case *ast.SelectorExpr:
		return shiftTargetIsPure(e.X)
	case *ast.StarExpr:
		return shiftTargetIsPure(e.X)
	case *ast.IndexExpr:
		return shiftTargetIsPure(e.X) && shiftIndexIsPure(e.Index)
	default:
		return false
	}
}

func shiftIndexIsPure(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.BasicLit:
		return true
	case *ast.Ident:
		return true
	case *ast.ParenExpr:
		return shiftIndexIsPure(e.X)
	case *ast.SelectorExpr:
		return shiftIndexIsPure(e.X)
	case *ast.BinaryExpr:
		return shiftIndexIsPure(e.X) && shiftIndexIsPure(e.Y)
	default:
		return false
	}
}

// basicIntegerBits is the width of a typed Go integer type, with int, uint
// and uintptr 64 bits wide as the translator emits them.
func basicIntegerBits(basic *types.Basic) int {
	switch basic.Kind() {
	case types.Int8, types.Uint8:
		return 8
	case types.Int16, types.Uint16:
		return 16
	case types.Int32, types.Uint32:
		return 32
	default:
		return 64
	}
}

// writeFoldedFloatConstant writes a constant float expression such as
// 0.1 + 0.2 as the value Go computes for it. Go evaluates constant
// expressions exactly and rounds once, so 0.1 + 0.2 is 0.3; Rust's
// floating-point addition gives 0.30000000000000004.
func writeFoldedFloatConstant(out *strings.Builder, e *ast.BinaryExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || e == nil {
		return false
	}
	tv, ok := typeInfo.info.Types[e]
	if !ok || tv.Value == nil || tv.Type == nil {
		return false
	}
	basic, ok := types.Unalias(tv.Type).Underlying().(*types.Basic)
	if !ok {
		return false
	}
	bits := 64
	switch basic.Kind() {
	case types.Float64, types.UntypedFloat:
	case types.Float32:
		bits = 32
	default:
		return false
	}
	if _, isNamed := types.Unalias(tv.Type).(*types.Named); isNamed {
		return false
	}
	value, _ := constant.Float64Val(constant.ToFloat(tv.Value))
	text := strconv.FormatFloat(value, 'g', -1, bits)
	if !strings.ContainsAny(text, ".eEn") {
		text += ".0"
	}
	out.WriteString(text)
	return true
}
