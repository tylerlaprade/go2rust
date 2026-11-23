package main

import "fmt"

func main() {
	value := 7
	seen := make(chan int, 21)
	go func() {
		seen <- value

		if true {
			value := 9
			seen <- value
		}
		seen <- value

		for i := 0; i < 1; i++ {
			value := 10
			seen <- value
		}
		seen <- value

		for range []int{0} {
			value := 11
			seen <- value
		}
		seen <- value

		switch {
		case true:
			value := 12
			seen <- value
		}
		seen <- value

		select {
		default:
			value := 13
			seen <- value
		}
		seen <- value

		if false {
			panic("unreachable")
		} else {
			value := 14
			seen <- value
		}
		seen <- value

		{
			value := 15
			seen <- value
		}
		seen <- value

		if value := 16; value == 16 {
			seen <- value
		}
		seen <- value

		for value := 17; value < 18; value++ {
			seen <- value
		}
		seen <- value

		switch value := 18; {
		case value == 18:
			seen <- value
		}
		seen <- value
	}()

	for i := 0; i < 21; i++ {
		fmt.Println(<-seen)
	}
}
