package main

import (
	"fmt"
	"os"
)

type stringerText string

func (stringerText) String() string { return "string-method" + string([]byte{0xfb}) }

type errorText string

func (errorText) Error() string { return "error-method" + string([]byte{0xfc}) }

func main() {
	invalid := string([]byte{0xfa})
	n, err := fmt.Print("print<", invalid, ">\n")
	fmt.Println(n, err == nil)
	nln, errln := fmt.Println("println<", invalid, ">")
	fmt.Println(nln, errln == nil)
	nprintf, errprintf := fmt.Printf("printf<%s>\n", invalid)
	fmt.Println(nprintf, errprintf == nil)
	nfprint, errfprint := fmt.Fprint(os.Stdout, "fprint<", invalid, ">\n")
	fmt.Println(nfprint, errfprint == nil)
	nfprintln, errfprintln := fmt.Fprintln(os.Stdout, "fprintln<", invalid, ">")
	fmt.Println(nfprintln, errfprintln == nil)
	nfprintf, errfprintf := fmt.Fprintf(os.Stderr, "fprintf<%s>\n", invalid)
	fmt.Println(nfprintf, errfprintf == nil)

	fmt.Println([]byte(fmt.Sprint("x", invalid)))
	fmt.Println([]byte(fmt.Sprintln(invalid)))
	fmt.Println([]byte(fmt.Sprint(1, 2, invalid)))
	fmt.Println([]byte(fmt.Sprintf("<%s>", invalid)))
	fmt.Println([]byte(fmt.Sprintf("%s%%:%s", invalid, "z")))
	fmt.Println([]byte(fmt.Sprint(stringerText(invalid))))
	fmt.Println([]byte(fmt.Sprintln(errorText(invalid))))
	fmt.Println([]byte(fmt.Sprintf("<%s>", stringerText(invalid))))
	fmt.Println([]byte(fmt.Sprintf("<%s>", errorText(invalid))))
}
