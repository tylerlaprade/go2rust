package main

import (
	"fmt"
	"time"
)

func main() {
	ticker := time.NewTicker(250 * time.Millisecond)
	count := 0

	for count < 3 {
		<-ticker.C
		count++
	}
	ticker.Stop()
	fmt.Println("Ticks:", count)
	fmt.Println("Ticker stopped")
}
