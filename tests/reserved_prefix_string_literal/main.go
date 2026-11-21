package main

import "fmt"

const collision = "\ufdd0\ufdd1fa"

func main() {
	runtime := string([]byte{0xfa})

	fmt.Println(len(collision), []byte(collision))
	fmt.Println(collision == runtime)

	joined := collision + "X"
	fmt.Println(len(joined), []byte(joined))

	for i, r := range collision {
		fmt.Printf("%d:%U|", i, r)
	}
	fmt.Println()
}
