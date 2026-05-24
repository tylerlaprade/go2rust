package main

import "fmt"

func main() {
	bytes := []byte("hi\tthere\nworld")
	for i, b := range bytes {
		switch b {
		case '\t', '\n', '\r':
			bytes[i] = ' '
		}
	}
	fmt.Println(string(bytes))
}
