package main

import (
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
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

func TestSliceElemPointerToSourceMappedInterfaceElementUsesCollectionElementHandle(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

import "go/types"

type reader struct {
	typs []types.Type
}

func lookup(r *reader, i int) types.Type {
	var where *types.Type
	where = &r.typs[i]
	if typ := *where; typ != nil {
		return typ
	}
	var typ types.Type
	*where = typ
	return typ
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile() error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	rust, _, _ := TranspileWithMapping(file, fset, typeInfo, map[string]string{"go/types": "go_types"})
	if strings.Contains(rust, "GoSliceElemPtr<Box<dyn go_types::Type") {
		t.Fatalf("slice element pointer to source-mapped interface should not use the bare trait object element type:\n%s", rust)
	}
	if !strings.Contains(rust, "Option<GoSliceElemPtr<Rc<RefCell<Option<Box<dyn go_types::Type") {
		t.Fatalf("slice element pointer to source-mapped interface should use the collection element handle:\n%s", rust)
	}
	if strings.Contains(rust, "typ.borrow().as_ref().unwrap()).borrow") {
		t.Fatalf("slice element pointer dereference should not treat the bare trait object as a wrapped handle:\n%s", rust)
	}
	if strings.Contains(rust, "let new_val = (*typ.borrow().as_ref().unwrap())") {
		t.Fatalf("slice element pointer assignment should store the interface handle, not the boxed payload:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = typ.clone()") {
		t.Fatalf("slice element pointer assignment should clone the interface handle into the slice element:\n%s", rust)
	}
}

func TestNamedSliceSliceElemAddressUsesInnerHandle(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Word uint
type nat []Word

func alias(x nat) {
	p := &x[0:cap(x)][cap(x)-1]
	_ = p
}
`)

	if strings.Contains(rust, "GoSliceElemPtr::new(nat(") {
		t.Fatalf("slice element pointer into named-slice slice expression should not use the bare named value as the sequence handle:\n%s", rust)
	}
	if !strings.Contains(rust, "__named_slice.0.clone()") {
		t.Fatalf("slice element pointer into named-slice slice expression should use the named slice inner handle:\n%s", rust)
	}
}

func TestSliceElemPointerEqualityComparesHandleAndIndex(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Word uint
type nat []Word

func alias(x, y nat) bool {
	return cap(x) > 0 && cap(y) > 0 && &x[0:cap(x)][cap(x)-1] == &y[0:cap(y)][cap(y)-1]
}
`)

	if strings.Contains(rust, "__left.borrow") || strings.Contains(rust, "__left.lock") {
		t.Fatalf("slice element pointer equality should not treat GoSliceElemPtr as a wrapped pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "::ptr_eq(&__left.slice, &__right.slice)") || !strings.Contains(rust, "__left.index == __right.index") {
		t.Fatalf("slice element pointer equality should compare the sequence handle and index:\n%s", rust)
	}
}

func TestArrayElemAddressUsesArrayElemPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func update() int {
	nums := [2]int{1, 2}
	p := &nums[0]
	*p = 7
	return nums[0]
}
`)

	if strings.Contains(rust, "GoSliceElemPtr::new") {
		t.Fatalf("array element address should not use the slice element pointer helper:\n%s", rust)
	}
	if strings.Contains(rust, "array element address requires pointer support") {
		t.Fatalf("array element address should use array element pointer support:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut p: Option<GoArrayElemPtr<i32, 2>> = Some(GoArrayElemPtr::new(nums.clone(), (0) as usize))") {
		t.Fatalf("array element pointer short declaration should preserve array identity and index:\n%s", rust)
	}
	if !strings.Contains(rust, "*p.as_ref().unwrap().borrow_mut() = Some(new_val)") {
		t.Fatalf("array element pointer dereference assignment should write back through the array helper:\n%s", rust)
	}
}

func TestArrayElemAddressShortDeclUsesArrayElemPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func update() {
	var recent [2][4]uint64
	cache := &recent[0]
	_ = cache
	_ = *cache
}
`)

	if strings.Contains(rust, "GoSliceElemPtr::new") {
		t.Fatalf("array element address short declaration should not use slice element pointer helpers:\n%s", rust)
	}
	if strings.Contains(rust, "cache.borrow()") {
		t.Fatalf("array element pointer variable should not use normal wrapped-value borrows:\n%s", rust)
	}
	if strings.Contains(rust, "array element address requires pointer support") {
		t.Fatalf("array element address short declaration should use array element pointer support:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut cache: Option<GoArrayElemPtr<[u64; 4], 2>> = Some(GoArrayElemPtr::new(recent.clone(), (0) as usize))") {
		t.Fatalf("array element address short declaration should preserve array length and element type:\n%s", rust)
	}
	if !strings.Contains(rust, "cache.as_ref().unwrap().borrow()") {
		t.Fatalf("array element pointer dereference read should borrow through the array helper:\n%s", rust)
	}
}

func TestArrayElemPointerToArraySupportsLenIndexRangeAndElemAddress(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func load(slot *uint64) uint64 {
	return *slot
}

func update(ch uint64) uint64 {
	var recent [2][4]uint64
	cache := &recent[uint(ch)%uint(len(recent))]
	var total uint64
	for i := 0; i < len(cache); i++ {
		total += load(&cache[i])
	}
	for _, x := range cache {
		total += x
	}
	return total
}
`)

	if strings.Contains(rust, "GoArrayElemPtr::new(cache.clone()") {
		t.Fatalf("address of an element through a pointer-to-array should not treat the pointer helper as an array handle:\n%s", rust)
	}
	if strings.Contains(rust, "cache.borrow()") || strings.Contains(rust, "cache.lock()") {
		t.Fatalf("pointer-to-array operations should borrow through GoArrayElemPtr, not normal wrapper handles:\n%s", rust)
	}
	if !strings.Contains(rust, "< (4 as i32)") {
		t.Fatalf("len(cache) should use the typed pointer-to-array length:\n%s", rust)
	}
	if !strings.Contains(rust, "load(Rc::new(RefCell::new({ let __seq = cache.as_ref().unwrap().borrow(); Some(__seq.as_ref().unwrap()[") {
		t.Fatalf("read-only &cache[i] call argument should borrow the pointed-to array element:\n%s", rust)
	}
	if !strings.Contains(rust, "let __range_values = { let __seq = cache.as_ref().unwrap().borrow(); __seq.as_ref().unwrap().clone() }; for x in __range_values.iter().copied()") {
		t.Fatalf("range over cache should materialize the pointed-to array through GoArrayElemPtr:\n%s", rust)
	}
}

func TestDeclaredArrayElemPointerVarUsesArrayElemPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type child struct {
	value int
}

func (c *child) Load() int {
	return c.value
}

func (c *child) Store(next child) {
	c.value = next.value
}

type indirect struct {
	children [16]child
}

func find(i *indirect, hash uint, shift uint) int {
	var slot *child
	for shift != 0 {
		slot = &i.children[(hash>>shift)&15]
		return slot.Load()
	}
	return 0
}
`)

	if !strings.Contains(rust, "let mut slot: Option<GoArrayElemPtr<child, 16>> = None") {
		t.Fatalf("declared array element pointer variable should use the array element pointer helper:\n%s", rust)
	}
	if !strings.Contains(rust, "slot = Some(GoArrayElemPtr::new(") {
		t.Fatalf("array element pointer assignment should preserve array identity and index:\n%s", rust)
	}
	if !strings.Contains(rust, "slot.as_ref().unwrap().borrow().as_ref().unwrap()).load(") {
		t.Fatalf("array element pointer method call should borrow through the array helper:\n%s", rust)
	}
}

func TestArrayElemPointerMutatingMethodCallUsesMutableBorrow(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func storeInt(dst *int, value int) {
	*dst = value
}

type child struct {
	value int
}

func (c *child) Store(next child) {
	storeInt(&c.value, next.value)
}

type indirect struct {
	children [16]child
}

func update(i *indirect, hash uint, shift uint, next child) {
	var slot *child
	for shift != 0 {
		slot = &i.children[(hash>>shift)&15]
		slot.Store(next)
		return
	}
}
`)

	if strings.Contains(rust, "slot.as_ref().unwrap().borrow().as_ref().unwrap()).store(") {
		t.Fatalf("mutating array element pointer method call should not borrow a cloned element immutably:\n%s", rust)
	}
	if !strings.Contains(rust, "slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).store(") {
		t.Fatalf("mutating array element pointer method call should write back through a mutable array helper borrow:\n%s", rust)
	}
}

func TestNamedReturnArrayElemPointerUsesArrayElemPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type child struct {
	value int
}

func (c *child) Load() int {
	return c.value
}

func (c *child) Store(next child) {
	c.value = next.value
}

type indirect struct {
	children [16]child
}

type box[T any] struct {
	root *indirect
}

func find(i *indirect, hash uint, shift uint) (slot *child, value int) {
	for shift != 0 {
		slot = &i.children[(hash>>shift)&15]
		value = slot.Load()
		return
	}
	return
}

func find4(i *indirect, hash uint, shift uint) (node *indirect, hashShift uint, slot *child, value int) {
	for shift != 0 {
		node = i
		hashShift = shift
		slot = &i.children[(hash>>shift)&15]
		value = slot.Load()
		return
	}
	return
}

func (b *box[T]) find4(hash uint, shift uint) (node *indirect, hashShift uint, slot *child, value int) {
	for shift != 0 {
		node = b.root
		hashShift = shift
		slot = &b.root.children[(hash>>shift)&15]
		value = slot.Load()
		return
	}
	return
}

func use(i *indirect, hash uint, shift uint) int {
	slot, _ := find(i, hash, shift)
	return slot.Load()
}

func pair() (child, bool) {
	return child{}, true
}

func useAfterMixedTuple(i *indirect, hash uint, shift uint) (swapped bool) {
	slot, _ := find(i, hash, shift)
	e, swapped := pair()
	if !swapped {
		return false
	}
	slot.Store(e)
	return true
}

func useAfterMixedTupleWithBlank(i *indirect, hash uint, shift uint) (swapped bool) {
	go func() {}()
	_, _, slot, _ := find4(i, hash, shift)
	if i != nil {
		defer slot.Load()
	}
	e, swapped := pair()
	if !swapped {
		return false
	}
	slot.Store(e)
	return true
}

func (b *box[T]) useAfterGenericMethodTuple(hash uint, shift uint) (swapped bool) {
	go func() {}()
	_, _, slot, _ := b.find4(hash, shift)
	e, swapped := pair()
	if !swapped {
		return false
	}
	slot.Store(e)
	return true
}
`)

	if !strings.Contains(rust, "-> (Option<GoArrayElemPtr<child, 16>>, i32)") {
		t.Fatalf("named array element pointer result should use the array element pointer helper in the signature:\n%s", rust)
	}
	if !strings.Contains(rust, "Option<GoArrayElemPtr<child, 16>>, i32)") {
		t.Fatalf("third named array element pointer result should use the array element pointer helper in the signature:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut slot: Option<GoArrayElemPtr<child, 16>> = None") {
		t.Fatalf("named array element pointer result should initialize to a nil array element pointer:\n%s", rust)
	}
	if !strings.Contains(rust, "slot = Some(GoArrayElemPtr::new(") {
		t.Fatalf("named array element pointer assignment should preserve array identity and index:\n%s", rust)
	}
	if !strings.Contains(rust, "slot.as_ref().unwrap().borrow().as_ref().unwrap()).load(") {
		t.Fatalf("named array element pointer method call should borrow through the array helper:\n%s", rust)
	}
	if strings.Contains(rust, "let __recv = slot.clone(); let __recv_ptr") {
		t.Fatalf("array element pointer call result should not use normal pointer receiver lowering:\n%s", rust)
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

func TestSliceElemPointerMetadataDoesNotLeakPastIfBody(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type method struct {
	Typ int
	Mtyp int
}

func read(methods []method, i int, ok bool) int {
	if ok {
		m := &methods[i]
		return m.Typ
	}
	m := methods[i]
	return m.Mtyp
}
`)

	if strings.Contains(rust, "m.as_ref().unwrap().borrow().as_ref().unwrap()).mtyp") {
		t.Fatalf("slice element pointer metadata should not leak past the if body:\n%s", rust)
	}
	if !strings.Contains(rust, "(*m.borrow().as_ref().unwrap()).mtyp") {
		t.Fatalf("post-if value local should use normal wrapped struct field access:\n%s", rust)
	}
}

func TestSliceElemPointerMethodCallBorrowsElement(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type field struct {
	Embedded bool
}

func (f *field) IsEmbedded() bool {
	return f.Embedded
}

func read(fields []field, i int) bool {
	p := &fields[i]
	return p.IsEmbedded()
}
`)

	if strings.Contains(rust, "p.borrow()") || strings.Contains(rust, "p.lock()") {
		t.Fatalf("slice element pointer method call should not use normal pointer wrapper locks:\n%s", rust)
	}
	if !strings.Contains(rust, "(*p.as_ref().unwrap().borrow().as_ref().unwrap()).is_embedded()") {
		t.Fatalf("slice element pointer method call should borrow the element receiver:\n%s", rust)
	}
}

func TestSliceElemPointerPointerFieldAssignmentMutatesElement(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type typ struct{}

type runtimeSelect struct {
	typ *typ
}

func assign(cases []runtimeSelect, i int, typ *typ) {
	rc := &cases[i]
	rc.typ = typ
}
`)

	if strings.Contains(rust, "(*rc.borrow_mut().as_mut().unwrap()).typ") ||
		strings.Contains(rust, "(*rc.lock().unwrap().as_mut().unwrap()).typ") {
		t.Fatalf("slice element pointer field assignment should not treat the pointer variable as a wrapped slot:\n%s", rust)
	}
	if !strings.Contains(rust, "(*rc.as_ref().unwrap().borrow_mut().as_mut().unwrap()).typ = new_val") {
		t.Fatalf("slice element pointer field assignment should mutate the borrowed element field:\n%s", rust)
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
	if strings.Contains(rust, "let __dst = last.clone()") {
		t.Fatalf("nil assignment through a slice element pointer should use GoSliceElemPtr directly, not pointer-slot lowering:\n%s", rust)
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

func TestSliceElemPointerFieldAssignmentFailsLoudly(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Name struct {
	Bytes *byte
}

func setName(n *Name, b []byte) {
	n.Bytes = &b[0]
}
`)

	if strings.Contains(rust, "bytes = GoSliceElemPtr::new") {
		t.Fatalf("slice element pointer field assignment should not emit an incompatible helper value:\n%s", rust)
	}
	if !strings.Contains(rust, `unimplemented!("slice element pointer cannot assign to pointer field")`) {
		t.Fatalf("slice element pointer field assignment should fail loudly:\n%s", rust)
	}
}

func TestSliceElemPointerLocalFieldAssignmentFailsLoudly(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Name struct {
	Bytes *byte
}

func setName(n *Name, b []byte) {
	p := &b[0]
	n.Bytes = p
}
`)

	if strings.Contains(rust, "let new_val = p.clone();") {
		t.Fatalf("slice element pointer local field assignment should not emit an incompatible helper value:\n%s", rust)
	}
	if !strings.Contains(rust, `unimplemented!("slice element pointer cannot assign to pointer field")`) {
		t.Fatalf("slice element pointer local field assignment should fail loudly:\n%s", rust)
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
		t.Fatalf("direct slice element pointer return should fail loudly until generic pointer fields can hold it:\n%s", rust)
	}
}

func TestSliceElemPointerReturnVariableFlowsThroughCall(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type inst struct {
	op  int
	out int
}

type prog struct {
	inst []inst
}

func (p *prog) skip(pc int) *inst {
	i := &p.inst[pc]
	for i.op == 0 {
		i = &p.inst[i.out]
	}
	return i
}

func use(p *prog) int {
	i := p.skip(0)
	i = p.skip(i.out)
	return i.out
}
`)

	if !strings.Contains(rust, "fn skip(&self, pc: Rc<RefCell<Option<i32>>>) -> Option<GoSliceElemPtr<inst>>") &&
		!strings.Contains(rust, "fn skip(&self, pc: Arc<Mutex<Option<i32>>>) -> Option<GoSliceElemPtr<inst>>") {
		t.Fatalf("slice element pointer method should expose the slice element pointer representation:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut i: Option<GoSliceElemPtr<inst>> =") {
		t.Fatalf("short declaration from a slice element pointer return should register the local representation:\n%s", rust)
	}
	if !strings.Contains(rust, "return i.clone()") {
		t.Fatalf("returning a slice element pointer local should preserve the handle:\n%s", rust)
	}
	if !strings.Contains(rust, "i = (*p.") || !strings.Contains(rust, ".skip(") || strings.Contains(rust, "slice element pointer assignment") {
		t.Fatalf("assignment from a slice element pointer returning call should preserve the handle:\n%s", rust)
	}
	if strings.Contains(rust, "i.lock()") {
		t.Fatalf("call result stored as a slice element pointer should not be treated as a wrapped pointer slot:\n%s", rust)
	}
}

func TestSliceElemPointerIfInitFromReturnKeepsMetadata(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type inst struct {
	out int
}

func find(p []inst) *inst {
	i := &p[0]
	return i
}

func dump(i *inst) int {
	return i.out
}

func use(p []inst) int {
	if i := find(p); i != nil {
		return dump(i)
	}
	return 0
}
`)

	if !strings.Contains(rust, "let mut i: Option<GoSliceElemPtr<inst>> = find(") {
		t.Fatalf("if-init slice element pointer return should keep GoSliceElemPtr local metadata:\n%s", rust)
	}
	if strings.Contains(rust, "i.lock()") || strings.Contains(rust, "(*i.borrow()).is_some()") || strings.Contains(rust, "dump(i.clone())") {
		t.Fatalf("if-init slice element pointer local should not be treated as a wrapped pointer slot:\n%s", rust)
	}
	if !strings.Contains(rust, "if i.is_some()") {
		t.Fatalf("if-init slice element pointer nil check should inspect the option directly:\n%s", rust)
	}
	if !strings.Contains(rust, "dump(Rc::new(RefCell::new((*i.as_ref().unwrap().borrow()).clone())))") &&
		!strings.Contains(rust, "dump(Arc::new(Mutex::new((*i.as_ref().unwrap().borrow()).clone())))") {
		t.Fatalf("if-init slice element pointer local should pass a cloned pointee to read-only pointer params:\n%s", rust)
	}
}

func TestReadOnlyPointerParamAcceptsSliceElemPointerLocal(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type inst struct {
	out int
}

func dump(i *inst) int {
	return i.out
}

func use(p []inst) int {
	i := &p[0]
	return dump(i)
}
`)

	if strings.Contains(rust, "dump(i.clone())") {
		t.Fatalf("read-only pointer parameter should not receive the slice element pointer option directly:\n%s", rust)
	}
	if !strings.Contains(rust, "dump(Rc::new(RefCell::new((*i.as_ref().unwrap().borrow()).clone())))") &&
		!strings.Contains(rust, "dump(Arc::new(Mutex::new((*i.as_ref().unwrap().borrow()).clone())))") {
		t.Fatalf("read-only pointer parameter should receive a cloned pointee handle:\n%s", rust)
	}
}

