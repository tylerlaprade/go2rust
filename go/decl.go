package main

import (
	"fmt"
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
	"math"
	"strconv"
	"strings"
)

// generateStructDisplay generates a Display implementation for a struct to match Go's output format
func generateStructDisplay(out *strings.Builder, structName string, structType *ast.StructType) {
	TrackImport("Display")
	TrackImport("Formatter")
	rustStructName := RustTypeNameForUse(structName)

	// If this type implements the error interface, Display should delegate to error()
	if IsErrorImplType(structName) {
		out.WriteString("impl std::fmt::Display for ")
		out.WriteString(rustStructName)
		out.WriteString(" {\n")
		out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
		out.WriteString("        write!(f, \"{}\", (*self.error()")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()))\n")
		out.WriteString("    }\n")
		out.WriteString("}\n")
		return
	}

	if namedTypeHasGoStringMethod(structName) {
		out.WriteString("impl std::fmt::Display for ")
		out.WriteString(rustStructName)
		out.WriteString(" {\n")
		out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
		out.WriteString("        write!(f, \"{}\", (*self.string()")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()))\n")
		out.WriteString("    }\n")
		out.WriteString("}\n")
		return
	}

	out.WriteString("impl std::fmt::Display for ")
	out.WriteString(rustStructName)
	out.WriteString(" {\n")
	out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
	out.WriteString("        write!(f, \"{{")

	// Collect all fields (including embedded)
	type fieldEntry struct {
		name           string
		isEmbedded     bool
		isSlice        bool
		isMap          bool
		isInterface    bool
		isFunction     bool
		funcSlice      bool
		hasTrait       bool
		mapOpaque      bool
		nestedSlice    bool
		ptrSlice       bool
		interfaceSlice bool
	}
	var fields []fieldEntry
	for _, field := range structType.Fields.List {
		// Skip sync types (Mutex, WaitGroup) — they're not data fields
		if isSyncParam(field.Type) {
			continue
		}
		_, isSlice := field.Type.(*ast.ArrayType)
		_, isMap := field.Type.(*ast.MapType)
		isInterface := isEmptyInterfaceExpr(field.Type)
		isFunction := isFunctionSignatureTypeExpr(field.Type)
		funcSlice := arrayFieldContainsFunction(field.Type)
		_, isChannel := field.Type.(*ast.ChanType)
		hasTrait := typeHasTraitField(field.Type)
		mapOpaque := mapFieldNeedsOpaqueDisplay(field.Type)
		nestedSlice := arrayFieldContainsSlice(field.Type)
		ptrSlice := arrayFieldContainsPointer(field.Type)
		interfaceSlice := arrayFieldContainsLocalInterface(field.Type)
		if isChannel {
			continue
		}
		if len(field.Names) > 0 {
			for _, name := range field.Names {
				fields = append(fields, fieldEntry{
					name:           name.Name,
					isEmbedded:     false,
					isSlice:        isSlice,
					isMap:          isMap,
					isInterface:    isInterface,
					isFunction:     isFunction,
					funcSlice:      funcSlice,
					hasTrait:       hasTrait,
					mapOpaque:      mapOpaque,
					nestedSlice:    nestedSlice,
					ptrSlice:       ptrSlice,
					interfaceSlice: interfaceSlice,
				})
			}
		} else {
			// Embedded field
			typeName := getEmbeddedFieldName(field.Type)
			fields = append(fields, fieldEntry{
				name:           typeName,
				isEmbedded:     true,
				isSlice:        isSlice,
				isMap:          isMap,
				isInterface:    isInterface,
				isFunction:     isFunction,
				funcSlice:      funcSlice,
				hasTrait:       hasTrait,
				mapOpaque:      mapOpaque,
				nestedSlice:    nestedSlice,
				ptrSlice:       ptrSlice,
				interfaceSlice: interfaceSlice,
			})
		}
	}

	// Generate format string with placeholders
	for i := range fields {
		if i > 0 {
			out.WriteString(" ")
		}
		out.WriteString("{}")
	}
	out.WriteString("}}\"")

	// Add field values
	for _, f := range fields {
		out.WriteString(", ")
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
		} else if f.nestedSlice {
			NeedFormatNestedSlice()
			out.WriteString("format_nested_slice(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.ptrSlice {
			NeedFormatSlice()
			out.WriteString("format_slice_wrapped(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.interfaceSlice {
			NeedFormatSliceWrappedStringer()
			out.WriteString("format_slice_wrapped_stringer(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
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

	out.WriteString(")\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func generateStructDebug(out *strings.Builder, structName string) {
	rustStructName := RustTypeNameForUse(structName)
	out.WriteString("impl std::fmt::Debug for ")
	out.WriteString(rustStructName)
	out.WriteString(" {\n")
	out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
	out.WriteString("        write!(f, \"{}\", self)\n")
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

func arrayFieldContainsFunction(expr ast.Expr) bool {
	arrayType, ok := expr.(*ast.ArrayType)
	if !ok {
		return false
	}
	return isFunctionSignatureTypeExpr(arrayType.Elt)
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
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		if _, isSlice := types.Unalias(named.Underlying()).(*types.Slice); isSlice {
			return false
		}
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Interface, *types.Pointer, *types.Signature, *types.Slice, *types.Map:
		return true
	default:
		return false
	}
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

func writeStructDerive(out *strings.Builder, structName string, structType *ast.StructType) {
	hasTraitField := structHasTraitField(structType)
	canDeriveDebug := !hasTraitField && structCanDeriveDebug(structType)
	needsCustomDefault := structNeedsCustomDefault(structType)
	needsPartialEq := !hasTraitField && structName != "" && comparableStructTypes[structName]
	derivePartialEq := needsPartialEq && !structNeedsCustomPartialEq(structName, structType)
	deriveOrd := derivePartialEq && structNeedsOrd(structName)

	var traits []string
	if canDeriveDebug {
		traits = append(traits, "Debug")
	}
	traits = append(traits, "Clone")
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

func structComparableFieldNames(structType *ast.StructType) []string {
	var fields []string
	if structType == nil {
		return fields
	}
	for _, field := range structType.Fields.List {
		if len(field.Names) > 0 {
			for _, name := range field.Names {
				fields = append(fields, ToSnakeCase(name.Name))
			}
			continue
		}
		fields = append(fields, ToSnakeCase(getEmbeddedFieldName(field.Type)))
	}
	return fields
}

func generateStructPartialEq(out *strings.Builder, structName string, structType *ast.StructType) {
	if !structNeedsCustomPartialEq(structName, structType) {
		return
	}

	rustStructName := RustTypeNameForUse(structName)
	out.WriteString("impl PartialEq for ")
	out.WriteString(rustStructName)
	out.WriteString(" {\n")
	out.WriteString("    fn eq(&self, other: &Self) -> bool {\n")

	fields := structComparableFieldNames(structType)
	if len(fields) == 0 {
		out.WriteString("        true\n")
	} else {
		out.WriteString("        (\n")
		for i, fieldName := range fields {
			if i > 0 {
				out.WriteString("\n                && ")
			} else {
				out.WriteString("            ")
			}
			out.WriteString("{ let __left = self.")
			out.WriteString(fieldName)
			out.WriteString(".lock().unwrap(); let __right = other.")
			out.WriteString(fieldName)
			out.WriteString(".lock().unwrap(); __left.as_ref() == __right.as_ref() }")
		}
		out.WriteString("\n        )\n")
	}

	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func generateStructOrd(out *strings.Builder, structName string, structType *ast.StructType) {
	if !structNeedsCustomOrd(structName, structType) {
		return
	}

	rustStructName := RustTypeNameForUse(structName)
	out.WriteString("\nimpl Eq for ")
	out.WriteString(rustStructName)
	out.WriteString(" {}\n")

	out.WriteString("\nimpl PartialOrd for ")
	out.WriteString(rustStructName)
	out.WriteString(" {\n")
	out.WriteString("    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {\n")
	out.WriteString("        Some(self.cmp(other))\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")

	out.WriteString("\nimpl Ord for ")
	out.WriteString(rustStructName)
	out.WriteString(" {\n")
	out.WriteString("    fn cmp(&self, other: &Self) -> std::cmp::Ordering {\n")

	fields := structComparableFieldNames(structType)
	for _, fieldName := range fields {
		out.WriteString("        {\n")
		out.WriteString("            let __left = { self.")
		out.WriteString(fieldName)
		out.WriteString(".lock().unwrap().as_ref().cloned() };\n")
		out.WriteString("            let __right = { other.")
		out.WriteString(fieldName)
		out.WriteString(".lock().unwrap().as_ref().cloned() };\n")
		out.WriteString("            match __left.cmp(&__right) {\n")
		out.WriteString("                std::cmp::Ordering::Equal => {}\n")
		out.WriteString("                __ord => return __ord,\n")
		out.WriteString("            }\n")
		out.WriteString("        }\n")
	}
	out.WriteString("        std::cmp::Ordering::Equal\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func generateStructValueClone(out *strings.Builder, structName string, structType *ast.StructType) {
	if structType == nil {
		return
	}
	rustStructName := RustTypeNameForUse(structName)
	out.WriteString("impl ")
	out.WriteString(rustStructName)
	out.WriteString(" {\n")
	out.WriteString("    pub fn __go_value_clone(&self) -> Self {\n")
	out.WriteString("        Self { ")
	needComma := false
	for _, field := range structType.Fields.List {
		fieldNames := field.Names
		if len(fieldNames) == 0 {
			fieldNames = []*ast.Ident{ast.NewIdent(getEmbeddedFieldName(field.Type))}
		}
		for _, name := range fieldNames {
			if needComma {
				out.WriteString(", ")
			}
			needComma = true
			fieldName := ToSnakeCase(name.Name)
			out.WriteString(fieldName)
			out.WriteString(": ")
			writeStructCloneField(out, fieldName, field.Type)
		}
	}
	out.WriteString(" }\n")
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
		out.WriteString(goTypeToRustBase(fieldType))
		out.WriteString("::new()")
		return
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if typ := typeInfo.GetType(fieldType); typ != nil {
			if structFieldHasNilZero(typ) {
				WriteWrappedNone(out)
				return
			}
			if _, isChan := types.Unalias(typ).Underlying().(*types.Chan); isChan {
				out.WriteString("Default::default()")
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

func generateStructDefault(out *strings.Builder, structName string, structType *ast.StructType) {
	if !structNeedsCustomDefault(structType) {
		return
	}
	out.WriteString("\nimpl Default for ")
	out.WriteString(RustTypeNameForUse(structName))
	out.WriteString(" {\n")
	out.WriteString("    fn default() -> Self {\n")
	out.WriteString("        Self { ")
	needComma := false
	for _, field := range structType.Fields.List {
		if len(field.Names) > 0 {
			for _, name := range field.Names {
				if needComma {
					out.WriteString(", ")
				}
				needComma = true
				out.WriteString(ToSnakeCase(name.Name))
				out.WriteString(": ")
				writeStructDefaultValue(out, field.Type)
			}
		} else {
			if needComma {
				out.WriteString(", ")
			}
			needComma = true
			fieldName := getEmbeddedFieldName(field.Type)
			out.WriteString(ToSnakeCase(fieldName))
			out.WriteString(": ")
			writeStructDefaultValue(out, field.Type)
		}
	}
	out.WriteString(" }\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func shouldGenerateJsonDecodeImpl() bool {
	return currentContext != nil && currentContext.UsePackageExternalStubs
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

func generateStructJsonDecode(out *strings.Builder, structName string, structType *ast.StructType) {
	if !shouldGenerateJsonDecodeImpl() || structType == nil {
		return
	}
	out.WriteString("\nimpl GoJsonDecode for ")
	out.WriteString(RustTypeNameForUse(structName))
	out.WriteString(" {\n")
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
			out.WriteString(" = <")
			out.WriteString(GoTypeToRust(field.Type))
			out.WriteString(" as GoJsonDecode>::go_json_decode(field_value)?;\n")
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
		if typ := typeInfo.GetType(expr); typ != nil && typeReferencesExternalNamedType(typ) {
			return false
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
			if typeInfo := GetTypeInfo(); typeInfo == nil || typeInfo.pkg == nil || t.Obj().Pkg() != typeInfo.pkg {
				if !isStdlibPackage(t.Obj().Pkg().Path()) {
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

func isOsArgsSelector(sel *ast.SelectorExpr) bool {
	if sel.Sel.Name != "Args" {
		return false
	}

	ident, ok := sel.X.(*ast.Ident)
	if !ok {
		return false
	}

	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
		if obj, ok := typeInfo.info.Uses[ident]; ok {
			if pkgName, ok := obj.(*types.PkgName); ok {
				return pkgName.Imported().Path() == "os"
			}
		}
	}

	return resolveStdlibPackageName(ident.Name) == "os"
}

func functionUsesOsArgs(fn *ast.FuncDecl) bool {
	if fn.Body == nil {
		return false
	}

	usesOsArgs := false
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		if usesOsArgs {
			return false
		}
		if sel, ok := node.(*ast.SelectorExpr); ok && isOsArgsSelector(sel) {
			usesOsArgs = true
			return false
		}
		return true
	})
	return usesOsArgs
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

// localInterfaceTypesByName looks up the go/types interface for a top-level
// type name in the current package.
func localInterfaceTypesByName(name string) *types.Interface {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil {
		return nil
	}
	obj := typeInfo.pkg.Scope().Lookup(name)
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
		traitName, ok := transpiledNamedInterfaceTypeNameFromExpr(field.Type)
		if !ok {
			continue
		}
		traitSnake := traitMethodSuffix(traitName)
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
			WriteWrapperPrefix(out)
			out.WriteString(RustLocalIdent(name.Name))
			out.WriteString(".__go_clone_box_")
			out.WriteString(traitSnake)
			out.WriteString("()")
			WriteWrapperSuffix(out)
			out.WriteString(";\n")
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
					registerStructDef(typeSpec.Name.Name, structType)
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
		RegisterTypeDefinition(typeSpec.Name.Name, typeDefinitionUnderlyingName(typeSpec.Type))
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
	out.WriteString("(")

	// Parameters
	if fn.Type.Params != nil {
		for i, field := range fn.Type.Params.List {
			if i > 0 {
				out.WriteString(", ")
			}
			for j, name := range field.Names {
				if j > 0 {
					out.WriteString(", ")
				}
				if blockIdentAssigned(fn.Body, name.Name) {
					out.WriteString("mut ")
				}
				out.WriteString(RustLocalIdent(name.Name))
				out.WriteString(": ")
				out.WriteString(GoTypeToRustParam(field.Type))
			}
		}
	}

	out.WriteString(")")

	// Return type
	if fn.Type.Results != nil && len(fn.Type.Results.List) > 0 {
		out.WriteString(" -> ")
		if len(fn.Type.Results.List) == 1 && len(fn.Type.Results.List[0].Names) <= 1 {
			// Single return value
			out.WriteString(GoTypeToRust(fn.Type.Results.List[0].Type))
		} else {
			// Multiple return values - use tuple
			out.WriteString("(")
			first := true
			for _, result := range fn.Type.Results.List {
				// Handle multiple names with same type
				if len(result.Names) > 0 {
					for range result.Names {
						if !first {
							out.WriteString(", ")
						}
						first = false
						out.WriteString(GoTypeToRust(result.Type))
					}
				} else {
					// Unnamed return value
					if !first {
						out.WriteString(", ")
					}
					first = false
					out.WriteString(GoTypeToRust(result.Type))
				}
			}
			out.WriteString(")")
		}
	}

	out.WriteString(" {\n")

	restoreLocalSyntaxInfo := pushFunctionLocalSyntaxInfo()
	defer restoreLocalSyntaxInfo()
	assignedInterfaceParams := assignedInterfaceParamNames(fn)

	// Register function parameters in VarTable
	if vt := GetVarTable(); vt != nil {
		vt.PushScope()
		defer vt.PopScope()
		if fn.Type.Params != nil {
			for _, field := range fn.Type.Params.List {
				for _, name := range field.Names {
					rustType := goTypeToRustBase(field.Type)
					if functionRustType, ok := functionTypeRustNameFromTypeExpr(field.Type); ok {
						rustType = functionRustType
					}
					registerTypeExprCollectionInfo(name.Name, field.Type)
					if varInfo, ok := interfaceParamVarInfo(field.Type); ok {
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
					} else {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapFull,
							RustType:  rustType,
							Source:    SourceParam,
						})
					}
				}
			}
		}
	}

	// Call package initialization at the start of main() if present
	if fn.Name.Name == "main" && hasInitFunction {
		out.WriteString("    __go_init_all();\n")
	}

	if functionUsesOsArgs(fn) {
		out.WriteString("    let __go_os_args = ")
		WriteWrapperPrefix(out)
		out.WriteString("std::env::args().collect::<Vec<String>>()")
		WriteWrapperSuffix(out)
		out.WriteString(";\n\n")
	}
	writeAssignedInterfaceParamShadows(out, fn, "    ")

	if fn.Body == nil {
		out.WriteString("    unimplemented!(\"Go function declaration has no body\");\n")
		out.WriteString("}\n")
		return
	}

	restoreSliceElemPtrCandidates := setSliceElemPtrCandidates(fn.Body)
	defer restoreSliceElemPtrCandidates()

	// Check if this function uses defer statements
	hasDefer := checkHasDefer(fn.Body.List)
	currentFunctionHasDefer = hasDefer
	oldActiveMutexGuards := activeMutexGuards
	activeMutexGuards = make(map[string]string)
	defer func() { activeMutexGuards = oldActiveMutexGuards }()

	// Initialize defer stack if needed
	if hasDefer {
		out.WriteString("    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();\n")
		// We'll execute defers before each return statement
		out.WriteString("\n")
	}

	// Declare named return values as mutable variables
	if fn.Type.Results != nil {
		for _, result := range fn.Type.Results.List {
			if len(result.Names) > 0 {
				for _, name := range result.Names {
					if name.Name == "_" {
						out.WriteString("    let ")
					} else {
						out.WriteString("    let mut ")
					}
					out.WriteString(RustLocalIdent(name.Name))
					out.WriteString(": ")
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
		if len(fn.Type.Results.List) > 0 {
			out.WriteString("\n")
		}
	}

	// Function body
	var prevStmt ast.Stmt
	var lastPos token.Pos = fn.Body.Lbrace
	if functionHasGoto(fn) {
		prevStmt = TranspileGotoStatementList(out, fn.Body.List, fn.Type, fileSet, comments, &lastPos, "    ")
	} else {
		for _, stmt := range fn.Body.List {
			// Add blank line if there was one in the source
			if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
				out.WriteString("\n")
			}

			out.WriteString("    ")
			TranspileStatement(out, stmt, fn.Type, fileSet, comments, &lastPos, "    ")
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
		out.WriteString("\n    // Execute deferred functions\n")
		out.WriteString("    while let Some(f) = __defer_stack.pop() {\n")
		out.WriteString("        f();\n")
		out.WriteString("    }\n")
	}

	out.WriteString("}")
}

// getEmbeddedFieldName extracts the type name from an embedded field
func getEmbeddedFieldName(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		return t.Name
	case *ast.StarExpr:
		// For pointer types, get the underlying type name
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

func TranspileTypeDecl(out *strings.Builder, typeSpec *ast.TypeSpec, genDecl *ast.GenDecl) {
	rustTypeName := RustTypeNameForUse(typeSpec.Name.Name)
	switch t := typeSpec.Type.(type) {
	case *ast.StructType:
		registerStructDef(typeSpec.Name.Name, t)

		writeStructDerive(out, typeSpec.Name.Name, t)
		out.WriteString("pub struct ")
		out.WriteString(rustTypeName)
		out.WriteString(" {\n")

		for _, field := range t.Fields.List {
			// Add struct tag as comment if present
			if field.Tag != nil && field.Tag.Value != "" {
				out.WriteString("    // tags: ")
				out.WriteString(field.Tag.Value)
				out.WriteString("\n")
			}

			if len(field.Names) > 0 {
				// Handle multiple names on one line (e.g., X, Y int)
				for _, name := range field.Names {
					out.WriteString("    pub ")
					out.WriteString(ToSnakeCase(name.Name))
					out.WriteString(": ")
					out.WriteString(GoTypeToRust(field.Type))
					out.WriteString(",\n")
				}
			} else {
				// Embedded field - extract the type name
				fieldName := getEmbeddedFieldName(field.Type)
				out.WriteString("    pub ")
				out.WriteString(ToSnakeCase(fieldName))
				out.WriteString(": ")
				out.WriteString(GoTypeToRust(field.Type))
				out.WriteString(",\n")
			}
		}

		out.WriteString("}\n\n")

		generateStructValueClone(out, typeSpec.Name.Name, t)
		out.WriteString("\n")

		generateStructDefault(out, typeSpec.Name.Name, t)
		if structNeedsCustomDefault(t) {
			out.WriteString("\n")
		}

		// Generate Display implementation to match Go's format
		generateStructDisplay(out, typeSpec.Name.Name, t)
		if IsErrorImplType(typeSpec.Name.Name) && !structCanDeriveDebug(t) {
			generateStructDebug(out, typeSpec.Name.Name)
		}
		generateStructPartialEq(out, typeSpec.Name.Name, t)
		generateStructOrd(out, typeSpec.Name.Name, t)
		generateStructJsonDecode(out, typeSpec.Name.Name, t)

	case *ast.InterfaceType:
		// Generate a trait for the interface
		// Add Display plus Any so trait object equality can downcast.
		// Embedded named local interfaces become Rust supertraits so that
		// `&dyn SubTrait` upcasts to `&dyn SuperTrait` automatically.
		TrackImport("Any")
		embeddedTraits := embeddedLocalInterfaceNames(t)
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
		traitSnake := ToSnakeCase(rustTypeName)
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
				out.WriteString("(&self")

				// Add other parameters
				if funcType.Params != nil && len(funcType.Params.List) > 0 {
					for _, param := range funcType.Params.List {
						out.WriteString(", ")
						for j, name := range param.Names {
							if j > 0 {
								out.WriteString(", ")
							}
							out.WriteString(RustLocalIdent(name.Name))
							out.WriteString(": ")
							out.WriteString(GoTypeToRustParam(param.Type))
						}
					}
				}

				out.WriteString(")")

				// Return type
				if funcType.Results != nil && len(funcType.Results.List) > 0 {
					out.WriteString(" -> ")
					if len(funcType.Results.List) == 1 && len(funcType.Results.List[0].Names) <= 1 {
						// Single return value
						out.WriteString(GoTypeToRust(funcType.Results.List[0].Type))
					} else {
						// Multiple return values - use tuple
						out.WriteString("(")
						first := true
						for _, result := range funcType.Results.List {
							if len(result.Names) > 0 {
								for range result.Names {
									if !first {
										out.WriteString(", ")
									}
									first = false
									out.WriteString(GoTypeToRust(result.Type))
								}
							} else {
								if !first {
									out.WriteString(", ")
								}
								first = false
								out.WriteString(GoTypeToRust(result.Type))
							}
						}
						out.WriteString(")")
					}
				}

				out.WriteString(";\n")
			}
		}

		out.WriteString("}")
		out.WriteString("\n\nimpl Clone for ")
		out.WriteString(rustLocalInterfaceTraitObject(rustTypeName))
		out.WriteString(" {\n")
		out.WriteString("    fn clone(&self) -> Self {\n")
		out.WriteString("        self.__go_clone_box_")
		out.WriteString(traitSnake)
		out.WriteString("()\n")
		out.WriteString("    }\n")
		out.WriteString("}")

	default:
		// Handle type aliases and type definitions
		if typeSpec.Assign != 0 {
			// Type alias: type A = B
			out.WriteString("pub type ")
			out.WriteString(rustTypeName)
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
			out.WriteString(" = ")
			out.WriteString(functionTypeSpecRustType(typeSpec, t.(*ast.FuncType)))
			out.WriteString(";\n")

			// Track as a type alias so GoTypeToRust won't double-wrap
			RegisterTypeAlias(typeSpec.Name.Name)
			RegisterFunctionTypeAlias(typeSpec.Name.Name)
			RegisterFunctionTypeAliasBox(typeSpec.Name.Name, functionTypeSpecRustBoxType(typeSpec, t.(*ast.FuncType)))
		} else {
			// Type definition: type A B
			// Create a newtype wrapper in Rust
			RegisterTypeDefinition(typeSpec.Name.Name, typeDefinitionUnderlyingName(t))
			out.WriteString("#[derive(Debug, Clone, Default)]\n")
			out.WriteString("pub struct ")
			out.WriteString(rustTypeName)
			out.WriteString("(pub ")
			out.WriteString(GoTypeToRust(t))
			out.WriteString(");\n")

			// Add Display implementation for displayable scalar type definitions
			if IsStringerImplType(typeSpec.Name.Name) {
				TrackImport("Display")
				TrackImport("Formatter")

				out.WriteString("\nimpl Display for ")
				out.WriteString(rustTypeName)
				out.WriteString(" {\n")
				out.WriteString("    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n")
				out.WriteString("        write!(f, \"{}\", (*self.string()")
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()))\n")
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
			} else if ident, ok := t.(*ast.Ident); ok {
				// Add Display impl when the underlying Rust type is displayable.
				if isDisplayableDefinedUnderlying(ident.Name) {
					// Track necessary imports
					TrackImport("Display")
					TrackImport("Formatter")
					TrackImport("fmt")

					out.WriteString("\nimpl Display for ")
					out.WriteString(rustTypeName)
					out.WriteString(" {\n")
					out.WriteString("    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n")
					if IsStringerImplType(typeSpec.Name.Name) {
						out.WriteString("        write!(f, \"{}\", (*self.string()")
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()))\n")
					} else {
						out.WriteString("        write!(f, \"{}\", self.0")
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap())\n")
					}
					out.WriteString("    }\n")
					out.WriteString("}\n")
				}
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
			tag := jsonStructTagFromSyntax(field)
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
	out.WriteString("        self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap() == other.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func writeScalarTypeDefinitionNumericOps(out *strings.Builder, typeName string, rustType string, underlying string) {
	rustTypeName := RustTypeNameForUse(typeName)
	writeScalarTypeDefinitionPartialOrd(out, rustTypeName, rustType)
	writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "Add", "add", "+", true)
	writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "Sub", "sub", "-", true)
	if isBitwiseDefinedUnderlying(underlying) {
		writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "BitAnd", "bitand", "&", true)
		writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "BitOr", "bitor", "|", true)
		writeScalarTypeDefinitionBinaryOp(out, rustTypeName, rustType, "BitXor", "bitxor", "^", true)
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
	out.WriteString("        self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap().partial_cmp(other.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap())\n")
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
	out.WriteString(rustType)
	out.WriteString(";\n")
	out.WriteString("    fn ")
	out.WriteString(methodName)
	out.WriteString("(self, other: ")
	out.WriteString(rustType)
	out.WriteString(") -> ")
	out.WriteString(rustType)
	out.WriteString(" {\n")
	out.WriteString("        *self.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap() ")
	out.WriteString(op)
	out.WriteString(" other\n")
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
	out.WriteString(rustType)
	out.WriteString(";\n")
	out.WriteString("    fn ")
	out.WriteString(methodName)
	out.WriteString("(self, other: ")
	out.WriteString(rustTypeName)
	out.WriteString(") -> ")
	out.WriteString(rustType)
	out.WriteString(" {\n")
	out.WriteString("        self ")
	out.WriteString(op)
	out.WriteString(" *other.0")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()\n")
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
				if valueSpec.Type != nil {
					constType = rustConstTypeForTypeExpr(valueSpec.Type)
					constTypeName, _ = constDeclaredNamedType(valueSpec.Type)
				} else if len(valueSpec.Values) == 0 && lastType != nil {
					constType = rustConstTypeForTypeExpr(lastType)
					constTypeName, _ = constDeclaredNamedType(lastType)
				} else if inferredType, ok := rustConstTypeForConstObject(name); ok {
					constType = inferredType
				} else if len(valueSpec.Values) > i && valueSpec.Values[i] != nil {
					constType = inferConstType(valueSpec.Values[i])
				} else if len(lastExpressions) > i && lastExpressions[i] != nil {
					constType = inferConstType(lastExpressions[i])
				} else {
					constType = "i32"
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
						TranspileConstExpr(out, valueSpec.Values[i], iotaValue)
					}
				} else if len(lastExpressions) > i && lastExpressions[i] != nil {
					// Use the corresponding expression from lastExpressions for this position
					if !writeExternalNamedIntegerConstValue(out, name) {
						TranspileConstExpr(out, lastExpressions[i], iotaValue)
					}
				} else if len(lastExpressions) > 0 && lastExpressions[0] != nil {
					// If we don't have an expression for this position, use the first one
					if !writeExternalNamedIntegerConstValue(out, name) {
						TranspileConstExpr(out, lastExpressions[0], iotaValue)
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

func rustConstTypeForConstObject(name *ast.Ident) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || name == nil {
		return "", false
	}
	obj, ok := typeInfo.GetObject(name).(*types.Const)
	if !ok || obj.Type() == nil {
		return "", false
	}
	return rustConstTypeForGoTypesType(obj.Type())
}

func evalIntConstExpr(expr ast.Expr) (int64, bool) {
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
		case token.SUB:
			return -v, true
		case token.XOR:
			return ^v, true
		}
	case *ast.ParenExpr:
		return evalIntConstExpr(e.X)
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
			out.WriteString(e.Value)
			out.WriteString(" as i32)")
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
			TranspileConstExpr(out, e.X, iotaValue)
			out.WriteString(" ")
			out.WriteString(e.Op.String())
			out.WriteString(" ")
			TranspileConstExpr(out, e.Y, iotaValue)
		}
	case *ast.ParenExpr:
		out.WriteString("(")
		TranspileConstExpr(out, e.X, iotaValue)
		out.WriteString(")")
	default:
		// Fallback to regular expression transpilation
		TranspileExpression(out, expr)
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
	transpileMethodImplWithVisibility(out, fn, true, false, fileSet, comments)
}

func TranspileTraitMethodImpl(out *strings.Builder, fn *ast.FuncDecl, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	transpileMethodImplWithVisibility(out, fn, false, true, fileSet, comments)
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
		TranspileTraitMethodImpl(out, method, fileSet, comments)
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
	if fn.Type.Results != nil && len(fn.Type.Results.List) > 0 {
		out.WriteString(" -> ")
		if len(fn.Type.Results.List) == 1 && len(fn.Type.Results.List[0].Names) <= 1 {
			out.WriteString(GoTypeToRust(fn.Type.Results.List[0].Type))
		} else {
			out.WriteString("(")
			first := true
			for _, result := range fn.Type.Results.List {
				count := len(result.Names)
				if count == 0 {
					count = 1
				}
				for i := 0; i < count; i++ {
					if !first {
						out.WriteString(", ")
					}
					first = false
					out.WriteString(GoTypeToRust(result.Type))
				}
			}
			out.WriteString(")")
		}
	}
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
				writeNamedReturnZeroValue(out, result.Type)
			} else {
				out.WriteString(RustLocalIdent(name.Name))
			}
		}
	}
	if len(names) > 1 {
		out.WriteString(")")
	}
}

func writeNamedReturnZeroValue(out *strings.Builder, typeExpr ast.Expr) {
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

var packageMethodReceiverMutability = make(map[string]bool)

func resetPackageMethodReceiverMutability() {
	packageMethodReceiverMutability = make(map[string]bool)
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

func transpileMethodImplWithVisibility(out *strings.Builder, fn *ast.FuncDecl, addPub bool, forceSharedReceiver bool, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	// Store the receiver name and type for self translation
	if fn.Recv != nil && len(fn.Recv.List) > 0 {
		recv := fn.Recv.List[0]
		if len(recv.Names) > 0 {
			currentReceiver = recv.Names[0].Name
		}
		// Store the receiver type
		currentReceiverType = getReceiverType(recv.Type)
	}

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
			currentReceiver = recv.Names[0].Name
		}

		if forceSharedReceiver {
			out.WriteString("&self")
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
	if fn.Type.Params != nil {
		for i, field := range fn.Type.Params.List {
			if i > 0 {
				out.WriteString(", ")
			}
			for j, name := range field.Names {
				if j > 0 {
					out.WriteString(", ")
				}
				if blockIdentAssigned(fn.Body, name.Name) {
					out.WriteString("mut ")
				}
				out.WriteString(RustLocalIdent(name.Name))
				out.WriteString(": ")
				out.WriteString(GoTypeToRustParam(field.Type))
			}
		}
	}

	out.WriteString(")")

	// Return type
	if fn.Type.Results != nil && len(fn.Type.Results.List) > 0 {
		out.WriteString(" -> ")
		if len(fn.Type.Results.List) == 1 && len(fn.Type.Results.List[0].Names) <= 1 {
			// Single return value
			out.WriteString(GoTypeToRust(fn.Type.Results.List[0].Type))
		} else {
			// Multiple return values - use tuple
			out.WriteString("(")
			first := true
			for _, result := range fn.Type.Results.List {
				// Handle multiple names with same type
				if len(result.Names) > 0 {
					for range result.Names {
						if !first {
							out.WriteString(", ")
						}
						first = false
						out.WriteString(GoTypeToRust(result.Type))
					}
				} else {
					// Unnamed return value
					if !first {
						out.WriteString(", ")
					}
					first = false
					out.WriteString(GoTypeToRust(result.Type))
				}
			}
			out.WriteString(")")
		}
	}

	out.WriteString(" {\n")

	if fn.Body != nil {
		restoreSliceElemPtrCandidates := setSliceElemPtrCandidates(fn.Body)
		defer restoreSliceElemPtrCandidates()

		hasDefer := checkHasDefer(fn.Body.List)
		oldFunctionHasDefer := currentFunctionHasDefer
		currentFunctionHasDefer = hasDefer
		defer func() { currentFunctionHasDefer = oldFunctionHasDefer }()
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
			for _, field := range fn.Type.Params.List {
				for _, name := range field.Names {
					rustType := goTypeToRustBase(field.Type)
					if functionRustType, ok := functionTypeRustNameFromTypeExpr(field.Type); ok {
						rustType = functionRustType
					}
					registerTypeExprCollectionInfo(name.Name, field.Type)
					if varInfo, ok := interfaceParamVarInfo(field.Type); ok {
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
					} else {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapFull,
							RustType:  rustType,
							Source:    SourceParam,
						})
					}
				}
			}
		}
	}

	// Method body - need to handle self references
	if fn.Body == nil {
		out.WriteString("        unimplemented!(\"Go method declaration has no body\");\n")
		out.WriteString("    }\n")
		currentReceiver = ""
		return
	}
	writeAssignedInterfaceParamShadows(out, fn, "        ")

	var prevStmt ast.Stmt
	var lastPos token.Pos = fn.Body.Lbrace
	for _, stmt := range fn.Body.List {
		out.WriteString("        ")
		TranspileStatement(out, stmt, fn.Type, fileSet, comments, &lastPos, "        ")
		out.WriteString("\n")
		prevStmt = stmt
	}

	lastTerminates := prevStmt != nil && stmtTerminates(prevStmt)
	if currentFunctionHasDefer && !lastTerminates {
		out.WriteString("\n        // Execute deferred functions\n")
		out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
		out.WriteString("            f();\n")
		out.WriteString("        }\n")
	}

	out.WriteString("    }\n")

	// Clear the receiver name
	currentReceiver = ""
}
