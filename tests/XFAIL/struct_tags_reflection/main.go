package main

import (
	"fmt"
	"reflect"
)

type User struct {
	ID       int    `json:"id" db:"user_id"`
	Name     string `json:"name,omitempty" db:"full_name"`
	Email    string `json:"email" db:"email_address" validate:"email"`
	IsActive bool   `json:"is_active" db:"active"`
	internal string // unexported field
}

func main() {
	u := User{ID: 1, Name: "Alice", Email: "alice@example.com"}
	t := reflect.TypeOf(u)

	for i := 0; i < t.NumField(); i++ {
		field := t.Field(i)
		fmt.Printf("%s: json=%q db=%q\n",
			field.Name,
			field.Tag.Get("json"),
			field.Tag.Get("db"))
	}
}
