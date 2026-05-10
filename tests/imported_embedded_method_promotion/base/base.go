package base

import "fmt"

type Decoder struct {
	Value int
}

type PkgDecoder struct {
	Base int
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

func (d *Decoder) Clone() Decoder {
	return Decoder{Value: d.Value}
}

func (p *PkgDecoder) NewDecoder(delta int) Decoder {
	return Decoder{Value: p.Base + delta}
}

func (p *PkgDecoder) RetireDecoder(d *Decoder) {
	_ = p
	_ = d
}
