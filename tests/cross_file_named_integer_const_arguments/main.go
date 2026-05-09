package main

import "fmt"

func takeReloc(k RelocKind) int {
	return int(k)
}

func main() {
	var e Encoder
	fmt.Println(takeReloc(RelocMeta))
	fmt.Println(e.sync(SyncBool))
	fmt.Println(e.callSync())
}