func TestReadOnlyPointerParamAcceptsSliceElemAddress(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func dump(values *[]int) int {
	return len(*values)
}

func use(chunks [][]int) int {
	return dump(&chunks[0])
}
`)

	if strings.Contains(rust, "dump(GoSliceElemPtr::new") {
		t.Fatalf("read-only pointer parameter should not receive a direct slice element pointer helper:\n%s", rust)
	}
	if !strings.Contains(rust, "dump(Rc::new(RefCell::new((*GoSliceElemPtr::new(chunks.clone(), (0) as usize).borrow()).clone())))") &&
		!strings.Contains(rust, "dump(Arc::new(Mutex::new((*GoSliceElemPtr::new(chunks.clone(), (0) as usize).borrow()).clone())))") {
		t.Fatalf("read-only pointer parameter should receive a cloned direct slice element pointee handle:\n%s", rust)
	}
}

func TestReadOnlyPointerParamAcceptsRangeSliceElemAddress(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func take(p *byte) byte {
	return *p
}

func use(chunks [][]byte) {
	for _, chunk := range chunks {
		if len(chunk) == 0 {
			continue
		}
		_ = take(&chunk[0])
	}
}
`)

	if strings.Contains(rust, "GoSliceElemPtr::new(chunk.clone()") {
		t.Fatalf("range slice element pointer should not pass a bare Vec where a slice handle is required:\n%s", rust)
	}
	if !strings.Contains(rust, "GoSliceElemPtr::new(Rc::new(RefCell::new(Some((*chunk).clone()))), (0) as usize)") &&
		!strings.Contains(rust, "GoSliceElemPtr::new(Arc::new(Mutex::new(Some((*chunk).clone()))), (0) as usize)") {
		t.Fatalf("range slice element pointer should wrap the range slice value in a temporary handle:\n%s", rust)
	}
}

