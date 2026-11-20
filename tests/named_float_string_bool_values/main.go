// Types the package defines over a float, a string or a bool, used the way
// Go programs use them: arithmetic and conversions, typed constants, map
// keys, sorting, len/index/slice/range on a named string, and conditions on
// a named bool. Most of these did not compile.
package main

import (
	"fmt"
	"math"
	"sort"
	"strings"
)

type Celsius float64
type Fahrenheit float64

func CToF(c Celsius) Fahrenheit { return Fahrenheit(c*9/5 + 32) }
func FToC(f Fahrenheit) Celsius { return Celsius((f - 32) * 5 / 9) }

func (c Celsius) String() string { return fmt.Sprintf("%.1f°C", float64(c)) }

func (c *Celsius) Warm(by Celsius) { *c += by }

func (c Celsius) Truncated() Celsius { return Celsius(float64(int(c))) }

func zero() Celsius { return 0 }

func average(xs []Celsius) Celsius {
	var sum Celsius
	for _, x := range xs {
		sum += x
	}
	return sum / Celsius(len(xs))
}

type Reading struct {
	Place string
	Temp  Celsius
}

type Color string

const (
	Red   Color = "red"
	Green Color = "green"
)

func (c Color) Upper() string { return strings.ToUpper(string(c)) }

type Status string

const (
	Active   Status = "active"
	Inactive Status = "inactive"
)

func (s Status) String() string { return "status:" + string(s) }

func label(s Status) string {
	switch s {
	case Active:
		return "A"
	case Inactive:
		return "I"
	default:
		return "?"
	}
}

type Enabled bool

func describe(e Enabled) string {
	if e {
		return "on"
	}
	return "off"
}

type Meters float64

func main() {
	boiling := Celsius(100)
	fmt.Println(boiling, CToF(boiling), FToC(212))
	fmt.Println(math.Sqrt(float64(boiling)), int(boiling)/3, -boiling, +boiling)
	temps := []Celsius{20.5, 18, 25.25}
	fmt.Println(average(temps))
	sort.Slice(temps, func(i, j int) bool { return temps[i] < temps[j] })
	fmt.Println(temps)
	hottest := temps[0]
	for _, t := range temps[1:] {
		if t > hottest {
			hottest = t
		}
	}
	fmt.Println(hottest)

	var c Celsius
	c += 1.5
	c.Warm(2)
	fmt.Println(c, c.Truncated(), zero(), c == 3.5, c != 0)
	c = 10.25
	fmt.Printf("%.2f %v %T\n", c, c, c)
	r := Reading{"lab", 21.5}
	r.Temp += 0.5
	fmt.Println(r.Temp, r.Temp > 20)
	rs := []Reading{{"a", 3}, {"b", 1}, {"c", 2}}
	sort.Slice(rs, func(i, j int) bool { return rs[i].Temp < rs[j].Temp })
	for _, x := range rs {
		fmt.Print(x.Place, x.Temp, " ")
	}
	fmt.Println()
	fmt.Println(math.Max(float64(rs[0].Temp), float64(rs[2].Temp)))
	byPlace := map[string]Celsius{"x": 1.25}
	byPlace["y"] = byPlace["x"] * 2
	fmt.Println(byPlace["y"], len(byPlace))

	a, b := Meters(7), Meters(2)
	fmt.Println(a/b, a*2+b, (a-b)*(a+b))

	col := Red
	fmt.Println(col, col.Upper(), len(col), col == Red, col != Green)
	colors := map[Color]int{Red: 1, Green: 2}
	fmt.Println(colors[Red], colors["green"])
	name := Color("blue")
	name += "ish"
	fmt.Println(name, strings.HasPrefix(string(name), "bl"), name[0], name[1:4], len(name))
	for i, ch := range name {
		if i == 2 {
			fmt.Println(string(ch))
		}
	}

	s := Active
	fmt.Println(s, label(s), label(Inactive), label("other"))
	statuses := []Status{Inactive, Active, "pending"}
	sort.Slice(statuses, func(i, j int) bool { return statuses[i] < statuses[j] })
	fmt.Println(statuses)
	parts := []string{}
	for _, st := range statuses {
		parts = append(parts, string(st))
	}
	fmt.Println(strings.Join(parts, ","))
	var boxed interface{} = Active
	switch v := boxed.(type) {
	case Status:
		fmt.Println("status", v)
	case string:
		fmt.Println("string", v)
	}
	if st, ok := boxed.(Status); ok {
		fmt.Println(st == Active, len(st))
	}
	counts := map[Status]int{}
	counts[Active]++
	counts[s] += 2
	fmt.Println(counts[Active], counts)

	var e Enabled = true
	fmt.Println(describe(e), describe(!e), e && true)
}
