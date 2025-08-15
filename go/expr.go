package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

// ExprContext represents how an expression is being used
type ExprContext int

const (
	RValue    ExprContext = iota // Expression is being read
	LValue                       // Expression is being written to
	AddressOf                    // Expression is having its address taken
)

// isFloatExpression checks if an expression involves floats
func isFloatExpression(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		typ := typeInfo.GetType(expr)
		if typ != nil {
			if basic, ok := typ.Underlying().(*types.Basic); ok {
				return basic.Kind() == types.Float32 || basic.Kind() == types.Float64
			}
		}
	}

	// Fallback: only check for float literals, never guess based on names
	switch e := expr.(type) {
	case *ast.BasicLit:
		return e.Kind == token.FLOAT
	case *ast.BinaryExpr:
		// Recursively check operands
		return isFloatExpression(e.X) || isFloatExpression(e.Y)
	case *ast.ParenExpr:
		return isFloatExpression(e.X)
	}
	return false
}

// TranspileExpression transpiles an expression (defaults to RValue context)
func TranspileExpression(out *strings.Builder, expr ast.Expr) {
	TranspileExpressionContext(out, expr, RValue)
}

// TranspileExpressionContext transpiles an expression with context about how it's used
func TranspileExpressionContext(out *strings.Builder, expr ast.Expr, ctx ExprContext) {
	switch e := expr.(type) {
	case *ast.BasicLit:
		switch e.Kind {
		case token.STRING:
			out.WriteString(e.Value)
			out.WriteString(".to_string()")
		case token.CHAR:
			// In Go, character literals are runes (int32)
			// Convert 'A' to the numeric value
			out.WriteString("(")
			out.WriteString(e.Value)
			out.WriteString(" as i32)")
		default:
			out.WriteString(e.Value)
		}

	case *ast.Ident:
		// Check if this variable has been renamed (captured in closure)
		varName := e.Name
		if currentCaptureRenames != nil {
			if renamed, exists := currentCaptureRenames[e.Name]; exists {
				varName = renamed
			}
		}

		if e.Name == "nil" {
			out.WriteString("None")
		} else if currentReceiver != "" && e.Name == currentReceiver {
			// Method receiver - translate to self
			// Check if this is a type definition that needs unwrapping
			if _, isTypeDef := typeDefinitions[currentReceiverType]; isTypeDef {
				// For type definitions, access the inner value
				out.WriteString("(*self.0.lock().unwrap().as_ref().unwrap())")
			} else {
				out.WriteString("self")
			}
		} else if e.Name[0] >= 'A' && e.Name[0] <= 'Z' && e.Name != "String" {
			// Likely a constant - convert to UPPER_SNAKE_CASE
			out.WriteString(strings.ToUpper(ToSnakeCase(e.Name)))
		} else if e.Name == "true" || e.Name == "false" || e.Name == "_" {
			out.WriteString(e.Name)
		} else if varType, isRangeVar := rangeLoopVars[e.Name]; isRangeVar {
			// Check if this is a wrapped type (contains Arc)
			if strings.Contains(varType, "Arc") {
				// It's a wrapped value from a map, need to unwrap for display
				if ctx == RValue {
					out.WriteString("(*")
					out.WriteString(varName)
					out.WriteString(".lock().unwrap().as_mut().unwrap())")
				} else {
					out.WriteString(varName)
				}
			} else {
				// Simple type (like usize for array indices)
				out.WriteString(varName)
			}
		} else if _, isLocalConst := localConstants[e.Name]; isLocalConst {
			out.WriteString(varName)
		} else {
			// All variables are wrapped in Arc<Mutex<Option<T>>>
			switch ctx {
			case RValue:
				// Reading a variable requires unwrapping to get the inner value
				out.WriteString("(*")
				out.WriteString(varName)
				out.WriteString(".lock().unwrap().as_mut().unwrap())")
			case AddressOf:
				// Taking address just returns the Arc itself
				out.WriteString(varName)
			case LValue:
				// Writing to a variable - we'll handle the actual assignment in AssignStmt
				out.WriteString(varName)
			}
		}

	case *ast.CallExpr:
		TranspileCall(out, e)

	case *ast.SelectorExpr:
		// Check if this is a package selector or field access using TypeInfo
		typeInfo := GetTypeInfo()
		isPackageSelector := false

		if typeInfo != nil && typeInfo.info != nil {
			// Check if this is a package selector
			if ident, ok := e.X.(*ast.Ident); ok {
				if obj, ok := typeInfo.info.Uses[ident]; ok {
					if _, ok := obj.(*types.PkgName); ok {
						isPackageSelector = true
					}
				}
			}
		}

		if isPackageSelector {
			// Package/type selector
			TranspileExpression(out, e.X)
			out.WriteString("::")
			out.WriteString(ToSnakeCase(e.Sel.Name))
		} else if ident, ok := e.X.(*ast.Ident); ok {
			// Field access on a variable
			if currentReceiver != "" && ident.Name == currentReceiver {
				// Field access on method receiver - use self directly
				fieldPath := resolveFieldAccess(currentReceiverType, e.Sel.Name)
				out.WriteString("self.")
				out.WriteString(fieldPath)
				// For return statements, we need to clone the Arc
				if ctx == RValue {
					out.WriteString(".clone()")
				}
			} else {
				// Regular field access - try to resolve through type info
				var fieldPath string

				if typeInfo != nil {
					// Try to get the type of the expression
					if t := typeInfo.GetType(e.X); t != nil {
						// Extract the struct type name
						typeStr := t.String()
						// Remove package prefix if present
						if idx := strings.LastIndex(typeStr, "."); idx >= 0 {
							typeStr = typeStr[idx+1:]
						}
						// Remove pointer prefix if present
						typeStr = strings.TrimPrefix(typeStr, "*")

						fieldPath = resolveFieldAccess(typeStr, e.Sel.Name)
					} else {
						fieldPath = ToSnakeCase(e.Sel.Name)
					}
				} else {
					fieldPath = ToSnakeCase(e.Sel.Name)
				}

				TranspileExpression(out, e.X)
				out.WriteString(".")
				out.WriteString(fieldPath)
			}
		} else {
			// Complex expression for X (not just an identifier)
			TranspileExpression(out, e.X)
			out.WriteString(".")
			out.WriteString(ToSnakeCase(e.Sel.Name))
		}

	case *ast.UnaryExpr:
		switch e.Op {
		case token.AND: // & - address-of
			// Check if we're taking address of a struct literal
			if _, isCompositeLit := e.X.(*ast.CompositeLit); isCompositeLit {
				// For struct literals, wrap the whole thing in Arc<Mutex<Option<>>>
				out.WriteString("Arc::new(Mutex::new(Some(")
				TranspileExpressionContext(out, e.X, AddressOf)
				out.WriteString(")))")
			} else {
				// Taking address of existing value just clones the Arc
				TranspileExpressionContext(out, e.X, AddressOf)
				out.WriteString(".clone()")
			}
		case token.MUL: // * - dereference
			out.WriteString("(*")
			TranspileExpression(out, e.X)
			out.WriteString(".lock().unwrap().as_ref().unwrap().lock().unwrap().as_ref().unwrap())")
		default:
			out.WriteString(e.Op.String())
			TranspileExpression(out, e.X)
		}

	case *ast.StarExpr:
		// Dereference pointer - unwrap the Arc<Mutex<Option<T>>> to get T
		out.WriteString("(*")
		// Use LValue context so the identifier doesn't get unwrapped
		TranspileExpressionContext(out, e.X, LValue)
		if ctx == RValue {
			out.WriteString(".lock().unwrap().as_ref().unwrap())")
		} else {
			out.WriteString(".lock().unwrap().as_mut().unwrap())")
		}
	case *ast.BinaryExpr:
		// Special handling for comparisons with nil
		if ident, ok := e.Y.(*ast.Ident); ok && ident.Name == "nil" {
			// Check if left side is the receiver (self)
			if leftIdent, ok := e.X.(*ast.Ident); ok && currentReceiver != "" && leftIdent.Name == currentReceiver {
				// Receiver nil check - this is a Go pattern that doesn't translate well
				// In Rust, methods can't be called on None values
				// We'll generate a false condition since self is never None in a method
				if e.Op.String() == "!=" {
					out.WriteString("true") // self is always != nil in a method
				} else if e.Op.String() == "==" {
					out.WriteString("false") // self is never == nil in a method
				}
				return
			}

			if e.Op.String() == "!=" {
				out.WriteString("(*")
				TranspileExpressionContext(out, e.X, LValue)
				out.WriteString(".lock().unwrap()).is_some()")
				return
			} else if e.Op.String() == "==" {
				out.WriteString("(*")
				TranspileExpressionContext(out, e.X, LValue)
				out.WriteString(".lock().unwrap()).is_none()")
				return
			}
		}

		// Special handling for string concatenation
		if e.Op == token.ADD {
			// Check if this might be string concatenation
			isStringConcat := false
			if lit, ok := e.X.(*ast.BasicLit); ok && lit.Kind == token.STRING {
				isStringConcat = true
			} else if lit, ok := e.Y.(*ast.BasicLit); ok && lit.Kind == token.STRING {
				isStringConcat = true
			}

			if isStringConcat {
				// Use format! for string concatenation
				out.WriteString("format!(\"{}{}\"")
				out.WriteString(", ")
				TranspileExpression(out, e.X)
				out.WriteString(", ")
				TranspileExpression(out, e.Y)
				out.WriteString(")")
				return
			}
		}

		// Use type info to determine if operands need unwrapping
		typeInfo := GetTypeInfo()
		needsUnwrapX := false
		needsUnwrapY := false

		if typeInfo != nil {
			needsUnwrapX = typeInfo.NeedsUnwrapping(e.X)
			needsUnwrapY = typeInfo.NeedsUnwrapping(e.Y)
		} else {
			// Fallback to old logic if no type info
			if currentReceiver != "" {
				// In a method, field accesses return wrapped values
				if _, ok := e.X.(*ast.SelectorExpr); ok {
					needsUnwrapX = true
				}
				if _, ok := e.Y.(*ast.SelectorExpr); ok {
					needsUnwrapY = true
				}
			}
		}

		if needsUnwrapX || needsUnwrapY {
			// At least one operand needs unwrapping
			if needsUnwrapX {
				out.WriteString("(*")
				TranspileExpression(out, e.X)
				out.WriteString(".lock().unwrap().as_ref().unwrap())")
			} else {
				TranspileExpression(out, e.X)
			}
			out.WriteString(" ")
			out.WriteString(e.Op.String())
			out.WriteString(" ")
			if needsUnwrapY {
				out.WriteString("(*")
				TranspileExpression(out, e.Y)
				out.WriteString(".lock().unwrap().as_ref().unwrap())")
			} else {
				TranspileExpression(out, e.Y)
			}
		} else {
			// No unwrapping needed
			// Special handling for numeric literals with float operations
			if lit, ok := e.X.(*ast.BasicLit); ok && lit.Kind == token.INT {
				// Check if the other operand might be a float
				if isFloatExpression(e.Y) {
					out.WriteString(lit.Value)
					out.WriteString(".0")
				} else {
					TranspileExpression(out, e.X)
				}
			} else {
				TranspileExpression(out, e.X)
			}

			out.WriteString(" ")
			out.WriteString(e.Op.String())
			out.WriteString(" ")

			if lit, ok := e.Y.(*ast.BasicLit); ok && lit.Kind == token.INT {
				// Check if the other operand might be a float
				if isFloatExpression(e.X) {
					out.WriteString(lit.Value)
					out.WriteString(".0")
				} else {
					TranspileExpression(out, e.Y)
				}
			} else {
				TranspileExpression(out, e.Y)
			}
		}

	case *ast.IndexExpr:
		// Use type information to determine if this is a map access
		typeInfo := GetTypeInfo()
		isMap := false

		if typeInfo != nil {
			isMap = typeInfo.IsMap(e.X)
		} else {
			// Type info not available - add error comment
			out.WriteString("/* ERROR: Cannot determine if map or slice access - type information required */ ")
			// Generate unimplemented to make the error obvious
			out.WriteString("unimplemented!(\"type info required for index expression\")")
			return
		}

		if isMap && ctx == RValue {
			// Map read access - need to clone the value
			out.WriteString("(*(*")
			if ident, ok := e.X.(*ast.Ident); ok {
				out.WriteString(ident.Name)
			} else {
				TranspileExpression(out, e.X)
			}
			out.WriteString(".lock().unwrap().as_ref().unwrap()).get(")
			// Check if the index is a range loop variable that's already a reference
			if ident, ok := e.Index.(*ast.Ident); ok {
				if _, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar {
					// Range variables from slice iteration are already references
					out.WriteString(ident.Name)
				} else {
					// Normal variables need &
					out.WriteString("&")
					TranspileExpression(out, e.Index)
				}
			} else {
				// Non-identifier expressions need &
				out.WriteString("&")
				TranspileExpression(out, e.Index)
			}
			out.WriteString(").unwrap().lock().unwrap().as_ref().unwrap())")
		} else {
			// Regular array/slice/string indexing
			// Check if it's a string (returns a byte)
			typeInfo := GetTypeInfo()
			isString := false
			if typeInfo != nil {
				basicKind := typeInfo.GetBasicKind(e.X)
				isString = (basicKind == types.String)
			}

			if isString {
				// String indexing returns a byte (u8)
				out.WriteString("(*")
				TranspileExpression(out, e.X)
				out.WriteString(".lock().unwrap().as_ref().unwrap()).as_bytes()[")
				// Check if index needs unwrapping
				if typeInfo != nil && typeInfo.NeedsUnwrapping(e.Index) {
					out.WriteString("(*")
					TranspileExpression(out, e.Index)
					out.WriteString(".lock().unwrap().as_ref().unwrap()) as usize")
				} else {
					TranspileExpression(out, e.Index)
					out.WriteString(" as usize")
				}
				out.WriteString("]")
			} else {
				// Array/slice indexing
				out.WriteString("(*")
				// Use LValue context so identifiers don't unwrap themselves
				TranspileExpressionContext(out, e.X, LValue)
				out.WriteString(".lock().unwrap().as_ref().unwrap())[")
				// Index handling - identifiers will unwrap themselves in RValue context
				TranspileExpression(out, e.Index)
				out.WriteString(" as usize]")
				// Array/slice elements are wrapped, so we need to clone
				out.WriteString(".clone()")
			}
		}

	case *ast.SliceExpr:
		// Slice expressions like arr[1:] or s[0:5]
		// The array/slice is wrapped, so we need to unwrap it first
		out.WriteString("Arc::new(Mutex::new(Some(")
		out.WriteString("(*")
		// Use LValue context so identifiers don't unwrap themselves
		TranspileExpressionContext(out, e.X, LValue)
		out.WriteString(".lock().unwrap().as_ref().unwrap())")
		out.WriteString("[")
		if e.Low != nil {
			// Indices will unwrap themselves in RValue context if needed
			TranspileExpression(out, e.Low)
			out.WriteString(" as usize")
		}
		out.WriteString("..")
		if e.High != nil {
			// Indices will unwrap themselves in RValue context if needed
			TranspileExpression(out, e.High)
			out.WriteString(" as usize")
		}
		out.WriteString("].to_vec()")
		out.WriteString(")))")

	case *ast.CompositeLit:
		// Handle array/slice literals
		if arrayType, ok := e.Type.(*ast.ArrayType); ok {
			// Wrap the entire array/slice in Arc<Mutex<Option<>>>
			out.WriteString("Arc::new(Mutex::new(Some(")
			if arrayType.Len != nil {
				// Fixed-size array
				out.WriteString("[")
			} else {
				// Slice
				out.WriteString("vec![")
			}
			for i, elt := range e.Elts {
				if i > 0 {
					out.WriteString(", ")
				}
				TranspileExpression(out, elt)
			}
			out.WriteString("]")
			out.WriteString(")))")
		} else if mapType, ok := e.Type.(*ast.MapType); ok {
			// Map literal - wrap the whole map in Arc<Mutex<Option<>>>
			TrackImport("Arc", "map literal")
			TrackImport("Mutex", "map literal")
			TrackImport("HashMap", "map literal")
			out.WriteString("Arc::new(Mutex::new(Some(HashMap::<")
			out.WriteString(goTypeToRustBase(mapType.Key))
			out.WriteString(", ")
			out.WriteString(GoTypeToRust(mapType.Value))
			out.WriteString(">::from([")
			for i, elt := range e.Elts {
				if i > 0 {
					out.WriteString(", ")
				}
				if kv, ok := elt.(*ast.KeyValueExpr); ok {
					out.WriteString("(")
					TranspileExpression(out, kv.Key)
					out.WriteString(", ")
					// Wrap map values in Arc<Mutex<Option<>>>
					out.WriteString("Arc::new(Mutex::new(Some(")
					TranspileExpression(out, kv.Value)
					out.WriteString(")))")
					out.WriteString(")")
				}
			}
			out.WriteString("]))))")
		} else if ident, ok := e.Type.(*ast.Ident); ok {
			// Struct literal
			out.WriteString(ident.Name)
			out.WriteString(" { ")
			for i, elt := range e.Elts {
				if i > 0 {
					out.WriteString(", ")
				}
				if kv, ok := elt.(*ast.KeyValueExpr); ok {
					if key, ok := kv.Key.(*ast.Ident); ok {
						out.WriteString(ToSnakeCase(key.Name))
						out.WriteString(": ")
						// Check if the value is an identifier (parameter/variable)
						if valIdent, ok := kv.Value.(*ast.Ident); ok {
							// It's already wrapped, just clone it
							out.WriteString(valIdent.Name)
							out.WriteString(".clone()")
						} else {
							// Wrap field values in Arc<Mutex<Option<T>>>
							out.WriteString("Arc::new(Mutex::new(Some(")
							TranspileExpression(out, kv.Value)
							out.WriteString(")))")
						}
					}
				}
			}
			out.WriteString(" }")
		}

	case *ast.ParenExpr:
		// Parenthesized expression
		out.WriteString("(")
		TranspileExpression(out, e.X)
		out.WriteString(")")

	case *ast.TypeAssertExpr:
		// Handle type assertions like value.(Type)
		// Type assertions work on interface{} values (Box<dyn Any>)
		if e.Type != nil {
			// Get the Rust type for the assertion
			rustType := ""
			if ident, ok := e.Type.(*ast.Ident); ok {
				switch ident.Name {
				case "string":
					rustType = "String"
				case "int":
					rustType = "i32"
				case "int8":
					rustType = "i8"
				case "int16":
					rustType = "i16"
				case "int32", "rune":
					rustType = "i32"
				case "int64":
					rustType = "i64"
				case "uint":
					rustType = "u32"
				case "uint8", "byte":
					rustType = "u8"
				case "uint16":
					rustType = "u16"
				case "uint32":
					rustType = "u32"
				case "uint64":
					rustType = "u64"
				case "bool":
					rustType = "bool"
				case "float32":
					rustType = "f32"
				case "float64":
					rustType = "f64"
				default:
					rustType = ident.Name
				}
			} else {
				// Complex type - use the base type
				rustType = goTypeToRustBase(e.Type)
			}

			// Generate type assertion that panics on failure (for single-value context)
			// The comma-ok form is handled specially in assignment statements
			out.WriteString("({\n")
			out.WriteString("        let val = ")
			// Check if e.X is an identifier (simple variable)
			if ident, ok := e.X.(*ast.Ident); ok && ident.Name != "nil" {
				out.WriteString(ident.Name)
			} else {
				TranspileExpression(out, e.X)
			}
			out.WriteString(".clone();\n")
			out.WriteString("        let guard = val.lock().unwrap();\n")
			out.WriteString("        if let Some(ref any_val) = *guard {\n")
			out.WriteString("            any_val.downcast_ref::<")
			out.WriteString(rustType)
			out.WriteString(">().expect(\"type assertion failed\").clone()\n")
			out.WriteString("        } else {\n")
			out.WriteString("            panic!(\"type assertion on nil interface\")\n")
			out.WriteString("        }\n")
			out.WriteString("    })")
		}

	case *ast.FuncLit:
		// Function literal (closure/anonymous function)
		TranspileFuncLit(out, e)

	default:
		// Unhandled expression type
		out.WriteString("/* TODO: Unhandled expression type: ")
		out.WriteString(strings.TrimPrefix(fmt.Sprintf("%T", e), "*ast."))
		out.WriteString(" */ Arc::new(Mutex::new(Some(())))")
	}
}

