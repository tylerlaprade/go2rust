package main

type Code interface {
	Value() int
}

type CodeVal int

func (c CodeVal) Value() int {
	return int(c)
}

const (
	ValBool CodeVal = iota
	ValString
)
