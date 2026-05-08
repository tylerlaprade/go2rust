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
var externalTypeStubFields = make(map[string]map[string]string)
var externalTypeStubMethods = make(map[string]map[string]externalTypeStubMethod)
var externalTypeStubConversions = make(map[string]map[string]bool)
var externalPackageStubs = make(map[string]*externalPackageStub)

type externalTypeStubMethod struct {
	ParamCount  int
	ReturnTypes []string
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
	recv := selection.Recv()
	if ptr, ok := recv.(*types.Pointer); ok {
		recv = ptr.Elem()
	}
	named, ok := recv.(*types.Named)
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
	recv := selection.Recv()
	if ptr, ok := recv.(*types.Pointer); ok {
		recv = ptr.Elem()
	}
	named, ok := recv.(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return
	}
	if !isStdlibPackage(named.Obj().Pkg().Path()) {
		return
	}
	if isKnownStdlibHelperType(named.Obj().Pkg().Path(), named.Obj().Name()) {
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
	recv := selection.Recv()
	if ptr, ok := recv.(*types.Pointer); ok {
		recv = ptr.Elem()
	}
	named, ok := recv.(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	if !isStdlibPackage(named.Obj().Pkg().Path()) {
		return false
	}
	return !isKnownStdlibHelperType(named.Obj().Pkg().Path(), named.Obj().Name())
}

func RegisterExternalInterfaceMethodsForSource(source types.Type, iface *types.Interface) {
	if source == nil || iface == nil {
		return
	}
	if ptr, ok := source.(*types.Pointer); ok {
		source = ptr.Elem()
	}
	named, ok := source.(*types.Named)
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
		RegisterExternalPackageStubConstant(pkgName, ToSnakeCase(sel.Sel.Name), obj.Type())
	case *types.Var:
		RegisterExternalPackageStubVariable(pkgName, ToSnakeCase(sel.Sel.Name), obj.Type())
	}
}

func RegisterExternalPackageStubFunction(pkgName string, funcName string, sig *types.Signature) {
	if pkgName == "" || funcName == "" || sig == nil {
		return
	}
	trackWrapperImports()
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
	pkg.Variables[varName] = goTypesTypeToRustWrapped(varType)
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

func currentExternalTypeStubs() map[string]bool {
	if usePackageExternalStubs() {
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

func currentExternalTypeStubFields() map[string]map[string]string {
	if usePackageExternalStubs() {
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
	return generateExternalStubs(currentExternalTypeStubs(), currentExternalTypeStubFields(), currentExternalTypeStubMethods(), currentExternalTypeStubConversions(), currentExternalPackageStubs())
}

func GeneratePackageExternalStubs(pkg *PackageState) string {
	if pkg == nil {
		return ""
	}
	return generateExternalStubs(pkg.ExternalTypeStubs, pkg.ExternalTypeStubFields, pkg.ExternalTypeStubMethods, pkg.ExternalTypeStubConversions, pkg.ExternalPackageStubs)
}

func WriteSharedStdlibStubCrate(workDir string, states []*PackageState) error {
	outputDir := filepath.Join(workDir, "vendor", sharedStdlibStubCrateName)
	if err := os.MkdirAll(outputDir, 0755); err != nil {
		return fmt.Errorf("failed to create shared stdlib stub crate: %v", err)
	}

	mergedState := MergeExternalStubPackageStates(states...)
	stubCode := GeneratePackageExternalStubs(mergedState)
	if stubCode != "" {
		stubCode = GenerateExternalStubModuleImports() + "\n" + stubCode
	}

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
		mergeNestedStringMap(merged.ExternalTypeStubFields, state.ExternalTypeStubFields)
		mergeNestedMethodMap(merged.ExternalTypeStubMethods, state.ExternalTypeStubMethods)
		mergeNestedBoolMap(merged.ExternalTypeStubConversions, state.ExternalTypeStubConversions)
		mergeExternalPackageStubs(merged.ExternalPackageStubs, state.ExternalPackageStubs)
	}
	return merged
}

func mergeBoolMap(dst map[string]bool, src map[string]bool) {
	for key, value := range src {
		if value {
			dst[key] = true
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

func generateExternalStubs(stubs map[string]bool, fieldsByType map[string]map[string]string, methodsByType map[string]map[string]externalTypeStubMethod, conversions map[string]map[string]bool, packageStubs map[string]*externalPackageStub) string {
	if len(stubs) == 0 && len(conversions) == 0 && len(packageStubs) == 0 {
		return ""
	}
	names := make([]string, 0, len(stubs))
	for name := range stubs {
		names = append(names, name)
	}
	slices.Sort(names)

	var out strings.Builder
	for i, name := range names {
		if i > 0 {
			out.WriteString("\n\n")
		}
		fields := fieldsByType[name]
		if len(fields) == 0 {
			out.WriteString("#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]\n")
			out.WriteString("pub struct ")
			out.WriteString(name)
			out.WriteString(";\n\n")
		} else {
			out.WriteString("#[derive(Debug, Clone, Default)]\n")
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
	writeExternalTypeStubConversions(&out, conversions)
	writeExternalPackageStubs(&out, packageStubs, len(names) > 0)
	return out.String()
}

func writeExternalTypeStubConversions(out *strings.Builder, conversions map[string]map[string]bool) {
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
			out.WriteString("        Self::default()\n")
			out.WriteString("    }\n")
			out.WriteString("}\n")
		}
	}
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

func writeExternalPackageStubs(out *strings.Builder, packageStubs map[string]*externalPackageStub, needsSeparator bool) {
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
			writeExternalStubConstDefaultValue(out, pkg.Constants[constName])
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

func writeExternalStubConstDefaultValue(out *strings.Builder, rustType string) {
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
