package main

import "fmt"

func decode(ver uint32, flags uint32) bool {
	var h Header
	h.version = Version(ver)
	if h.version >= numVersions {
		return false
	}
	return h.version.Has(V1) && flags&flagSyncMarkers != 0
}

func main() {
	fmt.Println(decode(1, 1))
	fmt.Println(decode(3, 1))
	fmt.Println(decode(1, 0))
}
