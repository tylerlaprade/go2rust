package main

import (
	"slices"
	"strings"
)

var externalTypeStubs = make(map[string]bool)

func RegisterExternalTypeStub(name string) {
	if name == "" {
		return
	}
	currentExternalTypeStubs()[name] = true
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
	for i, name := range names {
		if i > 0 {
			out.WriteString("\n\n")
		}
		out.WriteString("#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]\n")
		out.WriteString("pub struct ")
		out.WriteString(name)
		out.WriteString(";\n\n")
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
