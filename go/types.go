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
var anonymousStructAliases = make(map[string]string) // maps package global type aliases to anonymous struct names

func rustUintType() string {
	if strconv.IntSize == 64 {
		return "u64"
	}
	return "u32"
}

func isEmptyInterfaceExpr(expr ast.Expr) bool {
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "any" {
		return true
	}
	intf, ok := expr.(*ast.InterfaceType)
	if !ok {
		return false
	}
	if intf.Methods == nil {
		return true
	}
	return len(intf.Methods.List) == 0
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
	unaliased := types.Unalias(typ)
	if _, ok := unaliased.(*types.TypeParam); ok {
		return false
	}
	intf, ok := unaliased.Underlying().(*types.Interface)
	return ok && intf.NumMethods() == 0
}

func typeInfoTypeForTypeExpr(expr ast.Expr) (types.Type, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return nil, false
	}
	if typ := typeInfo.GetType(expr); typ != nil {
		return typ, true
	}
	if typeInfo.info == nil {
		return nil, false
	}
	switch e := expr.(type) {
	case *ast.Ident:
		if obj, ok := typeInfo.info.Uses[e].(*types.TypeName); ok {
			return obj.Type(), true
		}
	case *ast.SelectorExpr:
		if obj, ok := typeInfo.info.Uses[e.Sel].(*types.TypeName); ok {
			return obj.Type(), true
		}
	}
	return nil, false
}

func isInterfaceType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Interface)
	return ok
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
	if isStubBackedStdlibPackagePath(obj.Pkg().Path()) {
		return "", false
	}
	return goTypesNamedTypeToRust(named), true
}

