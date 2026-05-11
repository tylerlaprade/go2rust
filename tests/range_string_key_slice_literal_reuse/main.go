package main

import "fmt"

func main() {
	overlay := map[string][]byte{"file.go": nil}
	filename := "file.go"
	var goFiles []string
	var compiledGoFiles []string
	for path := range overlay {
		if path == filename {
			goFiles = []string{path}
			compiledGoFiles = []string{path}
		}
	}
	fmt.Println(goFiles[0], compiledGoFiles[0])
}
