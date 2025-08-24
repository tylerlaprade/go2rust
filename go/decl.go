package main

import (
	"fmt"
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
	"math"
	"strconv"
	"strings"
)

// Helper to check if a function body contains defer statements
func checkHasDefer(stmts []ast.Stmt) bool {
	for _, stmt := range stmts {
		switch s := stmt.(type) {
		case *ast.DeferStmt:
			return true
		case *ast.BlockStmt:
			if checkHasDefer(s.List) {
				return true
			}
		case *ast.IfStmt:
			if s.Body != nil && checkHasDefer(s.Body.List) {
				return true
			}
			if s.Else != nil {
				if elseBlock, ok := s.Else.(*ast.BlockStmt); ok {
					if checkHasDefer(elseBlock.List) {
						return true
					}
				}
			}
		case *ast.ForStmt:
			if s.Body != nil && checkHasDefer(s.Body.List) {
				return true
			}
		case *ast.RangeStmt:
			if s.Body != nil && checkHasDefer(s.Body.List) {
				return true
			}
		}
	}
	return false
}

func TranspileFunction(out *strings.Builder, fn *ast.FuncDecl, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	// Check if this is a method (has receiver)
	if fn.Recv != nil && len(fn.Recv.List) > 0 {
		// Methods will be collected and generated in impl blocks
		// For now, skip them here
		return
	}

	// Regular function
	if fn.Name.Name != "main" {
		out.WriteString("pub ")
	}
	out.WriteString("fn ")
	out.WriteString(ToSnakeCase(fn.Name.Name))
	out.WriteString("(")

	// Parameters
	if fn.Type.Params != nil {
		for i, field := range fn.Type.Params.List {
			if i > 0 {
				out.WriteString(", ")
			}
			for j, name := range field.Names {
				if j > 0 {
					out.WriteString(", ")
				}
				out.WriteString(name.Name)
				out.WriteString(": ")
				out.WriteString(GoTypeToRust(field.Type))
			}
		}
	}

	out.WriteString(")")

	// Return type
	if fn.Type.Results != nil && len(fn.Type.Results.List) > 0 {
		out.WriteString(" -> ")
		if len(fn.Type.Results.List) == 1 && len(fn.Type.Results.List[0].Names) <= 1 {
			// Single return value
			out.WriteString(GoTypeToRust(fn.Type.Results.List[0].Type))
		} else {
			// Multiple return values - use tuple
			out.WriteString("(")
			first := true
			for _, result := range fn.Type.Results.List {
				// Handle multiple names with same type
				if len(result.Names) > 0 {
					for range result.Names {
						if !first {
							out.WriteString(", ")
						}
						first = false
						out.WriteString(GoTypeToRust(result.Type))
					}
				} else {
					// Unnamed return value
					if !first {
						out.WriteString(", ")
					}
					first = false
					out.WriteString(GoTypeToRust(result.Type))
				}
			}
			out.WriteString(")")
		}
	}

	out.WriteString(" {\n")

	// Check if this function uses defer statements
	hasDefer := checkHasDefer(fn.Body.List)
	currentFunctionHasDefer = hasDefer

	// Initialize defer stack if needed
	if hasDefer {
		out.WriteString("    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();\n")
		// We'll execute defers before each return statement
		out.WriteString("\n")
	}

	// Declare named return values as mutable variables
	if fn.Type.Results != nil {
		for _, result := range fn.Type.Results.List {
			if len(result.Names) > 0 {
				for _, name := range result.Names {
					out.WriteString("    let mut ")
					out.WriteString(name.Name)
					out.WriteString(": ")
					out.WriteString(GoTypeToRust(result.Type))
					// Initialize with wrapped default values
					out.WriteString(" = ")
					WriteWrapperPrefix(out)
					switch t := result.Type.(type) {
					case *ast.Ident:
						switch t.Name {
						case "string":
							out.WriteString("Some(String::new())")
						case "int", "int64", "int32", "int16", "int8":
							out.WriteString("Some(0)")
						case "uint", "uint64", "uint32", "uint16", "uint8":
							out.WriteString("Some(0)")
						case "float64", "float32":
							out.WriteString("Some(0.0)")
						case "bool":
							out.WriteString("Some(false)")
						case "error":
							// error type is already Option, so we use None directly
							out.WriteString("None")
						default:
							out.WriteString("Some(Default::default())")
						}
					default:
						out.WriteString("Some(Default::default())")
					}
					out.WriteString("))")
					out.WriteString(";\n")
				}
			}
		}
		if len(fn.Type.Results.List) > 0 {
			out.WriteString("\n")
		}
	}

	// Function body
	var prevStmt ast.Stmt
	var lastPos token.Pos = fn.Body.Lbrace
	for _, stmt := range fn.Body.List {
		// Add blank line if there was one in the source
		if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
			out.WriteString("\n")
		}

		out.WriteString("    ")
		TranspileStatement(out, stmt, fn.Type, fileSet, comments, &lastPos, "    ")
		out.WriteString("\n")

		prevStmt = stmt
	}

	// Execute defers at the end if needed
	if hasDefer {
		out.WriteString("\n    // Execute deferred functions\n")
		out.WriteString("    while let Some(f) = __defer_stack.pop() {\n")
		out.WriteString("        f();\n")
		out.WriteString("    }\n")
	}

	out.WriteString("}")
}

