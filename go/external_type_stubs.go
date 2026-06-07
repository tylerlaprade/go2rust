package main

import (
	"fmt"
	"go/ast"
	"go/constant"
	"go/types"
	"os"
	"path/filepath"
	"slices"
	"sort"
	"strconv"
	"strings"
)

var externalTypeStubs = make(map[string]bool)
var externalTypeStubInterfaces = make(map[string]bool)
var externalTypeStubIntegerTypes = make(map[string]string)
var externalTypeStubTupleTypes = make(map[string]string)
var externalTypeStubFields = make(map[string]map[string]string)
var externalTypeStubMethods = make(map[string]map[string]externalTypeStubMethod)
var externalTypeStubConversions = make(map[string]map[string]bool)
var externalPackageStubs = make(map[string]*externalPackageStub)

type externalTypeStubMethod struct {
	ParamCount  int
	ReturnTypes []string
}

type externalPromotedMethod struct {
	EmbeddedFieldName  string
	MethodName         string
	RustMethodName     string
	Func               *types.Func
	Signature          *types.Signature
	GenericArguments   bool
	MutableReceiver    bool
	RawEmbeddedField   bool
	GoPtrEmbeddedField bool
}

type externalPackageStub struct {
	Functions      map[string]externalPackageStubFunction
	Constants      map[string]string
	ConstantValues map[string]constant.Value
	Variables      map[string]string
}

type externalPackageStubFunction struct {
	ParamCount        int
	ParamTypes        []string
	GenericParamNames []string
	ReturnTypes       []string
}

func RegisterExternalTypeStub(name string) {
	if !canDefineExternalTypeStub(name) {
		return
	}
	if currentExternalTypeStubs()[name] {
		return
	}
	currentExternalTypeStubs()[name] = true
	if name == "types_Tuple" {
		RegisterExternalTypeStubInterface("types_Type")
	}
	if name == "types_TypeName" {
		RegisterExternalIntegerTypeStub("token_Pos", "i32")
		RegisterExternalTypeStub("types_Package")
		RegisterExternalTypeStubInterface("types_Type")
	}
	if name == "types_TypeParam" {
		RegisterExternalTypeStub("types_TypeName")
		RegisterExternalTypeStubInterface("types_Type")
	}
}

func RegisterExternalTypeStubInterface(name string) {
	if !canDefineExternalTypeStub(name) {
		return
	}
	RegisterExternalTypeStub(name)
	currentExternalTypeStubInterfaces()[name] = true
}

func RegisterExternalIntegerTypeStub(name string, rustType string) {
	if !canDefineExternalTypeStub(name) || rustType == "" {
		return
	}
	RegisterExternalTypeStub(name)
	currentExternalTypeStubIntegerTypes()[name] = rustType
}

func RegisterExternalTypeStubNamed(named *types.Named, rustName string) {
	if !canDefineExternalTypeStub(rustName) {
		return
	}
	RegisterExternalTypeStub(rustName)
	if externalNamedIsInterface(named) {
		currentExternalTypeStubInterfaces()[rustName] = true
	}
	if rustType, ok := externalIntegerRustTypeForNamed(named); ok {
		currentExternalTypeStubIntegerTypes()[rustName] = rustType
	}
	if rustType, ok := externalTupleRustTypeForNamed(named); ok {
		currentExternalTypeStubTupleTypes()[rustName] = rustType
	}
	RegisterExternalErrorMethodForNamed(named, rustName)
}

func RegisterExternalTypeStubForTypeExpr(expr ast.Expr, rustName string) {
	if !canDefineExternalTypeStub(rustName) {
		return
	}
	RegisterExternalTypeStub(rustName)
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	var named *types.Named
	if typ := typeInfo.GetType(expr); typ != nil {
		named, _ = typ.(*types.Named)
	}
	if named == nil {
		if sel, ok := expr.(*ast.SelectorExpr); ok {
			if obj, ok := typeInfo.info.Uses[sel.Sel].(*types.TypeName); ok {
				named, _ = obj.Type().(*types.Named)
			}
		}
	}
	if externalNamedIsInterface(named) {
		currentExternalTypeStubInterfaces()[rustName] = true
	}
	if rustType, ok := externalIntegerRustTypeForNamed(named); ok {
		currentExternalTypeStubIntegerTypes()[rustName] = rustType
	}
	if rustType, ok := externalTupleRustTypeForNamed(named); ok {
		currentExternalTypeStubTupleTypes()[rustName] = rustType
	}
}

func canDefineExternalTypeStub(name string) bool {
	return name != "" && !strings.Contains(name, "::")
}

func externalNamedIsInterface(named *types.Named) bool {
	if named == nil || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	if !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return false
	}
	_, ok := types.Unalias(named.Underlying()).(*types.Interface)
	return ok
}

func externalIntegerRustTypeForNamed(named *types.Named) (string, bool) {
	if named == nil || named.Obj() == nil || named.Obj().Pkg() == nil {
		return "", false
	}
	if !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return "", false
	}
	basic, ok := types.Unalias(named.Underlying()).(*types.Basic)
	if !ok {
		return "", false
	}
	switch basic.Kind() {
	case types.Int:
		return "i32", true
	case types.Int8:
		return "i8", true
	case types.Int16:
		return "i16", true
	case types.Int32:
		return "i32", true
	case types.Int64:
		return "i64", true
	case types.Uint:
		return rustUintType(), true
	case types.Uint8:
		return "u8", true
	case types.Uint16:
		return "u16", true
	case types.Uint32:
		return "u32", true
	case types.Uint64:
		return "u64", true
	case types.Uintptr:
		return "usize", true
	default:
		return "", false
	}
}

func externalTupleRustTypeForNamed(named *types.Named) (string, bool) {
	if named == nil || named.Obj() == nil || named.Obj().Pkg() == nil {
		return "", false
	}
	if !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return "", false
	}
	switch underlying := types.Unalias(named.Underlying()).(type) {
	case *types.Slice:
		return goTypesTypeToRustWrapped(underlying), true
	default:
		return "", false
	}
}

func RegisterExternalTypeStubField(typeName string, fieldName string, fieldType types.Type, ownerPkgPath string) {
	if !canDefineExternalTypeStub(typeName) || fieldName == "" || fieldType == nil {
		return
	}
	RegisterExternalTypeStub(typeName)
	trackWrapperImports()
	fieldTypeRust := canonicalPackageExternalStubRustType(goTypesFieldTypeToRustForOwnerPackage(fieldType, ownerPkgPath))
	fields := currentExternalTypeStubFields()
	if fields[typeName] == nil {
		fields[typeName] = make(map[string]string)
	}
	fields[typeName][fieldName] = fieldTypeRust
}

func goTypesFieldTypeToRust(t types.Type) string {
	return goTypesFieldTypeToRustForOwnerPackage(t, "")
}

func goTypesFieldTypeToRustForOwnerPackage(t types.Type, ownerPkgPath string) string {
	if _, ok := types.Unalias(t).Underlying().(*types.Pointer); ok {
		return goTypesTypeToRust(t)
	}
	if mapType, ok := types.Unalias(t).Underlying().(*types.Map); ok {
		TrackImport("BTreeMap")
		return goTypesWrappedRustType("BTreeMap<" + goTypesMapKeyToRustForOwnerPackage(mapType.Key(), ownerPkgPath) + ", " + goTypesMapValueToRust(mapType.Elem()) + ">")
	}
	return goTypesTypeToRustWrapped(t)
}

func RegisterExternalTypeStubMethod(typeName string, methodName string, sig *types.Signature) {
	if !canDefineExternalTypeStub(typeName) || methodName == "" || sig == nil {
		return
	}
	RegisterExternalTypeStub(typeName)
	trackWrapperImports()
	method := externalTypeStubMethod{
		ParamCount: sig.Params().Len(),
	}
	results := sig.Results()
	for i := 0; i < results.Len(); i++ {
		method.ReturnTypes = append(method.ReturnTypes, canonicalPackageExternalStubRustType(goTypesReturnTypeToRust(results.At(i).Type())))
	}
	methods := currentExternalTypeStubMethods()
	if methods[typeName] == nil {
		methods[typeName] = make(map[string]externalTypeStubMethod)
	}
	methods[typeName][methodName] = method
}

func RegisterExternalErrorMethodForNamed(named *types.Named, rustName string) {
	if named == nil || rustName == "" {
		return
	}
	if sig, ok := externalErrorMethodSignature(named); ok {
		RegisterExternalTypeStubMethod(rustName, "error", sig)
	}
}

func externalErrorMethodSignature(named *types.Named) (*types.Signature, bool) {
	for _, recv := range []types.Type{named, types.NewPointer(named)} {
		methods := types.NewMethodSet(recv)
		for i := 0; i < methods.Len(); i++ {
			fn, ok := methods.At(i).Obj().(*types.Func)
			if !ok || fn.Name() != "Error" {
				continue
			}
			sig, ok := fn.Type().(*types.Signature)
			if !ok || sig.Params().Len() != 0 || sig.Results().Len() != 1 {
				continue
			}
			if basic, ok := types.Unalias(sig.Results().At(0).Type()).(*types.Basic); ok && basic.Kind() == types.String {
				return sig, true
			}
		}
	}
	return nil, false
}

func RegisterExternalTypeStubConversion(targetType string, sourceType string) {
	if !canDefineExternalTypeStub(targetType) || !canDefineExternalTypeStub(sourceType) || targetType == sourceType {
		return
	}
	RegisterExternalTypeStub(targetType)
	RegisterExternalTypeStub(sourceType)
	conversions := currentExternalTypeStubConversions()
	if conversions[targetType] == nil {
		conversions[targetType] = make(map[string]bool)
	}
	conversions[targetType][sourceType] = true
}

func RegisterExternalSelectorField(sel *ast.SelectorExpr) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok || selection.Kind() != types.FieldVal {
		return
	}
	named, ok := externalSelectorReceiverNamed(selection.Recv())
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return
	}
	if !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return
	}
	if isKnownStdlibHelperType(named.Obj().Pkg().Path(), named.Obj().Name()) {
		return
	}
	field, ok := selection.Obj().(*types.Var)
	if !ok {
		return
	}
	RegisterExternalTypeStubField(goTypesNamedTypeToRust(named), ToSnakeCase(field.Name()), field.Type(), named.Obj().Pkg().Path())
}

func RegisterExternalSelectorMethod(sel *ast.SelectorExpr) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok || (selection.Kind() != types.MethodVal && selection.Kind() != types.MethodExpr) {
		return
	}
	named, ok := externalSelectorReceiverNamed(selection.Recv())
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return
	}
	if !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return
	}
	if isKnownStdlibHelperType(named.Obj().Pkg().Path(), named.Obj().Name()) &&
		!knownStdlibHelperNeedsExternalMethodStub(named.Obj().Pkg().Path(), named.Obj().Name()) &&
		!useStubBackedStdlibNamedIntegerInSourceMappedStdlib(named) {
		return
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok {
		return
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok {
		return
	}
	RegisterExternalTypeStubMethod(goTypesNamedTypeToRust(named), ToSnakeCase(fn.Name()), sig)
}

func IsExternalStdlibSelectorMethod(sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok || (selection.Kind() != types.MethodVal && selection.Kind() != types.MethodExpr) {
		return false
	}
	named, ok := externalSelectorReceiverNamed(selection.Recv())
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	if !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return false
	}
	return !isKnownStdlibHelperType(named.Obj().Pkg().Path(), named.Obj().Name())
}

func knownStdlibHelperNeedsExternalMethodStub(pkgPath string, name string) bool {
	return pkgPath == "os" && name == "File"
}

func RegisterExternalInterfaceMethodsForSource(source types.Type, iface *types.Interface) {
	if source == nil || iface == nil {
		return
	}
	named, ok := externalSelectorReceiverNamed(source)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return
	}
	if !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return
	}
	if isKnownStdlibHelperType(named.Obj().Pkg().Path(), named.Obj().Name()) {
		return
	}
	typeName := goTypesNamedTypeToRust(named)
	for i := 0; i < iface.NumMethods(); i++ {
		method := iface.Method(i)
		if method == nil {
			continue
		}
		sig, ok := method.Type().(*types.Signature)
		if !ok {
			continue
		}
		RegisterExternalTypeStubMethod(typeName, ToSnakeCase(method.Name()), sig)
	}
}

func collectExternalPromotedMethods(ownerTypeName string, structDef *StructDef, existingRustNames map[string]bool) []externalPromotedMethod {
	if structDef == nil || structDef.ASTType == nil {
		return nil
	}

	var promoted []externalPromotedMethod
	for _, field := range structDef.ASTType.Fields.List {
		if len(field.Names) > 0 {
			continue
		}
		named, ok := externalEmbeddedNamed(field.Type)
		if !ok {
			continue
		}

		rustTypeName := goTypesNamedTypeToRust(named)
		methodSetType := types.Type(types.NewPointer(named))
		if iface, ok := types.Unalias(named.Underlying()).(*types.Interface); ok {
			iface.Complete()
			methodSetType = named
		}
		methodSet := types.NewMethodSet(methodSetType)
		for i := 0; i < methodSet.Len(); i++ {
			fn, ok := methodSet.At(i).Obj().(*types.Func)
			if !ok || !fn.Exported() {
				continue
			}
			methodName := fn.Name()
			rustMethodName := ToSnakeCase(methodName)
			if existingRustNames[rustMethodName] {
				continue
			}
			sig, ok := fn.Type().(*types.Signature)
			if !ok {
				continue
			}

			stubBacked := isStubBackedStdlibPackagePath(named.Obj().Pkg().Path())
			if stubBacked {
				RegisterExternalTypeStubMethod(rustTypeName, rustMethodName, sig)
			}
			existingRustNames[rustMethodName] = true
			embeddedFieldName := getEmbeddedFieldName(field.Type)
			promoted = append(promoted, externalPromotedMethod{
				EmbeddedFieldName:  ToSnakeCase(embeddedFieldName),
				MethodName:         methodName,
				RustMethodName:     rustMethodName,
				Func:               fn,
				Signature:          sig,
				GenericArguments:   stubBacked,
				MutableReceiver:    !stubBacked && signatureHasPointerReceiver(sig),
				RawEmbeddedField:   promotedExternalFieldUsesRawStorage(field.Type),
				GoPtrEmbeddedField: generatedGoPtrFieldForStructNameField(ownerTypeName, embeddedFieldName),
			})
		}
	}

	slices.SortFunc(promoted, func(a, b externalPromotedMethod) int {
		if a.MethodName < b.MethodName {
			return -1
		}
		if a.MethodName > b.MethodName {
			return 1
		}
		return 0
	})
	return promoted
}

func promotedExternalFieldUsesRawStorage(expr ast.Expr) bool {
	return isSourceMappedSyncParam(expr)
}

func externalEmbeddedNamed(expr ast.Expr) (*types.Named, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || expr == nil {
		return nil, false
	}
	if star, ok := expr.(*ast.StarExpr); ok {
		return externalEmbeddedNamed(star.X)
	}

	var typ types.Type
	if typ = typeInfo.GetType(expr); typ == nil {
		if sel, ok := expr.(*ast.SelectorExpr); ok {
			if obj, ok := typeInfo.info.Uses[sel.Sel].(*types.TypeName); ok {
				typ = obj.Type()
			}
		}
	}
	if typ == nil {
		return nil, false
	}
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		typ = ptr.Elem()
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return nil, false
	}
	pkgPath := named.Obj().Pkg().Path()
	if isStubBackedStdlibPackagePath(pkgPath) {
		if isKnownStdlibHelperType(pkgPath, named.Obj().Name()) {
			return nil, false
		}
		return named, true
	}
	if typeInfo.pkg != nil && named.Obj().Pkg() == typeInfo.pkg {
		return nil, false
	}
	return named, true
}

func signatureHasPointerReceiver(sig *types.Signature) bool {
	if sig == nil || sig.Recv() == nil {
		return false
	}
	_, ok := types.Unalias(sig.Recv().Type()).(*types.Pointer)
	return ok
}

func externalSelectorReceiverNamed(recv types.Type) (*types.Named, bool) {
	recv = types.Unalias(recv)
	if ptr, ok := recv.(*types.Pointer); ok {
		recv = types.Unalias(ptr.Elem())
	}
	named, ok := recv.(*types.Named)
	return named, ok
}

