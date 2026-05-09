package main

import (
	"fmt"
	"go/ast"
	"go/types"
)

func version(info *types.Info, file *ast.File) string {
	if v := info.FileVersions[file]; v != "" {
		return v
	}
	return ""
}

func main() {
	file := &ast.File{}
	info := &types.Info{
		FileVersions: map[*ast.File]string{
			file: "go1.22",
		},
	}
	_ = &types.Info{
		Instances: map[*ast.Ident]types.Instance{},
		Implicits: map[ast.Node]types.Object{},
		Scopes:    map[ast.Node]*types.Scope{},
	}
	_ = &ast.ChanType{Dir: ast.SEND}
	fmt.Println(version(info, file))
}