// getEmbeddedFieldName extracts the type name from an embedded field
func getEmbeddedFieldName(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		return t.Name
	case *ast.StarExpr:
		// For pointer types, get the underlying type name
		return getEmbeddedFieldName(t.X)
	case *ast.SelectorExpr:
		// For qualified types like pkg.Type
		return t.Sel.Name
	default:
		// Fallback to a generic name
		return "embedded"
	}
}

func TranspileTypeDecl(out *strings.Builder, typeSpec *ast.TypeSpec, genDecl *ast.GenDecl) {
	switch t := typeSpec.Type.(type) {
	case *ast.StructType:
		// Track struct definition
		structDef := &StructDef{
			Fields:        make(map[string]string),
			EmbeddedTypes: []string{},
		}

		// First pass: collect field information
		for _, field := range t.Fields.List {
			if len(field.Names) > 0 {
				// Named fields
				for _, name := range field.Names {
					structDef.Fields[name.Name] = "regular"
				}
			} else {
				// Embedded field
				typeName := getEmbeddedFieldName(field.Type)
				structDef.EmbeddedTypes = append(structDef.EmbeddedTypes, typeName)
			}
		}

		structDefs[typeSpec.Name.Name] = structDef

		out.WriteString("#[derive(Debug, Clone, Default)]\n")
		out.WriteString("struct ")
		out.WriteString(typeSpec.Name.Name)
		out.WriteString(" {\n")

		for _, field := range t.Fields.List {
			// Add struct tag as comment if present
			if field.Tag != nil && field.Tag.Value != "" {
				out.WriteString("    // tags: ")
				out.WriteString(field.Tag.Value)
				out.WriteString("\n")
			}

			if len(field.Names) > 0 {
				// Handle multiple names on one line (e.g., X, Y int)
				for _, name := range field.Names {
					out.WriteString("    ")
					out.WriteString(ToSnakeCase(name.Name))
					out.WriteString(": ")
					out.WriteString(GoTypeToRust(field.Type))
					out.WriteString(",\n")
				}
			} else {
				// Embedded field - extract the type name
				fieldName := getEmbeddedFieldName(field.Type)
				out.WriteString("    ")
				out.WriteString(ToSnakeCase(fieldName))
				out.WriteString(": ")
				out.WriteString(GoTypeToRust(field.Type))
				out.WriteString(",\n")
			}
		}

		out.WriteString("}")

	case *ast.InterfaceType:
		// Generate a trait for the interface
		out.WriteString("trait ")
		out.WriteString(typeSpec.Name.Name)
		out.WriteString(" {\n")

		// Generate method signatures
		for _, method := range t.Methods.List {
			if len(method.Names) > 0 {
				// Named method
				funcType, ok := method.Type.(*ast.FuncType)
				if !ok {
					continue
				}

				out.WriteString("    fn ")
				out.WriteString(ToSnakeCase(method.Names[0].Name))
				out.WriteString("(&self")

				// Add other parameters
				if funcType.Params != nil && len(funcType.Params.List) > 0 {
					for _, param := range funcType.Params.List {
						out.WriteString(", ")
						for j, name := range param.Names {
							if j > 0 {
								out.WriteString(", ")
							}
							out.WriteString(name.Name)
							out.WriteString(": ")
							out.WriteString(GoTypeToRust(param.Type))
						}
					}
				}

				out.WriteString(")")

				// Return type
				if funcType.Results != nil && len(funcType.Results.List) > 0 {
					out.WriteString(" -> ")
					if len(funcType.Results.List) == 1 && len(funcType.Results.List[0].Names) <= 1 {
						// Single return value
						out.WriteString(GoTypeToRust(funcType.Results.List[0].Type))
					} else {
						// Multiple return values - use tuple
						out.WriteString("(")
						first := true
						for _, result := range funcType.Results.List {
							if len(result.Names) > 0 {
								for range result.Names {
									if !first {
										out.WriteString(", ")
									}
									first = false
									out.WriteString(GoTypeToRust(result.Type))
								}
							} else {
								if !first {
									out.WriteString(", ")
								}
								first = false
								out.WriteString(GoTypeToRust(result.Type))
							}
						}
						out.WriteString(")")
					}
				}

				out.WriteString(";\n")
			}
		}

		out.WriteString("}")

	default:
		// Handle type aliases and type definitions
		if typeSpec.Assign != 0 {
			// Type alias: type A = B
			out.WriteString("type ")
			out.WriteString(typeSpec.Name.Name)
			out.WriteString(" = ")
			out.WriteString(GoTypeToRust(t))
			out.WriteString(";\n")

			// Track this as a type alias
			typeAliases[typeSpec.Name.Name] = true
		} else {
			// Type definition: type A B
			// Create a newtype wrapper in Rust
			out.WriteString("#[derive(Debug, Clone)]\n")
			out.WriteString("struct ")
			out.WriteString(typeSpec.Name.Name)
			out.WriteString("(")
			out.WriteString(GoTypeToRust(t))
			out.WriteString(");\n")

			// Add Display implementation for numeric type definitions
			if ident, ok := t.(*ast.Ident); ok {
				typeDefinitions[typeSpec.Name.Name] = ident.Name

				// Add Display impl for numeric types
				if ident.Name == "int" || ident.Name == "int64" || ident.Name == "float64" ||
					ident.Name == "float32" || ident.Name == "uint" || ident.Name == "uint64" {
					// Track necessary imports
					TrackImport("Display")
					TrackImport("Formatter")
					TrackImport("fmt")

					out.WriteString("\nimpl Display for ")
					out.WriteString(typeSpec.Name.Name)
					out.WriteString(" {\n")
					out.WriteString("    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n")
					out.WriteString("        write!(f, \"{}\", self.0")
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())\n")
					out.WriteString("    }\n")
					out.WriteString("}\n")
				}
			}
		}
	}
}

