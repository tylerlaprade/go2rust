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

// generateStructDisplay generates a Display implementation for a struct to match Go's output format
func generateStructDisplay(out *strings.Builder, structName string, structType *ast.StructType) {
	TrackImport("Display")
	TrackImport("Formatter")

	// If this type implements the error interface, Display should delegate to error()
	if IsErrorImplType(structName) {
		out.WriteString("impl std::fmt::Display for ")
		out.WriteString(structName)
		out.WriteString(" {\n")
		out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
		out.WriteString("        write!(f, \"{}\", (*self.error()")
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()))\n")
		out.WriteString("    }\n")
		out.WriteString("}\n")
		return
	}

	out.WriteString("impl std::fmt::Display for ")
	out.WriteString(structName)
	out.WriteString(" {\n")
	out.WriteString("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n")
	out.WriteString("        write!(f, \"{{")

	// Collect all fields (including embedded)
	type fieldEntry struct {
		name        string
		isEmbedded  bool
		isSlice     bool
		isMap       bool
		isInterface bool
		isFunction  bool
	}
	var fields []fieldEntry
	for _, field := range structType.Fields.List {
		// Skip sync types (Mutex, WaitGroup) — they're not data fields
		if isSyncParam(field.Type) {
			continue
		}
		_, isSlice := field.Type.(*ast.ArrayType)
		_, isMap := field.Type.(*ast.MapType)
		isInterface := isEmptyInterfaceExpr(field.Type)
		_, isFunction := field.Type.(*ast.FuncType)
		_, isChannel := field.Type.(*ast.ChanType)
		if isChannel {
			continue
		}
		if len(field.Names) > 0 {
			for _, name := range field.Names {
				fields = append(fields, fieldEntry{
					name:        name.Name,
					isEmbedded:  false,
					isSlice:     isSlice,
					isMap:       isMap,
					isInterface: isInterface,
					isFunction:  isFunction,
				})
			}
		} else {
			// Embedded field
			typeName := getEmbeddedFieldName(field.Type)
			fields = append(fields, fieldEntry{
				name:        typeName,
				isEmbedded:  true,
				isSlice:     isSlice,
				isMap:       isMap,
				isInterface: isInterface,
				isFunction:  isFunction,
			})
		}
	}

	// Generate format string with placeholders
	for i := range fields {
		if i > 0 {
			out.WriteString(" ")
		}
		out.WriteString("{}")
	}
	out.WriteString("}}\"")

	// Add field values
	for _, f := range fields {
		out.WriteString(", ")
		if f.isInterface {
			NeedFormatAny()
			out.WriteString("format_any(self.")
			out.WriteString(ToSnakeCase(f.name))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap().as_ref())")
		} else if f.isFunction {
			out.WriteString("\"<func>\"")
		} else if f.isMap {
			NeedFormatMap()
			out.WriteString("format_map(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else if f.isSlice {
			NeedFormatSlice()
			out.WriteString("format_slice(&self.")
			out.WriteString(ToSnakeCase(f.name))
			out.WriteString(")")
		} else {
			out.WriteString("(*self.")
			out.WriteString(ToSnakeCase(f.name))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		}
	}

	out.WriteString(")\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func structHasTraitField(structType *ast.StructType) bool {
	for _, field := range structType.Fields.List {
		if typeHasTraitField(field.Type) {
			return true
		}
	}
	return false
}

func structNeedsCustomDefault(structType *ast.StructType) bool {
	for _, field := range structType.Fields.List {
		if structFieldNeedsCustomDefault(field.Type) {
			return true
		}
	}
	return false
}

func structFieldNeedsCustomDefault(expr ast.Expr) bool {
	switch t := expr.(type) {
	case *ast.StructType:
		return true
	case *ast.Ident:
		_, isStruct := structDefs[t.Name]
		return isStruct
	default:
		return false
	}
}

