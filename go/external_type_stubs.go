package main

import (
	"fmt"
	"go/ast"
	"go/types"
	"os"
	"path/filepath"
	"slices"
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
	EmbeddedFieldName string
	MethodName        string
	RustMethodName    string
	Signature         *types.Signature
	GenericArguments  bool
	MutableReceiver   bool
}

type externalPackageStub struct {
	Functions map[string]externalPackageStubFunction
	Constants map[string]string
	Variables map[string]string
}

type externalPackageStubFunction struct {
	ParamCount  int
	ReturnTypes []string
}

func RegisterExternalTypeStub(name string) {
	if name == "" {
		return
	}
	currentExternalTypeStubs()[name] = true
}

func RegisterExternalTypeStubNamed(named *types.Named, rustName string) {
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

func externalNamedIsInterface(named *types.Named) bool {
	if named == nil || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	if !isStdlibPackage(named.Obj().Pkg().Path()) {
		return false
	}
	_, ok := types.Unalias(named.Underlying()).(*types.Interface)
	return ok
}

func externalIntegerRustTypeForNamed(named *types.Named) (string, bool) {
	if named == nil || named.Obj() == nil || named.Obj().Pkg() == nil {
		return "", false
	}
	if !isStdlibPackage(named.Obj().Pkg().Path()) {
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
		return "u32", true
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
	if !isStdlibPackage(named.Obj().Pkg().Path()) {
		return "", false
	}
	switch underlying := types.Unalias(named.Underlying()).(type) {
	case *types.Slice:
		return goTypesTypeToRustWrapped(underlying), true
	default:
		return "", false
	}
}

func RegisterExternalTypeStubField(typeName string, fieldName string, fieldType types.Type) {
	if typeName == "" || fieldName == "" || fieldType == nil {
		return
	}
	RegisterExternalTypeStub(typeName)
	trackWrapperImports()
	fieldTypeRust := goTypesFieldTypeToRust(fieldType)
	fields := currentExternalTypeStubFields()
	if fields[typeName] == nil {
		fields[typeName] = make(map[string]string)
	}
	fields[typeName][fieldName] = fieldTypeRust
}

func goTypesFieldTypeToRust(t types.Type) string {
	if _, ok := types.Unalias(t).Underlying().(*types.Pointer); ok {
		return goTypesTypeToRust(t)
	}
	return goTypesTypeToRustWrapped(t)
}

func RegisterExternalTypeStubMethod(typeName string, methodName string, sig *types.Signature) {
	if typeName == "" || methodName == "" || sig == nil {
		return
	}
	RegisterExternalTypeStub(typeName)
	trackWrapperImports()
	method := externalTypeStubMethod{
		ParamCount: sig.Params().Len(),
	}
	results := sig.Results()
	for i := 0; i < results.Len(); i++ {
		method.ReturnTypes = append(method.ReturnTypes, goTypesReturnTypeToRust(results.At(i).Type()))
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
	if targetType == "" || sourceType == "" || targetType == sourceType {
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
	if !isStdlibPackage(named.Obj().Pkg().Path()) {
		return
	}
	if isKnownStdlibHelperType(named.Obj().Pkg().Path(), named.Obj().Name()) {
		return
	}
	field, ok := selection.Obj().(*types.Var)
	if !ok {
		return
	}
	RegisterExternalTypeStubField(goTypesNamedTypeToRust(named), ToSnakeCase(field.Name()), field.Type())
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
	if !isStdlibPackage(named.Obj().Pkg().Path()) {
		return
	}
	if isKnownStdlibHelperType(named.Obj().Pkg().Path(), named.Obj().Name()) && !knownStdlibHelperNeedsExternalMethodStub(named.Obj().Pkg().Path(), named.Obj().Name()) {
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
	if !isStdlibPackage(named.Obj().Pkg().Path()) {
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
	if !isStdlibPackage(named.Obj().Pkg().Path()) {
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

func collectExternalPromotedMethods(structDef *StructDef, existing map[string]bool) []externalPromotedMethod {
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
		methodSet := types.NewMethodSet(types.NewPointer(named))
		for i := 0; i < methodSet.Len(); i++ {
			fn, ok := methodSet.At(i).Obj().(*types.Func)
			if !ok || !fn.Exported() {
				continue
			}
			methodName := fn.Name()
			if existing[methodName] {
				continue
			}
			sig, ok := fn.Type().(*types.Signature)
			if !ok {
				continue
			}

			rustMethodName := ToSnakeCase(methodName)
			stubBacked := isStdlibPackage(named.Obj().Pkg().Path())
			if stubBacked {
				RegisterExternalTypeStubMethod(rustTypeName, rustMethodName, sig)
			}
			existing[methodName] = true
			promoted = append(promoted, externalPromotedMethod{
				EmbeddedFieldName: ToSnakeCase(getEmbeddedFieldName(field.Type)),
				MethodName:        methodName,
				RustMethodName:    rustMethodName,
				Signature:         sig,
				GenericArguments:  stubBacked,
				MutableReceiver:   !stubBacked && signatureHasPointerReceiver(sig),
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
	if isStdlibPackage(pkgPath) {
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
		RegisterExternalPackageStubConstant(pkgName, rustConstName(sel.Sel.Name), obj.Type())
	case *types.Var:
		RegisterExternalPackageStubVariable(pkgName, rustPackageGlobalName(sel.Sel.Name), obj.Type())
	}
}

func RegisterExternalPackageStubFunction(pkgName string, funcName string, sig *types.Signature) {
	if pkgName == "" || funcName == "" || sig == nil {
		return
	}
	trackWrapperImports()
	if pkgName == "ast" && funcName == "new_ident" {
		RegisterExternalTypeStubFieldByRustType("ast_Ident", "name", goTypesTypeToRustWrapped(types.Typ[types.String]))
	}
	if pkgName == "parser" && funcName == "parse_file" {
		RegisterExternalTypeStubFieldByRustType("ast_File", "imports", wrappedExternalStubType("Vec<"+wrappedExternalStubType("ast_ImportSpec")+">"))
		RegisterExternalTypeStubFieldByRustType("ast_File", "name", wrappedExternalStubType("ast_Ident"))
		RegisterExternalTypeStubFieldByRustType("ast_Ident", "name", goTypesTypeToRustWrapped(types.Typ[types.String]))
		RegisterExternalTypeStubFieldByRustType("ast_ImportSpec", "name", wrappedExternalStubType("ast_Ident"))
		RegisterExternalTypeStubFieldByRustType("ast_ImportSpec", "path", wrappedExternalStubType("ast_BasicLit"))
		RegisterExternalTypeStubFieldByRustType("ast_BasicLit", "value", goTypesTypeToRustWrapped(types.Typ[types.String]))
	}
	fn := externalPackageStubFunction{
		ParamCount: sig.Params().Len(),
	}
	results := sig.Results()
	for i := 0; i < results.Len(); i++ {
		fn.ReturnTypes = append(fn.ReturnTypes, goTypesReturnTypeToRust(results.At(i).Type()))
	}
	pkg := ensureExternalPackageStub(pkgName)
	pkg.Functions[funcName] = fn
}

func RegisterExternalTypeStubFieldByRustType(typeName string, fieldName string, fieldTypeRust string) {
	if typeName == "" || fieldName == "" || fieldTypeRust == "" {
		return
	}
	RegisterExternalTypeStub(typeName)
	trackWrapperImports()
	fields := currentExternalTypeStubFields()
	if fields[typeName] == nil {
		fields[typeName] = make(map[string]string)
	}
	fields[typeName][fieldName] = fieldTypeRust
}

func RegisterExternalPackageStubConstant(pkgName string, constName string, constType types.Type) {
	if pkgName == "" || constName == "" || constType == nil {
		return
	}
	pkg := ensureExternalPackageStub(pkgName)
	pkg.Constants[constName] = goTypesConstTypeToRust(constType)
}

func RegisterExternalPackageStubVariable(pkgName string, varName string, varType types.Type) {
	if pkgName == "" || varName == "" || varType == nil {
		return
	}
	trackWrapperImports()
	pkg := ensureExternalPackageStub(pkgName)
	pkg.Variables[varName] = goTypesReturnTypeToRust(varType)
}

func ensureExternalPackageStub(pkgName string) *externalPackageStub {
	stubs := currentExternalPackageStubs()
	if stubs[pkgName] == nil {
		stubs[pkgName] = &externalPackageStub{
			Functions: make(map[string]externalPackageStubFunction),
			Constants: make(map[string]string),
			Variables: make(map[string]string),
		}
	}
	if stubs[pkgName].Functions == nil {
		stubs[pkgName].Functions = make(map[string]externalPackageStubFunction)
	}
	if stubs[pkgName].Constants == nil {
		stubs[pkgName].Constants = make(map[string]string)
	}
	if stubs[pkgName].Variables == nil {
		stubs[pkgName].Variables = make(map[string]string)
	}
	return stubs[pkgName]
}

func IsExternalStdlibPackageVariableSelector(sel *ast.SelectorExpr) bool {
	_, _, ok := externalStdlibPackageSelector(sel)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	_, ok = typeInfo.info.Uses[sel.Sel].(*types.Var)
	return ok
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
	if !ok || !isStdlibPackage(pkgPath) {
		return "", "", false
	}
	ctx := GetTranspileContext()
	if ctx != nil && ctx.PackageMapping != nil {
		if _, hasCrate := ctx.PackageMapping[pkgPath]; hasCrate {
			return "", "", false
		}
	}
	return ident.Name, pkgPath, true
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
		return name == "WaitGroup" || name == "Mutex" || name == "Once"
	case "time":
		return name == "Time" || name == "Duration" || name == "Timer" || name == "Ticker"
	case "unsafe":
		return name == "Pointer"
	default:
		return false
	}
}

func stdlibHelperTypeAllowsInterfaceConversion(pkgPath string, name string) bool {
	return pkgPath == "os" && name == "File"
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

func WriteSharedStdlibStubCrate(workDir string, states []*PackageState) error {
	outputDir := filepath.Join(workDir, "vendor", sharedStdlibStubCrateName)
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		return fmt.Errorf("failed to create shared stdlib stub crate: %v", err)
	}

	mergedState := MergeExternalStubPackageStates(states...)
	var parts []string
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

	cargoToml := fmt.Sprintf(`[package]
name = "%s"
version = "0.1.0"
edition = "2021"

[lib]
name = "%s"
path = "lib.rs"
`, sharedStdlibStubCrateName, sharedStdlibStubCrateName)
	cargoPath := filepath.Join(outputDir, "Cargo.toml")
	if err := os.WriteFile(cargoPath, []byte(cargoToml), 0644); err != nil {
		return fmt.Errorf("failed to write shared stdlib stub Cargo.toml: %v", err)
	}

	return nil
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
	if len(uses) == 0 {
		return strings.Join(body, "\n")
	}
	return strings.Join(uses, "\n") + "\n\n" + strings.TrimLeft(strings.Join(body, "\n"), "\n")
}

func mergeHelperTracker(dst *HelperTracker, src *HelperTracker) {
	if dst == nil || src == nil {
		return
	}
	dst.needsFormatMap = dst.needsFormatMap || src.needsFormatMap
	dst.needsFormatSlice = dst.needsFormatSlice || src.needsFormatSlice
	dst.needsFormatNestedSlice = dst.needsFormatNestedSlice || src.needsFormatNestedSlice
	dst.needsFormatAny = dst.needsFormatAny || src.needsFormatAny
	dst.needsFormatAnySlice = dst.needsFormatAnySlice || src.needsFormatAnySlice
	dst.needsGoChannel = dst.needsGoChannel || src.needsGoChannel
	dst.needsWaitGroup = dst.needsWaitGroup || src.needsWaitGroup
	dst.needsGoMutex = dst.needsGoMutex || src.needsGoMutex
	dst.needsGoOnce = dst.needsGoOnce || src.needsGoOnce
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
				Functions: make(map[string]externalPackageStubFunction),
				Constants: make(map[string]string),
				Variables: make(map[string]string),
			}
			dst[pkgName] = dstStub
		}
		if dstStub.Functions == nil {
			dstStub.Functions = make(map[string]externalPackageStubFunction)
		}
		if dstStub.Constants == nil {
			dstStub.Constants = make(map[string]string)
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
	generateGoPtrKeyHelper(&out, "GoPtrKey")
	return out.String()
}

func generateExternalStubs(stubs map[string]bool, interfaceTypes map[string]bool, integerTypes map[string]string, tupleTypes map[string]string, fieldsByType map[string]map[string]string, methodsByType map[string]map[string]externalTypeStubMethod, conversions map[string]map[string]bool, packageStubs map[string]*externalPackageStub) string {
	if len(stubs) == 0 && len(conversions) == 0 && len(packageStubs) == 0 {
		return ""
	}
	names := make([]string, 0, len(stubs))
	for name := range stubs {
		names = append(names, name)
	}
	slices.Sort(names)

	var out strings.Builder
	if externalStubNeedsInterfaceHelper(names, interfaceTypes) {
		writeExternalInterfaceIdHelper(&out)
	}
	for i, name := range names {
		if i > 0 || out.Len() > 0 {
			out.WriteString("\n\n")
		}
		if name == "fs_FileInfo" {
			writeFsFileInfoStub(&out, name, methodsByType[name])
			continue
		}
		if name == "fs_DirEntry" {
			writeFsDirEntryStub(&out, name)
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
			if externalStubFieldsCanDeriveDebug(fields) {
				out.WriteString("#[derive(Debug, Clone, Default)]\n")
			} else {
				out.WriteString("#[derive(Clone, Default)]\n")
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
			writeExternalTypeStubMethod(&out, methodName, method)
		}
		out.WriteString("}\n")
	}
	writeExternalTypeStubConversions(&out, conversions, interfaceTypes)
	writeExternalPackageStubs(&out, packageStubs, integerTypes, len(names) > 0)
	return out.String()
}

func externalStubNeedsInterfaceHelper(names []string, interfaceTypes map[string]bool) bool {
	for _, name := range names {
		if interfaceTypes[name] {
			return true
		}
	}
	return false
}

func externalTypeStubHasErrorMethod(methods map[string]externalTypeStubMethod) bool {
	method, ok := methods["error"]
	return ok && len(method.ReturnTypes) == 1 && strings.Contains(method.ReturnTypes[0], "String")
}

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
	out.WriteString("        Self { __go_id: __go_next_external_interface_id(), __go_value: ")
	out.WriteString(newValue)
	out.WriteString(" }\n")
	out.WriteString("    }\n")
	out.WriteString("    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {\n")
	out.WriteString("        self.__go_value.as_ref().downcast_ref::<T>()\n")
	out.WriteString("    }\n")
	methodNames := make([]string, 0, len(methods))
	for methodName := range methods {
		methodNames = append(methodNames, methodName)
	}
	slices.Sort(methodNames)
	for _, methodName := range methodNames {
		writeExternalTypeStubMethod(out, methodName, methods[methodName])
	}
	out.WriteString("}\n\n")

	out.WriteString("impl Default for ")
	out.WriteString(name)
	out.WriteString(" {\n")
	out.WriteString("    fn default() -> Self {\n")
	out.WriteString("        Self { __go_id: 0, __go_value: ")
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
				out.WriteString("        Self { __go_id: _value.__go_id, __go_value: _value.__go_value.clone() }\n")
			} else if interfaceTypes[targetName] {
				out.WriteString("        Self::__go_from(_value)\n")
			} else {
				out.WriteString("        Self::default()\n")
			}
			out.WriteString("    }\n")
			out.WriteString("}\n")
		}
	}
}

func writeExternalInterfaceIdHelper(out *strings.Builder) {
	out.WriteString("fn __go_next_external_interface_id() -> usize {\n")
	out.WriteString("    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);\n")
	out.WriteString("    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)\n")
	out.WriteString("}\n\n")
}

func writeExternalTypeStubDowncastMethod(out *strings.Builder) {
	out.WriteString("    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {\n")
	out.WriteString("        None\n")
	out.WriteString("    }\n")
}

func writeExternalTypeStubMethod(out *strings.Builder, methodName string, method externalTypeStubMethod) {
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
	if len(method.ReturnTypes) > 0 {
		out.WriteString("        ")
		if len(method.ReturnTypes) > 1 {
			out.WriteString("(")
		}
		for i, returnType := range method.ReturnTypes {
			if i > 0 {
				out.WriteString(", ")
			}
			writeExternalStubDefaultValue(out, returnType)
		}
		if len(method.ReturnTypes) > 1 {
			out.WriteString(")")
		}
		out.WriteString("\n")
	}
	out.WriteString("    }\n")
}

func writeExternalPackageStubs(out *strings.Builder, packageStubs map[string]*externalPackageStub, integerTypes map[string]string, needsSeparator bool) {
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
		if pkgName == "ast" {
			writeAstPackageStub(out, pkg, integerTypes)
			continue
		}
		if pkgName == "flag" {
			writeFlagPackageStub(out)
			continue
		}
		if pkgName == "os" {
			writeOsPackageStub(out, pkg, integerTypes)
			continue
		}
		if pkgName == "parser" {
			writeParserPackageStub(out, pkg, integerTypes)
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
			writeExternalStubConstDefaultValue(out, pkg.Constants[constName], integerTypes)
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
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName])
		}
		out.WriteString("}\n")
	}
}

func wrappedExternalStubType(innerType string) string {
	return fmt.Sprintf("%s<%s<Option<%s>>>", GetOuterWrapperType(), GetInnerWrapperType(), innerType)
}

func wrappedExternalStubExpr(innerType string, expr string) string {
	return fmt.Sprintf("%s::new(%s::new(Some::<%s>(%s)))", GetOuterWrapperType(), GetInnerWrapperType(), innerType, expr)
}

func wrappedExternalStubSomeExpr(innerType string, expr string) string {
	return wrappedExternalStubExpr(innerType, expr)
}

func wrappedExternalStubNoneExpr(innerType string) string {
	return fmt.Sprintf("%s::new(%s::new(None::<%s>))", GetOuterWrapperType(), GetInnerWrapperType(), innerType)
}

func writeFsFileInfoStub(out *strings.Builder, name string, methods map[string]externalTypeStubMethod) {
	boolType := wrappedExternalStubType("bool")
	stringType := wrappedExternalStubType("String")
	int64Type := wrappedExternalStubType("i64")

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
    pub fn size(&self) -> %s {
        %s
    }
    pub fn is_dir(&self) -> %s {
        %s
    }
`, name, name, name, name, stringType, wrappedExternalStubExpr("String", "self.name.clone()"), int64Type, wrappedExternalStubExpr("i64", "self.size"), boolType, wrappedExternalStubExpr("bool", "self.is_dir"))
	methodNames := make([]string, 0, len(methods))
	for methodName := range methods {
		if methodName == "name" || methodName == "size" || methodName == "is_dir" {
			continue
		}
		methodNames = append(methodNames, methodName)
	}
	slices.Sort(methodNames)
	for _, methodName := range methodNames {
		writeExternalTypeStubMethod(out, methodName, methods[methodName])
	}
	out.WriteString("}\n")
}

func writeFsDirEntryStub(out *strings.Builder, name string) {
	boolType := wrappedExternalStubType("bool")
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
    pub fn is_dir(&self) -> %s {
        %s
    }
}
`, name, name, name, name, stringType, wrappedExternalStubExpr("String", "self.name.clone()"), boolType, wrappedExternalStubExpr("bool", "self.is_dir"))
}

func writeAstPackageStub(out *strings.Builder, pkg *externalPackageStub, integerTypes map[string]string) {
	out.WriteString("pub mod ast {\n")
	out.WriteString("    use super::*;\n\n")
	writeGoStringArgTrait(out)

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
		writeExternalStubConstDefaultValue(out, pkg.Constants[constName], integerTypes)
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
		if funcName == "new_ident" {
			writeAstNewIdentFunction(out, pkg.Functions[funcName])
		} else {
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName])
		}
	}
	out.WriteString("}\n")
}

func writeAstNewIdentFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn new_ident<T0: GoStringArg>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        ")
	out.WriteString(wrappedExternalStubExpr("ast_Ident", "ast_Ident { name: "+wrappedExternalStubExpr("String", "_arg0.into_go_string()")+", ..Default::default() }"))
	out.WriteString("\n")
	out.WriteString("    }\n")
}

func writeParserPackageStub(out *strings.Builder, pkg *externalPackageStub, integerTypes map[string]string) {
	out.WriteString("pub mod parser {\n")
	out.WriteString("    use super::*;\n\n")
	writeParserArgTraits(out)

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
		writeExternalStubConstDefaultValue(out, pkg.Constants[constName], integerTypes)
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
		if funcName == "parse_file" {
			writeParserParseFileFunction(out, pkg.Functions[funcName])
		} else {
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName])
		}
	}
	out.WriteString("}\n")
}

func writeParserArgTraits(out *strings.Builder) {
	outerWrapper := GetOuterWrapperType()
	innerWrapper := GetInnerWrapperType()
	borrow := "borrow()"
	if NeedsConcurrentWrapper() {
		borrow = "lock().unwrap()"
	}
	fmt.Fprintf(out, `    pub trait GoParserFilenameArg {
        fn into_go_parser_filename(self) -> String;
    }

    impl GoParserFilenameArg for String {
        fn into_go_parser_filename(self) -> String {
            self
        }
    }

    impl<'a> GoParserFilenameArg for &'a str {
        fn into_go_parser_filename(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoParserFilenameArg for &'a String {
        fn into_go_parser_filename(self) -> String {
            self.clone()
        }
    }

    impl GoParserFilenameArg for %s<%s<Option<String>>> {
        fn into_go_parser_filename(self) -> String {
            self.%s.as_ref().cloned().unwrap_or_default()
        }
    }

    pub trait GoParserSourceArg {
        fn into_go_parser_source(self, filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>>;
    }

    impl GoParserSourceArg for () {
        fn into_go_parser_source(self, filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            std::fs::read_to_string(filename).map_err(|err| Box::new(err) as Box<dyn StdError + Send + Sync>)
        }
    }

    impl GoParserSourceArg for String {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            Ok(self)
        }
    }

    impl<'a> GoParserSourceArg for &'a str {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            Ok(self.to_string())
        }
    }

    impl<'a> GoParserSourceArg for &'a String {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            Ok(self.clone())
        }
    }

    impl GoParserSourceArg for Vec<u8> {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            String::from_utf8(self).map_err(|err| Box::new(err) as Box<dyn StdError + Send + Sync>)
        }
    }

    impl GoParserSourceArg for %s<%s<Option<String>>> {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            Ok(self.%s.as_ref().cloned().unwrap_or_default())
        }
    }

    impl GoParserSourceArg for %s<%s<Option<Vec<u8>>>> {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
            let bytes = self.%s.as_ref().cloned().unwrap_or_default();
            String::from_utf8(bytes).map_err(|err| Box::new(err) as Box<dyn StdError + Send + Sync>)
        }
    }

`, outerWrapper, innerWrapper, borrow, outerWrapper, innerWrapper, borrow, outerWrapper, innerWrapper, borrow)
}

func writeParserParseFileFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString(`    fn go_parser_error(message: String) -> Box<dyn StdError + Send + Sync> {
        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, message))
    }

    fn go_parser_string(value: String) -> `)
	out.WriteString(wrappedExternalStubType("String"))
	out.WriteString(` {
        `)
	out.WriteString(wrappedExternalStubExpr("String", "value"))
	out.WriteString(`
    }

    fn go_parser_ident(name: String) -> `)
	out.WriteString(wrappedExternalStubType("ast_Ident"))
	out.WriteString(` {
        `)
	out.WriteString(wrappedExternalStubExpr("ast_Ident", "ast_Ident { name: go_parser_string(name), ..Default::default() }"))
	out.WriteString(`
    }

    fn go_parser_basic_lit(value: String) -> `)
	out.WriteString(wrappedExternalStubType("ast_BasicLit"))
	out.WriteString(` {
        `)
	out.WriteString(wrappedExternalStubExpr("ast_BasicLit", "ast_BasicLit { value: go_parser_string(value), ..Default::default() }"))
	out.WriteString(`
    }

    fn go_parser_import_spec(name: Option<String>, path: String) -> `)
	out.WriteString(wrappedExternalStubType("ast_ImportSpec"))
	out.WriteString(` {
        `)
	out.WriteString(wrappedExternalStubExpr("ast_ImportSpec", "ast_ImportSpec { name: name.map(go_parser_ident).unwrap_or_else(|| "+wrappedExternalStubNoneExpr("ast_Ident")+"), path: go_parser_basic_lit(path), ..Default::default() }"))
	out.WriteString(`
    }

    fn go_parser_is_ident_start(ch: char) -> bool {
        ch == '_' || ch.is_alphabetic()
    }

    fn go_parser_is_ident_continue(ch: char) -> bool {
        ch == '_' || ch.is_alphanumeric()
    }

    fn go_parser_tokens(source: &str) -> Vec<String> {
        let chars: Vec<char> = source.chars().collect();
        let mut tokens = Vec::new();
        let mut i = 0usize;
        while i < chars.len() {
            let ch = chars[i];
            if ch.is_whitespace() {
                i += 1;
                continue;
            }
            if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                i += 2;
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            }
            if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
                i += 2;
                while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                    i += 1;
                }
                i = (i + 2).min(chars.len());
                continue;
            }
            if ch == '"' {
                let start = i;
                i += 1;
                while i < chars.len() {
                    if chars[i] == '\\' {
                        i = (i + 2).min(chars.len());
                        continue;
                    }
                    if chars[i] == '"' {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                tokens.push(chars[start..i].iter().collect());
                continue;
            }
            if ch == char::from(96) {
                let start = i;
                i += 1;
                while i < chars.len() && chars[i] != char::from(96) {
                    i += 1;
                }
                if i < chars.len() {
                    i += 1;
                }
                tokens.push(chars[start..i].iter().collect());
                continue;
            }
            if go_parser_is_ident_start(ch) {
                let start = i;
                i += 1;
                while i < chars.len() && go_parser_is_ident_continue(chars[i]) {
                    i += 1;
                }
                tokens.push(chars[start..i].iter().collect());
                continue;
            }
            if ch == '(' || ch == ')' || ch == ';' || ch == '.' {
                tokens.push(ch.to_string());
            }
            i += 1;
        }
        tokens
    }

    fn go_parser_is_string_lit(token: &str) -> bool {
        token.starts_with('"') || token.starts_with(char::from(96))
    }

    fn go_parser_import_from_tokens(tokens: &[String], start: usize) -> Option<(`)
	out.WriteString(wrappedExternalStubType("ast_ImportSpec"))
	out.WriteString(`, usize)> {
        if start >= tokens.len() {
            return None;
        }
        if go_parser_is_string_lit(&tokens[start]) {
            return Some((go_parser_import_spec(None, tokens[start].clone()), start + 1));
        }
        if start + 1 < tokens.len() && go_parser_is_string_lit(&tokens[start + 1]) {
            return Some((go_parser_import_spec(Some(tokens[start].clone()), tokens[start + 1].clone()), start + 2));
        }
        None
    }

    fn go_parser_parse_file(source: &str) -> Result<ast_File, Box<dyn StdError + Send + Sync>> {
        let tokens = go_parser_tokens(source);
        let package_name = tokens
            .windows(2)
            .find_map(|pair| if pair[0] == "package" { Some(pair[1].clone()) } else { None })
            .ok_or_else(|| go_parser_error("missing package clause".to_string()))?;
        let mut imports = Vec::new();
        let mut i = 0usize;
        while i < tokens.len() {
            if tokens[i] != "import" {
                i += 1;
                continue;
            }
            i += 1;
            if i < tokens.len() && tokens[i] == "(" {
                i += 1;
                while i < tokens.len() && tokens[i] != ")" {
                    if let Some((spec, next)) = go_parser_import_from_tokens(&tokens, i) {
                        imports.push(spec);
                        i = next;
                    } else {
                        i += 1;
                    }
                }
                if i < tokens.len() && tokens[i] == ")" {
                    i += 1;
                }
                continue;
            }
            if let Some((spec, next)) = go_parser_import_from_tokens(&tokens, i) {
                imports.push(spec);
                i = next;
            }
        }
        Ok(ast_File {
            imports: `)
	out.WriteString(wrappedExternalStubExpr("Vec<"+wrappedExternalStubType("ast_ImportSpec")+">", "imports"))
	out.WriteString(`,
            name: go_parser_ident(package_name),
            ..Default::default()
        })
    }

    pub fn parse_file<T0, T1: GoParserFilenameArg, T2: GoParserSourceArg, T3>(_arg0: T0, _arg1: T1, _arg2: T2, _arg3: T3) -> `)
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(` {
        let filename = _arg1.into_go_parser_filename();
        let source = match _arg2.into_go_parser_source(&filename) {
            Ok(source) => source,
            Err(err) => return (`)
	out.WriteString(wrappedExternalStubNoneExpr("ast_File"))
	out.WriteString(`, `)
	out.WriteString(wrappedExternalStubSomeExpr("Box<dyn StdError + Send + Sync>", "err"))
	out.WriteString(`),
        };
        match go_parser_parse_file(&source) {
            Ok(file) => (`)
	out.WriteString(wrappedExternalStubExpr("ast_File", "file"))
	out.WriteString(`, `)
	out.WriteString(wrappedExternalStubNoneExpr("Box<dyn StdError + Send + Sync>"))
	out.WriteString(`),
            Err(err) => (`)
	out.WriteString(wrappedExternalStubNoneExpr("ast_File"))
	out.WriteString(`, `)
	out.WriteString(wrappedExternalStubSomeExpr("Box<dyn StdError + Send + Sync>", "err"))
	out.WriteString(`),
        }
    }
`)
}

