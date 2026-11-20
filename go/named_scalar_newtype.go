package main

import (
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
	"math"
	"strconv"
	"strings"
)

// namedFloatNewtype reports whether typ is a type the package defines over
// a float, "type Celsius float64", and the Rust float it holds.
func namedFloatNewtype(typ types.Type) (*types.Named, string, bool) {
	if !anyFormatTypeIsLocalNamedBasic(typ) {
		return nil, "", false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return nil, "", false
	}
	basic, ok := named.Underlying().(*types.Basic)
	if !ok || basic.Info()&types.IsFloat == 0 {
		return nil, "", false
	}
	if basic.Kind() == types.Float32 {
		return named, "f32", true
	}
	return named, "f64", true
}

// writeRustFloatLiteral writes a constant as a Rust float literal, the
// value Go gives it when it becomes a float64.
func writeRustFloatLiteral(out *strings.Builder, value constant.Value) bool {
	f, _ := constant.Float64Val(constant.ToFloat(value))
	if math.IsInf(f, 0) || math.IsNaN(f) {
		return false
	}
	text := strconv.FormatFloat(f, 'g', -1, 64)
	if !strings.ContainsAny(text, ".eE") {
		text += ".0"
	}
	if f < 0 {
		out.WriteString("(")
		out.WriteString(text)
		out.WriteString(")")
		return true
	}
	out.WriteString(text)
	return true
}

// writeNamedFloatPrimitiveExpression writes an expression of a named float
// type as the float it holds, so `c*9/5 + 32` with c a Celsius is f64
// arithmetic on the value inside c.
func writeNamedFloatPrimitiveExpression(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return false
	}
	_, rustType, ok := namedFloatNewtype(typeInfo.GetType(expr))
	if !ok {
		return false
	}
	if value, ok := constExpressionValue(expr); ok && isConstantExpression(expr) {
		return writeRustFloatLiteral(out, value)
	}
	switch e := expr.(type) {
	case *ast.ParenExpr:
		var inner strings.Builder
		if !writeNamedFloatPrimitiveExpression(&inner, e.X) {
			return false
		}
		out.WriteString("(")
		out.WriteString(inner.String())
		out.WriteString(")")
		return true
	case *ast.BinaryExpr:
		switch e.Op {
		case token.ADD, token.SUB, token.MUL, token.QUO:
		default:
			return false
		}
		var left, right strings.Builder
		if !writeNamedFloatPrimitiveOperand(&left, e.X) || !writeNamedFloatPrimitiveOperand(&right, e.Y) {
			return false
		}
		out.WriteString("(")
		out.WriteString(left.String())
		out.WriteString(" ")
		out.WriteString(e.Op.String())
		out.WriteString(" ")
		out.WriteString(right.String())
		out.WriteString(")")
		return true
	case *ast.UnaryExpr:
		if e.Op != token.SUB && e.Op != token.ADD {
			return false
		}
		var operand strings.Builder
		if !writeNamedFloatPrimitiveOperand(&operand, e.X) {
			return false
		}
		if e.Op == token.SUB {
			out.WriteString("-")
		}
		out.WriteString(operand.String())
		return true
	case *ast.CallExpr:
		if len(e.Args) == 1 && typeInfo.IsTypeConversion(e) {
			out.WriteString("(")
			arg := e.Args[0]
			if numericConversionCastNeedsParens(arg) {
				out.WriteString("(")
				writeNumericConversionValueForRustType(out, arg, rustType)
				out.WriteString(")")
			} else {
				writeNumericConversionValueForRustType(out, arg, rustType)
			}
			out.WriteString(" as ")
			out.WriteString(rustType)
			out.WriteString(")")
			return true
		}
	}
	if ident, ok := expr.(*ast.Ident); ok && isCurrentReceiverIdent(ident) && currentReceiverScalarTypeDefinition() {
		TranspileExpression(out, expr)
		return true
	}
	var value strings.Builder
	if ident, ok := expr.(*ast.Ident); ok && isVarBare(ident.Name) {
		value.WriteString(rustIdentForUseWithCapture(ident))
	} else if isExpressionResultBare(expr) {
		TranspileExpression(&value, expr)
	} else if typeInfo.ReturnsWrappedValue(expr) {
		value.WriteString("(*")
		TranspileExpressionContext(&value, expr, LValue)
		WriteBorrowMethod(&value, false)
		value.WriteString(".as_ref().unwrap())")
	} else {
		TranspileExpression(&value, expr)
	}
	// Behind a Mutex the guard is dropped at once: `(a-b)*(a+b)` reads the
	// value a holds twice in one statement.
	if NeedsConcurrentWrapper() {
		out.WriteString("{ let __named_float = *")
		out.WriteString(value.String())
		out.WriteString(".0")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap(); __named_float }")
		return true
	}
	out.WriteString("(*")
	out.WriteString(value.String())
	out.WriteString(".0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
	return true
}

func writeNamedFloatPrimitiveOperand(out *strings.Builder, expr ast.Expr) bool {
	if value, ok := constExpressionValue(expr); ok && isConstantExpression(expr) {
		return writeRustFloatLiteral(out, value)
	}
	return writeNamedFloatPrimitiveExpression(out, expr)
}

