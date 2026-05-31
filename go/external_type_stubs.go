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
	if !canDefineExternalTypeStub(name) {
		return
	}
	if currentExternalTypeStubs()[name] {
		return
	}
	currentExternalTypeStubs()[name] = true
	// The types_Basic stub written by writeTypesBasicStub references the
	// types_BasicKind and types_BasicInfo integer stubs by name. Keep the
	// bridge internally consistent: any path that registers types_Basic
	// must also register the integer types its fields refer to.
	if name == "types_Basic" {
		RegisterExternalIntegerTypeStub("types_BasicKind", "i32")
		RegisterExternalIntegerTypeStub("types_BasicInfo", "i32")
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

func RegisterExternalTypeStubField(typeName string, fieldName string, fieldType types.Type) {
	if !canDefineExternalTypeStub(typeName) || fieldName == "" || fieldType == nil {
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
	if !canDefineExternalTypeStub(typeName) || methodName == "" || sig == nil {
		return
	}
	RegisterExternalTypeStub(typeName)
	trackWrapperImports()
	if typeName == "types_Config" && methodName == "check" {
		registerTypesCheckBridgeSurface()
	}
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

// TEMPORARY: this registers a hand-written Rust shim for go/types.Config.Check.
// The long-term fix is to transpile go/types itself. Do not grow this surface
// to cover *types.Named, *types.Pointer, *types.Map, etc. — each new bridge
// entry is a step toward writing a Rust port of the Go stdlib, which is what
// the transpiler exists to avoid. See AGENTS.md "Strategy:
// Transpile stdlib, don't bridge it". Any addition here MUST panic loudly on
// unsupported paths and carry its own TEMPORARY comment with a removal plan.
func registerTypesCheckBridgeSurface() {
	TrackImport("BTreeMap")
	RegisterExternalTypeStub("types_Info")
	RegisterExternalTypeStub("types_TypeAndValue")
	RegisterExternalTypeStubInterface("types_Type")
	RegisterExternalTypeStub("types_Basic")
	RegisterExternalIntegerTypeStub("types_BasicKind", "i32")
	RegisterExternalIntegerTypeStub("types_BasicInfo", "i32")
	RegisterExternalTypeStub("types_Package")
	RegisterExternalTypeStub("constant_Value")

	RegisterExternalTypeStubFieldByRustType("types_Info", "types", wrappedExternalStubType("BTreeMap<ast_Expr, "+wrappedExternalStubType("types_TypeAndValue")+">"))
	RegisterExternalTypeStubFieldByRustType("types_TypeAndValue", "r#type", wrappedExternalStubType("types_Type"))
	RegisterExternalTypeStubFieldByRustType("types_TypeAndValue", "value", wrappedExternalStubType("constant_Value"))
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
	if !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
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

func collectExternalPromotedMethods(structDef *StructDef, existingRustNames map[string]bool) []externalPromotedMethod {
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
		RegisterExternalPackageStubConstant(pkgName, rustConstName(sel.Sel.Name), obj.Type())
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
	case "bytes":
		if sel.Sel.Name != "NewBuffer" {
			return
		}
		RegisterExternalTypeStub("bytes_Buffer")
		fn.ReturnTypes = []string{wrappedExternalStubType("bytes_Buffer")}
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
		switch sel.Sel.Name {
		case "NewPackage":
			RegisterExternalTypeStub("types_Package")
			fn.ReturnTypes = []string{wrappedExternalStubType("types_Package")}
		case "NewChecker":
			RegisterExternalTypeStub("token_FileSet")
			RegisterExternalTypeStub("types_Info")
			RegisterExternalTypeStub("types_Package")
			RegisterExternalTypeStub("types_Checker")
			fn.ReturnTypes = []string{wrappedExternalStubType("types_Checker")}
		default:
			return
		}
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
	if pkgName == "ast" && funcName == "new_ident" {
		RegisterExternalTypeStubFieldByRustType("ast_Ident", "name", goTypesTypeToRustWrapped(types.Typ[types.String]))
	}
	if pkgName == "parser" && funcName == "parse_file" {
		registerParserParseFileStubSurface()
	}
	if pkgName == "build" {
		RegisterExternalTypeStubFieldByRustType("build_Context", "g_o_r_o_o_t", goTypesTypeToRustWrapped(types.Typ[types.String]))
		RegisterExternalTypeStubFieldByRustType("build_Package", "dir", goTypesTypeToRustWrapped(types.Typ[types.String]))
		RegisterExternalTypeStubFieldByRustType("build_Package", "goroot", goTypesTypeToRustWrapped(types.Typ[types.Bool]))
		RegisterExternalTypeStubFieldByRustType("build_Package", "import_path", goTypesTypeToRustWrapped(types.Typ[types.String]))
		RegisterExternalTypeStubFieldByRustType("build_Package", "pkg_obj", goTypesTypeToRustWrapped(types.Typ[types.String]))
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

// TEMPORARY: hand-written Rust shim surface for go/parser.ParseFile.
// Long-term fix: transpile go/parser source.
func registerParserParseFileStubSurface() {
	stringType := goTypesTypeToRustWrapped(types.Typ[types.String])
	boolType := goTypesTypeToRustWrapped(types.Typ[types.Bool])
	tokenType := wrappedExternalStubType("token_Token")
	posType := wrappedExternalStubType("token_Pos")
	exprType := wrappedExternalStubType("ast_Expr")
	stmtType := wrappedExternalStubType("ast_Stmt")
	declType := wrappedExternalStubType("ast_Decl")
	identType := wrappedExternalStubType("ast_Ident")
	fieldListType := wrappedExternalStubType("ast_FieldList")
	blockType := wrappedExternalStubType("ast_BlockStmt")
	callType := wrappedExternalStubType("ast_CallExpr")
	funcType := wrappedExternalStubType("ast_FuncType")
	basicLitType := wrappedExternalStubType("ast_BasicLit")
	chanDirType := wrappedExternalStubType("ast_ChanDir")
	vec := func(elem string) string {
		return wrappedExternalStubType("Vec<" + elem + ">")
	}
	vecWrapped := func(elem string) string {
		return vec(wrappedExternalStubType(elem))
	}

	RegisterExternalIntegerTypeStub("token_Pos", "i32")
	RegisterExternalIntegerTypeStub("token_Token", "i32")
	RegisterExternalIntegerTypeStub("ast_ChanDir", "i32")
	RegisterExternalTypeStubInterface("ast_Expr")
	RegisterExternalTypeStubInterface("ast_Stmt")
	RegisterExternalTypeStubInterface("ast_Decl")
	RegisterExternalTypeStubInterface("ast_Spec")
	RegisterExternalTypeStub("ast_EmptyStmt")

	RegisterExternalTypeStubFieldByRustType("ast_ArrayType", "elt", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_ArrayType", "len", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_AssignStmt", "lhs", vec("ast_Expr"))
	RegisterExternalTypeStubFieldByRustType("ast_AssignStmt", "rhs", vec("ast_Expr"))
	RegisterExternalTypeStubFieldByRustType("ast_AssignStmt", "tok", tokenType)
	RegisterExternalTypeStubFieldByRustType("ast_BasicLit", "kind", tokenType)
	RegisterExternalTypeStubFieldByRustType("ast_BasicLit", "pos", posType)
	RegisterExternalTypeStubFieldByRustType("ast_BasicLit", "value", stringType)
	RegisterExternalTypeStubFieldByRustType("ast_BinaryExpr", "op", tokenType)
	RegisterExternalTypeStubFieldByRustType("ast_BinaryExpr", "x", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_BinaryExpr", "y", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_BlockStmt", "lbrace", posType)
	RegisterExternalTypeStubFieldByRustType("ast_BlockStmt", "list", vec("ast_Stmt"))
	RegisterExternalTypeStubFieldByRustType("ast_BranchStmt", "label", identType)
	RegisterExternalTypeStubFieldByRustType("ast_BranchStmt", "tok", tokenType)
	RegisterExternalTypeStubFieldByRustType("ast_CallExpr", "args", vec("ast_Expr"))
	RegisterExternalTypeStubFieldByRustType("ast_CallExpr", "ellipsis", posType)
	RegisterExternalTypeStubFieldByRustType("ast_CallExpr", "fun", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_CaseClause", "body", vec("ast_Stmt"))
	RegisterExternalTypeStubFieldByRustType("ast_CaseClause", "colon", posType)
	RegisterExternalTypeStubFieldByRustType("ast_CaseClause", "list", vec("ast_Expr"))
	RegisterExternalTypeStubFieldByRustType("ast_ChanType", "dir", chanDirType)
	RegisterExternalTypeStubFieldByRustType("ast_ChanType", "value", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_CommClause", "body", vec("ast_Stmt"))
	RegisterExternalTypeStubFieldByRustType("ast_CommClause", "comm", stmtType)
	RegisterExternalTypeStubFieldByRustType("ast_CompositeLit", "elts", vec("ast_Expr"))
	RegisterExternalTypeStubFieldByRustType("ast_CompositeLit", "r#type", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_DeclStmt", "decl", declType)
	RegisterExternalTypeStubFieldByRustType("ast_DeferStmt", "call", callType)
	RegisterExternalTypeStubFieldByRustType("ast_Ellipsis", "elt", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_ExprStmt", "x", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_Field", "names", vecWrapped("ast_Ident"))
	RegisterExternalTypeStubFieldByRustType("ast_Field", "r#type", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_Field", "tag", basicLitType)
	RegisterExternalTypeStubFieldByRustType("ast_FieldList", "list", vecWrapped("ast_Field"))
	RegisterExternalTypeStubFieldByRustType("ast_File", "decls", vec("ast_Decl"))
	RegisterExternalTypeStubFieldByRustType("ast_File", "__go_filename", stringType)
	RegisterExternalTypeStubFieldByRustType("ast_File", "__go_source", stringType)
	RegisterExternalTypeStubFieldByRustType("ast_File", "imports", vecWrapped("ast_ImportSpec"))
	RegisterExternalTypeStubFieldByRustType("ast_File", "name", identType)
	RegisterExternalTypeStubFieldByRustType("ast_ForStmt", "body", blockType)
	RegisterExternalTypeStubFieldByRustType("ast_ForStmt", "cond", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_ForStmt", "init", stmtType)
	RegisterExternalTypeStubFieldByRustType("ast_ForStmt", "post", stmtType)
	RegisterExternalTypeStubFieldByRustType("ast_FuncDecl", "body", blockType)
	RegisterExternalTypeStubFieldByRustType("ast_FuncDecl", "name", identType)
	RegisterExternalTypeStubFieldByRustType("ast_FuncDecl", "recv", fieldListType)
	RegisterExternalTypeStubFieldByRustType("ast_FuncDecl", "r#type", funcType)
	RegisterExternalTypeStubFieldByRustType("ast_FuncLit", "body", blockType)
	RegisterExternalTypeStubFieldByRustType("ast_FuncLit", "r#type", funcType)
	RegisterExternalTypeStubFieldByRustType("ast_FuncType", "params", fieldListType)
	RegisterExternalTypeStubFieldByRustType("ast_FuncType", "results", fieldListType)
	RegisterExternalTypeStubFieldByRustType("ast_GenDecl", "specs", vec("ast_Spec"))
	RegisterExternalTypeStubFieldByRustType("ast_GenDecl", "tok", tokenType)
	RegisterExternalTypeStubFieldByRustType("ast_GoStmt", "call", callType)
	RegisterExternalTypeStubFieldByRustType("ast_Ident", "__go_pos", "i32")
	RegisterExternalTypeStubFieldByRustType("ast_Ident", "name", stringType)
	RegisterExternalTypeStubFieldByRustType("ast_IfStmt", "body", blockType)
	RegisterExternalTypeStubFieldByRustType("ast_IfStmt", "cond", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_IfStmt", "init", stmtType)
	RegisterExternalTypeStubFieldByRustType("ast_IfStmt", "r#else", stmtType)
	RegisterExternalTypeStubFieldByRustType("ast_ImportSpec", "name", identType)
	RegisterExternalTypeStubFieldByRustType("ast_ImportSpec", "path", basicLitType)
	RegisterExternalTypeStubFieldByRustType("ast_IncDecStmt", "tok", tokenType)
	RegisterExternalTypeStubFieldByRustType("ast_IncDecStmt", "x", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_IndexExpr", "index", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_IndexExpr", "x", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_IndexListExpr", "indices", vec("ast_Expr"))
	RegisterExternalTypeStubFieldByRustType("ast_IndexListExpr", "x", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_InterfaceType", "methods", fieldListType)
	RegisterExternalTypeStubFieldByRustType("ast_KeyValueExpr", "key", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_KeyValueExpr", "value", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_LabeledStmt", "label", identType)
	RegisterExternalTypeStubFieldByRustType("ast_LabeledStmt", "stmt", stmtType)
	RegisterExternalTypeStubFieldByRustType("ast_MapType", "key", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_MapType", "value", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_ParenExpr", "x", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_RangeStmt", "body", blockType)
	RegisterExternalTypeStubFieldByRustType("ast_RangeStmt", "key", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_RangeStmt", "tok", tokenType)
	RegisterExternalTypeStubFieldByRustType("ast_RangeStmt", "value", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_RangeStmt", "x", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_ReturnStmt", "results", vec("ast_Expr"))
	RegisterExternalTypeStubFieldByRustType("ast_SelectStmt", "body", blockType)
	RegisterExternalTypeStubFieldByRustType("ast_SelectorExpr", "sel", identType)
	RegisterExternalTypeStubFieldByRustType("ast_SelectorExpr", "x", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_SendStmt", "chan", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_SendStmt", "value", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_SliceExpr", "high", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_SliceExpr", "low", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_SliceExpr", "max", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_SliceExpr", "slice3", boolType)
	RegisterExternalTypeStubFieldByRustType("ast_SliceExpr", "x", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_StarExpr", "x", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_StructType", "fields", fieldListType)
	RegisterExternalTypeStubFieldByRustType("ast_SwitchStmt", "body", blockType)
	RegisterExternalTypeStubFieldByRustType("ast_SwitchStmt", "init", stmtType)
	RegisterExternalTypeStubFieldByRustType("ast_SwitchStmt", "tag", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_TypeAssertExpr", "r#type", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_TypeAssertExpr", "x", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_TypeSpec", "assign", posType)
	RegisterExternalTypeStubFieldByRustType("ast_TypeSpec", "name", identType)
	RegisterExternalTypeStubFieldByRustType("ast_TypeSpec", "r#type", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_TypeSwitchStmt", "assign", stmtType)
	RegisterExternalTypeStubFieldByRustType("ast_TypeSwitchStmt", "body", blockType)
	RegisterExternalTypeStubFieldByRustType("ast_TypeSwitchStmt", "init", stmtType)
	RegisterExternalTypeStubFieldByRustType("ast_UnaryExpr", "op", tokenType)
	RegisterExternalTypeStubFieldByRustType("ast_UnaryExpr", "x", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_ValueSpec", "names", vecWrapped("ast_Ident"))
	RegisterExternalTypeStubFieldByRustType("ast_ValueSpec", "r#type", exprType)
	RegisterExternalTypeStubFieldByRustType("ast_ValueSpec", "values", vec("ast_Expr"))
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
	fields[typeName][fieldName] = fieldTypeRust
}

func RegisterExternalPackageStubConstant(pkgName string, constName string, constType types.Type) {
	if pkgName == "" || constName == "" || constType == nil {
		return
	}
	if pkgName == "token" {
		RegisterExternalTypeStub("token_Token")
	}
	if pkgName == "ast" && goTypesConstTypeToRust(constType) == "ast_ChanDir" {
		RegisterExternalTypeStub("ast_ChanDir")
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
	dst.needsFormatNestedSlice = dst.needsFormatNestedSlice || src.needsFormatNestedSlice
	dst.needsFormatNestedSliceWrapped = dst.needsFormatNestedSliceWrapped || src.needsFormatNestedSliceWrapped
	dst.needsFormatAny = dst.needsFormatAny || src.needsFormatAny
	dst.needsFormatAnySlice = dst.needsFormatAnySlice || src.needsFormatAnySlice
	dst.needsAnyEq = dst.needsAnyEq || src.needsAnyEq
	dst.needsAnyClone = dst.needsAnyClone || src.needsAnyClone
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
func writeJsonSupportHelpers(out *strings.Builder, hasBytesBuffer bool) {
	outerWrapper := GetOuterWrapperType()
	innerWrapper := GetInnerWrapperType()
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
	if hasBytesBuffer {
		fmt.Fprintf(out, `impl GoJsonInputArg for bytes_Buffer {
    fn into_go_json_bytes(self) -> Vec<u8> {
        self.__go_bytes()
    }
}

`)
	}
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
	if packageStubs["token"] != nil {
		stubs["token_Token"] = true
	}
	if packageStubs["parser"] != nil {
		stubs["token_Token"] = true
	}
	if packageStubs["ast"] != nil {
		for _, typ := range packageStubs["ast"].Constants {
			if typ == "ast_ChanDir" {
				stubs["ast_ChanDir"] = true
				break
			}
		}
	}
	needsJsonSupport := usePackageExternalStubs() || externalStubsNeedJsonSupport(stubs, packageStubs)
	if len(stubs) == 0 && len(conversions) == 0 && len(packageStubs) == 0 && !needsJsonSupport {
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
	if needsJsonSupport {
		writeJsonSupportHelpers(&out, stubs["bytes_Buffer"])
	}
	// writeTypesBridgeSupport emits helper traits and __go_types_check that
	// only callers of types.Config.Check use. Don't emit them just because
	// types.Config was named — the helpers reference ast_Expr/ast_File/etc.
	// which are only registered along the Check path.
	if methodsByType["types_Config"] != nil {
		if _, hasCheck := methodsByType["types_Config"]["check"]; hasCheck {
			writeTypesBridgeSupport(&out)
		}
	}
	for i, name := range names {
		if i > 0 || out.Len() > 0 {
			out.WriteString("\n\n")
		}
		if name == "json_Decoder" {
			writeJsonDecoderStub(&out)
			continue
		}
		if name == "bytes_Buffer" {
			writeBytesBufferStub(&out)
			continue
		}
		if name == "io_Writer" {
			writeIoWriterStub(&out, stubs["bytes_Buffer"], stubs["os_File"])
			continue
		}
		if name == "os_File" {
			writeOsFileStub(&out)
			continue
		}
		if name == "exec_Cmd" {
			writeExecCmdTypeStub(&out, fieldsByType[name])
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
		if name == "token_Token" {
			writeTokenTokenStub(&out)
			continue
		}
		if name == "fs_FileInfo" {
			writeFsFileInfoStub(&out, name, methodsByType[name])
			continue
		}
		if name == "fs_DirEntry" {
			writeFsDirEntryStub(&out, name)
			continue
		}
		if name == "types_Basic" {
			writeTypesBasicStub(&out, methodsByType[name])
			continue
		}
		if name == "types_Term" {
			writeTypesTermStub(&out, methodsByType[name])
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
			if name == "build_Context" && methodName == "import" {
				writeBuildContextImportMethod(&out, methodName, method)
			} else if name == "types_Config" && methodName == "check" {
				writeTypesConfigCheckMethod(&out, method)
			} else if name == "types_Checker" && methodName == "files" {
				writeTypesCheckerFilesMethod(&out, method)
			} else if name == "token_Pos" && methodName == "is_valid" {
				writeTokenPosIsValidMethod(&out)
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

// TEMPORARY: hand-written Rust shim for bytes.Buffer.
// Long-term fix: transpile bytes package source (pure Go, no runtime ties).
func writeBytesBufferStub(out *strings.Builder) {
	vecType := wrappedExternalStubType("Vec<u8>")
	stringType := wrappedExternalStubType("String")
	// Predeclared Copy scalar return slots stay bare to match the widened
	// signatures callers expect. `*Wrap` variants keep the legacy wrapped
	// shape for argument downcast positions whose accessor logic still
	// assumes a wrapped argument.
	intType := "i32"
	intWrap := wrappedExternalStubType("i32")
	int64Type := "i64"
	byteType := "u8"
	byteWrap := wrappedExternalStubType("u8")
	errorInnerType := externalStubErrorInnerType()
	errorType := wrappedExternalStubType(errorInnerType)
	noneError := wrappedExternalStubNoneExpr(errorInnerType)
	zeroInt := "0 as i32"
	zeroInt64 := "0 as i64"
	zeroByte := "0 as u8"
	emptyBytes := wrappedExternalStubExpr("Vec<u8>", "Vec::new()")
	emptyString := wrappedExternalStubExpr("String", "String::new()")
	vecBorrow := externalStubBorrowExpr("v")
	stringBorrow := externalStubBorrowExpr("v")
	byteBorrow := externalStubBorrowExpr("v")
	intBorrow := externalStubBorrowExpr("v")
	fmt.Fprintf(out, `#[derive(Debug, Clone)]
pub struct bytes_Buffer {
    pub __go_data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
}

impl Default for bytes_Buffer {
    fn default() -> Self {
        Self { __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())) }
    }
}

impl std::fmt::Display for bytes_Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__go_string())
    }
}

impl bytes_Buffer {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn __go_from_string(value: String) -> Self {
        Self { __go_data: std::sync::Arc::new(std::sync::Mutex::new(value.into_bytes())) }
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        self.__go_data.lock().unwrap().extend_from_slice(data);
    }

    pub fn __go_bytes(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    pub fn __go_string(&self) -> String {
        String::from_utf8_lossy(&self.__go_data.lock().unwrap()).into_owned()
    }

    pub fn string(&self) -> %s {
        %s
    }

    pub fn bytes(&self) -> %s {
        %s
    }

    pub fn len(&self) -> %s {
        %s
    }

    pub fn reset(&self) {
        self.__go_data.lock().unwrap().clear();
    }

    pub fn available(&self) -> %s {
        self.len()
    }

    pub fn available_buffer(&self) -> %s {
        %s
    }

    pub fn cap(&self) -> %s {
        self.len()
    }

    pub fn grow<T0>(&self, _arg0: T0) {
    }

    pub fn next<T0>(&self, _arg0: T0) -> %s {
        %s
    }

    pub fn read<T0>(&self, _arg0: T0) -> (%s, %s) {
        (%s, %s)
    }

    pub fn read_byte(&self) -> (%s, %s) {
        (%s, %s)
    }

    pub fn read_bytes<T0>(&self, _arg0: T0) -> (%s, %s) {
        (%s, %s)
    }

    pub fn read_from<T0>(&self, _arg0: T0) -> (%s, %s) {
        (%s, %s)
    }

    pub fn read_rune(&self) -> (%s, %s, %s) {
        (%s, %s, %s)
    }

    pub fn read_string<T0>(&self, _arg0: T0) -> (%s, %s) {
        (%s, %s)
    }

    pub fn truncate<T0>(&self, _arg0: T0) {
        self.reset();
    }

    pub fn unread_byte(&self) -> %s {
        %s
    }

    pub fn unread_rune(&self) -> %s {
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

    pub fn write_byte<T0: 'static>(&self, arg0: T0) -> %s {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<u8>() {
            *v
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<i32>() {
            *v as u8
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<%s>() {
            %s.as_ref().copied().unwrap_or_default()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<%s>() {
            %s.as_ref().copied().unwrap_or_default() as u8
        } else {
            0
        };
        self.__go_write_bytes(&[value]);
        %s
    }

    pub fn write_rune<T0: 'static>(&self, arg0: T0) -> (%s, %s) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<char>() {
            *v
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<i32>() {
            char::from_u32(*v as u32).unwrap_or('\0')
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<%s>() {
            char::from_u32(%s.as_ref().copied().unwrap_or_default() as u32).unwrap_or('\0')
        } else {
            '\0'
        };
        let mut encoded = [0u8; 4];
        let bytes = value.encode_utf8(&mut encoded).as_bytes().to_vec();
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (%s, %s)
    }

    pub fn write_to<T0>(&self, _arg0: T0) -> (%s, %s) {
        (%s, %s)
    }
}
`,
		stringType, wrappedExternalStubExpr("String", "self.__go_string()"),
		vecType, wrappedExternalStubExpr("Vec<u8>", "self.__go_bytes()"),
		intType, "self.__go_data.lock().unwrap().len() as i32",
		intType,
		vecType, emptyBytes,
		intType,
		vecType, emptyBytes,
		intType, errorType, zeroInt, noneError,
		byteType, errorType, zeroByte, noneError,
		vecType, errorType, emptyBytes, noneError,
		int64Type, errorType, zeroInt64, noneError,
		intType, intType, errorType, zeroInt, zeroInt, noneError,
		stringType, errorType, emptyString, noneError,
		errorType, noneError,
		errorType, noneError,
		intType, errorType,
		vecType, vecBorrow,
		"n", noneError,
		intType, errorType,
		stringType, stringBorrow,
		"n", noneError,
		errorType,
		byteWrap, byteBorrow,
		intWrap, intBorrow,
		noneError,
		intType, errorType,
		intWrap, intBorrow,
		"n", noneError,
		int64Type, errorType,
		"self.__go_data.lock().unwrap().len() as i64", noneError)
}

// TEMPORARY: hand-written Rust shim for io.Writer trait bridging.
// Long-term fix: transpile io package interfaces (pure Go).
func writeIoWriterStub(out *strings.Builder, hasBytesBuffer bool, hasOsFile bool) {
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
	out.WriteString(` }
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
`)
	if hasBytesBuffer {
		out.WriteString(`        if let Some(buffer) = self.downcast_ref::<bytes_Buffer>() {
            buffer.__go_write_bytes(data);
        }
`)
	}
	if hasOsFile {
		out.WriteString(`        if let Some(file) = self.downcast_ref::<os_File>() {
            file.__go_write_bytes(data);
        }
`)
	}
	if NeedsConcurrentWrapper() {
		out.WriteString(`        if let Some(builder) = self.downcast_ref::<Arc<Mutex<Option<String>>>>() {
            let mut guard = builder.lock().unwrap();
            guard.get_or_insert_with(String::new).push_str(&String::from_utf8_lossy(data));
        }
`)
	} else {
		out.WriteString(`        if let Some(builder) = self.downcast_ref::<Rc<RefCell<Option<String>>>>() {
            let mut guard = builder.borrow_mut();
            guard.get_or_insert_with(String::new).push_str(&String::from_utf8_lossy(data));
        }
`)
	}
	fmt.Fprintf(out, `    }

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

// TEMPORARY: hand-written Rust shim for go/token.Token enum.
// Long-term fix: transpile go/token source (pure Go).
func writeTokenTokenStub(out *strings.Builder) {
	stringType := wrappedExternalStubType("String")
	fmt.Fprintf(out, `#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct token_Token(pub i32);

impl PartialEq<i32> for token_Token {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<token_Token> for i32 {
    fn eq(&self, other: &token_Token) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for token_Token {
    type Output = token_Token;
    fn bitand(self, other: Self) -> token_Token {
        token_Token(self.0 & other.0)
    }
}

impl std::ops::BitOr for token_Token {
    type Output = token_Token;
    fn bitor(self, other: Self) -> token_Token {
        token_Token(self.0 | other.0)
    }
}

impl std::fmt::Display for token_Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", token_string_value(*self))
    }
}

fn token_string_value(tok: token_Token) -> &'static str {
    match tok.0 {
        4 => "IDENT",
        5 => "INT",
        6 => "FLOAT",
        7 => "IMAG",
        8 => "CHAR",
        9 => "STRING",
        12 => "+",
        13 => "-",
        14 => "*",
        15 => "/",
        16 => "%%",
        17 => "&",
        18 => "|",
        19 => "^",
        20 => "<<",
        21 => ">>",
        22 => "&^",
        23 => "+=",
        24 => "-=",
        25 => "*=",
        26 => "/=",
        27 => "%%=",
        28 => "&=",
        29 => "|=",
        30 => "^=",
        31 => "<<=",
        32 => ">>=",
        33 => "&^=",
        34 => "&&",
        35 => "||",
        36 => "<-",
        37 => "++",
        38 => "--",
        39 => "==",
        40 => "<",
        41 => ">",
        42 => "=",
        43 => "!",
        44 => "!=",
        45 => "<=",
        46 => ">=",
        47 => ":=",
        48 => "...",
        61 => "break",
        62 => "case",
        63 => "chan",
        64 => "const",
        65 => "continue",
        66 => "default",
        67 => "defer",
        68 => "else",
        69 => "fallthrough",
        70 => "for",
        71 => "func",
        72 => "go",
        73 => "goto",
        74 => "if",
        75 => "import",
        76 => "interface",
        77 => "map",
        78 => "package",
        79 => "range",
        80 => "return",
        81 => "select",
        82 => "struct",
        83 => "switch",
        84 => "type",
        85 => "var",
        88 => "~",
        _ => "ILLEGAL",
    }
}

impl token_Token {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn string(&self) -> %s {
        %s
    }
}
`, stringType, wrappedExternalStubExpr("String", "token_string_value(*self).to_string()"))
}

// PERMANENT: not scaffold — Rust std::fs::File is the long-term implementation;
// os.File wraps OS file handles, no transpilable Go source equivalent.
func writeOsFileStub(out *strings.Builder) {
	vecType := wrappedExternalStubType("Vec<u8>")
	stringType := wrappedExternalStubType("String")
	intType := wrappedExternalStubType("i32")
	errorInnerType := externalStubErrorInnerType()
	errorType := wrappedExternalStubType(errorInnerType)
	noneError := wrappedExternalStubNoneExpr(errorInnerType)
	vecBorrow := externalStubBorrowExpr("v")
	stringBorrow := externalStubBorrowExpr("v")
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

    pub fn __go_read_all_for_copy(&self) -> Vec<u8> {
        while self.__go_wait_for_close && !self.__go_closed.load(std::sync::atomic::Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        self.__go_read_all()
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
}
`,
		errorType, noneError,
		intType, errorType, vecType, vecBorrow, wrappedExternalStubExpr("i32", "n"), noneError,
		intType, errorType, stringType, stringBorrow, wrappedExternalStubExpr("i32", "n"), noneError,
		intType, errorType, wrappedExternalStubExpr("i32", "0"), noneError)
}

// PERMANENT: not scaffold — Rust std::process::Command is the long-term implementation;
// exec.Cmd is OS process layer, no transpilable Go source equivalent.
func writeExecCmdTypeStub(out *strings.Builder, fields map[string]string) {
	if fields == nil {
		fields = make(map[string]string)
	}
	if _, ok := fields["args"]; !ok {
		fields["args"] = wrappedExternalStubType("Vec<String>")
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
		if name == "types_Type" && methodName == "string" {
			writeTypesTypeStringMethod(out)
		} else if name == "types_Type" && methodName == "underlying" {
			writeTypesTypeUnderlyingMethod(out)
		} else if externalInterfaceCarriesSourcePos(name) && methodName == "pos" {
			writeExternalInterfacePosMethod(out)
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

// TEMPORARY: hand-written Rust bridge for go/types.Type. The long-term fix is
// to transpile the relevant `go/types` source rather than maintain a Rust
// shim — see AGENTS.md "Strategy: Transpile stdlib, don't
// bridge it". Until then, every unsupported kind MUST panic so gaps are
// visible in CI. Returning `String::new()` or `types_Type::default()` here
// would silently synthesize type facts and re-enact the 2026 fallback incident.
func writeTypesTypeStringMethod(out *strings.Builder) {
	out.WriteString("    pub fn string(&self) -> ")
	out.WriteString(wrappedExternalStubType("String"))
	out.WriteString(" {\n")
	out.WriteString("        if let Some(value) = self.downcast_ref::<types_Basic>() {\n")
	out.WriteString("            return ")
	out.WriteString(wrappedExternalStubExpr("String", "value.__go_name.clone()"))
	out.WriteString(";\n")
	out.WriteString("        }\n")
	out.WriteString("        panic!(\"types.Type.String() bridge: unsupported types.Type kind; transpile go/types instead of extending the bridge — see AGENTS.md\")\n")
	out.WriteString("    }\n")
}

// TEMPORARY: see writeTypesTypeStringMethod. Same rule applies.
func writeTypesTypeUnderlyingMethod(out *strings.Builder) {
	out.WriteString("    pub fn underlying(&self) -> ")
	out.WriteString(wrappedExternalStubType("types_Type"))
	out.WriteString(" {\n")
	out.WriteString("        if self.downcast_ref::<types_Basic>().is_some() {\n")
	out.WriteString("            return ")
	out.WriteString(wrappedExternalStubExpr("types_Type", "self.clone()"))
	out.WriteString(";\n")
	out.WriteString("        }\n")
	out.WriteString("        panic!(\"types.Type.Underlying() bridge: unsupported types.Type kind; transpile go/types instead of extending the bridge — see AGENTS.md\")\n")
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
// behavior get a custom emitter (see writeTypesTypeStringMethod,
// writeTypesConfigCheckMethod, etc.). Methods routed through this generic
// emitter exist for type-system completeness only — calling them at
// runtime is a bug to be fixed at the call site, either by adding a custom
// emitter or by removing the call.
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

// TEMPORARY: hand-written Rust shim for go/types.Basic.
// Long-term fix: transpile go/types source. Do not grow surface for *types.Named,
// *types.Pointer, *types.Map, etc. — see AGENTS.md "Strategy: Transpile stdlib, don't bridge it".
func writeTypesBasicStub(out *strings.Builder, methods map[string]externalTypeStubMethod) {
	out.WriteString(`#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Basic {
    pub __go_kind: types_BasicKind,
    pub __go_info: types_BasicInfo,
    pub __go_name: String,
}

impl std::fmt::Display for types_Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__go_name)
    }
}


impl types_Basic {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
`)
	methodNames := make([]string, 0, len(methods))
	for methodName := range methods {
		methodNames = append(methodNames, methodName)
	}
	slices.Sort(methodNames)
	for _, methodName := range methodNames {
		switch methodName {
		case "info":
			out.WriteString("    pub fn info(&self) -> ")
			out.WriteString(wrappedExternalStubType("types_BasicInfo"))
			out.WriteString(" {\n        ")
			out.WriteString(wrappedExternalStubExpr("types_BasicInfo", "self.__go_info"))
			out.WriteString("\n    }\n")
		case "kind":
			out.WriteString("    pub fn kind(&self) -> ")
			out.WriteString(wrappedExternalStubType("types_BasicKind"))
			out.WriteString(" {\n        ")
			out.WriteString(wrappedExternalStubExpr("types_BasicKind", "self.__go_kind"))
			out.WriteString("\n    }\n")
		case "name":
			out.WriteString("    pub fn name(&self) -> ")
			out.WriteString(wrappedExternalStubType("String"))
			out.WriteString(" {\n        ")
			out.WriteString(wrappedExternalStubExpr("String", "self.__go_name.clone()"))
			out.WriteString("\n    }\n")
		default:
			writeExternalTypeStubMethod(out, "types_Basic", methodName, methods[methodName])
		}
	}
	out.WriteString("}\n")
}

// TEMPORARY: hand-written Rust shim for go/types.Term.
// Long-term fix: transpile go/types source — see AGENTS.md
// "Strategy: Transpile stdlib, don't bridge it" and docs/bridge_debt.md
// (row: types-term).
//
// Term is `struct Term { tilde bool; typ Type }`. NewTerm constructs one;
// Type() returns the typ field. The shim mirrors that shape so the fixture
// stdlib_indexed_pointer_method runs without the bridge silently faking
// values. The companion `new_term` emitter is `writeTypesNewTermFunction`
// (called from `writeExternalPackageStubFunction`), and the trait
// `GoTypesTypeArg` that lets `new_term` accept either nil (Go) or a
// wrapped types_Type comes from this stub too so both are in scope.
func writeTypesTermStub(out *strings.Builder, methods map[string]externalTypeStubMethod) {
	typesTypeWrapped := wrappedExternalStubType("types_Type")
	typesTypeNone := wrappedExternalStubNoneExpr("types_Type")

	// Trait at top level so both this stub and the `pub mod types` body
	// (where `new_term` lives) can see it via `use super::*`. The trait
	// converts Go's untyped nil (lowered as `()`) and a wrapped types_Type
	// handle into a uniform wrapped-Option carrier.
	out.WriteString("pub trait GoTypesTypeArg {\n")
	out.WriteString("    fn __go_into_types_type_arg(self) -> ")
	out.WriteString(typesTypeWrapped)
	out.WriteString(";\n}\n\n")
	out.WriteString("impl GoTypesTypeArg for () {\n")
	out.WriteString("    fn __go_into_types_type_arg(self) -> ")
	out.WriteString(typesTypeWrapped)
	out.WriteString(" {\n        ")
	out.WriteString(typesTypeNone)
	out.WriteString("\n    }\n}\n\n")
	out.WriteString("impl GoTypesTypeArg for ")
	out.WriteString(typesTypeWrapped)
	out.WriteString(" {\n")
	out.WriteString("    fn __go_into_types_type_arg(self) -> ")
	out.WriteString(typesTypeWrapped)
	out.WriteString(" {\n        self\n    }\n}\n\n")

	out.WriteString("#[derive(Debug, Clone, Default)]\n")
	out.WriteString("pub struct types_Term {\n")
	out.WriteString("    pub tilde: bool,\n")
	out.WriteString("    pub typ: ")
	out.WriteString(typesTypeWrapped)
	out.WriteString(",\n}\n\n")

	out.WriteString("impl std::fmt::Display for types_Term {\n")
	out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
	out.WriteString("        write!(f, \"<types_Term>\")\n")
	out.WriteString("    }\n}\n\n")

	out.WriteString("impl types_Term {\n")
	out.WriteString("    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {\n")
	out.WriteString("        None\n")
	out.WriteString("    }\n")

	methodNames := make([]string, 0, len(methods))
	for methodName := range methods {
		methodNames = append(methodNames, methodName)
	}
	slices.Sort(methodNames)
	for _, methodName := range methodNames {
		switch methodName {
		case "r#type":
			out.WriteString("    pub fn r#type(&self) -> ")
			out.WriteString(typesTypeWrapped)
			out.WriteString(" {\n        self.typ.clone()\n    }\n")
		case "tilde":
			out.WriteString("    pub fn tilde(&self) -> ")
			out.WriteString(wrappedExternalStubType("bool"))
			out.WriteString(" {\n        ")
			out.WriteString(wrappedExternalStubExpr("bool", "self.tilde"))
			out.WriteString("\n    }\n")
		default:
			// Unknown methods route through the generic emitter, which
			// panics on call per AGENTS.md. If a fixture surfaces such a
			// call, add the case here with a real impl.
			writeExternalTypeStubMethod(out, "types_Term", methodName, methods[methodName])
		}
	}
	out.WriteString("}\n")
}

// TEMPORARY: hand-written Rust bridge support for go/types (Info traits, helpers).
// Long-term fix: transpile go/types source.
func writeTypesBridgeSupport(out *strings.Builder) {
	TrackImport("BTreeMap")
	stringType := wrappedExternalStubType("String")
	fileType := wrappedExternalStubType("ast_File")
	filesType := wrappedExternalStubType("Vec<" + fileType + ">")
	infoType := wrappedExternalStubType("types_Info")
	exprType := wrappedExternalStubType("ast_Expr")
	stmtType := wrappedExternalStubType("ast_Stmt")
	declType := wrappedExternalStubType("ast_Decl")
	specType := wrappedExternalStubType("ast_Spec")
	identType := wrappedExternalStubType("ast_Ident")
	fieldListType := wrappedExternalStubType("ast_FieldList")
	blockType := wrappedExternalStubType("ast_BlockStmt")
	callType := wrappedExternalStubType("ast_CallExpr")
	basicLitType := wrappedExternalStubType("ast_BasicLit")
	errorType := rustStdErrorBoxType()
	borrow := ".borrow()"
	borrowMut := ".borrow_mut()"
	if NeedsConcurrentWrapper() {
		borrow = ".lock().unwrap()"
		borrowMut = ".lock().unwrap()"
	}
	wrapExpr := func(innerType, expr string) string {
		return wrappedExternalStubExpr(innerType, expr)
	}
	errExpr := func(message string) string {
		return fmt.Sprintf("Box::new(std::io::Error::new(std::io::ErrorKind::Other, %s)) as %s", message, errorType)
	}

	fmt.Fprintf(out, `pub trait GoTypesBridgeStringArg {
    fn into_go_types_bridge_string(self) -> String;
}

impl GoTypesBridgeStringArg for String {
    fn into_go_types_bridge_string(self) -> String { self }
}

impl<'a> GoTypesBridgeStringArg for &'a str {
    fn into_go_types_bridge_string(self) -> String { self.to_string() }
}

impl<'a> GoTypesBridgeStringArg for &'a String {
    fn into_go_types_bridge_string(self) -> String { self.clone() }
}

impl GoTypesBridgeStringArg for %[1]s {
    fn into_go_types_bridge_string(self) -> String {
        self%[2]s.as_ref().cloned().unwrap_or_default()
    }
}

pub trait GoTypesBridgeInfoArg {
    fn apply_go_types_bridge_facts(self, type_facts: &[serde_json::Value], exprs_by_pos: &BTreeMap<i32, Vec<ast_Expr>>);
}

impl GoTypesBridgeInfoArg for () {
    fn apply_go_types_bridge_facts(self, _type_facts: &[serde_json::Value], _exprs_by_pos: &BTreeMap<i32, Vec<ast_Expr>>) {}
}

impl GoTypesBridgeInfoArg for %[6]s {
    fn apply_go_types_bridge_facts(self, type_facts: &[serde_json::Value], exprs_by_pos: &BTreeMap<i32, Vec<ast_Expr>>) {
        let mut info_guard = self%[7]s;
        if let Some(info_value) = info_guard.as_mut() {
            let mut types_guard = info_value.types%[7]s;
            if let Some(types_map) = types_guard.as_mut() {
                for fact in type_facts {
                    if fact.get("kind").and_then(|v| v.as_str()) != Some("basic") {
                        continue;
                    }
                    let pos = fact.get("pos").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
                    let Some(exprs) = exprs_by_pos.get(&pos) else { continue; };
                    let name = fact.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
                    let kind = fact.get("basicKind").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
                    let info_bits = fact.get("basicInfo").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
                    for expr in exprs {
                        types_map.insert(expr.clone(), %[8]s);
                    }
                }
            }
        }
    }
}

const __GO_TYPES_BRIDGE_HELPER: &str = r#"
package main

import (
	"encoding/json"
	"fmt"
	"go/ast"
	"go/importer"
	"go/parser"
	"go/token"
	"go/types"
	"os"
	"sort"
)

type request struct {
	Path  string `+"`json:\"path\"`"+`
	Files []file `+"`json:\"files\"`"+`
}

type file struct {
	Filename string `+"`json:\"filename\"`"+`
	Source   string `+"`json:\"source\"`"+`
}

type response struct {
	Package packageFact `+"`json:\"package\"`"+`
	Errors  []string    `+"`json:\"errors\"`"+`
	Types   []typeFact  `+"`json:\"types\"`"+`
}

type packageFact struct {
	Path string `+"`json:\"path\"`"+`
	Name string `+"`json:\"name\"`"+`
}

type typeFact struct {
	Pos       int    `+"`json:\"pos\"`"+`
	Kind      string `+"`json:\"kind\"`"+`
	Name      string `+"`json:\"name\"`"+`
	BasicKind int    `+"`json:\"basicKind\"`"+`
	BasicInfo int    `+"`json:\"basicInfo\"`"+`
}

func main() {
	var req request
	if err := json.NewDecoder(os.Stdin).Decode(&req); err != nil {
		_ = json.NewEncoder(os.Stdout).Encode(response{Errors: []string{err.Error()}})
		return
	}

	fset := token.NewFileSet()
	files := make([]*ast.File, 0, len(req.Files))
	for _, input := range req.Files {
		file, err := parser.ParseFile(fset, input.Filename, input.Source, parser.ParseComments|parser.SkipObjectResolution)
		if err != nil {
			_ = json.NewEncoder(os.Stdout).Encode(response{Errors: []string{err.Error()}})
			return
		}
		files = append(files, file)
	}

	info := &types.Info{
		Types: make(map[ast.Expr]types.TypeAndValue),
		Defs:  make(map[*ast.Ident]types.Object),
		Uses:  make(map[*ast.Ident]types.Object),
	}
	var errs []string
	config := &types.Config{
		Importer: importer.Default(),
		Error: func(err error) {
			errs = append(errs, err.Error())
		},
	}
	pkg, err := config.Check(req.Path, fset, files, info)
	if err != nil {
		msg := err.Error()
		if len(errs) == 0 || errs[len(errs)-1] != msg {
			errs = append(errs, msg)
		}
	}

	resp := response{Errors: errs}
	if pkg != nil {
		resp.Package = packageFact{Path: pkg.Path(), Name: pkg.Name()}
	}
	for expr, tv := range info.Types {
		if tv.Type == nil || expr == nil {
			continue
		}
		if basic, ok := types.Unalias(tv.Type).Underlying().(*types.Basic); ok {
			resp.Types = append(resp.Types, typeFact{
				Pos:       int(expr.Pos()),
				Kind:      "basic",
				Name:      basic.Name(),
				BasicKind: int(basic.Kind()),
				BasicInfo: int(basic.Info()),
			})
		}
	}
	sort.Slice(resp.Types, func(i, j int) bool {
		if resp.Types[i].Pos != resp.Types[j].Pos {
			return resp.Types[i].Pos < resp.Types[j].Pos
		}
		return resp.Types[i].Name < resp.Types[j].Name
	})
	if err := json.NewEncoder(os.Stdout).Encode(resp); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
"#;

fn __go_types_bridge_error(message: String) -> %[3]s {
    %[4]s
}

fn __go_types_basic_type(name: String, kind: i32, info: i32) -> types_Type {
    types_Type::__go_from(types_Basic {
        __go_kind: types_BasicKind(kind),
        __go_info: types_BasicInfo(info),
        __go_name: name,
    })
}

fn __go_types_config_check<T0: GoTypesBridgeStringArg, T3: GoTypesBridgeInfoArg>(
    path_arg: T0,
    files: %[5]s,
    info: T3,
) -> Result<types_Package, %[3]s> {
    let path = path_arg.into_go_types_bridge_string();
    let file_values = files%[2]s.as_ref().cloned().unwrap_or_default();
    let mut request_files = Vec::<serde_json::Value>::new();
    let mut exprs_by_pos = BTreeMap::<i32, Vec<ast_Expr>>::new();
    for file_handle in file_values {
        let file_guard = file_handle%[2]s;
        let Some(file) = file_guard.as_ref() else { continue; };
        let filename = file.__go_filename%[2]s.as_ref().cloned().unwrap_or_default();
        let source = file.__go_source%[2]s.as_ref().cloned().unwrap_or_default();
        if source.is_empty() {
            continue;
        }
        __go_types_collect_file_exprs(file, &mut exprs_by_pos);
        request_files.push(serde_json::json!({
            "filename": filename,
            "source": source,
        }));
    }
    if request_files.is_empty() {
        return Err(__go_types_bridge_error("go/types bridge requires parser.ParseFile source metadata".to_string()));
    }

    let request = serde_json::json!({
        "path": path,
        "files": request_files,
    });
    let output = __go_types_run_bridge_helper(&request.to_string())?;
    let response: serde_json::Value = serde_json::from_slice(&output)
        .map_err(|err| __go_types_bridge_error(format!("failed to decode go/types bridge response: {}", err)))?;
    if let Some(errors) = response.get("errors").and_then(|v| v.as_array()) {
        if !errors.is_empty() {
            let message = errors.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("; ");
            return Err(__go_types_bridge_error(message));
        }
    }

    if let Some(type_facts) = response.get("types").and_then(|v| v.as_array()) {
        info.apply_go_types_bridge_facts(type_facts, &exprs_by_pos);
    }

    Ok(types_Package::default())
}

fn __go_types_run_bridge_helper(request_json: &str) -> Result<Vec<u8>, %[3]s> {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let unique = format!(
        "go2rust-types-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default()
    );
    let dir = std::env::temp_dir().join(unique);
    std::fs::create_dir_all(&dir)
        .map_err(|err| __go_types_bridge_error(format!("failed to create go/types bridge dir: {}", err)))?;
    let helper_path = dir.join("main.go");
    std::fs::write(&helper_path, __GO_TYPES_BRIDGE_HELPER)
        .map_err(|err| __go_types_bridge_error(format!("failed to write go/types bridge helper: {}", err)))?;
    let mut child = Command::new("go")
        .arg("run")
        .arg(&helper_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| __go_types_bridge_error(format!("failed to launch go/types bridge helper: {}", err)))?;
    {
        let stdin = child.stdin.as_mut().ok_or_else(|| __go_types_bridge_error("failed to open go/types bridge stdin".to_string()))?;
        stdin.write_all(request_json.as_bytes())
            .map_err(|err| __go_types_bridge_error(format!("failed to write go/types bridge request: {}", err)))?;
    }
    let output = child.wait_with_output()
        .map_err(|err| __go_types_bridge_error(format!("failed to wait for go/types bridge helper: {}", err)))?;
    let _ = std::fs::remove_dir_all(&dir);
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(__go_types_bridge_error(format!("go/types bridge helper failed: {}", stderr)));
    }
    Ok(output.stdout)
}

fn __go_types_record_expr(exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>, expr: &ast_Expr) {
    if expr.__go_pos != 0 {
        exprs_by_pos.entry(expr.__go_pos).or_default().push(expr.clone());
    }
}

fn __go_types_collect_file_exprs(file: &ast_File, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    let decls = file.decls%[2]s.as_ref().cloned().unwrap_or_default();
    for decl in decls {
        __go_types_collect_decl_exprs(&decl, exprs_by_pos);
    }
}

fn __go_types_collect_decl_exprs(decl: &ast_Decl, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(value) = decl.downcast_ref::<ast_GenDecl>() {
        let specs = value.specs%[2]s.as_ref().cloned().unwrap_or_default();
        for spec in specs {
            __go_types_collect_spec_exprs(&spec, exprs_by_pos);
        }
    } else if let Some(value) = decl.downcast_ref::<ast_FuncDecl>() {
        __go_types_collect_opt_field_list(&value.recv, exprs_by_pos);
        __go_types_collect_func_type(&value.r#type, exprs_by_pos);
        __go_types_collect_opt_block(&value.body, exprs_by_pos);
    }
}

fn __go_types_collect_spec_exprs(spec: &ast_Spec, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(value) = spec.downcast_ref::<ast_ValueSpec>() {
        __go_types_collect_opt_expr(&value.r#type, exprs_by_pos);
        let values = value.values%[2]s.as_ref().cloned().unwrap_or_default();
        for expr in values {
            __go_types_collect_expr(&expr, exprs_by_pos);
        }
    } else if let Some(value) = spec.downcast_ref::<ast_TypeSpec>() {
        __go_types_collect_opt_expr(&value.r#type, exprs_by_pos);
    }
}

fn __go_types_collect_opt_expr(value: &%[9]s, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(expr) = value%[2]s.as_ref().cloned() {
        __go_types_collect_expr(&expr, exprs_by_pos);
    }
}

fn __go_types_collect_expr(expr: &ast_Expr, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    __go_types_record_expr(exprs_by_pos, expr);
    if let Some(value) = expr.downcast_ref::<ast_ArrayType>() {
        __go_types_collect_opt_expr(&value.len, exprs_by_pos);
        __go_types_collect_opt_expr(&value.elt, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_BinaryExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_expr(&value.y, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_CallExpr>() {
        __go_types_collect_call_expr(value, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_CompositeLit>() {
        __go_types_collect_opt_expr(&value.r#type, exprs_by_pos);
        let elts = value.elts%[2]s.as_ref().cloned().unwrap_or_default();
        for elt in elts {
            __go_types_collect_expr(&elt, exprs_by_pos);
        }
    } else if let Some(value) = expr.downcast_ref::<ast_IndexExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_expr(&value.index, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_IndexListExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        let indices = value.indices%[2]s.as_ref().cloned().unwrap_or_default();
        for index in indices {
            __go_types_collect_expr(&index, exprs_by_pos);
        }
    } else if let Some(value) = expr.downcast_ref::<ast_KeyValueExpr>() {
        __go_types_collect_opt_expr(&value.key, exprs_by_pos);
        __go_types_collect_opt_expr(&value.value, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_MapType>() {
        __go_types_collect_opt_expr(&value.key, exprs_by_pos);
        __go_types_collect_opt_expr(&value.value, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_ParenExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_SelectorExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_SliceExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_expr(&value.low, exprs_by_pos);
        __go_types_collect_opt_expr(&value.high, exprs_by_pos);
        __go_types_collect_opt_expr(&value.max, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_StarExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_TypeAssertExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_expr(&value.r#type, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_UnaryExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    }
}

fn __go_types_collect_call_expr(value: &ast_CallExpr, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    __go_types_collect_opt_expr(&value.fun, exprs_by_pos);
    let args = value.args%[2]s.as_ref().cloned().unwrap_or_default();
    for arg in args {
        __go_types_collect_expr(&arg, exprs_by_pos);
    }
}

fn __go_types_collect_opt_stmt(value: &%[10]s, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(stmt) = value%[2]s.as_ref().cloned() {
        __go_types_collect_stmt_exprs(&stmt, exprs_by_pos);
    }
}

fn __go_types_collect_stmt_exprs(stmt: &ast_Stmt, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(value) = stmt.downcast_ref::<ast_AssignStmt>() {
        let lhs = value.lhs%[2]s.as_ref().cloned().unwrap_or_default();
        let rhs = value.rhs%[2]s.as_ref().cloned().unwrap_or_default();
        for expr in lhs.into_iter().chain(rhs.into_iter()) {
            __go_types_collect_expr(&expr, exprs_by_pos);
        }
    } else if let Some(value) = stmt.downcast_ref::<ast_DeclStmt>() {
        __go_types_collect_opt_decl(&value.decl, exprs_by_pos);
    } else if let Some(value) = stmt.downcast_ref::<ast_ExprStmt>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    } else if let Some(value) = stmt.downcast_ref::<ast_ReturnStmt>() {
        let results = value.results%[2]s.as_ref().cloned().unwrap_or_default();
        for expr in results {
            __go_types_collect_expr(&expr, exprs_by_pos);
        }
    } else if let Some(value) = stmt.downcast_ref::<ast_IfStmt>() {
        __go_types_collect_opt_stmt(&value.init, exprs_by_pos);
        __go_types_collect_opt_expr(&value.cond, exprs_by_pos);
        __go_types_collect_opt_block(&value.body, exprs_by_pos);
        __go_types_collect_opt_stmt(&value.r#else, exprs_by_pos);
    } else if let Some(value) = stmt.downcast_ref::<ast_ForStmt>() {
        __go_types_collect_opt_stmt(&value.init, exprs_by_pos);
        __go_types_collect_opt_expr(&value.cond, exprs_by_pos);
        __go_types_collect_opt_stmt(&value.post, exprs_by_pos);
        __go_types_collect_opt_block(&value.body, exprs_by_pos);
    } else if let Some(value) = stmt.downcast_ref::<ast_RangeStmt>() {
        __go_types_collect_opt_expr(&value.key, exprs_by_pos);
        __go_types_collect_opt_expr(&value.value, exprs_by_pos);
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_block(&value.body, exprs_by_pos);
    }
}

fn __go_types_collect_opt_decl(value: &%[11]s, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(decl) = value%[2]s.as_ref().cloned() {
        __go_types_collect_decl_exprs(&decl, exprs_by_pos);
    }
}

fn __go_types_collect_opt_block(value: &%[12]s, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(block) = value%[2]s.as_ref() {
        let list = block.list%[2]s.as_ref().cloned().unwrap_or_default();
        for stmt in list {
            __go_types_collect_stmt_exprs(&stmt, exprs_by_pos);
        }
    }
}

fn __go_types_collect_func_type(value: &%[13]s, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(func_type) = value%[2]s.as_ref() {
        __go_types_collect_opt_field_list(&func_type.params, exprs_by_pos);
        __go_types_collect_opt_field_list(&func_type.results, exprs_by_pos);
    }
}

fn __go_types_collect_opt_field_list(value: &%[14]s, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(field_list) = value%[2]s.as_ref() {
        let fields = field_list.list%[2]s.as_ref().cloned().unwrap_or_default();
        for field in fields {
            let field_guard = field%[2]s;
            if let Some(field_value) = field_guard.as_ref() {
                __go_types_collect_opt_expr(&field_value.r#type, exprs_by_pos);
                __go_types_collect_opt_basic_lit(&field_value.tag, exprs_by_pos);
            }
        }
    }
}

fn __go_types_collect_opt_basic_lit(value: &%[15]s, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(lit) = value%[2]s.as_ref() {
        let lit_pos = lit.pos%[2]s.as_ref().map(|pos| pos.0).unwrap_or_default();
        if lit_pos != 0 {
            exprs_by_pos.entry(lit_pos).or_default().push(ast_Expr::__go_from_with_pos(lit.clone(), lit_pos));
        }
    }
}

`, stringType, borrow, errorType, errExpr("message"), filesType, infoType, borrowMut, wrapExpr("types_TypeAndValue", "types_TypeAndValue { r#type: "+wrapExpr("types_Type", "__go_types_basic_type(name.clone(), kind, info_bits)")+", value: Default::default() }"), exprType, stmtType, declType, blockType, wrappedExternalStubType("ast_FuncType"), fieldListType, basicLitType)
	_ = identType
	_ = callType
	_ = specType
}

// TEMPORARY: hand-written Rust shim for go/types.Config.Check.
// Long-term fix: transpile go/types source.
func writeTypesConfigCheckMethod(out *strings.Builder, method externalTypeStubMethod) {
	out.WriteString("    pub fn check")
	out.WriteString("<T0: GoTypesBridgeStringArg, T1, T3: GoTypesBridgeInfoArg>")
	out.WriteString("(&self")
	out.WriteString(", _arg0: T0, _arg1: T1, _arg2: ")
	out.WriteString(wrappedExternalStubType("Vec<" + wrappedExternalStubType("ast_File") + ">"))
	out.WriteString(", _arg3: T3")
	out.WriteString(")")
	if len(method.ReturnTypes) > 0 {
		out.WriteString(" -> ")
		writeExternalStubReturnType(out, method.ReturnTypes)
	}
	out.WriteString(" {\n")
	out.WriteString("        match __go_types_config_check(_arg0, _arg2, _arg3) {\n")
	out.WriteString("            Ok(pkg) => (")
	out.WriteString(wrappedExternalStubExpr("types_Package", "pkg"))
	out.WriteString(", ")
	out.WriteString(wrappedExternalStubNoneExpr(rustStdErrorBoxType()))
	out.WriteString("),\n")
	out.WriteString("            Err(err) => (")
	out.WriteString(wrappedExternalStubNoneExpr("types_Package"))
	out.WriteString(", ")
	out.WriteString(wrappedExternalStubSomeExpr(rustStdErrorBoxType(), "err"))
	out.WriteString("),\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

// TEMPORARY: hand-written Rust shim for go/types.Checker.Files.
// Long-term fix: transpile go/types source.
func writeTypesCheckerFilesMethod(out *strings.Builder, method externalTypeStubMethod) {
	out.WriteString("    pub fn files")
	writeExternalStubGenericParams(out, method.ParamCount)
	out.WriteString("(&self")
	writeExternalStubArgs(out, method.ParamCount)
	out.WriteString(")")
	if len(method.ReturnTypes) > 0 {
		out.WriteString(" -> ")
		writeExternalStubReturnType(out, method.ReturnTypes)
	}
	out.WriteString(" {\n")
	out.WriteString("        panic!(\"go/types Checker.Files is required for TypeInfo; generated stdlib stubs must not synthesize type information\")\n")
	out.WriteString("    }\n")
}

// MACHINERY: generic-parameter emitter for stub function signatures.
func writeExternalStubGenericParams(out *strings.Builder, count int) {
	if count <= 0 {
		return
	}
	out.WriteString("<")
	for i := 0; i < count; i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString("T")
		out.WriteString(strconv.Itoa(i))
	}
	out.WriteString(">")
}

// MACHINERY: argument-list emitter for stub function signatures.
func writeExternalStubArgs(out *strings.Builder, count int) {
	for i := 0; i < count; i++ {
		out.WriteString(", _arg")
		out.WriteString(strconv.Itoa(i))
		out.WriteString(": T")
		out.WriteString(strconv.Itoa(i))
	}
}

// TEMPORARY: hand-written Rust shim for go/token.Pos.IsValid.
// Long-term fix: transpile go/token source.
func writeTokenPosIsValidMethod(out *strings.Builder) {
	out.WriteString("    pub fn is_valid(&self) -> Arc<Mutex<Option<bool>>> {\n")
	out.WriteString("        Arc::new(Mutex::new(Some(self.0 != 0)))\n")
	out.WriteString("    }\n")
}

// TEMPORARY: hand-written Rust shim for go/build.Context import methods.
// Long-term fix: transpile go/build source.
func writeBuildContextImportMethod(out *strings.Builder, methodName string, method externalTypeStubMethod) {
	out.WriteString("    pub fn ")
	out.WriteString(methodName)
	out.WriteString("<T0: build::GoStringArg, T1, T2>(&self, _arg0: T0, _arg1: T1, _arg2: T2)")
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
	out.WriteString("        build::go_build_import_path(_arg0.into_go_string())\n")
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
		if pkgName == "ast" {
			writeAstPackageStub(out, pkg, integerTypes)
			continue
		}
		if pkgName == "build" {
			writeBuildPackageStub(out, pkg, integerTypes)
			continue
		}
		if pkgName == "exec" {
			writeExecPackageStub(out, pkg, integerTypes)
			continue
		}
		if pkgName == "flag" {
			writeFlagPackageStub(out)
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
		if pkgName == "parser" {
			writeParserPackageStub(out, pkg, integerTypes)
			continue
		}
		if pkgName == "strconv" {
			writeStrconvPackageStub(out, pkg, integerTypes, stubs)
			continue
		}
		if pkgName == "token" {
			writeTokenPackageStub(out, pkg, integerTypes)
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
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName], stubs)
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

func writeExternalStubReturnValue(out *strings.Builder, rustType string, innerType string, expr string) {
	if rustType == innerType {
		out.WriteString(expr)
		return
	}
	out.WriteString(wrappedExternalStubExpr(innerType, expr))
}

func wrappedExternalStubSomeExpr(innerType string, expr string) string {
	return wrappedExternalStubExpr(innerType, expr)
}

func wrappedExternalStubNoneExpr(innerType string) string {
	return fmt.Sprintf("%s::new(%s::new(None::<%s>))", GetOuterWrapperType(), GetInnerWrapperType(), innerType)
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
	methodNames := make([]string, 0, len(methods))
	for methodName := range methods {
		if methodName == "name" || methodName == "size" || methodName == "is_dir" {
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

// PERMANENT: not scaffold — io/fs.DirEntry is OS-tied; Rust std::fs::DirEntry is the long-term implementation.
func writeFsDirEntryStub(out *strings.Builder, name string) {
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
}
`, name, name, name, name, stringType, wrappedExternalStubExpr("String", "self.name.clone()"))
}

// TEMPORARY: hand-written Rust shim for go/ast package.
// Long-term fix: transpile go/ast source (pure Go).
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
		if pkg.Constants[constName] == "ast_ChanDir" && constName == "S_E_N_D" {
			out.WriteString("ast_ChanDir(1)")
		} else if pkg.Constants[constName] == "ast_ChanDir" && constName == "R_E_C_V" {
			out.WriteString("ast_ChanDir(2)")
		} else {
			writeExternalStubConstDefaultValue(out, pkg.Constants[constName], integerTypes)
		}
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
		if funcName == "inspect" {
			writeAstInspectFunction(out)
		} else if funcName == "new_ident" {
			writeAstNewIdentFunction(out, pkg.Functions[funcName])
		} else {
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName], nil)
		}
	}
	out.WriteString("}\n")
}

// TEMPORARY: hand-written Rust shim for ast.Inspect / Walk.
// Long-term fix: transpile go/ast source.
func writeAstInspectFunction(out *strings.Builder) {
	out.WriteString(`    type InspectCallback = Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<ast_Node>>>) -> Arc<Mutex<Option<bool>>> + Send + Sync>>>>;

    pub trait InspectRoot {
        fn inspect_root_node(&self) -> Option<ast_Node>;
    }

    impl InspectRoot for Arc<Mutex<Option<ast_Node>>> {
        fn inspect_root_node(&self) -> Option<ast_Node> {
            self.lock().unwrap().as_ref().cloned()
        }
    }

    impl InspectRoot for ast_Node {
        fn inspect_root_node(&self) -> Option<ast_Node> {
            Some(self.clone())
        }
    }

    impl InspectRoot for Arc<Mutex<Option<ast_Expr>>> {
        fn inspect_root_node(&self) -> Option<ast_Node> {
            self.lock().unwrap().as_ref().map(|value| ast_Node { __go_id: value.__go_id, __go_pos: value.__go_pos, __go_value: value.__go_value.clone() })
        }
    }

    impl InspectRoot for ast_Expr {
        fn inspect_root_node(&self) -> Option<ast_Node> {
            Some(ast_Node { __go_id: self.__go_id, __go_pos: self.__go_pos, __go_value: self.__go_value.clone() })
        }
    }

    impl InspectRoot for Arc<Mutex<Option<ast_Stmt>>> {
        fn inspect_root_node(&self) -> Option<ast_Node> {
            self.lock().unwrap().as_ref().map(|value| ast_Node { __go_id: value.__go_id, __go_pos: value.__go_pos, __go_value: value.__go_value.clone() })
        }
    }

    impl InspectRoot for ast_Stmt {
        fn inspect_root_node(&self) -> Option<ast_Node> {
            Some(ast_Node { __go_id: self.__go_id, __go_pos: self.__go_pos, __go_value: self.__go_value.clone() })
        }
    }

    impl InspectRoot for Arc<Mutex<Option<ast_Decl>>> {
        fn inspect_root_node(&self) -> Option<ast_Node> {
            self.lock().unwrap().as_ref().map(|value| ast_Node { __go_id: value.__go_id, __go_pos: value.__go_pos, __go_value: value.__go_value.clone() })
        }
    }

    impl InspectRoot for ast_Decl {
        fn inspect_root_node(&self) -> Option<ast_Node> {
            Some(ast_Node { __go_id: self.__go_id, __go_pos: self.__go_pos, __go_value: self.__go_value.clone() })
        }
    }

    impl InspectRoot for Arc<Mutex<Option<ast_Spec>>> {
        fn inspect_root_node(&self) -> Option<ast_Node> {
            self.lock().unwrap().as_ref().map(|value| ast_Node { __go_id: value.__go_id, __go_pos: value.__go_pos, __go_value: value.__go_value.clone() })
        }
    }

    impl InspectRoot for ast_Spec {
        fn inspect_root_node(&self) -> Option<ast_Node> {
            Some(ast_Node { __go_id: self.__go_id, __go_pos: self.__go_pos, __go_value: self.__go_value.clone() })
        }
    }

    macro_rules! inspect_root_struct {
        ($($ty:ty),* $(,)?) => {
            $(
                impl InspectRoot for Arc<Mutex<Option<$ty>>> {
                    fn inspect_root_node(&self) -> Option<ast_Node> {
                        self.lock().unwrap().as_ref().map(|value| ast_Node::__go_from(value.clone()))
                    }
                }

                impl InspectRoot for $ty {
                    fn inspect_root_node(&self) -> Option<ast_Node> {
                        Some(ast_Node::__go_from(self.clone()))
                    }
                }
            )*
        };
    }

    inspect_root_struct!(
        ast_ArrayType, ast_AssignStmt, ast_BasicLit, ast_BinaryExpr, ast_BlockStmt, ast_BranchStmt,
        ast_CallExpr, ast_CaseClause, ast_ChanType, ast_CommClause, ast_CompositeLit, ast_DeclStmt,
        ast_DeferStmt, ast_Ellipsis, ast_ExprStmt, ast_Field, ast_FieldList, ast_File, ast_ForStmt,
        ast_FuncDecl, ast_FuncLit, ast_FuncType, ast_GenDecl, ast_GoStmt, ast_Ident, ast_IfStmt,
        ast_ImportSpec, ast_IncDecStmt, ast_IndexExpr, ast_IndexListExpr, ast_InterfaceType,
        ast_KeyValueExpr, ast_LabeledStmt, ast_MapType, ast_ParenExpr, ast_RangeStmt, ast_ReturnStmt,
        ast_SelectStmt, ast_SelectorExpr, ast_SendStmt, ast_SliceExpr, ast_StarExpr, ast_StructType,
        ast_SwitchStmt, ast_TypeAssertExpr, ast_TypeSpec, ast_TypeSwitchStmt, ast_UnaryExpr,
        ast_ValueSpec
    );

    pub fn inspect<T0: InspectRoot>(root: T0, callback: InspectCallback) {
        if let Some(node) = root.inspect_root_node() {
            visit_node(&callback, node);
        }
    }

    fn call_inspect_callback(callback: &InspectCallback, node: ast_Node) -> bool {
        let mut guard = callback.lock().unwrap();
        match guard.as_mut() {
            Some(f) => f(Arc::new(Mutex::new(Some(node)))).lock().unwrap().as_ref().copied().unwrap_or(false),
            None => false,
        }
    }

    fn visit_node(callback: &InspectCallback, node: ast_Node) -> bool {
        if !call_inspect_callback(callback, node.clone()) {
            return true;
        }

        if let Some(value) = node.downcast_ref::<ast_ArrayType>() {
            visit_opt_expr(callback, &value.len);
            visit_opt_expr(callback, &value.elt);
        } else if let Some(value) = node.downcast_ref::<ast_AssignStmt>() {
            visit_expr_list(callback, &value.lhs);
            visit_expr_list(callback, &value.rhs);
        } else if let Some(value) = node.downcast_ref::<ast_BinaryExpr>() {
            visit_opt_expr(callback, &value.x);
            visit_opt_expr(callback, &value.y);
        } else if let Some(value) = node.downcast_ref::<ast_BlockStmt>() {
            visit_stmt_list(callback, &value.list);
        } else if let Some(value) = node.downcast_ref::<ast_BranchStmt>() {
            visit_opt_ident(callback, &value.label);
        } else if let Some(value) = node.downcast_ref::<ast_CallExpr>() {
            visit_opt_expr(callback, &value.fun);
            visit_expr_list(callback, &value.args);
        } else if let Some(value) = node.downcast_ref::<ast_CaseClause>() {
            visit_expr_list(callback, &value.list);
            visit_stmt_list(callback, &value.body);
        } else if let Some(value) = node.downcast_ref::<ast_ChanType>() {
            visit_opt_expr(callback, &value.value);
        } else if let Some(value) = node.downcast_ref::<ast_CommClause>() {
            visit_opt_stmt(callback, &value.comm);
            visit_stmt_list(callback, &value.body);
        } else if let Some(value) = node.downcast_ref::<ast_CompositeLit>() {
            visit_opt_expr(callback, &value.r#type);
            visit_expr_list(callback, &value.elts);
        } else if let Some(value) = node.downcast_ref::<ast_DeclStmt>() {
            visit_opt_decl(callback, &value.decl);
        } else if let Some(value) = node.downcast_ref::<ast_DeferStmt>() {
            visit_opt_call(callback, &value.call);
        } else if let Some(value) = node.downcast_ref::<ast_Ellipsis>() {
            visit_opt_expr(callback, &value.elt);
        } else if let Some(value) = node.downcast_ref::<ast_ExprStmt>() {
            visit_opt_expr(callback, &value.x);
        } else if let Some(value) = node.downcast_ref::<ast_Field>() {
            visit_ident_list(callback, &value.names);
            visit_opt_expr(callback, &value.r#type);
            visit_opt_basic_lit(callback, &value.tag);
        } else if let Some(value) = node.downcast_ref::<ast_FieldList>() {
            visit_field_list(callback, &value.list);
        } else if let Some(value) = node.downcast_ref::<ast_File>() {
            visit_opt_ident(callback, &value.name);
            visit_decl_list(callback, &value.decls);
        } else if let Some(value) = node.downcast_ref::<ast_ForStmt>() {
            visit_opt_stmt(callback, &value.init);
            visit_opt_expr(callback, &value.cond);
            visit_opt_stmt(callback, &value.post);
            visit_opt_block(callback, &value.body);
        } else if let Some(value) = node.downcast_ref::<ast_FuncDecl>() {
            visit_opt_field_list(callback, &value.recv);
            visit_opt_ident(callback, &value.name);
            visit_opt_func_type(callback, &value.r#type);
            visit_opt_block(callback, &value.body);
        } else if let Some(value) = node.downcast_ref::<ast_FuncLit>() {
            visit_opt_func_type(callback, &value.r#type);
            visit_opt_block(callback, &value.body);
        } else if let Some(value) = node.downcast_ref::<ast_FuncType>() {
            visit_opt_field_list(callback, &value.params);
            visit_opt_field_list(callback, &value.results);
        } else if let Some(value) = node.downcast_ref::<ast_GenDecl>() {
            visit_spec_list(callback, &value.specs);
        } else if let Some(value) = node.downcast_ref::<ast_GoStmt>() {
            visit_opt_call(callback, &value.call);
        } else if let Some(value) = node.downcast_ref::<ast_IfStmt>() {
            visit_opt_stmt(callback, &value.init);
            visit_opt_expr(callback, &value.cond);
            visit_opt_block(callback, &value.body);
            visit_opt_stmt(callback, &value.r#else);
        } else if let Some(value) = node.downcast_ref::<ast_ImportSpec>() {
            visit_opt_ident(callback, &value.name);
            visit_opt_basic_lit(callback, &value.path);
        } else if let Some(value) = node.downcast_ref::<ast_IncDecStmt>() {
            visit_opt_expr(callback, &value.x);
        } else if let Some(value) = node.downcast_ref::<ast_IndexExpr>() {
            visit_opt_expr(callback, &value.x);
            visit_opt_expr(callback, &value.index);
        } else if let Some(value) = node.downcast_ref::<ast_IndexListExpr>() {
            visit_opt_expr(callback, &value.x);
            visit_expr_list(callback, &value.indices);
        } else if let Some(value) = node.downcast_ref::<ast_InterfaceType>() {
            visit_opt_field_list(callback, &value.methods);
        } else if let Some(value) = node.downcast_ref::<ast_KeyValueExpr>() {
            visit_opt_expr(callback, &value.key);
            visit_opt_expr(callback, &value.value);
        } else if let Some(value) = node.downcast_ref::<ast_LabeledStmt>() {
            visit_opt_ident(callback, &value.label);
            visit_opt_stmt(callback, &value.stmt);
        } else if let Some(value) = node.downcast_ref::<ast_MapType>() {
            visit_opt_expr(callback, &value.key);
            visit_opt_expr(callback, &value.value);
        } else if let Some(value) = node.downcast_ref::<ast_ParenExpr>() {
            visit_opt_expr(callback, &value.x);
        } else if let Some(value) = node.downcast_ref::<ast_RangeStmt>() {
            visit_opt_expr(callback, &value.key);
            visit_opt_expr(callback, &value.value);
            visit_opt_expr(callback, &value.x);
            visit_opt_block(callback, &value.body);
        } else if let Some(value) = node.downcast_ref::<ast_ReturnStmt>() {
            visit_expr_list(callback, &value.results);
        } else if let Some(value) = node.downcast_ref::<ast_SelectStmt>() {
            visit_opt_block(callback, &value.body);
        } else if let Some(value) = node.downcast_ref::<ast_SelectorExpr>() {
            visit_opt_expr(callback, &value.x);
            visit_opt_ident(callback, &value.sel);
        } else if let Some(value) = node.downcast_ref::<ast_SendStmt>() {
            visit_opt_expr(callback, &value.chan);
            visit_opt_expr(callback, &value.value);
        } else if let Some(value) = node.downcast_ref::<ast_SliceExpr>() {
            visit_opt_expr(callback, &value.x);
            visit_opt_expr(callback, &value.low);
            visit_opt_expr(callback, &value.high);
            visit_opt_expr(callback, &value.max);
        } else if let Some(value) = node.downcast_ref::<ast_StarExpr>() {
            visit_opt_expr(callback, &value.x);
        } else if let Some(value) = node.downcast_ref::<ast_StructType>() {
            visit_opt_field_list(callback, &value.fields);
        } else if let Some(value) = node.downcast_ref::<ast_SwitchStmt>() {
            visit_opt_stmt(callback, &value.init);
            visit_opt_expr(callback, &value.tag);
            visit_opt_block(callback, &value.body);
        } else if let Some(value) = node.downcast_ref::<ast_TypeAssertExpr>() {
            visit_opt_expr(callback, &value.x);
            visit_opt_expr(callback, &value.r#type);
        } else if let Some(value) = node.downcast_ref::<ast_TypeSpec>() {
            visit_opt_ident(callback, &value.name);
            visit_opt_expr(callback, &value.r#type);
        } else if let Some(value) = node.downcast_ref::<ast_TypeSwitchStmt>() {
            visit_opt_stmt(callback, &value.init);
            visit_opt_stmt(callback, &value.assign);
            visit_opt_block(callback, &value.body);
        } else if let Some(value) = node.downcast_ref::<ast_UnaryExpr>() {
            visit_opt_expr(callback, &value.x);
        } else if let Some(value) = node.downcast_ref::<ast_ValueSpec>() {
            visit_ident_list(callback, &value.names);
            visit_opt_expr(callback, &value.r#type);
            visit_expr_list(callback, &value.values);
        }

        true
    }

    fn visit_opt_expr(callback: &InspectCallback, value: &Arc<Mutex<Option<ast_Expr>>>) {
        if let Some(value) = value.lock().unwrap().as_ref().cloned() {
            visit_node(callback, ast_Node { __go_id: value.__go_id, __go_pos: value.__go_pos, __go_value: value.__go_value.clone() });
        }
    }

    fn visit_expr_list(callback: &InspectCallback, values: &Arc<Mutex<Option<Vec<ast_Expr>>>>) {
        let values = values.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for value in values {
            visit_node(callback, ast_Node { __go_id: value.__go_id, __go_pos: value.__go_pos, __go_value: value.__go_value.clone() });
        }
    }

    fn visit_opt_stmt(callback: &InspectCallback, value: &Arc<Mutex<Option<ast_Stmt>>>) {
        if let Some(value) = value.lock().unwrap().as_ref().cloned() {
            visit_node(callback, ast_Node { __go_id: value.__go_id, __go_pos: value.__go_pos, __go_value: value.__go_value.clone() });
        }
    }

    fn visit_stmt_list(callback: &InspectCallback, values: &Arc<Mutex<Option<Vec<ast_Stmt>>>>) {
        let values = values.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for value in values {
            visit_node(callback, ast_Node { __go_id: value.__go_id, __go_pos: value.__go_pos, __go_value: value.__go_value.clone() });
        }
    }

    fn visit_opt_decl(callback: &InspectCallback, value: &Arc<Mutex<Option<ast_Decl>>>) {
        if let Some(value) = value.lock().unwrap().as_ref().cloned() {
            visit_node(callback, ast_Node { __go_id: value.__go_id, __go_pos: value.__go_pos, __go_value: value.__go_value.clone() });
        }
    }

    fn visit_decl_list(callback: &InspectCallback, values: &Arc<Mutex<Option<Vec<ast_Decl>>>>) {
        let values = values.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for value in values {
            visit_node(callback, ast_Node { __go_id: value.__go_id, __go_pos: value.__go_pos, __go_value: value.__go_value.clone() });
        }
    }

    fn visit_spec_list(callback: &InspectCallback, values: &Arc<Mutex<Option<Vec<ast_Spec>>>>) {
        let values = values.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for value in values {
            visit_node(callback, ast_Node { __go_id: value.__go_id, __go_pos: value.__go_pos, __go_value: value.__go_value.clone() });
        }
    }

    fn visit_ident_list(callback: &InspectCallback, values: &Arc<Mutex<Option<Vec<Arc<Mutex<Option<ast_Ident>>>>>>> ) {
        let values = values.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for value in values {
            visit_opt_ident(callback, &value);
        }
    }

    fn visit_field_list(callback: &InspectCallback, values: &Arc<Mutex<Option<Vec<Arc<Mutex<Option<ast_Field>>>>>>> ) {
        let values = values.lock().unwrap().as_ref().cloned().unwrap_or_default();
        for value in values {
            visit_opt_field(callback, &value);
        }
    }

    fn visit_opt_ident(callback: &InspectCallback, value: &Arc<Mutex<Option<ast_Ident>>>) {
        if let Some(value) = value.lock().unwrap().as_ref().cloned() {
            visit_node(callback, ast_Node::__go_from(value));
        }
    }

    fn visit_opt_basic_lit(callback: &InspectCallback, value: &Arc<Mutex<Option<ast_BasicLit>>>) {
        if let Some(value) = value.lock().unwrap().as_ref().cloned() {
            visit_node(callback, ast_Node::__go_from(value));
        }
    }

    fn visit_opt_block(callback: &InspectCallback, value: &Arc<Mutex<Option<ast_BlockStmt>>>) {
        if let Some(value) = value.lock().unwrap().as_ref().cloned() {
            visit_node(callback, ast_Node::__go_from(value));
        }
    }

    fn visit_opt_call(callback: &InspectCallback, value: &Arc<Mutex<Option<ast_CallExpr>>>) {
        if let Some(value) = value.lock().unwrap().as_ref().cloned() {
            visit_node(callback, ast_Node::__go_from(value));
        }
    }

    fn visit_opt_field(callback: &InspectCallback, value: &Arc<Mutex<Option<ast_Field>>>) {
        if let Some(value) = value.lock().unwrap().as_ref().cloned() {
            visit_node(callback, ast_Node::__go_from(value));
        }
    }

    fn visit_opt_field_list(callback: &InspectCallback, value: &Arc<Mutex<Option<ast_FieldList>>>) {
        if let Some(value) = value.lock().unwrap().as_ref().cloned() {
            visit_node(callback, ast_Node::__go_from(value));
        }
    }

    fn visit_opt_func_type(callback: &InspectCallback, value: &Arc<Mutex<Option<ast_FuncType>>>) {
        if let Some(value) = value.lock().unwrap().as_ref().cloned() {
            visit_node(callback, ast_Node::__go_from(value));
        }
    }
`)
}

// TEMPORARY: hand-written Rust shim for ast.NewIdent.
// Long-term fix: transpile go/ast source.
func writeAstNewIdentFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn new_ident<T0: GoStringArg>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        ")
	out.WriteString(wrappedExternalStubExpr("ast_Ident", "ast_Ident { name: "+wrappedExternalStubExpr("String", "_arg0.into_go_string()")+", ..Default::default() }"))
	out.WriteString("\n")
	out.WriteString("    }\n")
}

// TEMPORARY: hand-written Rust shim for go/parser package.
// Long-term fix: transpile go/parser source.
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
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName], nil)
		}
	}
	out.WriteString("}\n")
}

// TEMPORARY: hand-written Rust shim for go/token package.
// Long-term fix: transpile go/token source.
func writeTokenPackageStub(out *strings.Builder, pkg *externalPackageStub, integerTypes map[string]string) {
	out.WriteString("pub mod token {\n")
	out.WriteString("    use super::*;\n\n")
	constTypes := make(map[string]string, len(pkg.Constants)+len(tokenConstValues()))
	for constName, constType := range pkg.Constants {
		constTypes[constName] = constType
	}
	for constName := range tokenConstValues() {
		if constTypes[constName] == "" {
			constTypes[constName] = "token_Token"
		}
	}
	constNames := make([]string, 0, len(pkg.Constants))
	for constName := range constTypes {
		constNames = append(constNames, constName)
	}
	slices.Sort(constNames)
	for _, constName := range constNames {
		out.WriteString("    pub const ")
		out.WriteString(constName)
		out.WriteString(": ")
		out.WriteString(constTypes[constName])
		out.WriteString(" = ")
		if value, ok := tokenConstValue(constName); ok {
			out.WriteString("token_Token(")
			out.WriteString(strconv.Itoa(value))
			out.WriteString(")")
		} else {
			writeExternalStubConstDefaultValue(out, constTypes[constName], integerTypes)
		}
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
		writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName], nil)
	}
	out.WriteString("}\n")
}

func tokenConstValue(name string) (int, bool) {
	values := tokenConstValues()
	value, ok := values[name]
	return value, ok
}

func tokenConstValues() map[string]int {
	return map[string]int{
		"I_L_L_E_G_A_L":             0,
		"E_O_F":                     1,
		"C_O_M_M_E_N_T":             2,
		"I_D_E_N_T":                 4,
		"I_N_T":                     5,
		"F_L_O_A_T":                 6,
		"I_M_A_G":                   7,
		"C_H_A_R":                   8,
		"S_T_R_I_N_G":               9,
		"A_D_D":                     12,
		"S_U_B":                     13,
		"M_U_L":                     14,
		"Q_U_O":                     15,
		"R_E_M":                     16,
		"A_N_D":                     17,
		"O_R":                       18,
		"X_O_R":                     19,
		"S_H_L":                     20,
		"S_H_R":                     21,
		"A_N_D__N_O_T":              22,
		"A_D_D__A_S_S_I_G_N":        23,
		"S_U_B__A_S_S_I_G_N":        24,
		"M_U_L__A_S_S_I_G_N":        25,
		"Q_U_O__A_S_S_I_G_N":        26,
		"R_E_M__A_S_S_I_G_N":        27,
		"A_N_D__A_S_S_I_G_N":        28,
		"O_R__A_S_S_I_G_N":          29,
		"X_O_R__A_S_S_I_G_N":        30,
		"S_H_L__A_S_S_I_G_N":        31,
		"S_H_R__A_S_S_I_G_N":        32,
		"A_N_D__N_O_T__A_S_S_I_G_N": 33,
		"L_A_N_D":                   34,
		"L_O_R":                     35,
		"A_R_R_O_W":                 36,
		"I_N_C":                     37,
		"D_E_C":                     38,
		"E_Q_L":                     39,
		"L_S_S":                     40,
		"G_T_R":                     41,
		"A_S_S_I_G_N":               42,
		"N_O_T":                     43,
		"N_E_Q":                     44,
		"L_E_Q":                     45,
		"G_E_Q":                     46,
		"D_E_F_I_N_E":               47,
		"E_L_L_I_P_S_I_S":           48,
		"L_P_A_R_E_N":               49,
		"L_B_R_A_C_K":               50,
		"L_B_R_A_C_E":               51,
		"C_O_M_M_A":                 52,
		"P_E_R_I_O_D":               53,
		"R_P_A_R_E_N":               54,
		"R_B_R_A_C_K":               55,
		"R_B_R_A_C_E":               56,
		"S_E_M_I_C_O_L_O_N":         57,
		"C_O_L_O_N":                 58,
		"B_R_E_A_K":                 61,
		"C_A_S_E":                   62,
		"C_H_A_N":                   63,
		"C_O_N_S_T":                 64,
		"C_O_N_T_I_N_U_E":           65,
		"D_E_F_A_U_L_T":             66,
		"D_E_F_E_R":                 67,
		"E_L_S_E":                   68,
		"F_A_L_L_T_H_R_O_U_G_H":     69,
		"F_O_R":                     70,
		"F_U_N_C":                   71,
		"G_O":                       72,
		"G_O_T_O":                   73,
		"I_F":                       74,
		"I_M_P_O_R_T":               75,
		"I_N_T_E_R_F_A_C_E":         76,
		"M_A_P":                     77,
		"P_A_C_K_A_G_E":             78,
		"R_A_N_G_E":                 79,
		"R_E_T_U_R_N":               80,
		"S_E_L_E_C_T":               81,
		"S_T_R_U_C_T":               82,
		"S_W_I_T_C_H":               83,
		"T_Y_P_E":                   84,
		"V_A_R":                     85,
		"T_I_L_D_E":                 88,
	}
}

// TEMPORARY: hand-written Rust shim for go/parser argument traits.
// Long-term fix: transpile go/parser source.
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
        fn into_go_parser_source(self, filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    }

    impl GoParserSourceArg for () {
        fn into_go_parser_source(self, filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            std::fs::read_to_string(filename).map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
        }
    }

    impl GoParserSourceArg for String {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self)
        }
    }

    impl<'a> GoParserSourceArg for &'a str {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self.to_string())
        }
    }

    impl<'a> GoParserSourceArg for &'a String {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self.clone())
        }
    }

    impl GoParserSourceArg for Vec<u8> {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            String::from_utf8(self).map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
        }
    }

    impl GoParserSourceArg for %s<%s<Option<String>>> {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self.%s.as_ref().cloned().unwrap_or_default())
        }
    }

    impl GoParserSourceArg for %s<%s<Option<Vec<u8>>>> {
        fn into_go_parser_source(self, _filename: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            let bytes = self.%s.as_ref().cloned().unwrap_or_default();
            String::from_utf8(bytes).map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
        }
    }

`, outerWrapper, innerWrapper, borrow, outerWrapper, innerWrapper, borrow, outerWrapper, innerWrapper, borrow)
}

// TEMPORARY: hand-written Rust shim for parser.ParseFile.
// Long-term fix: transpile go/parser source.
func writeParserParseFileFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString(`    fn go_parser_error(message: String) -> Box<dyn std::error::Error + Send + Sync> {
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

    fn go_parser_import_spec_from_parts(name: Option<String>, path: String) -> `)
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
            return Some((go_parser_import_spec_from_parts(None, tokens[start].clone()), start + 1));
        }
        if start + 1 < tokens.len() && go_parser_is_string_lit(&tokens[start + 1]) {
            return Some((go_parser_import_spec_from_parts(Some(tokens[start].clone()), tokens[start + 1].clone()), start + 2));
        }
        None
    }

    fn go_parser_some<T>(value: T) -> Arc<Mutex<Option<T>>> {
        Arc::new(Mutex::new(Some(value)))
    }

    fn go_parser_none<T>() -> Arc<Mutex<Option<T>>> {
        Arc::new(Mutex::new(None::<T>))
    }

    fn go_parser_pos(pos: usize) -> Arc<Mutex<Option<token_Pos>>> {
        go_parser_some(token_Pos(go_parser_pos_value(pos)))
    }

    fn go_parser_no_pos() -> Arc<Mutex<Option<token_Pos>>> {
        go_parser_some(token_Pos(0))
    }

    fn go_parser_pos_value(pos: usize) -> i32 {
        pos as i32 + 1
    }

    fn go_parser_token(tok: token_Token) -> Arc<Mutex<Option<token_Token>>> {
        go_parser_some(tok)
    }

    fn go_parser_lit_kind(kind: gosyn::token::LitKind) -> token_Token {
        match kind {
            gosyn::token::LitKind::Ident => token::I_D_E_N_T,
            gosyn::token::LitKind::String => token::S_T_R_I_N_G,
            gosyn::token::LitKind::Integer => token::I_N_T,
            gosyn::token::LitKind::Float => token::F_L_O_A_T,
            gosyn::token::LitKind::Imag => token::I_M_A_G,
            gosyn::token::LitKind::Char => token::C_H_A_R,
        }
    }

    fn go_parser_operator(op: gosyn::token::Operator) -> token_Token {
        match op {
            gosyn::token::Operator::Add => token::A_D_D,
            gosyn::token::Operator::Sub => token::S_U_B,
            gosyn::token::Operator::Star => token::M_U_L,
            gosyn::token::Operator::Quo => token::Q_U_O,
            gosyn::token::Operator::Rem => token::R_E_M,
            gosyn::token::Operator::And => token::A_N_D,
            gosyn::token::Operator::Or => token::O_R,
            gosyn::token::Operator::Xor => token::X_O_R,
            gosyn::token::Operator::Shl => token::S_H_L,
            gosyn::token::Operator::Shr => token::S_H_R,
            gosyn::token::Operator::AndNot => token::A_N_D__N_O_T,
            gosyn::token::Operator::AddAssign => token::A_D_D__A_S_S_I_G_N,
            gosyn::token::Operator::SubAssign => token::S_U_B__A_S_S_I_G_N,
            gosyn::token::Operator::MulAssign => token::M_U_L__A_S_S_I_G_N,
            gosyn::token::Operator::QuoAssign => token::Q_U_O__A_S_S_I_G_N,
            gosyn::token::Operator::RemAssign => token::R_E_M__A_S_S_I_G_N,
            gosyn::token::Operator::AndAssign => token::A_N_D__A_S_S_I_G_N,
            gosyn::token::Operator::OrAssign => token::O_R__A_S_S_I_G_N,
            gosyn::token::Operator::XorAssign => token::X_O_R__A_S_S_I_G_N,
            gosyn::token::Operator::ShlAssign => token::S_H_L__A_S_S_I_G_N,
            gosyn::token::Operator::ShrAssign => token::S_H_R__A_S_S_I_G_N,
            gosyn::token::Operator::AndAnd => token::L_A_N_D,
            gosyn::token::Operator::OrOr => token::L_O_R,
            gosyn::token::Operator::Arrow => token::A_R_R_O_W,
            gosyn::token::Operator::Inc => token::I_N_C,
            gosyn::token::Operator::Dec => token::D_E_C,
            gosyn::token::Operator::Equal => token::E_Q_L,
            gosyn::token::Operator::Less => token::L_S_S,
            gosyn::token::Operator::Greater => token::G_T_R,
            gosyn::token::Operator::Assign => token::A_S_S_I_G_N,
            gosyn::token::Operator::Not => token::N_O_T,
            gosyn::token::Operator::Tiled => token::T_I_L_D_E,
            gosyn::token::Operator::NotEqual => token::N_E_Q,
            gosyn::token::Operator::LessEqual => token::L_E_Q,
            gosyn::token::Operator::GreaterEqual => token::G_E_Q,
            gosyn::token::Operator::Define => token::D_E_F_I_N_E,
            gosyn::token::Operator::DotDotDot => token::E_L_L_I_P_S_I_S,
            _ => token_Token(0),
        }
    }

    fn go_parser_keyword(tok: gosyn::token::Keyword) -> token_Token {
        match tok {
            gosyn::token::Keyword::Break => token::B_R_E_A_K,
            gosyn::token::Keyword::Case => token::C_A_S_E,
            gosyn::token::Keyword::Chan => token::C_H_A_N,
            gosyn::token::Keyword::Const => token::C_O_N_S_T,
            gosyn::token::Keyword::Continue => token::C_O_N_T_I_N_U_E,
            gosyn::token::Keyword::Default => token::D_E_F_A_U_L_T,
            gosyn::token::Keyword::Defer => token::D_E_F_E_R,
            gosyn::token::Keyword::Else => token::E_L_S_E,
            gosyn::token::Keyword::FallThrough => token::F_A_L_L_T_H_R_O_U_G_H,
            gosyn::token::Keyword::For => token::F_O_R,
            gosyn::token::Keyword::Func => token::F_U_N_C,
            gosyn::token::Keyword::Go => token::G_O,
            gosyn::token::Keyword::Goto => token::G_O_T_O,
            gosyn::token::Keyword::If => token::I_F,
            gosyn::token::Keyword::Import => token::I_M_P_O_R_T,
            gosyn::token::Keyword::Interface => token::I_N_T_E_R_F_A_C_E,
            gosyn::token::Keyword::Map => token::M_A_P,
            gosyn::token::Keyword::Package => token::P_A_C_K_A_G_E,
            gosyn::token::Keyword::Range => token::R_A_N_G_E,
            gosyn::token::Keyword::Return => token::R_E_T_U_R_N,
            gosyn::token::Keyword::Select => token::S_E_L_E_C_T,
            gosyn::token::Keyword::Struct => token::S_T_R_U_C_T,
            gosyn::token::Keyword::Switch => token::S_W_I_T_C_H,
            gosyn::token::Keyword::Type => token::T_Y_P_E,
            gosyn::token::Keyword::Var => token::V_A_R,
        }
    }

    fn go_parser_ident_struct(id: gosyn::ast::Ident) -> ast_Ident {
        ast_Ident { __go_pos: go_parser_pos_value(id.pos), name: go_parser_some(id.name), ..Default::default() }
    }

    fn go_parser_ident_expr(id: gosyn::ast::Ident) -> ast_Expr {
        let pos = id.pos;
        ast_Expr::__go_from_with_pos(go_parser_ident_struct(id), go_parser_pos_value(pos))
    }

    fn go_parser_basic_lit_expr(lit: gosyn::ast::BasicLit) -> ast_Expr {
        ast_Expr::__go_from_with_pos(ast_BasicLit {
            kind: go_parser_token(go_parser_lit_kind(lit.kind)),
            pos: go_parser_pos(lit.pos),
            value: go_parser_some(lit.value),
            ..Default::default()
        }, go_parser_pos_value(lit.pos))
    }

    fn go_parser_field_list(list: gosyn::ast::FieldList) -> Arc<Mutex<Option<ast_FieldList>>> {
        let fields = list.list.into_iter().map(go_parser_field).map(|field| go_parser_some(field)).collect::<Vec<_>>();
        go_parser_some(ast_FieldList { list: go_parser_some(fields), ..Default::default() })
    }

    fn go_parser_field(field: gosyn::ast::Field) -> ast_Field {
        let names = field.name.into_iter().map(go_parser_ident_struct).map(go_parser_some).collect::<Vec<_>>();
        ast_Field {
            names: go_parser_some(names),
            r#type: go_parser_some(go_parser_expr(field.typ)),
            tag: field.tag.map(|tag| ast_BasicLit {
                kind: go_parser_token(token::S_T_R_I_N_G),
                value: go_parser_some(tag.value),
                ..Default::default()
            }).map(go_parser_some).unwrap_or_else(go_parser_none),
            ..Default::default()
        }
    }

    fn go_parser_func_type(typ: gosyn::ast::FuncType) -> ast_FuncType {
        ast_FuncType {
            params: go_parser_field_list(typ.params),
            results: go_parser_field_list(typ.result),
            ..Default::default()
        }
    }

    fn go_parser_call_expr(call: gosyn::ast::Call) -> ast_CallExpr {
        ast_CallExpr {
            fun: go_parser_some(go_parser_expr(*call.func)),
            args: go_parser_some(call.args.into_iter().map(go_parser_expr).collect()),
            ellipsis: call.dots.map(go_parser_pos).unwrap_or_else(go_parser_no_pos),
            ..Default::default()
        }
    }

    fn go_parser_lit_element(element: gosyn::ast::Element) -> ast_Expr {
        match element {
            gosyn::ast::Element::Expr(expr) => go_parser_expr(expr),
            gosyn::ast::Element::LitValue(value) => ast_Expr::__go_from_with_pos(ast_CompositeLit {
                elts: go_parser_some(go_parser_lit_values(value)),
                ..Default::default()
            }, 0),
        }
    }

    fn go_parser_lit_values(value: gosyn::ast::LiteralValue) -> Vec<ast_Expr> {
        value.values.into_iter().map(|element| {
            let val = go_parser_lit_element(element.val);
            match element.key {
                Some(key) => ast_Expr::__go_from_with_pos(ast_KeyValueExpr {
                    key: go_parser_some(go_parser_lit_element(key)),
                    value: go_parser_some(val),
                    ..Default::default()
                }, 0),
                None => val,
            }
        }).collect()
    }

    fn go_parser_expr(expr: gosyn::ast::Expression) -> ast_Expr {
        match expr {
            gosyn::ast::Expression::Ident(id) => go_parser_ident_expr(id),
            gosyn::ast::Expression::BasicLit(lit) => go_parser_basic_lit_expr(lit),
            gosyn::ast::Expression::Call(call) => {
                let pos = call.pos.0;
                ast_Expr::__go_from_with_pos(go_parser_call_expr(call), go_parser_pos_value(pos))
            }
            gosyn::ast::Expression::Selector(sel) => ast_Expr::__go_from_with_pos(ast_SelectorExpr {
                x: go_parser_some(go_parser_expr(*sel.x)),
                sel: go_parser_some(go_parser_ident_struct(sel.sel)),
                ..Default::default()
            }, go_parser_pos_value(sel.pos)),
            gosyn::ast::Expression::Index(index) => ast_Expr::__go_from_with_pos(ast_IndexExpr {
                x: go_parser_some(go_parser_expr(*index.left)),
                index: go_parser_some(go_parser_expr(*index.index)),
                ..Default::default()
            }, go_parser_pos_value(index.pos.0)),
            gosyn::ast::Expression::IndexList(index) => ast_Expr::__go_from_with_pos(ast_IndexListExpr {
                x: go_parser_some(go_parser_expr(*index.left)),
                indices: go_parser_some(index.indices.into_iter().map(go_parser_expr).collect()),
                ..Default::default()
            }, go_parser_pos_value(index.pos.0)),
            gosyn::ast::Expression::Slice(slice) => ast_Expr::__go_from_with_pos(ast_SliceExpr {
                x: go_parser_some(go_parser_expr(*slice.left)),
                low: slice.index[0].as_ref().map(|expr| go_parser_expr((**expr).clone())).map(go_parser_some).unwrap_or_else(go_parser_none),
                high: slice.index[1].as_ref().map(|expr| go_parser_expr((**expr).clone())).map(go_parser_some).unwrap_or_else(go_parser_none),
                max: slice.index[2].as_ref().map(|expr| go_parser_expr((**expr).clone())).map(go_parser_some).unwrap_or_else(go_parser_none),
                slice3: go_parser_some(slice.index[2].is_some()),
                ..Default::default()
            }, go_parser_pos_value(slice.pos.0)),
            gosyn::ast::Expression::FuncLit(lit) => ast_Expr::__go_from_with_pos(ast_FuncLit {
                r#type: go_parser_some(go_parser_func_type(lit.typ)),
                body: go_parser_some(go_parser_block(lit.body)),
                ..Default::default()
            }, 0),
            gosyn::ast::Expression::Ellipsis(ellipsis) => ast_Expr::__go_from_with_pos(ast_Ellipsis {
                elt: ellipsis.elt.map(|expr| go_parser_expr(*expr)).map(go_parser_some).unwrap_or_else(go_parser_none),
                ..Default::default()
            }, go_parser_pos_value(ellipsis.pos)),
            gosyn::ast::Expression::Star(star) => ast_Expr::__go_from_with_pos(ast_StarExpr {
                x: go_parser_some(go_parser_expr(*star.right)),
                ..Default::default()
            }, go_parser_pos_value(star.pos)),
            gosyn::ast::Expression::Paren(paren) => ast_Expr::__go_from_with_pos(ast_ParenExpr {
                x: go_parser_some(go_parser_expr(*paren.expr)),
                ..Default::default()
            }, go_parser_pos_value(paren.pos.0)),
            gosyn::ast::Expression::TypeAssert(assertion) => ast_Expr::__go_from_with_pos(ast_TypeAssertExpr {
                x: go_parser_some(go_parser_expr(*assertion.left)),
                r#type: assertion.right.map(|expr| go_parser_expr(*expr)).map(go_parser_some).unwrap_or_else(go_parser_none),
                ..Default::default()
            }, go_parser_pos_value(assertion.pos.0)),
            gosyn::ast::Expression::CompositeLit(lit) => {
                let pos = lit.val.pos.0;
                ast_Expr::__go_from_with_pos(ast_CompositeLit {
                    r#type: go_parser_some(go_parser_expr(*lit.typ)),
                    elts: go_parser_some(go_parser_lit_values(lit.val)),
                    ..Default::default()
                }, go_parser_pos_value(pos))
            }
            gosyn::ast::Expression::Operation(op) => {
                let token = go_parser_operator(op.op);
                match op.y {
                    Some(y) => ast_Expr::__go_from_with_pos(ast_BinaryExpr {
                        x: go_parser_some(go_parser_expr(*op.x)),
                        y: go_parser_some(go_parser_expr(*y)),
                        op: go_parser_token(token),
                        ..Default::default()
                    }, go_parser_pos_value(op.pos)),
                    None if token == token::M_U_L => ast_Expr::__go_from_with_pos(ast_StarExpr {
                        x: go_parser_some(go_parser_expr(*op.x)),
                        ..Default::default()
                    }, go_parser_pos_value(op.pos)),
                    None => ast_Expr::__go_from_with_pos(ast_UnaryExpr {
                        x: go_parser_some(go_parser_expr(*op.x)),
                        op: go_parser_token(token),
                        ..Default::default()
                    }, go_parser_pos_value(op.pos)),
                }
            }
            gosyn::ast::Expression::TypeMap(map) => ast_Expr::__go_from_with_pos(ast_MapType {
                key: go_parser_some(go_parser_expr(*map.key)),
                value: go_parser_some(go_parser_expr(*map.val)),
                ..Default::default()
            }, go_parser_pos_value(map.pos.0)),
            gosyn::ast::Expression::TypeArray(array) => ast_Expr::__go_from_with_pos(ast_ArrayType {
                len: go_parser_some(go_parser_expr(*array.len)),
                elt: go_parser_some(go_parser_expr(*array.typ)),
                ..Default::default()
            }, go_parser_pos_value(array.pos.0)),
            gosyn::ast::Expression::TypeSlice(slice) => ast_Expr::__go_from_with_pos(ast_ArrayType {
                len: go_parser_none(),
                elt: go_parser_some(go_parser_expr(*slice.typ)),
                ..Default::default()
            }, go_parser_pos_value(slice.pos.0)),
            gosyn::ast::Expression::TypeFunction(typ) => {
                let pos = typ.pos;
                ast_Expr::__go_from_with_pos(go_parser_func_type(typ), go_parser_pos_value(pos))
            }
            gosyn::ast::Expression::TypeStruct(typ) => ast_Expr::__go_from_with_pos(ast_StructType {
                fields: go_parser_some(ast_FieldList {
                    list: go_parser_some(typ.fields.into_iter().map(go_parser_field).map(go_parser_some).collect()),
                    ..Default::default()
                }),
                ..Default::default()
            }, go_parser_pos_value(typ.pos.0)),
            gosyn::ast::Expression::TypeInterface(typ) => ast_Expr::__go_from_with_pos(ast_InterfaceType {
                methods: go_parser_field_list(typ.methods),
                ..Default::default()
            }, go_parser_pos_value(typ.pos)),
            gosyn::ast::Expression::TypePointer(ptr) => ast_Expr::__go_from_with_pos(ast_StarExpr {
                x: go_parser_some(go_parser_expr(*ptr.typ)),
                ..Default::default()
            }, go_parser_pos_value(ptr.pos)),
            gosyn::ast::Expression::TypeChannel(chan) => {
                let dir = match chan.dir {
                    Some(gosyn::ast::ChanMode::Send) => ast_ChanDir(1),
                    Some(gosyn::ast::ChanMode::Recv) => ast_ChanDir(2),
                    None => ast_ChanDir(3),
                };
                ast_Expr::__go_from_with_pos(ast_ChanType {
                    dir: go_parser_some(dir),
                    value: go_parser_some(go_parser_expr(*chan.typ)),
                    ..Default::default()
                }, go_parser_pos_value(chan.pos.0))
            }
            gosyn::ast::Expression::List(list) => list.into_iter().next().map(go_parser_expr).unwrap_or_default(),
            gosyn::ast::Expression::Range(range) => ast_Expr::__go_from_with_pos(ast_UnaryExpr {
                op: go_parser_token(token::R_A_N_G_E),
                x: go_parser_some(go_parser_expr(*range.right)),
                ..Default::default()
            }, go_parser_pos_value(range.pos)),
        }
    }

    fn go_parser_block(block: gosyn::ast::BlockStmt) -> ast_BlockStmt {
        ast_BlockStmt {
            lbrace: go_parser_pos(block.pos.0),
            list: go_parser_some(block.list.into_iter().map(go_parser_stmt).collect()),
            ..Default::default()
        }
    }

    fn go_parser_expr_from_stmt(stmt: gosyn::ast::Statement) -> Arc<Mutex<Option<ast_Expr>>> {
        match stmt {
            gosyn::ast::Statement::Expr(expr) => go_parser_some(go_parser_expr(expr.expr)),
            _ => go_parser_none(),
        }
    }

    fn go_parser_decl_stmt(decl: gosyn::ast::DeclStmt) -> ast_Decl {
        match decl {
            gosyn::ast::DeclStmt::Type(decl) => go_parser_gen_decl(token::T_Y_P_E, decl.specs.into_iter().map(go_parser_type_spec).collect()),
            gosyn::ast::DeclStmt::Const(decl) => go_parser_gen_decl(token::C_O_N_S_T, decl.specs.into_iter().map(go_parser_const_spec).collect()),
            gosyn::ast::DeclStmt::Variable(decl) => go_parser_gen_decl(token::V_A_R, decl.specs.into_iter().map(go_parser_var_spec).collect()),
        }
    }

    fn go_parser_stmt(stmt: gosyn::ast::Statement) -> ast_Stmt {
        match stmt {
            gosyn::ast::Statement::Expr(stmt) => ast_Stmt::__go_from(ast_ExprStmt {
                x: go_parser_some(go_parser_expr(stmt.expr)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Assign(stmt) => ast_Stmt::__go_from(ast_AssignStmt {
                lhs: go_parser_some(stmt.left.into_iter().map(go_parser_expr).collect()),
                rhs: go_parser_some(stmt.right.into_iter().map(go_parser_expr).collect()),
                tok: go_parser_token(go_parser_operator(stmt.op)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Return(stmt) => ast_Stmt::__go_from(ast_ReturnStmt {
                results: go_parser_some(stmt.ret.into_iter().map(go_parser_expr).collect()),
                ..Default::default()
            }),
            gosyn::ast::Statement::Block(block) => ast_Stmt::__go_from(go_parser_block(block)),
            gosyn::ast::Statement::If(stmt) => ast_Stmt::__go_from(ast_IfStmt {
                init: stmt.init.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                cond: go_parser_some(go_parser_expr(stmt.cond)),
                body: go_parser_some(go_parser_block(stmt.body)),
                r#else: stmt.else_.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                ..Default::default()
            }),
            gosyn::ast::Statement::For(stmt) => ast_Stmt::__go_from(ast_ForStmt {
                init: stmt.init.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                cond: stmt.cond.map(|stmt| go_parser_expr_from_stmt(*stmt)).unwrap_or_else(go_parser_none),
                post: stmt.post.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                body: go_parser_some(go_parser_block(stmt.body)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Range(stmt) => ast_Stmt::__go_from(ast_RangeStmt {
                key: stmt.key.map(go_parser_expr).map(go_parser_some).unwrap_or_else(go_parser_none),
                value: stmt.value.map(go_parser_expr).map(go_parser_some).unwrap_or_else(go_parser_none),
                tok: go_parser_token(stmt.op.map(|op| go_parser_operator(op.1)).unwrap_or(token::A_S_S_I_G_N)),
                x: go_parser_some(go_parser_expr(stmt.expr)),
                body: go_parser_some(go_parser_block(stmt.body)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Go(stmt) => ast_Stmt::__go_from(ast_GoStmt {
                call: go_parser_some(go_parser_call_expr(stmt.call)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Defer(stmt) => ast_Stmt::__go_from(ast_DeferStmt {
                call: go_parser_some(go_parser_call_expr(stmt.call)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Send(stmt) => ast_Stmt::__go_from(ast_SendStmt {
                chan: go_parser_some(go_parser_expr(stmt.chan)),
                value: go_parser_some(go_parser_expr(stmt.value)),
                ..Default::default()
            }),
            gosyn::ast::Statement::IncDec(stmt) => ast_Stmt::__go_from(ast_IncDecStmt {
                x: go_parser_some(go_parser_expr(stmt.expr)),
                tok: go_parser_token(go_parser_operator(stmt.op)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Branch(stmt) => ast_Stmt::__go_from(ast_BranchStmt {
                tok: go_parser_token(go_parser_keyword(stmt.key)),
                label: stmt.ident.map(go_parser_ident_struct).map(go_parser_some).unwrap_or_else(go_parser_none),
                ..Default::default()
            }),
            gosyn::ast::Statement::Label(stmt) => ast_Stmt::__go_from(ast_LabeledStmt {
                label: go_parser_some(go_parser_ident_struct(stmt.name)),
                stmt: go_parser_some(go_parser_stmt(*stmt.stmt)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Declaration(decl) => ast_Stmt::__go_from(ast_DeclStmt {
                decl: go_parser_some(go_parser_decl_stmt(decl)),
                ..Default::default()
            }),
            gosyn::ast::Statement::Switch(stmt) => ast_Stmt::__go_from(ast_SwitchStmt {
                init: stmt.init.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                tag: stmt.tag.map(go_parser_expr).map(go_parser_some).unwrap_or_else(go_parser_none),
                body: go_parser_some(ast_BlockStmt {
                    lbrace: go_parser_pos(stmt.block.pos.0),
                    list: go_parser_some(stmt.block.body.into_iter().map(go_parser_case_clause).collect()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            gosyn::ast::Statement::TypeSwitch(stmt) => ast_Stmt::__go_from(ast_TypeSwitchStmt {
                init: stmt.init.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                assign: stmt.tag.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
                body: go_parser_some(ast_BlockStmt {
                    lbrace: go_parser_pos(stmt.block.pos.0),
                    list: go_parser_some(stmt.block.body.into_iter().map(go_parser_case_clause).collect()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            gosyn::ast::Statement::Select(stmt) => ast_Stmt::__go_from(ast_SelectStmt {
                body: go_parser_some(ast_BlockStmt {
                    lbrace: go_parser_pos(stmt.body.pos.0),
                    list: go_parser_some(stmt.body.body.into_iter().map(go_parser_comm_clause).collect()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            gosyn::ast::Statement::Empty(_) => ast_Stmt::__go_from(ast_EmptyStmt),
        }
    }

    fn go_parser_case_clause(clause: gosyn::ast::CaseClause) -> ast_Stmt {
        ast_Stmt::__go_from(ast_CaseClause {
            list: go_parser_some(clause.list.into_iter().map(go_parser_expr).collect()),
            body: go_parser_some((*clause.body).into_iter().map(go_parser_stmt).collect()),
            colon: go_parser_pos(clause.pos.1),
            ..Default::default()
        })
    }

    fn go_parser_comm_clause(clause: gosyn::ast::CommClause) -> ast_Stmt {
        ast_Stmt::__go_from(ast_CommClause {
            comm: clause.comm.map(|stmt| go_parser_stmt(*stmt)).map(go_parser_some).unwrap_or_else(go_parser_none),
            body: go_parser_some((*clause.body).into_iter().map(go_parser_stmt).collect()),
            ..Default::default()
        })
    }

    fn go_parser_import_spec(import: gosyn::ast::Import) -> Arc<Mutex<Option<ast_ImportSpec>>> {
        go_parser_some(ast_ImportSpec {
            name: import.name.map(go_parser_ident_struct).map(go_parser_some).unwrap_or_else(go_parser_none),
            path: go_parser_some(ast_BasicLit {
                kind: go_parser_token(token::S_T_R_I_N_G),
                value: go_parser_some(import.path.value),
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    fn go_parser_gen_decl(tok: token_Token, specs: Vec<ast_Spec>) -> ast_Decl {
        ast_Decl::__go_from(ast_GenDecl {
            tok: go_parser_token(tok),
            specs: go_parser_some(specs),
            ..Default::default()
        })
    }

    fn go_parser_var_spec(spec: gosyn::ast::VarSpec) -> ast_Spec {
        ast_Spec::__go_from(ast_ValueSpec {
            names: go_parser_some(spec.name.into_iter().map(go_parser_ident_struct).map(go_parser_some).collect()),
            r#type: spec.typ.map(go_parser_expr).map(go_parser_some).unwrap_or_else(go_parser_none),
            values: go_parser_some(spec.values.into_iter().map(go_parser_expr).collect()),
            ..Default::default()
        })
    }

    fn go_parser_const_spec(spec: gosyn::ast::ConstSpec) -> ast_Spec {
        ast_Spec::__go_from(ast_ValueSpec {
            names: go_parser_some(spec.name.into_iter().map(go_parser_ident_struct).map(go_parser_some).collect()),
            r#type: spec.typ.map(go_parser_expr).map(go_parser_some).unwrap_or_else(go_parser_none),
            values: go_parser_some(spec.values.into_iter().map(go_parser_expr).collect()),
            ..Default::default()
        })
    }

    fn go_parser_type_spec(spec: gosyn::ast::TypeSpec) -> ast_Spec {
        ast_Spec::__go_from(ast_TypeSpec {
            name: go_parser_some(go_parser_ident_struct(spec.name)),
            r#type: go_parser_some(go_parser_expr(spec.typ)),
            assign: if spec.alias { go_parser_pos(1) } else { go_parser_no_pos() },
            ..Default::default()
        })
    }

    fn go_parser_func_decl(decl: gosyn::ast::FuncDecl) -> ast_Decl {
        ast_Decl::__go_from(ast_FuncDecl {
            recv: decl.recv.map(go_parser_field_list).unwrap_or_else(go_parser_none),
            name: go_parser_some(go_parser_ident_struct(decl.name)),
            r#type: go_parser_some(go_parser_func_type(decl.typ)),
            body: decl.body.map(go_parser_block).map(go_parser_some).unwrap_or_else(go_parser_none),
            ..Default::default()
        })
    }

    fn go_parser_decl(decl: gosyn::ast::Declaration) -> ast_Decl {
        match decl {
            gosyn::ast::Declaration::Function(decl) => go_parser_func_decl(decl),
            gosyn::ast::Declaration::Type(decl) => go_parser_gen_decl(token::T_Y_P_E, decl.specs.into_iter().map(go_parser_type_spec).collect()),
            gosyn::ast::Declaration::Const(decl) => go_parser_gen_decl(token::C_O_N_S_T, decl.specs.into_iter().map(go_parser_const_spec).collect()),
            gosyn::ast::Declaration::Variable(decl) => go_parser_gen_decl(token::V_A_R, decl.specs.into_iter().map(go_parser_var_spec).collect()),
        }
    }

    fn go_parser_parse_file(filename: &str, source: &str) -> Result<ast_File, Box<dyn std::error::Error + Send + Sync>> {
        let parsed = gosyn::parse_source(source).map_err(|err| go_parser_error(err.to_string()))?;
        Ok(ast_File {
            __go_filename: go_parser_some(filename.to_string()),
            __go_source: go_parser_some(source.to_string()),
            imports: go_parser_some(parsed.imports.into_iter().map(go_parser_import_spec).collect()),
            decls: go_parser_some(parsed.decl.into_iter().map(go_parser_decl).collect()),
            name: go_parser_some(go_parser_ident_struct(parsed.pkg_name)),
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
	out.WriteString(wrappedExternalStubSomeExpr("Box<dyn std::error::Error + Send + Sync>", "err"))
	out.WriteString(`),
        };
        match go_parser_parse_file(&filename, &source) {
            Ok(file) => (`)
	out.WriteString(wrappedExternalStubExpr("ast_File", "file"))
	out.WriteString(`, `)
	out.WriteString(wrappedExternalStubNoneExpr("Box<dyn std::error::Error + Send + Sync>"))
	out.WriteString(`),
            Err(err) => (`)
	out.WriteString(wrappedExternalStubNoneExpr("ast_File"))
	out.WriteString(`, `)
	out.WriteString(wrappedExternalStubSomeExpr("Box<dyn std::error::Error + Send + Sync>", "err"))
	out.WriteString(`),
        }
    }
`)
}

// TEMPORARY: hand-written Rust shim for strconv package.
// Long-term fix: transpile strconv source (pure Go, no runtime ties).
func writeStrconvPackageStub(out *strings.Builder, pkg *externalPackageStub, integerTypes map[string]string, stubs map[string]bool) {
	out.WriteString("pub mod strconv {\n")
	out.WriteString("    use super::*;\n\n")
	writeStrconvHelpers(out)

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
		if funcName == "unquote" {
			writeStrconvUnquoteFunction(out, pkg.Functions[funcName])
		} else {
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName], stubs)
		}
	}
	out.WriteString("}\n")
}

// TEMPORARY: hand-written Rust shim for strconv helpers.
// Long-term fix: transpile strconv source.
func writeStrconvHelpers(out *strings.Builder) {
	borrow := ".borrow()"
	if NeedsConcurrentWrapper() {
		borrow = ".lock().unwrap()"
	}
	stringType := wrappedExternalStubType("String")
	errorType := "Box<dyn std::error::Error>"
	if NeedsConcurrentWrapper() {
		errorType = "Box<dyn std::error::Error + Send + Sync>"
	}
	fmt.Fprintf(out, `    pub trait GoStrconvStringArg {
        fn into_go_strconv_string(self) -> String;
    }

    impl GoStrconvStringArg for String {
        fn into_go_strconv_string(self) -> String {
            self
        }
    }

    impl<'a> GoStrconvStringArg for &'a str {
        fn into_go_strconv_string(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoStrconvStringArg for &'a String {
        fn into_go_strconv_string(self) -> String {
            self.clone()
        }
    }

    impl GoStrconvStringArg for %s {
        fn into_go_strconv_string(self) -> String {
            self%s.as_ref().cloned().unwrap_or_default()
        }
    }

    fn strconv_error(message: String) -> %s {
        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, message))
    }

    fn strconv_hex_digit(ch: char) -> Option<u32> {
        ch.to_digit(16)
    }

    fn strconv_read_hex<I: Iterator<Item = char>>(chars: &mut I, count: usize) -> Result<char, %s> {
        let mut value = 0u32;
        for _ in 0..count {
            let ch = chars.next().ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
            let digit = strconv_hex_digit(ch).ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
            value = (value << 4) | digit;
        }
        char::from_u32(value).ok_or_else(|| strconv_error("invalid quoted string".to_string()))
    }

    fn strconv_unquote_text(input: &str) -> Result<String, %s> {
        let mut chars = input.chars();
        let quote = chars.next().ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
        if quote != '"' && quote != '\'' && quote != char::from(96) {
            return Err(strconv_error("invalid quoted string".to_string()));
        }
        let inner = input.strip_prefix(quote).and_then(|s| s.strip_suffix(quote)).ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
        if quote == char::from(96) {
            return Ok(inner.to_string());
        }
        let mut out = String::new();
        let mut chars = inner.chars();
        while let Some(ch) = chars.next() {
            if ch != '\\' {
                out.push(ch);
                continue;
            }
            let esc = chars.next().ok_or_else(|| strconv_error("invalid quoted string".to_string()))?;
            match esc {
                'a' => out.push('\x07'),
                'b' => out.push('\x08'),
                'f' => out.push('\x0c'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'v' => out.push('\x0b'),
                '\\' => out.push('\\'),
                '"' => out.push('"'),
                '\'' => out.push('\''),
                'x' => out.push(strconv_read_hex(&mut chars, 2)?),
                'u' => out.push(strconv_read_hex(&mut chars, 4)?),
                'U' => out.push(strconv_read_hex(&mut chars, 8)?),
                '0'..='7' => {
                    let mut value = esc.to_digit(8).unwrap();
                    for _ in 0..2 {
                        let Some(next) = chars.clone().next() else { break };
                        let Some(digit) = next.to_digit(8) else { break };
                        chars.next();
                        value = (value << 3) | digit;
                    }
                    out.push(char::from_u32(value).ok_or_else(|| strconv_error("invalid quoted string".to_string()))?);
                }
                _ => return Err(strconv_error("invalid quoted string".to_string())),
            }
        }
        Ok(out)
    }

`, stringType, borrow, errorType, errorType, errorType)
}

// TEMPORARY: hand-written Rust shim for strconv.Unquote.
// Long-term fix: transpile strconv source.
func writeStrconvUnquoteFunction(out *strings.Builder, fn externalPackageStubFunction) {
	errorType := "Box<dyn std::error::Error>"
	if NeedsConcurrentWrapper() {
		errorType = "Box<dyn std::error::Error + Send + Sync>"
	}
	out.WriteString("    pub fn unquote<T0: GoStrconvStringArg>(_arg0: T0) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n        match strconv_unquote_text(&_arg0.into_go_strconv_string()) {\n            Ok(value) => (")
	out.WriteString(wrappedExternalStubExpr("String", "value"))
	out.WriteString(", ")
	out.WriteString(wrappedExternalStubNoneExpr(errorType))
	out.WriteString("),\n            Err(err) => (")
	out.WriteString(wrappedExternalStubNoneExpr("String"))
	out.WriteString(", ")
	out.WriteString(wrappedExternalStubSomeExpr(errorType, "err"))
	out.WriteString("),\n        }\n    }\n")
}

// TEMPORARY: hand-written Rust shim for go/build package.
// Long-term fix: transpile go/build source.
func writeBuildPackageStub(out *strings.Builder, pkg *externalPackageStub, integerTypes map[string]string) {
	out.WriteString("pub mod build {\n")
	out.WriteString("    use super::*;\n")
	out.WriteString("    use std::path::PathBuf;\n\n")
	writeGoStringArgTrait(out)
	writeBuildHelpers(out)

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
		if varName == "Default" {
			writeBuildDefaultFunction(out)
		} else {
			writeExternalPackageStubVariable(out, varName, pkg.Variables[varName])
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
		if funcName == "import" {
			writeBuildImportFunction(out, pkg.Functions[funcName])
		} else if funcName == "is_local_import" {
			writeBuildIsLocalImportFunction(out)
		} else {
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName], nil)
		}
	}
	out.WriteString("}\n")
}

// TEMPORARY: hand-written Rust shim for go/build helpers.
// Long-term fix: transpile go/build source.
func writeBuildHelpers(out *strings.Builder) {
	errorType := "Box<dyn std::error::Error>"
	if NeedsConcurrentWrapper() {
		errorType = "Box<dyn std::error::Error + Send + Sync>"
	}
	fmt.Fprintf(out, `    type GoError = %s;

    fn go_build_no_error() -> GoError {
        %s
    }

    fn go_build_error(message: String) -> GoError {
        %s
    }

    fn go_build_string(value: String) -> %s {
        %s
    }

    fn go_build_bool(value: bool) -> %s {
        %s
    }

    fn go_build_goroot() -> String {
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
    }

    fn go_build_package(import_path: String, dir: String, goroot: bool) -> build_Package {
        build_Package {
            dir: go_build_string(dir),
            goroot: go_build_bool(goroot),
            import_path: go_build_string(import_path),
            pkg_obj: go_build_string(String::new()),
            ..Default::default()
        }
    }

    fn go_build_dir_for_import(goroot: &str, import_path: &str) -> PathBuf {
        let mut dir = PathBuf::from(goroot);
        dir.push("src");
        for part in import_path.split('/') {
            if !part.is_empty() {
                dir.push(part);
            }
        }
        dir
    }

    pub(crate) fn go_build_import_path(import_path: String) -> (%s, GoError) {
        if import_path.is_empty() || go_build_is_local_import_str(&import_path) {
            return (
                %s,
                go_build_error(format!("cannot import {}", import_path)),
            );
        }
        let goroot = go_build_goroot();
        if !goroot.is_empty() {
            let dir = go_build_dir_for_import(&goroot, &import_path);
            if dir.is_dir() {
                return (
                    %s,
                    go_build_no_error(),
                );
            }
        }
        (
            %s,
            go_build_error(format!("cannot find package {}", import_path)),
        )
    }

    fn go_build_is_local_import_str(path: &str) -> bool {
        path == "." || path == ".." || path.starts_with("./") || path.starts_with("../")
    }

`, wrappedExternalStubType(errorType), wrappedExternalStubNoneExpr(errorType), wrappedExternalStubExpr(errorType, "Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, message))"), wrappedExternalStubType("String"), wrappedExternalStubExpr("String", "value"), wrappedExternalStubType("bool"), wrappedExternalStubExpr("bool", "value"), wrappedExternalStubType("build_Package"), wrappedExternalStubExpr("build_Package", "go_build_package(import_path.clone(), String::new(), false)"), wrappedExternalStubExpr("build_Package", "go_build_package(import_path.clone(), dir.to_string_lossy().into_owned(), true)"), wrappedExternalStubExpr("build_Package", "go_build_package(import_path.clone(), String::new(), false)"))
}

// TEMPORARY: hand-written Rust shim for build.Default.
// Long-term fix: transpile go/build source.
func writeBuildDefaultFunction(out *strings.Builder) {
	out.WriteString("    pub fn Default() -> ")
	out.WriteString(wrappedExternalStubType("build_Context"))
	out.WriteString(" {\n")
	out.WriteString("        ")
	out.WriteString(wrappedExternalStubExpr("build_Context", "build_Context { g_o_r_o_o_t: go_build_string(go_build_goroot()), ..Default::default() }"))
	out.WriteString("\n")
	out.WriteString("    }\n")
}

// TEMPORARY: hand-written Rust shim for build.Import.
// Long-term fix: transpile go/build source.
func writeBuildImportFunction(out *strings.Builder, fn externalPackageStubFunction) {
	out.WriteString("    pub fn import<T0: GoStringArg, T1: GoStringArg, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let import_path = _arg0.into_go_string();\n")
	out.WriteString("        go_build_import_path(import_path)\n")
	out.WriteString("    }\n")
}

// TEMPORARY: hand-written Rust shim for build.IsLocalImport.
// Long-term fix: transpile go/build source.
func writeBuildIsLocalImportFunction(out *strings.Builder) {
	out.WriteString("    pub fn is_local_import<T0: GoStringArg>(_arg0: T0) -> bool {\n")
	out.WriteString("        go_build_is_local_import_str(&_arg0.into_go_string())\n")
	out.WriteString("    }\n")
}

// TEMPORARY: hand-written Rust shim for flag package.
// Long-term fix: transpile flag source (mostly pure Go).
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
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName], nil)
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
		} else if funcName == "mkdir_all" {
			writeOsMkdirAllFunction(out, pkg.Functions[funcName])
		} else if funcName == "read_file" {
			writeOsReadFileFunction(out, pkg.Functions[funcName])
		} else if funcName == "read_dir" {
			writeOsReadDirFunction(out, pkg.Functions[funcName])
		} else if funcName == "stat" {
			writeOsStatFunction(out, pkg.Functions[funcName])
		} else if funcName == "write_file" {
			writeOsWriteFileFunction(out, pkg.Functions[funcName])
		} else {
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName], nil)
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
	_, needsReadDir := pkg.Functions["read_dir"]
	_, needsReadFile := pkg.Functions["read_file"]
	_, needsWriteFile := pkg.Functions["write_file"]
	return needsStat || needsMkdirAll || needsReadDir || needsReadFile || needsWriteFile
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

`, errorType, GetOuterWrapperType(), GetInnerWrapperType(), GetOuterWrapperType(), GetInnerWrapperType())
}

// PERMANENT: not scaffold — os.Exit maps to std::process::exit, runtime-tied.
func writeOsExitFunction(out *strings.Builder) {
	out.WriteString("    pub fn exit<T0: Into<i32>>(_arg0: T0) {\n")
	out.WriteString("        std::process::exit(_arg0.into());\n")
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

// MIXED: top-level path/filepath emitter. Pure path-manipulation funcs (Join, IsAbs) are
// TEMPORARY (transpile path/filepath source); Abs/EvalSymlinks are PERMANENT (OS-tied).
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
			writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName], nil)
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

// MACHINERY: helper trait for filepath.Join variadic-argument coercion.
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

`, errorType, GetOuterWrapperType(), GetInnerWrapperType(), GetOuterWrapperType(), GetInnerWrapperType())
}

// TEMPORARY: hand-written Rust shim for filepath single-string functions (Base, Dir, Ext, Clean).
// Long-term fix: transpile path/filepath source (pure string manipulation).
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

// TEMPORARY: hand-written Rust shim for filepath.Join.
// Long-term fix: transpile path/filepath source.
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

// TEMPORARY: hand-written Rust shim for filepath.IsAbs.
// Long-term fix: transpile path/filepath source (pure string check).
func writeFilepathIsAbsFunction(out *strings.Builder) {
	out.WriteString("    pub fn is_abs<T0: GoStringArg>(_arg0: T0) -> bool {\n")
	out.WriteString("        let path = _arg0.into_go_string();\n")
	out.WriteString("        Path::new(&path).is_absolute()\n")
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
		writeExternalPackageStubFunction(out, funcName, pkg.Functions[funcName], nil)
	}
	out.WriteString("}\n")
}

// MACHINERY: generic stub function emitter dispatch.
//
// Per AGENTS.md "Strategy: Transpile stdlib, don't bridge it", the
// generic body emits a panic instead of returning defaults. Functions
// whose stubs need real behavior (exec.Command, io.Copy, os.Pipe, etc.)
// have custom emitters dispatched above. Anything routing through the
// generic body has no implementation — calling it at runtime is a bug
// to be fixed at the call site or by adding a custom emitter.
func writeExternalPackageStubFunction(out *strings.Builder, funcName string, fn externalPackageStubFunction, stubs map[string]bool) {
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
	if funcName == "copy" && len(fn.ReturnTypes) == 2 {
		writeIoCopyStub(out, fn, stubs)
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
	if funcName == "new_term" && len(fn.ReturnTypes) == 1 {
		writeTypesNewTermFunction(out)
		return
	}
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
	out.WriteString("        panic!(\"")
	out.WriteString(funcName)
	out.WriteString(" bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md\")\n")
	out.WriteString("    }\n")
}

// TEMPORARY: hand-written Rust shim for go/types.NewTerm.
// Long-term fix: transpile go/types source. See AGENTS.md "Strategy:
// Transpile stdlib, don't bridge it" and docs/bridge_debt.md (row:
// types-new-term). Mirrors `func NewTerm(tilde bool, typ Type) *Term`
// — the typ arg is accepted via the GoTypesTypeArg trait emitted by
// writeTypesTermStub so the call site can pass either Go's nil
// (lowered as `()`) or a wrapped types_Type handle.
func writeTypesNewTermFunction(out *strings.Builder) {
	typesTermWrapped := wrappedExternalStubType("types_Term")
	out.WriteString("    pub fn new_term<T1: GoTypesTypeArg>(tilde: bool, typ: T1) -> ")
	out.WriteString(typesTermWrapped)
	out.WriteString(" {\n        ")
	out.WriteString(wrappedExternalStubExpr("types_Term", "types_Term { tilde, typ: typ.__go_into_types_type_arg() }"))
	out.WriteString("\n    }\n")
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

// TEMPORARY: hand-written Rust shim for io.Copy.
// Long-term fix: transpile io source (pure-Go algorithm over Reader/Writer interfaces).
func writeIoCopyStub(out *strings.Builder, fn externalPackageStubFunction, stubs map[string]bool) {
	out.WriteString("    pub fn copy<T0: 'static, T1: 'static>(_arg0: T0, _arg1: T1) -> ")
	writeExternalStubReturnType(out, fn.ReturnTypes)
	out.WriteString(" {\n")
	out.WriteString("        let mut data = Vec::new();\n")
	if stubs["os_File"] {
		fmt.Fprintf(out, `        if let Some(src) = (&_arg1 as &dyn std::any::Any).downcast_ref::<%s>() {
            data = %s.as_ref().map(|file| file.__go_read_all_for_copy()).unwrap_or_default();
        }
        if let Some(src) = (&_arg1 as &dyn std::any::Any).downcast_ref::<os_File>() {
            data = src.__go_read_all_for_copy();
        };
`, wrappedExternalStubType("os_File"), externalStubBorrowExpr("src"))
	}
	if stubs["bytes_Buffer"] {
		fmt.Fprintf(out, `        if let Some(src) = (&_arg1 as &dyn std::any::Any).downcast_ref::<%s>() {
            data = %s.as_ref().map(|buffer| buffer.__go_bytes()).unwrap_or_default();
        }
        if let Some(src) = (&_arg1 as &dyn std::any::Any).downcast_ref::<bytes_Buffer>() {
            data = src.__go_bytes();
        }
`, wrappedExternalStubType("bytes_Buffer"), externalStubBorrowExpr("src"))
	}
	if stubs["io_Writer"] {
		fmt.Fprintf(out, `        if let Some(dst) = (&_arg0 as &dyn std::any::Any).downcast_ref::<%s>() {
            if let Some(writer) = %s.as_ref() {
                writer.__go_write_bytes(&data);
            }
        }
        if let Some(dst) = (&_arg0 as &dyn std::any::Any).downcast_ref::<io_Writer>() {
            dst.__go_write_bytes(&data);
        }
`, wrappedExternalStubType("io_Writer"), externalStubBorrowExpr("dst"))
	}
	if stubs["bytes_Buffer"] {
		fmt.Fprintf(out, `        if let Some(dst) = (&_arg0 as &dyn std::any::Any).downcast_ref::<%s>() {
            if let Some(buffer) = %s.as_ref() {
                buffer.__go_write_bytes(&data);
            }
        }
        if let Some(dst) = (&_arg0 as &dyn std::any::Any).downcast_ref::<bytes_Buffer>() {
            dst.__go_write_bytes(&data);
        }
`, wrappedExternalStubType("bytes_Buffer"), externalStubBorrowExpr("dst"))
	}
	if stubs["os_File"] {
		fmt.Fprintf(out, `        if let Some(dst) = (&_arg0 as &dyn std::any::Any).downcast_ref::<%s>() {
            if let Some(file) = %s.as_ref() {
                file.__go_write_bytes(&data);
            }
        }
        if let Some(dst) = (&_arg0 as &dyn std::any::Any).downcast_ref::<os_File>() {
            dst.__go_write_bytes(&data);
        }
`, wrappedExternalStubType("os_File"), externalStubBorrowExpr("dst"))
	}
	out.WriteString("        (")
	writeExternalStubReturnValue(out, fn.ReturnTypes[0], "i64", "data.len() as i64")
	out.WriteString(", ")
	writeExternalStubDefaultValue(out, fn.ReturnTypes[1])
	out.WriteString(")\n    }\n")
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
	innerWrapper := GetInnerWrapperType()
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

// MACHINERY: const default-value emitter for stub variable declarations.
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
	case "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "usize":
		out.WriteString("0")
	case "f32", "f64":
		out.WriteString("0.0")
	default:
		out.WriteString(rustType)
	}
}
