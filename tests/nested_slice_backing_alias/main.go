package main

func main() {
	grid := [][]int{{1, 2}, {3, 4}}
	row := grid[0]
	row[1] = 9
	println(grid[0][1])

	r := []int{5}
	rows := [][]int{}
	rows = append(rows, r)
	r[0] = 6
	println(rows[0][0])

	for _, row := range grid {
		row[0] = 0
	}
	println(grid[0][0], grid[1][0])

	a := [1][]int{{7}}
	b := a
	b[0][0] = 24
	println(a[0][0], b[0][0])

	grid[1][1] += 5
	grid[1] = append(grid[1], 10)
	println(grid[1][1], grid[1][2], len(grid[1]))
}
