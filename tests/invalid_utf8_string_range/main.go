package main

import "fmt"

type label string

func main() {
	s := string([]byte{'A', 0xfa, 0xc2, 0xa2, 0xe2, '(', 0xa1, 0xff, 0xc2, 0xf0, 0x9f, 0x98, 0x80, 'Z'})
	for i, r := range s {
		fmt.Printf("%d:%U|", i, r)
	}
	fmt.Println()
	for i := range s {
		fmt.Printf("%d|", i)
	}
	fmt.Println()
	for _, r := range s {
		fmt.Printf("%d|", int32(r))
	}
	fmt.Println()
	count := 0
	for range s {
		count++
	}
	fmt.Println(count)
	for i, r := range label(s) {
		fmt.Printf("%d:%U|", i, r)
	}
	fmt.Println()
}