func RegisterExternalPackageSelector(sel *ast.SelectorExpr) {
	pkgName, pkgPath, ok := externalStdlibPackageSelector(sel)
	if !ok {
		return
	}
	if rustExpr := GetStdlibSelectorMapping(pkgPath, sel.Sel.Name); rustExpr != "" {
		return
	}
	if registerExternalPackageSelectorFallback(pkgName, pkgPath, sel.Sel.Name) {
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	obj := typeInfo.info.Uses[sel.Sel]
	if obj == nil {
		return
	}
	switch obj := obj.(type) {
	case *types.Func:
		sig, ok := obj.Type().(*types.Signature)
		if !ok {
			return
		}
		RegisterExternalPackageStubFunction(pkgName, ToSnakeCase(sel.Sel.Name), sig)
	case *types.Const:
		RegisterExternalPackageStubConstantValue(pkgName, rustConstName(sel.Sel.Name), obj.Type(), obj.Val())
	case *types.Var:
		RegisterExternalPackageStubVariable(pkgName, rustPackageGlobalName(sel.Sel.Name), obj.Type())
	}
}

func RegisterExternalPackageFunctionFallback(sel *ast.SelectorExpr, argCount int) {
	if sel == nil {
		return
	}
	pkgName, pkgPath, ok := externalStdlibPackageSelector(sel)
	if !ok {
		ident, identOK := sel.X.(*ast.Ident)
		if !identOK {
			return
		}
		pkgPath, ok = goPackageImports[ident.Name]
		if !ok {
			pkgPath, ok = fallbackStdlibPackagePathForImportName(ident.Name)
		}
		if !ok || !isStubBackedStdlibPackagePath(pkgPath) {
			return
		}
		pkgName = ident.Name
	}
	funcName := ToSnakeCase(sel.Sel.Name)
	pkg := ensureExternalPackageStub(pkgName)
	if _, exists := pkg.Functions[funcName]; exists {
		return
	}
	trackWrapperImports()
	fn := externalPackageStubFunction{ParamCount: argCount}
	switch pkgPath {
	case "os/exec":
		if sel.Sel.Name != "LookPath" {
			return
		}
		TrackImport("Error")
		fn.ReturnTypes = []string{wrappedExternalStubType("String"), wrappedExternalStubType(rustStdErrorBoxType())}
	case "io":
		if sel.Sel.Name != "MultiWriter" {
			return
		}
		RegisterExternalTypeStubInterface("io_Writer")
		fn.ReturnTypes = []string{wrappedExternalStubType("io_Writer")}
	case "crypto/md5":
		if sel.Sel.Name != "New" {
			return
		}
		RegisterExternalTypeStubInterface("hash_Hash")
		fn.ReturnTypes = []string{wrappedExternalStubType("hash_Hash")}
	case "go/token":
		if sel.Sel.Name != "NewFileSet" {
			return
		}
		RegisterExternalTypeStub("token_FileSet")
		fn.ReturnTypes = []string{wrappedExternalStubType("token_FileSet")}
	case "go/types":
		return
	default:
		return
	}
	pkg.Functions[funcName] = fn
}

func RegisterExternalPackageStubFunction(pkgName string, funcName string, sig *types.Signature) {
	if pkgName == "" || funcName == "" || sig == nil {
		return
	}
	trackWrapperImports()
	if pkgName == "types" && funcName == "new_pointer" {
		return
	}
	if pkgName == "types" && (funcName == "new_package" || funcName == "new_checker") {
		return
	}
	fn := externalPackageStubFunction{
		ParamCount: sig.Params().Len(),
	}
	typeParams := sig.TypeParams()
	for i := 0; typeParams != nil && i < typeParams.Len(); i++ {
		tp := typeParams.At(i)
		if tp == nil || tp.Obj() == nil {
			continue
		}
		fn.GenericParamNames = append(fn.GenericParamNames, RustTypeNameForUse(tp.Obj().Name()))
	}
	params := sig.Params()
	for i := 0; len(fn.GenericParamNames) > 0 && i < params.Len(); i++ {
		fn.ParamTypes = append(fn.ParamTypes, canonicalPackageExternalStubRustType(goTypesParamTypeToRust(params.At(i).Type())))
	}
	results := sig.Results()
	for i := 0; i < results.Len(); i++ {
		fn.ReturnTypes = append(fn.ReturnTypes, canonicalPackageExternalStubRustType(goTypesReturnTypeToRust(results.At(i).Type())))
	}
	pkg := ensureExternalPackageStub(pkgName)
	pkg.Functions[funcName] = fn
}

func RegisterExternalTypeStubFieldByRustType(typeName string, fieldName string, fieldTypeRust string) {
	if !canDefineExternalTypeStub(typeName) || fieldName == "" || fieldTypeRust == "" {
		return
	}
	RegisterExternalTypeStub(typeName)
	trackWrapperImports()
	fields := currentExternalTypeStubFields()
	if fields[typeName] == nil {
		fields[typeName] = make(map[string]string)
	}
	fields[typeName][fieldName] = canonicalPackageExternalStubRustType(fieldTypeRust)
}

func RegisterExternalPackageStubConstant(pkgName string, constName string, constType types.Type) {
	RegisterExternalPackageStubConstantValue(pkgName, constName, constType, nil)
}

func RegisterExternalPackageStubConstantValue(pkgName string, constName string, constType types.Type, constValue constant.Value) {
	if pkgName == "" || constName == "" || constType == nil {
		return
	}
	if pkgName == "token" {
		RegisterExternalIntegerTypeStub("token_Token", "i32")
	}
	pkg := ensureExternalPackageStub(pkgName)
	pkg.Constants[constName] = goTypesConstTypeToRust(constType)
	if constValue != nil {
		pkg.ConstantValues[constName] = constValue
	} else {
		delete(pkg.ConstantValues, constName)
	}
}

func RegisterExternalPackageStubVariable(pkgName string, varName string, varType types.Type) {
	if pkgName == "" || varName == "" || varType == nil {
		return
	}
	trackWrapperImports()
	pkg := ensureExternalPackageStub(pkgName)
	pkg.Variables[varName] = canonicalPackageExternalStubRustType(goTypesReturnTypeToRust(varType))
}

func canonicalPackageExternalStubRustType(rustType string) string {
	if !usePackageExternalStubs() || !NeedsConcurrentWrapper() {
		return rustType
	}
	return strings.ReplaceAll(rustType, "StdMutex", "Mutex")
}

func ensureExternalPackageStub(pkgName string) *externalPackageStub {
	stubs := currentExternalPackageStubs()
	if stubs[pkgName] == nil {
		stubs[pkgName] = &externalPackageStub{
			Functions:      make(map[string]externalPackageStubFunction),
			Constants:      make(map[string]string),
			ConstantValues: make(map[string]constant.Value),
			Variables:      make(map[string]string),
		}
	}
	if stubs[pkgName].Functions == nil {
		stubs[pkgName].Functions = make(map[string]externalPackageStubFunction)
	}
	if stubs[pkgName].Constants == nil {
		stubs[pkgName].Constants = make(map[string]string)
	}
	if stubs[pkgName].ConstantValues == nil {
		stubs[pkgName].ConstantValues = make(map[string]constant.Value)
	}
	if stubs[pkgName].Variables == nil {
		stubs[pkgName].Variables = make(map[string]string)
	}
	return stubs[pkgName]
}

func IsExternalStdlibPackageVariableSelector(sel *ast.SelectorExpr) bool {
	_, pkgPath, ok := externalStdlibPackageSelector(sel)
	if !ok {
		return false
	}
	if externalPackageSelectorFallbackVariableType(pkgPath, sel.Sel.Name) != "" {
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	_, ok = typeInfo.info.Uses[sel.Sel].(*types.Var)
	return ok
}

// MACHINERY: emission-framework helper for selector fallback registration, not a stdlib shim.
func registerExternalPackageSelectorFallback(pkgName string, pkgPath string, selName string) bool {
	rustType := externalPackageSelectorFallbackVariableType(pkgPath, selName)
	if rustType == "" {
		return false
	}
	pkg := ensureExternalPackageStub(pkgName)
	pkg.Variables[rustPackageGlobalName(selName)] = rustType
	return true
}

func externalPackageSelectorFallbackVariableType(pkgPath string, selName string) string {
	switch pkgPath {
	case "io":
		if selName == "Discard" {
			RegisterExternalTypeStubInterface("io_Writer")
			return wrappedExternalStubType("io_Writer")
		}
	}
	return ""
}

func externalStdlibInterfaceTypeExpr(expr ast.Expr) (string, bool) {
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return "", false
	}
	ident, ok := sel.X.(*ast.Ident)
	if !ok {
		return "", false
	}
	pkgPath := goPackageImports[ident.Name]
	if pkgPath == "" {
		var fallbackOK bool
		pkgPath, fallbackOK = fallbackStdlibPackagePathForImportName(ident.Name)
		if !fallbackOK {
			return "", false
		}
	}
	if !isStubBackedStdlibPackagePath(pkgPath) || !externalTypeExprFallbackIsInterface(pkgPath, sel.Sel.Name) {
		return "", false
	}
	return fmt.Sprintf("%s_%s", ident.Name, sel.Sel.Name), true
}

func externalTypeExprFallbackIsInterface(pkgPath string, name string) bool {
	switch pkgPath {
	case "hash":
		return name == "Hash"
	case "io":
		return name == "Writer"
	}
	return false
}

func externalStdlibPackageSelector(sel *ast.SelectorExpr) (string, string, bool) {
	if sel == nil {
		return "", "", false
	}
	ident, ok := sel.X.(*ast.Ident)
	if !ok {
		return "", "", false
	}
	pkgPath, ok := goPackageImports[ident.Name]
	if !ok {
		pkgPath, ok = fallbackStdlibPackagePathForImportName(ident.Name)
	}
	if !ok || !isStubBackedStdlibPackagePath(pkgPath) {
		return "", "", false
	}
	return ident.Name, pkgPath, true
}

func isStdlibPackageSelectorImport(sel *ast.SelectorExpr) bool {
	if sel == nil {
		return false
	}
	ident, ok := sel.X.(*ast.Ident)
	if !ok {
		return false
	}
	pkgPath, ok := goPackageImports[ident.Name]
	if !ok {
		pkgPath, ok = fallbackStdlibPackagePathForImportName(ident.Name)
	}
	return ok && isStdlibPackage(pkgPath)
}

func fallbackStdlibPackagePathForImportName(name string) (string, bool) {
	switch name {
	case "bytes":
		return "bytes", true
	case "exec":
		return "os/exec", true
	case "hash":
		return "hash", true
	case "io":
		return "io", true
	case "md5":
		return "crypto/md5", true
	case "token":
		return "go/token", true
	case "types":
		return "go/types", true
	}
	return "", false
}

func isKnownStdlibHelperType(pkgPath string, name string) bool {
	switch pkgPath {
	case "context":
		return name == "Context" || name == "CancelFunc" || name == "CancelCauseFunc"
	case "net/url":
		return name == "URL"
	case "os":
		return name == "File"
	case "reflect":
		return name == "StructField" || name == "StructTag" || name == "Type"
	case "regexp":
		return name == "Regexp"
	case "strings":
		return name == "Builder"
	case "sync":
		return name == "WaitGroup" || name == "Mutex" || name == "RWMutex" || name == "Once"
	case "sync/atomic":
		return name == "Pointer"
	case "time":
		return name == "Time" || name == "Duration" || name == "Timer" || name == "Ticker"
	case "unsafe":
		return name == "Pointer"
	default:
		return false
	}
}

func stdlibHelperTypeAllowsInterfaceConversion(pkgPath string, name string, targetPkgPath string, targetName string) bool {
	if pkgPath == "os" && name == "File" {
		return targetPkgPath != "io" || targetName != "Writer"
	}
	return false
}

func currentExternalTypeStubs() map[string]bool {
	if usePackageExternalStubs() {
		if currentContext.Package.ExternalTypeStubs == nil {
			currentContext.Package.ExternalTypeStubs = make(map[string]bool)
		}
		return currentContext.Package.ExternalTypeStubs
	}
	if currentContext != nil && currentContext.File != nil {
		if currentContext.File.ExternalTypeStubs == nil {
			currentContext.File.ExternalTypeStubs = make(map[string]bool)
		}
		return currentContext.File.ExternalTypeStubs
	}
	return externalTypeStubs
}

func currentExternalTypeStubInterfaces() map[string]bool {
	if usePackageExternalStubs() {
		if currentContext.Package.ExternalTypeStubInterfaces == nil {
			currentContext.Package.ExternalTypeStubInterfaces = make(map[string]bool)
		}
		return currentContext.Package.ExternalTypeStubInterfaces
	}
	if currentContext != nil && currentContext.File != nil {
		if currentContext.File.ExternalTypeStubInterfaces == nil {
			currentContext.File.ExternalTypeStubInterfaces = make(map[string]bool)
		}
		return currentContext.File.ExternalTypeStubInterfaces
	}
	return externalTypeStubInterfaces
}

func currentExternalTypeStubIntegerTypes() map[string]string {
	if usePackageExternalStubs() {
		if currentContext.Package.ExternalTypeStubIntegerTypes == nil {
			currentContext.Package.ExternalTypeStubIntegerTypes = make(map[string]string)
		}
		return currentContext.Package.ExternalTypeStubIntegerTypes
	}
	if currentContext != nil && currentContext.File != nil {
		if currentContext.File.ExternalTypeStubIntegerTypes == nil {
			currentContext.File.ExternalTypeStubIntegerTypes = make(map[string]string)
		}
		return currentContext.File.ExternalTypeStubIntegerTypes
	}
	return externalTypeStubIntegerTypes
}

func currentExternalTypeStubTupleTypes() map[string]string {
	if usePackageExternalStubs() {
		if currentContext.Package.ExternalTypeStubTupleTypes == nil {
			currentContext.Package.ExternalTypeStubTupleTypes = make(map[string]string)
		}
		return currentContext.Package.ExternalTypeStubTupleTypes
	}
	if currentContext != nil && currentContext.File != nil {
		if currentContext.File.ExternalTypeStubTupleTypes == nil {
			currentContext.File.ExternalTypeStubTupleTypes = make(map[string]string)
		}
		return currentContext.File.ExternalTypeStubTupleTypes
	}
	return externalTypeStubTupleTypes
}

func currentExternalTypeStubFields() map[string]map[string]string {
	if usePackageExternalStubs() {
		if currentContext.Package.ExternalTypeStubFields == nil {
			currentContext.Package.ExternalTypeStubFields = make(map[string]map[string]string)
		}
		return currentContext.Package.ExternalTypeStubFields
	}
	if currentContext != nil && currentContext.File != nil {
		if currentContext.File.ExternalTypeStubFields == nil {
			currentContext.File.ExternalTypeStubFields = make(map[string]map[string]string)
		}
		return currentContext.File.ExternalTypeStubFields
	}
	return externalTypeStubFields
}

func currentExternalTypeStubMethods() map[string]map[string]externalTypeStubMethod {
	if usePackageExternalStubs() {
		if currentContext.Package.ExternalTypeStubMethods == nil {
			currentContext.Package.ExternalTypeStubMethods = make(map[string]map[string]externalTypeStubMethod)
		}
		return currentContext.Package.ExternalTypeStubMethods
	}
	if currentContext != nil && currentContext.File != nil {
		if currentContext.File.ExternalTypeStubMethods == nil {
			currentContext.File.ExternalTypeStubMethods = make(map[string]map[string]externalTypeStubMethod)
		}
		return currentContext.File.ExternalTypeStubMethods
	}
	return externalTypeStubMethods
}

func currentExternalTypeStubConversions() map[string]map[string]bool {
	if usePackageExternalStubs() {
		if currentContext.Package.ExternalTypeStubConversions == nil {
			currentContext.Package.ExternalTypeStubConversions = make(map[string]map[string]bool)
		}
		return currentContext.Package.ExternalTypeStubConversions
	}
	if currentContext != nil && currentContext.File != nil {
		if currentContext.File.ExternalTypeStubConversions == nil {
			currentContext.File.ExternalTypeStubConversions = make(map[string]map[string]bool)
		}
		return currentContext.File.ExternalTypeStubConversions
	}
	return externalTypeStubConversions
}

func currentExternalPackageStubs() map[string]*externalPackageStub {
	if usePackageExternalStubs() {
		if currentContext.Package.ExternalPackageStubs == nil {
			currentContext.Package.ExternalPackageStubs = make(map[string]*externalPackageStub)
		}
		return currentContext.Package.ExternalPackageStubs
	}
	if currentContext != nil && currentContext.File != nil {
		if currentContext.File.ExternalPackageStubs == nil {
			currentContext.File.ExternalPackageStubs = make(map[string]*externalPackageStub)
		}
		return currentContext.File.ExternalPackageStubs
	}
	return externalPackageStubs
}

func usePackageExternalStubs() bool {
	return currentContext != nil && currentContext.UsePackageExternalStubs && currentContext.Package != nil
}

func GenerateExternalTypeStubs() string {
	if usePackageExternalStubs() {
		return ""
	}
	return generateExternalStubs(currentExternalTypeStubs(), currentExternalTypeStubInterfaces(), currentExternalTypeStubIntegerTypes(), currentExternalTypeStubTupleTypes(), currentExternalTypeStubFields(), currentExternalTypeStubMethods(), currentExternalTypeStubConversions(), currentExternalPackageStubs())
}

func GeneratePackageExternalStubs(pkg *PackageState) string {
	if pkg == nil {
		return ""
	}
	return generateExternalStubs(pkg.ExternalTypeStubs, pkg.ExternalTypeStubInterfaces, pkg.ExternalTypeStubIntegerTypes, pkg.ExternalTypeStubTupleTypes, pkg.ExternalTypeStubFields, pkg.ExternalTypeStubMethods, pkg.ExternalTypeStubConversions, pkg.ExternalPackageStubs)
}

func WriteSharedStdlibStubCrate(workDir string, states []*PackageState, packageMapping map[string]string) error {
	outputDir := filepath.Join(workDir, "vendor", sharedStdlibStubCrateName)
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		return fmt.Errorf("failed to create shared stdlib stub crate: %v", err)
	}

	mergedState := MergeExternalStubPackageStates(states...)
	parts := []string{}
	if helperCode := mergedState.Helpers.GenerateSharedStdlibHelperModule(); helperCode != "" {
		parts = append(parts, helperCode)
	}
	if stubCode := GeneratePackageExternalStubs(mergedState); stubCode != "" {
		parts = append(parts, GenerateExternalStubModuleImports(), stubCode)
	}
	stubCode := hoistAndDedupeUseLines(strings.Join(parts, "\n"))

	libPath := filepath.Join(outputDir, "lib.rs")
	if err := os.WriteFile(libPath, []byte(stubCode), 0644); err != nil {
		return fmt.Errorf("failed to write shared stdlib stub lib.rs: %v", err)
	}

	sourceMappedDeps := sourceMappedCrateDependenciesForStubCode(stubCode, packageMapping)
	cargoToml := fmt.Sprintf(`[package]
name = "%s"
version = "0.1.0"
edition = "2021"

[lib]
name = "%s"
path = "lib.rs"

[dependencies]
serde_json = "1"
gosyn = "0.2.9"
`, sharedStdlibStubCrateName, sharedStdlibStubCrateName)
	for _, depCrate := range sourceMappedDeps {
		cargoToml += fmt.Sprintf("%s = { path = \"../%s\" }\n", depCrate, depCrate)
	}
	cargoPath := filepath.Join(outputDir, "Cargo.toml")
	if err := os.WriteFile(cargoPath, []byte(cargoToml), 0644); err != nil {
		return fmt.Errorf("failed to write shared stdlib stub Cargo.toml: %v", err)
	}

	return nil
}