func transpiledNamedInterfaceTypeNameFromExpr(expr ast.Expr) (string, bool) {
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

func rustLocalInterfaceDynType(name string) string {
	if NeedsConcurrentWrapper() {
		return "dyn " + name + " + Send + Sync"
	}
	return "dyn " + name
}

// rustLocalInterfaceParam returns the Rust type used at a function parameter
// boundary for a named Go interface. Named interface params are wrapped in the
// same Arc<Mutex<Option<Box<dyn T>>>> / Rc<RefCell<Option<Box<dyn T>>>> shape
// used elsewhere in the wrapper-handle model, so a Go nil interface lowers to
// a None slot and `x == nil` lowers to an is_none() check.
func rustLocalInterfaceParam(name string) string {
	outerWrapper := GetOuterWrapperType()
	innerWrapper := GetInnerWrapperType()
	trackWrapperImports()
	return outerWrapper + "<" + innerWrapper + "<Option<" + rustLocalInterfaceTraitObject(name) + ">>>"
}

// rustLocalInterfaceParamBare returns the bare `&dyn T` reference form for
// trait-internal helpers (such as `__go_eq_*`) where the value never crosses
// a Go-level parameter boundary and ownership/nilability are not part of the
// contract.
func rustLocalInterfaceParamBare(name string) string {
	if NeedsConcurrentWrapper() {
		return "&(dyn " + name + " + Send + Sync)"
	}
	return "&dyn " + name
}

// traitMethodSuffix returns the snake_case suffix used on per-trait auxiliary
// methods such as `__go_clone_box_<suffix>` and `__go_eq_<suffix>`. The
// supplied trait name may be a simple identifier (current-package interface
// like `Key`) or a qualified Rust path (cross-package interface like
// `example_com_ifaceeq_label::Key`); the suffix is always derived from the
// final segment so it matches the suffix used in the trait declaration.
func traitMethodSuffix(ifaceName string) string {
	name := ifaceName
	if idx := strings.LastIndex(name, "::"); idx != -1 {
		name = name[idx+2:]
	}
	name = strings.TrimPrefix(name, "r#")
	suffix := strings.TrimPrefix(ToSnakeCase(name), "r#")
	if isRustPathKeyword(suffix) || isRustKeyword(suffix) {
		return suffix + "_"
	}
	return suffix
}

// interfaceTypeHasNamedEmbedded reports whether the given go/types interface
// embeds any other named interface that the transpiler emits as a Rust trait.
// Used to decide whether the implementation block must redeclare
// `__go_as_any` or inherit it from a supertrait.
func interfaceTypeHasNamedEmbedded(iface *types.Interface) bool {
	if iface == nil {
		return false
	}
	for i := 0; i < iface.NumEmbeddeds(); i++ {
		named, ok := types.Unalias(iface.EmbeddedType(i)).(*types.Named)
		if !ok {
			continue
		}
		if _, ok := named.Underlying().(*types.Interface); !ok {
			continue
		}
		if _, ok := transpiledNamedInterfaceTypeNameFromTypes(named); ok {
			return true
		}
	}
	return false
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

func typeParamConstraintLowersToRustString(t types.Type) bool {
	rustType, ok := goTypeParamConstraintToRust(t)
	return ok && rustType == "String"
}

func goTypeParamNameToRust(t types.Type) (string, bool) {
	tp, ok := types.Unalias(t).(*types.TypeParam)
	if !ok || tp.Obj() == nil {
		return "", false
	}
	return RustTypeNameForUse(tp.Obj().Name()), true
}

func goTypeParamTraitConstraintName(t types.Type) (string, bool) {
	tp, ok := types.Unalias(t).(*types.TypeParam)
	if !ok || tp.Constraint() == nil {
		return "", false
	}
	named, ok := types.Unalias(tp.Constraint()).(*types.Named)
	if !ok || named.Obj() == nil {
		return "", false
	}
	iface, ok := named.Underlying().(*types.Interface)
	if !ok || iface.NumMethods() == 0 {
		return "", false
	}
	return goTypesNamedTypeToRust(named), true
}

func goTypeParamHasAnyConstraint(t types.Type) bool {
	tp, ok := types.Unalias(t).(*types.TypeParam)
	if !ok || tp.Constraint() == nil {
		return false
	}
	iface, ok := tp.Constraint().Underlying().(*types.Interface)
	return ok && !iface.IsComparable() && iface.NumMethods() == 0 && iface.NumEmbeddeds() == 0
}

func goTypeParamHasComparableConstraint(t types.Type) bool {
	tp, ok := types.Unalias(t).(*types.TypeParam)
	if !ok || tp.Constraint() == nil {
		return false
	}
	iface, ok := tp.Constraint().Underlying().(*types.Interface)
	return ok && iface.IsComparable() && iface.NumMethods() == 0 && iface.NumEmbeddeds() == 0
}

func goTypeParamHasStringByteSliceConstraint(t types.Type) bool {
	tp, ok := types.Unalias(t).(*types.TypeParam)
	if !ok || tp.Constraint() == nil {
		return false
	}
	iface, ok := tp.Constraint().Underlying().(*types.Interface)
	if !ok {
		return false
	}
	hasString := false
	hasByteSlice := false
	for i := 0; i < iface.NumEmbeddeds(); i++ {
		embedded := types.Unalias(iface.EmbeddedType(i))
		union, ok := embedded.(*types.Union)
		if !ok {
			return false
		}
		for j := 0; j < union.Len(); j++ {
			switch {
			case isGoStringType(union.Term(j).Type()):
				hasString = true
			case isGoByteSliceType(union.Term(j).Type()):
				hasByteSlice = true
			default:
				return false
			}
		}
	}
	return hasString && hasByteSlice
}

func goTypeParamHasStringConstraint(t types.Type) bool {
	tp, ok := types.Unalias(t).(*types.TypeParam)
	if !ok || tp.Constraint() == nil {
		return false
	}
	iface, ok := tp.Constraint().Underlying().(*types.Interface)
	if !ok || iface.NumEmbeddeds() == 0 {
		return false
	}
	for i := 0; i < iface.NumEmbeddeds(); i++ {
		if !constraintTermIsStringOnly(iface.EmbeddedType(i)) {
			return false
		}
	}
	return true
}

func constraintTermIsStringOnly(t types.Type) bool {
	t = types.Unalias(t)
	if union, ok := t.(*types.Union); ok {
		if union.Len() == 0 {
			return false
		}
		for i := 0; i < union.Len(); i++ {
			if !isGoStringType(union.Term(i).Type()) {
				return false
			}
		}
		return true
	}
	return isGoStringType(t)
}

func isGoStringType(t types.Type) bool {
	basic, ok := types.Unalias(t).Underlying().(*types.Basic)
	return ok && basic.Kind() == types.String
}

func isGoByteSliceType(t types.Type) bool {
	slice, ok := types.Unalias(t).Underlying().(*types.Slice)
	return ok && isByteType(slice.Elem())
}

func goTypeParamHasIntegerConstraint(t types.Type) bool {
	tp, ok := types.Unalias(t).(*types.TypeParam)
	if !ok || tp.Constraint() == nil {
		return false
	}
	iface, ok := tp.Constraint().Underlying().(*types.Interface)
	if !ok {
		return false
	}
	hasTerm := false
	for i := 0; i < iface.NumEmbeddeds(); i++ {
		embedded := types.Unalias(iface.EmbeddedType(i))
		if union, ok := embedded.(*types.Union); ok {
			for j := 0; j < union.Len(); j++ {
				if !isGoIntegerType(union.Term(j).Type()) {
					return false
				}
				hasTerm = true
			}
			continue
		}
		if !isGoIntegerType(embedded) {
			return false
		}
		hasTerm = true
	}
	return hasTerm
}

func goTypeParamOrderedTraitConstraintName(t types.Type) (string, bool) {
	tp, ok := types.Unalias(t).(*types.TypeParam)
	if !ok || tp.Constraint() == nil {
		return "", false
	}
	named, ok := types.Unalias(tp.Constraint()).(*types.Named)
	if !ok || named.Obj() == nil {
		return "", false
	}
	iface, ok := named.Underlying().(*types.Interface)
	if !ok || !interfaceEmbedsOnlyOrderedTerms(iface) {
		return "", false
	}
	return goTypesNamedTypeToRust(named), true
}

func goTypeParamHasOrderedConstraint(t types.Type) bool {
	tp, ok := types.Unalias(t).(*types.TypeParam)
	if !ok || tp.Constraint() == nil {
		return false
	}
	iface, ok := tp.Constraint().Underlying().(*types.Interface)
	return ok && interfaceEmbedsOnlyOrderedTerms(iface)
}

func typeExprIsOrderedTypeParam(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || expr == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if _, ok := types.Unalias(typ).(*types.TypeParam); !ok {
		return false
	}
	return goTypeParamHasOrderedConstraint(typ)
}

func interfaceEmbedsOnlyOrderedTerms(iface *types.Interface) bool {
	if iface == nil || iface.NumEmbeddeds() == 0 {
		return false
	}
	for i := 0; i < iface.NumEmbeddeds(); i++ {
		if !constraintTermIsOrderedOnly(iface.EmbeddedType(i)) {
			return false
		}
	}
	return true
}

func constraintTermIsOrderedOnly(t types.Type) bool {
	if t == nil {
		return false
	}
	switch u := types.Unalias(t).(type) {
	case *types.Union:
		if u.Len() == 0 {
			return false
		}
		for i := 0; i < u.Len(); i++ {
			if !isGoOrderedType(u.Term(i).Type()) {
				return false
			}
		}
		return true
	default:
		if iface, ok := types.Unalias(t).Underlying().(*types.Interface); ok {
			return interfaceEmbedsOnlyOrderedTerms(iface)
		}
		return isGoOrderedType(t)
	}
}

func isGoOrderedType(t types.Type) bool {
	if t == nil {
		return false
	}
	basic, ok := types.Unalias(t).Underlying().(*types.Basic)
	if !ok {
		return false
	}
	info := basic.Info()
	return info&types.IsInteger != 0 || info&types.IsFloat != 0 || info&types.IsString != 0
}

func goTypeParamSliceConstraintToRust(t types.Type) (string, bool) {
	elem, ok := goTypeParamSliceConstraintElem(t)
	if !ok {
		return "", false
	}
	return "Vec<" + goTypesCollectionElemTypeToRust(elem) + ">", true
}

func goTypeParamSliceConstraintElem(t types.Type) (types.Type, bool) {
	tp, ok := types.Unalias(t).(*types.TypeParam)
	if !ok || tp.Constraint() == nil {
		return nil, false
	}
	return constraintSliceElem(tp.Constraint())
}

func constraintSliceElem(t types.Type) (types.Type, bool) {
	if t == nil {
		return nil, false
	}
	switch u := types.Unalias(t).(type) {
	case *types.Union:
		var elem types.Type
		for i := 0; i < u.Len(); i++ {
			termElem, ok := constraintSliceElem(u.Term(i).Type())
			if !ok {
				return nil, false
			}
			if elem == nil {
				elem = termElem
				continue
			}
			if !types.Identical(types.Unalias(elem), types.Unalias(termElem)) {
				return nil, false
			}
		}
		if elem == nil {
			return nil, false
		}
		return elem, true
	case *types.Slice:
		return types.Unalias(u.Elem()), true
	default:
		if iface, ok := types.Unalias(t).Underlying().(*types.Interface); ok {
			var elem types.Type
			for i := 0; i < iface.NumEmbeddeds(); i++ {
				embeddedElem, ok := constraintSliceElem(iface.EmbeddedType(i))
				if !ok {
					return nil, false
				}
				if elem == nil {
					elem = embeddedElem
					continue
				}
				if !types.Identical(types.Unalias(elem), types.Unalias(embeddedElem)) {
					return nil, false
				}
			}
			if elem != nil {
				return elem, true
			}
		}
		return nil, false
	}
}

func goTypeParamHasPointerConstraint(t types.Type) bool {
	tp, ok := types.Unalias(t).(*types.TypeParam)
	if !ok || tp.Constraint() == nil {
		return false
	}
	return constraintEmbedsOnlyPointerTypes(tp.Constraint())
}

func constraintEmbedsOnlyPointerTypes(t types.Type) bool {
	if t == nil {
		return false
	}
	iface, ok := types.Unalias(t).Underlying().(*types.Interface)
	if !ok || iface.NumEmbeddeds() == 0 {
		return false
	}
	for i := 0; i < iface.NumEmbeddeds(); i++ {
		if !constraintTermIsPointerOnly(iface.EmbeddedType(i)) {
			return false
		}
	}
	return true
}

func constraintTermIsPointerOnly(t types.Type) bool {
	if t == nil {
		return false
	}
	switch u := types.Unalias(t).(type) {
	case *types.Pointer:
		return true
	case *types.Union:
		if u.Len() == 0 {
			return false
		}
		for i := 0; i < u.Len(); i++ {
			if !constraintTermIsPointerOnly(u.Term(i).Type()) {
				return false
			}
		}
		return true
	default:
		if _, ok := types.Unalias(t).Underlying().(*types.Interface); ok {
			return constraintEmbedsOnlyPointerTypes(t)
		}
		return false
	}
}

func isGoIntegerType(t types.Type) bool {
	if t == nil {
		return false
	}
	basic, ok := types.Unalias(t).Underlying().(*types.Basic)
	return ok && basic.Info()&types.IsInteger != 0
}

func goTypeParamTraitConstraintNameFromExpr(expr ast.Expr) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return "", false
	}
	return goTypeParamTraitConstraintName(typ)
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
			if len(field.Names) == 0 {
				if idx >= numFields || structType.Field(idx).Name() != getEmbeddedFieldName(field.Type) {
					match = false
					break
				}
				idx++
				continue
			}
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
	if typ, ok := typeInfoTypeForTypeExpr(expr); ok {
		if rustType, ok := goTypesNamedFunctionTypeToRust(typ); ok {
			return rustType
		}
	}
	if typeExprIsOrderedTypeParam(expr) {
		return goTypeToRustBase(expr)
	}

	// For non-interface types, use regular wrapping
	return GoTypeToRust(expr)
}

