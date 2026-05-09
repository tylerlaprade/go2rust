package main

import (
	"fmt"
	"go/types"
)

func makeQualifier() types.Qualifier {
	return types.Qualifier(func(pkg *types.Package) string {
		return ""
	})
}

func useQualifier(qualifier types.Qualifier) string {
	return qualifier(nil)
}

func forwardQualifier(qualifier types.Qualifier) string {
	return useQualifier(qualifier)
}

func main() {
	fmt.Println("qualifier:" + forwardQualifier(makeQualifier()))
}
