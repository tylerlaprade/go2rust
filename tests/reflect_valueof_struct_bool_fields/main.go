package main

import (
	"fmt"
	"reflect"
	"strings"
)

type Flags struct {
	Alpha bool
	Beta  bool
}

func main() {
	flags := Flags{}
	names := map[string]func(bool){}

	rv := reflect.ValueOf(&flags).Elem()
	rt := rv.Type()
	for i := 0; i < rt.NumField(); i++ {
		field := rv.Field(i)
		names[strings.ToLower(rt.Field(i).Name)] = field.SetBool
	}

	names["alpha"](true)
	fmt.Println(flags.Alpha, flags.Beta)
}
