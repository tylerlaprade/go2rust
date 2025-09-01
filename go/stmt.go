package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"go/types"
	"slices"
	"strings"
)

// hasBlankLineBetween checks if there's more than one line between two positions
func hasBlankLineBetween(fileSet *token.FileSet, pos1, pos2 token.Pos) bool {
	if fileSet == nil || pos1 == token.NoPos || pos2 == token.NoPos {
		return false
	}

	p1 := fileSet.Position(pos1)
	p2 := fileSet.Position(pos2)

	// If there's more than 1 line between the positions, there's at least one blank line
	return p2.Line-p1.Line > 1
}

// outputCommentsBeforePos outputs any comments that appear before the given position
func outputCommentsBeforePos(out *strings.Builder, comments []*ast.CommentGroup, fileSet *token.FileSet, pos token.Pos, indent string, lastPos *token.Pos) {
	if fileSet == nil || pos == token.NoPos {
		return
	}

	targetLine := fileSet.Position(pos).Line
	startLine := 0
	if *lastPos != token.NoPos {
		startLine = fileSet.Position(*lastPos).Line
	}

	for _, cg := range comments {
		cgLine := fileSet.Position(cg.Pos()).Line
		// Output comments that are after our last position but before the current position
		if cgLine > startLine && cgLine < targetLine {
			for _, comment := range cg.List {
				out.WriteString(indent)
				out.WriteString(comment.Text)
				out.WriteString("\n")
				out.WriteString(indent)
			}
		}
	}

	*lastPos = pos
}
func outputComment(out *strings.Builder, cg *ast.CommentGroup, indent string, isDoc bool) {
	if cg == nil {
		return
	}

	for _, comment := range cg.List {
		out.WriteString(indent)

		text := comment.Text
		if isDoc && strings.HasPrefix(text, "//") {
			// Convert doc comment to Rust format
			out.WriteString("///")
			out.WriteString(text[2:]) // Skip the "//"
		} else {
			// Keep as-is for regular comments
			out.WriteString(text)
		}
		out.WriteString("\n")
	}
}

// TranspileStatementSimple is a wrapper for backward compatibility
func TranspileStatementSimple(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet) {
	var lastPos token.Pos
	TranspileStatement(out, stmt, fnType, fileSet, nil, &lastPos, "")
}

