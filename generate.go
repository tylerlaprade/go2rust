package main

import (
	"strings"
)

func Generate(program *RustProgram) string {
	var result strings.Builder

	// Generate imports
	for _, imp := range program.Imports {
		result.WriteString(imp)
		result.WriteString("\n")
	}
	result.WriteString("\n")

	// Generate functions
	for _, function := range program.Functions {
		result.WriteString("fn ")
		result.WriteString(function.Name)
		result.WriteString("() {\n")

		// Generate function body
		for _, stmt := range function.Body {
			result.WriteString("    ")
			generateStatement(&result, stmt)
			result.WriteString("\n")
		}

		result.WriteString("}")
	}

	return result.String()
}

func generateStatement(result *strings.Builder, stmt RustStatement) {
	result.WriteString(stmt.Target)
	result.WriteString("(")
	for i, arg := range stmt.Args {
		if i > 0 {
			result.WriteString(", ")
		}
		result.WriteString(arg)
	}
	result.WriteString(");")
}
