package main

import "fmt"

// Celsius is a defined type: fmt scans into it through reflect, which the
// translated scanner does not have (docs/bridge_debt.md, fmt-scan).
type Celsius float64

type Level int

func main() {
	var c Celsius
	var l Level
	n, err := fmt.Sscan("21.5 3", &c, &l)
	fmt.Println(n, err, c, l)
}
