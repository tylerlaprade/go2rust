package main

import (
	"fmt"
	"math/rand"
)

func main() {
	rand.Seed(1)
	n := rand.Intn(100)
	f := rand.Float64()
	fmt.Println("Random int in range:", n >= 0 && n < 100)
	fmt.Println("Random float in range:", f >= 0 && f < 1)
}
