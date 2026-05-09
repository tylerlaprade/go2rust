package main

type Version uint32

const (
	V0 Version = iota
	V1
	V2
	numVersions = iota
)

const (
	flagSyncMarkers = 1 << iota
)

type Header struct {
	version Version
}

func (v Version) Has(f Version) bool {
	return V0 <= v && (v < V2 || f == V0)
}
