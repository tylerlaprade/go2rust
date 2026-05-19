package main

import (
	"fmt"
	"os"
	"path/filepath"
)

func main() {
	entries, err := os.ReadDir("data")
	if err != nil {
		fmt.Println("read error")
		return
	}

	for _, entry := range entries {
		joined := filepath.Join("data", entry.Name())
		fmt.Println("entry", joined, filepath.Base(joined), entry.IsDir())
	}

	info, err := os.Stat("data/nested")
	if err != nil {
		fmt.Println("stat error")
		return
	}
	fmt.Println("nested", info.IsDir())
}
