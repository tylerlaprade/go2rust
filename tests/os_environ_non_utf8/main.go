package main

import (
	"fmt"
	"os"
)

func main() {
	prefix := "GO2RUST_LIST="
	found := false
	for _, entry := range os.Environ() {
		if len(entry) >= len(prefix) && entry[:len(prefix)] == prefix {
			fmt.Println([]byte(entry))
			found = true
		}
	}
	fmt.Println(found)
}
