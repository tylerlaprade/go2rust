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
var currentSliceElemPtrSyntaxCandidates map[*ast.Ident]string

func setSliceElemPtrCandidates(body *ast.BlockStmt) func() {
	old := currentSliceElemPtrCandidates
	oldSyntax := currentSliceElemPtrSyntaxCandidates
	currentSliceElemPtrCandidates, currentSliceElemPtrSyntaxCandidates = collectSliceElemPtrCandidates(body)
	return func() {
		currentSliceElemPtrCandidates = old
		currentSliceElemPtrSyntaxCandidates = oldSyntax
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
	if currentSliceElemPtrSyntaxCandidates != nil {
		elemRustType, ok := currentSliceElemPtrSyntaxCandidates[name]
		return elemRustType, ok
	}
	return "", false
}

func collectSliceElemPtrCandidates(body *ast.BlockStmt) (map[types.Object]string, map[*ast.Ident]string) {
	typeInfo := GetTypeInfo()
	if body == nil {
		return nil, nil
	}

	candidates := map[types.Object]*sliceElemPtrCandidate{}
	syntaxCandidates := map[*ast.Ident]*sliceElemPtrCandidate{}
	syntaxCandidatesByName := map[string]*sliceElemPtrCandidate{}

	ast.Inspect(body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.ValueSpec:
			star, ok := n.Type.(*ast.StarExpr)
			if !ok {
				return true
			}
			elemRustType := goTypeToRustBase(star.X)
			for i, name := range n.Names {
				if name.Name == "_" {
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
				if typeInfo != nil {
					if obj := typeInfo.GetObject(name); obj != nil {
						candidates[obj] = state
						continue
					}
				}
				syntaxCandidates[name] = state
				syntaxCandidatesByName[name.Name] = state
			}
		}
		return true
	})

	if len(candidates) == 0 && len(syntaxCandidates) == 0 {
		return nil, nil
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
				if typeInfo != nil {
					if obj := typeInfo.GetObject(ident); obj != nil {
						state = candidates[obj]
					}
				}
				if state == nil {
					state = syntaxCandidatesByName[ident.Name]
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
	syntaxResult := map[*ast.Ident]string{}
	for obj, state := range candidates {
		if state.valid && state.sawSliceAddr {
			result[obj] = state.elemRustType
		}
	}
	for ident, state := range syntaxCandidates {
		if state.valid && state.sawSliceAddr {
			syntaxResult[ident] = state.elemRustType
		}
	}
	if len(result) == 0 {
		result = nil
	}
	if len(syntaxResult) == 0 {
		syntaxResult = nil
	}
	return result, syntaxResult
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
	unary, ok := expr.(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return false, false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return false, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return true, true
	}
	return !typeInfo.IsMap(indexExpr.X), !typeInfo.IsMap(indexExpr.X)
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

func writeSliceElemPtrDerefRead(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("{ let __v = (*")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(".as_ref().unwrap().borrow().as_ref().unwrap()).clone(); __v }")
}

func writeSliceElemPtrDerefLValue(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("(*")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(".as_ref().unwrap().borrow_mut().as_mut().unwrap())")
}
