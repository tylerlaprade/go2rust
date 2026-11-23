package main

import "fmt"

func vals() (int, int) {
	return 3, 7
}

func count(input int) (n int, err error) {
	n = input
	return n, nil
}

func main() {
	a, b := vals()
	fmt.Println(a)
	fmt.Println(b)

	_, c := vals()
	fmt.Println(c)

	n, err := count(11)
	fmt.Println(n, err == nil)
}