// Helper to check if an identifier is a function (not a closure variable)
func isFunctionName(ident *ast.Ident) bool {
	// Use go/types to properly determine if this is a function
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		return typeInfo.IsFunction(ident)
	}

	// Fallback: if no type info, assume it's not a function
	// This ensures we don't make incorrect assumptions
	return false
}

// Helper to check if a name is a builtin function
func isBuiltinFunction(name string) bool {
	builtins := map[string]bool{
		"len": true, "cap": true, "make": true, "new": true,
		"append": true, "copy": true, "delete": true,
		"panic": true, "recover": true, "print": true, "println": true,
	}
	return builtins[name]
}

// TranspileFuncLit transpiles a function literal (closure)
func TranspileFuncLit(out *strings.Builder, funcLit *ast.FuncLit) {
	// Find captured variables
	captured := findCapturedVars(funcLit)

	// Build capture renames but don't generate clones here
	// The clones need to be generated at the statement level
	captureRenames := make(map[string]string)
	for varName := range captured {
		// For now, just use the original names
		// A proper implementation would handle this at statement level
		captureRenames[varName] = varName
	}

	// Store current capture renames for nested transpilation
	oldCaptureRenames := currentCaptureRenames
	currentCaptureRenames = captureRenames
	defer func() { currentCaptureRenames = oldCaptureRenames }()

	// Wrap the closure in Arc<Mutex<Option<Box<dyn Fn>>>
	out.WriteString("Arc::new(Mutex::new(Some(")

	// Generate the closure wrapped in Box
	out.WriteString("Box::new(move |")

	// Parameters
	if funcLit.Type.Params != nil {
		var params []string
		for _, field := range funcLit.Type.Params.List {
			paramType := GoTypeToRust(field.Type)
			for _, name := range field.Names {
				params = append(params, name.Name+": "+paramType)
			}
			// Handle unnamed parameters
			if len(field.Names) == 0 {
				params = append(params, "_: "+paramType)
			}
		}
		out.WriteString(strings.Join(params, ", "))
	}
	out.WriteString("| ")

	// Return type
	if funcLit.Type.Results != nil && len(funcLit.Type.Results.List) > 0 {
		out.WriteString("-> ")
		if len(funcLit.Type.Results.List) == 1 && len(funcLit.Type.Results.List[0].Names) == 0 {
			// Single unnamed return
			out.WriteString(GoTypeToRust(funcLit.Type.Results.List[0].Type))
		} else {
			// Multiple returns
			var retTypes []string
			for _, field := range funcLit.Type.Results.List {
				retType := GoTypeToRust(field.Type)
				count := len(field.Names)
				if count == 0 {
					count = 1
				}
				for i := 0; i < count; i++ {
					retTypes = append(retTypes, retType)
				}
			}
			out.WriteString("(" + strings.Join(retTypes, ", ") + ")")
		}
		out.WriteString(" ")
	}

	// Body
	out.WriteString("{\n")
	if funcLit.Body != nil {
		for _, stmt := range funcLit.Body.List {
			out.WriteString("        ") // Indent for closure body
			TranspileStatementSimple(out, stmt, funcLit.Type, nil)
			out.WriteString("\n")
		}
	}
	out.WriteString("    })")

	// Cast to the right type and close wrappers
	out.WriteString(" as ")
	out.WriteString(generateClosureType(funcLit.Type))
	out.WriteString(")))")
}

