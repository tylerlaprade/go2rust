package main

import "fmt"

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
}
