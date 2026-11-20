// A value of a named slice type, "type Ints []int", did not print: the
// newtype has no Display, and the elements are in the handle inside it.
// Writing an element of an element, m[i][j] = v, indexed the newtype.
package main

import "fmt"

type Matrix [][]float64
type Ints []int
type Words []string

func (m Matrix) Transpose() Matrix {
	t := make(Matrix, len(m[0]))
	for i := range t {
		t[i] = make([]float64, len(m))
		for j := range m {
			t[i][j] = m[j][i]
		}
	}
	return t
}

func mk() Ints { return Ints{1, 2} }

func main() {
	m := Matrix{{1, 2, 3}, {4, 5, 6}}
	xs := Ints{5, 6}
	ws := Words{"a", "b"}
	fmt.Println(m, xs, ws)
	fmt.Println(m.Transpose(), mk())
	fmt.Printf("%v %v\n", m, mk())
	m[1][2] = 9
	grid := make(Matrix, 2)
	for i := range grid {
		grid[i] = make([]float64, 2)
		grid[i][i] = 1
	}
	fmt.Println(m, grid, len(m.Transpose()))
}