func TranspileStatement(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType, fileSet *token.FileSet, comments []*ast.CommentGroup, lastPos *token.Pos, indent string) {
	// Output any comments before this statement
	if stmt != nil && comments != nil && lastPos != nil {
		outputCommentsBeforePos(out, comments, fileSet, stmt.Pos(), indent, lastPos)
	}

	// Preprocess the statement to find closures and generate clone statements
	// Skip defer statements as they handle captures themselves
	var captureInfo *CaptureInfo
	if _, isDefer := stmt.(*ast.DeferStmt); !isDefer && statementPreprocessor != nil {
		captureInfo = statementPreprocessor.PreprocessStatement(stmt, fnType)
		if captureInfo != nil && len(captureInfo.CapturedVars) > 0 {
			// Generate clone statements before the actual statement
			statementPreprocessor.GenerateCloneStatements(out, captureInfo)

			// Set up capture renames for this statement
			oldCaptureRenames := currentCaptureRenames
			currentCaptureRenames = captureInfo.CapturedVars
			defer func() { currentCaptureRenames = oldCaptureRenames }()
		}
	}

	switch s := stmt.(type) {
	case *ast.ExprStmt:
		TranspileExpression(out, s.X)
		out.WriteString(";")

	case *ast.ReturnStmt:
		// Execute defers before returning if needed
		if currentFunctionHasDefer {
			out.WriteString("{\n")
			out.WriteString("        // Execute deferred functions\n")
			out.WriteString("        while let Some(f) = __defer_stack.pop() {\n")
			out.WriteString("            f();\n")
			out.WriteString("        }\n")
			out.WriteString("        return")
		} else {
			out.WriteString("return")
		}

		// Handle naked return (no explicit values but function has named returns)
		if len(s.Results) == 0 && fnType.Results != nil {
			hasNamedReturns := false
			for _, result := range fnType.Results.List {
				if len(result.Names) > 0 {
					hasNamedReturns = true
					break
				}
			}

			if hasNamedReturns {
				out.WriteString(" ")
				// Return the named values
				needsTuple := false
				totalReturns := 0
				for _, result := range fnType.Results.List {
					if len(result.Names) > 0 {
						totalReturns += len(result.Names)
					} else {
						totalReturns++
					}
				}
				needsTuple = totalReturns > 1

				if needsTuple {
					out.WriteString("(")
				}

				first := true
				for _, result := range fnType.Results.List {
					for _, name := range result.Names {
						if !first {
							out.WriteString(", ")
						}
						first = false
						out.WriteString(name.Name)
					}
				}

				if needsTuple {
					out.WriteString(")")
				}
			}
		} else if len(s.Results) > 0 {
			out.WriteString(" ")
			// Check if we need a tuple for multiple return values
			needsTuple := len(s.Results) > 1
			if needsTuple {
				out.WriteString("(")
			}

			for i, result := range s.Results {
				if i > 0 {
					out.WriteString(", ")
				}

				// Check if this is nil being returned
				isNil := false
				if ident, ok := result.(*ast.Ident); ok && ident.Name == "nil" {
					isNil = true
				}

				if isNil {
					WriteWrappedNone(out)
				} else {
					// Check if this is a field access on self (already wrapped)
					if sel, ok := result.(*ast.SelectorExpr); ok {
						if ident, ok := sel.X.(*ast.Ident); ok && currentReceiver != "" && ident.Name == currentReceiver {
							// Returning self.field - just clone it, don't double-wrap
							out.WriteString("self.")
							out.WriteString(ToSnakeCase(sel.Sel.Name))
							out.WriteString(".clone()")
						} else {
							// Regular selector - wrap it
							WriteWrapperPrefix(out)
							TranspileExpression(out, result)
							WriteWrapperSuffix(out)
						}
					} else if callExpr, ok := result.(*ast.CallExpr); ok {
						// Check if this is a function that returns an already-wrapped value
						needsWrapping := true

						// Check if it's errors.New or a function returning error
						if sel, ok := callExpr.Fun.(*ast.SelectorExpr); ok {
							if ident, ok := sel.X.(*ast.Ident); ok {
								if ident.Name == "errors" && sel.Sel.Name == "New" {
									needsWrapping = false
								} else if ident.Name == "fmt" && sel.Sel.Name == "Errorf" {
									needsWrapping = false
								} else if ident.Name == "fmt" && sel.Sel.Name == "Sprintf" {
									// fmt.Sprintf already wraps its result
									needsWrapping = false
								}
							}
						}

						// Check if it's a user function that returns error
						if fnType.Results != nil && i < len(fnType.Results.List) {
							if resultType, ok := fnType.Results.List[i].Type.(*ast.Ident); ok && resultType.Name == "error" {
								// This position is an error type, and we have a function call
								needsWrapping = false
							}
						}

						if needsWrapping {
							WriteWrapperPrefix(out)
							TranspileExpression(out, result)
							WriteWrapperSuffix(out)
						} else {
							// Already wrapped
							TranspileExpression(out, result)
						}
					} else if _, ok := result.(*ast.FuncLit); ok {
						// Function literal - already wrapped by TranspileFuncLit
						TranspileExpression(out, result)
					} else if ident, ok := result.(*ast.Ident); ok {
						// Check if this is a wrapped variable that needs cloning
						// Use a combination of TypeInfo and heuristics
						isWrappedVariable := false

						// First check TypeInfo
						typeInfo := GetTypeInfo()
						if typeInfo != nil && typeInfo.ReturnsWrappedValue(result) {
							isWrappedVariable = true
						} else {
							// Fallback: check if this looks like a local variable
							// (not a special identifier and not a range variable)
							if ident.Name != "true" && ident.Name != "false" && ident.Name != "nil" {
								if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
									if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
										// This is likely a wrapped variable
										isWrappedVariable = true
									}
								}
							}
						}

						if isWrappedVariable {
							// This is a wrapped variable - clone it to avoid move errors
							// Check if this variable has been renamed (captured in closure)
							varName := ident.Name
							if currentCaptureRenames != nil {
								if renamed, exists := currentCaptureRenames[ident.Name]; exists {
									varName = renamed
								}
							}
							out.WriteString(varName)
							out.WriteString(".clone()")
						} else {
							// This needs wrapping (constants, literals, etc.)
							if ident.Name == "nil" {
								WriteWrappedNone(out)
							} else {
								WriteWrapperPrefix(out)
								TranspileExpression(out, result)
								WriteWrapperSuffix(out)
							}
						}
					} else if _, ok := result.(*ast.SliceExpr); ok {
						// Slice expressions already return wrapped values (Arc<Mutex<Option<Vec<T>>>>)
						TranspileExpression(out, result)
					} else if unaryExpr, ok := result.(*ast.UnaryExpr); ok {
						// Check if this is address-of a struct literal
						if unaryExpr.Op == token.AND {
							if _, isCompositeLit := unaryExpr.X.(*ast.CompositeLit); isCompositeLit {
								// Already wrapped by UnaryExpr handling, don't double-wrap
								TranspileExpression(out, result)
							} else {
								// Regular address-of, already returns wrapped value
								TranspileExpression(out, result)
							}
						} else {
							// Other unary expressions, wrap them
							WriteWrapperPrefix(out)
							TranspileExpression(out, result)
							WriteWrapperSuffix(out)
						}
					} else if binExpr, ok := result.(*ast.BinaryExpr); ok {
						// Binary expressions need special handling to avoid multiple locks
						// Check if operands are identifiers that would need unwrapping
						needsExtraction := false
						if ident, ok := binExpr.X.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "true" && ident.Name != "false" {
							if _, isConst := localConstants[ident.Name]; !isConst {
								if _, isRange := rangeLoopVars[ident.Name]; !isRange {
									needsExtraction = true
								}
							}
						}
						if ident, ok := binExpr.Y.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "true" && ident.Name != "false" {
							if _, isConst := localConstants[ident.Name]; !isConst {
								if _, isRange := rangeLoopVars[ident.Name]; !isRange {
									needsExtraction = true
								}
							}
						}

						if needsExtraction {
							// Extract values first to avoid multiple locks
							// We need both operands to be unwrapped for the binary operation
							out.WriteString("{\n")

							// Get TypeInfo to check if expressions return wrapped values
							typeInfo := GetTypeInfo()

							// Extract X operand
							out.WriteString("            let __tmp_x = ")
							// Check if X returns a wrapped value that needs unwrapping
							if typeInfo != nil && typeInfo.ReturnsWrappedValue(binExpr.X) {
								// Expression returns wrapped value, unwrap it
								out.WriteString("(*")
								TranspileExpression(out, binExpr.X)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap())")
							} else {
								// Either a literal/constant or an identifier that will unwrap itself in RValue context
								TranspileExpressionContext(out, binExpr.X, RValue)
							}
							out.WriteString(";\n")

							// Extract Y operand
							out.WriteString("            let __tmp_y = ")
							// Check if Y returns a wrapped value that needs unwrapping
							if typeInfo != nil && typeInfo.ReturnsWrappedValue(binExpr.Y) {
								// Expression returns wrapped value, unwrap it
								out.WriteString("(*")
								TranspileExpression(out, binExpr.Y)
								WriteBorrowMethod(out, false)
								out.WriteString(".as_ref().unwrap())")
							} else {
								// Either a literal/constant or an identifier that will unwrap itself in RValue context
								TranspileExpressionContext(out, binExpr.Y, RValue)
							}
							out.WriteString(";\n")

							out.WriteString("            ")
							WriteWrapperPrefix(out)
							out.WriteString("__tmp_x ")
							out.WriteString(binExpr.Op.String())
							out.WriteString(" __tmp_y")
							WriteWrapperSuffix(out)
							out.WriteString("\n")
							out.WriteString("        }")
						} else {
							// No extraction needed
							WriteWrapperPrefix(out)
							TranspileExpression(out, result)
							WriteWrapperSuffix(out)
						}
					} else {
						// Wrap all other return values in Arc<Mutex<Option<>>>
						WriteWrapperPrefix(out)

						// Special handling for string literals
						if lit, ok := result.(*ast.BasicLit); ok && lit.Kind == token.STRING {
							out.WriteString(lit.Value)
							out.WriteString(".to_string()")
						} else {
							TranspileExpression(out, result)
						}

						WriteWrapperSuffix(out)
					}
				}
			}

			if needsTuple {
				out.WriteString(")")
			}
		}

		// Close the defer execution block if needed
		if currentFunctionHasDefer {
			out.WriteString("\n    }")
		} else {
			out.WriteString(";")
		}

	case *ast.AssignStmt:
		// Check if this is a map index assignment using type information
		isMapIndexAssign := false
		if len(s.Lhs) == 1 && len(s.Rhs) == 1 && s.Tok == token.ASSIGN {
			if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok {
				// Use TypeInfo to determine if this is actually a map
				typeInfo := GetTypeInfo()
				if typeInfo != nil {
					isMapIndexAssign = typeInfo.IsMap(indexExpr.X)
				} else {
					// Type info not available - can't determine if it's a map
					// Generate an error comment to make this obvious
					out.WriteString("/* ERROR: Cannot determine if map assignment - type information required */ ")
				}
			}
		}

		if isMapIndexAssign {
			// Handle map[key] = value as map.insert(key, value)
			if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok {
				out.WriteString("(*")
				// For map access, we need the raw identifier, not the unwrapped value
				if ident, ok := indexExpr.X.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, indexExpr.X)
				}
				WriteBorrowMethod(out, true)
				out.WriteString(".as_mut().unwrap()).insert(")
				TranspileExpression(out, indexExpr.Index)
				out.WriteString(", ")

				// Check if RHS is a variable that's already wrapped
				if ident, ok := s.Rhs[0].(*ast.Ident); ok && ident.Name != "_" {
					// Check if this is a variable (not a constant)
					if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
						if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
							// It's a variable, just clone it
							out.WriteString(ident.Name)
							out.WriteString(".clone()")
						} else {
							// It's a constant, wrap it
							WriteWrapperPrefix(out)
							TranspileExpression(out, s.Rhs[0])
							WriteWrapperSuffix(out)
						}
					} else {
						// Range variable, wrap it
						WriteWrapperPrefix(out)
						TranspileExpression(out, s.Rhs[0])
						WriteWrapperSuffix(out)
					}
				} else {
					// Not a simple identifier (literal or expression), wrap it
					WriteWrapperPrefix(out)
					TranspileExpression(out, s.Rhs[0])
					WriteWrapperSuffix(out)
				}
				out.WriteString(")")
			}
		} else if s.Tok == token.ADD_ASSIGN || s.Tok == token.SUB_ASSIGN ||
			s.Tok == token.MUL_ASSIGN || s.Tok == token.QUO_ASSIGN || s.Tok == token.REM_ASSIGN ||
			s.Tok == token.AND_ASSIGN || s.Tok == token.OR_ASSIGN || s.Tok == token.XOR_ASSIGN ||
			s.Tok == token.SHL_ASSIGN || s.Tok == token.SHR_ASSIGN {
			// Compound assignment operators

			isString := false
			if s.Tok == token.ADD_ASSIGN {
				typeInfo := GetTypeInfo()
				if typeInfo != nil {
					isString = typeInfo.IsString(s.Lhs[0])
				} else {
					// Type info not available - check if RHS is a string literal at least
					if lit, ok := s.Rhs[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
						isString = true
						out.WriteString("/* WARNING: Assuming string type based on literal */ ")
					}
				}
			}

			if isString {
				// For string +=, we need mutable access to the LHS
				out.WriteString("(*")
				TranspileExpressionContext(out, s.Lhs[0], LValue)
				WriteBorrowMethod(out, true)
				out.WriteString(".as_mut().unwrap()).push_str(&")
				TranspileExpression(out, s.Rhs[0])
				out.WriteString(")")
			} else {
				// Numeric compound assignment for wrapped values
				// Generate: { let mut guard = lhs.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() OP rhs); }
				out.WriteString("{ let mut guard = ")
				TranspileExpressionContext(out, s.Lhs[0], LValue)
				WriteBorrowMethod(out, true)
				out.WriteString("; *guard = Some(guard.as_ref().unwrap() ")

				// Output the appropriate operator
				switch s.Tok {
				case token.ADD_ASSIGN:
					out.WriteString("+")
				case token.SUB_ASSIGN:
					out.WriteString("-")
				case token.MUL_ASSIGN:
					out.WriteString("*")
				case token.QUO_ASSIGN:
					out.WriteString("/")
				case token.REM_ASSIGN:
					out.WriteString("%")
				case token.AND_ASSIGN:
					out.WriteString("&")
				case token.OR_ASSIGN:
					out.WriteString("|")
				case token.XOR_ASSIGN:
					out.WriteString("^")
				case token.SHL_ASSIGN:
					out.WriteString("<<")
				case token.SHR_ASSIGN:
					out.WriteString(">>")
				}

				out.WriteString(" ")
				// Handle RHS based on its type
				if ident, ok := s.Rhs[0].(*ast.Ident); ok {
					// It's an identifier - need to unwrap it
					// Check if it's a special identifier that shouldn't be unwrapped
					_, isRangeVar := rangeLoopVars[ident.Name]
					_, isLocalConst := localConstants[ident.Name]
					if !isRangeVar && !isLocalConst && ident.Name != "true" && ident.Name != "false" &&
						ident.Name != "nil" && ident.Name != "_" {
						// Regular wrapped variable - unwrap it
						out.WriteString("(*")
						out.WriteString(ident.Name)
						WriteBorrowMethod(out, true)
						out.WriteString(".as_mut().unwrap())")
					} else {
						// Special identifier - use as-is
						out.WriteString(ident.Name)
					}
				} else if lit, ok := s.Rhs[0].(*ast.BasicLit); ok {
					// It's a literal - use directly
					out.WriteString(lit.Value)
				} else {
					// It's an expression - transpile it
					TranspileExpression(out, s.Rhs[0])
				}
				out.WriteString("); }")
			}
		} else { // Check if we have multiple LHS with single RHS (tuple unpacking)
			needsTupleUnpack := len(s.Lhs) > 1 && len(s.Rhs) == 1

			// Check if this is a map access with existence check: value, ok := map[key]
			isMapAccess := false
			isTypeAssertion := false
			if needsTupleUnpack && len(s.Lhs) == 2 {
				if indexExpr, ok := s.Rhs[0].(*ast.IndexExpr); ok {
					// Check if the indexed expression is a map
					typeInfo := GetTypeInfo()
					if typeInfo != nil {
						isMapAccess = typeInfo.IsMap(indexExpr.X)
					} else {
						// Type info not available - cannot determine
						out.WriteString("/* ERROR: Cannot determine if map access - type information required */ ")
						isMapAccess = false
					}
				} else if _, ok := s.Rhs[0].(*ast.TypeAssertExpr); ok {
					// This is a type assertion with comma-ok
					isTypeAssertion = true
				}
			}

			if isTypeAssertion && needsTupleUnpack {
				// Handle type assertion with comma-ok: value, ok := x.(Type)
				typeAssert := s.Rhs[0].(*ast.TypeAssertExpr)

				if s.Tok == token.DEFINE {
					out.WriteString("let (")
					// First variable for value
					if ident, ok := s.Lhs[0].(*ast.Ident); ok && ident.Name != "_" {
						out.WriteString("mut ")
						out.WriteString(ident.Name)
					} else {
						out.WriteString("_")
					}
					out.WriteString(", ")
					// Second variable for ok
					if ident, ok := s.Lhs[1].(*ast.Ident); ok && ident.Name != "_" {
						out.WriteString("mut ")
						out.WriteString(ident.Name)
					} else {
						out.WriteString("_")
					}
					out.WriteString(") = ")
				} else {
					out.WriteString("(")
					TranspileExpressionContext(out, s.Lhs[0], LValue)
					out.WriteString(", ")
					TranspileExpressionContext(out, s.Lhs[1], LValue)
					out.WriteString(") = ")
				}

				// Generate type assertion code with comma-ok
				TranspileTypeAssertionCommaOk(out, typeAssert)
			} else if isMapAccess && needsTupleUnpack {
				// Handle map access with existence check
				indexExpr := s.Rhs[0].(*ast.IndexExpr)

				if s.Tok == token.DEFINE {
					out.WriteString("let (")
					// First variable for value
					if ident, ok := s.Lhs[0].(*ast.Ident); ok && ident.Name != "_" {
						out.WriteString("mut ")
						out.WriteString(ident.Name)
					} else {
						out.WriteString("_")
					}
					out.WriteString(", ")
					// Second variable for existence
					if ident, ok := s.Lhs[1].(*ast.Ident); ok && ident.Name != "_" {
						out.WriteString("mut ")
						out.WriteString(ident.Name)
					} else {
						out.WriteString("_")
					}
					out.WriteString(") = ")
				} else {
					out.WriteString("(")
					TranspileExpressionContext(out, s.Lhs[0], LValue)
					out.WriteString(", ")
					TranspileExpressionContext(out, s.Lhs[1], LValue)
					out.WriteString(") = ")
				}

				// Generate the map access code
				out.WriteString("match (*")
				// For map access, we need the raw identifier, not the unwrapped value
				if ident, ok := indexExpr.X.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, indexExpr.X)
				}
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).get(&")
				TranspileExpression(out, indexExpr.Index)
				out.WriteString(") { /* MAP_COMMA_OK */ Some(v) => (v.clone(), ")
				WriteWrapperPrefix(out)
				out.WriteString("true")
				WriteWrapperSuffix(out)
				out.WriteString("), None => (")
				WriteWrapperPrefix(out)
				// Default value for the type - for now assume i32
				// TODO: Use proper type information
				out.WriteString("0")
				WriteWrapperSuffix(out)
				out.WriteString(", ")
				WriteWrapperPrefix(out)
				out.WriteString("false")
				WriteWrapperSuffix(out)
				out.WriteString(") }")
			} else if needsTupleUnpack {
				if s.Tok == token.DEFINE {
					out.WriteString("let ")
				}
				out.WriteString("(")
				for i, lhs := range s.Lhs {
					if i > 0 {
						out.WriteString(", ")
					}
					if s.Tok == token.DEFINE {
						// Don't add mut before blank identifier
						if ident, ok := lhs.(*ast.Ident); !ok || ident.Name != "_" {
							out.WriteString("mut ")
						}
					}
					TranspileExpressionContext(out, lhs, LValue)
				}
				out.WriteString(")")

				out.WriteString(" = ")

				TranspileExpression(out, s.Rhs[0])
			} else if len(s.Lhs) > 1 && len(s.Rhs) > 1 {
				// Multiple assignments - need to handle specially
				// For now, just handle the simple case of parallel assignment
				if s.Tok == token.DEFINE {
					out.WriteString("let (")
					for i, lhs := range s.Lhs {
						if i > 0 {
							out.WriteString(", ")
						}
						// Don't add mut before blank identifier
						if ident, ok := lhs.(*ast.Ident); !ok || ident.Name != "_" {
							out.WriteString("mut ")
						}
						TranspileExpressionContext(out, lhs, LValue)
					}
					out.WriteString(") = (")
					for i, rhs := range s.Rhs {
						if i > 0 {
							out.WriteString(", ")
						}
						// Check if RHS already returns wrapped values
						if _, isCall := rhs.(*ast.CallExpr); isCall {
							// Function calls already return wrapped values
							TranspileExpression(out, rhs)
						} else if _, isSlice := rhs.(*ast.SliceExpr); isSlice {
							// Slice expressions already return wrapped values
							TranspileExpression(out, rhs)
						} else {
							// Wrap new variables
							WriteWrapperPrefix(out)
							TranspileExpression(out, rhs)
							WriteWrapperSuffix(out)
						}
					}
					out.WriteString(")")
				} else {
					// For reassignment, we need temporary variables in Rust
					// This is a simplification - just do individual assignments
					out.WriteString("{ ")
					for i := range s.Lhs {
						if i > 0 {
							out.WriteString("; ")
						}
						// Assignment to wrapped variable: *var.lock().unwrap() = Some(value)
						out.WriteString("*")
						TranspileExpression(out, s.Lhs[i])
						WriteBorrowMethod(out, true)
						out.WriteString(" = Some(")
						TranspileExpression(out, s.Rhs[i])
						out.WriteString(")")
					}
					out.WriteString(" }")
				}
			} else {
				// Single assignment
				// Check if we're assigning to just blank identifier
				if len(s.Lhs) == 1 {
					if ident, ok := s.Lhs[0].(*ast.Ident); ok && ident.Name == "_" {
						// Assignment to _ only - just evaluate the RHS for side effects
						out.WriteString("let _ = ")
						for i, rhs := range s.Rhs {
							if i > 0 {
								out.WriteString(", ")
							}
							TranspileExpression(out, rhs)
						}
					} else {
						// Normal single assignment
						if s.Tok == token.ASSIGN {
							// Assignment to wrapped variable
							// Check if LHS is a dereference (*p = value)
							if star, ok := s.Lhs[0].(*ast.StarExpr); ok {
								// Assignment through pointer: *p = value
								out.WriteString("{ ")
								out.WriteString("let new_val = ")
								TranspileExpression(out, s.Rhs[0])
								out.WriteString("; ")
								out.WriteString("*")
								TranspileExpressionContext(out, star.X, LValue)
								WriteBorrowMethod(out, true)
								out.WriteString(" = Some(new_val); }")
							} else if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok && !isMapIndexAssign {
								// Array/slice element assignment: arr[i] = value
								out.WriteString("(*")
								TranspileExpressionContext(out, indexExpr.X, LValue)
								WriteBorrowMethod(out, true)
								out.WriteString(".as_mut().unwrap())[")
								TranspileExpression(out, indexExpr.Index)
								out.WriteString("] = ")

								// Check if RHS is a call that returns a wrapped value
								needsUnwrap := false
								if call, ok := s.Rhs[0].(*ast.CallExpr); ok {
									// Use TypeInfo to check if this returns a wrapped value
									typeInfo := GetTypeInfo()
									if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) {
										needsUnwrap = true
									} else {
										// Fallback: Check if it's calling a closure variable
										if ident, ok := call.Fun.(*ast.Ident); ok {
											// If it's not a known function, it might be a closure variable
											if !isBuiltinFunction(ident.Name) && !isFunctionName(ident) {
												needsUnwrap = true
											}
										}
									}
								}

								if needsUnwrap {
									out.WriteString("(*")
									TranspileExpression(out, s.Rhs[0])
									WriteBorrowMethod(out, false)
									out.WriteString(".as_ref().unwrap())")
								} else {
									TranspileExpression(out, s.Rhs[0])
								}
							} else {
								// Direct assignment: x = value
								// Check if RHS is nil
								if ident, ok := s.Rhs[0].(*ast.Ident); ok && ident.Name == "nil" {
									// Assigning nil to pointer
									out.WriteString("*")
									TranspileExpressionContext(out, s.Lhs[0], LValue)
									WriteBorrowMethod(out, true)
									out.WriteString(" = None")
								} else if unary, ok := s.Rhs[0].(*ast.UnaryExpr); ok && unary.Op == token.AND {
									// Special case: p = &x where p is a pointer
									// We need to extract the value from x, not clone the whole Arc
									out.WriteString("{ ")
									out.WriteString("let new_val = (*")
									TranspileExpressionContext(out, unary.X, LValue)
									WriteBorrowMethod(out, false)
									out.WriteString(").clone(); ")
									out.WriteString("*")
									TranspileExpressionContext(out, s.Lhs[0], LValue)
									WriteBorrowMethod(out, true)
									out.WriteString(" = new_val; }")
								} else if call, ok := s.Rhs[0].(*ast.CallExpr); ok {
									// Check if it's an append call using TypeInfo
									isAppend := false
									isErrorFunc := false
									typeInfo := GetTypeInfo()
									if typeInfo != nil && typeInfo.info != nil {
										if ident, ok := call.Fun.(*ast.Ident); ok {
											// Check if this is the builtin append function
											if obj, ok := typeInfo.info.Uses[ident]; ok {
												if builtin, ok := obj.(*types.Builtin); ok {
													isAppend = builtin.Name() == "append"
												}
											}
										}
									}

									// Check if it's fmt.Errorf or errors.New which return error types
									if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
										if pkg, ok := sel.X.(*ast.Ident); ok {
											if (pkg.Name == "fmt" && sel.Sel.Name == "Errorf") ||
												(pkg.Name == "errors" && sel.Sel.Name == "New") {
												isErrorFunc = true
											}
										}
									}

									if isAppend {
										// append() returns the same wrapped type, don't wrap in Some()
										// Just execute the append for its side effect
										TranspileExpression(out, s.Rhs[0])
									} else if isErrorFunc {
										// fmt.Errorf and errors.New already return wrapped Option<Box<dyn Error>>
										// Don't wrap in Some()
										out.WriteString("{ ")
										out.WriteString("let new_val = ")
										TranspileExpression(out, s.Rhs[0])
										out.WriteString("; ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = new_val; }")
									} else { // Regular function call
										out.WriteString("{ ")
										out.WriteString("let new_val = ")
										TranspileExpression(out, s.Rhs[0])
										out.WriteString("; ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									}
								} else {
									// Check if LHS is interface{} type
									isInterface := false
									typeInfo := GetTypeInfo()
									if typeInfo != nil {
										if lhsType := typeInfo.GetType(s.Lhs[0]); lhsType != nil {
											// Check if it's the empty interface
											if intf, ok := lhsType.Underlying().(*types.Interface); ok && intf.NumMethods() == 0 {
												isInterface = true
											}
										}
									}

									if isInterface {
										// Assignment to interface{} - need to box the value
										out.WriteString("{ ")
										out.WriteString("let new_val = Box::new(")
										TranspileExpression(out, s.Rhs[0])
										out.WriteString(") as Box<dyn Any>; ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									} else {
										out.WriteString("{ ")
										out.WriteString("let new_val = ")
										TranspileExpression(out, s.Rhs[0])
										out.WriteString("; ")
										out.WriteString("*")
										TranspileExpressionContext(out, s.Lhs[0], LValue)
										WriteBorrowMethod(out, true)
										out.WriteString(" = Some(new_val); }")
									}
								}
							}
						} else {
							// Regular assignment or definition
							for i, lhs := range s.Lhs {
								if i > 0 {
									out.WriteString(", ")
								}
								if s.Tok == token.DEFINE {
									out.WriteString("let mut ")
								}
								TranspileExpressionContext(out, lhs, LValue)
							}

							out.WriteString(" = ")

							for i, rhs := range s.Rhs {
								if i > 0 {
									out.WriteString(", ")
								}
								if s.Tok == token.DEFINE {
									// Check if RHS is nil
									if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
										WriteWrappedNone(out)
									} else if unary, ok := rhs.(*ast.UnaryExpr); ok && unary.Op == token.AND {
										// Taking address - don't wrap, the & operator will handle it
										TranspileExpression(out, rhs)
									} else if _, isCall := rhs.(*ast.CallExpr); isCall {
										// Function calls already return wrapped values, don't wrap again
										TranspileExpression(out, rhs)
									} else if _, isFuncLit := rhs.(*ast.FuncLit); isFuncLit {
										// Function literals are already wrapped by TranspileFuncLit
										TranspileExpression(out, rhs)
									} else if compositeLit, isCompositeLit := rhs.(*ast.CompositeLit); isCompositeLit {
										// Check if it's a struct literal vs array/slice/map literal
										isStructLiteral := false
										if _, ok := compositeLit.Type.(*ast.Ident); ok {
											isStructLiteral = true
										} else if _, ok := compositeLit.Type.(*ast.StructType); ok {
											isStructLiteral = true
										}

										if isStructLiteral {
											// Struct literals need to be wrapped
											WriteWrapperPrefix(out)
											TranspileExpression(out, rhs)
											WriteWrapperSuffix(out)
										} else {
											// Array/slice/map literals already wrap themselves
											TranspileExpression(out, rhs)
										}
									} else if _, isSliceExpr := rhs.(*ast.SliceExpr); isSliceExpr {
										// Slice expressions already return wrapped values
										TranspileExpression(out, rhs)
									} else {
										// Wrap new variables
										WriteWrapperPrefix(out)
										TranspileExpression(out, rhs)
										WriteWrapperSuffix(out)
									}
								} else {
									TranspileExpression(out, rhs)
								}
							}
						}
					}
				} else {
					// Multiple LHS
					for i, lhs := range s.Lhs {
						if i > 0 {
							out.WriteString(", ")
						}
						if s.Tok == token.DEFINE {
							out.WriteString("let mut ")
						}
						TranspileExpression(out, lhs)
					}

					out.WriteString(" = ")

					for i, rhs := range s.Rhs {
						if i > 0 {
							out.WriteString(", ")
						}
						if s.Tok == token.DEFINE {
							// Check if RHS is an expression that already returns wrapped values
							if _, isCall := rhs.(*ast.CallExpr); isCall {
								// Function calls already return wrapped values, don't wrap again
								TranspileExpression(out, rhs)
							} else if _, isSlice := rhs.(*ast.SliceExpr); isSlice {
								// Slice expressions already return wrapped values
								TranspileExpression(out, rhs)
							} else {
								// Wrap new variables in Arc<Mutex<Option<>>>
								WriteWrapperPrefix(out)
								TranspileExpression(out, rhs)
								WriteWrapperSuffix(out)
							}
						} else {
							TranspileExpression(out, rhs)
						}
					}
				}
			}
		}
		out.WriteString(";")
	case *ast.DeclStmt:
		if genDecl, ok := s.Decl.(*ast.GenDecl); ok {
			switch genDecl.Tok {
			case token.VAR:
				for _, spec := range genDecl.Specs {
					if valueSpec, ok := spec.(*ast.ValueSpec); ok {
						for i, name := range valueSpec.Names {
							if i > 0 {
								out.WriteString(", ")
							}
							out.WriteString("let mut ")
							out.WriteString(name.Name)

							// Add type annotation if type is specified
							if valueSpec.Type != nil {
								out.WriteString(": ")
								out.WriteString(GoTypeToRust(valueSpec.Type))
							}

							if len(valueSpec.Values) > i {
								out.WriteString(" = ")
								// Check if value is nil
								if ident, ok := valueSpec.Values[i].(*ast.Ident); ok && ident.Name == "nil" {
									// Initializing with nil
									WriteWrappedNone(out)
								} else if _, isCall := valueSpec.Values[i].(*ast.CallExpr); isCall {
									// Function calls already return wrapped values, don't wrap again
									TranspileExpression(out, valueSpec.Values[i])
								} else if compositeLit, isCompositeLit := valueSpec.Values[i].(*ast.CompositeLit); isCompositeLit {
									// Check if it's a struct literal vs array/slice/map literal
									isStructLiteral := false
									if _, ok := compositeLit.Type.(*ast.Ident); ok {
										isStructLiteral = true
									} else if _, ok := compositeLit.Type.(*ast.StructType); ok {
										isStructLiteral = true
									}

									if isStructLiteral {
										// Struct literals need to be wrapped
										WriteWrapperPrefix(out)
										TranspileExpression(out, valueSpec.Values[i])
										WriteWrapperSuffix(out)
									} else {
										// Array/slice/map literals already wrap themselves
										TranspileExpression(out, valueSpec.Values[i])
									}
								} else if unary, ok := valueSpec.Values[i].(*ast.UnaryExpr); ok && unary.Op == token.AND {
									// Address-of operator already produces wrapped value
									TranspileExpression(out, valueSpec.Values[i])
								} else {
									// Check if the target type is interface{}
									isInterface := false
									if valueSpec.Type != nil {
										if intf, ok := valueSpec.Type.(*ast.InterfaceType); ok && len(intf.Methods.List) == 0 {
											isInterface = true
										}
									}

									if isInterface {
										// For interface{}, box the value
										WriteWrapperPrefix(out)
										out.WriteString("Box::new(")
										TranspileExpression(out, valueSpec.Values[i])
										out.WriteString(") as Box<dyn Any>)))")
									} else {
										// Wrap all variables in Arc<Mutex<Option<>>>
										WriteWrapperPrefix(out)
										TranspileExpression(out, valueSpec.Values[i])
										WriteWrapperSuffix(out)
									}
								}
							} else {
								// Default initialization for uninitialized vars
								if valueSpec.Type != nil {
									switch t := valueSpec.Type.(type) {
									case *ast.Ident:
										switch t.Name {
										case "string":
											out.WriteString(" = String::new()")
										case "int":
											out.WriteString(" = 0")
										default:
											out.WriteString(" = Default::default()")
										}
									case *ast.StarExpr:
										// Pointer type - initialize with None
										out.WriteString(" = ")
										WriteWrappedNone(out)
									case *ast.InterfaceType:
										// interface{} - initialize with None
										if len(t.Methods.List) == 0 {
											out.WriteString(" = ")
											WriteWrappedNone(out)
										}
									case *ast.ArrayType:
										// Initialize array with default values
										// Arrays are wrapped, so we need Some(default array)
										out.WriteString(" = ")
										WriteWrapperPrefix(out)
										out.WriteString("Default::default())))")
									}
								}
							}
							out.WriteString(";")
						}
					}
				}
			case token.CONST:
				// Handle local const declarations - keep original case
				transpileConstDeclWithCase(out, genDecl, false)
			case token.TYPE:
				// Handle local type declarations
				for _, spec := range genDecl.Specs {
					if typeSpec, ok := spec.(*ast.TypeSpec); ok {
						// For now, just generate type aliases inside functions
						// These should be hoisted to module level in a real implementation
						out.WriteString("type ")
						out.WriteString(typeSpec.Name.Name)
						out.WriteString(" = ")
						out.WriteString(GoTypeToRust(typeSpec.Type))
						out.WriteString(";")
					}
				}
			}
		}

	case *ast.ForStmt:
		if s.Init != nil {
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			out.WriteString("\n    ")
		}

		out.WriteString("while ")
		if s.Cond != nil {
			TranspileExpression(out, s.Cond)
		} else {
			out.WriteString("true")
		}
		out.WriteString(" {\n")

		var prevStmt ast.Stmt
		var forBodyLastPos token.Pos = s.Body.Lbrace
		for _, stmt := range s.Body.List {
			// Add blank line if there was one in the source
			if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
				out.WriteString("\n")
			}

			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &forBodyLastPos, "        ")
			out.WriteString("\n")

			prevStmt = stmt
		}

		// Add the post statement (increment) if present
		if s.Post != nil {
			out.WriteString("        ")
			TranspileStatementSimple(out, s.Post, fnType, fileSet)
			out.WriteString("\n")
		}

		out.WriteString("    }")

	case *ast.BlockStmt:
		out.WriteString("{\n")
		var prevStmt ast.Stmt
		var blockLastPos token.Pos = s.Lbrace
		for _, stmt := range s.List {
			// Add blank line if there was one in the source
			if prevStmt != nil && hasBlankLineBetween(fileSet, prevStmt.End(), stmt.Pos()) {
				out.WriteString("\n")
			}

			out.WriteString(indent)
			out.WriteString("    ")
			// Pass comments through for nested blocks
			TranspileStatement(out, stmt, fnType, fileSet, comments, &blockLastPos, indent+"    ")
			out.WriteString("\n")

			prevStmt = stmt
		}
		out.WriteString(indent)
		out.WriteString("}")

	case *ast.IncDecStmt:
		// For wrapped variables, we need to update the value inside
		out.WriteString("{ ")
		out.WriteString("let mut guard = ")
		TranspileExpressionContext(out, s.X, LValue)
		WriteBorrowMethod(out, true)
		out.WriteString("; ")
		out.WriteString("*guard = Some(guard.as_ref().unwrap() ")
		if s.Tok == token.INC {
			out.WriteString("+ 1")
		} else {
			out.WriteString("- 1")
		}
		out.WriteString("); }")

	case *ast.RangeStmt:
		// Handle for range loops
		out.WriteString("for ")

		// Track range loop variables so we don't try to unwrap them
		var keyName, valueName string

		// Use type information to determine what we're iterating over
		typeInfo := GetTypeInfo()
		isMap := false
		isString := false

		if typeInfo != nil {
			isMap = typeInfo.IsMap(s.X)
			isString = typeInfo.IsString(s.X)
		} else {
			// Type info not available - generate error
			out.WriteString("/* ERROR: Cannot determine range type - type information required */\n")
			out.WriteString("unimplemented!(\"type info required for range statement\")")
			return
		}

		// Determine types based on what we're iterating over
		keyType := "usize" // Default for slice indices
		valueType := "T"   // Generic placeholder

		if isMap {
			keyType = "String"                    // Common key type for maps
			valueType = "Arc<Mutex<Option<i32>>>" // Map values are wrapped
		} else if typeInfo != nil && typeInfo.IsSlice(s.X) {
			// Check if it's a slice of interface{}
			elemType := typeInfo.GetSliceElemType(s.X)
			if elemType != nil {
				if intf, ok := elemType.Underlying().(*types.Interface); ok && intf.NumMethods() == 0 {
					// It's []interface{} - elements are Box<dyn Any>
					// When iterating with &, we get &Box<dyn Any>
					valueType = "&Box<dyn Any>"
				}
			}
		}

		if s.Key != nil {
			if ident, ok := s.Key.(*ast.Ident); ok {
				keyName = ident.Name
				rangeLoopVars[keyName] = keyType
			}
		}
		if s.Value != nil {
			if ident, ok := s.Value.(*ast.Ident); ok {
				valueName = ident.Name
				// When using iter().enumerate(), the value is a reference
				// But keep the specific type if we already determined it (e.g., &Box<dyn Any>)
				if s.Key != nil && !isMap && !isString && valueType == "T" {
					rangeLoopVars[valueName] = "ref_value"
				} else {
					rangeLoopVars[valueName] = valueType
				}
			}
		}

		if isString {
			// String iteration - iterate over chars
			if s.Key != nil && s.Value != nil {
				// for i, c := range str
				out.WriteString("(")
				TranspileExpression(out, s.Key)
				out.WriteString(", ")
				TranspileExpression(out, s.Value)
				out.WriteString(") in (*")
				TranspileExpression(out, s.X)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).chars().enumerate()")
			} else if s.Value != nil {
				// for _, c := range str
				TranspileExpression(out, s.Value)
				out.WriteString(" in (*")
				TranspileExpression(out, s.X)
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).chars()")
			}
		} else if isMap {
			// Map iteration - need to unwrap the Arc<Mutex<Option<HashMap>>>
			if s.Key != nil && s.Value != nil {
				// for k, v := range map
				out.WriteString("(")
				if ident, ok := s.Key.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, s.Key)
				}
				out.WriteString(", ")
				if ident, ok := s.Value.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, s.Value)
				}
				out.WriteString(") in (*")
				// For map access, we need the raw identifier, not the unwrapped value
				if ident, ok := s.X.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, s.X)
				}
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
			} else if s.Value != nil {
				// for _, v := range map (values only)
				out.WriteString("(_, ")
				if ident, ok := s.Value.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, s.Value)
				}
				out.WriteString(") in (*")
				// For map access, we need the raw identifier, not the unwrapped value
				if ident, ok := s.X.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, s.X)
				}
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
			} else if s.Key != nil {
				// for k := range map (keys only)
				out.WriteString("(")
				if ident, ok := s.Key.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, s.Key)
				}
				out.WriteString(", _) in (*")
				// For map access, we need the raw identifier, not the unwrapped value
				if ident, ok := s.X.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, s.X)
				}
				WriteBorrowMethod(out, false)
				out.WriteString(".as_ref().unwrap()).clone()")
			}
		} else {
			// Array/slice iteration
			if s.Key != nil && s.Value != nil {
				// Check if key is blank identifier
				if keyIdent, ok := s.Key.(*ast.Ident); ok && keyIdent.Name == "_" {
					// for _, v := range arr - just iterate values
					if ident, ok := s.Value.(*ast.Ident); ok {
						out.WriteString(ident.Name)
					} else {
						TranspileExpression(out, s.Value)
					}
					out.WriteString(" in &")
					TranspileExpressionContext(out, s.X, RValue)
				} else {
					// for i, v := range arr
					out.WriteString("(")
					// Just output the identifier names, don't wrap them
					if ident, ok := s.Key.(*ast.Ident); ok {
						out.WriteString(ident.Name)
					} else {
						TranspileExpression(out, s.Key)
					}
					out.WriteString(", ")
					if ident, ok := s.Value.(*ast.Ident); ok {
						out.WriteString(ident.Name)
					} else {
						TranspileExpression(out, s.Value)
					}
					out.WriteString(") in ")
					// Need to unwrap the collection
					TranspileExpressionContext(out, s.X, RValue)
					out.WriteString(".iter().enumerate()")
				}
			} else if s.Value != nil {
				// for _, v := range arr
				if ident, ok := s.Value.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, s.Value)
				}
				out.WriteString(" in &")
				TranspileExpressionContext(out, s.X, RValue)
			} else if s.Key != nil {
				// for i := range arr
				if ident, ok := s.Key.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, s.Key)
				}
				out.WriteString(" in 0..")
				TranspileExpressionContext(out, s.X, RValue)
				out.WriteString(".len()")
			}
		}
		out.WriteString(" {\n")

		var rangeBodyLastPos token.Pos = s.Body.Lbrace
		for _, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &rangeBodyLastPos, "        ")
			out.WriteString("\n")
		}

		out.WriteString("    }")

		// Clean up range loop variables
		if keyName != "" {
			delete(rangeLoopVars, keyName)
		}
		if valueName != "" {
			delete(rangeLoopVars, valueName)
		}

	case *ast.IfStmt:
		// Handle init statement if present
		if s.Init != nil {
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			out.WriteString("\n    ")
		}

		out.WriteString("if ")
		TranspileExpression(out, s.Cond)
		out.WriteString(" {\n")

		// Use comment-aware transpilation for the body
		var ifBodyLastPos token.Pos = s.Body.Lbrace
		for _, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType, fileSet, comments, &ifBodyLastPos, "        ")
			out.WriteString("\n")
		}

		out.WriteString("    }")

		if s.Else != nil {
			out.WriteString(" else ")
			if elseIf, ok := s.Else.(*ast.IfStmt); ok {
				// else if case
				if elseIf.Init != nil {
					// If the else-if has an init statement, we need to wrap it in a block
					out.WriteString("{\n        ")
					TranspileStatementSimple(out, elseIf.Init, fnType, fileSet)
					out.WriteString(";\n        ")
					out.WriteString("if ")
					TranspileExpression(out, elseIf.Cond)
					out.WriteString(" {\n")
					for _, stmt := range elseIf.Body.List {
						out.WriteString("            ")
						TranspileStatementSimple(out, stmt, fnType, fileSet)
						out.WriteString(";\n")
					}
					out.WriteString("        }")
					// Handle nested else
					if elseIf.Else != nil {
						out.WriteString(" else ")
						if nestedElseIf, ok := elseIf.Else.(*ast.IfStmt); ok {
							// Recursively handle nested else-if
							TranspileStatementSimple(out, nestedElseIf, fnType, fileSet)
						} else if block, ok := elseIf.Else.(*ast.BlockStmt); ok {
							out.WriteString("{\n")
							for _, stmt := range block.List {
								out.WriteString("            ")
								TranspileStatementSimple(out, stmt, fnType, fileSet)
								out.WriteString(";\n")
							}
							out.WriteString("        }")
						}
					}
					out.WriteString("\n    }")
				} else {
					// No init statement, handle normally
					TranspileStatementSimple(out, elseIf, fnType, fileSet)
				}
			} else if block, ok := s.Else.(*ast.BlockStmt); ok {
				// else block
				out.WriteString("{\n")
				var elseBodyLastPos token.Pos = block.Lbrace
				for _, stmt := range block.List {
					out.WriteString("        ")
					TranspileStatement(out, stmt, fnType, fileSet, comments, &elseBodyLastPos, "        ")
					out.WriteString("\n")
				}
				out.WriteString("    }")
			}
		}

	case *ast.SwitchStmt:
		// Handle init statement if present
		if s.Init != nil {
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			out.WriteString("\n    ")
		}

		out.WriteString("match ")
		if s.Tag != nil {
			// Switch with expression
			TranspileExpression(out, s.Tag)
		} else {
			// Switch without expression - use true
			out.WriteString("true")
		}
		out.WriteString(" {\n")

		// Process cases
		hasDefault := false
		for _, stmt := range s.Body.List {
			if caseClause, ok := stmt.(*ast.CaseClause); ok {
				out.WriteString("        ")
				if caseClause.List == nil {
					// default case
					out.WriteString("_")
					hasDefault = true
				} else {
					// Regular case(s)
					for i, expr := range caseClause.List {
						if i > 0 {
							out.WriteString(" | ")
						}
						// For switch without expression, we need to handle boolean conditions
						if s.Tag == nil {
							// The expression is a condition, we need to match on true
							// and put the condition as a guard
							out.WriteString("true if ")
							TranspileExpression(out, expr)
						} else {
							TranspileExpression(out, expr)
						}
					}
				}
				out.WriteString(" => {\n")

				// Case body
				var caseBodyLastPos token.Pos = caseClause.Colon
				for _, bodyStmt := range caseClause.Body {
					out.WriteString("            ")
					TranspileStatement(out, bodyStmt, fnType, fileSet, comments, &caseBodyLastPos, "            ")
					out.WriteString("\n")
				}

				out.WriteString("        }\n")
			}
		}

		// Add default case if not present (required for exhaustive matching in Rust)
		if !hasDefault {
			out.WriteString("        _ => {}\n")
		}

		out.WriteString("    }")

	case *ast.BranchStmt:
		switch s.Tok {
		case token.BREAK:
			out.WriteString("break")
		case token.CONTINUE:
			out.WriteString("continue")
		case token.GOTO:
			out.WriteString("// TODO: goto not supported")
		case token.FALLTHROUGH:
			out.WriteString("// TODO: fallthrough not supported")
		}

	case *ast.DeferStmt:
		// Check if the defer contains a closure that captures variables
		captured := findCapturedInCall(s.Call)

		// Generate clones for captured variables
		// Sort variable names for deterministic output
		var capturedVars []string
		for varName := range captured {
			capturedVars = append(capturedVars, varName)
		}
		slices.Sort(capturedVars)

		captureRenames := make(map[string]string)
		for _, varName := range capturedVars {
			cloneName := varName + "_defer_captured"
			captureRenames[varName] = cloneName
			out.WriteString("let ")
			out.WriteString(cloneName)
			out.WriteString(" = ")
			out.WriteString(varName)
			out.WriteString(".clone(); ")
		}

		// Store current capture renames for nested transpilation
		oldCaptureRenames := currentCaptureRenames
		currentCaptureRenames = captureRenames

		// Check if the defer is calling an immediately invoked function literal
		// e.g., defer func(x int) { ... }(y)
		if funcLit, ok := s.Call.Fun.(*ast.FuncLit); ok && len(s.Call.Args) > 0 {
			// It's an immediately invoked function literal with arguments
			// We need to capture the argument values immediately

			// Generate captures for the arguments
			argCaptures := make([]string, len(s.Call.Args))
			for i, arg := range s.Call.Args {
				captureVar := fmt.Sprintf("__defer_arg_%d", i)
				argCaptures[i] = captureVar
				out.WriteString("let ")
				out.WriteString(captureVar)
				out.WriteString(" = ")

				// Check if argument needs wrapping
				// For defer arguments, we need to capture the VALUE at this moment,
				// not a reference that could change later
				if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "_" {
					// Check if this is a variable (not a constant)
					if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
						if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
							// It's a variable - capture its current value, not the reference
							// This ensures each defer gets the value at the time of deferring
							WriteWrapperPrefix(out)
							out.WriteString("(*")
							out.WriteString(ident.Name)
							WriteBorrowMethod(out, false)
							out.WriteString(".as_ref().unwrap()).clone()")
							WriteWrapperSuffix(out)
						} else {
							// It's a constant, wrap it
							WriteWrapperPrefix(out)
							TranspileExpression(out, arg)
							WriteWrapperSuffix(out)
						}
					} else {
						// Range variable, wrap it
						WriteWrapperPrefix(out)
						TranspileExpression(out, arg)
						WriteWrapperSuffix(out)
					}
				} else {
					// Complex expression or literal, wrap it
					WriteWrapperPrefix(out)
					TranspileExpression(out, arg)
					WriteWrapperSuffix(out)
				}
				out.WriteString("; ")
			}

			// Now generate the defer with the captured arguments
			out.WriteString("__defer_stack.push(Box::new(move || {\n")
			out.WriteString("        ")

			// Generate the closure directly (without Arc wrapper)
			out.WriteString("(move |")
			// Parameters
			if funcLit.Type.Params != nil {
				var params []string
				for _, field := range funcLit.Type.Params.List {
					paramType := GoTypeToRust(field.Type)
					for _, name := range field.Names {
						params = append(params, name.Name+": "+paramType)
					}
				}
				out.WriteString(strings.Join(params, ", "))
			}
			out.WriteString("| {\n        ")

			// Body
			for _, stmt := range funcLit.Body.List {
				TranspileStatementSimple(out, stmt, funcLit.Type, fileSet)
				out.WriteString("; ")
			}

			out.WriteString("\n        })(")
			for i, capture := range argCaptures {
				if i > 0 {
					out.WriteString(", ")
				}
				out.WriteString(capture)
			}
			out.WriteString(");\n")
			out.WriteString("    }))")
		} else {
			// Regular defer call
			out.WriteString("__defer_stack.push(Box::new(move || {\n")
			out.WriteString("        ")
			TranspileCall(out, s.Call)
			out.WriteString(";\n")
			out.WriteString("    }))")
		}
		out.WriteString(";")

		// Restore previous capture renames
		currentCaptureRenames = oldCaptureRenames

	case *ast.GoStmt:
		// Track that we need thread import
		TrackImport("thread")

		// Check if the go statement contains a closure that captures variables
		captured := findCapturedInCall(s.Call)

		// Generate clones for captured variables
		// Sort variable names for deterministic output
		var capturedVars []string
		for varName := range captured {
			capturedVars = append(capturedVars, varName)
		}
		slices.Sort(capturedVars)

		for _, varName := range capturedVars {
			out.WriteString("let ")
			out.WriteString(varName)
			out.WriteString("_thread = ")
			out.WriteString(varName)
			out.WriteString(".clone(); ")
		}

		// Store current capture renames for nested transpilation
		captureRenames := make(map[string]string)
		for _, varName := range capturedVars {
			captureRenames[varName] = varName + "_thread"
		}
		oldCaptureRenames := currentCaptureRenames
		currentCaptureRenames = captureRenames

		// Generate the thread::spawn call
		out.WriteString("std::thread::spawn(move || {\n")
		out.WriteString("        ")

		// Check if it's an immediately invoked function literal
		if funcLit, ok := s.Call.Fun.(*ast.FuncLit); ok {
			// Generate the closure body inline
			if len(s.Call.Args) > 0 {
				// Has arguments - need to create parameter bindings
				out.WriteString("let __closure = move |")
				// Parameters
				if funcLit.Type.Params != nil {
					var params []string
					for _, field := range funcLit.Type.Params.List {
						paramType := GoTypeToRust(field.Type)
						for _, name := range field.Names {
							params = append(params, name.Name+": "+paramType)
						}
					}
					out.WriteString(strings.Join(params, ", "))
				}
				out.WriteString("| {\n            ")

				// Body
				for i, stmt := range funcLit.Body.List {
					if i > 0 {
						out.WriteString("\n            ")
					}
					TranspileStatementSimple(out, stmt, funcLit.Type, fileSet)
					out.WriteString(";")
				}

				out.WriteString("\n        };\n")
				out.WriteString("        __closure(")

				// Arguments
				for i, arg := range s.Call.Args {
					if i > 0 {
						out.WriteString(", ")
					}
					// Wrap arguments appropriately
					if ident, ok := arg.(*ast.Ident); ok && ident.Name != "nil" && ident.Name != "_" {
						// Check if this is a variable (not a constant)
						if _, isRangeVar := rangeLoopVars[ident.Name]; !isRangeVar {
							if _, isLocalConst := localConstants[ident.Name]; !isLocalConst {
								// It's a variable, clone it
								if captureRenames[ident.Name] != "" {
									out.WriteString(captureRenames[ident.Name])
								} else {
									out.WriteString(ident.Name)
									out.WriteString(".clone()")
								}
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
						// Complex expression or literal, wrap it
						out.WriteString("Arc::new(Mutex::new(Some(")
						TranspileExpression(out, arg)
						out.WriteString(")))")
					}
				}
				out.WriteString(")")
			} else {
				// No arguments - just inline the body
				for i, stmt := range funcLit.Body.List {
					if i > 0 {
						out.WriteString("\n        ")
					}
					TranspileStatementSimple(out, stmt, funcLit.Type, fileSet)
					out.WriteString(";")
				}
			}
		} else {
			// Regular function call
			TranspileCall(out, s.Call)
		}

		out.WriteString(";\n")
		out.WriteString("    })")
		out.WriteString(";")

		// Restore previous capture renames
		currentCaptureRenames = oldCaptureRenames

	case *ast.TypeSwitchStmt:
		// Type switch: switch v := x.(type) { ... }
		// We'll convert this to a series of if-else type checks

		// Extract the variable name and expression
		var varName string
		var expr ast.Expr

		if s.Assign != nil {
			// Has assignment: v := x.(type)
			if assign, ok := s.Assign.(*ast.AssignStmt); ok && len(assign.Lhs) == 1 && len(assign.Rhs) == 1 {
				if ident, ok := assign.Lhs[0].(*ast.Ident); ok {
					varName = ident.Name
				}
				if typeAssert, ok := assign.Rhs[0].(*ast.TypeAssertExpr); ok {
					expr = typeAssert.X
				}
			}
		} else if s.Init != nil {
			// Has init statement
			TranspileStatementSimple(out, s.Init, fnType, fileSet)
			out.WriteString(";\n")
		}

		if expr == nil {
			out.WriteString("// ERROR: Invalid type switch format")
			break
		}

		// Generate match-like if-else chain
		for i, clause := range s.Body.List {
			caseClause := clause.(*ast.CaseClause)

			if i > 0 {
				out.WriteString(" else ")
			}

			if len(caseClause.List) == 0 {
				// default case - provide access to the original value
				out.WriteString("{\n")
				if varName != "" {
					out.WriteString("        let ")
					out.WriteString(varName)
					out.WriteString(" = ")
					TranspileExpression(out, expr)
					out.WriteString(";\n")
				}
			} else {
				// Type case(s)
				out.WriteString("if let Some(")
				if varName != "" {
					out.WriteString(varName)
				} else {
					out.WriteString("_val")
				}
				out.WriteString(") = (|| -> Option<Box<dyn Any>> {\n")
				out.WriteString("        let val = ")
				TranspileExpression(out, expr)
				out.WriteString(";\n")

				// Check if this is a range variable from an interface{} slice
				isRangeVar := false
				if ident, ok := expr.(*ast.Ident); ok {
					if varType, exists := rangeLoopVars[ident.Name]; exists && strings.Contains(varType, "&Box<dyn Any>") {
						isRangeVar = true
					}
				}

				if isRangeVar {
					// It's a reference to Box<dyn Any>, no need to unwrap RefCell
					// But we need to work with the Box<dyn Any> itself for downcast
					out.WriteString("        let any_val = val;\n")
					out.WriteString("        {\n")
				} else {
					// It's wrapped, need to unwrap
					out.WriteString("        let guard = val")
					WriteBorrowMethod(out, false)
					out.WriteString(";\n")
					out.WriteString("        if let Some(ref any_val) = *guard {\n")
				}

				// Check each type in the case
				for j, typeExpr := range caseClause.List {
					if j > 0 {
						out.WriteString("            } else if ")
					} else {
						out.WriteString("            if ")
					}

					// Get the Rust type name
					rustType := ""
					if ident, ok := typeExpr.(*ast.Ident); ok {
						switch ident.Name {
						case "string":
							rustType = "String"
						case "int":
							rustType = "i32"
						case "float64":
							rustType = "f64"
						case "bool":
							rustType = "bool"
						default:
							rustType = ident.Name
						}
					}

					// Check if we're dealing with a reference to Box<dyn Any>
					if isRangeVar {
						// For &Box<dyn Any>, use as_ref() to get &dyn Any
						out.WriteString("let Some(val) = any_val.as_ref().downcast_ref::<")
					} else {
						// For regular wrapped values
						out.WriteString("let Some(val) = any_val.downcast_ref::<")
					}
					out.WriteString(rustType)
					out.WriteString(">() {\n")
					out.WriteString("                return Some(Box::new(val.clone()) as Box<dyn Any>);\n")
				}

				out.WriteString("            }\n")
				out.WriteString("        }\n")
				out.WriteString("        None\n")
				out.WriteString("    })() {\n")

				// If we have a variable, we need to extract the actual typed value
				if varName != "" {
					// Extract the first type for the actual variable
					if len(caseClause.List) > 0 {
						rustType := ""
						if ident, ok := caseClause.List[0].(*ast.Ident); ok {
							switch ident.Name {
							case "string":
								rustType = "String"
							case "int":
								rustType = "i32"
							case "float64":
								rustType = "f64"
							case "bool":
								rustType = "bool"
							default:
								rustType = ident.Name
							}
						}
						// Wrap the extracted value so it works with the rest of the transpiler
						out.WriteString("        let ")
						out.WriteString(varName)
						out.WriteString(" = ")
						WriteWrapperPrefix(out)
						out.WriteString("(*")
						out.WriteString(varName)
						out.WriteString(".downcast_ref::<")
						out.WriteString(rustType)
						out.WriteString(">().unwrap()).clone())));\n")
					}
				}
			}

			// Case body
			for _, stmt := range caseClause.Body {
				out.WriteString("        ")
				TranspileStatementSimple(out, stmt, fnType, fileSet)
				out.WriteString(";\n")
			}

			out.WriteString("    }")
		}

	default:
		out.WriteString("// TODO: Unhandled statement type: " + strings.TrimPrefix(fmt.Sprintf("%T", s), "*ast."))
	}
}
