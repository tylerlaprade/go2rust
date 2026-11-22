// The same reserved-prefix literal checks with a goroutine present, so
// generated strings use concurrent wrappers.
package main

import "fmt"

const collision = "\ufdd0\ufdd1fa"

func main() {
	go func() {}()
	runtime := string([]byte{0xfa})
	value := collision

	fmt.Println(len(value), []byte(value))
	fmt.Println(value == runtime)

	joined := value + "X"
	fmt.Println(len(joined), []byte(joined))

	for i, r := range value {
		fmt.Printf("%d:%U|", i, r)
	}
	fmt.Println()
}
