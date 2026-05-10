package main

import "fmt"

type internalError string

func main() {
	defer func() {
		if e := recover(); e != nil {
			if ierr, ok := e.(internalError); ok {
				fmt.Println(ierr)
			}
		}
	}()

	fmt.Println("done")
}
