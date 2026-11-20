package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

// writeUnsafeScalarReinterpretValue writes *(*T)(unsafe.Pointer(&x)), where
// x and T are numbers of the same size, as the bits of x read as a T. This is
// how math.Float64bits and math.Float64frombits are written, and how a
// program reads a float's bits. The value is copied, as the Go read is.
func writeUnsafeScalarReinterpretValue(out *strings.Builder, star *ast.StarExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || star == nil {
		return false
	}
	conv, ok := unwrapParens(star.X).(*ast.CallExpr)
	if !ok || len(conv.Args) != 1 || !typeInfo.IsTypeConversion(conv) {
		return false
	}
	inner, ok := unwrapParens(conv.Args[0]).(*ast.CallExpr)
	if !ok || len(inner.Args) != 1 || !typeInfo.IsTypeConversion(inner) || !isUnsafePointerLikeType(typeInfo.GetType(inner)) {
		return false
	}
	source := unwrapParens(inner.Args[0])
	var value ast.Expr
	if addr, ok := source.(*ast.UnaryExpr); ok && addr.Op == token.AND {
		value = addr.X
	} else if ptr, ok := types.Unalias(typeInfo.GetType(source)).Underlying().(*types.Pointer); ok && ptr != nil {
		value = &ast.StarExpr{X: source}
		typeInfo.info.Types[value] = types.TypeAndValue{Type: ptr.Elem()}
	} else {
		return false
	}
	from, ok := types.Unalias(typeInfo.GetType(value)).Underlying().(*types.Basic)
	if !ok {
		return false
	}
	to, ok := types.Unalias(typeInfo.GetType(star)).Underlying().(*types.Basic)
	if !ok {
		return false
	}
	_, fromSize, ok := reinterpretableScalar(from)
	if !ok {
		return false
	}
	toRust, toSize, ok := reinterpretableScalar(to)
	if !ok || fromSize != toSize {
		return false
	}
	bitsType := "u64"
	if fromSize == 4 {
		bitsType = "u32"
	}
	// Through the unsigned integer of that size: f64::to_bits and from_bits
	// for floats, `as` between integers, which keeps the bits.
	var bits strings.Builder
	bits.WriteString("(")
	TranspileExpressionContext(&bits, value, RValue)
	bits.WriteString(")")
	switch {
	case from.Info()&types.IsFloat != 0:
		bits.WriteString(".to_bits()")
	default:
		bits.WriteString(" as ")
		bits.WriteString(bitsType)
	}
	switch {
	case to.Info()&types.IsFloat != 0:
		out.WriteString(toRust)
		out.WriteString("::from_bits(")
		out.WriteString(bits.String())
		out.WriteString(")")
	default:
		out.WriteString("(")
		out.WriteString(bits.String())
		out.WriteString(" as ")
		out.WriteString(toRust)
		out.WriteString(")")
	}
	return true
}

// reinterpretableScalar returns the Rust type and size in bytes of a number
// whose bits can be read as another number of the same size.
func reinterpretableScalar(basic *types.Basic) (string, int, bool) {
	switch basic.Kind() {
	case types.Float64:
		return "f64", 8, true
	case types.Float32:
		return "f32", 4, true
	case types.Uint64:
		return "u64", 8, true
	case types.Int64:
		return "i64", 8, true
	case types.Uint32:
		return "u32", 4, true
	case types.Int32:
		return "i32", 4, true
	}
	return "", 0, false
}
