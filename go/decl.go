package main

import (
	"fmt"
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
	"math"
	"sort"
	"strconv"
	"strings"
)

func rustStructFieldName(name *ast.Ident, fieldIndex int, nameIndex int) string {
	if name == nil {
		return ""
	}
	if name.Name == "_" {
		return fmt.Sprintf("__blank_%d_%d", fieldIndex, nameIndex)
	}
	return ToSnakeCase(name.Name)
}

// generateStructDisplay generates a Display implementation for a struct to match Go's output format
type rustTypeGenerics struct {
	Decl    string
	Use     string
	Where   string
	Phantom []string
}

const minStructDisplayMultilineWriteFields = 10

func writeRustInherentImplHeader(out *strings.Builder, generics rustTypeGenerics, rustTypeName string) {
	out.WriteString("impl")
	out.WriteString(generics.Decl)
	out.WriteString(" ")
	out.WriteString(rustTypeName)
	out.WriteString(generics.Use)
	out.WriteString(generics.Where)
	out.WriteString(" {\n")
}

func writeRustTraitImplHeader(out *strings.Builder, generics rustTypeGenerics, traitName string, rustTypeName string) {
	out.WriteString("impl")
	out.WriteString(generics.Decl)
	out.WriteString(" ")
	out.WriteString(traitName)
	out.WriteString(" for ")
	out.WriteString(rustTypeName)
	out.WriteString(generics.Use)
	out.WriteString(generics.Where)
	out.WriteString(" {\n")
}

