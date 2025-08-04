package main

import (
	"fmt"
	"go/ast"
	"go/token"
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
	switch e := expr.(type) {
	case *ast.BasicLit:
		return e.Kind == token.FLOAT
	case *ast.SelectorExpr:
		// Field access - assume float if it's a common float field name
		name := strings.ToLower(e.Sel.Name)
		return name == "width" || name == "height" || name == "radius" ||
			name == "x" || name == "y" || name == "z"
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
		if e.Kind == token.STRING {
			out.WriteString(e.Value)
			out.WriteString(".to_string()")
		} else {
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
			out.WriteString("self")
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
		// Check if this is a package selector or field access
		// For now, assume lowercase identifiers are variables (field access)
		// and uppercase are packages/types
		if ident, ok := e.X.(*ast.Ident); ok && ident.Name[0] >= 'a' && ident.Name[0] <= 'z' {
			// Field access on a variable
			if currentReceiver != "" && ident.Name == currentReceiver {
				// Field access on method receiver - use self directly
				out.WriteString("self.")
				out.WriteString(ToSnakeCase(e.Sel.Name))
				// For return statements, we need to clone the Arc
				if ctx == RValue {
					out.WriteString(".clone()")
				}
			} else {
				// Regular field access
				TranspileExpression(out, e.X)
				out.WriteString(".")
				out.WriteString(ToSnakeCase(e.Sel.Name))
			}
		} else {
			// Package/type selector
			TranspileExpression(out, e.X)
			out.WriteString("::")
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
		out.WriteString(".lock().unwrap().as_mut().unwrap())")
	case *ast.BinaryExpr:
		// Special handling for comparisons with nil
		if ident, ok := e.Y.(*ast.Ident); ok && ident.Name == "nil" {
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

		// Check if we're in a method and dealing with wrapped values
		needsUnwrap := false
		if currentReceiver != "" {
			// In a method, field accesses return wrapped values
			if _, ok := e.X.(*ast.SelectorExpr); ok {
				needsUnwrap = true
			}
			if _, ok := e.Y.(*ast.SelectorExpr); ok {
				needsUnwrap = true
			}
		}

		if needsUnwrap {
			// Unwrap the operands for arithmetic
			out.WriteString("(*")
			TranspileExpression(out, e.X)
			out.WriteString(".lock().unwrap().as_mut().unwrap())")
			out.WriteString(" ")
			out.WriteString(e.Op.String())
			out.WriteString(" ")
			out.WriteString("(*")
			TranspileExpression(out, e.Y)
			out.WriteString(".lock().unwrap().as_mut().unwrap())")
		} else {
			// Regular binary expression
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
			// Regular array/slice indexing or map assignment (handled elsewhere)
			// Need to unwrap the array/slice first
			TranspileExpressionContext(out, e.X, RValue)
			out.WriteString("[")
			TranspileExpression(out, e.Index)
			out.WriteString("]")
		}

	case *ast.SliceExpr:
		TranspileExpression(out, e.X)
		out.WriteString("[")
		if e.Low != nil {
			TranspileExpression(out, e.Low)
		}
		out.WriteString("..")
		if e.High != nil {
			TranspileExpression(out, e.High)
		}
		out.WriteString("].to_vec()")

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
						// Wrap field values in Arc<Mutex<Option<T>>>
						out.WriteString("Arc::new(Mutex::new(Some(")
						TranspileExpression(out, kv.Value)
						out.WriteString(")))")
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
		// Handle type assertions like value.(string)
		if e.Type != nil {
			// Generate a match expression for type assertion
			out.WriteString("match ")
			TranspileExpression(out, e.X)
			out.WriteString(".downcast_ref::<")

			// Get the asserted type
			if ident, ok := e.Type.(*ast.Ident); ok {
				switch ident.Name {
				case "string":
					out.WriteString("String")
				case "int":
					out.WriteString("i32")
				case "bool":
					out.WriteString("bool")
				case "float64":
					out.WriteString("f64")
				default:
					out.WriteString(ident.Name)
				}
			} else {
				out.WriteString(GoTypeToRust(e.Type))
			}

			out.WriteString(">() { Some(v) => (v.clone(), true), None => (")

			// Default value for the type
			if ident, ok := e.Type.(*ast.Ident); ok {
				switch ident.Name {
				case "string":
					out.WriteString("String::new()")
				case "int":
					out.WriteString("0")
				case "bool":
					out.WriteString("false")
				case "float64":
					out.WriteString("0.0")
				default:
					out.WriteString("Default::default()")
				}
			} else {
				out.WriteString("Default::default()")
			}

			out.WriteString(", false) }")
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
			TranspileStatement(out, stmt, funcLit.Type, nil)
			out.WriteString("\n")
		}
	}
	out.WriteString("    })")

	// Cast to the right type and close wrappers
	out.WriteString(" as ")
	out.WriteString(generateClosureType(funcLit.Type))
	out.WriteString(")))")
}

func TranspileCall(out *strings.Builder, call *ast.CallExpr) {
	// Check if this is a stdlib function we need to replace
	if handler := GetStdlibHandler(call); handler != nil {
		handler(out, call)
		return
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
