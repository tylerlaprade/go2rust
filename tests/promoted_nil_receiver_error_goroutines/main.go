package main

import "fmt"

type innerError struct{}

func (err *innerError) Error() string {
	if err == nil {
		return "inner-nil"
	}
	return "inner"
}

type outerError struct {
	*innerError
}

type E struct{}
type e struct{}

func (*E) Error() string { return "E" }
func (*e) Error() string { return "e" }

func main() {
	go func() {}()

	var nilOuter *outerError
	var nilOuterError error = nilOuter
	fmt.Println(nilOuterError)

	outerWithNilInner := &outerError{}
	var nilInnerError error = outerWithNilInner
	fmt.Println(nilInnerError)
	fmt.Println(outerWithNilInner.Error())
	var upper *E
	var lower *e
	fmt.Println(upper.Error(), lower.Error())
}
