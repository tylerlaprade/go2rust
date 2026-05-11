package main

import "fmt"

type Error struct {
	Msg string
}

func main() {
	stack := []string{"importer", "imported"}
	importingPkg := stack[len(stack)-2]
	additionalErrors := make(map[string][]Error)

	additionalErrors[importingPkg] = append(additionalErrors[importingPkg], Error{
		Msg: importingPkg,
	})

	fmt.Println(len(additionalErrors))
	fmt.Println(additionalErrors["importer"][0].Msg)
}
