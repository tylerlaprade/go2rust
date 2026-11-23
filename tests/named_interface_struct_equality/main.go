package main

import (
	"fmt"
	"sync"
)

type Value interface {
	Value() float64
}

type Number struct {
	value float64
}

func (number Number) Value() float64 {
	return number.value
}

type Holder struct {
	value  Value
	extra  any
	values [1]Value
	ptr    *int
	lock   *sync.Mutex
}

func main() {
	zero := 0.0
	holder := Holder{value: Number{value: zero / zero}}
	copied := holder
	fmt.Println(holder == holder, holder == copied)

	var empty Holder
	fmt.Println(empty == empty)

	anyNaN := Holder{value: Number{value: 1}, extra: zero / zero}
	arrayNaN := Holder{value: Number{value: 1}, values: [1]Value{Number{value: zero / zero}}}
	fmt.Println(anyNaN == anyNaN, arrayNaN == arrayNaN)

	mixed := Holder{value: Number{value: 1}, extra: 7, values: [1]Value{Number{value: 2}}}
	mixedCopy := mixed
	differentArray := Holder{value: Number{value: 1}, extra: 7, values: [1]Value{Number{value: 3}}}
	fmt.Println(mixed == mixedCopy, mixed == differentArray)

	n := 1
	samePointer := Holder{value: Number{value: 1}, ptr: &n}
	pointerCopy := samePointer
	other := 1
	otherPointer := Holder{value: Number{value: 1}, ptr: &other}
	fmt.Println(samePointer == pointerCopy, samePointer == otherPointer)

	mutex := sync.Mutex{}
	sameMutex := Holder{value: Number{value: 1}, lock: &mutex}
	mutexCopy := sameMutex
	otherMutexValue := sync.Mutex{}
	otherMutex := Holder{value: Number{value: 1}, lock: &otherMutexValue}
	fmt.Println(sameMutex == mutexCopy, sameMutex == otherMutex)
}
