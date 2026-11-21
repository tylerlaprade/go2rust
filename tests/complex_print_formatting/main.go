package main

import "fmt"

func printDynamic(format string, args ...any) {
	fmt.Printf(format, args...)
}

func main() {
	var positive complex64 = 3 + 4i
	var negative complex128 = 3 - 4i
	zero := complex(3, 0)
	pureImaginary := complex(0, 4)

	fmt.Print("print:", positive, "\n")
	fmt.Println("println:", negative)
	fmt.Printf("printf: %v %v\n", positive, zero)
	fmt.Println("sprint:", fmt.Sprint(positive))
	fmt.Println("sprintf:", fmt.Sprintf("%v", negative))
	fmt.Println("pure:", pureImaginary)
	fmt.Printf("numeric: %f %.2f %e %g\n", positive, negative, positive, negative)
	fmt.Printf("flags: %+v % v |%20.2f| |%020.2f| |%-20.2f|\n", positive, positive, negative, negative, negative)
	fmt.Printf("signs: %+f % f %+f\n", positive, positive, negative)
	fmt.Printf("bases: %b %x %X\n", positive, negative, positive)
	fmt.Printf("base flags: %#x %#X\n", positive, negative)
	fmt.Printf("base precision: %#.0x %#.4x %.3X\n", complex(1.23, 1), complex(123, 1), complex(123, 1))
	fmt.Printf("binary flags: %#b %.4b\n", positive, negative)
	printDynamic("spread: %.1f %g\n", positive, negative)
	printDynamic("spread bases: %b %x %X\n", positive, negative, positive)
}