func TranspileConstDecl(out *strings.Builder, genDecl *ast.GenDecl) {
	transpileConstDeclWithCase(out, genDecl, true)
}

func transpileConstDeclWithCase(out *strings.Builder, genDecl *ast.GenDecl, toUpper bool) {
	// Track iota value and the last expression pattern for each position
	iotaValue := 0
	var lastExpressions []ast.Expr

	for specIndex, spec := range genDecl.Specs {
		if valueSpec, ok := spec.(*ast.ValueSpec); ok {
			// Set iota for this spec
			iotaValue = specIndex

			// Update lastExpressions if this spec has values
			if len(valueSpec.Values) > 0 {
				lastExpressions = valueSpec.Values
			}

			for i, name := range valueSpec.Names {
				// Skip blank identifier
				if name.Name == "_" {
					continue
				}
				out.WriteString("const ")
				var constName string
				if toUpper {
					constName = strings.ToUpper(ToSnakeCase(name.Name))
				} else {
					// Keep original name for local constants
					constName = name.Name
					// Track local constants with their actual type
					var constType string
					if valueSpec.Type != nil {
						constType = goTypeToRustBase(valueSpec.Type)
					} else if len(valueSpec.Values) > i && valueSpec.Values[i] != nil {
						constType = inferConstType(valueSpec.Values[i])
					} else if len(lastExpressions) > i && lastExpressions[i] != nil {
						constType = inferConstType(lastExpressions[i])
					} else {
						constType = "i32"
					}
					localConstants[name.Name] = constType
				}
				out.WriteString(constName)
				out.WriteString(": ")

				// Determine type - constants should not be wrapped
				if valueSpec.Type != nil {
					// For const string type, use &'static str
					if ident, ok := valueSpec.Type.(*ast.Ident); ok && ident.Name == "string" {
						out.WriteString("&'static str")
					} else {
						// Check if this is a type definition and get underlying type
						baseType := goTypeToRustBase(valueSpec.Type)
						// If it's a custom type (like Color), we need to use the underlying type
						// For now, assume custom int types map to i32
						if ident, ok := valueSpec.Type.(*ast.Ident); ok {
							// Check if this is a known type definition
							if _, isTypeDef := typeDefinitions[ident.Name]; isTypeDef {
								// Type definitions for constants should use the underlying type
								// For now, assume int-based type definitions
								baseType = "i32"
							}
						}
						out.WriteString(baseType)
					}
				} else if len(valueSpec.Values) > i && valueSpec.Values[i] != nil {
					// Infer type from value
					out.WriteString(inferConstType(valueSpec.Values[i]))
				} else if len(lastExpressions) > i && lastExpressions[i] != nil {
					// Infer type from the last expression pattern
					out.WriteString(inferConstType(lastExpressions[i]))
				} else {
					// Default to i32 for iota
					out.WriteString("i32")
				}

				out.WriteString(" = ")

				// Handle value
				if len(valueSpec.Values) > i && valueSpec.Values[i] != nil {
					// Replace iota with actual value
					TranspileConstExpr(out, valueSpec.Values[i], iotaValue)
				} else if len(lastExpressions) > i && lastExpressions[i] != nil {
					// Use the corresponding expression from lastExpressions for this position
					TranspileConstExpr(out, lastExpressions[i], iotaValue)
				} else if len(lastExpressions) > 0 && lastExpressions[0] != nil {
					// If we don't have an expression for this position, use the first one
					TranspileConstExpr(out, lastExpressions[0], iotaValue)
				} else {
					// No previous expression pattern, just use iota value
					out.WriteString(fmt.Sprintf("%d", iotaValue))
				}

				out.WriteString(";\n")
			}
		}
	}
}

