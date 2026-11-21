package main

import "fmt"

var calls int

func pair() (int, int) {
	calls++
	return 2, 3
}

func sum(values ...int) int {
	total := 0
	for _, value := range values {
		total += value
	}
	return total
}

func collect[T any](values ...T) T {
	return values[1]
}

func withPrefix(prefix string, values ...int) int {
	return len(prefix) + sum(values...)
}

func mixed() (string, int, int) {
	return "abc", 4, 5
}

func noExtras(first int, second int, rest ...int) bool {
	return first+second == 5 && rest == nil
}

func words() (string, string) {
	return "go", "rust"
}

func join(values ...string) string {
	return values[0] + values[1]
}

func triple[T any](a, b, c T) (T, T, T) {
	return a, b, c
}

func report(first int, rest ...int) {
	fmt.Println(first, rest[0], rest[1])
}

func reportTag(tag any, values ...int) {
	switch tag := tag.(type) {
	case int:
		fmt.Println(tag, values[0])
	case string:
		fmt.Println(tag, values[0])
	}
}

func taggedNumber() (int, int)    { return 8, 9 }
func taggedString() (string, int) { return "go", 10 }

type tupleError struct{ message string }

func (e tupleError) Error() string { return e.message }

func errorFields() (tupleError, int, int) { return tupleError{"bad"}, 11, 12 }

func reportError(err error, values ...int) {
	fmt.Println(err.Error(), values[0], values[1])
}

func main() {
	fmt.Println(sum(pair()), calls)
	fmt.Println(collect(pair()))
	fmt.Println(withPrefix(mixed()))
	fmt.Println(noExtras(pair()))
	fmt.Println(join(words()))
	report(triple(4, 2, 3))
	reportTag(taggedNumber())
	reportTag(taggedString())
	reportError(errorFields())
}
