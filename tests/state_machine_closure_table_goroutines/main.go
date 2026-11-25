// In a program with goroutines: a state machine whose callbacks, stored in a map field, capture the
// machine itself, and whose error formats a defined-type receiver field at
// run time.
package main

import (
	"fmt"
	"sort"
	"strings"
	"sync"
)

type State int

const (
	Idle State = iota
	Running
	Paused
	Done
)

var stateNames = map[State]string{Idle: "idle", Running: "running", Paused: "paused", Done: "done"}

func (s State) String() string { return stateNames[s] }

type Event string

type Machine struct {
	state       State
	transitions map[State]map[Event]State
	history     []State
	onEnter     map[State]func(from State)
	log         *strings.Builder
}

func NewMachine() *Machine {
	m := &Machine{
		state: Idle,
		transitions: map[State]map[Event]State{
			Idle:    {"start": Running},
			Running: {"pause": Paused, "finish": Done},
			Paused:  {"resume": Running, "finish": Done},
		},
		onEnter: map[State]func(State){},
		log:     &strings.Builder{},
	}
	m.onEnter[Done] = func(from State) { fmt.Fprintf(m.log, "done after %v;", from) }
	m.onEnter[Paused] = func(from State) { m.log.WriteString("paused;") }
	return m
}

func (m *Machine) Fire(e Event) error {
	next, ok := m.transitions[m.state][e]
	if !ok {
		return fmt.Errorf("no transition %q from %v", e, m.state)
	}
	prev := m.state
	m.history = append(m.history, prev)
	m.state = next
	if f, ok := m.onEnter[next]; ok {
		f(prev)
	}
	return nil
}

type Stats struct {
	Counts map[State]int
	Last   *State
}

func (m *Machine) Stats() Stats {
	s := Stats{Counts: map[State]int{}}
	for _, h := range m.history {
		s.Counts[h]++
	}
	if len(m.history) > 0 {
		last := m.history[len(m.history)-1]
		s.Last = &last
	}
	return s
}

func main() {
	m := NewMachine()
	for _, e := range []Event{"start", "pause", "pause", "resume", "finish", "start"} {
		if err := m.Fire(e); err != nil {
			fmt.Println("error:", err)
		}
	}
	fmt.Println(m.state, m.history, m.log.String())
	st := m.Stats()
	keys := make([]State, 0, len(st.Counts))
	for k := range st.Counts {
		keys = append(keys, k)
	}
	sort.Slice(keys, func(i, j int) bool { return keys[i] < keys[j] })
	for _, k := range keys {
		fmt.Printf("%-8v %d\n", k, st.Counts[k])
	}
	fmt.Println(*st.Last, st.Last != nil)
	fire := m.Fire
	fmt.Println(fire("start"))
	var wg sync.WaitGroup
	errs := make([]error, 2)
	for i, e := range []Event{"start", "bogus"} {
		wg.Add(1)
		go func(i int, e Event) {
			defer wg.Done()
			other := NewMachine()
			errs[i] = other.Fire(e)
		}(i, e)
		wg.Wait()
	}
	fmt.Println(errs[0], errs[1])
	fmt.Printf("%d %v %s|%5.1f|%-6s|%x\n", Running, Running, Paused, 3.14159, "ab", 255)
}
