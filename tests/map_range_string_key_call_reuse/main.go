package main

import "fmt"

func keep(s string) bool {
	return len(s) > 0
}

func main() {
	src := map[string]string{
		"pkg": "crate",
		"imp": "dep",
	}
	paths := []string{}
	seen := make(map[string]string)
	for pkgPath := range src {
		if keep(pkgPath) {
			paths = append(paths, pkgPath)
		}
		name, ok := src[pkgPath]
		if ok {
			seen[name] = pkgPath
		}
	}
	fmt.Println(len(paths))
	fmt.Println(seen["crate"] != "")
	fmt.Println(seen["dep"] != "")
}