func inferConstType(expr ast.Expr) string {
	switch e := expr.(type) {
	case *ast.BasicLit:
		switch e.Kind {
		case token.INT:
			// Check if the value might overflow i32
			if val, err := strconv.ParseInt(e.Value, 0, 64); err == nil {
				if val > math.MaxInt32 || val < math.MinInt32 {
					return "i64"
				}
			}
			return "i32"
		case token.FLOAT:
			return "f64"
		case token.STRING:
			return "&'static str"
		}
	case *ast.Ident:
		if e.Name == "true" || e.Name == "false" {
			return "bool"
		}
		// Check if it's a known constant
		if constType, exists := localConstants[e.Name]; exists {
			return constType
		}
	case *ast.BinaryExpr:
		// For binary expressions, check the type of operands
		leftType := inferConstType(e.X)
		if leftType == "&'static str" {
			return "&'static str"
		}
		rightType := inferConstType(e.Y)
		if rightType == "&'static str" {
			return "&'static str"
		}
		// For bit shift operations that might overflow, use i64
		if e.Op == token.SHL {
			// Try to evaluate if this might overflow
			return "i64"
		}
		// If either operand is i64, result is i64
		if leftType == "i64" || rightType == "i64" {
			return "i64"
		}
		// Default to i32 for other numeric operations
		return "i32"
	}
	return "i32" // default
}