// writeNamedFloatArithmeticValue writes arithmetic on a named float as a new
// value of that type. The newtype's operator impls take their operands by
// value, and an operand read from a variable is behind a borrow.
func writeNamedFloatArithmeticValue(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	named, _, ok := namedFloatNewtype(typeInfo.GetType(expr))
	if !ok {
		return false
	}
	var primitive strings.Builder
	if !writeNamedFloatPrimitiveExpression(&primitive, expr) {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	out.WriteString(primitive.String())
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

// writeNamedScalarConstValue writes a constant of a type the package
// defines over a float, bool or string as a value of that type. The Rust
// constant holds the basic value; `c := Red` with `const Red Color = "red"`
// needs a Color. Named integer constants are converted where they are used.
func writeNamedScalarConstValue(out *strings.Builder, ident *ast.Ident, rustName string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(ident)
	if !anyFormatTypeIsLocalNamedBasic(typ) || isNamedIntegerType(typ) {
		return false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return false
	}
	basic, ok := named.Underlying().(*types.Basic)
	if !ok {
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	out.WriteString(rustName)
	if basic.Info()&types.IsString != 0 {
		out.WriteString(".to_string()")
	}
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

// writeNamedStringNewtypeInnerValue writes a copy of the String a named
// string value holds, for the string operations Go allows on it: len,
// indexing, slicing and ranging.
func writeNamedStringNewtypeInnerValue(out *strings.Builder, expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if !anyFormatTypeIsLocalNamedBasic(typ) {
		return false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return false
	}
	if basic, ok := named.Underlying().(*types.Basic); !ok || basic.Kind() != types.String {
		return false
	}
	if isConstantExpression(expr) {
		if ident, ok := expr.(*ast.Ident); ok && isConstIdent(ident) && !isLocalConstantIdent(ident) {
			out.WriteString(rustConstName(ident.Name))
			out.WriteString(".to_string()")
			return true
		}
		var literal strings.Builder
		if !writeConstStringLiteralValue(&literal, expr) {
			return false
		}
		out.WriteString(literal.String())
		out.WriteString(".to_string()")
		return true
	}
	writeNamedStringUnderlyingClone(out, expr, named)
	return true
}

// writeNamedScalarLiteralValue writes a literal that go/types gave a type
// the package defines over a float or string as a value of that type:
// `label("other")` with `func label(s Status)` passes a Status.
func writeNamedScalarLiteralValue(out *strings.Builder, lit *ast.BasicLit) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || lit == nil {
		return false
	}
	typ := typeInfo.GetType(lit)
	if !anyFormatTypeIsLocalNamedBasic(typ) || isNamedIntegerType(typ) {
		return false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return false
	}
	basic, ok := named.Underlying().(*types.Basic)
	if !ok {
		return false
	}
	var value strings.Builder
	switch {
	case basic.Info()&types.IsString != 0 && lit.Kind == token.STRING:
		value.WriteString(RustStringLiteral(lit.Value))
		value.WriteString(".to_string()")
	case basic.Info()&types.IsFloat != 0:
		constValue, ok := constExpressionValue(lit)
		if !ok || !writeRustFloatLiteral(&value, constValue) {
			return false
		}
	default:
		return false
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	out.WriteString(value.String())
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

// writeNamedFloatValueForExpected writes a value for a slot of a named
// float type from a constant or from a float expression of another type.
func writeNamedFloatValueForExpected(out *strings.Builder, value ast.Expr, named *types.Named) bool {
	_, rustType, ok := namedFloatNewtype(named)
	if !ok {
		return false
	}
	var inner strings.Builder
	if constValue, ok := constExpressionValue(value); ok && isConstantExpression(value) {
		if !writeRustFloatLiteral(&inner, constValue) {
			return false
		}
	} else {
		if numericConversionCastNeedsParens(value) {
			inner.WriteString("(")
			writeNumericConversionValueForRustType(&inner, value, rustType)
			inner.WriteString(")")
		} else {
			writeNumericConversionValueForRustType(&inner, value, rustType)
		}
		inner.WriteString(" as ")
		inner.WriteString(rustType)
	}
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(out)
	out.WriteString(inner.String())
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

// writeNamedFloatCompoundAssignUnderlyingValue writes the float a named
// float holds for a compound assignment into a float slot: `*c += by` in a
// method of Celsius updates the f64 the receiver holds.
func writeNamedFloatCompoundAssignUnderlyingValue(out *strings.Builder, expr ast.Expr, expected types.Type) bool {
	basic, ok := types.Unalias(expected).(*types.Basic)
	if !ok || basic.Info()&types.IsFloat == 0 {
		return false
	}
	return writeNamedFloatPrimitiveExpression(out, expr)
}

// writeNamedScalarVarSpecValue writes the initial value of `var c Celsius
// = 21.5` or `var s Status = other`: a value of the declared type, which
// the literal or the variable already is.
func writeNamedScalarVarSpecValue(out *strings.Builder, valueSpec *ast.ValueSpec, i int) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || valueSpec == nil || valueSpec.Type == nil || i >= len(valueSpec.Values) {
		return false
	}
	declared := typeInfo.GetType(valueSpec.Type)
	if !anyFormatTypeIsLocalNamedBasic(declared) || isNamedIntegerType(declared) {
		return false
	}
	named, ok := types.Unalias(declared).(*types.Named)
	if !ok {
		return false
	}
	value := valueSpec.Values[i]
	var inner strings.Builder
	if !writeExpressionForExpectedTypesType(&inner, value, named) {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(inner.String())
	WriteWrapperSuffix(out)
	return true
}
