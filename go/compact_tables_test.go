package main

import (
	"fmt"
	"strings"
	"testing"
)

func TestCompactHomogeneousVecLiteralsWritesATable(t *testing.T) {
	var elems []string
	for i := 0; i < 20; i++ {
		elems = append(elems, fmt.Sprintf("Range16 { lo: Rc::new(RefCell::new(Some(0x%04x as u16))), hi: Rc::new(RefCell::new(Some(%d as u16))), name: \"x, y\".to_string(), ..Default::default() }", i, i*2))
	}
	code := "fn init() {\n    let t = vec![" + strings.Join(elems, ", ") + "];\n}\n" + strings.Repeat("// padding\n", 400)
	got := compactHomogeneousVecLiterals(code, true)
	for _, want := range []string{
		"const __GO_TABLE: [(i128, i128); 20] = [(0x0000, 0), (0x0001, 2),",
		"__GO_TABLE.iter().map(|&(__go_c0, __go_c1)| Range16 { lo: Rc::new(RefCell::new(Some(__go_c0 as u16))), hi: Rc::new(RefCell::new(Some(__go_c1 as u16))), name: \"x, y\".to_string(), ..Default::default() }).collect();",
		"GoSlice::from(__go_rows)",
	} {
		if !strings.Contains(got, want) {
			t.Errorf("missing %q:\n%s", want, got[:min(len(got), 2000)])
		}
	}
	if strings.Contains(got, "vec![") {
		t.Errorf("the vec! is still there:\n%s", got[:min(len(got), 2000)])
	}
}

func TestCompactHomogeneousVecLiteralsLeavesOtherLiteralsAlone(t *testing.T) {
	padding := strings.Repeat("// padding\n", 400)
	var short []string
	for i := 0; i < 5; i++ {
		short = append(short, fmt.Sprintf("Some(%d as u16)", i))
	}
	var mixed []string
	for i := 0; i < 20; i++ {
		if i%2 == 0 {
			mixed = append(mixed, fmt.Sprintf("A { x: %d as i32 }", i))
		} else {
			mixed = append(mixed, fmt.Sprintf("B { x: %d as i32 }", i))
		}
	}
	var uncast []string
	for i := 0; i < 20; i++ {
		uncast = append(uncast, fmt.Sprintf("Some(%d)", i))
	}
	for _, list := range [][]string{short, mixed, uncast} {
		code := "let v = vec![" + strings.Join(list, ", ") + "];\n" + padding
		if got := compactHomogeneousVecLiterals(code, true); got != code {
			t.Errorf("rewrote a vec! that does not qualify:\n%s", got[:min(len(got), 600)])
		}
	}
	// std::vec! is not the GoSlice macro, and a "vec![" inside a string is
	// not a macro at all.
	var elems []string
	for i := 0; i < 20; i++ {
		elems = append(elems, fmt.Sprintf("Some(%d as u16)", i))
	}
	code := "let a = std::vec![" + strings.Join(elems, ", ") + "]; let s = \"vec![1 as u8]\";\n" + padding
	if got := compactHomogeneousVecLiterals(code, true); got != code {
		t.Errorf("rewrote std::vec! or a string:\n%s", got[:min(len(got), 600)])
	}
}
