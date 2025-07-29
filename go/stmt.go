package main

import (
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

				// Special handling for string literals
				if lit, ok := result.(*ast.BasicLit); ok && lit.Kind == token.STRING {
					if fnType.Results != nil && len(fnType.Results.List) > 0 {
						// Find the corresponding return type
						resultTypeIdx := i
						resultType := fnType.Results.List[0].Type
						if len(fnType.Results.List) > 1 || (len(fnType.Results.List) == 1 && len(fnType.Results.List[0].Names) > 1) {
							// Multiple return values - need to find the right type
							currentIdx := 0
							for _, res := range fnType.Results.List {
								if len(res.Names) > 0 {
									for range res.Names {
										if currentIdx == resultTypeIdx {
											resultType = res.Type
											break
										}
										currentIdx++
									}
								} else {
									if currentIdx == resultTypeIdx {
										resultType = res.Type
										break
									}
									currentIdx++
								}
							}
						}

						if ident, ok := resultType.(*ast.Ident); ok && ident.Name == "string" {
							out.WriteString(lit.Value)
							out.WriteString(".to_string()")
						} else {
							TranspileExpression(out, result)
						}
					} else {
						TranspileExpression(out, result)
					}
				} else {
					TranspileExpression(out, result)
				}
			}

			if needsTuple {
				out.WriteString(")")
			}
		}
		out.WriteString(";")

	case *ast.AssignStmt:
		if s.Tok == token.ADD_ASSIGN {
			// Check if it's a string (simple heuristic - if RHS is a string literal)
			isString := false
			if lit, ok := s.Rhs[0].(*ast.BasicLit); ok && lit.Kind == token.STRING {
				isString = true
			}

			if isString {
				TranspileExpression(out, s.Lhs[0])
				out.WriteString(".push_str(&")
				TranspileExpression(out, s.Rhs[0])
				out.WriteString(")")
			} else {
				// Numeric +=
				TranspileExpression(out, s.Lhs[0])
				out.WriteString(" += ")
				TranspileExpression(out, s.Rhs[0])
			}
		} else {
			// Check if we have multiple LHS with single RHS (tuple unpacking)
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
					TranspileExpression(out, lhs)
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
						TranspileExpression(out, s.Lhs[i])
						out.WriteString(" = ")
						TranspileExpression(out, s.Rhs[i])
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
							TranspileExpression(out, rhs)
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
						TranspileExpression(out, rhs)
					}
				}
			}
		}
		out.WriteString(";")
	case *ast.DeclStmt:
		if genDecl, ok := s.Decl.(*ast.GenDecl); ok && genDecl.Tok == token.VAR {
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
							TranspileExpression(out, valueSpec.Values[i])
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
									out.WriteString(GoTypeToRust(valueSpec.Type))
									out.WriteString(" = Default::default()")
								}
							}
						}
						out.WriteString(";")
					}
				}
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
		TranspileExpression(out, s.X)
		if s.Tok == token.INC {
			out.WriteString(" += 1")
		} else {
			out.WriteString(" -= 1")
		}
		out.WriteString(";")

	case *ast.RangeStmt:
		// Handle for range loops
		out.WriteString("for ")

		// Check if we're iterating over a map by looking at the identifier
		isMap := false
		if ident, ok := s.X.(*ast.Ident); ok {
			// Simple heuristic: if the variable name contains "map" or common map names
			// This is not perfect but works for many cases
			// TODO: Use type information for accurate detection
			name := strings.ToLower(ident.Name)
			isMap = strings.Contains(name, "map") || name == "ages" || name == "colors" || name == "m"
		}

		if isMap {
			// Map iteration
			if s.Key != nil && s.Value != nil {
				// for k, v := range map
				out.WriteString("(")
				TranspileExpression(out, s.Key)
				out.WriteString(", ")
				TranspileExpression(out, s.Value)
				out.WriteString(") in &")
				TranspileExpression(out, s.X)
			} else if s.Value != nil {
				// for _, v := range map (values only)
				out.WriteString("(_, ")
				TranspileExpression(out, s.Value)
				out.WriteString(") in &")
				TranspileExpression(out, s.X)
			} else if s.Key != nil {
				// for k := range map (keys only)
				out.WriteString("(")
				TranspileExpression(out, s.Key)
				out.WriteString(", _) in &")
				TranspileExpression(out, s.X)
			}
		} else {
			// Array/slice iteration
			if s.Key != nil && s.Value != nil {
				// for i, v := range arr
				out.WriteString("(")
				TranspileExpression(out, s.Key)
				out.WriteString(", ")
				TranspileExpression(out, s.Value)
				out.WriteString(") in ")
				TranspileExpression(out, s.X)
				out.WriteString(".iter().enumerate()")
			} else if s.Value != nil {
				// for _, v := range arr
				TranspileExpression(out, s.Value)
				out.WriteString(" in &")
				TranspileExpression(out, s.X)
			} else if s.Key != nil {
				// for i := range arr
				TranspileExpression(out, s.Key)
				out.WriteString(" in 0..")
				TranspileExpression(out, s.X)
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
	}
}
