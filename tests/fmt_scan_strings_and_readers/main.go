package main

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"strings"
)

func main() {
	var a, b int
	var s string
	var f float64
	var ok bool
	var u uint8

	n, err := fmt.Sscan("10 20", &a, &b)
	fmt.Println(n, err, a, b)
	n, err = fmt.Sscan("  7\n\n  hello  3.5 true", &a, &s, &f, &ok)
	fmt.Println(n, err, a, s, f, ok)
	n, err = fmt.Sscan("1", &a, &b)
	fmt.Println(n, err, errors.Is(err, io.EOF), err == io.ErrUnexpectedEOF)
	n, err = fmt.Sscan("", &a)
	fmt.Println(n, err, err == io.EOF)
	n, err = fmt.Sscan("x", &a)
	fmt.Println(n, err)
	n, err = fmt.Sscan("0x1f 010 -0b11 1_000", &a, &b, &a, &b)
	fmt.Println(n, err, a, b)
	n, err = fmt.Sscan("300", &u)
	fmt.Println(n, err, u)
	n, err = fmt.Sscan("-1e3 +Inf", &f, &f)
	fmt.Println(n, err, f)
	n, err = fmt.Sscan("TRUE f 1", &ok, &ok, &ok)
	fmt.Println(n, err, ok)
	n, err = fmt.Sscan("tru", &ok)
	fmt.Println(n, err)

	n, err = fmt.Sscanln("4 5\n6", &a, &b)
	fmt.Println(n, err, a, b)
	n, err = fmt.Sscanln("4\n5", &a, &b)
	fmt.Println(n, err, a)
	n, err = fmt.Sscanln("4 5 6", &a, &b)
	fmt.Println(n, err, a, b)

	var name string
	var age int
	n, err = fmt.Sscanf("name=bob age=42", "name=%s age=%d", &name, &age)
	fmt.Println(n, err, name, age)
	n, err = fmt.Sscanf("7-8", "%d-%d", &a, &b)
	fmt.Println(n, err, a, b)
	n, err = fmt.Sscanf("(3, 4)", "(%d, %d)", &a, &b)
	fmt.Println(n, err, a, b)
	n, err = fmt.Sscanf("x:5", "y:%d", &a)
	fmt.Println(n, err)
	n, err = fmt.Sscanf("12 ab", "%d %s", &a, &s)
	fmt.Println(n, err, a, s)
	n, err = fmt.Sscanf("3.25 ff", "%f %x", &f, &a)
	fmt.Println(n, err, f, a)
	var r rune
	n, err = fmt.Sscanf("Z9", "%c%d", &r, &a)
	fmt.Println(n, err, string(r), a)
	n, err = fmt.Sscanf("5", "%d %d", &a, &b)
	fmt.Println(n, err)
	n, err = fmt.Sscanf("\"hi there\" x", "%q %s", &s, &name)
	fmt.Println(n, err, s, name)
	n, err = fmt.Sscanf("1 2", "%v %v", &a, &f)
	fmt.Println(n, err, a, f)

	rd := bufio.NewReader(strings.NewReader("3\n1 2 3\nend\n"))
	var cnt int
	fmt.Fscan(rd, &cnt)
	total := 0
	for i := 0; i < cnt; i++ {
		var x int
		fmt.Fscan(rd, &x)
		total += x
	}
	var word string
	n, err = fmt.Fscan(rd, &word)
	fmt.Println(cnt, total, word, n, err)
	n, err = fmt.Fscan(rd, &word)
	fmt.Println(n, err)
	n, err = fmt.Fscanln(strings.NewReader("9 8\n"), &a, &b)
	fmt.Println(n, err, a, b)
	n, err = fmt.Fscanf(strings.NewReader("k=v"), "k=%s", &s)
	fmt.Println(n, err, s)
}
