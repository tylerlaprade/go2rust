package main

import "fmt"

func qualify(dir, file string) string {
	return dir + "/" + file
}

func main() {
	files := []string{"a.go", "b.go"}
	var out []string
	for _, file := range files {
		if file == "a.go" {
			file = qualify("src", file)
		}
		out = append(out, file)
	}
	for _, file := range out {
		fmt.Println(file)
	}
}
