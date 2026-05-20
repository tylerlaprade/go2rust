package dep

import "sync/atomic"

type Counter struct {
	n atomic.Int32
}

func NewCounter() *Counter {
	return &Counter{}
}

func (c *Counter) Add(delta int32) int32 {
	return c.n.Add(delta)
}