func GoTypeToRust(expr ast.Expr) string {
	if typ, ok := typeInfoTypeForTypeExpr(expr); ok {
		if rustType, ok := goTypesNamedFunctionTypeToRust(typ); ok {
			return rustType
		}
		if rustType, ok := goTypeParamSliceConstraintToRust(typ); ok {
			return goTypesWrappedRustType(rustType)
		}
	}

	baseType := goTypeToRustBase(expr)

	// Determine wrapper types based on concurrency needs
	outerWrapper := GetOuterWrapperType()
	innerWrapper := GetInnerWrapperType()

	trackWrapperImports()

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

	// Bare sync helper types handle synchronization internally. Other sync
	// package values, such as sync.Map, still use normal Go field handles.
	if isSyncParam(expr) {
		return baseType
	}

	// Wrap everything in appropriate wrapper
	// Don't double-wrap pointers - they're already wrapped
	if _, isPointer := expr.(*ast.StarExpr); !isPointer {
		return outerWrapper + "<" + innerWrapper + "<Option<" + baseType + ">>>"
	}

	return baseType
}

func pointerAliasElemTypeToRust(star *ast.StarExpr) (string, bool) {
	if star == nil || isFunctionSignatureTypeExpr(star.X) {
		return "", false
	}
	typ, ok := typeInfoTypeForTypeExpr(star)
	if !ok {
		return "", false
	}
	ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	if !ok {
		return "", false
	}
	alias, ok := ptr.Elem().(*types.Alias)
	if !ok {
		return "", false
	}
	return goTypesTypeToRust(types.Unalias(alias)), true
}