// Helper function to check if an expression is a string constant
func isStringConstExpr(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.BasicLit:
		return e.Kind == token.STRING
	case *ast.Ident:
		if constType, exists := localConstants[e.Name]; exists {
			return constType == "&'static str"
		}
		return false
	case *ast.BinaryExpr:
		// String concatenation
		if e.Op == token.ADD {
			return isStringConstExpr(e.X) || isStringConstExpr(e.Y)
		}
		return false
	}
	return false
}

// Helper function to fully evaluate a const string expression including identifiers
func evaluateConstStringExpr(expr ast.Expr) string {
	switch e := expr.(type) {
	case *ast.BasicLit:
		if e.Kind == token.STRING {
			// Remove quotes
			str := e.Value
			if len(str) >= 2 && str[0] == '"' && str[len(str)-1] == '"' {
				return str[1 : len(str)-1]
			}
		}
	case *ast.Ident:
		// Look up the value of the constant using TypeInfo
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.info != nil {
			if obj, ok := typeInfo.info.Uses[e]; ok {
				if constObj, ok := obj.(*types.Const); ok {
					if constObj.Val() != nil {
						// Extract the string value from the constant
						return constant.StringVal(constObj.Val())
					}
				}
			}
		}
		// Type info not available or not a constant
		return ""
	case *ast.BinaryExpr:
		if e.Op == token.ADD {
			left := evaluateConstStringExpr(e.X)
			right := evaluateConstStringExpr(e.Y)
			if left != "" || right != "" {
				return left + right
			}
		}
	}
	return ""
}

func TranspileConstExpr(out *strings.Builder, expr ast.Expr, iotaValue int) {
	switch e := expr.(type) {
	case *ast.BasicLit:
		if e.Kind == token.STRING {
			// For const strings, use &str instead of String
			out.WriteString(e.Value)
		} else {
			out.WriteString(e.Value)
		}
	case *ast.Ident:
		if e.Name == "iota" {
			out.WriteString(fmt.Sprintf("%d", iotaValue))
		} else if e.Name == "true" || e.Name == "false" {
			// Boolean literals
			out.WriteString(e.Name)
		} else if _, exists := localConstants[e.Name]; exists {
			// Local constant - keep original name
			out.WriteString(e.Name)
		} else if e.Name[0] >= 'a' && e.Name[0] <= 'z' {
			// Package-level constant reference - convert to uppercase
			out.WriteString(strings.ToUpper(ToSnakeCase(e.Name)))
		} else {
			out.WriteString(e.Name)
		}
	case *ast.BinaryExpr:
		// Special handling for string concatenation in const context
		if e.Op == token.ADD && isStringConstExpr(e.X) && isStringConstExpr(e.Y) {
			// For string concatenation in const context, try to evaluate at compile time
			result := evaluateConstStringExpr(expr)
			if result != "" {
				// Successfully evaluated the entire expression
				out.WriteString(fmt.Sprintf("%q", result))
			} else {
				// Fall back - this won't work for const but at least generates something
				out.WriteString("/* TODO: Complex string concatenation in const */ ")
				out.WriteString(`""`)
			}
		} else {
			// Handle binary expressions in const context
			TranspileConstExpr(out, e.X, iotaValue)
			out.WriteString(" ")
			out.WriteString(e.Op.String())
			out.WriteString(" ")
			TranspileConstExpr(out, e.Y, iotaValue)
		}
	case *ast.ParenExpr:
		out.WriteString("(")
		TranspileConstExpr(out, e.X, iotaValue)
		out.WriteString(")")
	default:
		// Fallback to regular expression transpilation
		TranspileExpression(out, expr)
	}
}

