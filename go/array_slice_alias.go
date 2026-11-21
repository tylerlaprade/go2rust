package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

// currentSliceBackedArrayObjs contains local fixed-array variables whose
// storage must be shared with a slice expression such as a[:] or a[lo:hi].
// Ordinary arrays keep their fixed Rust array representation; only arrays
// that are actually sliced use GoSlice as their backing store.
var currentSliceBackedArrayObjs map[types.Object]bool

// emitFixedArrayAsGoSlice is scoped to the initializer of a marked local.
// The composite-literal writer otherwise has no parent link back to the
// declaration whose representation was selected by the pre-pass.
var emitFixedArrayAsGoSlice bool

func setSliceBackedArrayCandidates(body *ast.BlockStmt) func() {
	previous := currentSliceBackedArrayObjs
	currentSliceBackedArrayObjs = collectSliceBackedArrayCandidates(body)
	return func() { currentSliceBackedArrayObjs = previous }
}

func setSliceBackedArrayCandidatesForFunc(fn *ast.FuncDecl) func() {
	if fn == nil {
		return setSliceBackedArrayCandidates(nil)
	}
	return setSliceBackedArrayCandidates(fn.Body)
}

func collectSliceBackedArrayCandidates(body *ast.BlockStmt) map[types.Object]bool {
	typeInfo := GetTypeInfo()
	if body == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	locals := make(map[types.Object]bool)
	ast.Inspect(body, func(node ast.Node) bool {
		if _, nested := node.(*ast.FuncLit); nested {
			return false
		}
		addIdent := func(expr ast.Expr) {
			ident, ok := expr.(*ast.Ident)
			if !ok || ident.Name == "_" {
				return
			}
			if obj := typeInfo.GetObject(ident); obj != nil {
				locals[obj] = true
			}
		}
		switch current := node.(type) {
		case *ast.AssignStmt:
			if current.Tok == token.DEFINE {
				for _, lhs := range current.Lhs {
					addIdent(lhs)
				}
			}
		case *ast.ValueSpec:
			for _, name := range current.Names {
				addIdent(name)
			}
		case *ast.RangeStmt:
			if current.Tok == token.DEFINE {
				addIdent(current.Key)
				addIdent(current.Value)
			}
		}
		return true
	})
	result := make(map[types.Object]bool)
	ast.Inspect(body, func(node ast.Node) bool {
		switch node.(type) {
		case *ast.FuncLit:
			return false
		}
		slice, ok := node.(*ast.SliceExpr)
		if !ok {
			return true
		}
		ident, ok := unwrapParens(slice.X).(*ast.Ident)
		if !ok {
			return true
		}
		obj := typeInfo.GetObject(ident)
		variable, ok := obj.(*types.Var)
		if !ok || variable.IsField() || !locals[obj] {
			return true
		}
		if _, named := types.Unalias(variable.Type()).(*types.Named); named {
			return true
		}
		if _, ok := types.Unalias(variable.Type()).Underlying().(*types.Array); ok {
			result[obj] = true
		}
		return true
	})
	return result
}

func sliceBackedArrayObject(obj types.Object) bool {
	return obj != nil && currentSliceBackedArrayObjs != nil && currentSliceBackedArrayObjs[obj]
}

func sliceBackedArrayCandidateForDecl(ident *ast.Ident) bool {
	if ident == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && sliceBackedArrayObject(typeInfo.GetObject(ident))
}

func isSliceBackedArrayExpr(expr ast.Expr) bool {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && sliceBackedArrayObject(typeInfo.GetObject(ident))
}

func withFixedArrayAsGoSlice(enabled bool, emit func()) {
	previous := emitFixedArrayAsGoSlice
	emitFixedArrayAsGoSlice = enabled
	defer func() { emitFixedArrayAsGoSlice = previous }()
	emit()
}

func sliceBackedArrayWrappedRustType(typ types.Type) (string, bool) {
	array, ok := types.Unalias(typ).Underlying().(*types.Array)
	if !ok {
		return "", false
	}
	return goTypesTypeToRustWrapped(types.NewSlice(array.Elem())), true
}

