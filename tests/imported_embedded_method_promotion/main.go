package main

import (
	"fmt"

	"example.com/importedembed/base"
)

type Reader struct {
	base.Decoder
	name string
}

type pkgReader struct {
	base.PkgDecoder
}

func (pr *pkgReader) newReader(delta int) *Reader {
	return &Reader{
		Decoder: pr.NewDecoder(delta),
		name:    "frompkg",
	}
}

func (pr *pkgReader) retireReader(r *Reader) {
	pr.RetireDecoder(&r.Decoder)
}

func forceConcurrentWrappers() {
	done := make(chan bool)
	go func() {
		done <- true
	}()
	<-done
}

func main() {
	forceConcurrentWrappers()

	r := Reader{
		Decoder: base.Decoder{Value: 3},
		name:    "reader",
	}

	r.Add(4)
	fmt.Println(r.Label("reader"))
	fmt.Println(r.Snapshot())

	copied := Reader{
		Decoder: r.Clone(),
		name:    "copy",
	}
	fmt.Println(copied.Label("copy"))

	pr := pkgReader{
		PkgDecoder: base.PkgDecoder{Base: 10},
	}
	fromPkg := pr.newReader(5)
	fmt.Println(fromPkg.Label("frompkg"))
	pr.retireReader(fromPkg)
	fmt.Println(fromPkg.Label("retired"))
}
