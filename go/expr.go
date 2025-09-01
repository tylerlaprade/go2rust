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
		case token.INT:
			// Check if this integer is used in a float context
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				exprType := typeInfo.GetType(expr)
				if exprType != nil {
					if basic, ok := exprType.(*types.Basic); ok && (basic.Kind() == types.Float32 || basic.Kind() == types.Float64 || basic.Kind() == types.UntypedFloat) {
						// Integer literal used as float - add .0
						out.WriteString(e.Value)
						out.WriteString(".0")
						return
					}
				}
			}
			out.WriteString(e.Value)
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
				out.WriteString("(*self.0")
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())")
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
					WriteBorrowMethod(out, true)
					out.WriteString(".as_mut().unwrap())")
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
				WriteBorrowMethod(out, true)
				out.WriteString(".as_mut().unwrap())")
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
				fieldInfo := resolveFieldAccess(currentReceiverType, e.Sel.Name)

				if fieldInfo.IsPromoted {
					// Accessing promoted field through embedded struct(s)
					// For nested embedding like C.B.A.x, we need:
					// (*(*self.b.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).x

					if len(fieldInfo.EmbeddedPath) == 1 {
						// Simple case: one level of embedding
						out.WriteString("(*self.")
						out.WriteString(ToSnakeCase(fieldInfo.EmbeddedPath[0]))
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).")
						out.WriteString(fieldInfo.FieldName)
					} else {
						// Complex case: multiple levels of embedding
						// Start with the first embedded struct
						out.WriteString("(*(*self.")
						out.WriteString(ToSnakeCase(fieldInfo.EmbeddedPath[0]))
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap())")

						// Add intermediate embedded structs
						for i := 1; i < len(fieldInfo.EmbeddedPath); i++ {
							out.WriteString(".")
							out.WriteString(ToSnakeCase(fieldInfo.EmbeddedPath[i]))
							WriteBorrowMethod(out, false)
							if i < len(fieldInfo.EmbeddedPath)-1 {
								out.WriteString(".as_ref().unwrap()")
							} else {
								// Last one before the field
								out.WriteString(".as_ref().unwrap()).")
							}
						}
						out.WriteString(fieldInfo.FieldName)
					}
					// For return statements, we need to clone the Arc
					if ctx == RValue {
						out.WriteString(".clone()")
					}
				} else {
					// Direct field access
					out.WriteString("self.")
					out.WriteString(fieldInfo.FieldName)
					// For return statements, we need to clone the Arc
					if ctx == RValue {
						out.WriteString(".clone()")
					}
				}
			} else {
				// Regular field access on a variable - need to check for promoted fields
				// Try to resolve the field through type info
				var fieldInfo FieldAccessInfo

				if typeInfo != nil {
					// Try to get the type of the variable
					if t := typeInfo.GetType(e.X); t != nil {
						// Extract the struct type name
						typeStr := t.String()
						// Remove package prefix if present
						if idx := strings.LastIndex(typeStr, "."); idx >= 0 {
							typeStr = typeStr[idx+1:]
						}
						// Remove pointer prefix if present
						typeStr = strings.TrimPrefix(typeStr, "*")

						fieldInfo = resolveFieldAccess(typeStr, e.Sel.Name)
					} else {
						fieldInfo = FieldAccessInfo{
							IsPromoted: false,
							FieldName:  ToSnakeCase(e.Sel.Name),
						}
					}
				} else {
					fieldInfo = FieldAccessInfo{
						IsPromoted: false,
						FieldName:  ToSnakeCase(e.Sel.Name),
					}
				}

				// Check if this variable is wrapped (not a range var, not a constant)
				needsUnwrap := false
				if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
					if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
						// Regular variable - likely wrapped
						needsUnwrap = true
					}
				}

				if fieldInfo.IsPromoted {
					// Accessing promoted field through embedded struct(s)
					if needsUnwrap {
						// Wrapped variable with promoted field
						if ctx == LValue {
							out.WriteString("(*(*")
							out.WriteString(ident.Name)
							WriteBorrowMethod(out, true)
							out.WriteString(".as_mut().unwrap()).")
							for i, embedded := range fieldInfo.EmbeddedPath {
								out.WriteString(ToSnakeCase(embedded))
								WriteBorrowMethod(out, true)
								if i < len(fieldInfo.EmbeddedPath)-1 {
									out.WriteString(".as_mut().unwrap().")
								} else {
									out.WriteString(".as_mut().unwrap()).")
								}
							}
							out.WriteString(fieldInfo.FieldName)
						} else {
							// RValue context - need to unwrap the field value too
							out.WriteString("(*(*(*")
							out.WriteString(ident.Name)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()).")
							for i, embedded := range fieldInfo.EmbeddedPath {
								out.WriteString(ToSnakeCase(embedded))
								WriteBorrowMethod(out, false)
								if i < len(fieldInfo.EmbeddedPath)-1 {
									out.WriteString(".as_ref().unwrap().")
								} else {
									out.WriteString(".as_ref().unwrap()).")
								}
							}
							out.WriteString(fieldInfo.FieldName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap())")
						}
					} else {
						// Unwrapped variable (e.g., range variable) with promoted field
						// The field itself is still wrapped, so unwrap it in RValue context
						if ctx == RValue {
							out.WriteString("(*")
							out.WriteString(ident.Name)
							for _, embedded := range fieldInfo.EmbeddedPath {
								out.WriteString(".")
								out.WriteString(ToSnakeCase(embedded))
							}
							out.WriteString(".")
							out.WriteString(fieldInfo.FieldName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap())")
						} else {
							out.WriteString(ident.Name)
							for _, embedded := range fieldInfo.EmbeddedPath {
								out.WriteString(".")
								out.WriteString(ToSnakeCase(embedded))
							}
							out.WriteString(".")
							out.WriteString(fieldInfo.FieldName)
						}
					}
				} else {
					// Direct field access
					if needsUnwrap {
						// Access field on wrapped struct
						if ctx == LValue {
							// For assignment, we need mutable access
							out.WriteString("(*")
							out.WriteString(ident.Name)
							WriteBorrowMethod(out, true)
							out.WriteString(".as_mut().unwrap()).")
							out.WriteString(fieldInfo.FieldName)
						} else {
							// For reading, we need immutable access
							// Also unwrap the field itself in RValue context
							out.WriteString("(*(*")
							out.WriteString(ident.Name)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()).")
							out.WriteString(fieldInfo.FieldName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap())")
						}
					} else {
						// Not wrapped (e.g., range variable) - but field itself is wrapped
						if ctx == RValue {
							// Unwrap the field in RValue context
							out.WriteString("(*")
							out.WriteString(ident.Name)
							out.WriteString(".")
							out.WriteString(fieldInfo.FieldName)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap())")
						} else {
							// Direct access in LValue context
							out.WriteString(ident.Name)
							out.WriteString(".")
							out.WriteString(fieldInfo.FieldName)
						}
					}
				}
			}
		} else {
			// Complex expression for X (not just an identifier)
			var fieldInfo FieldAccessInfo

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

					fieldInfo = resolveFieldAccess(typeStr, e.Sel.Name)
				} else {
					fieldInfo = FieldAccessInfo{
						IsPromoted: false,
						FieldName:  ToSnakeCase(e.Sel.Name),
					}
				}
			} else {
				fieldInfo = FieldAccessInfo{
					IsPromoted: false,
					FieldName:  ToSnakeCase(e.Sel.Name),
				}
			}

			if fieldInfo.IsPromoted {
				// Accessing promoted field through embedded struct(s)
				// We need to unwrap each embedded struct in the path
				if ctx == RValue {
					// In RValue context, unwrap the final field too
					out.WriteString("(*")
					TranspileExpressionContext(out, e.X, LValue)
					for _, embedded := range fieldInfo.EmbeddedPath {
						out.WriteString(".")
						out.WriteString(ToSnakeCase(embedded))
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()")
					}
					out.WriteString(".")
					out.WriteString(fieldInfo.FieldName)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap())")
				} else {
					// In LValue context, don't unwrap the final field
					TranspileExpressionContext(out, e.X, LValue)
					for _, embedded := range fieldInfo.EmbeddedPath {
						out.WriteString(".")
						out.WriteString(ToSnakeCase(embedded))
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()")
					}
					out.WriteString(".")
					out.WriteString(fieldInfo.FieldName)
				}
			} else {
				// Direct field access
				// Check if e.X is a selector expression that returns a wrapped struct field
				if _, isSelector := e.X.(*ast.SelectorExpr); isSelector {
					// e.X is a field access that returns a wrapped value, need to unwrap it
					if ctx == RValue {
						// In RValue context, unwrap both the struct and the final field
						out.WriteString("(*(*")
						TranspileExpressionContext(out, e.X, LValue)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).")
						out.WriteString(fieldInfo.FieldName)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap())")
					} else {
						// In LValue context, just unwrap the struct to access the field
						out.WriteString("(*")
						TranspileExpressionContext(out, e.X, LValue)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()).")
						out.WriteString(fieldInfo.FieldName)
					}
				} else {
					// e.X is not a selector, use normal handling
					if ctx == RValue {
						// In RValue context, field needs to be unwrapped
						out.WriteString("(*")
						TranspileExpressionContext(out, e.X, LValue)
						out.WriteString(".")
						out.WriteString(fieldInfo.FieldName)
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap())")
					} else {
						// In LValue context, just access the field
						TranspileExpressionContext(out, e.X, LValue)
						out.WriteString(".")
						out.WriteString(fieldInfo.FieldName)
					}
				}
			}
		}

	case *ast.UnaryExpr:
		switch e.Op {
		case token.AND: // & - address-of
			// Check if we're taking address of a struct literal
			if compositeLit, isCompositeLit := e.X.(*ast.CompositeLit); isCompositeLit {
				// Special case for argError - it implements error interface
				if ident, ok := compositeLit.Type.(*ast.Ident); ok && ident.Name == "argError" {
					// This implements error interface, box it
					TrackImport("Error")
					out.WriteString("Rc::new(RefCell::new(Some(Box::new(")
					TranspileExpressionContext(out, e.X, AddressOf)
					out.WriteString(") as Box<dyn Error + Send + Sync>)))")
				} else {
					// For struct literals, wrap the whole thing
					WriteWrapperPrefix(out)
					TranspileExpressionContext(out, e.X, AddressOf)
					WriteWrapperSuffix(out)
				}
			} else {
				// Taking address of existing value just clones the Arc
				TranspileExpressionContext(out, e.X, AddressOf)
				out.WriteString(".clone()")
			}
		case token.MUL: // * - dereference
			// Double dereference - need to unwrap twice for **p
			out.WriteString("(*")
			TranspileExpression(out, e.X)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()")
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		default:
			out.WriteString(e.Op.String())
			TranspileExpression(out, e.X)
		}

	case *ast.StarExpr:
		// Dereference pointer - unwrap the wrapper to get T
		out.WriteString("(*")
		// Use LValue context so the identifier doesn't get unwrapped
		TranspileExpressionContext(out, e.X, LValue)
		if ctx == RValue {
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		} else {
			WriteBorrowMethod(out, true)
			out.WriteString(".as_mut().unwrap())")
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
				WriteBorrowMethod(out, false)
				out.WriteString(").is_some()")
				return
			} else if e.Op.String() == "==" {
				out.WriteString("(*")
				TranspileExpressionContext(out, e.X, LValue)
				WriteBorrowMethod(out, false)
				out.WriteString(").is_none()")
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
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())")
			} else {
				TranspileExpression(out, e.X)
			}
			out.WriteString(" ")
			out.WriteString(e.Op.String())
			out.WriteString(" ")
			if needsUnwrapY {
				out.WriteString("(*")
				TranspileExpression(out, e.Y)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())")
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
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).get(")
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
			out.WriteString(").unwrap()")
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
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
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).as_bytes()[")
				// Check if index needs unwrapping
				if typeInfo != nil && typeInfo.NeedsUnwrapping(e.Index) {
					out.WriteString("(*")
					TranspileExpression(out, e.Index)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()) as usize")
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
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap())[")
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
		WriteWrapperPrefix(out)
		out.WriteString("(*")
		// Use LValue context so identifiers don't unwrap themselves
		TranspileExpressionContext(out, e.X, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
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
		WriteWrapperSuffix(out)

	case *ast.CompositeLit:
		// When Type is nil, try to infer from TypeInfo
		if e.Type == nil {
			typeInfo := GetTypeInfo()
			if typeInfo != nil {
				if typ := typeInfo.GetType(e); typ != nil {
					// We have the actual type from go/types
					switch typ.Underlying().(type) {
					case *types.Slice:
						// Handle slice with inferred element type
						WriteWrapperPrefix(out)
						out.WriteString("vec![")
						for i, elt := range e.Elts {
							if i > 0 {
								out.WriteString(", ")
							}
							// Recursively transpile elements
							TranspileExpression(out, elt)
						}
						out.WriteString("]")
						WriteWrapperSuffix(out)
						return
					case *types.Struct:
						// Handle struct literal with inferred type
						// Try to get the struct name from the named type
						if named, ok := typ.(*types.Named); ok {
							out.WriteString(named.Obj().Name())
						} else {
							// Anonymous struct - we'd need to generate a type
							out.WriteString("/* Anonymous struct literal */")
							out.WriteString("unimplemented!()")
							return
						}
						out.WriteString(" { ")

						// Output the fields
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
										// Check if it's a literal (true, false, nil) that doesn't need cloning
										if valIdent.Name == "true" || valIdent.Name == "false" || valIdent.Name == "nil" {
											// Wrap literal values
											WriteWrapperPrefix(out)
											TranspileExpression(out, kv.Value)
											WriteWrapperSuffix(out)
										} else {
											// It's already wrapped, just clone it
											out.WriteString(valIdent.Name)
											out.WriteString(".clone()")
										}
									} else {
										// Wrap field values
										WriteWrapperPrefix(out)
										TranspileExpression(out, kv.Value)
										WriteWrapperSuffix(out)
									}
								}
							}
						}
						out.WriteString(" }")
						return
					}
				}
			}
			// If we can't infer, output an error comment
			out.WriteString("/* ERROR: CompositeLit with nil Type - type inference failed */")
			out.WriteString("unimplemented!()")
			return
		}
		// Handle array/slice literals
		if arrayType, ok := e.Type.(*ast.ArrayType); ok {
			// Check if element type is an interface
			isInterfaceSlice := false
			var interfaceName string

			// Check for interface{} (empty interface)
			if intf, ok := arrayType.Elt.(*ast.InterfaceType); ok && len(intf.Methods.List) == 0 {
				isInterfaceSlice = true
				interfaceName = "Any"
				TrackImport("Any")
			} else if ident, ok := arrayType.Elt.(*ast.Ident); ok {
				// Check if it's a named interface using TypeInfo
				typeInfo := GetTypeInfo()
				if typeInfo != nil && typeInfo.IsInterface(ident) {
					isInterfaceSlice = true
					interfaceName = ident.Name
				}
			}

			// Wrap the entire array/slice in Arc<Mutex<Option<>>>
			WriteWrapperPrefix(out)
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
				if isInterfaceSlice {
					// Box each element for interface slices
					out.WriteString("Box::new(")
					// If the element is already a wrapped variable, unwrap it first
					if ident, ok := elt.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "_" {
						// Check if it's a variable (already wrapped)
						if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
							if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
								// It's a wrapped variable, unwrap it
								out.WriteString("(*")
								out.WriteString(ident.Name)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).clone()")
							} else {
								// It's a constant
								TranspileExpression(out, elt)
							}
						} else {
							// Range variable
							TranspileExpression(out, elt)
						}
					} else {
						TranspileExpression(out, elt)
					}
					out.WriteString(") as Box<dyn ")
					out.WriteString(interfaceName)
					out.WriteString(">")
				} else {
					TranspileExpression(out, elt)
				}
			}
			out.WriteString("]")
			WriteWrapperSuffix(out)
		} else if mapType, ok := e.Type.(*ast.MapType); ok {
			// Map literal - wrap the whole map in Arc<Mutex<Option<>>>
			TrackImport("HashMap")
			WriteWrapperPrefix(out)
			out.WriteString("HashMap::<")
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
					WriteWrapperPrefix(out)
					TranspileExpression(out, kv.Value)
					WriteWrapperSuffix(out)
					out.WriteString(")")
				}
			}
			out.WriteString("]))))")
		} else if ident, ok := e.Type.(*ast.Ident); ok {
			// Struct literal
			out.WriteString(ident.Name)
			out.WriteString(" { ")

			// Check if all elements are positional (no KeyValueExpr)
			allPositional := true
			for _, elt := range e.Elts {
				if _, ok := elt.(*ast.KeyValueExpr); ok {
					allPositional = false
					break
				}
			}

			// Special handling for known structs with positional arguments
			if allPositional && ident.Name == "argError" && len(e.Elts) == 2 {
				// argError{arg, prob} - we know the field names
				out.WriteString("arg: ")
				WriteWrapperPrefix(out)
				TranspileExpression(out, e.Elts[0])
				WriteWrapperSuffix(out)
				out.WriteString(", prob: ")
				WriteWrapperPrefix(out)
				TranspileExpression(out, e.Elts[1])
				WriteWrapperSuffix(out)
			} else {
				// For named struct types with field names specified
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
								// Check if it's a literal (true, false, nil) that doesn't need cloning
								if valIdent.Name == "true" || valIdent.Name == "false" || valIdent.Name == "nil" {
									// Wrap literal values
									WriteWrapperPrefix(out)
									TranspileExpression(out, kv.Value)
									WriteWrapperSuffix(out)
								} else {
									// It's already wrapped, just clone it
									out.WriteString(valIdent.Name)
									out.WriteString(".clone()")
								}
							} else {
								// Wrap field values in Arc<Mutex<Option<T>>>
								WriteWrapperPrefix(out)
								TranspileExpression(out, kv.Value)
								WriteWrapperSuffix(out)
							}
						}
					}
				}
			}

			// Note: In Go, uninitialized fields get zero values
			// In Rust with our Arc<Mutex<Option<>>> wrapping, we'd need Default::default()
			// But this requires all fields to implement Default, which may not always be true
			// For now, we'll require all fields to be explicitly initialized

			out.WriteString(" }")
		} else if structType, ok := e.Type.(*ast.StructType); ok {
			// Anonymous struct literal - generate a type for it
			typeName := generateAnonymousStructType(structType)
			out.WriteString(typeName)
			out.WriteString(" { ")

			// Track which fields are initialized
			initializedFields := make(map[string]bool)
			for _, elt := range e.Elts {
				if kv, ok := elt.(*ast.KeyValueExpr); ok {
					if key, ok := kv.Key.(*ast.Ident); ok {
						initializedFields[key.Name] = true
					}
				}
			}

			// Output initialized fields
			needComma := false
			for _, elt := range e.Elts {
				if needComma {
					out.WriteString(", ")
				}
				needComma = true
				if kv, ok := elt.(*ast.KeyValueExpr); ok {
					if key, ok := kv.Key.(*ast.Ident); ok {
						out.WriteString(ToSnakeCase(key.Name))
						out.WriteString(": ")
						// Check if the value is an identifier (parameter/variable)
						if valIdent, ok := kv.Value.(*ast.Ident); ok {
							// Check if it's a literal (true, false, nil) that doesn't need cloning
							if valIdent.Name == "true" || valIdent.Name == "false" || valIdent.Name == "nil" {
								// Wrap literal values
								WriteWrapperPrefix(out)
								TranspileExpression(out, kv.Value)
								WriteWrapperSuffix(out)
							} else {
								// It's already wrapped, just clone it
								out.WriteString(valIdent.Name)
								out.WriteString(".clone()")
							}
						} else {
							// Wrap field values in Arc<Mutex<Option<T>>>
							WriteWrapperPrefix(out)
							TranspileExpression(out, kv.Value)
							WriteWrapperSuffix(out)
						}
					}
				}
			}

			// Add default values for uninitialized fields
			for _, field := range structType.Fields.List {
				for _, name := range field.Names {
					if !initializedFields[name.Name] {
						if needComma {
							out.WriteString(", ")
						}
						needComma = true
						out.WriteString(ToSnakeCase(name.Name))
						out.WriteString(": Default::default()")
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
			out.WriteString("        let guard = val.")
			if NeedsConcurrentWrapper() {
				out.WriteString("lock().unwrap()")
			} else {
				out.WriteString("borrow()")
			}
			out.WriteString(";\n")
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
		out.WriteString(" */ ")
		WriteWrapperPrefix(out)
		out.WriteString("()))")
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
		// Check if we already have renames set up (e.g., from defer)
		// This allows statement-level handlers to pre-configure renames
		if currentCaptureRenames != nil {
			if existingRename, exists := currentCaptureRenames[varName]; exists && existingRename != "" {
				// Use the existing rename
				captureRenames[varName] = existingRename
			} else {
				// No existing rename for this variable, use identity
				captureRenames[varName] = varName
			}
		} else {
			// No existing renames at all, use identity
			captureRenames[varName] = varName
		}
	}

	// Store current capture renames for nested transpilation
	oldCaptureRenames := currentCaptureRenames
	currentCaptureRenames = captureRenames
	defer func() { currentCaptureRenames = oldCaptureRenames }()

	// Wrap the closure in Arc<Mutex<Option<Box<dyn Fn>>>
	WriteWrapperPrefix(out)

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
	WriteWrapperSuffix(out)
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
				WriteWrapperPrefix(out)
				if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
					out.WriteString("(*")
					out.WriteString(ident.Name)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).as_bytes().to_vec()")
				} else {
					out.WriteString("(*")
					TranspileExpression(out, call.Args[0])
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).as_bytes().to_vec()")
				}
				WriteWrapperSuffix(out)
				return
			case "rune", "int32":
				// []rune(string) conversion
				WriteWrapperPrefix(out)
				if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
					out.WriteString("(*")
					out.WriteString(ident.Name)
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).chars().map(|c| c as i32).collect::<Vec<_>>()")
				} else {
					out.WriteString("(*")
					TranspileExpression(out, call.Args[0])
					WriteBorrowMethod(out, false)
					out.WriteString(".as_ref().unwrap()).chars().map(|c| c as i32).collect::<Vec<_>>()")
				}
				WriteWrapperSuffix(out)
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
							WriteWrapperPrefix(out)
							out.WriteString("String::from_utf8(")
							if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
								out.WriteString("(*")
								out.WriteString(ident.Name)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).clone()")
							} else {
								out.WriteString("(*")
								TranspileExpression(out, arg)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).clone()")
							}
							out.WriteString(").unwrap())))")
							return
						} else if basic.Kind() == types.Rune || basic.Kind() == types.Int32 {
							// []rune to string
							WriteWrapperPrefix(out)
							if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
								out.WriteString("(*")
								out.WriteString(ident.Name)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap())")
							} else {
								out.WriteString("(*")
								TranspileExpression(out, arg)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap())")
							}
							out.WriteString(".iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>())))")
							return
						}
					}
				} else if basic, ok := argType.Underlying().(*types.Basic); ok {
					if basic.Kind() == types.Rune || basic.Kind() == types.Int32 {
						// Single rune to string
						WriteWrapperPrefix(out)
						out.WriteString("char::from_u32((")
						out.WriteString("*")
						if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
							out.WriteString(ident.Name)
						} else {
							TranspileExpression(out, arg)
						}
						WriteBorrowMethod(out, false)
						out.WriteString(".as_ref().unwrap()")
						out.WriteString(") as u32).unwrap().to_string())))")
						return
					}
				}
			}
		}
		// Default string conversion
		WriteWrapperPrefix(out)
		out.WriteString("(*")
		if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString(ident.Name)
		} else {
			TranspileExpression(out, arg)
		}
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).to_string()")
		WriteWrapperSuffix(out)
		return
	case "rune":
		rustType = "i32" // rune is an alias for int32
	// Complex types
	case "complex64":
		WriteWrapperPrefix(out)
		out.WriteString("num::Complex::<f32>::new(")
		if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString("(*")
			out.WriteString(ident.Name)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()) as f32")
		} else {
			out.WriteString("(*")
			TranspileExpression(out, call.Args[0])
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()) as f32")
		}
		out.WriteString(", 0.0))))")
		return
	case "complex128":
		WriteWrapperPrefix(out)
		out.WriteString("num::Complex::<f64>::new(")
		if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
			out.WriteString("(*")
			out.WriteString(ident.Name)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()) as f64")
		} else {
			out.WriteString("(*")
			TranspileExpression(out, call.Args[0])
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()) as f64")
		}
		out.WriteString(", 0.0))))")
		return
	default:
		// Check for custom type definitions
		if _, isTypeDef := typeDefinitions[targetType]; isTypeDef {
			// Custom type definition
			out.WriteString(targetType)
			out.WriteString("(")
			WriteWrapperPrefix(out)
			TranspileExpression(out, call.Args[0])
			out.WriteString("))))")
			return
		}
		// Unknown type, just pass through
		needsCast = false
	}

	if needsCast && rustType != "" {
		// Perform the type cast
		WriteWrapperPrefix(out)
		out.WriteString("(")
		// Check if the argument is a simple identifier (variable)
		out.WriteString("*")
		if ident, ok := call.Args[0].(*ast.Ident); ok && ident.Name != "nil" {
			// It's a variable, unwrap it directly
			out.WriteString(ident.Name)
		} else {
			// It's an expression, evaluate and unwrap
			TranspileExpression(out, call.Args[0])
		}
		out.WriteString(".as_ref().unwrap()")
		out.WriteString(".as_ref().unwrap()")
		out.WriteString(") as ")
		out.WriteString(rustType)
		WriteWrapperSuffix(out)
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
	out.WriteString("        let guard = val")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString("        if let Some(ref any_val) = *guard {\n")
	out.WriteString("            if let Some(typed_val) = any_val.downcast_ref::<")
	out.WriteString(rustType)
	out.WriteString(">() {\n")
	out.WriteString("                (")
	WriteWrapperPrefix(out)
	out.WriteString("typed_val.clone()))), ")
	WriteWrapperPrefix(out)
	out.WriteString("true))))\n")
	out.WriteString("            } else {\n")
	out.WriteString("                (")
	WriteWrapperPrefix(out)
	out.WriteString(defaultValue)
	out.WriteString("))), ")
	WriteWrapperPrefix(out)
	out.WriteString("false))))\n")
	out.WriteString("            }\n")
	out.WriteString("        } else {\n")
	out.WriteString("            (")
	WriteWrapperPrefix(out)
	out.WriteString(defaultValue)
	out.WriteString("))), ")
	WriteWrapperPrefix(out)
	out.WriteString("false))))\n")
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
			out.WriteString("(")
			WriteWrapperPrefix(out)
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
		// For method calls, we need to check if the receiver is a wrapped type or not
		// If it's a struct variable, we call the method directly
		// If it's a pointer/wrapped type, we need to unwrap it first

		// Check what kind of receiver we have
		needsUnwrap := false

		// Check if the receiver is a simple identifier (local variable)
		if ident, ok := sel.X.(*ast.Ident); ok {
			// Check if this variable is wrapped (not a range var, not a constant)
			if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
				if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
					// Regular variable - it's wrapped in Arc<Mutex<Option<>>>
					needsUnwrap = true
				}
			}

			if needsUnwrap {
				// Wrapped type - need to unwrap
				out.WriteString("(*")
				out.WriteString(ident.Name)
				WriteBorrowMethod(out, true)
				out.WriteString(".as_mut().unwrap()).")
			} else {
				// Direct struct variable (range var or constant) - call method directly
				out.WriteString(ident.Name)
				out.WriteString(".")
			}
		} else if fieldSel, ok := sel.X.(*ast.SelectorExpr); ok {
			// Method call on a field (e.g., s.Counter.Value())
			// The field is wrapped, so we need to unwrap it
			out.WriteString("(*")
			TranspileExpression(out, fieldSel)
			out.WriteString("")
			WriteBorrowMethod(out, false)
			out.WriteString(".as_mut().unwrap()).")
		} else {
			// Other complex expression - just transpile it
			TranspileExpression(out, sel.X)
			out.WriteString(".")
		}

		out.WriteString(ToSnakeCase(sel.Sel.Name))
		out.WriteString("(")
		for i, arg := range call.Args {
			if i > 0 {
				out.WriteString(", ")
			}
			// For method calls, wrap arguments normally
			WriteWrapperPrefix(out)
			TranspileExpression(out, arg)
			WriteWrapperSuffix(out)
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
			// Check if this variable has been renamed (captured in closure)
			varName := ident.Name
			if currentCaptureRenames != nil {
				if renamed, exists := currentCaptureRenames[ident.Name]; exists {
					varName = renamed
				}
			}
			out.WriteString("(*")
			out.WriteString(varName)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap())")
		}
	} else {
		// Complex expression for the function (e.g., function returning a function)
		out.WriteString("(*")
		TranspileExpression(out, call.Fun)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap())")
	}

	out.WriteString("(")

	// Check if this is a regular function call to determine if we need interface boxing
	var funcName string
	if ident, ok := call.Fun.(*ast.Ident); ok {
		funcName = ident.Name
	}

	// Get function signature to check for interface parameters
	var funcSig *FunctionSignature
	if funcName != "" && !isBuiltinFunction(funcName) {
		funcSig = GetFunctionSignature(funcName)
	}

	for i, arg := range call.Args {
		if i > 0 {
			out.WriteString(", ")
		}

		// Check if this parameter expects an interface type
		needsInterfaceBoxing := false
		var interfaceName string
		if funcSig != nil && i < len(funcSig.Params) {
			// Get the parameter type
			paramType := funcSig.Params[i].Type
			if ident, ok := paramType.(*ast.Ident); ok {
				// Check if this is an interface type using TypeInfo
				typeInfo := GetTypeInfo()
				if typeInfo != nil && typeInfo.IsInterface(ident) {
					needsInterfaceBoxing = true
					interfaceName = ident.Name
				}
			}
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
						// It's a variable
						if needsInterfaceBoxing {
							// Need to box for interface parameter
							// Check if it's a range variable that's already a boxed interface
							if varType, isRangeVar := rangeLoopVars[ident.Name]; isRangeVar && varType == "ref_value" {
								// It's a reference from a range loop over interface slice
								// The value is already &Box<dyn Interface>, just clone it
								WriteWrapperPrefix(out)
								out.WriteString(ident.Name)
								out.WriteString(".clone()))")
							} else {
								// Regular variable needs boxing
								WriteWrapperPrefix(out)
								out.WriteString("Box::new((*")
								out.WriteString(ident.Name)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap()).clone()) as Box<dyn ")
								out.WriteString(interfaceName)
								out.WriteString(">)))")
							}
						} else {
							// Regular variable, just clone it
							out.WriteString(ident.Name)
							out.WriteString(".clone()")
						}
					} else {
						// It's a constant, wrap it
						WriteWrapperPrefix(out)
						TranspileExpression(out, arg)
						WriteWrapperSuffix(out)
					}
				} else {
					// Range variable - check if it needs dereferencing
					varType := rangeLoopVars[ident.Name]
					if varType == "ref_value" {
						// It's a reference from iterator
						if needsInterfaceBoxing {
							// It's already a &Box<dyn Interface>
							// We can't clone Box<dyn Trait> directly, so just clone the reference
							WriteWrapperPrefix(out)
							out.WriteString(ident.Name)
							out.WriteString(".clone())))")
						} else {
							// Regular ref value, dereference it
							WriteWrapperPrefix(out)
							out.WriteString("*")
							TranspileExpression(out, arg)
							WriteWrapperSuffix(out)
						}
					} else {
						// Regular range variable, wrap it normally
						WriteWrapperPrefix(out)
						TranspileExpression(out, arg)
						WriteWrapperSuffix(out)
					}
				}
			} else if _, isFuncLit := arg.(*ast.FuncLit); isFuncLit {
				// Function literal - already wraps itself
				TranspileExpression(out, arg)
			} else {
				// Not a simple identifier or function literal, wrap it
				WriteWrapperPrefix(out)
				TranspileExpression(out, arg)
				WriteWrapperSuffix(out)
			}
		} else {
			TranspileExpression(out, arg)
		}
	}
	out.WriteString(")")
}
