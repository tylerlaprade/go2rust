package main

import (
	"go/ast"
	"go/parser"
	"go/token"
)

func ParseFile(filename string) (*ast.File, error) {
	file_set := token.NewFileSet()
	file, err := parser.ParseFile(file_set, filename, nil, parser.ParseComments)
	if err != nil {
		return nil, err
	}
	return file, nil
}
