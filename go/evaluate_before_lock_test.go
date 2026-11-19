package main

import (
	"strings"
	"testing"
)

// A statement that writes to a slice or a map may read the same slice or map
// in its index, its value, or what it appends. The generated code used to
// take the write lock first and evaluate those inside it, so the read
// panicked ("RefCell already mutably borrowed"), or hung in a program with
// goroutines. "s[len(s)-1] = x", "s[0] = s[1]" and "s = append(s, s[0])" all
// failed that way.
func TestStatementReadsItsTargetBeforeLockingIt(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func main() {
	s := []int{1, 2, 3}
	s[len(s)-1] = s[0]
	n := []int{1, 2}
	n = append(n, n[0])
	n = append(n, n[0], len(n))
	n = append(n, n...)
	m := map[string]int{"a": 1}
	m["a"] += m["a"]
	m["a"]++
	byKey := map[string][]int{"a": {1}}
	byKey["a"] = append(byKey["a"], byKey["a"][0])
	_ = s
}
`)

	for what, want := range map[string]string{
		"s[len(s)-1] = s[0]":          "{ let __idx = (((*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize; let __rhs = (*s.borrow().as_ref().unwrap())[(0) as usize].clone(); (*s.borrow_mut().as_mut().unwrap())[__idx] = __rhs; }",
		"append(n, n[0])":             "{ let __append_target = n.clone(); let __append_value = (*n.borrow().as_ref().unwrap())[(0) as usize].clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }",
		"append(n, n[0], len(n))":     "let __append_values = vec![(*n.borrow().as_ref().unwrap())[(0) as usize].clone(), (*n.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32]; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(__append_values);",
		"append(n, n...)":             ".iter().cloned().collect(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(__append_values);",
		`m["a"] += m["a"]`:            "{ let __key = \"a\".to_string(); let __rhs = { let __map_holder = m.clone();",
		`m["a"] += m["a"], the write`: "let __entry = __map.entry(__key).or_insert_with(|| Rc::new(RefCell::new(Some(0)))); let mut __value = __entry.borrow_mut(); * __value = Some((*__value.as_ref().unwrap()).clone() + __rhs); }",
		`m["a"]++`:                    "{ let __key = \"a\".to_string(); let mut __map_guard = m.borrow_mut();",
		"append to a slice in a map":  ".borrow().as_ref().unwrap())[(0) as usize].clone(); let __slice = { let __map_holder = byKey.clone();",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s: missing %q", what, want)
		}
	}

	// No write lock may be open while an index, a value or an appended value
	// is still being written out.
	for _, held := range []string{
		".borrow_mut().as_mut().unwrap())[(",
		".get_or_insert_with(Vec::new).push((",
		".get_or_insert_with(Vec::new).extend(vec![",
		".get_or_insert_with(Vec::new).extend({",
	} {
		if strings.Contains(rust, held) {
			t.Errorf("a value is evaluated while the target is locked: found %q", held)
		}
	}
	if t.Failed() {
		t.Logf("generated Rust:\n%s", rust)
	}
}

// The same for the other element writes: an increment, a compound
// assignment, an assignment of several values at once, and an element of a
// slice of slices. The first two also cast the whole index, where
// "s[len(s)-1]++" used to cast only the 1 and did not compile.
func TestElementWritesEvaluateTheirIndexBeforeLocking(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func main() {
	s := []int{1, 2, 3}
	s[len(s)-1]++
	s[len(s)-1] += s[0]
	s[len(s)-1], s[0] = s[0], s[len(s)-1]
	grid := [][]int{{1, 2}, {3, 4}}
	grid[len(grid)-1][0] = grid[0][1]
}
`)
	const lastIndex = "(((*s.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize"
	for what, want := range map[string]string{
		"s[len(s)-1]++":                         "{ let __idx = " + lastIndex + "; let mut __seq_guard = s.borrow_mut(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + 1; }",
		"s[len(s)-1] += s[0]":                   "{ let __idx = " + lastIndex + "; let __rhs = (*s.borrow().as_ref().unwrap())[(0) as usize].clone(); let mut __seq_guard = s.borrow_mut();",
		"s[len(s)-1], s[0] = s[0], s[len(s)-1]": "{ let __idx = " + lastIndex + "; (*s.borrow_mut().as_mut().unwrap())[__idx] = __tmp_0; }",
		"grid[len(grid)-1][0] = grid[0][1]":     "let __idx = (0) as usize; let __rhs = (*grid.borrow().as_ref().unwrap())[(0) as usize].clone()[(1) as usize].clone(); (*grid.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; }",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s: missing %q", what, want)
		}
	}
	if strings.Contains(rust, "- (1 as i32) as usize") {
		t.Errorf("an index is cast in part only: found %q", "- (1 as i32) as usize")
	}
	if t.Failed() {
		t.Logf("generated Rust:\n%s", rust)
	}
}