func writeFlagPackageStub(out *strings.Builder) {
	outerWrapper := GetOuterWrapperType()
	innerWrapper := GetInnerWrapperType()
	stringFlagType := fmt.Sprintf("%s<%s<Option<String>>>", outerWrapper, innerWrapper)
	boolFlagType := fmt.Sprintf("%s<%s<Option<bool>>>", outerWrapper, innerWrapper)
	argsType := fmt.Sprintf("%s<%s<Option<Vec<String>>>>", outerWrapper, innerWrapper)
	borrowMut := ".borrow_mut()"
	if NeedsConcurrentWrapper() {
		borrowMut = ".lock().unwrap()"
	}

	fmt.Fprintf(out, `pub mod flag {
    use super::*;

    type StringFlag = %s;
    type BoolFlag = %s;

    thread_local! {
        static STRING_FLAGS: std::cell::RefCell<Vec<(String, StringFlag)>> = std::cell::RefCell::new(Vec::new());
        static BOOL_FLAGS: std::cell::RefCell<Vec<(String, BoolFlag)>> = std::cell::RefCell::new(Vec::new());
        static REMAINING_ARGS: std::cell::RefCell<Option<Vec<String>>> = std::cell::RefCell::new(None);
    }

    pub fn string<T0: Into<String>, T1: Into<String>, T2>(_name: T0, value: T1, _usage: T2) -> StringFlag {
        let name = _name.into();
        let handle = %s::new(%s::new(Some(value.into())));
        STRING_FLAGS.with(|flags| flags.borrow_mut().push((name, handle.clone())));
        handle
    }

    pub fn bool<T0: Into<String>, T2>(_name: T0, value: bool, _usage: T2) -> BoolFlag {
        let name = _name.into();
        let handle = %s::new(%s::new(Some(value)));
        BOOL_FLAGS.with(|flags| flags.borrow_mut().push((name, handle.clone())));
        handle
    }

    pub fn parse() {
        let argv: Vec<String> = std::env::args().skip(1).collect();
        let mut remaining = Vec::new();
        let mut index = 0usize;
        while index < argv.len() {
            let arg = argv[index].clone();
            if arg == "--" {
                remaining.extend(argv[index + 1..].iter().cloned());
                break;
            }
            if !arg.starts_with('-') || arg == "-" {
                remaining.push(arg);
                index += 1;
                continue;
            }

            let flag_text = arg.trim_start_matches('-');
            let (name, inline_value) = match flag_text.split_once('=') {
                Some((name, value)) => (name.to_string(), Some(value.to_string())),
                None => (flag_text.to_string(), None),
            };

            if set_bool_flag(&name, inline_value.as_deref().map(parse_bool_value).unwrap_or(true)) {
                index += 1;
                continue;
            }

            if has_string_flag(&name) {
                let value = if let Some(value) = inline_value {
                    value
                } else if index + 1 < argv.len() {
                    index += 1;
                    argv[index].clone()
                } else {
                    String::new()
                };
                set_string_flag(&name, value);
                index += 1;
                continue;
            }

            remaining.push(arg);
            index += 1;
        }

        REMAINING_ARGS.with(|args| *args.borrow_mut() = Some(remaining));
    }

    pub fn args() -> %s {
        let needs_parse = REMAINING_ARGS.with(|args| args.borrow().is_none());
        if needs_parse {
            parse();
        }
        %s::new(%s::new(Some(REMAINING_ARGS.with(|args| args.borrow().as_ref().cloned().unwrap_or_default()))))
    }

    fn has_string_flag(name: &str) -> bool {
        STRING_FLAGS.with(|flags| flags.borrow().iter().any(|(flag_name, _)| flag_name == name))
    }

    fn set_string_flag(name: &str, value: String) -> bool {
        let mut found = false;
        STRING_FLAGS.with(|flags| {
            for (flag_name, target) in flags.borrow().iter() {
                if flag_name == name {
                    *target%s = Some(value.clone());
                    found = true;
                }
            }
        });
        found
    }

    fn set_bool_flag(name: &str, value: bool) -> bool {
        let mut found = false;
        BOOL_FLAGS.with(|flags| {
            for (flag_name, target) in flags.borrow().iter() {
                if flag_name == name {
                    *target%s = Some(value);
                    found = true;
                }
            }
        });
        found
    }

    fn parse_bool_value(value: &str) -> bool {
        matches!(value, "1" | "t" | "T" | "true" | "TRUE" | "True" | "y" | "yes" | "on")
    }
}
`, stringFlagType, boolFlagType, outerWrapper, innerWrapper, outerWrapper, innerWrapper, argsType, outerWrapper, innerWrapper, borrowMut, borrowMut)
}

