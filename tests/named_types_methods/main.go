package main

import "fmt"

type Celsius float64
type Fahrenheit float64

func (c Celsius) ToFahrenheit() Fahrenheit {
	return Fahrenheit(c*9/5 + 32)
}

func (f Fahrenheit) ToCelsius() Celsius {
	return Celsius((f - 32) * 5 / 9)
}

type StringAlias = string

func main() {
	var temp Celsius = 100
	fmt.Printf("%v°C = %v°F\n", temp, temp.ToFahrenheit())

	var f Fahrenheit = 212
	fmt.Printf("%v°F = %v°C\n", f, f.ToCelsius())

	var s StringAlias = "hello"
	fmt.Println(s)
}