func sourceMappedCrateDependenciesForStubCode(stubCode string, packageMapping map[string]string) []string {
	if stubCode == "" || len(packageMapping) == 0 {
		return nil
	}
	seen := make(map[string]bool)
	for _, crateName := range packageMapping {
		if crateName == "" || crateName == sharedStdlibStubCrateName {
			continue
		}
		if stubCodeReferencesCrate(stubCode, crateName) {
			seen[crateName] = true
		}
	}
	deps := make([]string, 0, len(seen))
	for crateName := range seen {
		deps = append(deps, crateName)
	}
	sort.Strings(deps)
	return deps
}

func stubCodeReferencesCrate(stubCode, crateName string) bool {
	needle := crateName + "::"
	for offset := 0; ; {
		index := strings.Index(stubCode[offset:], needle)
		if index < 0 {
			return false
		}
		start := offset + index
		if start == 0 || !isRustPathIdentByte(stubCode[start-1]) {
			return true
		}
		offset = start + len(needle)
	}
}

func isRustPathIdentByte(ch byte) bool {
	return ch == ':' || ch == '_' || ('0' <= ch && ch <= '9') || ('A' <= ch && ch <= 'Z') || ('a' <= ch && ch <= 'z')
}

func MergeExternalStubPackageStates(states ...*PackageState) *PackageState {
	merged := NewPackageState()
	for _, state := range states {
		if state == nil {
			continue
		}
		mergeBoolMap(merged.ExternalTypeStubs, state.ExternalTypeStubs)
		mergeBoolMap(merged.ExternalTypeStubInterfaces, state.ExternalTypeStubInterfaces)
		mergeStringMap(merged.ExternalTypeStubIntegerTypes, state.ExternalTypeStubIntegerTypes)
		mergeStringMap(merged.ExternalTypeStubTupleTypes, state.ExternalTypeStubTupleTypes)
		mergeNestedStringMap(merged.ExternalTypeStubFields, state.ExternalTypeStubFields)
		mergeNestedMethodMap(merged.ExternalTypeStubMethods, state.ExternalTypeStubMethods)
		mergeNestedBoolMap(merged.ExternalTypeStubConversions, state.ExternalTypeStubConversions)
		mergeExternalPackageStubs(merged.ExternalPackageStubs, state.ExternalPackageStubs)
		mergeHelperTracker(merged.Helpers, state.Helpers)
	}
	return merged
}

func hoistAndDedupeUseLines(code string) string {
	if code == "" {
		return ""
	}
	lines := strings.Split(code, "\n")
	seenUses := make(map[string]bool)
	var uses []string
	var body []string
	for _, line := range lines {
		if strings.HasPrefix(line, "use ") {
			if !seenUses[line] {
				seenUses[line] = true
				uses = append(uses, line)
			}
			continue
		}
		body = append(body, line)
	}
	slices.Sort(uses)
	bodyText := strings.Trim(strings.Join(body, "\n"), "\n")
	if len(uses) == 0 {
		if bodyText == "" {
			return ""
		}
		return bodyText + "\n"
	}
	if bodyText == "" {
		return strings.Join(uses, "\n") + "\n"
	}
	return strings.Join(uses, "\n") + "\n\n" + bodyText + "\n"
}

func mergeHelperTracker(dst *HelperTracker, src *HelperTracker) {
	if dst == nil || src == nil {
		return
	}
	dst.needsFormatMap = dst.needsFormatMap || src.needsFormatMap
	dst.needsFormatSlice = dst.needsFormatSlice || src.needsFormatSlice
	dst.needsFormatSliceWrappedValues = dst.needsFormatSliceWrappedValues || src.needsFormatSliceWrappedValues
	dst.needsFormatSliceWrappedStringer = dst.needsFormatSliceWrappedStringer || src.needsFormatSliceWrappedStringer
	dst.needsFormatNestedSlice = dst.needsFormatNestedSlice || src.needsFormatNestedSlice
	dst.needsFormatNestedSliceWrapped = dst.needsFormatNestedSliceWrapped || src.needsFormatNestedSliceWrapped
	dst.needsFormatNestedPointerSlice = dst.needsFormatNestedPointerSlice || src.needsFormatNestedPointerSlice
	dst.needsFormatNestedPointerSliceWrapped = dst.needsFormatNestedPointerSliceWrapped || src.needsFormatNestedPointerSliceWrapped
	dst.needsFormatAny = dst.needsFormatAny || src.needsFormatAny
	dst.needsFormatAnySlice = dst.needsFormatAnySlice || src.needsFormatAnySlice
	dst.needsAnyEq = dst.needsAnyEq || src.needsAnyEq
	dst.needsAnyClone = dst.needsAnyClone || src.needsAnyClone
	dst.needsGoValueClone = dst.needsGoValueClone || src.needsGoValueClone
	dst.needsGoComparable = dst.needsGoComparable || src.needsGoComparable
	dst.needsGoAnyTypeMetadata = dst.needsGoAnyTypeMetadata || src.needsGoAnyTypeMetadata
	dst.needsEmbeddedOwnerRegistry = dst.needsEmbeddedOwnerRegistry || src.needsEmbeddedOwnerRegistry
	if len(src.anyCloneTypes) > 0 {
		if dst.anyCloneTypes == nil {
			dst.anyCloneTypes = make(map[string]bool)
		}
		for rustType := range src.anyCloneTypes {
			dst.anyCloneTypes[rustType] = true
		}
	}
	dst.needsGoByteSequence = dst.needsGoByteSequence || src.needsGoByteSequence
	dst.needsGoChannel = dst.needsGoChannel || src.needsGoChannel
	dst.needsWaitGroup = dst.needsWaitGroup || src.needsWaitGroup
	dst.needsGoMutex = dst.needsGoMutex || src.needsGoMutex
	dst.needsGoRWMutex = dst.needsGoRWMutex || src.needsGoRWMutex
	dst.needsGoOnce = dst.needsGoOnce || src.needsGoOnce
	dst.needsGoAtomicPointer = dst.needsGoAtomicPointer || src.needsGoAtomicPointer
	dst.needsGoTypeName = dst.needsGoTypeName || src.needsGoTypeName
	dst.needsBase64 = dst.needsBase64 || src.needsBase64
	dst.needsSha256 = dst.needsSha256 || src.needsSha256
	dst.needsHexFormat = dst.needsHexFormat || src.needsHexFormat
	dst.needsStrconvFormat = dst.needsStrconvFormat || src.needsStrconvFormat
	dst.needsUrl = dst.needsUrl || src.needsUrl
	dst.needsRegexp = dst.needsRegexp || src.needsRegexp
	dst.needsJsonEscape = dst.needsJsonEscape || src.needsJsonEscape
	dst.needsOsFile = dst.needsOsFile || src.needsOsFile
	dst.needsSliceElemPtr = dst.needsSliceElemPtr || src.needsSliceElemPtr
	dst.needsGoTime = dst.needsGoTime || src.needsGoTime
	dst.needsGoTimer = dst.needsGoTimer || src.needsGoTimer
	dst.needsGoAfter = dst.needsGoAfter || src.needsGoAfter
	dst.needsGoTicker = dst.needsGoTicker || src.needsGoTicker
	dst.needsGoTick = dst.needsGoTick || src.needsGoTick
	dst.needsGoContext = dst.needsGoContext || src.needsGoContext
	dst.needsGoRand = dst.needsGoRand || src.needsGoRand
	dst.needsReflect = dst.needsReflect || src.needsReflect
	dst.needsGoHttpResponse = dst.needsGoHttpResponse || src.needsGoHttpResponse
	dst.needsGoPtrKey = dst.needsGoPtrKey || src.needsGoPtrKey
	dst.needsGoAnyPtrKey = dst.needsGoAnyPtrKey || src.needsGoAnyPtrKey
}

func mergeBoolMap(dst map[string]bool, src map[string]bool) {
	for key, value := range src {
		if value {
			dst[key] = true
		}
	}
}

func mergeStringMap(dst map[string]string, src map[string]string) {
	for key, value := range src {
		if value != "" {
			dst[key] = value
		}
	}
}

func mergeNestedStringMap(dst map[string]map[string]string, src map[string]map[string]string) {
	for outerKey, srcInner := range src {
		if dst[outerKey] == nil {
			dst[outerKey] = make(map[string]string)
		}
		for innerKey, value := range srcInner {
			dst[outerKey][innerKey] = value
		}
	}
}

func mergeNestedMethodMap(dst map[string]map[string]externalTypeStubMethod, src map[string]map[string]externalTypeStubMethod) {
	for outerKey, srcInner := range src {
		if dst[outerKey] == nil {
			dst[outerKey] = make(map[string]externalTypeStubMethod)
		}
		for innerKey, value := range srcInner {
			dst[outerKey][innerKey] = value
		}
	}
}

func mergeNestedBoolMap(dst map[string]map[string]bool, src map[string]map[string]bool) {
	for outerKey, srcInner := range src {
		if dst[outerKey] == nil {
			dst[outerKey] = make(map[string]bool)
		}
		for innerKey, value := range srcInner {
			if value {
				dst[outerKey][innerKey] = true
			}
		}
	}
}

func mergeExternalPackageStubs(dst map[string]*externalPackageStub, src map[string]*externalPackageStub) {
	for pkgName, srcStub := range src {
		if srcStub == nil {
			continue
		}
		dstStub := dst[pkgName]
		if dstStub == nil {
			dstStub = &externalPackageStub{
				Functions:      make(map[string]externalPackageStubFunction),
				Constants:      make(map[string]string),
				ConstantValues: make(map[string]constant.Value),
				Variables:      make(map[string]string),
			}
			dst[pkgName] = dstStub
		}
		if dstStub.Functions == nil {
			dstStub.Functions = make(map[string]externalPackageStubFunction)
		}
		if dstStub.Constants == nil {
			dstStub.Constants = make(map[string]string)
		}
		if dstStub.ConstantValues == nil {
			dstStub.ConstantValues = make(map[string]constant.Value)
		}
		if dstStub.Variables == nil {
			dstStub.Variables = make(map[string]string)
		}
		for name, fn := range srcStub.Functions {
			dstStub.Functions[name] = fn
		}
		for name, constantType := range srcStub.Constants {
			dstStub.Constants[name] = constantType
		}
		for name, constantValue := range srcStub.ConstantValues {
			if constantValue != nil {
				dstStub.ConstantValues[name] = constantValue
			}
		}
		for name, variableType := range srcStub.Variables {
			dstStub.Variables[name] = variableType
		}
	}
}

func GenerateExternalStubModuleImports() string {
	var out strings.Builder
	if NeedsConcurrentWrapper() {
		out.WriteString("use std::sync::{Arc, Mutex};\n")
	} else {
		out.WriteString("use std::cell::{RefCell};\n")
		out.WriteString("use std::rc::{Rc};\n")
	}
	out.WriteString("use std::any::Any;\n")
	out.WriteString("use std::collections::BTreeMap;\n")
	out.WriteString("use std::error::Error as StdError;\n")
	generateGoPtrKeyHelper(&out, "GoPtrKey", false)
	return out.String()
}

func externalStubsNeedJsonSupport(stubs map[string]bool, packageStubs map[string]*externalPackageStub) bool {
	if stubs["json_Decoder"] {
		return true
	}
	if pkg := packageStubs["json"]; pkg != nil {
		if _, ok := pkg.Functions["new_decoder"]; ok {
			return true
		}
		if _, ok := pkg.Functions["unmarshal"]; ok {
			return true
		}
	}
	return false
}

// TEMPORARY: hand-written Rust shim for encoding/json marshal helpers.
// Long-term fix: transpile encoding/json source (mostly pure Go reflection-driven code).
func writeJsonSupportHelpers(out *strings.Builder) {
	outerWrapper := GetOuterWrapperType()
	innerWrapper := externalStubInnerWrapperType()
	borrow := ".borrow()"
	borrowMut := ".borrow_mut()"
	errorInnerType := "Box<dyn StdError>"
	errorDynSuffix := ""
	if NeedsConcurrentWrapper() {
		borrow = ".lock().unwrap()"
		borrowMut = ".lock().unwrap()"
		errorInnerType = "Box<dyn StdError + Send + Sync>"
		errorDynSuffix = " + Send + Sync"
	}
	errorType := wrappedExternalStubType(errorInnerType)
	fmt.Fprintf(out, `pub use serde_json;

pub trait GoJsonInputArg {
    fn into_go_json_bytes(self) -> Vec<u8>;
}

pub trait GoJsonDecode: Sized {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String>;
}

pub trait GoJsonDecodeTarget {
    fn assign_go_json(self, value: &serde_json::Value) -> Result<(), String>;
}

fn go_json_no_error() -> %s {
    %s::new(%s::new(None))
}

fn go_json_error(message: String) -> %s {
    %s::new(%s::new(Some(Box::<dyn StdError%s>::from(message))))
}

pub fn go_json_expected(value: &serde_json::Value, want: &str) -> String {
    format!("expected {}, got {}", want, value)
}

impl GoJsonInputArg for Vec<u8> {
    fn into_go_json_bytes(self) -> Vec<u8> {
        self
    }
}

impl GoJsonInputArg for String {
    fn into_go_json_bytes(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl<'a> GoJsonInputArg for &'a str {
    fn into_go_json_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl<T> GoJsonInputArg for %s<%s<Option<T>>>
where
    T: GoJsonInputArg + Clone,
{
    fn into_go_json_bytes(self) -> Vec<u8> {
        self%s.as_ref().cloned().map(|value| value.into_go_json_bytes()).unwrap_or_default()
    }
}

impl GoJsonDecode for String {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_str().map(|value| value.to_string()).ok_or_else(|| go_json_expected(value, "string"))
    }
}

impl GoJsonDecode for bool {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_bool().ok_or_else(|| go_json_expected(value, "bool"))
    }
}

impl GoJsonDecode for i32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_i64().map(|value| value as i32).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for i16 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_i64().map(|value| value as i16).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for i8 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_i64().map(|value| value as i8).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for i64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_i64().ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for u8 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().map(|value| value as u8).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for u16 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().map(|value| value as u16).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for u32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().map(|value| value as u32).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for u64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for usize {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_u64().map(|value| value as usize).ok_or_else(|| go_json_expected(value, "integer"))
    }
}

impl GoJsonDecode for f64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_f64().ok_or_else(|| go_json_expected(value, "number"))
    }
}

impl GoJsonDecode for f32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        value.as_f64().map(|value| value as f32).ok_or_else(|| go_json_expected(value, "number"))
    }
}

impl<T> GoJsonDecode for Vec<T>
where
    T: GoJsonDecode,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let array = value.as_array().ok_or_else(|| go_json_expected(value, "array"))?;
        array.iter().map(T::go_json_decode).collect()
    }
}

impl<T, const N: usize> GoJsonDecode for [T; N]
where
    T: GoJsonDecode + Default,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let array = value.as_array().ok_or_else(|| go_json_expected(value, "array"))?;
        if array.len() != N {
            return Err(format!("expected array of length {}, got {}", N, array.len()));
        }
        let mut out = std::array::from_fn(|_| T::default());
        for (index, item) in array.iter().enumerate() {
            out[index] = T::go_json_decode(item)?;
        }
        Ok(out)
    }
}

impl<V> GoJsonDecode for BTreeMap<String, V>
where
    V: GoJsonDecode,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = BTreeMap::new();
        for (key, value) in object {
            out.insert(key.clone(), V::go_json_decode(value)?);
        }
        Ok(out)
    }
}

impl<T> GoJsonDecode for %s<%s<Option<T>>>
where
    T: GoJsonDecode,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        if value.is_null() {
            Ok(%s::new(%s::new(None)))
        } else {
            Ok(%s::new(%s::new(Some(T::go_json_decode(value)?))))
        }
    }
}

impl<T> GoJsonDecodeTarget for %s<%s<Option<T>>>
where
    T: GoJsonDecode,
{
    fn assign_go_json(self, value: &serde_json::Value) -> Result<(), String> {
        if value.is_null() {
            *self%s = None;
        } else {
            *self%s = Some(T::go_json_decode(value)?);
        }
        Ok(())
    }
}

`, errorType, outerWrapper, innerWrapper, errorType, outerWrapper, innerWrapper, errorDynSuffix, outerWrapper, innerWrapper, borrow, outerWrapper, innerWrapper, outerWrapper, innerWrapper, outerWrapper, innerWrapper, outerWrapper, innerWrapper, borrowMut, borrowMut)
}

