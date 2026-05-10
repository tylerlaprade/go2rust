package main

import "fmt"

type ReportFunc func(string, ...any)

type importer struct {
	reportf ReportFunc
	name    string
}

func report(label string, values ...any) {
	fmt.Println(label, len(values))
}

func main() {
	p := importer{
		reportf: report,
		name:    "alpha",
	}
	p.reportf(p.name, 1, "x")
}
