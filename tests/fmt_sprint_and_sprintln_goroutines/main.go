// The same program with a goroutine, so every value is behind a lock.
package main

import (
	"fmt"
	"os"
	"strings"
)

type Point struct{ X, Y int }

type Named string

type upper struct{ data []byte }

func (u *upper) Write(p []byte) (int, error) {
	for _, b := range p {
		if b >= 'a' && b <= 'z' {
			b -= 'a' - 'A'
		}
		u.data = append(u.data, b)
	}
	return len(p), nil
}

func fprint() {
	fmt.Fprint(os.Stdout, "a", 1, 2, "b\n")
	var sb strings.Builder
	fmt.Fprint(&sb, 1, 2, "x", 3)
	fmt.Println(sb.String())
	u := &upper{}
	fmt.Fprint(u, "hi ", 7)
	fmt.Println(string(u.data))
}

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started

	s := fmt.Sprint("a", 1, true)
	fmt.Println(s, len(s))
	fmt.Println(fmt.Sprint(1, 2, "x", 3, 4.5, Point{1, 2}, nil))
	var v interface{} = 42
	fmt.Printf("%q\n", fmt.Sprintln("a", 1, true, v, Point{3, 4}))
	fmt.Printf("%q %q\n", fmt.Sprint(), fmt.Sprintln())
	parts := []interface{}{1, "b", 2, 3}
	fmt.Printf("%q %q\n", fmt.Sprint(parts...), fmt.Sprintln(parts...))
	msg := fmt.Sprint("total: ", 3+4) + "!"
	fmt.Println(strings.ToUpper(msg), fmt.Sprint(Named("n"), 5))
	words := []string{}
	for i := 0; i < 3; i++ {
		words = append(words, fmt.Sprint(i))
	}
	fmt.Println(strings.Join(words, "-"))
	err := fmt.Errorf("boom")
	fmt.Println(fmt.Sprint(err), fmt.Sprintln(err) == "boom\n")
	fprint()
}
