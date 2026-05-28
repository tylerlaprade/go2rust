package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

type sliceElemPtrCandidate struct {
	elemRustType string
	valid        bool
	sawSliceAddr bool
}

var currentSliceElemPtrCandidates map[types.Object]string

func setSliceElemPtrCandidates(body *ast.BlockStmt) func() {
	old := currentSliceElemPtrCandidates
	currentSliceElemPtrCandidates = collectSliceElemPtrCandidates(body)
	return func() {
		currentSliceElemPtrCandidates = old
	}
}

func sliceElemPtrCandidateForDecl(name *ast.Ident) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && currentSliceElemPtrCandidates != nil {
		if obj := typeInfo.GetObject(name); obj != nil {
			if elemRustType, ok := currentSliceElemPtrCandidates[obj]; ok {
				return elemRustType, true
			}
		}
	}
	return "", false
}

func collectSliceElemPtrCandidates(body *ast.BlockStmt) map[types.Object]string {
	typeInfo := GetTypeInfo()
	if body == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}

	candidates := map[types.Object]*sliceElemPtrCandidate{}

	ast.Inspect(body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.ValueSpec:
			if _, ok := n.Type.(*ast.StarExpr); !ok {
				return true
			}
			for i, name := range n.Names {
				if name.Name == "_" {
					continue
				}
				obj := typeInfo.GetObject(name)
				if obj == nil {
					continue
				}
				elemRustType, ok := sliceElemPtrRustTypeForPointerType(obj.Type())
				if !ok {
					continue
				}
				state := &sliceElemPtrCandidate{
					elemRustType: elemRustType,
					valid:        true,
				}
				if len(n.Values) > i {
					ok, sawSliceAddr := isSliceElemPtrAssignmentValue(n.Values[i])
					state.valid = ok
					state.sawSliceAddr = sawSliceAddr
				}
				candidates[obj] = state
			}
		case *ast.AssignStmt:
			if n.Tok != token.DEFINE {
				return true
			}
			for i, lhs := range n.Lhs {
				ident, ok := lhs.(*ast.Ident)
				if !ok || ident.Name == "_" {
					continue
				}
				if typeInfo == nil {
					continue
				}
				obj := typeInfo.GetObject(ident)
				if obj == nil {
					continue
				}
				if _, exists := candidates[obj]; exists {
					continue
				}
				elemRustType, ok := sliceElemPtrRustTypeForPointerType(obj.Type())
				if !ok {
					continue
				}
				rhs := assignmentRHSForLHS(n, i)
				if rhs == nil {
					continue
				}
				rhsOk, sawSliceAddr := isSliceElemPtrAssignmentValue(rhs)
				if !rhsOk || !sawSliceAddr {
					continue
				}
				candidates[obj] = &sliceElemPtrCandidate{
					elemRustType: elemRustType,
					valid:        true,
					sawSliceAddr: true,
				}
			}
		}
		return true
	})

	if len(candidates) == 0 {
		return nil
	}

	ast.Inspect(body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.AssignStmt:
			for i, lhs := range n.Lhs {
				ident, ok := lhs.(*ast.Ident)
				if !ok {
					continue
				}
				var state *sliceElemPtrCandidate
				if obj := typeInfo.GetObject(ident); obj != nil {
					state = candidates[obj]
				}
				if state == nil {
					continue
				}
				rhs := assignmentRHSForLHS(n, i)
				if rhs == nil {
					state.valid = false
					continue
				}
				ok, sawSliceAddr := isSliceElemPtrAssignmentValue(rhs)
				if !ok {
					state.valid = false
					continue
				}
				if sawSliceAddr {
					state.sawSliceAddr = true
				}
			}
		}
		return true
	})

	result := map[types.Object]string{}
	for obj, state := range candidates {
		if state.valid && state.sawSliceAddr {
			result[obj] = state.elemRustType
		}
	}
	if len(result) == 0 {
		result = nil
	}
	return result
}

func sliceElemPtrRustTypeForPointerType(t types.Type) (string, bool) {
	if t == nil {
		return "", false
	}
	ptr, ok := types.Unalias(t).Underlying().(*types.Pointer)
	if !ok {
		return "", false
	}
	return goTypesTypeToRust(coreType(ptr.Elem())), true
}

func assignmentRHSForLHS(stmt *ast.AssignStmt, lhsIndex int) ast.Expr {
	if len(stmt.Rhs) == len(stmt.Lhs) {
		return stmt.Rhs[lhsIndex]
	}
	if len(stmt.Rhs) == 1 && len(stmt.Lhs) == 1 {
		return stmt.Rhs[0]
	}
	return nil
}

