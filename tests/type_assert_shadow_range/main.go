package main

import "fmt"

type ImportSpec struct {
	Path string
}

func main() {
	imports := []*ImportSpec{{Path: "fmt"}, {Path: "os"}}
	for _, s := range imports {
		s := s
		fmt.Println(s.Path)
	}
}
