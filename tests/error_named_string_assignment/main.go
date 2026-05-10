package main

import "fmt"

type internalError string

func (e internalError) Error() string {
	return string(e)
}

func assigned() error {
	var err error
	ierr := internalError("assigned")
	err = ierr
	return err
}

func direct() error {
	return internalError("direct")
}

func main() {
	fmt.Println(assigned())
	fmt.Println(direct())
}
