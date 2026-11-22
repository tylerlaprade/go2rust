package main

import (
	"fmt"
	"os"
)

type stringerText string

func (stringerText) String() string { return "string-method" + string([]byte{0xfb}) }

type errorText string

func (errorText) Error() string { return "error-method" + string([]byte{0xfc}) }

type valueStringer struct{}

func (valueStringer) String() string { return "value-stringer" + string([]byte{0xfd}) }

type valueError struct{}

func (valueError) Error() string { return "value-error" + string([]byte{0xfe}) }

type boxedStringer struct{}

func (boxedStringer) String() string { return "boxed-stringer" + string([]byte{0xf8}) }

type boxedError struct{}

func (boxedError) Error() string { return "boxed-error" + string([]byte{0xf9}) }

func main() {
	done := make(chan struct{})
	go func() { close(done) }()
	<-done

	invalid := string([]byte{0xfa})
	n, err := fmt.Print("print<", invalid, ">\n")
	fmt.Println(n, err == nil)
	n, err = fmt.Println("println<", invalid, ">")
	fmt.Println(n, err == nil)
	n, err = fmt.Printf("printf<%s>\n", invalid)
	fmt.Println(n, err == nil)
	n, err = fmt.Fprint(os.Stdout, "fprint<", invalid, ">\n")
	fmt.Println(n, err == nil)
	n, err = fmt.Fprintln(os.Stdout, "fprintln<", invalid, ">")
	fmt.Println(n, err == nil)
	n, err = fmt.Fprintf(os.Stderr, "fprintf<%s>\n", invalid)
	fmt.Println(n, err == nil)
	n, err = fmt.Print(valueStringer{})
	fmt.Print("\n")
	fmt.Println(n, err == nil)
	n, err = fmt.Println(valueError{})
	fmt.Println(n, err == nil)
	n, err = fmt.Fprint(os.Stdout, valueStringer{})
	fmt.Fprint(os.Stdout, "\n")
	fmt.Println(n, err == nil)
	n, err = fmt.Fprintln(os.Stderr, valueError{})
	fmt.Println(n, err == nil)
	n, err = fmt.Printf("printf-method<%s>\n", valueStringer{})
	fmt.Println(n, err == nil)
	n, err = fmt.Fprintf(os.Stderr, "fprintf-method<%s>\n", valueError{})
	fmt.Println(n, err == nil)
	fmt.Println([]byte(fmt.Sprintf("<%s>", valueStringer{})))
	fmt.Println([]byte(fmt.Sprintf("<%s>", valueError{})))
	var boxedText any = boxedStringer{}
	var boxedProblem any = boxedError{}
	boxedValues := []any{boxedStringer{}, boxedError{}}
	n, err = fmt.Print(boxedText)
	fmt.Print("\n")
	fmt.Println(n, err == nil)
	n, err = fmt.Println(boxedProblem)
	fmt.Println(n, err == nil)
	n, err = fmt.Fprint(os.Stdout, boxedText)
	fmt.Fprint(os.Stdout, "\n")
	fmt.Println(n, err == nil)
	n, err = fmt.Fprintln(os.Stderr, boxedProblem)
	fmt.Println(n, err == nil)
	fmt.Println([]byte(fmt.Sprint(boxedText)))
	fmt.Println([]byte(fmt.Sprintln(boxedProblem)))
	fmt.Println([]byte(fmt.Sprint(boxedValues...)))
	fmt.Println([]byte(fmt.Sprintln(boxedValues...)))
	n, err = fmt.Print(boxedValues...)
	fmt.Print("\n")
	fmt.Println(n, err == nil)
	n, err = fmt.Println(boxedValues...)
	fmt.Println(n, err == nil)
	n, err = fmt.Fprint(os.Stdout, boxedValues...)
	fmt.Fprint(os.Stdout, "\n")
	fmt.Println(n, err == nil)
	n, err = fmt.Fprintln(os.Stderr, boxedValues...)
	fmt.Println(n, err == nil)

	fmt.Println([]byte(fmt.Sprint("x", invalid)))
	fmt.Println([]byte(fmt.Sprintln(invalid)))
	fmt.Println([]byte(fmt.Sprint(1, 2, invalid)))
	fmt.Println([]byte(fmt.Sprintf("<%s>", invalid)))
	fmt.Println([]byte(fmt.Sprintf("%s%%:%s", invalid, "z")))
	invalidMixed := string([]byte{0xfa, 'A'})
	fmt.Println([]byte(fmt.Sprintf("[%4s][%-4s][%.1s][%4.1s][%04s][%04.1s][%-04s]", invalidMixed, invalidMixed, invalidMixed, invalidMixed, invalidMixed, invalidMixed, invalidMixed)))
	quotedFormat := "%q|%.1q|%#q|%x"
	fmt.Println([]byte(fmt.Sprintf(quotedFormat, invalidMixed, invalidMixed, invalidMixed, invalidMixed)))
	n, err = fmt.Fprintf(os.Stdout, quotedFormat+"\n", invalidMixed, invalidMixed, invalidMixed, invalidMixed)
	fmt.Println(n, err == nil)
	textFormat := "%s|%v|%4s|%.1s|%04s"
	fmt.Println([]byte(fmt.Sprintf(textFormat, invalidMixed, invalidMixed, invalidMixed, invalidMixed, invalidMixed)))
	n, err = fmt.Fprintf(os.Stdout, textFormat+"\n", invalidMixed, invalidMixed, invalidMixed, invalidMixed, invalidMixed)
	fmt.Println(n, err == nil)
	n, err = fmt.Printf(textFormat+"\n", invalidMixed, invalidMixed, invalidMixed, invalidMixed, invalidMixed)
	fmt.Println(n, err == nil)
	methodFormat := "%s|%v"
	fmt.Println([]byte(fmt.Sprintf(methodFormat, stringerText(""), errorText(""))))
	collisionFormat := "%s"
	fmt.Println([]byte(fmt.Sprintf(collisionFormat, "\ufdd0\ufdd1fa")))
	fmt.Println([]byte(fmt.Sprint(stringerText(invalid))))
	fmt.Println([]byte(fmt.Sprintln(errorText(invalid))))
	fmt.Println([]byte(fmt.Sprintf("<%s>", stringerText(invalid))))
	fmt.Println([]byte(fmt.Sprintf("<%s>", errorText(invalid))))
}
