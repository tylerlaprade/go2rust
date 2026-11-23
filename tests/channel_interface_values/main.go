// A channel of an interface carries interface values: nil can be sent, a
// receive from a closed channel gives nil, and the value received is the one
// the variable held when it was sent.
package main

import "fmt"

type Shape interface {
	Area() int
	Name() string
}

type Rect struct{ W, H int }

func (r Rect) Area() int    { return r.W * r.H }
func (r Rect) Name() string { return "rect" }

type Square struct{ S int }

func (s *Square) Area() int    { return s.S * s.S }
func (s *Square) Name() string { return "square" }

type Pipeline struct {
	in  chan Shape
	out chan Shape
}

func describe(s Shape) string {
	if s == nil {
		return "<nil shape>"
	}
	return fmt.Sprintf("%s:%d", s.Name(), s.Area())
}

func receive(ch <-chan Shape) Shape {
	return <-ch
}

func receiveNamed(ch <-chan Shape) (s Shape) {
	s = <-ch
	return
}

func send(ch chan<- Shape, s Shape) {
	ch <- s
}

func main() {
	ch := make(chan Shape, 8)
	var nothing Shape
	sq := &Square{3}

	ch <- Rect{2, 3}
	ch <- sq
	ch <- nil
	ch <- nothing
	send(ch, Rect{1, 1})

	first := <-ch
	fmt.Println(describe(first))
	var second Shape
	second = <-ch
	fmt.Println(describe(second))
	fmt.Println(describe(<-ch))
	fmt.Println(<-ch == nil)
	fmt.Println(describe(receive(ch)))

	// The value sent is the interface value at the time of the send.
	cur := Shape(Rect{4, 4})
	ch <- cur
	cur = Rect{5, 5}
	got := <-ch
	fmt.Println(describe(got), describe(cur))

	// A pointer inside the value still points at the same struct.
	ch <- sq
	sq.S = 10
	fmt.Println(describe(receiveNamed(ch)))

	// Comma-ok, and a closed channel gives nil.
	ch <- Rect{6, 1}
	close(ch)
	s, ok := <-ch
	fmt.Println(describe(s), ok)
	s, ok = <-ch
	fmt.Println(describe(s), ok)
	last := <-ch
	fmt.Println(last == nil)

	// Range, select and a channel in a struct field.
	p := Pipeline{in: make(chan Shape, 4), out: make(chan Shape, 4)}
	p.in <- Rect{1, 2}
	p.in <- &Square{2}
	p.in <- nil
	close(p.in)
	total := 0
	for shape := range p.in {
		if shape == nil {
			fmt.Println("skip nil")
			continue
		}
		total += shape.Area()
		p.out <- shape
	}
	fmt.Println("total", total)
	for i := 0; i < 3; i++ {
		select {
		case shape := <-p.out:
			fmt.Println("selected", describe(shape))
		default:
			fmt.Println("nothing ready")
		}
	}
	select {
	case p.out <- Rect{7, 7}:
		fmt.Println("sent")
	default:
		fmt.Println("full")
	}
	fmt.Println(describe(<-p.out))

	shapes := []Shape{Rect{1, 3}, &Square{4}}
	for _, shape := range shapes {
		p.out <- shape
	}
	for range shapes {
		fmt.Println(describe(<-p.out))
	}
}
