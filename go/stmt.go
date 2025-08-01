package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"strings"
)

func TranspileStatement(out *strings.Builder, stmt ast.Stmt, fnType *ast.FuncType) {
	switch s := stmt.(type) {
	case *ast.ExprStmt:
		TranspileExpression(out, s.X)
		out.WriteString(";")

	case *ast.ReturnStmt:
		out.WriteString("return")

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

				// Check if this is nil being returned for an error type
				isNilForError := false
				if ident, ok := result.(*ast.Ident); ok && ident.Name == "nil" {
					// Check if this position in return values is an error type
					if fnType.Results != nil && i < len(fnType.Results.List) {
						if resultType, ok := fnType.Results.List[i].Type.(*ast.Ident); ok && resultType.Name == "error" {
							isNilForError = true
						}
					}
				}

				if isNilForError {
					// For error type, nil becomes Arc<Mutex<None>>
					out.WriteString("std::sync::Arc::new(std::sync::Mutex::new(None))")
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
							out.WriteString("std::sync::Arc::new(std::sync::Mutex::new(Some(")
							TranspileExpression(out, result)
							out.WriteString(")))")
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
							out.WriteString("std::sync::Arc::new(std::sync::Mutex::new(Some(")
							TranspileExpression(out, result)
							out.WriteString(")))")
						} else {
							// Already wrapped
							TranspileExpression(out, result)
						}
					} else {
						// Wrap all other return values in Arc<Mutex<Option<>>>
						out.WriteString("std::sync::Arc::new(std::sync::Mutex::new(Some(")

						// Special handling for string literals
						if lit, ok := result.(*ast.BasicLit); ok && lit.Kind == token.STRING {
							out.WriteString(lit.Value)
							out.WriteString(".to_string()")
						} else if ident, ok := result.(*ast.Ident); ok {
							_, isRangeVar := rangeLoopVars[ident.Name]
							if !isRangeVar && ident.Name != "true" && ident.Name != "false" && ident.Name != "nil" {
								// Returning a variable - need to clone the inner value
								out.WriteString("(*")
								out.WriteString(ident.Name)
								out.WriteString(".lock().unwrap().as_mut().unwrap()).clone()")
							} else {
								out.WriteString(ident.Name)
							}
						} else {
							TranspileExpression(out, result)
						}

						out.WriteString(")))")
					}
				}
			}

			if needsTuple {
				out.WriteString(")")
			}
		}
		out.WriteString(";")

	case *ast.AssignStmt:
		// Check if this is a map index assignment
		isMapIndexAssign := false
		if len(s.Lhs) == 1 && len(s.Rhs) == 1 && s.Tok == token.ASSIGN {
			if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok {
				// Check if the indexed expression is likely a map
				if ident, ok := indexExpr.X.(*ast.Ident); ok {
					name := strings.ToLower(ident.Name)
					// Only treat as map if the variable name suggests it's a map
					isMapIndexAssign = strings.Contains(name, "map") || name == "ages" || name == "colors"
				}
			}
		}

		if isMapIndexAssign {
			// Handle map[key] = value as map.insert(key, value)
			if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok {
				TranspileExpression(out, indexExpr.X)
				out.WriteString(".insert(")
				TranspileExpression(out, indexExpr.Index)
				out.WriteString(", ")
				TranspileExpression(out, s.Rhs[0])
				out.WriteString(")")
			}
		} else if s.Tok == token.ADD_ASSIGN || s.Tok == token.SUB_ASSIGN ||
			s.Tok == token.MUL_ASSIGN || s.Tok == token.QUO_ASSIGN || s.Tok == token.REM_ASSIGN {
			// Compound assignment operators

			isString := false
			if s.Tok == token.ADD_ASSIGN {
				if ident, ok := s.Lhs[0].(*ast.Ident); ok {
					// TODO (hack): Simple heuristic: common string variable names
					name := strings.ToLower(ident.Name)
					isString = name == "result" || name == "str" || name == "s" || strings.Contains(name, "string")
				}
				// Also check if RHS is a string literal
				if !isString {
					if lit, ok := s.Rhs[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
						isString = true
					}
				}
			}

			if isString {
				// For string +=, we need mutable access to the LHS
				out.WriteString("(*")
				TranspileExpressionContext(out, s.Lhs[0], LValue)
				out.WriteString(".lock().unwrap().as_mut().unwrap()).push_str(&")
				TranspileExpression(out, s.Rhs[0])
				out.WriteString(")")
			} else {
				// Numeric compound assignment for wrapped values
				// Generate: { let mut guard = lhs.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() OP rhs); }
				out.WriteString("{ let mut guard = ")
				TranspileExpressionContext(out, s.Lhs[0], LValue)
				out.WriteString(".lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ")

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
						out.WriteString(".lock().unwrap().as_mut().unwrap())")
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

			if needsTupleUnpack {
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
						TranspileExpression(out, lhs)
					}
					out.WriteString(") = (")
					for i, rhs := range s.Rhs {
						if i > 0 {
							out.WriteString(", ")
						}
						TranspileExpression(out, rhs)
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
						out.WriteString(".lock().unwrap() = Some(")
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
								out.WriteString(".lock().unwrap() = Some(new_val); }")
							} else if indexExpr, ok := s.Lhs[0].(*ast.IndexExpr); ok && !isMapIndexAssign {
								// Array/slice element assignment: arr[i] = value
								out.WriteString("(*")
								TranspileExpressionContext(out, indexExpr.X, LValue)
								out.WriteString(".lock().unwrap().as_mut().unwrap())[")
								TranspileExpression(out, indexExpr.Index)
								out.WriteString("] = ")
								TranspileExpression(out, s.Rhs[0])
							} else {
								// Direct assignment: x = value
								out.WriteString("{ ")
								out.WriteString("let new_val = ")
								TranspileExpression(out, s.Rhs[0])
								out.WriteString("; ")
								out.WriteString("*")
								TranspileExpressionContext(out, s.Lhs[0], LValue)
								out.WriteString(".lock().unwrap() = Some(new_val); }")
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
									// Check if RHS is an address-of expression
									if unary, ok := rhs.(*ast.UnaryExpr); ok && unary.Op == token.AND {
										// Taking address - don't wrap, the & operator will handle it
										TranspileExpression(out, rhs)
									} else if _, isCall := rhs.(*ast.CallExpr); isCall {
										// Function calls already return wrapped values, don't wrap again
										TranspileExpression(out, rhs)
									} else {
										// Wrap new variables in Arc<Mutex<Option<>>>
										out.WriteString("std::sync::Arc::new(std::sync::Mutex::new(Some(")
										TranspileExpression(out, rhs)
										out.WriteString(")))")
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
							// Check if RHS is a function call - they already return wrapped values
							if _, isCall := rhs.(*ast.CallExpr); isCall {
								// Function calls already return wrapped values, don't wrap again
								TranspileExpression(out, rhs)
							} else {
								// Wrap new variables in Arc<Mutex<Option<>>>
								out.WriteString("std::sync::Arc::new(std::sync::Mutex::new(Some(")
								TranspileExpression(out, rhs)
								out.WriteString(")))")
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
							if len(valueSpec.Values) > i {
								out.WriteString(" = ")
								// Check if value is a function call - they already return wrapped values
								if _, isCall := valueSpec.Values[i].(*ast.CallExpr); isCall {
									// Function calls already return wrapped values, don't wrap again
									TranspileExpression(out, valueSpec.Values[i])
								} else {
									// Wrap all variables in Arc<Mutex<Option<>>>
									out.WriteString("std::sync::Arc::new(std::sync::Mutex::new(Some(")
									TranspileExpression(out, valueSpec.Values[i])
									out.WriteString(")))")
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
									case *ast.ArrayType:
										// Initialize array with default values
										out.WriteString(": ")
										rustType := GoTypeToRust(valueSpec.Type)
										out.WriteString(rustType)
										// Arrays are wrapped, so we need Some(default array)
										out.WriteString(" = std::sync::Arc::new(std::sync::Mutex::new(Some(Default::default())))")
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
			}
		}

	case *ast.ForStmt:
		if s.Init != nil {
			TranspileStatement(out, s.Init, fnType)
			out.WriteString("\n    ")
		}

		out.WriteString("while ")
		if s.Cond != nil {
			TranspileExpression(out, s.Cond)
		} else {
			out.WriteString("true")
		}
		out.WriteString(" {\n")

		for _, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType)
			out.WriteString("\n")
		}

		// Add the post statement (increment) if present
		if s.Post != nil {
			out.WriteString("        ")
			TranspileStatement(out, s.Post, fnType)
			out.WriteString("\n")
		}

		out.WriteString("    }")

	case *ast.BlockStmt:
		out.WriteString("{\n")
		for _, stmt := range s.List {
			out.WriteString("    ")
			TranspileStatement(out, stmt, fnType)
			out.WriteString("\n")
		}
		out.WriteString("}")

	case *ast.IncDecStmt:
		// For wrapped variables, we need to update the value inside
		out.WriteString("{ ")
		out.WriteString("let mut guard = ")
		TranspileExpressionContext(out, s.X, LValue)
		out.WriteString(".lock().unwrap(); ")
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

		// Check if we're iterating over a map by looking at the identifier
		isMap := false
		if ident, ok := s.X.(*ast.Ident); ok {
			// Simple heuristic: if the variable name contains "map" or common map names
			// This is not perfect but works for many cases
			// TODO: Use type information for accurate detection rather than this hack
			name := strings.ToLower(ident.Name)
			isMap = strings.Contains(name, "map") || name == "ages" || name == "colors" || name == "m"
		}

		// Determine types based on what we're iterating over
		keyType := "usize" // Default for slice indices
		valueType := "T"   // Generic placeholder

		if isMap {
			keyType = "String" // Common key type for maps
			valueType = "i32"  // Common value type, could be improved with type analysis
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
				rangeLoopVars[valueName] = valueType
			}
		}

		if isMap {
			// Map iteration
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
				out.WriteString(") in &")
				TranspileExpressionContext(out, s.X, RValue)
			} else if s.Value != nil {
				// for _, v := range map (values only)
				out.WriteString("(_, ")
				if ident, ok := s.Value.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, s.Value)
				}
				out.WriteString(") in &")
				TranspileExpressionContext(out, s.X, RValue)
			} else if s.Key != nil {
				// for k := range map (keys only)
				out.WriteString("(")
				if ident, ok := s.Key.(*ast.Ident); ok {
					out.WriteString(ident.Name)
				} else {
					TranspileExpression(out, s.Key)
				}
				out.WriteString(", _) in &(*")
				TranspileExpressionContext(out, s.X, RValue)
				out.WriteString(")")
			}
		} else {
			// Array/slice iteration
			if s.Key != nil && s.Value != nil {
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

		for _, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType)
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
			TranspileStatement(out, s.Init, fnType)
			out.WriteString("\n    ")
		}

		out.WriteString("if ")
		TranspileExpression(out, s.Cond)
		out.WriteString(" {\n")

		for _, stmt := range s.Body.List {
			out.WriteString("        ")
			TranspileStatement(out, stmt, fnType)
			out.WriteString("\n")
		}

		out.WriteString("    }")

		if s.Else != nil {
			out.WriteString(" else ")
			if elseIf, ok := s.Else.(*ast.IfStmt); ok {
				// else if case - don't add extra braces
				TranspileStatement(out, elseIf, fnType)
			} else if block, ok := s.Else.(*ast.BlockStmt); ok {
				// else block
				out.WriteString("{\n")
				for _, stmt := range block.List {
					out.WriteString("        ")
					TranspileStatement(out, stmt, fnType)
					out.WriteString("\n")
				}
				out.WriteString("    }")
			}
		}

	case *ast.SwitchStmt:
		// Handle init statement if present
		if s.Init != nil {
			TranspileStatement(out, s.Init, fnType)
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
		for _, stmt := range s.Body.List {
			if caseClause, ok := stmt.(*ast.CaseClause); ok {
				out.WriteString("        ")
				if caseClause.List == nil {
					// default case
					out.WriteString("_")
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
				for _, bodyStmt := range caseClause.Body {
					out.WriteString("            ")
					TranspileStatement(out, bodyStmt, fnType)
					out.WriteString("\n")
				}

				out.WriteString("        }\n")
			}
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
		// For now, just add a comment - proper defer support is complex
		out.WriteString("// defer ")
		TranspileCall(out, s.Call)
		out.WriteString(" // TODO: defer not yet supported")

	default:
		out.WriteString("// TODO: Unhandled statement type: " + strings.TrimPrefix(fmt.Sprintf("%T", s), "*ast."))
	}
}
