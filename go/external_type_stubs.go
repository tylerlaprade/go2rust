package main

import (
	"go/ast"
	"go/types"
	"slices"
	"strings"
)

var externalTypeStubs = make(map[string]bool)
var externalTypeStubFields = make(map[string]map[string]string)

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
	fieldTypeRust := goTypesTypeToRustWrapped(fieldType)
	fields := currentExternalTypeStubFields()
	if fields[typeName] == nil {
		fields[typeName] = make(map[string]string)
	}
	fields[typeName][fieldName] = fieldTypeRust
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

func isKnownStdlibHelperType(pkgPath string, name string) bool {
	switch pkgPath {
	case "context":
		return name == "Context" || name == "CancelFunc" || name == "CancelCauseFunc"
	case "net/url":
		return name == "URL"
	case "reflect":
		return name == "StructField" || name == "StructTag" || name == "Type"
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
	if currentContext != nil && currentContext.File != nil {
		if currentContext.File.ExternalTypeStubs == nil {
			currentContext.File.ExternalTypeStubs = make(map[string]bool)
		}
		return currentContext.File.ExternalTypeStubs
	}
	return externalTypeStubs
}

func currentExternalTypeStubFields() map[string]map[string]string {
	if currentContext != nil && currentContext.File != nil {
		if currentContext.File.ExternalTypeStubFields == nil {
			currentContext.File.ExternalTypeStubFields = make(map[string]map[string]string)
		}
		return currentContext.File.ExternalTypeStubFields
	}
	return externalTypeStubFields
}

func GenerateExternalTypeStubs() string {
	stubs := currentExternalTypeStubs()
	if len(stubs) == 0 {
		return ""
	}

	names := make([]string, 0, len(stubs))
	for name := range stubs {
		names = append(names, name)
	}
	slices.Sort(names)

	var out strings.Builder
	fieldsByType := currentExternalTypeStubFields()
	for i, name := range names {
		if i > 0 {
			out.WriteString("\n\n")
		}
		fields := fieldsByType[name]
		if len(fields) == 0 {
			out.WriteString("#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]\n")
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
	}
	return out.String()
}