// TranspileTypeConversion handles type conversions like int(x), float64(y), etc.
func TranspileTypeConversion(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 1 {
		// Not a type conversion
		return
	}

	// Check for []byte(string) and []rune(string) conversions
	if compLit, ok := call.Fun.(*ast.ArrayType); ok {
		if compLit.Len == nil { // It's a slice
			elemType := ""
			if ident, ok := compLit.Elt.(*ast.Ident); ok {
				elemType = ident.Name
			}

			switch elemType {
			case "byte", "uint8":
				// []byte(string) conversion
				out.WriteString("Arc::new(Mutex::new(Some(")
				if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
					out.WriteString("(*")
					out.WriteString(ident.Name)
					out.WriteString(".lock().unwrap().as_ref().unwrap()).as_bytes().to_vec()")
				} else {
					out.WriteString("(*")
					TranspileExpression(out, call.Args[0])
					out.WriteString(".lock().unwrap().as_ref().unwrap()).as_bytes().to_vec()")
				}
				out.WriteString(")))")
				return
			case "rune", "int32":
				// []rune(string) conversion
				out.WriteString("Arc::new(Mutex::new(Some(")
				if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
					out.WriteString("(*")
					out.WriteString(ident.Name)
					out.WriteString(".lock().unwrap().as_ref().unwrap()).chars().map(|c| c as i32).collect::<Vec<_>>()")
				} else {
					out.WriteString("(*")
					TranspileExpression(out, call.Args[0])
					out.WriteString(".lock().unwrap().as_ref().unwrap()).chars().map(|c| c as i32).collect::<Vec<_>>()")
				}
				out.WriteString(")))")
				return
			}
		}
	}

	targetType := ""
	if ident, ok := call.Fun.(*ast.Ident); ok {
		targetType = ident.Name
	} else if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
		// Handle package.Type conversions
		targetType = sel.Sel.Name
	}

	// Map Go types to Rust types and handle the conversion
	rustType := ""
	needsCast := true

	switch targetType {
	// Integer types
	case "int":
		rustType = "i32"
	case "int8":
		rustType = "i8"
	case "int16":
		rustType = "i16"
	case "int32":
		rustType = "i32"
	case "int64":
		rustType = "i64"
	case "uint":
		rustType = "u32"
	case "uint8", "byte":
		rustType = "u8"
	case "uint16":
		rustType = "u16"
	case "uint32":
		rustType = "u32"
	case "uint64":
		rustType = "u64"
	case "uintptr":
		rustType = "usize"
	// Float types
	case "float32":
		rustType = "f32"
	case "float64":
		rustType = "f64"
	// String conversions
	case "string":
		// Special handling for string conversions
		arg := call.Args[0]
		typeInfo := GetTypeInfo()
		if typeInfo != nil {
			argType := typeInfo.GetType(arg)
			if argType != nil {
				// Check if converting from []byte or []rune
				if slice, ok := argType.Underlying().(*types.Slice); ok {
					elemType := slice.Elem()
					if basic, ok := elemType.(*types.Basic); ok {
						if basic.Kind() == types.Byte || basic.Kind() == types.Uint8 {
							// []byte to string
							out.WriteString("Arc::new(Mutex::new(Some(String::from_utf8(")
							if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
								out.WriteString("(*")
								out.WriteString(ident.Name)
								out.WriteString(".lock().unwrap().as_ref().unwrap()).clone()")
							} else {
								out.WriteString("(*")
								TranspileExpression(out, arg)
								out.WriteString(".lock().unwrap().as_ref().unwrap()).clone()")
							}
							out.WriteString(").unwrap())))")
							return
						} else if basic.Kind() == types.Rune || basic.Kind() == types.Int32 {
							// []rune to string
							out.WriteString("Arc::new(Mutex::new(Some(")
							if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
								out.WriteString("(*")
								out.WriteString(ident.Name)
								out.WriteString(".lock().unwrap().as_ref().unwrap())")
							} else {
								out.WriteString("(*")
								TranspileExpression(out, arg)
								out.WriteString(".lock().unwrap().as_ref().unwrap())")
							}
							out.WriteString(".iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>())))")
							return
						}
					}
				} else if basic, ok := argType.Underlying().(*types.Basic); ok {
					if basic.Kind() == types.Rune || basic.Kind() == types.Int32 {
						// Single rune to string
						out.WriteString("Arc::new(Mutex::new(Some(")
						out.WriteString("char::from_u32((")
						if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
							out.WriteString("*")
							out.WriteString(ident.Name)
							out.WriteString(".lock().unwrap().as_ref().unwrap()")
						} else {
							out.WriteString("*")
							TranspileExpression(out, arg)
							out.WriteString(".lock().unwrap().as_ref().unwrap()")
						}
						out.WriteString(") as u32).unwrap().to_string())))")
						return
					}
				}
			}
		}
		// Default string conversion
		out.WriteString("Arc::new(Mutex::new(Some(")
		if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString("(*")
			out.WriteString(ident.Name)
			out.WriteString(".lock().unwrap().as_ref().unwrap()).to_string()")
		} else {
			out.WriteString("(*")
			TranspileExpression(out, arg)
			out.WriteString(".lock().unwrap().as_ref().unwrap()).to_string()")
		}
		out.WriteString(")))")
		return
	case "rune":
		rustType = "i32" // rune is an alias for int32
	// Complex types
	case "complex64":
		out.WriteString("Arc::new(Mutex::new(Some(num::Complex::<f32>::new(")
		if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString("(*")
			out.WriteString(ident.Name)
			out.WriteString(".lock().unwrap().as_ref().unwrap()) as f32")
		} else {
			out.WriteString("(*")
			TranspileExpression(out, call.Args[0])
			out.WriteString(".lock().unwrap().as_ref().unwrap()) as f32")
		}
		out.WriteString(", 0.0))))")
		return
	case "complex128":
		out.WriteString("Arc::new(Mutex::new(Some(num::Complex::<f64>::new(")
		if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString("(*")
			out.WriteString(ident.Name)
			out.WriteString(".lock().unwrap().as_ref().unwrap()) as f64")
		} else {
			out.WriteString("(*")
			TranspileExpression(out, call.Args[0])
			out.WriteString(".lock().unwrap().as_ref().unwrap()) as f64")
		}
		out.WriteString(", 0.0))))")
		return
	default:
		// Check for custom type definitions
		if _, isTypeDef := typeDefinitions[targetType]; isTypeDef {
			// Custom type definition
			out.WriteString(targetType)
			out.WriteString("(Arc::new(Mutex::new(Some(")
			TranspileExpression(out, call.Args[0])
			out.WriteString("))))")
			return
		}
		// Unknown type, just pass through
		needsCast = false
	}

	if needsCast && rustType != "" {
		// Perform the type cast
		out.WriteString("Arc::new(Mutex::new(Some((")
		// Check if the argument is a simple identifier (variable)
		if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
			// It's a variable, unwrap it directly
			out.WriteString("*")
			out.WriteString(ident.Name)
			out.WriteString(".lock().unwrap().as_ref().unwrap()")
		} else {
			// It's an expression, evaluate and unwrap
			out.WriteString("*")
			TranspileExpression(out, call.Args[0])
			out.WriteString(".lock().unwrap().as_ref().unwrap()")
		}
		out.WriteString(") as ")
		out.WriteString(rustType)
		out.WriteString(")))")
	} else {
		// No cast needed or unknown type
		TranspileExpression(out, call.Args[0])
	}
}

