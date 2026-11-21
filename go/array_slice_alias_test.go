package main

import (
	"strings"
	"testing"
)

func TestSlicedLocalArrayUsesSharedBackingAndArrayCopy(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func fill(dst []byte) { dst[0] = 9 }

func f() {
	a := [4]byte{1, 2, 3, 4}
	fill(a[:])
	middle := a[1:3]
	b := a
	copyView := b[:]
	_, _ = middle, copyView
}
`)

	if !strings.Contains(rust, "Vec::from([1 as u8, 2 as u8, 3 as u8, 4 as u8])") {
		t.Fatalf("sliced local array should use GoSlice backing:\n%s", rust)
	}
	if !strings.Contains(rust, ".reslice3(") {
		t.Fatalf("array slices should reslice the array's shared backing:\n%s", rust)
	}
	if !strings.Contains(rust, ".as_slice().to_vec()") {
		t.Fatalf("array assignment should copy elements into fresh backing:\n%s", rust)
	}
}

func TestSlicedZeroArrayUsesSharedBacking(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func encode(dst []byte) { dst[0] = 7 }

func f() byte {
	var buf [10]byte
	encode(buf[0:])
	return buf[0]
}
`)

	if !strings.Contains(rust, "Vec::from(std::array::from_fn::<_, 10, _>(|_| 0))") {
		t.Fatalf("sliced zero array should allocate fixed-length GoSlice backing:\n%s", rust)
	}
	if !strings.Contains(rust, ".reslice3(") || !strings.Contains(rust, ".get_cloned(") {
		t.Fatalf("slice write and later array read should share GoSlice backing:\n%s", rust)
	}
}

func TestSliceBackedArraySelectionDoesNotRewriteParametersOrNamedArrays(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type named [2]byte

func parameter(a [2]byte) []byte { return a[:] }

func local() {
	a := named{1, 2}
	_ = a[:]
}
`)

	if !strings.Contains(rust, "a: Rc<RefCell<Option<[u8; 2]>>>") {
		t.Fatalf("fixed-array parameters must retain their declared representation:\n%s", rust)
	}
	if !strings.Contains(rust, "named(Rc::new(RefCell::new(Some([1 as u8, 2 as u8]))))") {
		t.Fatalf("named-array locals must retain their newtype representation:\n%s", rust)
	}
}

func TestSliceBackedArrayCopiesFunctionAndFixedArrayInitializers(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func makeArray() [3]int { return [3]int{1, 2, 3} }
type named [3]int

func f() {
	a := makeArray()
	_ = a[:]

	src := [3]int{4, 5, 6}
	b := src
	_ = b[:]

	var explicit [3]int = makeArray()
	_ = explicit[:]

	namedSource := named{10, 11, 12}
	var fromNamed [3]int = namedSource
	_ = fromNamed[:]
}
`)

	if strings.Count(rust, "let __go_array = make_array();") != 2 {
		t.Fatalf("function-returned arrays should be converted into slice backing:\n%s", rust)
	}
	if !strings.Contains(rust, "let __go_array = src.clone();") {
		t.Fatalf("fixed-array variables should be copied into slice backing:\n%s", rust)
	}
	if !strings.Contains(rust, "__go_array.borrow().as_ref().unwrap().0.borrow().as_ref().unwrap().as_slice().to_vec()") {
		t.Fatalf("named arrays should be copied from their inner fixed-array value:\n%s", rust)
	}
	if strings.Contains(rust, "let mut a = make_array();") || strings.Contains(rust, "let mut b = { let __owned") {
		t.Fatalf("slice-backed array initializers retained a fixed-array representation:\n%s", rust)
	}
}
