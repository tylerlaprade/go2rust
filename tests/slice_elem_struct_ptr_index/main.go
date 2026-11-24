package main

import (
	"fmt"
	"sort"
)

type Employee struct {
	Name   string
	Dept   string
	Salary int
}

// An index of pointers into a slice, kept in a struct field.
type Registry struct {
	all    []Employee
	byDept map[string][]*Employee
	top    *Employee
}

func (r *Registry) Index() {
	r.byDept = map[string][]*Employee{}
	for i := range r.all {
		e := &r.all[i]
		r.byDept[e.Dept] = append(r.byDept[e.Dept], e)
		if r.top == nil || e.Salary > r.top.Salary {
			r.top = e
		}
	}
}

func raise(e *Employee, pct int) {
	e.Salary += e.Salary * pct / 100
}

func main() {
	r := &Registry{all: []Employee{{"ann", "eng", 100}, {"bo", "ops", 80}, {"cy", "eng", 120}}}
	r.Index()
	for _, e := range r.byDept["eng"] {
		raise(e, 10)
	}
	raise(&r.all[1], 50)
	r.top.Name = "CY"
	for _, e := range r.all {
		fmt.Println(e.Name, e.Dept, e.Salary)
	}
	depts := []string{}
	for d, es := range r.byDept {
		depts = append(depts, fmt.Sprintf("%s:%d", d, len(es)))
	}
	sort.Strings(depts)
	fmt.Println(depts, r.top.Salary)

	// A *T parameter given &s[i] writes the element's fields.
	raise(&r.all[0], 100)
	fmt.Println(r.all[0].Salary, r.byDept["eng"][0].Salary)
}
