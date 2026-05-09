package main

import "fmt"

type RelocKind int32
type SyncMarker int
type Field int

const (
	RelocString RelocKind = iota
	RelocMeta
)

const (
	SyncEOF SyncMarker = iota + 1
	SyncBool
)

const (
	Flags Field = iota
	HasInit
)

type Encoder struct{}

func takeReloc(k RelocKind) int {
	return int(k)
}

func (*Encoder) sync(m SyncMarker) int {
	return int(m)
}

func (e *Encoder) callSync() int {
	return e.sync(SyncBool)
}

func fieldEnabled(f Field) int {
	return int(f) + 10
}

func main() {
	var e Encoder
	fmt.Println(takeReloc(RelocMeta))
	fmt.Println(e.sync(SyncBool))
	fmt.Println(e.callSync())
	fmt.Println(fieldEnabled(HasInit))
}
