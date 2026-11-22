package main

import "fmt"

type stringerText string

func (stringerText) String() string { return "string-method" + string([]byte{0xfb}) }

type errorText string

func (errorText) Error() string { return "error-method" + string([]byte{0xfc}) }

func main() {
	done := make(chan struct{})
	go func() { close(done) }()
	<-done

	invalid := string([]byte{0xfa})

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
