package main

import "fmt"

// GAP: copy(dst, src) where dst is a wrapped slice held in a struct field.
// transpileCopy's non-SliceExpr branch computes the length via
// (TranspileExpression(dst)).len() without unwrapping the handle (E0599),
// and only fully-unwraps the per-element place for *ast.Ident, not a
// selector (E0608). Root cause: go/stdlib.go transpileCopy.
type buf struct{ dst []byte }

func (b *buf) fill(src []byte) {
	b.dst = make([]byte, len(src))
	copy(b.dst, src)
}

func main() {
	b := &buf{}
	b.fill([]byte("hello"))
	fmt.Println(string(b.dst))
}
