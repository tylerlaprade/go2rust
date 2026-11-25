// An everyday program: shapes behind an interface printed with Printf
// verbs, an embedded *Animal, a wrapped custom error found with errors.As,
// variadic calls, a type switch, fallthrough, strings and strconv, method
// values and expressions, uint8 wrap-around and byte and rune loops.
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
func (c Circle) String() string    { return fmt.Sprintf("Circle(%.1f)", c.R) }

type Animal struct{ Name string }

func (a *Animal) Speak() string { return a.Name + " makes a sound" }
func (a *Animal) Rename(n string) { a.Name = n }

type Dog struct {
	*Animal
	Breed string
}

func (d Dog) Speak() string { return d.Name + " barks" }

type NotFoundError struct{ Key string }

func (e *NotFoundError) Error() string { return "not found: " + e.Key }

func lookup(m map[string]int, k string) (int, error) {
	if v, ok := m[k]; ok {
		return v, nil
	}
	return 0, fmt.Errorf("lookup failed: %w", &NotFoundError{k})
}

func sum(nums ...int) int {
	t := 0
	for _, n := range nums {
		t += n
	}
	return t
}

func classify(v any) string {
	switch x := v.(type) {
	case int, int64:
		return fmt.Sprintf("integer %v", x)
	case string:
		return "string " + strconv.Quote(x)
	case []int:
		return fmt.Sprintf("slice of %d", len(x))
	case nil:
		return "nil"
	case fmt.Stringer:
		return "stringer " + x.String()
	default:
		return fmt.Sprintf("other %T", x)
	}
}

func grade(n int) string {
	switch {
	case n >= 90:
		return "A"
	case n >= 80:
		return "B"
	case n >= 70:
		fallthrough
	case n >= 60:
		return "C/D"
	}
	return "F"
}

func main() {
	shapes := []Shape{Rect{3, 4}, Circle{1}}
	total := 0.0
	for _, s := range shapes {
		total += s.Area()
		fmt.Printf("%-8T %8.3f %v\n", s, s.Perimeter(), s)
	}
	fmt.Printf("total=%.2f\n", total)
	d := Dog{&Animal{"Rex"}, "lab"}
	fmt.Println(d.Speak(), "|", d.Animal.Speak())
	d.Rename("Max")
	fmt.Println(d.Name, d.Breed)
	m := map[string]int{"a": 1}
	if _, err := lookup(m, "z"); err != nil {
		var nf *NotFoundError
		fmt.Println(err, errors.As(err, &nf), nf.Key)
	}
	nums := []int{1, 2, 3}
	fmt.Println(sum(), sum(1, 2), sum(nums...))
	for _, v := range []any{42, "hi", []int{1}, nil, Circle{2}, 3.5} {
		fmt.Println(classify(v))
	}
	for _, n := range []int{95, 85, 75, 65, 10} {
		fmt.Print(grade(n), " ")
	}
	fmt.Println()
	parts := strings.Split("a,b,,c", ",")
	fmt.Printf("%q %d\n", parts, len(parts))
	fmt.Println(strings.Join(parts, "|"), strings.Repeat("ab", 3), strings.ToUpper("héllo"))
	n, err := strconv.Atoi("12x")
	fmt.Println(n, err)
	f := d.Speak
	g := (*Animal).Speak
	fmt.Println(f(), g(d.Animal))
	var x uint8 = 250
	x += 10
	fmt.Println(x, 1<<10, 0xff&^0x0f, -7/2, -7%2)
	fmt.Fprintln(os.Stderr, "to stderr")
	b := []byte("hello")
	b[0] = 'H'
	fmt.Println(string(b), b[1:3])
	for i, r := range "héy" {
		fmt.Print(i, ":", string(r), " ")
	}
	fmt.Println()
	fmt.Printf("%5d|%-5d|%05d|%x|%X|%o|%b|%e|%g|%8.2f|%+v\n", 42, 42, 42, 255, 255, 8, 5, 1234.5678, 0.000012, 3.14159, Rect{1, 2})
}
