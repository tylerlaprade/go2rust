package main

import "fmt"

type Package struct {
	Errors []string
}

func main() {
	pkgs := map[string]*Package{
		"pkg": {},
	}
	additional := map[string][]string{
		"pkg": {"missing file"},
	}

	for id, errs := range additional {
		if p, ok := pkgs[id]; ok {
			p.Errors = append(p.Errors, errs...)
		}
	}

	fmt.Println(pkgs["pkg"].Errors[0])
}
