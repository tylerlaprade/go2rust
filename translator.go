package main

import (
	"bytes"
	"fmt"
	"go/ast"
	"unicode"
)

type RustProgram struct {
	Imports   []string
	Functions []RustFunction
}

type RustFunction struct {
	Name string
	Body []RustStatement
}

type RustStatement struct {
	Target string // e.g. "println!"
	Args   []string
}

type ImportTracker struct {
	needsHashMap bool
	needsArc     bool
	needsMutex   bool
	needsOption  bool
}

func Translate(file *ast.File) (*RustProgram, error) {
	program := &RustProgram{
		Imports:   []string{},
		Functions: []RustFunction{},
	}

	// Find main function
	for _, decl := range file.Decls {
		if funcDecl, ok := decl.(*ast.FuncDecl); ok {
			if funcDecl.Name.Name == "main" {
				rustFunc, err := translateFunction(funcDecl)
				if err != nil {
					return nil, err
				}
				program.Functions = append(program.Functions, *rustFunc)
			}
		}
	}

	return program, nil
}

func translateFunction(funcDecl *ast.FuncDecl) (*RustFunction, error) {
	rustFunc := &RustFunction{
		Name: funcDecl.Name.Name,
		Body: []RustStatement{},
	}

	// Translate function body
	for _, stmt := range funcDecl.Body.List {
		if exprStmt, ok := stmt.(*ast.ExprStmt); ok {
			if callExpr, ok := exprStmt.X.(*ast.CallExpr); ok {
				rustStmt, err := translateCall(callExpr)
				if err != nil {
					return nil, err
				}
				rustFunc.Body = append(rustFunc.Body, *rustStmt)
			}
		}
	}

	return rustFunc, nil
}

func translateCall(callExpr *ast.CallExpr) (*RustStatement, error) {
	// Handle fmt.Println specifically
	if selExpr, ok := callExpr.Fun.(*ast.SelectorExpr); ok {
		if ident, ok := selExpr.X.(*ast.Ident); ok {
			if ident.Name == "fmt" && selExpr.Sel.Name == "Println" {
				if len(callExpr.Args) > 0 {
					if basicLit, ok := callExpr.Args[0].(*ast.BasicLit); ok {
						return &RustStatement{
							Target: "println!",
							Args:   []string{basicLit.Value},
						}, nil
					}
				}
			}
		}
	}

	return nil, fmt.Errorf("unsupported call expression")
}

// Adapted from github.com/drewstone/go2rs (MIT License)
func toSnakeCase(s string) string {
	var result bytes.Buffer
	for i, r := range s {
		if i > 0 && unicode.IsUpper(r) {
			result.WriteByte('_')
		}
		result.WriteRune(unicode.ToLower(r))
	}
	return result.String()
}

func toRustMethodName(goMethod string) string {
	if len(goMethod) == 0 {
		return goMethod
	}
	return string(unicode.ToLower(rune(goMethod[0]))) + goMethod[1:]
}
