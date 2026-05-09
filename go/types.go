package main

import (
	"fmt"
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
	"strconv"
	"strings"
)

// Track anonymous struct definitions
var anonymousStructCounter = 0
var anonymousStructs = make(map[string]*ast.StructType)
var anonymousStructTypeMap = make(map[string]string) // maps struct signature to type name

func isEmptyInterfaceExpr(expr ast.Expr) bool {
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "any" {
		return true
	}
	intf, ok := expr.(*ast.InterfaceType)
	return ok && (intf.Methods == nil || len(intf.Methods.List) == 0)
}

func isEmptyInterfaceTypeExpr(expr ast.Expr) bool {
	if isEmptyInterfaceExpr(expr) {
		return true
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && isEmptyInterfaceType(typeInfo.GetType(expr))
}

func isEmptyInterfaceType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	intf, ok := typ.Underlying().(*types.Interface)
	return ok && intf.NumMethods() == 0
}

func localNamedInterfaceTypeName(typ types.Type) (string, bool) {
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil {
		return "", false
	}
	name := named.Obj().Name()
	if !IsInterfaceType(name) {
		return "", false
	}
	_, ok = named.Underlying().(*types.Interface)
	return name, ok
}

func localNamedInterfaceTypeNameFromTypes(typ types.Type) (string, bool) {
	if typ == nil {
		return "", false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil {
		return "", false
	}
	intf, ok := named.Underlying().(*types.Interface)
	if !ok || intf.NumMethods() == 0 {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || named.Obj().Pkg() != typeInfo.pkg {
		return "", false
	}
	return named.Obj().Name(), true
}

func transpiledNamedInterfaceTypeNameFromTypes(typ types.Type) (string, bool) {
	if typ == nil {
		return "", false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil {
		return "", false
	}
	intf, ok := named.Underlying().(*types.Interface)
	if !ok || intf.NumMethods() == 0 {
		return "", false
	}
	obj := named.Obj()
	if obj.Pkg() == nil {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil && obj.Pkg() == typeInfo.pkg {
		return obj.Name(), true
	}
	if isStdlibPackage(obj.Pkg().Path()) {
		return "", false
	}
	return goTypesNamedTypeToRust(named), true
}

func transpiledNamedInterfaceTypeNameFromExpr(expr ast.Expr) (string, bool) {
	if ident, ok := expr.(*ast.Ident); ok && IsInterfaceType(ident.Name) {
		return ident.Name, true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", false
	}
	return transpiledNamedInterfaceTypeNameFromTypes(typeInfo.GetType(expr))
}

func localInterfaceNameFromTypeExpr(expr ast.Expr) (string, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || !IsInterfaceType(ident.Name) {
		return "", false
	}
	return ident.Name, true
}

func rustAnyTraitObject() string {
	TrackImport("Any")
	if NeedsConcurrentWrapper() {
		return "Box<dyn Any + Send + Sync>"
	}
	return "Box<dyn Any>"
}

func rustLocalInterfaceTraitObject(name string) string {
	if NeedsConcurrentWrapper() {
		return "Box<dyn " + name + " + Send + Sync>"
	}
	return "Box<dyn " + name + ">"
}

func rustLocalInterfaceParam(name string) string {
	if NeedsConcurrentWrapper() {
		return "&(dyn " + name + " + Send + Sync)"
	}
	return "&dyn " + name
}

func goTypeParamConstraintToRust(t types.Type) (string, bool) {
	tp, ok := types.Unalias(t).(*types.TypeParam)
	if !ok || tp.Constraint() == nil {
		return "", false
	}
	iface, ok := tp.Constraint().Underlying().(*types.Interface)
	if !ok {
		return "", false
	}
	for i := 0; i < iface.NumEmbeddeds(); i++ {
		switch embedded := types.Unalias(iface.EmbeddedType(i)).(type) {
		case *types.Union:
			if embedded.Len() == 0 {
				continue
			}
			return goTypesTypeToRust(embedded.Term(0).Type()), true
		default:
			return goTypesTypeToRust(embedded), true
		}
	}
	return "", false
}

// getStructSignature creates a unique signature for a struct type based on its fields
func getStructSignature(structType *ast.StructType) string {
	var sig strings.Builder
	sig.WriteString("struct{")
	for i, field := range structType.Fields.List {
		if i > 0 {
			sig.WriteString(";")
		}
		// Add field names
		for j, name := range field.Names {
			if j > 0 {
				sig.WriteString(",")
			}
			sig.WriteString(name.Name)
		}
		sig.WriteString(":")
		// Add field type - need to handle nested structs specially
		if nestedStruct, ok := field.Type.(*ast.StructType); ok {
			// For nested anonymous structs, include their full signature
			sig.WriteString(getStructSignature(nestedStruct))
		} else {
			// For other types, just use the type string representation
			sig.WriteString(fmt.Sprintf("%T", field.Type))
		}
	}
	sig.WriteString("}")
	return sig.String()
}

// generateAnonymousStructType generates a unique type name for an anonymous struct
func generateAnonymousStructType(structType *ast.StructType) string {
	// Check if we've already generated a type for this struct signature
	sig := getStructSignature(structType)
	if typeName, exists := anonymousStructTypeMap[sig]; exists {
		return typeName
	}

	// Generate a new type name for this struct
	anonymousStructCounter++
	typeName := fmt.Sprintf("AnonymousStruct%d", anonymousStructCounter)
	anonymousStructs[typeName] = structType
	anonymousStructTypeMap[sig] = typeName

	// Process nested structs in fields to ensure they're also generated
	for _, field := range structType.Fields.List {
		if nestedStruct, ok := field.Type.(*ast.StructType); ok {
			// Recursively generate type for nested struct
			generateAnonymousStructType(nestedStruct)
		}
	}

	return typeName
}

// lookupAnonymousStructName finds the anonymous struct name for a *types.Struct
// by matching field names against registered anonymous structs.
func lookupAnonymousStructName(structType *types.Struct) string {
	numFields := structType.NumFields()
	for anonName, anonAST := range anonymousStructs {
		// Count AST fields (expand grouped names)
		astFieldCount := 0
		for _, field := range anonAST.Fields.List {
			if len(field.Names) == 0 {
				astFieldCount++ // embedded field
			} else {
				astFieldCount += len(field.Names)
			}
		}
		if astFieldCount != numFields {
			continue
		}
		// Compare field names in order
		match := true
		idx := 0
		for _, field := range anonAST.Fields.List {
			for _, name := range field.Names {
				if idx >= numFields || structType.Field(idx).Name() != name.Name {
					match = false
					break
				}
				idx++
			}
			if !match {
				break
			}
		}
		if match && idx == numFields {
			return anonName
		}
	}
	return ""
}

// GoTypeToRustParam generates Rust type for function parameters
// Interface parameters are not wrapped to avoid trait object issues
func GoTypeToRustParam(expr ast.Expr) string {
	if interfaceName, ok := transpiledNamedInterfaceTypeNameFromExpr(expr); ok {
		return rustLocalInterfaceParam(interfaceName)
	}

	// For non-interface types, use regular wrapping
	return GoTypeToRust(expr)
}

func GoTypeToRust(expr ast.Expr) string {
	baseType := goTypeToRustBase(expr)

	// Determine wrapper types based on concurrency needs
	outerWrapper := GetOuterWrapperType()
	innerWrapper := GetInnerWrapperType()

	// Track imports for the wrappers we're using
	if NeedsConcurrentWrapper() {
		TrackImport("Arc")
		TrackImport("Mutex")
	} else {
		TrackImport("Rc")
		TrackImport("RefCell")
	}

	// Special case for error type - it's already Option
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "error" {
		return outerWrapper + "<" + innerWrapper + "<" + baseType + ">>"
	}

	// Check if this is a type alias - type aliases are already fully typed
	if ident, ok := expr.(*ast.Ident); ok {
		if IsTypeAlias(ident.Name) {
			// Type alias - already includes wrapper
			return baseType
		}
	}

	// Channel types are not wrapped - GoChannel is already a shared, cloneable type
	if _, isChan := expr.(*ast.ChanType); isChan {
		return baseType
	}

	// sync types are not wrapped - they handle synchronization internally
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "sync" {
			return baseType
		}
	}

	// Wrap everything in appropriate wrapper
	// Don't double-wrap pointers - they're already wrapped
	if _, isPointer := expr.(*ast.StarExpr); !isPointer {
		return outerWrapper + "<" + innerWrapper + "<Option<" + baseType + ">>>"
	}

	return baseType
}

// Generate Rust closure type from Go function type
func generateClosureType(funcType *ast.FuncType) string {
	var paramTypes []string
	if funcType.Params != nil {
		for _, field := range funcType.Params.List {
			paramType := GoTypeToRust(field.Type)
			// Add one param type for each name (or one if no names)
			count := len(field.Names)
			if count == 0 {
				count = 1
			}
			for i := 0; i < count; i++ {
				paramTypes = append(paramTypes, paramType)
			}
		}
	}

	// Determine return type
	var returnType string
	if funcType.Results == nil || len(funcType.Results.List) == 0 {
		returnType = "()"
	} else if len(funcType.Results.List) == 1 && len(funcType.Results.List[0].Names) == 0 {
		// Single unnamed return
		returnType = GoTypeToRust(funcType.Results.List[0].Type)
	} else {
		// Multiple returns or named returns
		var retTypes []string
		for _, field := range funcType.Results.List {
			retType := GoTypeToRust(field.Type)
			count := len(field.Names)
			if count == 0 {
				count = 1
			}
			for i := 0; i < count; i++ {
				retTypes = append(retTypes, retType)
			}
		}
		returnType = "(" + strings.Join(retTypes, ", ") + ")"
	}

	// Build the closure type
	paramsStr := strings.Join(paramTypes, ", ")
	if NeedsConcurrentWrapper() {
		// For concurrent code, closures need Send + Sync
		return fmt.Sprintf("Box<dyn Fn(%s) -> %s + Send + Sync>", paramsStr, returnType)
	} else {
		// For single-threaded code, no Send + Sync requirement
		return fmt.Sprintf("Box<dyn Fn(%s) -> %s>", paramsStr, returnType)
	}
}

func fixedArrayLengthFromTypeInfo(arrayType *ast.ArrayType) (int64, bool) {
	if arrayType == nil || arrayType.Len == nil {
		return 0, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return 0, false
	}
	if tv, ok := typeInfo.info.Types[arrayType.Len]; ok && tv.Value != nil {
		if n, exact := constant.Int64Val(tv.Value); exact && n >= 0 {
			return n, true
		}
	}
	if tv, ok := typeInfo.info.Types[arrayType]; ok && tv.Type != nil {
		if array, ok := types.Unalias(tv.Type).Underlying().(*types.Array); ok {
			return array.Len(), true
		}
	}
	return 0, false
}

func goTypeToRustBase(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		switch t.Name {
		case "string":
			return "String"
		case "int":
			return "i32"
		case "int8":
			return "i8"
		case "int16":
			return "i16"
		case "int32", "rune":
			return "i32"
		case "int64":
			return "i64"
		case "uint":
			return "u32"
		case "uint8", "byte":
			return "u8"
		case "uint16":
			return "u16"
		case "uint32":
			return "u32"
		case "uint64":
			return "u64"
		case "uintptr":
			return "usize"
		case "float32":
			return "f32"
		case "float64":
			return "f64"
		case "complex64":
			TrackImport("num::Complex")
			return "num::Complex<f32>"
		case "complex128":
			TrackImport("num::Complex")
			return "num::Complex<f64>"
		case "bool":
			return "bool"
		case "any":
			return rustAnyTraitObject()
		case "error":
			TrackImport("Error")
			if NeedsConcurrentWrapper() {
				return "Option<Box<dyn StdError + Send + Sync>>"
			}
			return "Option<Box<dyn StdError>>"
		default:
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				if rustType, ok := goTypeParamConstraintToRust(typeInfo.GetType(t)); ok {
					return rustType
				}
			}
			// Check if this is an interface type
			if IsInterfaceType(t.Name) {
				return rustLocalInterfaceTraitObject(RustTypeNameForUse(t.Name))
			}
			return RustTypeNameForUse(t.Name)
		}
	case *ast.InterfaceType:
		// Empty interface{} becomes Box<dyn Any>
		if len(t.Methods.List) == 0 {
			return rustAnyTraitObject()
		}
		return "Unknown"
	case *ast.ArrayType:
		elemType := goTypeToRustBase(t.Elt)
		if t.Len != nil {
			// Fixed-size array
			if lit, ok := t.Len.(*ast.BasicLit); ok {
				return "[" + elemType + "; " + lit.Value + "]"
			}
			if length, ok := fixedArrayLengthFromTypeInfo(t); ok {
				return "[" + elemType + "; " + strconv.FormatInt(length, 10) + "]"
			}
		}
		// Slice
		return "Vec<" + elemType + ">"
	case *ast.MapType:
		TrackImport("BTreeMap")
		keyType := goMapKeyTypeToRustBase(t.Key)
		valueType := GoTypeToRust(t.Value)
		return "BTreeMap<" + keyType + ", " + valueType + ">"
	case *ast.StarExpr:
		// Pointer to sync types → bare type (they handle sharing internally)
		if isSyncParam(t) {
			return goTypeToRustBase(t.X)
		}
		// Pointer type - wrap the base type (not already wrapped)
		innerType := goTypeToRustBase(t.X)
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		return outerWrapper + "<" + innerWrapper + "<Option<" + innerType + ">>>"
	case *ast.FuncType:
		// Function type - generate a closure type
		return generateClosureType(t)
	case *ast.IndexExpr:
		if rustType, ok := goCallableTypeFromTypeInfo(t); ok {
			return rustType
		}
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		return fmt.Sprintf("/* ERROR: Unsupported instantiated generic type */ %s<%s<Option<()>>>", outerWrapper, innerWrapper)
	case *ast.IndexListExpr:
		if rustType, ok := goCallableTypeFromTypeInfo(t); ok {
			return rustType
		}
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		return fmt.Sprintf("/* ERROR: Unsupported instantiated generic type */ %s<%s<Option<()>>>", outerWrapper, innerWrapper)
	case *ast.ChanType:
		elemType := goTypeToRustBase(t.Value)
		return "GoChannel<" + elemType + ">"
	case *ast.SelectorExpr:
		// Package-qualified types like sync.WaitGroup, sync.Mutex
		if ident, ok := t.X.(*ast.Ident); ok {
			if ident.Name == "sync" {
				switch t.Sel.Name {
				case "WaitGroup":
					NeedWaitGroup()
					return "WaitGroup"
				case "Mutex":
					NeedGoMutex()
					return "GoMutex"
				case "Once":
					NeedGoOnce()
					return "GoOnce"
				}
			}
			if ident.Name == "strings" && t.Sel.Name == "Builder" {
				return "String"
			}
			if ident.Name == "unsafe" && t.Sel.Name == "Pointer" {
				return "usize"
			}
			if ident.Name == "time" {
				switch t.Sel.Name {
				case "Time":
					NeedGoTime()
					return "GoTime"
				case "Duration":
					return "std::time::Duration"
				case "Timer":
					return "GoTimer"
				case "Ticker":
					return "GoTicker"
				}
			}
			if ident.Name == "context" && t.Sel.Name == "Context" {
				NeedGoContext()
				return "GoContext"
			}
			if ident.Name == "context" && t.Sel.Name == "CancelFunc" {
				NeedGoContext()
				return "GoCancelFunc"
			}
			if ident.Name == "context" && t.Sel.Name == "CancelCauseFunc" {
				NeedGoContext()
				return "GoCancelCauseFunc"
			}
			if isStdlibPackage(goPackageImports[ident.Name]) {
				if named, ok := namedTypeForTypeExpr(t); ok {
					if sig, ok := signatureFromType(named); ok {
						return signatureToBoxDynFn(sig)
					}
				}
			}
			if rustName, ok := rustTypeNameForImportedPackagePath(goPackageImports[ident.Name], t.Sel.Name); ok {
				return rustName
			}
		}
		rustName := fmt.Sprintf("%s_%s", t.X, t.Sel.Name)
		if ident, ok := t.X.(*ast.Ident); ok && isStdlibPackage(goPackageImports[ident.Name]) {
			RegisterExternalTypeStubForTypeExpr(t, rustName)
		}
		return rustName
	case *ast.Ellipsis:
		// Variadic parameter ...T is treated as []T (slice) in Go
		elemType := goTypeToRustBase(t.Elt)
		return "Vec<" + elemType + ">"
	case *ast.StructType:
		// Anonymous struct type - generate a unique type name
		return generateAnonymousStructType(t)
	default:
		// Unhandled type
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		return fmt.Sprintf("/* TODO: Unhandled type %T */ %s<%s<Option<()>>>", t, outerWrapper, innerWrapper)
	}
}

func goMapKeyTypeToRustBase(expr ast.Expr) string {
	if star, ok := expr.(*ast.StarExpr); ok {
		keyHelper := "GoLocalPtrKey"
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if typ := typeInfo.GetType(expr); typ != nil {
				keyHelper = goPtrKeyHelperNameForType(typ)
			} else {
				NeedGoPtrKey()
			}
		} else {
			NeedGoPtrKey()
		}
		return keyHelper + "<" + goTypeToRustBase(star.X) + ">"
	}
	return goTypeToRustBase(expr)
}