func writeOsPackageStub(out *strings.Builder, pkg *externalPackageStub, integerTypes map[string]string) {
	out.WriteString("pub mod os {\n")
	out.WriteString("    use super::*;\n")
	needsFilesystemHelpers := osPackageStubNeedsFilesystemHelpers(pkg)
	if needsFilesystemHelpers {
		out.WriteString("    use std::path::Path;\n\n")
		writeGoStringArgTrait(out)
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
		writeExternalStubConstDefaultValue(out, pkg.Constants[constName], integerTypes)
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
		} else if funcName == "read_dir" {
			writeOsReadDirFunction(out, pkg.Functions[funcName])
		} else if funcName == "stat" {
			writeOsStatFunction(out, pkg.Functions[funcName])
		} else {
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName])
		}
	}
	out.WriteString("}\n")
}

func osPackageStubNeedsFilesystemHelpers(pkg *externalPackageStub) bool {
	if pkg == nil {
		return false
	}
	_, needsStat := pkg.Functions["stat"]
	_, needsReadDir := pkg.Functions["read_dir"]
	return needsStat || needsReadDir
}

func writeOsErrorHelpers(out *strings.Builder) {
	errorType := wrappedExternalStubType("Box<dyn StdError>")
	if NeedsConcurrentWrapper() {
		errorType = wrappedExternalStubType("Box<dyn StdError + Send + Sync>")
	}
	fmt.Fprintf(out, `    type GoError = %s;

    fn no_error() -> GoError {
        %s::new(%s::new(None))
    }

    fn io_error(err: std::io::Error) -> GoError {
        %s::new(%s::new(Some(Box::new(err))))
    }

`, errorType, GetOuterWrapperType(), GetInnerWrapperType(), GetOuterWrapperType(), GetInnerWrapperType())
}

