package main

import (
	"fmt"
	"strings"

	"example.com/stringconst/dep"
)

func inside(path, dir string) bool {
	return strings.HasPrefix(path, dir+string(dep.Separator))
}

func main() {
	fmt.Println(inside("root/file.go", "root"))
}