func TestWritablePointerParamRejectsSliceElemAddressLoudly(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func mutate(p *byte) {
	*p = 1
}

func use(buf []byte) {
	mutate(&buf[0])
}
`)

	if strings.Contains(rust, "mutate(GoSliceElemPtr::new") {
		t.Fatalf("writable pointer parameter should not receive an incompatible slice element pointer helper:\n%s", rust)
	}
	if !strings.Contains(rust, `unimplemented!("slice element pointer cannot pass to writable pointer parameter")`) {
		t.Fatalf("writable pointer parameter should fail loudly until pointer params can hold slice element identity:\n%s", rust)
	}
}

func TestReadOnlyPointerParamThroughFuncLitAcceptsSliceElemAddress(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func dump(values *[]int) int {
	read := func(p *[]int) int {
		return len(*p)
	}
	return read(values)
}

func use(chunks [][]int) int {
	return dump(&chunks[0])
}
`)

	if strings.Contains(rust, "dump(GoSliceElemPtr::new") {
		t.Fatalf("read-only pointer parameter passed through a function literal should not receive a direct helper:\n%s", rust)
	}
	if !strings.Contains(rust, "dump(Rc::new(RefCell::new((*GoSliceElemPtr::new(chunks.clone(), (0) as usize).borrow()).clone())))") &&
		!strings.Contains(rust, "dump(Arc::new(Mutex::new((*GoSliceElemPtr::new(chunks.clone(), (0) as usize).borrow()).clone())))") {
		t.Fatalf("read-only pointer parameter passed through a function literal should receive a cloned pointee handle:\n%s", rust)
	}
}

