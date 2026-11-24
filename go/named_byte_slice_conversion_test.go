package main

import (
	"strings"
	"testing"
)

func TestDefinedByteSliceConversionsWrapBytes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Bytes []byte
type Runes []rune

func main() {
	b := Bytes("xyz")
	var c Bytes = []byte("abc")
	r := Runes("hi")
	c = []byte("again")
	_, _, _ = b, c, r
}
`)
	for _, want := range []string{
		`Bytes(Rc::new(RefCell::new(Some(("xyz".to_string()).as_bytes().to_go_slice()))))`,
		`let mut c: Rc<RefCell<Option<Bytes>>> = Rc::new(RefCell::new(Some(Bytes(Rc::new(RefCell::new(Some(("abc".to_string()).as_bytes().to_go_slice())))))));`,
		`Runes(Rc::new(RefCell::new(Some(("hi".to_string()).chars().map(|c| c as i32).collect::<Vec<_>>()))))`,
		`Some(Bytes(Rc::new(RefCell::new(Some(("again".to_string()).as_bytes().to_go_slice())))))`,
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
}
