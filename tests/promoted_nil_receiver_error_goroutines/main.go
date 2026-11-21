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

func main() {
	go func() {}()

	var nilOuter *outerError
	var nilOuterError error = nilOuter
	fmt.Println(nilOuterError)

	outerWithNilInner := &outerError{}
	var nilInnerError error = outerWithNilInner
	fmt.Println(nilInnerError)
}