func TestReadOnlyMethodPointerParamAcceptsSliceElemPointerLocal(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type inst struct {
	out int
}

type machine struct{}
type thread struct {
	inst *inst
}

func (m *machine) alloc(i *inst) *thread {
	t := new(thread)
	t.inst = i
	return t
}

func use(m *machine, p []inst) *thread {
	i := &p[0]
	return m.alloc(i)
}
`)

	if strings.Contains(rust, ".alloc(i.clone())") {
		t.Fatalf("read-only method pointer parameter should not receive the slice element pointer option directly:\n%s", rust)
	}
	if !strings.Contains(rust, ".alloc(Rc::new(RefCell::new((*i.as_ref().unwrap().borrow()).clone())))") &&
		!strings.Contains(rust, ".alloc(Arc::new(Mutex::new((*i.as_ref().unwrap().borrow()).clone())))") {
		t.Fatalf("read-only method pointer parameter should receive a cloned pointee handle:\n%s", rust)
	}
}

func TestParallelAssignmentUpdatesSliceElemPointerLocal(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type inst struct {
	out int
}

func walk(p []inst, start int) int {
	i := &p[start]
	pc := start
	pc, i = i.out, &p[i.out]
	return pc + i.out
}
`)

	if strings.Contains(rust, "*i.borrow") || strings.Contains(rust, "*i.lock") {
		t.Fatalf("parallel assignment to slice element pointer local should not treat it as a wrapped pointer slot:\n%s", rust)
	}
	if !strings.Contains(rust, "i = Some(__tmp_1);") {
		t.Fatalf("parallel assignment should store the new slice element pointer option directly:\n%s", rust)
	}
}

