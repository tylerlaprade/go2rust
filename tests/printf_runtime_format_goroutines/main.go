package main

import (
	"errors"
	"fmt"
	"os"
	"strings"
)

type Point struct{ X, Y int }

type Status struct{ Code int }

func (s Status) String() string { return fmt.Sprintf("status %d", s.Code) }

// logf is the wrapper most programs have: the format arrives as a value, so
// it is only known when the program runs.
func logf(format string, args ...interface{}) {
	fmt.Printf(format+"\n", args...)
}

func sprintf(format string, args ...interface{}) string {
	return fmt.Sprintf(format, args...)
}

func errorf(format string, args ...interface{}) error {
	return fmt.Errorf(format, args...)
}

func pick(named bool) string {
	if named {
		return "name=%s ok=%v\n"
	}
	return "%v %v\n"
}

func main() {
	// A goroutine makes every shared value a locked one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	// The format is a parameter, the result of a call, or a variable.
	logf("count %d of %d", 1, 2)
	fmt.Printf(pick(true), "x", true)
	fmt.Println(sprintf("%d-%s", 7, "z"))
	f := "value: %d\n"
	fmt.Printf(f, 42)

	// Verbs, flags, width and precision.
	logf("%5d|%-5d|%05d|%+d|%x|%X|%o|%b|%c|%q|%U", 42, 42, 42, 42, 255, 255, 8, 5, 65, 65, 65)
	logf("%s|%10s|%-10s|%.2s|%q|%x", "go", "go", "go", "golang", "go", "go")
	logf("%v|%f|%.2f|%8.3f|%e|%g", 2.5, 2.5, 2.5, 2.5, 123456.789, 1e21)
	logf("%t|%v|%5t", true, false, true)
	logf("%v %v %v %v", 1, "two", 3.5, true)
	logf("%T %T %T %T", 1, "two", 3.5, true)
	logf("100%% of %d", 3)
	logf("%*d|%-*d|%.*f", 5, 42, 5, 42, 2, 3.14159)

	// Values that print the way Println prints them.
	logf("point %v, pointer %v", Point{1, 2}, &Point{3, 4})
	logf("slice %v, map %v", []int{1, 2}, map[string]int{"a": 1})
	logf("error: %v / %s", errors.New("boom"), errors.New("bang"))
	failed := errors.New("failed")
	logf("%v|%s|%10v|", failed, failed, failed)
	logf("%v|%s|%-12v|%v", Status{1}, Status{2}, Status{3}, &Status{4})

	// A []byte prints as text where a string would.
	logf("bytes %s, hex %x, list %v", []byte("Hi"), []byte("Hi"), []byte("Hi"))
	logf("%*d|%#v|%#v", -4, 7, "quoted", uint8(255))

	// What fmt writes when the format and the arguments do not fit.
	fmt.Println(sprintf("%d", "text"))
	fmt.Println(sprintf("%s", 5))
	fmt.Println(sprintf("%d %d", 1))
	fmt.Println(sprintf("%d", 1, 2))
	fmt.Println(sprintf("%z", 1))
	fmt.Println(sprintf("trailing %"))
	fmt.Println(sprintf("no verbs", "left", 2))
	fmt.Println(sprintf("plain"))

	// The same through Sprintf, Errorf and Fprintf.
	err := errorf("code %d: %w", 7, errors.New("inner"))
	fmt.Println(err)
	fmt.Fprintf(os.Stdout, f, 43)
	pair := "%s=%d"
	var b strings.Builder
	fmt.Fprintf(&b, pair, "k", 1)
	fmt.Println(b.String())

	// A constant format is known when the program is translated.
	const layout = "%s has %d items\n"
	fmt.Printf(layout, "cart", 3)
}