// generateStructDisplay generates a Display implementation for a struct to match Go's output format
func generateStructDisplay(out *strings.Builder, structName string, structType *ast.StructType, generics rustTypeGenerics) {
	TrackImport("Display")
	TrackImport("Formatter")
	rustStructName := RustTypeNameForUse(structName)

	// If this type implements the error interface, Display should delegate to error()
	if IsErrorImplType(structName) {
		writeRustTraitImplHeader(out, generics, "std::fmt::Display", rustStructName)
		out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
		out.WriteString("        write!(f, \"{}\", (*self.error()")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()))\n")
		out.WriteString("    }\n")
		out.WriteString("}\n")
		return
	}

	if namedTypeHasGoStringMethod(structName) {
		writeRustTraitImplHeader(out, generics, "std::fmt::Display", rustStructName)
		out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
		writeStringerDisplayBody(out, structName, "        ")
		out.WriteString("    }\n")
		out.WriteString("}\n")
		return
	}

	writeRustTraitImplHeader(out, generics, "std::fmt::Display", rustStructName)
	out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")

	// Collect all fields (including embedded)
	type fieldEntry struct {
		name                      string
		isEmbedded                bool
		isSlice                   bool
		isMap                     bool
		isInterface               bool
		isFunction                bool
		funcSlice                 bool
		anySlice                  bool
		hasTrait                  bool
		mapOpaque                 bool
		nestedSlice               bool
		nestedSliceWrapped        bool
		nestedPointerSlice        bool
		nestedPointerSliceWrapped bool
		ptrSlice                  bool
		ptrToSlice                bool
		ptrToPtrSlice             bool
		isPointer                 bool
		goPtr                     bool
		goPtrSlice                bool
		goPtrArray                bool
		interfaceSlice            bool
		zeroLenArray              bool
	}
	var fields []fieldEntry
	for fieldIndex, field := range structType.Fields.List {
		// Skip sync types (Mutex, WaitGroup) — they're not data fields
		if isSyncParam(field.Type) {
			continue
		}
		_, isSlice := field.Type.(*ast.ArrayType)
		_, isMap := field.Type.(*ast.MapType)
		isInterface := isEmptyInterfaceExpr(field.Type)
		isFunction := isFunctionSignatureTypeExpr(field.Type)
		funcSlice := arrayFieldContainsFunction(field.Type)
		anySlice := sliceFieldContainsEmptyInterface(field.Type)
		_, isChannel := field.Type.(*ast.ChanType)
		hasTrait := typeHasTraitField(field.Type)
		mapOpaque := mapFieldNeedsOpaqueDisplay(field.Type)
		nestedSlice := arrayFieldContainsSlice(field.Type)
		nestedSliceWrapped := arrayFieldNestedInnerIsPointer(field.Type)
		nestedPointerSlice := arrayFieldNestedPointerInnerIsValue(field.Type)
		nestedPointerSliceWrapped := arrayFieldNestedPointerInnerIsPointer(field.Type)
		ptrSlice := arrayFieldContainsPointer(field.Type)
		ptrToSlice := pointerFieldContainsSlice(field.Type)
		ptrToPtrSlice := pointerFieldContainsPointerSlice(field.Type)
		isPointer := structDisplayFieldIsPointer(field.Type)
		interfaceSlice := arrayFieldContainsLocalInterface(field.Type)
		zeroLenArray := structDisplayFieldIsZeroLenArray(field.Type)
		if isChannel {
			continue
		}
		if len(field.Names) > 0 {
			for nameIndex, name := range field.Names {
				_, goPtrField := sliceElemPtrFieldInfoForStructField(nil, structType, structName, name.Name)
				_, goPtrSliceField := sliceElemPtrSliceFieldInfoForStructField(nil, structType, structName, name.Name)
				_, goPtrArrayField := goPtrArrayFieldInfoForStructField(nil, structType, structName, name.Name)
				fields = append(fields, fieldEntry{
					name:                      rustStructFieldName(name, fieldIndex, nameIndex),
					isEmbedded:                false,
					isSlice:                   isSlice,
					isMap:                     isMap,
					isInterface:               isInterface,
					isFunction:                isFunction,
					funcSlice:                 funcSlice,
					anySlice:                  anySlice,
					hasTrait:                  hasTrait,
					mapOpaque:                 mapOpaque,
					nestedSlice:               nestedSlice,
					nestedSliceWrapped:        nestedSliceWrapped,
					nestedPointerSlice:        nestedPointerSlice,
					nestedPointerSliceWrapped: nestedPointerSliceWrapped,
					ptrSlice:                  ptrSlice,
					ptrToSlice:                ptrToSlice,
					ptrToPtrSlice:             ptrToPtrSlice,
					isPointer:                 isPointer || syncAtomicPointerDisplayField(structName, field, name),
					goPtr:                     goPtrField,
					goPtrSlice:                goPtrSliceField,
					goPtrArray:                goPtrArrayField,
					interfaceSlice:            interfaceSlice,
					zeroLenArray:              zeroLenArray,
				})
			}
		} else {
			// Embedded field
			typeName := getEmbeddedFieldName(field.Type)
			_, goPtrField := sliceElemPtrFieldInfoForStructField(nil, structType, structName, typeName)
			_, goPtrSliceField := sliceElemPtrSliceFieldInfoForStructField(nil, structType, structName, typeName)
			_, goPtrArrayField := goPtrArrayFieldInfoForStructField(nil, structType, structName, typeName)
			fields = append(fields, fieldEntry{
				name:                      typeName,
				isEmbedded:                true,
				isSlice:                   isSlice,
				isMap:                     isMap,
				isInterface:               isInterface,
				isFunction:                isFunction,
				funcSlice:                 funcSlice,
				anySlice:                  anySlice,
				hasTrait:                  hasTrait,
				mapOpaque:                 mapOpaque,
				nestedSlice:               nestedSlice,
				nestedSliceWrapped:        nestedSliceWrapped,
				nestedPointerSlice:        nestedPointerSlice,
				nestedPointerSliceWrapped: nestedPointerSliceWrapped,
				ptrSlice:                  ptrSlice,
				ptrToSlice:                ptrToSlice,
				ptrToPtrSlice:             ptrToPtrSlice,
				isPointer:                 isPointer,
				goPtr:                     goPtrField,
				goPtrSlice:                goPtrSliceField,
				goPtrArray:                goPtrArrayField,
				interfaceSlice:            interfaceSlice,
				zeroLenArray:              zeroLenArray,
			})
		}
	}

	writeDisplayFieldValue := func(f fieldEntry) {
		if f.isInterface {
			NeedFormatAny()
			out.WriteString("format_any(self.")
			out.WriteString(ToSnakeCase(f.name))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap().as_ref())")
		} else if f.isFunction {
			out.WriteString("\"<func>\"")
		} else if f.funcSlice {
			out.WriteString("{ let __guard = self.")
			out.WriteString(ToSnakeCase(f.name))
			WriteBorrowMethod(out, false)
			out.WriteString("; match __guard.as_ref() { Some(__v) => format!(\"[{}]\", std::iter::repeat(\"<func>\").take(__v.len()).collect::<Vec<_>>().join(\" \")), None => \"[]\".to_string() } }")
		} else if f.isMap && f.mapOpaque {
			out.WriteString("\"<map>\"")
		} else if f.isMap {
			NeedFormatMap()
			out.WriteString("format_map(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.zeroLenArray {
			out.WriteString("\"[]\"")
		} else if f.nestedSliceWrapped {
			NeedFormatNestedSliceWrapped()
			out.WriteString("format_nested_slice_wrapped(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.nestedPointerSlice {
			NeedFormatNestedPointerSlice()
			out.WriteString("format_nested_pointer_slice(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.nestedPointerSliceWrapped {
			NeedFormatNestedPointerSliceWrapped()
			out.WriteString("format_nested_pointer_slice_wrapped(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.nestedSlice {
			NeedFormatNestedSlice()
			out.WriteString("format_nested_slice(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.goPtr {
			out.WriteString("{ if self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(".is_nil() { \"<nil>\".to_string() } else { \"<ptr>\".to_string() } }")
		} else if f.goPtrSlice || f.goPtrArray {
			out.WriteString("{ let __guard = self.")
			out.WriteString(ToSnakeCase(f.name))
			WriteBorrowMethod(out, false)
			out.WriteString("; match __guard.as_ref() { Some(__v) => format!(\"[{}]\", __v.iter().map(|__p| if __p.is_nil() { \"<nil>\".to_string() } else { \"<ptr>\".to_string() }).collect::<Vec<_>>().join(\" \")), None => \"[]\".to_string() } }")
		} else if f.ptrToPtrSlice {
			NeedFormatSlice()
			out.WriteString("format_slice_wrapped(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.ptrToSlice {
			NeedFormatSlice()
			out.WriteString("format_slice(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.ptrSlice {
			NeedFormatSlice()
			out.WriteString("format_slice_wrapped(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.anySlice {
			NeedFormatAnySlice()
			out.WriteString("format_any_slice(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.interfaceSlice {
			NeedFormatSliceWrappedStringer()
			out.WriteString("format_slice_wrapped_stringer(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.isPointer {
			out.WriteString("{ let __guard = self.")
			out.WriteString(ToSnakeCase(f.name))
			WriteBorrowMethod(out, false)
			out.WriteString("; match __guard.as_ref() { Some(__v) => format!(\"{:p}\", __v as *const _), None => \"<nil>\".to_string() } }")
		} else if f.isSlice {
			NeedFormatSlice()
			out.WriteString("format_slice(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else {
			out.WriteString("(*self.")
			out.WriteString(ToSnakeCase(f.name))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		}
	}

	for i, f := range fields {
		out.WriteString("        let __go_fmt_")
		out.WriteString(fmt.Sprintf("%d", i))
		out.WriteString(" = format!(\"{}\", ")
		writeDisplayFieldValue(f)
		out.WriteString(");\n")
	}

	writeStructDisplayWriteCall(out, len(fields))
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func writeStructDisplayWriteCall(out *strings.Builder, fieldCount int) {
	if fieldCount < minStructDisplayMultilineWriteFields {
		out.WriteString("        write!(f, \"{{")
		writeStructDisplayFormatPlaceholders(out, fieldCount)
		out.WriteString("}}\"")
		for i := 0; i < fieldCount; i++ {
			out.WriteString(", __go_fmt_")
			out.WriteString(fmt.Sprintf("%d", i))
		}
		out.WriteString(")\n")
		return
	}

	out.WriteString("        write!(\n")
	out.WriteString("            f,\n")
	out.WriteString("            \"{{")
	writeStructDisplayFormatPlaceholders(out, fieldCount)
	out.WriteString("}}\"")
	for i := 0; i < fieldCount; i++ {
		out.WriteString(",\n")
		out.WriteString("            __go_fmt_")
		out.WriteString(fmt.Sprintf("%d", i))
	}
	out.WriteString("\n")
	out.WriteString("        )\n")
}

func writeStructDisplayFormatPlaceholders(out *strings.Builder, fieldCount int) {
	for i := 0; i < fieldCount; i++ {
		if i > 0 {
			out.WriteString(" ")
		}
		out.WriteString("{}")
	}
}

func structDisplayFieldIsZeroLenArray(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	array, ok := types.Unalias(typ).Underlying().(*types.Array)
	return ok && array.Len() == 0
}

func generateStructDebug(out *strings.Builder, structName string, generics rustTypeGenerics) {
	rustStructName := RustTypeNameForUse(structName)
	writeRustTraitImplHeader(out, generics, "std::fmt::Debug", rustStructName)
	out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
	out.WriteString("        write!(f, \"{}\", self)\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func generateStructDisplayTypeInfoRequired(out *strings.Builder, structName string, generics rustTypeGenerics) {
	TrackImport("Display")
	TrackImport("Formatter")
	rustStructName := RustTypeNameForUse(structName)
	writeRustTraitImplHeader(out, generics, "std::fmt::Display", rustStructName)
	out.WriteString("    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
	out.WriteString("        unimplemented!(\"type info required for generic struct Display bounds\")\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func arrayFieldContainsSlice(expr ast.Expr) bool {
	arrayType, ok := expr.(*ast.ArrayType)
	if !ok {
		return false
	}
	_, ok = arrayType.Elt.(*ast.ArrayType)
	return ok
}

func arrayFieldContainsPointer(expr ast.Expr) bool {
	arrayType, ok := expr.(*ast.ArrayType)
	if !ok {
		return false
	}
	_, ok = arrayType.Elt.(*ast.StarExpr)
	return ok
}

func pointerFieldContainsSlice(expr ast.Expr) bool {
	_, ok := pointerFieldSliceElem(expr)
	return ok
}

func pointerFieldContainsPointerSlice(expr ast.Expr) bool {
	elem, ok := pointerFieldSliceElem(expr)
	if !ok {
		return false
	}
	_, ok = elem.(*ast.StarExpr)
	return ok
}

func pointerFieldSliceElem(expr ast.Expr) (ast.Expr, bool) {
	star, ok := expr.(*ast.StarExpr)
	if !ok {
		return nil, false
	}
	arrayType, ok := star.X.(*ast.ArrayType)
	if !ok {
		return nil, false
	}
	return arrayType.Elt, true
}

func structDisplayFieldIsPointer(expr ast.Expr) bool {
	typ, ok := typeInfoTypeForTypeExpr(expr)
	if !ok {
		return false
	}
	_, ok = types.Unalias(typ).Underlying().(*types.Pointer)
	return ok
}

func typeSpecIsSyncAtomicPointer(typeSpec *ast.TypeSpec) bool {
	if typeSpec == nil || typeSpec.Name == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	obj, ok := typeInfo.GetObject(typeSpec.Name).(*types.TypeName)
	return ok && obj.Pkg() != nil && obj.Pkg().Path() == "sync/atomic" && obj.Name() == "Pointer"
}

func currentPackageIsSyncAtomic() bool {
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.pkg != nil && typeInfo.pkg.Path() == "sync/atomic"
}

func fieldTypeIsUnsafePointer(field *ast.Field) bool {
	if field == nil || field.Type == nil {
		return false
	}
	typ, ok := typeInfoTypeForTypeExpr(field.Type)
	return ok && isUnsafePointerLikeType(typ)
}

func syncAtomicPointerStorageField(typeSpec *ast.TypeSpec, field *ast.Field, name *ast.Ident) bool {
	return typeSpecIsSyncAtomicPointer(typeSpec) &&
		field != nil &&
		name != nil &&
		name.Name == "v" &&
		fieldTypeIsUnsafePointer(field)
}

func syncAtomicPointerDisplayField(structName string, field *ast.Field, name *ast.Ident) bool {
	return currentPackageIsSyncAtomic() &&
		structName == "Pointer" &&
		field != nil &&
		name != nil &&
		name.Name == "v" &&
		fieldTypeIsUnsafePointer(field)
}

func syncAtomicPointerStorageRustType(typeSpec *ast.TypeSpec) string {
	trackWrapperImports()
	NeedSliceElemPtr()
	typeParam := "T"
	if typeSpec != nil && typeSpec.TypeParams != nil && len(typeSpec.TypeParams.List) > 0 {
		for _, name := range typeSpec.TypeParams.List[0].Names {
			if name != nil && name.Name != "" {
				typeParam = RustTypeNameForUse(name.Name)
				break
			}
		}
	}
	outerWrapper := GetOuterWrapperType()
	innerWrapper := GetInnerWrapperType()
	return outerWrapper + "<" + innerWrapper + "<Option<GoPtr<" + typeParam + ">>>>"
}

// arrayFieldNestedInnerIsPointer reports whether expr is a nested slice ([][]X)
// whose innermost element is a pointer (e.g. [][]*T). Such fields store the
// innermost element as a wrapped Arc/Rc handle, which format_nested_slice cannot
// Display directly — format_nested_slice_wrapped unwraps the handle.
func arrayFieldNestedInnerIsPointer(expr ast.Expr) bool {
	outer, ok := expr.(*ast.ArrayType)
	if !ok {
		return false
	}
	inner, ok := outer.Elt.(*ast.ArrayType)
	if !ok {
		return false
	}
	elt := inner.Elt
	for {
		next, ok := elt.(*ast.ArrayType)
		if !ok {
			break
		}
		elt = next.Elt
	}
	_, isPtr := elt.(*ast.StarExpr)
	return isPtr
}

func arrayFieldNestedPointerInnerIsPointer(expr ast.Expr) bool {
	typ, ok := typeInfoTypeForTypeExpr(expr)
	if !ok {
		return false
	}
	outer, ok := sequenceType(typ)
	if !ok {
		return false
	}
	ptr, ok := types.Unalias(outer.Elem()).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	inner, ok := sequenceType(ptr.Elem())
	if !ok {
		return false
	}
	_, ok = types.Unalias(inner.Elem()).Underlying().(*types.Pointer)
	return ok
}

func arrayFieldNestedPointerInnerIsValue(expr ast.Expr) bool {
	typ, ok := typeInfoTypeForTypeExpr(expr)
	if !ok {
		return false
	}
	outer, ok := sequenceType(typ)
	if !ok {
		return false
	}
	ptr, ok := types.Unalias(outer.Elem()).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	inner, ok := sequenceType(ptr.Elem())
	if !ok {
		return false
	}
	_, innerIsPointer := types.Unalias(inner.Elem()).Underlying().(*types.Pointer)
	return !innerIsPointer
}

type goSequenceType interface {
	Elem() types.Type
}

func sequenceType(typ types.Type) (goSequenceType, bool) {
	switch seq := types.Unalias(typ).Underlying().(type) {
	case *types.Array:
		return seq, true
	case *types.Slice:
		return seq, true
	default:
		return nil, false
	}
}

func arrayFieldContainsFunction(expr ast.Expr) bool {
	arrayType, ok := expr.(*ast.ArrayType)
	if !ok {
		return false
	}
	return isFunctionSignatureTypeExpr(arrayType.Elt)
}

func sliceFieldContainsEmptyInterface(expr ast.Expr) bool {
	typ, ok := typeInfoTypeForTypeExpr(expr)
	if !ok {
		return false
	}
	slice, ok := types.Unalias(typ).Underlying().(*types.Slice)
	return ok && isEmptyInterfaceType(slice.Elem())
}

func arrayFieldContainsLocalInterface(expr ast.Expr) bool {
	arrayType, ok := expr.(*ast.ArrayType)
	if !ok {
		return false
	}
	_, ok = transpiledNamedInterfaceTypeNameFromExpr(arrayType.Elt)
	return ok
}

func structHasTraitField(structType *ast.StructType) bool {
	for _, field := range structType.Fields.List {
		if typeHasTraitField(field.Type) {
			return true
		}
	}
	return false
}

func structCanDeriveDebug(structType *ast.StructType) bool {
	return structCanDeriveDebugSeen(structType, make(map[string]bool))
}

func structCanDeriveDebugSeen(structType *ast.StructType, seen map[string]bool) bool {
	for _, field := range structType.Fields.List {
		if !typeCanDeriveDebug(field.Type, seen) {
			return false
		}
	}
	return true
}

func structNeedsCustomDefault(structType *ast.StructType) bool {
	for _, field := range structType.Fields.List {
		if structFieldNeedsCustomDefault(field.Type) {
			return true
		}
	}
	return false
}

func structFieldNeedsCustomDefault(expr ast.Expr) bool {
	if expr == nil {
		return false
	}
	if isSyncParam(expr) {
		return true
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if typ := typeInfo.GetType(expr); typ != nil {
			if _, isChan := types.Unalias(typ).Underlying().(*types.Chan); isChan {
				return false
			}
			return !structFieldHasNilZero(typ)
		}
	}
	switch t := expr.(type) {
	case *ast.StructType:
		return true
	case *ast.Ident:
		if _, isStruct := structDefs[t.Name]; isStruct {
			return true
		}
		switch t.Name {
		case "string", "bool", "int", "int8", "int16", "int32", "int64", "rune",
			"uint", "uint8", "uint16", "uint32", "uint64", "uintptr", "byte",
			"float32", "float64":
			return true
		default:
			return false
		}
	case *ast.ArrayType:
		return t.Len != nil
	case *ast.SelectorExpr:
		if ident, ok := t.X.(*ast.Ident); ok {
			return (ident.Name == "strings" && t.Sel.Name == "Builder") ||
				(ident.Name == "bytes" && t.Sel.Name == "Buffer")
		}
		return false
	default:
		return false
	}
}

func structFieldHasNilZero(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if _, _, isNamedMap := namedMapTypeFromType(typ); isNamedMap {
		return false
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		if _, isSlice := types.Unalias(named.Underlying()).(*types.Slice); isSlice {
			return false
		}
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Interface, *types.Pointer, *types.Signature, *types.Slice, *types.Map, *types.Chan:
		return true
	default:
		return false
	}
}

func writeWrappedNamedMapZeroValue(out *strings.Builder, typ types.Type) bool {
	named, _, ok := namedMapTypeFromType(typ)
	if !ok {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("::default()")
	WriteWrapperSuffix(out)
	return true
}

func namedTypeHasGoStringMethod(typeName string) bool {
	if IsStringerImplType(typeName) {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return false
	}
	obj, ok := typeInfo.pkg.Scope().Lookup(typeName).(*types.TypeName)
	if !ok {
		return false
	}
	return typeHasGoStringMethod(obj.Type()) || typeHasGoStringMethod(types.NewPointer(obj.Type()))
}

func namedTypeGoStringMethodRequiresMutableReceiver(typeName string) bool {
	for _, fn := range methodsForReceiverType(typeName) {
		if fn != nil && fn.Name != nil && fn.Name.Name == "String" {
			return methodRequiresMutableReceiver(fn)
		}
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return false
	}
	obj, ok := typeInfo.pkg.Scope().Lookup(typeName).(*types.TypeName)
	if !ok {
		return false
	}
	for _, typ := range []types.Type{obj.Type(), types.NewPointer(obj.Type())} {
		methodSet := types.NewMethodSet(typ)
		selection := methodSet.Lookup(nil, "String")
		if selection == nil {
			continue
		}
		fn, ok := selection.Obj().(*types.Func)
		if !ok {
			continue
		}
		key := methodOverrideKey(fn)
		if key == "" {
			continue
		}
		if mutable, ok := typeInfo.methodMutableReceiverMap[key]; ok {
			return mutable
		}
	}
	return false
}

func writeStringerDisplayBody(out *strings.Builder, typeName string, indent string) {
	if namedTypeGoStringMethodRequiresMutableReceiver(typeName) {
		out.WriteString(indent)
		out.WriteString("let mut __self = self.clone();\n")
		out.WriteString(indent)
		out.WriteString("write!(f, \"{}\", (*__self.string()")
	} else {
		out.WriteString(indent)
		out.WriteString("write!(f, \"{}\", (*self.string()")
	}
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()))\n")
}

func typeHasGoStringMethod(typ types.Type) bool {
	if typ == nil {
		return false
	}
	methodSet := types.NewMethodSet(typ)
	selection := methodSet.Lookup(nil, "String")
	if selection == nil {
		return false
	}
	sig, ok := selection.Obj().Type().(*types.Signature)
	if !ok {
		return false
	}
	if sig.Params().Len() != 0 || sig.Results().Len() != 1 {
		return false
	}
	basic, ok := types.Unalias(sig.Results().At(0).Type()).Underlying().(*types.Basic)
	return ok && basic.Kind() == types.String
}

func isPointerType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).(*types.Pointer)
	return ok
}

func writeStructDerive(out *strings.Builder, structName string, structType *ast.StructType, deriveClone bool) {
	hasTraitField := structHasTraitField(structType)
	canDeriveDebug := !hasTraitField && structCanDeriveDebug(structType)
	needsCustomDefault := structNeedsCustomDefault(structType)
	needsPartialEq := structName != "" && comparableStructTypes[structName]
	derivePartialEq := needsPartialEq && !hasTraitField && !structNeedsCustomPartialEq(structName, structType)
	deriveOrd := derivePartialEq && structNeedsOrd(structName)

	var traits []string
	if canDeriveDebug {
		traits = append(traits, "Debug")
	}
	if deriveClone {
		traits = append(traits, "Clone")
	}
	if !needsCustomDefault {
		traits = append(traits, "Default")
	}
	if derivePartialEq {
		traits = append(traits, "PartialEq")
	}
	if deriveOrd {
		traits = append(traits, "Eq", "PartialOrd", "Ord")
	}
	if len(traits) == 0 {
		return
	}
	out.WriteString("#[derive(")
	out.WriteString(strings.Join(traits, ", "))
	out.WriteString(")]\n")
}

func structNeedsOrd(structName string) bool {
	return structName != "" && currentMapKeyStructTypes()[structName]
}

func currentMapKeyStructTypes() map[string]bool {
	if currentContext != nil && currentContext.Package != nil {
		return currentContext.Package.MapKeyStructTypes
	}
	return nil
}

func structNeedsCustomPartialEq(structName string, structType *ast.StructType) bool {
	return structName != "" && structType != nil && comparableStructTypes[structName] && NeedsConcurrentWrapper()
}

func structNeedsCustomOrd(structName string, structType *ast.StructType) bool {
	return structName != "" && structType != nil && structNeedsOrd(structName) && NeedsConcurrentWrapper()
}

type structComparableField struct {
	name   string
	goName string
	expr   ast.Expr
}

func structComparableFields(structType *ast.StructType) []structComparableField {
	var fields []structComparableField
	if structType == nil {
		return fields
	}
	for fieldIndex, field := range structType.Fields.List {
		if len(field.Names) > 0 {
			for nameIndex, name := range field.Names {
				if name.Name == "_" {
					continue
				}
				fields = append(fields, structComparableField{
					name:   rustStructFieldName(name, fieldIndex, nameIndex),
					goName: name.Name,
					expr:   field.Type,
				})
			}
			continue
		}
		goName := getEmbeddedFieldName(field.Type)
		fields = append(fields, structComparableField{
			name:   ToSnakeCase(goName),
			goName: goName,
			expr:   field.Type,
		})
	}
	return fields
}

func structComparableFieldNames(structType *ast.StructType) []string {
	fields := structComparableFields(structType)
	names := make([]string, 0, len(fields))
	for _, field := range fields {
		names = append(names, field.name)
	}
	return names
}

func structComparableFieldType(field structComparableField) (types.Type, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || field.expr == nil {
		return nil, false
	}
	typ := typeInfo.GetType(field.expr)
	return typ, typ != nil
}

func writeStructComparableFieldTypeInfoRequired(out *strings.Builder, operation string, field structComparableField) {
	out.WriteString("unimplemented!(")
	out.WriteString(strconv.Quote(fmt.Sprintf("type info required to %s struct field %s", operation, field.name)))
	out.WriteString(")")
}

func writeStructComparablePointerFieldEq(out *strings.Builder, field structComparableField) {
	outerWrapper := GetOuterWrapperType()
	out.WriteString("{ let __left_some = self.")
	out.WriteString(field.name)
	out.WriteString(".lock().unwrap().is_some(); let __right_some = other.")
	out.WriteString(field.name)
	out.WriteString(".lock().unwrap().is_some(); (!__left_some && !__right_some) || (__left_some && __right_some && ")
	out.WriteString(outerWrapper)
	out.WriteString("::ptr_eq(&self.")
	out.WriteString(field.name)
	out.WriteString(", &other.")
	out.WriteString(field.name)
	out.WriteString(")) }")
}

func writeStructComparableGoPtrFieldEq(out *strings.Builder, field structComparableField) {
	out.WriteString("GoPtr::ptr_eq(&self.")
	out.WriteString(field.name)
	out.WriteString(", &other.")
	out.WriteString(field.name)
	out.WriteString(")")
}

func writeStructComparablePointerFieldOrd(out *strings.Builder, field structComparableField) {
	outerWrapper := GetOuterWrapperType()
	out.WriteString("            let __left_some = self.")
	out.WriteString(field.name)
	out.WriteString(".lock().unwrap().is_some();\n")
	out.WriteString("            let __right_some = other.")
	out.WriteString(field.name)
	out.WriteString(".lock().unwrap().is_some();\n")
	out.WriteString("            let __ord = match (__left_some, __right_some) {\n")
	out.WriteString("                (false, false) => std::cmp::Ordering::Equal,\n")
	out.WriteString("                (false, true) => std::cmp::Ordering::Less,\n")
	out.WriteString("                (true, false) => std::cmp::Ordering::Greater,\n")
	out.WriteString("                (true, true) => (")
	out.WriteString(outerWrapper)
	out.WriteString("::as_ptr(&self.")
	out.WriteString(field.name)
	out.WriteString(") as usize).cmp(&(")
	out.WriteString(outerWrapper)
	out.WriteString("::as_ptr(&other.")
	out.WriteString(field.name)
	out.WriteString(") as usize)),\n")
	out.WriteString("            };\n")
	out.WriteString("            match __ord {\n")
	out.WriteString("                std::cmp::Ordering::Equal => {}\n")
	out.WriteString("                __ord => return __ord,\n")
	out.WriteString("            }\n")
}

func writeStructComparableGoPtrFieldOrd(out *strings.Builder, field structComparableField) {
	out.WriteString("            match self.")
	out.WriteString(field.name)
	out.WriteString(".addr().cmp(&other.")
	out.WriteString(field.name)
	out.WriteString(".addr()) {\n")
	out.WriteString("                std::cmp::Ordering::Equal => {}\n")
	out.WriteString("                __ord => return __ord,\n")
	out.WriteString("            }\n")
}

func structComparableFieldIsGeneratedGoPtr(structName string, field structComparableField) bool {
	return field.goName != "" && generatedGoPtrFieldForStructNameField(structName, field.goName)
}

func writeStructComparableFieldEq(out *strings.Builder, structName string, field structComparableField) {
	typ, ok := structComparableFieldType(field)
	if !ok {
		writeStructComparableFieldTypeInfoRequired(out, "compare", field)
		return
	}
	if isPointerType(typ) {
		if structComparableFieldIsGeneratedGoPtr(structName, field) {
			writeStructComparableGoPtrFieldEq(out, field)
			return
		}
		writeStructComparablePointerFieldEq(out, field)
		return
	}
	if ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(typ); ok {
		out.WriteString("{ let __left = self.")
		out.WriteString(field.name)
		out.WriteString(".lock().unwrap(); let __right = other.")
		out.WriteString(field.name)
		out.WriteString(".lock().unwrap(); match (__left.as_ref(), __right.as_ref()) { (Some(__left), Some(__right)) => __left.as_ref().__go_eq_")
		out.WriteString(traitMethodSuffix(ifaceName))
		out.WriteString("(__right.as_ref()), (None, None) => true, _ => false } }")
		return
	}
	out.WriteString("{ let __left = self.")
	out.WriteString(field.name)
	out.WriteString(".lock().unwrap(); let __right = other.")
	out.WriteString(field.name)
	out.WriteString(".lock().unwrap(); __left.as_ref() == __right.as_ref() }")
}

func writeStructComparableFieldOrd(out *strings.Builder, structName string, field structComparableField) {
	out.WriteString("        {\n")
	typ, ok := structComparableFieldType(field)
	if !ok {
		out.WriteString("            ")
		writeStructComparableFieldTypeInfoRequired(out, "order", field)
		out.WriteString(";\n")
		out.WriteString("        }\n")
		return
	}
	if isPointerType(typ) {
		if structComparableFieldIsGeneratedGoPtr(structName, field) {
			writeStructComparableGoPtrFieldOrd(out, field)
		} else {
			writeStructComparablePointerFieldOrd(out, field)
		}
		out.WriteString("        }\n")
		return
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(typ); ok {
		out.WriteString("            let __left = self.")
		out.WriteString(field.name)
		out.WriteString(".lock().unwrap();\n")
		out.WriteString("            let __right = other.")
		out.WriteString(field.name)
		out.WriteString(".lock().unwrap();\n")
		out.WriteString("            let __ord = match (__left.as_ref(), __right.as_ref()) {\n")
		out.WriteString("                (None, None) => std::cmp::Ordering::Equal,\n")
		out.WriteString("                (None, Some(_)) => std::cmp::Ordering::Less,\n")
		out.WriteString("                (Some(_), None) => std::cmp::Ordering::Greater,\n")
		out.WriteString("                (Some(__left), Some(__right)) => format!(\"{}\", __left.as_ref()).cmp(&format!(\"{}\", __right.as_ref())),\n")
		out.WriteString("            };\n")
		out.WriteString("            match __ord {\n")
		out.WriteString("                std::cmp::Ordering::Equal => {}\n")
		out.WriteString("                __ord => return __ord,\n")
		out.WriteString("            }\n")
		out.WriteString("        }\n")
		return
	}
	out.WriteString("            let __left = { self.")
	out.WriteString(field.name)
	out.WriteString(".lock().unwrap().as_ref().cloned() };\n")
	out.WriteString("            let __right = { other.")
	out.WriteString(field.name)
	out.WriteString(".lock().unwrap().as_ref().cloned() };\n")
	out.WriteString("            match __left.cmp(&__right) {\n")
	out.WriteString("                std::cmp::Ordering::Equal => {}\n")
	out.WriteString("                __ord => return __ord,\n")
	out.WriteString("            }\n")
	out.WriteString("        }\n")
}

func generateStructPartialEq(out *strings.Builder, structName string, structType *ast.StructType, generics rustTypeGenerics) {
	if !structNeedsCustomPartialEq(structName, structType) {
		return
	}

	rustStructName := RustTypeNameForUse(structName)
	writeRustTraitImplHeader(out, generics, "PartialEq", rustStructName)
	out.WriteString("    fn eq(&self, other: &Self) -> bool {\n")

	fields := structComparableFields(structType)
	if len(fields) == 0 {
		out.WriteString("        true\n")
	} else {
		out.WriteString("        (\n")
		for i, field := range fields {
			if i > 0 {
				out.WriteString("\n                && ")
			} else {
				out.WriteString("            ")
			}
			writeStructComparableFieldEq(out, structName, field)
		}
		out.WriteString("\n        )\n")
	}

	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func structNeedsPointerComparableIdentity(structName string) bool {
	if structName == "" || currentContext == nil || currentContext.Package == nil {
		return false
	}
	return currentContext.Package.PointerComparablePointeeTypes[structName]
}

func generateStructPointerComparableIdentity(out *strings.Builder, structName string, generics rustTypeGenerics) {
	if !structNeedsPointerComparableIdentity(structName) {
		return
	}
	NeedGoComparable()
	rustStructName := RustTypeNameForUse(structName)
	writeRustTraitImplHeader(out, generics, "GoComparable", rustStructName)
	out.WriteString("    fn go_eq(&self, other: &Self) -> bool {\n")
	out.WriteString("        std::ptr::eq(self, other)\n")
	out.WriteString("    }\n")
	out.WriteString("    fn go_hash(&self, seed: usize) -> usize {\n")
	out.WriteString("        let mut __hasher = std::collections::hash_map::DefaultHasher::new();\n")
	out.WriteString("        std::hash::Hash::hash(&seed, &mut __hasher);\n")
	out.WriteString("        std::hash::Hash::hash(&(self as *const Self as usize), &mut __hasher);\n")
	out.WriteString("        std::hash::Hasher::finish(&__hasher) as usize\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func generateStructOrd(out *strings.Builder, structName string, structType *ast.StructType, generics rustTypeGenerics) {
	if !structNeedsCustomOrd(structName, structType) {
		return
	}

	rustStructName := RustTypeNameForUse(structName)
	out.WriteString("\nimpl")
	out.WriteString(generics.Decl)
	out.WriteString(" Eq for ")
	out.WriteString(rustStructName)
	out.WriteString(generics.Use)
	out.WriteString(" {}\n")

	out.WriteString("\n")
	writeRustTraitImplHeader(out, generics, "PartialOrd", rustStructName)
	out.WriteString("    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {\n")
	out.WriteString("        Some(self.cmp(other))\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")

	out.WriteString("\n")
	writeRustTraitImplHeader(out, generics, "Ord", rustStructName)
	out.WriteString("    fn cmp(&self, other: &Self) -> std::cmp::Ordering {\n")

	fields := structComparableFields(structType)
	for _, field := range fields {
		writeStructComparableFieldOrd(out, structName, field)
	}
	out.WriteString("        std::cmp::Ordering::Equal\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func generateStructValueClone(out *strings.Builder, structName string, structType *ast.StructType, generics rustTypeGenerics) {
	if structType == nil {
		return
	}
	type cloneFieldReturn struct {
		fieldName string
		localName string
	}
	rustStructName := RustTypeNameForUse(structName)
	writeRustInherentImplHeader(out, generics, rustStructName)
	out.WriteString("    pub fn __go_value_clone(&self) -> Self {\n")
	fieldsForReturn := []cloneFieldReturn{}
	for fieldIndex, field := range structType.Fields.List {
		fieldNames := field.Names
		if len(fieldNames) == 0 {
			fieldNames = []*ast.Ident{ast.NewIdent(getEmbeddedFieldName(field.Type))}
		}
		for nameIndex, name := range fieldNames {
			fieldName := rustStructFieldName(name, fieldIndex, nameIndex)
			localName := fmt.Sprintf("__go_clone_%d_%d", fieldIndex, nameIndex)
			out.WriteString("        let ")
			out.WriteString(localName)
			out.WriteString(" = ")
			writeStructCloneField(out, fieldName, field.Type)
			out.WriteString(";\n")
			fieldsForReturn = append(fieldsForReturn, cloneFieldReturn{
				fieldName: fieldName,
				localName: localName,
			})
		}
	}
	if len(generics.Phantom) > 0 {
		out.WriteString("        let __go_clone_phantom = std::marker::PhantomData;\n")
		fieldsForReturn = append(fieldsForReturn, cloneFieldReturn{
			fieldName: "__go_phantom",
			localName: "__go_clone_phantom",
		})
	}
	out.WriteString("        Self {\n")
	for _, field := range fieldsForReturn {
		out.WriteString("            ")
		out.WriteString(field.fieldName)
		out.WriteString(": ")
		out.WriteString(field.localName)
		out.WriteString(",\n")
	}
	out.WriteString("        }\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func generateStructClone(out *strings.Builder, structName string, generics rustTypeGenerics) {
	rustStructName := RustTypeNameForUse(structName)
	writeRustTraitImplHeader(out, generics, "Clone", rustStructName)
	out.WriteString("    fn clone(&self) -> Self {\n")
	out.WriteString("        self.__go_value_clone()\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func generateStructGoValueClone(out *strings.Builder, structName string, generics rustTypeGenerics) {
	rustStructName := RustTypeNameForUse(structName)
	writeRustTraitImplHeader(out, generics, "GoValueClone", rustStructName)
	out.WriteString("    fn go_value_clone(&self) -> Self {\n")
	out.WriteString("        self.__go_value_clone()\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func writeStructCloneField(out *strings.Builder, fieldName string, fieldType ast.Expr) {
	if structCloneFieldKeepsHandle(fieldType) {
		out.WriteString("self.")
		out.WriteString(fieldName)
		out.WriteString(".clone()")
		return
	}
	out.WriteString("{ let __guard = self.")
	out.WriteString(fieldName)
	WriteBorrowMethod(out, false)
	out.WriteString("; ")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::new(")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("::new((*__guard).clone())) }")
}

func structCloneFieldKeepsHandle(expr ast.Expr) bool {
	if expr == nil || isSyncParam(expr) || isEmptyInterfaceExpr(expr) || isFunctionSignatureTypeExpr(expr) || isGoErrorTypeExpr(expr) {
		return true
	}
	switch t := expr.(type) {
	case *ast.StarExpr, *ast.MapType, *ast.ChanType, *ast.FuncType, *ast.InterfaceType:
		return true
	case *ast.ArrayType:
		return t.Len == nil
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if typ := typeInfo.GetType(expr); typ != nil {
			switch types.Unalias(typ).Underlying().(type) {
			case *types.Pointer, *types.Slice, *types.Map, *types.Chan, *types.Signature, *types.Interface:
				return true
			}
		}
	}
	return false
}

func writeStructDefaultValue(out *strings.Builder, fieldType ast.Expr) {
	if nestedStruct, ok := fieldType.(*ast.StructType); ok {
		nestedName := generateAnonymousStructType(nestedStruct)
		WriteWrapperPrefix(out)
		out.WriteString(nestedName)
		out.WriteString("::default()")
		WriteWrapperSuffix(out)
		return
	}
	if fieldIdent, ok := fieldType.(*ast.Ident); ok {
		if _, isStruct := structDefs[fieldIdent.Name]; isStruct {
			WriteWrapperPrefix(out)
			out.WriteString(RustTypeNameForUse(fieldIdent.Name))
			out.WriteString("::default()")
			WriteWrapperSuffix(out)
			return
		}
	}
	if isSyncParam(fieldType) {
		if isSourceMappedSyncParam(fieldType) {
			out.WriteString("Default::default()")
			return
		}
		out.WriteString(goTypeToRustBase(fieldType))
		out.WriteString("::new()")
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if typ := typeInfo.GetType(fieldType); typ != nil {
			if _, isChan := types.Unalias(typ).Underlying().(*types.Chan); isChan {
				out.WriteString("Default::default()")
				return
			}
			if writeWrappedNamedMapZeroValue(out, typ) {
				return
			}
			if structFieldHasNilZero(typ) {
				WriteWrappedNone(out)
				return
			}
			WriteWrapperPrefix(out)
			out.WriteString(zeroValueForTypesType(typ))
			WriteWrapperSuffix(out)
			return
		}
	}
	out.WriteString("Default::default()")
}

func generateStructDefault(out *strings.Builder, typeSpec *ast.TypeSpec, structName string, structType *ast.StructType, generics rustTypeGenerics) {
	if !structNeedsCustomDefault(structType) {
		return
	}
	type defaultFieldValue struct {
		fieldName string
		localName string
	}
	writeDefaultFieldValue := func(field *ast.Field, name *ast.Ident, fieldName string) {
		if fieldInfo, ok := goPtrArrayFieldInfoForStructField(typeSpec, structType, structName, fieldName); ok {
			writeGoPtrArrayFieldDefaultValue(out, fieldInfo)
		} else if _, ok := sliceElemPtrSliceFieldInfoForStructField(typeSpec, structType, structName, fieldName); ok {
			WriteWrappedNone(out)
		} else if _, ok := sliceElemPtrFieldInfoForStructField(typeSpec, structType, structName, fieldName); ok {
			NeedSliceElemPtr()
			out.WriteString("GoPtr::nil()")
		} else if name != nil && syncAtomicPointerStorageField(typeSpec, field, name) {
			WriteWrappedNone(out)
		} else {
			writeStructDefaultValue(out, field.Type)
		}
	}
	out.WriteString("\n")
	writeRustTraitImplHeader(out, generics, "Default", RustTypeNameForUse(structName))
	out.WriteString("    fn default() -> Self {\n")
	var fieldValues []defaultFieldValue
	for fieldIndex, field := range structType.Fields.List {
		if len(field.Names) > 0 {
			for nameIndex, name := range field.Names {
				localName := fmt.Sprintf("__go_default_%d_%d", fieldIndex, nameIndex)
				out.WriteString("        let ")
				out.WriteString(localName)
				out.WriteString(" = ")
				writeDefaultFieldValue(field, name, name.Name)
				out.WriteString(";\n")
				fieldValues = append(fieldValues, defaultFieldValue{
					fieldName: rustStructFieldName(name, fieldIndex, nameIndex),
					localName: localName,
				})
			}
		} else {
			fieldName := getEmbeddedFieldName(field.Type)
			localName := fmt.Sprintf("__go_default_%d_0", fieldIndex)
			out.WriteString("        let ")
			out.WriteString(localName)
			out.WriteString(" = ")
			writeDefaultFieldValue(field, nil, fieldName)
			out.WriteString(";\n")
			fieldValues = append(fieldValues, defaultFieldValue{
				fieldName: ToSnakeCase(fieldName),
				localName: localName,
			})
		}
	}
	if len(generics.Phantom) > 0 {
		out.WriteString("        let __go_default_phantom = std::marker::PhantomData;\n")
		fieldValues = append(fieldValues, defaultFieldValue{
			fieldName: "__go_phantom",
			localName: "__go_default_phantom",
		})
	}
	out.WriteString("        Self {\n")
	for _, fieldValue := range fieldValues {
		out.WriteString("            ")
		out.WriteString(fieldValue.fieldName)
		out.WriteString(": ")
		out.WriteString(fieldValue.localName)
		out.WriteString(",\n")
	}
	out.WriteString("        }\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func shouldGenerateJsonDecodeImpl() bool {
	return currentContext != nil &&
		currentContext.UsePackageExternalStubs &&
		currentContext.Package != nil &&
		currentContext.Package.JsonDecodeSupportNeeded
}

func packageNeedsJsonDecodeSupport(files []*ast.File, typeInfo *TypeInfo) bool {
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	for _, file := range files {
		if file == nil {
			continue
		}
		needed := false
		ast.Inspect(file, func(node ast.Node) bool {
			if needed {
				return false
			}
			ident, ok := node.(*ast.Ident)
			if !ok {
				return true
			}
			obj := typeInfo.info.Uses[ident]
			if obj == nil || obj.Pkg() == nil || obj.Pkg().Path() != "encoding/json" {
				return true
			}
			switch obj.Name() {
			case "Decoder", "Decode", "More", "NewDecoder", "Unmarshal":
				needed = true
				return false
			default:
				return true
			}
		})
		if needed {
			return true
		}
	}
	return false
}

func jsonDecodeFieldName(fieldName string, tag *ast.BasicLit) (string, bool) {
	tagValue := ""
	if tag != nil && tag.Value != "" {
		if unquoted, err := strconv.Unquote(tag.Value); err == nil {
			tagValue = unquoted
		}
	}
	jsonName, include, _ := jsonFieldName(fieldName, tagValue)
	return jsonName, include
}

func jsonDecodeTypeSupported(typ types.Type) bool {
	if typ == nil {
		return false
	}
	typ = types.Unalias(typ)
	switch t := typ.(type) {
	case *types.Named:
		if t.Obj() != nil && t.Obj().Pkg() != nil && isStdlibPackage(t.Obj().Pkg().Path()) {
			return false
		}
		_, ok := types.Unalias(t.Underlying()).(*types.Struct)
		return ok
	case *types.Pointer:
		return jsonDecodeTypeSupported(t.Elem())
	case *types.Slice:
		return jsonDecodeTypeSupported(t.Elem())
	case *types.Array:
		return jsonDecodeTypeSupported(t.Elem())
	case *types.Map:
		key, ok := types.Unalias(t.Key()).Underlying().(*types.Basic)
		return ok && key.Kind() == types.String && jsonDecodeTypeSupported(t.Elem())
	case *types.Struct:
		return true
	case *types.Basic:
		switch t.Kind() {
		case types.Bool,
			types.Int, types.Int8, types.Int16, types.Int32, types.Int64,
			types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64, types.Uintptr,
			types.Float32, types.Float64,
			types.String:
			return true
		default:
			return false
		}
	default:
		underlying := typ.Underlying()
		if underlying == typ {
			return false
		}
		return jsonDecodeTypeSupported(underlying)
	}
}

func jsonDecodeFieldSupported(fieldType ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return jsonDecodeTypeSupported(typeInfo.GetType(fieldType))
}

func generateStructJsonDecode(out *strings.Builder, structName string, structType *ast.StructType, generics rustTypeGenerics) {
	if !shouldGenerateJsonDecodeImpl() || structType == nil {
		return
	}
	out.WriteString("\n")
	writeRustTraitImplHeader(out, generics, "GoJsonDecode", RustTypeNameForUse(structName))
	out.WriteString("    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {\n")
	out.WriteString("        let object = value.as_object().ok_or_else(|| go_json_expected(value, \"object\"))?;\n")
	out.WriteString("        let mut out = Self::default();\n")
	for _, field := range structType.Fields.List {
		if !jsonDecodeFieldSupported(field.Type) {
			continue
		}
		for _, name := range field.Names {
			if name == nil || !name.IsExported() {
				continue
			}
			jsonName, include := jsonDecodeFieldName(name.Name, field.Tag)
			if !include {
				continue
			}
			out.WriteString("        if let Some(field_value) = object.get(")
			out.WriteString(strconv.Quote(jsonName))
			out.WriteString(") {\n")
			out.WriteString("            out.")
			out.WriteString(ToSnakeCase(name.Name))
			out.WriteString(" = ")
			if _, ok := sliceElemPtrFieldInfoForStructField(nil, structType, structName, name.Name); ok {
				NeedSliceElemPtr()
				out.WriteString("GoPtr::local(")
				out.WriteString("<")
				out.WriteString(GoTypeToRust(field.Type))
				out.WriteString(" as GoJsonDecode>::go_json_decode(field_value)?)")
			} else {
				out.WriteString("<")
				out.WriteString(GoTypeToRust(field.Type))
				out.WriteString(" as GoJsonDecode>::go_json_decode(field_value)?")
			}
			out.WriteString(";\n")
			out.WriteString("        }\n")
		}
	}
	out.WriteString("        Ok(out)\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func typeHasTraitField(expr ast.Expr) bool {
	return typeHasTraitFieldSeen(expr, make(map[string]bool))
}

func typeCanDeriveDebug(expr ast.Expr, seen map[string]bool) bool {
	if isFunctionSignatureTypeExpr(expr) {
		return false
	}

	fieldType := goTypeToRustBase(expr)
	if strings.Contains(fieldType, "dyn ") {
		return false
	}

	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if typ := typeInfo.GetType(expr); typ != nil {
			if typeReferencesExternalNamedType(typ) {
				return false
			}
			return goTypesTypeCanDeriveDebug(typ, make(map[*types.Named]bool))
		}
	}

	switch t := expr.(type) {
	case *ast.ArrayType:
		return typeCanDeriveDebug(t.Elt, seen)
	case *ast.MapType:
		return typeCanDeriveDebug(t.Key, seen) && typeCanDeriveDebug(t.Value, seen)
	case *ast.StarExpr:
		return typeCanDeriveDebug(t.X, seen)
	case *ast.StructType:
		return structCanDeriveDebug(t)
	case *ast.FuncType, *ast.InterfaceType:
		return false
	case *ast.Ident:
		if seen[t.Name] {
			return true
		}
		if def, ok := structDefs[t.Name]; ok && def.ASTType != nil {
			seen[t.Name] = true
			return structCanDeriveDebugSeen(def.ASTType, seen)
		}
	case *ast.SelectorExpr:
		if ident, ok := t.X.(*ast.Ident); ok {
			return goPackageImports[ident.Name] == "" || isStdlibPackage(goPackageImports[ident.Name])
		}
	}
	return true
}

func typeDefinitionCanDeriveDebug(typeSpec *ast.TypeSpec) bool {
	if typeSpec == nil || typeSpec.Name == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj, ok := typeInfo.info.Defs[typeSpec.Name].(*types.TypeName)
	if !ok || obj == nil || obj.Type() == nil {
		return false
	}
	return goTypesTypeCanDeriveDebug(obj.Type(), make(map[*types.Named]bool))
}

func goTypesTypeCanDeriveDebug(typ types.Type, seen map[*types.Named]bool) bool {
	if typ == nil {
		return false
	}
	typ = types.Unalias(typ)
	switch t := typ.(type) {
	case *types.Named:
		if !goTypesNamedTypeGeneratedInCurrentPackage(t) {
			return false
		}
		if seen[t] {
			return true
		}
		seen[t] = true
		return goTypesTypeCanDeriveDebug(t.Underlying(), seen)
	case *types.Basic:
		return t.Kind() != types.Invalid
	case *types.Pointer:
		return goTypesTypeCanDeriveDebug(t.Elem(), seen)
	case *types.Slice:
		return goTypesTypeCanDeriveDebug(t.Elem(), seen)
	case *types.Array:
		return goTypesTypeCanDeriveDebug(t.Elem(), seen)
	case *types.Map:
		return goTypesMapKeyCanDeriveDebug(t.Key(), seen) && goTypesTypeCanDeriveDebug(t.Elem(), seen)
	case *types.Struct:
		for i := 0; i < t.NumFields(); i++ {
			if !goTypesTypeCanDeriveDebug(t.Field(i).Type(), seen) {
				return false
			}
		}
		return true
	case *types.Chan:
		return true
	case *types.Interface, *types.Signature, *types.TypeParam:
		return false
	default:
		return false
	}
}

func goTypesNamedTypeGeneratedInCurrentPackage(named *types.Named) bool {
	if named == nil || named.Obj() == nil || named.Obj().Pkg() == nil {
		return true
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.pkg != nil && named.Obj().Pkg().Path() == typeInfo.pkg.Path()
}

func goTypesMapKeyCanDeriveDebug(typ types.Type, seen map[*types.Named]bool) bool {
	if typ == nil {
		return false
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Pointer, *types.Interface:
		return true
	default:
		return goTypesTypeCanDeriveDebug(typ, seen)
	}
}

func typeReferencesExternalNamedType(typ types.Type) bool {
	return typeReferencesExternalNamedTypeSeen(types.Unalias(typ), make(map[types.Type]bool))
}

func typeReferencesExternalNamedTypeSeen(typ types.Type, seen map[types.Type]bool) bool {
	if typ == nil {
		return false
	}
	typ = types.Unalias(typ)
	if seen[typ] {
		return false
	}
	seen[typ] = true

	switch t := typ.(type) {
	case *types.Named:
		if t.Obj() != nil && t.Obj().Pkg() != nil {
			pkgPath := t.Obj().Pkg().Path()
			if typeInfo := GetTypeInfo(); typeInfo == nil || typeInfo.pkg == nil || t.Obj().Pkg() != typeInfo.pkg {
				if !isStdlibPackage(pkgPath) || isSourceMappedPackagePath(pkgPath) {
					return true
				}
			}
		}
		return typeReferencesExternalNamedTypeSeen(t.Underlying(), seen)
	case *types.Array:
		return typeReferencesExternalNamedTypeSeen(t.Elem(), seen)
	case *types.Slice:
		return typeReferencesExternalNamedTypeSeen(t.Elem(), seen)
	case *types.Map:
		return typeReferencesExternalNamedTypeSeen(t.Key(), seen) || typeReferencesExternalNamedTypeSeen(t.Elem(), seen)
	case *types.Pointer:
		return typeReferencesExternalNamedTypeSeen(t.Elem(), seen)
	case *types.Struct:
		for i := 0; i < t.NumFields(); i++ {
			if typeReferencesExternalNamedTypeSeen(t.Field(i).Type(), seen) {
				return true
			}
		}
	}
	return false
}

func mapFieldNeedsOpaqueDisplay(expr ast.Expr) bool {
	mapType, ok := expr.(*ast.MapType)
	if !ok {
		return false
	}
	return !mapValueCanUseDisplay(mapType.Value)
}

func mapValueCanUseDisplay(expr ast.Expr) bool {
	if isEmptyInterfaceExpr(expr) {
		return false
	}
	if isFunctionSignatureTypeExpr(expr) {
		return false
	}
	switch expr.(type) {
	case *ast.ArrayType, *ast.MapType, *ast.FuncType:
		return false
	default:
		return true
	}
}

func typeHasTraitFieldSeen(expr ast.Expr, seen map[string]bool) bool {
	fieldType := goTypeToRustBase(expr)
	if strings.Contains(fieldType, "dyn ") {
		return true
	}

	switch t := expr.(type) {
	case *ast.ArrayType:
		return typeHasTraitFieldSeen(t.Elt, seen)
	case *ast.MapType:
		return typeHasTraitFieldSeen(t.Value, seen)
	case *ast.StructType:
		return structHasTraitField(t)
	case *ast.Ident:
		if seen[t.Name] {
			return false
		}
		if def, ok := structDefs[t.Name]; ok && def.ASTType != nil {
			seen[t.Name] = true
			for _, field := range def.ASTType.Fields.List {
				if typeHasTraitFieldSeen(field.Type, seen) {
					return true
				}
			}
		}
	default:
		return false
	}
	return false
}

// Helper to check if a function body contains defer statements
func checkHasDefer(stmts []ast.Stmt) bool {
	for _, stmt := range stmts {
		if checkStmtHasDefer(stmt) {
			return true
		}
	}
	return false
}

func checkStmtHasDefer(stmt ast.Stmt) bool {
	switch s := stmt.(type) {
	case *ast.DeferStmt:
		return !isMutexUnlockDefer(s.Call)
	case *ast.BlockStmt:
		return checkHasDefer(s.List)
	case *ast.IfStmt:
		return checkHasDefer(s.Body.List) || checkStmtHasDefer(s.Else)
	case *ast.ForStmt:
		return s.Body != nil && checkHasDefer(s.Body.List)
	case *ast.RangeStmt:
		return s.Body != nil && checkHasDefer(s.Body.List)
	case *ast.SwitchStmt:
		return checkHasDeferClauses(s.Body)
	case *ast.TypeSwitchStmt:
		return checkHasDeferClauses(s.Body)
	case *ast.SelectStmt:
		return checkHasDeferClauses(s.Body)
	case *ast.LabeledStmt:
		return checkStmtHasDefer(s.Stmt)
	default:
		return false
	}
}

func checkHasDeferClauses(body *ast.BlockStmt) bool {
	if body == nil {
		return false
	}
	for _, stmt := range body.List {
		switch clause := stmt.(type) {
		case *ast.CaseClause:
			if checkHasDefer(clause.Body) {
				return true
			}
		case *ast.CommClause:
			if checkHasDefer(clause.Body) {
				return true
			}
		}
	}
	return false
}

func writeFunctionTypeParams(out *strings.Builder, fnType *ast.FuncType) {
	writeFunctionTypeParamsWithParam(out, fnType, rustFunctionTypeParam)
}

func writeFunctionDeclTypeParams(out *strings.Builder, fn *ast.FuncDecl) {
	if fn == nil || fn.Type == nil {
		return
	}
	paramFunc := rustTypeDeclarationParam
	boundKind := genericFunctionBoundKind(fn)
	if boundKind&genericMethodBoundRustClone != 0 && boundKind&genericMethodBoundGoValueClone != 0 {
		paramFunc = rustCloneAndGoValueCloneTypeParam
	} else if boundKind&genericMethodBoundRustClone != 0 {
		paramFunc = rustFunctionTypeParam
	} else if boundKind&genericMethodBoundGoValueClone != 0 {
		paramFunc = rustGoValueCloneTypeParam
	}
	writeFunctionTypeParamsWithParam(out, fn.Type, paramFunc)
}

func genericFunctionBoundKind(fn *ast.FuncDecl) genericMethodBoundKind {
	boundKind := genericMethodBoundNone
	if ctx := GetTranspileContext(); ctx != nil && ctx.Package != nil && ctx.Package.FunctionBoundKinds != nil {
		if cached, ok := ctx.Package.FunctionBoundKinds[fn]; ok {
			boundKind |= cached
		}
	}
	if genericFunctionUsesDirectTypeParamValue(fn) {
		boundKind |= genericMethodBoundGoValueClone
	}
	if genericSignatureUsesGoPtrDirectTypeParam(fn) {
		boundKind |= genericMethodBoundRustClone
	}
	return boundKind
}

func writeFunctionTypeParamsWithParam(out *strings.Builder, fnType *ast.FuncType, paramFunc func(*ast.Ident) string) {
	if fnType == nil || fnType.TypeParams == nil || len(fnType.TypeParams.List) == 0 {
		return
	}
	var params []string
	for _, field := range fnType.TypeParams.List {
		for _, name := range field.Names {
			params = append(params, paramFunc(name))
		}
	}
	if len(params) == 0 {
		return
	}
	out.WriteString("<")
	out.WriteString(strings.Join(params, ", "))
	out.WriteString(">")
}

// writeTypeSpecAliasTypeParams declares a generic type alias's parameters
// (`pub type Seq<V> = ...` for `type Seq[V any] func(yield func(V) bool)`).
// Bare names only: Rust does not enforce bounds on type aliases
// (type_alias_bounds), and the alias body already wraps the parameter
// wherever the underlying signature requires it.
func writeTypeSpecAliasTypeParams(out *strings.Builder, typeSpec *ast.TypeSpec) {
	if typeSpec == nil || typeSpec.TypeParams == nil || len(typeSpec.TypeParams.List) == 0 {
		return
	}
	var params []string
	for _, field := range typeSpec.TypeParams.List {
		for _, name := range field.Names {
			params = append(params, RustTypeNameForUse(name.Name))
		}
	}
	if len(params) == 0 {
		return
	}
	out.WriteString("<")
	out.WriteString(strings.Join(params, ", "))
	out.WriteString(">")
}

func genericFunctionUsesDirectTypeParamValue(fn *ast.FuncDecl) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || fn == nil || fn.Type == nil {
		return false
	}
	if fieldListHasDirectTypeParam(typeInfo, fn.Type.Params) || fieldListHasDirectTypeParam(typeInfo, fn.Type.Results) {
		return true
	}
	return methodBodyUsesDirectTypeParamValue(typeInfo, fn.Body)
}

func rustTypeGenericsForTypeSpec(typeSpec *ast.TypeSpec) rustTypeGenerics {
	return rustTypeGenericsForTypeSpecWithParam(typeSpec, rustFunctionTypeParam)
}

func rustTypeGenericsForDeclarationTypeSpec(typeSpec *ast.TypeSpec) rustTypeGenerics {
	return rustTypeGenericsForTypeSpecWithParam(typeSpec, rustTypeDeclarationParam)
}

func rustTypeGenericsForTypeSpecWithParam(typeSpec *ast.TypeSpec, paramFunc func(*ast.Ident) string) rustTypeGenerics {
	if typeSpec == nil || typeSpec.TypeParams == nil || len(typeSpec.TypeParams.List) == 0 {
		return rustTypeGenerics{}
	}
	var declParams []string
	var useParams []string
	for _, field := range typeSpec.TypeParams.List {
		for _, name := range field.Names {
			declParams = append(declParams, paramFunc(name))
			useParams = append(useParams, RustTypeNameForUse(name.Name))
		}
	}
	if len(declParams) == 0 {
		return rustTypeGenerics{}
	}
	return rustTypeGenerics{
		Decl: "<" + strings.Join(declParams, ", ") + ">",
		Use:  "<" + strings.Join(useParams, ", ") + ">",
	}
}

func rustTypeGenericsForStructTypeSpec(typeSpec *ast.TypeSpec, structType *ast.StructType) rustTypeGenerics {
	generics := rustTypeGenericsForDeclarationTypeSpec(typeSpec)
	generics.Phantom = rustUnusedTypeParamsForStruct(typeSpec, structType)
	return generics
}

func rustStructDisplayGenerics(generics rustTypeGenerics, typeSpec *ast.TypeSpec, structType *ast.StructType) (rustTypeGenerics, bool) {
	displayParams, ok := rustDisplayTypeParamsForStruct(typeSpec, structType)
	if !ok {
		return generics, false
	}
	if len(displayParams) == 0 {
		return generics, true
	}
	clauses := make([]string, 0, len(displayParams))
	for _, param := range displayParams {
		clauses = append(clauses, param+": std::fmt::Display")
	}
	generics.Where = " where " + strings.Join(clauses, ", ")
	return generics, true
}

func rustDisplayTypeParamsForStruct(typeSpec *ast.TypeSpec, structType *ast.StructType) ([]string, bool) {
	if typeSpec == nil || typeSpec.TypeParams == nil || structType == nil {
		return nil, true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	used := make(map[string]bool)
	for _, field := range structType.Fields.List {
		if field == nil || field.Type == nil || !structDisplayFieldUsesDefaultFormatter(field.Type) {
			continue
		}
		typ := typeInfo.GetType(field.Type)
		if typ == nil {
			return nil, false
		}
		if !collectRustDisplayTypeParamUses(typ, used, make(map[types.Type]bool)) {
			return nil, false
		}
	}
	if len(used) == 0 {
		return nil, true
	}
	var params []string
	for _, field := range typeSpec.TypeParams.List {
		for _, name := range field.Names {
			rustName := RustTypeNameForUse(name.Name)
			if used[rustName] {
				params = append(params, rustName)
			}
		}
	}
	return params, true
}

func structDisplayFieldUsesDefaultFormatter(expr ast.Expr) bool {
	if expr == nil || isSyncParam(expr) || isEmptyInterfaceExpr(expr) || isFunctionSignatureTypeExpr(expr) {
		return false
	}
	switch expr.(type) {
	case *ast.ChanType, *ast.MapType:
		return false
	}
	if arrayFieldContainsFunction(expr) ||
		sliceFieldContainsEmptyInterface(expr) ||
		mapFieldNeedsOpaqueDisplay(expr) ||
		arrayFieldNestedInnerIsPointer(expr) ||
		arrayFieldNestedPointerInnerIsValue(expr) ||
		arrayFieldNestedPointerInnerIsPointer(expr) ||
		arrayFieldContainsSlice(expr) ||
		pointerFieldContainsPointerSlice(expr) ||
		pointerFieldContainsSlice(expr) ||
		arrayFieldContainsPointer(expr) ||
		arrayFieldContainsLocalInterface(expr) ||
		structDisplayFieldIsPointer(expr) {
		return false
	}
	if _, ok := expr.(*ast.ArrayType); ok {
		return false
	}
	return true
}

func collectRustDisplayTypeParamUses(typ types.Type, used map[string]bool, seen map[types.Type]bool) bool {
	if typ == nil {
		return true
	}
	typ = types.Unalias(typ)
	if seen[typ] {
		return true
	}
	seen[typ] = true

	switch t := typ.(type) {
	case *types.TypeParam:
		if t.Obj() != nil {
			used[RustTypeNameForUse(t.Obj().Name())] = true
		}
	case *types.Named:
		indexes, ok := rustNamedDisplayTypeArgIndexes(t, seen)
		if !ok {
			return false
		}
		typeArgs := t.TypeArgs()
		for _, index := range indexes {
			if typeArgs == nil || index < 0 || index >= typeArgs.Len() {
				continue
			}
			if !collectRustDisplayTypeParamUses(typeArgs.At(index), used, seen) {
				return false
			}
		}
	case *types.Pointer, *types.Slice, *types.Array, *types.Map, *types.Chan, *types.Signature, *types.Interface:
		return true
	case *types.Tuple:
		for i := 0; i < t.Len(); i++ {
			if !collectRustDisplayTypeParamUses(t.At(i).Type(), used, seen) {
				return false
			}
		}
	case *types.Struct:
		for i := 0; i < t.NumFields(); i++ {
			fieldType := t.Field(i).Type()
			if !structDisplayTypeUsesDefaultFormatter(fieldType) {
				continue
			}
			if !collectRustDisplayTypeParamUses(fieldType, used, seen) {
				return false
			}
		}
	}
	return true
}

func rustNamedDisplayTypeArgIndexes(named *types.Named, seen map[types.Type]bool) ([]int, bool) {
	if named == nil || named.TypeArgs() == nil || named.TypeArgs().Len() == 0 {
		return nil, true
	}
	origin := named
	if named.Origin() != nil {
		origin = named.Origin()
	}
	if namedDisplayIgnoresTypeArgs(origin) {
		return nil, true
	}
	typeParams := origin.TypeParams()
	if typeParams == nil || typeParams.Len() == 0 {
		return nil, true
	}
	obj := origin.Obj()
	typeInfo := GetTypeInfo()
	if obj == nil || obj.Pkg() == nil || typeInfo == nil || typeInfo.pkg == nil || obj.Pkg() != typeInfo.pkg {
		indexes := make([]int, named.TypeArgs().Len())
		for i := range indexes {
			indexes[i] = i
		}
		return indexes, true
	}
	structType, ok := origin.Underlying().(*types.Struct)
	if !ok {
		indexes := make([]int, named.TypeArgs().Len())
		for i := range indexes {
			indexes[i] = i
		}
		return indexes, true
	}
	formalUses := make(map[string]bool)
	for i := 0; i < structType.NumFields(); i++ {
		fieldType := structType.Field(i).Type()
		if !structDisplayTypeUsesDefaultFormatter(fieldType) {
			continue
		}
		if !collectRustDisplayTypeParamUses(fieldType, formalUses, seen) {
			return nil, false
		}
	}
	var indexes []int
	for i := 0; i < typeParams.Len(); i++ {
		typeParam := typeParams.At(i)
		if typeParam == nil || typeParam.Obj() == nil {
			continue
		}
		if formalUses[RustTypeNameForUse(typeParam.Obj().Name())] {
			indexes = append(indexes, i)
		}
	}
	return indexes, true
}

func namedDisplayIgnoresTypeArgs(named *types.Named) bool {
	if named == nil || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	obj := named.Obj()
	return obj.Pkg().Path() == "sync/atomic" && obj.Name() == "Pointer"
}

func structDisplayTypeUsesDefaultFormatter(typ types.Type) bool {
	if typ == nil {
		return false
	}
	typ = types.Unalias(typ)
	if isGoSyncNamedType(typ) || isEmptyInterfaceType(typ) {
		return false
	}
	if _, ok := typ.(*types.TypeParam); ok {
		return true
	}
	switch typ.Underlying().(type) {
	case *types.Pointer, *types.Slice, *types.Array, *types.Map, *types.Chan, *types.Signature, *types.Interface:
		return false
	default:
		return true
	}
}

func rustUnusedTypeParamsForStruct(typeSpec *ast.TypeSpec, structType *ast.StructType) []string {
	if typeSpec == nil || typeSpec.TypeParams == nil || structType == nil {
		return nil
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	typeParams := make([]string, 0, len(typeSpec.TypeParams.List))
	paramSet := make(map[string]bool)
	for _, field := range typeSpec.TypeParams.List {
		for _, name := range field.Names {
			if _, ok := typeInfo.info.Defs[name].(*types.TypeName); !ok {
				continue
			}
			rustName := RustTypeNameForUse(name.Name)
			typeParams = append(typeParams, rustName)
			paramSet[rustName] = true
		}
	}
	if len(typeParams) == 0 {
		return nil
	}
	used := make(map[string]bool)
	for _, field := range structType.Fields.List {
		if field.Type == nil {
			continue
		}
		collectRustTypeParamUses(typeInfo.GetType(field.Type), used, make(map[types.Type]bool))
	}
	var unused []string
	for _, param := range typeParams {
		if paramSet[param] && !used[param] {
			unused = append(unused, param)
		}
	}
	return unused
}

func collectRustTypeParamUses(typ types.Type, used map[string]bool, seen map[types.Type]bool) {
	if typ == nil || seen[typ] {
		return
	}
	seen[typ] = true
	switch t := types.Unalias(typ).(type) {
	case *types.TypeParam:
		if t.Obj() != nil {
			used[RustTypeNameForUse(t.Obj().Name())] = true
		}
	case *types.Named:
		typeArgs := t.TypeArgs()
		for i := 0; typeArgs != nil && i < typeArgs.Len(); i++ {
			collectRustTypeParamUses(typeArgs.At(i), used, seen)
		}
	case *types.Pointer:
		collectRustTypeParamUses(t.Elem(), used, seen)
	case *types.Slice:
		collectRustTypeParamUses(t.Elem(), used, seen)
	case *types.Array:
		collectRustTypeParamUses(t.Elem(), used, seen)
	case *types.Map:
		collectRustTypeParamUses(t.Key(), used, seen)
		collectRustTypeParamUses(t.Elem(), used, seen)
	case *types.Chan:
		collectRustTypeParamUses(t.Elem(), used, seen)
	case *types.Tuple:
		for i := 0; i < t.Len(); i++ {
			collectRustTypeParamUses(t.At(i).Type(), used, seen)
		}
	case *types.Signature:
		collectRustTypeParamUses(t.Params(), used, seen)
		collectRustTypeParamUses(t.Results(), used, seen)
	case *types.Struct:
		for i := 0; i < t.NumFields(); i++ {
			collectRustTypeParamUses(t.Field(i).Type(), used, seen)
		}
	}
}

func writeRustPhantomField(out *strings.Builder, generics rustTypeGenerics) {
	if len(generics.Phantom) == 0 {
		return
	}
	out.WriteString("    pub __go_phantom: std::marker::PhantomData<")
	writeRustPhantomType(out, generics.Phantom)
	out.WriteString(">,\n")
}

func writeRustPhantomValue(out *strings.Builder, generics rustTypeGenerics, needComma *bool) {
	if len(generics.Phantom) == 0 {
		return
	}
	if *needComma {
		out.WriteString(", ")
	}
	out.WriteString("__go_phantom: std::marker::PhantomData")
	*needComma = true
}

func writeRustPhantomValueForStructDef(out *strings.Builder, structName string, needComma *bool) {
	def := structDefs[structName]
	if def == nil || len(def.PhantomTypeParams) == 0 {
		return
	}
	writeRustPhantomValue(out, rustTypeGenerics{Phantom: def.PhantomTypeParams}, needComma)
}

func writeRustPhantomType(out *strings.Builder, params []string) {
	if len(params) == 1 {
		out.WriteString(params[0])
		return
	}
	out.WriteString("(")
	out.WriteString(strings.Join(params, ", "))
	out.WriteString(")")
}

func rustFunctionTypeParam(name *ast.Ident) string {
	return rustTypeParam(name, []string{"Clone"})
}

func rustTypeDeclarationParam(name *ast.Ident) string {
	return rustTypeParam(name, nil)
}

func rustGoValueCloneTypeParam(name *ast.Ident) string {
	return rustTypeParam(name, []string{"GoValueClone"})
}

func rustCloneAndGoValueCloneTypeParam(name *ast.Ident) string {
	return rustTypeParam(name, []string{"Clone", "GoValueClone"})
}

func rustTypeParam(name *ast.Ident, cloneBounds []string) string {
	rustName := RustTypeNameForUse(name.Name)
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return rustName
	}
	obj, ok := typeInfo.info.Defs[name].(*types.TypeName)
	if !ok {
		return rustName
	}
	if goTypeParamHasComparableConstraint(obj.Type()) {
		TrackImport("Any")
		NeedGoComparable()
		bounds := []string{"Any", "GoComparable"}
		bounds = appendTypeParamCloneBounds(bounds, cloneBounds, true)
		if NeedsConcurrentWrapper() {
			bounds = append(bounds, "Send", "Sync")
		}
		bounds = append(bounds, "'static")
		return rustName + ": " + strings.Join(bounds, " + ")
	}
	if goTypeParamHasAnyConstraint(obj.Type()) {
		TrackImport("Any")
		bounds := []string{"Any"}
		bounds = appendTypeParamCloneBounds(bounds, cloneBounds, true)
		if NeedsConcurrentWrapper() {
			bounds = append(bounds, "Send", "Sync")
		}
		bounds = append(bounds, "'static")
		return rustName + ": " + strings.Join(bounds, " + ")
	}
	if goTypeParamHasStringByteSliceConstraint(obj.Type()) {
		NeedGoByteSequence()
		bounds := []string{"GoByteSequence"}
		bounds = appendTypeParamCloneBounds(bounds, cloneBounds, false)
		if NeedsConcurrentWrapper() {
			bounds = append(bounds, "Send", "Sync")
		}
		bounds = append(bounds, "'static")
		return rustName + ": " + strings.Join(bounds, " + ")
	}
	if goTypeParamHasIntegerConstraint(obj.Type()) {
		NeedGoInteger()
		bounds := []string{"GoInteger"}
		bounds = appendTypeParamCloneBounds(bounds, cloneBounds, false)
		if NeedsConcurrentWrapper() {
			bounds = append(bounds, "Send", "Sync")
		}
		bounds = append(bounds, "'static")
		return rustName + ": " + strings.Join(bounds, " + ")
	}
	if traitName, ok := goTypeParamOrderedTraitConstraintName(obj.Type()); ok {
		bounds := []string{traitName}
		bounds = appendTypeParamCloneBounds(bounds, cloneBounds, false)
		bounds = append(bounds, "PartialOrd")
		if goTypeParamHasStringConstraint(obj.Type()) {
			bounds = append(bounds, "ToString")
		}
		if NeedsConcurrentWrapper() {
			bounds = append(bounds, "Send", "Sync")
		}
		bounds = append(bounds, "'static")
		return rustName + ": " + strings.Join(bounds, " + ")
	}
	if goTypeParamHasOrderedConstraint(obj.Type()) {
		var bounds []string
		bounds = appendTypeParamCloneBounds(bounds, cloneBounds, false)
		bounds = append(bounds, "PartialOrd")
		if goTypeParamHasStringConstraint(obj.Type()) {
			bounds = append(bounds, "ToString")
		}
		if NeedsConcurrentWrapper() {
			bounds = append(bounds, "Send", "Sync")
		}
		bounds = append(bounds, "'static")
		return rustName + ": " + strings.Join(bounds, " + ")
	}
	if goTypeParamHasPointerConstraint(obj.Type()) {
		var bounds []string
		bounds = appendTypeParamCloneBounds(bounds, cloneBounds, false)
		if NeedsConcurrentWrapper() {
			bounds = append(bounds, "Send", "Sync")
		}
		bounds = append(bounds, "'static")
		return rustName + ": " + strings.Join(bounds, " + ")
	}
	traitName, ok := goTypeParamTraitConstraintName(obj.Type())
	if !ok {
		return rustName
	}
	bounds := []string{traitName}
	bounds = appendTypeParamCloneBounds(bounds, cloneBounds, false)
	if NeedsConcurrentWrapper() {
		bounds = append(bounds, "Send", "Sync")
	}
	bounds = append(bounds, "'static")
	return rustName + ": " + strings.Join(bounds, " + ")
}

func appendTypeParamCloneBounds(bounds []string, cloneBounds []string, allowGoValueClone bool) []string {
	for _, bound := range cloneBounds {
		if bound == "" {
			continue
		}
		if bound == "GoValueClone" && !allowGoValueClone {
			bound = "Clone"
		}
		if bound == "GoValueClone" {
			NeedGoValueClone()
		}
		bounds = append(bounds, bound)
	}
	return bounds
}

// localInterfaceTypesFromTypeSpec returns the go/types representation of the
// interface declared by typeSpec, or nil if type information is unavailable or
// the type is not an interface.
func localInterfaceTypesFromTypeSpec(typeSpec *ast.TypeSpec) *types.Interface {
	if typeSpec == nil {
		return nil
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	obj := typeInfo.info.Defs[typeSpec.Name]
	if obj == nil {
		return nil
	}
	named, ok := obj.Type().(*types.Named)
	if !ok {
		return nil
	}
	iface, ok := named.Underlying().(*types.Interface)
	if !ok {
		return nil
	}
	return iface
}

// packageLocalInterfaceNames returns the names of every named interface type
// declared in the current package — across all files — unioned with the AST
// interfaces of the file being emitted (a fallback for when type info is
// unavailable). Trait-impl generation iterates this so a concrete type picks up
// the interfaces it implements regardless of which file declares them.
func packageLocalInterfaceNames(fileInterfaces map[string]*ast.InterfaceType) []string {
	seen := map[string]bool{}
	var names []string
	add := func(name string) {
		if name == "" || seen[name] {
			return
		}
		seen[name] = true
		names = append(names, name)
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.pkg != nil {
		scope := typeInfo.pkg.Scope()
		for _, name := range scope.Names() {
			tn, ok := scope.Lookup(name).(*types.TypeName)
			if !ok {
				continue
			}
			named, ok := tn.Type().(*types.Named)
			if !ok {
				continue
			}
			if _, ok := named.Underlying().(*types.Interface); ok {
				add(name)
			}
		}
	}
	for name := range fileInterfaces {
		add(name)
	}
	return names
}

// localInterfaceTypesByName looks up the go/types interface for a top-level
// type name in the current package.
func localInterfaceTypesByName(name string) *types.Interface {
	_, iface := localInterfaceNamedTypeByName(name)
	return iface
}

func localInterfaceNamedTypeByName(name string) (*types.Named, *types.Interface) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil {
		return nil, nil
	}
	obj := typeInfo.pkg.Scope().Lookup(name)
	if obj == nil {
		return nil, nil
	}
	typeName, ok := obj.(*types.TypeName)
	if !ok {
		return nil, nil
	}
	named, ok := typeName.Type().(*types.Named)
	if !ok {
		return nil, nil
	}
	iface, ok := named.Underlying().(*types.Interface)
	if !ok {
		return nil, nil
	}
	return named, iface
}

func localInterfaceCanRustTraitUpcast(sourceName, targetName string) bool {
	if sourceName == "" || targetName == "" || sourceName == targetName {
		return false
	}
	source := localInterfaceTypesByName(sourceName)
	return localInterfaceEmbedsNamed(source, targetName)
}

func localInterfaceEmbedsNamed(iface *types.Interface, targetName string) bool {
	if iface == nil || targetName == "" {
		return false
	}
	for i := 0; i < iface.NumEmbeddeds(); i++ {
		named, ok := types.Unalias(iface.EmbeddedType(i)).(*types.Named)
		if !ok || named.Obj() == nil {
			continue
		}
		if named.Obj().Name() == targetName {
			return true
		}
		embedded, _ := named.Underlying().(*types.Interface)
		if localInterfaceEmbedsNamed(embedded, targetName) {
			return true
		}
	}
	return false
}

// embeddedLocalInterfaceNames returns the Rust trait names of named local
// interfaces directly embedded in the given InterfaceType AST. The result is
// in source order.
func embeddedLocalInterfaceNames(t *ast.InterfaceType) []string {
	if t == nil || t.Methods == nil {
		return nil
	}
	var names []string
	for _, method := range t.Methods.List {
		if len(method.Names) > 0 {
			continue
		}
		if name, ok := transpiledNamedInterfaceTypeNameFromExpr(method.Type); ok {
			names = append(names, name)
		}
	}
	return names
}

// localInterfaceHasEmbeddedInterfaces reports whether the named local
// interface embeds any other named local interface (i.e., whether its Rust
// trait has Rust supertraits beyond Display/Any).
func localInterfaceHasEmbeddedInterfaces(ifaceName string) bool {
	return interfaceTypeHasNamedEmbedded(localInterfaceTypesByName(ifaceName))
}

// methodFromEmbeddedInterface reports whether the given method on the named
// local interface is inherited from an embedded named local interface (rather
// than declared directly in the interface body).
func methodFromEmbeddedInterface(ifaceName, methodName string) bool {
	iface := localInterfaceTypesByName(ifaceName)
	if iface == nil {
		return false
	}
	for i := 0; i < iface.NumEmbeddeds(); i++ {
		named, ok := types.Unalias(iface.EmbeddedType(i)).(*types.Named)
		if !ok {
			continue
		}
		embedded, ok := named.Underlying().(*types.Interface)
		if !ok {
			continue
		}
		if _, ok := transpiledNamedInterfaceTypeNameFromTypes(named); !ok {
			continue
		}
		for j := 0; j < embedded.NumMethods(); j++ {
			if embedded.Method(j).Name() == methodName {
				return true
			}
		}
	}
	return false
}

// writeTraitMethodSigFromTypes writes a Rust trait method signature derived
// from a go/types signature. Used to emit methods inherited from embedded
// interfaces whose declarations aren't directly walked by the AST loop.
func writeTraitMethodSigFromTypes(out *strings.Builder, name string, sig *types.Signature) {
	out.WriteString("    fn ")
	out.WriteString(ToSnakeCase(name))
	out.WriteString("(&self")
	params := sig.Params()
	for j := 0; j < params.Len(); j++ {
		p := params.At(j)
		pName := p.Name()
		if pName == "" {
			pName = fmt.Sprintf("_arg%d", j)
		}
		out.WriteString(", ")
		out.WriteString(RustLocalIdent(pName))
		out.WriteString(": ")
		out.WriteString(goTypesParamTypeToRust(p.Type()))
	}
	out.WriteString(")")
	res := sig.Results()
	switch res.Len() {
	case 0:
	case 1:
		out.WriteString(" -> ")
		out.WriteString(goTypesReturnTypeToRust(res.At(0).Type()))
	default:
		out.WriteString(" -> (")
		for j := 0; j < res.Len(); j++ {
			if j > 0 {
				out.WriteString(", ")
			}
			out.WriteString(goTypesReturnTypeToRust(res.At(j).Type()))
		}
		out.WriteString(")")
	}
	out.WriteString(";\n")
}

func writeEmbeddedTraitObjectAdapters(out *strings.Builder, ifaceName string, embeddedTraits []string) {
	for _, embeddedName := range embeddedTraits {
		embeddedIface := localInterfaceTypesByName(embeddedName)
		if embeddedIface == nil {
			continue
		}
		writeEmbeddedTraitObjectAdapter(out, ifaceName, embeddedName, embeddedIface, true)
	}
}

func writeLocalInterfaceGoValueCloneImpl(out *strings.Builder, ifaceName, traitSnake string) {
	NeedGoValueClone()
	out.WriteString("\n\nimpl GoValueClone for ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString(" {\n")
	out.WriteString("    fn go_value_clone(&self) -> Self {\n")
	out.WriteString("        ")
	out.WriteString(ifaceName)
	out.WriteString("::__go_clone_box_")
	out.WriteString(traitSnake)
	out.WriteString("(self.as_ref())\n")
	out.WriteString("    }\n")
	out.WriteString("}")
}

func localInterfaceNeedsGoValueCloneImpl(ifaceName string) bool {
	ctx := GetTranspileContext()
	if ctx == nil || ctx.Package == nil || ctx.Package.LocalInterfaceGoValueClone == nil {
		return false
	}
	return ctx.Package.LocalInterfaceGoValueClone[ifaceName]
}

func writeLocalInterfaceGoComparableImpl(out *strings.Builder, ifaceName, traitSnake string) {
	NeedGoComparable()
	out.WriteString("\n\nimpl GoComparable for ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString(" {\n")
	out.WriteString("    fn go_eq(&self, other: &Self) -> bool {\n")
	out.WriteString("        self.__go_eq_")
	out.WriteString(traitSnake)
	out.WriteString("(other.as_ref())\n")
	out.WriteString("    }\n")
	out.WriteString("    fn go_hash(&self, _seed: usize) -> usize {\n")
	out.WriteString("        panic!(\"interface hash with uncomparable dynamic type\")\n")
	out.WriteString("    }\n")
	out.WriteString("}")
}

func localInterfaceNeedsGoComparableImpl(ifaceName string) bool {
	ctx := GetTranspileContext()
	if ctx == nil || ctx.Package == nil || ctx.Package.LocalInterfaceGoComparable == nil {
		return false
	}
	return ctx.Package.LocalInterfaceGoComparable[ifaceName]
}

func localInterfaceNeedsMapKeyType(ifaceName string) bool {
	ctx := GetTranspileContext()
	if ctx == nil || ctx.Package == nil || ctx.Package.LocalInterfaceMapKeyTypes == nil {
		return false
	}
	return ctx.Package.LocalInterfaceMapKeyTypes[ifaceName]
}

func writeLocalInterfaceMapKeyType(out *strings.Builder, ifaceName, traitSnake string) {
	keyType := localInterfaceMapKeyTypeName(ifaceName)
	traitObject := rustLocalInterfaceTraitObject(ifaceName)
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
		out.WriteString("\n\n#[derive(Clone)]\n")
		out.WriteString("pub struct ")
		out.WriteString(keyType)
		out.WriteString("(pub Arc<Mutex<Option<")
		out.WriteString(traitObject)
		out.WriteString(">>>);\n\n")
		out.WriteString("impl ")
		out.WriteString(keyType)
		out.WriteString(" {\n")
		out.WriteString("    pub fn new(value: Arc<Mutex<Option<")
		out.WriteString(traitObject)
		out.WriteString(">>>) -> Self { ")
		out.WriteString(keyType)
		out.WriteString("(value) }\n")
		out.WriteString("    pub fn value(&self) -> Arc<Mutex<Option<")
		out.WriteString(traitObject)
		out.WriteString(">>> { self.0.clone() }\n")
		out.WriteString("    fn addr(&self) -> usize { Arc::as_ptr(&self.0) as usize }\n")
		out.WriteString("    fn identity(&self) -> (u64, String) {\n")
		out.WriteString("        let __guard = self.0.lock().unwrap();\n")
		out.WriteString("        match __guard.as_ref() {\n")
		out.WriteString("            None => (0, String::new()),\n")
		out.WriteString("            Some(__v) => {\n")
		out.WriteString("                let mut __hasher = std::collections::hash_map::DefaultHasher::new();\n")
		out.WriteString("                std::hash::Hash::hash(&__v.as_ref().__go_as_any().type_id(), &mut __hasher);\n")
		out.WriteString("                (std::hash::Hasher::finish(&__hasher), format!(\"{}\", __v))\n")
		out.WriteString("            }\n")
		out.WriteString("        }\n")
		out.WriteString("    }\n")
		out.WriteString("}\n")
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
		out.WriteString("\n\n#[derive(Clone)]\n")
		out.WriteString("pub struct ")
		out.WriteString(keyType)
		out.WriteString("(pub Rc<RefCell<Option<")
		out.WriteString(traitObject)
		out.WriteString(">>>);\n\n")
		out.WriteString("impl ")
		out.WriteString(keyType)
		out.WriteString(" {\n")
		out.WriteString("    pub fn new(value: Rc<RefCell<Option<")
		out.WriteString(traitObject)
		out.WriteString(">>>) -> Self { ")
		out.WriteString(keyType)
		out.WriteString("(value) }\n")
		out.WriteString("    pub fn value(&self) -> Rc<RefCell<Option<")
		out.WriteString(traitObject)
		out.WriteString(">>> { self.0.clone() }\n")
		out.WriteString("    fn addr(&self) -> usize { Rc::as_ptr(&self.0) as usize }\n")
		out.WriteString("    fn identity(&self) -> (u64, String) {\n")
		out.WriteString("        let __guard = self.0.borrow();\n")
		out.WriteString("        match __guard.as_ref() {\n")
		out.WriteString("            None => (0, String::new()),\n")
		out.WriteString("            Some(__v) => {\n")
		out.WriteString("                let mut __hasher = std::collections::hash_map::DefaultHasher::new();\n")
		out.WriteString("                std::hash::Hash::hash(&__v.as_ref().__go_as_any().type_id(), &mut __hasher);\n")
		out.WriteString("                (std::hash::Hasher::finish(&__hasher), format!(\"{}\", __v))\n")
		out.WriteString("            }\n")
		out.WriteString("        }\n")
		out.WriteString("    }\n")
		out.WriteString("}\n")
	}
	out.WriteString("impl PartialEq for ")
	out.WriteString(keyType)
	out.WriteString(" {\n")
	out.WriteString("    fn eq(&self, other: &Self) -> bool {\n")
	if NeedsConcurrentWrapper() {
		out.WriteString("        let __left_guard = self.0.lock().unwrap();\n")
		out.WriteString("        let __right_guard = other.0.lock().unwrap();\n")
	} else {
		out.WriteString("        let __left_guard = self.0.borrow();\n")
		out.WriteString("        let __right_guard = other.0.borrow();\n")
	}
	out.WriteString("        match (__left_guard.as_ref(), __right_guard.as_ref()) {\n")
	out.WriteString("            (None, None) => true,\n")
	out.WriteString("            (Some(__left), Some(__right)) => __left.as_ref().__go_eq_")
	out.WriteString(traitSnake)
	out.WriteString("(__right.as_ref()),\n")
	out.WriteString("            _ => false,\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
	out.WriteString("impl Eq for ")
	out.WriteString(keyType)
	out.WriteString(" {}\n")
	out.WriteString("impl PartialOrd for ")
	out.WriteString(keyType)
	out.WriteString(" {\n")
	out.WriteString("    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }\n")
	out.WriteString("}\n")
	out.WriteString("impl Ord for ")
	out.WriteString(keyType)
	out.WriteString(" {\n")
	out.WriteString("    fn cmp(&self, other: &Self) -> std::cmp::Ordering {\n")
	out.WriteString("        if self == other { return std::cmp::Ordering::Equal; }\n")
	out.WriteString("        match self.identity().cmp(&other.identity()) {\n")
	out.WriteString("            std::cmp::Ordering::Equal => self.addr().cmp(&other.addr()),\n")
	out.WriteString("            ordering => ordering,\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
	out.WriteString("impl std::fmt::Debug for ")
	out.WriteString(keyType)
	out.WriteString(" {\n")
	out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, \"{}\", self.identity().1) }\n")
	out.WriteString("}\n")
	out.WriteString("impl std::fmt::Display for ")
	out.WriteString(keyType)
	out.WriteString(" {\n")
	out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, \"{}\", self.identity().1) }\n")
	out.WriteString("}")
}

func writeAssignableInterfaceObjectAdapters(out *strings.Builder, ifaceName string) {
	sourceNamed, _ := localInterfaceNamedTypeByName(ifaceName)
	if sourceNamed == nil {
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return
	}
	scope := typeInfo.pkg.Scope()
	var targetNames []string
	for _, name := range scope.Names() {
		if name == ifaceName {
			continue
		}
		targetNamed, targetIface := localInterfaceNamedTypeByName(name)
		if targetNamed == nil || targetIface == nil || targetIface.NumMethods() == 0 {
			continue
		}
		if !types.AssignableTo(sourceNamed, targetNamed) {
			continue
		}
		if localInterfaceCanRustTraitUpcast(ifaceName, name) {
			continue
		}
		targetNames = append(targetNames, name)
	}
	sort.Strings(targetNames)
	for _, targetName := range targetNames {
		writeEmbeddedTraitObjectAdapter(out, ifaceName, targetName, localInterfaceTypesByName(targetName), false)
	}
}

func writeEmbeddedTraitObjectAdapter(out *strings.Builder, ifaceName, embeddedName string, embeddedIface *types.Interface, delegateEquality bool) {
	out.WriteString("\n\nimpl ")
	out.WriteString(embeddedName)
	out.WriteString(" for ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString(" {\n")
	writeEmbeddedTraitObjectClone(out, embeddedName)
	if !interfaceTypeHasNamedEmbedded(embeddedIface) {
		out.WriteString("    fn __go_as_any(&self) -> &dyn Any {\n")
		out.WriteString("        (**self).__go_as_any()\n")
		out.WriteString("    }\n")
	}
	writeEmbeddedTraitObjectEq(out, ifaceName, embeddedName, delegateEquality)
	for i := 0; i < embeddedIface.NumMethods(); i++ {
		method := embeddedIface.Method(i)
		writeEmbeddedTraitObjectMethod(out, method)
	}
	out.WriteString("}")
}

func writeEmbeddedTraitObjectClone(out *strings.Builder, embeddedName string) {
	traitSnake := traitMethodSuffix(embeddedName)
	out.WriteString("    fn __go_clone_box_")
	out.WriteString(traitSnake)
	out.WriteString("(&self) -> ")
	out.WriteString(rustLocalInterfaceTraitObject(embeddedName))
	out.WriteString(" {\n")
	out.WriteString("        Box::new((*self).clone()) as ")
	out.WriteString(rustLocalInterfaceTraitObject(embeddedName))
	out.WriteString("\n")
	out.WriteString("    }\n")
}

func writeEmbeddedTraitObjectEq(out *strings.Builder, ifaceName, embeddedName string, delegateEquality bool) {
	traitSnake := traitMethodSuffix(embeddedName)
	out.WriteString("    fn __go_eq_")
	out.WriteString(traitSnake)
	out.WriteString("(&self, other: ")
	out.WriteString(rustLocalInterfaceParamBare(embeddedName))
	out.WriteString(") -> bool {\n")
	if !delegateEquality {
		out.WriteString("        let _ = other;\n")
		out.WriteString("        panic!(\"interface equality for structurally adapted ")
		out.WriteString(ifaceName)
		out.WriteString(" as ")
		out.WriteString(embeddedName)
		out.WriteString("\")\n")
		out.WriteString("    }\n")
		return
	}
	out.WriteString("        (**self).__go_eq_")
	out.WriteString(traitSnake)
	out.WriteString("(other)\n")
	out.WriteString("    }\n")
}

func writeEmbeddedTraitObjectMethod(out *strings.Builder, method *types.Func) {
	if method == nil {
		return
	}
	sig, ok := method.Type().(*types.Signature)
	if !ok {
		return
	}
	out.WriteString("    fn ")
	out.WriteString(ToSnakeCase(method.Name()))
	out.WriteString("(")
	if interfaceMethodRequiresMutableReceiver(method) {
		out.WriteString("&mut self")
	} else {
		out.WriteString("&self")
	}
	params := sig.Params()
	argNames := make([]string, 0, params.Len())
	for j := 0; j < params.Len(); j++ {
		p := params.At(j)
		pName := p.Name()
		if pName == "" {
			pName = fmt.Sprintf("_arg%d", j)
		}
		pName = RustLocalIdent(pName)
		argNames = append(argNames, pName)
		out.WriteString(", ")
		out.WriteString(pName)
		out.WriteString(": ")
		out.WriteString(goTypesParamTypeToRust(p.Type()))
	}
	out.WriteString(")")
	writeInterfaceMethodResultTypesFromFunc(out, method)
	out.WriteString(" {\n")
	out.WriteString("        (**self).")
	out.WriteString(ToSnakeCase(method.Name()))
	out.WriteString("(")
	for i, argName := range argNames {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(argName)
	}
	out.WriteString(")\n")
	out.WriteString("    }\n")
}

func writeLocalInterfaceForwardMethodFromTypes(out *strings.Builder, method *types.Func, receiverType string) {
	if method == nil {
		return
	}
	sig, ok := method.Type().(*types.Signature)
	if !ok {
		return
	}
	out.WriteString("    fn ")
	out.WriteString(ToSnakeCase(method.Name()))
	out.WriteString("(")
	if interfaceMethodRequiresMutableReceiver(method) {
		out.WriteString("&mut self")
	} else {
		out.WriteString("&self")
	}
	params := sig.Params()
	argNames := make([]string, 0, params.Len())
	for j := 0; j < params.Len(); j++ {
		param := params.At(j)
		paramName := param.Name()
		if paramName == "" {
			paramName = fmt.Sprintf("__arg%d", j)
		}
		paramName = RustLocalIdent(paramName)
		argNames = append(argNames, paramName)
		out.WriteString(", ")
		out.WriteString(paramName)
		out.WriteString(": ")
		out.WriteString(goTypesParamTypeToRust(param.Type()))
	}
	out.WriteString(")")
	writeInterfaceMethodResultTypesFromFunc(out, method)
	out.WriteString(" {\n")
	out.WriteString("        ")
	if receiverType != "" {
		out.WriteString(receiverType)
		out.WriteString("::")
		out.WriteString(rustMethodNameForTypesFunc(method))
		out.WriteString("(self")
		if len(argNames) > 0 {
			out.WriteString(", ")
		}
	} else {
		out.WriteString("self.")
		out.WriteString(rustMethodNameForTypesFunc(method))
		out.WriteString("(")
	}
	for j, argName := range argNames {
		if j > 0 {
			out.WriteString(", ")
		}
		out.WriteString(argName)
	}
	out.WriteString(")\n")
	out.WriteString("    }\n")
}

func writeGoTypesSignatureResultTypes(out *strings.Builder, sig *types.Signature) {
	if sig == nil || sig.Results() == nil {
		return
	}
	res := sig.Results()
	switch res.Len() {
	case 0:
	case 1:
		out.WriteString(" -> ")
		out.WriteString(goTypesReturnTypeToRust(res.At(0).Type()))
	default:
		out.WriteString(" -> (")
		for j := 0; j < res.Len(); j++ {
			if j > 0 {
				out.WriteString(", ")
			}
			out.WriteString(goTypesReturnTypeToRust(res.At(j).Type()))
		}
		out.WriteString(")")
	}
}

func writeInterfaceMethodResultTypesFromFunc(out *strings.Builder, method *types.Func) bool {
	if method == nil {
		return false
	}
	sig, ok := method.Type().(*types.Signature)
	if !ok {
		return false
	}
	if resultInfos, ok := goPtrResultInfosForFuncObject(method); ok {
		if writeGoPtrFuncResultTypesFromInfos(out, sig, resultInfos) {
			return true
		}
	}
	writeGoTypesSignatureResultTypes(out, sig)
	return true
}

func writeAnonymousStructEmbeddedInterfaceImpls(out *strings.Builder, typeName string, structType *ast.StructType) {
	if structType == nil || structType.Fields == nil {
		return
	}
	seen := make(map[string]bool)
	for fieldIndex, field := range structType.Fields.List {
		if field == nil || len(field.Names) > 0 {
			continue
		}
		ifaceName, ifaceType, ok := embeddedInterfaceTraitForTypeExpr(field.Type)
		if !ok || seen[ifaceName] {
			continue
		}
		seen[ifaceName] = true
		fieldName := rustStructFieldName(ast.NewIdent(getEmbeddedFieldName(field.Type)), fieldIndex, 0)
		out.WriteString("\n\nimpl ")
		out.WriteString(ifaceName)
		out.WriteString(" for ")
		out.WriteString(typeName)
		out.WriteString(" {\n")
		for i := 0; i < ifaceType.NumMethods(); i++ {
			writeEmbeddedInterfaceFieldTraitMethod(out, fieldName, ifaceType.Method(i))
		}
		writeLocalInterfaceSupportImpl(out, ifaceName, typeName, ifaceType)
		out.WriteString("}")
	}
}

func embeddedInterfaceTraitForTypeExpr(expr ast.Expr) (string, *types.Interface, bool) {
	typ, ok := typeInfoTypeForTypeExpr(expr)
	if !ok {
		return "", nil, false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil {
		return "", nil, false
	}
	iface, ok := types.Unalias(named.Underlying()).(*types.Interface)
	if !ok || iface.NumMethods() == 0 {
		return "", nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil && named.Obj().Pkg() == typeInfo.pkg {
		return RustTypeNameForUse(named.Obj().Name()), iface, true
	}
	if ifaceName, ifaceType, ok := importedTranspiledInterfaceFromType(named); ok {
		return ifaceName, ifaceType, true
	}
	return "", nil, false
}

func writeEmbeddedInterfaceFieldTraitMethod(out *strings.Builder, fieldName string, method *types.Func) {
	if method == nil {
		return
	}
	sig, ok := method.Type().(*types.Signature)
	if !ok {
		return
	}
	params := sig.Params()
	methodName := rustMethodNameForTypesFunc(method)
	mutableReceiver := interfaceMethodRequiresMutableReceiver(method)

	out.WriteString("    fn ")
	out.WriteString(methodName)
	out.WriteString("(")
	if mutableReceiver {
		out.WriteString("&mut self")
	} else {
		out.WriteString("&self")
	}
	for i := 0; i < params.Len(); i++ {
		fmt.Fprintf(out, ", _arg%d: %s", i, goTypesParamTypeToRust(params.At(i).Type()))
	}
	out.WriteString(")")
	writeInterfaceMethodResultTypesFromFunc(out, method)
	out.WriteString(" {\n")
	out.WriteString("        let embedded = self.")
	out.WriteString(fieldName)
	out.WriteString(".clone();\n")
	if mutableReceiver {
		out.WriteString("        let mut guard = embedded")
		WriteBorrowMethod(out, true)
		out.WriteString(";\n")
		out.WriteString("        let embedded_ref = guard.as_mut().unwrap();\n")
	} else {
		out.WriteString("        let guard = embedded")
		WriteBorrowMethod(out, false)
		out.WriteString(";\n")
		out.WriteString("        let embedded_ref = guard.as_ref().unwrap();\n")
	}
	out.WriteString("        embedded_ref.")
	out.WriteString(methodName)
	out.WriteString("(")
	for i := 0; i < params.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		fmt.Fprintf(out, "_arg%d", i)
	}
	out.WriteString(")\n")
	out.WriteString("    }\n")
}

func interfaceParamVarInfo(typeExpr ast.Expr) (*VarInfo, bool) {
	interfaceName, ok := transpiledNamedInterfaceTypeNameFromExpr(typeExpr)
	if !ok {
		return nil, false
	}
	return &VarInfo{
		WrapLevel: WrapFull,
		RustType:  rustLocalInterfaceParam(interfaceName),
		Source:    SourceParam,
	}, true
}

// writeFunctionTypeInterfaceImpls emits per-interface wrapper structs for a
// named function-type alias. For every locally-declared interface that the
// function-type implements (via go/types' method set check), it emits a
// `<FuncType>As<Iface>` struct holding the function value and an `impl Iface`
// block whose methods forward to the function-type's inherent methods. This
// works around Rust's orphan rule, which would block `impl Iface for FuncType`
// directly because `FuncType` is a type alias to entirely foreign types.
func writeFunctionTypeInterfaceImpls(out *strings.Builder, goName, rustTypeName string) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return
	}
	scope := typeInfo.pkg.Scope()
	funcObj, ok := scope.Lookup(goName).(*types.TypeName)
	if !ok {
		return
	}
	funcNamed, ok := funcObj.Type().(*types.Named)
	if !ok {
		return
	}
	for _, ifaceName := range scope.Names() {
		ifaceObj, ok := scope.Lookup(ifaceName).(*types.TypeName)
		if !ok {
			continue
		}
		ifaceNamed, ok := ifaceObj.Type().(*types.Named)
		if !ok {
			continue
		}
		iface, ok := ifaceNamed.Underlying().(*types.Interface)
		if !ok {
			continue
		}
		iface.Complete()
		if iface.NumMethods() == 0 {
			continue
		}
		if !types.Implements(funcNamed, iface) {
			continue
		}
		writeFunctionTypeInterfaceImpl(out, rustTypeName, ifaceName, iface)
	}
}

// functionTypeInterfaceImplResultRust mirrors the Rust signature emitted for
// the interface trait method, including bare pointer handles and wrapped
// named-interface results.
func functionTypeInterfaceImplResultRust(typ types.Type) string {
	return goTypesReturnTypeToRust(typ)
}

func writeFunctionTypeInterfaceImpl(out *strings.Builder, funcTypeName, ifaceName string, iface *types.Interface) {
	wrapperName := functionTypeInterfaceWrapperName(funcTypeName, ifaceName)
	out.WriteString("\n#[derive(Clone)]\n")
	out.WriteString("pub struct ")
	out.WriteString(wrapperName)
	out.WriteString("(pub ")
	out.WriteString(funcTypeName)
	out.WriteString(");\n\n")
	out.WriteString("impl std::fmt::Display for ")
	out.WriteString(wrapperName)
	out.WriteString(" {\n")
	out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
	out.WriteString("        write!(f, \"<")
	out.WriteString(wrapperName)
	out.WriteString(">\")\n")
	out.WriteString("    }\n")
	out.WriteString("}\n\n")
	seen := map[string]bool{ifaceName: true}
	writeFunctionTypeEmbeddedInterfaceImpls(out, funcTypeName, wrapperName, iface, seen)
	writeFunctionTypeInterfaceTraitImpl(out, funcTypeName, wrapperName, ifaceName, iface)
}

type namedInterfaceImpl struct {
	name  string
	iface *types.Interface
}

func functionTypeEmbeddedInterfaces(iface *types.Interface) []namedInterfaceImpl {
	if iface == nil {
		return nil
	}
	var embedded []namedInterfaceImpl
	for i := 0; i < iface.NumEmbeddeds(); i++ {
		embeddedType := iface.EmbeddedType(i)
		if name, ok := transpiledNamedInterfaceTypeNameFromTypes(embeddedType); ok {
			if named, ok := types.Unalias(embeddedType).(*types.Named); ok {
				if embeddedIface, ok := named.Underlying().(*types.Interface); ok {
					embedded = append(embedded, namedInterfaceImpl{name: name, iface: embeddedIface})
				}
			}
			continue
		}
		if name, embeddedIface, ok := importedTranspiledInterfaceFromType(embeddedType); ok {
			embedded = append(embedded, namedInterfaceImpl{name: name, iface: embeddedIface})
		}
	}
	return embedded
}

func writeFunctionTypeEmbeddedInterfaceImpls(out *strings.Builder, funcTypeName, wrapperName string, iface *types.Interface, seen map[string]bool) {
	for _, embedded := range functionTypeEmbeddedInterfaces(iface) {
		if seen[embedded.name] {
			continue
		}
		seen[embedded.name] = true
		writeFunctionTypeEmbeddedInterfaceImpls(out, funcTypeName, wrapperName, embedded.iface, seen)
		writeFunctionTypeInterfaceTraitImpl(out, funcTypeName, wrapperName, embedded.name, embedded.iface)
	}
}

func writeFunctionTypeInterfaceTraitImpl(out *strings.Builder, funcTypeName, wrapperName, ifaceName string, iface *types.Interface) {
	traitSnake := traitMethodSuffix(ifaceName)
	out.WriteString("impl ")
	out.WriteString(ifaceName)
	out.WriteString(" for ")
	out.WriteString(wrapperName)
	out.WriteString(" {\n")
	explicitMethodCount := 0
	if iface != nil {
		explicitMethodCount = iface.NumExplicitMethods()
	}
	for i := 0; i < explicitMethodCount; i++ {
		method := iface.ExplicitMethod(i)
		sig, ok := method.Type().(*types.Signature)
		if !ok {
			continue
		}
		out.WriteString("    fn ")
		out.WriteString(ToSnakeCase(method.Name()))
		out.WriteString("(")
		if interfaceTraitMethodRequiresMutableReceiver(ifaceName, method.Name(), method) {
			out.WriteString("&mut self")
		} else {
			out.WriteString("&self")
		}
		for j := 0; j < sig.Params().Len(); j++ {
			param := sig.Params().At(j)
			out.WriteString(", ")
			paramName := param.Name()
			if paramName == "" {
				paramName = fmt.Sprintf("__arg%d", j)
			}
			out.WriteString(EscapeRustIdent(paramName))
			out.WriteString(": ")
			out.WriteString(goTypesParamTypeToRust(param.Type()))
		}
		out.WriteString(")")
		if sig.Results().Len() > 0 {
			out.WriteString(" -> ")
			if sig.Results().Len() == 1 {
				out.WriteString(functionTypeInterfaceImplResultRust(sig.Results().At(0).Type()))
			} else {
				out.WriteString("(")
				for j := 0; j < sig.Results().Len(); j++ {
					if j > 0 {
						out.WriteString(", ")
					}
					out.WriteString(functionTypeInterfaceImplResultRust(sig.Results().At(j).Type()))
				}
				out.WriteString(")")
			}
		}
		out.WriteString(" {\n")
		out.WriteString("        ")
		out.WriteString(funcTypeName)
		out.WriteString("Methods::")
		out.WriteString(ToSnakeCase(method.Name()))
		out.WriteString("(&self.0")
		for j := 0; j < sig.Params().Len(); j++ {
			out.WriteString(", ")
			paramName := sig.Params().At(j).Name()
			if paramName == "" {
				paramName = fmt.Sprintf("__arg%d", j)
			}
			out.WriteString(EscapeRustIdent(paramName))
			out.WriteString(".clone()")
		}
		out.WriteString(")\n")
		out.WriteString("    }\n")
	}
	out.WriteString("    fn __go_clone_box_")
	out.WriteString(traitSnake)
	out.WriteString("(&self) -> ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString(" {\n")
	out.WriteString("        Box::new(self.clone())\n")
	out.WriteString("    }\n")
	if len(functionTypeEmbeddedInterfaces(iface)) == 0 {
		out.WriteString("    fn __go_as_any(&self) -> &dyn std::any::Any {\n")
		out.WriteString("        &self.0\n")
		out.WriteString("    }\n")
	}
	out.WriteString("    fn __go_eq_")
	out.WriteString(traitSnake)
	out.WriteString("(&self, _other: ")
	out.WriteString(rustLocalInterfaceParamBare(ifaceName))
	out.WriteString(") -> bool {\n")
	out.WriteString("        false\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func functionTypeInterfaceWrapperName(funcTypeName, ifaceName string) string {
	if !strings.Contains(ifaceName, "::") {
		return funcTypeName + "As" + ifaceName
	}
	parts := strings.Split(ifaceName, "::")
	for i, part := range parts {
		parts[i] = strings.TrimPrefix(part, "r#")
	}
	return funcTypeName + "As" + strings.Join(parts, "_")
}

func pointerLocalInterfaceWrapperName(typeName, ifaceName string) string {
	return strings.TrimPrefix(RustTypeNameForUse(typeName), "r#") + "Ptr"
}

func pointerLocalInterfaceWrapperNameForUse(typeName, ifaceName string) string {
	wrapperName := pointerLocalInterfaceWrapperName(typeName, ifaceName)
	ctx := GetTranspileContext()
	if ctx == nil || ctx.Package == nil || ctx.CurrentModuleName == "" {
		return wrapperName
	}
	moduleName := ctx.Package.TypeModuleNames[typeName]
	if moduleName == "" || moduleName == ctx.CurrentModuleName {
		return wrapperName
	}
	return "crate::" + moduleName + "::" + wrapperName
}

func writePointerLocalInterfaceWrapper(out *strings.Builder, typeName, ifaceName string, ifaceType *types.Interface, emitStruct bool) {
	trackWrapperImports()
	rustTypeName := RustTypeNameForUse(typeName)
	wrapperName := pointerLocalInterfaceWrapperName(typeName, ifaceName)
	pointerType := GetOuterWrapperType() + "<" + GetInnerWrapperType() + "<Option<" + rustTypeName + ">>>"

	if emitStruct {
		out.WriteString("#[derive(Clone)]\n")
		out.WriteString("pub struct ")
		out.WriteString(wrapperName)
		out.WriteString("(pub ")
		out.WriteString(pointerType)
		out.WriteString(");\n\n")

		out.WriteString("impl std::fmt::Display for ")
		out.WriteString(wrapperName)
		out.WriteString(" {\n")
		out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
		out.WriteString("        let __guard = self.0")
		WriteBorrowMethod(out, false)
		out.WriteString(";\n")
		out.WriteString("        match __guard.as_ref() { Some(__v) => write!(f, \"{:p}\", __v as *const _), None => write!(f, \"<nil>\") }\n")
		out.WriteString("    }\n")
		out.WriteString("}\n\n")
	}

	out.WriteString("impl ")
	out.WriteString(ifaceName)
	out.WriteString(" for ")
	out.WriteString(wrapperName)
	out.WriteString(" {\n")
	for _, method := range explicitInterfaceMethods(ifaceType) {
		writePointerLocalInterfaceForwardMethodFromTypes(out, method, rustTypeName, ifaceName)
	}
	writePointerLocalInterfaceSupportImpl(out, ifaceName, wrapperName, ifaceType)
	out.WriteString("}")
}

func writePointerLocalInterfaceForwardMethodFromTypes(out *strings.Builder, method *types.Func, receiverType, ifaceName string) {
	if method == nil {
		return
	}
	sig, ok := method.Type().(*types.Signature)
	if !ok {
		return
	}
	out.WriteString("    fn ")
	out.WriteString(ToSnakeCase(method.Name()))
	out.WriteString("(")
	mutableReceiver := interfaceTraitMethodRequiresMutableReceiver(ifaceName, method.Name(), method)
	if mutableReceiver {
		out.WriteString("&mut self")
	} else {
		out.WriteString("&self")
	}
	params := sig.Params()
	argNames := make([]string, 0, params.Len())
	for j := 0; j < params.Len(); j++ {
		param := params.At(j)
		paramName := param.Name()
		if paramName == "" {
			paramName = fmt.Sprintf("__arg%d", j)
		}
		paramName = RustLocalIdent(paramName)
		argNames = append(argNames, paramName)
		out.WriteString(", ")
		out.WriteString(paramName)
		out.WriteString(": ")
		out.WriteString(goTypesParamTypeToRust(param.Type()))
	}
	out.WriteString(")")
	writeInterfaceMethodResultTypesFromFunc(out, method)
	out.WriteString(" {\n")
	out.WriteString("        let ")
	if mutableReceiver {
		out.WriteString("mut ")
	}
	out.WriteString("__recv_guard = self.0")
	WriteBorrowMethod(out, mutableReceiver)
	out.WriteString(";\n")
	out.WriteString("        let __recv = __recv_guard.")
	if mutableReceiver {
		out.WriteString("as_mut")
	} else {
		out.WriteString("as_ref")
	}
	out.WriteString("().unwrap();\n")
	out.WriteString("        ")
	out.WriteString(receiverType)
	out.WriteString("::")
	out.WriteString(rustMethodNameForTypesFunc(method))
	out.WriteString("(__recv")
	for _, argName := range argNames {
		out.WriteString(", ")
		out.WriteString(argName)
	}
	out.WriteString(")\n")
	out.WriteString("    }\n")
}

func writePointerLocalInterfaceSupportImpl(out *strings.Builder, ifaceName, wrapperName string, ifaceType *types.Interface) {
	TrackImport("Any")
	traitSnake := traitMethodSuffix(ifaceName)
	hasEmbedded := ifaceType != nil && interfaceTypeHasNamedEmbedded(ifaceType)
	if ifaceType == nil {
		hasEmbedded = localInterfaceHasEmbeddedInterfaces(ifaceName)
	}
	out.WriteString("    fn __go_clone_box_")
	out.WriteString(traitSnake)
	out.WriteString("(&self) -> ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString(" {\n")
	out.WriteString("        Box::new(self.clone()) as ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString("\n")
	out.WriteString("    }\n")
	if !hasEmbedded {
		out.WriteString("    fn __go_as_any(&self) -> &dyn Any {\n")
		out.WriteString("        self\n")
		out.WriteString("    }\n")
	}
	out.WriteString("    fn __go_eq_")
	out.WriteString(traitSnake)
	out.WriteString("(&self, other: ")
	out.WriteString(rustLocalInterfaceParamBare(ifaceName))
	out.WriteString(") -> bool {\n")
	out.WriteString("        if let Some(__other) = other.__go_as_any().downcast_ref::<")
	out.WriteString(wrapperName)
	out.WriteString(">() {\n")
	if NeedsConcurrentWrapper() {
		out.WriteString("            Arc::ptr_eq(&self.0, &__other.0)\n")
	} else {
		out.WriteString("            Rc::ptr_eq(&self.0, &__other.0)\n")
	}
	out.WriteString("        } else {\n")
	out.WriteString("            false\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

func writeFunctionTypeImportedInterfaceImpls(out *strings.Builder, funcTypeName string, impls map[string]*types.Interface) {
	if len(impls) == 0 {
		return
	}
	var ifaceNames []string
	for ifaceName := range impls {
		ifaceNames = append(ifaceNames, ifaceName)
	}
	sort.Strings(ifaceNames)
	for _, ifaceName := range ifaceNames {
		writeFunctionTypeInterfaceImpl(out, funcTypeName, ifaceName, impls[ifaceName])
	}
}

func assignedInterfaceParamNames(fn *ast.FuncDecl) map[string]bool {
	assigned := make(map[string]bool)
	if fn == nil || fn.Body == nil || fn.Type.Params == nil {
		return assigned
	}
	for _, field := range fn.Type.Params.List {
		if _, ok := transpiledNamedInterfaceTypeNameFromExpr(field.Type); !ok {
			continue
		}
		for _, name := range field.Names {
			if blockIdentAssigned(fn.Body, name.Name) {
				assigned[name.Name] = true
			}
		}
	}
	return assigned
}

func writeAssignedInterfaceParamShadows(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	if fn == nil || fn.Body == nil || fn.Type.Params == nil {
		return
	}
	for _, field := range fn.Type.Params.List {
		interfaceName, ok := transpiledNamedInterfaceTypeNameFromExpr(field.Type)
		if !ok {
			continue
		}
		for _, name := range field.Names {
			if name.Name == "_" || !blockIdentAssigned(fn.Body, name.Name) {
				continue
			}
			out.WriteString(indent)
			out.WriteString("let mut ")
			out.WriteString(RustLocalIdent(name.Name))
			out.WriteString(": ")
			out.WriteString(GoTypeToRust(field.Type))
			out.WriteString(" = ")
			if NeedsConcurrentWrapper() {
				out.WriteString("Arc::new(Mutex::new(")
				out.WriteString(RustLocalIdent(name.Name))
				out.WriteString(".lock().unwrap().as_ref().map(|__v| ")
			} else {
				out.WriteString("Rc::new(RefCell::new(")
				out.WriteString(RustLocalIdent(name.Name))
				out.WriteString(".borrow().as_ref().map(|__v| ")
			}
			out.WriteString(interfaceName)
			out.WriteString("::__go_clone_box_")
			out.WriteString(traitMethodSuffix(interfaceName))
			out.WriteString("(__v.as_ref()))")
			out.WriteString("));\n")
		}
	}
}

func registerFunctionSignatureDecl(fn *ast.FuncDecl) {
	var params []*ast.Field
	if fn.Type.Params != nil {
		params = fn.Type.Params.List
	}
	var results []*ast.Field
	if fn.Type.Results != nil {
		results = fn.Type.Results.List
	}
	RegisterFunctionSignature(fn.Name.Name, &FunctionSignature{
		Params:  params,
		Results: results,
	})
}

func registerFunctionSignaturesFromFiles(files []*ast.File) {
	for _, file := range files {
		registerFunctionSignaturesFromFile(file)
	}
}

func registerFunctionSignaturesFromFile(file *ast.File) {
	for _, decl := range file.Decls {
		fn, ok := decl.(*ast.FuncDecl)
		if !ok || fn.Recv != nil {
			continue
		}
		registerFunctionSignatureDecl(fn)
	}
}

func registerPackageTypeFactsFromFiles(files []*ast.File) {
	for _, file := range files {
		registerPackageTypeFactsFromFile(file)
	}
	registerPackageStructDefsFromFiles(files)
}

func registerPackageTypeModuleNames(pkgState *PackageState, files []*ast.File, moduleNamesByIndex []string) {
	if pkgState == nil {
		return
	}
	if pkgState.TypeModuleNames == nil {
		pkgState.TypeModuleNames = make(map[string]string)
	}
	for i, file := range files {
		if file == nil || i >= len(moduleNamesByIndex) {
			continue
		}
		moduleName := moduleNamesByIndex[i]
		if moduleName == "" {
			continue
		}
		registerPackageTypeModuleNamesForFile(pkgState, file, moduleName)
	}
}

func registerPackageTypeModuleNamesForFile(pkgState *PackageState, file *ast.File, moduleName string) {
	if pkgState == nil || file == nil || moduleName == "" {
		return
	}
	if pkgState.TypeModuleNames == nil {
		pkgState.TypeModuleNames = make(map[string]string)
	}
	for _, decl := range file.Decls {
		genDecl, ok := decl.(*ast.GenDecl)
		if !ok || genDecl.Tok != token.TYPE {
			continue
		}
		for _, spec := range genDecl.Specs {
			typeSpec, ok := spec.(*ast.TypeSpec)
			if !ok || typeSpec.Name == nil {
				continue
			}
			pkgState.TypeModuleNames[typeSpec.Name.Name] = moduleName
		}
	}
}

func rustImplTypeNameForUse(typeName string) string {
	rustTypeName := RustTypeNameForUse(typeName)
	ctx := GetTranspileContext()
	if ctx == nil || ctx.Package == nil || ctx.CurrentModuleName == "" {
		return rustTypeName
	}
	moduleName := ctx.Package.TypeModuleNames[typeName]
	if moduleName == "" || moduleName == ctx.CurrentModuleName {
		return rustTypeName
	}
	return "crate::" + moduleName + "::" + rustTypeName
}

// registerPackageStructDefsFromFiles populates structDefs for every struct type
// declared across the package's files before any file is emitted. Per-file
// registration (in TranspileWithMapping) only sees types from the current file,
// so cross-file references like a *T field where T lives in another file would
// otherwise miss structDefs and bypass field-driven decisions such as Debug
// derivation.
func registerPackageStructDefsFromFiles(files []*ast.File) {
	for _, file := range files {
		for _, decl := range file.Decls {
			genDecl, ok := decl.(*ast.GenDecl)
			if !ok || genDecl.Tok != token.TYPE {
				continue
			}
			for _, spec := range genDecl.Specs {
				typeSpec, ok := spec.(*ast.TypeSpec)
				if !ok {
					continue
				}
				if structType, ok := typeSpec.Type.(*ast.StructType); ok {
					registerStructTypeSpecDef(typeSpec, structType)
				}
			}
		}
	}
}

func registerPackageTypeFactsFromFile(file *ast.File) {
	for _, decl := range file.Decls {
		genDecl, ok := decl.(*ast.GenDecl)
		if !ok || genDecl.Tok != token.TYPE {
			continue
		}
		for _, spec := range genDecl.Specs {
			typeSpec, ok := spec.(*ast.TypeSpec)
			if !ok {
				continue
			}
			registerPackageTypeFact(typeSpec)
		}
	}
}

func registerPackageTypeFact(typeSpec *ast.TypeSpec) {
	if typeSpec == nil {
		return
	}
	isFunctionType := typeSpecHasFunctionSignature(typeSpec)
	if typeSpec.Assign != 0 || isFunctionType {
		RegisterTypeAlias(typeSpec.Name.Name)
		if isFunctionType {
			RegisterFunctionTypeAlias(typeSpec.Name.Name)
			if funcType, ok := typeSpec.Type.(*ast.FuncType); ok {
				RegisterFunctionTypeAliasBox(typeSpec.Name.Name, functionTypeSpecRustBoxType(typeSpec, funcType))
			}
		}
		return
	}

	_, isStruct := typeSpec.Type.(*ast.StructType)
	if _, isInterface := typeSpec.Type.(*ast.InterfaceType); isInterface {
		RegisterInterfaceType(typeSpec.Name.Name)
		return
	}
	if !isStruct {
		registerTypeDefinitionForTypeExpr(typeSpec.Name.Name, typeSpec.Type)
	}
}

func registerTypeDefinitionForTypeExpr(name string, expr ast.Expr) {
	if name == "" || expr == nil {
		return
	}
	RegisterTypeDefinition(name, typeDefinitionUnderlyingName(expr))
	if typ, ok := typeInfoTypeForTypeExpr(expr); ok {
		RegisterTypeDefinitionUnderlyingType(name, typ)
	}
}

func typeSpecHasFunctionSignature(typeSpec *ast.TypeSpec) bool {
	if typeSpec == nil {
		return false
	}
	if _, ok := typeSpec.Type.(*ast.FuncType); ok {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	if obj, ok := typeInfo.info.Defs[typeSpec.Name].(*types.TypeName); ok {
		if _, ok := signatureFromType(obj.Type()); ok {
			return true
		}
	}
	if typ := typeInfo.GetType(typeSpec.Type); typ != nil {
		if _, ok := signatureFromType(typ); ok {
			return true
		}
	}
	return false
}

func TranspileFunction(out *strings.Builder, fn *ast.FuncDecl, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	// Check if this is a method (has receiver)
	if fn.Recv != nil && len(fn.Recv.List) > 0 {
		// Methods will be collected and generated in impl blocks
		// For now, skip them here
		return
	}

	// Register the function signature for later use
	registerFunctionSignatureDecl(fn)

	// Regular function
	if fn.Name.Name != "main" && fn.Name.Name != "init" {
		out.WriteString("pub ")
	}
	out.WriteString("fn ")
	out.WriteString(rustFunctionName(fn))
	writeFunctionDeclTypeParams(out, fn)
	out.WriteString("(")

	writeFuncDeclParams(out, fn)

	out.WriteString(")")

	writeFuncDeclResultTypes(out, fn)

	out.WriteString(" {\n")

	restoreLocalSyntaxInfo := pushFunctionLocalSyntaxInfo()
	defer restoreLocalSyntaxInfo()
	assignedInterfaceParams := assignedInterfaceParamNames(fn)

	// Register function parameters in VarTable
	if vt := GetVarTable(); vt != nil {
		vt.PushScope()
		defer vt.PopScope()
		if fn.Type.Params != nil {
			paramIndex := 0
			for _, field := range fn.Type.Params.List {
				for _, name := range field.Names {
					rustType := goTypeToRustBase(field.Type)
					if elemRustType, ok := sliceElemPtrSliceParamInfoForDeclObject(fn, paramIndex); ok {
						rustType = "Vec<GoPtr<" + elemRustType + ">>"
					}
					if elemRustType, ok := goPtrSlotParamDeclElemRustType(fn, paramIndex); ok {
						rustType = goPtrSlotParamRustType(elemRustType)
					}
					if elemRustType, ok := goPtrParamDeclElemRustType(fn, paramIndex); ok {
						rustType = "GoPtr<" + elemRustType + ">"
					}
					if functionRustType, ok := functionValueGoPtrAwareBoxTypeForNamedTypeExpr(name, field.Type); ok {
						rustType = functionRustType
					} else if functionRustType, ok := functionTypeRustNameFromTypeExpr(field.Type); ok {
						rustType = functionRustType
					}
					registerTypeExprCollectionInfo(name.Name, field.Type)
					if elemRustType, ok := goPtrSlotParamDeclElemRustType(fn, paramIndex); ok {
						var goType types.Type
						if typeInfo := GetTypeInfo(); typeInfo != nil {
							goType = typeInfo.GetType(field.Type)
						}
						vt.Register(name.Name, &VarInfo{
							WrapLevel:   WrapNone,
							RustType:    goPtrSlotParamRustType(elemRustType),
							Source:      SourceParam,
							PointerKind: PointerGoPtr,
							GoType:      goType,
						})
					} else if elemRustType, ok := goPtrParamDeclElemRustType(fn, paramIndex); ok {
						var goType types.Type
						if typeInfo := GetTypeInfo(); typeInfo != nil {
							goType = typeInfo.GetType(field.Type)
						}
						vt.Register(name.Name, &VarInfo{
							WrapLevel:   WrapNone,
							RustType:    "GoPtr<" + elemRustType + ">",
							Source:      SourceParam,
							PointerKind: PointerGoPtr,
							GoType:      goType,
						})
					} else if varInfo, ok := interfaceParamVarInfo(field.Type); ok {
						if assignedInterfaceParams[name.Name] {
							vt.Register(name.Name, &VarInfo{
								WrapLevel: WrapFull,
								RustType:  rustType,
								Source:    SourceLocal,
							})
						} else {
							varInfo.RustType = rustType
							vt.Register(name.Name, varInfo)
						}
					} else if _, ok := field.Type.(*ast.ChanType); ok {
						// Channel parameters are bare (GoChannel<T>)
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  rustType,
							Source:    SourceParam,
						})
					} else if isSyncParam(field.Type) {
						// sync.WaitGroup / sync.Mutex parameters are bare
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  rustType,
							Source:    SourceParam,
						})
					} else if typeExprIsRegisteredBareStructAlias(field.Type) {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  rustType,
							Source:    SourceParam,
						})
					} else if typeExprIsOrderedTypeParam(field.Type) {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  rustType,
							Source:    SourceParam,
						})
					} else {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapFull,
							RustType:  rustType,
							Source:    SourceParam,
						})
					}
					paramIndex++
				}
			}
		}
	}

	// Call package initialization at the start of main() if present
	if fn.Name.Name == "main" && hasInitFunction {
		out.WriteString("    __go_init_all();\n")
	}

	writeAssignedInterfaceParamShadows(out, fn, "    ")

	if fn.Body != nil && writeRuntimeLinkedFunctionBody(out, fn, "    ") {
		out.WriteString("}\n")
		return
	}

	if fn.Body == nil {
		if writeRuntimeLinkedFunctionBody(out, fn, "    ") {
			out.WriteString("}\n")
			return
		}
		out.WriteString("    unimplemented!(\"Go function declaration has no body\");\n")
		out.WriteString("}\n")
		return
	}

	restoreSliceElemPtrCandidates := setSliceElemPtrCandidatesForFunc(fn)
	defer restoreSliceElemPtrCandidates()
	restoreFuncLitGoPtrParamInfos := setFuncLitGoPtrParamInfosForFunc(fn)
	defer restoreFuncLitGoPtrParamInfos()
	restoreSliceElemPtrReturn := pushCurrentSliceElemPtrReturn(fn)
	defer restoreSliceElemPtrReturn()
	registerArrayElemPtrNamedReturnVars(fn)

	// Check if this function uses defer statements
	hasDefer := checkHasDefer(fn.Body.List)
	currentFunctionHasDefer = hasDefer
	oldActiveMutexGuards := activeMutexGuards
	activeMutexGuards = make(map[string]string)
	defer func() { activeMutexGuards = oldActiveMutexGuards }()

	oldFunctionBodyLbrace := currentFunctionBodyLbrace
	if fn.Body != nil {
		currentFunctionBodyLbrace = fn.Body.Lbrace
	} else {
		currentFunctionBodyLbrace = token.NoPos
	}
	defer func() { currentFunctionBodyLbrace = oldFunctionBodyLbrace }()

	// Initialize defer stack if needed
	if hasDefer {
		out.WriteString("    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();\n")
		// We'll execute defers before each return statement
		out.WriteString("\n")
	}

	// Declare named return values as mutable variables
	hasNamedReturns := false
	if fn.Type.Results != nil {
		for _, result := range fn.Type.Results.List {
			if len(result.Names) > 0 {
				hasNamedReturns = true
				for _, name := range result.Names {
					if name.Name == "_" {
						out.WriteString("    let ")
					} else {
						out.WriteString("    let mut ")
					}
					out.WriteString(RustLocalIdent(name.Name))
					out.WriteString(": ")
					if info, ok := arrayElemPtrCandidateForDecl(name); ok {
						out.WriteString(arrayElemPtrOptionRustType(info))
						out.WriteString(" = None;\n")
						continue
					}
					if info, ok := goPtrCandidateForDecl(name); ok {
						elemRustType := goPtrResultElemRustType(info)
						registerGoPtrVar(name.Name, elemRustType, nil)
						out.WriteString("GoPtr<")
						out.WriteString(elemRustType)
						out.WriteString("> = GoPtr::nil();\n")
						continue
					}
					out.WriteString(GoTypeToRust(result.Type))
					// Initialize with wrapped default values
					out.WriteString(" = ")

					// Special handling for error type
					if t, ok := result.Type.(*ast.Ident); ok && t.Name == "error" {
						// error type is wrapped as Rc<RefCell<Option<Box<dyn Error>>>>
						// We need to write the wrapper manually without the Some()
						if NeedsConcurrentWrapper() {
							TrackImport("Arc")
							TrackImport("Mutex")
							out.WriteString("Arc::new(Mutex::new(None))")
						} else {
							TrackImport("Rc")
							TrackImport("RefCell")
							out.WriteString("Rc::new(RefCell::new(None))")
						}
						out.WriteString(";\n")
						continue
					}

					if _, ok := functionSignatureFromTypeExpr(result.Type); ok {
						WriteWrappedNone(out)
						out.WriteString(";\n")
						continue
					}

					if isEmptyInterfaceExpr(result.Type) {
						WriteWrappedNone(out)
						out.WriteString(";\n")
						continue
					}

					if _, ok := transpiledNamedInterfaceTypeNameFromExpr(result.Type); ok {
						WriteWrappedNone(out)
						out.WriteString(";\n")
						continue
					}

					if writeDirectTypeParamWrappedZeroValue(out, result.Type, "named return zero value") {
						out.WriteString(";\n")
						continue
					}

					if writeNamedReturnNilZeroValueFromTypeInfo(out, result.Type) {
						out.WriteString(";\n")
						continue
					}

					// For all other types
					WriteWrapperPrefix(out)
					switch t := result.Type.(type) {
					case *ast.Ident:
						switch t.Name {
						case "string":
							out.WriteString("String::new()")
						case "int", "int64", "int32", "int16", "int8":
							out.WriteString("0")
						case "uint", "uint64", "uint32", "uint16", "uint8":
							out.WriteString("0")
						case "float64", "float32":
							out.WriteString("0.0")
						case "bool":
							out.WriteString("false")
						default:
							out.WriteString("Default::default()")
						}
					default:
						out.WriteString("Default::default()")
					}
					out.WriteString(")))")
					out.WriteString(";\n")
				}
			}
		}
		// Separator after the named-return declarations, but only when we
		// actually emitted some. Unnamed returns leave the block empty, so
		// emitting a newline here would leave a blank line at the top of
		// every function body for no reason.
		if hasNamedReturns {
			out.WriteString("\n")
		}
	}

	recoverCatch := shouldWrapDeferPanicRecover(hasDefer)
	bodyIndent := "    "
	if recoverCatch {
		bodyIndent = beginPanicRecoverCatch(out, "    ")
	}

	// Function body
	var prevStmt ast.Stmt
	var lastPos token.Pos = fn.Body.Lbrace
	if functionHasGoto(fn) {
		prevStmt = TranspileGotoStatementList(out, fn.Body.List, fn.Type, fileSet, comments, &lastPos, bodyIndent)
		if lastStmt := lastNonEmptyStmt(fn.Body.List); lastStmt != nil && stmtTerminates(lastStmt) {
			out.WriteString(bodyIndent)
			out.WriteString("unreachable!()\n")
		}
	} else {
		for i, stmt := range fn.Body.List {
			// Add blank line if there was one in the source
			if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
				out.WriteString("\n")
			}

			out.WriteString(bodyIndent)
			if i == len(fn.Body.List)-1 {
				TranspileTailStatement(out, stmt, fn.Type, fileSet, comments, &lastPos, bodyIndent)
			} else {
				TranspileStatement(out, stmt, fn.Type, fileSet, comments, &lastPos, bodyIndent)
			}
			out.WriteString("\n")

			prevStmt = stmt
		}
	}

	// Execute defers at the end if needed.
	// Skip if the last statement already terminates; those paths emitted their own
	// cleanup and return, so a trailing defer block would become the function's
	// final Rust expression.
	lastStmt := lastNonEmptyStmt(fn.Body.List)
	lastTerminates := lastStmt != nil && stmtTerminates(lastStmt)
	if hasDefer && !lastTerminates {
		out.WriteString("\n")
		out.WriteString(bodyIndent)
		out.WriteString("// Execute deferred functions\n")
		out.WriteString(bodyIndent)
		out.WriteString("while let Some(f) = __defer_stack.pop() {\n")
		out.WriteString(bodyIndent)
		out.WriteString("    f();\n")
		out.WriteString(bodyIndent)
		out.WriteString("}\n")
	}

	if recoverCatch {
		finishPanicRecoverCatch(out, fn, "    ")
	}

	out.WriteString("}")
}

func writeRuntimeLinkedFunctionBody(out *strings.Builder, fn *ast.FuncDecl, indent string) bool {
	if fn == nil || fn.Name == nil {
		return false
	}
	switch functionPackagePath(fn) {
	case "sync":
		switch fn.Name.Name {
		case "runtime_registerPoolCleanup":
			out.WriteString(indent)
			out.WriteString("let _ = ")
			out.WriteString(RustLocalIdent(functionParamName(fn, 0, "cleanup")))
			out.WriteString(";\n")
			return true
		case "runtime_procPin":
			out.WriteString(indent)
			out.WriteString("0\n")
			return true
		case "runtime_procUnpin":
			return true
		case "runtime_LoadAcquintptr":
			ptrName := RustLocalIdent(functionParamName(fn, 0, "ptr"))
			out.WriteString(indent)
			out.WriteString("let __value = (*")
			out.WriteString(ptrName)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone();\n")
			out.WriteString(indent)
			out.WriteString("__value\n")
			return true
		case "runtime_StoreReluintptr":
			ptrName := RustLocalIdent(functionParamName(fn, 0, "ptr"))
			valName := RustLocalIdent(functionParamName(fn, 1, "val"))
			out.WriteString(indent)
			out.WriteString("let __stored = (*")
			out.WriteString(valName)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone();\n")
			out.WriteString(indent)
			out.WriteString("*")
			out.WriteString(ptrName)
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap() = __stored;\n")
			out.WriteString(indent)
			out.WriteString("__stored\n")
			return true
		case "runtime_randn":
			out.WriteString(indent)
			out.WriteString("let _ = ")
			out.WriteString(RustLocalIdent(functionParamName(fn, 0, "n")))
			out.WriteString(";\n")
			out.WriteString(indent)
			out.WriteString("0\n")
			return true
		case "runtime_Semacquire", "runtime_SemacquireWaitGroup", "runtime_SemacquireRWMutexR", "runtime_SemacquireRWMutex":
			writeSyncRuntimeSemacquireBody(out, fn, indent)
			return true
		case "runtime_Semrelease":
			writeSyncRuntimeSemreleaseBody(out, fn, indent)
			return true
		case "runtime_notifyListAdd":
			out.WriteString(indent)
			out.WriteString("let _ = ")
			out.WriteString(RustLocalIdent(functionParamName(fn, 0, "l")))
			out.WriteString(";\n")
			out.WriteString(indent)
			out.WriteString("0\n")
			return true
		case "runtime_notifyListWait", "runtime_notifyListNotifyAll", "runtime_notifyListNotifyOne":
			out.WriteString(indent)
			out.WriteString("let _ = ")
			out.WriteString(RustLocalIdent(functionParamName(fn, 0, "l")))
			out.WriteString(";\n")
			if fn.Name.Name == "runtime_notifyListWait" {
				out.WriteString(indent)
				out.WriteString("let _ = ")
				out.WriteString(RustLocalIdent(functionParamName(fn, 1, "t")))
				out.WriteString(";\n")
			}
			return true
		case "runtime_notifyListCheck":
			out.WriteString(indent)
			out.WriteString("let _ = ")
			out.WriteString(RustLocalIdent(functionParamName(fn, 0, "size")))
			out.WriteString(";\n")
			return true
		case "throw", "fatal":
			paramName := RustLocalIdent(functionParamName(fn, 0, "__arg0"))
			out.WriteString(indent)
			out.WriteString("let __message = { let __arg_holder = ")
			out.WriteString(paramName)
			out.WriteString(".clone(); let __arg_guard = __arg_holder")
			WriteBorrowMethod(out, false)
			out.WriteString("; (*__arg_guard.as_ref().unwrap()).clone() };\n")
			out.WriteString(indent)
			out.WriteString("panic!(\"{}\", __message);\n")
			return true
		}
	case "sync/atomic":
		if writeSyncAtomicRuntimeLinkedBody(out, fn, indent) {
			return true
		}
	case "syscall":
		switch fn.Name.Name {
		case "runtime_envs":
			out.WriteString(indent)
			out.WriteString("let __envs: Vec<String> = std::env::vars().map(|(__k, __v)| format!(\"{}={}\", __k, __v)).collect();\n")
			out.WriteString(indent)
			WriteWrapperPrefix(out)
			out.WriteString("__envs")
			WriteWrapperSuffix(out)
			out.WriteString("\n")
			return true
		case "runtimeSetenv":
			writeRuntimeLinkedStringParamClone(out, "k", "__key", indent)
			writeRuntimeLinkedStringParamClone(out, "v", "__value", indent)
			out.WriteString(indent)
			out.WriteString("std::env::set_var(__key, __value);\n")
			return true
		case "runtimeUnsetenv":
			writeRuntimeLinkedStringParamClone(out, "k", "__key", indent)
			out.WriteString(indent)
			out.WriteString("std::env::remove_var(__key);\n")
			return true
		}
	case "internal/abi":
		switch fn.Name.Name {
		case "FuncPCABI0", "FuncPCABIInternal":
			writeFuncPCIntrinsicBody(out, fn, indent)
			return true
		case "TypeOf":
			writeInternalABITypeOfIntrinsicBody(out, fn, indent)
			return true
		}
	case "internal/sync":
		switch fn.Name.Name {
		case "runtime_rand":
			out.WriteString(indent)
			out.WriteString("1u64\n")
			return true
		}
	case "internal/bytealg":
		switch fn.Name.Name {
		case "Count":
			writeInternalBytealgCountIntrinsicBody(out, fn, indent, false)
			return true
		case "CountString":
			writeInternalBytealgCountIntrinsicBody(out, fn, indent, true)
			return true
		case "MakeNoZero":
			writeInternalBytealgMakeNoZeroBody(out, fn, indent)
			return true
		case "IndexByteString":
			writeInternalBytealgIndexByteStringBody(out, fn, indent)
			return true
		case "IndexString":
			writeInternalBytealgIndexStringBody(out, fn, indent)
			return true
		case "abigen_runtime_cmpstring":
			writeInternalBytealgCompareStringBody(out, fn, indent)
			return true
		}
	case "internal/buildcfg":
		switch fn.Name.Name {
		case "expList":
			writeInternalBuildcfgExpListIntrinsicBody(out, fn, indent)
			return true
		}
	case "internal/godebug":
		switch fn.Name.Name {
		case "setUpdate":
			writeInternalGodebugSetUpdateBody(out, fn, indent)
			return true
		case "setNewIncNonDefault":
			writeInternalGodebugSetNewIncNonDefaultBody(out, fn, indent)
			return true
		case "registerMetric":
			writeInternalGodebugRegisterMetricBody(out, fn, indent)
			return true
		case "write":
			writeInternalGodebugWriteBody(out, fn, indent)
			return true
		}
	case "internal/syscall/unix":
		switch fn.Name.Name {
		case "ARC4Random":
			writeInternalSyscallUnixARC4RandomBody(out, fn, indent)
			return true
		}
	case "crypto/internal/boring/sig":
		switch fn.Name.Name {
		case "BoringCrypto", "FIPSOnly", "StandardCrypto":
			// The Go stdlib implements these signature markers in assembly as no-ops.
			return true
		}
	case "strings":
		switch fn.Name.Name {
		case "copyCheck":
			if runtimeLinkedReceiverIsNamed(fn, "Builder") {
				writeStringsBuilderCopyCheckBody(out, indent)
				return true
			}
		case "String":
			if runtimeLinkedReceiverIsNamed(fn, "Builder") {
				writeStringsBuilderStringBody(out, indent)
				return true
			}
		}
	}
	return false
}

func writeSyncRuntimeSemacquireBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	semName := RustLocalIdent(functionParamName(fn, 0, "s"))
	out.WriteString(indent)
	out.WriteString("loop {\n")
	out.WriteString(indent)
	out.WriteString("    let __acquired = {\n")
	out.WriteString(indent)
	out.WriteString("        let mut __sem_guard = ")
	out.WriteString(semName)
	WriteBorrowMethod(out, true)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("        let __sem = __sem_guard.as_mut().unwrap();\n")
	out.WriteString(indent)
	out.WriteString("        if *__sem > 0 {\n")
	out.WriteString(indent)
	out.WriteString("            *__sem -= 1;\n")
	out.WriteString(indent)
	out.WriteString("            true\n")
	out.WriteString(indent)
	out.WriteString("        } else {\n")
	out.WriteString(indent)
	out.WriteString("            false\n")
	out.WriteString(indent)
	out.WriteString("        }\n")
	out.WriteString(indent)
	out.WriteString("    };\n")
	out.WriteString(indent)
	out.WriteString("    if __acquired {\n")
	out.WriteString(indent)
	out.WriteString("        break;\n")
	out.WriteString(indent)
	out.WriteString("    }\n")
	out.WriteString(indent)
	out.WriteString("    std::thread::yield_now();\n")
	out.WriteString(indent)
	out.WriteString("}\n")
}

func writeSyncRuntimeSemreleaseBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	semName := RustLocalIdent(functionParamName(fn, 0, "s"))
	out.WriteString(indent)
	out.WriteString("{\n")
	out.WriteString(indent)
	out.WriteString("    let mut __sem_guard = ")
	out.WriteString(semName)
	WriteBorrowMethod(out, true)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("    let __sem = __sem_guard.as_mut().unwrap();\n")
	out.WriteString(indent)
	out.WriteString("    *__sem = __sem.saturating_add(1);\n")
	out.WriteString(indent)
	out.WriteString("}\n")
	for i, fallback := range []string{"handoff", "skipframes"} {
		if fn.Type == nil || fn.Type.Params == nil || len(fn.Type.Params.List) <= i+1 {
			continue
		}
		out.WriteString(indent)
		out.WriteString("let _ = ")
		out.WriteString(RustLocalIdent(functionParamName(fn, i+1, fallback)))
		out.WriteString(";\n")
	}
}

func writeInternalSyscallUnixARC4RandomBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	paramName := functionParamName(fn, 0, "p")
	out.WriteString(indent)
	out.WriteString("let __arc4_holder = ")
	out.WriteString(RustLocalIdent(paramName))
	out.WriteString(".clone();\n")
	out.WriteString(indent)
	out.WriteString("let mut __arc4_guard = __arc4_holder")
	WriteBorrowMethod(out, true)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("if let Some(__arc4_bytes) = __arc4_guard.as_mut() {\n")
	out.WriteString(indent)
	out.WriteString("    if !__arc4_bytes.is_empty() {\n")
	out.WriteString(indent)
	out.WriteString("        let mut __arc4_file = std::fs::File::open(\"/dev/urandom\").expect(\"internal/syscall/unix.ARC4Random failed to open /dev/urandom\");\n")
	out.WriteString(indent)
	out.WriteString("        std::io::Read::read_exact(&mut __arc4_file, __arc4_bytes.as_mut_slice()).expect(\"internal/syscall/unix.ARC4Random failed to read /dev/urandom\");\n")
	out.WriteString(indent)
	out.WriteString("    }\n")
	out.WriteString(indent)
	out.WriteString("}\n")
}

func writeSyncAtomicRuntimeLinkedBody(out *strings.Builder, fn *ast.FuncDecl, indent string) bool {
	if fn == nil || fn.Name == nil {
		return false
	}
	if writeSyncAtomicPointerMethodBody(out, fn, indent) {
		return true
	}
	switch fn.Name.Name {
	case "LoadInt32", "LoadInt64", "LoadUint32", "LoadUint64", "LoadUintptr":
		writeSyncAtomicLoadScalarBody(out, fn, indent)
		return true
	case "StoreInt32", "StoreInt64", "StoreUint32", "StoreUint64", "StoreUintptr":
		writeSyncAtomicStoreScalarBody(out, fn, indent)
		return true
	case "SwapInt32", "SwapInt64", "SwapUint32", "SwapUint64", "SwapUintptr":
		writeSyncAtomicSwapScalarBody(out, fn, indent)
		return true
	case "CompareAndSwapInt32", "CompareAndSwapInt64", "CompareAndSwapUint32", "CompareAndSwapUint64", "CompareAndSwapUintptr":
		writeSyncAtomicCompareAndSwapScalarBody(out, fn, indent)
		return true
	case "AddInt32", "AddInt64", "AddUint32", "AddUint64", "AddUintptr":
		writeSyncAtomicAddScalarBody(out, fn, indent)
		return true
	case "AndInt32", "AndInt64", "AndUint32", "AndUint64", "AndUintptr":
		writeSyncAtomicBitwiseScalarBody(out, fn, indent, "&=")
		return true
	case "OrInt32", "OrInt64", "OrUint32", "OrUint64", "OrUintptr":
		writeSyncAtomicBitwiseScalarBody(out, fn, indent, "|=")
		return true
	case "LoadPointer":
		writeSyncAtomicLoadPointerBody(out, fn, indent)
		return true
	case "StorePointer":
		writeSyncAtomicStorePointerBody(out, fn, indent)
		return true
	case "SwapPointer":
		writeSyncAtomicSwapPointerBody(out, fn, indent)
		return true
	case "CompareAndSwapPointer":
		writeSyncAtomicCompareAndSwapPointerBody(out, fn, indent)
		return true
	default:
		return false
	}
}

func writeSyncAtomicPointerMethodBody(out *strings.Builder, fn *ast.FuncDecl, indent string) bool {
	if !runtimeLinkedReceiverIsNamed(fn, "Pointer") {
		return false
	}
	switch fn.Name.Name {
	case "Load":
		writeSyncAtomicPointerLoadMethodBody(out, indent)
		return true
	case "Store":
		valName := RustLocalIdent(functionParamName(fn, 0, "val"))
		writeSyncAtomicPointerStoredOption(out, valName, "__stored", indent)
		out.WriteString(indent)
		out.WriteString("*self.v")
		WriteBorrowMethod(out, true)
		out.WriteString(" = __stored;\n")
		return true
	case "Swap":
		newName := RustLocalIdent(functionParamName(fn, 0, "new"))
		writeSyncAtomicPointerStoredOption(out, newName, "__stored", indent)
		out.WriteString(indent)
		out.WriteString("let mut __guard = self.v")
		WriteBorrowMethod(out, true)
		out.WriteString(";\n")
		out.WriteString(indent)
		out.WriteString("let __old = __guard.as_ref().cloned().unwrap_or_else(|| ")
		out.WriteString("GoPtr::nil());\n")
		out.WriteString(indent)
		out.WriteString("*__guard = __stored;\n")
		out.WriteString(indent)
		out.WriteString("__old\n")
		return true
	case "CompareAndSwap":
		oldName := RustLocalIdent(functionParamName(fn, 0, "old"))
		newName := RustLocalIdent(functionParamName(fn, 1, "new"))
		writeSyncAtomicPointerCompareAndSwapMethodBody(out, oldName, newName, indent)
		return true
	default:
		return false
	}
}

func writeSyncAtomicPointerLoadMethodBody(out *strings.Builder, indent string) {
	out.WriteString(indent)
	out.WriteString("let __guard = self.v")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("__guard.as_ref().cloned().unwrap_or_else(|| GoPtr::nil())\n")
}

func writeSyncAtomicPointerStoredOption(out *strings.Builder, paramName string, localName string, indent string) {
	out.WriteString(indent)
	out.WriteString("let ")
	out.WriteString(localName)
	out.WriteString(" = if ")
	out.WriteString(paramName)
	out.WriteString(".is_nil() { None } else { Some(")
	out.WriteString(paramName)
	out.WriteString(".clone()) };\n")
}

func writeSyncAtomicPointerCompareAndSwapMethodBody(out *strings.Builder, oldName string, newName string, indent string) {
	writeSyncAtomicPointerStoredOption(out, newName, "__new_value", indent)
	out.WriteString(indent)
	out.WriteString("let __old_is_nil = ")
	out.WriteString(oldName)
	out.WriteString(".is_nil();\n")
	out.WriteString(indent)
	out.WriteString("let mut __guard = self.v")
	WriteBorrowMethod(out, true)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("let __matches = match __guard.as_ref() {\n")
	out.WriteString(indent)
	out.WriteString("    Some(__current) => {\n")
	out.WriteString(indent)
	out.WriteString("        if __old_is_nil {\n")
	out.WriteString(indent)
	out.WriteString("            __current.is_nil()\n")
	out.WriteString(indent)
	out.WriteString("        } else {\n")
	out.WriteString(indent)
	out.WriteString("            GoPtr::ptr_eq(__current, &")
	out.WriteString(oldName)
	out.WriteString(")\n")
	out.WriteString(indent)
	out.WriteString("        }\n")
	out.WriteString(indent)
	out.WriteString("    }\n")
	out.WriteString(indent)
	out.WriteString("    None => __old_is_nil,\n")
	out.WriteString(indent)
	out.WriteString("};\n")
	out.WriteString(indent)
	out.WriteString("if __matches {\n")
	out.WriteString(indent)
	out.WriteString("    *__guard = __new_value;\n")
	out.WriteString(indent)
	out.WriteString("    true\n")
	out.WriteString(indent)
	out.WriteString("} else {\n")
	out.WriteString(indent)
	out.WriteString("    false\n")
	out.WriteString(indent)
	out.WriteString("}\n")
}

func writeSyncAtomicLoadScalarBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	addrName := RustLocalIdent(functionParamName(fn, 0, "addr"))
	out.WriteString(indent)
	out.WriteString("(*")
	out.WriteString(addrName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()\n")
}

func writeSyncAtomicStoreScalarBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	addrName := RustLocalIdent(functionParamName(fn, 0, "addr"))
	valueName := functionParamName(fn, 1, "val")
	writeRuntimeLinkedParamClone(out, valueName, "__val", indent)
	out.WriteString(indent)
	out.WriteString("*")
	out.WriteString(addrName)
	WriteBorrowMethod(out, true)
	out.WriteString(".as_mut().unwrap() = __val;\n")
}

func writeSyncAtomicSwapScalarBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	addrName := RustLocalIdent(functionParamName(fn, 0, "addr"))
	newName := functionParamName(fn, 1, "new")
	writeRuntimeLinkedParamClone(out, newName, "__new", indent)
	writeSyncAtomicMutableGuard(out, addrName, indent)
	out.WriteString(indent)
	out.WriteString("let __old = (*__guard.as_ref().unwrap()).clone();\n")
	out.WriteString(indent)
	out.WriteString("*__guard.as_mut().unwrap() = __new;\n")
	out.WriteString(indent)
	out.WriteString("__old\n")
}

func writeSyncAtomicCompareAndSwapScalarBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	addrName := RustLocalIdent(functionParamName(fn, 0, "addr"))
	oldName := functionParamName(fn, 1, "old")
	newName := functionParamName(fn, 2, "new")
	writeRuntimeLinkedParamClone(out, oldName, "__old", indent)
	writeRuntimeLinkedParamClone(out, newName, "__new", indent)
	writeSyncAtomicMutableGuard(out, addrName, indent)
	out.WriteString(indent)
	out.WriteString("if *__guard.as_ref().unwrap() == __old {\n")
	out.WriteString(indent)
	out.WriteString("    *__guard.as_mut().unwrap() = __new;\n")
	out.WriteString(indent)
	out.WriteString("    true\n")
	out.WriteString(indent)
	out.WriteString("} else {\n")
	out.WriteString(indent)
	out.WriteString("    false\n")
	out.WriteString(indent)
	out.WriteString("}\n")
}

func writeSyncAtomicAddScalarBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	addrName := RustLocalIdent(functionParamName(fn, 0, "addr"))
	deltaName := functionParamName(fn, 1, "delta")
	writeRuntimeLinkedParamClone(out, deltaName, "__delta", indent)
	writeSyncAtomicMutableGuard(out, addrName, indent)
	out.WriteString(indent)
	out.WriteString("let __current = (*__guard.as_ref().unwrap()).clone();\n")
	out.WriteString(indent)
	out.WriteString("let __new = __current.wrapping_add(__delta);\n")
	out.WriteString(indent)
	out.WriteString("*__guard.as_mut().unwrap() = __new;\n")
	out.WriteString(indent)
	out.WriteString("__new\n")
}

func writeSyncAtomicBitwiseScalarBody(out *strings.Builder, fn *ast.FuncDecl, indent string, op string) {
	addrName := RustLocalIdent(functionParamName(fn, 0, "addr"))
	maskName := functionParamName(fn, 1, "mask")
	writeRuntimeLinkedParamClone(out, maskName, "__mask", indent)
	writeSyncAtomicMutableGuard(out, addrName, indent)
	out.WriteString(indent)
	out.WriteString("let __old = (*__guard.as_ref().unwrap()).clone();\n")
	out.WriteString(indent)
	out.WriteString("*__guard.as_mut().unwrap() ")
	out.WriteString(op)
	out.WriteString(" __mask;\n")
	out.WriteString(indent)
	out.WriteString("__old\n")
}

func writeSyncAtomicLoadPointerBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	addrName := RustLocalIdent(functionParamName(fn, 0, "addr"))
	out.WriteString(indent)
	out.WriteString("let __value = ")
	out.WriteString(addrName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().copied().unwrap_or(0);\n")
	out.WriteString(indent)
	WriteWrapperPrefix(out)
	out.WriteString("__value")
	WriteWrapperSuffix(out)
	out.WriteString("\n")
}

func writeSyncAtomicStorePointerBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	addrName := RustLocalIdent(functionParamName(fn, 0, "addr"))
	valName := functionParamName(fn, 1, "val")
	writeSyncAtomicPointerParamValue(out, valName, "__val", indent)
	writeSyncAtomicMutableGuard(out, addrName, indent)
	out.WriteString(indent)
	out.WriteString("*__guard.as_mut().unwrap() = __val;\n")
}

func writeSyncAtomicSwapPointerBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	addrName := RustLocalIdent(functionParamName(fn, 0, "addr"))
	newName := functionParamName(fn, 1, "new")
	writeSyncAtomicPointerParamValue(out, newName, "__new", indent)
	writeSyncAtomicMutableGuard(out, addrName, indent)
	out.WriteString(indent)
	out.WriteString("let __old = __guard.as_ref().copied().unwrap_or(0);\n")
	out.WriteString(indent)
	out.WriteString("*__guard.as_mut().unwrap() = __new;\n")
	out.WriteString(indent)
	WriteWrapperPrefix(out)
	out.WriteString("__old")
	WriteWrapperSuffix(out)
	out.WriteString("\n")
}

func writeSyncAtomicCompareAndSwapPointerBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	addrName := RustLocalIdent(functionParamName(fn, 0, "addr"))
	oldName := functionParamName(fn, 1, "old")
	newName := functionParamName(fn, 2, "new")
	writeSyncAtomicPointerParamValue(out, oldName, "__old", indent)
	writeSyncAtomicPointerParamValue(out, newName, "__new", indent)
	writeSyncAtomicMutableGuard(out, addrName, indent)
	out.WriteString(indent)
	out.WriteString("if __guard.as_ref().copied().unwrap_or(0) == __old {\n")
	out.WriteString(indent)
	out.WriteString("    *__guard.as_mut().unwrap() = __new;\n")
	out.WriteString(indent)
	out.WriteString("    true\n")
	out.WriteString(indent)
	out.WriteString("} else {\n")
	out.WriteString(indent)
	out.WriteString("    false\n")
	out.WriteString(indent)
	out.WriteString("}\n")
}

func writeSyncAtomicMutableGuard(out *strings.Builder, addrName string, indent string) {
	out.WriteString(indent)
	out.WriteString("let mut __guard = ")
	out.WriteString(addrName)
	WriteBorrowMethod(out, true)
	out.WriteString(";\n")
}

func writeSyncAtomicPointerParamValue(out *strings.Builder, paramName string, localName string, indent string) {
	out.WriteString(indent)
	out.WriteString("let ")
	out.WriteString(localName)
	out.WriteString(" = ")
	out.WriteString(RustLocalIdent(paramName))
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().copied().unwrap_or(0);\n")
}

func functionPackagePath(fn *ast.FuncDecl) string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return ""
	}
	if obj, ok := typeInfo.GetObject(fn.Name).(*types.Func); ok && obj.Pkg() != nil {
		return obj.Pkg().Path()
	}
	if typeInfo.pkg != nil {
		return typeInfo.pkg.Path()
	}
	return ""
}

func writeFuncPCIntrinsicBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	paramName := "f"
	if fn != nil && fn.Type != nil && fn.Type.Params != nil && len(fn.Type.Params.List) > 0 {
		firstParam := fn.Type.Params.List[0]
		if len(firstParam.Names) > 0 && firstParam.Names[0] != nil {
			paramName = firstParam.Names[0].Name
		}
	}
	out.WriteString(indent)
	out.WriteString("let __guard = ")
	out.WriteString(RustLocalIdent(paramName))
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("let __value = __guard.as_ref().expect(\"internal/abi.")
	out.WriteString(fn.Name.Name)
	out.WriteString(" requires a function value\");\n")
	out.WriteString(indent)
	out.WriteString("let mut __hasher = std::collections::hash_map::DefaultHasher::new();\n")
	out.WriteString(indent)
	out.WriteString("std::hash::Hash::hash(&std::any::Any::type_id(__value.as_ref()), &mut __hasher);\n")
	out.WriteString(indent)
	out.WriteString("std::hash::Hasher::finish(&__hasher) as usize\n")
}

func writeInternalABITypeOfIntrinsicBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	NeedGoAnyTypeMetadata()
	NeedEmbeddedOwnerRegistry()
	paramName := "a"
	if fn != nil && fn.Type != nil && fn.Type.Params != nil && len(fn.Type.Params.List) > 0 {
		firstParam := fn.Type.Params.List[0]
		if len(firstParam.Names) > 0 && firstParam.Names[0] != nil {
			paramName = firstParam.Names[0].Name
		}
	}

	sliceHandleType := "Rc<RefCell<Option<Vec<u8>>>>"
	anyBoxType := rustAnyTraitObject()
	anyHandleType := "Rc<RefCell<Option<" + anyBoxType + ">>>"
	anySliceType := "Vec<" + anyBoxType + ">"
	anySliceHandleType := "Rc<RefCell<Option<" + anySliceType + ">>>"
	if NeedsConcurrentWrapper() {
		sliceHandleType = "Arc<Mutex<Option<Vec<u8>>>>"
		anyHandleType = "Arc<Mutex<Option<" + anyBoxType + ">>>"
		anySliceHandleType = "Arc<Mutex<Option<" + anySliceType + ">>>"
	}

	out.WriteString(indent)
	out.WriteString("let __guard = ")
	out.WriteString(RustLocalIdent(paramName))
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("let __value = match __guard.as_ref() { Some(__value) => __value.as_ref(), None => return ")
	if !writeGoPtrNilReturnValueForFunc(out, fn, 0) {
		WriteWrappedNone(out)
	}
	out.WriteString(" };\n")
	out.WriteString(indent)
	out.WriteString("let mut __typ = Type::default();\n")
	out.WriteString(indent)
	out.WriteString("let __go_any_metadata = go_any_type_metadata(__value);\n")
	writeInternalABIKindAssignment(out, indent, sliceHandleType, anySliceType, anySliceHandleType, anyBoxType, anyHandleType)
	out.WriteString(indent)
	out.WriteString("*__typ.kind_")
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(Kind(")
	WriteWrapperPrefix(out)
	out.WriteString("__kind")
	WriteWrapperSuffix(out)
	out.WriteString("));\n")
	out.WriteString(indent)
	out.WriteString("*__typ.size_")
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(std::mem::size_of_val(__value));\n")
	out.WriteString(indent)
	out.WriteString("if let Some(__go_meta) = __go_any_metadata { if __go_meta.comparable { *__typ.equal")
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(Box::new(|_, _| false) as ")
	writeInternalABIEqualFunctionType(out)
	out.WriteString("); } }\n")
	writeInternalABIPointerMetadataBranch(out, fn, indent)
	out.WriteString(indent)
	if !writeGoPtrLocalReturnValueForFunc(out, fn, 0, func() {
		WriteWrapperPrefix(out)
		out.WriteString("__typ")
		WriteWrapperSuffix(out)
	}) {
		WriteWrapperPrefix(out)
		out.WriteString("__typ")
		WriteWrapperSuffix(out)
	}
	out.WriteString("\n")
}

func writeInternalABIKindAssignment(out *strings.Builder, indent string, sliceHandleType string, anySliceType string, anySliceHandleType string, anyBoxType string, anyHandleType string) {
	out.WriteString(indent)
	out.WriteString("let __kind: u8 = if let Some(__go_meta) = __go_any_metadata {\n")
	writeInternalABIKindMetadataMatch(out, indent+"    ", "__go_meta.kind")
	out.WriteString("\n")

	checks := []struct {
		rustType string
		kind     string
	}{
		{"bool", "BOOL"},
		{"i32", "INT"},
		{"isize", "INT"},
		{"i8", "INT8"},
		{"i16", "INT16"},
		{"i64", "INT64"},
		{"u8", "UINT8"},
		{"u16", "UINT16"},
		{"u32", "UINT32"},
		{"u64", "UINT64"},
		{"usize", "UINTPTR"},
		{"f32", "FLOAT32"},
		{"f64", "FLOAT64"},
		{"String", "STRING"},
		{"&'static str", "STRING"},
		{"char", "INT32"},
		{"Vec<u8>", "SLICE"},
		{"Vec<i32>", "SLICE"},
		{"Vec<i64>", "SLICE"},
		{"Vec<f64>", "SLICE"},
		{"Vec<String>", "SLICE"},
		{"Vec<bool>", "SLICE"},
		{sliceHandleType, "SLICE"},
		{anySliceType, "SLICE"},
		{anySliceHandleType, "SLICE"},
		{anyBoxType, "INTERFACE"},
		{anyHandleType, "INTERFACE"},
	}
	for _, check := range checks {
		out.WriteString(indent)
		out.WriteString("} else if <dyn std::any::Any>::is::<")
		out.WriteString(check.rustType)
		out.WriteString(">(__value) {\n")
		out.WriteString(indent)
		out.WriteString("    ")
		out.WriteString(check.kind)
		out.WriteString("\n")
	}
	out.WriteString(indent)
	out.WriteString("} else {\n")
	out.WriteString(indent)
	out.WriteString("    panic!(\"internal/abi.TypeOf unsupported Rust Any payload: {}\", std::any::type_name_of_val(__value))\n")
	out.WriteString(indent)
	out.WriteString("};\n")
}

func writeInternalABIKindMetadataMatch(out *strings.Builder, indent string, valueExpr string) {
	out.WriteString(indent)
	out.WriteString("match ")
	out.WriteString(valueExpr)
	out.WriteString(" {\n")
	for _, arm := range []struct {
		value string
		kind  string
	}{
		{"struct", "STRUCT"},
		{"pointer", "POINTER"},
		{"slice", "SLICE"},
		{"map", "MAP"},
		{"interface", "INTERFACE"},
		{"chan", "CHAN"},
		{"func", "FUNC"},
		{"array", "ARRAY"},
		{"basic", "INVALID"},
	} {
		out.WriteString(indent)
		out.WriteString("    \"")
		out.WriteString(arm.value)
		out.WriteString("\" => ")
		out.WriteString(arm.kind)
		out.WriteString(",\n")
	}
	out.WriteString(indent)
	out.WriteString("    _ => panic!(\"internal/abi.TypeOf unsupported Go metadata kind: {}\", ")
	out.WriteString(valueExpr)
	out.WriteString("),\n")
	out.WriteString(indent)
	out.WriteString("}")
}

func writeInternalABIPointerMetadataBranch(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	out.WriteString(indent)
	out.WriteString("if let Some(__go_meta) = __go_any_metadata {\n")
	out.WriteString(indent)
	out.WriteString("    if __go_meta.kind == \"pointer\" {\n")
	out.WriteString(indent)
	out.WriteString("        let mut __ptr_type = PtrType::default();\n")
	out.WriteString(indent)
	out.WriteString("        *__ptr_type.r#type")
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(__typ);\n")
	out.WriteString(indent)
	out.WriteString("        if let Some(__go_elem_kind) = __go_meta.elem_kind {\n")
	out.WriteString(indent)
	out.WriteString("            let mut __elem_type = Type::default();\n")
	out.WriteString(indent)
	out.WriteString("            let __elem_kind: u8 = ")
	writeInternalABIKindMetadataMatch(out, indent+"            ", "__go_elem_kind")
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("            *__elem_type.kind_")
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(Kind(")
	WriteWrapperPrefix(out)
	out.WriteString("__elem_kind")
	WriteWrapperSuffix(out)
	out.WriteString("));\n")
	out.WriteString(indent)
	out.WriteString("            if __go_meta.elem_comparable {\n")
	out.WriteString(indent)
	out.WriteString("                *__elem_type.equal")
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(Box::new(|_, _| false) as ")
	writeInternalABIEqualFunctionType(out)
	out.WriteString(");\n")
	out.WriteString(indent)
	out.WriteString("            }\n")
	out.WriteString(indent)
	out.WriteString("            *__ptr_type.elem")
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(__elem_type);\n")
	out.WriteString(indent)
	out.WriteString("        }\n")
	out.WriteString(indent)
	out.WriteString("        let __owner = ")
	WriteWrapperPrefix(out)
	out.WriteString("__ptr_type")
	WriteWrapperSuffix(out)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("        let __embedded = { let __owner_guard = __owner")
	WriteBorrowMethod(out, false)
	out.WriteString("; __owner_guard.as_ref().unwrap().r#type.clone() };\n")
	out.WriteString(indent)
	out.WriteString("        let __embedded_key = { let __embedded_guard = __embedded")
	WriteBorrowMethod(out, false)
	out.WriteString("; __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };\n")
	out.WriteString(indent)
	out.WriteString("        go_register_embedded_owner(__embedded_key, __owner.clone());\n")
	out.WriteString(indent)
	out.WriteString("        return ")
	if !writeGoPtrLocalReturnValueForFunc(out, fn, 0, func() {
		out.WriteString("__embedded")
	}) {
		out.WriteString("__embedded")
	}
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("    }\n")
	out.WriteString(indent)
	out.WriteString("}\n")
}

func writeInternalABIEqualFunctionType(out *strings.Builder) {
	out.WriteString("Box<dyn FnMut(")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("<")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("<Option<usize>>>, ")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("<")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("<Option<usize>>>) -> bool")
	if NeedsConcurrentWrapper() {
		out.WriteString(" + Send + Sync")
	}
	out.WriteString(">")
}

func writeInternalBytealgCountIntrinsicBody(out *strings.Builder, fn *ast.FuncDecl, indent string, stringInput bool) {
	haystackName := functionParamName(fn, 0, "b")
	if stringInput {
		haystackName = functionParamName(fn, 0, "s")
	}
	needleName := functionParamName(fn, 1, "c")

	out.WriteString(indent)
	out.WriteString("let __needle = (*")
	out.WriteString(RustLocalIdent(needleName))
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone();\n")
	out.WriteString(indent)
	out.WriteString("let __haystack = ")
	out.WriteString(RustLocalIdent(haystackName))
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("let __count = __haystack.as_ref().map(|__v| __v")
	if stringInput {
		out.WriteString(".as_bytes()")
	}
	out.WriteString(".iter().filter(|&&__b| __b == __needle).count()).unwrap_or(0) as i32;\n")
	out.WriteString(indent)
	if runtimeLinkedSingleResultReturnsBareScalar(fn) {
		out.WriteString("__count\n")
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("__count")
	WriteWrapperSuffix(out)
	out.WriteString("\n")
}

func writeInternalBytealgMakeNoZeroBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	nName := RustLocalIdent(functionParamName(fn, 0, "n"))
	out.WriteString(indent)
	out.WriteString("let __n = (*")
	out.WriteString(nName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone();\n")
	out.WriteString(indent)
	out.WriteString("if __n < 0 { panic!(\"internal/bytealg.MakeNoZero: negative length\"); }\n")
	out.WriteString(indent)
	out.WriteString("let __len = __n as usize;\n")
	out.WriteString(indent)
	WriteWrapperPrefix(out)
	out.WriteString("vec![0u8; __len]")
	WriteWrapperSuffix(out)
	out.WriteString("\n")
}

func writeInternalBytealgIndexByteStringBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	sName := functionParamName(fn, 0, "s")
	cName := RustLocalIdent(functionParamName(fn, 1, "c"))
	writeRuntimeLinkedStringParamClone(out, sName, "__s", indent)
	out.WriteString(indent)
	out.WriteString("let __c = (*")
	out.WriteString(cName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone();\n")
	out.WriteString(indent)
	out.WriteString("__s.as_bytes().iter().position(|&__b| __b == __c).map(|__i| __i as i32).unwrap_or(-1)\n")
}

func writeInternalBytealgIndexStringBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	aName := functionParamName(fn, 0, "a")
	bName := functionParamName(fn, 1, "b")
	writeRuntimeLinkedStringParamClone(out, aName, "__a", indent)
	writeRuntimeLinkedStringParamClone(out, bName, "__b", indent)
	out.WriteString(indent)
	out.WriteString("__a.find(&__b).map(|__i| __i as i32).unwrap_or(-1)\n")
}

func writeInternalBytealgCompareStringBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	aName := functionParamName(fn, 0, "a")
	bName := functionParamName(fn, 1, "b")
	writeRuntimeLinkedStringParamClone(out, aName, "__a", indent)
	writeRuntimeLinkedStringParamClone(out, bName, "__b", indent)
	out.WriteString(indent)
	out.WriteString("match __a.cmp(&__b) { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 }\n")
}

func writeInternalBuildcfgExpListIntrinsicBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	fields, ok := internalBuildcfgExpListBoolFields(fn)
	if !ok {
		out.WriteString(indent)
		out.WriteString("unimplemented!(\"type info required for internal/buildcfg.expList\")\n")
		return
	}

	expName := RustLocalIdent(functionParamName(fn, 0, "exp"))
	baseName := RustLocalIdent(functionParamName(fn, 1, "base"))
	allName := RustLocalIdent(functionParamName(fn, 2, "all"))

	trackWrapperImports()
	out.WriteString(indent)
	out.WriteString("let mut list: ")
	out.WriteString(GetOuterWrapperType())
	out.WriteString("<")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("<Option<Vec<String>>>> = ")
	WriteWrappedNone(out)
	out.WriteString(";\n")

	out.WriteString(indent)
	out.WriteString("{\n")
	out.WriteString(indent)
	out.WriteString("    let __exp_guard = ")
	out.WriteString(expName)
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("    let __exp_value = __exp_guard.as_ref().expect(\"internal/buildcfg.expList requires exp flags\");\n")
	out.WriteString(indent)
	out.WriteString("    let __base_guard = ")
	out.WriteString(baseName)
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("    let __base_value = __base_guard.as_ref();\n")
	out.WriteString(indent)
	out.WriteString("    let __all = (*")
	out.WriteString(allName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone();\n")

	for _, field := range fields {
		rustField := ToSnakeCase(field.Name())
		enabledName := strings.ToLower(field.Name())
		disabledName := "no" + enabledName

		out.WriteString(indent)
		out.WriteString("    let __val = (*__exp_value.")
		out.WriteString(rustField)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone();\n")
		out.WriteString(indent)
		out.WriteString("    let __base_val = __base_value.map(|__base| (*__base.")
		out.WriteString(rustField)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()).unwrap_or(false);\n")
		out.WriteString(indent)
		out.WriteString("    if __all || __val != __base_val {\n")
		out.WriteString(indent)
		out.WriteString("        let mut __list_guard = list")
		WriteBorrowMethod(out, true)
		out.WriteString(";\n")
		out.WriteString(indent)
		out.WriteString("        if __list_guard.is_none() { *__list_guard = Some(Vec::new()); }\n")
		out.WriteString(indent)
		out.WriteString("        if __val { __list_guard.as_mut().unwrap().push(\"")
		out.WriteString(enabledName)
		out.WriteString("\".to_string()); } else { __list_guard.as_mut().unwrap().push(\"")
		out.WriteString(disabledName)
		out.WriteString("\".to_string()); }\n")
		out.WriteString(indent)
		out.WriteString("    }\n")
	}

	out.WriteString(indent)
	out.WriteString("}\n")
	out.WriteString(indent)
	out.WriteString("list.clone()\n")
}

func internalBuildcfgExpListBoolFields(fn *ast.FuncDecl) ([]*types.Var, bool) {
	sig, ok := funcDeclSignatureFromTypeInfo(fn)
	if !ok || sig.Params() == nil || sig.Params().Len() < 1 {
		return nil, false
	}
	ptr, ok := types.Unalias(sig.Params().At(0).Type()).(*types.Pointer)
	if !ok {
		return nil, false
	}
	named, ok := types.Unalias(ptr.Elem()).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return nil, false
	}
	if named.Obj().Pkg().Path() != "internal/goexperiment" || named.Obj().Name() != "Flags" {
		return nil, false
	}
	st, ok := named.Underlying().(*types.Struct)
	if !ok {
		return nil, false
	}
	fields := make([]*types.Var, 0, st.NumFields())
	for i := 0; i < st.NumFields(); i++ {
		field := st.Field(i)
		if !types.Identical(types.Unalias(field.Type()), types.Typ[types.Bool]) {
			return nil, false
		}
		fields = append(fields, field)
	}
	return fields, true
}

func writeInternalGodebugSetUpdateBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	updateName := RustLocalIdent(functionParamName(fn, 0, "update"))
	out.WriteString(indent)
	out.WriteString("let __env = std::env::var(\"GODEBUG\").unwrap_or_default();\n")
	out.WriteString(indent)
	out.WriteString("if !__env.is_empty() {\n")
	out.WriteString(indent)
	out.WriteString("    let mut __update_guard = ")
	out.WriteString(updateName)
	WriteBorrowMethod(out, true)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("    if let Some(__update) = __update_guard.as_mut() {\n")
	out.WriteString(indent)
	out.WriteString("        __update(")
	WriteWrapperPrefix(out)
	out.WriteString("String::new()")
	WriteWrapperSuffix(out)
	out.WriteString(", ")
	WriteWrapperPrefix(out)
	out.WriteString("__env")
	WriteWrapperSuffix(out)
	out.WriteString(");\n")
	out.WriteString(indent)
	out.WriteString("    }\n")
	out.WriteString(indent)
	out.WriteString("}\n")
}

func writeInternalGodebugSetNewIncNonDefaultBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	paramName := RustLocalIdent(functionParamName(fn, 0, "newIncNonDefault"))
	out.WriteString(indent)
	out.WriteString("let _ = ")
	out.WriteString(paramName)
	out.WriteString(";\n")
}

func writeInternalGodebugRegisterMetricBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	nameParam := RustLocalIdent(functionParamName(fn, 0, "name"))
	readParam := RustLocalIdent(functionParamName(fn, 1, "read"))
	out.WriteString(indent)
	out.WriteString("let _ = (")
	out.WriteString(nameParam)
	out.WriteString(", ")
	out.WriteString(readParam)
	out.WriteString(");\n")
}

func writeInternalGodebugWriteBody(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	fdName := RustLocalIdent(functionParamName(fn, 0, "fd"))
	ptrName := RustLocalIdent(functionParamName(fn, 1, "p"))
	nName := RustLocalIdent(functionParamName(fn, 2, "n"))

	out.WriteString(indent)
	out.WriteString("let __fd = (*")
	out.WriteString(fdName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone();\n")
	out.WriteString(indent)
	out.WriteString("let __ptr = (*")
	out.WriteString(ptrName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone();\n")
	out.WriteString(indent)
	out.WriteString("let __n = (*")
	out.WriteString(nName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone();\n")
	out.WriteString(indent)
	out.WriteString("if __n <= 0 { return 0; }\n")
	out.WriteString(indent)
	out.WriteString("let __bytes = unsafe { std::slice::from_raw_parts(__ptr as *const u8, __n as usize) };\n")
	out.WriteString(indent)
	out.WriteString("let __result = match __fd {\n")
	out.WriteString(indent)
	out.WriteString("    1 => std::io::Write::write_all(&mut std::io::stdout(), __bytes),\n")
	out.WriteString(indent)
	out.WriteString("    2 => std::io::Write::write_all(&mut std::io::stderr(), __bytes),\n")
	out.WriteString(indent)
	out.WriteString("    _ => Err(std::io::Error::new(std::io::ErrorKind::Unsupported, \"unsupported runtime.write fd\")),\n")
	out.WriteString(indent)
	out.WriteString("};\n")
	out.WriteString(indent)
	out.WriteString("if __result.is_ok() { __n } else { -1 }\n")
}

func runtimeLinkedReceiverIsNamed(fn *ast.FuncDecl, name string) bool {
	sig, ok := funcDeclSignatureFromTypeInfo(fn)
	if !ok || sig.Recv() == nil {
		return false
	}
	recvType := types.Unalias(sig.Recv().Type())
	if ptr, ok := recvType.(*types.Pointer); ok {
		recvType = types.Unalias(ptr.Elem())
	}
	named, ok := recvType.(*types.Named)
	return ok && named.Obj() != nil && named.Obj().Name() == name
}

func writeStringsBuilderCopyCheckBody(out *strings.Builder, indent string) {
	out.WriteString(indent)
	out.WriteString("let _ = self;\n")
}

func writeStringsBuilderStringBody(out *strings.Builder, indent string) {
	out.WriteString(indent)
	out.WriteString("let __buf_guard = self.buf")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("let __text = __buf_guard.as_ref().map(|__buf| String::from_utf8_lossy(__buf).to_string()).unwrap_or_default();\n")
	out.WriteString(indent)
	WriteWrapperPrefix(out)
	out.WriteString("__text")
	WriteWrapperSuffix(out)
	out.WriteString("\n")
}

func runtimeLinkedSingleResultReturnsBareScalar(fn *ast.FuncDecl) bool {
	sig, ok := funcDeclSignatureFromTypeInfo(fn)
	return ok && signatureReturnsBareScalar(sig)
}

func functionParamName(fn *ast.FuncDecl, index int, fallback string) string {
	if fn == nil || fn.Type == nil || fn.Type.Params == nil {
		return fallback
	}
	seen := 0
	for _, field := range fn.Type.Params.List {
		names := field.Names
		if len(names) == 0 {
			if seen == index {
				return fallback
			}
			seen++
			continue
		}
		for _, name := range names {
			if seen == index && name != nil {
				return name.Name
			}
			seen++
		}
	}
	return fallback
}

func writeRuntimeLinkedStringParamClone(out *strings.Builder, paramName string, localName string, indent string) {
	writeRuntimeLinkedParamClone(out, paramName, localName, indent)
}

func writeRuntimeLinkedParamClone(out *strings.Builder, paramName string, localName string, indent string) {
	out.WriteString(indent)
	out.WriteString("let ")
	out.WriteString(localName)
	out.WriteString(" = (*")
	out.WriteString(RustLocalIdent(paramName))
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone();\n")
}

func writeFuncDeclParams(out *strings.Builder, fn *ast.FuncDecl) {
	if fn == nil || fn.Type == nil || fn.Type.Params == nil {
		return
	}
	paramIndex := 0
	first := true
	for _, field := range fn.Type.Params.List {
		if len(field.Names) == 0 {
			if !first {
				out.WriteString(", ")
			}
			writeFuncDeclParam(out, fn, paramIndex, fmt.Sprintf("__arg%d", paramIndex), field.Type, false)
			paramIndex++
			first = false
			continue
		}
		for _, name := range field.Names {
			if !first {
				out.WriteString(", ")
			}
			writeFuncDeclParam(out, fn, paramIndex, name.Name, field.Type, blockIdentAssigned(fn.Body, name.Name))
			paramIndex++
			first = false
		}
	}
}

func writeFuncDeclParam(out *strings.Builder, fn *ast.FuncDecl, paramIndex int, name string, typ ast.Expr, mutable bool) {
	if mutable {
		out.WriteString("mut ")
	}
	out.WriteString(RustLocalIdent(name))
	out.WriteString(": ")
	if elemRustType, ok := sliceElemPtrSliceParamInfoForDeclObject(fn, paramIndex); ok {
		out.WriteString(sliceElemPtrSliceRustType(elemRustType))
		return
	}
	if elemRustType, ok := goPtrSlotParamDeclElemRustType(fn, paramIndex); ok {
		out.WriteString(goPtrSlotParamRustType(elemRustType))
		return
	}
	if elemRustType, ok := goPtrParamDeclElemRustType(fn, paramIndex); ok {
		NeedSliceElemPtr()
		out.WriteString("GoPtr<")
		out.WriteString(elemRustType)
		out.WriteString(">")
		return
	}
	if functionRustType, ok := functionValueGoPtrAwareWrappedTypeForFuncDeclParam(fn, paramIndex, typ); ok {
		out.WriteString(functionRustType)
		return
	}
	out.WriteString(GoTypeToRustParam(typ))
}

// getEmbeddedFieldName extracts the type name from an embedded field
func getEmbeddedFieldName(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		return t.Name
	case *ast.StarExpr:
		// For pointer types, get the underlying type name
		return getEmbeddedFieldName(t.X)
	case *ast.IndexExpr:
		return getEmbeddedFieldName(t.X)
	case *ast.IndexListExpr:
		return getEmbeddedFieldName(t.X)
	case *ast.SelectorExpr:
		// For qualified types like pkg.Type
		return t.Sel.Name
	default:
		// Fallback to a generic name
		return "embedded"
	}
}

func typeDefinitionUnderlyingName(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		return t.Name
	case *ast.ArrayType:
		prefix := "[]"
		if t.Len != nil {
			prefix = "[_]"
		}
		return prefix + typeDefinitionUnderlyingName(t.Elt)
	case *ast.MapType:
		return "map[" + typeDefinitionUnderlyingName(t.Key) + "]" + typeDefinitionUnderlyingName(t.Value)
	case *ast.StarExpr:
		return "*" + typeDefinitionUnderlyingName(t.X)
	case *ast.SelectorExpr:
		if ident, ok := t.X.(*ast.Ident); ok {
			return ident.Name + "." + t.Sel.Name
		}
		return t.Sel.Name
	case *ast.ChanType:
		return "chan " + typeDefinitionUnderlyingName(t.Value)
	case *ast.FuncType:
		return "func"
	default:
		return fmt.Sprintf("%T", expr)
	}
}

func functionTypeSpecRustType(typeSpec *ast.TypeSpec, fallback *ast.FuncType) string {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.info != nil {
		if obj, ok := typeInfo.info.Defs[typeSpec.Name].(*types.TypeName); ok {
			if _, ok := signatureFromType(obj.Type()); ok {
				return goTypesTypeToRustWrapped(obj.Type())
			}
		}
	}
	return GoTypeToRust(fallback)
}

func functionTypeSpecRustBoxType(typeSpec *ast.TypeSpec, fallback *ast.FuncType) string {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.info != nil {
		if obj, ok := typeInfo.info.Defs[typeSpec.Name].(*types.TypeName); ok {
			if sig, ok := signatureFromType(obj.Type()); ok {
				return signatureToBoxDynFn(sig)
			}
		}
	}
	return generateClosureType(fallback)
}

// emitStructTypeDeclBody writes the Rust struct definition plus its derived
// impls (clone, Default, Display, PartialEq, Ord, JSON) for a Go struct type.
// Shared by a direct `type T struct{...}` and a defined type whose underlying is
// a named struct (`type Term term`), which must expose the same fields rather
// than lower to a newtype over a wrapped handle.
// definedTypeUnderlyingStructAST returns the struct AST for `type A B` when B is
// a named type whose underlying is a struct, so A can be emitted with B's fields.
// It confirms the struct shape through go/types and fetches the AST from the
// struct registry; returns nil (caller falls back to a newtype) when either is
// unavailable, e.g. the underlying isn't a struct or wasn't registered yet.
func definedTypeUnderlyingStructAST(t ast.Expr) *ast.StructType {
	ident, ok := t.(*ast.Ident)
	if !ok {
		return nil
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.pkg != nil && typeInfo.pkg.Scope() != nil {
		obj := typeInfo.pkg.Scope().Lookup(ident.Name)
		named, ok := obj.(*types.TypeName)
		if !ok {
			return nil
		}
		n, ok := named.Type().(*types.Named)
		if !ok {
			return nil
		}
		if _, ok := n.Underlying().(*types.Struct); !ok {
			return nil
		}
	}
	if def, ok := structDefs[ident.Name]; ok && def != nil {
		return def.ASTType
	}
	return nil
}

func emitStructTypeDeclBody(out *strings.Builder, typeSpec *ast.TypeSpec, t *ast.StructType) {
	structName := typeSpec.Name.Name
	rustTypeName := RustTypeNameForUse(structName)
	generics := rustTypeGenericsForStructTypeSpec(typeSpec, t)
	registerStructDef(structName, t)
	setStructDefPhantomTypeParams(structName, generics.Phantom)

	deriveClone := typeSpec.TypeParams == nil || len(typeSpec.TypeParams.List) == 0
	writeStructDerive(out, structName, t, deriveClone)
	out.WriteString("pub struct ")
	out.WriteString(rustTypeName)
	out.WriteString(generics.Decl)
	out.WriteString(" {\n")

	for fieldIndex, field := range t.Fields.List {
		// Add struct tag as comment if present
		if field.Tag != nil && field.Tag.Value != "" {
			out.WriteString("    // tags: ")
			out.WriteString(field.Tag.Value)
			out.WriteString("\n")
		}

		if len(field.Names) > 0 {
			// Handle multiple names on one line (e.g., X, Y int)
			for nameIndex, name := range field.Names {
				out.WriteString("    pub ")
				out.WriteString(rustStructFieldName(name, fieldIndex, nameIndex))
				out.WriteString(": ")
				if fieldInfo, ok := goPtrArrayFieldInfoForStructField(typeSpec, t, structName, name.Name); ok {
					out.WriteString(goPtrArrayFieldRustType(fieldInfo))
				} else if fieldInfo, ok := sliceElemPtrSliceFieldInfoForStructField(typeSpec, t, structName, name.Name); ok {
					out.WriteString(sliceElemPtrSliceRustType(sliceElemPtrSliceFieldElemRustType(fieldInfo)))
				} else if fieldInfo, ok := sliceElemPtrFieldInfoForStructField(typeSpec, t, structName, name.Name); ok {
					recordGeneratedGoPtrFieldForStructField(typeSpec, t, structName, name.Name)
					NeedSliceElemPtr()
					out.WriteString("GoPtr<")
					out.WriteString(sliceElemPtrFieldElemRustType(fieldInfo))
					out.WriteString(">")
				} else if syncAtomicPointerStorageField(typeSpec, field, name) {
					out.WriteString(syncAtomicPointerStorageRustType(typeSpec))
				} else if functionRustType, ok := functionValueGoPtrAwareWrappedTypeForNamedTypeExpr(name, field.Type); ok {
					out.WriteString(functionRustType)
				} else {
					out.WriteString(GoTypeToRust(field.Type))
				}
				out.WriteString(",\n")
			}
		} else {
			// Embedded field - extract the type name
			fieldName := getEmbeddedFieldName(field.Type)
			out.WriteString("    pub ")
			out.WriteString(ToSnakeCase(fieldName))
			out.WriteString(": ")
			if fieldInfo, ok := goPtrArrayFieldInfoForStructField(typeSpec, t, structName, fieldName); ok {
				out.WriteString(goPtrArrayFieldRustType(fieldInfo))
			} else if fieldInfo, ok := sliceElemPtrSliceFieldInfoForStructField(typeSpec, t, structName, fieldName); ok {
				out.WriteString(sliceElemPtrSliceRustType(sliceElemPtrSliceFieldElemRustType(fieldInfo)))
			} else if fieldInfo, ok := sliceElemPtrFieldInfoForStructField(typeSpec, t, structName, fieldName); ok {
				recordGeneratedGoPtrFieldForStructField(typeSpec, t, structName, fieldName)
				NeedSliceElemPtr()
				out.WriteString("GoPtr<")
				out.WriteString(sliceElemPtrFieldElemRustType(fieldInfo))
				out.WriteString(">")
			} else {
				out.WriteString(GoTypeToRust(field.Type))
			}
			out.WriteString(",\n")
		}
	}
	writeRustPhantomField(out, generics)

	out.WriteString("}\n\n")

	generateStructValueClone(out, structName, t, generics)
	if !deriveClone {
		out.WriteString("\n")
		generateStructClone(out, structName, generics)
	}
	out.WriteString("\n")

	generateStructDefault(out, typeSpec, structName, t, generics)
	if structNeedsCustomDefault(t) {
		out.WriteString("\n")
	}

	// Generate Display implementation to match Go's format
	displayGenerics, hasDisplayTypeInfo := rustStructDisplayGenerics(generics, typeSpec, t)
	if hasDisplayTypeInfo {
		generateStructDisplay(out, structName, t, displayGenerics)
	} else {
		generateStructDisplayTypeInfoRequired(out, structName, displayGenerics)
	}
	if IsErrorImplType(structName) && !structCanDeriveDebug(t) {
		generateStructDebug(out, structName, generics)
	}
	generateStructPartialEq(out, structName, t, generics)
	generateStructPointerComparableIdentity(out, structName, generics)
	generateStructOrd(out, structName, t, generics)
	generateStructJsonDecode(out, structName, t, generics)
}

func TranspileTypeDecl(out *strings.Builder, typeSpec *ast.TypeSpec, genDecl *ast.GenDecl) {
	rustTypeName := RustTypeNameForUse(typeSpec.Name.Name)
	switch t := typeSpec.Type.(type) {
	case *ast.StructType:
		emitStructTypeDeclBody(out, typeSpec, t)

	case *ast.InterfaceType:
		// Generate a trait for the interface
		// Add Display plus Any so trait object equality can downcast.
		// Embedded named local interfaces become Rust supertraits so that
		// `&dyn SubTrait` upcasts to `&dyn SuperTrait` automatically.
		TrackImport("Any")
		embeddedTraits := embeddedLocalInterfaceNames(t)
		ifaceTypes := localInterfaceTypesFromTypeSpec(typeSpec)
		out.WriteString("pub trait ")
		out.WriteString(rustTypeName)
		out.WriteString(":")
		for _, st := range embeddedTraits {
			out.WriteString(" ")
			out.WriteString(st)
			out.WriteString(" +")
		}
		out.WriteString(" std::fmt::Display + Any {\n")
		TrackImport("Display")
		traitSnake := traitMethodSuffix(rustTypeName)
		out.WriteString("    fn __go_clone_box_")
		out.WriteString(traitSnake)
		out.WriteString("(&self) -> ")
		out.WriteString(rustLocalInterfaceTraitObject(rustTypeName))
		out.WriteString(";\n")
		if len(embeddedTraits) == 0 {
			out.WriteString("    fn __go_as_any(&self) -> &dyn Any;\n")
		}
		out.WriteString("    fn __go_eq_")
		out.WriteString(traitSnake)
		out.WriteString("(&self, other: ")
		out.WriteString(rustLocalInterfaceParamBare(rustTypeName))
		out.WriteString(") -> bool;\n")

		// Generate method signatures for directly-declared methods only.
		// Inherited methods come through supertraits — redeclaring them
		// here would cause method-resolution ambiguity (E0034).
		for _, method := range t.Methods.List {
			if len(method.Names) > 0 {
				// Named method
				funcType, ok := method.Type.(*ast.FuncType)
				if !ok {
					continue
				}

				out.WriteString("    fn ")
				out.WriteString(ToSnakeCase(method.Names[0].Name))
				out.WriteString("(")
				out.WriteString(interfaceTraitMethodReceiver(typeSpec.Name.Name, method.Names[0].Name))

				// Add other parameters
				if funcType.Params != nil && len(funcType.Params.List) > 0 {
					paramIndex := 0
					for _, param := range funcType.Params.List {
						if len(param.Names) == 0 {
							out.WriteString(", ")
							out.WriteString(RustLocalIdent(fmt.Sprintf("__arg%d", paramIndex)))
							out.WriteString(": ")
							out.WriteString(GoTypeToRustParam(param.Type))
							paramIndex++
							continue
						}
						for _, name := range param.Names {
							out.WriteString(", ")
							out.WriteString(RustLocalIdent(name.Name))
							out.WriteString(": ")
							out.WriteString(GoTypeToRustParam(param.Type))
							paramIndex++
						}
					}
				}

				out.WriteString(")")

				if methodObj := interfaceMethodByName(ifaceTypes, method.Names[0].Name); methodObj != nil {
					writeInterfaceMethodResultTypesFromFunc(out, methodObj)
				} else {
					writeFunctionResultTypes(out, funcType)
				}

				out.WriteString(";\n")
			}
		}

		out.WriteString("}")
		out.WriteString("\n\nimpl Clone for ")
		out.WriteString(rustLocalInterfaceTraitObject(rustTypeName))
		out.WriteString(" {\n")
		out.WriteString("    fn clone(&self) -> Self {\n")
		out.WriteString("        ")
		out.WriteString(rustTypeName)
		out.WriteString("::__go_clone_box_")
		out.WriteString(traitSnake)
		out.WriteString("(self.as_ref())\n")
		out.WriteString("    }\n")
		out.WriteString("}")
		if localInterfaceNeedsGoValueCloneImpl(rustTypeName) {
			writeLocalInterfaceGoValueCloneImpl(out, rustTypeName, traitSnake)
		}
		if localInterfaceNeedsGoComparableImpl(rustTypeName) {
			writeLocalInterfaceGoComparableImpl(out, rustTypeName, traitSnake)
		}
		if localInterfaceNeedsMapKeyType(rustTypeName) {
			writeLocalInterfaceMapKeyType(out, rustTypeName, traitSnake)
		}
		writeEmbeddedTraitObjectAdapters(out, rustTypeName, embeddedTraits)
		writeAssignableInterfaceObjectAdapters(out, rustTypeName)
		if iface := localInterfaceTypesByName(typeSpec.Name.Name); interfaceEmbedsOnlyOrderedTerms(iface) {
			writeOrderedPrimitiveInterfaceImpls(out, rustTypeName)
		}

	default:
		// Handle type aliases and type definitions
		if typeSpec.Assign != 0 {
			// Type alias: type A = B
			out.WriteString("pub type ")
			out.WriteString(rustTypeName)
			writeTypeSpecAliasTypeParams(out, typeSpec)
			out.WriteString(" = ")
			if funcType, ok := t.(*ast.FuncType); ok {
				out.WriteString(functionTypeSpecRustType(typeSpec, funcType))
			} else {
				out.WriteString(GoTypeToRust(t))
			}
			out.WriteString(";\n")

			// Track this as a type alias
			RegisterTypeAlias(typeSpec.Name.Name)
			if _, isFuncType := t.(*ast.FuncType); isFuncType {
				RegisterFunctionTypeAlias(typeSpec.Name.Name)
				RegisterFunctionTypeAliasBox(typeSpec.Name.Name, functionTypeSpecRustBoxType(typeSpec, t.(*ast.FuncType)))
			}
		} else if _, isFuncType := t.(*ast.FuncType); isFuncType {
			// Named function type: type BinaryOp func(int, int) int
			// Emit as a type alias to the callable shape, not a newtype struct
			out.WriteString("pub type ")
			out.WriteString(rustTypeName)
			writeTypeSpecAliasTypeParams(out, typeSpec)
			out.WriteString(" = ")
			out.WriteString(functionTypeSpecRustType(typeSpec, t.(*ast.FuncType)))
			out.WriteString(";\n")

			// Track as a type alias so GoTypeToRust won't double-wrap
			RegisterTypeAlias(typeSpec.Name.Name)
			RegisterFunctionTypeAlias(typeSpec.Name.Name)
			RegisterFunctionTypeAliasBox(typeSpec.Name.Name, functionTypeSpecRustBoxType(typeSpec, t.(*ast.FuncType)))

			// Function types satisfying local interfaces need per-interface
			// wrapper structs: emitting `impl LocalIface for funcAlias` directly
			// violates Rust's orphan rule because the alias resolves to entirely
			// foreign types. The wrapper holds the alias value and forwards
			// trait methods to the function-type's inherent methods so call
			// sites can convert via Box::new(<FuncType>As<Iface>(value)).
			writeFunctionTypeInterfaceImpls(out, typeSpec.Name.Name, rustTypeName)
		} else if structAST := definedTypeUnderlyingStructAST(t); structAST != nil {
			// `type Term term` where term is a struct: Term has term's fields (Go
			// promotes them through the underlying), so emit Term as a struct with
			// those fields rather than a newtype over a wrapped handle. Otherwise
			// field access (t.field) resolves to nothing (E0609/E0615) — go/types'
			// union.Term hits this.
			registerTypeDefinitionForTypeExpr(typeSpec.Name.Name, t)
			emitStructTypeDeclBody(out, typeSpec, structAST)
			return
		} else {
			// Type definition: type A B
			// Create a newtype wrapper in Rust
			registerTypeDefinitionForTypeExpr(typeSpec.Name.Name, t)
			canDeriveDebug := typeDefinitionCanDeriveDebug(typeSpec)
			customDefault, hasCustomDefault := typeDefinitionCustomDefaultValue(typeSpec)
			if canDeriveDebug && !hasCustomDefault {
				out.WriteString("#[derive(Debug, Clone, Default)]\n")
			} else if canDeriveDebug {
				out.WriteString("#[derive(Debug, Clone)]\n")
			} else if hasCustomDefault {
				out.WriteString("#[derive(Clone)]\n")
			} else {
				out.WriteString("#[derive(Clone, Default)]\n")
			}
			out.WriteString("pub struct ")
			out.WriteString(rustTypeName)
			out.WriteString("(pub ")
			out.WriteString(GoTypeToRust(t))
			out.WriteString(");\n")
			if hasCustomDefault {
				out.WriteString("\nimpl Default for ")
				out.WriteString(rustTypeName)
				out.WriteString(" {\n")
				out.WriteString("    fn default() -> Self {\n")
				out.WriteString("        ")
				out.WriteString(customDefault)
				out.WriteString("\n")
				out.WriteString("    }\n")
				out.WriteString("}\n")
			}

			// Add Display implementation for displayable scalar type definitions
			if IsStringerImplType(typeSpec.Name.Name) {
				TrackImport("Display")
				TrackImport("Formatter")

				out.WriteString("\nimpl Display for ")
				out.WriteString(rustTypeName)
				out.WriteString(" {\n")
				out.WriteString("    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n")
				writeStringerDisplayBody(out, typeSpec.Name.Name, "        ")
				out.WriteString("    }\n")
				out.WriteString("}\n")
			} else if IsErrorImplType(typeSpec.Name.Name) {
				TrackImport("Display")
				TrackImport("Formatter")

				out.WriteString("\nimpl Display for ")
				out.WriteString(rustTypeName)
				out.WriteString(" {\n")
				out.WriteString("    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n")
				out.WriteString("        write!(f, \"{}\", (*self.error()")
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()))\n")
				out.WriteString("    }\n")
				out.WriteString("}\n")
				if !canDeriveDebug {
					generateStructDebug(out, typeSpec.Name.Name, rustTypeGenerics{})
				}
			} else if _, ok := typeDefinitionUnderlyingNamedArray(typeSpec); ok {
				TrackImport("Display")
				TrackImport("Formatter")

				out.WriteString("\nimpl Display for ")
				out.WriteString(rustTypeName)
				out.WriteString(" {\n")
				out.WriteString("    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n")
				out.WriteString("        let __inner_guard = self.0")
				WriteBorrowMethod(out, false)
				out.WriteString(";\n")
				out.WriteString("        write!(f, \"{}\", __inner_guard.as_ref().unwrap())\n")
				out.WriteString("    }\n")
				out.WriteString("}\n")
			} else if typeDefinitionUnderlyingNamedDisplayable(typeSpec) {
				TrackImport("Display")
				TrackImport("Formatter")

				out.WriteString("\nimpl Display for ")
				out.WriteString(rustTypeName)
				out.WriteString(" {\n")
				out.WriteString("    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n")
				out.WriteString("        write!(f, \"{}\", self.0")
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())\n")
				out.WriteString("    }\n")
				out.WriteString("}\n")
			} else if arrayFormatter, ok := typeDefinitionArrayDisplayFormatter(typeSpec); ok {
				TrackImport("Display")
				TrackImport("Formatter")
				if arrayFormatter == "format_slice_wrapped_stringer" {
					NeedFormatSliceWrappedStringer()
				} else {
					NeedFormatSlice()
				}

				out.WriteString("\nimpl Display for ")
				out.WriteString(rustTypeName)
				out.WriteString(" {\n")
				out.WriteString("    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n")
				out.WriteString("        write!(f, \"{}\", ")
				out.WriteString(arrayFormatter)
				out.WriteString("(&self.0))\n")
				out.WriteString("    }\n")
				out.WriteString("}\n")
			} else if typeDefinitionMapUnderlyingDisplayable(typeSpec) {
				TrackImport("Display")
				TrackImport("Formatter")
				NeedFormatMap()

				out.WriteString("\nimpl Display for ")
				out.WriteString(rustTypeName)
				out.WriteString(" {\n")
				out.WriteString("    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n")
				out.WriteString("        write!(f, \"{}\", format_map(&self.0))\n")
				out.WriteString("    }\n")
				out.WriteString("}\n")
			} else if typeDefinitionScalarUnderlyingDisplayable(typeSpec) {
				// Track necessary imports
				TrackImport("Display")
				TrackImport("Formatter")
				TrackImport("fmt")

				out.WriteString("\nimpl Display for ")
				out.WriteString(rustTypeName)
				out.WriteString(" {\n")
				out.WriteString("    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n")
				out.WriteString("        write!(f, \"{}\", self.0")
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())\n")
				out.WriteString("    }\n")
				out.WriteString("}\n")
			}
			if ident, ok := t.(*ast.Ident); ok && isEqualityComparableDefinedUnderlying(ident.Name) {
				writeScalarTypeDefinitionPartialEq(out, typeSpec.Name.Name)
				if rustType, ok := rustCastTypeForDefinedUnderlying(ident.Name); ok {
					writeScalarTypeDefinitionNumericOps(out, typeSpec.Name.Name, rustType, ident.Name)
				}
				if isBitwiseDefinedUnderlying(ident.Name) {
					writeScalarTypeDefinitionOrd(out, typeSpec.Name.Name)
				}
			}
		}
	}
}

func writeOrderedPrimitiveInterfaceImpls(out *strings.Builder, rustTypeName string) {
	traitSnake := traitMethodSuffix(rustTypeName)
	traitObject := rustLocalInterfaceTraitObject(rustTypeName)
	paramBare := rustLocalInterfaceParamBare(rustTypeName)
	for _, primitive := range []string{"i8", "i16", "i32", "i64", "isize", "u8", "u16", "u32", "u64", "usize", "f32", "f64", "String"} {
		out.WriteString("\n\nimpl ")
		out.WriteString(rustTypeName)
		out.WriteString(" for ")
		out.WriteString(primitive)
		out.WriteString(" {\n")
		out.WriteString("    fn __go_clone_box_")
		out.WriteString(traitSnake)
		out.WriteString("(&self) -> ")
		out.WriteString(traitObject)
		out.WriteString(" {\n")
		out.WriteString("        Box::new(self.clone()) as ")
		out.WriteString(traitObject)
		out.WriteString("\n")
		out.WriteString("    }\n")
		out.WriteString("    fn __go_as_any(&self) -> &dyn Any {\n")
		out.WriteString("        self\n")
		out.WriteString("    }\n")
		out.WriteString("    fn __go_eq_")
		out.WriteString(traitSnake)
		out.WriteString("(&self, other: ")
		out.WriteString(paramBare)
		out.WriteString(") -> bool {\n")
		out.WriteString("        if let Some(__other) = other.__go_as_any().downcast_ref::<")
		out.WriteString(primitive)
		out.WriteString(">() {\n")
		out.WriteString("            self == __other\n")
		out.WriteString("        } else {\n")
		out.WriteString("            false\n")
		out.WriteString("        }\n")
		out.WriteString("    }\n")
		out.WriteString("}")
	}
}

func typeDefinitionScalarUnderlyingDisplayable(typeSpec *ast.TypeSpec) bool {
	if typeSpec == nil || typeSpec.Name == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.info != nil {
		if obj, ok := typeInfo.info.Defs[typeSpec.Name].(*types.TypeName); ok {
			if named, ok := types.Unalias(obj.Type()).(*types.Named); ok {
				if basic, ok := types.Unalias(named.Underlying()).(*types.Basic); ok {
					return basic.Info()&(types.IsBoolean|types.IsInteger|types.IsFloat|types.IsString) != 0
				}
			}
		}
	}
	return false
}

func typeDefinitionUnderlyingNamedDisplayable(typeSpec *ast.TypeSpec) bool {
	named, ok := typeDefinitionUnderlyingNamedType(typeSpec)
	if !ok {
		return false
	}
	if typeSpec.TypeParams != nil {
		return false
	}
	if typeArgs := named.TypeArgs(); typeArgs != nil && typeArgs.Len() > 0 {
		return false
	}
	if !namedTypeHasGeneratedDisplay(named) {
		return false
	}
	if _, ok := types.Unalias(named.Underlying()).(*types.Array); ok {
		return false
	}
	return true
}

func typeDefinitionUnderlyingNamedType(typeSpec *ast.TypeSpec) (*types.Named, bool) {
	if typeSpec == nil || typeSpec.Name == nil {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	typ := typeInfo.GetType(typeSpec.Type)
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil {
		return nil, false
	}
	return named, true
}

func namedTypeHasGeneratedDisplay(named *types.Named) bool {
	if named == nil || named.Obj() == nil {
		return false
	}
	if !namedTypeRustDisplayImplAvailable(named) {
		return false
	}
	if typeHasGoStringMethod(named) || typeHasGoStringMethod(types.NewPointer(named)) {
		return true
	}
	switch underlying := types.Unalias(named.Underlying()).(type) {
	case *types.Basic:
		return goTypeBasicDisplayable(underlying)
	case *types.Struct:
		return true
	case *types.Slice:
		_, ok := sequenceDisplayFormatter(underlying.Elem())
		return ok
	case *types.Array:
		_, ok := sequenceDisplayFormatter(underlying.Elem())
		return ok
	case *types.Map:
		return mapKeyTypeDisplayable(underlying.Key()) && mapValueTypeDisplayable(underlying.Elem())
	default:
		return false
	}
}

func namedTypeRustDisplayImplAvailable(named *types.Named) bool {
	if named == nil || named.Obj() == nil {
		return false
	}
	obj := named.Obj()
	if obj.Pkg() == nil {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil && obj.Pkg() == typeInfo.pkg {
		return true
	}
	pkgPath := obj.Pkg().Path()
	return isStdlibPackage(pkgPath) || isSourceMappedPackagePath(pkgPath)
}

func typeDefinitionUnderlyingNamedArray(typeSpec *ast.TypeSpec) (*types.Named, bool) {
	if typeSpec == nil || typeSpec.Name == nil {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	typ := typeInfo.GetType(typeSpec.Type)
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil {
		return nil, false
	}
	if _, ok := types.Unalias(named.Underlying()).(*types.Array); !ok {
		return nil, false
	}
	return named, true
}

func typeDefinitionArrayDisplayFormatter(typeSpec *ast.TypeSpec) (string, bool) {
	if typeSpec == nil || typeSpec.Name == nil {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return "", false
	}
	obj, ok := typeInfo.info.Defs[typeSpec.Name].(*types.TypeName)
	if !ok {
		return "", false
	}
	named, ok := types.Unalias(obj.Type()).(*types.Named)
	if !ok {
		return "", false
	}
	switch seq := types.Unalias(named.Underlying()).(type) {
	case *types.Slice:
		return sequenceDisplayFormatter(seq.Elem())
	case *types.Array:
		return sequenceDisplayFormatter(seq.Elem())
	default:
		return "", false
	}
}

func typeDefinitionCustomDefaultValue(typeSpec *ast.TypeSpec) (string, bool) {
	if typeSpec == nil || typeSpec.Name == nil {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return "", false
	}
	obj, ok := typeInfo.info.Defs[typeSpec.Name].(*types.TypeName)
	if !ok {
		return "", false
	}
	named, ok := types.Unalias(obj.Type()).(*types.Named)
	if !ok {
		return "", false
	}
	if _, ok := types.Unalias(named.Underlying()).(*types.Array); !ok {
		return "", false
	}
	return zeroValueForTypesType(named), true
}

func sequenceDisplayFormatter(elem types.Type) (string, bool) {
	if elem == nil {
		return "", false
	}
	elem = types.Unalias(elem)
	if ptr, ok := elem.Underlying().(*types.Pointer); ok {
		if typeHasGoStringMethod(ptr.Elem()) ||
			typeHasGoStringMethod(types.NewPointer(ptr.Elem())) ||
			pointerElementHasGeneratedStructDisplay(ptr.Elem()) {
			return "format_slice_wrapped", true
		}
		return "", false
	}
	if intf, ok := elem.Underlying().(*types.Interface); ok {
		if intf.NumMethods() > 0 {
			return "format_slice_wrapped_stringer", true
		}
		return "", false
	}
	if goTypeBasicDisplayable(elem) || typeHasGoStringMethod(elem) || typeHasGoStringMethod(types.NewPointer(elem)) {
		return "format_slice", true
	}
	return "", false
}

func pointerElementHasGeneratedStructDisplay(elem types.Type) bool {
	named, ok := types.Unalias(elem).(*types.Named)
	if !ok || !namedTypeRustDisplayImplAvailable(named) {
		return false
	}
	_, ok = types.Unalias(named.Underlying()).(*types.Struct)
	return ok
}

func typeDefinitionMapUnderlyingDisplayable(typeSpec *ast.TypeSpec) bool {
	if typeSpec == nil || typeSpec.Name == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	obj, ok := typeInfo.info.Defs[typeSpec.Name].(*types.TypeName)
	if !ok {
		return false
	}
	named, ok := types.Unalias(obj.Type()).(*types.Named)
	if !ok {
		return false
	}
	mapType, ok := types.Unalias(named.Underlying()).(*types.Map)
	if !ok {
		return false
	}
	return mapKeyTypeDisplayable(mapType.Key()) && mapValueTypeDisplayable(mapType.Elem())
}

func mapKeyTypeDisplayable(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if _, ok := types.Unalias(typ).Underlying().(*types.Pointer); ok {
		return true
	}
	if _, ok := types.Unalias(typ).Underlying().(*types.Interface); ok {
		return true
	}
	return goTypeBasicDisplayable(typ)
}

func mapValueTypeDisplayable(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if intf, ok := types.Unalias(typ).Underlying().(*types.Interface); ok {
		return intf.NumMethods() > 0
	}
	return goTypeBasicDisplayable(typ)
}

func goTypeBasicDisplayable(typ types.Type) bool {
	if typ == nil {
		return false
	}
	basic, ok := types.Unalias(typ).Underlying().(*types.Basic)
	if !ok {
		return false
	}
	return basic.Info()&(types.IsBoolean|types.IsInteger|types.IsFloat|types.IsString) != 0
}

func registerStructDef(name string, structType *ast.StructType) {
	if name == "" || structType == nil {
		return
	}
	structDef := &StructDef{
		Fields:        make(map[string]string),
		FieldTypes:    make(map[string]ast.Expr),
		FieldTags:     make(map[string]string),
		FieldOrder:    []string{},
		EmbeddedTypes: []string{},
		ASTType:       structType,
	}
	for _, field := range structType.Fields.List {
		if len(field.Names) > 0 {
			tag := jsonStructTagFromField(field)
			for _, name := range field.Names {
				structDef.Fields[name.Name] = "regular"
				structDef.FieldTypes[name.Name] = field.Type
				structDef.FieldTags[name.Name] = tag
				structDef.FieldOrder = append(structDef.FieldOrder, name.Name)
			}
		} else {
			typeName := getEmbeddedFieldName(field.Type)
			structDef.EmbeddedTypes = append(structDef.EmbeddedTypes, typeName)
			if structFieldEmbedsGoError(field) {
				structDef.EmbedsError = true
				RegisterErrorImplType(name)
			}
		}
	}
	structDefs[name] = structDef
}

func registerStructTypeSpecDef(typeSpec *ast.TypeSpec, structType *ast.StructType) {
	if typeSpec == nil {
		return
	}
	registerStructDef(typeSpec.Name.Name, structType)
	generics := rustTypeGenericsForStructTypeSpec(typeSpec, structType)
	setStructDefPhantomTypeParams(typeSpec.Name.Name, generics.Phantom)
}

func setStructDefPhantomTypeParams(name string, phantomTypeParams []string) {
	if len(phantomTypeParams) == 0 {
		return
	}
	if def, ok := structDefs[name]; ok && def != nil {
		def.PhantomTypeParams = append([]string(nil), phantomTypeParams...)
	}
}

func structFieldEmbedsGoError(field *ast.Field) bool {
	if field == nil || len(field.Names) > 0 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return isGoErrorType(typeInfo.GetType(field.Type))
}

func writeScalarTypeDefinitionPartialEq(out *strings.Builder, typeName string) {
	out.WriteString("\nimpl PartialEq for ")
	out.WriteString(RustTypeNameForUse(typeName))
	out.WriteString(" {\n")
	out.WriteString("    fn eq(&self, other: &Self) -> bool {\n")
	out.WriteString("        let __left = { self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().cloned() };\n")
	out.WriteString("        let __right = { other.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().cloned() };\n")
	out.WriteString("        __left == __right\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func writeScalarTypeDefinitionNumericOps(out *strings.Builder, typeName string, rustType string, underlying string) {
	rustTypeName := RustTypeNameForUse(typeName)
	writeScalarTypeDefinitionPartialOrd(out, rustTypeName, rustType)
	writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "Add", "add", "+", true)
	writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "Sub", "sub", "-", true)
	writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "Mul", "mul", "*", true)
	writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "Div", "div", "/", true)
	if definedUnderlyingSupportsUnaryMinus(underlying) {
		writeScalarTypeDefinitionUnaryOp(out, rustTypeName, "Neg", "neg", "-")
	}
	if isBitwiseDefinedUnderlying(underlying) {
		writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "Rem", "rem", "%", true)
		writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "BitAnd", "bitand", "&", true)
		writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "BitOr", "bitor", "|", true)
		writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "BitXor", "bitxor", "^", true)
		writeScalarTypeDefinitionUnaryOp(out, rustTypeName, "Not", "not", "!")
		writeScalarTypeDefinitionShiftOps(out, rustTypeName)
	}
}

func definedUnderlyingSupportsUnaryMinus(underlying string) bool {
	switch underlying {
	case "int", "int8", "int16", "int32", "int64", "rune", "float32", "float64":
		return true
	default:
		return false
	}
}

func writeScalarTypeDefinitionPartialOrd(out *strings.Builder, rustTypeName string, rustType string) {
	out.WriteString("\nimpl PartialEq<")
	out.WriteString(rustType)
	out.WriteString("> for ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	out.WriteString("    fn eq(&self, other: &")
	out.WriteString(rustType)
	out.WriteString(") -> bool {\n")
	out.WriteString("        *self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap() == *other\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")

	out.WriteString("\nimpl PartialOrd for ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	out.WriteString("    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {\n")
	out.WriteString("        let __left = { self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().cloned() };\n")
	out.WriteString("        let __right = { other.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().cloned() };\n")
	out.WriteString("        __left.partial_cmp(&__right)\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")

	out.WriteString("\nimpl PartialOrd<")
	out.WriteString(rustType)
	out.WriteString("> for ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	out.WriteString("    fn partial_cmp(&self, other: &")
	out.WriteString(rustType)
	out.WriteString(") -> Option<std::cmp::Ordering> {\n")
	out.WriteString("        self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap().partial_cmp(other)\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")

	out.WriteString("\nimpl PartialEq<")
	out.WriteString(rustTypeName)
	out.WriteString("> for ")
	out.WriteString(rustType)
	out.WriteString(" {\n")
	out.WriteString("    fn eq(&self, other: &")
	out.WriteString(rustTypeName)
	out.WriteString(") -> bool {\n")
	out.WriteString("        *self == *other.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")

	out.WriteString("\nimpl PartialOrd<")
	out.WriteString(rustTypeName)
	out.WriteString("> for ")
	out.WriteString(rustType)
	out.WriteString(" {\n")
	out.WriteString("    fn partial_cmp(&self, other: &")
	out.WriteString(rustTypeName)
	out.WriteString(") -> Option<std::cmp::Ordering> {\n")
	out.WriteString("        self.partial_cmp(other.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func writeScalarTypeDefinitionOrd(out *strings.Builder, typeName string) {
	rustTypeName := RustTypeNameForUse(typeName)

	out.WriteString("\nimpl Eq for ")
	out.WriteString(rustTypeName)
	out.WriteString(" {}\n")

	out.WriteString("\nimpl Ord for ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	out.WriteString("    fn cmp(&self, other: &Self) -> std::cmp::Ordering {\n")
	out.WriteString("        let __left = { self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().cloned() };\n")
	out.WriteString("        let __right = { other.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().cloned() };\n")
	out.WriteString("        __left.cmp(&__right)\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func writeScalarTypeDefinitionBinaryOp(out *strings.Builder, rustTypeName string, rustType string, traitName string, methodName string, op string, sameTypeOutput bool) {
	out.WriteString("\nimpl std::ops::")
	out.WriteString(traitName)
	out.WriteString(" for ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	out.WriteString("    type Output = ")
	if sameTypeOutput {
		out.WriteString(rustTypeName)
	} else {
		out.WriteString(rustType)
	}
	out.WriteString(";\n")
	out.WriteString("    fn ")
	out.WriteString(methodName)
	out.WriteString("(self, other: Self) -> ")
	if sameTypeOutput {
		out.WriteString(rustTypeName)
	} else {
		out.WriteString(rustType)
	}
	out.WriteString(" {\n")
	out.WriteString("        ")
	if sameTypeOutput {
		out.WriteString(rustTypeName)
		out.WriteString("(")
		WriteWrapperPrefix(out)
	}
	out.WriteString("*self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap() ")
	out.WriteString(op)
	out.WriteString(" *other.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	if sameTypeOutput {
		WriteWrapperSuffix(out)
		out.WriteString(")")
	}
	out.WriteString("\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")

	out.WriteString("\nimpl std::ops::")
	out.WriteString(traitName)
	out.WriteString("<")
	out.WriteString(rustType)
	out.WriteString("> for ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	out.WriteString("    type Output = ")
	if sameTypeOutput {
		out.WriteString(rustTypeName)
	} else {
		out.WriteString(rustType)
	}
	out.WriteString(";\n")
	out.WriteString("    fn ")
	out.WriteString(methodName)
	out.WriteString("(self, other: ")
	out.WriteString(rustType)
	out.WriteString(") -> ")
	if sameTypeOutput {
		out.WriteString(rustTypeName)
	} else {
		out.WriteString(rustType)
	}
	out.WriteString(" {\n")
	out.WriteString("        ")
	if sameTypeOutput {
		out.WriteString(rustTypeName)
		out.WriteString("(")
		WriteWrapperPrefix(out)
	}
	out.WriteString("*self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap() ")
	out.WriteString(op)
	out.WriteString(" other")
	if sameTypeOutput {
		WriteWrapperSuffix(out)
		out.WriteString(")")
	}
	out.WriteString("\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")

	out.WriteString("\nimpl std::ops::")
	out.WriteString(traitName)
	out.WriteString("<")
	out.WriteString(rustTypeName)
	out.WriteString("> for ")
	out.WriteString(rustType)
	out.WriteString(" {\n")
	out.WriteString("    type Output = ")
	if sameTypeOutput {
		out.WriteString(rustTypeName)
	} else {
		out.WriteString(rustType)
	}
	out.WriteString(";\n")
	out.WriteString("    fn ")
	out.WriteString(methodName)
	out.WriteString("(self, other: ")
	out.WriteString(rustTypeName)
	out.WriteString(") -> ")
	if sameTypeOutput {
		out.WriteString(rustTypeName)
	} else {
		out.WriteString(rustType)
	}
	out.WriteString(" {\n")
	out.WriteString("        ")
	if sameTypeOutput {
		out.WriteString(rustTypeName)
		out.WriteString("(")
		WriteWrapperPrefix(out)
	}
	out.WriteString("self ")
	out.WriteString(op)
	out.WriteString(" *other.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	if sameTypeOutput {
		WriteWrapperSuffix(out)
		out.WriteString(")")
	}
	out.WriteString("\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func writeScalarTypeDefinitionUnaryOp(out *strings.Builder, rustTypeName string, traitName string, methodName string, op string) {
	out.WriteString("\nimpl std::ops::")
	out.WriteString(traitName)
	out.WriteString(" for ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	out.WriteString("    type Output = ")
	out.WriteString(rustTypeName)
	out.WriteString(";\n")
	out.WriteString("    fn ")
	out.WriteString(methodName)
	out.WriteString("(self) -> ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	out.WriteString("        ")
	out.WriteString(rustTypeName)
	out.WriteString("(")
	WriteWrapperPrefix(out)
	out.WriteString(op)
	out.WriteString("*self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	WriteWrapperSuffix(out)
	out.WriteString(")\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func writeScalarTypeDefinitionShiftOps(out *strings.Builder, rustTypeName string) {
	for _, spec := range []struct {
		traitName  string
		methodName string
		op         string
	}{
		{"Shl", "shl", "<<"},
		{"Shr", "shr", ">>"},
	} {
		writeScalarTypeDefinitionShiftOp(out, rustTypeName, rustTypeName, spec.traitName, spec.methodName, spec.op, true)
		for _, rhsType := range []string{"i32", "i8", "i16", "i64", "u32", "u8", "u16", "u64", "usize"} {
			writeScalarTypeDefinitionShiftOp(out, rustTypeName, rhsType, spec.traitName, spec.methodName, spec.op, false)
		}
	}
}

func writeScalarTypeDefinitionShiftOp(out *strings.Builder, rustTypeName string, rhsType string, traitName string, methodName string, op string, rhsWrapped bool) {
	out.WriteString("\nimpl std::ops::")
	out.WriteString(traitName)
	if !rhsWrapped {
		out.WriteString("<")
		out.WriteString(rhsType)
		out.WriteString(">")
	}
	out.WriteString(" for ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	out.WriteString("    type Output = ")
	out.WriteString(rustTypeName)
	out.WriteString(";\n")
	out.WriteString("    fn ")
	out.WriteString(methodName)
	out.WriteString("(self, other: ")
	out.WriteString(rhsType)
	out.WriteString(") -> ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	out.WriteString("        ")
	out.WriteString(rustTypeName)
	out.WriteString("(")
	WriteWrapperPrefix(out)
	out.WriteString("*self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap() ")
	out.WriteString(op)
	out.WriteString(" ")
	if rhsWrapped {
		out.WriteString("*other.0")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
	} else {
		out.WriteString("other")
	}
	WriteWrapperSuffix(out)
	out.WriteString(")\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func localConcreteTypeCanUsePartialEq(typeName string) bool {
	if comparableStructTypes[typeName] {
		return true
	}
	if underlying, ok := LookupTypeDefinition(typeName); ok {
		return isEqualityComparableDefinedUnderlying(underlying)
	}
	return false
}

// writeLocalInterfaceSupportImpl emits the auxiliary trait-method bodies
// (`__go_clone_box_<suffix>`, optional `__go_as_any`, `__go_eq_<suffix>`)
// inside an `impl <Trait> for <Concrete>` block. `ifaceName` is the Rust
// trait name as it appears in the impl signature (simple for the current
// package, qualified for cross-package interfaces); `ifaceType` is the
// go/types Interface used to decide whether `__go_as_any` must be redeclared
// here or is already inherited from a supertrait.
func writeLocalInterfaceSupportImpl(out *strings.Builder, ifaceName, typeName string, ifaceType *types.Interface) {
	TrackImport("Any")
	traitSnake := traitMethodSuffix(ifaceName)
	hasEmbedded := ifaceType != nil && interfaceTypeHasNamedEmbedded(ifaceType)
	if ifaceType == nil {
		hasEmbedded = localInterfaceHasEmbeddedInterfaces(ifaceName)
	}
	out.WriteString("    fn __go_clone_box_")
	out.WriteString(traitSnake)
	out.WriteString("(&self) -> ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString(" {\n")
	out.WriteString("        Box::new(self.clone()) as ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString("\n")
	out.WriteString("    }\n")
	if !hasEmbedded {
		out.WriteString("    fn __go_as_any(&self) -> &dyn Any {\n")
		out.WriteString("        self\n")
		out.WriteString("    }\n")
	}
	out.WriteString("    fn __go_eq_")
	out.WriteString(traitSnake)
	out.WriteString("(&self, other: ")
	out.WriteString(rustLocalInterfaceParamBare(ifaceName))
	out.WriteString(") -> bool {\n")
	out.WriteString("        if let Some(__other) = other.__go_as_any().downcast_ref::<")
	out.WriteString(RustTypeNameForUse(typeName))
	out.WriteString(">() {\n")
	if localConcreteTypeCanUsePartialEq(typeName) {
		out.WriteString("            self == __other\n")
	} else if localInterfaceEqualityTypes[ifaceName] {
		out.WriteString("            panic!(\"interface comparison with uncomparable dynamic type\")\n")
	} else {
		out.WriteString("            false\n")
	}
	out.WriteString("        } else {\n")
	out.WriteString("            false\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

func TranspileConstDecl(out *strings.Builder, genDecl *ast.GenDecl) {
	transpileConstDeclWithCase(out, genDecl, true)
}

func transpileConstDeclWithCase(out *strings.Builder, genDecl *ast.GenDecl, toUpper bool) {
	// Track iota value and the last expression pattern for each position
	iotaValue := 0
	var lastExpressions []ast.Expr
	var lastType ast.Expr

	for specIndex, spec := range genDecl.Specs {
		if valueSpec, ok := spec.(*ast.ValueSpec); ok {
			// Set iota for this spec
			iotaValue = specIndex

			// Update lastExpressions if this spec has values
			if len(valueSpec.Values) > 0 {
				lastExpressions = valueSpec.Values
			}
			if valueSpec.Type != nil {
				lastType = valueSpec.Type
			}

			for i, name := range valueSpec.Names {
				// Skip blank identifier
				if name.Name == "_" {
					continue
				}
				var constExpr ast.Expr
				if len(valueSpec.Values) > i && valueSpec.Values[i] != nil {
					constExpr = valueSpec.Values[i]
				} else if len(lastExpressions) > i && lastExpressions[i] != nil {
					constExpr = lastExpressions[i]
				} else if len(lastExpressions) > 0 && lastExpressions[0] != nil {
					constExpr = lastExpressions[0]
				}
				if toUpper {
					if ast.IsExported(name.Name) {
						out.WriteString("pub ")
					} else {
						out.WriteString("pub(crate) ")
					}
				}
				out.WriteString("const ")
				var constName string
				var constType string
				var constTypeName string
				declaredConstType := false
				if valueSpec.Type != nil {
					constType = rustConstTypeForTypeExpr(valueSpec.Type)
					constTypeName, _ = constDeclaredNamedType(valueSpec.Type)
					declaredConstType = true
				} else if len(valueSpec.Values) == 0 && lastType != nil {
					constType = rustConstTypeForTypeExpr(lastType)
					constTypeName, _ = constDeclaredNamedType(lastType)
					declaredConstType = true
				} else if inferredType, ok := rustConstTypeForConstObject(name, constExpr); ok {
					constType = inferredType
				} else if len(valueSpec.Values) > i && valueSpec.Values[i] != nil {
					constType = inferConstType(valueSpec.Values[i])
				} else if len(lastExpressions) > i && lastExpressions[i] != nil {
					constType = inferConstType(lastExpressions[i])
				} else {
					constType = "i32"
				}
				if constType == "&'static str" && constExpr != nil && constStringNeedsByteSlice(constExpr) {
					constType = "&'static [u8]"
				}
				if toUpper {
					constName = rustConstName(name.Name)
					registerPackageConstant(name.Name, constType)
					registerPackageConstantTypeName(name.Name, constTypeName)
				} else {
					// Keep original name for local constants
					constName = name.Name
					// Track local constants with their actual type
					localConstants[name.Name] = constType
				}
				out.WriteString(constName)
				out.WriteString(": ")

				// Determine type - constants should not be wrapped
				out.WriteString(constType)

				out.WriteString(" = ")

				// Handle value
				if len(valueSpec.Values) > i && valueSpec.Values[i] != nil {
					// Replace iota with actual value
					if !writeExternalNamedIntegerConstValue(out, name) {
						writeConstExprForRustType(out, valueSpec.Values[i], iotaValue, constType, declaredConstType)
					}
				} else if len(lastExpressions) > i && lastExpressions[i] != nil {
					// Use the corresponding expression from lastExpressions for this position
					if !writeExternalNamedIntegerConstValue(out, name) {
						writeConstExprForRustType(out, lastExpressions[i], iotaValue, constType, declaredConstType)
					}
				} else if len(lastExpressions) > 0 && lastExpressions[0] != nil {
					// If we don't have an expression for this position, use the first one
					if !writeExternalNamedIntegerConstValue(out, name) {
						writeConstExprForRustType(out, lastExpressions[0], iotaValue, constType, declaredConstType)
					}
				} else {
					// No previous expression pattern, just use iota value
					out.WriteString(fmt.Sprintf("%d", iotaValue))
				}

				out.WriteString(";\n")
			}
		}
	}
}

func writeConstExprForRustType(out *strings.Builder, expr ast.Expr, iotaValue int, rustType string, declaredConstType bool) {
	if rustType == "&'static [u8]" && writeConstByteSliceLiteralValue(out, expr) {
		return
	}
	if rustType == "&'static str" && writeConstStringLiteralValue(out, expr) {
		return
	}
	if writeOutOfRangeGoIntConstExprForRustType(out, expr, iotaValue, rustType) {
		return
	}
	if writeIntegerTypedFloatConstLiteral(out, expr, rustType) {
		return
	}
	if writeConstLenCallForRustType(out, expr, iotaValue, rustType) {
		return
	}
	if writeConstSelectorForIntegerRustType(out, expr, iotaValue, rustType) {
		return
	}
	if declaredConstType && writeConstBinaryExprForDeclaredIntegerRustType(out, expr, rustType) {
		return
	}
	if !declaredConstType && writeConstBinaryExprForInferredIntegerRustType(out, expr, rustType) {
		return
	}
	TranspileConstExpr(out, expr, iotaValue)
}

func writeConstBinaryExprForDeclaredIntegerRustType(out *strings.Builder, expr ast.Expr, rustType string) bool {
	if _, ok := rustIntegerCastTypeFromRustType(rustType); !ok || !isConstantExpression(expr) {
		return false
	}
	if constExprContainsIota(expr) {
		return false
	}
	if _, ok := unwrapParens(expr).(*ast.BinaryExpr); !ok {
		return false
	}
	if !writeDeclaredConstExpressionWithRustIntegerOperands(out, expr, rustType) {
		return false
	}
	return true
}

func writeConstBinaryExprForInferredIntegerRustType(out *strings.Builder, expr ast.Expr, rustType string) bool {
	if _, ok := rustIntegerCastTypeFromRustType(rustType); !ok || !isConstantExpression(expr) {
		return false
	}
	if constExprContainsIota(expr) {
		return false
	}
	if constExprContainsTypeConversion(expr) {
		return false
	}
	binary, ok := unwrapParens(expr).(*ast.BinaryExpr)
	if !ok || binary.Op == token.SHL || binary.Op == token.SHR {
		return false
	}
	return writeConstExpressionWithRustIntegerOperandsForExpected(out, expr, rustType)
}

func constExprContainsTypeConversion(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	found := false
	ast.Inspect(expr, func(node ast.Node) bool {
		if found {
			return false
		}
		call, ok := node.(*ast.CallExpr)
		if ok && typeInfo.IsTypeConversion(call) {
			found = true
			return false
		}
		return true
	})
	return found
}

func constExprContainsIota(expr ast.Expr) bool {
	found := false
	ast.Inspect(expr, func(node ast.Node) bool {
		if found {
			return false
		}
		ident, ok := node.(*ast.Ident)
		if ok && ident.Name == "iota" {
			found = true
			return false
		}
		return true
	})
	return found
}

func writeOutOfRangeGoIntConstExprForRustType(out *strings.Builder, expr ast.Expr, iotaValue int, rustType string) bool {
	if rustType != "i32" || expr == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	basic, ok := types.Unalias(typeInfo.GetType(expr)).(*types.Basic)
	if !ok || basic.Kind() != types.Int {
		return false
	}
	value, ok := constExpressionValue(expr)
	if !ok || goIntConstFitsRustInt(value) {
		return false
	}
	if literal, ok := goIntBoundaryLiteralForRustModel(value); ok {
		out.WriteString(literal)
		return true
	}
	TranspileConstExpr(out, expr, iotaValue)
	return true
}

func writeIntegerTypedFloatConstLiteral(out *strings.Builder, expr ast.Expr, rustType string) bool {
	if _, ok := rustIntegerCastTypeFromRustType(rustType); !ok {
		return false
	}
	lit, ok := expr.(*ast.BasicLit)
	if !ok || lit.Kind != token.FLOAT {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		out.WriteString(`/* ERROR: Type information required for integer const literal */ unimplemented!("type info required for integer const literal")`)
		return true
	}
	tv, ok := typeInfo.info.Types[lit]
	if !ok || tv.Value == nil {
		out.WriteString(`/* ERROR: Type information required for integer const literal */ unimplemented!("type info required for integer const literal")`)
		return true
	}
	if value, exact := constant.Int64Val(tv.Value); exact {
		out.WriteString(strconv.FormatInt(value, 10))
		return true
	}
	if value, exact := constant.Uint64Val(tv.Value); exact {
		out.WriteString(strconv.FormatUint(value, 10))
		return true
	}
	return false
}

func writeConstLenCallForRustType(out *strings.Builder, expr ast.Expr, iotaValue int, rustType string) bool {
	castType, ok := rustIntegerCastTypeFromRustType(rustType)
	if !ok {
		return false
	}
	call, ok := expr.(*ast.CallExpr)
	if !ok || !constBuiltinCallName(call, "len") || len(call.Args) != 1 {
		return false
	}
	if length, ok := constArrayLenValue(call); ok {
		out.WriteString(strconv.FormatInt(length, 10))
		out.WriteString(" as ")
		out.WriteString(castType)
		return true
	}
	if !constExprIsStringValue(call.Args[0]) {
		return false
	}
	TranspileConstExpr(out, call.Args[0], iotaValue)
	out.WriteString(".len() as ")
	out.WriteString(castType)
	return true
}

func constArrayLenValue(call *ast.CallExpr) (int64, bool) {
	if !constBuiltinCallName(call, "len") || len(call.Args) != 1 {
		return 0, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return 0, false
	}
	typ := typeInfo.GetType(call.Args[0])
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		typ = ptr.Elem()
	}
	array, ok := types.Unalias(typ).(*types.Array)
	if !ok {
		return 0, false
	}
	return array.Len(), true
}

func writeConstArrayLenCall(out *strings.Builder, call *ast.CallExpr) bool {
	length, ok := constArrayLenValue(call)
	if !ok {
		return false
	}
	out.WriteString(strconv.FormatInt(length, 10))
	return true
}

func writeConstSelectorForIntegerRustType(out *strings.Builder, expr ast.Expr, iotaValue int, rustType string) bool {
	castType, ok := rustIntegerCastTypeFromRustType(rustType)
	if !ok {
		return false
	}
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	if hasStdlibSelectorMapping(sel) {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		out.WriteString(`/* ERROR: Type information required for const selector */ unimplemented!("type info required for const selector")`)
		return true
	}
	obj, ok := typeInfo.GetObject(sel.Sel).(*types.Const)
	if !ok || obj.Val() == nil || obj.Val().Kind() != constant.Int {
		return false
	}
	basic, ok := types.Unalias(obj.Type()).(*types.Basic)
	if !ok || !isIntegerBasicKind(basic.Kind()) {
		return false
	}
	TranspileConstExpr(out, expr, iotaValue)
	out.WriteString(" as ")
	out.WriteString(castType)
	return true
}

func constBuiltinCallName(call *ast.CallExpr, name string) bool {
	if call == nil {
		return false
	}
	ident, ok := call.Fun.(*ast.Ident)
	if !ok || ident.Name != name {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	builtin, ok := typeInfo.GetObject(ident).(*types.Builtin)
	return ok && builtin.Name() == name
}

func constExprIsStringValue(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || expr == nil {
		return false
	}
	if tv, ok := typeInfo.info.Types[expr]; ok && tv.Value != nil {
		return tv.Value.Kind() == constant.String
	}
	switch e := expr.(type) {
	case *ast.Ident:
		if obj, ok := typeInfo.GetObject(e).(*types.Const); ok && obj.Val() != nil {
			return obj.Val().Kind() == constant.String
		}
	case *ast.SelectorExpr:
		if obj, ok := typeInfo.GetObject(e.Sel).(*types.Const); ok && obj.Val() != nil {
			return obj.Val().Kind() == constant.String
		}
	case *ast.ParenExpr:
		return constExprIsStringValue(e.X)
	}
	return false
}

func constDeclaredNamedType(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok {
		return "", false
	}
	if _, isTypeDef := LookupTypeDefinition(ident.Name); !isTypeDef {
		return "", false
	}
	return ident.Name, true
}

func rustConstTypeForConstObject(name *ast.Ident, expr ast.Expr) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || name == nil {
		return "", false
	}
	obj, ok := typeInfo.GetObject(name).(*types.Const)
	if !ok || obj.Type() == nil {
		return "", false
	}
	if constStringValueNeedsByteSlice(obj.Val()) {
		return "&'static [u8]", true
	}
	if basic, ok := types.Unalias(obj.Type()).(*types.Basic); ok && (basic.Kind() == types.UntypedInt || basic.Kind() == types.UntypedRune) {
		if rustType, ok := rustConstTypeForUntypedIntegerValue(expr, obj.Val()); ok {
			return rustType, true
		}
	}
	return rustConstTypeForGoTypesType(obj.Type())
}

func evalIntConstExpr(expr ast.Expr) (int64, bool) {
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
		if tv, ok := typeInfo.info.Types[expr]; ok && tv.Value != nil {
			return constant.Int64Val(tv.Value)
		}
	}
	switch e := expr.(type) {
	case *ast.BasicLit:
		if e.Kind != token.INT {
			return 0, false
		}
		v, err := strconv.ParseInt(e.Value, 0, 64)
		if err != nil {
			return 0, false
		}
		return v, true
	case *ast.BinaryExpr:
		l, lok := evalIntConstExpr(e.X)
		r, rok := evalIntConstExpr(e.Y)
		if !lok || !rok {
			return 0, false
		}
		switch e.Op {
		case token.ADD:
			return l + r, true
		case token.SUB:
			return l - r, true
		case token.MUL:
			return l * r, true
		case token.QUO:
			if r == 0 {
				return 0, false
			}
			return l / r, true
		case token.REM:
			if r == 0 {
				return 0, false
			}
			return l % r, true
		case token.SHL:
			if r < 0 || r >= 63 {
				return 0, false
			}
			return l << uint(r), true
		case token.SHR:
			if r < 0 || r >= 63 {
				return 0, false
			}
			return l >> uint(r), true
		case token.AND:
			return l & r, true
		case token.OR:
			return l | r, true
		case token.XOR:
			return l ^ r, true
		}
	case *ast.UnaryExpr:
		v, ok := evalIntConstExpr(e.X)
		if !ok {
			return 0, false
		}
		switch e.Op {
		case token.ADD:
			return v, true
		case token.SUB:
			return -v, true
		case token.XOR:
			return ^v, true
		}
	case *ast.ParenExpr:
		return evalIntConstExpr(e.X)
	case *ast.CallExpr:
		if len(e.Args) != 1 {
			return 0, false
		}
		typeInfo := GetTypeInfo()
		if typeInfo == nil || !typeInfo.IsTypeConversion(e) {
			return 0, false
		}
		return evalIntConstExpr(e.Args[0])
	}
	return 0, false
}

func inferConstType(expr ast.Expr) string {
	switch e := expr.(type) {
	case *ast.BasicLit:
		switch e.Kind {
		case token.INT:
			// Check if the value might overflow i32
			if val, err := strconv.ParseInt(e.Value, 0, 64); err == nil {
				if val > math.MaxInt32 || val < math.MinInt32 {
					return "i64"
				}
			} else if val, err := strconv.ParseUint(e.Value, 0, 64); err == nil {
				if val > math.MaxInt32 {
					return "u64"
				}
			}
			return "i32"
		case token.FLOAT:
			return "f64"
		case token.STRING:
			return "&'static str"
		}
	case *ast.Ident:
		if e.Name == "true" || e.Name == "false" {
			return "bool"
		}
		// Check if it's a known constant
		if constType, exists := localConstants[e.Name]; exists {
			return constType
		}
	case *ast.BinaryExpr:
		// For binary expressions, check the type of operands
		leftType := inferConstType(e.X)
		if leftType == "&'static str" {
			return "&'static str"
		}
		rightType := inferConstType(e.Y)
		if rightType == "&'static str" {
			return "&'static str"
		}
		// For bit shift operations that might overflow, use i64
		if e.Op == token.SHL {
			// If we can constant-fold and the result fits in i32, prefer i32.
			if v, ok := evalIntConstExpr(e); ok {
				if v >= int64(math.MinInt32) && v <= int64(math.MaxInt32) {
					return "i32"
				}
			}
			return "i64"
		}
		// If either operand is i64, result is i64
		if leftType == "i64" || rightType == "i64" {
			return "i64"
		}
		// Default to i32 for other numeric operations
		return "i32"
	}
	return "i32" // default
}

// Helper function to check if an expression is a string constant
func isStringConstExpr(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.BasicLit:
		return e.Kind == token.STRING
	case *ast.Ident:
		if constType, exists := localConstants[e.Name]; exists {
			return constType == "&'static str"
		}
		typeInfo := GetTypeInfo()
		if typeInfo != nil {
			if obj, ok := typeInfo.GetObject(e).(*types.Const); ok {
				return obj.Val() != nil && obj.Val().Kind() == constant.String
			}
		}
		return false
	case *ast.SelectorExpr:
		typeInfo := GetTypeInfo()
		if typeInfo != nil {
			if obj, ok := typeInfo.GetObject(e.Sel).(*types.Const); ok {
				return obj.Val() != nil && obj.Val().Kind() == constant.String
			}
		}
		return false
	case *ast.BinaryExpr:
		// String concatenation
		if e.Op == token.ADD {
			return isStringConstExpr(e.X) && isStringConstExpr(e.Y)
		}
		return false
	}
	return false
}

// Helper function to fully evaluate a const string expression including identifiers
func evaluateConstStringExpr(expr ast.Expr) string {
	switch e := expr.(type) {
	case *ast.BasicLit:
		if e.Kind == token.STRING {
			if value, err := strconv.Unquote(e.Value); err == nil {
				return value
			}
		}
	case *ast.Ident:
		// Look up the value of the constant using TypeInfo
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.info != nil {
			if obj, ok := typeInfo.info.Uses[e]; ok {
				if constObj, ok := obj.(*types.Const); ok {
					if constObj.Val() != nil {
						// Extract the string value from the constant
						return constant.StringVal(constObj.Val())
					}
				}
			}
		}
		// Type info not available or not a constant
		return ""
	case *ast.SelectorExpr:
		typeInfo := GetTypeInfo()
		if typeInfo != nil {
			if obj, ok := typeInfo.GetObject(e.Sel).(*types.Const); ok && obj.Val() != nil {
				return constant.StringVal(obj.Val())
			}
		}
		return ""
	case *ast.BinaryExpr:
		if e.Op == token.ADD {
			left := evaluateConstStringExpr(e.X)
			right := evaluateConstStringExpr(e.Y)
			if left != "" || right != "" {
				return left + right
			}
		}
	}
	return ""
}

func TranspileConstExpr(out *strings.Builder, expr ast.Expr, iotaValue int) {
	switch e := expr.(type) {
	case *ast.BasicLit:
		if e.Kind == token.STRING {
			// For const strings, use &str instead of String
			out.WriteString(RustStringLiteral(e.Value))
		} else if e.Kind == token.CHAR {
			out.WriteString("(")
			out.WriteString(RustCharLiteral(e.Value))
			out.WriteString(" as i32)")
		} else if e.Kind == token.FLOAT {
			if value, ok := rustFloatLiteral(e); ok {
				out.WriteString(value)
			} else {
				out.WriteString(e.Value)
			}
		} else {
			out.WriteString(e.Value)
		}
	case *ast.Ident:
		if e.Name == "iota" {
			out.WriteString(fmt.Sprintf("%d", iotaValue))
		} else if e.Name == "true" || e.Name == "false" {
			// Boolean literals
			out.WriteString(e.Name)
		} else if _, exists := localConstants[e.Name]; exists {
			// Local constant - keep original name
			out.WriteString(e.Name)
		} else if isConstIdent(e) {
			out.WriteString(rustConstName(e.Name))
		} else if e.Name[0] >= 'a' && e.Name[0] <= 'z' {
			// Package-level constant reference - convert to uppercase
			out.WriteString(rustConstName(e.Name))
		} else {
			out.WriteString(e.Name)
		}
	case *ast.BinaryExpr:
		if writeConstStringEquality(out, e, iotaValue) {
			return
		}
		// Special handling for string concatenation in const context
		if e.Op == token.ADD && isStringConstExpr(e.X) && isStringConstExpr(e.Y) {
			// For string concatenation in const context, try to evaluate at compile time
			result := evaluateConstStringExpr(expr)
			if result != "" {
				// Successfully evaluated the entire expression
				out.WriteString(RustStringLiteral(strconv.Quote(result)))
			} else {
				// Fall back - this won't work for const but at least generates something
				out.WriteString("/* TODO: Complex string concatenation in const */ ")
				out.WriteString(`""`)
			}
		} else {
			// Handle binary expressions in const context
			if e.Op == token.SHL || e.Op == token.SHR {
				writeConstBinaryOperand(out, e.X, e.Op, false, func() {
					TranspileConstExpr(out, e.X, iotaValue)
				})
				out.WriteString(" ")
				out.WriteString(e.Op.String())
				out.WriteString(" ")
				writeConstBinaryOperand(out, e.Y, e.Op, true, func() {
					if !writeConstShiftCountValue(out, e.Y) {
						TranspileConstExpr(out, e.Y, iotaValue)
					}
				})
			} else {
				writeConstBinaryOperand(out, e.X, e.Op, false, func() {
					if _, isCall := e.X.(*ast.CallExpr); isCall || !writePrimitiveConstExpressionForBinaryPeer(out, e.X, e.Y, iotaValue) {
						TranspileConstExpr(out, e.X, iotaValue)
					}
				})
				out.WriteString(" ")
				out.WriteString(e.Op.String())
				out.WriteString(" ")
				writeConstBinaryOperand(out, e.Y, e.Op, true, func() {
					if _, isCall := e.Y.(*ast.CallExpr); isCall || !writePrimitiveConstExpressionForBinaryPeer(out, e.Y, e.X, iotaValue) {
						TranspileConstExpr(out, e.Y, iotaValue)
					}
				})
			}
		}
	case *ast.ParenExpr:
		out.WriteString("(")
		TranspileConstExpr(out, e.X, iotaValue)
		out.WriteString(")")
	case *ast.UnaryExpr:
		writeConstUnaryExpr(out, e, iotaValue)
	case *ast.CallExpr:
		if writeConstUnsafeTypeSizeCall(out, e) {
			return
		}
		if writeConstArrayLenCall(out, e) {
			return
		}
		if !writeConstTypeConversion(out, e, iotaValue) {
			TranspileExpression(out, expr)
		}
	default:
		// Fallback to regular expression transpilation
		TranspileExpression(out, expr)
	}
}

func writeConstStringEquality(out *strings.Builder, expr *ast.BinaryExpr, iotaValue int) bool {
	if expr == nil || expr.Op != token.EQL && expr.Op != token.NEQ {
		return false
	}
	if value, ok := constBoolValueForStubBackedStringSelectorEquality(expr); ok {
		out.WriteString(strconv.FormatBool(value))
		return true
	}
	leftPattern, leftIsPattern := constStringLiteral(expr.X)
	rightPattern, rightIsPattern := constStringLiteral(expr.Y)
	var subject ast.Expr
	pattern := ""
	if rightIsPattern && isStringConstExpr(expr.X) {
		subject = expr.X
		pattern = rightPattern
	} else if leftIsPattern && isStringConstExpr(expr.Y) {
		subject = expr.Y
		pattern = leftPattern
	} else {
		return false
	}
	if expr.Op == token.NEQ {
		out.WriteString("!")
	}
	NeedGoConstStrEq()
	out.WriteString("go_const_str_eq(")
	TranspileConstExpr(out, subject, iotaValue)
	out.WriteString(", ")
	out.WriteString(pattern)
	out.WriteString(")")
	return true
}

func constBoolValueForStubBackedStringSelectorEquality(expr *ast.BinaryExpr) (bool, bool) {
	if expr == nil || !constStringEqualityHasStubBackedSelector(expr.X) && !constStringEqualityHasStubBackedSelector(expr.Y) {
		return false, false
	}
	value, ok := constExpressionValue(expr)
	if !ok || value.Kind() != constant.Bool {
		return false, false
	}
	return constant.BoolVal(value), true
}

func constStringEqualityHasStubBackedSelector(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.SelectorExpr:
		typeInfo := GetTypeInfo()
		if typeInfo == nil {
			return false
		}
		obj, ok := typeInfo.GetObject(e.Sel).(*types.Const)
		if !ok || obj.Pkg() == nil || obj.Val() == nil || obj.Val().Kind() != constant.String {
			return false
		}
		return isStubBackedStdlibPackagePath(obj.Pkg().Path())
	case *ast.ParenExpr:
		return constStringEqualityHasStubBackedSelector(e.X)
	default:
		return false
	}
}

func writeConstBinaryOperand(out *strings.Builder, expr ast.Expr, parentOp token.Token, isRight bool, write func()) {
	if constBinaryOperandNeedsParens(expr, parentOp, isRight) {
		out.WriteString("(")
		write()
		out.WriteString(")")
		return
	}
	write()
}

func constBinaryOperandNeedsParens(expr ast.Expr, parentOp token.Token, isRight bool) bool {
	binary, ok := expr.(*ast.BinaryExpr)
	if !ok {
		return false
	}
	childPrec := rustConstBinaryPrecedence(binary.Op)
	parentPrec := rustConstBinaryPrecedence(parentOp)
	if childPrec == 0 || parentPrec == 0 {
		return false
	}
	if childPrec < parentPrec {
		return true
	}
	return isRight && childPrec == parentPrec
}

func rustConstBinaryPrecedence(op token.Token) int {
	switch op {
	case token.LOR:
		return 1
	case token.LAND:
		return 2
	case token.EQL, token.NEQ, token.LSS, token.LEQ, token.GTR, token.GEQ:
		return 3
	case token.OR:
		return 4
	case token.XOR:
		return 5
	case token.AND, token.AND_NOT:
		return 6
	case token.SHL, token.SHR:
		return 7
	case token.ADD, token.SUB:
		return 8
	case token.MUL, token.QUO, token.REM:
		return 9
	default:
		return 0
	}
}

func writeConstUnaryExpr(out *strings.Builder, expr *ast.UnaryExpr, iotaValue int) {
	switch expr.Op {
	case token.ADD:
		TranspileConstExpr(out, expr.X, iotaValue)
	case token.SUB:
		out.WriteString("-")
		TranspileConstExpr(out, expr.X, iotaValue)
	case token.NOT, token.XOR:
		out.WriteString("!")
		TranspileConstExpr(out, expr.X, iotaValue)
	default:
		TranspileExpression(out, expr)
	}
}

func writeConstUnsafeTypeSizeCall(out *strings.Builder, call *ast.CallExpr) bool {
	key, ok := stdlibCallKey(call.Fun)
	if !ok {
		return false
	}

	var goFunc string
	var rustFunc string
	switch key {
	case "unsafe.Sizeof":
		goFunc = "Sizeof"
		rustFunc = "size_of"
	case "unsafe.Alignof":
		goFunc = "Alignof"
		rustFunc = "align_of"
	case "unsafe.Offsetof":
		return writeConstUnsafeOffsetofCall(out, call)
	default:
		return false
	}

	if len(call.Args) == 0 {
		out.WriteString("/* ERROR: unsafe.")
		out.WriteString(goFunc)
		out.WriteString(" requires an argument */ unimplemented!()")
		return true
	}

	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("unimplemented!(\"type info required for unsafe.")
		out.WriteString(goFunc)
		out.WriteString("\")")
		return true
	}

	argType := typeInfo.GetType(call.Args[0])
	if argType == nil {
		out.WriteString("unimplemented!(\"type info required for unsafe.")
		out.WriteString(goFunc)
		out.WriteString("\")")
		return true
	}

	out.WriteString("std::mem::")
	out.WriteString(rustFunc)
	out.WriteString("::<")
	out.WriteString(goTypesTypeToRust(argType))
	out.WriteString(">()")
	return true
}

func writeConstUnsafeOffsetofCall(out *strings.Builder, call *ast.CallExpr) bool {
	if len(call.Args) == 0 {
		out.WriteString("/* ERROR: unsafe.Offsetof requires an argument */ unimplemented!()")
		return true
	}
	sel, ok := call.Args[0].(*ast.SelectorExpr)
	if !ok {
		out.WriteString("unimplemented!(\"unsafe.Offsetof requires selector\")")
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("unimplemented!(\"type info required for unsafe.Offsetof\")")
		return true
	}
	containerType := typeInfo.GetType(sel.X)
	if containerType == nil {
		out.WriteString("unimplemented!(\"type info required for unsafe.Offsetof\")")
		return true
	}
	if ptr, ok := types.Unalias(containerType).(*types.Pointer); ok {
		containerType = ptr.Elem()
	}
	containerRustType := goTypesTypeToRust(containerType)
	if containerRustType == "/* unknown struct */" {
		if rustType, ok := importedPackageGlobalTypePath(sel.X, typeInfo); ok {
			containerRustType = rustType
		}
	}
	out.WriteString("std::mem::offset_of!(")
	out.WriteString(containerRustType)
	out.WriteString(", ")
	out.WriteString(ToSnakeCase(sel.Sel.Name))
	out.WriteString(")")
	return true
}

func importedPackageGlobalTypePath(expr ast.Expr, typeInfo *TypeInfo) (string, bool) {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok || typeInfo == nil {
		return "", false
	}
	obj, ok := typeInfo.GetObject(sel.Sel).(*types.Var)
	if !ok || obj.Pkg() == nil {
		return "", false
	}
	if typeInfo.pkg != nil && obj.Pkg() == typeInfo.pkg {
		return "", false
	}
	return rustTypeNameForImportedPackagePath(obj.Pkg().Path(), obj.Name())
}

func writeConstTypeConversion(out *strings.Builder, call *ast.CallExpr, iotaValue int) bool {
	if call == nil || len(call.Args) != 1 {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsTypeConversion(call) {
		return false
	}
	rustType, ok := rustConstTypeForGoTypesType(typeInfo.GetType(call))
	if !ok {
		return false
	}
	out.WriteString("(")
	argRustType := constTypeConversionArgumentRustType(typeInfo, call.Args[0], rustType)
	if writeConstNumericConversionValueForRustType(out, call.Args[0], argRustType) {
		// Constant operands were emitted in the argument expression's typed integer width.
	} else if constTypeConversionArgNeedsParens(call.Args[0]) {
		out.WriteString("(")
		TranspileConstExpr(out, call.Args[0], iotaValue)
		out.WriteString(")")
	} else {
		TranspileConstExpr(out, call.Args[0], iotaValue)
	}
	out.WriteString(" as ")
	out.WriteString(rustType)
	out.WriteString(")")
	return true
}

func constTypeConversionArgumentRustType(typeInfo *TypeInfo, arg ast.Expr, conversionRustType string) string {
	if typeInfo == nil {
		return conversionRustType
	}
	if argRustType, ok := rustIntegerCastTypeForExpected(typeInfo.GetType(arg)); ok && rustIntegerTypeWidth(argRustType) > 0 {
		return argRustType
	}
	return conversionRustType
}

func constTypeConversionArgNeedsParens(arg ast.Expr) bool {
	switch arg.(type) {
	case *ast.BinaryExpr:
		return true
	default:
		return false
	}
}

func writeExternalNamedIntegerConstValue(out *strings.Builder, name *ast.Ident) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || name == nil {
		return false
	}
	obj, ok := typeInfo.GetObject(name).(*types.Const)
	if !ok || obj.Val() == nil {
		return false
	}
	named, ok := types.Unalias(obj.Type()).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	rustType, ok := externalIntegerRustTypeForNamed(named)
	if !ok || isKnownStdlibHelperType(named.Obj().Pkg().Path(), named.Obj().Name()) {
		return false
	}

	var value string
	if isUnsignedIntegerType(named) {
		u, exact := constant.Uint64Val(obj.Val())
		if !exact {
			return false
		}
		value = strconv.FormatUint(u, 10)
	} else {
		i, exact := constant.Int64Val(obj.Val())
		if !exact {
			return false
		}
		value = strconv.FormatInt(i, 10)
	}

	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	out.WriteString(value)
	out.WriteString(" as ")
	out.WriteString(rustType)
	out.WriteString(")")
	return true
}

// TranspileMethodImpl transpiles a method inside an impl block
func TranspileMethodImpl(out *strings.Builder, fn *ast.FuncDecl, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	transpileMethodImplWithVisibility(out, fn, true, false, false, fileSet, comments)
}

// TranspileTraitMethodImpl transpiles a method inside a trait impl block.
// traitReceiverMutable forces a `&mut self` receiver when the implemented
// interface method lowers to a mutable receiver (see
// interfaceMethodMutableReceiver); the trait definition and every other impl of
// that method use the same decision so the signatures stay consistent.
func TranspileTraitMethodImpl(out *strings.Builder, fn *ast.FuncDecl, traitReceiverMutable bool, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	transpileMethodImplWithVisibility(out, fn, false, true, traitReceiverMutable, fileSet, comments)
}

func writeFunctionTypeAliasMethodImpl(out *strings.Builder, rustTypeName string, methods []*ast.FuncDecl, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	traitName := rustTypeName + "Methods"
	out.WriteString("pub trait ")
	out.WriteString(traitName)
	out.WriteString(" {\n")
	for _, method := range methods {
		writeFunctionTypeAliasMethodSignature(out, method)
	}
	out.WriteString("}\n\n")
	out.WriteString("impl ")
	out.WriteString(traitName)
	out.WriteString(" for ")
	out.WriteString(rustTypeName)
	out.WriteString(" {\n")
	for i, method := range methods {
		if i > 0 {
			out.WriteString("\n")
		}
		TranspileTraitMethodImpl(out, method, false, fileSet, comments)
	}
	out.WriteString("}")
}

func writeFunctionTypeAliasMethodSignature(out *strings.Builder, fn *ast.FuncDecl) {
	out.WriteString("    fn ")
	out.WriteString(rustMethodName(fn))
	out.WriteString("(&self")
	if fn.Type.Params != nil {
		for _, field := range fn.Type.Params.List {
			for _, name := range field.Names {
				out.WriteString(", ")
				out.WriteString(RustLocalIdent(name.Name))
				out.WriteString(": ")
				out.WriteString(GoTypeToRustParam(field.Type))
			}
		}
	}
	out.WriteString(")")
	writeFuncDeclResultTypes(out, fn)
	out.WriteString(";\n")
}

func namedReturnIdents(fnType *ast.FuncType) []*ast.Ident {
	if fnType == nil || fnType.Results == nil {
		return nil
	}
	var names []*ast.Ident
	for _, result := range fnType.Results.List {
		names = append(names, result.Names...)
	}
	return names
}

func hasNamedReturns(fnType *ast.FuncType) bool {
	return len(namedReturnIdents(fnType)) > 0
}

func writeNamedReturnDeclarations(out *strings.Builder, fnType *ast.FuncType) {
	if fnType == nil || fnType.Results == nil {
		return
	}
	wrote := false
	for _, result := range fnType.Results.List {
		if len(result.Names) == 0 {
			continue
		}
		for _, name := range result.Names {
			if name.Name == "_" {
				out.WriteString("    let ")
			} else {
				out.WriteString("    let mut ")
			}
			out.WriteString(RustLocalIdent(name.Name))
			out.WriteString(": ")
			if info, ok := arrayElemPtrCandidateForDecl(name); ok {
				out.WriteString(arrayElemPtrOptionRustType(info))
				out.WriteString(" = None;\n")
				wrote = true
				continue
			}
			if info, ok := goPtrCandidateForDecl(name); ok {
				elemRustType := goPtrResultElemRustType(info)
				registerGoPtrVar(name.Name, elemRustType, nil)
				out.WriteString("GoPtr<")
				out.WriteString(elemRustType)
				out.WriteString("> = GoPtr::nil();\n")
				wrote = true
				continue
			}
			out.WriteString(GoTypeToRust(result.Type))
			out.WriteString(" = ")

			if t, ok := result.Type.(*ast.Ident); ok && t.Name == "error" {
				if NeedsConcurrentWrapper() {
					TrackImport("Arc")
					TrackImport("Mutex")
					out.WriteString("Arc::new(Mutex::new(None))")
				} else {
					TrackImport("Rc")
					TrackImport("RefCell")
					out.WriteString("Rc::new(RefCell::new(None))")
				}
				out.WriteString(";\n")
				wrote = true
				continue
			}

			if _, ok := functionSignatureFromTypeExpr(result.Type); ok {
				WriteWrappedNone(out)
				out.WriteString(";\n")
				wrote = true
				continue
			}

			if isEmptyInterfaceExpr(result.Type) {
				WriteWrappedNone(out)
				out.WriteString(";\n")
				wrote = true
				continue
			}

			if _, ok := transpiledNamedInterfaceTypeNameFromExpr(result.Type); ok {
				WriteWrappedNone(out)
				out.WriteString(";\n")
				wrote = true
				continue
			}

			if writeDirectTypeParamWrappedZeroValue(out, result.Type, "named return zero value") {
				out.WriteString(";\n")
				wrote = true
				continue
			}

			if writeNamedReturnNilZeroValueFromTypeInfo(out, result.Type) {
				out.WriteString(";\n")
				wrote = true
				continue
			}

			WriteWrapperPrefix(out)
			switch t := result.Type.(type) {
			case *ast.Ident:
				switch t.Name {
				case "string":
					out.WriteString("String::new()")
				case "int", "int64", "int32", "int16", "int8":
					out.WriteString("0")
				case "uint", "uint64", "uint32", "uint16", "uint8":
					out.WriteString("0")
				case "float64", "float32":
					out.WriteString("0.0")
				case "bool":
					out.WriteString("false")
				default:
					out.WriteString("Default::default()")
				}
			default:
				out.WriteString("Default::default()")
			}
			out.WriteString(")))")
			out.WriteString(";\n")
			wrote = true
		}
	}
	if wrote {
		out.WriteString("\n")
	}
}

func writeNamedReturnValues(out *strings.Builder, fnType *ast.FuncType) {
	names := namedReturnIdents(fnType)
	if len(names) == 0 {
		return
	}
	if len(names) > 1 {
		out.WriteString("(")
	}
	first := true
	for _, result := range fnType.Results.List {
		for _, name := range result.Names {
			if !first {
				out.WriteString(", ")
			}
			first = false
			if name.Name == "_" {
				if resultTypeExprIsBareScalar(result.Type) {
					writeBareScalarZeroValue(out, result.Type)
				} else {
					writeNamedReturnZeroValue(out, result.Type)
				}
			} else {
				if _, ok := goPtrCandidateForDecl(name); ok {
					out.WriteString(RustLocalIdent(name.Name))
					out.WriteString(".clone()")
				} else if resultTypeExprIsBareScalar(result.Type) {
					out.WriteString("(*")
					out.WriteString(RustLocalIdent(name.Name))
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())")
				} else {
					out.WriteString(RustLocalIdent(name.Name))
					out.WriteString(".clone()")
				}
			}
		}
	}
	if len(names) > 1 {
		out.WriteString(")")
	}
}

func writeNamedReturnZeroValue(out *strings.Builder, typeExpr ast.Expr) {
	if resultTypeExprIsBareScalar(typeExpr) {
		writeBareScalarZeroValue(out, typeExpr)
		return
	}
	if t, ok := typeExpr.(*ast.Ident); ok && t.Name == "error" {
		WriteWrappedNone(out)
		return
	}
	if _, ok := functionSignatureFromTypeExpr(typeExpr); ok {
		WriteWrappedNone(out)
		return
	}
	if isEmptyInterfaceExpr(typeExpr) {
		WriteWrappedNone(out)
		return
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromExpr(typeExpr); ok {
		WriteWrappedNone(out)
		return
	}
	if writeDirectTypeParamWrappedZeroValue(out, typeExpr, "named return zero value") {
		return
	}
	if writeNamedReturnNilZeroValueFromTypeInfo(out, typeExpr) {
		return
	}

	WriteWrapperPrefix(out)
	switch t := typeExpr.(type) {
	case *ast.Ident:
		switch t.Name {
		case "string":
			out.WriteString("String::new()")
		case "int", "int64", "int32", "int16", "int8":
			out.WriteString("0")
		case "uint", "uint64", "uint32", "uint16", "uint8":
			out.WriteString("0")
		case "float64", "float32":
			out.WriteString("0.0")
		case "bool":
			out.WriteString("false")
		default:
			out.WriteString("Default::default()")
		}
	default:
		out.WriteString("Default::default()")
	}
	WriteWrapperSuffix(out)
}

func shouldWrapDeferPanicRecover(hasDefer bool) bool {
	if !hasDefer || !NeedsConcurrentWrapper() {
		return false
	}
	NeedPanicRecover()
	return true
}

func beginPanicRecoverCatch(out *strings.Builder, indent string) string {
	out.WriteString(indent)
	out.WriteString("let __go_previous_panic_hook = std::panic::take_hook();\n")
	out.WriteString(indent)
	out.WriteString("std::panic::set_hook(Box::new(|_| {}));\n")
	out.WriteString(indent)
	out.WriteString("let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {\n")
	return indent + "    "
}

func finishPanicRecoverCatch(out *strings.Builder, fn *ast.FuncDecl, indent string) {
	out.WriteString(indent)
	out.WriteString("}));\n")
	out.WriteString(indent)
	out.WriteString("std::panic::set_hook(__go_previous_panic_hook);\n")
	out.WriteString(indent)
	out.WriteString("match __go_panic_result {\n")
	out.WriteString(indent)
	out.WriteString("    Ok(__go_value) => __go_value,\n")
	out.WriteString(indent)
	out.WriteString("    Err(__go_panic_payload) => {\n")
	out.WriteString(indent)
	out.WriteString("        go_store_panic_payload(__go_panic_payload);\n")
	out.WriteString(indent)
	out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
	out.WriteString(indent)
	out.WriteString("            f();\n")
	out.WriteString(indent)
	out.WriteString("        }\n")
	out.WriteString(indent)
	out.WriteString("        go_resume_unrecovered_panic();\n")
	out.WriteString(indent)
	out.WriteString("        ")
	writeRecoveredPanicReturnValue(out, fn)
	out.WriteString("\n")
	out.WriteString(indent)
	out.WriteString("    }\n")
	out.WriteString(indent)
	out.WriteString("}\n")
}

func writeRecoveredPanicReturnValue(out *strings.Builder, fn *ast.FuncDecl) {
	if fn == nil {
		out.WriteString("()")
		return
	}
	if hasNamedReturns(fn.Type) {
		writeNamedReturnValues(out, fn.Type)
		return
	}
	writeFunctionZeroReturnValues(out, fn)
}

func writeFunctionZeroReturnValues(out *strings.Builder, fn *ast.FuncDecl) {
	resultTypes := functionResultTypeExprs(fn.Type)
	if len(resultTypes) == 0 {
		out.WriteString("()")
		return
	}
	if len(resultTypes) > 1 {
		out.WriteString("(")
	}
	for i, resultType := range resultTypes {
		if i > 0 {
			out.WriteString(", ")
		}
		if !writeGoPtrNilReturnValueForFunc(out, fn, i) {
			writeNamedReturnZeroValue(out, resultType)
		}
	}
	if len(resultTypes) > 1 {
		out.WriteString(")")
	}
}

func functionResultTypeExprs(fnType *ast.FuncType) []ast.Expr {
	if fnType == nil || fnType.Results == nil {
		return nil
	}
	var resultTypes []ast.Expr
	for _, result := range fnType.Results.List {
		count := len(result.Names)
		if count == 0 {
			count = 1
		}
		for i := 0; i < count; i++ {
			resultTypes = append(resultTypes, result.Type)
		}
	}
	return resultTypes
}

func writeNamedReturnNilZeroValueFromTypeInfo(out *strings.Builder, typeExpr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(typeExpr)
	if typ == nil {
		return false
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		if _, isSlice := types.Unalias(named.Underlying()).(*types.Slice); isSlice {
			return false
		}
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Interface, *types.Pointer, *types.Signature, *types.Slice:
		WriteWrappedNone(out)
		return true
	default:
		return false
	}
}

func exprReferencesReceiver(expr ast.Expr, receiverName string) bool {
	switch e := expr.(type) {
	case *ast.Ident:
		return e.Name == receiverName
	case *ast.SelectorExpr:
		return exprReferencesReceiver(e.X, receiverName)
	case *ast.IndexExpr:
		return exprReferencesReceiver(e.X, receiverName)
	case *ast.StarExpr:
		return exprReferencesReceiver(e.X, receiverName)
	case *ast.UnaryExpr:
		return exprReferencesReceiver(e.X, receiverName)
	case *ast.ParenExpr:
		return exprReferencesReceiver(e.X, receiverName)
	default:
		return false
	}
}

func receiverNameForMethod(fn *ast.FuncDecl) string {
	if fn == nil || fn.Recv == nil || len(fn.Recv.List) == 0 || len(fn.Recv.List[0].Names) == 0 {
		return ""
	}
	return fn.Recv.List[0].Names[0].Name
}

func methodMutatesReceiver(fn *ast.FuncDecl, receiverName string) bool {
	return methodMutatesReceiverWithSeen(fn, receiverName, getMethodReceiverType(fn), make(map[*ast.FuncDecl]bool))
}

func methodRequiresMutableReceiver(fn *ast.FuncDecl) bool {
	if fn == nil || fn.Recv == nil || len(fn.Recv.List) == 0 {
		return false
	}
	if _, isPointer := fn.Recv.List[0].Type.(*ast.StarExpr); !isPointer {
		return false
	}
	return methodMutatesReceiver(fn, receiverNameForMethod(fn))
}

func methodReassignsReceiver(fn *ast.FuncDecl) bool {
	if fn == nil || fn.Recv == nil || len(fn.Recv.List) == 0 || fn.Body == nil {
		return false
	}
	found := false
	ast.Inspect(fn.Body, func(n ast.Node) bool {
		if found {
			return false
		}
		if _, ok := n.(*ast.FuncLit); ok {
			return false
		}
		assign, ok := n.(*ast.AssignStmt)
		if !ok {
			return true
		}
		for _, lhs := range assign.Lhs {
			ident, ok := unwrapParens(lhs).(*ast.Ident)
			if ok && isCurrentReceiverIdent(ident) {
				found = true
				return false
			}
		}
		return true
	})
	return found
}

func methodReceiverComparedToNil(fn *ast.FuncDecl) bool {
	if fn == nil || fn.Recv == nil || len(fn.Recv.List) == 0 || fn.Body == nil {
		return false
	}
	receiverName := receiverNameForMethod(fn)
	if receiverName == "" {
		return false
	}
	found := false
	ast.Inspect(fn.Body, func(n ast.Node) bool {
		if found {
			return false
		}
		if _, ok := n.(*ast.FuncLit); ok {
			return false
		}
		binary, ok := n.(*ast.BinaryExpr)
		if !ok || (binary.Op != token.EQL && binary.Op != token.NEQ) {
			return true
		}
		left, leftIsIdent := unwrapParens(binary.X).(*ast.Ident)
		right, rightIsIdent := unwrapParens(binary.Y).(*ast.Ident)
		if leftIsIdent && left.Name == receiverName && rightIsIdent && right.Name == "nil" {
			found = true
			return false
		}
		if rightIsIdent && right.Name == receiverName && leftIsIdent && left.Name == "nil" {
			found = true
			return false
		}
		return true
	})
	return found
}

func methodNeedsPointerReceiverHandleAlias(fn *ast.FuncDecl) bool {
	if fn == nil || fn.Recv == nil || len(fn.Recv.List) == 0 {
		return false
	}
	if _, isPointer := fn.Recv.List[0].Type.(*ast.StarExpr); !isPointer {
		return false
	}
	return methodReassignsReceiver(fn) && methodReceiverComparedToNil(fn)
}

func methodReassignsValueReceiver(fn *ast.FuncDecl) bool {
	if fn == nil || fn.Recv == nil || len(fn.Recv.List) == 0 {
		return false
	}
	if _, isPointer := fn.Recv.List[0].Type.(*ast.StarExpr); isPointer {
		return false
	}
	return methodReassignsReceiver(fn)
}

func methodValueReceiverNeedsLocalAlias(fn *ast.FuncDecl) bool {
	if fn == nil || fn.Recv == nil || len(fn.Recv.List) == 0 {
		return false
	}
	if _, isPointer := fn.Recv.List[0].Type.(*ast.StarExpr); isPointer {
		return false
	}
	return methodMutatesReceiver(fn, receiverNameForMethod(fn))
}

func collectMethodReceiverMutability(files []*ast.File, typeInfo *TypeInfo) map[string]bool {
	mutableByMethod := make(map[string]bool)
	if typeInfo == nil || typeInfo.info == nil {
		return mutableByMethod
	}
	methodsByReceiver := make(map[string][]*ast.FuncDecl)
	for _, file := range files {
		if file == nil {
			continue
		}
		for _, decl := range file.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Recv == nil || len(fn.Recv.List) == 0 {
				continue
			}
			receiverKey := methodReceiverGroupKey(fn, typeInfo)
			if receiverKey == "" {
				continue
			}
			methodsByReceiver[receiverKey] = append(methodsByReceiver[receiverKey], fn)
		}
	}
	for _, methods := range methodsByReceiver {
		for _, fn := range methods {
			obj := methodFuncForDecl(fn, typeInfo)
			if obj == nil {
				continue
			}
			key := methodOverrideKey(obj)
			if key == "" {
				continue
			}
			mutableByMethod[key] = methodRequiresMutableReceiverFromMap(fn, methodsByReceiver, typeInfo)
		}
	}
	return mutableByMethod
}

func collectMethodReceiverOriginalReceiver(files []*ast.File, typeInfo *TypeInfo) map[string]bool {
	originalByMethod := make(map[string]bool)
	if typeInfo == nil || typeInfo.info == nil {
		return originalByMethod
	}
	methodsByReceiver := make(map[string][]*ast.FuncDecl)
	for _, file := range files {
		if file == nil {
			continue
		}
		for _, decl := range file.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Recv == nil || len(fn.Recv.List) == 0 {
				continue
			}
			receiverKey := methodReceiverGroupKey(fn, typeInfo)
			if receiverKey == "" {
				continue
			}
			methodsByReceiver[receiverKey] = append(methodsByReceiver[receiverKey], fn)
		}
	}
	for _, methods := range methodsByReceiver {
		for _, fn := range methods {
			obj := methodFuncForDecl(fn, typeInfo)
			if obj == nil {
				continue
			}
			key := methodOverrideKey(obj)
			if key == "" {
				continue
			}
			originalByMethod[key] = methodRequiresOriginalReceiverFromMap(fn, methodsByReceiver, typeInfo)
		}
	}
	return originalByMethod
}

var packageMethodReceiverMutability = make(map[string]bool)
var packageMethodReceiverOriginalReceiver = make(map[string]bool)

func resetPackageMethodReceiverMutability() {
	packageMethodReceiverMutability = make(map[string]bool)
	packageMethodReceiverOriginalReceiver = make(map[string]bool)
}

func registerPackageMethodReceiverMutability(pkgPath string, files []*ast.File) {
	if pkgPath == "" {
		pkgPath = "main"
	}
	methodsByReceiver := collectPackageMethods(files)
	for receiverType, methods := range methodsByReceiver {
		for _, fn := range methods {
			if fn == nil || fn.Name == nil {
				continue
			}
			key := packageMethodReceiverMutabilityKey(pkgPath, receiverType, fn.Name.Name)
			packageMethodReceiverMutability[key] = methodRequiresMutableReceiverFromMap(fn, methodsByReceiver, nil)
		}
	}
}

func registerPackageMethodReceiverOriginalReceiver(pkgPath string, files []*ast.File, typeInfo *TypeInfo) {
	if pkgPath == "" {
		pkgPath = "main"
	}
	methodsByReceiver := collectPackageMethods(files)
	for receiverType, methods := range methodsByReceiver {
		for _, fn := range methods {
			if fn == nil || fn.Name == nil {
				continue
			}
			key := packageMethodReceiverMutabilityKey(pkgPath, receiverType, fn.Name.Name)
			packageMethodReceiverOriginalReceiver[key] = methodRequiresOriginalReceiverFromMap(fn, methodsByReceiver, typeInfo)
		}
	}
}

func packageMethodReceiverMutabilityKey(pkgPath string, receiverType string, methodName string) string {
	if pkgPath == "" || receiverType == "" || methodName == "" {
		return ""
	}
	return pkgPath + "\x00" + receiverType + "\x00" + methodName
}

func packageMethodReceiverMutabilityForSelector(sel *ast.SelectorExpr) (bool, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || sel == nil {
		return false, false
	}
	// Dispatching through an interface method mutates the dynamic value when an
	// implementor does; such methods lower to `&mut self`, so the call site must
	// borrow mutably (.as_mut()/.borrow_mut()).
	if interfaceMethodSelectorRequiresMutableReceiver(sel) {
		return true, true
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok {
		return false, false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok || fn == nil {
		return false, false
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Recv() == nil {
		return false, false
	}
	recv := types.Unalias(sig.Recv().Type())
	if ptr, ok := recv.(*types.Pointer); ok {
		recv = types.Unalias(ptr.Elem())
	}
	named, ok := recv.(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false, false
	}
	key := packageMethodReceiverMutabilityKey(named.Obj().Pkg().Path(), named.Obj().Name(), fn.Name())
	if key == "" {
		return false, false
	}
	mutable, ok := packageMethodReceiverMutability[key]
	return mutable, ok
}

func packageMethodReceiverOriginalReceiverForSelector(sel *ast.SelectorExpr) (bool, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || sel == nil {
		return false, false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok {
		return false, false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok || fn == nil {
		return false, false
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Recv() == nil {
		return false, false
	}
	recv := types.Unalias(sig.Recv().Type())
	if ptr, ok := recv.(*types.Pointer); ok {
		recv = types.Unalias(ptr.Elem())
	}
	named, ok := recv.(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false, false
	}
	key := packageMethodReceiverMutabilityKey(named.Obj().Pkg().Path(), named.Obj().Name(), fn.Name())
	if key == "" {
		return false, false
	}
	original, ok := packageMethodReceiverOriginalReceiver[key]
	return original, ok
}

// interfaceMethodMutableReceiver records whether any concrete implementor's
// corresponding method requires a mutable Rust receiver. A Go interface method
// carries no receiver mutability of its own, but dispatching through it mutates
// the dynamic value whenever an implementor does. The interface trait method,
// every impl of it, and every interface-dispatch call site consult this single
// map so the three stay consistent: a method that mutates through any
// implementor lowers to `&mut self` everywhere.
var interfaceMethodMutableReceiver = make(map[*types.Func]bool)
var interfaceMethodMutableReceiverByName = make(map[string]bool)
var interfaceMethodMutableReceiverByTrait = make(map[string]bool)

func resetInterfaceMethodMutableReceiver() {
	interfaceMethodMutableReceiver = make(map[*types.Func]bool)
	interfaceMethodMutableReceiverByName = make(map[string]bool)
	interfaceMethodMutableReceiverByTrait = make(map[string]bool)
}

// registerInterfaceMethodMutableReceivers walks every interface across all
// loaded packages and ORs in, per interface method, whether any concrete
// implementor's matching method requires a mutable receiver. It must run after
// packageMethodReceiverMutability is fully populated for every package and
// before any emission (see PackageLoader.TranspileAll).
func registerInterfaceMethodMutableReceivers(pkgs []*types.Package) {
	resetInterfaceMethodMutableReceiver()
	var ifaceTypes []*types.Named
	var concreteTypes []*types.Named
	for _, pkg := range pkgs {
		if pkg == nil || pkg.Scope() == nil {
			continue
		}
		for _, name := range pkg.Scope().Names() {
			tn, ok := pkg.Scope().Lookup(name).(*types.TypeName)
			if !ok {
				continue
			}
			named, ok := types.Unalias(tn.Type()).(*types.Named)
			if !ok {
				continue
			}
			if _, ok := named.Underlying().(*types.Interface); ok {
				ifaceTypes = append(ifaceTypes, named)
			} else {
				concreteTypes = append(concreteTypes, named)
			}
		}
	}
	for _, ifaceNamed := range ifaceTypes {
		iface, _ := ifaceNamed.Underlying().(*types.Interface)
		if iface == nil || iface.NumMethods() == 0 {
			continue
		}
		ifaceTraitNames := interfaceMethodMutableReceiverTraitNames(ifaceNamed)
		for _, concrete := range concreteTypes {
			obj := concrete.Obj()
			if obj == nil || obj.Pkg() == nil {
				continue
			}
			if !types.Implements(concrete, iface) && !types.Implements(types.NewPointer(concrete), iface) {
				continue
			}
			pkgPath := obj.Pkg().Path()
			if pkgPath == "" {
				pkgPath = "main"
			}
			for j := 0; j < iface.NumMethods(); j++ {
				m := iface.Method(j)
				concreteKey := packageMethodReceiverMutabilityKey(pkgPath, obj.Name(), m.Name())
				if mutable, ok := packageMethodReceiverMutability[concreteKey]; ok && mutable {
					interfaceMethodMutableReceiver[m] = true
					for _, key := range interfaceMethodMutableReceiverKeys(m) {
						interfaceMethodMutableReceiverByName[key] = true
					}
					for _, traitName := range ifaceTraitNames {
						interfaceMethodMutableReceiverByTrait[interfaceMethodMutableReceiverTraitKey(traitName, m.Name())] = true
					}
				}
			}
		}
	}
}

func interfaceMethodMutableReceiverTraitNames(ifaceNamed *types.Named) []string {
	if ifaceNamed == nil || ifaceNamed.Obj() == nil {
		return nil
	}
	seen := make(map[string]bool)
	var names []string
	add := func(name string) {
		if name == "" || seen[name] {
			return
		}
		seen[name] = true
		names = append(names, name)
	}
	add(ifaceNamed.Obj().Name())
	add(goTypesNamedTypeToRust(ifaceNamed))
	if pkg := ifaceNamed.Obj().Pkg(); pkg != nil && pkg.Path() != "" {
		add(RustCrateNameForGoImportPath(pkg.Path()) + "::" + RustTypeNameForUse(ifaceNamed.Obj().Name()))
	}
	return names
}

func interfaceMethodMutableReceiverTraitKey(ifaceName string, methodName string) string {
	if ifaceName == "" || methodName == "" {
		return ""
	}
	return ifaceName + "." + methodName
}

func interfaceMethodMutableReceiverKeys(method *types.Func) []string {
	if method == nil {
		return nil
	}
	seen := make(map[string]bool)
	var keys []string
	add := func(key string) {
		if key == "" || seen[key] {
			return
		}
		seen[key] = true
		keys = append(keys, key)
	}
	add(method.FullName())
	if method.Pkg() != nil {
		add(method.Pkg().Path() + "." + method.Name())
	}
	return keys
}

// interfaceMethodRequiresMutableReceiver reports whether the given interface
// method (an interface's *types.Func) lowers to a `&mut self` Rust trait method.
func interfaceMethodRequiresMutableReceiver(method *types.Func) bool {
	if method == nil {
		return false
	}
	if interfaceMethodMutableReceiver[method] {
		return true
	}
	keys := interfaceMethodMutableReceiverKeys(method)
	for _, key := range keys {
		if interfaceMethodMutableReceiverByName[key] {
			return true
		}
	}
	for registered, mutable := range interfaceMethodMutableReceiver {
		if !mutable {
			continue
		}
		for _, registeredKey := range interfaceMethodMutableReceiverKeys(registered) {
			for _, key := range keys {
				if registeredKey == key {
					return true
				}
			}
		}
	}
	return false
}

func interfaceTraitMethodRequiresMutableReceiver(ifaceName string, methodName string, method *types.Func) bool {
	if interfaceMethodRequiresMutableReceiver(method) {
		return true
	}
	if methodName == "" && method != nil {
		methodName = method.Name()
	}
	if methodName == "" {
		return false
	}
	key := interfaceMethodMutableReceiverTraitKey(ifaceName, methodName)
	return key != "" && interfaceMethodMutableReceiverByTrait[key]
}

// interfaceMethodSelectorRequiresMutableReceiver reports whether a method-call
// selector dispatches through an interface method that lowers to `&mut self`.
func interfaceMethodSelectorRequiresMutableReceiver(sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || sel == nil {
		return false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok {
		return false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok {
		return false
	}
	return interfaceMethodRequiresMutableReceiver(fn)
}

// interfaceMethodByName returns the interface's method with the given Go name,
// or nil. Only the interface's own method set is considered.
func interfaceMethodByName(iface *types.Interface, name string) *types.Func {
	if iface == nil {
		return nil
	}
	for j := 0; j < iface.NumMethods(); j++ {
		if m := iface.Method(j); m != nil && m.Name() == name {
			return m
		}
	}
	return nil
}

// interfaceTraitMethodReceiver returns "&mut self" when the named local
// interface's method lowers to a mutable receiver, else "&self".
func interfaceTraitMethodReceiver(ifaceName, methodName string) string {
	if iface := localInterfaceTypesByName(ifaceName); iface != nil {
		if m := interfaceMethodByName(iface, methodName); interfaceTraitMethodRequiresMutableReceiver(ifaceName, methodName, m) {
			return "&mut self"
		}
	}
	return "&self"
}

func methodReceiverGroupKey(fn *ast.FuncDecl, typeInfo *TypeInfo) string {
	if obj := methodFuncForDecl(fn, typeInfo); obj != nil {
		if sig, ok := obj.Type().(*types.Signature); ok && sig.Recv() != nil {
			return methodReceiverKey(sig.Recv().Type())
		}
	}
	return getMethodReceiverType(fn)
}

func methodRequiresMutableReceiverFromMap(fn *ast.FuncDecl, methodsByReceiver map[string][]*ast.FuncDecl, typeInfo *TypeInfo) bool {
	if fn == nil || fn.Recv == nil || len(fn.Recv.List) == 0 {
		return false
	}
	if _, isPointer := fn.Recv.List[0].Type.(*ast.StarExpr); !isPointer {
		return false
	}
	return methodMutatesReceiverWithMethodMap(fn, receiverNameForMethod(fn), methodReceiverGroupKey(fn, typeInfo), methodsByReceiver, typeInfo, make(map[*ast.FuncDecl]bool))
}

func methodRequiresOriginalReceiverFromMap(fn *ast.FuncDecl, methodsByReceiver map[string][]*ast.FuncDecl, typeInfo *TypeInfo) bool {
	if fn == nil || fn.Recv == nil || len(fn.Recv.List) == 0 {
		return false
	}
	if _, isPointer := fn.Recv.List[0].Type.(*ast.StarExpr); !isPointer {
		return false
	}
	return methodNeedsOriginalReceiverWithMethodMap(fn, receiverNameForMethod(fn), methodReceiverGroupKey(fn, typeInfo), methodsByReceiver, typeInfo, make(map[*ast.FuncDecl]bool))
}

func getMethodReceiverType(fn *ast.FuncDecl) string {
	if fn == nil || fn.Recv == nil || len(fn.Recv.List) == 0 {
		return ""
	}
	return getReceiverType(fn.Recv.List[0].Type)
}

func methodsForReceiverType(receiverType string) []*ast.FuncDecl {
	if receiverType != "" {
		if ctx := GetTranspileContext(); ctx != nil && ctx.Package != nil && len(ctx.Package.MethodsByType[receiverType]) > 0 {
			return ctx.Package.MethodsByType[receiverType]
		}
	}
	return currentTypeMethods
}

func unsafePointerCallArgFromTypeInfo(expr ast.Expr, typeInfo *TypeInfo) (ast.Expr, bool) {
	call, ok := unwrapParens(expr).(*ast.CallExpr)
	if !ok || len(call.Args) != 1 || typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	sel, ok := unwrapParens(call.Fun).(*ast.SelectorExpr)
	if !ok || sel.Sel == nil {
		return nil, false
	}
	obj, ok := typeInfo.GetObject(sel.Sel).(*types.TypeName)
	if !ok || obj.Pkg() == nil || obj.Pkg().Path() != "unsafe" || obj.Name() != "Pointer" {
		return nil, false
	}
	return call.Args[0], true
}

func methodNeedsOriginalReceiverWithMethodMap(fn *ast.FuncDecl, receiverName string, receiverKey string, methodsByReceiver map[string][]*ast.FuncDecl, typeInfo *TypeInfo, seen map[*ast.FuncDecl]bool) bool {
	if fn == nil || fn.Body == nil || receiverName == "" {
		return false
	}
	if seen[fn] {
		return false
	}
	seen[fn] = true
	needsOriginal := false
	ast.Inspect(fn.Body, func(n ast.Node) bool {
		if needsOriginal {
			return false
		}
		call, ok := n.(*ast.CallExpr)
		if !ok {
			return true
		}
		if source, ok := unsafePointerCallArgFromTypeInfo(call, typeInfo); ok && exprReferencesReceiver(source, receiverName) {
			needsOriginal = true
			return false
		}
		sel, ok := call.Fun.(*ast.SelectorExpr)
		if !ok || !exprReferencesReceiver(sel.X, receiverName) {
			return true
		}
		called := methodDeclByName(methodsByReceiver[receiverKey], sel.Sel.Name)
		if called == nil {
			return true
		}
		if methodNeedsOriginalReceiverWithMethodMap(called, receiverNameForMethod(called), methodReceiverGroupKey(called, typeInfo), methodsByReceiver, typeInfo, seen) {
			needsOriginal = true
			return false
		}
		return true
	})
	return needsOriginal
}

func methodMutatesReceiverWithMethodMap(fn *ast.FuncDecl, receiverName string, receiverKey string, methodsByReceiver map[string][]*ast.FuncDecl, typeInfo *TypeInfo, seen map[*ast.FuncDecl]bool) bool {
	if fn == nil || fn.Body == nil || receiverName == "" {
		return false
	}
	if seen[fn] {
		return false
	}
	seen[fn] = true
	mutates := false
	ast.Inspect(fn.Body, func(n ast.Node) bool {
		if mutates {
			return false
		}
		switch stmt := n.(type) {
		case *ast.AssignStmt:
			for _, lhs := range stmt.Lhs {
				if exprReferencesReceiver(lhs, receiverName) {
					mutates = true
					return false
				}
			}
		case *ast.IncDecStmt:
			if exprReferencesReceiver(stmt.X, receiverName) {
				mutates = true
				return false
			}
		case *ast.CallExpr:
			for _, arg := range stmt.Args {
				if callArgumentMayMutateReceiver(arg, receiverName) {
					mutates = true
					return false
				}
			}
			sel, ok := stmt.Fun.(*ast.SelectorExpr)
			if !ok {
				return true
			}
			if !exprReferencesReceiver(sel.X, receiverName) {
				// fmt.Fprintf(receiver, ...) transitively invokes receiver.Write(...)
				// when receiver implements io.Writer. Treat that as a method call
				// on the receiver so its mutation propagates here.
				if callIsFprintfWritingToReceiver(stmt, sel, receiverName) {
					if called := methodDeclByName(methodsByReceiver[receiverKey], "Write"); called != nil {
						if methodMutatesReceiverWithMethodMap(called, receiverNameForMethod(called), methodReceiverGroupKey(called, typeInfo), methodsByReceiver, typeInfo, seen) {
							mutates = true
							return false
						}
					}
				}
				return true
			}
			called := methodDeclByName(methodsByReceiver[receiverKey], sel.Sel.Name)
			if called == nil {
				return true
			}
			if methodMutatesReceiverWithMethodMap(called, receiverNameForMethod(called), methodReceiverGroupKey(called, typeInfo), methodsByReceiver, typeInfo, seen) {
				mutates = true
				return false
			}
		}
		return true
	})
	return mutates
}

func callArgumentMayMutateReceiver(arg ast.Expr, receiverName string) bool {
	unary, ok := unwrapParens(arg).(*ast.UnaryExpr)
	return ok && unary.Op == token.AND && exprReferencesReceiver(unary.X, receiverName)
}

// callIsFprintfWritingToReceiver reports whether call is `fmt.Fprintf(<recv>,
// ...)` (or Fprint/Fprintln) where the first argument references the named
// receiver. The transpiler lowers such calls to <recv>.write(...), so the
// receiver participates as the write target and inherits its Write method's
// mutation behavior.
func callIsFprintfWritingToReceiver(call *ast.CallExpr, sel *ast.SelectorExpr, receiverName string) bool {
	if len(call.Args) == 0 {
		return false
	}
	pkgIdent, ok := sel.X.(*ast.Ident)
	if !ok || pkgIdent.Name != "fmt" {
		return false
	}
	switch sel.Sel.Name {
	case "Fprintf", "Fprint", "Fprintln":
	default:
		return false
	}
	return exprReferencesReceiver(call.Args[0], receiverName)
}

func methodMutatesReceiverWithSeen(fn *ast.FuncDecl, receiverName string, receiverType string, seen map[*ast.FuncDecl]bool) bool {
	if fn == nil || fn.Body == nil || receiverName == "" {
		return false
	}
	if seen[fn] {
		return false
	}
	seen[fn] = true
	mutates := false
	ast.Inspect(fn.Body, func(n ast.Node) bool {
		if mutates {
			return false
		}
		switch stmt := n.(type) {
		case *ast.AssignStmt:
			for _, lhs := range stmt.Lhs {
				if exprReferencesReceiver(lhs, receiverName) {
					mutates = true
					return false
				}
			}
		case *ast.IncDecStmt:
			if exprReferencesReceiver(stmt.X, receiverName) {
				mutates = true
				return false
			}
		case *ast.CallExpr:
			sel, ok := stmt.Fun.(*ast.SelectorExpr)
			if !ok {
				return true
			}
			if !exprReferencesReceiver(sel.X, receiverName) {
				// fmt.Fprintf(receiver, ...) lowers to receiver.write(...);
				// propagate the Write method's mutation through this indirect call.
				if callIsFprintfWritingToReceiver(stmt, sel, receiverName) {
					if called := methodDeclByName(methodsForReceiverType(receiverType), "Write"); called != nil {
						if methodMutatesReceiverWithSeen(called, receiverNameForMethod(called), getMethodReceiverType(called), seen) {
							mutates = true
							return false
						}
					}
				}
				return true
			}
			called := methodDeclByName(methodsForReceiverType(receiverType), sel.Sel.Name)
			if called == nil {
				return true
			}
			if methodMutatesReceiverWithSeen(called, receiverNameForMethod(called), getMethodReceiverType(called), seen) {
				mutates = true
				return false
			}
		}
		return true
	})
	return mutates
}

func transpileMethodImplWithVisibility(out *strings.Builder, fn *ast.FuncDecl, addPub bool, forceSharedReceiver bool, traitReceiverMutable bool, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	// Store the receiver name and type for self translation
	if fn.Recv != nil && len(fn.Recv.List) > 0 {
		recv := fn.Recv.List[0]
		if len(recv.Names) > 0 {
			setCurrentReceiverFromIdent(recv.Names[0])
		}
		// Store the receiver type
		currentReceiverType = getReceiverType(recv.Type)
	}
	prevCurrentReceiverRustAlias := currentReceiverRustAlias
	prevCurrentReceiverRustAliasIsPointerHandle := currentReceiverRustAliasIsPointerHandle
	prevCurrentReceiverRustAliasIsGoPtr := currentReceiverRustAliasIsGoPtr
	prevCurrentReceiverRustAliasGoPtrInfo := currentReceiverRustAliasGoPtrInfo
	currentReceiverRustAlias = ""
	currentReceiverRustAliasIsPointerHandle = false
	currentReceiverRustAliasIsGoPtr = false
	currentReceiverRustAliasGoPtrInfo = goPtrResultInfo{}
	if methodReassignsReceiver(fn) || methodValueReceiverNeedsLocalAlias(fn) {
		currentReceiverRustAlias = "__self"
		currentReceiverRustAliasIsPointerHandle = methodNeedsPointerReceiverHandleAlias(fn)
		if info, ok := methodNeedsGoPtrReceiverAlias(fn); ok {
			currentReceiverRustAliasIsGoPtr = true
			currentReceiverRustAliasGoPtrInfo = info
			currentReceiverRustAliasIsPointerHandle = false
		}
	}
	defer func() {
		currentReceiverRustAlias = prevCurrentReceiverRustAlias
		currentReceiverRustAliasIsPointerHandle = prevCurrentReceiverRustAliasIsPointerHandle
		currentReceiverRustAliasIsGoPtr = prevCurrentReceiverRustAliasIsGoPtr
		currentReceiverRustAliasGoPtrInfo = prevCurrentReceiverRustAliasGoPtrInfo
	}()

	// Output doc comments if present (with indentation for methods)
	outputComment(out, fn.Doc, "    ", true)

	out.WriteString("    ")
	if addPub {
		out.WriteString("pub ")
	}
	out.WriteString("fn ")
	out.WriteString(rustMethodName(fn))
	out.WriteString("(")

	// Receiver
	if fn.Recv != nil && len(fn.Recv.List) > 0 {
		recv := fn.Recv.List[0]
		// Store the receiver name for self translation
		if len(recv.Names) > 0 {
			setCurrentReceiverFromIdent(recv.Names[0])
		}

		if forceSharedReceiver {
			if traitReceiverMutable {
				out.WriteString("&mut self")
			} else {
				out.WriteString("&self")
			}
		} else if methodRequiresMutableReceiver(fn) {
			out.WriteString("&mut self")
		} else {
			out.WriteString("&self")
		}

		// Add comma if there are more parameters
		if fn.Type.Params != nil && len(fn.Type.Params.List) > 0 {
			out.WriteString(", ")
		}
	}
	// Other parameters
	writeFuncDeclParams(out, fn)

	out.WriteString(")")

	writeFuncDeclResultTypes(out, fn)

	out.WriteString(" {\n")

	if fn.Body != nil {
		restoreSliceElemPtrCandidates := setSliceElemPtrCandidatesForFunc(fn)
		defer restoreSliceElemPtrCandidates()
		restoreSliceElemPtrReturn := pushCurrentSliceElemPtrReturn(fn)
		defer restoreSliceElemPtrReturn()

		hasDefer := checkHasDefer(fn.Body.List)
		oldFunctionHasDefer := currentFunctionHasDefer
		currentFunctionHasDefer = hasDefer
		defer func() { currentFunctionHasDefer = oldFunctionHasDefer }()
		oldFunctionBodyLbrace := currentFunctionBodyLbrace
		currentFunctionBodyLbrace = fn.Body.Lbrace
		defer func() { currentFunctionBodyLbrace = oldFunctionBodyLbrace }()
		oldActiveMutexGuards := activeMutexGuards
		activeMutexGuards = make(map[string]string)
		defer func() { activeMutexGuards = oldActiveMutexGuards }()
		if hasDefer {
			out.WriteString("        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();\n\n")
		}
		writeNamedReturnDeclarations(out, fn.Type)
	}

	restoreLocalSyntaxInfo := pushFunctionLocalSyntaxInfo()
	defer restoreLocalSyntaxInfo()
	assignedInterfaceParams := assignedInterfaceParamNames(fn)

	// Register method parameters in VarTable
	if vt := GetVarTable(); vt != nil {
		vt.PushScope()
		defer vt.PopScope()
		if fn.Type.Params != nil {
			paramIndex := 0
			for _, field := range fn.Type.Params.List {
				for _, name := range field.Names {
					rustType := goTypeToRustBase(field.Type)
					if elemRustType, ok := sliceElemPtrSliceParamInfoForDeclObject(fn, paramIndex); ok {
						rustType = "Vec<GoPtr<" + elemRustType + ">>"
					}
					if elemRustType, ok := goPtrSlotParamDeclElemRustType(fn, paramIndex); ok {
						rustType = goPtrSlotParamRustType(elemRustType)
					}
					if elemRustType, ok := goPtrParamDeclElemRustType(fn, paramIndex); ok {
						rustType = "GoPtr<" + elemRustType + ">"
					}
					if functionRustType, ok := functionTypeRustNameFromTypeExpr(field.Type); ok {
						rustType = functionRustType
					}
					registerTypeExprCollectionInfo(name.Name, field.Type)
					if elemRustType, ok := goPtrSlotParamDeclElemRustType(fn, paramIndex); ok {
						var goType types.Type
						if typeInfo := GetTypeInfo(); typeInfo != nil {
							goType = typeInfo.GetType(field.Type)
						}
						vt.Register(name.Name, &VarInfo{
							WrapLevel:   WrapNone,
							RustType:    goPtrSlotParamRustType(elemRustType),
							Source:      SourceParam,
							PointerKind: PointerGoPtr,
							GoType:      goType,
						})
					} else if elemRustType, ok := goPtrParamDeclElemRustType(fn, paramIndex); ok {
						var goType types.Type
						if typeInfo := GetTypeInfo(); typeInfo != nil {
							goType = typeInfo.GetType(field.Type)
						}
						vt.Register(name.Name, &VarInfo{
							WrapLevel:   WrapNone,
							RustType:    "GoPtr<" + elemRustType + ">",
							Source:      SourceParam,
							PointerKind: PointerGoPtr,
							GoType:      goType,
						})
					} else if varInfo, ok := interfaceParamVarInfo(field.Type); ok {
						if assignedInterfaceParams[name.Name] {
							vt.Register(name.Name, &VarInfo{
								WrapLevel: WrapFull,
								RustType:  rustType,
								Source:    SourceLocal,
							})
						} else {
							varInfo.RustType = rustType
							vt.Register(name.Name, varInfo)
						}
					} else if _, ok := field.Type.(*ast.ChanType); ok {
						// Channel parameters are bare (GoChannel<T>)
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  rustType,
							Source:    SourceParam,
						})
					} else if isSyncParam(field.Type) {
						// sync.WaitGroup / sync.Mutex parameters are bare
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  rustType,
							Source:    SourceParam,
						})
					} else if typeExprIsRegisteredBareStructAlias(field.Type) {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  rustType,
							Source:    SourceParam,
						})
					} else if typeExprIsOrderedTypeParam(field.Type) {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  rustType,
							Source:    SourceParam,
						})
					} else {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapFull,
							RustType:  rustType,
							Source:    SourceParam,
						})
					}
					paramIndex++
				}
			}
		}
		registerArrayElemPtrNamedReturnVars(fn)
	}

	if writeRuntimeLinkedFunctionBody(out, fn, "        ") {
		out.WriteString("    }\n")
		currentReceiver = ""
		currentReceiverObject = nil
		return
	}

	// Method body - need to handle self references
	if fn.Body == nil {
		out.WriteString("        unimplemented!(\"Go method declaration has no body\");\n")
		out.WriteString("    }\n")
		currentReceiver = ""
		currentReceiverObject = nil
		return
	}
	writeAssignedInterfaceParamShadows(out, fn, "        ")
	if currentReceiverRustAlias != "" {
		out.WriteString("        let mut ")
		out.WriteString(currentReceiverRustAlias)
		out.WriteString(" = ")
		if currentReceiverRustAliasIsGoPtr {
			out.WriteString("GoPtr::local(")
			WriteWrapperOptionPrefix(out)
			out.WriteString("Some(self.clone())")
			WriteWrapperOptionSuffix(out)
			out.WriteString(")")
		} else if currentReceiverRustAliasIsPointerHandle {
			WriteWrapperOptionPrefix(out)
			out.WriteString("Some(self.clone())")
			WriteWrapperOptionSuffix(out)
		} else {
			out.WriteString("self.clone()")
		}
		out.WriteString(";\n")
	}

	recoverCatch := shouldWrapDeferPanicRecover(currentFunctionHasDefer)
	bodyIndent := "        "
	if recoverCatch {
		bodyIndent = beginPanicRecoverCatch(out, "        ")
	}

	var prevStmt ast.Stmt
	var lastPos token.Pos = fn.Body.Lbrace
	if functionHasGoto(fn) {
		prevStmt = TranspileGotoStatementList(out, fn.Body.List, fn.Type, fileSet, comments, &lastPos, bodyIndent)
		if lastStmt := lastNonEmptyStmt(fn.Body.List); lastStmt != nil && stmtTerminates(lastStmt) {
			out.WriteString(bodyIndent)
			out.WriteString("unreachable!()\n")
		}
	} else {
		for i, stmt := range fn.Body.List {
			out.WriteString(bodyIndent)
			if i == len(fn.Body.List)-1 {
				TranspileTailStatement(out, stmt, fn.Type, fileSet, comments, &lastPos, bodyIndent)
			} else {
				TranspileStatement(out, stmt, fn.Type, fileSet, comments, &lastPos, bodyIndent)
			}
			out.WriteString("\n")
			prevStmt = stmt
		}
	}

	lastTerminates := prevStmt != nil && stmtTerminates(prevStmt)
	if currentFunctionHasDefer && !lastTerminates {
		out.WriteString("\n")
		out.WriteString(bodyIndent)
		out.WriteString("// Execute deferred functions\n")
		out.WriteString(bodyIndent)
		out.WriteString("while let Some(f) = __defer_stack.pop() {\n")
		out.WriteString(bodyIndent)
		out.WriteString("    f();\n")
		out.WriteString(bodyIndent)
		out.WriteString("}\n")
	}

	if recoverCatch {
		finishPanicRecoverCatch(out, fn, "        ")
	}

	out.WriteString("    }\n")

	// Clear the receiver name
	currentReceiver = ""
	currentReceiverObject = nil
}

func setCurrentReceiverFromIdent(ident *ast.Ident) {
	currentReceiver = ident.Name
	currentReceiverObject = nil
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
		currentReceiverObject = typeInfo.info.Defs[ident]
	}
}
