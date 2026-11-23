package main

import (
	"errors"
	"fmt"
)

type failer interface {
	Error() string
}

type customError struct{ text string }

func (e customError) Error() string { return e.text }

func describe(f failer) string { return f.Error() }

func switchNamed(v any) string {
	switch x := v.(type) {
	case failer:
		return x.Error()
	default:
		return "default"
	}
}

func switchAnonymous(v any) string {
	switch x := v.(type) {
	case interface{ Error() string }:
		return x.Error()
	default:
		return "default"
	}
}

func main() {
	go func() {}()

	var direct failer = errors.New("x")
	fmt.Println("direct", direct.Error())
	fmt.Println("call", describe(errors.New("z")))
	var custom failer = customError{text: "custom"}
	fmt.Println("custom", custom.Error())

	var anonymous interface{ Error() string } = errors.New("y")
	fmt.Println("anonymous", anonymous.Error())
	var embedded interface{ error } = errors.New("embedded")
	fmt.Println("embedded", embedded.Error())

	var value any = errors.New("bad")
	asserted, ok := value.(failer)
	fmt.Println("assert", ok, asserted.Error())
	single := value.(failer)
	fmt.Println("single", single.Error())
	fmt.Println("switch named", switchNamed(value))
	fmt.Println("switch anonymous", switchAnonymous(value))

	wrapped := fmt.Errorf("wrap: %w", errors.New("inner"))
	fmt.Println("wrapped", describe(wrapped))
}
