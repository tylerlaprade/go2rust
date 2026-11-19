package main

import (
	"errors"
	"fmt"
)

// show and count take their arguments as interface values, the way a logging
// helper does. An error made by errors.New or fmt.Errorf has to survive that.
func show(args ...any) {
	for _, arg := range args {
		fmt.Println(arg)
	}
}

func count(args ...any) int {
	return len(args)
}

type failure = error

// kind tells values apart with a type switch, the way a handler does.
func kind(v any) string {
	switch x := v.(type) {
	case int:
		return fmt.Sprintf("int %d", x)
	case string:
		return "string " + x
	case error:
		return "error " + x.Error()
	default:
		return "other"
	}
}

func isError(v any) bool {
	switch v.(type) {
	case error:
		return true
	}
	return false
}

func main() {
	plain := errors.New("bad")
	wrapped := fmt.Errorf("wrapped: %w", plain)
	show("first", plain, wrapped, 3)
	fmt.Println(count(plain, wrapped))

	// Putting an error in an interface value does not turn it into a string,
	// and taking it back out gives the same error.
	var boxed any = plain
	_, isString := boxed.(string)
	fmt.Println("is a string:", isString)
	if err, ok := boxed.(error); ok {
		fmt.Println("asserted:", err, err == plain)
	}
	fmt.Println(boxed)
	fmt.Printf("%v|%s\n", boxed, boxed)

	var aliased failure = errors.New("aliased")
	var boxedAlias any = aliased
	_, aliasIsError := boxedAlias.(error)
	fmt.Println(boxedAlias, aliasIsError)

	items := []interface{}{plain, "text", wrapped}
	for _, item := range items {
		fmt.Println(item)
	}

	var number any = 5
	_, numberIsError := number.(error)
	fmt.Println(numberIsError)

	// A type switch finds the error as well, and its case variable is the
	// error.
	fmt.Println(kind(plain), "|", kind(wrapped), "|", kind(3), "|", kind("s"), "|", kind(2.5))
	fmt.Println(isError(plain), isError("bad"), isError(4))
}
