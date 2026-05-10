package main

import "fmt"

type exporter struct {
	indent int
}

type writer struct {
	p *exporter
}

func (w *writer) doTrace() {
	w.p.indent++
	defer func() {
		w.p.indent--
	}()
}

func main() {
	w := &writer{p: &exporter{}}
	w.doTrace()
	fmt.Println(w.p.indent)
}