func TestFuncLiteralSliceElemPointerLocalUsesMetadata(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type inst struct {
	out int
}

func run(p []inst) int {
	check := func(pc int) int {
		inst := &p[pc]
		return inst.out
	}
	return check(0)
}
`)

	if strings.Contains(rust, "inst.lock()") || strings.Contains(rust, "inst.borrow()") {
		t.Fatalf("function literal slice element pointer local should not be treated as a normal pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "inst: Option<GoSliceElemPtr<inst>>") {
		t.Fatalf("function literal slice element pointer local should be registered with GoSliceElemPtr metadata:\n%s", rust)
	}
	if !strings.Contains(rust, "inst.as_ref().unwrap().borrow().as_ref().unwrap()).out") {
		t.Fatalf("function literal slice element pointer selector should borrow the element:\n%s", rust)
	}
}

func TestSliceElemPointerMapKeyUsesSliceIdentity(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type cache struct {
	active map[*byte][]byte
}

func remember(c *cache, data []byte) []byte {
	p := &data[len(data)-1]
	c.active[p] = data
	got := c.active[p]
	delete(c.active, p)
	return got
}
`)

	if strings.Contains(rust, "GoLocalPtrKey::new(p.clone())") {
		t.Fatalf("slice element pointer map key should not use local pointer handle identity:\n%s", rust)
	}
	if !strings.Contains(rust, "GoLocalPtrKey::from_slice_elem(p.clone())") {
		t.Fatalf("slice element pointer map key should preserve backing slice identity and index:\n%s", rust)
	}
}

