package main

import (
	"go/ast"
	"go/types"
	"strings"
)

// A struct in the generated Rust is a bundle of cells, one for each field.
// Its Clone clones the handles of those cells, so a clone is a second handle
// on the same struct. The run-time helpers and the code for pointer
// receivers rely on that: they clone a struct out of its lock and keep
// working on the same fields.
//
// Go's copy of a struct is a different operation: every field that Go copies
// gets a cell of its own. That is __go_value_clone. Wherever Go copies a
// value (an assignment, an argument, a result, a range variable, a value
// receiver) the generated code must use the call this file returns, not
// Clone. Using Clone there made "b := a; b.X = 9" change a.

const goStructValueCopyCall = ".__go_value_clone()"

// typeNeedsGoValueClone reports whether Go's copy of a value of this type
// is the struct copy helper: a struct type that the program or a translated
// package declares. A struct of a hand-written standard-library stub has no
// such helper.
func typeNeedsGoValueClone(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		// A stub's type has no copy helper; a package translated from
		// source declares its structs as the program's are.
		if obj := named.Obj(); obj != nil && obj.Pkg() != nil && isStubBackedStdlibPackagePath(obj.Pkg().Path()) {
			return false
		}
		switch named.Underlying().(type) {
		case *types.Struct:
			return true
		case *types.Array:
			// "type Grid [3]Cell" is a newtype around a shared cell, so its
			// Clone is the same array again. writeNamedArrayValueClone
			// gives it the copy helper.
			return namedArrayHasGoValueClone(named)
		}
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Struct)
	return ok
}

// namedArrayHasGoValueClone reports whether the declaration of the named
// array type gets the copy helper: every one without type parameters does.
func namedArrayHasGoValueClone(named *types.Named) bool {
	return named.TypeParams().Len() == 0 && named.TypeArgs().Len() == 0
}

