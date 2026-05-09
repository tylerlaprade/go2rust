package main

import "fmt"

func setFlag() uint32 {
	var flags uint32
	flags |= flagSyncMarkers
	return flags
}

func main() {
	fmt.Println(setFlag())
}
