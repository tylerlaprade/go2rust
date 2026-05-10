package main

import "fmt"

func main() {
	roots := map[string]string{
		"/src": "module/path",
	}

	for _, rpath := range roots {
		if rpath != "" {
			fmt.Println(rpath)
		}
	}
}
