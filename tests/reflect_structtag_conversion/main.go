package main

import (
	"fmt"
	"reflect"
)

func main() {
	tag := `json:"name,omitempty" db:"full_name"`
	fmt.Println(reflect.StructTag(tag).Get("json"))
	fmt.Println(reflect.StructTag(tag).Get("db"))
}