// TEMPORARY: hand-written Rust shim for encoding/json.Decoder.
// Long-term fix: transpile encoding/json source.
func writeJsonDecoderStub(out *strings.Builder) {
	errorInnerType := externalStubErrorInnerType()
	fmt.Fprintf(out, `#[derive(Debug, Clone)]
pub struct json_Decoder {
    values: std::sync::Arc<std::sync::Mutex<Vec<serde_json::Value>>>,
    index: std::sync::Arc<std::sync::Mutex<usize>>,
    error: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

impl Default for json_Decoder {
    fn default() -> Self {
        Self {
            values: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            index: std::sync::Arc::new(std::sync::Mutex::new(0)),
            error: std::sync::Arc::new(std::sync::Mutex::new(None)),
        }
    }
}

impl std::fmt::Display for json_Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<json_Decoder>")
    }
}

impl json_Decoder {
    pub fn __go_from_input<T: GoJsonInputArg>(input: T) -> Self {
        let bytes = input.into_go_json_bytes();
        let text = String::from_utf8_lossy(&bytes);
        let mut values = Vec::new();
        let mut error = None;
        for item in serde_json::Deserializer::from_str(&text).into_iter::<serde_json::Value>() {
            match item {
                Ok(value) => values.push(value),
                Err(err) => {
                    error = Some(err.to_string());
                    break;
                }
            }
        }
        Self {
            values: std::sync::Arc::new(std::sync::Mutex::new(values)),
            index: std::sync::Arc::new(std::sync::Mutex::new(0)),
            error: std::sync::Arc::new(std::sync::Mutex::new(error)),
        }
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn decode<T0: GoJsonDecodeTarget>(&mut self, target: T0) -> %s {
        if let Some(err) = self.error.lock().unwrap().clone() {
            return go_json_error(err);
        }
        let value = {
            let mut index = self.index.lock().unwrap();
            let values = self.values.lock().unwrap();
            if *index >= values.len() {
                return go_json_error("EOF".to_string());
            }
            let value = values[*index].clone();
            *index += 1;
            value
        };
        match target.assign_go_json(&value) {
            Ok(()) => go_json_no_error(),
            Err(err) => go_json_error(err),
        }
    }

    pub fn more(&self) -> %s {
        let has_more = *self.index.lock().unwrap() < self.values.lock().unwrap().len();
        %s
    }
}
`, wrappedExternalStubType(errorInnerType), "bool", "has_more")
}

func generateExternalStubs(stubs map[string]bool, interfaceTypes map[string]bool, integerTypes map[string]string, tupleTypes map[string]string, fieldsByType map[string]map[string]string, methodsByType map[string]map[string]externalTypeStubMethod, conversions map[string]map[string]bool, packageStubs map[string]*externalPackageStub) string {
	if methodsByType["exec_Cmd"] != nil {
		if _, ok := methodsByType["exec_Cmd"]["stderr_pipe"]; ok {
			stubs["os_File"] = true
		}
	}
	if methodsByType["io_ReadCloser"] != nil {
		if _, ok := methodsByType["io_ReadCloser"]["close"]; ok {
			stubs["os_File"] = true
		}
	}
	needsJsonSupport := usePackageExternalStubs() || externalStubsNeedJsonSupport(stubs, packageStubs)
	if len(stubs) == 0 && len(conversions) == 0 && len(packageStubs) == 0 && !needsJsonSupport {
		return ""
	}
	names := make([]string, 0, len(stubs))
	for name := range stubs {
		if name == "types_Term" {
			continue
		}
		names = append(names, name)
	}
	slices.Sort(names)

	var out strings.Builder
	if externalStubNeedsInterfaceHelper(names, interfaceTypes) {
		writeExternalInterfaceIdHelper(&out)
	}
	if needsJsonSupport {
		writeJsonSupportHelpers(&out)
	}
	if externalPackageStubsNeedGoTimer(packageStubs) {
		generateGoTimerHelper(&out)
	}
	for i, name := range names {
		if i > 0 || out.Len() > 0 {
			out.WriteString("\n\n")
		}
		if name == "json_Decoder" {
			writeJsonDecoderStub(&out)
			continue
		}
		if name == "io_Writer" {
			writeIoWriterStub(&out)
			continue
		}
		if name == "os_File" {
			writeOsFileStub(&out)
			continue
		}
		if name == "exec_Cmd" {
			writeExecCmdTypeStub(&out, fieldsByType[name], methodsByType[name])
			continue
		}
		if name == "atomic_Int32" {
			writeAtomicInt32Stub(&out)
			continue
		}
		if name == "atomic_Uint64" {
			writeAtomicUint64Stub(&out)
			continue
		}
		if name == "fs_FileInfo" {
			writeFsFileInfoStub(&out, name, methodsByType[name])
			continue
		}
		if name == "fs_DirEntry" {
			writeFsDirEntryStub(&out, name, methodsByType[name])
			continue
		}
		if interfaceTypes[name] {
			writeExternalInterfaceStub(&out, name, methodsByType[name])
			continue
		}
		fields := fieldsByType[name]
		if len(fields) == 0 {
			if integerType := integerTypes[name]; integerType != "" {
				out.WriteString("#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]\n")
				out.WriteString("pub struct ")
				out.WriteString(name)
				out.WriteString("(pub ")
				out.WriteString(integerType)
				out.WriteString(");\n\n")
				writeExternalIntegerStubOps(&out, name, integerType)
			} else if tupleType := tupleTypes[name]; tupleType != "" {
				out.WriteString("#[derive(Debug, Clone, Default)]\n")
				out.WriteString("pub struct ")
				out.WriteString(name)
				out.WriteString("(pub ")
				out.WriteString(tupleType)
				out.WriteString(");\n")
			} else {
				out.WriteString("#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]\n")
				out.WriteString("pub struct ")
				out.WriteString(name)
				out.WriteString(";\n\n")
			}
		} else {
			customDefault := externalStubStructNeedsCustomDefault(name)
			if externalStubFieldsCanDeriveDebug(fields) {
				if customDefault {
					out.WriteString("#[derive(Debug, Clone)]\n")
				} else {
					out.WriteString("#[derive(Debug, Clone, Default)]\n")
				}
			} else {
				if customDefault {
					out.WriteString("#[derive(Clone)]\n")
				} else {
					out.WriteString("#[derive(Clone, Default)]\n")
				}
			}
			out.WriteString("pub struct ")
			out.WriteString(name)
			out.WriteString(" {\n")
			fieldNames := make([]string, 0, len(fields))
			for fieldName := range fields {
				fieldNames = append(fieldNames, fieldName)
			}
			slices.Sort(fieldNames)
			for _, fieldName := range fieldNames {
				out.WriteString("    pub ")
				out.WriteString(fieldName)
				out.WriteString(": ")
				out.WriteString(fields[fieldName])
				out.WriteString(",\n")
			}
			out.WriteString("}\n\n")
			if customDefault {
				writeExternalStubStructDefault(&out, name, fields)
			}
		}
		out.WriteString("impl std::fmt::Display for ")
		out.WriteString(name)
		out.WriteString(" {\n")
		out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
		out.WriteString("        write!(f, \"<")
		out.WriteString(name)
		out.WriteString(">\")\n")
		out.WriteString("    }\n")
		out.WriteString("}\n")
		methods := methodsByType[name]
		if externalTypeStubHasErrorMethod(methods) {
			out.WriteString("\nimpl std::error::Error for ")
			out.WriteString(name)
			out.WriteString(" {}\n")
		}
		out.WriteString("\n\nimpl ")
		out.WriteString(name)
		out.WriteString(" {\n")
		writeExternalTypeStubDowncastMethod(&out)
		methodNames := make([]string, 0, len(methods))
		for methodName := range methods {
			methodNames = append(methodNames, methodName)
		}
		slices.Sort(methodNames)
		for _, methodName := range methodNames {
			method := methods[methodName]
			if name == "types_Checker" && methodName == "files" {
				continue
			}
			if name == "types_Config" && methodName == "check" {
				continue
			}
			if name == "token_Pos" && methodName == "is_valid" {
				continue
			}
			if name == "fs_FileMode" && methodName == "is_dir" {
				writeFsFileModeIsDirMethod(&out)
			} else {
				writeExternalTypeStubMethod(&out, name, methodName, method)
			}
		}
		out.WriteString("}\n")
	}
	writeExternalTypeStubConversions(&out, conversions, interfaceTypes)
	writeExternalPackageStubs(&out, packageStubs, integerTypes, stubs, len(names) > 0)
	return out.String()
}

func externalPackageStubsNeedGoTimer(packageStubs map[string]*externalPackageStub) bool {
	for _, pkg := range packageStubs {
		if pkg == nil {
			continue
		}
		for _, fn := range pkg.Functions {
			for _, returnType := range fn.ReturnTypes {
				if strings.Contains(returnType, "GoTimer") {
					return true
				}
			}
		}
		for _, rustType := range pkg.Variables {
			if strings.Contains(rustType, "GoTimer") {
				return true
			}
		}
	}
	return false
}

func externalStubNeedsInterfaceHelper(names []string, interfaceTypes map[string]bool) bool {
	for _, name := range names {
		if interfaceTypes[name] {
			return true
		}
	}
	return false
}

func externalStubBorrowExpr(expr string) string {
	if NeedsConcurrentWrapper() {
		return expr + ".lock().unwrap()"
	}
	return expr + ".borrow()"
}

func externalStubErrorInnerType() string {
	if NeedsConcurrentWrapper() {
		return "Box<dyn StdError + Send + Sync>"
	}
	return "Box<dyn StdError>"
}

// TEMPORARY: hand-written Rust shim for io.Writer trait bridging.
// Long-term fix: transpile io package interfaces (pure Go).
func writeIoWriterStub(out *strings.Builder) {
	holderType := "Rc<dyn std::any::Any>"
	fromBound := "T: 'static"
	newValue := "Rc::new(value)"
	defaultValue := "Rc::new(())"
	if NeedsConcurrentWrapper() {
		holderType = "Arc<dyn std::any::Any + Send + Sync>"
		fromBound = "T: 'static + Send + Sync"
		newValue = "Arc::new(value)"
		defaultValue = "Arc::new(())"
	}
	vecType := wrappedExternalStubType("Vec<u8>")
	// Predeclared Copy scalar return slots stay bare to match the widened
	// signatures user-defined io.Writer impls and direct callers use.
	intType := "i32"
	errorInnerType := externalStubErrorInnerType()
	errorType := wrappedExternalStubType(errorInnerType)
	noneError := wrappedExternalStubNoneExpr(errorInnerType)
	vecBorrow := externalStubBorrowExpr("v")
	out.WriteString(`#[derive(Clone)]
pub struct io_Writer {
    pub __go_id: usize,
    pub __go_value: `)
	out.WriteString(holderType)
	out.WriteString(`,
}

impl io_Writer {
    pub fn __go_from<`)
	out.WriteString(fromBound)
	out.WriteString(`>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: `)
	out.WriteString(newValue)
	fmt.Fprintf(out, ` }
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        let _ = data;
        panic!("io_Writer.__go_write_bytes bridge: unsupported concrete receiver; transpile io source instead - see AGENTS.md")
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (%s, %s) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<%s>() {
            %s.as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (%s, %s)
    }
}

impl Default for io_Writer {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: `, intType, errorType, vecType, vecBorrow, "n", noneError)
	out.WriteString(defaultValue)
	out.WriteString(` }
    }
}

impl std::fmt::Debug for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<io_Writer>")
    }
}

impl std::fmt::Display for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Writer>")
    }
}

impl PartialEq for io_Writer {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for io_Writer {}

impl PartialOrd for io_Writer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for io_Writer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}
`)
}

// PERMANENT: not scaffold — Rust std::sync::atomic is the long-term implementation;
// Go's sync/atomic semantics cannot be transpiled from Go source (runtime-tied).
func writeAtomicInt32Stub(out *strings.Builder) {
	// Predeclared Copy scalar return slots stay bare to match the rest of
	// the transpiler's signature shape — callers expect a raw value.
	intType := "i32"
	boolType := "bool"
	fmt.Fprintf(out, `#[derive(Debug, Clone)]
pub struct atomic_Int32 {
    __go_value: std::sync::Arc<std::sync::atomic::AtomicI32>,
}

impl Default for atomic_Int32 {
    fn default() -> Self {
        Self { __go_value: std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0)) }
    }
}

impl std::fmt::Display for atomic_Int32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<atomic_Int32>")
    }
}

fn __go_atomic_i32_arg<T: 'static>(arg: &T) -> i32 {
    let any = arg as &dyn std::any::Any;
    if let Some(v) = any.downcast_ref::<i32>() {
        *v
    } else if let Some(v) = any.downcast_ref::<i64>() {
        *v as i32
    } else if let Some(v) = any.downcast_ref::<u32>() {
        *v as i32
    } else if let Some(v) = any.downcast_ref::<u64>() {
        *v as i32
    } else if let Some(v) = any.downcast_ref::<usize>() {
        *v as i32
    } else if let Some(v) = any.downcast_ref::<isize>() {
        *v as i32
    } else if let Some(v) = any.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<i32>>>>() {
        v.lock().unwrap().as_ref().copied().unwrap_or_default()
    } else if let Some(v) = any.downcast_ref::<std::rc::Rc<std::cell::RefCell<Option<i32>>>>() {
        v.borrow().as_ref().copied().unwrap_or_default()
    } else {
        0
    }
}

impl atomic_Int32 {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn add<T0: 'static>(&self, arg0: T0) -> %s {
        let delta = __go_atomic_i32_arg(&arg0);
        let previous = self.__go_value.fetch_add(delta, std::sync::atomic::Ordering::SeqCst);
        %s
    }

    pub fn load(&self) -> %s {
        %s
    }

    pub fn store<T0: 'static>(&self, arg0: T0) {
        self.__go_value.store(__go_atomic_i32_arg(&arg0), std::sync::atomic::Ordering::SeqCst);
    }

    pub fn swap<T0: 'static>(&self, arg0: T0) -> %s {
        %s
    }

    pub fn compare_and_swap<T0: 'static, T1: 'static>(&self, old: T0, new: T1) -> %s {
        let old = __go_atomic_i32_arg(&old);
        let new = __go_atomic_i32_arg(&new);
        %s
    }
}
`,
		intType, "previous.wrapping_add(delta)",
		intType, "self.__go_value.load(std::sync::atomic::Ordering::SeqCst)",
		intType, "self.__go_value.swap(__go_atomic_i32_arg(&arg0), std::sync::atomic::Ordering::SeqCst)",
		boolType, "self.__go_value.compare_exchange(old, new, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst).is_ok()")
}

// PERMANENT: not scaffold - Rust std::sync::atomic is the long-term implementation;
// Go's sync/atomic semantics cannot be transpiled from Go source (runtime-tied).
func writeAtomicUint64Stub(out *strings.Builder) {
	out.WriteString(`#[derive(Debug, Clone)]
pub struct atomic_Uint64 {
    __go_value: std::sync::Arc<std::sync::atomic::AtomicU64>,
}

impl Default for atomic_Uint64 {
    fn default() -> Self {
        Self { __go_value: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)) }
    }
}

impl std::fmt::Display for atomic_Uint64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<atomic_Uint64>")
    }
}

fn __go_atomic_u64_arg<T: 'static>(arg: &T) -> u64 {
    let any = arg as &dyn std::any::Any;
    if let Some(v) = any.downcast_ref::<u64>() {
        *v
    } else if let Some(v) = any.downcast_ref::<u32>() {
        *v as u64
    } else if let Some(v) = any.downcast_ref::<usize>() {
        *v as u64
    } else if let Some(v) = any.downcast_ref::<i32>() {
        *v as u64
    } else if let Some(v) = any.downcast_ref::<i64>() {
        *v as u64
    } else if let Some(v) = any.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<u64>>>>() {
        v.lock().unwrap().as_ref().copied().unwrap_or_default()
    } else if let Some(v) = any.downcast_ref::<std::rc::Rc<std::cell::RefCell<Option<u64>>>>() {
        v.borrow().as_ref().copied().unwrap_or_default()
    } else {
        panic!("atomic_Uint64 helper: unsupported argument type; use a typed sync/atomic.Uint64 value")
    }
}

impl atomic_Uint64 {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn add<T0: 'static>(&self, arg0: T0) -> u64 {
        let delta = __go_atomic_u64_arg(&arg0);
        let previous = self.__go_value.fetch_add(delta, std::sync::atomic::Ordering::SeqCst);
        previous.wrapping_add(delta)
    }

    pub fn load(&self) -> u64 {
        self.__go_value.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn store<T0: 'static>(&self, arg0: T0) {
        self.__go_value.store(__go_atomic_u64_arg(&arg0), std::sync::atomic::Ordering::SeqCst);
    }

    pub fn compare_and_swap<T0: 'static, T1: 'static>(&self, old: T0, new: T1) -> bool {
        let old = __go_atomic_u64_arg(&old);
        let new = __go_atomic_u64_arg(&new);
        self.__go_value.compare_exchange(old, new, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst).is_ok()
    }
}
`)
}