func pointerTypeExprElemIsGoError(star *ast.StarExpr) bool {
	typ, ok := typeInfoTypeForTypeExpr(star)
	if !ok {
		return false
	}
	ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	return ok && isGoErrorType(ptr.Elem())
}

func rustGoErrorHandleType() string {
	trackWrapperImports()
	return GetOuterWrapperType() + "<" + GetInnerWrapperType() + "<Option<" + rustStdErrorBoxType() + ">>>"
}

// Generate Rust closure type from Go function type
func generateClosureType(funcType *ast.FuncType) string {
	var paramTypes []string
	if funcType.Params != nil {
		for _, field := range funcType.Params.List {
			paramType := GoTypeToRustParam(field.Type)
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
		returnType = GoReturnTypeToRust(funcType.Results.List[0].Type)
	} else {
		// Multiple returns or named returns
		var retTypes []string
		for _, field := range funcType.Results.List {
			retType := GoReturnTypeToRust(field.Type)
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
		// For concurrent code, closures need Send + Sync.
		return fmt.Sprintf("Box<dyn FnMut(%s) -> %s + Send + Sync>", paramsStr, returnType)
	} else {
		// For single-threaded code, no Send + Sync requirement.
		return fmt.Sprintf("Box<dyn FnMut(%s) -> %s>", paramsStr, returnType)
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
			return rustUintType()
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
				typ := typeInfo.GetType(t)
				if rustType, ok := goTypeParamNameToRust(typ); ok {
					return rustType
				}
				if rustType, ok := goTypeParamConstraintToRust(typ); ok {
					return rustType
				}
			}
			if typeInfo != nil {
				if interfaceName, ok := localNamedInterfaceTypeNameFromTypes(typeInfo.GetType(t)); ok {
					return rustLocalInterfaceTraitObject(RustTypeNameForUse(interfaceName))
				}
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
		elemType := goCollectionElemTypeToRust(t.Elt)
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
		if pointerTypeExprElemIsGoError(t) {
			return rustGoErrorHandleType()
		}
		// Pointer to sync types → bare type (they handle sharing internally)
		if isSyncParam(t) {
			return goTypeToRustBase(t.X)
		}
		// Pointer type - wrap the base type (not already wrapped)
		innerType := goTypeToRustBase(t.X)
		if aliasElemType, ok := pointerAliasElemTypeToRust(t); ok {
			innerType = aliasElemType
		}
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		return outerWrapper + "<" + innerWrapper + "<Option<" + innerType + ">>>"
	case *ast.FuncType:
		// Function type - generate a closure type
		return generateClosureType(t)
	case *ast.IndexExpr:
		typeInfo := GetTypeInfo()
		if typeInfo != nil {
			if typ := typeInfo.GetType(t); typ != nil {
				return goTypesTypeToRust(typ)
			}
		}
		if rustType, ok := goCallableTypeFromTypeInfo(t); ok {
			return rustType
		}
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		return fmt.Sprintf("/* ERROR: Unsupported instantiated generic type */ %s<%s<Option<()>>>", outerWrapper, innerWrapper)
	case *ast.IndexListExpr:
		typeInfo := GetTypeInfo()
		if typeInfo != nil {
			if typ := typeInfo.GetType(t); typ != nil {
				return goTypesTypeToRust(typ)
			}
		}
		if rustType, ok := goCallableTypeFromTypeInfo(t); ok {
			return rustType
		}
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		return fmt.Sprintf("/* ERROR: Unsupported instantiated generic type */ %s<%s<Option<()>>>", outerWrapper, innerWrapper)
	case *ast.ChanType:
		NeedGoChannel()
		elemType := goTypeToRustBase(t.Value)
		return "GoChannel<" + elemType + ">"
	case *ast.SelectorExpr:
		// Package-qualified types like sync.WaitGroup, sync.Mutex
		if ident, ok := t.X.(*ast.Ident); ok {
			if ifaceName, ok := transpiledNamedInterfaceTypeNameFromExpr(t); ok {
				return rustLocalInterfaceTraitObject(ifaceName)
			}
			if isSourceMappedPackagePath(goPackageImports[ident.Name]) {
				if rustName, ok := rustTypeNameForImportedPackagePath(goPackageImports[ident.Name], t.Sel.Name); ok {
					return rustName
				}
			}
			if ident.Name == "sync" {
				switch t.Sel.Name {
				case "WaitGroup":
					NeedWaitGroup()
					return "WaitGroup"
				case "Mutex":
					NeedGoMutex()
					return "GoMutex"
				case "RWMutex":
					NeedGoRWMutex()
					return "GoRWMutex"
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
			if goPackageImports[ident.Name] == "regexp" && t.Sel.Name == "Regexp" {
				NeedRegexp()
				return "GoRegexp"
			}
			if isStdlibPackage(goPackageImports[ident.Name]) {
				if named, ok := namedTypeForTypeExpr(t); ok {
					if ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(named); ok {
						return rustLocalInterfaceTraitObject(ifaceName)
					}
					if sig, ok := signatureFromType(named); ok {
						return signatureToBoxDynFn(sig)
					}
				}
			}
			if named, ok := namedTypeForTypeExpr(t); ok {
				if ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(named); ok {
					return rustLocalInterfaceTraitObject(ifaceName)
				}
			}
			if rustName, ok := rustTypeNameForImportedPackagePath(goPackageImports[ident.Name], t.Sel.Name); ok {
				return rustName
			}
		}
		if ident, ok := t.X.(*ast.Ident); ok {
			rustName := fmt.Sprintf("%s_%s", ident.Name, t.Sel.Name)
			pkgPath := goPackageImports[ident.Name]
			if pkgPath == "" {
				if fallback, ok := fallbackStdlibPackagePathForImportName(ident.Name); ok {
					pkgPath = fallback
				}
			}
			if isStubBackedStdlibPackagePath(pkgPath) {
				RegisterExternalTypeStubForTypeExpr(t, rustName)
				if externalTypeExprFallbackIsInterface(pkgPath, t.Sel.Name) {
					RegisterExternalTypeStubInterface(rustName)
				} else {
					RegisterExternalTypeStub(rustName)
				}
			}
			return rustName
		}
		return fmt.Sprintf("%T_%s", t.X, t.Sel.Name)
	case *ast.Ellipsis:
		// Variadic parameter ...T is treated as []T (slice) in Go
		elemType := goCollectionElemTypeToRust(t.Elt)
		return "Vec<" + elemType + ">"
	case *ast.StructType:
		// Anonymous struct type - generate a unique type name
		return generateAnonymousStructType(t)
	default:
		// Unhandled type
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		return fmt.Sprintf("/* TODO: Unhandled type */ %s<%s<Option<()>>>", outerWrapper, innerWrapper)
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
	// Interface-typed map keys: trait objects don't impl Ord/Borrow, so wrap
	// them in GoLocalPtrKey for identity-based comparison (matches Go's
	// interface equality semantics for pointer-backed interface keys).
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if ifaceName, ok := transpiledNamedInterfaceTypeNameFromExpr(expr); ok {
			NeedGoPtrKey()
			return "GoLocalPtrKey<" + rustLocalInterfaceTraitObject(ifaceName) + ">"
		}
	}
	if isEmptyInterfaceExpr(expr) {
		NeedGoPtrKey()
		return "GoLocalPtrKey<" + rustAnyTraitObject() + ">"
	}
	return goTypeToRustBase(expr)
}

func isStdlibStubBackedType(t types.Type) bool {
	t = types.Unalias(t)
	switch typ := t.(type) {
	case *types.Named:
		return typ.Obj() != nil && typ.Obj().Pkg() != nil && isStubBackedStdlibPackagePath(typ.Obj().Pkg().Path())
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
		case "error":
			return rustEmptyErrorHandleValue()
		default:
			return "Default::default()"
		}
	case *ast.SelectorExpr:
		if ident, ok := t.X.(*ast.Ident); ok {
			if ident.Name == "strings" && t.Sel.Name == "Builder" && !isSourceMappedPackagePath(goPackageImports[ident.Name]) {
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
		TrackImport("BTreeMap")
		return "BTreeMap::new()"
	default:
		return "Default::default()"
	}
}

func isDirectTypeParamType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).(*types.TypeParam)
	return ok
}

func writeDirectTypeParamWrappedZeroValue(out *strings.Builder, typeExpr ast.Expr, operation string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(typeExpr)
	if typ == nil {
		out.WriteString("/* ERROR: Type information required for ")
		out.WriteString(operation)
		out.WriteString(" */ unimplemented!(\"type info required for ")
		out.WriteString(operation)
		out.WriteString("\")")
		return true
	}
	if !isDirectTypeParamType(typ) {
		return false
	}
	WriteWrappedNone(out)
	return true
}

func zeroValueForTypesType(typ types.Type) string {
	if typ == nil {
		return "Default::default()"
	}
	if isGoErrorType(typ) {
		return rustEmptyErrorHandleValue()
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		if zeroValue, ok := zeroValueForNamedOverNamedScalar(named); ok {
			return zeroValue
		}
		if isTimeDurationType(named) {
			return "std::time::Duration::from_nanos(0)"
		}
		if _, isSlice := types.Unalias(named.Underlying()).(*types.Slice); isSlice {
			return "Default::default()"
		}
		if mapType, isMap := types.Unalias(named.Underlying()).(*types.Map); isMap {
			TrackImport("BTreeMap")
			var out strings.Builder
			out.WriteString(goTypesNamedTypeToRust(named))
			out.WriteString("(")
			WriteWrapperPrefix(&out)
			out.WriteString("BTreeMap::<")
			out.WriteString(goTypesMapKeyToRust(mapType.Key()))
			out.WriteString(", ")
			out.WriteString(goTypesMapValueToRust(mapType.Elem()))
			out.WriteString(">::new()")
			WriteWrapperSuffix(&out)
			out.WriteString(")")
			return out.String()
		}
		if array, isArray := types.Unalias(named.Underlying()).(*types.Array); isArray {
			var out strings.Builder
			out.WriteString(goTypesNamedTypeToRust(named))
			out.WriteString("(")
			WriteWrapperPrefix(&out)
			out.WriteString("std::array::from_fn(|_| ")
			out.WriteString(zeroValueForTypesType(array.Elem()))
			out.WriteString(")")
			WriteWrapperSuffix(&out)
			out.WriteString(")")
			return out.String()
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
		TrackImport("BTreeMap")
		return "BTreeMap::new()"
	case *types.Pointer:
		trackWrapperImports()
		return GetOuterWrapperType() + "::new(" + GetInnerWrapperType() + "::new(None))"
	default:
		return "Default::default()"
	}
}

func zeroValueForNamedOverNamedScalar(named *types.Named) (string, bool) {
	if named == nil || named.Obj() == nil {
		return "", false
	}
	underlyingType, ok := LookupTypeDefinitionUnderlyingType(named.Obj().Name())
	if !ok || underlyingType == nil || types.Identical(named, underlyingType) {
		return "", false
	}
	underlyingNamed, ok := types.Unalias(underlyingType).(*types.Named)
	if !ok {
		return "", false
	}
	if _, ok := types.Unalias(underlyingNamed.Underlying()).(*types.Basic); !ok {
		return "", false
	}
	var out strings.Builder
	out.WriteString(goTypesNamedTypeToRust(named))
	out.WriteString("(")
	WriteWrapperPrefix(&out)
	out.WriteString(zeroValueForTypesType(underlyingType))
	WriteWrapperSuffix(&out)
	out.WriteString(")")
	return out.String(), true
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
		types.Uint, types.Uint8, types.Uint16, types.Uint32, types.Uint64, types.Uintptr,
		types.UnsafePointer:
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

func isSourceMappedSyncParam(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		typ = ptr.Elem()
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	return named.Obj().Pkg().Path() == "sync" &&
		isSourceMappedPackagePath(named.Obj().Pkg().Path()) &&
		isBareSyncTypeName(named.Obj().Name())
}

func goCollectionElemTypeToRust(expr ast.Expr) string {
	if isFunctionSignatureTypeExpr(expr) {
		return GoTypeToRust(expr)
	}
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "error" {
		return GoTypeToRust(expr)
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		typ := typeInfo.GetType(expr)
		if goTypeParamHasOrderedConstraint(typ) {
			return goTypeToRustBase(expr)
		}
		if _, ok := types.Unalias(typ).(*types.TypeParam); ok {
			return GoTypeToRust(expr)
		}
	}
	if _, ok := goTypeParamTraitConstraintNameFromExpr(expr); ok {
		return GoTypeToRust(expr)
	}
	// Local named interfaces need wrapped slice elements: Box<dyn Trait> has no
	// Default (so make([]Trait, n) breaks) and can't be nil (so
	// interface-typed slice elements can't represent Go's nil interface value).
	// Wrapping mirrors the [`error` -> Box<dyn StdError>] handling above.
	if _, ok := transpiledNamedInterfaceTypeNameFromExpr(expr); ok {
		return GoTypeToRust(expr)
	}
	return goTypeToRustBase(expr)
}

func isBareSyncTypeName(name string) bool {
	return name == "WaitGroup" || name == "Mutex" || name == "RWMutex" || name == "Once"
}

func isGoSyncNamedType(typ types.Type) bool {
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	return named.Obj().Pkg().Path() == "sync" && isBareSyncTypeName(named.Obj().Name())
}

func isSourceMappedGoSyncNamedType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		typ = ptr.Elem()
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	return named.Obj().Pkg().Path() == "sync" &&
		isBareSyncTypeName(named.Obj().Name()) &&
		isSourceMappedPackagePath(named.Obj().Pkg().Path())
}

func isGoSyncOnceNamedType(typ types.Type) bool {
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	return named.Obj().Pkg().Path() == "sync" && named.Obj().Name() == "Once"
}

// goTypesTypeToRust converts a go/types.Type to the base Rust type string (unwrapped)
func goTypesTypeToRust(t types.Type) string {
	if rustType, ok := goTypeParamNameToRust(t); ok {
		return rustType
	}
	if rustType, ok := goTypeParamConstraintToRust(t); ok {
		return rustType
	}
	if rustType, ok := goTypesKnownStdlibNamedTypeToRust(t); ok {
		return rustType
	}
	if interfaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(t); ok {
		return rustLocalInterfaceTraitObject(interfaceName)
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
		if _, isMap := types.Unalias(named.Underlying()).(*types.Map); isMap {
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
			return rustUintType()
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
		case types.Complex64:
			TrackImport("num::Complex")
			return "num::Complex<f32>"
		case types.Complex128:
			TrackImport("num::Complex")
			return "num::Complex<f64>"
		case types.UntypedComplex:
			TrackImport("num::Complex")
			return "num::Complex<f64>"
		case types.Bool:
			return "bool"
		case types.UntypedBool:
			return "bool"
		default:
			return fmt.Sprintf("/* unknown basic type: %s */", ut.Name())
		}
	case *types.Slice:
		return "Vec<" + goTypesCollectionElemTypeToRust(ut.Elem()) + ">"
	case *types.Array:
		return "[" + goTypesCollectionElemTypeToRust(ut.Elem()) + "; " + strconv.FormatInt(ut.Len(), 10) + "]"
	case *types.Pointer:
		if isGoErrorType(ut.Elem()) {
			return rustGoErrorHandleType()
		}
		outerWrapper := GetOuterWrapperType()
		innerWrapper := GetInnerWrapperType()
		trackWrapperImports()
		return outerWrapper + "<" + innerWrapper + "<Option<" + goTypesTypeToRust(ut.Elem()) + ">>>"
	case *types.Map:
		TrackImport("BTreeMap")
		return "BTreeMap<" + goTypesMapKeyToRust(ut.Key()) + ", " + goTypesMapValueToRust(ut.Elem()) + ">"
	case *types.Chan:
		NeedGoChannel()
		return "GoChannel<" + goTypesChannelElemTypeToRust(ut.Elem()) + ">"
	case *types.Struct:
		if named, ok := types.Unalias(t).(*types.Named); ok {
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

func goTypesCollectionElemTypeToRust(t types.Type) string {
	if isFunctionSignatureType(t) {
		return goTypesTypeToRustWrapped(t)
	}
	if isGoErrorType(t) {
		return goTypesTypeToRustWrapped(t)
	}
	if goTypeParamHasOrderedConstraint(t) {
		return goTypesTypeToRust(t)
	}
	if _, ok := types.Unalias(t).(*types.TypeParam); ok {
		return goTypesTypeToRustWrapped(t)
	}
	// Mirror the syntax-side rule in goCollectionElemTypeToRust: local named
	// interfaces lower to Box<dyn Trait>, which has no Default and can't be
	// nil. Wrapping the element preserves Go's nullable interface semantics.
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(t); ok {
		return goTypesTypeToRustWrapped(t)
	}
	return goTypesTypeToRust(t)
}

func goTypesChannelElemTypeToRust(t types.Type) string {
	if isGoErrorType(t) {
		TrackImport("Error")
		if NeedsConcurrentWrapper() {
			return "Option<Box<dyn StdError + Send + Sync>>"
		}
		return "Option<Box<dyn StdError>>"
	}
	return goTypesTypeToRust(t)
}

func goTypesMapKeyToRust(t types.Type) string {
	if t == nil {
		return "()"
	}
	if ptr, ok := types.Unalias(t).Underlying().(*types.Pointer); ok {
		return goPtrKeyHelperNameForType(t) + "<" + goTypesTypeToRust(ptr.Elem()) + ">"
	}
	// Interface-typed keys: wrap in GoLocalPtrKey so the map satisfies Ord
	// via Arc/Rc pointer identity (matches Go's interface equality for
	// pointer-backed dynamic values).
	if ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(t); ok {
		NeedGoPtrKey()
		return "GoLocalPtrKey<" + rustLocalInterfaceTraitObject(ifaceName) + ">"
	}
	if isEmptyInterfaceType(t) {
		NeedGoPtrKey()
		return "GoLocalPtrKey<" + rustAnyTraitObject() + ">"
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
		case types.String, types.UntypedString:
			return "&'static str"
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
	if rustType, ok := goTypeParamSliceConstraintToRust(t); ok {
		return goTypesWrappedRustType(rustType)
	}
	if rustType, ok := goTypesNamedFunctionTypeToRust(t); ok {
		return rustType
	}
	if typeIsPredeclaredCopyScalar(t) {
		return goTypesTypeToRust(types.Unalias(t))
	}
	if _, ok := t.Underlying().(*types.Pointer); ok {
		return goTypesTypeToRust(t)
	}
	if _, ok := t.Underlying().(*types.Chan); ok {
		// Channels stay bare — GoChannel<T> is already a shared cloneable
		// handle and the value-channel emitters match this shape.
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
		return rustNamedTypeWithArgs(named, RustTypeNameForUse(obj.Name()))
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil && obj.Pkg() == typeInfo.pkg {
		return rustNamedTypeWithArgs(named, RustTypeNameForUse(obj.Name()))
	}
	if rustName, ok := rustTypeNameForImportedPackagePath(obj.Pkg().Path(), obj.Name()); ok {
		return rustNamedTypeWithArgs(named, rustName)
	}
	rustName := obj.Pkg().Name() + "_" + RustTypeNameForUse(obj.Name())
	if isStubBackedStdlibPackagePath(obj.Pkg().Path()) {
		RegisterExternalTypeStubNamed(named, rustName)
	}
	return rustNamedTypeWithArgs(named, rustName)
}

func rustNamedTypeWithArgs(named *types.Named, rustName string) string {
	typeArgs := named.TypeArgs()
	if typeArgs == nil || typeArgs.Len() == 0 {
		return rustName
	}
	args := make([]string, 0, typeArgs.Len())
	for i := 0; i < typeArgs.Len(); i++ {
		args = append(args, goTypesTypeToRust(typeArgs.At(i)))
	}
	return rustName + "<" + strings.Join(args, ", ") + ">"
}

func goTypesKnownStdlibNamedTypeToRust(t types.Type) (string, bool) {
	named, ok := t.(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return "", false
	}
	obj := named.Obj()
	if isSourceMappedPackagePath(obj.Pkg().Path()) {
		return "", false
	}
	switch obj.Pkg().Path() {
	case "sync":
		switch obj.Name() {
		case "WaitGroup":
			NeedWaitGroup()
			return "WaitGroup", true
		case "Mutex":
			NeedGoMutex()
			return "GoMutex", true
		case "RWMutex":
			NeedGoRWMutex()
			return "GoRWMutex", true
		case "Once":
			NeedGoOnce()
			return "GoOnce", true
		}
	case "sync/atomic":
		if obj.Name() == "Pointer" {
			NeedGoAtomicPointer()
			elemType := "()"
			if typeArgs := named.TypeArgs(); typeArgs != nil && typeArgs.Len() > 0 {
				elemType = goTypesTypeToRust(typeArgs.At(0))
			}
			return "GoAtomicPointer<" + elemType + ">", true
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
	case "regexp":
		if obj.Name() == "Regexp" {
			NeedRegexp()
			return "GoRegexp", true
		}
	}
	return "", false
}

func rustTypeNameForImportedPackagePath(pkgPath, name string) (string, bool) {
	if pkgPath == "" {
		return "", false
	}
	ctx := GetTranspileContext()
	if ctx != nil && ctx.PackageMapping != nil {
		if crateName, ok := ctx.PackageMapping[pkgPath]; ok {
			TrackGeneratedCrateDependency(crateName)
			if moduleName := importedPackageTypeModuleName(ctx, pkgPath, name); moduleName != "" {
				return crateName + "::" + moduleName + "::" + RustTypeNameForUse(name), true
			}
			return crateName + "::" + RustTypeNameForUse(name), true
		}
	}
	if isStdlibPackage(pkgPath) {
		return "", false
	}
	return RustCrateNameForGoImportPath(pkgPath) + "::" + RustTypeNameForUse(name), true
}

func importedPackageTypeModuleName(ctx *TranspileContext, pkgPath, name string) string {
	if ctx == nil || ctx.Session == nil || ctx.Session.PackageTypeModuleNames == nil {
		return ""
	}
	typeModules := ctx.Session.PackageTypeModuleNames[pkgPath]
	if typeModules == nil {
		return ""
	}
	return typeModules[name]
}

// goTypesTypeToRustWrapped converts a go/types.Type to the wrapped Rust type string
func goTypesTypeToRustWrapped(t types.Type) string {
	base := goTypesTypeToRust(t)
	return goTypesWrappedRustType(base)
}

func goTypesWrappedRustType(base string) string {
	outerWrapper := GetOuterWrapperType()
	innerWrapper := GetInnerWrapperType()
	trackWrapperImports()
	return outerWrapper + "<" + innerWrapper + "<Option<" + base + ">>>"
}

func goTypesParamTypeToRust(t types.Type) string {
	if interfaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(t); ok {
		return rustLocalInterfaceParam(interfaceName)
	}
	if rustType, ok := goTypesNamedFunctionTypeToRust(t); ok {
		return rustType
	}
	if goTypeParamHasOrderedConstraint(t) {
		return goTypesTypeToRust(t)
	}
	if rustType, ok := goTypeParamSliceConstraintToRust(t); ok {
		return goTypesWrappedRustType(rustType)
	}
	if _, ok := types.Unalias(t).Underlying().(*types.Pointer); ok {
		return goTypesTypeToRust(t)
	}
	return goTypesTypeToRustWrapped(t)
}

func goTypesFunctionParamTypeToRust(t types.Type) string {
	// Imported interfaces stay a bare cross-crate trait reference in closure
	// types; the func values that cross the crate boundary are bare too, so the
	// type, the value, and the call site agree. A LOCAL interface instead uses
	// the wrapped nilable handle that local func values and func-typed params
	// use (via goTypesParamTypeToRust -> rustLocalInterfaceParam), so a
	// `func(Node) bool` value converts to a named `type T func(Node) bool`
	// within the same package, for example go/ast's ast.Inspect / type inspector.
	if _, isLocal := localNamedInterfaceTypeNameFromTypes(t); !isLocal {
		if interfaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(t); ok {
			return rustLocalInterfaceParamBare(interfaceName)
		}
	}
	return goTypesParamTypeToRust(t)
}

func goTypesNamedFunctionTypeToRust(t types.Type) (string, bool) {
	named, ok := types.Unalias(t).(*types.Named)
	if !ok || named.Obj() == nil {
		return "", false
	}
	if named.Obj().Pkg() != nil && isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return "", false
	}
	if _, ok := types.Unalias(named.Underlying()).(*types.Signature); !ok {
		return "", false
	}
	return goTypesNamedTypeToRust(named), true
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
	if _, ok := expr.(*ast.FuncType); ok {
		return true
	}
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

func signatureToBoxDynFnWithParamShape(sig *types.Signature, paramType func(types.Type) string) string {
	var paramTypes []string
	params := sig.Params()
	for i := 0; i < params.Len(); i++ {
		paramTypes = append(paramTypes, paramType(params.At(i).Type()))
	}

	var returnType string
	results := sig.Results()
	if results.Len() == 0 {
		returnType = "()"
	} else if results.Len() == 1 {
		returnType = goTypesReturnTypeToRust(results.At(0).Type())
	} else {
		var retTypes []string
		for i := 0; i < results.Len(); i++ {
			retTypes = append(retTypes, goTypesReturnTypeToRust(results.At(i).Type()))
		}
		returnType = "(" + strings.Join(retTypes, ", ") + ")"
	}

	paramsStr := strings.Join(paramTypes, ", ")
	if NeedsConcurrentWrapper() {
		return fmt.Sprintf("Box<dyn FnMut(%s) -> %s + Send + Sync>", paramsStr, returnType)
	}
	return fmt.Sprintf("Box<dyn FnMut(%s) -> %s>", paramsStr, returnType)
}

// signatureToBoxDynFn converts a go/types Signature to a boxed Go function
// string for go/types-only function shapes.
func signatureToBoxDynFn(sig *types.Signature) string {
	return signatureToBoxDynFnWithParamShape(sig, goTypesFunctionParamTypeToRust)
}

// signatureToGoParamBoxDynFn converts a go/types Signature to a boxed function
// string that matches generated Go-level function parameter declarations.
func signatureToGoParamBoxDynFn(sig *types.Signature) string {
	return signatureToBoxDynFnWithParamShape(sig, goTypesParamTypeToRust)
}
