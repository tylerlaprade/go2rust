package main

import (
	"fmt"
	"time"
)

type event struct {
	when time.Time
}

func main() {
	localBase := time.Unix(1700000000, 0)
	base := localBase.UTC()
	fmt.Println("Base time:", base)

	future := base.Add(time.Hour)
	fmt.Println("One hour later:", future)

	fmt.Println("Unix timestamp:", base.Unix())

	var ev event
	fmt.Println("Zero field:", ev.when.IsZero())
}
