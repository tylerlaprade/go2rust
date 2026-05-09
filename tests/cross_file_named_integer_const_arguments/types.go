package main

type RelocKind int32
type SyncMarker int

const (
	RelocString RelocKind = iota
	RelocMeta
)

const (
	SyncEOF SyncMarker = iota + 1
	SyncBool
)

type Encoder struct{}

func (*Encoder) sync(m SyncMarker) int {
	return int(m)
}

func (e *Encoder) callSync() int {
	return e.sync(SyncBool)
}
