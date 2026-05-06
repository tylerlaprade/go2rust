package main

import "fmt"

func main() {
	grouped := map[string][]int{
		"odd":  {1, 3, 5},
		"even": {2, 4, 6},
	}
	fmt.Println(grouped["odd"][1], grouped["even"][2])

	rows := [][]string{
		{"a", "b"},
		{"c", "d"},
	}
	fmt.Println(rows[0][1], rows[1][0])
}