func TestPointerSliceStoresSliceElemPointerSlots(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func collect(parts []string) []*byte {
	out := make([]*byte, len(parts)+2)
	first := new(byte)
	buf := make([]byte, 4)
	out[0] = first
	out[1] = &buf[1]
	return out
}
`)

	if strings.Contains(rust, "Vec<Rc<RefCell<Option<u8>>>>") ||
		strings.Contains(rust, "Vec<Arc<Mutex<Option<u8>>>>") {
		t.Fatalf("pointer slice storing slice element addresses should not use local pointer handles:\n%s", rust)
	}
	if !strings.Contains(rust, "Vec<GoPtr<u8>>") {
		t.Fatalf("pointer slice storing slice element addresses should use pointer slots that preserve slice identity:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::local(first.clone())") {
		t.Fatalf("pointer slice assignment should preserve ordinary pointer handles:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::slice_elem(GoSliceElemPtr::new(buf.clone(), (1) as usize))") {
		t.Fatalf("pointer slice assignment should preserve backing slice identity and index:\n%s", rust)
	}
}

func TestPointerSliceFromSpecializedReturnKeepsSlotRepresentation(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func collect() []*byte {
	out := make([]*byte, 1)
	buf := make([]byte, 1)
	out[0] = &buf[0]
	return out
}

func use() []*byte {
	p := new(byte)
	out := collect()
	out[0] = p
	return out
}
`)

	if strings.Contains(rust, "] = p.clone()") {
		t.Fatalf("pointer slice returned with slice element slots should not receive ordinary pointer handles directly:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::local(p.clone())") {
		t.Fatalf("pointer slice returned with slice element slots should wrap ordinary pointer handles:\n%s", rust)
	}
}

