package main

import "fmt"

func (err *splitError) Error() string {
	return fmt.Sprintf("split:%d", err.code)
}
