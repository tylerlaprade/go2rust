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
	"syscall"
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
	// RWMutexAccessor marks __go_rw_mutex, which gives a clone of the
	// embedded sync.RWMutex, the same lock, with no guard held.
	RWMutexAccessor bool
	// ThroughRWMutexAccessor marks a forwarder of a sync.RWMutex method
	// that reaches the mutex through __go_rw_mutex, so it waits for the
	// lock holding no guard of the struct the mutex is embedded in.
	ThroughRWMutexAccessor bool
	// Unimplemented, when set, is the message of the unimplemented! body a
	// sync.RWMutex forwarder or accessor gets when go/types could not say
	// which embedded field the method is promoted through.
	Unimplemented string
}

type externalPackageStub struct {
	Functions      map[string]externalPackageStubFunction
	Constants      map[string]string
	ConstantValues map[string]constant.Value
	Variables      map[string]string
}

type externalPackageStubFunction struct {
	ImportPath        string
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
	if bridge := stubIoFieldBridgeType(fieldType); bridge != "" {
		fieldTypeRust = wrappedExternalStubType(bridge)
	}
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
	// sync.RWMutex methods are forwarded only through the embedded field Go
	// selects them through: the shallowest one, whatever the field order.
	// Directly embedded ones are written first, then the rest in field order.
	embeddedFieldIndex := make(map[*ast.Field]int)
	fieldIndex := 0
	for _, field := range structDef.ASTType.Fields.List {
		if len(field.Names) > 0 {
			fieldIndex += len(field.Names)
			continue
		}
		embeddedFieldIndex[field] = fieldIndex
		fieldIndex++
		selected, known := promotedThroughField(structDef.ASTType, embeddedFieldIndex[field])
		if methods, ok := embeddedRWMutexPromotedMethods(field, selected, existingRustNames); ok {
			promoted = append(promoted, rwMutexForwardersForSelection(methods, known)...)
		}
	}
	for _, field := range structDef.ASTType.Fields.List {
		if len(field.Names) > 0 {
			continue
		}
		if isSyncRWMutexField(field) {
			continue
		}
		selected, known := promotedThroughField(structDef.ASTType, embeddedFieldIndex[field])
		promoted = append(promoted, rwMutexForwardersForSelection(embeddedStructRWMutexPromotedMethods(field, selected, existingRustNames), known)...)
		named, ok := externalEmbeddedNamed(field.Type)
		local := false
		if !ok {
			named, ok = localEmbeddedNamedWithInterfaceMethods(field.Type)
			if !ok {
				continue
			}
			local = true
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
			if !ok || (!fn.Exported() && !local) {
				continue
			}
			methodName := fn.Name()
			rustMethodName := ToSnakeCase(methodName)
			if local {
				rustMethodName = rustMethodNameForTypesFunc(fn)
			}
			if existingRustNames[rustMethodName] {
				continue
			}
			sig, ok := fn.Type().(*types.Signature)
			if !ok {
				continue
			}
			if local && !signatureIsInterfaceMethod(sig) {
				// A method declared on a struct of the package has a
				// FuncDecl, and collectPromotedMethods forwards it.
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

// localEmbeddedNamedWithInterfaceMethods reports an embedded field whose
// type is declared in the package being translated and is an interface, or
// is a struct that has methods of an interface it embeds itself. Those
// methods have no FuncDecls for collectPromotedMethods to forward. Without a
// forwarder, a promoted call on the outer struct, and the outer struct's impl
// of the interface, resolved to the trait method itself and recursed until
// the stack overflowed.
func localEmbeddedNamedWithInterfaceMethods(expr ast.Expr) (*types.Named, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || typeInfo.pkg == nil || expr == nil {
		return nil, false
	}
	if star, ok := expr.(*ast.StarExpr); ok {
		expr = star.X
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return nil, false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() != typeInfo.pkg || named.TypeParams().Len() > 0 || named.TypeArgs().Len() > 0 {
		return nil, false
	}
	switch named.Underlying().(type) {
	case *types.Interface, *types.Struct:
		return named, true
	}
	return nil, false
}

// signatureIsInterfaceMethod reports whether sig is the signature of a method
// of an interface, rather than of a method declared on a type.
func signatureIsInterfaceMethod(sig *types.Signature) bool {
	if sig == nil || sig.Recv() == nil {
		return false
	}
	_, ok := sig.Recv().Type().Underlying().(*types.Interface)
	return ok
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
		RegisterExternalPackageStubFunction(pkgName, obj.Pkg().Path(), ToSnakeCase(obj.Name()), sig)
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

func RegisterExternalPackageStubFunction(pkgName string, importPath string, funcName string, sig *types.Signature) {
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
		ImportPath: importPath,
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
		// time translated from source has its own Time and Duration.
		if isSourceMappedPackagePath(pkgPath) {
			return false
		}
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
	dst.needsAnyHandleCopy = dst.needsAnyHandleCopy || src.needsAnyHandleCopy
	dst.needsGoValueClone = dst.needsGoValueClone || src.needsGoValueClone
	dst.needsGoTypeParamFieldClone = dst.needsGoTypeParamFieldClone || src.needsGoTypeParamFieldClone
	dst.needsGoComparable = dst.needsGoComparable || src.needsGoComparable
	dst.needsGoAnyRuntimeOps = dst.needsGoAnyRuntimeOps || src.needsGoAnyRuntimeOps
	dst.needsAnyPointerFormatter = dst.needsAnyPointerFormatter || src.needsAnyPointerFormatter
	dst.needsGoAnyTypeMetadata = dst.needsGoAnyTypeMetadata || src.needsGoAnyTypeMetadata
	dst.needsEmbeddedOwnerRegistry = dst.needsEmbeddedOwnerRegistry || src.needsEmbeddedOwnerRegistry
	dst.needsReceiverHandles = dst.needsReceiverHandles || src.needsReceiverHandles
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
	dst.needsGoTimeRuntime = dst.needsGoTimeRuntime || src.needsGoTimeRuntime
	dst.needsGoChannelIdentity = dst.needsGoChannelIdentity || src.needsGoChannelIdentity
	dst.needsGoChannel = dst.needsGoChannel || src.needsGoChannel
	dst.needsGoSelectSend = dst.needsGoSelectSend || src.needsGoSelectSend
	dst.needsWaitGroup = dst.needsWaitGroup || src.needsWaitGroup
	dst.needsGoMutex = dst.needsGoMutex || src.needsGoMutex
	dst.needsGoRWMutex = dst.needsGoRWMutex || src.needsGoRWMutex
	dst.needsGoOnce = dst.needsGoOnce || src.needsGoOnce
	dst.needsGoAtomicPointer = dst.needsGoAtomicPointer || src.needsGoAtomicPointer
	dst.needsGoTypeName = dst.needsGoTypeName || src.needsGoTypeName
	dst.needsGoFloatString = dst.needsGoFloatString || src.needsGoFloatString
	dst.needsGoComplexDisplay = dst.needsGoComplexDisplay || src.needsGoComplexDisplay
	dst.needsGoErrorChain = dst.needsGoErrorChain || src.needsGoErrorChain
	dst.needsGoSprintf = dst.needsGoSprintf || src.needsGoSprintf
	dst.needsGoFmtScan = dst.needsGoFmtScan || src.needsGoFmtScan
	dst.needsGoSprintfSpread = dst.needsGoSprintfSpread || src.needsGoSprintfSpread
	dst.needsBase64 = dst.needsBase64 || src.needsBase64
	dst.needsSha256 = dst.needsSha256 || src.needsSha256
	dst.needsHexFormat = dst.needsHexFormat || src.needsHexFormat
	dst.needsStrconvFormat = dst.needsStrconvFormat || src.needsStrconvFormat
	dst.needsUrl = dst.needsUrl || src.needsUrl
	dst.needsJsonEscape = dst.needsJsonEscape || src.needsJsonEscape
	dst.needsGoNilAny = dst.needsGoNilAny || src.needsGoNilAny
	dst.needsGoSliceAppendCopying = dst.needsGoSliceAppendCopying || src.needsGoSliceAppendCopying
	dst.needsGoFuncValue = dst.needsGoFuncValue || src.needsGoFuncValue
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
	if osStubReturnsPathError(packageStubs["os"]) {
		stubs["fs_PathError"] = true
	}
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
	if writer, reader := externalStubsNeedIoFieldBridges(fieldsByType); writer || reader {
		writeStubIoFieldBridges(&out, writer, reader)
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
		if name == "atomic_Int64" {
			writeAtomicIntegerTypeStub(&out, name, "AtomicI64", "i64")
			continue
		}
		if name == "atomic_Uint32" {
			writeAtomicIntegerTypeStub(&out, name, "AtomicU32", "u32")
			continue
		}
		if name == "atomic_Bool" {
			writeAtomicBoolStub(&out)
			continue
		}
		if name == "sync_Map" {
			writeSyncMapStub(&out)
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
		if name == "fs_PathError" {
			writeFsPathErrorStub(&out, methodsByType[name])
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
			} else if name == "fs_FileMode" && methodName == "perm" {
				writeFsFileModePermMethod(&out, method)
			} else if name == "sync_Pool" && writeSyncPoolMethod(&out, methodName, method, fieldsByType[name]) {
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
	return rustErrorPayloadTypeFor("StdError", NeedsConcurrentWrapper())
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
	intType := rustIntType()
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
        panic!("io_Writer.__go_write_bytes bridge: unsupported concrete receiver; transpile io source instead - see AGENTS.md and docs/bridge_debt.md#io-writer-trait-bridging")
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (%s, %s) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<%s>() {
            %s.as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i64;
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
        Some(std::cmp::Ord::cmp(self, other))
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
// osFileStubCoreMethods are os_File's writes, reads and Close. The
// upper-case names are replaced with the program's types.
const osFileStubCoreMethods = `    pub fn __go_write_bytes(&self, data: &[u8]) {
        let _ = self.__go_write_bytes_result(data);
    }

    /// Go's text for an I/O error: the C library's text with a lowercase
    /// first letter, without Rust's " (os error N)".
    fn __go_io_text(err: &std::io::Error) -> String {
        let mut text = err.to_string();
        if let Some(code) = err.raw_os_error() {
            let suffix = format!(" (os error {})", code);
            if text.ends_with(&suffix) {
                text.truncate(text.len() - suffix.len());
            }
            let mut chars = text.chars();
            if let Some(first) = chars.next() {
                text = first.to_lowercase().collect::<String>() + chars.as_str();
            }
        }
        text
    }

    fn __go_op_error(&self, op: &str, err: std::io::Error) -> std::io::Error {
        std::io::Error::new(err.kind(), format!("{} {}: {}", op, self.__go_name, Self::__go_io_text(&err)))
    }

    fn __go_closed_error(&self, op: &str) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, format!("{} {}: file already closed", op, self.__go_name))
    }

    /// Writes data to the disk file os.Create or os.OpenFile opened, or
    /// else to the File's bytes, and reports how many bytes it wrote. A
    /// closed File takes no writes, as in Go. The disk lock is held from
    /// the closed check to the write, so Close cannot come in between.
    pub fn __go_write_counted(&self, data: &[u8]) -> (usize, Option<std::io::Error>) {
        let mut __disk = self.__go_disk.lock().unwrap();
        if self.__go_handle_closed.load(std::sync::atomic::Ordering::SeqCst) {
            return (0, Some(self.__go_closed_error("write")));
        }
        if self.__go_std_stream != 0 {
            drop(__disk);
            // os.Stdout is not buffered in Go: each write reaches it.
            let __result = if self.__go_std_stream == 1 {
                let mut __out = std::io::stdout().lock();
                std::io::Write::write_all(&mut __out, data).and_then(|_| std::io::Write::flush(&mut __out))
            } else {
                let mut __out = std::io::stderr().lock();
                std::io::Write::write_all(&mut __out, data)
            };
            return match __result {
                Ok(()) => (data.len(), None),
                Err(__err) => (0, Some(self.__go_op_error("write", __err))),
            };
        }
        if let Some(__file) = __disk.as_mut() {
            let mut __done = 0usize;
            while __done < data.len() {
                match std::io::Write::write(__file, &data[__done..]) {
                    Ok(0) => return (__done, Some(std::io::Error::new(std::io::ErrorKind::WriteZero, format!("write {}: short write", self.__go_name)))),
                    Ok(__count) => __done += __count,
                    Err(__err) if __err.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(__err) => return (__done, Some(self.__go_op_error("write", __err))),
                }
            }
            return (__done, None);
        }
        drop(__disk);
        self.__go_data.lock().unwrap().extend_from_slice(data);
        (data.len(), None)
    }

    pub fn __go_write_bytes_result(&self, data: &[u8]) -> std::io::Result<()> {
        match self.__go_write_counted(data) {
            (_, None) => Ok(()),
            (_, Some(__err)) => Err(__err),
        }
    }

    pub fn __go_read_all(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    /// File.Name: the name the File was opened with (part of the
    /// os-file-create shim).
    pub fn name(&self) -> STRING_TYPE {
        NAME_VALUE
    }

    pub fn close(&self) -> ERROR_TYPE {
        let mut __disk = self.__go_disk.lock().unwrap();
        if self.__go_handle_closed.swap(true, std::sync::atomic::Ordering::SeqCst) {
            drop(__disk);
            let __err = self.__go_closed_error("close");
            return WRITE_ERROR;
        }
        self.__go_closed.store(true, std::sync::atomic::Ordering::SeqCst);
        // Dropping a File closes it and discards the result. Go's Close
        // reports a failed close(2), so on Unix close the descriptor
        // directly. The handle stays closed either way.
        if let Some(__file) = __disk.take() {
            drop(__disk);
            #[cfg(unix)]
            {
                let __fd = std::os::fd::IntoRawFd::into_raw_fd(__file);
                unsafe extern "C" {
                    fn close(fd: i32) -> i32;
                }
                if unsafe { close(__fd) } != 0 {
                    let __err = self.__go_op_error("close", std::io::Error::last_os_error());
                    return WRITE_ERROR;
                }
            }
            #[cfg(not(unix))]
            drop(__file);
        }
        NONE_ERROR
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (INT_TYPE, ERROR_TYPE) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<VEC_TYPE>() {
            VEC_BORROW.as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let (__count, __failure) = self.__go_write_counted(&bytes);
        let n = __count as i64;
        match __failure {
            Some(__err) => (n as INT_TYPE, WRITE_ERROR),
            None => (n as INT_TYPE, NONE_ERROR),
        }
    }

    pub fn write_string<T0: 'static>(&self, arg0: T0) -> (INT_TYPE, ERROR_TYPE) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<&str>() {
            (*v).to_string()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<STRING_TYPE>() {
            STRING_BORROW.as_ref().cloned().unwrap_or_default()
        } else {
            String::new()
        };
        let (__count, __failure) = self.__go_write_counted(value.as_bytes());
        let n = __count as i64;
        match __failure {
            Some(__err) => (n as INT_TYPE, WRITE_ERROR),
            None => (n as INT_TYPE, NONE_ERROR),
        }
    }

    pub fn read<T0: 'static>(&self, arg0: T0) -> (INT_TYPE, ERROR_TYPE) {
        // Go's Read: fill as much of p as the input has, or report the end
        // of the input with no bytes read. The destination is locked before
        // the disk file, as in ReadAt.
        enum __ReadOutcome { Bytes(usize), End, Failed(std::io::Error) }
        // Close sets the closed flag while it holds the disk lock, so the
        // flag is read under that lock: a Close that lands between the check
        // and the read cannot turn a closed-file error into EOF. Go checks
        // for a closed file before the buffer length, nil buffers included.
        let __outcome = if let Some(__buf) = (&arg0 as &dyn std::any::Any).downcast_ref::<VEC_TYPE>() {
            let mut __guard = __buf VEC_MUT_BORROW;
            let mut __disk = self.__go_disk.lock().unwrap();
            if self.__go_handle_closed.load(std::sync::atomic::Ordering::SeqCst) {
                __ReadOutcome::Failed(self.__go_closed_error("read"))
            } else {
                match __guard.as_mut() {
                    Some(__dst) => {
                        let __dst = __dst.as_mut_slice();
                        if __dst.is_empty() {
                            __ReadOutcome::Bytes(0)
                        } else if self.__go_stdin {
                            match std::io::Read::read(&mut std::io::stdin(), __dst) {
                                Ok(0) | Err(_) => __ReadOutcome::End,
                                Ok(__count) => __ReadOutcome::Bytes(__count),
                            }
                        } else if let Some(__disk) = __disk.as_mut() {
                            match std::io::Read::read(__disk, __dst) {
                                Ok(0) => __ReadOutcome::End,
                                Ok(__count) => __ReadOutcome::Bytes(__count),
                                Err(__err) => __ReadOutcome::Failed(self.__go_op_error("read", __err)),
                            }
                        } else {
                            let __data = self.__go_data.lock().unwrap();
                            let __pos = self.__go_read_pos.load(std::sync::atomic::Ordering::SeqCst).min(__data.len());
                            let __count = (__data.len() - __pos).min(__dst.len());
                            __dst[..__count].copy_from_slice(&__data[__pos..__pos + __count]);
                            self.__go_read_pos.store(__pos + __count, std::sync::atomic::Ordering::SeqCst);
                            if __count == 0 { __ReadOutcome::End } else { __ReadOutcome::Bytes(__count) }
                        }
                    }
                    None => __ReadOutcome::Bytes(0),
                }
            }
        } else {
            __ReadOutcome::Bytes(0)
        };
        match __outcome {
            __ReadOutcome::Bytes(__n) => {
                let n = __n;
                (n as INT_TYPE, NONE_ERROR)
            }
            __ReadOutcome::End => (0, EOF_ERROR),
            __ReadOutcome::Failed(__err) => (0, WRITE_ERROR),
        }
    }

    pub fn read_at<T0: 'static, T1: 'static>(&self, arg0: T0, arg1: T1) -> (INT_TYPE, ERROR_TYPE) {
        let offset = if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<i64>() {
            *v
        } else if let Some(v) = (&arg1 as &dyn std::any::Any).downcast_ref::<INT64_TYPE>() {
            v INT64_BORROW.as_ref().copied().unwrap_or_default()
        } else {
            0
        };
        if offset < 0 {
            let __err = std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("readat {}: negative offset", self.__go_name));
            return (0, WRITE_ERROR);
        }
        let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<VEC_TYPE>() else {
            return (0, NONE_ERROR);
        };
        // The destination first, then the disk file, as in Read.
        let mut guard = v VEC_MUT_BORROW;
        let Some(target) = guard.as_mut() else {
            return (0, NONE_ERROR);
        };
        // Go's ReadAt reads nothing into an empty p, so it succeeds even on
        // a closed file.
        if target.is_empty() {
            return (0, NONE_ERROR);
        }
        let mut __done = 0usize;
        let mut __failure: Option<std::io::Error> = None;
        // The closed flag is read under the disk lock, as in Read.
        let mut __disk = self.__go_disk.lock().unwrap();
        if self.__go_handle_closed.load(std::sync::atomic::Ordering::SeqCst) {
            drop(__disk);
            let __err = self.__go_closed_error("read");
            return (0, WRITE_ERROR);
        }
        if let Some(__file) = __disk.as_mut() {
            // ReadAt reads at offset without moving the File's own position.
            #[cfg(unix)]
            while __done < target.len() {
                match std::os::unix::fs::FileExt::read_at(&*__file, &mut target[__done..], offset as u64 + __done as u64) {
                    Ok(0) => break,
                    Ok(__count) => __done += __count,
                    Err(__err) if __err.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(__err) => {
                        __failure = Some(self.__go_op_error("read", __err));
                        break;
                    }
                }
            }
            #[cfg(not(unix))]
            {
                // ReadAt must leave the File's position where it was: without
                // the saved position, do not seek; a failed restore is the
                // call's error.
                match std::io::Seek::stream_position(__file) {
                    Err(__err) => __failure = Some(self.__go_op_error("read", __err)),
                    Ok(__saved) => {
                        if let Err(__err) = std::io::Seek::seek(__file, std::io::SeekFrom::Start(offset as u64)) {
                            __failure = Some(self.__go_op_error("read", __err));
                        } else {
                            while __done < target.len() {
                                match std::io::Read::read(__file, &mut target[__done..]) {
                                    Ok(0) => break,
                                    Ok(__count) => __done += __count,
                                    Err(__err) if __err.kind() == std::io::ErrorKind::Interrupted => {}
                                    Err(__err) => {
                                        __failure = Some(self.__go_op_error("read", __err));
                                        break;
                                    }
                                }
                            }
                        }
                        if let Err(__err) = std::io::Seek::seek(__file, std::io::SeekFrom::Start(__saved)) {
                            __failure.get_or_insert(self.__go_op_error("read", __err));
                        }
                    }
                }
            }
        } else {
            drop(__disk);
            let data = self.__go_read_all();
            let start = offset as usize;
            if start < data.len() {
                __done = std::cmp::min(target.len(), data.len() - start);
                target[..__done].copy_from_slice(&data[start..start + __done]);
            }
        }
        let n = __done as i64;
        match __failure {
            Some(__err) => (n as INT_TYPE, WRITE_ERROR),
            // Go's ReadAt reports the end of the file when it cannot fill p.
            None if __done < target.len() => (n as INT_TYPE, EOF_ERROR),
            None => (n as INT_TYPE, NONE_ERROR),
        }
    }
}
`

func writeOsFileStub(out *strings.Builder) {
	vecType := wrappedExternalStubType("Vec<u8>")
	int64Type := wrappedExternalStubType("i64")
	stringType := wrappedExternalStubType("String")
	// An int result is bare, as a stub function's is (callReturnsBareScalar):
	// n, err := f.Read(buf) binds n as an int.
	intType := rustIntType()
	errorInnerType := externalStubErrorInnerType()
	errorType := wrappedExternalStubType(errorInnerType)
	noneError := wrappedExternalStubNoneExpr(errorInnerType)
	writeError := wrappedExternalStubExpr(errorInnerType, rustErrorFromValueFor("std::error::Error", NeedsConcurrentWrapper(), "__err"))
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
    /// How far Read has read __go_data; shared by every copy of the File.
    pub __go_read_pos: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    /// os.Stdin: Read reads the process's standard input.
    pub __go_stdin: bool,
    /// os.Stdout (1) or os.Stderr (2): writes go to the process's standard
    /// output or error, through the same handle as print!.
    pub __go_std_stream: u8,
    /// The file os.Create opened: reads and writes go to it, and Close
    /// closes it.
    pub __go_disk: std::sync::Arc<std::sync::Mutex<Option<std::fs::File>>>,
    /// The name the File was opened with, for its errors.
    pub __go_name: String,
    /// This File was closed: shared by its copies, but not by the other
    /// end of a pipe, which __go_closed is.
    pub __go_handle_closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl Default for os_File {
    fn default() -> Self {
        Self {
            __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            __go_wait_for_close: false,
            __go_read_pos: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            __go_stdin: false,
            __go_std_stream: 0,
            __go_disk: std::sync::Arc::new(std::sync::Mutex::new(None)),
            __go_name: String::new(),
            __go_handle_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
}

/// The end of a File's input until a source-translated io registers its
/// io.EOF (__go_register_io_eof), the value callers compare with.
#[derive(Debug)]
pub struct GoStubEOF;

impl std::fmt::Display for GoStubEOF {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "EOF")
    }
}

impl std::error::Error for GoStubEOF {}

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

`)
	out.WriteString(strings.NewReplacer(
		"ERROR_TYPE", errorType,
		"NONE_ERROR", noneError,
		"WRITE_ERROR", writeError,
		"EOF_ERROR", stubEOFError(errorInnerType),
		"INT_TYPE", intType,
		"INT64_TYPE", int64Type,
		"INT64_BORROW", int64Borrow,
		"VEC_TYPE", vecType,
		"VEC_BORROW", vecBorrow,
		"VEC_MUT_BORROW", vecMutBorrow,
		"STRING_TYPE", stringType,
		"STRING_BORROW", stringBorrow,
		"NAME_VALUE", wrappedExternalStubExpr("String", "self.__go_name.clone()"),
	).Replace(osFileStubCoreMethods))
	writeStubIoEOFRegistry(out, errorInnerType, errorType)
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
	errorFromMessage := rustErrorFromTextFor("StdError", NeedsConcurrentWrapper(), "message")
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

    /// Go's ExitError text: "exit status 3", not Rust's "exit status: 3".
    fn __go_exit_message(status: &std::process::ExitStatus) -> String {
        match status.code() {
            Some(code) => format!("exit status {}", code),
            None => status.to_string(),
        }
    }

    fn __go_run_output(&self) -> Result<std::process::Output, std::io::Error> {
        let args = %s.as_ref().cloned().unwrap_or_default();
        if args.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "empty command"));
        }
        let mut command = std::process::Command::new(&args[0]);
        command.args(&args[1..]);
`, errorType, wrappedExternalStubExpr(errorInnerType, errorFromMessage), externalStubBorrowExpr("self.args"))
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
	if fields["stdin"] == wrappedExternalStubType(stubReaderTypeName) {
		fmt.Fprintf(out, `        if let Some(stdin) = %s.as_ref() {
            let data = stdin.__go_read_all();
            command.stdin(std::process::Stdio::piped());
            command.stdout(std::process::Stdio::piped());
            command.stderr(std::process::Stdio::piped());
            let mut child = command.spawn()?;
            if let Some(mut pipe) = child.stdin.take() {
                use std::io::Write;
                let _ = pipe.write_all(&data);
            }
            return child.wait_with_output();
        }
`, externalStubBorrowExpr("self.stdin"))
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
                    Self::__go_error(Self::__go_exit_message(&output.status))
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
                    Self::__go_error(Self::__go_exit_message(&output.status))
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
	if name == fmtStringerStubName {
		writeFmtStringerStub(out, methods)
		return
	}
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
	out.WriteString("        Some(std::cmp::Ord::cmp(self, other))\n")
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
	if packageStubsUseFilesystemSentinel(packageStubs) {
		if needsSeparator || out.Len() > 0 {
			out.WriteString("\n\n")
		}
		needsSeparator = true
		writeStubFilesystemSentinels(out)
	}
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
		if _, ok := pkg.Functions["look_path"]; ok {
			// An aliased os/exec import is keyed by its local package name, so it
			// takes this generic package path rather than writeExecPackageStub.
			writeGoStringArgTrait(out)
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
			writeExternalPackageStubVariable(out, pkgName, varName, pkg.Variables[varName])
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
	if goSliceModeEnabled() && strings.HasPrefix(innerType, "Vec<") {
		// The value may be the standard library's Vec, from a std API.
		expr = "Vec::from(" + expr + ")"
	}
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
    /// The modification time: seconds and nanoseconds since the Unix epoch.
    pub mod_time: (i64, i64),
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
	// Part of the os-file-create shim (docs/bridge_debt.md). ModTime gives the
	// time Stat read from the file's metadata.
	modTime, hasModTime := methods["mod_time"]
	hasModTime = hasModTime && len(modTime.ReturnTypes) == 1 && strings.Contains(modTime.ReturnTypes[0], "GoTime")
	if hasModTime {
		fmt.Fprintf(out, `    pub fn mod_time(&self) -> %s {
        %s
    }
`, modTime.ReturnTypes[0], wrappedExternalStubExpr("GoTime", "GoTime::from_unix(self.mod_time.0, self.mod_time.1)"))
	}
	methodNames := make([]string, 0, len(methods))
	for methodName := range methods {
		if methodName == "name" || methodName == "size" || methodName == "is_dir" || methodName == "mode" || (methodName == "mod_time" && hasModTime) {
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

// PERMANENT: sync is native tier (docs/stdlib_boundary.md), so hand-written
// Rust is its implementation and not bridge debt.
//
// writeSyncPoolMethod writes sync.Pool's Get and Put and reports whether
// methodName is one of them. A Pool may drop any value put in it at any
// time, so a Pool that keeps nothing is a correct one: Get returns what New
// makes, or nil without New, and Put drops its value. regexp's pools have
// no New and make a new machine when Get gives nil. A program that never
// names New gets a Pool stub without the field, whose Get is always nil.
//
// Get takes New out of its cell before calling it, so New may use the
// field. (The receiver's own cell is the caller's to hold; New that calls
// Get or Put on the same pool still waits on it.)
func writeSyncPoolMethod(out *strings.Builder, methodName string, method externalTypeStubMethod, fields map[string]string) bool {
	switch methodName {
	case "get":
		if len(method.ReturnTypes) != 1 {
			return false
		}
		result := method.ReturnTypes[0]
		outer, inner, ok := externalStubWrapperNames(result)
		if !ok {
			return false
		}
		nilValue := outer + "::new(" + inner + "::new(None))"
		newField, hasNew := fields["new"]
		if !hasNew {
			fmt.Fprintf(out, "    pub fn get(&self) -> %s {\n        %s\n    }\n", result, nilValue)
			return true
		}
		readNew := "self.new.lock().unwrap().clone()"
		if fieldOuter, _, ok := externalStubWrapperNames(newField); ok && fieldOuter == "Rc" {
			readNew = "self.new.borrow().clone()"
		}
		fmt.Fprintf(out, `    pub fn get(&self) -> %s {
        let __new = %s;
        match __new {
            Some(__new) => __new(),
            None => %s,
        }
    }
`, result, readNew, nilValue)
		return true
	case "put":
		if method.ParamCount != 1 || len(method.ReturnTypes) != 0 {
			return false
		}
		out.WriteString("    pub fn put<T0>(&self, _arg0: T0) {}\n")
		return true
	}
	return false
}

// externalStubWrapperNames returns the outer and inner wrapper of a stub
// cell type: Arc and Mutex for Arc<Mutex<Option<T>>>, Rc and RefCell for
// Rc<RefCell<Option<T>>>, or the name a program's own Mutex leaves the
// standard one (StdMutex).
func externalStubWrapperNames(typ string) (outer string, inner string, ok bool) {
	for _, candidate := range []string{"Arc", "Rc"} {
		rest, found := strings.CutPrefix(typ, candidate+"<")
		if !found {
			continue
		}
		end := strings.Index(rest, "<")
		if end <= 0 {
			return "", "", false
		}
		return candidate, rest[:end], true
	}
	return "", "", false
}

// PERMANENT: not scaffold — io/fs.FileMode is OS-tied; Rust std::fs::Metadata is the long-term implementation.
func writeFsFileModeIsDirMethod(out *strings.Builder) {
	out.WriteString(`    pub fn is_dir(&self) -> bool {
        (self.0 & (1u32 << 31)) != 0
    }
`)
}

// TEMPORARY: fs-filemode-perm — remove when ordinary io/fs.FileMode callers use the source method.
func writeFsFileModePermMethod(out *strings.Builder, method externalTypeStubMethod) {
	out.WriteString("    pub fn perm(&self) -> ")
	writeExternalStubReturnType(out, method.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        ")
	out.WriteString(wrappedExternalStubExpr("fs_FileMode", "fs_FileMode(self.0 & 0o777)"))
	out.WriteString("\n")
	out.WriteString("    }\n")
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

// TEMPORARY: io-fs-path-error in docs/bridge_debt.md. Remove when io/fs.PathError
// is source-transpiled for ordinary callers instead of emitted as a stub.
func writeFsPathErrorStub(out *strings.Builder, methods map[string]externalTypeStubMethod) {
	borrow := ".borrow()"
	if NeedsConcurrentWrapper() {
		borrow = ".lock().unwrap()"
	}
	stringType := wrappedExternalStubType("String")
	errorType := wrappedExternalStubType(externalStubErrorInnerType())
	fmt.Fprintf(out, `#[derive(Debug, Clone, Default)]
pub struct fs_PathError {
    pub err: %s,
    pub op: %s,
    pub path: %s,
}

impl std::fmt::Display for fs_PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __op_guard = self.op%s;
        let __path_guard = self.path%s;
        let __err_guard = self.err%s;
        let __op = __op_guard.as_deref().unwrap_or("");
        let __path = __path_guard.as_deref().unwrap_or("");
        let __err = __err_guard.as_ref().map(|e| e.to_string()).unwrap_or_default();
        if __op.is_empty() {
            write!(f, "{}: {}", __path, __err)
        } else {
            write!(f, "{} {}: {}", __op, __path, __err)
        }
    }
}

impl std::error::Error for fs_PathError {}

impl fs_PathError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> %s {
        %s
    }
    pub fn unwrap(&self) -> %s {
        self.err.clone()
    }
`, errorType, stringType, stringType, borrow, borrow, borrow, stringType, wrappedExternalStubExpr("String", "self.to_string()"), errorType)
	methodNames := make([]string, 0, len(methods))
	for methodName := range methods {
		if methodName == "error" || methodName == "unwrap" {
			continue
		}
		methodNames = append(methodNames, methodName)
	}
	slices.Sort(methodNames)
	for _, methodName := range methodNames {
		writeExternalTypeStubMethod(out, "fs_PathError", methodName, methods[methodName])
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
		case "valid":
			out.WriteString("    pub fn valid<T0: GoJsonInputArg>(_arg0: T0) -> bool {\n")
			out.WriteString("        let bytes = _arg0.into_go_json_bytes();\n")
			out.WriteString("        let mut depth = 0usize;\n")
			out.WriteString("        let mut in_string = false;\n")
			out.WriteString("        let mut escaped = false;\n")
			out.WriteString("        for byte in &bytes {\n")
			out.WriteString("            if in_string {\n")
			out.WriteString("                if escaped {\n")
			out.WriteString("                    escaped = false;\n")
			out.WriteString("                } else if *byte == b'\\\\' {\n")
			out.WriteString("                    escaped = true;\n")
			out.WriteString("                } else if *byte == b'\"' {\n")
			out.WriteString("                    in_string = false;\n")
			out.WriteString("                }\n")
			out.WriteString("            } else {\n")
			out.WriteString("                match *byte {\n")
			out.WriteString("                    b'\"' => in_string = true,\n")
			out.WriteString("                    b'{' | b'[' => {\n")
			out.WriteString("                        depth += 1;\n")
			out.WriteString("                        if depth > 10_000 { return false; }\n")
			out.WriteString("                    }\n")
			out.WriteString("                    b'}' | b']' => depth = depth.saturating_sub(1),\n")
			out.WriteString("                    _ => {}\n")
			out.WriteString("                }\n")
			out.WriteString("            }\n")
			out.WriteString("        }\n")
			out.WriteString("        let text = String::from_utf8_lossy(&bytes);\n")
			out.WriteString("        serde_json::from_str::<Box<serde_json::value::RawValue>>(&text).is_ok()\n")
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
	_, needsExecutable := pkg.Functions["executable"]
	_, needsHostname := pkg.Functions["hostname"]
	if needsFilesystemHelpers {
		out.WriteString("    use std::path::Path;\n\n")
		writeGoStringArgTrait(out)
		if _, ok := pkg.Functions["write_file"]; ok {
			writeGoBytesArgTrait(out)
		}
	}
	if needsFilesystemHelpers || needsExecutable || needsHostname {
		writeOsErrorHelpers(out, osStubReturnsPathError(pkg))
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
		if varName == "Stdin" && strings.Contains(pkg.Variables[varName], "os_File") {
			// os.Stdin reads the process's standard input.
			out.WriteString("    pub fn Stdin() -> ")
			out.WriteString(pkg.Variables[varName])
			out.WriteString(" {\n        ")
			out.WriteString(wrappedExternalStubExpr("os_File", "os_File { __go_stdin: true, ..Default::default() }"))
			out.WriteString("\n    }\n")
		} else if stream, isStd := map[string]string{"Stdout": "1", "Stderr": "2"}[varName]; isStd && strings.Contains(pkg.Variables[varName], "os_File") {
			// TEMPORARY: os-std-streams in docs/bridge_debt.md. Remove when os
			// translates from source with syscall.Write as the bridge.
			// os.Stdout and os.Stderr write to the process's streams. Every
			// read of the variable is the same *File, as in Go, so
			// out == os.Stdout compares equal.
			handleType := pkg.Variables[varName]
			file := wrappedExternalStubExpr("os_File", "os_File { __go_std_stream: "+stream+", __go_name: \"/dev/"+strings.ToLower(varName)+"\".to_string(), ..Default::default() }")
			out.WriteString("    pub fn " + varName + "() -> ")
			out.WriteString(handleType)
			out.WriteString(" {\n")
			if NeedsConcurrentWrapper() {
				out.WriteString("        static __GO_FILE: std::sync::OnceLock<" + handleType + "> = std::sync::OnceLock::new();\n")
				out.WriteString("        __GO_FILE.get_or_init(|| " + file + ").clone()\n")
			} else {
				out.WriteString("        thread_local! { static __GO_FILE: " + handleType + " = " + file + "; }\n")
				out.WriteString("        __GO_FILE.with(|__file| __file.clone())\n")
			}
			out.WriteString("    }\n")
		} else {
			writeExternalPackageStubVariable(out, "os", varName, pkg.Variables[varName])
		}
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
		} else if funcName == "executable" {
			writeOsExecutableFunction(out, pkg.Functions[funcName])
		} else if funcName == "getwd" {
			writeOsGetwdFunction(out, pkg.Functions[funcName])
		} else if funcName == "getenv" {
			writeOsGetenvFunction(out, pkg.Functions[funcName])
		} else if funcName == "hostname" {
			writeOsHostnameFunction(out, pkg.Functions[funcName])
		} else if funcName == "is_path_separator" {
			writeOsIsPathSeparatorFunction(out, pkg.Functions[funcName])
		} else if funcName == "lstat" {
			writeOsLstatFunction(out, pkg.Functions[funcName])
		} else if funcName == "mkdir_all" {
			writeOsMkdirAllFunction(out, pkg.Functions[funcName])
		} else if funcName == "create" {
			writeOsCreateFunction(out, pkg.Functions[funcName])
		} else if funcName == "open_file" {
			writeOsOpenFileFunction(out, pkg.Functions[funcName])
		} else if funcName == "is_not_exist" || funcName == "is_exist" || funcName == "is_permission" {
			writeOsIsErrorKindFunction(out, funcName, osStubReturnsPathError(pkg) || currentExternalTypeStubs()["fs_PathError"])
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

// osStubReturnsPathError reports whether the os stub has a function that
// returns an *fs.PathError: os.Open and os.Create.
func osStubReturnsPathError(pkg *externalPackageStub) bool {
	if pkg == nil {
		return false
	}
	_, open := pkg.Functions["open"]
	_, create := pkg.Functions["create"]
	_, openFile := pkg.Functions["open_file"]
	return open || create || openFile
}

func osPackageStubNeedsFilesystemHelpers(pkg *externalPackageStub) bool {
	if pkg == nil {
		return false
	}
	_, needsStat := pkg.Functions["stat"]
	_, needsMkdirAll := pkg.Functions["mkdir_all"]
	_, needsOpen := pkg.Functions["open"]
	_, needsCreate := pkg.Functions["create"]
	_, needsOpenFile := pkg.Functions["open_file"]
	_, needsReadDir := pkg.Functions["read_dir"]
	_, needsReadFile := pkg.Functions["read_file"]
	_, needsWriteFile := pkg.Functions["write_file"]
	_, needsGetwd := pkg.Functions["getwd"]
	_, needsLstat := pkg.Functions["lstat"]
	return needsStat || needsMkdirAll || needsOpen || needsCreate || needsOpenFile || needsReadDir || needsReadFile || needsWriteFile || needsGetwd || needsLstat
}

func writeOsErrorHelpers(out *strings.Builder, includeOpenPathError bool) {
	errorType := wrappedExternalStubType(rustErrorPayloadTypeFor("std::error::Error", NeedsConcurrentWrapper()))
	fmt.Fprintf(out, `    type GoError = %s;

    fn no_error() -> GoError {
        %s::new(%s::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        %s::new(%s::new(Some(%s)))
    }

`, errorType, GetOuterWrapperType(), externalStubInnerWrapperType(), GetOuterWrapperType(), externalStubInnerWrapperType(), rustErrorFromValueFor("std::error::Error", NeedsConcurrentWrapper(), "err"))
	if !includeOpenPathError {
		return
	}

	// TEMPORARY: os-open-path-error in docs/bridge_debt.md. Remove when os.Open
	// and io/fs.PathError share the source-transpiled stdlib path.
	pathError := "fs_PathError { err: " + wrappedExternalStubExpr(externalStubErrorInnerType(), rustErrorFromValueFor("std::error::Error", NeedsConcurrentWrapper(), "err")) +
		", op: " + wrappedExternalStubExpr("String", "op.to_string()") +
		", path: " + wrappedExternalStubExpr("String", "path") + " }"
	fmt.Fprintf(out, `    fn path_error(op: &str, path: String, err: std::io::Error) -> GoError {
        let path_error = %s;
        %s::new(%s::new(Some(%s)))
    }

`, pathError, GetOuterWrapperType(), externalStubInnerWrapperType(), rustErrorFromValueFor("std::error::Error", NeedsConcurrentWrapper(), "path_error"))
}

// PERMANENT: not scaffold — os.Exit maps to std::process::exit, runtime-tied.
func writeOsExitFunction(out *strings.Builder) {
	out.WriteString("    pub fn exit<T0: Into<i64>>(_arg0: T0) {\n")
	out.WriteString("        std::process::exit(_arg0.into() as i32);\n")
	out.WriteString("    }\n")
}

// PERMANENT: not scaffold — os.Getenv maps to std::env::var in the host process environment.
func writeOsGetenvFunction(out *strings.Builder, fn externalPackageStubFunction) {
	stringType := wrappedExternalStubType("String")
	stringBorrow := externalStubBorrowExpr("v")

	out.WriteString("    pub fn getenv<T0: 'static>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let key = if let Some(v) = (&_arg0 as &dyn std::any::Any).downcast_ref::<String>() {\n")
	out.WriteString("            v.clone()\n")
	out.WriteString("        } else if let Some(v) = (&_arg0 as &dyn std::any::Any).downcast_ref::<")
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

// PERMANENT: not scaffold — os.Executable is a host OS/runtime query.
func writeOsExecutableFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn executable() -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        match std::env::current_exe() {\n")
	out.WriteString("            Ok(path) => (")
	out.WriteString(wrappedExternalStubExpr("String", "path.to_string_lossy().into_owned()"))
	out.WriteString(", no_error()),\n")
	out.WriteString("            Err(err) => (")
	out.WriteString(wrappedExternalStubExpr("String", "String::new()"))
	out.WriteString(", io_error(err)),\n")
	out.WriteString("        }\n")
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

// PERMANENT: not scaffold — os.Hostname is a host OS/runtime query.
// Linux reads the kernel's node name from /proc/sys/kernel/hostname, the same
// value Go's uname and /proc lookups return. Windows reads COMPUTERNAME. Other
// systems ask the host's hostname command.
func writeOsHostnameFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn hostname() -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(` {
        #[cfg(windows)]
        let bytes = std::env::var("COMPUTERNAME")
            .map(String::into_bytes)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::NotFound, err.to_string()));

        #[cfg(target_os = "linux")]
        let bytes = std::fs::read("/proc/sys/kernel/hostname");

        #[cfg(not(any(windows, target_os = "linux")))]
        let bytes = std::process::Command::new("hostname").output().and_then(|output| {
            if !output.status.success() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("hostname exited with {}", output.status),
                ));
            }
            Ok(output.stdout)
        });

        let result = bytes.and_then(|mut bytes| {
            while matches!(bytes.last(), Some(b'\n' | b'\r')) {
                bytes.pop();
            }
            String::from_utf8(bytes)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
        });

        match result {
            Ok(value) => (`)
	out.WriteString(wrappedExternalStubExpr("String", "value"))
	out.WriteString(`, no_error()),
            Err(err) => (`)
	out.WriteString(wrappedExternalStubExpr("String", "String::new()"))
	out.WriteString(`, io_error(err)),
        }
    }
`)
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
	out.WriteString(wrappedExternalStubExpr("fs_FileInfo", "fs_FileInfo { name, is_dir: metadata.is_dir(), size: metadata.len() as i64, mod_time: metadata.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| (d.as_secs() as i64, d.subsec_nanos() as i64)).unwrap_or((0, 0)) }"))
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
	out.WriteString(wrappedExternalStubExpr("fs_FileInfo", "fs_FileInfo { name, is_dir: metadata.is_dir(), size: metadata.len() as i64, mod_time: metadata.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| (d.as_secs() as i64, d.subsec_nanos() as i64)).unwrap_or((0, 0)) }"))
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
	out.WriteString("                let file = os_File { __go_data: std::sync::Arc::new(std::sync::Mutex::new(data)), __go_closed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), __go_wait_for_close: false, __go_name: path.clone(), ..Default::default() };\n")
	out.WriteString("                (")
	out.WriteString(wrappedExternalStubExpr("os_File", "file"))
	out.WriteString(", no_error())\n")
	out.WriteString("            }\n")
	out.WriteString("            Err(err) => (")
	out.WriteString(wrappedExternalStubNoneExpr("os_File"))
	out.WriteString(", path_error(\"open\", path, err)),\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

// TEMPORARY: os-file-create in docs/bridge_debt.md. os.Create maps to
// std::fs::File::create for the os.File stub, which writes through to the
// file on disk, until os translates from source over syscall.Open.
func writeOsCreateFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn create<T0: GoStringArg>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        match std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(&path) {\n")
	out.WriteString("            Ok(disk) => {\n")
	out.WriteString("                let file = os_File { __go_disk: std::sync::Arc::new(std::sync::Mutex::new(Some(disk))), __go_name: path.clone(), ..Default::default() };\n")
	out.WriteString("                (")
	out.WriteString(wrappedExternalStubExpr("os_File", "file"))
	out.WriteString(", no_error())\n")
	out.WriteString("            }\n")
	out.WriteString("            Err(err) => (")
	out.WriteString(wrappedExternalStubNoneExpr("os_File"))
	out.WriteString(", path_error(\"open\", path, err)),\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

// Part of the os-file-create shim (docs/bridge_debt.md). os.OpenFile opens the
// file with std::fs::OpenOptions set from Go's flags, which the type checker
// has folded to this platform's values, for the os.File stub.
func writeOsOpenFileFunction(out *strings.Builder, fn externalPackageStubFunction) {
	intType := wrappedExternalStubType("i64")
	intBorrow := ".borrow()"
	if NeedsConcurrentWrapper() {
		intBorrow = ".lock().unwrap()"
	}
	out.WriteString("    pub fn open_file<T0: GoStringArg, T1: 'static, T2: 'static>(_arg0: T0, _arg1: T1, _arg2: T2) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        let flag: i64 = if let Some(v) = (&_arg1 as &dyn std::any::Any).downcast_ref::<i64>() {\n")
	out.WriteString("            *v\n")
	out.WriteString("        } else if let Some(v) = (&_arg1 as &dyn std::any::Any).downcast_ref::<i32>() {\n")
	out.WriteString("            *v as i64\n")
	out.WriteString("        } else if let Some(v) = (&_arg1 as &dyn std::any::Any).downcast_ref::<" + intType + ">() {\n")
	out.WriteString("            v" + intBorrow + ".as_ref().copied().unwrap_or_default()\n")
	out.WriteString("        } else {\n")
	out.WriteString("            0\n")
	out.WriteString("        };\n")
	fmt.Fprintf(out, "        let access = flag & %d;\n", syscall.O_RDONLY|syscall.O_WRONLY|syscall.O_RDWR)
	out.WriteString("        let mut options = std::fs::OpenOptions::new();\n")
	fmt.Fprintf(out, "        options.read(access == %d || access == %d).write(access == %d || access == %d);\n", syscall.O_RDONLY, syscall.O_RDWR, syscall.O_WRONLY, syscall.O_RDWR)
	fmt.Fprintf(out, "        if flag & %d != 0 { options.append(true); }\n", syscall.O_APPEND)
	fmt.Fprintf(out, "        if flag & %d != 0 { if flag & %d != 0 { options.create_new(true); } else { options.create(true); } }\n", syscall.O_CREAT, syscall.O_EXCL)
	// Rust refuses append with truncate; on Unix the truncation goes in as
	// the open flag Go passes, so both hold in one open. O_SYNC has no
	// OpenOptions setter and goes in the same way. custom_flags replaces
	// the mask, so both share one call.
	fmt.Fprintf(out, "        #[cfg(unix)]\n")
	fmt.Fprintf(out, "        {\n")
	fmt.Fprintf(out, "            let mut native = flag & %d;\n", syscall.O_SYNC)
	fmt.Fprintf(out, "            if flag & %d != 0 { if flag & %d != 0 { native |= %d; } else { options.truncate(true); } }\n", syscall.O_TRUNC, syscall.O_APPEND, syscall.O_TRUNC)
	fmt.Fprintf(out, "            if native != 0 { std::os::unix::fs::OpenOptionsExt::custom_flags(&mut options, native as i32); }\n")
	fmt.Fprintf(out, "        }\n")
	fmt.Fprintf(out, "        #[cfg(not(unix))]\n")
	fmt.Fprintf(out, "        if flag & %d != 0 { options.truncate(true); }\n", syscall.O_TRUNC)
	// The permission bits a created file gets, before the umask, as Go's.
	if currentExternalTypeStubs()["fs_FileMode"] {
		out.WriteString("        let perm: Option<u32> = if let Some(v) = (&_arg2 as &dyn std::any::Any).downcast_ref::<fs_FileMode>() { Some(v.0 & 0o777) } else if let Some(v) = (&_arg2 as &dyn std::any::Any).downcast_ref::<u32>() { Some(*v & 0o777) } else { None };\n")
	} else {
		out.WriteString("        let perm: Option<u32> = if let Some(v) = (&_arg2 as &dyn std::any::Any).downcast_ref::<u32>() { Some(*v & 0o777) } else { None };\n")
	}
	out.WriteString("        #[cfg(unix)]\n")
	out.WriteString("        if let Some(perm) = perm { std::os::unix::fs::OpenOptionsExt::mode(&mut options, perm); }\n")
	out.WriteString("        #[cfg(not(unix))]\n")
	out.WriteString("        let _ = perm;\n")
	out.WriteString("        match options.open(&path) {\n")
	out.WriteString("            Ok(disk) => {\n")
	out.WriteString("                let file = os_File { __go_disk: std::sync::Arc::new(std::sync::Mutex::new(Some(disk))), __go_name: path.clone(), ..Default::default() };\n")
	out.WriteString("                (")
	out.WriteString(wrappedExternalStubExpr("os_File", "file"))
	out.WriteString(", no_error())\n")
	out.WriteString("            }\n")
	out.WriteString("            Err(err) => (")
	out.WriteString(wrappedExternalStubNoneExpr("os_File"))
	out.WriteString(", path_error(\"open\", path, err)),\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

// Part of the os-file-create shim (docs/bridge_debt.md). os.IsNotExist,
// os.IsExist and os.IsPermission look at the kind of the Rust I/O error
// that the os stub's functions return, through an *fs.PathError as Go's
// do, until os translates from source and they compare syscall errnos.
func writeOsIsErrorKindFunction(out *strings.Builder, funcName string, unwrapPathError bool) {
	kind := map[string]string{
		"is_not_exist":  "NotFound",
		"is_exist":      "AlreadyExists",
		"is_permission": "PermissionDenied",
	}[funcName]
	errorType := wrappedExternalStubType(rustErrorPayloadTypeFor("std::error::Error", NeedsConcurrentWrapper()))
	borrow := ".borrow()"
	if NeedsConcurrentWrapper() {
		borrow = ".lock().unwrap()"
	}
	fmt.Fprintf(out, "    pub fn %s<T0: 'static>(_arg0: T0) -> bool {\n", funcName)
	fmt.Fprintf(out, "        fn kind_is(err: &(dyn std::error::Error + 'static)) -> bool {\n")
	fmt.Fprintf(out, "            if let Some(io) = err.downcast_ref::<std::io::Error>() {\n")
	fmt.Fprintf(out, "                return io.kind() == std::io::ErrorKind::%s;\n", kind)
	fmt.Fprintf(out, "            }\n")
	if unwrapPathError {
		// Go's underlyingError unwraps one *PathError and classifies its
		// cause; a PathError inside a PathError is not looked through.
		fmt.Fprintf(out, "            if let Some(path_error) = err.downcast_ref::<fs_PathError>() {\n")
		fmt.Fprintf(out, "                if let Some(inner) = path_error.err%s.as_ref() {\n", borrow)
		fmt.Fprintf(out, "                    return inner.downcast_ref::<std::io::Error>().map_or(false, |io| io.kind() == std::io::ErrorKind::%s);\n", kind)
		fmt.Fprintf(out, "                }\n")
		fmt.Fprintf(out, "            }\n")
	}
	fmt.Fprintf(out, "            false\n")
	fmt.Fprintf(out, "        }\n")
	fmt.Fprintf(out, "        let Some(err) = (&_arg0 as &dyn std::any::Any).downcast_ref::<%s>() else { return false; };\n", errorType)
	fmt.Fprintf(out, "        let guard = err%s;\n", borrow)
	fmt.Fprintf(out, "        match guard.as_ref() {\n")
	fmt.Fprintf(out, "            Some(err) => kind_is(&**err),\n")
	fmt.Fprintf(out, "            None => false,\n")
	fmt.Fprintf(out, "        }\n")
	fmt.Fprintf(out, "    }\n")
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
	errorType := wrappedExternalStubType(rustErrorPayloadTypeFor("std::error::Error", NeedsConcurrentWrapper()))
	fmt.Fprintf(out, `    type GoError = %s;

    fn no_error() -> GoError {
        %s::new(%s::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        %s::new(%s::new(Some(%s)))
    }

    fn normalize_path(path: PathBuf) -> String {
        path.components().collect::<PathBuf>().to_string_lossy().into_owned()
    }

`, errorType, GetOuterWrapperType(), externalStubInnerWrapperType(), GetOuterWrapperType(), externalStubInnerWrapperType(), rustErrorFromValueFor("std::error::Error", NeedsConcurrentWrapper(), "err"))
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
// stubEOFError is the error handle a File's Read returns at the end of its
// input: io.EOF's value once a source-translated io registered it.
func stubEOFError(errorInnerType string) string {
	return "__go_stub_eof()"
}

// TEMPORARY: os-file-read — remove when os translates from source with syscall.Read as the bridge.
// writeStubIoEOFRegistry writes where a source-translated io registers
// io.EOF, so a File's end of input is the value callers compare with,
// read directly (n, err := f.Read(buf)) or through io.Reader. The stub
// crate cannot name io's EOF: io depends on it. Until io registers, the end
// of input is GoStubEOF, which prints as EOF.
func writeStubIoEOFRegistry(out *strings.Builder, errorInnerType string, errorType string) {
	pointer := "std::rc::Rc"
	if strings.HasPrefix(errorInnerType, "Arc<") || strings.HasPrefix(errorInnerType, "std::sync::Arc<") {
		pointer = "std::sync::Arc"
	}
	fallback := wrappedExternalStubExpr(errorInnerType, pointer+"::new(GoStubEOF) as "+errorInnerType)
	registered := wrappedExternalStubExpr(errorInnerType, "__eof.clone()")
	if NeedsConcurrentWrapper() {
		fmt.Fprintf(out, `
static __GO_STUB_IO_EOF: std::sync::OnceLock<%s> = std::sync::OnceLock::new();

/// io's init hands over io.EOF's value.
pub fn __go_register_io_eof(eof: Option<%s>) {
    if let Some(eof) = eof {
        let _ = __GO_STUB_IO_EOF.set(eof);
    }
}

fn __go_stub_eof() -> %s {
    match __GO_STUB_IO_EOF.get() {
        Some(__eof) => %s,
        None => %s,
    }
}
`, errorInnerType, errorInnerType, errorType, registered, fallback)
		return
	}
	fmt.Fprintf(out, `
thread_local! {
    static __GO_STUB_IO_EOF: std::cell::RefCell<Option<%s>> = std::cell::RefCell::new(None);
}

/// io's init hands over io.EOF's value.
pub fn __go_register_io_eof(eof: Option<%s>) {
    if eof.is_some() {
        __GO_STUB_IO_EOF.with(|__slot| *__slot.borrow_mut() = eof);
    }
}

fn __go_stub_eof() -> %s {
    __GO_STUB_IO_EOF.with(|__slot| match __slot.borrow().as_ref() {
        Some(__eof) => %s,
        None => %s,
    })
}
`, errorInnerType, errorInnerType, errorType, registered, fallback)
}

// stubFilesystemSentinels are io/fs's ErrNotExist, ErrExist and
// ErrPermission, which os also exports: each one's Rust I/O error kind and
// Go's text.
var stubFilesystemSentinels = []struct{ name, kind, text string }{
	{"ErrNotExist", "NotFound", "file does not exist"},
	{"ErrExist", "AlreadyExists", "file already exists"},
	{"ErrPermission", "PermissionDenied", "permission denied"},
}

// stubFilesystemSentinelIndex returns the index in stubFilesystemSentinels
// of the fs or os variable varName.
func stubFilesystemSentinelIndex(pkgName, varName string) (int, bool) {
	if pkgName != "fs" && pkgName != "os" {
		return 0, false
	}
	for i, sentinel := range stubFilesystemSentinels {
		if sentinel.name == varName {
			return i, true
		}
	}
	return 0, false
}

// packageStubsUseFilesystemSentinel reports whether the fs or os stub has a
// variable that is one of stubFilesystemSentinels.
func packageStubsUseFilesystemSentinel(packageStubs map[string]*externalPackageStub) bool {
	for _, pkgName := range []string{"fs", "os"} {
		if pkg := packageStubs[pkgName]; pkg != nil {
			for varName := range pkg.Variables {
				if _, ok := stubFilesystemSentinelIndex(pkgName, varName); ok {
					return true
				}
			}
		}
	}
	return false
}

// Part of the io-fs-err-not-exist shim in docs/bridge_debt.md. The filesystem
// sentinels, made once, so that every read of one is the same error.
func writeStubFilesystemSentinels(out *strings.Builder) {
	errorType := externalStubErrorInnerType()
	values := make([]string, len(stubFilesystemSentinels))
	for i, sentinel := range stubFilesystemSentinels {
		values[i] = rustErrorFromValueFor("std::error::Error", NeedsConcurrentWrapper(), "std::io::Error::new(std::io::ErrorKind::"+sentinel.kind+", \""+sentinel.text+"\")")
	}
	list := strings.Join(values, ", ")
	n := len(stubFilesystemSentinels)
	if NeedsConcurrentWrapper() {
		fmt.Fprintf(out, "static __GO_FS_SENTINELS: std::sync::OnceLock<[%s; %d]> = std::sync::OnceLock::new();\n\n", errorType, n)
		fmt.Fprintf(out, "/// fs.ErrNotExist, ErrExist and ErrPermission, and os's names for them.\npub fn __go_fs_sentinel(index: usize) -> %s {\n    __GO_FS_SENTINELS.get_or_init(|| [%s])[index].clone()\n}\n", errorType, list)
		return
	}
	fmt.Fprintf(out, "thread_local! {\n    static __GO_FS_SENTINELS: [%s; %d] = [%s];\n}\n\n", errorType, n, list)
	fmt.Fprintf(out, "/// fs.ErrNotExist, ErrExist and ErrPermission, and os's names for them.\npub fn __go_fs_sentinel(index: usize) -> %s {\n    __GO_FS_SENTINELS.with(|__sentinels| __sentinels[index].clone())\n}\n", errorType)
}

func writeExternalPackageStubVariable(out *strings.Builder, pkgName string, varName string, rustType string) {
	out.WriteString("    pub fn ")
	out.WriteString(varName)
	out.WriteString("() -> ")
	out.WriteString(rustType)
	out.WriteString(" {\n")
	out.WriteString("        ")
	if index, ok := stubFilesystemSentinelIndex(pkgName, varName); ok {
		// TEMPORARY: io-fs-err-not-exist in docs/bridge_debt.md. Remove when the
		// source-transpiled io/fs sentinels are available to ordinary callers.
		// os.ErrNotExist is fs.ErrNotExist in Go, and so on for the others:
		// each read is the same error, so == compares them as Go does.
		out.WriteString(wrappedExternalStubExpr(externalStubErrorInnerType(), fmt.Sprintf("crate::__go_fs_sentinel(%d)", index)))
	} else {
		writeExternalStubDefaultValue(out, rustType)
	}
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
		writeExternalPackageStubVariable(out, "exec", varName, pkg.Variables[varName])
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
	// internal/bytealg supplies this entry point in assembly. Its stdlib callers
	// have already converted both arguments to ordinary Rust values here.
	if fn.ImportPath == "internal/bytealg" && funcName == "index_byte_string" {
		fmt.Fprintf(out, "    pub fn index_byte_string(s: impl AsRef<str>, c: u8) -> %s {\n", rustIntType())
		fmt.Fprintf(out, "        %s\n    }\n", rustIndexByteStringExpr("s.as_ref()", "c"))
		return
	}
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
	if pkgName == "runtime" {
		if body, ok := runtimeStubBodies[funcName]; ok && len(fn.ReturnTypes) <= 1 {
			writeRuntimeStubFunction(out, fn, funcName, body)
			return
		}
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
	writeExternalStubReturnValue(out, fn.ReturnTypes[0], rustIntType(), "std::thread::available_parallelism().map(|n| n.get() as i64).unwrap_or(1).max(1)")
	out.WriteString("\n    }\n")
}

// runtimeStubBodies are the bodies of runtime functions an ordinary program
// calls. runtime is in the native tier (stdlib_boundary.go), so these are
// its permanent implementation; without them the calls panicked. A
// goroutine is a thread, so Gosched yields the thread; memory is freed when
// its last handle goes, so GC has nothing to do; KeepAlive only takes its
// argument.
var runtimeStubBodies = map[string]string{
	"gosched":    "std::thread::yield_now();",
	"g_c":        "",
	"keep_alive": "",
	"num_c_p_u":  "std::thread::available_parallelism().map(|n| n.get() as i64).unwrap_or(1).max(1)",
}

// PERMANENT: not scaffold — runtime is in the native tier; see runtimeStubBodies.
func writeRuntimeStubFunction(out *strings.Builder, fn externalPackageStubFunction, funcName string, body string) {
	out.WriteString("    pub fn ")
	out.WriteString(funcName)
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
	out.WriteString(")")
	if len(fn.ReturnTypes) == 1 {
		out.WriteString(" -> ")
		writeExternalStubReturnType(out, fn.ReturnTypes)
		out.WriteString(" {\n        ")
		writeExternalStubReturnValue(out, fn.ReturnTypes[0], rustIntType(), body)
		out.WriteString("\n    }\n")
		return
	}
	out.WriteString(" {\n")
	if body != "" {
		out.WriteString("        ")
		out.WriteString(body)
		out.WriteString("\n")
	}
	out.WriteString("    }\n")
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
        let read = os_File { __go_data: data.clone(), __go_closed: closed.clone(), __go_wait_for_close: true, ..Default::default() };
        let write = os_File { __go_data: data, __go_closed: closed, __go_wait_for_close: true, ..Default::default() };
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

// TEMPORARY: exec-lookpath-path-search — source-transpiled os/exec still
// reaches an unimplemented native Eaccess hook and internal/godebug methods;
// remove this emitter when those paths compile and run. See bridge_debt.md.
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
			out.WriteString(": GoStringArg")
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
	errorType := rustErrorPayloadTypeFor("std::error::Error", NeedsConcurrentWrapper())
	out.WriteString(" {\n")
	out.WriteString("        let file = _arg0.into_go_string();\n")
	out.WriteString("        let file_path = std::path::PathBuf::from(&file);\n")
	out.WriteString("        let has_separator = {\n")
	out.WriteString("            #[cfg(unix)]\n")
	out.WriteString("            { file.as_bytes().contains(&b'/') }\n")
	out.WriteString("            #[cfg(windows)]\n")
	out.WriteString("            { file.contains('\\\\') || file.contains('/') || file.as_bytes().get(1) == Some(&b':') }\n")
	out.WriteString("            #[cfg(not(any(unix, windows)))]\n")
	out.WriteString("            { file_path.components().count() > 1 }\n")
	out.WriteString("        };\n")
	out.WriteString("        let candidates: std::vec::Vec<(std::path::PathBuf, bool)> = if has_separator {\n")
	out.WriteString("            std::vec![(file_path, false)]\n")
	out.WriteString("        } else {\n")
	out.WriteString("            let clean_path = |path: std::path::PathBuf| {\n")
	out.WriteString("                let mut cleaned = std::path::PathBuf::new();\n")
	out.WriteString("                for component in path.components() {\n")
	out.WriteString("                    match component {\n")
	out.WriteString("                        std::path::Component::CurDir => {}\n")
	out.WriteString("                        std::path::Component::ParentDir => match cleaned.components().next_back() {\n")
	out.WriteString("                            Some(std::path::Component::Normal(_)) => { cleaned.pop(); }\n")
	out.WriteString("                            Some(std::path::Component::ParentDir) | None => cleaned.push(\"..\"),\n")
	out.WriteString("                            _ => {}\n")
	out.WriteString("                        },\n")
	out.WriteString("                        other => cleaned.push(other.as_os_str()),\n")
	out.WriteString("                    }\n")
	out.WriteString("                }\n")
	out.WriteString("                cleaned\n")
	out.WriteString("            };\n")
	out.WriteString("            std::env::var_os(\"PATH\").map(|path| {\n")
	out.WriteString("                if path.is_empty() { return std::vec![]; }\n")
	out.WriteString("                std::env::split_paths(&path).map(|dir| {\n")
	out.WriteString("                    let relative = !dir.is_absolute();\n")
	out.WriteString("                    (clean_path(dir.join(&file)), relative)\n")
	out.WriteString("                }).collect()\n")
	out.WriteString("            }).unwrap_or_default()\n")
	out.WriteString("        };\n")
	out.WriteString("        for (candidate, from_relative_path) in candidates {\n")
	out.WriteString("            let Ok(metadata) = std::fs::metadata(&candidate) else { continue; };\n")
	out.WriteString("            if !metadata.is_file() { continue; }\n")
	out.WriteString("            #[cfg(target_os = \"linux\")]\n")
	out.WriteString("            {\n")
	out.WriteString("                use std::os::unix::ffi::OsStrExt;\n")
	out.WriteString("                let Ok(path) = std::ffi::CString::new(candidate.as_os_str().as_bytes()) else { continue; };\n")
	out.WriteString("                unsafe extern \"C\" {\n")
	out.WriteString("                    fn faccessat(dirfd: i32, path: *const std::os::raw::c_char, mode: i32, flags: i32) -> i32;\n")
	out.WriteString("                    fn __errno_location() -> *mut i32;\n")
	out.WriteString("                }\n")
	out.WriteString("                if unsafe { faccessat(-100, path.as_ptr(), 1, 0x200) } != 0 {\n")
	out.WriteString("                    let errno = unsafe { *__errno_location() };\n")
	out.WriteString("                    if errno != 1 && errno != 38 { continue; }\n")
	out.WriteString("                    use std::os::unix::fs::PermissionsExt;\n")
	out.WriteString("                    if metadata.permissions().mode() & 0o111 == 0 { continue; }\n")
	out.WriteString("                }\n")
	out.WriteString("            }\n")
	out.WriteString("            #[cfg(all(unix, not(target_os = \"linux\")))]\n")
	out.WriteString("            {\n")
	out.WriteString("                use std::os::unix::fs::PermissionsExt;\n")
	out.WriteString("                if metadata.permissions().mode() & 0o111 == 0 { continue; }\n")
	out.WriteString("            }\n")
	out.WriteString("            let Some(candidate_text) = candidate.to_str() else {\n")
	out.WriteString("                return (")
	out.WriteString(wrappedExternalStubExpr("String", "String::new()"))
	out.WriteString(", ")
	out.WriteString(wrappedExternalStubExpr(errorType, rustErrorFromTextFor("std::error::Error", NeedsConcurrentWrapper(), "\"executable path is not valid UTF-8\"")))
	out.WriteString(");\n")
	out.WriteString("            };\n")
	out.WriteString("            if from_relative_path {\n")
	out.WriteString("                return (")
	out.WriteString(wrappedExternalStubExpr("String", "candidate_text.to_owned()"))
	out.WriteString(", ")
	out.WriteString(wrappedExternalStubExpr(errorType, rustErrorFromTextFor("std::error::Error", NeedsConcurrentWrapper(), "\"cannot run executable found relative to current directory\"")))
	out.WriteString(");\n")
	out.WriteString("            }\n")
	out.WriteString("            return (")
	out.WriteString(wrappedExternalStubExpr("String", "candidate_text.to_owned()"))
	out.WriteString(", ")
	out.WriteString(wrappedExternalStubNoneExpr(errorType))
	out.WriteString(");\n")
	out.WriteString("        }\n")
	out.WriteString("        (")
	writeExternalStubDefaultValue(out, fn.ReturnTypes[0])
	out.WriteString(", ")
	out.WriteString(wrappedExternalStubExpr(errorType, rustErrorFromTextFor("std::error::Error", NeedsConcurrentWrapper(), "\"executable file not found\"")))
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
		if rustTypeStartsWithErrorPayload(innerType) || strings.HasPrefix(innerType, "Box<dyn Any") || strings.HasPrefix(innerType, "Box<dyn std::any::Any") {
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

// embeddedRWMutexPromotedMethods returns the forwarders of an embedded
// sync.RWMutex's methods. The field is the bare GoRWMutex, so each one calls
// the helper's own method on it: t.RLock() on a struct embedding one is
// t.r_w_mutex.r_lock().
func embeddedRWMutexPromotedMethods(field *ast.Field, selected func(string) bool, existingRustNames map[string]bool) ([]externalPromotedMethod, bool) {
	if !isSyncRWMutexField(field) {
		return nil, false
	}
	named := types.Unalias(GetTypeInfo().GetType(field.Type)).(*types.Named)
	var promoted []externalPromotedMethod
	// The accessor is written whenever Go selects the mutex through this
	// field, even when every method name is already taken, as by forwarders
	// of methods of the same names promoted from deeper in the struct: the
	// calls on a struct variable reach the mutex through it.
	mutexSelected := false
	methodSet := types.NewMethodSet(types.NewPointer(named))
	for i := 0; i < methodSet.Len(); i++ {
		fn, ok := methodSet.At(i).Obj().(*types.Func)
		if !ok || !isEmbeddedRWMutexMethod(fn) || !selected(fn.Name()) {
			continue
		}
		mutexSelected = true
		rustMethodName := ToSnakeCase(fn.Name())
		if existingRustNames[rustMethodName] {
			continue
		}
		sig, ok := fn.Type().(*types.Signature)
		if !ok {
			continue
		}
		existingRustNames[rustMethodName] = true
		promoted = append(promoted, externalPromotedMethod{
			EmbeddedFieldName: ToSnakeCase(getEmbeddedFieldName(field.Type)),
			MethodName:        fn.Name(),
			RustMethodName:    rustMethodName,
			Func:              fn,
			Signature:         sig,
			RawEmbeddedField:  true,
		})
	}
	if mutexSelected && !existingRustNames[rwMutexAccessorName] {
		existingRustNames[rwMutexAccessorName] = true
		promoted = append(promoted, externalPromotedMethod{
			EmbeddedFieldName: ToSnakeCase(getEmbeddedFieldName(field.Type)),
			RustMethodName:    rwMutexAccessorName,
			RawEmbeddedField:  true,
			RWMutexAccessor:   true,
		})
	}
	return promoted, true
}

const rwMutexAccessorName = "__go_rw_mutex"

// promotedThroughField returns a check of whether Go selects a method name
// on the struct through its field at fieldIndex. With Outer{ *A; *B }, A
// embedding a struct that embeds sync.RWMutex and B embedding one directly,
// Outer.Lock is B's: the shallower one wins. The second result is false when
// go/types has no type for the struct, and the check then takes every name
// as selected.
func promotedThroughField(structType *ast.StructType, fieldIndex int) (func(string) bool, bool) {
	all := func(string) bool { return true }
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || structType == nil {
		return all, false
	}
	st, ok := types.Unalias(typeInfo.GetType(structType)).(*types.Struct)
	if !ok {
		return all, false
	}
	return func(name string) bool {
		obj, index, _ := types.LookupFieldOrMethod(st, true, typeInfo.pkg, name)
		_, isFunc := obj.(*types.Func)
		return isFunc && len(index) > 1 && index[0] == fieldIndex
	}, true
}

// rwMutexForwardersForSelection returns the sync.RWMutex forwarders as they
// are when go/types said which field each is promoted through, and with an
// unimplemented! body when it could not, so a wrong mutex is never locked.
func rwMutexForwardersForSelection(methods []externalPromotedMethod, known bool) []externalPromotedMethod {
	if known {
		return methods
	}
	for i := range methods {
		methods[i].Unimplemented = "type info required for promoting sync.RWMutex methods"
	}
	return methods
}

// embeddedStructRWMutexPromotedMethods returns forwarders, through an
// embedded struct of the package, of the sync.RWMutex methods that struct
// has by embedding one itself: with Registry embedding *Table and Table
// embedding sync.RWMutex, r.RLock() is Table's forwarder called on r's
// Table.
func embeddedStructRWMutexPromotedMethods(field *ast.Field, selected func(string) bool, existingRustNames map[string]bool) []externalPromotedMethod {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || field == nil {
		return nil
	}
	typ := typeInfo.GetType(field.Type)
	if ptr, ok := types.Unalias(typ).(*types.Pointer); ok {
		typ = ptr.Elem()
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() != typeInfo.pkg {
		return nil
	}
	if _, isStruct := named.Underlying().(*types.Struct); !isStruct {
		return nil
	}
	var promoted []externalPromotedMethod
	// The accessor is written whenever Go selects the mutex through this
	// field, even when every method name is already taken, as by forwarders
	// of methods of the same names promoted from deeper in the struct: the
	// calls on a struct variable reach the mutex through it.
	mutexSelected := false
	methodSet := types.NewMethodSet(types.NewPointer(named))
	for i := 0; i < methodSet.Len(); i++ {
		fn, ok := methodSet.At(i).Obj().(*types.Func)
		if !ok || !isEmbeddedRWMutexMethod(fn) || !selected(fn.Name()) {
			continue
		}
		mutexSelected = true
		rustMethodName := ToSnakeCase(fn.Name())
		if existingRustNames[rustMethodName] {
			continue
		}
		sig, ok := fn.Type().(*types.Signature)
		if !ok {
			continue
		}
		existingRustNames[rustMethodName] = true
		promoted = append(promoted, externalPromotedMethod{
			EmbeddedFieldName:      ToSnakeCase(getEmbeddedFieldName(field.Type)),
			MethodName:             fn.Name(),
			RustMethodName:         rustMethodName,
			Func:                   fn,
			Signature:              sig,
			ThroughRWMutexAccessor: true,
		})
	}
	if mutexSelected && !existingRustNames[rwMutexAccessorName] {
		existingRustNames[rwMutexAccessorName] = true
		promoted = append(promoted, externalPromotedMethod{
			EmbeddedFieldName: ToSnakeCase(getEmbeddedFieldName(field.Type)),
			RustMethodName:    rwMutexAccessorName,
			RWMutexAccessor:   true,
		})
	}
	return promoted
}

// isSyncRWMutexField reports whether field embeds a sync.RWMutex value.
func isSyncRWMutexField(field *ast.Field) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || field == nil {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(field.Type)).(*types.Named)
	return ok && named.Obj() != nil && named.Obj().Pkg() != nil && named.Obj().Pkg().Path() == "sync" && named.Obj().Name() == "RWMutex"
}
