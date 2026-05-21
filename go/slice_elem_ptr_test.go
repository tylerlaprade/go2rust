package main

import (
	"strings"
	"testing"
)

func TestNoTypeInfoDeclaredSliceElemPointerUsesSyntax(t *testing.T) {
	src := `package main

type entry struct {
	key int
	value string
}

func update() {
	bucket := []entry{{key: 1, value: "old"}}
	var hole *entry
	for i, e := range bucket {
		if e.key == 1 {
			hole = &bucket[i]
		}
	}
	if hole != nil {
		*hole = entry{key: 1, value: "new"}
	}
}
`
	assertDeclaredSliceElemPointerUsesSyntax(t, transpileNoTypeInfoRegression(t, src))
	assertDeclaredSliceElemPointerUsesSyntax(t, transpileRegression(t, src, &TypeInfo{}))
}

func assertDeclaredSliceElemPointerUsesSyntax(t *testing.T, rust string) {
	t.Helper()
	if strings.Contains(rust, "CompositeLit with nil Type") || strings.Contains(rust, "unimplemented!()") {
		t.Fatalf("elided struct slice literal should use outer syntax type:\n%s", rust)
	}
	if !strings.Contains(rust, "vec![entry {") {
		t.Fatalf("elided struct slice literal should emit entry values:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut hole: Option<GoSliceElemPtr<entry>> = None") {
		t.Fatalf("slice element pointer variable should use GoSliceElemPtr option:\n%s", rust)
	}
	if !strings.Contains(rust, "hole = Some(GoSliceElemPtr::new(bucket.clone(), (i) as usize))") {
		t.Fatalf("slice element address assignment should preserve slice/index identity:\n%s", rust)
	}
	if !strings.Contains(rust, "*hole.as_ref().unwrap().borrow_mut() = Some(new_val)") {
		t.Fatalf("dereference assignment should write through GoSliceElemPtr:\n%s", rust)
	}
}
