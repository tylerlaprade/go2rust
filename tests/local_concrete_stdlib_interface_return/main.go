package main

import (
	"fmt"
	"go/types"
)

type localType struct{}

func (localType) Underlying() types.Type {
	return localType{}
}

func (localType) String() string {
	return "local"
}

func makeType() types.Type {
	return localType{}
}

func main() {
	typesList := []types.Type{localType{}, makeType()}
	fmt.Println(len(typesList))
}
