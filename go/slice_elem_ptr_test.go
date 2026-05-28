package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"strings"
	"testing"
)

func TestDeclaredSliceElemPointerUsesTypedAddress(t *testing.T) {
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
	assertDeclaredSliceElemPointerUsesTypedAddress(t, transpileTypedSliceElemPtrRegression(t, src))
}

func assertDeclaredSliceElemPointerUsesTypedAddress(t *testing.T, rust string) {
	t.Helper()
	if strings.Contains(rust, "CompositeLit with nil Type") || strings.Contains(rust, "unimplemented!()") {
		t.Fatalf("typed slice element pointer should not require syntax fallback:\n%s", rust)
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

func TestNoTypeInfoSliceElemAddressRequiresTypeInfo(t *testing.T) {
	rust := transpileNoTypeInfoRegression(t, `package main

func update() {
	nums := []int{1, 2}
	_ = &nums[0]
}
`)

	if !strings.Contains(rust, "Type information required for slice element address") {
		t.Fatalf("slice element address without type information should fail loudly:\n%s", rust)
	}
	if strings.Contains(rust, "GoSliceElemPtr::new") {
		t.Fatalf("slice element address without type information must not guess a pointer representation:\n%s", rust)
	}
}

func TestArrayElemAddressDoesNotUseSliceElemPtr(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func update() {
	nums := [2]int{1, 2}
	_ = &nums[0]
}
`)

	if strings.Contains(rust, "GoSliceElemPtr::new") {
		t.Fatalf("array element address should not use the slice element pointer helper:\n%s", rust)
	}
	if !strings.Contains(rust, "array element address requires pointer support") {
		t.Fatalf("array element address should fail loudly until array element pointers are supported:\n%s", rust)
	}
}

func TestShortDeclSliceElemPointerSelectorBorrowsElement(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type lineInfo struct {
	Filename string
	Offset int
}

func read(infos []lineInfo, i int) (string, int) {
	alt := &infos[i]
	return alt.Filename, alt.Offset
}
`)

	if strings.Contains(rust, "alt.lock()") {
		t.Fatalf("slice element pointer selector should not use normal pointer wrapper locks:\n%s", rust)
	}
	if !strings.Contains(rust, "alt.as_ref().unwrap().borrow().as_ref().unwrap()).filename.clone()") {
		t.Fatalf("slice element pointer string selector should borrow the element before cloning the field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "(*(*alt.as_ref().unwrap().borrow().as_ref().unwrap()).offset.borrow().as_ref().unwrap())") {
		t.Fatalf("slice element pointer int selector should borrow the element before unwrapping the field value:\n%s", rust)
	}
}

func TestSliceElemPointerDerefNilStoresPointerHandle(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type File struct {
	Base int
}

func clear(files []*File) {
	last := &files[len(files)-1]
	*last = nil
}
`)

	if strings.Contains(rust, "let new_val = None; *last.as_ref().unwrap().borrow_mut() = Some(new_val)") {
		t.Fatalf("nil assignment through a pointer-valued slice element should not store raw None:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = Rc::new(RefCell::new(None))") {
		t.Fatalf("nil assignment through a pointer-valued slice element should store a nil pointer handle:\n%s", rust)
	}
}

func TestSliceElemPointerStructFieldInitializerFailsLoudly(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Name struct {
	Bytes *byte
}

func makeName() Name {
	b := []byte{1}
	return Name{Bytes: &b[0]}
}
`)

	if strings.Contains(rust, "bytes: GoSliceElemPtr::new") {
		t.Fatalf("slice element pointer field initializer should not emit an incompatible helper value:\n%s", rust)
	}
	if !strings.Contains(rust, `unimplemented!("slice element pointer cannot initialize pointer field")`) {
		t.Fatalf("slice element pointer field initializer should fail loudly:\n%s", rust)
	}
}

func TestSliceElemPointerReturnFailsLoudlyInsteadOfInvalidHelper(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type entry struct {
	value int
}

func pick(bucket []entry) *entry {
	return &bucket[len(bucket)-1]
}
`)

	if strings.Contains(rust, "return GoSliceElemPtr::new") {
		t.Fatalf("slice element pointer return should not emit an incompatible helper value:\n%s", rust)
	}
	if !strings.Contains(rust, `unimplemented!("slice element pointer return requires pointer representation support")`) {
		t.Fatalf("slice element pointer return should fail loudly:\n%s", rust)
	}
}

func transpileTypedSliceElemPtrRegression(t *testing.T, src string) string {
	t.Helper()

	prevTypeInfo := currentTypeInfo
	prevContext := currentContext
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
		currentContext = prevContext
	})

	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", src, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	rust, _, _ := Transpile(file, fset, typeInfo)
	return rust
}
