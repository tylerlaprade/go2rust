package main

import (
	"fmt"

	"example.com/namedfield/dep"
)

func main() {
	cfg := &dep.Config{Mode: dep.NeedName | dep.NeedFiles | dep.NeedTypes}
	fmt.Println(dep.Enabled(cfg, dep.NeedFiles))
}