func isSliceElemPtrAssignmentValue(expr ast.Expr) (bool, bool) {
	expr = unwrapParens(expr)
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
		return true, false
	}
	_, ok := sliceElemPtrAddressElemRustType(expr)
	return ok, ok
}

func sliceElemPtrAddressElemRustType(expr ast.Expr) (string, bool) {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return "", false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.GetType(indexExpr.X) == nil || typeInfo.IsMap(indexExpr.X) {
		return "", false
	}
	elemType := typeInfo.GetArrayOrSliceElemType(indexExpr.X)
	if elemType == nil {
		return "", false
	}
	return goTypesTypeToRust(elemType), true
}

func unwrapParens(expr ast.Expr) ast.Expr {
	for {
		paren, ok := expr.(*ast.ParenExpr)
		if !ok {
			return expr
		}
		expr = paren.X
	}
}

func writeSliceElemPtrOptionValue(out *strings.Builder, rhs ast.Expr) bool {
	if ok, _ := isSliceElemPtrAssignmentValue(rhs); !ok {
		return false
	}
	rhs = unwrapParens(rhs)
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("None")
		return true
	}
	out.WriteString("Some(")
	TranspileExpression(out, rhs)
	out.WriteString(")")
	return true
}

func writeUnsupportedSliceElemPointerHandleValue(out *strings.Builder, rhs ast.Expr, message string) bool {
	if _, ok := sliceElemPtrAddressElemRustType(rhs); !ok {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(`unimplemented!("`)
	out.WriteString(message)
	out.WriteString(`")`)
	WriteWrapperSuffix(out)
	return true
}

func writeSliceElemPtrDerefAssignmentValue(out *strings.Builder, target *ast.StarExpr, rhs ast.Expr) bool {
	ident, ok := rhs.(*ast.Ident)
	if !ok || ident.Name != "nil" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	targetType := typeInfo.GetType(target)
	if targetType == nil {
		return false
	}
	if _, ok := types.Unalias(targetType).Underlying().(*types.Pointer); !ok {
		return false
	}
	out.WriteString(zeroValueForTypesType(targetType))
	return true
}

func writeSliceElemPtrDerefRead(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("{ let __v = (*")
	writeSliceElemPtrBorrow(out, ident, false)
	out.WriteString(".as_ref().unwrap()).clone(); __v }")
}

func writeSliceElemPtrDerefLValue(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("(*")
	writeSliceElemPtrBorrow(out, ident, true)
	out.WriteString(".as_mut().unwrap())")
}

func writeSliceElemPtrBorrow(out *strings.Builder, ident *ast.Ident, mutable bool) {
	out.WriteString(RustIdentForUse(ident))
	if info, ok := sliceElemPtrVarInfo(ident.Name); ok && info.WrapLevel == WrapOption {
		out.WriteString(".as_ref().unwrap()")
	}
	if mutable {
		out.WriteString(".borrow_mut()")
	} else {
		out.WriteString(".borrow()")
	}
}

func writeSliceElemPtrFieldHandle(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo) {
	out.WriteString("(*")
	writeSliceElemPtrBorrow(out, ident, false)
	out.WriteString(".as_ref().unwrap())")
	if fieldInfo.IsPromoted {
		for _, embedded := range fieldInfo.EmbeddedPath {
			out.WriteString(".")
			out.WriteString(ToSnakeCase(embedded))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()")
		}
	}
	out.WriteString(".")
	out.WriteString(fieldInfo.FieldName)
}

func writeSliceElemPtrFieldSelector(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo, sel *ast.SelectorExpr, ctx ExprContext) bool {
	if !isSliceElemPtrVar(ident.Name) {
		return false
	}
	if ctx == LValue || ctx == AddressOf {
		writeSliceElemPtrFieldHandle(out, ident, fieldInfo)
		return true
	}
	if typeInfoIsPointerExpr(sel) || selectorExpressionKeepsHandle(sel) {
		writeSliceElemPtrFieldHandle(out, ident, fieldInfo)
		out.WriteString(".clone()")
		return true
	}
	out.WriteString("(*")
	if NeedsConcurrentWrapper() {
		out.WriteString("{ let __field = ")
		writeSliceElemPtrFieldHandle(out, ident, fieldInfo)
		out.WriteString(".clone(); __field }")
	} else {
		writeSliceElemPtrFieldHandle(out, ident, fieldInfo)
	}
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	writeSelectorRValueClose(out, sel)
	return true
}
