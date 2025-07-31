package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {
	rand.Seed(time.Now().UnixNano())
	fmt.Println("Random int:", rand.Intn(100))
	fmt.Println("Random float:", rand.Float64())
}
