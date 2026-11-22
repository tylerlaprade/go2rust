package main

import "fmt"

type stringerText string

func (stringerText) String() string { return "string-method" }

type errorText string

func (errorText) Error() string { return "error-method" }

func main() {
	invalid := string([]byte{0xfa})

	fmt.Println([]byte(fmt.Sprint("x", invalid)))
	fmt.Println([]byte(fmt.Sprintln(invalid)))
	fmt.Println([]byte(fmt.Sprint(1, 2, invalid)))
	fmt.Println([]byte(fmt.Sprintf("<%s>", invalid)))
	fmt.Println([]byte(fmt.Sprintf("%s%%:%s", invalid, "z")))
	fmt.Println([]byte(fmt.Sprint(stringerText(invalid))))
	fmt.Println([]byte(fmt.Sprintln(errorText(invalid))))
}
