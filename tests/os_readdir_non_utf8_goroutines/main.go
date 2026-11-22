package main

import (
	"fmt"
	"os"
)

func main() {
	ready := make(chan struct{})
	go func() { close(ready) }()
	<-ready

	err := os.WriteFile("\U0010ffff", nil, 0o600)
	fmt.Println(err == nil)

	entries, err := os.ReadDir(".")
	fmt.Println(err == nil, len(entries))
	for _, entry := range entries {
		fmt.Println([]byte(entry.Name()), entry.IsDir())
	}
}
