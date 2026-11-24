package main

import "fmt"

type Celsius struct{ Deg int }

func (c Celsius) String() string { return fmt.Sprintf("%d°C", c.Deg) }

type Counter struct{ N int }

func (c *Counter) String() string { return fmt.Sprintf("Counter<%d>", c.N) }

type Plain struct{ X int }

func makeCelsius(d int) Celsius { return Celsius{d} }

// Stringer returns its argument as a fmt.Stringer.
func Stringer(v fmt.Stringer) fmt.Stringer { return v }

func join(items []fmt.Stringer) string {
	out := ""
	for i, it := range items {
		if i > 0 {
			out += ", "
		}
		out += it.String()
	}
	return out
}

func show(s fmt.Stringer) { fmt.Println("show:", s.String(), s) }

func describe(v any) string {
	switch x := v.(type) {
	case fmt.Stringer:
		return "stringer " + x.String()
	case int:
		return fmt.Sprintf("int %d", x)
	default:
		return fmt.Sprintf("other %T", x)
	}
}

func label(v any) string {
	if s, ok := v.(fmt.Stringer); ok {
		return "labelled " + s.String()
	}
	return "unlabelled"
}

func mustString(v any) (out string) {
	defer func() {
		if r := recover(); r != nil {
			out = fmt.Sprint("panic: ", r)
		}
	}()
	return v.(fmt.Stringer).String()
}

func main() {
	var s fmt.Stringer = Celsius{21}
	fmt.Println(s.String())
	fmt.Println(s)

	c := &Counter{N: 1}
	show(c)
	c.N = 2
	show(c)

	fmt.Println(join([]fmt.Stringer{Celsius{-3}, &Counter{N: 9}, s}))

	fmt.Println(describe(Celsius{5}))
	fmt.Println(describe(c))
	fmt.Println(describe(Counter{}))
	fmt.Println(describe(7))
	fmt.Println(describe(Plain{}))

	fmt.Println(label(Celsius{0}))
	fmt.Println(label(c))
	fmt.Println(label("text"))
	var nothing any
	fmt.Println(label(nothing))

	fmt.Println(mustString(&Counter{N: 4}))
	fmt.Println(mustString(5))
	fmt.Println(mustString(nothing))

	// A value receiver's String makes *Celsius a Stringer too.
	fmt.Println(label(&Celsius{8}), describe(&Celsius{9}))
	// An any conversion asserted in place.
	if st, ok := any(Celsius{10}).(fmt.Stringer); ok {
		fmt.Println("in place", st.String())
	}
	fmt.Printf("%T %T %v\n", s, Stringer(c), s)
	// A call's result, an assertion printed directly, and %T of one.
	var made fmt.Stringer = makeCelsius(11)
	var held any = Celsius{12}
	fmt.Println(made.String(), held.(fmt.Stringer))
	fmt.Printf("%T\n", held.(fmt.Stringer))
	// A failed assertion that is recovered leaves the value usable.
	var five any = 5
	fmt.Println(mustString(five), label(five))
}
