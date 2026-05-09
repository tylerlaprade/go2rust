package main

import "fmt"

func rawMarker() uint64 {
	return 5
}

func main() {
	marker := Marker(rawMarker())
	fmt.Println(int(marker))
}