// TranspileMethodImpl transpiles a method inside an impl block
func TranspileMethodImpl(out *strings.Builder, fn *ast.FuncDecl, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	transpileMethodImplWithVisibility(out, fn, true, fileSet, comments)
}

func transpileMethodImplWithVisibility(out *strings.Builder, fn *ast.FuncDecl, addPub bool, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	// Store the receiver name and type for self translation
	if fn.Recv != nil && len(fn.Recv.List) > 0 {
		recv := fn.Recv.List[0]
		if len(recv.Names) > 0 {
			currentReceiver = recv.Names[0].Name
		}
		// Store the receiver type
		currentReceiverType = getReceiverType(recv.Type)
	}

	// Output doc comments if present (with indentation for methods)
	outputComment(out, fn.Doc, "    ", true)

	out.WriteString("    ")
	if addPub {
		out.WriteString("pub ")
	}
	out.WriteString("fn ")
	out.WriteString(ToSnakeCase(fn.Name.Name))
	out.WriteString("(")

	// Receiver
	if fn.Recv != nil && len(fn.Recv.List) > 0 {
		recv := fn.Recv.List[0]
		// Store the receiver name for self translation
		if len(recv.Names) > 0 {
			currentReceiver = recv.Names[0].Name
		}

		// Check if pointer receiver
		if _, isPointer := recv.Type.(*ast.StarExpr); isPointer {
			out.WriteString("&mut self")
		} else {
			out.WriteString("&self")
		}

		// Add comma if there are more parameters
		if fn.Type.Params != nil && len(fn.Type.Params.List) > 0 {
			out.WriteString(", ")
		}
	}
	// Other parameters
	if fn.Type.Params != nil {
		for i, field := range fn.Type.Params.List {
			if i > 0 {
				out.WriteString(", ")
			}
			for j, name := range field.Names {
				if j > 0 {
					out.WriteString(", ")
				}
				out.WriteString(name.Name)
				out.WriteString(": ")
				out.WriteString(GoTypeToRust(field.Type))
			}
		}
	}

	out.WriteString(")")

	// Return type
	if fn.Type.Results != nil && len(fn.Type.Results.List) > 0 {
		out.WriteString(" -> ")
		if len(fn.Type.Results.List) == 1 && len(fn.Type.Results.List[0].Names) <= 1 {
			// Single return value
			out.WriteString(GoTypeToRust(fn.Type.Results.List[0].Type))
		} else {
			// Multiple return values - use tuple
			out.WriteString("(")
			first := true
			for _, result := range fn.Type.Results.List {
				// Handle multiple names with same type
				if len(result.Names) > 0 {
					for range result.Names {
						if !first {
							out.WriteString(", ")
						}
						first = false
						out.WriteString(GoTypeToRust(result.Type))
					}
				} else {
					// Unnamed return value
					if !first {
						out.WriteString(", ")
					}
					first = false
					out.WriteString(GoTypeToRust(result.Type))
				}
			}
			out.WriteString(")")
		}
	}

	out.WriteString(" {\n")

	// Method body - need to handle self references
	var lastPos token.Pos = fn.Body.Lbrace
	for _, stmt := range fn.Body.List {
		out.WriteString("        ")
		TranspileStatement(out, stmt, fn.Type, fileSet, comments, &lastPos, "        ")
		out.WriteString("\n")
	}

	out.WriteString("    }\n")

	// Clear the receiver name
	currentReceiver = ""
}