func writeOsExitFunction(out *strings.Builder) {
	out.WriteString("    pub fn exit<T0: Into<i32>>(_arg0: T0) {\n")
	out.WriteString("        std::process::exit(_arg0.into());\n")
	out.WriteString("    }\n")
}

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

func writeFilepathPackageStub(out *strings.Builder, pkg *externalPackageStub, integerTypes map[string]string) {
	out.WriteString("pub mod filepath {\n")
	out.WriteString("    use super::*;\n")
	out.WriteString("    use std::path::{Path, PathBuf};\n\n")
	writeGoStringArgTrait(out)
	writeFilepathJoinTrait(out)
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
			writeExternalStubConstDefaultValue(out, pkg.Constants[constName], integerTypes)
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
	for i, funcName := range funcNames {
		if i > 0 {
			out.WriteString("\n")
		}
		if funcName == "abs" {
			writeFilepathAbsFunction(out, pkg.Functions[funcName])
		} else if funcName == "base" {
			writeFilepathSingleStringFunction(out, "base", "Path::new(&path).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or(path)")
		} else if funcName == "clean" {
			writeFilepathSingleStringFunction(out, "clean", "normalize_path(PathBuf::from(path))")
		} else if funcName == "dir" {
			writeFilepathSingleStringFunction(out, "dir", "Path::new(&path).parent().map(|parent| parent.to_string_lossy().into_owned()).unwrap_or_else(|| \".\".to_string())")
		} else if funcName == "eval_symlinks" {
			writeFilepathEvalSymlinksFunction(out, pkg.Functions[funcName])
		} else if funcName == "is_abs" {
			writeFilepathIsAbsFunction(out)
		} else if funcName == "join" {
			writeFilepathJoinFunction(out)
		} else {
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName])
		}
	}
	out.WriteString("}\n")
}

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

