package main

import "fmt"

type version struct {
	major string
	minor string
}

func parse(x string) version {
	if x == "" {
		return version{}
	}
	return version{major: x, minor: "0"}
}

func valid(x string) bool {
	return parse(x) != version{}
}

func main() {
	fmt.Println(valid("1"))
	fmt.Println(valid(""))
}
