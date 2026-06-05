package main

import "fmt"

type bailout struct {
	msg string
}

type surprise struct{}

func handle(err *string) {
	switch p := recover().(type) {
	case nil:
		*err = "nil"
	case bailout:
		*err = p.msg
	default:
		if p == nil {
			*err = "lost"
		} else {
			*err = "default"
		}
	}
}

func run(payload any) (err string) {
	defer handle(&err)
	done := make(chan bool)
	_ = done
	panic(payload)
}

func main() {
	fmt.Println(run(bailout{"caught"}))
	fmt.Println(run(surprise{}))
}