// TranspileTypeAssertionCommaOk generates code for type assertion with comma-ok form
func TranspileTypeAssertionCommaOk(out *strings.Builder, e *ast.TypeAssertExpr) {
	if e.Type == nil {
		return
	}

	// Get the Rust type for the assertion
	rustType := ""
	defaultValue := ""
	if ident, ok := e.Type.(*ast.Ident); ok {
		switch ident.Name {
		case "string":
			rustType = "String"
			defaultValue = "String::new()"
		case "int":
			rustType = "i32"
			defaultValue = "0"
		case "int8":
			rustType = "i8"
			defaultValue = "0"
		case "int16":
			rustType = "i16"
			defaultValue = "0"
		case "int32", "rune":
			rustType = "i32"
			defaultValue = "0"
		case "int64":
			rustType = "i64"
			defaultValue = "0"
		case "uint":
			rustType = "u32"
			defaultValue = "0"
		case "uint8", "byte":
			rustType = "u8"
			defaultValue = "0"
		case "uint16":
			rustType = "u16"
			defaultValue = "0"
		case "uint32":
			rustType = "u32"
			defaultValue = "0"
		case "uint64":
			rustType = "u64"
			defaultValue = "0"
		case "bool":
			rustType = "bool"
			defaultValue = "false"
		case "float32":
			rustType = "f32"
			defaultValue = "0.0"
		case "float64":
			rustType = "f64"
			defaultValue = "0.0"
		default:
			rustType = ident.Name
			defaultValue = "Default::default()"
		}
	} else {
		// Complex type - use the base type
		rustType = goTypeToRustBase(e.Type)
		defaultValue = "Default::default()"
	}

	// Generate the type assertion code that returns (value, ok)
	out.WriteString("({\n")
	out.WriteString("        let val = ")
	// Check if e.X is an identifier (simple variable)
	if ident, ok := e.X.(*ast.Ident); ok && ident.Name != "nil" {
		out.WriteString(ident.Name)
	} else {
		TranspileExpression(out, e.X)
	}
	out.WriteString(".clone();\n")
	out.WriteString("        let guard = val.lock().unwrap();\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n")
	out.WriteString("            if let Some(typed_val) = any_val.downcast_ref::<")
	out.WriteString(rustType)
	out.WriteString(">() {\n")
	out.WriteString("                (Arc::new(Mutex::new(Some(typed_val.clone()))), Arc::new(Mutex::new(Some(true))))\n")
	out.WriteString("            } else {\n")
	out.WriteString("                (Arc::new(Mutex::new(Some(")
	out.WriteString(defaultValue)
	out.WriteString("))), Arc::new(Mutex::new(Some(false))))\n")
	out.WriteString("            }\n")
	out.WriteString("        } else {\n")
	out.WriteString("            (Arc::new(Mutex::new(Some(")
	out.WriteString(defaultValue)
	out.WriteString("))), Arc::new(Mutex::new(Some(false))))\n")
	out.WriteString("        }\n")
	out.WriteString("    })")
}

