package main

import "fmt"

type Package struct {
	Dir      string
	GoFiles  []string
	CgoFiles []string
}

func absJoin(dir string, fileses [][]string) []string {
	return []string{dir, fileses[0][0], fileses[1][0]}
}

func main() {
	p := &Package{
		Dir:      "root",
		GoFiles:  []string{"a.go"},
		CgoFiles: []string{"c.go"},
	}
	files := absJoin(p.Dir, [][]string{p.GoFiles, p.CgoFiles})
	p.Dir = "changed"
	p.GoFiles[0] = "b.go"
	fmt.Println(files[0])
	fmt.Println(files[1])
	fmt.Println(files[2])
}
