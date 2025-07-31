package main

import (
	"fmt"
	"time"
)

func main() {
	now := time.Now()
	fmt.Println("Current time:", now)

	future := now.Add(24 * time.Hour)
	fmt.Println("Tomorrow:", future)

	fmt.Println("Unix timestamp:", now.Unix())
}