// PERMANENT: not scaffold — Rust std::fs::File is the long-term implementation;
// os.File wraps OS file handles, no transpilable Go source equivalent.
func writeOsFileStub(out *strings.Builder) {
	vecType := wrappedExternalStubType("Vec<u8>")
	int64Type := wrappedExternalStubType("i64")
	stringType := wrappedExternalStubType("String")
	intType := wrappedExternalStubType("i32")
	errorInnerType := externalStubErrorInnerType()
	errorType := wrappedExternalStubType(errorInnerType)
	noneError := wrappedExternalStubNoneExpr(errorInnerType)
	vecBorrow := externalStubBorrowExpr("v")
	stringBorrow := externalStubBorrowExpr("v")
	int64Borrow := ".borrow()"
	vecMutBorrow := ".borrow_mut()"
	if NeedsConcurrentWrapper() {
		int64Borrow = ".lock().unwrap()"
		vecMutBorrow = ".lock().unwrap()"
	}
	fmt.Fprintf(out, `#[derive(Debug, Clone)]
pub struct os_File {
    pub __go_data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
    pub __go_closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub __go_wait_for_close: bool,
}

impl Default for os_File {
    fn default() -> Self {
        Self {
            __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            __go_wait_for_close: false,
        }
    }
}

impl std::fmt::Display for os_File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<os_File>")
    }
}

impl PartialEq for os_File {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__go_data, &other.__go_data)
    }
}

impl Eq for os_File {}

impl os_File {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        self.__go_data.lock().unwrap().extend_from_slice(data);
    }

    pub fn __go_read_all(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    pub fn close(&self) -> %s {
        self.__go_closed.store(true, std::sync::atomic::Ordering::SeqCst);
        %s
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (%s, %s) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<%s>() {
            %s.as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (%s, %s)
    }

    pub fn write_string<T0: 'static>(&self, arg0: T0) -> (%s, %s) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<&str>() {
            (*v).to_string()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<%s>() {
            %s.as_ref().cloned().unwrap_or_default()
        } else {
            String::new()
        };
        let bytes = value.into_bytes();
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (%s, %s)
    }

    pub fn read<T0>(&self, _arg0: T0) -> (%s, %s) {
        (%s, %s)
    }

    pub fn read_at<T0: 'static, T1: 'static>(&self, arg0: T0, arg1: T1) -> (%s, %s) {
        let offset = if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<i64>() {
            *v
        } else if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<%s>() {
            v%s.as_ref().copied().unwrap_or_default()
        } else {
            0
        };
        let data = self.__go_read_all();
        let mut n = 0i32;
        if offset >= 0 {
            let start = offset as usize;
            if start < data.len() {
                if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<%s>() {
                    let mut guard = v%s;
                    if let Some(target) = guard.as_mut() {
                        let count = std::cmp::min(target.len(), data.len() - start);
                        target[..count].copy_from_slice(&data[start..start + count]);
                        n = count as i32;
                    }
                }
            }
        }
        (%s, %s)
    }
}
`,
		errorType, noneError,
		intType, errorType, vecType, vecBorrow, wrappedExternalStubExpr("i32", "n"), noneError,
		intType, errorType, stringType, stringBorrow, wrappedExternalStubExpr("i32", "n"), noneError,
		intType, errorType, wrappedExternalStubExpr("i32", "0"), noneError,
		intType, errorType, int64Type, int64Borrow, vecType, vecMutBorrow, wrappedExternalStubExpr("i32", "n"), noneError)
}

// PERMANENT: not scaffold — Rust std::process::Command is the long-term implementation;
// exec.Cmd is OS process layer, no transpilable Go source equivalent.
func writeExecCmdTypeStub(out *strings.Builder, fields map[string]string, methods map[string]externalTypeStubMethod) {
	if fields == nil {
		fields = make(map[string]string)
	}
	if _, ok := fields["args"]; !ok {
		fields["args"] = wrappedExternalStubType("Vec<String>")
	}
	if _, ok := fields["env"]; !ok {
		fields["env"] = wrappedExternalStubType("Vec<String>")
	}
	out.WriteString("#[derive(Debug, Clone, Default)]\n")
	out.WriteString("pub struct exec_Cmd {\n")
	fieldNames := make([]string, 0, len(fields))
	for fieldName := range fields {
		fieldNames = append(fieldNames, fieldName)
	}
	slices.Sort(fieldNames)
	for _, fieldName := range fieldNames {
		out.WriteString("    pub ")
		out.WriteString(fieldName)
		out.WriteString(": ")
		out.WriteString(fields[fieldName])
		out.WriteString(",\n")
	}
	errorInnerType := externalStubErrorInnerType()
	errorType := wrappedExternalStubType(errorInnerType)
	noneError := wrappedExternalStubNoneExpr(errorInnerType)
	errorTrait := strings.TrimSuffix(strings.TrimPrefix(errorInnerType, "Box<"), ">")
	out.WriteString(`}

impl std::fmt::Display for exec_Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<exec_Cmd>")
    }
}

impl exec_Cmd {
	    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
	        None
	    }

	`)
	fmt.Fprintf(out, `    pub fn environ(&self) -> %s {
        let mut env: Vec<String> = std::env::vars().map(|(__k, __v)| format!("{}={}", __k, __v)).collect();
        if let Some(cmd_env) = %s.as_ref() {
            env.extend(cmd_env.iter().cloned());
        }
        %s
    }

`, wrappedExternalStubType("Vec<String>"), externalStubBorrowExpr("self.env"), wrappedExternalStubExpr("Vec<String>", "env"))
	if method, ok := methods["stderr_pipe"]; ok && len(method.ReturnTypes) == 2 {
		fmt.Fprintf(out, `    pub fn stderr_pipe(&mut self) -> (%s, %s) {
        let file = os_File::default();
        (%s, %s)
    }

`, method.ReturnTypes[0], method.ReturnTypes[1],
			wrappedExternalStubExpr("io_ReadCloser", "io_ReadCloser::__go_from(file)"), noneError)
	}
	fmt.Fprintf(out, `    fn __go_error(message: String) -> %s {
	        %s
	    }

    fn __go_run_output(&self) -> Result<std::process::Output, std::io::Error> {
        let args = %s.as_ref().cloned().unwrap_or_default();
        if args.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "empty command"));
        }
        let mut command = std::process::Command::new(&args[0]);
        command.args(&args[1..]);
`, errorType, wrappedExternalStubExpr(errorInnerType, fmt.Sprintf("Box::<%s>::from(message)", errorTrait)), externalStubBorrowExpr("self.args"))
	if _, ok := fields["dir"]; ok {
		fmt.Fprintf(out, `        if let Some(dir) = %s.as_ref() {
            if !dir.is_empty() {
                command.current_dir(dir);
            }
        }
`, externalStubBorrowExpr("self.dir"))
	}
	if _, ok := fields["env"]; ok {
		fmt.Fprintf(out, `        if let Some(env) = %s.as_ref() {
            for item in env {
                if let Some((key, value)) = item.split_once('=') {
                    command.env(key, value);
                }
            }
        }
`, externalStubBorrowExpr("self.env"))
	}
	out.WriteString(`        command.output()
    }

    fn __go_write_output(&self, output: &std::process::Output) {
`)
	if _, ok := fields["stdout"]; ok {
		fmt.Fprintf(out, `        if let Some(stdout) = %s.as_ref() {
            stdout.__go_write_bytes(&output.stdout);
        }
`, externalStubBorrowExpr("self.stdout"))
	}
	if _, ok := fields["stderr"]; ok {
		fmt.Fprintf(out, `        if let Some(stderr) = %s.as_ref() {
            stderr.__go_write_bytes(&output.stderr);
        }
`, externalStubBorrowExpr("self.stderr"))
	}
	fmt.Fprintf(out, `    }

    pub fn output(&self) -> (%s, %s) {
        match self.__go_run_output() {
            Ok(output) => {
                let err = if output.status.success() {
                    %s
                } else {
                    Self::__go_error(format!("exit status {}", output.status))
                };
                (%s, err)
            }
            Err(err) => (%s, Self::__go_error(err.to_string())),
        }
    }

    pub fn run(&self) -> %s {
        self.start()
    }

    pub fn start(&self) -> %s {
        match self.__go_run_output() {
            Ok(output) => {
                self.__go_write_output(&output);
                if output.status.success() {
                    %s
                } else {
                    Self::__go_error(format!("exit status {}", output.status))
                }
            }
            Err(err) => Self::__go_error(err.to_string()),
        }
    }

    pub fn wait(&self) -> %s {
        %s
    }
}
`, wrappedExternalStubType("Vec<u8>"), errorType, noneError, wrappedExternalStubExpr("Vec<u8>", "output.stdout"), wrappedExternalStubExpr("Vec<u8>", "Vec::new()"), errorType, errorType, noneError, errorType, noneError)
}

func externalTypeStubHasErrorMethod(methods map[string]externalTypeStubMethod) bool {
	if methods == nil {
		return false
	}
	method, ok := methods["error"]
	return ok && len(method.ReturnTypes) == 1 && strings.Contains(method.ReturnTypes[0], "String")
}

