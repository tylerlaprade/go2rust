package main

import (
	"fmt"
	"go/parser"
	"go/token"
)

func main() {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "input.go", `package main

import (
	"fmt"
	alias "strings"
	_ "os"
)
`, parser.ImportsOnly)
	fmt.Println(err == nil, file.Name.Name, len(file.Imports))
	fmt.Println(file.Imports[0].Path.Value)
	fmt.Println(file.Imports[1].Name.Name, file.Imports[1].Path.Value)
	fmt.Println(file.Imports[2].Name.Name, file.Imports[2].Path.Value)
}