func writeStructDerive(out *strings.Builder, structName string, structType *ast.StructType) {
	hasTraitField := structHasTraitField(structType)
	needsCustomDefault := structNeedsCustomDefault(structType)
	needsPartialEq := !hasTraitField && structName != "" && comparableStructTypes[structName]
	if hasTraitField {
		if needsCustomDefault {
			out.WriteString("#[derive(Clone)]\n")
		} else {
			out.WriteString("#[derive(Clone, Default)]\n")
		}
	} else {
		if needsCustomDefault {
			if needsPartialEq {
				out.WriteString("#[derive(Debug, Clone, PartialEq)]\n")
			} else {
				out.WriteString("#[derive(Debug, Clone)]\n")
			}
		} else {
			if needsPartialEq {
				out.WriteString("#[derive(Debug, Clone, Default, PartialEq)]\n")
			} else {
				out.WriteString("#[derive(Debug, Clone, Default)]\n")
			}
		}
	}
}

func writeStructDefaultValue(out *strings.Builder, fieldType ast.Expr) {
	if nestedStruct, ok := fieldType.(*ast.StructType); ok {
		nestedName := generateAnonymousStructType(nestedStruct)
		WriteWrapperPrefix(out)
		out.WriteString(nestedName)
		out.WriteString("::default()")
		WriteWrapperSuffix(out)
		return
	}
	if fieldIdent, ok := fieldType.(*ast.Ident); ok {
		if _, isStruct := structDefs[fieldIdent.Name]; isStruct {
			WriteWrapperPrefix(out)
			out.WriteString(fieldIdent.Name)
			out.WriteString("::default()")
			WriteWrapperSuffix(out)
			return
		}
	}
	if isSyncParam(fieldType) {
		out.WriteString(goTypeToRustBase(fieldType))
		out.WriteString("::new()")
		return
	}
	out.WriteString("Default::default()")
}

func generateStructDefault(out *strings.Builder, structName string, structType *ast.StructType) {
	if !structNeedsCustomDefault(structType) {
		return
	}
	out.WriteString("\nimpl Default for ")
	out.WriteString(structName)
	out.WriteString(" {\n")
	out.WriteString("    fn default() -> Self {\n")
	out.WriteString("        Self { ")
	needComma := false
	for _, field := range structType.Fields.List {
		if len(field.Names) > 0 {
			for _, name := range field.Names {
				if needComma {
					out.WriteString(", ")
				}
				needComma = true
				out.WriteString(ToSnakeCase(name.Name))
				out.WriteString(": ")
				writeStructDefaultValue(out, field.Type)
			}
		} else {
			if needComma {
				out.WriteString(", ")
			}
			needComma = true
			fieldName := getEmbeddedFieldName(field.Type)
			out.WriteString(ToSnakeCase(fieldName))
			out.WriteString(": ")
			writeStructDefaultValue(out, field.Type)
		}
	}
	out.WriteString(" }\n")
	out.WriteString("    }\n")
	out.WriteString("}\n")
}

func typeHasTraitField(expr ast.Expr) bool {
	fieldType := goTypeToRustBase(expr)
	if strings.Contains(fieldType, "dyn ") {
		return true
	}

	switch t := expr.(type) {
	case *ast.ArrayType:
		return typeHasTraitField(t.Elt)
	case *ast.MapType:
		return typeHasTraitField(t.Value)
	case *ast.StructType:
		return structHasTraitField(t)
	default:
		return false
	}
}

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

func isOsArgsSelector(sel *ast.SelectorExpr) bool {
	if sel.Sel.Name != "Args" {
		return false
	}

	ident, ok := sel.X.(*ast.Ident)
	if !ok {
		return false
	}

	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
		if obj, ok := typeInfo.info.Uses[ident]; ok {
			if pkgName, ok := obj.(*types.PkgName); ok {
				return pkgName.Imported().Path() == "os"
			}
		}
	}

	return resolveStdlibPackageName(ident.Name) == "os"
}

func functionUsesOsArgs(fn *ast.FuncDecl) bool {
	if fn.Body == nil {
		return false
	}

	usesOsArgs := false
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		if usesOsArgs {
			return false
		}
		if sel, ok := node.(*ast.SelectorExpr); ok && isOsArgsSelector(sel) {
			usesOsArgs = true
			return false
		}
		return true
	})
	return usesOsArgs
}