func isStdlibStubBackedType(t types.Type) bool {
	t = types.Unalias(t)
	switch typ := t.(type) {
	case *types.Named:
		return typ.Obj() != nil && typ.Obj().Pkg() != nil && isStdlibPackage(typ.Obj().Pkg().Path())
	case *types.Pointer:
		return isStdlibStubBackedType(typ.Elem())
	default:
		return false
	}
}

func goPtrKeyHelperNameForType(t types.Type) string {
	if ptr, ok := types.Unalias(t).Underlying().(*types.Pointer); ok && isStdlibStubBackedType(ptr.Elem()) &&
		currentContext != nil && currentContext.UsePackageExternalStubs {
		return "GoPtrKey"
	}
	NeedGoPtrKey()
	return "GoLocalPtrKey"
}

func goCallableTypeFromTypeInfo(expr ast.Expr) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return "", false
	}
	sig, ok := signatureFromType(typ)
	if !ok {
		return "", false
	}
	return signatureToBoxDynFn(sig), true
}

// zeroValueForGoType returns the Rust zero value for a Go type expression
func zeroValueForGoType(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		switch t.Name {
		case "string":
			return "String::new()"
		case "int", "int8", "int16", "int32", "int64", "rune":
			return "0"
		case "uint", "uint8", "uint16", "uint32", "uint64", "uintptr", "byte":
			return "0"
		case "float32", "float64":
			return "0.0"
		case "bool":
			return "false"
		default:
			return "Default::default()"
		}
	case *ast.SelectorExpr:
		if ident, ok := t.X.(*ast.Ident); ok {
			if ident.Name == "strings" && t.Sel.Name == "Builder" {
				return "String::new()"
			}
		}
		return "Default::default()"
	case *ast.ArrayType:
		if t.Len != nil {
			return "std::array::from_fn(|_| " + zeroValueForGoType(t.Elt) + ")"
		}
		return "vec![]"
	case *ast.MapType:
		return "BTreeMap::new()"
	default:
		return "Default::default()"
	}
}

