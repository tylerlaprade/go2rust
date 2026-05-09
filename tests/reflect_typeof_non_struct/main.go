package main

import (
	"fmt"
	"reflect"
)

type namedInt int64

func main() {
	var n namedInt = 7
	var s string = "value"

	fmt.Println(reflect.TypeOf(n).String())
	fmt.Println(reflect.TypeOf(s).String())
}
