package main

import (
	"fmt"
	"go/token"
)

func main() {
	keyword := token.Lookup("func")
	ident := token.Lookup("not_keyword")

	fmt.Println(keyword.String(), keyword.IsKeyword())
	fmt.Println(ident.String(), ident.IsKeyword())
	fmt.Println(token.Pos(1).IsValid(), token.NoPos.IsValid())
}