func zeroValueForTypesType(typ types.Type) string {
	if typ == nil {
		return "Default::default()"
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		if _, isSlice := types.Unalias(named.Underlying()).(*types.Slice); isSlice {
			return "Default::default()"
		}
		if basic, ok := types.Unalias(named.Underlying()).(*types.Basic); ok {
			zeroValue := zeroValueForBasicType(basic)
			if zeroValue != "" {
				rustType := goTypesNamedTypeToRust(named)
				if _, isExternalInteger := externalIntegerRustTypeForNamed(named); isExternalInteger {
					return rustType + "(" + zeroValue + ")"
				}
				var out strings.Builder
				out.WriteString(rustType)
				out.WriteString("(")
				WriteWrapperPrefix(&out)
				out.WriteString(zeroValue)
				WriteWrapperSuffix(&out)
				out.WriteString(")")
				return out.String()
			}
		}
	}
	switch t := typ.Underlying().(type) {
	case *types.Basic:
		if zeroValue := zeroValueForBasicType(t); zeroValue != "" {
			return zeroValue
		}
		return "Default::default()"
	case *types.Slice:
		return "vec![]"
	case *types.Array:
		return "std::array::from_fn(|_| " + zeroValueForTypesType(t.Elem()) + ")"
	case *types.Map:
		return "BTreeMap::new()"
	default:
		return "Default::default()"
	}
}

