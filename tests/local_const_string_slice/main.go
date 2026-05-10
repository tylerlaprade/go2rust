package main

import "fmt"

func buildArgs() []string {
	const format = "{{.Path}}\n"
	return []string{"-m", "-f", format}
}

func main() {
	args := buildArgs()
	fmt.Println(len(args), args[2] == "{{.Path}}\n")
}
