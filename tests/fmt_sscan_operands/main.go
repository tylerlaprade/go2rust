package main

import "fmt"

type point struct{ x, y int }

func parsePoint(s string) (point, error) {
	var p point
	_, err := fmt.Sscanf(s, "(%d,%d)", &p.x, &p.y)
	return p, err
}

func main() {
	var w, h int
	var unit string
	n, err := fmt.Sscan("3 4 cm", &w, &h, &unit)
	fmt.Println(n, err, w*h, unit)
	grid := make([][]int, 2)
	for i := range grid {
		grid[i] = make([]int, 3)
		for j := range grid[i] {
			fmt.Sscan(fmt.Sprint(i*3+j), &grid[i][j])
		}
	}
	fmt.Println(grid)
	cell := &grid[1][2]
	*cell = 50
	fmt.Println(grid[1], *cell)
	var vals [3]float32
	fmt.Sscan("1.5 2.25 -3", &vals[0], &vals[1], &vals[2])
	fmt.Println(vals)
	p, err := parsePoint("(5,-2)")
	fmt.Println(p, err)
	_, err = parsePoint("5,2")
	fmt.Println(err)
	var b []byte
	var r rune
	var u8 uint8
	n, err = fmt.Sscanf("abc Z 255", "%s %c %d", &b, &r, &u8)
	fmt.Println(n, err, string(b), string(r), u8)
	var c int
	n, err = fmt.Sscanf("50%", "%d%%", &c)
	fmt.Println(n, err, c)
	n, err = fmt.Sscanf("12345", "%3d%d", &c, &w)
	fmt.Println(n, err, c, w)
}
