package main

import "fmt"

func main() {
	seen := map[string]bool{"ready": true}

	ready, okReady := seen["ready"]
	missing, okMissing := seen["missing"]

	fmt.Println(ready, okReady)
	fmt.Println(missing, okMissing)
}
