package main

import (
	"strings"
	"testing"
)

// A pointer to a struct element of a slice, where a *T cell is wanted, is
// a cell holding a Clone of the element: the Clone shares the element's
// field cells (DEV-157).
func TestSliceElemStructPointerBecomesFieldSharingCell(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Employee struct{ Salary int }

type Registry struct {
	all []Employee
	top []*Employee
}

func raise(e *Employee) { e.Salary++ }

func (r *Registry) Index() {
	for i := range r.all {
		e := &r.all[i]
		r.top = append(r.top, e)
	}
	raise(&r.all[0])
}

func main() {
	r := &Registry{all: []Employee{{1}}}
	r.Index()
	fmt.Println(r.all[0].Salary)
}
`)
	if strings.Contains(rust, "unimplemented!(\"slice element pointer cannot pass to writable pointer parameter\")") {
		t.Fatalf("a struct element pointer should not panic:\n%s", rust)
	}
	for _, want := range []string{
		"match e.as_ref() { Some(__elem_ptr) => Rc::new(RefCell::new((*__elem_ptr.borrow()).clone())), None => Rc::new(RefCell::new(None)) }",
		"Rc::new(RefCell::new((*__elem_ptr.borrow()).clone())) }",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}