func TranspileFunction(out *strings.Builder, fn *ast.FuncDecl, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	// Check if this is a method (has receiver)
	if fn.Recv != nil && len(fn.Recv.List) > 0 {
		// Methods will be collected and generated in impl blocks
		// For now, skip them here
		return
	}

	// Register the function signature for later use
	var params []*ast.Field
	if fn.Type.Params != nil {
		params = fn.Type.Params.List
	}
	var results []*ast.Field
	if fn.Type.Results != nil {
		results = fn.Type.Results.List
	}
	RegisterFunctionSignature(fn.Name.Name, &FunctionSignature{
		Params:  params,
		Results: results,
	})

	// Regular function
	if fn.Name.Name != "main" {
		out.WriteString("pub ")
	}
	out.WriteString("fn ")
	out.WriteString(rustFunctionName(fn))
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
				out.WriteString(RustLocalIdent(name.Name))
				out.WriteString(": ")
				out.WriteString(GoTypeToRustParam(field.Type))
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

	// Register function parameters in VarTable
	if vt := GetVarTable(); vt != nil {
		vt.PushScope()
		defer vt.PopScope()
		if fn.Type.Params != nil {
			for _, field := range fn.Type.Params.List {
				for _, name := range field.Names {
					if ident, ok := field.Type.(*ast.Ident); ok && IsInterfaceType(ident.Name) {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  "&dyn " + ident.Name,
							Source:    SourceParam,
							IsRef:     true,
						})
					} else if _, ok := field.Type.(*ast.ChanType); ok {
						// Channel parameters are bare (GoChannel<T>)
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							Source:    SourceParam,
						})
					} else if isSyncParam(field.Type) {
						// sync.WaitGroup / sync.Mutex parameters are bare
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							Source:    SourceParam,
						})
					} else {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapFull,
							Source:    SourceParam,
						})
					}
				}
			}
		}
	}

	// Call package initialization at the start of main() if present
	if fn.Name.Name == "main" && hasInitFunction {
		out.WriteString("    __go_init_all();\n")
	}

	if functionUsesOsArgs(fn) {
		out.WriteString("    let __go_os_args = ")
		WriteWrapperPrefix(out)
		out.WriteString("std::env::args().collect::<Vec<String>>()")
		WriteWrapperSuffix(out)
		out.WriteString(";\n\n")
	}

	if fn.Body == nil {
		out.WriteString("    unimplemented!(\"Go function declaration has no body\");\n")
		out.WriteString("}\n")
		return
	}

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
					if name.Name == "_" {
						out.WriteString("    let ")
					} else {
						out.WriteString("    let mut ")
					}
					out.WriteString(RustLocalIdent(name.Name))
					out.WriteString(": ")
					out.WriteString(GoTypeToRust(result.Type))
					// Initialize with wrapped default values
					out.WriteString(" = ")

					// Special handling for error type
					if t, ok := result.Type.(*ast.Ident); ok && t.Name == "error" {
						// error type is wrapped as Rc<RefCell<Option<Box<dyn Error>>>>
						// We need to write the wrapper manually without the Some()
						if NeedsConcurrentWrapper() {
							TrackImport("Arc")
							TrackImport("Mutex")
							out.WriteString("Arc::new(Mutex::new(None))")
						} else {
							TrackImport("Rc")
							TrackImport("RefCell")
							out.WriteString("Rc::new(RefCell::new(None))")
						}
						out.WriteString(";\n")
						continue
					}

					// For all other types
					WriteWrapperPrefix(out)
					switch t := result.Type.(type) {
					case *ast.Ident:
						switch t.Name {
						case "string":
							out.WriteString("String::new()")
						case "int", "int64", "int32", "int16", "int8":
							out.WriteString("0")
						case "uint", "uint64", "uint32", "uint16", "uint8":
							out.WriteString("0")
						case "float64", "float32":
							out.WriteString("0.0")
						case "bool":
							out.WriteString("false")
						default:
							out.WriteString("Default::default()")
						}
					default:
						out.WriteString("Default::default()")
					}
					out.WriteString(")))")
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
	if functionHasGoto(fn) {
		prevStmt = TranspileGotoStatementList(out, fn.Body.List, fn.Type, fileSet, comments, &lastPos, "    ")
	} else {
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
	}

	// Execute defers at the end if needed.
	// Skip if the last statement was a return — that already emitted cleanup + return,
	// so the trailing block would be unreachable and cause a type error in Rust.
	_, lastIsReturn := prevStmt.(*ast.ReturnStmt)
	if hasDefer && !lastIsReturn {
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

func typeDefinitionUnderlyingName(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		return t.Name
	case *ast.ArrayType:
		prefix := "[]"
		if t.Len != nil {
			prefix = "[_]"
		}
		return prefix + typeDefinitionUnderlyingName(t.Elt)
	case *ast.MapType:
		return "map[" + typeDefinitionUnderlyingName(t.Key) + "]" + typeDefinitionUnderlyingName(t.Value)
	case *ast.StarExpr:
		return "*" + typeDefinitionUnderlyingName(t.X)
	case *ast.SelectorExpr:
		if ident, ok := t.X.(*ast.Ident); ok {
			return ident.Name + "." + t.Sel.Name
		}
		return t.Sel.Name
	case *ast.ChanType:
		return "chan " + typeDefinitionUnderlyingName(t.Value)
	case *ast.FuncType:
		return "func"
	default:
		return fmt.Sprintf("%T", expr)
	}
}

