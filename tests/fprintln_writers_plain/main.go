// fmt.Fprintln writes to a type with its own Write method, in a program
// that needs no shared-state wrappers. Fprintln(w, args...) and
// Fprint(w, args...) print each element of args, and a field of the
// method's receiver goes into a []any as its value.
package main

import "fmt"

type Lines struct {
	got   []string
	bytes int
}

func (l *Lines) Write(p []byte) (int, error) {
	s := string(p)
	if len(s) > 0 && s[len(s)-1] == '\n' {
		s = s[:len(s)-1]
	}
	l.got = append(l.got, s)
	l.bytes += len(p)
	return len(p), nil
}

type Logger struct {
	out    *Lines
	prefix string
}

func (lg Logger) Log(args ...any) {
	fmt.Fprintln(lg.out, append([]any{lg.prefix}, args...)...)
}

func main() {
	l := &Lines{}
	fmt.Fprintln(l, "a", 1, true)
	n, err := fmt.Fprintln(l, "b", 2.5)
	fmt.Fprintln(l)
	fmt.Println(n, err, len(l.got), l.bytes)
	for i, s := range l.got {
		fmt.Printf("%d %q\n", i, s)
	}
	lg := Logger{out: l, prefix: "[app]"}
	lg.Log("started", 3)
	fmt.Println(l.got[len(l.got)-1])
	vals := []any{"x", 1, 2, "y", "z"}
	fmt.Fprint(l, vals...)
	fmt.Println(l.got[len(l.got)-1])
	fmt.Println(vals...)
}
