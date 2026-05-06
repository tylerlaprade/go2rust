package main

import (
	"fmt"
	"time"
)

func main() {
	localBase := time.Unix(1700000000, 0)
	base := localBase.UTC()
	fmt.Println("Base time:", base)

	future := base.Add(time.Hour)
	fmt.Println("One hour later:", future)

	fmt.Println("Unix timestamp:", base.Unix())
}