func TranspileTypeDecl(out *strings.Builder, typeSpec *ast.TypeSpec, genDecl *ast.GenDecl) {
	switch t := typeSpec.Type.(type) {
	case *ast.StructType:
		// Track struct definition
		structDef := &StructDef{
			Fields:        make(map[string]string),
			EmbeddedTypes: []string{},
			ASTType:       t,
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

		writeStructDerive(out, typeSpec.Name.Name, t)
		out.WriteString("pub struct ")
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
					out.WriteString("    pub ")
					out.WriteString(ToSnakeCase(name.Name))
					out.WriteString(": ")
					out.WriteString(GoTypeToRust(field.Type))
					out.WriteString(",\n")
				}
			} else {
				// Embedded field - extract the type name
				fieldName := getEmbeddedFieldName(field.Type)
				out.WriteString("    pub ")
				out.WriteString(ToSnakeCase(fieldName))
				out.WriteString(": ")
				out.WriteString(GoTypeToRust(field.Type))
				out.WriteString(",\n")
			}
		}

		out.WriteString("}\n\n")

		generateStructDefault(out, typeSpec.Name.Name, t)
		if structNeedsCustomDefault(t) {
			out.WriteString("\n")
		}

		// Generate Display implementation to match Go's format
		generateStructDisplay(out, typeSpec.Name.Name, t)

	case *ast.InterfaceType:
		// Generate a trait for the interface
		// Add Display and Clone as supertraits
		out.WriteString("pub trait ")
		out.WriteString(typeSpec.Name.Name)
		out.WriteString(": std::fmt::Display {\n")
		TrackImport("Display")

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
							out.WriteString(RustLocalIdent(name.Name))
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
			out.WriteString("pub type ")
			out.WriteString(typeSpec.Name.Name)
			out.WriteString(" = ")
			out.WriteString(GoTypeToRust(t))
			out.WriteString(";\n")

			// Track this as a type alias
			RegisterTypeAlias(typeSpec.Name.Name)
		} else if _, isFuncType := t.(*ast.FuncType); isFuncType {
			// Named function type: type BinaryOp func(int, int) int
			// Emit as a type alias to the callable shape, not a newtype struct
			out.WriteString("pub type ")
			out.WriteString(typeSpec.Name.Name)
			out.WriteString(" = ")
			out.WriteString(GoTypeToRust(t))
			out.WriteString(";\n")

			// Track as a type alias so GoTypeToRust won't double-wrap
			RegisterTypeAlias(typeSpec.Name.Name)
		} else {
			// Type definition: type A B
			// Create a newtype wrapper in Rust
			RegisterTypeDefinition(typeSpec.Name.Name, typeDefinitionUnderlyingName(t))
			out.WriteString("#[derive(Debug, Clone)]\n")
			out.WriteString("pub struct ")
			out.WriteString(typeSpec.Name.Name)
			out.WriteString("(")
			out.WriteString(GoTypeToRust(t))
			out.WriteString(");\n")

			// Add Display implementation for displayable scalar type definitions
			if ident, ok := t.(*ast.Ident); ok {
				// Add Display impl when the underlying Rust type is displayable.
				if isDisplayableDefinedUnderlying(ident.Name) {
					// Track necessary imports
					TrackImport("Display")
					TrackImport("Formatter")
					TrackImport("fmt")

					out.WriteString("\nimpl Display for ")
					out.WriteString(typeSpec.Name.Name)
					out.WriteString(" {\n")
					out.WriteString("    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n")
					if IsStringerImplType(typeSpec.Name.Name) {
						out.WriteString("        write!(f, \"{}\", (*self.string()")
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()))\n")
					} else {
						out.WriteString("        write!(f, \"{}\", self.0")
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap())\n")
					}
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
	var lastType ast.Expr

	for specIndex, spec := range genDecl.Specs {
		if valueSpec, ok := spec.(*ast.ValueSpec); ok {
			// Set iota for this spec
			iotaValue = specIndex

			// Update lastExpressions if this spec has values
			if len(valueSpec.Values) > 0 {
				lastExpressions = valueSpec.Values
			}
			if valueSpec.Type != nil {
				lastType = valueSpec.Type
			}

			for i, name := range valueSpec.Names {
				// Skip blank identifier
				if name.Name == "_" {
					continue
				}
				if toUpper {
					if ast.IsExported(name.Name) {
						out.WriteString("pub ")
					} else {
						out.WriteString("pub(crate) ")
					}
				}
				out.WriteString("const ")
				var constName string
				if toUpper {
					constName = rustConstName(name.Name)
				} else {
					// Keep original name for local constants
					constName = name.Name
					// Track local constants with their actual type
					var constType string
					if valueSpec.Type != nil {
						constType = rustConstTypeForTypeExpr(valueSpec.Type)
					} else if len(valueSpec.Values) == 0 && lastType != nil {
						constType = rustConstTypeForTypeExpr(lastType)
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
					out.WriteString(rustConstTypeForTypeExpr(valueSpec.Type))
				} else if len(valueSpec.Values) == 0 && lastType != nil {
					out.WriteString(rustConstTypeForTypeExpr(lastType))
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
			if value, err := strconv.Unquote(e.Value); err == nil {
				return value
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
			out.WriteString(RustStringLiteral(e.Value))
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
		} else if isConstIdent(e) {
			out.WriteString(rustConstName(e.Name))
		} else if e.Name[0] >= 'a' && e.Name[0] <= 'z' {
			// Package-level constant reference - convert to uppercase
			out.WriteString(rustConstName(e.Name))
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
				out.WriteString(RustStringLiteral(strconv.Quote(result)))
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
	out.WriteString(RustFunctionName(fn.Name.Name))
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
			// Error() methods should use &self since they only read
			if fn.Name.Name == "Error" || fn.Name.Name == "String" {
				out.WriteString("&self")
			} else {
				out.WriteString("&mut self")
			}
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
				out.WriteString(RustLocalIdent(name.Name))
				out.WriteString(": ")
				out.WriteString(GoTypeToRustParam(field.Type))
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

	// Register method parameters in VarTable
	if vt := GetVarTable(); vt != nil {
		vt.PushScope()
		defer vt.PopScope()
		if fn.Type.Params != nil {
			for _, field := range fn.Type.Params.List {
				for _, name := range field.Names {
					if ident, ok := field.Type.(*ast.Ident); ok && IsInterfaceType(ident.Name) {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							RustType:  "&dyn " + ident.Name,
							Source:    SourceParam,
							IsRef:     true,
						})
					} else if _, ok := field.Type.(*ast.ChanType); ok {
						// Channel parameters are bare (GoChannel<T>)
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							Source:    SourceParam,
						})
					} else if isSyncParam(field.Type) {
						// sync.WaitGroup / sync.Mutex parameters are bare
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapNone,
							Source:    SourceParam,
						})
					} else {
						vt.Register(name.Name, &VarInfo{
							WrapLevel: WrapFull,
							Source:    SourceParam,
						})
					}
				}
			}
		}
	}

	// Method body - need to handle self references
	if fn.Body == nil {
		out.WriteString("        unimplemented!(\"Go method declaration has no body\");\n")
		out.WriteString("    }\n")
		currentReceiver = ""
		return
	}

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