func writeFilepathJoinTrait(out *strings.Builder) {
	out.WriteString(`    pub trait GoPathJoinArgs {
        fn into_path_parts(self) -> Vec<String>;
    }

    impl<T0: GoStringArg> GoPathJoinArgs for (T0,) {
        fn into_path_parts(self) -> Vec<String> {
            vec![self.0.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg> GoPathJoinArgs for (T0, T1) {
        fn into_path_parts(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string()]
        }
    }

    impl<T0: GoStringArg, T1: GoStringArg, T2: GoStringArg> GoPathJoinArgs for (T0, T1, T2) {
        fn into_path_parts(self) -> Vec<String> {
            vec![self.0.into_go_string(), self.1.into_go_string(), self.2.into_go_string()]
        }
    }

`)
}

func writeFilepathErrorHelpers(out *strings.Builder) {
	errorType := wrappedExternalStubType("Box<dyn StdError>")
	if NeedsConcurrentWrapper() {
		errorType = wrappedExternalStubType("Box<dyn StdError + Send + Sync>")
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

`, errorType, GetOuterWrapperType(), GetInnerWrapperType(), GetOuterWrapperType(), GetInnerWrapperType())
}

func writeFilepathSingleStringFunction(out *strings.Builder, funcName string, expr string) {
	out.WriteString("    pub fn ")
	out.WriteString(funcName)
	out.WriteString("<T0: GoStringArg>(_arg0: T0) -> ")
	out.WriteString(wrappedExternalStubType("String"))
	out.WriteString(" {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        ")
	out.WriteString(wrappedExternalStubExpr("String", expr))
	out.WriteString("\n")
	out.WriteString("    }\n")
}

func writeFilepathJoinFunction(out *strings.Builder) {
	out.WriteString("    pub fn join<T0: GoPathJoinArgs>(_arg0: T0) -> ")
	out.WriteString(wrappedExternalStubType("String"))
	out.WriteString(" {\n")
	out.WriteString("        let mut path = PathBuf::new();\n")
	out.WriteString("        for part in _arg0.into_path_parts() {\n")
	out.WriteString("            if !part.is_empty() {\n")
	out.WriteString("                path.push(part);\n")
	out.WriteString("            }\n")
	out.WriteString("        }\n")
	out.WriteString("        ")
	out.WriteString(wrappedExternalStubExpr("String", "path.to_string_lossy().into_owned()"))
	out.WriteString("\n")
	out.WriteString("    }\n")
}

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

func writeFilepathIsAbsFunction(out *strings.Builder) {
	out.WriteString("    pub fn is_abs<T0: GoStringArg>(_arg0: T0) -> ")
	out.WriteString(wrappedExternalStubType("bool"))
	out.WriteString(" {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        ")
	out.WriteString(wrappedExternalStubExpr("bool", "Path::new(&path).is_absolute()"))
	out.WriteString("\n")
	out.WriteString("    }\n")
}

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

func writeExternalPackageStubFunction(out *strings.Builder, funcName string, fn externalPackageStubFunction) {
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
	if len(fn.ReturnTypes) > 0 {
		out.WriteString(" -> ")
		writeExternalStubReturnType(out, fn.ReturnTypes)
	}
	out.WriteString(" {\n")
	if len(fn.ReturnTypes) > 0 {
		out.WriteString("        ")
		writeExternalStubReturnValues(out, fn.ReturnTypes)
		out.WriteString("\n")
	}
	out.WriteString("    }\n")
}

func externalStubFieldsCanDeriveDebug(fields map[string]string) bool {
	for _, fieldType := range fields {
		if strings.Contains(fieldType, "dyn Fn") {
			return false
		}
	}
	return true
}

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

func writeExternalStubReturnValues(out *strings.Builder, returnTypes []string) {
	if len(returnTypes) > 1 {
		out.WriteString("(")
	}
	for i, returnType := range returnTypes {
		if i > 0 {
			out.WriteString(", ")
		}
		writeExternalStubDefaultValue(out, returnType)
	}
	if len(returnTypes) > 1 {
		out.WriteString(")")
	}
}

func writeExternalStubDefaultValue(out *strings.Builder, rustType string) {
	outerWrapper := GetOuterWrapperType()
	innerWrapper := GetInnerWrapperType()
	wrappedPrefix := outerWrapper + "<" + innerWrapper + "<Option<"
	if strings.HasPrefix(rustType, wrappedPrefix) && strings.HasSuffix(rustType, ">>>") {
		innerType := strings.TrimSuffix(strings.TrimPrefix(rustType, wrappedPrefix), ">>>")
		if strings.HasPrefix(innerType, "Box<dyn StdError") || strings.HasPrefix(innerType, "Box<dyn Any") {
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

func writeExternalStubConstDefaultValue(out *strings.Builder, rustType string, integerTypes map[string]string) {
	if integerTypes[rustType] != "" {
		out.WriteString(rustType)
		out.WriteString("(0)")
		return
	}
	switch rustType {
	case "String":
		out.WriteString("String::new()")
	case "bool":
		out.WriteString("false")
	case "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "usize", "f32", "f64":
		out.WriteString("0")
	default:
		out.WriteString(rustType)
	}
}
