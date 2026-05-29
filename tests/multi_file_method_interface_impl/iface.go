package main

// Stepper is implemented by Counter, whose methods are split across two files.
type Stepper interface {
	Step()
	Value() int
}
