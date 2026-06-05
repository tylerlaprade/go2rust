package main

import (
	"fmt"
	"go/token"
)

func main() {
	src := "package p\nvar x int\n"
	fset := token.NewFileSet()
	file := fset.AddFile("p.go", -1, len(src))
	file.SetLinesForContent([]byte(src))

	pos := file.Pos(len("package p\n") + 1)
	position := fset.Position(pos)

	fmt.Println(fset.Base() > file.Base(), file.Name(), position.Filename, position.Line > 0, position.Column > 0)
}
