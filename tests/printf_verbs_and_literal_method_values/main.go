package main

import (
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Shape interface {
	Area() float64
	Perimeter() float64
}

type Rect struct{ W, H float64 }
type Circle struct{ R float64 }

func (r Rect) Area() float64       { return r.W * r.H }
func (r Rect) Perimeter() float64  { return 2 * (r.W + r.H) }
func (c Circle) Area() float64     { return 3.14159 * c.R * c.R }
func (c Circle) Perimeter() float64 { return 2 * 3.14159 * c.R }
func (c Circle) String() string    { return "Circle(" + strconv.FormatFloat(c.R, 'f', 1, 64) + ")" }

type ParseError struct {
	Line int
	Err  error
}

func (e *ParseError) Error() string { return fmt.Sprintf("line %d: %v", e.Line, e.Err) }
func (e *ParseError) Unwrap() error { return e.Err }

var ErrEmpty = errors.New("empty input")

func parse(lines []string) (total int, err error) {
	defer func() {
		if r := recover(); r != nil {
			err = fmt.Errorf("recovered: %v", r)
			total = -1
		}
	}()
	for i, l := range lines {
		if l == "" {
			return 0, &ParseError{Line: i + 1, Err: ErrEmpty}
		}
		if l == "boom" {
			panic("bad line " + strconv.Itoa(i))
		}
		n, err := strconv.Atoi(l)
		if err != nil {
			return 0, &ParseError{i + 1, err}
		}
		total += n
	}
	return total, nil
}

type Matrix [2][3]int

func main() {
	shapes := []Shape{Rect{3, 4}, Circle{1}, &Rect{1, 1}}
	for _, s := range shapes {
		fmt.Printf("%T %.2f %.2f\n", s, s.Area(), s.Perimeter())
	}
	fmt.Printf("%v|%+v|%s\n", shapes[0], shapes[0], shapes[1])
	fmt.Printf("[%-6s][%6s][%06.2f][%x][%X][%q][%c][%U]\n", "ab", "cd", 3.14159, 255, "hi", "q\"s", 'Z', 'é')
	fmt.Printf("%5d|%-5d|%+d|%o|%b|%e|%g\n", 42, 42, 42, 8, 5, 12345.678, 0.000012)
	fmt.Printf("%t %v %p\n", true, nil, nil)

	for _, in := range [][]string{{"1", "2", "3"}, {"4", ""}, {"5", "x"}, {"boom"}} {
		total, err := parse(in)
		var pe *ParseError
		switch {
		case errors.As(err, &pe) && errors.Is(err, ErrEmpty):
			fmt.Println("empty at", pe.Line, err)
		case errors.As(err, &pe):
			fmt.Println("parse error at", pe.Line, "-", pe.Err)
		case err != nil:
			fmt.Println("other:", err, total)
		default:
			fmt.Println("total", total)
		}
	}

	var m Matrix
	m2 := m
	m2[1][2] = 7
	fmt.Println(m, m2, m == m2, len(m), len(m[0]))

	var sb strings.Builder
outer:
	for i := 0; i < 4; i++ {
		for j := 0; j < 4; j++ {
			switch {
			case j > i:
				continue outer
			case i == 3:
				break outer
			}
			fmt.Fprintf(&sb, "%d%d ", i, j)
		}
	}
	fmt.Println(strings.TrimSpace(sb.String()))

	switch x := 2; x {
	case 1:
		fmt.Println("one")
	case 2:
		fmt.Println("two")
		fallthrough
	case 3:
		fmt.Println("three")
	default:
		fmt.Println("default")
	}

	f := Rect{2, 5}.Area
	g := Circle.Perimeter
	fmt.Printf("%.1f %.3f\n", f(), g(Circle{2}))
	r := strings.NewReplacer("a", "1", "b", "2")
	fmt.Println(r.Replace("abcab"), strconv.Quote("tab\there"), strings.Fields("  a b   c "))
	fmt.Fprintln(os.Stderr, "done")
}
