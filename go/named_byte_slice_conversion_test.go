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

// A defined string converts through its String; a nil []byte, a variable
// assigned later and a [][]byte row go into the defined type; and a nil or
// invalid []rune converts to a string without panicking.
func TestDefinedByteSliceFromVariablesRowsAndDefinedStrings(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Bytes []byte
type Runes []rune
type Text string

func main() {
	t := Text("hey")
	b := Bytes(t)
	var nb []byte
	var c Bytes = nb
	raw := []byte{1}
	c = raw
	rows := [][]byte{{7}}
	var e Bytes = rows[0]
	s := string(Runes(nil))
	_, _, _, _ = b, c, e, s
}
`)
	for _, want := range []string{
		"Bytes(Rc::new(RefCell::new(Some((*(*t.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()).clone().as_bytes().to_go_slice()))))",
		"Bytes(Rc::new(RefCell::new(nb.borrow().clone())))",
		"let new_val = Rc::new(RefCell::new(Some(Bytes(Rc::new(RefCell::new(raw.borrow().clone()))))));",
		"Bytes(Rc::new(RefCell::new(Some((*rows.borrow().as_ref().unwrap())[(0) as usize].clone().clone()))))",
		"match __rune_slice_guard.as_ref() { Some(__runes) => (*__runes).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\\u{FFFD}')).collect::<String>(), None => String::new() }",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
}
