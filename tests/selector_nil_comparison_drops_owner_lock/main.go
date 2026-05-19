package main

import "fmt"

type Config struct {
	Env []string
	Dir string
}

type Invocation struct {
	CleanEnv   bool
	Env        []string
	WorkingDir string
}

type State struct {
	cfg *Config
}

func (s *State) invocation() Invocation {
	cfg := s.cfg
	return Invocation{
		Env:        cfg.Env,
		CleanEnv:   cfg.Env != nil,
		WorkingDir: cfg.Dir,
	}
}

func worker(state *State, done chan bool) {
	inv := state.invocation()
	done <- inv.CleanEnv && len(inv.Env) == 1 && inv.WorkingDir == "work"
}

func main() {
	state := &State{cfg: &Config{Env: []string{"A=B"}, Dir: "work"}}
	done := make(chan bool)
	go worker(state, done)
	go worker(state, done)
	fmt.Println(<-done && <-done)
}
