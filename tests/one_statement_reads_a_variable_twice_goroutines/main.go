// The same program with a goroutine, so every value is behind a lock.
// One statement reads the same variable two or more times: the arguments of
// a call, the fields and elements of a literal, an append, a swap. A read
// that holds a lock until the statement ends made the next read wait on it.
package main

import "fmt"

type P struct{ X, Y int }

func two(a, b int) int              { return a*10 + b }
func mixed(a float64, b int) string { return fmt.Sprintf("%.1f/%d", a, b) }
func flags(a, b bool) bool          { return a != b }
func strs(a string, n int) string   { return fmt.Sprintf("%s:%d", a, n) }
func pts(a int, p P) int            { return a + p.X }
func sl(a int, s []int) int         { return a + len(s) }

func (p P) Sum(k int) int { return p.X + p.Y + k }

func pair(x int) (int, int) { return -x, x }

func main() {
	started := make(chan bool)
	go func() { started <- true }()
	<-started
	x := 4
	b := true
	s := "go"
	p := P{1, 2}
	xs := []int{5, 6, 7}
	m := map[string]int{"k": 9}
	fmt.Println("conv", two(int(int32(x)), x))
	fmt.Println("neg", two(-x, x))
	fmt.Println("mul", two(x*2, x))
	fmt.Println("float", mixed(float64(x)/2, x))
	fmt.Println("not", flags(!b, b))
	fmt.Println("len", strs(s, len(s)))
	fmt.Println("index", two(xs[0], xs[1]))
	fmt.Println("field", two(p.X, p.Y))
	fmt.Println("struct", pts(p.X, p))
	fmt.Println("slice", sl(xs[2], xs))
	fmt.Println("map", two(m["k"], m["k"]))
	fmt.Println("conv2", two(int(x), int(x)))
	fmt.Println("paren", two((x), -(x)))
	fmt.Println("bit", two(x&1, x|1))
	fmt.Println("cmp", flags(x > 2, x < 9))
	fmt.Println("byte", two(int(s[0]), int(s[1])))

	x = 3
	s = "abc"
	xs = []int{1, 2}
	fmt.Println("lit", P{-x, x}, []int{-x, x, int(int64(x))}, map[int]int{-x: x})
	lo, hi := pair(x)
	fmt.Println("ret", lo, hi)
	fmt.Printf("printf %d %d %d %d\n", -x, x, len(s), len(xs))
	xs = append(xs, -x, x, len(xs))
	fmt.Println("append", xs)
	y := 10
	x, y = -y, x
	fmt.Println("swap", x, y)
	p = P{1, 2}
	fmt.Println("method", p.Sum(-p.X), p.Sum(len(xs)))
	fmt.Println("cmp", -x == y, float64(x) < float64(y), len(s) == len(xs))
	var u uint8 = 200
	fmt.Println("uint", int(u)+int(u), -int(u))
	z := float64(x) * float64(y)
	fmt.Println("mul", z, int(z)/len(s))
}