// MACHINERY: emission framework for integer-stub operations, not a stdlib shim.
func writeExternalIntegerStubOps(out *strings.Builder, name string, integerType string) {
	out.WriteString("impl PartialEq<")
	out.WriteString(integerType)
	out.WriteString("> for ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    fn eq(&self, other: &")
	out.WriteString(integerType)
	out.WriteString(") -> bool {\n")
	out.WriteString("        self.0 == *other\n")
	out.WriteString("    }\n")
	out.WriteString("}\n\n")

	out.WriteString("impl PartialEq<")
	out.WriteString(name)
	out.WriteString("> for ")
	out.WriteString(integerType)
	out.WriteString(" {\n")
	out.WriteString("    fn eq(&self, other: &")
	out.WriteString(name)
	out.WriteString(") -> bool {\n")
	out.WriteString("        *self == other.0\n")
	out.WriteString("    }\n")
	out.WriteString("}\n\n")

	out.WriteString("impl std::ops::BitAnd for ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    type Output = ")
	out.WriteString(name)
	out.WriteString(";\n")
	out.WriteString("    fn bitand(self, other: Self) -> ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("        ")
	out.WriteString(name)
	out.WriteString("(self.0 & other.0)\n")
	out.WriteString("    }\n")
	out.WriteString("}\n\n")

	out.WriteString("impl std::ops::BitOr for ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    type Output = ")
	out.WriteString(name)
	out.WriteString(";\n")
	out.WriteString("    fn bitor(self, other: Self) -> ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("        ")
	out.WriteString(name)
	out.WriteString("(self.0 | other.0)\n")
	out.WriteString("    }\n")
	out.WriteString("}\n\n")
}

// MACHINERY: generic interface-stub emitter framework, not a stdlib shim.
func writeExternalInterfaceStub(out *strings.Builder, name string, methods map[string]externalTypeStubMethod) {
	holderType := "Rc<dyn std::any::Any>"
	fromBound := "T: 'static"
	newValue := "Rc::new(value)"
	defaultValue := "Rc::new(())"
	if NeedsConcurrentWrapper() {
		holderType = "Arc<dyn std::any::Any + Send + Sync>"
		fromBound = "T: 'static + Send + Sync"
		newValue = "Arc::new(value)"
		defaultValue = "Arc::new(())"
	}

	out.WriteString("#[derive(Clone)]\n")
	out.WriteString("pub struct ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    pub __go_id: usize,\n")
	if externalInterfaceCarriesSourcePos(name) {
		out.WriteString("    pub __go_pos: i32,\n")
	}
	out.WriteString("    pub __go_value: ")
	out.WriteString(holderType)
	out.WriteString(",\n")
	out.WriteString("}\n\n")

	out.WriteString("impl ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    pub fn __go_from<")
	out.WriteString(fromBound)
	out.WriteString(">(value: T) -> Self {\n")
	out.WriteString("        Self { __go_id: __go_next_external_interface_id(), ")
	if externalInterfaceCarriesSourcePos(name) {
		out.WriteString("__go_pos: 0, ")
	}
	out.WriteString("__go_value: ")
	out.WriteString(newValue)
	out.WriteString(" }\n")
	out.WriteString("    }\n")
	if externalInterfaceCarriesSourcePos(name) {
		out.WriteString("    pub fn __go_from_with_pos<")
		out.WriteString(fromBound)
		out.WriteString(">(value: T, pos: i32) -> Self {\n")
		out.WriteString("        Self { __go_id: __go_next_external_interface_id(), __go_pos: pos, __go_value: ")
		out.WriteString(newValue)
		out.WriteString(" }\n")
		out.WriteString("    }\n")
	}
	out.WriteString("    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {\n")
	out.WriteString("        self.__go_value.as_ref().downcast_ref::<T>()\n")
	out.WriteString("    }\n")
	methodNames := make([]string, 0, len(methods))
	for methodName := range methods {
		methodNames = append(methodNames, methodName)
	}
	slices.Sort(methodNames)
	for _, methodName := range methodNames {
		if name == "types_Type" && (methodName == "string" || methodName == "underlying") {
			continue
		}
		if externalInterfaceCarriesSourcePos(name) && methodName == "pos" {
			writeExternalInterfacePosMethod(out)
		} else if name == "io_ReadCloser" && methodName == "close" {
			writeIoReadCloserCloseMethod(out, methods[methodName])
		} else {
			writeExternalTypeStubMethod(out, name, methodName, methods[methodName])
		}
	}
	out.WriteString("}\n\n")

	out.WriteString("impl Default for ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    fn default() -> Self {\n")
	out.WriteString("        Self { __go_id: 0, ")
	if externalInterfaceCarriesSourcePos(name) {
		out.WriteString("__go_pos: 0, ")
	}
	out.WriteString("__go_value: ")
	out.WriteString(defaultValue)
	out.WriteString(" }\n")
	out.WriteString("    }\n")
	out.WriteString("}\n\n")

	out.WriteString("impl std::fmt::Debug for ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
	out.WriteString("        write!(f, \"<")
	out.WriteString(name)
	out.WriteString(">\")\n")
	out.WriteString("    }\n")
	out.WriteString("}\n\n")

	out.WriteString("impl std::fmt::Display for ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
	out.WriteString("        write!(f, \"<")
	out.WriteString(name)
	out.WriteString(">\")\n")
	out.WriteString("    }\n")
	out.WriteString("}\n\n")

	out.WriteString("impl PartialEq for ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    fn eq(&self, other: &Self) -> bool {\n")
	out.WriteString("        self.__go_id == other.__go_id\n")
	out.WriteString("    }\n")
	out.WriteString("}\n\n")
	out.WriteString("impl Eq for ")
	out.WriteString(name)
	out.WriteString(" {}\n\n")
	out.WriteString("impl PartialOrd for ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {\n")
	out.WriteString("        Some(self.cmp(other))\n")
	out.WriteString("    }\n")
	out.WriteString("}\n\n")
	out.WriteString("impl Ord for ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    fn cmp(&self, other: &Self) -> std::cmp::Ordering {\n")
	out.WriteString("        self.__go_id.cmp(&other.__go_id)\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func externalInterfaceCarriesSourcePos(name string) bool {
	switch name {
	case "ast_Decl", "ast_Expr", "ast_Node", "ast_Spec", "ast_Stmt":
		return true
	default:
		return false
	}
}

// MACHINERY: shared Pos() method emitter used by interface-stub framework.
func writeExternalInterfacePosMethod(out *strings.Builder) {
	out.WriteString("    pub fn pos(&self) -> Arc<Mutex<Option<token_Pos>>> {\n")
	out.WriteString("        Arc::new(Mutex::new(Some(token_Pos(self.__go_pos))))\n")
	out.WriteString("    }\n")
}

// TEMPORARY: hand-written Rust shim for io.ReadCloser close dispatch.
// Long-term fix: source-map io/os and delete the external io_ReadCloser bridge.
// Registry row: docs/bridge_debt.md#io-readcloser-close-dispatch.
// Interface dispatch bridge for io.ReadCloser backed by os.File.
// Unsupported concrete receivers panic loudly instead of synthesizing success.
func writeIoReadCloserCloseMethod(out *strings.Builder, method externalTypeStubMethod) {
	out.WriteString("    pub fn close(&self) -> ")
	writeExternalStubReturnType(out, method.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        if let Some(file) = self.downcast_ref::<os_File>() {\n")
	out.WriteString("            return file.close();\n")
	out.WriteString("        }\n")
	out.WriteString("        panic!(\"io_ReadCloser.close bridge: unsupported concrete receiver; transpile io/os source instead - see AGENTS.md and docs/bridge_debt.md#io-readcloser-close-dispatch\")\n")
	out.WriteString("    }\n")
}

// MACHINERY: framework for emitting From/Into impls between external stub types.
func writeExternalTypeStubConversions(out *strings.Builder, conversions map[string]map[string]bool, interfaceTypes map[string]bool) {
	if len(conversions) == 0 {
		return
	}
	targetNames := make([]string, 0, len(conversions))
	for targetName, sourceNames := range conversions {
		if len(sourceNames) == 0 {
			continue
		}
		targetNames = append(targetNames, targetName)
	}
	slices.Sort(targetNames)
	for _, targetName := range targetNames {
		sourceNames := make([]string, 0, len(conversions[targetName]))
		for sourceName := range conversions[targetName] {
			sourceNames = append(sourceNames, sourceName)
		}
		slices.Sort(sourceNames)
		for _, sourceName := range sourceNames {
			if out.Len() > 0 {
				out.WriteString("\n\n")
			}
			out.WriteString("impl From<")
			out.WriteString(sourceName)
			out.WriteString("> for ")
			out.WriteString(targetName)
			out.WriteString(" {\n")
			out.WriteString("    fn from(_value: ")
			out.WriteString(sourceName)
			out.WriteString(") -> Self {\n")
			if interfaceTypes[targetName] && interfaceTypes[sourceName] {
				out.WriteString("        Self { __go_id: _value.__go_id, ")
				if externalInterfaceCarriesSourcePos(targetName) {
					if externalInterfaceCarriesSourcePos(sourceName) {
						out.WriteString("__go_pos: _value.__go_pos, ")
					} else {
						out.WriteString("__go_pos: 0, ")
					}
				}
				out.WriteString("__go_value: _value.__go_value.clone() }\n")
			} else if interfaceTypes[targetName] {
				out.WriteString("        Self::__go_from(_value)\n")
			} else {
				// Per AGENTS.md "Strategy: Transpile stdlib, don't bridge it":
				// From<X> for Y impls between two concrete stub types have no
				// meaningful conversion. Returning Self::default() silently
				// synthesizes type facts. Panic instead so callers fail loudly.
				out.WriteString("        panic!(\"From<")
				out.WriteString(sourceName)
				out.WriteString("> for ")
				out.WriteString(targetName)
				out.WriteString(" bridge: concrete-to-concrete stub conversion has no implementation; transpile the underlying Go source or remove the conversion — see AGENTS.md and docs/bridge_debt.md\")\n")
			}
			out.WriteString("    }\n")
			out.WriteString("}\n")
		}
	}
}

// MACHINERY: interface identity helper for the emission framework.
func writeExternalInterfaceIdHelper(out *strings.Builder) {
	out.WriteString("fn __go_next_external_interface_id() -> usize {\n")
	out.WriteString("    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);\n")
	out.WriteString("    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)\n")
	out.WriteString("}\n\n")
}

// MACHINERY: downcast helper for the external-stub interface framework.
func writeExternalTypeStubDowncastMethod(out *strings.Builder) {
	out.WriteString("    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {\n")
	out.WriteString("        None\n")
	out.WriteString("    }\n")
}

// MACHINERY: generic method emitter for external-stub framework.
//
// Per AGENTS.md "Strategy: Transpile stdlib, don't bridge it", the body
// MUST panic instead of returning a default value. A bridge method that
// silently returns Default::default() synthesizes type facts and re-enacts
// the 2026 fallback incident one layer deeper. Methods that need real
// behavior get a custom emitter. Generic methods exist for type-system
// completeness only — calling them at runtime is a bug to be fixed by
// source-transpiling the Go package that defines the method.
func writeExternalTypeStubMethod(out *strings.Builder, typeName string, methodName string, method externalTypeStubMethod) {
	out.WriteString("    pub fn ")
	out.WriteString(methodName)
	if method.ParamCount > 0 {
		out.WriteString("<")
		for i := 0; i < method.ParamCount; i++ {
			if i > 0 {
				out.WriteString(", ")
			}
			out.WriteString("T")
			out.WriteString(strconv.Itoa(i))
		}
		out.WriteString(">")
	}
	out.WriteString("(&self")
	for i := 0; i < method.ParamCount; i++ {
		out.WriteString(", _arg")
		out.WriteString(strconv.Itoa(i))
		out.WriteString(": T")
		out.WriteString(strconv.Itoa(i))
	}
	out.WriteString(")")
	if len(method.ReturnTypes) > 0 {
		out.WriteString(" -> ")
		if len(method.ReturnTypes) == 1 {
			out.WriteString(method.ReturnTypes[0])
		} else {
			out.WriteString("(")
			for i, returnType := range method.ReturnTypes {
				if i > 0 {
					out.WriteString(", ")
				}
				out.WriteString(returnType)
			}
			out.WriteString(")")
		}
	}
	out.WriteString(" {\n")
	out.WriteString("        panic!(\"")
	out.WriteString(typeName)
	out.WriteString(".")
	out.WriteString(methodName)
	out.WriteString(" bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md\")\n")
	out.WriteString("    }\n")
}

// MACHINERY: top-level dispatcher that emits all package stub blocks.
func writeExternalPackageStubs(out *strings.Builder, packageStubs map[string]*externalPackageStub, integerTypes map[string]string, stubs map[string]bool, needsSeparator bool) {
	if len(packageStubs) == 0 {
		return
	}
	pkgNames := make([]string, 0, len(packageStubs))
	for pkgName := range packageStubs {
		pkgNames = append(pkgNames, pkgName)
	}
	slices.Sort(pkgNames)
	for _, pkgName := range pkgNames {
		pkg := packageStubs[pkgName]
		if pkg == nil || (len(pkg.Functions) == 0 && len(pkg.Constants) == 0 && len(pkg.Variables) == 0) {
			continue
		}
		if needsSeparator || out.Len() > 0 {
			out.WriteString("\n\n")
		}
		needsSeparator = true
		if pkgName == "exec" {
			writeExecPackageStub(out, pkg, integerTypes)
			continue
		}
		if pkgName == "json" {
			writeJsonPackageStub(out, pkg)
			continue
		}
		if pkgName == "os" {
			writeOsPackageStub(out, pkg, integerTypes)
			continue
		}
		if pkgName == "filepath" {
			writeFilepathPackageStub(out, pkg, integerTypes)
			continue
		}
		out.WriteString("pub mod ")
		out.WriteString(ToSnakeCase(pkgName))
		out.WriteString(" {\n")
		out.WriteString("    use super::*;\n")
		constNames := make([]string, 0, len(pkg.Constants))
		for constName := range pkg.Constants {
			constNames = append(constNames, constName)
		}
		slices.Sort(constNames)
		for _, constName := range constNames {
			out.WriteString("    pub const ")
			out.WriteString(constName)
			out.WriteString(": ")
			out.WriteString(pkg.Constants[constName])
			out.WriteString(" = ")
			writeExternalPackageStubConstValue(out, pkg, constName, integerTypes)
			out.WriteString(";\n")
		}
		if len(constNames) > 0 && (len(pkg.Functions) > 0 || len(pkg.Variables) > 0) {
			out.WriteString("\n")
		}
		varNames := make([]string, 0, len(pkg.Variables))
		for varName := range pkg.Variables {
			varNames = append(varNames, varName)
		}
		slices.Sort(varNames)
		for i, varName := range varNames {
			if i > 0 {
				out.WriteString("\n")
			}
			writeExternalPackageStubVariable(out, varName, pkg.Variables[varName])
		}
		if len(varNames) > 0 && len(pkg.Functions) > 0 {
			out.WriteString("\n")
		}
		funcNames := make([]string, 0, len(pkg.Functions))
		for funcName := range pkg.Functions {
			funcNames = append(funcNames, funcName)
		}
		slices.Sort(funcNames)
		for i, funcName := range funcNames {
			if i > 0 {
				out.WriteString("\n")
			}
			writeExternalPackageStubFunction(out, pkgName, funcName, pkg.Functions[funcName])
		}
		out.WriteString("}\n")
	}
}

func wrappedExternalStubType(innerType string) string {
	return fmt.Sprintf("%s<%s<Option<%s>>>", GetOuterWrapperType(), externalStubInnerWrapperType(), innerType)
}

func wrappedExternalStubExpr(innerType string, expr string) string {
	return fmt.Sprintf("%s::new(%s::new(Some::<%s>(%s)))", GetOuterWrapperType(), externalStubInnerWrapperType(), innerType, expr)
}

func writeExternalStubReturnValue(out *strings.Builder, rustType string, innerType string, expr string) {
	if rustType == innerType {
		out.WriteString(expr)
		return
	}
	out.WriteString(wrappedExternalStubExpr(innerType, expr))
}

func singleExternalReturnType(returnTypes []string) string {
	if len(returnTypes) != 1 {
		return ""
	}
	return returnTypes[0]
}

func wrappedExternalStubSomeExpr(innerType string, expr string) string {
	return wrappedExternalStubExpr(innerType, expr)
}

func wrappedExternalStubNoneExpr(innerType string) string {
	return fmt.Sprintf("%s::new(%s::new(None::<%s>))", GetOuterWrapperType(), externalStubInnerWrapperType(), innerType)
}

func externalStubInnerWrapperType() string {
	if usePackageExternalStubs() && NeedsConcurrentWrapper() {
		return "Mutex"
	}
	return GetInnerWrapperType()
}

// PERMANENT: not scaffold — io/fs.FileInfo is OS-tied; Rust std::fs::Metadata is the long-term implementation.
func writeFsFileInfoStub(out *strings.Builder, name string, methods map[string]externalTypeStubMethod) {
	stringType := wrappedExternalStubType("String")
	// Predeclared Copy scalar return slots stay bare to match the widened
	// signatures callers use.

	fmt.Fprintf(out, `#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct %s {
    pub name: String,
    pub is_dir: bool,
    pub size: i64,
}

impl std::fmt::Display for %s {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<%s>")
    }
}


impl %s {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn name(&self) -> %s {
        %s
    }
    pub fn size(&self) -> i64 {
        self.size
    }
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
`, name, name, name, name, stringType, wrappedExternalStubExpr("String", "self.name.clone()"))
	if method, ok := methods["mode"]; ok && len(method.ReturnTypes) == 1 {
		fmt.Fprintf(out, `    pub fn mode(&self) -> %s {
        if self.is_dir {
            %s
        } else {
            %s
        }
    }
`, method.ReturnTypes[0],
			wrappedExternalStubExpr("fs_FileMode", "fs_FileMode(1u32 << 31)"),
			wrappedExternalStubExpr("fs_FileMode", "fs_FileMode(0)"))
	}
	methodNames := make([]string, 0, len(methods))
	for methodName := range methods {
		if methodName == "name" || methodName == "size" || methodName == "is_dir" || methodName == "mode" {
			continue
		}
		methodNames = append(methodNames, methodName)
	}
	slices.Sort(methodNames)
	for _, methodName := range methodNames {
		writeExternalTypeStubMethod(out, name, methodName, methods[methodName])
	}
	out.WriteString("}\n")
}

// PERMANENT: not scaffold — io/fs.FileMode is OS-tied; Rust std::fs::Metadata is the long-term implementation.
func writeFsFileModeIsDirMethod(out *strings.Builder) {
	out.WriteString(`    pub fn is_dir(&self) -> bool {
        (self.0 & (1u32 << 31)) != 0
    }
`)
}

// PERMANENT: not scaffold — io/fs.DirEntry is OS-tied; Rust std::fs::DirEntry is the long-term implementation.
func writeFsDirEntryStub(out *strings.Builder, name string, methods map[string]externalTypeStubMethod) {
	stringType := wrappedExternalStubType("String")

	fmt.Fprintf(out, `#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct %s {
    pub name: String,
    pub is_dir: bool,
}

impl std::fmt::Display for %s {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<%s>")
    }
}


impl %s {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn name(&self) -> %s {
        %s
    }
	    pub fn is_dir(&self) -> bool {
	        self.is_dir
	    }
`, name, name, name, name, stringType, wrappedExternalStubExpr("String", "self.name.clone()"))
	if method, ok := methods["r#type"]; ok && len(method.ReturnTypes) == 1 {
		fmt.Fprintf(out, `    pub fn r#type(&self) -> %s {
        if self.is_dir {
            %s
        } else {
            %s
        }
    }
`, method.ReturnTypes[0],
			wrappedExternalStubExpr("fs_FileMode", "fs_FileMode(1u32 << 31)"),
			wrappedExternalStubExpr("fs_FileMode", "fs_FileMode(0)"))
	}
	methodNames := make([]string, 0, len(methods))
	for methodName := range methods {
		if methodName == "name" || methodName == "is_dir" || methodName == "r#type" {
			continue
		}
		methodNames = append(methodNames, methodName)
	}
	slices.Sort(methodNames)
	for _, methodName := range methodNames {
		writeExternalTypeStubMethod(out, name, methodName, methods[methodName])
	}
	out.WriteString("}\n")
}

// TEMPORARY: hand-written Rust shim for encoding/json package.
// Long-term fix: transpile encoding/json source.
func writeJsonPackageStub(out *strings.Builder, pkg *externalPackageStub) {
	out.WriteString("pub mod json {\n")
	out.WriteString("    use super::*;\n")
	funcNames := make([]string, 0, len(pkg.Functions))
	for funcName := range pkg.Functions {
		funcNames = append(funcNames, funcName)
	}
	slices.Sort(funcNames)
	for i, funcName := range funcNames {
		if i > 0 {
			out.WriteString("\n")
		}
		switch funcName {
		case "new_decoder":
			out.WriteString("    pub fn new_decoder<T0: GoJsonInputArg>(_arg0: T0) -> ")
			writeExternalStubReturnType(out, pkg.Functions[funcName].ReturnTypes)
			out.WriteString(" {\n")
			out.WriteString("        ")
			out.WriteString(wrappedExternalStubExpr("json_Decoder", "json_Decoder::__go_from_input(_arg0)"))
			out.WriteString("\n")
			out.WriteString("    }\n")
		case "unmarshal":
			errorInnerType := externalStubErrorInnerType()
			out.WriteString("    pub fn unmarshal<T0: GoJsonInputArg, T1: GoJsonDecodeTarget>(_arg0: T0, _arg1: T1) -> ")
			out.WriteString(wrappedExternalStubType(errorInnerType))
			out.WriteString(" {\n")
			out.WriteString("        let bytes = _arg0.into_go_json_bytes();\n")
			out.WriteString("        match serde_json::from_slice::<serde_json::Value>(&bytes) {\n")
			out.WriteString("            Ok(value) => match _arg1.assign_go_json(&value) {\n")
			out.WriteString("                Ok(()) => go_json_no_error(),\n")
			out.WriteString("                Err(err) => go_json_error(err),\n")
			out.WriteString("            },\n")
			out.WriteString("            Err(err) => go_json_error(err.to_string()),\n")
			out.WriteString("        }\n")
			out.WriteString("    }\n")
		default:
			writeExternalPackageStubFunction(out, "json", funcName, pkg.Functions[funcName])
		}
	}
	out.WriteString("}\n")
}

// PERMANENT: not scaffold — os.* is the syscall/OS layer; Rust std::env / std::process are the long-term implementation.
func writeOsPackageStub(out *strings.Builder, pkg *externalPackageStub, integerTypes map[string]string) {
	out.WriteString("pub mod os {\n")
	out.WriteString("    use super::*;\n")
	needsFilesystemHelpers := osPackageStubNeedsFilesystemHelpers(pkg)
	if needsFilesystemHelpers {
		out.WriteString("    use std::path::Path;\n\n")
		writeGoStringArgTrait(out)
		if _, ok := pkg.Functions["write_file"]; ok {
			writeGoBytesArgTrait(out)
		}
		writeOsErrorHelpers(out)
	}

	constNames := make([]string, 0, len(pkg.Constants))
	for constName := range pkg.Constants {
		constNames = append(constNames, constName)
	}
	slices.Sort(constNames)
	for _, constName := range constNames {
		out.WriteString("    pub const ")
		out.WriteString(constName)
		out.WriteString(": ")
		out.WriteString(pkg.Constants[constName])
		out.WriteString(" = ")
		writeExternalPackageStubConstValue(out, pkg, constName, integerTypes)
		out.WriteString(";\n")
	}
	if len(constNames) > 0 && (len(pkg.Variables) > 0 || len(pkg.Functions) > 0) {
		out.WriteString("\n")
	}

	varNames := make([]string, 0, len(pkg.Variables))
	for varName := range pkg.Variables {
		varNames = append(varNames, varName)
	}
	slices.Sort(varNames)
	for _, varName := range varNames {
		writeExternalPackageStubVariable(out, varName, pkg.Variables[varName])
		out.WriteString("\n")
	}

	funcNames := make([]string, 0, len(pkg.Functions))
	for funcName := range pkg.Functions {
		funcNames = append(funcNames, funcName)
	}
	slices.Sort(funcNames)
	for i, funcName := range funcNames {
		if i > 0 || len(varNames) > 0 {
			out.WriteString("\n")
		}
		if funcName == "exit" {
			writeOsExitFunction(out)
		} else if funcName == "getwd" {
			writeOsGetwdFunction(out, pkg.Functions[funcName])
		} else if funcName == "getenv" {
			writeOsGetenvFunction(out, pkg.Functions[funcName])
		} else if funcName == "is_path_separator" {
			writeOsIsPathSeparatorFunction(out, pkg.Functions[funcName])
		} else if funcName == "lstat" {
			writeOsLstatFunction(out, pkg.Functions[funcName])
		} else if funcName == "mkdir_all" {
			writeOsMkdirAllFunction(out, pkg.Functions[funcName])
		} else if funcName == "open" {
			writeOsOpenFunction(out, pkg.Functions[funcName])
		} else if funcName == "read_file" {
			writeOsReadFileFunction(out, pkg.Functions[funcName])
		} else if funcName == "read_dir" {
			writeOsReadDirFunction(out, pkg.Functions[funcName])
		} else if funcName == "stat" {
			writeOsStatFunction(out, pkg.Functions[funcName])
		} else if funcName == "write_file" {
			writeOsWriteFileFunction(out, pkg.Functions[funcName])
		} else {
			writeExternalPackageStubFunction(out, "os", funcName, pkg.Functions[funcName])
		}
	}
	out.WriteString("}\n")
}

func osPackageStubNeedsFilesystemHelpers(pkg *externalPackageStub) bool {
	if pkg == nil {
		return false
	}
	_, needsStat := pkg.Functions["stat"]
	_, needsMkdirAll := pkg.Functions["mkdir_all"]
	_, needsOpen := pkg.Functions["open"]
	_, needsReadDir := pkg.Functions["read_dir"]
	_, needsReadFile := pkg.Functions["read_file"]
	_, needsWriteFile := pkg.Functions["write_file"]
	_, needsGetwd := pkg.Functions["getwd"]
	_, needsLstat := pkg.Functions["lstat"]
	return needsStat || needsMkdirAll || needsOpen || needsReadDir || needsReadFile || needsWriteFile || needsGetwd || needsLstat
}

// PERMANENT: not scaffold — OS error types map to Rust std::io::Error, no transpilable Go source.
func writeOsErrorHelpers(out *strings.Builder) {
	errorType := wrappedExternalStubType("Box<dyn std::error::Error>")
	if NeedsConcurrentWrapper() {
		errorType = wrappedExternalStubType("Box<dyn std::error::Error + Send + Sync>")
	}
	fmt.Fprintf(out, `    type GoError = %s;

    fn no_error() -> GoError {
        %s::new(%s::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        %s::new(%s::new(Some(Box::new(err))))
    }

`, errorType, GetOuterWrapperType(), externalStubInnerWrapperType(), GetOuterWrapperType(), externalStubInnerWrapperType())
}

// PERMANENT: not scaffold — os.Exit maps to std::process::exit, runtime-tied.
func writeOsExitFunction(out *strings.Builder) {
	out.WriteString("    pub fn exit<T0: Into<i32>>(_arg0: T0) {\n")
	out.WriteString("        std::process::exit(_arg0.into());\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — os.Getenv maps to std::env::var in the host process environment.
func writeOsGetenvFunction(out *strings.Builder, fn externalPackageStubFunction) {
	stringType := wrappedExternalStubType("String")
	stringBorrow := externalStubBorrowExpr("v")

	out.WriteString("    pub fn getenv<T0: 'static>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let key = if let Some(v) = (&_arg0 as &dyn Any).downcast_ref::<String>() {\n")
	out.WriteString("            v.clone()\n")
	out.WriteString("        } else if let Some(v) = (&_arg0 as &dyn Any).downcast_ref::<")
	out.WriteString(stringType)
	out.WriteString(">() {\n")
	out.WriteString("            ")
	out.WriteString(stringBorrow)
	out.WriteString(".as_ref().cloned().unwrap_or_default()\n")
	out.WriteString("        } else {\n")
	out.WriteString("            panic!(\"os.Getenv bridge: expected string argument\")\n")
	out.WriteString("        };\n")
	out.WriteString("        ")
	out.WriteString(wrappedExternalStubExpr("String", "std::env::var(key).unwrap_or_default()"))
	out.WriteString("\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — os.Getwd maps to std::env::current_dir in the host process environment.
func writeOsGetwdFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn getwd() -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        match std::env::current_dir() {\n")
	out.WriteString("            Ok(path) => (")
	out.WriteString(wrappedExternalStubExpr("String", "path.to_string_lossy().into_owned()"))
	out.WriteString(", no_error()),\n")
	out.WriteString("            Err(err) => (")
	out.WriteString(wrappedExternalStubExpr("String", "String::new()"))
	out.WriteString(", io_error(err)),\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — os.IsPathSeparator follows host OS path separator rules.
func writeOsIsPathSeparatorFunction(out *strings.Builder, fn externalPackageStubFunction) {
	u8Wrapper := wrappedExternalStubType("u8")
	u8Borrow := externalStubBorrowExpr("v")
	i32Wrapper := wrappedExternalStubType("i32")
	i32Borrow := externalStubBorrowExpr("v")

	out.WriteString("    pub fn is_path_separator<T0: 'static>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let c = if let Some(v) = (&_arg0 as &dyn Any).downcast_ref::<u8>() {\n")
	out.WriteString("            *v\n")
	out.WriteString("        } else if let Some(v) = (&_arg0 as &dyn Any).downcast_ref::<i32>() {\n")
	out.WriteString("            *v as u8\n")
	out.WriteString("        } else if let Some(v) = (&_arg0 as &dyn Any).downcast_ref::<")
	out.WriteString(u8Wrapper)
	out.WriteString(">() {\n")
	out.WriteString("            ")
	out.WriteString(u8Borrow)
	out.WriteString(".as_ref().copied().unwrap_or_default()\n")
	out.WriteString("        } else if let Some(v) = (&_arg0 as &dyn Any).downcast_ref::<")
	out.WriteString(i32Wrapper)
	out.WriteString(">() {\n")
	out.WriteString("            ")
	out.WriteString(i32Borrow)
	out.WriteString(".as_ref().copied().unwrap_or_default() as u8\n")
	out.WriteString("        } else {\n")
	out.WriteString("            panic!(\"os.IsPathSeparator bridge: expected byte argument\")\n")
	out.WriteString("        };\n")
	out.WriteString("        #[cfg(windows)]\n")
	out.WriteString("        { c == b'/' || c == b'\\\\' }\n")
	out.WriteString("        #[cfg(not(windows))]\n")
	out.WriteString("        { c == b'/' }\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — os.MkdirAll maps to std::fs::create_dir_all, syscall-tied.
func writeOsMkdirAllFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn mkdir_all<T0: GoStringArg, T1>(_arg0: T0, _arg1: T1) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        match std::fs::create_dir_all(&path) {\n")
	out.WriteString("            Ok(()) => no_error(),\n")
	out.WriteString("            Err(err) => io_error(err),\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — os.Stat maps to std::fs::metadata, syscall-tied.
func writeOsStatFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn stat<T0: GoStringArg>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        match std::fs::metadata(&path) {\n")
	out.WriteString("            Ok(metadata) => {\n")
	out.WriteString("                let name = Path::new(&path).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_else(|| path.clone());\n")
	out.WriteString("                (")
	out.WriteString(wrappedExternalStubExpr("fs_FileInfo", "fs_FileInfo { name, is_dir: metadata.is_dir(), size: metadata.len() as i64 }"))
	out.WriteString(", no_error())\n")
	out.WriteString("            }\n")
	out.WriteString("            Err(err) => (")
	out.WriteString(wrappedExternalStubExpr("fs_FileInfo", "fs_FileInfo::default()"))
	out.WriteString(", io_error(err)),\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — os.Lstat maps to std::fs::symlink_metadata, syscall-tied.
func writeOsLstatFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn lstat<T0: GoStringArg>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        match std::fs::symlink_metadata(&path) {\n")
	out.WriteString("            Ok(metadata) => {\n")
	out.WriteString("                let name = Path::new(&path).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_else(|| path.clone());\n")
	out.WriteString("                (")
	out.WriteString(wrappedExternalStubExpr("fs_FileInfo", "fs_FileInfo { name, is_dir: metadata.is_dir(), size: metadata.len() as i64 }"))
	out.WriteString(", no_error())\n")
	out.WriteString("            }\n")
	out.WriteString("            Err(err) => (")
	out.WriteString(wrappedExternalStubExpr("fs_FileInfo", "fs_FileInfo::default()"))
	out.WriteString(", io_error(err)),\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — os.Open maps to std::fs::read for the read-oriented os.File stub.
func writeOsOpenFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn open<T0: GoStringArg>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        match std::fs::read(&path) {\n")
	out.WriteString("            Ok(data) => {\n")
	out.WriteString("                let file = os_File { __go_data: std::sync::Arc::new(std::sync::Mutex::new(data)), __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), __go_wait_for_close: false };\n")
	out.WriteString("                (")
	out.WriteString(wrappedExternalStubExpr("os_File", "file"))
	out.WriteString(", no_error())\n")
	out.WriteString("            }\n")
	out.WriteString("            Err(err) => (")
	out.WriteString(wrappedExternalStubNoneExpr("os_File"))
	out.WriteString(", io_error(err)),\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — os.ReadFile maps to std::fs::read, syscall-tied.
func writeOsReadFileFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn read_file<T0: GoStringArg>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        match std::fs::read(&path) {\n")
	out.WriteString("            Ok(data) => (")
	out.WriteString(wrappedExternalStubExpr("Vec<u8>", "data"))
	out.WriteString(", no_error()),\n")
	out.WriteString("            Err(err) => (")
	out.WriteString(wrappedExternalStubExpr("Vec<u8>", "Vec::new()"))
	out.WriteString(", io_error(err)),\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — os.WriteFile maps to std::fs::write, syscall-tied.
func writeOsWriteFileFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn write_file<T0: GoStringArg, T1: GoBytesArg, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        let data = _arg1.into_go_bytes();\n")
	out.WriteString("        match std::fs::write(&path, data) {\n")
	out.WriteString("            Ok(()) => no_error(),\n")
	out.WriteString("            Err(err) => io_error(err),\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — os.ReadDir maps to std::fs::read_dir, syscall-tied.
func writeOsReadDirFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn read_dir<T0: GoStringArg>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        let entries = match std::fs::read_dir(&path) {\n")
	out.WriteString("            Ok(entries) => entries,\n")
	out.WriteString("            Err(err) => return (")
	out.WriteString(wrappedExternalStubExpr("Vec<fs_DirEntry>", "Vec::new()"))
	out.WriteString(", io_error(err)),\n")
	out.WriteString("        };\n")
	out.WriteString("        let mut result = Vec::new();\n")
	out.WriteString("        for entry in entries {\n")
	out.WriteString("            match entry {\n")
	out.WriteString("                Ok(entry) => {\n")
	out.WriteString("                    let name = entry.file_name().to_string_lossy().into_owned();\n")
	out.WriteString("                    let is_dir = entry.file_type().map(|file_type| file_type.is_dir()).unwrap_or(false);\n")
	out.WriteString("                    result.push(fs_DirEntry { name, is_dir });\n")
	out.WriteString("                }\n")
	out.WriteString("                Err(err) => return (")
	out.WriteString(wrappedExternalStubExpr("Vec<fs_DirEntry>", "Vec::new()"))
	out.WriteString(", io_error(err)),\n")
	out.WriteString("            }\n")
	out.WriteString("        }\n")
	out.WriteString("        result.sort_by(|left, right| left.name.cmp(&right.name));\n")
	out.WriteString("        (")
	out.WriteString(wrappedExternalStubExpr("Vec<fs_DirEntry>", "result"))
	out.WriteString(", no_error())\n")
	out.WriteString("    }\n")
}

// MIXED: top-level path/filepath emitter. Pure path-manipulation functions are
// source-transpiled; Abs/EvalSymlinks are PERMANENT (OS-tied).
func writeFilepathPackageStub(out *strings.Builder, pkg *externalPackageStub, integerTypes map[string]string) {
	out.WriteString("pub mod filepath {\n")
	out.WriteString("    use super::*;\n")
	out.WriteString("    use std::path::PathBuf;\n\n")
	writeGoStringArgTrait(out)
	writeFilepathErrorHelpers(out)

	constNames := make([]string, 0, len(pkg.Constants))
	for constName := range pkg.Constants {
		constNames = append(constNames, constName)
	}
	slices.Sort(constNames)
	for _, constName := range constNames {
		if constName == "SEPARATOR" {
			out.WriteString("    pub const SEPARATOR: i32 = 47;\n")
		} else if constName == "LIST_SEPARATOR" {
			out.WriteString("    pub const LIST_SEPARATOR: i32 = 58;\n")
		} else {
			out.WriteString("    pub const ")
			out.WriteString(constName)
			out.WriteString(": ")
			out.WriteString(pkg.Constants[constName])
			out.WriteString(" = ")
			writeExternalPackageStubConstValue(out, pkg, constName, integerTypes)
			out.WriteString(";\n")
		}
	}
	if len(constNames) > 0 && len(pkg.Functions) > 0 {
		out.WriteString("\n")
	}

	funcNames := make([]string, 0, len(pkg.Functions))
	for funcName := range pkg.Functions {
		funcNames = append(funcNames, funcName)
	}
	slices.Sort(funcNames)
	wroteFunction := false
	for _, funcName := range funcNames {
		switch funcName {
		case "base", "clean", "dir", "is_abs", "join":
			continue
		}
		if wroteFunction {
			out.WriteString("\n")
		}
		wroteFunction = true
		if funcName == "abs" {
			writeFilepathAbsFunction(out, pkg.Functions[funcName])
		} else if funcName == "eval_symlinks" {
			writeFilepathEvalSymlinksFunction(out, pkg.Functions[funcName])
		} else {
			writeExternalPackageStubFunction(out, "filepath", funcName, pkg.Functions[funcName])
		}
	}
	out.WriteString("}\n")
}

// MACHINERY: cross-package helper trait for string-argument coercion.
func writeGoStringArgTrait(out *strings.Builder) {
	stringType := wrappedExternalStubType("String")
	borrow := ".borrow()"
	if NeedsConcurrentWrapper() {
		borrow = ".lock().unwrap()"
	}
	fmt.Fprintf(out, `    pub trait GoStringArg {
        fn into_go_string(self) -> String;
    }

    impl GoStringArg for String {
        fn into_go_string(self) -> String {
            self
        }
    }

    impl<'a> GoStringArg for &'a str {
        fn into_go_string(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoStringArg for &'a String {
        fn into_go_string(self) -> String {
            self.clone()
        }
    }

    impl GoStringArg for %s {
        fn into_go_string(self) -> String {
            self%s.as_ref().cloned().unwrap_or_default()
        }
    }

`, stringType, borrow)
}

// MACHINERY: cross-package helper trait for byte-argument coercion.
func writeGoBytesArgTrait(out *strings.Builder) {
	vecType := wrappedExternalStubType("Vec<u8>")
	stringType := wrappedExternalStubType("String")
	borrow := ".borrow()"
	if NeedsConcurrentWrapper() {
		borrow = ".lock().unwrap()"
	}
	fmt.Fprintf(out, `    pub trait GoBytesArg {
        fn into_go_bytes(self) -> Vec<u8>;
    }

    impl GoBytesArg for Vec<u8> {
        fn into_go_bytes(self) -> Vec<u8> {
            self
        }
    }

    impl<'a> GoBytesArg for &'a [u8] {
        fn into_go_bytes(self) -> Vec<u8> {
            self.to_vec()
        }
    }

    impl GoBytesArg for String {
        fn into_go_bytes(self) -> Vec<u8> {
            self.into_bytes()
        }
    }

    impl<'a> GoBytesArg for &'a str {
        fn into_go_bytes(self) -> Vec<u8> {
            self.as_bytes().to_vec()
        }
    }

    impl GoBytesArg for %s {
        fn into_go_bytes(self) -> Vec<u8> {
            self%s.as_ref().cloned().unwrap_or_default()
        }
    }

    impl GoBytesArg for %s {
        fn into_go_bytes(self) -> Vec<u8> {
            self%s.as_ref().map(|value| value.as_bytes().to_vec()).unwrap_or_default()
        }
    }

`, vecType, borrow, stringType, borrow)
}

// PERMANENT: not scaffold — filepath error helpers map to std::io::Error, OS-tied.
func writeFilepathErrorHelpers(out *strings.Builder) {
	errorType := wrappedExternalStubType("Box<dyn std::error::Error>")
	if NeedsConcurrentWrapper() {
		errorType = wrappedExternalStubType("Box<dyn std::error::Error + Send + Sync>")
	}
	fmt.Fprintf(out, `    type GoError = %s;

    fn no_error() -> GoError {
        %s::new(%s::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        %s::new(%s::new(Some(Box::new(err))))
    }

    fn normalize_path(path: PathBuf) -> String {
        path.components().collect::<PathBuf>().to_string_lossy().into_owned()
    }

`, errorType, GetOuterWrapperType(), externalStubInnerWrapperType(), GetOuterWrapperType(), externalStubInnerWrapperType())
}

// PERMANENT: not scaffold — filepath.Abs requires CWD resolution, OS-tied.
func writeFilepathAbsFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn abs<T0: GoStringArg>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let path = PathBuf::from(_arg0.into_go_string());\n")
	out.WriteString("        let absolute = if path.is_absolute() {\n")
	out.WriteString("            path\n")
	out.WriteString("        } else {\n")
	out.WriteString("            match std::env::current_dir() {\n")
	out.WriteString("                Ok(current) => current.join(path),\n")
	out.WriteString("                Err(err) => return (")
	out.WriteString(wrappedExternalStubExpr("String", "String::new()"))
	out.WriteString(", io_error(err)),\n")
	out.WriteString("            }\n")
	out.WriteString("        };\n")
	out.WriteString("        (")
	out.WriteString(wrappedExternalStubExpr("String", "normalize_path(absolute)"))
	out.WriteString(", no_error())\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — filepath.EvalSymlinks requires syscall, OS-tied.
func writeFilepathEvalSymlinksFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn eval_symlinks<T0: GoStringArg>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        match std::fs::canonicalize(&path) {\n")
	out.WriteString("            Ok(path) => (")
	out.WriteString(wrappedExternalStubExpr("String", "path.to_string_lossy().into_owned()"))
	out.WriteString(", no_error()),\n")
	out.WriteString("            Err(err) => (")
	out.WriteString(wrappedExternalStubExpr("String", "String::new()"))
	out.WriteString(", io_error(err)),\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

// MACHINERY: variable-declaration emitter for stub packages.
func writeExternalPackageStubVariable(out *strings.Builder, varName string, rustType string) {
	out.WriteString("    pub fn ")
	out.WriteString(varName)
	out.WriteString("() -> ")
	out.WriteString(rustType)
	out.WriteString(" {\n")
	out.WriteString("        ")
	writeExternalStubDefaultValue(out, rustType)
	out.WriteString("\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — os/exec.* is the process layer; Rust std::process is the long-term implementation.
func writeExecPackageStub(out *strings.Builder, pkg *externalPackageStub, integerTypes map[string]string) {
	out.WriteString("pub mod exec {\n")
	out.WriteString("    use super::*;\n\n")
	writeGoStringArgTrait(out)
	writeExecCommandArgsTrait(out)

	constNames := make([]string, 0, len(pkg.Constants))
	for constName := range pkg.Constants {
		constNames = append(constNames, constName)
	}
	slices.Sort(constNames)
	for _, constName := range constNames {
		out.WriteString("    pub const ")
		out.WriteString(constName)
		out.WriteString(": ")
		out.WriteString(pkg.Constants[constName])
		out.WriteString(" = ")
		writeExternalPackageStubConstValue(out, pkg, constName, integerTypes)
		out.WriteString(";\n")
	}
	if len(constNames) > 0 && (len(pkg.Functions) > 0 || len(pkg.Variables) > 0) {
		out.WriteString("\n")
	}

	varNames := make([]string, 0, len(pkg.Variables))
	for varName := range pkg.Variables {
		varNames = append(varNames, varName)
	}
	slices.Sort(varNames)
	for i, varName := range varNames {
		if i > 0 {
			out.WriteString("\n")
		}
		writeExternalPackageStubVariable(out, varName, pkg.Variables[varName])
	}
	if len(varNames) > 0 && len(pkg.Functions) > 0 {
		out.WriteString("\n")
	}

	funcNames := make([]string, 0, len(pkg.Functions))
	for funcName := range pkg.Functions {
		funcNames = append(funcNames, funcName)
	}
	slices.Sort(funcNames)
	for i, funcName := range funcNames {
		if i > 0 {
			out.WriteString("\n")
		}
		writeExternalPackageStubFunction(out, "exec", funcName, pkg.Functions[funcName])
	}
	out.WriteString("}\n")
}

// MACHINERY: generic stub function emitter dispatch.
//
// Per AGENTS.md "Strategy: Transpile stdlib, don't bridge it", the
// generic body emits a panic instead of returning defaults. Functions
// whose stubs need real behavior (exec.Command, os.Pipe, etc.)
// have custom emitters dispatched above. Anything routing through the
// generic body has no implementation — calling it at runtime is a bug
// to be fixed at the call site or by adding a custom emitter.
func writeExternalPackageStubFunction(out *strings.Builder, pkgName string, funcName string, fn externalPackageStubFunction) {
	if funcName == "command" && len(fn.ReturnTypes) == 1 {
		writeExecCommandStub(out, fn, false)
		return
	}
	if funcName == "command_context" && len(fn.ReturnTypes) == 1 {
		writeExecCommandStub(out, fn, true)
		return
	}
	if funcName == "look_path" && len(fn.ReturnTypes) == 2 {
		writeExecLookPathStub(out, fn)
		return
	}
	if funcName == "pipe" && len(fn.ReturnTypes) == 3 {
		writeOsPipeStub(out, fn)
		return
	}
	if funcName == "g_o_m_a_x_p_r_o_c_s" && len(fn.ReturnTypes) == 1 {
		writeRuntimeGOMAXPROCSStub(out, fn)
		return
	}
	if funcName == "g_o_r_o_o_t" && len(fn.ReturnTypes) == 1 {
		writeRuntimeGOROOTStub(out, fn)
		return
	}
	if funcName == "new_tuple" || funcName == "new_type_name" || funcName == "new_type_param" {
		return
	}
	if pkgName == "types" && funcName == "new_pointer" {
		return
	}
	if pkgName == "types" && (funcName == "new_package" || funcName == "new_checker") {
		return
	}
	if funcName == "new_term" {
		return
	}
	out.WriteString("    pub fn ")
	out.WriteString(funcName)
	if len(fn.GenericParamNames) > 0 {
		out.WriteString("<")
		for i, name := range fn.GenericParamNames {
			if i > 0 {
				out.WriteString(", ")
			}
			out.WriteString(name)
		}
		out.WriteString(">")
	} else if fn.ParamCount > 0 {
		out.WriteString("<")
		for i := 0; i < fn.ParamCount; i++ {
			if i > 0 {
				out.WriteString(", ")
			}
			out.WriteString("T")
			out.WriteString(strconv.Itoa(i))
		}
		out.WriteString(">")
	}
	out.WriteString("(")
	for i := 0; i < fn.ParamCount; i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString("_arg")
		out.WriteString(strconv.Itoa(i))
		out.WriteString(": ")
		if len(fn.GenericParamNames) > 0 && i < len(fn.ParamTypes) && fn.ParamTypes[i] != "" {
			out.WriteString(fn.ParamTypes[i])
		} else {
			out.WriteString("T")
			out.WriteString(strconv.Itoa(i))
		}
	}
	out.WriteString(")")
	if len(fn.ReturnTypes) > 0 {
		out.WriteString(" -> ")
		writeExternalStubReturnType(out, fn.ReturnTypes)
	}
	out.WriteString(" {\n")
	out.WriteString("        panic!(\"")
	out.WriteString(funcName)
	out.WriteString(" bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md\")\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — runtime.GOMAXPROCS is runtime-tied; Rust has no direct equivalent.
func writeRuntimeGOMAXPROCSStub(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn g_o_m_a_x_p_r_o_c_s")
	if fn.ParamCount > 0 {
		out.WriteString("<")
		for i := 0; i < fn.ParamCount; i++ {
			if i > 0 {
				out.WriteString(", ")
			}
			out.WriteString("T")
			out.WriteString(strconv.Itoa(i))
		}
		out.WriteString(">")
	}
	out.WriteString("(")
	for i := 0; i < fn.ParamCount; i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString("_arg")
		out.WriteString(strconv.Itoa(i))
		out.WriteString(": T")
		out.WriteString(strconv.Itoa(i))
	}
	out.WriteString(") -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n        ")
	writeExternalStubReturnValue(out, fn.ReturnTypes[0], "i32", "std::thread::available_parallelism().map(|n| n.get() as i32).unwrap_or(1).max(1)")
	out.WriteString("\n    }\n")
}

// PERMANENT: not scaffold — runtime.GOROOT is runtime-tied; use the host Go toolchain root.
func writeRuntimeGOROOTStub(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn g_o_r_o_o_t() -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n        ")
	writeExternalStubReturnValue(out, fn.ReturnTypes[0], "String", `{
            static GOROOT: std::sync::OnceLock<String> = std::sync::OnceLock::new();
            GOROOT.get_or_init(|| {
                if let Ok(value) = std::env::var("GOROOT") {
                    if !value.is_empty() {
                        return value;
                    }
                }
                std::process::Command::new("go")
                    .args(["env", "GOROOT"])
                    .output()
                    .ok()
                    .and_then(|output| if output.status.success() { String::from_utf8(output.stdout).ok() } else { None })
                    .map(|value| value.trim().to_string())
                    .filter(|value| !value.is_empty())
                    .unwrap_or_default()
            }).clone()
        }`)
	out.WriteString("\n    }\n")
}

// PERMANENT: not scaffold — os.Pipe maps to OS pipe syscalls, runtime-tied.
func writeOsPipeStub(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn pipe() -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString(`        let data = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let closed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let read = os_File { __go_data: data.clone(), __go_closed: closed.clone(), __go_wait_for_close: true };
        let write = os_File { __go_data: data, __go_closed: closed, __go_wait_for_close: true };
        (`)
	out.WriteString(wrappedExternalStubExpr("os_File", "read"))
	out.WriteString(", ")
	out.WriteString(wrappedExternalStubExpr("os_File", "write"))
	out.WriteString(", ")
	writeExternalStubDefaultValue(out, fn.ReturnTypes[2])
	out.WriteString(")\n    }\n")
}

// PERMANENT: not scaffold — exec.Command spawns OS processes, runtime-tied.
func writeExecCommandStub(out *strings.Builder, fn externalPackageStubFunction, hasContext bool) {
	if hasContext {
		out.WriteString("    pub fn command_context<T0, T1: GoStringArg, T2: GoExecCommandArgs>(_arg0: T0, _arg1: T1, _arg2: T2) -> ")
		writeExternalStubReturnType(out, fn.ReturnTypes)
		out.WriteString(" {\n")
		writeExecCommandStubBody(out, "_arg1", "_arg2")
		return
	}
	out.WriteString("    pub fn command<T0: GoStringArg, T1: GoExecCommandArgs>(_arg0: T0, _arg1: T1) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	writeExecCommandStubBody(out, "_arg0", "_arg1")
}

// PERMANENT: not scaffold — exec.Command body construction, runtime-tied.
func writeExecCommandStubBody(out *strings.Builder, nameArg string, argsArg string) {
	out.WriteString("        let mut args = vec![")
	out.WriteString(nameArg)
	out.WriteString(".into_go_string()];\n")
	out.WriteString("        args.extend(")
	out.WriteString(argsArg)
	out.WriteString(".into_exec_args());\n")
	out.WriteString("        ")
	out.WriteString(wrappedExternalStubExpr("exec_Cmd", "exec_Cmd { args: "+wrappedExternalStubExpr("Vec<String>", "args")+", ..Default::default() }"))
	out.WriteString("\n    }\n")
}

// PERMANENT: not scaffold — exec.Command variadic-args coercion trait, paired with PERMANENT exec stubs.
func writeExecCommandArgsTrait(out *strings.Builder) {
	sliceType := wrappedExternalStubType("Vec<String>")
	borrow := ".borrow()"
	if NeedsConcurrentWrapper() {
		borrow = ".lock().unwrap()"
	}
	fmt.Fprintf(out, `    pub trait GoExecCommandArgs {
        fn into_exec_args(self) -> Vec<String>;
    }

    impl GoExecCommandArgs for () {
        fn into_exec_args(self) -> Vec<String> {
            Vec::new()
        }
    }

    impl GoExecCommandArgs for %s {
        fn into_exec_args(self) -> Vec<String> {
            self%s.as_ref().cloned().unwrap_or_default()
        }
    }

    impl<T0: GoStringArg> GoExecCommandArgs for (T0,) {
        fn into_exec_args(self) -> Vec<String> {
            vec![self.0.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg> GoExecCommandArgs for (T0, T1) {
        fn into_exec_args(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg, T2: GoStringArg> GoExecCommandArgs for (T0, T1, T2) {
        fn into_exec_args(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string(), self.2.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg, T2: GoStringArg, T3: GoStringArg> GoExecCommandArgs for (T0, T1, T2, T3) {
        fn into_exec_args(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string(), self.2.into_go_string(), self.3.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg, T2: GoStringArg, T3: GoStringArg, T4: GoStringArg> GoExecCommandArgs for (T0, T1, T2, T3, T4) {
        fn into_exec_args(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string(), self.2.into_go_string(), self.3.into_go_string(), self.4.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg, T2: GoStringArg, T3: GoStringArg, T4: GoStringArg, T5: GoStringArg> GoExecCommandArgs for (T0, T1, T2, T3, T4, T5) {
        fn into_exec_args(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string(), self.2.into_go_string(), self.3.into_go_string(), self.4.into_go_string(), self.5.into_go_string()]
        }
    }

`, sliceType, borrow)
}

// PERMANENT: not scaffold — exec.LookPath walks $PATH, syscall/env-tied.
func writeExecLookPathStub(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn look_path")
	if fn.ParamCount > 0 {
		out.WriteString("<")
		for i := 0; i < fn.ParamCount; i++ {
			if i > 0 {
				out.WriteString(", ")
			}
			out.WriteString("T")
			out.WriteString(strconv.Itoa(i))
		}
		out.WriteString(">")
	}
	out.WriteString("(")
	for i := 0; i < fn.ParamCount; i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString("_arg")
		out.WriteString(strconv.Itoa(i))
		out.WriteString(": T")
		out.WriteString(strconv.Itoa(i))
	}
	out.WriteString(") -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n        (")
	writeExternalStubDefaultValue(out, fn.ReturnTypes[0])
	out.WriteString(", ")
	errorTrait := "dyn std::error::Error"
	errorType := "Box<dyn std::error::Error>"
	if NeedsConcurrentWrapper() {
		errorTrait = "dyn std::error::Error + Send + Sync"
		errorType = "Box<dyn std::error::Error + Send + Sync>"
	}
	out.WriteString(wrappedExternalStubExpr(errorType, fmt.Sprintf("Box::<%s>::from(\"executable file not found\")", errorTrait)))
	out.WriteString(")\n    }\n")
}

func externalStubFieldsCanDeriveDebug(fields map[string]string) bool {
	for _, fieldType := range fields {
		if strings.Contains(fieldType, "dyn Fn") {
			return false
		}
	}
	return true
}

func externalStubStructNeedsCustomDefault(name string) bool {
	return name == "types_Config"
}

// MACHINERY: Default impl emitter for stub structs.
func writeExternalStubStructDefault(out *strings.Builder, name string, fields map[string]string) {
	out.WriteString("impl Default for ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    fn default() -> Self {\n")
	out.WriteString("        Self {\n")
	fieldNames := make([]string, 0, len(fields))
	for fieldName := range fields {
		fieldNames = append(fieldNames, fieldName)
	}
	slices.Sort(fieldNames)
	for _, fieldName := range fieldNames {
		out.WriteString("            ")
		out.WriteString(fieldName)
		out.WriteString(": ")
		out.WriteString(externalStubStructFieldDefault(name, fieldName))
		out.WriteString(",\n")
	}
	out.WriteString("        }\n")
	out.WriteString("    }\n")
	out.WriteString("}\n\n")
}

func externalStubStructFieldDefault(typeName, fieldName string) string {
	if typeName == "types_Config" {
		switch fieldName {
		case "fake_import_c", "ignore_func_bodies":
			return "Arc::new(Mutex::new(Some(false)))"
		case "go_version":
			return "Arc::new(Mutex::new(Some(String::new())))"
		}
	}
	return "Default::default()"
}

// MACHINERY: return-type signature emitter.
func writeExternalStubReturnType(out *strings.Builder, returnTypes []string) {
	if len(returnTypes) == 1 {
		out.WriteString(returnTypes[0])
		return
	}
	out.WriteString("(")
	for i, returnType := range returnTypes {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(returnType)
	}
	out.WriteString(")")
}

// MACHINERY: default-value emitter for stub return types.
func writeExternalStubDefaultValue(out *strings.Builder, rustType string) {
	outerWrapper := GetOuterWrapperType()
	innerWrapper := externalStubInnerWrapperType()
	wrappedPrefix := outerWrapper + "<" + innerWrapper + "<Option<"
	if strings.HasPrefix(rustType, wrappedPrefix) && strings.HasSuffix(rustType, ">>>") {
		innerType := strings.TrimSuffix(strings.TrimPrefix(rustType, wrappedPrefix), ">>>")
		if strings.HasPrefix(innerType, "Box<dyn StdError") || strings.HasPrefix(innerType, "Box<dyn std::error::Error") || strings.HasPrefix(innerType, "Box<dyn Any") || strings.HasPrefix(innerType, "Box<dyn std::any::Any") {
			out.WriteString(outerWrapper)
			out.WriteString("::new(")
			out.WriteString(innerWrapper)
			out.WriteString("::new(None::<")
			out.WriteString(innerType)
			out.WriteString(">))")
			return
		}
		out.WriteString(outerWrapper)
		out.WriteString("::new(")
		out.WriteString(innerWrapper)
		out.WriteString("::new(Some::<")
		out.WriteString(innerType)
		out.WriteString(">(Default::default())))")
		return
	}
	out.WriteString("Default::default()")
}

func writeExternalPackageStubConstValue(out *strings.Builder, pkg *externalPackageStub, constName string, integerTypes map[string]string) {
	if writeExternalPackageStubConstLiteral(out, pkg, constName, integerTypes) {
		return
	}
	rustType := ""
	if pkg != nil && pkg.Constants != nil {
		rustType = pkg.Constants[constName]
	}
	writeExternalStubConstDefaultValue(out, rustType, integerTypes)
}

func writeExternalPackageStubConstLiteral(out *strings.Builder, pkg *externalPackageStub, constName string, integerTypes map[string]string) bool {
	rustType := ""
	if pkg != nil && pkg.Constants != nil {
		rustType = pkg.Constants[constName]
	}
	if constValue := externalPackageStubConstValue(pkg, constName); constValue != nil {
		return writeExternalStubConstLiteral(out, rustType, constValue, integerTypes)
	}
	return false
}

func externalPackageStubConstValue(pkg *externalPackageStub, constName string) constant.Value {
	if pkg == nil || pkg.ConstantValues == nil {
		return nil
	}
	return pkg.ConstantValues[constName]
}

func writeExternalStubConstLiteral(out *strings.Builder, rustType string, constValue constant.Value, integerTypes map[string]string) bool {
	if constValue == nil {
		return false
	}
	if integerTypes[rustType] != "" {
		intValue := constant.ToInt(constValue)
		if intValue.Kind() != constant.Int {
			return false
		}
		out.WriteString(rustType)
		out.WriteString("(")
		out.WriteString(intValue.String())
		out.WriteString(")")
		return true
	}
	switch rustType {
	case "String", "&'static str":
		if constValue.Kind() != constant.String {
			return false
		}
		out.WriteString(strconv.Quote(constant.StringVal(constValue)))
		return true
	case "bool":
		if constValue.Kind() != constant.Bool {
			return false
		}
		out.WriteString(strconv.FormatBool(constant.BoolVal(constValue)))
		return true
	case "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "usize":
		intValue := constant.ToInt(constValue)
		if intValue.Kind() != constant.Int {
			return false
		}
		out.WriteString(intValue.String())
		return true
	case "f32":
		floatValue, ok := constant.Float32Val(constant.ToFloat(constValue))
		if !ok {
			return false
		}
		out.WriteString(rustFloatConstLiteral(float64(floatValue), 32))
		return true
	case "f64":
		floatValue, ok := constant.Float64Val(constant.ToFloat(constValue))
		if !ok {
			return false
		}
		out.WriteString(rustFloatConstLiteral(floatValue, 64))
		return true
	default:
		return false
	}
}

func rustFloatConstLiteral(value float64, bitSize int) string {
	lit := strconv.FormatFloat(value, 'g', -1, bitSize)
	if !strings.ContainsAny(lit, ".eE") {
		lit += ".0"
	}
	return lit
}

// MACHINERY: const default-value emitter for stub variable declarations.
func writeExternalStubConstDefaultValue(out *strings.Builder, rustType string, integerTypes map[string]string) {
	if integerTypes[rustType] != "" {
		out.WriteString(rustType)
		out.WriteString("(0)")
		return
	}
	switch rustType {
	case "&'static str":
		out.WriteString("\"\"")
	case "String":
		out.WriteString("String::new()")
	case "bool":
		out.WriteString("false")
	case "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "usize":
		out.WriteString("0")
	case "f32", "f64":
		out.WriteString("0.0")
	default:
		out.WriteString(rustType)
	}
}
