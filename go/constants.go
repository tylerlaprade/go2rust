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
		return false
	}
	if _, isTypeDef := LookupTypeDefinition(expectedIdent.Name); !isTypeDef {
		return false
	}
	out.WriteString(expectedIdent.Name)
	out.WriteString("(")
	WriteWrapperPrefix(out)
	TranspileExpression(out, value)
	WriteWrapperSuffix(out)
	out.WriteString(")")
	return true
}

func writeWrappedExpressionForExpectedType(out *strings.Builder, value ast.Expr, expected ast.Expr) {
	WriteWrapperPrefix(out)
	if !writeExpressionForExpectedType(out, value, expected) {
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
