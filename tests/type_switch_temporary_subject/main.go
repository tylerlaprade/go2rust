package main

import "fmt"

type sampleBox struct {
	value interface{}
}

func (b sampleBox) current() interface{} {
	return b.value
}

func classify(b sampleBox) string {
	switch v := b.current().(type) {
	case int:
		return fmt.Sprintf("int:%d", v)
	default:
		return "other"
	}
}

func main() {
	fmt.Println(classify(sampleBox{value: 7}))
}