func zeroValueForBasicType(t *types.Basic) string {
	if t == nil {
		return ""
	}
	switch t.Kind() {
	case types.String:
		return "String::new()"
	case types.Bool:
		return "false"
	case types.Float32, types.Float64:
		return "0.0"
	case types.Int, types.Int8, types.Int16, types.Int32, types.Int64,
		types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64, types.Uintptr:
		return "0"
	default:
		return ""
	}
}

// isSyncParam checks if a type expression is sync.WaitGroup, sync.Mutex,
// sync.Once, or pointers to them.
func isSyncParam(expr ast.Expr) bool {
	if star, ok := expr.(*ast.StarExpr); ok {
		return isSyncParam(star.X)
	}
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if ident, ok := sel.X.(*ast.Ident); ok && ident.Name == "sync" {
			return isBareSyncTypeName(sel.Sel.Name)
		}
	}
	return false
}

func isBareSyncTypeName(name string) bool {
	return name == "WaitGroup" || name == "Mutex" || name == "Once"
}

func isGoSyncNamedType(typ types.Type) bool {
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	return named.Obj().Pkg().Path() == "sync" && isBareSyncTypeName(named.Obj().Name())
}

// goTypesTypeToRust converts a go/types.Type to the base Rust type string (unwrapped)
func goTypesTypeToRust(t types.Type) string {
	if rustType, ok := goTypeParamConstraintToRust(t); ok {
		return rustType
	}
	if rustType, ok := goTypesKnownStdlibNamedTypeToRust(t); ok {
		return rustType
	}
	if sig, ok := signatureFromType(t); ok {
		return signatureToBoxDynFn(sig)
	}
	if named, ok := types.Unalias(t).(*types.Named); ok && named.Obj() != nil && named.Obj().Pkg() != nil && isStdlibPackage(named.Obj().Pkg().Path()) {
		return goTypesNamedTypeToRust(named)
	}
	if named, ok := t.(*types.Named); ok && named.Obj() != nil {
		obj := named.Obj()
		if obj.Pkg() == nil && obj.Name() == "error" {
			TrackImport("Error")
			if NeedsConcurrentWrapper() {
				return "Box<dyn StdError + Send + Sync>"
			}
			return "Box<dyn StdError>"
		}
		if obj.Pkg() != nil && isStdlibPackage(obj.Pkg().Path()) {
			return goTypesNamedTypeToRust(named)
		}
		if _, isBasic := types.Unalias(named.Underlying()).(*types.Basic); isBasic {
			return goTypesNamedTypeToRust(named)
		}
		if _, isSlice := types.Unalias(named.Underlying()).(*types.Slice); isSlice {
			return goTypesNamedTypeToRust(named)
		}
	}
	switch ut := t.Underlying().(type) {
	case *types.Basic:
		switch ut.Kind() {
		case types.String:
			return "String"
		case types.UntypedString:
			return "String"
		case types.Int:
			return "i32"
		case types.UntypedInt:
			return "i32"
		case types.UntypedRune:
			return "i32"
		case types.Int8:
			return "i8"
		case types.Int16:
			return "i16"
		case types.Int32:
			return "i32"
		case types.Int64:
			return "i64"
		case types.Uint:
			return "u32"
		case types.Uint8:
			return "u8"
		case types.Uint16:
			return "u16"
		case types.Uint32:
			return "u32"
		case types.Uint64:
			return "u64"
		case types.Uintptr:
			return "usize"
		case types.UnsafePointer:
			return "usize"
		case types.Float32:
			return "f32"
		case types.Float64:
			return "f64"
		case types.UntypedFloat:
			return "f64"
		case types.Bool:
			return "bool"
		case types.UntypedBool:
			return "bool"
		default:
			return fmt.Sprintf("/* unknown basic type: %s */", ut.Name())
		}
	case *types.Slice:
		return "Vec<" + goTypesTypeToRust(ut.Elem()) + ">"
	case *types.Array:
		return "[" + goTypesTypeToRust(ut.Elem()) + "; " + strconv.FormatInt(ut.Len(), 10) + "]"
	case *types.Pointer:
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		return outerWrapper + "<" + innerWrapper + "<Option<" + goTypesTypeToRust(ut.Elem()) + ">>>"
	case *types.Map:
		TrackImport("BTreeMap")
		return "BTreeMap<" + goTypesMapKeyToRust(ut.Key()) + ", " + goTypesMapValueToRust(ut.Elem()) + ">"
	case *types.Struct:
		if named, ok := t.(*types.Named); ok {
			return goTypesNamedTypeToRust(named)
		}
		if anonName := lookupAnonymousStructName(ut); anonName != "" {
			return anonName
		}
		return "/* unknown struct */"
	case *types.Interface:
		if ut.NumMethods() == 0 {
			return rustAnyTraitObject()
		}
		if named, ok := t.(*types.Named); ok {
			return rustLocalInterfaceTraitObject(goTypesNamedTypeToRust(named))
		}
		return rustAnyTraitObject()
	case *types.Signature:
		return signatureToBoxDynFn(ut)
	default:
		// Fallback for named types
		if named, ok := t.(*types.Named); ok {
			return goTypesNamedTypeToRust(named)
		}
		return "/* unknown type */"
	}
}

