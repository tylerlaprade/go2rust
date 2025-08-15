package main
func apply(f func() int) int { return f() }
func main() {
    x := 5
    apply(func() int { return x })
}
