package main

import "fmt"

type Small complex64
type Phasor complex128

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	p := Phasor(3 + 4i)
	q := Phasor(1 - 2i)
	var smallA Small = 2 + 3i
	var smallB Small = -1 + 0.5i

	fmt.Println("ops:", p+q, p-q, p*q, p/q, -p, +q)
	fmt.Println("constants:", p+2, 2+p, p*(1-1i))
	fmt.Println("small:", smallA+smallB, smallA*smallB)
	fmt.Println("parts:", real(p), imag(p), real(smallA), imag(smallA))
	fmt.Println("compare:", p == q, p != q, p == Phasor(3+4i))
	fmt.Println("convert:", complex128(p), complex64(smallA), Phasor(smallA))
}
