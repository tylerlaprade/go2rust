package main

import (
	"strings"
	"testing"
)

// A struct type declared inside a function is emitted as an alias of the
// generated struct for its underlying type, so a field holding a pointer to
// a slice element is a GoPtr there too, and its literal builds one
// (DEV-131).
func TestLocalStructSliceElemPointerFieldIsGoPtr(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Item struct {
	Title string
	Price int
}

func main() {
	items := []Item{{"a", 10}, {"b", 20}}
	type holder struct{ p *Item }
	h := holder{p: &items[1]}
	h.p.Title = "B"
	anon := struct{ cur *Item }{cur: &items[0]}
	anon.cur.Price = 1
	fmt.Println(items)
}
`)
	if strings.Contains(rust, "pub p: Rc<RefCell<Option<Item>>>") || !strings.Contains(rust, "pub p: GoPtr<Item>") {
		t.Errorf("the local struct's field should be a GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "slice element pointer cannot initialize pointer field") {
		t.Errorf("the anonymous struct literal should build the GoPtr:\n%s", rust)
	}
	if strings.Count(rust, "GoPtr::slice_elem(GoSliceElemPtr::new(items.clone()") < 2 {
		t.Errorf("both literals should point at the slice element:\n%s", rust)
	}
}
