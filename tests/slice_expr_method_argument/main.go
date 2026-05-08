package main

import "fmt"

type sink struct{}

func (s sink) Count(values []byte) int {
	return len(values)
}

func main() {
	var buf [128]byte
	sink := sink{}
	fmt.Println(sink.Count(buf[:0]))
}
