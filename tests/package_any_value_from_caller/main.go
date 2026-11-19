package main

import (
	"errors"
	"fmt"

	"example.com/anyfromcaller/logger"
)

// A value put into an interface does not stay in the package that put it
// there. main creates these values; another package prints and copies them.
func main() {
	// An error, handed over directly and as one of several arguments.
	logger.Show(errors.New("bad"))
	logger.Each(errors.New("later"))
	logger.Each(fmt.Errorf("wrapped %d", 7), "text", 3)

	// The other package can still tell that it is an error.
	fmt.Println(logger.Describe(errors.New("described")))
	fmt.Println(logger.Describe(3))

	// Slices and maps of basic types.
	logger.Show([]int{1, 2})
	logger.Show([]string{"a", "b"})
	logger.Show(map[string]int{"k": 1})
	logger.Each([]int{3, 4}, map[string]int{"m": 2}, []float64{1.5})
	fmt.Println(logger.Kind([]string{"x"}), logger.Kind(map[string]int{}))

	// The other direction: a value created over there, printed here.
	var numbers interface{} = logger.Numbers()
	fmt.Println(numbers)

	logger.Show(7)
}
