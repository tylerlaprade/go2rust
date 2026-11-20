package main

import "fmt"

type sample struct {
	Large float64
	Small float32
}

type floatError float64

func (floatError) Error() string { return "failed" }

func main() {
	values := []float64{100000, 1000000, 1234567, 2500000.5, 8100000000, 1e21, 0.0001, 0.00001, float64(-1000000)}
	for _, value := range values {
		fmt.Println(value)
	}

	var singleSmall float32 = 0.000001
	var singleThird float32 = float32(1) / 3
	fmt.Println(singleSmall, singleThird)

	var boxed any = 8100000000.0
	fmt.Println(boxed)
	fmt.Println([]float64{1000000, 0.00001})
	float32s := []float32{1000000, 0.00001}
	fmt.Println(float32s)
	fmt.Printf("%T\n", float32s[0])
	fmt.Println(floatError(1000000))
	var boxedSlice any = []float64{1000000, 0.00001}
	fmt.Println(boxedSlice.([]float64))
	boxedCollections := []any{[]float64{1000000, 0.00001}, map[string]float64{"large": 1000000}}
	fmt.Println(boxedCollections[0].([]float64))
	fmt.Println(boxedCollections[1].(map[string]float64))
	fmt.Println(map[string]float64{"large": 1000000, "small": 0.00001})
	fmt.Println(map[string]float32{"large": 1000000, "small": 0.00001})
	fmt.Println(sample{Large: 1000000, Small: 0.00001})
	fmt.Printf("%v %g %.1f\n", 8100000000.0, 8100000000.0, 2.5)
}
