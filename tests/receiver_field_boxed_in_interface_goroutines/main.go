// In a program with goroutines: a field of a defined type, read through the method receiver and put into
// an interface: as a format argument only known at run time, as a returned
// any, and as an error argument. The interface holds the field's value.
package main

import (
	"fmt"
	"sync"
)

type State int

func (s State) String() string { return [...]string{"idle", "busy"}[s] }

type Plain int

type Event string

type M struct {
	state State
	plain Plain
	name  string
}

func (m *M) fire(e Event) error {
	return fmt.Errorf("no %q from %v", e, m.state)
}

func (m *M) describe() string {
	return fmt.Sprintf("%q %v %v %d %s", Event("go"), m.state, m.plain, m.plain, m.name)
}

func (m M) byValue() string {
	return fmt.Sprintf("%v %q", m.state, "x")
}

func (m *M) get() any { return m.state }

func main() {
	m := &M{state: 1, plain: 7, name: "x"}
	fmt.Println(m.fire("stop"))
	fmt.Println(m.describe())
	fmt.Println(m.byValue())
	a := m.get()
	m.state = 0
	fmt.Println(a, m.get())
	if s, ok := a.(State); ok {
		fmt.Println("held", int(s))
	}
	v := M{state: 0, plain: 3}
	fmt.Printf("%q %v %v\n", Event("z"), v.state, v.plain)
	var wg sync.WaitGroup
	out := make([]string, 2)
	for i := 0; i < 2; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			w := &M{state: State(i)}
			out[i] = fmt.Sprint(w.fire("go"))
		}(i)
		wg.Wait()
	}
	fmt.Println(out[0])
	fmt.Println(out[1])
}
