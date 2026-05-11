package main

import "fmt"

func main() {
	overlay := make(map[string][]byte)
	overlay["file.go"] = []byte("go")

	var src []byte
	for filename, contents := range overlay {
		if filename == "file.go" {
			src = contents
			break
		}
	}

	fmt.Println(len(src))
	fmt.Println(src[0])
}
