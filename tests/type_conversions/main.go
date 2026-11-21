package main

import "fmt"

func main() {
	// Basic numeric type conversions
	fmt.Println("=== Basic numeric conversions ===")

	var i int = 42
	var f float64 = float64(i)
	var i2 int = int(f)

	fmt.Printf("int: %d\n", i)
	fmt.Printf("float64: %.2f\n", f)
	fmt.Printf("back to int: %d\n", i2)

	// Different integer sizes
	fmt.Println("\n=== Integer size conversions ===")

	var i8 int8 = 127
	var i16 int16 = int16(i8)
	var i32 int32 = int32(i16)
	var i64 int64 = int64(i32)

	fmt.Printf("int8: %d\n", i8)
	fmt.Printf("int16: %d\n", i16)
	fmt.Printf("int32: %d\n", i32)
	fmt.Printf("int64: %d\n", i64)

	// Unsigned integers
	fmt.Println("\n=== Unsigned integer conversions ===")

	var ui uint = 42
	var ui8 uint8 = uint8(ui)
	var ui16 uint16 = uint16(ui8)
	var ui32 uint32 = uint32(ui16)
	var ui64 uint64 = uint64(ui32)

	fmt.Printf("uint: %d\n", ui)
	fmt.Printf("uint8: %d\n", ui8)
	fmt.Printf("uint16: %d\n", ui16)
	fmt.Printf("uint32: %d\n", ui32)
	fmt.Printf("uint64: %d\n", ui64)

	// Float conversions
	fmt.Println("\n=== Float conversions ===")

	var f64 float64 = 3.14159265359
	var f32 float32 = float32(f64)
	var backToF64 float64 = float64(f32)

	fmt.Printf("float64: %.10f\n", f64)
	fmt.Printf("float32: %.10f\n", f32)
	fmt.Printf("back to float64: %.10f\n", backToF64)

	// String conversions
	fmt.Println("\n=== String conversions ===")

	var r rune = 'A'
	var b byte = 65

	fmt.Printf("rune 'A': %c (%d)\n", r, r)
	fmt.Printf("byte 65: %c (%d)\n", b, b)

	// Rune to string
	str := string(r)
	fmt.Printf("rune to string: %s\n", str)

	// Byte slice to string
	bytes := []byte{72, 101, 108, 108, 111}
	strFromBytes := string(bytes)
	fmt.Printf("bytes to string: %s\n", strFromBytes)

	// String to byte slice
	backToBytes := []byte(strFromBytes)
	fmt.Printf("string to bytes: %v\n", backToBytes)

	// String to rune slice
	runes := []rune("Hello, 世界")
	fmt.Printf("string to runes: %v\n", runes)
	fmt.Printf("rune count: %d\n", len(runes))

	// Rune slice back to string
	backToString := string(runes)
	fmt.Printf("runes to string: %s\n", backToString)

	// Boolean conversions (not direct, but showing concept)
	fmt.Println("\n=== Boolean-like conversions ===")

	var zero int = 0
	var nonZero int = 42

	// Go doesn't have direct bool conversion, but we can demonstrate the concept
	fmt.Printf("zero == 0: %t\n", zero == 0)
	fmt.Printf("nonZero != 0: %t\n", nonZero != 0)

	// Pointer conversions
	fmt.Println("\n=== Pointer conversions ===")

	var num int = 100
	var ptr *int = &num

	fmt.Printf("value: %d\n", num)
	fmt.Printf("pointer: %s\n", "0xDEADBEEF") // Fixed address for deterministic output
	fmt.Printf("dereferenced: %d\n", *ptr)

	// Interface conversions (basic)
	fmt.Println("\n=== Interface conversions ===")

	var any interface{} = 42
	fmt.Printf("interface{} value: %v\n", any)
	fmt.Printf("interface{} type: %T\n", any)

	// Type assertion
	if intVal, ok := any.(int); ok {
		fmt.Printf("asserted as int: %d\n", intVal)
	}

	// Change interface value
	any = "hello"
	fmt.Printf("new interface{} value: %v\n", any)
	fmt.Printf("new interface{} type: %T\n", any)

	if strVal, ok := any.(string); ok {
		fmt.Printf("asserted as string: %s\n", strVal)
	}

	// Complex number conversions
	fmt.Println("\n=== Complex number conversions ===")

	var c64 complex64 = 3 + 4i
	var c128 complex128 = complex128(c64)

	fmt.Printf("complex64: %v\n", c64)
	fmt.Printf("complex128: %v\n", c128)

	// Extract real and imaginary parts
	real := real(c128)
	imag := imag(c128)
	fmt.Printf("real part: %.2f\n", real)
	fmt.Printf("imaginary part: %.2f\n", imag)

	// Create complex from parts
	newComplex := complex(real, imag)
	fmt.Printf("reconstructed: %v\n", newComplex)

	// Overflow demonstration (be careful!)
	fmt.Println("\n=== Overflow examples ===")

	var bigInt int64 = 1000000
	var smallInt int8 = int8(bigInt) // This will overflow!
	fmt.Printf("int64: %d\n", bigInt)
	fmt.Printf("int8 (overflow): %d\n", smallInt)

	// Precision loss in float conversion
	var preciseFloat float64 = 1.23456789012345
	var lessPrec float32 = float32(preciseFloat)
	fmt.Printf("float64: %.15f\n", preciseFloat)
	fmt.Printf("float32: %.15f\n", lessPrec)

	// Custom type conversions
	fmt.Println("\n=== Custom type conversions ===")

	type MyInt int
	type MyString string

	var mi MyInt = 42
	var regularInt int = int(mi)
	var backToMyInt MyInt = MyInt(regularInt)

	fmt.Printf("MyInt: %d\n", mi)
	fmt.Printf("regular int: %d\n", regularInt)
	fmt.Printf("back to MyInt: %d\n", backToMyInt)

	var ms MyString = "hello"
	var regularString string = string(ms)
	var backToMyString MyString = MyString(regularString)

	fmt.Printf("MyString: %s\n", ms)
	fmt.Printf("regular string: %s\n", regularString)
	fmt.Printf("back to MyString: %s\n", backToMyString)
}
