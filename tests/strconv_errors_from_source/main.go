// strconv is translated from its Go source, so its errors are *NumError
// values: errors.As finds them and errors.Is follows their Unwrap to
// ErrSyntax and ErrRange.
package main

import (
	"errors"
	"fmt"
	"strconv"
)

func main() {
	_, err := strconv.Atoi("12x")
	var ne *strconv.NumError
	if errors.As(err, &ne) {
		fmt.Println("func", ne.Func, "num", ne.Num, "err", ne.Err, errors.Is(ne.Err, strconv.ErrSyntax))
	}
	fmt.Println(errors.Is(err, strconv.ErrSyntax), errors.Unwrap(err) == strconv.ErrSyntax)
	_, err = strconv.ParseInt("99999999999999999999", 10, 64)
	fmt.Println(errors.Is(err, strconv.ErrRange), err)
	wrapped := fmt.Errorf("config: %w", err)
	fmt.Println(errors.Is(wrapped, strconv.ErrRange), errors.As(wrapped, &ne), ne.Num)
}
