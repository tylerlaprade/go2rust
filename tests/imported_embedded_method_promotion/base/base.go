package base

import "fmt"

type Decoder struct {
	Value int
}

func (d *Decoder) Add(n int) {
	d.Value += n
}

func (d *Decoder) Label(prefix string) string {
	return fmt.Sprintf("%s:%d", prefix, d.Value)
}

func (d Decoder) Snapshot() int {
	return d.Value
}
