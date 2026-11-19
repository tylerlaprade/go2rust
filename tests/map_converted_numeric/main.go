package main

type Span int64
type Code = uint16

const Tick Span = 3

var units = map[string]uint64{"tick": uint64(Tick)}

func main() {
	n := int16(-4)
	signed := map[string]int32{"n": int32(n)}
	decimal := map[string]float64{"tick": float64(Tick)}
	println(units["tick"], signed["n"], int(decimal["tick"]))
	wide := int32(270)
	codes := map[string]Code{"x": Code(wide)}
	flags := map[string]bool{"x": bool(true)}
	labels := map[string]string{"x": string('x')}
	println(codes["x"], flags["x"], labels["x"])
}