func goTypesMapKeyToRust(t types.Type) string {
	if t == nil {
		return "()"
	}
	if ptr, ok := types.Unalias(t).Underlying().(*types.Pointer); ok {
		return goPtrKeyHelperNameForType(t) + "<" + goTypesTypeToRust(ptr.Elem()) + ">"
	}
	return goTypesTypeToRust(t)
}

func goTypesMapValueToRust(t types.Type) string {
	if t == nil {
		return "()"
	}
	if _, ok := types.Unalias(t).Underlying().(*types.Pointer); ok {
		return goTypesTypeToRust(t)
	}
	return goTypesTypeToRustWrapped(t)
}

func goTypesConstTypeToRust(t types.Type) string {
	if basic, ok := t.Underlying().(*types.Basic); ok {
		switch basic.Kind() {
		case types.UntypedString:
			return "String"
		case types.UntypedBool:
			return "bool"
		case types.UntypedInt, types.UntypedRune:
			return "i32"
		case types.UntypedFloat:
			return "f64"
		}
	}
	return goTypesTypeToRust(t)
}

func goTypesReturnTypeToRust(t types.Type) string {
	if _, ok := t.Underlying().(*types.Pointer); ok {
		return goTypesTypeToRust(t)
	}
	return goTypesTypeToRustWrapped(t)
}

