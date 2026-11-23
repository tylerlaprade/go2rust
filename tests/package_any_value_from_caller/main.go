package main

import (
	"errors"
	"fmt"

	"example.com/anyfromcaller/logger"
)

type Point struct {
	X int
	Y int
}

type callerError struct {
	message string
}

func (err callerError) Error() string { return err.message }

type pointerError struct {
	message string
}

func (err *pointerError) Error() string { return err.message }

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
	logger.Show(Point{1, 2})
	logger.Each([]int{3, 4}, map[string]int{"m": 2}, []float64{1.5})
	fmt.Println(logger.Kind([]string{"x"}), logger.Kind(map[string]int{}))

	// Caller-defined dynamic types must keep both their concrete identity and
	// any interface capabilities after crossing into the dependency crate.
	// Start with a statically typed error so this path cannot borrow a registry
	// entry created by an earlier concrete-to-any conversion.
	var asError error = callerError{"typed error"}
	logger.Show(asError)
	fmt.Println(logger.Describe(asError), logger.Kind(asError), logger.Classify(asError))
	value := callerError{"caller error"}
	logger.Show(value)
	fmt.Println(logger.Describe(value))
	fmt.Println(logger.Kind(value))
	fmt.Println(logger.Classify(value))
	logger.Each(Point{5, 6}, callerError{"copied error"})
	var boxed any = callerError{"preboxed"}
	fmt.Println(logger.Describe(boxed))
	fmt.Println(logger.RequireError(boxed).Error())
	var pointerAsError error = &pointerError{"typed pointer error"}
	logger.Show(pointerAsError)
	fmt.Println(logger.Describe(pointerAsError), logger.Kind(pointerAsError), logger.Classify(pointerAsError))
	pointer := &pointerError{"pointer error"}
	logger.Show(pointer)
	logger.Each(pointer)
	fmt.Println(logger.Describe(pointer), logger.Kind(pointer), logger.Classify(pointer))
	bound := logger.RequireError(pointer)
	*pointer = pointerError{"replaced"}
	fmt.Println(bound.Error())
	var typedNil *pointerError
	fmt.Println(logger.RequireError(typedNil) == nil)

	// The other direction: a value created over there, printed here.
	var numbers interface{} = logger.Numbers()
	fmt.Println(numbers)

	logger.Show(7)
}