func TestPointerSliceSpecializedReturnPropagatesToParam(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func collect() []*byte {
	out := make([]*byte, 1)
	buf := make([]byte, 1)
	out[0] = &buf[0]
	return out
}

func consume(items []*byte) {
}

func use() {
	items := collect()
	consume(items)
}
`)

	if strings.Contains(rust, "fn consume(items: Rc<RefCell<Option<Vec<Rc<RefCell<Option<u8") ||
		strings.Contains(rust, "fn consume(items: Arc<Mutex<Option<Vec<Arc<Mutex<Option<u8") {
		t.Fatalf("callee receiving a specialized pointer slice should not keep ordinary pointer-slice params:\n%s", rust)
	}
	if !strings.Contains(rust, "fn consume(items: Rc<RefCell<Option<Vec<GoPtr<u8>>>>>") &&
		!strings.Contains(rust, "fn consume(items: Arc<Mutex<Option<Vec<GoPtr<u8>>>>>") {
		t.Fatalf("callee receiving a specialized pointer slice should use GoPtr slots:\n%s", rust)
	}
}

func transpileTypedSliceElemPtrRegression(t *testing.T, src string) string {
	t.Helper()

	prevTypeInfo := currentTypeInfo
	prevContext := currentContext
	prevSourceFunctionDecls := sourceFunctionDeclsByFunc
	prevSourceFunctionReadOnlyCache := sourceFunctionReadOnlyParamCache
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
		currentContext = prevContext
		sourceFunctionDeclsByFunc = prevSourceFunctionDecls
		sourceFunctionReadOnlyParamCache = prevSourceFunctionReadOnlyCache
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
	sourceDecls := make(map[*types.Func]sourceFunctionDeclInfo)
	for _, decl := range file.Decls {
		fnDecl, ok := decl.(*ast.FuncDecl)
		if !ok {
			continue
		}
		fn, ok := typeInfo.info.Defs[fnDecl.Name].(*types.Func)
		if !ok || fn == nil {
			continue
		}
		sourceDecls[fn] = sourceFunctionDeclInfo{decl: fnDecl, info: typeInfo.info}
	}
	SetSourceFunctionDeclsByFunc(sourceDecls)
	rust, _, _ := Transpile(file, fset, typeInfo)
	return rust
}
