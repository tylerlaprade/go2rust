package main

import "fmt"

type Missing struct{ Name string }

func (m Missing) Error() string { return "missing " + m.Name }

type NotFound struct{ Code int }

func (n *NotFound) Error() string { return fmt.Sprintf("not found %d", n.Code) }

// log and logAll take their arguments as interface values, the way a logging
// helper does. An error of a type the program defines has to keep its text
// there, and it has to survive being copied.
func find() error { return Missing{"key"} }

func lookup() error { return &NotFound{Code: 404} }

func log(v interface{}) {
	fmt.Println(v)
	fmt.Printf("%v|%s\n", v, v)
}

func logAll(args ...interface{}) {
	for _, arg := range args {
		fmt.Println(arg)
	}
	fmt.Println(args...)
}

func keep(v interface{}) interface{} {
	w := v
	return w
}

func main() {
	err := find()
	log(err)
	other := lookup()
	log(other)
	logAll("failed:", err, other)
	fmt.Println(keep(err), keep(other))
	log(Missing{"direct"})
	// The interface value holds the error under its own type.
	var boxed interface{} = err
	if m, ok := boxed.(Missing); ok {
		fmt.Println("missing:", m.Name)
	}
	_, isString := boxed.(string)
	_, isNotFound := boxed.(*NotFound)
	fmt.Println(isString, isNotFound)
	var boxedPointer interface{} = other
	if n, ok := boxedPointer.(*NotFound); ok {
		fmt.Println("not found:", n.Code)
	}
}