func goTypesNamedTypeToRust(named *types.Named) string {
	if named == nil || named.Obj() == nil {
		return "Unknown"
	}
	if rustType, ok := goTypesKnownStdlibNamedTypeToRust(named); ok {
		return rustType
	}
	obj := named.Obj()
	if obj.Pkg() == nil {
		return RustTypeNameForUse(obj.Name())
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil && obj.Pkg() == typeInfo.pkg {
		return RustTypeNameForUse(obj.Name())
	}
	if rustName, ok := rustTypeNameForImportedPackagePath(obj.Pkg().Path(), obj.Name()); ok {
		return rustName
	}
	rustName := obj.Pkg().Name() + "_" + RustTypeNameForUse(obj.Name())
	if isStdlibPackage(obj.Pkg().Path()) {
		RegisterExternalTypeStubNamed(named, rustName)
	}
	return rustName
}

func goTypesKnownStdlibNamedTypeToRust(t types.Type) (string, bool) {
	named, ok := t.(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return "", false
	}
	obj := named.Obj()
	switch obj.Pkg().Path() {
	case "sync":
		switch obj.Name() {
		case "WaitGroup":
			NeedWaitGroup()
			return "WaitGroup", true
		case "Mutex":
			NeedGoMutex()
			return "GoMutex", true
		case "Once":
			NeedGoOnce()
			return "GoOnce", true
		}
	case "strings":
		if obj.Name() == "Builder" {
			return "String", true
		}
	case "time":
		switch obj.Name() {
		case "Time":
			NeedGoTime()
			return "GoTime", true
		case "Duration":
			return "std::time::Duration", true
		case "Timer":
			NeedGoTimer()
			return "GoTimer", true
		case "Ticker":
			NeedGoTicker()
			return "GoTicker", true
		}
	case "context":
		switch obj.Name() {
		case "Context":
			NeedGoContext()
			return "GoContext", true
		case "CancelFunc":
			NeedGoContext()
			return "GoCancelFunc", true
		case "CancelCauseFunc":
			NeedGoContext()
			return "GoCancelCauseFunc", true
		}
	}
	return "", false
}

func rustTypeNameForImportedPackagePath(pkgPath, name string) (string, bool) {
	if pkgPath == "" || isStdlibPackage(pkgPath) {
		return "", false
	}
	ctx := GetTranspileContext()
	if ctx != nil && ctx.PackageMapping != nil {
		if crateName, ok := ctx.PackageMapping[pkgPath]; ok {
			return crateName + "::" + RustTypeNameForUse(name), true
		}
	}
	return RustCrateNameForGoImportPath(pkgPath) + "::" + RustTypeNameForUse(name), true
}

// goTypesTypeToRustWrapped converts a go/types.Type to the wrapped Rust type string
func goTypesTypeToRustWrapped(t types.Type) string {
	base := goTypesTypeToRust(t)
	outerWrapper := GetOuterWrapperType()
	innerWrapper := GetInnerWrapperType()
	return outerWrapper + "<" + innerWrapper + "<Option<" + base + ">>>"
}

func goTypesParamTypeToRust(t types.Type) string {
	if interfaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(t); ok {
		return rustLocalInterfaceParam(interfaceName)
	}
	if _, ok := types.Unalias(t).Underlying().(*types.Pointer); ok {
		return goTypesTypeToRust(t)
	}
	return goTypesTypeToRustWrapped(t)
}

func isFunctionSignatureType(t types.Type) bool {
	_, ok := signatureFromType(t)
	return ok
}

func isFunctionSignatureDerefExpression(expr ast.Expr) bool {
	if paren, ok := expr.(*ast.ParenExpr); ok {
		expr = paren.X
	}
	var target ast.Expr
	switch deref := expr.(type) {
	case *ast.StarExpr:
		target = deref.X
	case *ast.UnaryExpr:
		if deref.Op != token.MUL {
			return false
		}
		target = deref.X
	default:
		return false
	}
	if ident, ok := target.(*ast.Ident); ok {
		if vt := GetVarTable(); vt != nil {
			if info := vt.Lookup(ident.Name); info != nil && info.RustType == "function_signature_pointer" {
				return true
			}
		}
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if isFunctionSignatureType(typeInfo.GetType(expr)) {
		return true
	}
	if ptrType := typeInfo.GetType(target); ptrType != nil {
		if ptr, ok := types.Unalias(ptrType).Underlying().(*types.Pointer); ok {
			return isFunctionSignatureType(ptr.Elem())
		}
	}
	return false
}

func isPointerDerefExpression(expr ast.Expr) bool {
	if paren, ok := expr.(*ast.ParenExpr); ok {
		expr = paren.X
	}
	switch deref := expr.(type) {
	case *ast.StarExpr:
		return true
	case *ast.UnaryExpr:
		return deref.Op == token.MUL
	default:
		return false
	}
}

func isFunctionSignatureTypeExpr(expr ast.Expr) bool {
	if ident, ok := expr.(*ast.Ident); ok && IsFunctionTypeAlias(ident.Name) {
		return true
	}
	if named, ok := namedTypeForTypeExpr(expr); ok {
		return isFunctionSignatureType(named)
	}
	return false
}

func signatureFromType(t types.Type) (*types.Signature, bool) {
	if t == nil {
		return nil, false
	}
	t = types.Unalias(t)
	if sig, ok := t.(*types.Signature); ok {
		return sig, true
	}
	if named, ok := t.(*types.Named); ok {
		if sig, ok := types.Unalias(named.Underlying()).(*types.Signature); ok {
			return sig, true
		}
	}
	if sig, ok := types.Unalias(t.Underlying()).(*types.Signature); ok {
		return sig, true
	}
	return nil, false
}

// signatureToBoxDynFn converts a go/types Signature to a "Box<dyn Fn(...)>" string
func signatureToBoxDynFn(sig *types.Signature) string {
	var paramTypes []string
	params := sig.Params()
	for i := 0; i < params.Len(); i++ {
		paramTypes = append(paramTypes, goTypesParamTypeToRust(params.At(i).Type()))
	}

	var returnType string
	results := sig.Results()
	if results.Len() == 0 {
		returnType = "()"
	} else if results.Len() == 1 {
		returnType = goTypesTypeToRustWrapped(results.At(0).Type())
	} else {
		var retTypes []string
		for i := 0; i < results.Len(); i++ {
			retTypes = append(retTypes, goTypesTypeToRustWrapped(results.At(i).Type()))
		}
		returnType = "(" + strings.Join(retTypes, ", ") + ")"
	}

	paramsStr := strings.Join(paramTypes, ", ")
	if NeedsConcurrentWrapper() {
		return fmt.Sprintf("Box<dyn Fn(%s) -> %s + Send + Sync>", paramsStr, returnType)
	}
	return fmt.Sprintf("Box<dyn Fn(%s) -> %s>", paramsStr, returnType)
}
