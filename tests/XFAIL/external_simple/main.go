package main

import (
	"fmt"

	"github.com/shopspring/decimal"
)

func main() {
	a := decimal.NewFromInt(10)
	b := decimal.NewFromInt(3)
	result := a.Div(b)

	fmt.Printf("10 / 3 = %s\n", result.String())

	c := decimal.NewFromFloat(3.14159)
	d := decimal.NewFromFloat(2.0)
	product := c.Mul(d)

	fmt.Printf("3.14159 * 2 = %s\n", product.String())
}
