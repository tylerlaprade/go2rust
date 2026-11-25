package main

import "fmt"

// readPair scans two ints; its file does not import io.
func readPair(s string) (int, error) {
	var x, y int
	_, err := fmt.Sscan(s, &x, &y)
	return x + y, err
}
