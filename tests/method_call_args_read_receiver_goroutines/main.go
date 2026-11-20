// The same program with a goroutine, so every value is behind a lock.
package main

import "fmt"

type Counter struct{ n int }

func (c *Counter) Add(x int)     { c.n += x }
func (c Counter) Plus(x int) int { return c.n + x }

type Named struct {
	Counter
	name string
}

type Logger struct {
	prefix string
	lines  []string
}

func (l *Logger) Log(format string, args ...any) {
	l.lines = append(l.lines, l.prefix+fmt.Sprintf(format, args...))
}

type Service struct {
	*Logger
	Name string
	Port int
}

type Stack struct{ items []int }

func (s *Stack) Push(v int) { s.items = append(s.items, v) }
func (s *Stack) Len() int   { return len(s.items) }

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	c := Counter{1}
	c.Add(c.n)
	c.Add(c.Plus(c.n))
	fmt.Println(c.n, c.Plus(c.n))

	w := Named{Counter{2}, "w"}
	w.Add(w.n)
	w.Counter.Add(w.n + 1)
	fmt.Println(w.n, w.name)

	s := Service{&Logger{prefix: "[svc] "}, "api", 8080}
	s.Log("starting %s", s.Name)
	s.Log("port %d", s.Port)
	s.Logger.Log("%d lines so far", len(s.lines))
	for _, line := range s.lines {
		fmt.Println(line)
	}

	var st Stack
	for i := 0; i < 3; i++ {
		st.Push(st.Len() * 10)
	}
	fmt.Println(st.items)
}
