package main

import "fmt"

type Object interface {
	Name() string
}

type TypeName struct {
	name string
}

func (t *TypeName) Name() string {
	return t.name
}

type Builtin struct {
	name string
}

func (b *Builtin) Name() string {
	return b.name
}

func classify(obj Object) string {
	switch obj := obj.(type) {
	case *TypeName:
		return "type:" + obj.Name()
	case *Builtin:
		return "builtin:" + obj.Name()
	default:
		panic("unreachable")
	}
}

func assertTypeName(obj Object) string {
	if t, ok := obj.(*TypeName); ok {
		return t.Name()
	}
	return "not type"
}

func main() {
	typeName := &TypeName{name: "T"}
	builtin := &Builtin{name: "B"}

	fmt.Println(classify(typeName))
	fmt.Println(classify(builtin))
	fmt.Println(assertTypeName(typeName))
	fmt.Println(assertTypeName(builtin))
}
