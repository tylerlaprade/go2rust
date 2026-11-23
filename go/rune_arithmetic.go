package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

// A rune from ranging over a string is a Rust char, which has no
// arithmetic. Go code computes with such runes all the time: c - '0' for a
// digit, 'a' + (c-'a'+k)%26 for a Caesar shift. writeRangeRuneArithmetic
// writes an arithmetic expression of rune type that involves such a rune
// with every char operand, the rune and the rune literals alike, as an
// i32, which is how Go's rune is represented elsewhere.
func writeRangeRuneArithmetic(out *strings.Builder, e *ast.BinaryExpr) bool {
	if !isRuneArithmeticOp(e.Op) || !isRuneTypedExpr(e) || !runeArithmeticUsesRangeChar(e) {
		return false
	}
	writeRuneArithmeticOperand(out, e)
	return true
}

func isRuneArithmeticOp(op token.Token) bool {
	switch op {
	case token.ADD, token.SUB, token.MUL, token.QUO, token.REM, token.AND, token.OR, token.XOR, token.AND_NOT:
		return true
	}
	return false
}

func isRuneTypedExpr(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	basic, ok := types.Unalias(typ).(*types.Basic)
	return ok && (basic.Kind() == types.Int32 || basic.Kind() == types.UntypedRune)
}

func runeArithmeticUsesRangeChar(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.ParenExpr:
		return runeArithmeticUsesRangeChar(e.X)
	case *ast.BinaryExpr:
		if !isRuneArithmeticOp(e.Op) {
			return false
		}
		return runeArithmeticUsesRangeChar(e.X) || runeArithmeticUsesRangeChar(e.Y)
	case *ast.Ident:
		return rangeLoopVars[e.Name] == "char"
	}
	return false
}

func writeRuneArithmeticOperand(out *strings.Builder, expr ast.Expr) {
	switch e := expr.(type) {
	case *ast.ParenExpr:
		out.WriteString("(")
		writeRuneArithmeticOperand(out, e.X)
		out.WriteString(")")
		return
	case *ast.BinaryExpr:
		if isRuneArithmeticOp(e.Op) && isRuneTypedExpr(e) {
			writeRuneArithmeticOperand(out, e.X)
			out.WriteString(" ")
			out.WriteString(rustBinaryOp(e.Op))
			out.WriteString(" ")
			writeRuneArithmeticOperand(out, e.Y)
			return
		}
	case *ast.Ident:
		if rangeLoopVars[e.Name] == "char" {
			out.WriteString("(")
			out.WriteString(RustIdentForUse(e))
			out.WriteString(" as i32)")
			return
		}
	case *ast.BasicLit:
		if e.Kind == token.CHAR {
			out.WriteString("(")
			out.WriteString(RustCharLiteral(e.Value))
			out.WriteString(" as i32)")
			return
		}
	}
	out.WriteString("(")
	writeBareCompoundAssignValue(out, expr, types.Typ[types.Int32])
	out.WriteString(")")
}
