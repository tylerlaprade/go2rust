package helper

import "fmt"

type Pkg struct {
	Name string
}

func Use(p *Pkg) {
	fmt.Println(p.Name)
}
