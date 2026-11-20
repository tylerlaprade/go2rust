// Reading a number's bits through unsafe.Pointer, as math.Float64bits does,
// panicked with "unsafe.Pointer conversion to u64". A float and an integer
// of the same size are now read as each other's bits.
package main

import (
	"fmt"
	"unsafe"
)

func Float64bits(f float64) uint64     { return *(*uint64)(unsafe.Pointer(&f)) }
func Float64frombits(b uint64) float64 { return *(*float64)(unsafe.Pointer(&b)) }
func Float32bits(f float32) uint32     { return *(*uint32)(unsafe.Pointer(&f)) }

func signedOf(p *uint64) int64 { return *(*int64)(unsafe.Pointer(p)) }

func abs(x float64) float64 { return Float64frombits(Float64bits(x) &^ (1 << 63)) }

func main() {
	b := Float64bits(1.5)
	fmt.Printf("%x %v %x\n", b, Float64frombits(b), Float32bits(-2))
	u := uint64(1 << 63)
	fmt.Println(signedOf(&u), abs(-0.25), abs(3))
}