func writeSliceBackedArrayZeroValue(out *strings.Builder, typ types.Type) bool {
	array, ok := types.Unalias(typ).Underlying().(*types.Array)
	if !ok {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString("Vec::from(")
	out.WriteString(zeroValueForTypesType(array))
	out.WriteString(")")
	WriteWrapperSuffix(out)
	return true
}

// writeSliceBackedArrayInitializerCopy converts an existing fixed-array value
// into the GoSlice backing selected for a local that is later sliced. Function
// results and ordinary array variables still use the fixed-array
// representation, so assigning either one directly would leave the marked
// local with a Rust type that cannot support reslicing.
func writeSliceBackedArrayInitializerCopy(out *strings.Builder, lhs, rhs ast.Expr) bool {
	lhsIdent, ok := lhs.(*ast.Ident)
	if !ok || !sliceBackedArrayCandidateForDecl(lhsIdent) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	rhsType := typeInfo.GetType(rhs)
	if rhsType == nil {
		return false
	}
	if _, ok := types.Unalias(rhsType).Underlying().(*types.Array); !ok {
		return false
	}
	_, rhsIsNamedArray := types.Unalias(rhsType).(*types.Named)

	var source strings.Builder
	switch value := rhs.(type) {
	case *ast.Ident:
		if value.Name == "_" || value.Name == "nil" {
			return false
		}
		name := RustIdentForUse(value)
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[value.Name]; exists {
				name = RustLocalIdent(renamed)
			}
		}
		source.WriteString(name)
		source.WriteString(".clone()")
	case *ast.CallExpr:
		// Named-array call results still have a separate return-representation
		// gap. Do not guess whether the generated call returns the newtype or
		// its underlying fixed-array handle here.
		if rhsIsNamedArray {
			return false
		}
		registerCallResultSyntaxInfo(lhs, value)
		writeCallExpressionForInitializer(&source, value)
	default:
		return false
	}
	out.WriteString("{ let __go_array = ")
	out.WriteString(source.String())
	out.WriteString("; let __owned = ")
	var value strings.Builder
	value.WriteString("__go_array")
	if rhsIsNamedArray {
		WriteBorrowMethod(&value, false)
		value.WriteString(".as_ref().unwrap().0")
	}
	WriteBorrowMethod(&value, false)
	value.WriteString(".as_ref().unwrap()")
	if !writeSliceBackedArrayValueCopy(out, value.String(), rhsType) {
		out.WriteString(goValueCopyNeedsTypeInfo)
	}
	out.WriteString("; ")
	WriteWrapperPrefix(out)
	out.WriteString("__owned")
	WriteWrapperSuffix(out)
	out.WriteString(" }")
	return true
}

func writeSliceBackedArrayValueCopy(out *strings.Builder, valueExpr string, typ types.Type) bool {
	array, ok := types.Unalias(typ).Underlying().(*types.Array)
	if !ok {
		return false
	}
	elemCopy := goValueCopyCallForType(array.Elem())
	out.WriteString("Vec::from(")
	out.WriteString(valueExpr)
	out.WriteString(".as_slice()")
	if elemCopy == ".clone()" {
		out.WriteString(".to_vec()")
	} else {
		out.WriteString(".iter().map(|__go_elem| __go_elem")
		out.WriteString(elemCopy)
		out.WriteString(").collect::<std::vec::Vec<_>>())")
		return true
	}
	out.WriteString(")")
	return true
}

func writeSliceBackedArraySliceExpression(out *strings.Builder, slice *ast.SliceExpr) bool {
	if slice == nil || !isSliceBackedArrayExpr(slice.X) || !goSliceModeEnabled() {
		return false
	}
	WriteWrapperPrefix(out)
	if sliceCopyShouldUseMultiline(slice) {
		closeIndent := currentLineIndent(out)
		indent := closeIndent + "    "
		out.WriteString("{\n")
		out.WriteString(indent)
		out.WriteString("let mut __seq = ")
		writeClonedWrappedExpression(out, slice.X, "__seq_holder", "__seq_guard")
		writeSliceVecFromSeqMultiline(out, slice.Low, slice.High, slice.Max, "__seq.capacity()", true, indent, closeIndent)
	} else {
		out.WriteString("{ let mut __seq = ")
		writeClonedWrappedExpression(out, slice.X, "__seq_holder", "__seq_guard")
		writeSliceVecFromSeq(out, slice.Low, slice.High, slice.Max, "__seq.capacity()", true)
	}
	WriteWrapperSuffix(out)
	return true
}
