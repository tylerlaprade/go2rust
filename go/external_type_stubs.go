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
		RegisterExternalIntegerTypeStub("token_Pos", rustIntType())
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
		return rustIntType(), true
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
		fn.ReturnTypes = []string{wrappedExternalStubType("String"), wrappedExternalStubType(rustErrorPayloadType())}
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
		RegisterExternalIntegerTypeStub("token_Token", rustIntType())
	}
	pkg := ensureExternalPackageStub(pkgName)
	pkg.Constants[constName] = goTypesConstTypeToRust(constType)
	// An untyped integer constant too big for i32, math.MaxInt64, gets the
	// smallest Rust integer that holds it. Uses cast it to their type.
	if basic, ok := constType.Underlying().(*types.Basic); ok && basic.Kind() == types.UntypedInt && constValue != nil {
		if v := constant.ToInt(constValue); v.Kind() == constant.Int {
			if _, fits := constant.Int64Val(v); !fits {
				pkg.Constants[constName] = "u64"
			} else if n, _ := constant.Int64Val(v); n > 2147483647 || n < -2147483648 {
				pkg.Constants[constName] = "i64"
			}
		}
	}
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
	return generateExternalStubsForPackage(pkg.ExternalTypeStubs, pkg.ExternalTypeStubInterfaces, pkg.ExternalTypeStubIntegerTypes, pkg.ExternalTypeStubTupleTypes, pkg.ExternalTypeStubFields, pkg.ExternalTypeStubMethods, pkg.ExternalTypeStubConversions, pkg.ExternalPackageStubs, pkg.JsonDecodeSupportNeeded)
}

func WriteSharedStdlibStubCrate(workDir string, states []*PackageState, packageMapping map[string]string) error {
	outputDir := filepath.Join(workDir, "vendor", sharedStdlibStubCrateName)
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		return fmt.Errorf("failed to create shared stdlib stub crate: %v", err)
	}

	mergedState := MergeExternalStubPackageStates(states...)
	parts := []string{}
	if goSliceModeEnabled() {
		parts = append(parts, goSliceVecMacro)
	}
	if helperCode := mergedState.Helpers.GenerateSharedStdlibHelperModule(); helperCode != "" {
		parts = append(parts, helperCode)
	}
	if stubCode := goSliceStubText(GeneratePackageExternalStubs(mergedState)); stubCode != "" {
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
gosyn = "0.2.9"
`, sharedStdlibStubCrateName, sharedStdlibStubCrateName)
	if strings.Contains(stubCode, "serde_json::") || strings.Contains(stubCode, "pub use serde_json") {
		cargoToml += "serde_json = { version = \"1\", features = [\"raw_value\"] }\n"
	}
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
		merged.JsonDecodeSupportNeeded = merged.JsonDecodeSupportNeeded || state.JsonDecodeSupportNeeded
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
	dst.needsFormatAnyNested = dst.needsFormatAnyNested || src.needsFormatAnyNested
	dst.needsAnyEq = dst.needsAnyEq || src.needsAnyEq
	dst.needsAnyClone = dst.needsAnyClone || src.needsAnyClone
	dst.needsGoValueClone = dst.needsGoValueClone || src.needsGoValueClone
	dst.needsGoTypeParamFieldClone = dst.needsGoTypeParamFieldClone || src.needsGoTypeParamFieldClone
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
	if len(src.anyFormatArms) > 0 {
		if dst.anyFormatArms == nil {
			dst.anyFormatArms = make(map[string]anyFormatArm)
		}
		for rustType, arm := range src.anyFormatArms {
			dst.anyFormatArms[rustType] = arm
		}
	}
	if len(src.anyTypeNames) > 0 {
		if dst.anyTypeNames == nil {
			dst.anyTypeNames = make(map[string]string)
		}
		for rustType, goName := range src.anyTypeNames {
			if existing, seen := dst.anyTypeNames[rustType]; !seen || goName < existing {
				dst.anyTypeNames[rustType] = goName
			}
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
	dst.needsGoErrorChain = dst.needsGoErrorChain || src.needsGoErrorChain
	dst.needsGoSprintf = dst.needsGoSprintf || src.needsGoSprintf
	dst.needsGoSprintfSpread = dst.needsGoSprintfSpread || src.needsGoSprintfSpread
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
		if _, ok := pkg.Functions["valid"]; ok {
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
	errorInnerType := externalStubErrorInnerType()
	errorFromMessage := rustErrorFromTextFor("StdError", NeedsConcurrentWrapper(), "message")
	if NeedsConcurrentWrapper() {
		borrow = ".lock().unwrap()"
		borrowMut = ".lock().unwrap()"
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
    %s::new(%s::new(Some(%s)))
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

impl<V> GoJsonDecode for std::collections::BTreeMap<String, V>
where
    V: GoJsonDecode,
{
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = std::collections::BTreeMap::new();
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

`, errorType, outerWrapper, innerWrapper, errorType, outerWrapper, innerWrapper, errorFromMessage, outerWrapper, innerWrapper, borrow, outerWrapper, innerWrapper, outerWrapper, innerWrapper, outerWrapper, innerWrapper, outerWrapper, innerWrapper, borrowMut, borrowMut)
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
	return generateExternalStubsForPackage(stubs, interfaceTypes, integerTypes, tupleTypes, fieldsByType, methodsByType, conversions, packageStubs, false)
}