func TranspileCall(out *strings.Builder, call *ast.CallExpr) {
	// Check if this is a stdlib function we need to replace
	if handler := GetStdlibHandler(call); handler != nil {
		handler(out, call)
		return
	}

	// Check if this is a type conversion
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.IsTypeConversion(call) {
		// Handle type conversion
		TranspileTypeConversion(out, call)
		return
	}

	// Check if this is a type conversion for a type definition
	if ident, ok := call.Fun.(*ast.Ident); ok {
		if _, isTypeDef := typeDefinitions[ident.Name]; isTypeDef {
			// This is a type definition constructor
			out.WriteString(ident.Name)
			out.WriteString("(Arc::new(Mutex::new(Some(")
			if len(call.Args) > 0 {
				TranspileExpression(out, call.Args[0])
			}
			out.WriteString("))))")
			return
		}
	}

	// Check if this is a method call (selector expression)
	if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
		// This is a method call - handle it specially
		TranspileExpression(out, sel.X)
		out.WriteString(".")
		out.WriteString(ToSnakeCase(sel.Sel.Name))
		out.WriteString("(")
		for i, arg := range call.Args {
			if i > 0 {
				out.WriteString(", ")
			}
			// For method calls, wrap arguments normally
			out.WriteString("Arc::new(Mutex::new(Some(")
			TranspileExpression(out, arg)
			out.WriteString(")))")
		}
		out.WriteString(")")
		return
	}

	// Check if this is a closure call (calling a variable that holds a function)
	if ident, ok := call.Fun.(*ast.Ident); ok {
		// Check if this is a known function or a variable
		if isBuiltinFunction(ident.Name) || isFunctionName(ident) {
			// Regular function call
			out.WriteString(ToSnakeCase(ident.Name))
		} else {
			// Likely a closure variable - need to unwrap and call
			out.WriteString("(")
			out.WriteString(ident.Name)
			out.WriteString(".lock().unwrap().as_ref().unwrap())")
		}
	} else {
		// Complex expression for the function (e.g., function returning a function)
		out.WriteString("(")
		TranspileExpression(out, call.Fun)
		out.WriteString(".lock().unwrap().as_ref().unwrap())")
	}

	out.WriteString("(")
	for i, arg := range call.Args {
		if i > 0 {
			out.WriteString(", ")
		}

		// Check if we're calling a closure - closures take wrapped arguments
		isClosureCall := false
		if ident, ok := call.Fun.(*ast.Ident); ok {
			isClosureCall = !isBuiltinFunction(ident.Name) && !isFunctionName(ident)
		} else {
			// Complex expression, likely a closure
			isClosureCall = true
		}

		// Wrap arguments appropriately
		handler := GetStdlibHandler(call)
		if isClosureCall || handler == nil {
			// Check if the argument is already a wrapped variable
			if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "_" {
				// Check if this is a variable (not a constant)
				if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
					if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
						// It's a variable, just clone it
						out.WriteString(ident.Name)
						out.WriteString(".clone()")
					} else {
						// It's a constant, wrap it
						out.WriteString("Arc::new(Mutex::new(Some(")
						TranspileExpression(out, arg)
						out.WriteString(")))")
					}
				} else {
					// Range variable, wrap it
					out.WriteString("Arc::new(Mutex::new(Some(")
					TranspileExpression(out, arg)
					out.WriteString(")))")
				}
			} else {
				// Not a simple identifier, wrap it
				out.WriteString("Arc::new(Mutex::new(Some(")
				TranspileExpression(out, arg)
				out.WriteString(")))")
			}
		} else {
			TranspileExpression(out, arg)
		}
	}
	out.WriteString(")")
}
