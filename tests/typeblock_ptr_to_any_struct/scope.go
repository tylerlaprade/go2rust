package main

type ObjKind int

type Object struct {
	Kind ObjKind
	Name string
	Decl any
	Data any
	Type any
}