func generateExternalStubsForPackage(stubs map[string]bool, interfaceTypes map[string]bool, integerTypes map[string]string, tupleTypes map[string]string, fieldsByType map[string]map[string]string, methodsByType map[string]map[string]externalTypeStubMethod, conversions map[string]map[string]bool, packageStubs map[string]*externalPackageStub, jsonDecodeSupportNeeded bool) string {
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
	needsJsonSupport := jsonDecodeSupportNeeded || externalStubsNeedJsonSupport(stubs, packageStubs)
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
	µç[h‘éì¶»§q«^uÑåÁ•Ì¹9•İQåÁ•A…É…´‰É¥‘”ˆ°(%ôì($%¥˜ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°Õ¹İ…¹Ñ•¤ì($$%Ğ¹…Ñ…±˜ ‰¼½ÑåÁ•Ì½¹ÍÑÉÕÑ½È•áÑ•É¹…°‰É¥‘”µÕÍĞ‰”É•Ñ¥É•ì™½Õ¹€•Äéq¸•Ìˆ°Õ¹İ…¹Ñ•°½Ğ¤($%ô(%ô(%™½È|°İ…¹Ğ€èôÉ…¹”muÍÑÉ¥¹ì($$‰ÁÕˆÍÑÉÕĞÑåÁ•Í}QÕÁ±”ˆ°($$‰ÁÕˆÍÑÉÕĞÑåÁ•Í}QåÁ•9…µ”ˆ°($$‰ÁÕˆÍÑÉÕĞÑåÁ•Í}QåÁ•A…É…´ˆ°(%ôì($%¥˜€…ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°İ…¹Ğ¤ì($$%Ğ¹…Ñ…±˜ ‰É•Ñ¥É¥¹œ½¹ÍÑÉÕÑ½ÉÌµÕÍĞ¹½ĞÉ•µ½Ù”•áÑ•É¹…°ÑåÁ”¥‘•¹Ñ¥Ñä€•Äéq¸•Ìˆ°İ…¹Ğ°½Ğ¤($%ô(%ô)ô()™Õ¹ŒQ•ÍÑQåÁ•Í	…Í¥	•¡…Ù¥½ÉM¡¥µ%ÍI•Ñ¥É•¡Ğ€©Ñ•ÍÑ¥¹œ¹P¤ì(%½Ğ€èô•¹•É…Ñ•áÑ•É¹…±MÑÕ‰Ì ($%µ…ÁmÍÑÉ¥¹u‰½½±ì($$$‰ÑåÁ•Í}	…Í¥Œˆè€€€€ÑÉÕ”°($$$‰ÑåÁ•Í}	…Í¥%¹™¼ˆèÑÉÕ”°($$$‰ÑåÁ•Í}	…Í¥-¥¹ˆèÑÉÕ”°($%ô°($%¹¥°°($%µ…ÁmÍÑÉ¥¹uÍÑÉ¥¹ì($$$‰ÑåÁ•Í}	…Í¥%¹™¼ˆè€‰¤ÌÈˆ°($$$‰ÑåÁ•Í}	…Í¥-¥¹ˆè€‰¤ÌÈˆ°($%ô°($%¹¥°°¹¥°°($%µ…ÁmÍÑÉ¥¹uµ…ÁmÍÑÉ¥¹u•áÑ•É¹…±QåÁ•MÑÕ‰5•Ñ¡½‘ì($$$‰ÑåÁ•Í}	…Í¥Œˆèì($$$$‰¥¹™¼ˆèì($$$$%I•ÑÕÉ¹QåÁ•ÌèmuÍÑÉ¥¹ì‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñÑåÁ•Í}	…Í¥%¹™¼øøø‰ô°($$$%ô°($$$$‰­¥¹ˆèì($$$$%I•ÑÕÉ¹QåÁ•ÌèmuÍÑÉ¥¹ì‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñÑåÁ•Í}	…Í¥-¥¹øøø‰ô°($$$%ô°($$$$‰¹…µ”ˆèì($$$$%I•ÑÕÉ¹QåÁ•ÌèmuÍÑÉ¥¹ì‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñMÑÉ¥¹œøøø‰ô°($$$%ô°($$%ô°($%ô°($%¹¥°°¹¥°°($¤(%™½È|°Õ¹İ…¹Ñ•€èôÉ…¹”muÍÑÉ¥¹ì($$‰ÁÕˆÍÑÉÕĞÑåÁ•Í}	…Í¥Œìˆ°($$‰}}½}­¥¹ˆ°($$‰}}½}¥¹™¼ˆ°($$‰}}½}¹…µ”ˆ°($$‰Í•±˜¹}}½}­¥¹ˆ°($$‰Í•±˜¹}}½}¥¹™¼ˆ°($$‰Í•±˜¹}}½}¹…µ”ˆ°(%ôì($%¥˜ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°Õ¹İ…¹Ñ•¤ì($$%Ğ¹…Ñ…±˜ ‰¼½ÑåÁ•Ì¹	…Í¥Œ•áÑ•É¹…°‰•¡…Ù¥½ÈÍ¡¥´µÕÍĞ‰”É•Ñ¥É•ì™½Õ¹€•Äéq¸•Ìˆ°Õ¹İ…¹Ñ•°½Ğ¤($%ô(%ô(%™½È|°İ…¹Ğ€èôÉ…¹”muÍÑÉ¥¹ì($$‰ÁÕˆÍÑÉÕĞÑåÁ•Í}	…Í¥Œìˆ°($$‰ÑåÁ•Í}	…Í¥Œ¹­¥¹‰É¥‘”è•¹•É¥ŒÍÑÕˆµ•Ñ¡½‰½‘ä¡…Ì¹¼¥µÁ±•µ•¹Ñ…Ñ¥½¸ˆ°($$‰ÑåÁ•Í}	…Í¥Œ¹¥¹™¼‰É¥‘”è•¹•É¥ŒÍÑÕˆµ•Ñ¡½‰½‘ä¡…Ì¹¼¥µÁ±•µ•¹Ñ…Ñ¥½¸ˆ°($$‰ÑåÁ•Í}	…Í¥Œ¹¹…µ”‰É¥‘”è•¹•É¥ŒÍÑÕˆµ•Ñ¡½‰½‘ä¡…Ì¹¼¥µÁ±•µ•¹Ñ…Ñ¥½¸ˆ°(%ôì($%¥˜€…ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°İ…¹Ğ¤ì($$%Ğ¹…Ñ…±˜ ‰É•Ñ¥É¥¹œ	…Í¥Œ‰•¡…Ù¥½ÈµÕÍĞ­••À±½ÕÁ±…•¡½±‘•È€•Äéq¸•Ìˆ°İ…¹Ğ°½Ğ¤($%ô(%ô)ô()™Õ¹ŒQ•ÍÑQ½­•¹Q½­•¹	•¡…Ù¥½ÉM¡¥µ%ÍI•Ñ¥É•¡Ğ€©Ñ•ÍÑ¥¹œ¹P¤ì(%½Ğ€èô•¹•É…Ñ•áÑ•É¹…±MÑÕ‰Ì ($%µ…ÁmÍÑÉ¥¹u‰½½±ì‰Ñ½­•¹}Q½­•¸ˆèÑÉÕ•ô°($%¹¥°°($%µ…ÁmÍÑÉ¥¹uÍÑÉ¥¹ì‰Ñ½­•¹}Q½­•¸ˆè€‰¤ÌÈ‰ô°($%¹¥°°¹¥°°($%µ…ÁmÍÑÉ¥¹uµ…ÁmÍÑÉ¥¹u•áÑ•É¹…±QåÁ•MÑÕ‰5•Ñ¡½‘ì($$$‰Ñ½­•¹}Q½­•¸ˆèì($$$$‰ÍÑÉ¥¹œˆèì($$$$%I•ÑÕÉ¹QåÁ•ÌèmuÍÑÉ¥¹ì‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñMÑÉ¥¹œøøø‰ô°($$$%ô°($$%ô°($%ô°($%¹¥°°¹¥°°($¤(%™½È|°Õ¹İ…¹Ñ•€èôÉ…¹”muÍÑÉ¥¹ì($$‰Ñ½­•¹}ÍÑÉ¥¹}Ù…±Õ”ˆ°($%€‰™Õ¹Œ‰€°($%€‰%110‰€°(%ôì($%¥˜ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°Õ¹İ…¹Ñ•¤ì($$%Ğ¹…Ñ…±˜ ‰¼½Ñ½­•¸¹Q½­•¸•áÑ•É¹…°‰•¡…Ù¥½ÈÍ¡¥´µÕÍĞ‰”É•Ñ¥É•ì™½Õ¹€•Äéq¸•Ìˆ°Õ¹İ…¹Ñ•°½Ğ¤($%ô(%ô(%™½È|°İ…¹Ğ€èôÉ…¹”muÍÑÉ¥¹ì($$‰ÁÕˆÍÑÉÕĞÑ½­•¹}Q½­•¸¡ÁÕˆ¤ÌÈ¤ìˆ°($%İÉ¥Ñ”„¡˜°€ˆñÑ½­•¹}Q½­•¸øˆ¥€°($$‰Ñ½­•¹}Q½­•¸¹ÍÑÉ¥¹œ‰É¥‘”è•¹•É¥ŒÍÑÕˆµ•Ñ¡½‰½‘ä¡…Ì¹¼¥µÁ±•µ•¹Ñ…Ñ¥½¸ˆ°(%ôì($%¥˜€…ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°İ…¹Ğ¤ì($$%Ğ¹…Ñ…±˜ ‰É•Ñ¥É¥¹œQ½­•¸‰•¡…Ù¥½ÈµÕÍĞ­••À±½ÕÁ±…•¡½±‘•È€•Äéq¸•Ìˆ°İ…¹Ğ°½Ğ¤($%ô(%ô)ô()™Õ¹ŒQ•ÍÑÍÑ9•İ%‘•¹Ñ	•¡…Ù¥½ÉM¡¥µ%ÍI•Ñ¥É•¡Ğ€©Ñ•ÍÑ¥¹œ¹P¤ì(%½Ğ€èô•¹•É…Ñ•áÑ•É¹…±MÑÕ‰Ì ($%µ…ÁmÍÑÉ¥¹u‰½½±ì‰…ÍÑ}%‘•¹ĞˆèÑÉÕ•ô°($%¹¥°°¹¥°°¹¥°°($%µ…ÁmÍÑÉ¥¹uµ…ÁmÍÑÉ¥¹uÍÑÉ¥¹ì($$$‰…ÍÑ}%‘•¹Ğˆèì($$$$‰¹…µ”ˆè€‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñMÑÉ¥¹œøøøˆ°($$%ô°($%ô°($%¹¥°°¹¥°°($%µ…ÁmÍÑÉ¥¹t©•áÑ•É¹…±A…­…•MÑÕ‰ì($$$‰…ÍĞˆèì($$$%Õ¹Ñ¥½¹Ìèµ…ÁmÍÑÉ¥¹u•áÑ•É¹…±A…­…•MÑÕ‰Õ¹Ñ¥½¹ì($$$$$‰¹•İ}¥‘•¹Ğˆèì($$$$$%A…É…µ½Õ¹Ğè€€Ä°($$$$$%I•ÑÕÉ¹QåÁ•ÌèmuÍÑÉ¥¹ì‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñ…ÍÑ}%‘•¹Ğøøø‰ô°($$$$%ô°($$$%ô°($$%ô°($%ô°($¤(%™½È|°Õ¹İ…¹Ñ•€èôÉ…¹”muÍÑÉ¥¹ì($$‰½MÑÉ¥¹Éœø¡}…ÉœÀˆ°($$‰…ÍÑ}%‘•¹Ğì¹…µ”èˆ°($$‰}…ÉœÀ¹¥¹Ñ½}½}ÍÑÉ¥¹œ ¤ˆ°(%ôì($%¥˜ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°Õ¹İ…¹Ñ•¤ì($$%Ğ¹…Ñ…±˜ ‰…ÍĞ¹9•İ%‘•¹Ğ•áÑ•É¹…°‰•¡…Ù¥½ÈÍ¡¥´µÕÍĞ‰”É•Ñ¥É•ì™½Õ¹€•Äéq¸•Ìˆ°Õ¹İ…¹Ñ•°½Ğ¤($%ô(%ô(%™½È|°İ…¹Ğ€èôÉ…¹”muÍÑÉ¥¹ì($$‰ÁÕˆÍÑÉÕĞ…ÍÑ}%‘•¹Ğˆ°($$‰ÁÕˆ™¸¹•İ}¥‘•¹ĞñPÀø¡}…ÉœÀèPÀ¤€´øÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñ…ÍÑ}%‘•¹Ğøøøˆ°($$‰¹•İ}¥‘•¹Ğ‰É¥‘”è•¹•É¥ŒÍÑÕˆ™Õ¹Ñ¥½¸‰½‘ä¡…Ì¹¼¥µÁ±•µ•¹Ñ…Ñ¥½¸ˆ°(%ôì($%¥˜€…ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°İ…¹Ğ¤ì($$%Ğ¹…Ñ…±˜ ‰É•Ñ¥É¥¹œ…ÍĞ¹9•İ%‘•¹Ğ‰•¡…Ù¥½ÈµÕÍĞ­••À±½ÕÁ±…•¡½±‘•È€•Äéq¸•Ìˆ°İ…¹Ğ°½Ğ¤($%ô(%ô)ô()™Õ¹ŒQ•ÍÑÍÑ%¹ÍÁ•Ñ	•¡…Ù¥½ÉM¡¥µ%ÍI•Ñ¥É•¡Ğ€©Ñ•ÍÑ¥¹œ¹P¤ì(%½Ğ€èô•¹•É…Ñ•áÑ•É¹…±MÑÕ‰Ì ($%µ…ÁmÍÑÉ¥¹u‰½½±ì‰…ÍÑ}9½‘”ˆèÑÉÕ•ô°($%µ…ÁmÍÑÉ¥¹u‰½½±ì‰…ÍÑ}9½‘”ˆèÑÉÕ•ô°($%¹¥°°¹¥°°¹¥°°¹¥°°¹¥°°($%µ…ÁmÍÑÉ¥¹t©•áÑ•É¹…±A…­…•MÑÕ‰ì($$$‰…ÍĞˆèì($$$%Õ¹Ñ¥½¹Ìèµ…ÁmÍÑÉ¥¹u•áÑ•É¹…±A…­…•MÑÕ‰Õ¹Ñ¥½¹ì($$$$$‰¥¹ÍÁ•Ğˆèì($$$$$%A…É…µ½Õ¹Ğè€È°($$$$%ô°($$$%ô°($$%ô°($%ô°($¤(%™½È|°Õ¹İ…¹Ñ•€èôÉ…¹”muÍÑÉ¥¹ì($$‰%¹ÍÁ•ÑI½½Ğˆ°($$‰Ù¥Í¥Ñ}¹½‘”ˆ°($$‰…±±}¥¹ÍÁ•Ñ}…±±‰…¬ˆ°(%ôì($%¥˜ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°Õ¹İ…¹Ñ•¤ì($$%Ğ¹…Ñ…±˜ ‰…ÍĞ¹%¹ÍÁ•Ğ•áÑ•É¹…°‰•¡…Ù¥½ÈÍ¡¥´µÕÍĞ‰”É•Ñ¥É•ì™½Õ¹€•Äéq¸•Ìˆ°Õ¹İ…¹Ñ•°½Ğ¤($%ô(%ô(%™½È|°İ…¹Ğ€èôÉ…¹”muÍÑÉ¥¹ì($$‰ÁÕˆ™¸¥¹ÍÁ•ĞñPÀ°PÄø¡}…ÉœÀèPÀ°}…ÉœÄèPÄ¤ˆ°($$‰¥¹ÍÁ•Ğ‰É¥‘”è•¹•É¥ŒÍÑÕˆ™Õ¹Ñ¥½¸‰½‘ä¡…Ì¹¼¥µÁ±•µ•¹Ñ…Ñ¥½¸ˆ°(%ôì($%¥˜€…ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°İ…¹Ğ¤ì($$%Ğ¹…Ñ…±˜ ‰É•Ñ¥É¥¹œ…ÍĞ¹%¹ÍÁ•Ğ‰•¡…Ù¥½ÈµÕÍĞ­••À±½ÕÁ±…•¡½±‘•È€•Äéq¸•Ìˆ°İ…¹Ğ°½Ğ¤($%ô(%ô)ô()™Õ¹ŒQ•ÍÑA…ÉÍ•ÉA…­…•	•¡…Ù¥½ÉM¡¥µÍÉ•I•Ñ¥É•¡Ğ€©Ñ•ÍÑ¥¹œ¹P¤ì(%½Ğ€èô•¹•É…Ñ•áÑ•É¹…±MÑÕ‰Ì ($%µ…ÁmÍÑÉ¥¹u‰½½±ì‰…ÍÑ}¥±”ˆèÑÉÕ•ô°($%¹¥°°($%µ…ÁmÍÑÉ¥¹uÍÑÉ¥¹ì‰Á…ÉÍ•É}5½‘”ˆè€‰¤ÌÈ‰ô°($%¹¥°°¹¥°°¹¥°°¹¥°°($%µ…ÁmÍÑÉ¥¹t©•áÑ•É¹…±A…­…•MÑÕ‰ì($$$‰Á…ÉÍ•Èˆèì($$$%½¹ÍÑ…¹ÑÌèµ…ÁmÍÑÉ¥¹uÍÑÉ¥¹ì($$$$$‰M}-}%}A}=	)Q}IM=1UQ%=8ˆè€‰Á…ÉÍ•É}5½‘”ˆ°($$$%ô°($$$%Õ¹Ñ¥½¹Ìèµ…ÁmÍÑÉ¥¹u•áÑ•É¹…±A…­…•MÑÕ‰Õ¹Ñ¥½¹ì($$$$$‰Á…ÉÍ•}™¥±”ˆèì($$$$$%A…É…µ½Õ¹Ğè€Ğ°($$$$$%I•ÑÕÉ¹QåÁ•ÌèmuÍÑÉ¥¹ì($$$$$$$‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñ…ÍÑ}¥±”øøøˆ°($$$$$$$‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñ	½àñ‘å¸MÑ‘ÉÉ½È€¬M•¹€¬Må¹Œøøøøˆ°($$$$$%ô°($$$$%ô°($$$%ô°($$%ô°($%ô°($¤(%™½È|°Õ¹İ…¹Ñ•€èôÉ…¹”muÍÑÉ¥¹ì($$‰½A…ÉÍ•É¥±•¹…µ•Éœˆ°($$‰½A…ÉÍ•ÉM½ÕÉ•Éœˆ°($$‰½}Á…ÉÍ•É|ˆ°($$‰½Íå¸èèˆ°($$‰…ÍÑ}µÁÑåMÑµĞˆ°(%ôì($%¥˜ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°Õ¹İ…¹Ñ•¤ì($$%Ğ¹…Ñ…±˜ ‰Á…ÉÍ•È•áÑ•É¹…°‰•¡…Ù¥½ÈÍ¡¥µÌµÕÍĞ‰”É•Ñ¥É•ì™½Õ¹€•Äéq¸•Ìˆ°Õ¹İ…¹Ñ•°½Ğ¤($%ô(%ô(%™½È|°İ…¹Ğ€èôÉ…¹”muÍÑÉ¥¹ì($$‰ÁÕˆ½¹ÍĞM}-}%}A}=	)Q}IM=1UQ%=8èÁ…ÉÍ•É}5½‘”ˆ°($$‰ÁÕˆ™¸Á…ÉÍ•}™¥±”ñPÀ°PÄ°PÈ°PÌø¡}…ÉœÀèPÀ°}…ÉœÄèPÄ°}…ÉœÈèPÈ°}…ÉœÌèPÌ¤€´ø€¡ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñ…ÍÑ}¥±”øøø°ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñ	½àñ‘å¸MÑ‘ÉÉ½È€¬M•¹€¬Må¹Œøøøø¤ˆ°($$‰Á…ÉÍ•}™¥±”‰É¥‘”è•¹•É¥ŒÍÑÕˆ™Õ¹Ñ¥½¸‰½‘ä¡…Ì¹¼¥µÁ±•µ•¹Ñ…Ñ¥½¸ˆ°(%ôì($%¥˜€…ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°İ…¹Ğ¤ì($$%Ğ¹…Ñ…±˜ ‰É•Ñ¥É¥¹œÁ…ÉÍ•È‰•¡…Ù¥½ÈµÕÍĞ­••À•¹•É¥ŒÁ±…•¡½±‘•È€•Äéq¸•Ìˆ°İ…¹Ğ°½Ğ¤($%ô(%ô)ô()™Õ¹ŒQ•ÍÑQåÁ•Í9•İA½¥¹Ñ•É	É¥‘•%ÍI•Ñ¥É•¡Ğ€©Ñ•ÍÑ¥¹œ¹P¤ì(%½Ğ€èô•¹•É…Ñ•áÑ•É¹…±MÑÕ‰Ì ($%µ…ÁmÍÑÉ¥¹u‰½½±ì‰ÑåÁ•Í}A½¥¹Ñ•ÈˆèÑÉÕ•ô°($%¹¥°°¹¥°°¹¥°°¹¥°°($%¹¥°°¹¥°°($%µ…ÁmÍÑÉ¥¹t©•áÑ•É¹…±A…­…•MÑÕ‰ì($$$‰ÑåÁ•Ìˆèì($$$%Õ¹Ñ¥½¹Ìèµ…ÁmÍÑÉ¥¹u•áÑ•É¹…±A…­…•MÑÕ‰Õ¹Ñ¥½¹ì($$$$$‰¹•İ}Á½¥¹Ñ•Èˆèì($$$$$%A…É…µ½Õ¹Ğè€€Ä°($$$$$%I•ÑÕÉ¹QåÁ•ÌèmuÍÑÉ¥¹ì‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñÑåÁ•Í}A½¥¹Ñ•Èøøø‰ô°($$$$%ô°($$$%ô°($$%ô°($%ô°($¤(%¥˜ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°€‰ÁÕˆ™¸¹•İ}Á½¥¹Ñ•Èˆ¤ì($%Ğ¹…Ñ…±˜ ‰¼½ÑåÁ•Ì¹9•İA½¥¹Ñ•È•áÑ•É¹…°Á…­…”µ™Õ¹Ñ¥½¸‰É¥‘”µÕÍĞ‰”É•Ñ¥É•éq¸•Ìˆ°½Ğ¤(%ô(%¥˜€…ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°€‰ÁÕˆÍÑÉÕĞÑåÁ•Í}A½¥¹Ñ•Èˆ¤ì($%Ğ¹…Ñ…±˜ ‰É•Ñ¥É¥¹œ9•İA½¥¹Ñ•ÈµÕÍĞ¹½ĞÉ•µ½Ù”•áÑ•É¹…°Á½¥¹Ñ•ÈÑåÁ”¥‘•¹Ñ¥Ñäéq¸•Ìˆ°½Ğ¤(%ô)ô()™Õ¹ŒQ•ÍÑQåÁ•Í9•İA…­…•¡•­•É	É¥‘•ÍÉ•I•Ñ¥É•¡Ğ€©Ñ•ÍÑ¥¹œ¹P¤ì(%½Ğ€èô•¹•É…Ñ•áÑ•É¹…±MÑÕ‰Ì ($%µ…ÁmÍÑÉ¥¹u‰½½±ì($$$‰ÑåÁ•Í}¡•­•ÈˆèÑÉÕ”°($$$‰ÑåÁ•Í}%¹™¼ˆè€€€ÑÉÕ”°($$$‰ÑåÁ•Í}A…­…”ˆèÑÉÕ”°($%ô°($%¹¥°°¹¥°°¹¥°°¹¥°°($%¹¥°°¹¥°°($%µ…ÁmÍÑÉ¥¹t©•áÑ•É¹…±A…­…•MÑÕ‰ì($$$‰ÑåÁ•Ìˆèì($$$%Õ¹Ñ¥½¹Ìèµ…ÁmÍÑÉ¥¹u•áÑ•É¹…±A…­…•MÑÕ‰Õ¹Ñ¥½¹ì($$$$$‰¹•İ}Á…­…”ˆèì($$$$$%A…É…µ½Õ¹Ğè€€È°($$$$$%I•ÑÕÉ¹QåÁ•ÌèmuÍÑÉ¥¹ì‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñÑåÁ•Í}A…­…”øøø‰ô°($$$$%ô°($$$$$‰¹•İ}¡•­•Èˆèì($$$$$%A…É…µ½Õ¹Ğè€€Ğ°($$$$$%I•ÑÕÉ¹QåÁ•ÌèmuÍÑÉ¥¹ì‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñÑåÁ•Í}¡•­•Èøøø‰ô°($$$$%ô°($$$%ô°($$%ô°($%ô°($¤(%™½È|°Õ¹İ…¹Ñ•€èôÉ…¹”muÍÑÉ¥¹ì($$‰ÁÕˆ™¸¹•İ}Á…­…”ˆ°($$‰ÁÕˆ™¸¹•İ}¡•­•Èˆ°($$‰¹•İ}Á…­…”‰É¥‘”ˆ°($$‰¹•İ}¡•­•È‰É¥‘”ˆ°(%ôì($%¥˜ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°Õ¹İ…¹Ñ•¤ì($$%Ğ¹…Ñ…±˜ ‰¼½ÑåÁ•Ì9•İA…­…”½9•İ¡•­•È•áÑ•É¹…°Á…­…”µ™Õ¹Ñ¥½¸‰É¥‘•ÌµÕÍĞ‰”É•Ñ¥É•ì™½Õ¹€•Äéq¸•Ìˆ°Õ¹İ…¹Ñ•°½Ğ¤($%ô(%ô(%™½È|°İ…¹Ğ€èôÉ…¹”muÍÑÉ¥¹ì($$‰ÁÕˆÍÑÉÕĞÑåÁ•Í}¡•­•Èˆ°($$‰ÁÕˆÍÑÉÕĞÑåÁ•Í}%¹™¼ˆ°($$‰ÁÕˆÍÑÉÕĞÑåÁ•Í}A…­…”ˆ°(%ôì($%¥˜€…ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°İ…¹Ğ¤ì($$%Ğ¹…Ñ…±˜ ‰É•Ñ¥É¥¹œ9•İA…­…”½9•İ¡•­•ÈµÕÍĞ¹½ĞÉ•µ½Ù”•áÑ•É¹…°ÑåÁ”¥‘•¹Ñ¥Ñä€•Äéq¸•Ìˆ°İ…¹Ğ°½Ğ¤($%ô(%ô)ô()™Õ¹ŒQ•ÍÑQåÁ•Í¡•­•É¥±•ÍMÑÕ‰%ÍI•Ñ¥É•¡Ğ€©Ñ•ÍÑ¥¹œ¹P¤ì(%½Ğ€èô•¹•É…Ñ•áÑ•É¹…±MÑÕ‰Ì ($%µ…ÁmÍÑÉ¥¹u‰½½±ì‰ÑåÁ•Í}¡•­•ÈˆèÑÉÕ•ô°($%¹¥°°¹¥°°¹¥°°¹¥°°($%µ…ÁmÍÑÉ¥¹uµ…ÁmÍÑÉ¥¹u•áÑ•É¹…±QåÁ•MÑÕ‰5•Ñ¡½‘ì($$$‰ÑåÁ•Í}¡•­•Èˆèì($$$$‰™¥±•Ìˆèì($$$$%A…É…µ½Õ¹Ğè€€Ä°($$$$%I•ÑÕÉ¹QåÁ•ÌèmuÍÑÉ¥¹ì‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñ	½àñ‘å¸MÑ‘ÉÉ½È€¬M•¹€¬Må¹Œøøøø‰ô°($$$%ô°($$%ô°($%ô°($%¹¥°°¹¥°°($¤(%™½È|°Õ¹İ…¹Ñ•€èôÉ…¹”muÍÑÉ¥¹ì‰ÁÕˆ™¸™¥±•Ìˆ°€‰}}½}ÑåÁ•Í}¡•­•É}™¥±•Ì‰ôì($%¥˜ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°Õ¹İ…¹Ñ•¤ì($$%Ğ¹…Ñ…±˜ ‰ÑåÁ•Ì¹¡•­•È¥±•ÌÍ¡½Õ±ÕÍ”Í½ÕÉ”µÑÉ…¹ÍÁ¥±•¼½ÑåÁ•Ì°¹½Ğ…¸•áÑ•É¹…°ÍÑÕˆµ•Ñ¡½ì™½Õ¹€•Äéq¸•Ìˆ°Õ¹İ…¹Ñ•°½Ğ¤($%ô(%ô)ô()™Õ¹ŒQ•ÍÑQåÁ•ÍQåÁ•MÑÉ¥¹U¹‘•É±å¥¹	É¥‘•5•Ñ¡½‘ÍÉ•I•Ñ¥É•¡Ğ€©Ñ•ÍÑ¥¹œ¹P¤ì(%½Ğ€èô•¹•É…Ñ•áÑ•É¹…±MÑÕ‰Ì ($%µ…ÁmÍÑÉ¥¹u‰½½±ì‰ÑåÁ•Í}QåÁ”ˆèÑÉÕ•ô°($%µ…ÁmÍÑÉ¥¹u‰½½±ì‰ÑåÁ•Í}QåÁ”ˆèÑÉÕ•ô°($%¹¥°°¹¥°°¹¥°°($%µ…ÁmÍÑÉ¥¹uµ…ÁmÍÑÉ¥¹u•áÑ•É¹…±QåÁ•MÑÕ‰5•Ñ¡½‘ì($$$‰ÑåÁ•Í}QåÁ”ˆèì($$$$‰ÍÑÉ¥¹œˆèì($$$$%I•ÑÕÉ¹QåÁ•ÌèmuÍÑÉ¥¹ì‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñMÑÉ¥¹œøøø‰ô°($$$%ô°($$$$‰Õ¹‘•É±å¥¹œˆèì($$$$%I•ÑÕÉ¹QåÁ•ÌèmuÍÑÉ¥¹ì‰ÉŒñ5ÕÑ•àñ=ÁÑ¥½¸ñÑåÁ•Í}QåÁ”øøø‰ô°($$$%ô°($$%ô°($%ô°($%¹¥°°¹¥°°($¤(%™½È|°Õ¹İ…¹Ñ•€èôÉ…¹”muÍÑÉ¥¹ì($$‰ÁÕˆ™¸ÍÑÉ¥¹œˆ°($$‰ÁÕˆ™¸Õ¹‘•É±å¥¹œˆ°($$‰ÑåÁ•Ì¹QåÁ”¹MÑÉ¥¹œ ¤‰É¥‘”ˆ°($$‰ÑåÁ•Ì¹QåÁ”¹U¹‘•É±å¥¹œ ¤‰É¥‘”ˆ°($$‰‘½İ¹…ÍÑ}É•˜èèñÑåÁ•Í}	…Í¥Œøˆ°(%ôì($%¥˜ÍÑÉ¥¹Ì¹½¹Ñ…¥¹Ì¡½Ğ°Õ¹İ…¹Ñ•¤ì($$%Ğ¹…Ñ…±˜ ‰ÑåÁ•Ì¹QåÁ”MÑÉ¥¹œ½U¹‘•É±å¥¹œ•áÑ•É¹…°‰É¥‘”µÕÍĞ‰”É•Ñ¥É•ì™½Õ¹€•Äéq¸•Ìˆ°Õ¹İ…¹Ñ•°½Ğ¤($%ô(%ô)ô((¼¼Q•ÍÑ	É¥‘••‰ÑI•¥ÍÑÉå½Ù•ÉÍ±±M¡¥µÌ¥ÌÑ¡”ÑÉ¥Áİ¥É”Ñ¡…Ğ•¹™½É•Ì(¼¼9QL¹µƒŠH€‰MÑÉ…Ñ•äèQÉ…¹ÍÁ¥±”ÍÑ‘±¥ˆ°‘½¸Ğ‰É¥‘”¥Ğˆ¸Ù•Éä(¼¼€¼¼Q5A=IIdé€½µµ•¹Ğ¥¸•áÑ•É¹…±}ÑåÁ•}ÍÑÕ‰Ì¹¼µÕÍĞ¡…Ù”„µ…Ñ¡¥¹œ(¼¼É½Ü¥¸‘½Ì½‰É¥‘•}‘•‰Ğ¹µ¸‘‘¥¹œ„Í¡¥´İ¥Ñ¡½ÕĞ„É½Ü°½ÈÉ•µ½Ù¥¹œ„(¼¼É½Üİ¥Ñ¡½ÕĞÉ•µ½Ù¥¹œÑ¡”Í¡¥´°™…¥±ÌÑ¡¥ÌÑ•ÍĞ¸(¼¼(¼¼Q¼µ…­”Ñ¡¥ÌÑ•ÍĞÁ…ÍÌİ¡•¸å½Ô±•¥Ñ¥µ…Ñ•±ä¹••Ñ¼…‘„Í¡¥´°•‘¥Ğ(¼¼‘½Ì½‰É¥‘•}‘•‰Ğ¹µ€©™¥ÉÍĞ¨…¹…‘„É½ÜÑ¡…Ğ¹…µ•ÌÑ¡”¼Íåµ‰½°°Ñ¡”(¼¼ÑÉ…¹ÍÁ¥±•È…À°…¹„™¥áÑÕÉ”¸Q¡”Í¡¥´½‘”½•Ì¥¸…™Ñ•È¸M•”9QL¹µ(¼¼™½ÈÑ¡”™Õ±°¡•­±¥ÍĞ¸)™Õ¹ŒQ•ÍÑ	É¥‘••‰ÑI•¥ÍÑÉå½Ù•ÉÍ±±M¡¥µÌ¡Ğ€©Ñ•ÍÑ¥¹œ¹P¤ì(%ÍÑÕ‰ÍA…Ñ €èô€‰•áÑ•É¹…±}ÑåÁ•}ÍÑÕ‰Ì¹¼ˆ(%É•¥ÍÑÉåA…Ñ €èô™¥±•Á…Ñ ¹)½¥¸ ˆ¸¸ˆ°€‰‘½Ìˆ°€‰‰É¥‘•}‘•‰Ğ¹µˆ¤((%ÍÑÕ‰Í	åÑ•Ì°•ÉÈ€èô½Ì¹I•…‘¥±”¡ÍÑÕ‰ÍA…Ñ ¤(%¥˜•ÉÈ€„ô¹¥°ì($%Ğ¹…Ñ…±˜ ‰É•…€•Ìè€•Øˆ°ÍÑÕ‰ÍA…Ñ °•ÉÈ¤(%ô(%É•¥ÍÑÉå	åÑ•Ì°•ÉÈ€èô½Ì¹I•…‘¥±”¡É•¥ÍÑÉåA…Ñ ¤(%¥˜•ÉÈ€„ô¹¥°ì($%Ğ¹…Ñ…±˜ ‰É•…€•Ìè€•Øˆ°É•¥ÍÑÉåA…Ñ °•ÉÈ¤(%ô((%Í¡¥µ1¥¹•Ì€èômu¥¹Ñíô(%™½È¤°±¥¹”€èôÉ…¹”ÍÑÉ¥¹Ì¹MÁ±¥Ğ¡ÍÑÉ¥¹œ¡ÍÑÕ‰Í	åÑ•Ì¤°€‰q¸ˆ¤ì($%¥˜ÍÑÉ¥¹Ì¹!…ÍAÉ•™¥à¡ÍÑÉ¥¹Ì¹QÉ¥µMÁ…”¡±¥¹”¤°€ˆ¼¼Q5A=IIdèˆ¤ì($$%Í¡¥µ1¥¹•Ì€ô…ÁÁ•¹¡Í¡¥µ1¥¹•Ì°¤¬Ä¤($%ô(%ô((%É•¥ÍÑÉä€èôÍÑÉ¥¹œ¡É•¥ÍÑÉå	åÑ•Ì¤(%Í¡¥µÍ!•…‘•È€èô€ˆŒŒM¡¥µÌˆ(%¥‘à€èôÍÑÉ¥¹Ì¹%¹‘•à¡É•¥ÍÑÉä°Í¡¥µÍ!•…‘•È¤(%¥˜¥‘à€ğ€Àì($%Ğ¹…Ñ…±˜ ˆ•Ì¥Ìµ¥ÍÍ¥¹œÑ¡”€•ÄÍ•Ñ¥½¸Ñ¡…Ğ¡½±‘ÌÍ¡¥´É½İÌˆ°É•¥ÍÑÉåA…Ñ °Í¡¥µÍ!•…‘•È¤(%ô(%É•¥ÍÑÉåI½İ½Õ¹Ğ€èô€À(%™½È|°±¥¹”€èôÉ…¹”ÍÑÉ¥¹Ì¹MÁ±¥Ğ¡É•¥ÍÑÉåm¥‘à­±•¸¡Í¡¥µÍ!•…‘•È¤ét°€‰q¸ˆ¤ì($%¥˜ÍÑÉ¥¹Ì¹!…ÍAÉ•™¥à¡±¥¹”°€ˆŒŒŒ€ˆ¤ì($$%É•¥ÍÑÉåI½İ½Õ¹Ğ¬¬($%ô(%ô((%¥˜±•¸¡Í¡¥µ1¥¹•Ì¤€„ôÉ•¥ÍÑÉåI½İ½Õ¹Ğì($%Ğ¹…Ñ…±˜¡‰É¥‘”‘•‰Ğ‘É¥™Ğ‘•Ñ•Ñ•¸((•Ì¡…Ì€•€¬‰€¼¼Q5A=IIdé€ˆ­€Í¡¥´½µµ•¹ÑÌ€¡±¥¹•Ìè€•Ø¤¸(•Ì¡…Ì€•±•Ù•°´ÌÉ½İÌÕ¹‘•È€•Ä¸()Ù•ÉäÍ¡¥´µÕÍĞ¡…Ù”„É•¥ÍÑÉäÉ½Ü¸M•”9QL¹µƒŠH€‰MÑÉ…Ñ•äèQÉ…¹ÍÁ¥±”)ÍÑ‘±¥ˆ°‘½¸Ğ‰É¥‘”¥Ğˆ¸Q¼É•Í½±Ù”è((´%˜å½Ô…‘‘•„Í¡¥´è…‘„É½ÜÑ¼€•Ì™¥ÉÍĞ°Ñ¡•¸É”µÉÕ¸Ñ¡¥ÌÑ•ÍĞ¸(´%˜å½Ô‘•±•Ñ•„Í¡¥´è…±Í¼‘•±•Ñ”¥ÑÌÉ½Ü¸(´%˜å½Ôµ½Ù•„Í¡¥´èÕÁ‘…Ñ”¥ÑÌÉ½ÜÌ1½…Ñ¥½¸™¥•±¸)€°($$%ÍÑÕ‰ÍA…Ñ °±•¸¡Í¡¥µ1¥¹•Ì¤°Í¡¥µ1¥¹•Ì°($$%É•¥ÍÑÉåA…Ñ °É•¥ÍÑÉåI½İ½Õ¹Ğ°Í¡¥µÍ!•…‘•È°($$%É•¥ÍÑÉåA…Ñ ¤(%ô)ô(