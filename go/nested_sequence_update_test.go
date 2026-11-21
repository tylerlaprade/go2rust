package main

import (
	"strings"
	"testing"
)

func TestNestedSliceElementUpdatesLockTheOuterSequence(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func update(a [][]int, words []string) {
	a[0][1] += 10
	a[1][0]++
	a[1][0], a[1][1] = a[1][1], a[1][0]
	words[0] += "x"
}
`)

	for _, want := range []string{
		"let __outer_idx = (0) as usize; let __idx = (1) as usize; let __rhs = 10; let __seq = { let __go_slice_guard = a.borrow(); __go_slice_guard.as_ref().unwrap().get_cloned(__outer_idx).clone() }; let __current = __seq.get_cloned(__idx); __seq.set(__idx, __current + __rhs);",
		"let __seq = { let __go_slice_guard = a.borrow(); __go_slice_guard.as_ref().unwrap().get_cloned(__outer_idx).clone() }; let __current = __seq.get_cloned(__idx); __seq.set(__idx, __current + 1);",
		"__go_lhs_sequence_0.borrow_mut().as_mut().unwrap().get_cloned(__go_lhs_outer_index_0_0).set(__go_lhs_index_0, __tmp_0);",
		"let __current = __seq.get_cloned(__idx); __seq.set(__idx, __current + &__rhs);",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("nested slice element update should lock the outer slice and index down, missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "[(0) as usize].clone().borrow_mut()") || strings.Contains(rust, "[(1) as usize].clone().clone();") {
		t.Fatalf("nested slice element update should not lock a copy of the inner slice:\n%s", rust)
	}
}

func TestDeeplyNestedSliceAssignmentIndexesEveryLevel(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func set(cube [][][]int) {
	cube[0][1][2] = 3
}
`)

	if !strings.Contains(rust, "cube.borrow().as_ref().unwrap().get_cloned(__outer_idx).get_cloned(__outer_idx_1).set(__idx, __rhs);") {
		t.Fatalf("three-level slice assignment should index every level under one lock:\n%s", rust)
	}
}

func TestNestedArrayZeroValueSpellsItsLength(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func board() [3][3]int {
	var grid [3][3]int
	grid[1][1] += 4
	return grid
}
`)

	if !strings.Contains(rust, "std::array::from_fn::<_, 3, _>(|_| std::array::from_fn::<_, 3, _>(|_| 0))") {
		t.Fatalf("a nested zero array should spell out both lengths:\n%s", rust)
	}
}

func TestMethodCallArgumentsThatReadTheReceiverAreEvaluatedFirst(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Counter struct{ n int }

func (c *Counter) Add(x int) { c.n += x }

func use() int {
	c := Counter{1}
	c.Add(c.n)
	c.Add(2)
	return c.n
}
`)

	if !strings.Contains(rust, "{ let __method_args = (") || !strings.Contains(rust, ",); (*c.borrow_mut().as_mut().unwrap()).add(__method_args.0) }") {
		t.Fatalf("an argument that reads the receiver should be evaluated before the receiver is borrowed:\n%s", rust)
	}
	if strings.Count(rust, "__method_args") != 2 {
		t.Fatalf("only the call whose argument reads the receiver should be rewritten:\n%s", rust)
	}
}

func TestMethodCallArgumentsReadingTheReceiverInGoroutineProgram(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Counter struct{ n int }

func (c Counter) Plus(x int) int { return c.n + x }

func use() int {
	go func() {}()
	c := Counter{1}
	return c.Plus(c.n)
}
`)

	if !strings.Contains(rust, ",); (*c.lock().unwrap().as_ref().unwrap()).plus(__method_args.0) }") {
		t.Fatalf("in a program with goroutines a value-receiver call should not hold the lock while its arguments lock the receiver:\n%s", rust)
	}
}

func TestMultilineInterfaceSliceLiteralSeparatesEveryElement(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Shape interface{ Area() float64 }

type Rect struct{ W, H float64 }

func (r Rect) Area() float64 { return r.W * r.H }

func square(s float64) Rect { return Rect{s, s} }

func use() []Shape {
	go func() {}()
	return []Shape{Rect{1, 2}, square(2), Rect{3, 4}, square(5)}
}
`)

	if strings.Contains(rust, ")))        Arc::new(") {
		t.Fatalf("elements of a multiline interface slice literal should be separated by commas:\n%s", rust)
	}
	if strings.Count(rust, " as Box<dyn Shape + Send + Sync>))),\n") != 4 {
		t.Fatalf("every element of a multiline interface slice literal should end its line with a comma:\n%s", rust)
	}
}

func TestStringElementCompoundAssignOnMapAppends(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func tag(m map[string]string, score map[string]float64) {
	m["k"] += "w"
	score["a"]++
}
`)

	if !strings.Contains(rust, "Some((*__value.as_ref().unwrap()).clone() + &__rhs)") {
		t.Fatalf("string map value += should append a &str:\n%s", rust)
	}
	if !strings.Contains(rust, "Some((*__value.as_ref().unwrap()).clone() + 1.0)") {
		t.Fatalf("float map value ++ should add 1.0:\n%s", rust)
	}
}