// writeNamedArrayValueClone gives the newtype of a named array type its Go
// copy. The newtype holds the array in a cell and derives Clone, so a clone
// shares the array: "b := a; b[0] = 9" changed a. The copy is a new cell
// holding Go's copy of each element.
// writeNamedArrayPartialEq gives "type d [3]rune" Go's ==, which compares
// the elements. Each side is read under its own borrow or lock, one after
// the other, as the struct equality does.
func writeNamedArrayPartialEq(out *strings.Builder, typeSpec *ast.TypeSpec, rustTypeName string) {
	arrayAST, ok := typeSpec.Type.(*ast.ArrayType)
	if !ok || arrayAST.Len == nil {
		return
	}
	if typeSpec.TypeParams != nil && len(typeSpec.TypeParams.List) > 0 {
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	obj := typeInfo.info.Defs[typeSpec.Name]
	if obj == nil {
		return
	}
	array, ok := obj.Type().Underlying().(*types.Array)
	if !ok {
		return
	}
	if !arrayCellIsPrintedBasic(array.Elem()) {
		return
	}
	out.WriteString("\nimpl PartialEq for ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	out.WriteString("    fn eq(&self, other: &Self) -> bool {\n")
	out.WriteString("        let __left = self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".clone();\n")
	out.WriteString("        let __right = other.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".clone();\n")
	out.WriteString("        __left == __right\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

// arrayCellIsPrintedBasic reports whether elem is a bool, number or string,
// or an unnamed array of them at any depth: "[3]int" and "[2][2]int". Such an
// array is a plain Rust array, which compares and prints its cells.
func arrayCellIsPrintedBasic(elem types.Type) bool {
	for {
		elem = types.Unalias(elem)
		array, ok := elem.(*types.Array)
		if !ok {
			break
		}
		elem = array.Elem()
	}
	basic, ok := elem.(*types.Basic)
	return ok && basic.Info()&(types.IsBoolean|types.IsInteger|types.IsFloat|types.IsString) != 0
}

func writeNamedArrayValueClone(out *strings.Builder, typeSpec *ast.TypeSpec, rustTypeName string) {
	arrayAST, ok := typeSpec.Type.(*ast.ArrayType)
	if !ok || arrayAST.Len == nil {
		return
	}
	if typeSpec.TypeParams != nil && len(typeSpec.TypeParams.List) > 0 {
		return
	}
	elementsCopy := ""
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
		if obj := typeInfo.info.Defs[typeSpec.Name]; obj != nil {
			if array, ok := obj.Type().Underlying().(*types.Array); ok {
				elementsCopy = goValueCopyCallForType(array)
			}
		}
	}
	out.WriteString("\nimpl ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	out.WriteString("    pub fn __go_value_clone(&self) -> Self {\n")
	out.WriteString("        ")
	if elementsCopy == "" {
		out.WriteString(goValueCopyNeedsTypeInfo)
	} else {
		out.WriteString(rustTypeName)
		out.WriteString("(")
		WriteWrapperOptionPrefix(out)
		out.WriteString("self.0")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().map(|__go_array| __go_array")
		out.WriteString(elementsCopy)
		out.WriteString(")")
		WriteWrapperOptionSuffix(out)
		out.WriteString(")")
	}
	out.WriteString("\n    }\n}\n")
}

// goValueCopyCallForType is what to append to a reference to a value of this
// Go type to get Go's copy of it.
//
// An array is copied element by element when its elements need the struct
// copy; the array's own Clone would clone each element's handles.
func goValueCopyCallForType(typ types.Type) string {
	if typeNeedsGoValueClone(typ) {
		return goStructValueCopyCall
	}
	if typ != nil {
		if array, ok := types.Unalias(typ).(*types.Array); ok {
			if elem := goValueCopyCallForType(array.Elem()); elem != ".clone()" {
				return ".each_ref().map(|__go_elem| __go_elem" + elem + ")"
			}
		}
	}
	return ".clone()"
}

// goValueCopyNeedsTypeInfo is written in place of a copy whose type the type
// checker did not supply. Choosing Clone there would be a guess, and a wrong
// guess is the bug this file exists to fix: a struct that silently shares its
// fields.
const goValueCopyNeedsTypeInfo = `unimplemented!("type info required to copy a value")`

// goValueCopyCallForExpr is goValueCopyCallForType for the type of expr.
// known is false when there is no type for expr; the caller then writes
// goValueCopyNeedsTypeInfo instead of a copy.
func goValueCopyCallForExpr(expr ast.Expr) (call string, known bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return "", false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return "", false
	}
	return goValueCopyCallForType(typ), true
}

// currentReceiverIsStructValue reports whether the method being written has
// a value receiver of a struct type that has the copy helper.
func currentReceiverIsStructValue() bool {
	if currentReceiverObject == nil {
		return false
	}
	typ := currentReceiverObject.Type()
	if _, isPointer := types.Unalias(typ).(*types.Pointer); isPointer {
		return false
	}
	return typeNeedsGoValueClone(typ)
}

// writeValueOfVariableAsGoCopy writes the value of expr for an assignment
// when expr is a variable whose Go copy is not Clone, and reports whether it
// did. The ordinary read of a variable ends in Clone, which for a struct is a
// second handle on the same fields, so Go's copy is applied to it. For any
// other expression it writes nothing and reports false.
func writeValueOfVariableAsGoCopy(out *strings.Builder, expr ast.Expr) bool {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	if _, isVariable := typeInfo.info.Uses[ident].(*types.Var); !isVariable {
		return false
	}
	copyCall, known := goValueCopyCallForExpr(ident)
	if !known {
		out.WriteString(goValueCopyNeedsTypeInfo)
		return true
	}
	if copyCall == ".clone()" {
		return false
	}
	writeExpressionWithGoCopy(out, expr, copyCall)
	return true
}

// writeExpressionWithGoCopy writes expr followed by copyCall, unless the
// lowering of expr already ends in that copy: reading "*receiver" does, and
// so does reading a locked variable, which wraps the copy in a block.
func writeExpressionWithGoCopy(out *strings.Builder, expr ast.Expr, copyCall string) {
	var value strings.Builder
	TranspileExpression(&value, expr)
	out.WriteString(value.String())
	if strings.HasSuffix(value.String(), copyCall) || strings.HasSuffix(value.String(), copyCall+"; __v }") {
		return
	}
	out.WriteString(copyCall)
}

// elementGoValueCopyCall is goValueCopyCallForType for the element type of
// the slice or array rangeExpr: what a range loop, a spread append and copy()
// take one element at a time. known is false when the type checker has no
// type for the expression.
func elementGoValueCopyCall(rangeExpr ast.Expr) (call string, known bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || rangeExpr == nil {
		return "", false
	}
	typ := typeInfo.GetType(rangeExpr)
	if typ == nil {
		return "", false
	}
	switch seq := types.Unalias(typ).Underlying().(type) {
	case *types.Slice:
		return goValueCopyCallForType(seq.Elem()), true
	case *types.Array:
		return goValueCopyCallForType(seq.Elem()), true
	case *types.Pointer:
		if array, ok := types.Unalias(seq.Elem()).Underlying().(*types.Array); ok {
			return goValueCopyCallForType(array.Elem()), true
		}
	}
	return ".clone()", true
}

// rangeExprIsArrayValue reports whether a range loop goes over an array
// value. Go copies the array before the loop starts, so the loop does not see
// writes made to the array while it runs. Over a slice, or a pointer to an
// array, each element is read when its turn comes.
func rangeExprIsArrayValue(rangeExpr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || rangeExpr == nil {
		return false
	}
	typ := typeInfo.GetType(rangeExpr)
	if typ == nil {
		return false
	}
	_, isArray := types.Unalias(typ).Underlying().(*types.Array)
	return isArray
}

// goCopiedPlaceExpr reports whether expr reads an existing value in a way
// whose ordinary lowering ends in Clone: an element of a slice or an array
// ("s[i]") or the target of a pointer ("*p"). For a struct that Clone is a
// second handle on the same fields, which is right when the read is the
// base of a field access or a method call and wrong when the value is
// stored somewhere else. It returns Go's copy call for the value, and
// whether the type checker knew its type.
func goCopiedPlaceExpr(expr ast.Expr) (copyCall string, known bool, isPlace bool) {
	expr = unwrapParens(expr)
	switch place := expr.(type) {
	case *ast.IndexExpr:
		typeInfo := GetTypeInfo()
		if typeInfo == nil {
			return "", false, true
		}
		sequenceType := typeInfo.GetType(place.X)
		if sequenceType == nil {
			return "", false, true
		}
		switch types.Unalias(sequenceType).Underlying().(type) {
		case *types.Slice, *types.Array:
		default:
			// A map lookup, a generic instantiation and anything else keep
			// their own lowering.
			return "", true, false
		}
	case *ast.StarExpr:
	default:
		return "", true, false
	}
	copyCall, known = goValueCopyCallForExpr(expr)
	return copyCall, known, true
}

// writeWrappedGoValueCopyOfElement writes the value of "s[i]" or "*p" as a
// new variable's cell when the value is one whose Go copy is not Clone:
// "e := s[0]; e.X = 1" changed the slice. It reports false, and writes
// nothing, for any other expression.
func writeWrappedGoValueCopyOfElement(out *strings.Builder, expr ast.Expr) bool {
	copyCall, known, isPlace := goCopiedPlaceExpr(expr)
	if !isPlace {
		return false
	}
	if !known {
		WriteWrapperPrefix(out)
		out.WriteString(goValueCopyNeedsTypeInfo)
		WriteWrapperSuffix(out)
		return true
	}
	if copyCall == ".clone()" {
		return false
	}
	WriteWrapperPrefix(out)
	writeExpressionWithGoCopy(out, expr, copyCall)
	WriteWrapperSuffix(out)
	return true
}

// writeGoValueCopyOfElement is writeWrappedGoValueCopyOfElement without the
// cell: the copied value itself, for an assignment to an existing variable.
func writeGoValueCopyOfElement(out *strings.Builder, expr ast.Expr) bool {
	copyCall, known, isPlace := goCopiedPlaceExpr(expr)
	if !isPlace {
		return false
	}
	if !known {
		out.WriteString(goValueCopyNeedsTypeInfo)
		return true
	}
	if copyCall == ".clone()" {
		return false
	}
	writeExpressionWithGoCopy(out, expr, copyCall)
	return true
}

// goElementCopyIter returns the iterator over Go copies of the elements of
// the slice expression sliceExpr, to follow the Rust value of that slice:
// "append(dst, src...)" gives dst its own copy of every element of src.
func goElementCopyIter(sliceExpr ast.Expr) string {
	// A []any element is a Box<dyn Any>, which has no Clone: append(dst,
	// src...) copies each with go_any_clone.
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if slice, ok := types.Unalias(typeInfo.GetType(sliceExpr)).(*types.Slice); ok && isEmptyInterfaceType(slice.Elem()) {
			NeedAnyClone()
			return ".iter().map(|__go_elem| go_any_clone(__go_elem.as_ref()))"
		}
	}
	copyCall, known := elementGoValueCopyCall(sliceExpr)
	if !known {
		return ".iter().map(|_| " + goValueCopyNeedsTypeInfo + ")"
	}
	if copyCall == ".clone()" {
		return ".iter().cloned()"
	}
	return ".iter().map(|__go_elem| __go_elem" + copyCall + ")"
}

// rangeVarGoValueCopyCall returns the call that makes Go's copy of the value
// a range loop variable holds. The temporary of "for _, p = range s" is made
// by the translator and unknown to the type checker, so the type the loop
// recorded for it comes first. known is false when neither has a type.
func rangeVarGoValueCopyCall(ident *ast.Ident) (call string, known bool) {
	if typ := rangeLoopVarGoTypes[ident.Name]; typ != nil {
		return goValueCopyCallForType(typ), true
	}
	return goValueCopyCallForExpr(ident)
}

// pointeeReadSharesStorage is the dereference being written as the operand
// of an index, which reads the pointee's storage rather than a copy of it.
var pointeeReadSharesStorage ast.Expr

// writePointeeValueCopy writes the value of deref, the dereference of the
// pointer pointer, as Go's copy of the pointee. The pointee's Clone is a
// second handle on the same struct, so "bump(*p)" changed *p through its
// parameter; a dereference read as a value is a Go copy wherever it is used.
func writePointeeValueCopy(out *strings.Builder, deref ast.Expr, pointer ast.Expr) {
	copyCall := ".clone()"
	if typeInfo := GetTypeInfo(); typeInfo != nil && deref != pointeeReadSharesStorage {
		copyCall = goValueCopyCallForType(typeInfo.GetType(deref))
	}
	out.WriteString("{ let __v = (*")
	TranspileExpressionContext(out, pointer, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())")
	out.WriteString(copyCall)
	out.WriteString("; __v }")
}
