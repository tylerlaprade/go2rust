package helper

type Counter struct {
	Value int
}

func NewCounter() *Counter {
	return &Counter{Value: 7}
}

func (c *Counter) Total() int {
	return c.Value + c.Value
}
