package main

import (
	"go/ast"
	"go/types"
	"strings"
)

func isConstIdent(ident *ast.Ident) bool {
	if ident == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	_, ok := typeInfo.GetObject(ident).(*types.Const)
	return ok
}

func rustConstName(name string) string {
	return strings.ToUpper(strings.TrimPrefix(ToSnakeCase(name), "r#"))
}

func writeExpressionForExpectedType(out *strings.Builder, value ast.Expr, expected ast.Expr) bool {
	expectedIdent, ok := expected.(*ast.Ident)
	if !ok {
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			return writeExpressionForExpectedTypesType(out, value, typeInfo.GetType(expected))
		}
		return false
	}
	underlying, isTypeDef := LookupTypeDefinition(expectedIdent.Name)
	if !isTypeDef {
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			return writeExpressionForExpectedTypesType(out, value, typeInfo.GetType(expected))
		}
		return false
	}
	out.WriteString(expectedIdent.Name)
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
		return rustConstTypeForGoTypesType(t.Underlying())
	case *types.Basic:
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
	writeExpressionForExpectedTypesType(out, expr, otherNamed)
	return true
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
