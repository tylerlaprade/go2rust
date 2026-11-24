package main

import (
	"strings"
	"testing"
)

// The translator bugs strings.Replacer and strings.Map exposed, written as
// an ordinary program: a defined [256]byte table, a [256][]byte table with
// nil elements, len stored into an int table, a defined []byte that
// appends strings and converts to string, and a range over a string whose
// body reads the string again (DEV-147).
func TestStringsReplacerShapesTranslate(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type byteTable [256]byte

type appendWriter []byte

func (w *appendWriter) WriteString(s string) {
	*w = append(*w, s...)
}

type replacer struct {
	mapping      [256]byte
	replacements [256][]byte
	skip         [256]int
}

func newTable() *byteTable {
	r := new(byteTable)
	for i := range r {
		r[i] = byte(i)
	}
	return r
}

func main() {
	local := byteTable{}
	for i := range local {
		local[i] = 1
	}
	var rep replacer
	for _, b := range rep.mapping {
		_ = b
	}
	nils := 0
	for i := range rep.replacements {
		if rep.replacements[i] == nil {
			nils++
		}
	}
	rep.skip[0] = len("abc")
	var buf appendWriter
	buf.WriteString("hi")
	s := "abc"
	n := 0
	for i := range s {
		n += len(s[i:])
	}
	fmt.Println(newTable()[1], nils, rep.skip[0], string(buf), n)
}
`)
	for _, want := range []string{
		"pub fn new_table() -> Rc<RefCell<Option<byteTable>>>",
		"for i in 0..(256)",
		`.cloned().expect("array value")`,
		"__nil_element.is_nil()",
		"std::array::from_fn::<_, 256, _>(|_| Vec::nil())",
		".len()) as i64;",
		".as_bytes().iter().cloned());",
		"let __bytes_value = __bytes_guard.as_ref().cloned().unwrap_or_default(); __bytes_value }",
		"let __range_string = (*s.borrow().as_ref().unwrap()).clone(); __range_string }.char_indices()",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}
