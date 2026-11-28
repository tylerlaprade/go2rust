package main

import "fmt"

type reading struct {
	Values []float64
}

func main() {
	fmt.Println(map[string][]float64{"y": {1e21, 2}})
	fmt.Println(map[string]map[string][]float64{"o": {"x": {1e6, 0.5}}})
	fmt.Println(map[string][]float32{"f": {0.1, 1e7}})
	m := map[int][]float64{1: {3.0, 2.5e-5}, 2: nil}
	fmt.Println(m, len(m[2]))
	fmt.Println([]float64{1e21, 0.1}, [][]float64{{1e6}})
	fmt.Println(reading{[]float64{1e6, 2}})
	fmt.Printf("%v %v\n", m, map[string][]float64{"z": {100000000}})
}
