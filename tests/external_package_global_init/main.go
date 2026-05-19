package main

import (
	"fmt"

	"example.com/externalinit/dep"
)

func main() {
	fmt.Println(dep.IsEnabled())
}
