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

func TestGoPtrArrayPointerFieldSlotsPreserveHandle(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type node struct {
	value int
}

type cache struct {
	buf [2]*node
	len int
}

func fill(c *cache, items []node) {
	var p *node
	p = &items[0]
	c.buf[0] = p
	c.len = 1
}

func pop(c *cache) *node {
	s := c.buf[c.len-1]
	c.len--
	return s
}

func use(c *cache, items []node) *node {
	fill(c, items)
	p := pop(c)
	return p
}
`)

	if !strings.Contains(rust, "pub buf: Rc<RefCell<Option<[GoPtr<node>; 2]>>>") &&
		!strings.Contains(rust, "pub buf: Arc<Mutex<Option<[GoPtr<node>; 2]>>>") {
		t.Fatalf("pointer array field assigned a GoPtr value should store GoPtr slots:\n%s", rust)
	}
	if strings.Contains(rust, "pub buf: Rc<RefCell<Option<[Rc<RefCell<Option<node>>>; 2]>>>") ||
		strings.Contains(rust, "pub buf: Arc<Mutex<Option<[Arc<Mutex<Option<node>>>; 2]>>>") {
		t.Fatalf("pointer array field should not use ordinary pointer wrappers once GoPtr slot identity is proven:\n%s", rust)
	}
	if !strings.Contains(rust, "std::array::from_fn(|_| GoPtr::nil())") {
		t.Fatalf("GoPtr pointer array field default should initialize nil slots:\n%s", rust)
	}
	if !strings.Contains(rust, "(*(*c.borrow().as_ref().unwrap()).buf.borrow_mut().as_mut().unwrap())[(0) as usize] = GoPtr::slice_elem_opt(p.clone())") &&
		!strings.Contains(rust, "(*(*c.lock().unwrap().as_ref().unwrap()).buf.lock().unwrap().as_mut().unwrap())[(0) as usize] = GoPtr::slice_elem_opt(p.clone())") {
		t.Fatalf("assignment into a GoPtr pointer array slot should convert the slice-element handle:\n%s", rust)
	}
	if !strings.Contains(rust, "fn pop(c: Rc<RefCell<Option<cache>>>) -> GoPtr<node>") &&
		!strings.Contains(rust, "fn pop(c: Arc<Mutex<Option<cache>>>) -> GoPtr<node>") {
		t.Fatalf("return from a GoPtr pointer array slot should use GoPtr result type:\n%s", rust)
	}
	if strings.Contains(rust, "return GoPtr::local(s.clone())") {
		t.Fatalf("return from a GoPtr pointer array slot should not rewrap the handle:\n%s", rust)
	}
}

func TestGoPtrArrayPointerAnonymousStructFieldSlotsPreserveHandle(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type node struct {
	value int
}

func fill(items []node) struct {
	len int
	buf [2]*node
} {
	var c struct {
		len int
		buf [2]*node
	}
	p := &items[0]
	c.buf[0] = p
	c.len = 1
	return c
}
`)

	if !strings.Contains(rust, "pub buf: Rc<RefCell<Option<[GoPtr<node>; 2]>>>") &&
		!strings.Contains(rust, "pub buf: Arc<Mutex<Option<[GoPtr<node>; 2]>>>") {
		t.Fatalf("anonymous pointer array field assigned a GoPtr value should store GoPtr slots:\n%s", rust)
	}
	if strings.Contains(rust, "pub buf: Rc<RefCell<Option<[Rc<RefCell<Option<node>>>; 2]>>>") ||
		strings.Contains(rust, "pub buf: Arc<Mutex<Option<[Arc<Mutex<Option<node>>>; 2]>>>") {
		t.Fatalf("anonymous pointer array field should not use ordinary pointer wrappers once GoPtr slot identity is proven:\n%s", rust)
	}
	if !strings.Contains(rust, "std::array::from_fn(|_| GoPtr::nil())") {
		t.Fatalf("anonymous GoPtr pointer array field default should initialize nil slots:\n%s", rust)
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
	if strings.Contains(rust, "array element address through pointer-to-array requires nested pointer representation") {
		t.Fatalf("address of an element through a pointer-to-array should not use the old unimplemented path:\n%s", rust)
	}
	if !strings.Contains(rust, "load(GoPtr::array_elem(GoArrayElemPtr::from_array_elem(cache.as_ref().unwrap().clone(),") {
		t.Fatalf("address of an element through a pointer-to-array should preserve pointer identity:\n%s", rust)
	}
	if !strings.Contains(rust, "let __range_values = { let __seq = cache.as_ref().unwrap().borrow(); __seq.as_ref().unwrap().clone() }; for x in __range_values.iter().copied()") {
		t.Fatalf("range over cache should materialize the pointed-to array through GoArrayElemPtr:\n%s", rust)
	}
}

func TestNestedArrayElemAddressWritesBackThroughOuterArray(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type cache struct {
	entries [2][4]uint64
}

func update(c *cache, ck int, i int) uint64 {
	ent := &c.entries[ck][i]
	*ent = 7
	return c.entries[ck][i]
}
`)

	if strings.Contains(rust, "let __seq =") && strings.Contains(rust, "GoArrayElemPtr::new({") {
		t.Fatalf("nested array element address should not build a pointer from a cloned inner array:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut ent: Option<GoArrayElemPtr<u64, 4>>") {
		t.Fatalf("nested array element address should keep the inner array pointer type:\n%s", rust)
	}
	if !strings.Contains(rust, "GoArrayElemPtr::nested(") {
		t.Fatalf("nested array element address should use a nested array pointer helper:\n%s", rust)
	}
	if !strings.Contains(rust, "*ent.as_ref().unwrap().borrow_mut() = Some(new_val)") {
		t.Fatalf("nested array element pointer field assignment should write back through the helper:\n%s", rust)
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

func TestArrayElemPointerReceiverBeatsGoPtrCandidate(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Pointer[T any] struct {
	value *T
}

func (p *Pointer[T]) Load() *T {
	return p.value
}

type node struct {
	value int
}

type indirect struct {
	children [16]Pointer[node]
}

func find(i *indirect, hash uint, shift uint) *node {
	var slot *Pointer[node]
	for shift != 0 {
		slot = &i.children[(hash>>shift)&15]
		return slot.Load()
	}
	return nil
}
`)

	if !strings.Contains(rust, "let mut slot: Option<GoArrayElemPtr<Pointer<node>, 16>> = None") {
		t.Fatalf("declared array element pointer variable should use the array element pointer helper:\n%s", rust)
	}
	if strings.Contains(rust, "slot.with_mut(") || strings.Contains(rust, "slot.borrow()") {
		t.Fatalf("array element pointer receiver should not be handled by the broader GoPtr receiver path:\n%s", rust)
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

func TestUnnamedReturnArrayElemPointerUsesArrayElemPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func pick(values *[8]byte, i int) *byte {
	if i > 0 {
		return &(*values)[7]
	}
	return &(*values)[0]
}

func use(values *[8]byte) byte {
	p := pick(values, 0)
	return *p
}
`)

	if !strings.Contains(rust, "fn pick(values: Rc<RefCell<Option<[u8; 8]>>>, i: Rc<RefCell<Option<i32>>>) -> Option<GoArrayElemPtr<u8, 8>>") &&
		!strings.Contains(rust, "fn pick(values: Arc<Mutex<Option<[u8; 8]>>>, i: Arc<Mutex<Option<i32>>>) -> Option<GoArrayElemPtr<u8, 8>>") {
		t.Fatalf("unnamed array element pointer result should use the array element pointer helper in the signature:\n%s", rust)
	}
	if !strings.Contains(rust, "return Some(GoArrayElemPtr::new(values.clone(), (7) as usize));") ||
		!strings.Contains(rust, "Some(GoArrayElemPtr::new(values.clone(), (0) as usize))") {
		t.Fatalf("direct array element pointer returns should preserve array identity and index:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut p") || !strings.Contains(rust, "= pick(") || !strings.Contains(rust, "p.as_ref().unwrap().borrow()") {
		t.Fatalf("short declaration from an unnamed array element pointer return should register the local representation:\n%s", rust)
	}
	if strings.Contains(rust, "p.lock()") || strings.Contains(rust, "p.borrow()") {
		t.Fatalf("array element pointer call result should not use normal pointer wrapper borrows:\n%s", rust)
	}
}

func TestUnnamedMethodReturnArrayElemPointerSelectorBorrowsElement(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type delta struct {
	committed *int64
}

type holder struct {
	stats [3]delta
}

func (h *holder) acquire() *delta {
	return &h.stats[0]
}

func use(h *holder) *int64 {
	stats := h.acquire()
	return stats.committed
}
`)

	if !strings.Contains(rust, "fn acquire(&self) -> Option<GoArrayElemPtr<delta, 3>>") {
		t.Fatalf("unnamed method array element pointer result should use the array element pointer helper in the signature:\n%s", rust)
	}
	if strings.Contains(rust, "(*stats.lock()") || strings.Contains(rust, "(*stats.borrow()") {
		t.Fatalf("selector on array element pointer method result should not use normal pointer wrapper borrows:\n%s", rust)
	}
	if !strings.Contains(rust, "stats.as_ref().unwrap().borrow().as_ref().unwrap()).committed.clone()") {
		t.Fatalf("selector on array element pointer method result should borrow through the array helper:\n%s", rust)
	}
}

func TestUnnamedMethodReturnArrayElemPointerCallResultReceiverBorrowsElement(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type delta struct {
	committed *int64
}

func (d *delta) get() *int64 {
	return d.committed
}

type holder struct {
	stats [3]delta
}

func (h *holder) acquire() *delta {
	return &h.stats[0]
}

func use(h *holder) *int64 {
	return h.acquire().get()
}
`)

	if !strings.Contains(rust, "fn acquire(&self) -> Option<GoArrayElemPtr<delta, 3>>") {
		t.Fatalf("unnamed method array element pointer result should use the array element pointer helper in the signature:\n%s", rust)
	}
	if strings.Contains(rust, "(*__recv.lock()") || strings.Contains(rust, "(*__recv.borrow()") {
		t.Fatalf("method call on array element pointer call result should not use normal pointer wrapper borrows:\n%s", rust)
	}
	if !strings.Contains(rust, "__recv.as_ref().unwrap().borrow().as_ref().unwrap()).get(") {
		t.Fatalf("method call on array element pointer call result should borrow through the array helper:\n%s", rust)
	}
}

func TestArrayElemPointerCallResultFieldMethodBorrowsElement(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type bits struct {
	words [1]uint64
}

func (b *bits) setRange(i, n uint) {
	b.words[0] = uint64(i + n)
}

type chunk struct {
	scavenged bits
}

type holder struct {
	chunks [3]chunk
}

func (h *holder) acquire() *chunk {
	return &h.chunks[0]
}

func use(h *holder) {
	h.acquire().scavenged.setRange(1, 2)
}
`)

	if strings.Contains(rust, "acquire().lock()") || strings.Contains(rust, "acquire().borrow()") {
		t.Fatalf("field method on array element pointer call result should not use normal wrapper borrows:\n%s", rust)
	}
	if !strings.Contains(rust, "__recv.as_ref().unwrap().borrow().as_ref().unwrap()).scavenged") {
		t.Fatalf("field method on array element pointer call result should borrow the returned element before selecting the field:\n%s", rust)
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

func TestSliceElemPointerIfInitFieldAssignmentMutatesElement(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type expr interface {
	exprNode()
}

type ident struct{}

func (*ident) exprNode() {}

type field struct {
	name *ident
	typ  expr
}

func propagate(list []field) {
	var typ expr
	for i := len(list) - 1; i >= 0; i-- {
		if par := &list[i]; par.typ != nil {
			typ = par.typ
		} else if typ != nil {
			par.typ = typ
		}
	}
}
`)

	if !strings.Contains(rust, "let mut par: Option<GoSliceElemPtr<field>> = Some(GoSliceElemPtr::new(list.clone(),") {
		t.Fatalf("if-init slice element pointer should preserve backing slice identity:\n%s", rust)
	}
	if strings.Contains(rust, "par.lock()") || strings.Contains(rust, "par.borrow()") {
		t.Fatalf("if-init slice element pointer should not be treated as a wrapped pointer slot:\n%s", rust)
	}
	if !strings.Contains(rust, "*(*par.as_ref().unwrap().borrow_mut().as_mut().unwrap()).typ.borrow_mut() = (*__iface_guard).clone()") {
		t.Fatalf("if-init slice element pointer field assignment should mutate the element:\n%s", rust)
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

func TestSliceElemPointerStructFieldInitializerUsesGoPtrField(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Name struct {
	Bytes *byte
}

func makeName() Name {
	b := []byte{1}
	return Name{Bytes: &b[0]}
}
`)

	if !strings.Contains(rust, "pub bytes: GoPtr<u8>") {
		t.Fatalf("slice element pointer field initializer should promote the field to GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "bytes: GoPtr::slice_elem(") {
		t.Fatalf("slice element pointer field initializer should store a GoPtr slice element handle:\n%s", rust)
	}
	if strings.Contains(rust, `unimplemented!("slice element pointer cannot initialize pointer field")`) {
		t.Fatalf("slice element pointer field initializer should no longer fail loudly once the field is GoPtr-backed:\n%s", rust)
	}
}

func TestGoPtrCurrentReceiverFieldDerefAndUnsafePointerUseGoPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Name struct {
	Bytes *byte
}

func makeName(b []byte) Name {
	return Name{Bytes: &b[0]}
}

func forceConcurrent(ch chan bool) {
	go func() {
		ch <- true
	}()
}

func (n Name) flag() bool {
	return (*n.Bytes & 1) != 0
}

func (n Name) addr() uintptr {
	return uintptr(unsafe.Pointer(n.Bytes))
}
`)

	if strings.Contains(rust, "self.bytes.lock()") || strings.Contains(rust, "Arc::as_ptr(&self.bytes") {
		t.Fatalf("current receiver GoPtr field reads should not use wrapper lock/as_ptr paths:\n%s", rust)
	}
	if !strings.Contains(rust, "self.bytes.borrow()") {
		t.Fatalf("dereferencing a current receiver GoPtr field should borrow through GoPtr:\n%s", rust)
	}
	if !strings.Contains(rust, "self.bytes.addr()") {
		t.Fatalf("unsafe.Pointer conversion of a current receiver GoPtr field should use GoPtr::addr:\n%s", rust)
	}
}

func TestSliceElemPointerFieldAssignmentUsesGoPtrField(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Name struct {
	Bytes *byte
}

func setName(n *Name, b []byte) {
	n.Bytes = &b[0]
}
`)

	if !strings.Contains(rust, "pub bytes: GoPtr<u8>") {
		t.Fatalf("slice element pointer field should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::slice_elem(GoSliceElemPtr::new(b.clone(), (0) as usize))") {
		t.Fatalf("slice element pointer field assignment should preserve backing slice identity:\n%s", rust)
	}
}

func TestSliceElemPointerLocalFieldAssignmentUsesGoPtrField(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Name struct {
	Bytes *byte
}

func setName(n *Name, b []byte) {
	p := &b[0]
	n.Bytes = p
}
`)

	if !strings.Contains(rust, "pub bytes: GoPtr<u8>") {
		t.Fatalf("slice element pointer local field should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::slice_elem_opt(p.clone())") {
		t.Fatalf("slice element pointer local field assignment should preserve backing slice identity:\n%s", rust)
	}
}

func TestSliceElemPointerReturnAssignedFieldUsesGoPtrField(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Info struct {
	Opaque bool
}

var All = []Info{{Opaque: true}}

func Lookup(name string) *Info {
	return &All[0]
}

type setting struct {
	info *Info
}

func load(s *setting, name string) bool {
	s.info = Lookup(name)
	return s.info != nil && s.info.Opaque
}
`)

	if !strings.Contains(rust, "pub info: GoPtr<Info>") {
		t.Fatalf("slice element pointer return field should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::slice_elem_opt(lookup(") {
		t.Fatalf("slice element pointer return assignment should preserve backing slice identity:\n%s", rust)
	}
	if !strings.Contains(rust, ".info.is_nil()") {
		t.Fatalf("slice element pointer field nil comparison should use GoPtr nil state:\n%s", rust)
	}
	if !strings.Contains(rust, ".info.borrow()") {
		t.Fatalf("slice element pointer field read should borrow through GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, `unimplemented!("slice element pointer cannot assign to pointer field")`) {
		t.Fatalf("slice element pointer return field assignment should not fail loudly once GoPtr storage is selected:\n%s", rust)
	}
}

func TestGoPtrFieldReturnedFromFunctionUsesGoPtrResult(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Type struct {
	Data *byte
}

func set(t *Type, buf []byte) {
	t.Data = &buf[0]
}

func get(t *Type) *byte {
	return t.Data
}
`)

	if !strings.Contains(rust, "pub data: GoPtr<u8>") {
		t.Fatalf("pointer field assigned a slice element address should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn get(") || !strings.Contains(rust, " -> GoPtr<u8>") {
		t.Fatalf("function returning a GoPtr field should use a GoPtr result type:\n%s", rust)
	}
	if !strings.Contains(rust, ".data.clone()") {
		t.Fatalf("returning a GoPtr field should clone the field handle:\n%s", rust)
	}
}

func TestAnonymousNestedPointerFieldAssignedSliceElemAddressUsesGoPtrField(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Hint struct {
	next *Hint
}

type Heap struct {
	arenaHints *Hint
	userArena struct {
		arenaHints *Hint
	}
}

var hints = []Hint{{}}

func initHeap(h *Heap) {
	h.arenaHints = &hints[0]
	h.userArena.arenaHints = &hints[0]
}

func choose(h *Heap, useMain bool) {
	hintList := &h.userArena.arenaHints
	if useMain {
		hintList = &h.arenaHints
	}
	_ = hintList
}
`)

	if count := strings.Count(rust, "pub arena_hints: GoPtr<Hint>"); count < 2 {
		t.Fatalf("both named and anonymous pointer fields assigned slice element addresses should use GoPtr storage, found %d:\n%s", count, rust)
	}
	if strings.Contains(rust, "pub arena_hints: Rc<RefCell<Option<Hint>>>") ||
		strings.Contains(rust, "pub arena_hints: Arc<Mutex<Option<Hint>>>") {
		t.Fatalf("anonymous pointer field assigned a slice element address should not keep the old pointer wrapper:\n%s", rust)
	}
}

func TestGoPtrReturnMergesFieldAndUnsafeRawPointer(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Type struct {
	Data *byte
}

func set(t *Type, buf []byte) {
	t.Data = &buf[0]
}

func raw(addr uintptr) *byte {
	return (*byte)(unsafe.Pointer(addr))
}

func get(t *Type, addr uintptr, flag bool) *byte {
	if flag {
		return raw(addr)
	}
	return t.Data
}
`)

	if !strings.Contains(rust, "pub data: GoPtr<u8>") {
		t.Fatalf("pointer field assigned a slice element address should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn raw(") || !strings.Contains(rust, " -> GoPtr<u8>") {
		t.Fatalf("function returning an unsafe raw pointer should use GoPtr result type:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::raw(") {
		t.Fatalf("unsafe raw pointer conversion should produce a GoPtr raw address:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn get(") || !strings.Contains(rust, " -> GoPtr<u8>") {
		t.Fatalf("function merging raw pointer and GoPtr field returns should use GoPtr result type:\n%s", rust)
	}
	if strings.Contains(rust, " -> Rc<RefCell<Option<u8>>>") ||
		strings.Contains(rust, " -> Arc<Mutex<Option<u8>>>") {
		t.Fatalf("mixed GoPtr/raw pointer returns should not keep the old pointer wrapper result:\n%s", rust)
	}
}

func TestGoPtrReturnPropagatesThroughMultiResultCallReturn(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct {
	value int
}

func raw(addr uintptr) *node {
	return (*node)(unsafe.Pointer(addr))
}

func slow(addr uintptr) (*node, int) {
	return raw(addr), 1
}

func pick(addr uintptr, fast bool) (*node, int) {
	if fast {
		return raw(addr), 0
	}
	return slow(addr)
}
`)

	if !strings.Contains(rust, "pub fn slow(") || !strings.Contains(rust, " -> (GoPtr<node>, i32)") {
		t.Fatalf("callee returning raw pointer plus scalar should use GoPtr result type:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn pick(") || !strings.Contains(rust, " -> (GoPtr<node>, i32)") {
		t.Fatalf("multi-result call return should propagate GoPtr result type to caller:\n%s", rust)
	}
	if strings.Contains(rust, "pub fn pick(") && strings.Contains(rust, " -> (Rc<RefCell<Option<node>>>, i32)") {
		t.Fatalf("multi-result call return should not leave caller with ordinary pointer wrapper:\n%s", rust)
	}
}

func TestGoPtrLocalPromotedFieldAssignmentBorrowsPointee(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type poolLocalInternal struct {
	private any
}

type local struct {
	poolLocalInternal
}

type pool struct {
	addr uintptr
}

func (p *pool) pick() (*local, int) {
	addr := p.addr
	return (*local)(unsafe.Pointer(addr)), 0
}

func (p *pool) put(value any) {
	go func() {}()
	l, _ := p.pick()
	l.private = value
}
`)

	if strings.Contains(rust, "l.pool_local_internal") {
		t.Fatalf("promoted field assignment through a GoPtr local should not access the GoPtr as a struct:\n%s", rust)
	}
	if !strings.Contains(rust, "l.with_mut(|__ptr_value|") ||
		!strings.Contains(rust, "(*__ptr_value.pool_local_internal") {
		t.Fatalf("promoted field assignment through a GoPtr local should mutate the pointee before selecting fields:\n%s", rust)
	}
}

func TestGoPtrCallResultDerefReadUsesGoPtrBorrow(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Name struct {
	Bytes *byte
}

func set(n *Name, buf []byte) {
	n.Bytes = &buf[0]
}

func add(p unsafe.Pointer, off uintptr) unsafe.Pointer {
	return p
}

func (n Name) Data(off int) *byte {
	return (*byte)(add(unsafe.Pointer(n.Bytes), uintptr(off)))
}

func read(n Name) byte {
	return *n.Data(2)
}
`)

	if !strings.Contains(rust, "pub fn data(") || !strings.Contains(rust, " -> GoPtr<u8>") {
		t.Fatalf("method returning an unsafe raw pointer should use GoPtr result type:\n%s", rust)
	}
	if strings.Contains(rust, ".data(Rc::new(RefCell::new(Some(2)))).borrow().as_ref().unwrap()") ||
		strings.Contains(rust, ".data(Arc::new(Mutex::new(Some(2)))).lock().unwrap().as_ref().unwrap()") {
		t.Fatalf("dereferencing a GoPtr call result should not treat it as the old pointer wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "let __ptr_handle =") || !strings.Contains(rust, "__ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone()") {
		t.Fatalf("dereferencing a GoPtr call result should borrow through GoPtr:\n%s", rust)
	}
}

func TestGoPtrCallResultMethodCallBorrowsThroughGoPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type bits struct {
	value int
}

func (b *bits) get(i int) int {
	return b.value + i
}

func (b *bits) set(i int) {
	b.value = i
}

type heap struct {
	marks *bits
}

func initHeap(h *heap, all []bits) {
	h.marks = &all[0]
}

func (h *heap) current() *bits {
	return h.marks
}

func read(h *heap) int {
	return h.current().get(3)
}

func write(h *heap) {
	h.current().set(4)
}

func readLocal(h *heap) int {
	current := h.current()
	return current.get(5)
}

func writeLocal(h *heap) {
	current := h.current()
	current.set(6)
}

func forceConcurrent(ch chan bool) {
	go func() {
		ch <- true
	}()
}
`)

	if !strings.Contains(rust, "pub fn current(") || !strings.Contains(rust, " -> GoPtr<bits>") {
		t.Fatalf("method returning a GoPtr field should return GoPtr<bits>:\n%s", rust)
	}
	if strings.Contains(rust, "__recv.lock()") || strings.Contains(rust, "__recv.borrow_mut()") {
		t.Fatalf("method call on GoPtr call result should not use wrapper borrows directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let __recv_value = __recv.borrow(); let __result = (*__recv_value.as_ref().unwrap()).get(") {
		t.Fatalf("read-only method call on GoPtr call result should borrow through GoPtr:\n%s", rust)
	}
	if !strings.Contains(rust, "let __result = __recv.with_mut(|__recv_value| __recv_value.set(") {
		t.Fatalf("mutating method call on GoPtr call result should dispatch through with_mut:\n%s", rust)
	}
	if strings.Contains(rust, "current.lock()") || strings.Contains(rust, "current.borrow_mut()") {
		t.Fatalf("method call on local copied from GoPtr call result should not use wrapper borrows directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let __recv_value = current.borrow(); let __result = (*__recv_value.as_ref().unwrap()).get(") {
		t.Fatalf("read-only method call on local copied from GoPtr call result should borrow through GoPtr:\n%s", rust)
	}
	if !strings.Contains(rust, "let __result = current.with_mut(|__recv_value| __recv_value.set(") {
		t.Fatalf("mutating method call on local copied from GoPtr call result should dispatch through with_mut:\n%s", rust)
	}
}

func TestGoPtrCallResultFieldMethodCallBorrowsThroughGoPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type counter struct {
	value int
}

func (c *counter) get() int {
	return c.value
}

type p struct {
	counter counter
}

var sink p

func getp() *p {
	return (*p)(unsafe.Pointer(&sink))
}

func read() int {
	return getp().counter.get()
}

func forceConcurrent(ch chan bool) {
	go func() {
		ch <- true
	}()
}
`)

	if !strings.Contains(rust, "pub fn getp() -> GoPtr<p>") {
		t.Fatalf("unsafe pointer result should use GoPtr result type:\n%s", rust)
	}
	if strings.Contains(rust, "getp().lock().unwrap().as_ref().unwrap()).counter") ||
		strings.Contains(rust, "getp().borrow().as_ref().unwrap()).counter") {
		t.Fatalf("field method call on GoPtr call result should not use wrapper borrows directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let __ptr = getp(); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().counter.clone()") {
		t.Fatalf("field method call on GoPtr call result should borrow through GoPtr before selecting the field:\n%s", rust)
	}
}

func TestGoPtrCallResultNamedScalarValueMethodBorrowsThroughGoPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type word uint64

func (w word) mask(off uint64) uint64 {
	return uint64(w) + off
}

type holder struct {
	current *word
}

func initHolder(h *holder, all []word) {
	h.current = &all[0]
}

func read(h *holder) uint64 {
	return h.controls().mask(3)
}

func readLocal(h *holder) uint64 {
	controls := h.controls()
	return controls.mask(4)
}

func (h *holder) controls() *word {
	return h.current
}

func forceConcurrent(ch chan bool) {
	go func() {
		ch <- true
	}()
}
`)

	if !strings.Contains(rust, "pub fn controls(") || !strings.Contains(rust, " -> GoPtr<word>") {
		t.Fatalf("method returning a GoPtr named scalar should return GoPtr<word>:\n%s", rust)
	}
	if strings.Contains(rust, ".controls().lock()") || strings.Contains(rust, ".controls().borrow()") {
		t.Fatalf("named scalar value method on GoPtr call result should not borrow the returned handle as a wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "let __recv = (*h.") ||
		!strings.Contains(rust, ".controls(); let __recv_value = __recv.borrow(); __recv_value.as_ref().unwrap().clone()") {
		t.Fatalf("named scalar value method on GoPtr call result should borrow through GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "controls.lock()") || strings.Contains(rust, "controls.borrow().as_ref().unwrap()") {
		t.Fatalf("named scalar value method on local copied from GoPtr call result should not use wrapper borrows directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let __recv_value = controls.borrow(); __recv_value.as_ref().unwrap().clone()") {
		t.Fatalf("named scalar value method on local copied from GoPtr call result should borrow through GoPtr:\n%s", rust)
	}
}

func TestGoPtrReturnMergesNilLocalAddressAndFieldPointer(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Type struct {
	Value int
}

type Holder struct {
	Data *Type
}

func set(h *Holder, values []Type) {
	h.Data = &values[0]
}

func makePtr(h *Holder, flag int) *Type {
	if flag == 0 {
		return nil
	}
	if flag == 1 {
		var typ Type
		return &typ
	}
	return h.Data
}
`)

	if !strings.Contains(rust, "pub fn make_ptr(") || !strings.Contains(rust, " -> GoPtr<Type>") {
		t.Fatalf("function merging nil, local address, and GoPtr field returns should use GoPtr result type:\n%s", rust)
	}
	if strings.Contains(rust, "return Rc::new(RefCell::new(None))") ||
		strings.Contains(rust, "return Arc::new(Mutex::new(None))") ||
		strings.Contains(rust, "Rc::new(RefCell::new(Some(typ") ||
		strings.Contains(rust, "Arc::new(Mutex::new(Some(typ") {
		t.Fatalf("GoPtr return function should not emit old pointer wrapper returns:\n%s", rust)
	}
	if !strings.Contains(rust, "return GoPtr::nil();") {
		t.Fatalf("nil return in a GoPtr function should emit GoPtr::nil():\n%s", rust)
	}
	if !strings.Contains(rust, "return GoPtr::local(") {
		t.Fatalf("local address return in a GoPtr function should emit GoPtr::local(...):\n%s", rust)
	}
}

func TestGoPtrReturnMergesEmbeddedPointerFieldAndRawPointer(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Type struct {
	Value int
}

func (t *Type) hasName() bool {
	return t.Value != 0
}

func (t *Type) setValue(v int) {
	t.Value = v
}

type rtype struct {
	*Type
}

func raw(addr uintptr) *Type {
	return (*Type)(unsafe.Pointer(addr))
}

func forceConcurrent(ch chan bool) {
	go func() {
		ch <- true
	}()
}

func common(r rtype, addr uintptr, flag bool) *Type {
	if flag {
		return raw(addr)
	}
	return r.Type
}

func (r rtype) value() int {
	return r.Value
}

func (r rtype) name() bool {
	return r.hasName()
}

func (r rtype) set(v int) {
	r.setValue(v)
}
	`)

	if !strings.Contains(rust, "pub r#type: GoPtr<Type>") {
		t.Fatalf("embedded pointer field returned through a GoPtr result should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn common(") || !strings.Contains(rust, " -> GoPtr<Type>") {
		t.Fatalf("function merging raw pointer and embedded pointer field returns should use GoPtr result type:\n%s", rust)
	}
	if strings.Contains(rust, "pub r#type: Rc<RefCell<Option<Type>>>") ||
		strings.Contains(rust, "pub r#type: Arc<Mutex<Option<Type>>>") {
		t.Fatalf("embedded pointer field should not keep ordinary pointer wrapper storage once returned as GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "GoPtr::local(r.r#type.clone())") {
		t.Fatalf("returning an embedded GoPtr field should not rewrap the field handle:\n%s", rust)
	}
	if strings.Contains(rust, "self.r#type.lock()") {
		t.Fatalf("promoted field read through an embedded GoPtr field should not lock the GoPtr as an ordinary wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.value.clone(); __field })") {
		t.Fatalf("promoted field read through an embedded GoPtr field should use the GoPtr pointee handle:\n%s", rust)
	}
	if strings.Contains(rust, "__promoted_recv.lock()") ||
		strings.Contains(rust, "__promoted_recv.borrow()") ||
		strings.Contains(rust, "__promoted_recv.borrow_mut()") {
		t.Fatalf("current receiver promoted method call through an embedded GoPtr field should not lock GoPtr as an ordinary wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "__promoted_recv.with_mut(|__promoted_ref| { __promoted_ref.has_name() })") {
		t.Fatalf("current receiver promoted method call through an embedded GoPtr field should call the GoPtr pointee:\n%s", rust)
	}
	if strings.Contains(rust, "let guard = embedded.borrow();") ||
		strings.Contains(rust, "let mut guard = embedded.borrow_mut();") ||
		strings.Contains(rust, "let mut guard = embedded.lock().unwrap();") {
		t.Fatalf("promoted method forwarder through an embedded GoPtr field should not lock GoPtr as an ordinary wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "embedded.with_mut(|embedded_ref| { embedded_ref.set_value(v) })") {
		t.Fatalf("promoted method forwarder through an embedded GoPtr field should call the GoPtr pointee:\n%s", rust)
	}
}

func TestGoPtrEmbeddedPointerResultShadowedShortDeclUsesOuterGoPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct {
	value int
}

type locked struct {
	*node
}

func raw(addr uintptr) *node {
	return (*node)(unsafe.Pointer(addr))
}

func acquire(s *node) (locked, bool) {
	return locked{s}, true
}

func consume(s *node) {
}

func use(addr uintptr) int {
	s := raw(addr)
	if s, ok := acquire(s); ok {
		consume(s.node)
		if s.value != 0 {
			s.value = 1
		}
		return s.value
	}
	return 0
}
	`)

	if !strings.Contains(rust, "pub fn acquire(s: GoPtr<node>)") {
		t.Fatalf("callee receiving the shadowed outer GoPtr local should use a GoPtr parameter:\n%s", rust)
	}
	if strings.Contains(rust, "acquire(GoPtr::local(s.clone()))") {
		t.Fatalf("short-decl RHS should pass the outer GoPtr local without rewrapping it:\n%s", rust)
	}
	if strings.Contains(rust, ".node.lock()") || strings.Contains(rust, ".node.borrow()") {
		t.Fatalf("promoted selectors through an embedded GoPtr field should not lock the GoPtr field as a wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "consume(") || !strings.Contains(rust, ".node.clone()") {
		t.Fatalf("embedded GoPtr field selected from the tuple result should be forwarded as the field handle:\n%s", rust)
	}
}

func TestGoPtrInterfaceMethodReturnUsesConcreteGoPtrResult(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Type struct {
	Value int
}

type HasCommon interface {
	common() *Type
}

type rtype struct {
	*Type
}

func raw(addr uintptr) *Type {
	return (*Type)(unsafe.Pointer(addr))
}

func forceFieldGoPtr(r rtype, addr uintptr, flag bool) *Type {
	if flag {
		return raw(addr)
	}
	return r.Type
}

func (r rtype) common() *Type {
	return r.Type
}

func use(h HasCommon) *Type {
	return h.common()
}
	`)

	if !strings.Contains(rust, "pub trait HasCommon") || !strings.Contains(rust, "fn common(&self) -> GoPtr<Type>;") {
		t.Fatalf("interface method returning a concrete GoPtr result should use GoPtr in the trait signature:\n%s", rust)
	}
	if !strings.Contains(rust, "impl HasCommon for rtype") || !strings.Contains(rust, "fn common(&self) -> GoPtr<Type>") {
		t.Fatalf("interface implementation should match the concrete GoPtr-returning method:\n%s", rust)
	}
	if strings.Contains(rust, "fn common(&self) -> Rc<RefCell<Option<Type>>>") ||
		strings.Contains(rust, "fn common(&self) -> Arc<Mutex<Option<Type>>>") {
		t.Fatalf("interface method should not keep the old pointer wrapper result when the concrete method returns GoPtr:\n%s", rust)
	}
}

func TestGoPtrLocalAssignedFromRegisteredFieldUsesGoPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct{}

type list struct {
	first *node
}

func use(l *list, n *node, flag bool) *node {
	var p *node
	p = l.first
	if flag {
		p = ordinary(n)
	}
	return p
}

func ordinary(n *node) *node {
	return n
}

func raw(n *node) *node {
	return (*node)(unsafe.Pointer(n))
}

func fill(l *list, n *node) {
	l.first = raw(n)
}
`)

	if !strings.Contains(rust, "pub first: GoPtr<node>") {
		t.Fatalf("field assigned a GoPtr value should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut p: GoPtr<node> = GoPtr::nil();") {
		t.Fatalf("local assigned from a GoPtr field should use GoPtr storage:\n%s", rust)
	}
	if strings.Contains(rust, "let mut p: Rc<RefCell<Option<node>>>") ||
		strings.Contains(rust, "let mut p: Arc<Mutex<Option<node>>>") {
		t.Fatalf("local assigned from a GoPtr field should not keep ordinary pointer wrapper storage:\n%s", rust)
	}
}

func TestGoPtrFieldAssignmentUsesParamDiscoveredFromFieldBackedReturn(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct{}

type box struct {
	p *node
	q *node
}

func raw(n *node) *node {
	return (*node)(unsafe.Pointer(n))
}

func seed(b *box, n *node) {
	b.p = raw(n)
}

func field(b *box) *node {
	return b.p
}

func assign(dst *box, src *node) {
	dst.q = src
}

func call(dst *box, src *box) {
	assign(dst, field(src))
}
`)

	if !strings.Contains(rust, "pub p: GoPtr<node>") {
		t.Fatalf("source field should use GoPtr storage before field-backed returns are collected:\n%s", rust)
	}
	if !strings.Contains(rust, "pub q: GoPtr<node>") {
		t.Fatalf("field assigned from a parameter discovered through a field-backed return should use GoPtr storage:\n%s", rust)
	}
	if strings.Contains(rust, "pub q: Rc<RefCell<Option<node>>>") ||
		strings.Contains(rust, "pub q: Arc<Mutex<Option<node>>>") {
		t.Fatalf("field assigned from a late-discovered GoPtr parameter should not keep ordinary pointer wrapper storage:\n%s", rust)
	}
}

func TestGoPtrPointerSlotDerefShortDeclUsesGoPtrHandle(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct{}

func rawSlot(addr uintptr) **node {
	return (**node)(unsafe.Pointer(addr))
}

func load(addr uintptr) *node {
	p := rawSlot(addr)
	n := *p
	return n
}
`)

	if !strings.Contains(rust, "let mut p: GoPtr<") {
		t.Fatalf("pointer-to-pointer unsafe conversion should use a GoPtr slot handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut n: GoPtr<node>") {
		t.Fatalf("dereferencing a GoPtr pointer slot should initialize a GoPtr pointee handle:\n%s", rust)
	}
	if !strings.Contains(rust, "__ptr_slot.as_ref().unwrap().clone()") {
		t.Fatalf("dereferencing a GoPtr pointer slot should preserve the stored pointer handle:\n%s", rust)
	}
	if strings.Contains(rust, "GoPtr::local(__ptr_slot.as_ref().unwrap().clone())") {
		t.Fatalf("dereferencing a GoPtr pointer slot should not rewrap the stored pointer handle:\n%s", rust)
	}
	if strings.Contains(rust, `unimplemented!("GoPtr dereference assignment should be lowered by statement assignment")`) {
		t.Fatalf("pointer slot dereference short declaration should not fall back to unimplemented lowering:\n%s", rust)
	}
}

func TestGoPtrPointerSlotDerefAssignmentStoresGoPtrHandle(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct{}

func rawSlot(addr uintptr) **node {
	return (**node)(unsafe.Pointer(addr))
}

func rawNode(addr uintptr) *node {
	return (*node)(unsafe.Pointer(addr))
}

func store(slotAddr uintptr, nodeAddr uintptr) {
	p := rawSlot(slotAddr)
	n := rawNode(nodeAddr)
	*p = n
}
`)

	if !strings.Contains(rust, "let mut p: GoPtr<GoPtr<node>>") {
		t.Fatalf("raw pointer-to-pointer unsafe conversion should use a GoPtr slot of GoPtr handles:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = n.clone(); p.assign(Some(new_val));") {
		t.Fatalf("dereference assignment through a raw pointer-to-pointer should store the GoPtr handle:\n%s", rust)
	}
	if strings.Contains(rust, "let new_val = n.clone(); p.assign(Some(Some(new_val)))") {
		t.Fatalf("dereference assignment through a raw pointer-to-pointer should not double-wrap the GoPtr handle:\n%s", rust)
	}
}

func TestGoPtrPointerSlotParamForwardingPromotesCallee(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct{}

func rawSlot(addr uintptr) **node {
	return (**node)(unsafe.Pointer(addr))
}

func rawNode(addr uintptr) *node {
	return (*node)(unsafe.Pointer(addr))
}

func set(slot **node, n *node) {
	*slot = n
	forward(slot)
}

func forward(slot **node) bool {
	return *slot != nil
}

func use(slotAddr uintptr, nodeAddr uintptr) {
	set(rawSlot(slotAddr), rawNode(nodeAddr))
}
`)

	if !strings.Contains(rust, "pub fn set(slot: GoPtr<GoPtr<node>>, n: GoPtr<node>)") {
		t.Fatalf("pointer-to-pointer setter should use a GoPtr slot parameter:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn forward(slot: GoPtr<GoPtr<node>>) -> bool") {
		t.Fatalf("forwarded pointer-to-pointer parameter should promote the callee slot parameter:\n%s", rust)
	}
	if !strings.Contains(rust, "forward(slot.clone())") {
		t.Fatalf("forwarded pointer-to-pointer slot argument should pass the GoPtr slot handle:\n%s", rust)
	}
}

func TestGenericUnsafePointerLoadKeepsTypeParamPointerWrapper(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "unsafe"

type UnsafePointer struct {
	value unsafe.Pointer
}

func (u *UnsafePointer) Load() unsafe.Pointer {
	return u.value
}

type Pointer[T any] struct {
	u UnsafePointer
}

func (p *Pointer[T]) Load() *T {
	return (*T)(p.u.Load())
}

func main() {
	go func() {}()
}
`)

	if strings.Contains(rust, "GoPtr<Box<dyn Any") {
		t.Fatalf("generic pointer load should not erase T into a GoPtr<Box<dyn Any>>:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn load(&self) -> Arc<Mutex<Option<T>>>") {
		t.Fatalf("generic pointer load should return the typed pointer wrapper:\n%s", rust)
	}
}

func TestGoPtrLocalSeededFromOrdinaryPointerThenAssignedGoPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Pointer[T any] struct {
	value *T
}

func (p *Pointer[T]) Load() *T {
	return p.value
}

type indirect struct {
	child Pointer[node]
}

type node struct {
	isEntry bool
}

func (n *node) indirect() *indirect {
	return (*indirect)(unsafe.Pointer(n))
}

type table struct {
	root Pointer[indirect]
}

func walk(t *table, flag bool) *indirect {
	i := t.root.Load()
	for flag {
		n := i.child.Load()
		if n == nil {
			return i
		}
		i = n.indirect()
		flag = false
	}
	return i
}
`)

	if !strings.Contains(rust, "pub fn load(&self) -> Rc<RefCell<Option<T>>>") {
		t.Fatalf("generic pointer load should keep the ordinary typed pointer wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn indirect(&self) -> GoPtr<indirect>") {
		t.Fatalf("unsafe pointer conversion method should still return GoPtr<indirect>:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut i: GoPtr<indirect> = GoPtr::local(") {
		t.Fatalf("local later assigned a GoPtr should be seeded from the ordinary pointer as GoPtr::local:\n%s", rust)
	}
	if !strings.Contains(rust, "i = ") || !strings.Contains(rust, ".indirect();") {
		t.Fatalf("assignment from GoPtr-returning method should preserve the GoPtr handle:\n%s", rust)
	}
}

func TestGoPtrCandidateArgPromotesPointerParam(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Pointer[T any] struct {
	value *T
}

func (p *Pointer[T]) Load() *T {
	return p.value
}

type indirect struct {
	child Pointer[node]
}

type node struct {
	isEntry bool
}

func (n *node) indirect() *indirect {
	return (*indirect)(unsafe.Pointer(n))
}

type table struct {
	root Pointer[indirect]
}

func consume(i *indirect) {
}

func walk(t *table, flag bool) *indirect {
	i := t.root.Load()
	for flag {
		n := i.child.Load()
		i = n.indirect()
		consume(i)
		flag = false
	}
	return i
}
`)

	if !strings.Contains(rust, "pub fn consume(i: GoPtr<indirect>)") {
		t.Fatalf("callee receiving a promoted GoPtr local should use a GoPtr parameter:\n%s", rust)
	}
	if strings.Contains(rust, "pub fn consume(i: Rc<RefCell<Option<indirect>>>)") ||
		strings.Contains(rust, "pub fn consume(i: Arc<Mutex<Option<indirect>>>)") {
		t.Fatalf("callee receiving a promoted GoPtr local should not keep the ordinary pointer wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "consume(i.clone())") {
		t.Fatalf("call should pass the GoPtr local without wrapping it as an ordinary pointer:\n%s", rust)
	}
}

func TestGoPtrDerefCompoundAssignUsesGoPtrMutation(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type holder struct {
	ptr *byte
}

func remember(h *holder, buf []byte) {
	h.ptr = &buf[0]
}

func edit(h *holder, mask byte) {
	p := h.ptr
	*p &= mask
	*p |= 1
}
`)

	if strings.Contains(rust, "p.lock()") || strings.Contains(rust, "p.borrow_mut()") {
		t.Fatalf("compound assignment through a GoPtr should not use wrapper mutation:\n%s", rust)
	}
	if strings.Count(rust, "p.with_mut(") < 2 {
		t.Fatalf("compound assignment through a GoPtr should mutate through GoPtr::with_mut:\n%s", rust)
	}
}

func TestGoPtrFieldDerefCompoundAssignUsesGoPtrMutation(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type bits struct {
	bytep *byte
	mask byte
}

func makeBits(buf []byte) bits {
	return bits{bytep: &buf[0], mask: 1}
}

func set(m bits) {
	*m.bytep |= m.mask
}
`)

	if strings.Contains(rust, "bytep.lock()") || strings.Contains(rust, "bytep.borrow_mut()") {
		t.Fatalf("compound assignment through a GoPtr field should not use wrapper mutation:\n%s", rust)
	}
	if !strings.Contains(rust, "bytep.clone().with_mut(") {
		t.Fatalf("compound assignment through a GoPtr field should mutate through GoPtr::with_mut:\n%s", rust)
	}
}

func TestGoPtrCompositeLiteralFieldFromCapturedLocalUsesGoPtrField(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type cursor struct {
	ptr *byte
	n int
}

func use(buf []byte) {
	p := &buf[0]
	func() {
		_ = cursor{ptr: p, n: 1}
	}()
}
`)

	if !strings.Contains(rust, "pub ptr: GoPtr<u8>") {
		t.Fatalf("struct field initialized from a captured GoPtr local should use GoPtr storage:\n%s", rust)
	}
	if strings.Contains(rust, "pub ptr: Rc<RefCell<Option<u8>>>") ||
		strings.Contains(rust, "pub ptr: Arc<Mutex<Option<u8>>>") {
		t.Fatalf("struct field initialized from a captured GoPtr local should not keep wrapper storage:\n%s", rust)
	}
	if !strings.Contains(rust, "ptr: GoPtr::slice_elem_opt(") {
		t.Fatalf("composite literal should initialize the GoPtr field from the slice element handle:\n%s", rust)
	}
}

func TestGoPtrCandidateArgPromotesMethodPointerParam(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Pointer[T any] struct {
	value *T
}

func (p *Pointer[T]) Load() *T {
	return p.value
}

type indirect struct {
	child Pointer[node]
}

type node struct {
	isEntry bool
}

func (n *node) indirect() *indirect {
	return (*indirect)(unsafe.Pointer(n))
}

type table struct {
	root Pointer[indirect]
}

func (t *table) consume(i *indirect) {
}

func walk(t *table, flag bool) *indirect {
	i := t.root.Load()
	for flag {
		n := i.child.Load()
		i = n.indirect()
		t.consume(i)
		flag = false
	}
	return i
}
`)

	if !strings.Contains(rust, "pub fn consume(&self, i: GoPtr<indirect>)") {
		t.Fatalf("method receiving a promoted GoPtr local should use a GoPtr parameter:\n%s", rust)
	}
	if strings.Contains(rust, "pub fn consume(&self, i: Rc<RefCell<Option<indirect>>>)") ||
		strings.Contains(rust, "pub fn consume(&self, i: Arc<Mutex<Option<indirect>>>)") {
		t.Fatalf("method receiving a promoted GoPtr local should not keep the ordinary pointer wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, ".consume(i.clone())") {
		t.Fatalf("method call should pass the GoPtr local without wrapping it as an ordinary pointer:\n%s", rust)
	}
}

func TestGoPtrCallResultArgPromotesMethodPointerParam(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type indirect struct {
	value int
}

type holder struct {
	current *indirect
}

func initHolder(h *holder, all []indirect) {
	h.current = &all[0]
}

func (h *holder) Current() *indirect {
	return h.current
}

type table struct {
	seen int
}

func (t *table) consume(i *indirect) {
	t.seen = i.value
}

func walk(t *table, h *holder) {
	t.consume(h.Current())
}
`)

	if !strings.Contains(rust, "pub fn current(") || !strings.Contains(rust, " -> GoPtr<indirect>") {
		t.Fatalf("source method should return a GoPtr handle for this regression:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn consume(&mut self, i: GoPtr<indirect>)") {
		t.Fatalf("method receiving a GoPtr-returning call result should use a GoPtr parameter:\n%s", rust)
	}
	if strings.Contains(rust, "pub fn consume(&mut self, i: Rc<RefCell<Option<indirect>>>)") ||
		strings.Contains(rust, "pub fn consume(&mut self, i: Arc<Mutex<Option<indirect>>>)") {
		t.Fatalf("method receiving a GoPtr-returning call result should not keep the ordinary pointer wrapper:\n%s", rust)
	}
	if strings.Contains(rust, ".consume(Rc::new") || strings.Contains(rust, ".consume(Arc::new") {
		t.Fatalf("method call should pass the GoPtr call result without wrapping it as an ordinary pointer:\n%s", rust)
	}
}

func TestGoPtrCallResultNilComparisonUsesGoPtrNilCheck(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type indirect struct {
	value int
}

type holder struct {
	current *indirect
}

func initHolder(h *holder, all []indirect) {
	h.current = &all[0]
}

func (h *holder) Current() *indirect {
	return h.current
}

func hasCurrent(h *holder) bool {
	return h.Current() != nil
}
`)

	if !strings.Contains(rust, "pub fn current(") || !strings.Contains(rust, " -> GoPtr<indirect>") {
		t.Fatalf("source method should return a GoPtr handle for this regression:\n%s", rust)
	}
	if strings.Contains(rust, ".current().borrow()") || strings.Contains(rust, ".current().lock()") {
		t.Fatalf("GoPtr call result nil comparison should not use ordinary pointer wrapper borrows:\n%s", rust)
	}
	if !strings.Contains(rust, ".current().is_nil()") {
		t.Fatalf("GoPtr call result nil comparison should call is_nil():\n%s", rust)
	}
}

func TestGoPtrCallResultInLaterReturnSlotUsesGoPtrResult(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type node struct {
	value int
}

type holder struct {
	current *node
}

func initHolder(h *holder, all []node) {
	h.current = &all[0]
}

func (h *holder) Current() *node {
	return h.current
}

func pick(h *holder) (int, *node, bool) {
	return 1, h.Current(), true
}
`)

	if !strings.Contains(rust, "pub fn current(") || !strings.Contains(rust, " -> GoPtr<node>") {
		t.Fatalf("source method should return a GoPtr handle for this regression:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn pick(") || !strings.Contains(rust, " -> (i32, GoPtr<node>, bool)") {
		t.Fatalf("GoPtr call result in a later return slot should promote that result type:\n%s", rust)
	}
	if strings.Contains(rust, " -> (i32, Rc<RefCell<Option<node>>>, bool)") ||
		strings.Contains(rust, " -> (i32, Arc<Mutex<Option<node>>>, bool)") {
		t.Fatalf("GoPtr call result in a later return slot should not keep the ordinary pointer wrapper:\n%s", rust)
	}
}

func TestGoPtrImportedGenericCallResultInLaterReturnSlotUsesInstantiatedType(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "sync/atomic"

type entry[K comparable, V any] struct {
	overflow atomic.Pointer[entry[K, V]]
	key K
	value V
}

func (head *entry[K, V]) loadAndDelete(hit bool) (V, *entry[K, V], bool) {
	if hit {
		return head.value, head.overflow.Load(), true
	}
	return head.value, head, false
}

func (head *entry[K, V]) next() *entry[K, V] {
	return head.overflow.Load()
}

func use(head *entry[int, string]) bool {
	ok := false
	_, e, ok := head.next().loadAndDelete(false)
	if e != nil {
		head.overflow.Store(e)
	}
	return ok
}
`)

	if !strings.Contains(rust, "pub fn load_and_delete(") ||
		!strings.Contains(rust, "GoPtr<entry<K, V>>") {
		t.Fatalf("imported generic GoPtr call result should use its instantiated pointer element type:\n%s", rust)
	}
	if strings.Contains(rust, "Rc<RefCell<Option<entry<K, V>>>>") ||
		strings.Contains(rust, "Arc<Mutex<Option<entry<K, V>>>>") {
		t.Fatalf("imported generic GoPtr call result should not keep the ordinary pointer wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::local(") || !strings.Contains(rust, "Some(self.clone())") {
		t.Fatalf("current receiver returned through a promoted GoPtr slot should be wrapped as a local GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "e.lock()") || strings.Contains(rust, "e.borrow().is_some()") {
		t.Fatalf("tuple local receiving a promoted GoPtr result should not use ordinary pointer wrapper operations:\n%s", rust)
	}
	if !strings.Contains(rust, "!e.is_nil()") {
		t.Fatalf("tuple local receiving a promoted GoPtr result should use GoPtr nil checks:\n%s", rust)
	}
}

func TestGoPtrTupleResultFromPromotedReceiverRegistersLocal(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node[K comparable, V any] struct {
	isEntry bool
}

type entry[K comparable, V any] struct {
	node node[K, V]
	key K
	value V
}

type table[K comparable, V any] struct {
}

func (n *node[K, V]) entryPtr() *entry[K, V] {
	return (*entry[K, V])(unsafe.Pointer(n))
}

func consume[K comparable, V any](n *node[K, V]) {
}

func (ht *table[K, V]) use(n *node[K, V]) (loaded bool) {
	_, e, loaded := n.entryPtr().loadAndDelete(false)
	if e != nil {
		consume(&e.node)
	}
	return loaded
}

func (head *entry[K, V]) next() *entry[K, V] {
	return (*entry[K, V])(unsafe.Pointer(head))
}

func (head *entry[K, V]) loadAndDelete(hit bool) (V, *entry[K, V], bool) {
	if hit {
		return head.value, head.next(), true
	}
	return head.value, head, false
}
`)

	if !strings.Contains(rust, "GoPtr<entry<K, V>>") {
		t.Fatalf("promoted tuple result should use a GoPtr entry result:\n%s", rust)
	}
	if strings.Contains(rust, "e.lock()") || strings.Contains(rust, "e.borrow().is_some()") {
		t.Fatalf("tuple local from promoted receiver result should not use ordinary pointer wrapper operations:\n%s", rust)
	}
	if !strings.Contains(rust, "!e.is_nil()") {
		t.Fatalf("tuple local from promoted receiver result should use GoPtr nil checks:\n%s", rust)
	}
}

func TestGoPtrReassignedPointerReceiverAliasUsesGoPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type entry[K comparable, V any] struct {
	key K
	value V
}

func (e *entry[K, V]) next() *entry[K, V] {
	return (*entry[K, V])(unsafe.Pointer(e))
}

func (e *entry[K, V]) lookup(key K) (V, bool) {
	for e != nil {
		if e.key == key {
			return e.value, true
		}
		e = e.next()
	}
	var zero V
	return zero, false
}
`)

	if !strings.Contains(rust, "let mut __self = GoPtr::local(") {
		t.Fatalf("reassigned pointer receiver receiving GoPtr values should use a GoPtr alias:\n%s", rust)
	}
	if !strings.Contains(rust, "while !__self.is_nil()") {
		t.Fatalf("GoPtr receiver alias nil checks should use GoPtr::is_nil:\n%s", rust)
	}
	if strings.Contains(rust, "__self.lock()") || strings.Contains(rust, "__self.borrow().is_some()") {
		t.Fatalf("GoPtr receiver alias should not use ordinary pointer wrapper operations:\n%s", rust)
	}
}

func TestGoPtrDeferredFieldSelectorUsesCapturedClone(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type lock struct{}

func (l *lock) Unlock() {}

type indirect[T any] struct {
	mu lock
	value T
}

type node[T any] struct {
	value T
}

func (n *node[T]) indirect() *indirect[T] {
	return (*indirect[T])(unsafe.Pointer(n))
}

func use[T any](*indirect[T]) {}

func test[T any](n *node[T]) {
	i := n.indirect()
	defer i.mu.Unlock()
	use(i)
}
`)

	if !strings.Contains(rust, "let i_defer_captured = i.clone();") {
		t.Fatalf("deferred GoPtr local selector should capture the local clone:\n%s", rust)
	}
	if !strings.Contains(rust, "i_defer_captured.with_mut(") {
		t.Fatalf("deferred GoPtr field selector should use the captured clone:\n%s", rust)
	}
	if strings.Contains(rust, "let __ptr_value = i.borrow()") {
		t.Fatalf("deferred GoPtr field selector should not move the outer local into the closure:\n%s", rust)
	}
}

func TestGoPtrCandidateArgPromotesGenericMethodPointerParam(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Pointer[T any] struct {
	value *T
}

func (p *Pointer[T]) Load() *T {
	return p.value
}

type indirect[T any] struct {
	child Pointer[node[T]]
}

type node[T any] struct {
	value T
}

func (n *node[T]) indirect() *indirect[T] {
	return (*indirect[T])(unsafe.Pointer(n))
}

type table[T any] struct {
	root Pointer[indirect[T]]
}

func (t *table[T]) consume(i *indirect[T]) {
}

func walk[T any](t *table[T], flag bool) *indirect[T] {
	i := t.root.Load()
	for flag {
		n := i.child.Load()
		i = n.indirect()
		t.consume(i)
		flag = false
	}
	return i
}
`)

	if !strings.Contains(rust, "pub fn consume(&self, i: GoPtr<indirect<T>>)") {
		t.Fatalf("generic method receiving a promoted GoPtr local should use a GoPtr parameter:\n%s", rust)
	}
	if strings.Contains(rust, "pub fn consume(&self, i: Rc<RefCell<Option<indirect<T>>>>)") ||
		strings.Contains(rust, "pub fn consume(&self, i: Arc<Mutex<Option<indirect<T>>>>)") {
		t.Fatalf("generic method receiving a promoted GoPtr local should not keep the ordinary pointer wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, ".consume(i.clone())") {
		t.Fatalf("generic method call should pass the GoPtr local without wrapping it as an ordinary pointer:\n%s", rust)
	}
}

func TestGoPtrVarAssignedFromGenericMethodPromotesGenericMethodPointerParam(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type entry[T any] struct {
	value T
}

type indirect[T any] struct {
	next *entry[T]
}

type node[T any] struct {
	value T
}

func (n *node[T]) entry() *entry[T] {
	return (*entry[T])(unsafe.Pointer(n))
}

func (n *node[T]) indirect() *indirect[T] {
	return (*indirect[T])(unsafe.Pointer(n))
}

type table[T any] struct {
}

func (t *table[T]) expand(old *entry[T], parent *indirect[T]) {
}

func walk[T any](t *table[T], n *node[T]) {
	var old *entry[T]
	var parent *indirect[T]
	if n != nil {
		old = n.entry()
		parent = n.indirect()
	}
	if old != nil {
		t.expand(old, parent)
	}
}
`)

	if !strings.Contains(rust, "pub fn expand(&self, old: GoPtr<entry<T>>, parent: GoPtr<indirect<T>>)") {
		t.Fatalf("generic method receiving var-assigned GoPtr locals should use GoPtr parameters:\n%s", rust)
	}
	if strings.Contains(rust, "old: Rc<RefCell<Option<entry<T>>>>") ||
		strings.Contains(rust, "old: Arc<Mutex<Option<entry<T>>>>") ||
		strings.Contains(rust, "parent: Rc<RefCell<Option<indirect<T>>>>") ||
		strings.Contains(rust, "parent: Arc<Mutex<Option<indirect<T>>>>") {
		t.Fatalf("generic method receiving var-assigned GoPtr locals should not keep ordinary pointer wrappers:\n%s", rust)
	}
	if !strings.Contains(rust, ".expand(old.clone(), parent.clone())") {
		t.Fatalf("generic method call should pass var-assigned GoPtr locals directly:\n%s", rust)
	}
}

func TestGoPtrVarsPromoteGroupedGenericMethodPointerParams(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type entry[T any] struct {
	value T
}

type indirect[T any] struct {
	next *entry[T]
}

type node[T any] struct {
	value T
}

func (n *node[T]) entry() *entry[T] {
	return (*entry[T])(unsafe.Pointer(n))
}

func (n *node[T]) indirect() *indirect[T] {
	return (*indirect[T])(unsafe.Pointer(n))
}

type table[T any] struct {
}

func (t *table[T]) expand(old, next *entry[T], parent *indirect[T]) {
}

func walk[T any](t *table[T], n *node[T]) {
	var old *entry[T]
	var next *entry[T]
	var parent *indirect[T]
	if n != nil {
		old = n.entry()
		next = n.entry()
		parent = n.indirect()
	}
	if old != nil {
		t.expand(old, next, parent)
	}
}
`)

	if !strings.Contains(rust, "pub fn expand(&self, old: GoPtr<entry<T>>, next: GoPtr<entry<T>>, parent: GoPtr<indirect<T>>)") {
		t.Fatalf("grouped generic method pointer params receiving GoPtr locals should use GoPtr parameters:\n%s", rust)
	}
	if strings.Contains(rust, "pub fn expand(&self, old: Rc<RefCell<Option<entry<T>>>>") ||
		strings.Contains(rust, "pub fn expand(&self, old: Arc<Mutex<Option<entry<T>>>>") ||
		strings.Contains(rust, "next: Rc<RefCell<Option<entry<T>>>>, parent") ||
		strings.Contains(rust, "next: Arc<Mutex<Option<entry<T>>>>, parent") ||
		strings.Contains(rust, "parent: Rc<RefCell<Option<indirect<T>>>>)") ||
		strings.Contains(rust, "parent: Arc<Mutex<Option<indirect<T>>>>)") {
		t.Fatalf("grouped generic method pointer params should not keep ordinary pointer wrappers:\n%s", rust)
	}
	if !strings.Contains(rust, ".expand(old.clone(), next.clone(), parent.clone())") {
		t.Fatalf("grouped generic method call should pass GoPtr locals directly:\n%s", rust)
	}
}

func TestGoPtrVarPromotesOnlyMatchingGroupedGenericMethodPointerParam(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type entry[T any] struct {
	value T
}

type indirect[T any] struct {
	next *entry[T]
}

type node[T any] struct {
	value T
}

func (n *node[T]) entry() *entry[T] {
	return (*entry[T])(unsafe.Pointer(n))
}

func (n *node[T]) indirect() *indirect[T] {
	return (*indirect[T])(unsafe.Pointer(n))
}

func newEntry[T any]() *entry[T] {
	return new(entry[T])
}

type table[T any] struct {
}

func (t *table[T]) expand(old, fresh *entry[T], parent *indirect[T]) {
}

func walk[T any](t *table[T], n *node[T]) {
	var old *entry[T]
	var parent *indirect[T]
	if n != nil {
		old = n.entry()
		parent = n.indirect()
	}
	fresh := newEntry[T]()
	if old != nil {
		t.expand(old, fresh, parent)
	}
}
`)

	if !strings.Contains(rust, "pub fn expand(&self, old: GoPtr<entry<T>>, fresh: Rc<RefCell<Option<entry<T>>>>, parent: GoPtr<indirect<T>>)") {
		t.Fatalf("mixed grouped generic method params should promote only the GoPtr arguments:\n%s", rust)
	}
	if !strings.Contains(rust, ".expand(old.clone(), fresh.clone(), parent.clone())") {
		t.Fatalf("mixed grouped generic method call should pass each local with its own representation:\n%s", rust)
	}
}

func TestGoPtrVarPromotesHashTrieStyleGenericMethodPointerParams(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type entry[K comparable, V any] struct {
	key K
	value V
}

type indirect[K comparable, V any] struct {
	next *entry[K, V]
}

type node[K comparable, V any] struct {
	key K
	value V
}

func (n *node[K, V]) entry() *entry[K, V] {
	return (*entry[K, V])(unsafe.Pointer(n))
}

func (n *node[K, V]) indirect() *indirect[K, V] {
	return (*indirect[K, V])(unsafe.Pointer(n))
}

func newEntryNode[K comparable, V any](key K, value V) *entry[K, V] {
	return &entry[K, V]{key: key, value: value}
}

type HashTrieMap[K comparable, V any] struct {
}

func (ht *HashTrieMap[K, V]) expand(oldEntry, newEntry *entry[K, V], parent *indirect[K, V]) *node[K, V] {
	return nil
}

func (ht *HashTrieMap[K, V]) store(n *node[K, V], key K, value V) {
	var oldEntry *entry[K, V]
	var parent *indirect[K, V]
	if n != nil {
		oldEntry = n.entry()
		parent = n.indirect()
	}
	newEntry := newEntryNode(key, value)
	if oldEntry != nil {
		_ = ht.expand(oldEntry, newEntry, parent)
	}
}
`)

	if !strings.Contains(rust, "pub fn expand(&self, oldEntry: GoPtr<entry<K, V>>, newEntry: Rc<RefCell<Option<entry<K, V>>>>, parent: GoPtr<indirect<K, V>>)") {
		t.Fatalf("HashTrie-style generic method params should promote only GoPtr locals:\n%s", rust)
	}
	if !strings.Contains(rust, ".expand(oldEntry.clone(), newEntry.clone(), parent.clone())") {
		t.Fatalf("HashTrie-style generic method call should pass each local with its own representation:\n%s", rust)
	}
}

func TestGoPtrConcreteArgPromotesGenericTypeParamPointerParam(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Pointer[T any] struct {
	value *T
}

func (p *Pointer[T]) Store(value *T) {
	p.value = value
}

type entry[T any] struct {
	value T
}

type node[T any] struct {
	value T
}

func (n *node[T]) entry() *entry[T] {
	return (*entry[T])(unsafe.Pointer(n))
}

func use[T any](slot *Pointer[entry[T]], n *node[T]) {
	var old *entry[T]
	if n != nil {
		old = n.entry()
	}
	slot.Store(old)
}
`)

	if !strings.Contains(rust, "pub fn store(&mut self, value: GoPtr<T>)") {
		t.Fatalf("generic *T method param receiving a concrete GoPtr should use GoPtr<T> in the declaration:\n%s", rust)
	}
	if !strings.Contains(rust, "impl<T: Any + Clone + 'static> Pointer<T>") {
		t.Fatalf("generic method impl containing a GoPtr<T> parameter should add the Clone bound required by GoPtr<T>:\n%s", rust)
	}
	if strings.Contains(rust, "pub fn store(&mut self, value: Rc<RefCell<Option<T>>>)") ||
		strings.Contains(rust, "pub fn store(&mut self, value: Arc<Mutex<Option<T>>>)") {
		t.Fatalf("generic *T method param receiving a concrete GoPtr should not keep the ordinary pointer wrapper:\n%s", rust)
	}
	if !strings.Contains(rust, ".store(old.clone())") {
		t.Fatalf("generic *T method call should pass the concrete GoPtr local directly:\n%s", rust)
	}
}

func TestGoPtrParamAssignmentPromotesGenericStructField(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type holder[T any] struct {
	value *entry[T]
}

func (h *holder[T]) init(value *entry[T]) {
	h.value = value
}

func (h *holder[T]) pass(s *sink[T]) {
	s.take(h.value)
}

type entry[T any] struct {
	value T
}

type sink[T any] struct {
}

func (s *sink[T]) take(value *entry[T]) {
}

type node[T any] struct {
	value T
}

func (n *node[T]) entry() *entry[T] {
	return (*entry[T])(unsafe.Pointer(n))
}

func use[T any](h *holder[T], n *node[T]) {
	var value *entry[T]
	if n != nil {
		value = n.entry()
	}
	h.init(value)
}
`)

	if !strings.Contains(rust, "pub value: GoPtr<entry<T>>") {
		t.Fatalf("field assigned from a promoted GoPtr parameter should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn init(&mut self, value: GoPtr<entry<T>>)") {
		t.Fatalf("method parameter receiving a GoPtr local should be promoted:\n%s", rust)
	}
	if !strings.Contains(rust, "{ let new_val = value.clone(); self.value = new_val; };") {
		t.Fatalf("field assignment from a promoted GoPtr parameter should assign the GoPtr handle:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn take(&self, value: GoPtr<entry<T>>)") {
		t.Fatalf("GoPtr field selector passed to another method should promote that method parameter:\n%s", rust)
	}
	if !strings.Contains(rust, ".take(self.value.clone())") {
		t.Fatalf("GoPtr field selector call argument should pass the field handle directly:\n%s", rust)
	}
}

func TestGoPtrFieldAddressPromotesPointerToPointerParamSlot(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type arenaHint struct {
	next *arenaHint
}

type heap struct {
	arenaHints *arenaHint
}

type rawHint struct {
}

func (r *rawHint) hint() *arenaHint {
	return (*arenaHint)(unsafe.Pointer(r))
}

func (h *heap) sysAlloc(hintList **arenaHint) {
	for *hintList != nil {
		hint := *hintList
		*hintList = hint.next
	}
}

func (h *heap) seed(r *rawHint) {
	h.arenaHints = r.hint()
}

func (h *heap) alloc() {
	hintList := &h.arenaHints
	h.sysAlloc(hintList)
}
`)

	if !strings.Contains(rust, "pub arena_hints: GoPtr<arenaHint>") {
		t.Fatalf("field assigned from a GoPtr source should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "hintList: GoPtr<GoPtr<arenaHint>>") {
		t.Fatalf("pointer-to-pointer param targeting a GoPtr field slot should receive a GoPtr value slot:\n%s", rust)
	}
	if strings.Contains(rust, "pub fn sys_alloc(&mut self, hintList: Rc<RefCell<Option<Rc<RefCell<Option<arenaHint>>>>>>)") ||
		strings.Contains(rust, "pub fn sys_alloc(&mut self, hintList: Arc<Mutex<Option<Arc<Mutex<Option<arenaHint>>>>>>)") {
		t.Fatalf("pointer-to-pointer param targeting a GoPtr field slot must not use the old pointer wrapper slot:\n%s", rust)
	}
	if !strings.Contains(rust, "hintList.assign(Some(new_val));") {
		t.Fatalf("assignment through a pointer-to-GoPtr slot should replace the stored GoPtr handle:\n%s", rust)
	}
	if !strings.Contains(rust, "__ptr_slot.as_ref().unwrap().is_nil()") {
		t.Fatalf("nil checks through a pointer-to-GoPtr slot should test the stored GoPtr handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut hint: GoPtr<arenaHint> = { let __ptr_slot = hintList.borrow(); __ptr_slot.as_ref().unwrap().clone() }") {
		t.Fatalf("short declarations from a pointer-to-GoPtr slot should create GoPtr locals:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut hintList: GoPtr<GoPtr<arenaHint>> = GoPtr::local(") {
		t.Fatalf("addressing a GoPtr field should create a local GoPtr slot handle:\n%s", rust)
	}
	if strings.Contains(rust, "__dst_guard.as_ref().unwrap().borrow_mut()") {
		t.Fatalf("assignment through a pointer-to-GoPtr slot must not borrow through the old pointer wrapper shape:\n%s", rust)
	}
	if strings.Contains(rust, "(*(*hintList.borrow_mut().as_mut().unwrap()).borrow())") ||
		strings.Contains(rust, "(*(*hintList.borrow().as_ref().unwrap()).borrow())") {
		t.Fatalf("reads through a pointer-to-GoPtr slot must not borrow through the old pointer wrapper shape:\n%s", rust)
	}
}

func TestGoPtrParamDeclUsesTypedCalleeModulePath(t *testing.T) {
	const src = `package maps

type Map struct{}
type Iter struct{}

func (it *Iter) Init(m *Map) {
}
`
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "table.go", src, 0)
	if err != nil {
		t.Fatalf("ParseFile(table.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo error = %v", err)
	}
	var initFn *ast.FuncDecl
	for _, decl := range file.Decls {
		if fn, ok := decl.(*ast.FuncDecl); ok && fn.Name.Name == "Init" {
			initFn = fn
			break
		}
	}
	if initFn == nil {
		t.Fatalf("fixture missing Init method")
	}

	prevTypeInfo := GetTypeInfo()
	prevCtx := GetTranspileContext()
	t.Cleanup(func() {
		SetTypeInfo(prevTypeInfo)
		SetTranspileContext(prevCtx)
	})
	pkgState := NewPackageState()
	pkgState.TypeModuleNames["Map"] = "map"
	pkgState.TypeModuleNames["Iter"] = "table"
	ctx := &TranspileContext{
		CurrentModuleName: "table",
		Package:           pkgState,
		Session:           NewTranspileSession(typeInfo, nil),
	}
	SetTypeInfo(typeInfo)
	SetTranspileContext(ctx)
	fnObj, ok := sliceElemPtrReturnFuncObject(initFn)
	if !ok {
		t.Fatalf("missing Init method object")
	}
	if ctx.Package.GoPtrParamFuncs == nil {
		ctx.Package.GoPtrParamFuncs = make(map[*types.Func]map[int]string)
	}
	ctx.Package.GoPtrParamFuncs[fnObj] = map[int]string{
		0: "wrong_cached_crate::wrong::Map",
	}
	SetTypeInfo(&TypeInfo{
		info: typeInfo.info,
		pkg:  types.NewPackage("other/pkg", "other"),
	})

	var out strings.Builder
	writeFuncDeclParam(&out, initFn, 0, "m", initFn.Type.Params.List[0].Type, false)
	got := out.String()
	if got != "m: GoPtr<crate::map::Map>" {
		t.Fatalf("GoPtr param declaration should render the callee package module path, got %q", got)
	}
}

func TestSliceElemPointerGoPtrFieldJsonDecodeWrapsLocalPointer(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegressionWithExternalStubs(t, `package main

type Name struct {
	Bytes *byte
}

func setName(n *Name, b []byte) {
	n.Bytes = &b[0]
}
`)

	if !strings.Contains(rust, "pub bytes: GoPtr<u8>") {
		t.Fatalf("slice element pointer field should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "out.bytes = GoPtr::local(<") ||
		!strings.Contains(rust, "as GoJsonDecode>::go_json_decode(field_value)?);") {
		t.Fatalf("GoPtr field JSON decode should wrap decoded local pointer handles:\n%s", rust)
	}
}

func TestSliceElemPointerGoPtrLocalSelectorBorrowsField(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Inst struct {
	Op int
	Out int
}

type runner struct {
	inst *Inst
}

func (i Inst) matchRune(c int) bool {
	return i.Op == c
}

func assign(r *runner, prog []Inst) {
	r.inst = &prog[0]
}

func step(r *runner) int {
	i := r.inst
	if i.matchRune(2) {
		return i.Out
	}
	switch i.Op {
	case 1:
		return i.Out
	}
	return 0
}

func forceConcurrent(ch chan bool) {
	go func() {
		ch <- true
	}()
}
`)

	if strings.Contains(rust, "i.lock()") {
		t.Fatalf("local copied from GoPtr field should not be treated as a normal pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __ptr_value = i.borrow(); __ptr_value.as_ref().unwrap().op.clone()") {
		t.Fatalf("local copied from GoPtr field should borrow through GoPtr before selecting a field:\n%s", rust)
	}
	if !strings.Contains(rust, "let __recv_value = i.borrow(); let __result = (*__recv_value.as_ref().unwrap()).match_rune(") {
		t.Fatalf("local copied from GoPtr field should borrow through GoPtr before calling value methods:\n%s", rust)
	}
}

func TestGoPtrFieldPointerEqualityUsesAddressIdentity(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Type struct {
	id int
}

type ITab struct {
	Inter *Type
}

func initTab(tab *ITab, types []Type) {
	tab.Inter = &types[0]
}

func find(m *ITab, inter *Type) bool {
	return m.Inter == inter
}

func use(tabs []ITab, types []Type) bool {
	initTab(&tabs[0], types)
	return find(&tabs[0], &types[0])
}

func forceConcurrent(ch chan bool) {
	go func() {
		ch <- true
	}()
}
`)

	if !strings.Contains(rust, "fn find(m: GoPtr<ITab>, inter: GoPtr<Type>)") {
		t.Fatalf("pointer params receiving slice element addresses should use GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "__left.lock()") || strings.Contains(rust, "__right.lock()") || strings.Contains(rust, "Arc::ptr_eq(&__left, &__right)") {
		t.Fatalf("GoPtr pointer equality should not use wrapper lock or Arc pointer equality:\n%s", rust)
	}
	if !strings.Contains(rust, "let __left_addr = { let __ptr_value = m.borrow(); let __field_value = __ptr_value.as_ref().unwrap().inter.clone(); __field_value }.addr()") ||
		!strings.Contains(rust, "let __right_addr = inter.addr()") {
		t.Fatalf("GoPtr pointer equality should compare pointer addresses:\n%s", rust)
	}
}

func TestGoPtrPointerEqualityDoesNotTreatCandidateFieldAsGeneratedStorage(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Type struct {
	id int
}

type ITab struct {
	Typ *Type
}

func find(m *ITab, typ *Type) bool {
	return m.Typ == typ
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	var expr *ast.BinaryExpr
	ast.Inspect(file, func(node ast.Node) bool {
		binary, ok := node.(*ast.BinaryExpr)
		if ok && binary.Op == token.EQL {
			expr = binary
			return false
		}
		return true
	})
	if expr == nil {
		t.Fatalf("fixture missing pointer equality expression")
	}
	sel, ok := expr.X.(*ast.SelectorExpr)
	if !ok {
		t.Fatalf("fixture equality left side should be a selector")
	}

	prevTypeInfo := GetTypeInfo()
	prevCtx := GetTranspileContext()
	prevVarTable := GetVarTable()
	t.Cleanup(func() {
		SetTypeInfo(prevTypeInfo)
		SetTranspileContext(prevCtx)
		SetVarTable(prevVarTable)
	})
	ctx := &TranspileContext{
		Session: NewTranspileSession(typeInfo, nil),
		Package: NewPackageState(),
	}
	SetTypeInfo(typeInfo)
	SetTranspileContext(ctx)
	key, fieldInfo, ok := sliceElemPtrFieldKeyForSelector(sel)
	if !ok {
		t.Fatalf("fixture selector should expose pointer field metadata")
	}
	ctx.Package.SliceElemPtrFields[key] = fieldInfo

	vt := NewVarTable()
	vt.Register("m", &VarInfo{WrapLevel: WrapNone, RustType: "GoPtr<ITab>", Source: SourceParam, PointerKind: PointerGoPtr})
	vt.Register("typ", &VarInfo{WrapLevel: WrapNone, RustType: "GoPtr<Type>", Source: SourceParam, PointerKind: PointerGoPtr})
	SetVarTable(vt)

	var out strings.Builder
	if !writeGoPtrPointerEquality(&out, expr) {
		t.Fatalf("GoPtr pointer equality writer should handle candidate field plus GoPtr param")
	}
	rust := out.String()
	if strings.Contains(rust, ".typ.clone() }.addr()") {
		t.Fatalf("candidate-only field should not be treated as generated GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::local({ let __ptr_value = m.borrow(); let __field_value = __ptr_value.as_ref().unwrap().typ.clone(); __field_value })") {
		t.Fatalf("candidate-only field should be converted from its wrapped pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let __right_addr = typ.addr()") {
		t.Fatalf("GoPtr parameter side should still use its address token:\n%s", rust)
	}
}

func TestGoPtrGeneratedForeignFieldCallArgumentConvertsHelperType(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Type struct {
	id int
}

type ITab struct {
	Inter *Type
}

func find(m *ITab) {
	use(m.Inter)
}

func use(inter *Type) {
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	var sel *ast.SelectorExpr
	ast.Inspect(file, func(node ast.Node) bool {
		call, ok := node.(*ast.CallExpr)
		if !ok || len(call.Args) != 1 {
			return true
		}
		if argSel, ok := call.Args[0].(*ast.SelectorExpr); ok {
			sel = argSel
			return false
		}
		return true
	})
	if sel == nil {
		t.Fatalf("fixture missing selector call argument")
	}

	prevTypeInfo := GetTypeInfo()
	prevCtx := GetTranspileContext()
	prevVarTable := GetVarTable()
	t.Cleanup(func() {
		SetTypeInfo(prevTypeInfo)
		SetTranspileContext(prevCtx)
		SetVarTable(prevVarTable)
	})
	ctx := &TranspileContext{
		Session:        NewTranspileSession(typeInfo, map[string]string{"example.com/abi": "example_com_abi"}),
		Package:        NewPackageState(),
		PackageMapping: map[string]string{"example.com/abi": "example_com_abi"},
	}
	SetTypeInfo(typeInfo)
	SetTranspileContext(ctx)
	key, fieldInfo, ok := sliceElemPtrFieldKeyForSelector(sel)
	if !ok {
		t.Fatalf("fixture selector should expose pointer field metadata")
	}
	fieldInfo.ownerPkgPath = "example.com/abi"
	ctx.Package.SliceElemPtrFields[key] = fieldInfo
	recordGeneratedGoPtrFieldForKey(key)

	vt := NewVarTable()
	vt.Register("m", &VarInfo{WrapLevel: WrapNone, RustType: "GoPtr<ITab>", Source: SourceParam, PointerKind: PointerGoPtr})
	SetVarTable(vt)

	var out strings.Builder
	if !writeGoPtrCallArgumentWithQualifierForInfo(&out, sel, goPtrResultInfo{elemRustType: "Type"}, "") {
		t.Fatalf("foreign generated GoPtr field should lower as a call argument")
	}
	rust := out.String()
	if !strings.Contains(rust, "match __go_ptr { example_com_abi::GoPtr::Nil => GoPtr::nil()") {
		t.Fatalf("foreign generated GoPtr field should convert from the owner helper type:\n%s", rust)
	}
	if strings.Contains(rust, "GoPtr::local({ let __ptr_value = m.borrow(); __ptr_value.as_ref().unwrap().inter.clone() }.clone())") {
		t.Fatalf("foreign generated GoPtr field should not be wrapped as a local pointer handle:\n%s", rust)
	}
}

func TestGoPtrForeignFieldAssignmentConvertsHelperType(t *testing.T) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "main.go", `package main

type Type struct {
	id int
}

type EmptyInterface struct {
	Type *Type
}

func store(e *EmptyInterface, t *Type) {
	e.Type = t
}
`, 0)
	if err != nil {
		t.Fatalf("ParseFile(main.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{file}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	var assign *ast.AssignStmt
	ast.Inspect(file, func(node ast.Node) bool {
		if stmt, ok := node.(*ast.AssignStmt); ok {
			assign = stmt
			return false
		}
		return true
	})
	if assign == nil || len(assign.Lhs) != 1 || len(assign.Rhs) != 1 {
		t.Fatalf("fixture missing single assignment")
	}
	sel, ok := assign.Lhs[0].(*ast.SelectorExpr)
	if !ok {
		t.Fatalf("fixture assignment lhs should be a selector")
	}

	prevTypeInfo := GetTypeInfo()
	prevCtx := GetTranspileContext()
	prevVarTable := GetVarTable()
	t.Cleanup(func() {
		SetTypeInfo(prevTypeInfo)
		SetTranspileContext(prevCtx)
		SetVarTable(prevVarTable)
	})
	ctx := &TranspileContext{
		Session:        NewTranspileSession(typeInfo, map[string]string{"example.com/abi": "example_com_abi"}),
		Package:        NewPackageState(),
		PackageMapping: map[string]string{"example.com/abi": "example_com_abi"},
	}
	SetTypeInfo(typeInfo)
	SetTranspileContext(ctx)
	key, fieldInfo, ok := sliceElemPtrFieldKeyForSelector(sel)
	if !ok {
		t.Fatalf("fixture selector should expose pointer field metadata")
	}
	fieldInfo.ownerPkgPath = "example.com/abi"
	ctx.Package.SliceElemPtrFields[key] = fieldInfo
	recordGeneratedGoPtrFieldForKey(key)

	vt := NewVarTable()
	vt.Register("e", &VarInfo{WrapLevel: WrapNone, RustType: "GoPtr<EmptyInterface>", Source: SourceParam, PointerKind: PointerGoPtr})
	vt.Register("t", &VarInfo{WrapLevel: WrapNone, RustType: "GoPtr<Type>", Source: SourceParam, PointerKind: PointerGoPtr})
	SetVarTable(vt)

	var out strings.Builder
	if !writeSliceElemPtrFieldAssignment(&out, assign.Lhs[0], assign.Rhs[0]) {
		t.Fatalf("foreign GoPtr field assignment should lower")
	}
	rust := out.String()
	if strings.Contains(rust, `unimplemented!("slice element pointer field assignment requires compatible pointer value")`) {
		t.Fatalf("foreign GoPtr field assignment should not fall back to an unimplemented value path:\n%s", rust)
	}
	if !strings.Contains(rust, "match __go_ptr { GoPtr::Nil => example_com_abi::GoPtr::nil()") {
		t.Fatalf("foreign GoPtr field assignment should convert the RHS helper type:\n%s", rust)
	}
	if !strings.Contains(rust, "e.with_mut(|__ptr_value| { __ptr_value.r#type = new_val; });") {
		t.Fatalf("foreign GoPtr field assignment should mutate the pointed struct field:\n%s", rust)
	}
	if strings.Contains(rust, "} = new_val") {
		t.Fatalf("foreign GoPtr field assignment should not assign to a cloned field expression:\n%s", rust)
	}
}

func TestSliceElemPointerPromotedFieldAssignmentUsesGoPtrField(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Type struct {
	GCData *byte
}

type StructType struct {
	Type
}

func assign(x *StructType, b []byte) {
	x.Type.GCData = nil
	x.Type.GCData = &b[0]
}
`)

	if !strings.Contains(rust, "pub g_c_data: GoPtr<u8>") {
		t.Fatalf("promoted slice element pointer field should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::nil()") {
		t.Fatalf("promoted slice element pointer nil assignment should use GoPtr nil state:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::slice_elem(GoSliceElemPtr::new(b.clone(), (0) as usize))") {
		t.Fatalf("promoted slice element pointer assignment should preserve backing slice identity:\n%s", rust)
	}
}

func TestSliceElemPointerReturnMethodCallBorrowsElement(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type action struct {
	desc int
}

func (a *action) describef(desc int) {
	a.desc = desc
}

func later(actions []action) *action {
	return &actions[0]
}

func use(actions []action) {
	later(actions).describef(1)
}
`)

	if strings.Contains(rust, "__recv.lock()") || strings.Contains(rust, "__recv.borrow()") {
		t.Fatalf("method call on slice element pointer return should not treat the receiver option as a normal pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "__recv.as_ref().unwrap().borrow_mut()") {
		t.Fatalf("method call on slice element pointer return should borrow the returned element pointer:\n%s", rust)
	}
}

func TestSliceElemPointerReturnMethodCallFuncLitReceiverClonesSharedCapture(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type action struct {
	desc int
	f func()
}

func (a *action) describef(desc int) {
	a.desc = desc
}

type checker struct {
	delayed []action
}

func (c *checker) later(f func()) *action {
	i := len(c.delayed)
	c.delayed = append(c.delayed, action{f: f})
	return &c.delayed[i]
}

func (c *checker) use(pos int) {
	c.later(func() {
		_ = c.delayed
		_ = pos
	}).describef(pos)
}
`)

	if !strings.Contains(rust, "pos_closure_clone_closure_clone") {
		t.Fatalf("function literal in slice element pointer receiver should clone captures also used by outer method args:\n%s", rust)
	}
}

func TestNestedSliceElemPointerReturnMethodCallFuncLitClonesSharedCapture(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Pos struct {
	value int
}

type action struct {
	desc *Pos
	f func()
}

func (a *action) describef(desc *Pos) {
	a.desc = desc
}

type checker struct {
	delayed []action
}

func (c *checker) later(f func()) *action {
	i := len(c.delayed)
	c.delayed = append(c.delayed, action{f: f})
	return &c.delayed[i]
}

func (c *checker) outer(pos *Pos) func() {
	return func() {
		c.later(func() {
			_ = pos.value
		}).describef(pos)
	}
}
`)

	if !strings.Contains(rust, "pos_closure_clone_closure_clone") {
		t.Fatalf("nested function literal in slice element pointer receiver should clone captures also used by outer method args:\n%s", rust)
	}
}

func TestSliceElemPointerDirectReturnUsesSliceElemPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type entry struct {
	value int
}

func pick(bucket []entry) *entry {
	return &bucket[len(bucket)-1]
}
`)

	if !strings.Contains(rust, "fn pick(bucket: Rc<RefCell<Option<Vec<entry>>>>) -> Option<GoSliceElemPtr<entry>>") &&
		!strings.Contains(rust, "fn pick(bucket: Arc<Mutex<Option<Vec<entry>>>>) -> Option<GoSliceElemPtr<entry>>") {
		t.Fatalf("direct slice element pointer return should expose the slice element pointer representation:\n%s", rust)
	}
	if strings.Contains(rust, `unimplemented!("slice element pointer return requires pointer representation support")`) {
		t.Fatalf("direct slice element pointer return should not hit the loud fallback:\n%s", rust)
	}
	if !strings.Contains(rust, "Some(GoSliceElemPtr::new(bucket.clone(),") {
		t.Fatalf("direct slice element pointer return should preserve slice/index identity:\n%s", rust)
	}
}

func TestSliceElemPointerLocalReturnUsesSliceElemPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type entry struct {
	value int
}

func pick(bucket []entry, i int) *entry {
	ptr := &bucket[i]
	if ptr.value > 0 {
		return ptr
	}
	return nil
}
`)

	if !strings.Contains(rust, "fn pick(") || !strings.Contains(rust, "-> Option<GoSliceElemPtr<entry>>") {
		t.Fatalf("slice element pointer local return should expose the slice element pointer representation:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut ptr: Option<GoSliceElemPtr<entry>> = Some(GoSliceElemPtr::new(bucket.clone(),") {
		t.Fatalf("slice element pointer local should preserve slice/index identity:\n%s", rust)
	}
	if !strings.Contains(rust, "return ptr.clone()") {
		t.Fatalf("returning a slice element pointer local should return the slice element handle, not a GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "return GoPtr::slice_elem_opt(ptr.clone())") {
		t.Fatalf("slice element pointer local return should not widen to GoPtr:\n%s", rust)
	}
}

func TestSliceElemPointerFieldPointeePointerFieldSelectorReturnsHandle(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type holder struct {
	current *outer
}

type outer struct {
	inner *inner
}

type inner struct {
	check func(int) bool
}

func (h *holder) init(items []outer) {
	h.current = &items[0]
}

func (h *holder) call(v int) bool {
	return h.current.inner.check(v)
}
`)

	if strings.Contains(rust, "slice element pointer field selector requires rvalue support") {
		t.Fatalf("pointer field selected through a GoPtr field should emit a handle, not a placeholder:\n%s", rust)
	}
	if !strings.Contains(rust, "let __ptr_value = self.current.with_mut(|__ptr_value| __ptr_value.inner.clone())") {
		t.Fatalf("pointer field selected through a GoPtr field should clone the field handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".check.clone()") {
		t.Fatalf("function field call should continue through the selected pointer field handle:\n%s", rust)
	}
}

func TestSliceElemPointerFieldPointeePromotedFieldSelector(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type holder struct {
	current *workbuf
}

type workbufhdr struct {
	nobj int
}

type workbuf struct {
	workbufhdr
}

func (h *holder) init(items []workbuf) {
	h.current = &items[0]
}

func (h *holder) empty() bool {
	return h.current.nobj == 0
}

func (h *holder) localCount() int {
	w := h.current
	return w.nobj
}
`)

	if strings.Contains(rust, "slice element pointer field selector requires promoted-field support") {
		t.Fatalf("promoted field selected through a GoPtr field should emit a typed embedded-field traversal:\n%s", rust)
	}
	if !strings.Contains(rust, "let __ptr_value = self.current.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.borrow().as_ref().unwrap().nobj.clone(); __field })") {
		t.Fatalf("promoted field selected through a GoPtr field should traverse the embedded field handle:\n%s", rust)
	}
	if strings.Contains(rust, "GoPtr local field selector requires promoted-field support") {
		t.Fatalf("promoted field selected through a GoPtr local should emit a typed embedded-field traversal:\n%s", rust)
	}
	if !strings.Contains(rust, "let __ptr_value = w.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.borrow().as_ref().unwrap().nobj.clone(); __field })") {
		t.Fatalf("promoted field selected through a GoPtr local should traverse the embedded field handle:\n%s", rust)
	}
}

func TestPackageGlobalSliceElemPointerDirectReturnUsesSliceElemPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type Info struct {
	Name string
}

var All = []Info{{Name: "panicnil"}}

func Lookup(name string) *Info {
	for i := 0; i < len(All); i++ {
		if name == All[i].Name {
			return &All[i]
		}
	}
	return nil
}
`)

	if !strings.Contains(rust, "fn lookup(name: Rc<RefCell<Option<String>>>) -> Option<GoSliceElemPtr<Info>>") &&
		!strings.Contains(rust, "fn lookup(name: Arc<Mutex<Option<String>>>) -> Option<GoSliceElemPtr<Info>>") {
		t.Fatalf("package-global slice element pointer return should expose the slice element pointer representation:\n%s", rust)
	}
	if strings.Contains(rust, `unimplemented!("slice element pointer return requires pointer representation support")`) {
		t.Fatalf("package-global slice element pointer return should not hit the loud fallback:\n%s", rust)
	}
	if !strings.Contains(rust, "return Some(GoSliceElemPtr::new(") {
		t.Fatalf("package-global slice element pointer return should preserve slice/index identity:\n%s", rust)
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
	if !strings.Contains(rust, "dump(GoPtr::slice_elem_opt(i.clone()))") {
		t.Fatalf("if-init slice element pointer local should preserve pointer identity for pointer params:\n%s", rust)
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

	if strings.Contains(rust, "dump(i.clone())") ||
		strings.Contains(rust, "dump(Rc::new(RefCell::new((*i.as_ref().unwrap().borrow()).clone())))") ||
		strings.Contains(rust, "dump(Arc::new(Mutex::new((*i.as_ref().unwrap().borrow()).clone())))") {
		t.Fatalf("pointer parameter should not receive a cloned slice element pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "fn dump(i: GoPtr<inst>)") ||
		!strings.Contains(rust, "dump(GoPtr::slice_elem_opt(i.clone()))") {
		t.Fatalf("pointer parameter should preserve slice element pointer identity:\n%s", rust)
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

	if strings.Contains(rust, "dump(Rc::new(RefCell::new((*GoSliceElemPtr::new") ||
		strings.Contains(rust, "dump(Arc::new(Mutex::new((*GoSliceElemPtr::new") {
		t.Fatalf("pointer parameter should not receive a cloned direct slice element pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "fn dump(values: GoPtr<Vec<i32>>)") ||
		!strings.Contains(rust, "dump(GoPtr::slice_elem(GoSliceElemPtr::new(chunks.clone(), (0) as usize)))") {
		t.Fatalf("pointer parameter should receive a GoPtr slice element handle:\n%s", rust)
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

func TestWritablePointerParamAcceptsSliceElemAddressWithGoPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func mutate(p *byte) {
	*p = 1
}

func use(buf []byte) {
mutate(&buf[0])
}
`)

	if strings.Contains(rust, `unimplemented!("slice element pointer cannot pass to writable pointer parameter")`) ||
		strings.Contains(rust, "mutate(GoSliceElemPtr::new") {
		t.Fatalf("writable pointer parameter should not use the old slice element rejection path:\n%s", rust)
	}
	if !strings.Contains(rust, "fn mutate(p: GoPtr<u8>)") ||
		!strings.Contains(rust, "mutate(GoPtr::slice_elem(GoSliceElemPtr::new(buf.clone(), (0) as usize)))") ||
		!strings.Contains(rust, "p.assign(Some(new_val));") {
		t.Fatalf("writable pointer parameter should preserve and write through slice element identity:\n%s", rust)
	}
}

func TestWritablePointerParamAcceptsArrayElemAddressWithGoPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func mutate(p *byte) {
	*p = 1
}

func use() {
	var buf [4]byte
	mutate(&buf[0])
}
`)

	if strings.Contains(rust, `unimplemented!("slice element pointer cannot pass to writable pointer parameter")`) ||
		strings.Contains(rust, "mutate(GoArrayElemPtr::new") {
		t.Fatalf("writable pointer parameter should not use the old array element temporary path:\n%s", rust)
	}
	if !strings.Contains(rust, "fn mutate(p: GoPtr<u8>)") ||
		!strings.Contains(rust, "mutate(GoPtr::array_elem(GoArrayElemPtr::new(buf.clone(), (0) as usize)))") ||
		!strings.Contains(rust, "p.assign(Some(new_val));") {
		t.Fatalf("writable pointer parameter should preserve and write through array element identity:\n%s", rust)
	}
}

func TestGoPtrParamPropagatesThroughForwardingCall(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func leaf(p *byte) byte {
	return *p
}

func mid(p *byte) byte {
	return leaf(p)
}

func use(buf []byte) byte {
	return mid(&buf[0])
}
`)

	if !strings.Contains(rust, "fn mid(p: GoPtr<u8>)") ||
		!strings.Contains(rust, "fn leaf(p: GoPtr<u8>)") {
		t.Fatalf("GoPtr pointer parameters should propagate through forwarding calls:\n%s", rust)
	}
	if !strings.Contains(rust, "leaf(p.clone())") {
		t.Fatalf("forwarded GoPtr parameter should be passed through without converting to a local handle:\n%s", rust)
	}
}

func TestGoPtrParamPropagatesThroughNoEscapeForwardingCall(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

//go:noescape
func leaf(p *byte)

func mid(p *byte) {
	leaf(p)
}

func use(buf []byte) {
	mid(&buf[0])
}
`)

	if !strings.Contains(rust, "fn mid(p: GoPtr<u8>)") ||
		!strings.Contains(rust, "fn leaf(p: GoPtr<u8>)") {
		t.Fatalf("forwarded GoPtr pointer parameters should propagate into noescape callees:\n%s", rust)
	}
	if !strings.Contains(rust, "leaf(p.clone())") {
		t.Fatalf("forwarded GoPtr parameter should pass through to noescape callee:\n%s", rust)
	}
}

func TestGoPtrParamPromotesNoEscapeBodylessExistingGoPtrArg(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Type struct {
	Value int
}

//go:noescape
func touch(t *Type)

func raw(addr uintptr) *Type {
	return (*Type)(unsafe.Pointer(addr))
}

func get(addr uintptr) *Type {
	return raw(addr)
}

func use(addr uintptr) {
	t := get(addr)
	touch(t)
	touch(get(addr))
}
`)

	if !strings.Contains(rust, "fn touch(t: GoPtr<Type>)") {
		t.Fatalf("noescape bodyless function should accept existing GoPtr arguments directly:\n%s", rust)
	}
	if !strings.Contains(rust, "touch(t.clone())") {
		t.Fatalf("noescape bodyless call should pass a GoPtr local directly:\n%s", rust)
	}
	if strings.Contains(rust, `unimplemented!("GoPtr parameter argument requires pointer-compatible value")`) {
		t.Fatalf("noescape GoPtr call argument should not hit the unsupported GoPtr path:\n%s", rust)
	}
}

func TestGoPtrParamPromotesBodylessExistingGoPtrArg(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Type struct {
	Value int
}

func touch(t *Type)

func raw(addr uintptr) *Type {
	return (*Type)(unsafe.Pointer(addr))
}

func get(addr uintptr) *Type {
	return raw(addr)
}

func use(addr uintptr) {
	t := get(addr)
	touch(t)
	touch(get(addr))
}
`)

	if !strings.Contains(rust, "fn touch(t: GoPtr<Type>)") {
		t.Fatalf("bodyless function should accept existing GoPtr arguments directly:\n%s", rust)
	}
	if !strings.Contains(rust, "touch(t.clone())") {
		t.Fatalf("bodyless call should pass a GoPtr local directly:\n%s", rust)
	}
	if strings.Contains(rust, `unimplemented!("GoPtr parameter argument requires pointer-compatible value")`) {
		t.Fatalf("bodyless GoPtr call argument should not hit the unsupported GoPtr path:\n%s", rust)
	}
}

func TestGoPtrReturnPropagatesThroughMixedAtomicPointerAndLocalReturns(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "sync/atomic"

type File struct {
	base int
}

type Set struct {
	last atomic.Pointer[File]
	files []*File
}

func (s *Set) file(hit bool) *File {
	if hit {
		f := s.last.Load()
		if f != nil {
			return f
		}
	}
	if len(s.files) > 0 {
		f := s.files[0]
		s.last.Store(f)
		return f
	}
	return nil
}
`)

	if !strings.Contains(rust, "pub fn file(&self") || !strings.Contains(rust, " -> GoPtr<File>") {
		t.Fatalf("mixed atomic/local pointer returns should promote the function result to GoPtr<File>:\n%s", rust)
	}
	if !strings.Contains(rust, "return f.clone();") {
		t.Fatalf("GoPtr-returning branch should preserve the atomic pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "return GoPtr::local(f.clone());") {
		t.Fatalf("ordinary local pointer return should adapt to GoPtr::local:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::nil()") {
		t.Fatalf("nil pointer return should adapt to GoPtr::nil:\n%s", rust)
	}
}

func TestGoPtrMethodArgAndSelectorReadUseGoPtr(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type pair struct {
	v int
}

func (p *pair) merge(other *pair) {
	p.v += other.v
}

func use(items []pair) {
	items[0].merge(&items[1])
}
`)

	if !strings.Contains(rust, "pub fn merge(&mut self, other: GoPtr<pair>)") {
		t.Fatalf("method pointer parameter should use GoPtr when passed a slice element address:\n%s", rust)
	}
	if !strings.Contains(rust, ".merge(GoPtr::slice_elem(GoSliceElemPtr::new(items.clone(), (1) as usize)))") {
		t.Fatalf("method call argument should wrap slice element address in GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "other.lock()") {
		t.Fatalf("selector reads from GoPtr params should not use wrapper lock directly:\n%s", rust)
	}
	if !strings.Contains(rust, "other.borrow()") {
		t.Fatalf("selector reads from GoPtr params should borrow through GoPtr:\n%s", rust)
	}
}

func TestGoPtrPromotedMethodForwardingUsesGoPtrParam(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type base struct {
	v int
}

func (b *base) merge(other *base) {
	b.v += other.v
}

type outer struct {
	base
}

func use(dst *outer, items []base) {
	dst.merge(&items[0])
}
`)

	if !strings.Contains(rust, "pub fn merge(&mut self, other: GoPtr<base>)") {
		t.Fatalf("promoted method forwarding should keep the embedded method's GoPtr parameter type:\n%s", rust)
	}
	if !strings.Contains(rust, "embedded_ref.merge(other)") {
		t.Fatalf("promoted method should forward the GoPtr parameter to the embedded method:\n%s", rust)
	}
	if strings.Contains(rust, "pub fn merge(&mut self, other: Rc<RefCell<Option<base>>>") ||
		strings.Contains(rust, "pub fn merge(&mut self, other: Arc<Mutex<Option<base>>>") {
		t.Fatalf("promoted method forwarding should not keep the old local pointer wrapper parameter:\n%s", rust)
	}
}

func TestGoPtrPromotedMethodForwardingUsesGoPtrReturn(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type item struct {
	v int
}

type base struct{}

func (b *base) pick(items []item) *item {
	p := &items[0]
	return p
}

type outer struct {
	base
}

func use(o *outer, items []item) *item {
	return o.pick(items)
}
`)

	if !strings.Contains(rust, "pub fn pick(&self, items: ") || !strings.Contains(rust, " -> GoPtr<item>") {
		t.Fatalf("promoted method forwarding should keep the embedded method's GoPtr return type:\n%s", rust)
	}
	if !strings.Contains(rust, "embedded_ref.pick(items)") {
		t.Fatalf("promoted method should forward the embedded GoPtr-returning call:\n%s", rust)
	}
	if strings.Contains(rust, "pub fn pick(&self, items: ") &&
		(strings.Contains(rust, " -> Rc<RefCell<Option<item>>>") ||
			strings.Contains(rust, " -> Arc<Mutex<Option<item>>>")) {
		t.Fatalf("promoted method forwarding should not keep the old local pointer wrapper return:\n%s", rust)
	}
}

func TestGoPtrParamPropagatesThroughFuncLiteralForwardingCall(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type node struct {
	value int
}

func sink(p *node) {
	p.value = 1
}

func outer(p *node) {
	func() {
		sink(p)
	}()
}

func use(items []node) {
	outer(&items[0])
}
`)

	if !strings.Contains(rust, "pub fn outer(mut p: GoPtr<node>)") &&
		!strings.Contains(rust, "pub fn outer(p: GoPtr<node>)") {
		t.Fatalf("outer should receive the slice element pointer as GoPtr:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn sink(mut p: GoPtr<node>)") &&
		!strings.Contains(rust, "pub fn sink(p: GoPtr<node>)") {
		t.Fatalf("callee reached only through a function literal should receive GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "sink(p.clone())") && strings.Contains(rust, "pub fn sink(mut p: Arc<Mutex<Option<node>>>") {
		t.Fatalf("function literal forwarding should not leave the callee expecting the old wrapper type:\n%s", rust)
	}
}

func TestGoPtrPointerSwitchUsesPointerIdentity(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

func match(p *byte, q *byte) int {
	switch p {
	case q:
		return 1
	case nil:
		return 2
	}
	return 0
}

func use(buf []byte) int {
	return match(&buf[0], &buf[0])
}
`)

	if !strings.Contains(rust, "fn r#match(p: GoPtr<u8>, q: GoPtr<u8>)") {
		t.Fatalf("pointer switch params receiving slice element addresses should use GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "p.lock()") || strings.Contains(rust, "let __switch_guard =") {
		t.Fatalf("GoPtr pointer switch should not use wrapper lock directly:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::ptr_eq(&__switch_val, &__case)") &&
		!strings.Contains(rust, "GoPtr::ptr_eq(&_switch_val, &__case)") {
		t.Fatalf("GoPtr pointer switch should compare GoPtr identities:\n%s", rust)
	}
}

func TestGoPtrPointerSwitchAddressOfFieldCaseUsesPointerIdentity(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type mutex struct {
	state int
}

type scheduler struct {
	lock mutex
}

var sched scheduler

func prefer(l *mutex) bool {
	switch l {
	case &sched.lock:
		return true
	default:
		return false
	}
}

func use(buf []mutex) bool {
	return prefer(&buf[0])
}
`)

	if !strings.Contains(rust, "fn prefer(l: GoPtr<mutex>)") {
		t.Fatalf("pointer switch param receiving slice element addresses should use GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "GoPtr switch case requires pointer-compatible value") {
		t.Fatalf("address-of-field GoPtr switch case should not fall back to an unsupported case:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::ptr_eq(&__switch_val, &__case)") &&
		!strings.Contains(rust, "GoPtr::ptr_eq(&_switch_val, &__case)") {
		t.Fatalf("address-of-field GoPtr switch case should compare GoPtr identities:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::local(") || !strings.Contains(rust, ".lock.clone()") {
		t.Fatalf("address-of-field GoPtr switch case should wrap the field handle as a local GoPtr:\n%s", rust)
	}
}

func TestGoPtrPointerSwitchAddressOfCrossModuleFieldCaseUsesPointerIdentity(t *testing.T) {
	prevTypeInfo := currentTypeInfo
	prevContext := currentContext
	prevSourceFunctionDecls := sourceFunctionDeclsByFunc
	t.Cleanup(func() {
		currentTypeInfo = prevTypeInfo
		currentContext = prevContext
		sourceFunctionDeclsByFunc = prevSourceFunctionDecls
	})

	fset := token.NewFileSet()
	runtime2File, err := parser.ParseFile(fset, "runtime2.go", `package main

type mutex struct {
	state int
}

type scheduler struct {
	lock mutex
}

var sched scheduler
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(runtime2.go) error = %v", err)
	}
	lockFile, err := parser.ParseFile(fset, "lock_spinbit.go", `package main

func prefer(l *mutex) bool {
	switch l {
	case &sched.lock:
		return true
	default:
		return false
	}
}

func use(buf []mutex) bool {
	return prefer(&buf[0])
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(lock_spinbit.go) error = %v", err)
	}
	typeInfo, err := NewTypeInfo([]*ast.File{runtime2File, lockFile}, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}
	sourceDecls := make(map[*types.Func]sourceFunctionDeclInfo)
	for _, decl := range lockFile.Decls {
		fnDecl, ok := decl.(*ast.FuncDecl)
		if !ok {
			continue
		}
		fn, ok := typeInfo.info.Defs[fnDecl.Name].(*types.Func)
		if ok && fn != nil {
			sourceDecls[fn] = sourceFunctionDeclInfo{decl: fnDecl, info: typeInfo.info}
		}
	}
	SetSourceFunctionDeclsByFunc(sourceDecls)

	pkgState := NewPackageState()
	pkgState.TypeModuleNames["mutex"] = "runtime2"
	pkgState.TypeModuleNames["scheduler"] = "runtime2"
	for fn := range sourceDecls {
		if fn.Name() == "prefer" {
			pkgState.GoPtrParamFuncs[fn] = map[int]string{0: "mutex"}
		}
	}
	SetTranspileContext(&TranspileContext{
		Session:           NewTranspileSession(typeInfo, nil),
		Package:           pkgState,
		CurrentModuleName: "lock_spinbit",
	})

	rust, _, _ := TranspileWithMapping(lockFile, fset, typeInfo, nil)
	if !strings.Contains(rust, "fn prefer(l: GoPtr<") {
		t.Fatalf("cross-module pointer switch param receiving slice element addresses should use GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "GoPtr switch case requires pointer-compatible value") {
		t.Fatalf("cross-module address-of-field GoPtr switch case should not fall back to an unsupported case:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::ptr_eq(&__switch_val, &__case)") &&
		!strings.Contains(rust, "GoPtr::ptr_eq(&_switch_val, &__case)") {
		t.Fatalf("cross-module address-of-field GoPtr switch case should compare GoPtr identities:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::local(") || !strings.Contains(rust, ".lock.clone()") {
		t.Fatalf("cross-module address-of-field GoPtr switch case should wrap the field handle as a local GoPtr:\n%s", rust)
	}
}

func TestGoPtrPointerToArrayIndexBorrowsPointee(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

func load(raw unsafe.Pointer) byte {
	q := (*[8]byte)(raw)
	return q[7]
}
`)

	if !strings.Contains(rust, "let mut q: GoPtr<[u8; 8]>") {
		t.Fatalf("unsafe pointer conversion to array pointer should use GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "q[(7) as usize]") {
		t.Fatalf("GoPtr pointer-to-array index should not index GoPtr directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let __seq = q.borrow()") ||
		!strings.Contains(rust, "__seq.as_ref().unwrap()[(7) as usize].clone()") {
		t.Fatalf("GoPtr pointer-to-array index should borrow the pointed-to array:\n%s", rust)
	}
}

func TestGoPtrPointerToArrayIndexAssignmentMutatesPointee(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

func store(raw unsafe.Pointer, value byte) {
	q := (*[2]byte)(raw)
	q[0] = value
	q[1] = 3
}
`)

	if !strings.Contains(rust, "let mut q: GoPtr<[u8; 2]>") {
		t.Fatalf("unsafe pointer conversion to array pointer should use GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "q.lock().unwrap().as_mut().unwrap()") ||
		strings.Contains(rust, "q.borrow_mut().as_mut().unwrap()") {
		t.Fatalf("GoPtr pointer-to-array assignment should not use ordinary wrapper mutation:\n%s", rust)
	}
	if !strings.Contains(rust, "q.with_mut(|__seq| { __seq[(0) as usize] = new_val; })") ||
		!strings.Contains(rust, "q.with_mut(|__seq| { __seq[(1) as usize] = new_val; })") {
		t.Fatalf("GoPtr pointer-to-array assignment should mutate through GoPtr::with_mut:\n%s", rust)
	}
}

func TestGoPtrUnsafePointerToUintptrUsesGoPtrAddress(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

func addr(p *byte) uintptr {
	return uintptr(unsafe.Pointer(p))
}

func use(buf []byte) uintptr {
	return addr(&buf[0])
}
`)

	if !strings.Contains(rust, "fn addr(p: GoPtr<u8>)") {
		t.Fatalf("unsafe pointer address helper should receive slice element address as GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "Arc::as_ptr(&p)") || strings.Contains(rust, "Rc::as_ptr(&p)") {
		t.Fatalf("unsafe.Pointer conversion from GoPtr should not call wrapper as_ptr on GoPtr:\n%s", rust)
	}
	if !strings.Contains(rust, "p.addr()") {
		t.Fatalf("unsafe.Pointer conversion from GoPtr should use the GoPtr address token:\n%s", rust)
	}
}

func TestGoPtrUnsafePointerToUintptrUsesCallResultAddress(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type Type struct {
	Value int
}

type Value struct {
	ptr uintptr
}

func raw(addr uintptr) *Type {
	return (*Type)(unsafe.Pointer(addr))
}

func (v Value) typ() *Type {
	return raw(v.ptr)
}

func addr(v Value) uintptr {
	return uintptr(unsafe.Pointer(v.typ()))
}
`)

	if strings.Contains(rust, "Arc::as_ptr(&(*v.borrow().as_ref().unwrap()).typ())") ||
		strings.Contains(rust, "Rc::as_ptr(&(*v.borrow().as_ref().unwrap()).typ())") {
		t.Fatalf("unsafe.Pointer conversion from a GoPtr call result should not call wrapper as_ptr on GoPtr:\n%s", rust)
	}
	if !strings.Contains(rust, ".typ().addr()") {
		t.Fatalf("unsafe.Pointer conversion from a GoPtr call result should use the GoPtr address token:\n%s", rust)
	}
}

func TestGoPtrMethodParamUnsafePointerToUintptrUsesGoPtrAddress(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type recorder struct{}

func (rec *recorder) addr(p *byte) uintptr {
	return uintptr(unsafe.Pointer(p))
}

func use(buf []byte, rec *recorder) uintptr {
	return rec.addr(&buf[0])
}
`)

	if !strings.Contains(rust, "pub fn addr(&self, p: GoPtr<u8>)") {
		t.Fatalf("method pointer parameter should receive slice element address as GoPtr:\n%s", rust)
	}
	if strings.Contains(rust, "Arc::as_ptr(&p)") || strings.Contains(rust, "Rc::as_ptr(&p)") {
		t.Fatalf("unsafe.Pointer conversion from method GoPtr parameter should not call wrapper as_ptr on GoPtr:\n%s", rust)
	}
	if !strings.Contains(rust, "p.addr()") {
		t.Fatalf("unsafe.Pointer conversion from method GoPtr parameter should use the GoPtr address token:\n%s", rust)
	}
}

func TestArrayElemPointerTupleReturnAssignedFieldUsesGoPtrField(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type node struct {
	next *node
}

type buf struct {
	items [4]node
}

type state struct {
	root *node
}

func pick(b *buf, i int) (*node, *buf, int) {
	return &b.items[i], b, i
}

func build(s *state, b *buf) {
	s.root, _, _ = pick(b, 0)
}
`)

	if !strings.Contains(rust, "pub root: GoPtr<node>") {
		t.Fatalf("field assigned an array element pointer return should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = GoPtr::array_elem_opt(__tmp_0.clone());") ||
		!strings.Contains(rust, ".root = new_val;") {
		t.Fatalf("tuple assignment into GoPtr field should preserve array element identity:\n%s", rust)
	}
}

func TestArrayElemPointerRecursiveCallResultAnalysisDoesNotRecurse(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type node struct {
	next *node
}

type buf struct {
	items [4]node
}

func pick(b *buf, i int) (root *node, rest *buf, out int) {
	if i > 0 {
		child, _, _ := pick(b, i-1)
		_ = child
	}
	root = &b.items[i]
	return root, b, i
}
`)

	if !strings.Contains(rust, "Option<GoArrayElemPtr<node, 4>>") {
		t.Fatalf("recursive result analysis should still classify direct array element pointer returns:\n%s", rust)
	}
}

func TestRecursiveArrayElemPointerTupleLocalsRegisterGoPtrFields(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type node struct {
	left *node
	right *node
}

type buf struct {
	items [4]node
}

func build(b *buf, i int, n int) (root *node, rest *buf, out int) {
	var left, right *node
	if n > 1 {
		left, b, i = build(b, i, n/2)
	}
	root = &b.items[i]
	if n > 2 {
		right, b, i = build(b, i, n-n/2-1)
	}
	root.left = left
	root.right = right
	return root, b, i
}
`)

	if !strings.Contains(rust, "pub left: GoPtr<node>") || !strings.Contains(rust, "pub right: GoPtr<node>") {
		t.Fatalf("fields assigned recursive tuple GoPtr locals should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut left: GoPtr<node> = GoPtr::nil()") ||
		!strings.Contains(rust, "let mut right: GoPtr<node> = GoPtr::nil()") {
		t.Fatalf("recursive tuple locals receiving array element pointer results should use GoPtr storage:\n%s", rust)
	}
	if strings.Contains(rust, "GoPtr::local(left.clone())") || strings.Contains(rust, "GoPtr::local(right.clone())") {
		t.Fatalf("assigning recursive tuple GoPtr locals into fields should not re-wrap existing GoPtr handles:\n%s", rust)
	}
	if !strings.Contains(rust, ".left = new_val;") || !strings.Contains(rust, ".right = new_val;") {
		t.Fatalf("recursive tuple GoPtr local field assignments should store through GoPtr fields:\n%s", rust)
	}
}

func TestGoPtrFieldValueMethodCallBorrowsThroughGoPtr(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type bits struct {
	value int
}

func (b *bits) bitp(i int) int {
	b.value += i
	return b.value + i
}

type heap struct {
	marks *bits
}

func initHeap(h *heap, all []bits) {
	h.marks = &all[0]
}

func use(h *heap) int {
	return h.marks.bitp(3)
}

func forceConcurrent(ch chan bool) {
	go func() {
		ch <- true
	}()
}
`)

	if !strings.Contains(rust, "pub marks: GoPtr<bits>") {
		t.Fatalf("field assigned a slice element pointer should use GoPtr storage:\n%s", rust)
	}
	if strings.Contains(rust, ".marks.lock()") {
		t.Fatalf("GoPtr field method call should not call lock on GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "let __result = __recv_field.with_mut(|__recv_value| __recv_value.bitp(") {
		t.Fatalf("GoPtr field pointer-receiver method call should dispatch through with_mut:\n%s", rust)
	}
}

func TestGoPtrPointerReceiverMethodValueUsesGoPtrReceiver(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type entry struct{}

type table struct {
	root *entry
	count int
}

func rawTable(addr uintptr) *table {
	return (*table)(unsafe.Pointer(addr))
}

func rawEntry(addr uintptr) *entry {
	return (*entry)(unsafe.Pointer(addr))
}

func initTable(t *table, entries []entry) {
	t.root = &entries[0]
}

func (t *table) add(e *entry) {
	if t.root == e {
		t.count++
	}
}

func iterate(fn func(*entry)) {}

func copy(addr uintptr) {
	t := rawTable(addr)
	t.add(rawEntry(addr))
	iterate(t.add)
}
`)

	if strings.Contains(rust, "__recv.lock()") || strings.Contains(rust, "__recv.borrow_mut()") {
		t.Fatalf("method value bound to a GoPtr receiver should not borrow it as a wrapper handle:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn add(&mut self, e: GoPtr<entry>)") {
		t.Fatalf("method parameter promoted to GoPtr should be reflected in the generated method signature:\n%s", rust)
	}
	if !strings.Contains(rust, "__recv.with_mut(|__recv_value| __recv_value.add(GoPtr::local(__arg0)))") {
		t.Fatalf("method value bound to a GoPtr receiver should dispatch through GoPtr::with_mut and adapt GoPtr-promoted args:\n%s", rust)
	}
}

func TestGoPtrLocalReassignedFromFieldAddressToArrayElementAddress(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type cell struct {
	value int
}

func (c *cell) Load() int {
	return c.value
}

func (c *cell) Store(v int) {
	c.value = v
}

type node struct {
	children [4]cell
}

type table struct {
	root cell
}

func walk(t *table, n *node, i int) int {
	m := &t.root
	m.Store(1)
	m = &n.children[i]
	m.Store(2)
	return m.Load()
}
`)

	if !strings.Contains(rust, "let mut m: GoPtr<cell> = GoPtr::local(") {
		t.Fatalf("pointer local initialized from a field and later assigned an array element should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "m = GoPtr::array_elem(GoArrayElemPtr::new(") {
		t.Fatalf("GoPtr local assignment from an array element address should preserve array element identity:\n%s", rust)
	}
	if strings.Contains(rust, `unimplemented!("GoPtr local method call requires mutable receiver support")`) {
		t.Fatalf("GoPtr local pointer-receiver methods should lower without a placeholder:\n%s", rust)
	}
	if !strings.Contains(rust, "m.with_mut(|__recv_value| __recv_value.store(") {
		t.Fatalf("mutating GoPtr local method call should dispatch through with_mut:\n%s", rust)
	}
	if !strings.Contains(rust, "let __recv_value = m.borrow(); let __result = (*__recv_value.as_ref().unwrap()).load(") {
		t.Fatalf("read-only GoPtr local method call should borrow through GoPtr:\n%s", rust)
	}
}

func TestGoPtrLocalPointerReturningMethodCallUsesOriginalPointee(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct {
	value int
}

type entry struct {
	node
}

func pick(addr uintptr) (*node, int) {
	return (*node)(unsafe.Pointer(addr)), 0
}

func (n *node) entry() *entry {
	return (*entry)(unsafe.Pointer(n))
}

func use(addr uintptr) *entry {
	n, _ := pick(addr)
	return n.entry()
}
`)

	if strings.Contains(rust, "let __recv_value = n.borrow(); let __result = (*__recv_value.as_ref().unwrap()).entry(") {
		t.Fatalf("GoPtr local pointer-returning method call should not call through a cloned pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "n.with_mut(|__recv_value| __recv_value.entry(") {
		t.Fatalf("GoPtr local pointer-returning method call should use the original pointee:\n%s", rust)
	}
}

func TestGoPtrLocalUnsafeReceiverIdentityMethodCallUsesOriginalPointee(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct {
	isEntry bool
}

type entry struct {
	node
	value int
}

func pick(addr uintptr) (*node, int) {
	return (*node)(unsafe.Pointer(addr)), 0
}

func (n *node) value() int {
	if !n.isEntry {
		return 0
	}
	return (*entry)(unsafe.Pointer(n)).value
}

func use(addr uintptr) int {
	n, _ := pick(addr)
	return n.value()
}
`)

	if strings.Contains(rust, "let __recv_value = n.borrow(); let __result = (*__recv_value.as_ref().unwrap()).value(") {
		t.Fatalf("GoPtr local unsafe receiver-identity method call should not call through a cloned pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "n.with_mut(|__recv_value| __recv_value.value(") {
		t.Fatalf("GoPtr local unsafe receiver-identity method call should use the original pointee:\n%s", rust)
	}
}

func TestGoPtrLocalFieldHandleUsesOriginalPointee(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type cell struct {
	value int
}

func (c *cell) Store(v int) {
	c.value = v
}

func (c *cell) Load() int {
	return c.value
}

type node struct {
	children [4]cell
}

func pick(addr uintptr) *node {
	return (*node)(unsafe.Pointer(addr))
}

func touch(addr uintptr, i int) int {
	n := pick(addr)
	c := &n.children[i]
	c.Store(7)
	return n.children[i].Load()
}
`)

	if strings.Contains(rust, "let __ptr_value = n.borrow(); __ptr_value.as_ref().unwrap().children.clone()") {
		t.Fatalf("GoPtr local field handle should not select from a cloned pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "n.with_mut(|__ptr_value| __ptr_value.children.clone())") {
		t.Fatalf("GoPtr local field handle should borrow the original pointee:\n%s", rust)
	}
}

func TestGoPtrLocalFieldReadHandleBindsCloneBeforeReturning(t *testing.T) {
	var out strings.Builder
	writeGoPtrLocalFieldReadHandle(&out, ast.NewIdent("b"), FieldAccessInfo{
		EmbeddedPath: []string{"workbufhdr"},
		FieldName:    "nobj",
	})

	rust := out.String()
	if !strings.Contains(rust, "let __field_value = __ptr_value.as_ref().unwrap().workbufhdr") ||
		!strings.Contains(rust, "; __field_value }") {
		t.Fatalf("GoPtr field read handle should bind the cloned field before returning it:\n%s", rust)
	}
}

func TestGoPtrLocalFieldAssignmentBorrowsOriginalPointee(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct {
	next *node
}

func raw(addr uintptr) *node {
	return (*node)(unsafe.Pointer(addr))
}

func link(addr uintptr) {
	n := raw(addr)
	n.next = raw(addr)
}
`)

	if strings.Contains(rust, "{ let __ptr_value = n.with_mut(|__ptr_value| __ptr_value.next.clone()); __ptr_value } =") {
		t.Fatalf("GoPtr local field assignment should not use the read-handle expression as an assignment target:\n%s", rust)
	}
	if !strings.Contains(rust, "n.with_mut(|__ptr_value| { __ptr_value.next = new_val; });") {
		t.Fatalf("GoPtr local field assignment should mutate through the original pointee:\n%s", rust)
	}
}

func TestGoPtrLocalSliceFieldAssignmentBorrowsOriginalPointee(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type histogram struct {
	buckets []int
}

func raw(addr uintptr) *histogram {
	return (*histogram)(unsafe.Pointer(addr))
}

func store(addr uintptr, buckets []int) {
	h := raw(addr)
	h.buckets = buckets
}
`)

	if strings.Contains(rust, "{ let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.buckets.clone()); __ptr_value } =") {
		t.Fatalf("GoPtr local slice field assignment should not use the read-handle expression as an assignment target:\n%s", rust)
	}
	if !strings.Contains(rust, "h.with_mut(|__ptr_value| { __ptr_value.buckets = new_val; });") {
		t.Fatalf("GoPtr local slice field assignment should mutate through the original pointee:\n%s", rust)
	}
}

func TestGoPtrLocalPromotedPointerFieldAssignmentBorrowsOriginalPointee(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type header struct {
	next *node
}

type node struct {
	header
}

func raw(addr uintptr) *node {
	return (*node)(unsafe.Pointer(addr))
}

func link(addr uintptr) {
	n := raw(addr)
	other := raw(addr + 1)
	n.next = nil
	n.next = other
}
`)

	if strings.Contains(rust, "{ let __ptr_value = n.with_mut(|__ptr_value| { let __field = __ptr_value.header") &&
		strings.Contains(rust, "__field }); __ptr_value } = new_val") {
		t.Fatalf("GoPtr local promoted pointer field assignment should not use the read-handle expression as an assignment target:\n%s", rust)
	}
	if !strings.Contains(rust, "n.with_mut(|__ptr_value| { (*__ptr_value.header") ||
		!strings.Contains(rust, ".next = new_val; });") {
		t.Fatalf("GoPtr local promoted pointer field assignment should mutate through the embedded pointee:\n%s", rust)
	}
}

func TestParallelGoPtrLocalFieldAssignmentBorrowsOriginalPointee(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct {
	next *node
}

func raw(addr uintptr) *node {
	return (*node)(unsafe.Pointer(addr))
}

func rotate(addr uintptr) {
	n := raw(addr)
	other := raw(addr + 1)
	n.next, other = other, n
}
`)

	if strings.Contains(rust, "{ let __ptr_value = n.with_mut(|__ptr_value| __ptr_value.next.clone()); __ptr_value } =") {
		t.Fatalf("parallel GoPtr local field assignment should not use the read-handle expression as an assignment target:\n%s", rust)
	}
	if !strings.Contains(rust, "n.with_mut(|__ptr_value| { __ptr_value.next = __tmp_0.clone(); });") {
		t.Fatalf("parallel GoPtr local field assignment should mutate through the original pointee:\n%s", rust)
	}
}

func TestCrossFileParallelGoPtrFieldAssignmentConvergesFieldFacts(t *testing.T) {
	fset := token.NewFileSet()
	typesFile, err := parser.ParseFile(fset, "types.go", `package main

type node struct {
	next *node
}

type heap struct {
	root *node
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(types.go) error = %v", err)
	}
	useFile, err := parser.ParseFile(fset, "use.go", `package main

import "unsafe"

func raw(addr uintptr) *node {
	return (*node)(unsafe.Pointer(addr))
}

func link(h *heap, addr uintptr) {
	n := raw(addr)
	n.next, h.root = h.root, n
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(use.go) error = %v", err)
	}
	files := []*ast.File{typesFile, useFile}
	typeInfo, err := NewTypeInfo(files, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	prevTypeInfo := currentTypeInfo
	prevContext := currentContext
	prevVarTable := currentVarTable
	t.Cleanup(func() {
		SetTypeInfo(prevTypeInfo)
		SetTranspileContext(prevContext)
		SetVarTable(prevVarTable)
	})

	SetTypeInfo(typeInfo)
	ctx := &TranspileContext{
		Session: NewTranspileSession(typeInfo, nil),
		Package: NewPackageState(),
	}
	SetTranspileContext(ctx)
	registerSliceElemPtrFactsFromFiles(files)

	rust, _, _ := TranspileWithMapping(typesFile, fset, typeInfo, nil)
	if !strings.Contains(rust, "pub next: GoPtr<node>") || !strings.Contains(rust, "pub root: GoPtr<node>") {
		t.Fatalf("cross-file parallel pointer-field facts should converge before struct emission:\n%s", rust)
	}

	rust, _, _ = TranspileWithMapping(useFile, fset, typeInfo, nil)
	if !strings.Contains(rust, "n.with_mut(|__ptr_value| { __ptr_value.next = __tmp_0.clone(); });") {
		t.Fatalf("cross-file parallel GoPtr field assignment should store the GoPtr handle:\n%s", rust)
	}
}

func TestCrossFileGoPtrParamFieldAssignmentAndAddressPromotesField(t *testing.T) {
	fset := token.NewFileSet()
	typesFile, err := parser.ParseFile(fset, "types.go", `package main

type abiType struct {
	ptrBytes uintptr
}

type mspan struct {
	largeType *abiType
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(types.go) error = %v", err)
	}
	useFile, err := parser.ParseFile(fset, "use.go", `package main

import "unsafe"

func raw(addr uintptr) *abiType {
	return (*abiType)(unsafe.Pointer(addr))
}

func rawSpan(addr uintptr) *mspan {
	return (*mspan)(unsafe.Pointer(addr))
}

func set(span *mspan, typ *abiType) {
	gctyp := typ
	span.largeType = gctyp
	check(&span.largeType)
}

func check(slot **abiType) bool {
	return *slot != nil
}

func call(spanAddr uintptr, typAddr uintptr) {
	set(rawSpan(spanAddr), raw(typAddr))
}
`, parser.ParseComments)
	if err != nil {
		t.Fatalf("ParseFile(use.go) error = %v", err)
	}
	files := []*ast.File{typesFile, useFile}
	typeInfo, err := NewTypeInfo(files, fset)
	if err != nil {
		t.Fatalf("NewTypeInfo() error = %v", err)
	}

	prevTypeInfo := currentTypeInfo
	prevContext := currentContext
	prevVarTable := currentVarTable
	prevSourceFunctionDecls := sourceFunctionDeclsByFunc
	t.Cleanup(func() {
		SetTypeInfo(prevTypeInfo)
		SetTranspileContext(prevContext)
		SetVarTable(prevVarTable)
		SetSourceFunctionDeclsByFunc(prevSourceFunctionDecls)
	})

	sourceDecls := make(map[*types.Func]sourceFunctionDeclInfo)
	for _, decl := range useFile.Decls {
		fnDecl, ok := decl.(*ast.FuncDecl)
		if !ok {
			continue
		}
		fn, ok := typeInfo.info.Defs[fnDecl.Name].(*types.Func)
		if ok && fn != nil {
			sourceDecls[fn] = sourceFunctionDeclInfo{decl: fnDecl, info: typeInfo.info}
		}
	}
	SetSourceFunctionDeclsByFunc(sourceDecls)
	SetTypeInfo(typeInfo)
	ctx := &TranspileContext{
		Session: NewTranspileSession(typeInfo, nil),
		Package: NewPackageState(),
	}
	SetTranspileContext(ctx)
	registerSliceElemPtrFactsFromFiles(files)

	rust, _, _ := TranspileWithMapping(typesFile, fset, typeInfo, nil)
	if !strings.Contains(rust, "pub large_type: GoPtr<abiType>") {
		t.Fatalf("cross-file field assigned from a GoPtr parameter should use GoPtr storage before struct emission:\n%s", rust)
	}

	rust, _, _ = TranspileWithMapping(useFile, fset, typeInfo, nil)
	if !strings.Contains(rust, "pub fn set(span: GoPtr<mspan>, typ: GoPtr<abiType>)") {
		t.Fatalf("callee receiving a raw GoPtr result should promote the pointer parameter:\n%s", rust)
	}
	if !strings.Contains(rust, "span.with_mut(|__ptr_value| { __ptr_value.large_type = new_val; });") {
		t.Fatalf("field assignment from a promoted GoPtr parameter should store the pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "check(GoPtr::local(") || !strings.Contains(rust, ".large_type.clone()") {
		t.Fatalf("addressing the promoted field should pass a local GoPtr slot handle:\n%s", rust)
	}
}

func TestGoPtrLocalAssignmentNilComparisonAndReturnPreservesHandle(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type node struct {
	off int
	size int
	left *node
	right *node
}

type buf struct {
	items [4]node
}

type state struct {
	root *node
}

func pick(b *buf, i int) *node {
	return &b.items[i]
}

func initState(s *state, b *buf) {
	s.root = pick(b, 0)
	obj := s.root
	obj.left = pick(b, 1)
	obj.right = pick(b, 2)
}

func find(s *state, a int) *node {
	obj := s.root
	for obj != nil {
		if a < obj.off {
			obj = obj.left
			continue
		}
		if a >= obj.off+obj.size {
			obj = obj.right
			continue
		}
		return obj
	}
	return nil
}
`)

	if !strings.Contains(rust, "pub fn find(") || !strings.Contains(rust, " -> GoPtr<node>") {
		t.Fatalf("function returning a GoPtr local should return GoPtr<node>:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut obj: GoPtr<node> =") {
		t.Fatalf("local copied from a GoPtr field should be declared as GoPtr<node>:\n%s", rust)
	}
	if strings.Contains(rust, "obj.lock()") || strings.Contains(rust, "obj.borrow().is_none()") {
		t.Fatalf("GoPtr local nil comparison should use GoPtr::is_nil, not wrapper borrowing:\n%s", rust)
	}
	if !strings.Contains(rust, "obj.is_nil()") {
		t.Fatalf("GoPtr local nil comparison should call is_nil:\n%s", rust)
	}
	if (!strings.Contains(rust, ".left.clone() }.clone()") && !strings.Contains(rust, ".left.clone(); __field_value }")) ||
		(!strings.Contains(rust, ".right.clone() }.clone()") && !strings.Contains(rust, ".right.clone(); __field_value }")) {
		t.Fatalf("GoPtr local assignment from GoPtr fields should clone the field handle:\n%s", rust)
	}
	if !strings.Contains(rust, "return obj.clone();") {
		t.Fatalf("returning a GoPtr local should clone the GoPtr handle:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::nil()") {
		t.Fatalf("nil return from a GoPtr-returning function should produce GoPtr::nil():\n%s", rust)
	}
}

func TestGoPtrNamedResultsAssignedFromGoPtrCallsUseGoPtrSlots(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct {
	value int
}

func (n *node) identity() *node {
	return (*node)(unsafe.Pointer(n))
}

func find(n *node, ok bool) (root *node, child *node) {
	root = n.identity()
	if ok {
		child = n.identity()
		return
	}
	return
}
`)

	if !strings.Contains(rust, "pub fn find(") || !strings.Contains(rust, " -> (GoPtr<node>, GoPtr<node>)") {
		t.Fatalf("named pointer results assigned GoPtr calls should use GoPtr result types:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut root: GoPtr<node> = GoPtr::nil();") ||
		!strings.Contains(rust, "let mut child: GoPtr<node> = GoPtr::nil();") {
		t.Fatalf("named pointer result locals assigned GoPtr calls should use GoPtr slots:\n%s", rust)
	}
	if strings.Contains(rust, "root.lock()") || strings.Contains(rust, "child.lock()") ||
		strings.Contains(rust, "root.borrow()") || strings.Contains(rust, "child.borrow()") {
		t.Fatalf("named GoPtr results should not be returned through ordinary pointer wrapper borrows:\n%s", rust)
	}
	if !strings.Contains(rust, "(root.clone(), child.clone())") {
		t.Fatalf("naked return should clone named GoPtr result handles:\n%s", rust)
	}
}

func TestMixedGoPtrAndArrayElemNamedResultsComposeResultTypes(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

import "unsafe"

type node struct {
	value int
}

func (n *node) read() int {
	return n.value
}

func (n *node) identity() *node {
	return (*node)(unsafe.Pointer(n))
}

type Pointer struct {
	value *node
}

type table struct {
	slots [2]Pointer
}

func pickSlot(t *table, i int) *Pointer {
	return &t.slots[i]
}

func find(n *node, t *table) (root *node, slot *Pointer, child *node) {
	root = n.identity()
	slot = pickSlot(t, 0)
	child = n.identity()
	return
}

func use(n *node, t *table) int {
	root, _, child := find(n, t)
	if root == nil || child == nil {
		return 0
	}
	return root.read() + child.read()
}
`)

	if !strings.Contains(rust, " -> (GoPtr<node>, Option<GoArrayElemPtr<Pointer, 2>>, GoPtr<node>)") {
		t.Fatalf("mixed GoPtr and array-element named results should compose result types:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut root: GoPtr<node> = GoPtr::nil();") ||
		!strings.Contains(rust, "let mut slot: Option<GoArrayElemPtr<Pointer, 2>> = None;") ||
		!strings.Contains(rust, "let mut child: GoPtr<node> = GoPtr::nil();") {
		t.Fatalf("mixed named result locals should keep their specialized slots:\n%s", rust)
	}
	if !strings.Contains(rust, "(root.clone(), slot.clone(), child.clone())") {
		t.Fatalf("mixed naked return should clone specialized named result handles:\n%s", rust)
	}
	if strings.Contains(rust, "root.lock()") || strings.Contains(rust, "child.lock()") ||
		strings.Contains(rust, "root.borrow().is_none()") || strings.Contains(rust, "child.borrow().is_none()") {
		t.Fatalf("GoPtr tuple result locals should not use ordinary pointer wrapper operations:\n%s", rust)
	}
	if !strings.Contains(rust, "root.is_nil()") || !strings.Contains(rust, "child.is_nil()") {
		t.Fatalf("GoPtr tuple result locals should use GoPtr nil checks:\n%s", rust)
	}
	if !strings.Contains(rust, "let __recv_value = root.borrow(); let __result = (*__recv_value.as_ref().unwrap()).read(") ||
		!strings.Contains(rust, "let __recv_value = child.borrow(); let __result = (*__recv_value.as_ref().unwrap()).read(") {
		t.Fatalf("GoPtr tuple result locals should borrow through GoPtr for method calls:\n%s", rust)
	}
}

func TestGoPtrMethodReturnPreservesLocalHandle(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type node struct {
	off int
	left *node
}

type buf struct {
	items [3]node
}

type state struct {
	root *node
}

func pick(b *buf, i int) *node {
	return &b.items[i]
}

func initState(s *state, b *buf) {
	s.root = pick(b, 0)
	obj := s.root
	obj.left = pick(b, 1)
}

func (s *state) find(a int) *node {
	obj := s.root
	for obj != nil {
		if a < obj.off {
			obj = obj.left
			continue
		}
		return obj
	}
	return nil
}
`)

	if !strings.Contains(rust, "pub fn find(&self") || !strings.Contains(rust, " -> GoPtr<node>") {
		t.Fatalf("method returning a GoPtr local should return GoPtr<node>:\n%s", rust)
	}
	if !strings.Contains(rust, "return obj.clone();") {
		t.Fatalf("method returning a GoPtr local should clone the GoPtr handle:\n%s", rust)
	}
	if !strings.Contains(rust, "GoPtr::nil()") {
		t.Fatalf("nil return from a GoPtr-returning method should produce GoPtr::nil():\n%s", rust)
	}
}

func TestGoPtrFieldAssignedFromTupleLocalUsesGoPtrField(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type node struct {
	left *node
}

type buf struct {
	items [3]node
}

type state struct {
	root *node
}

func pick(b *buf, i int) (*node, *buf) {
	return &b.items[i], b
}

func build(s *state, b *buf) {
	var child *node
	s.root, b = pick(b, 0)
	child, b = pick(b, 1)
	s.root.left = child
}
`)

	if !strings.Contains(rust, "pub left: GoPtr<node>") {
		t.Fatalf("field assigned from a GoPtr tuple local should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "let mut child: GoPtr<node> = GoPtr::nil()") {
		t.Fatalf("tuple local receiving an array element pointer result should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, ".left = new_val;") {
		t.Fatalf("assignment from GoPtr tuple local should update the GoPtr field handle:\n%s", rust)
	}
}

func TestGoPtrFieldAssignedFromGoPtrReturnDoesNotRewrap(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type bits struct {
	value int
}

type heap struct {
	current *bits
}

func initHeap(h *heap, values []bits) {
	h.current = &values[0]
}

func choose(h *heap) *bits {
	p := h.current
	return p
}

func refresh(h *heap) {
	h.current = choose(h)
}
`)

	if !strings.Contains(rust, "pub current: GoPtr<bits>") {
		t.Fatalf("field assigned a slice element pointer should use GoPtr storage:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn choose(") || !strings.Contains(rust, " -> GoPtr<bits>") {
		t.Fatalf("function returning a GoPtr field should return GoPtr<bits>:\n%s", rust)
	}
	if strings.Contains(rust, "GoPtr::local(choose(") {
		t.Fatalf("GoPtr field assignment from GoPtr-returning call should not re-wrap the returned handle:\n%s", rust)
	}
	if !strings.Contains(rust, "let new_val = choose(") || !strings.Contains(rust, ".current = new_val;") {
		t.Fatalf("GoPtr field assignment from GoPtr-returning call should store the returned handle:\n%s", rust)
	}
}

func TestPointerFieldPromotedSelectorDereferencesIntermediatePointerField(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type core struct {
	id int
}

type info struct {
	*core
}

type frame struct {
	fn info
}

func check(fr *frame) int {
	return fr.fn.id
}
`)

	if strings.Contains(rust, ".r#fn.core") {
		t.Fatalf("promoted selector through pointer-valued field should dereference the intermediate pointer field:\n%s", rust)
	}
	if !strings.Contains(rust, ".r#fn") ||
		!strings.Contains(rust, ".as_ref().unwrap()).core") {
		t.Fatalf("promoted selector should borrow the pointer-valued field before selecting embedded fields:\n%s", rust)
	}
}

func TestNoEscapeWritablePointerParamAcceptsSliceElemAddressWithWriteback(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

//go:noescape
func mutate(p *byte)

func use(buf []byte) {
	mutate(&buf[0])
}
`)

	if strings.Contains(rust, `unimplemented!("slice element pointer cannot pass to writable pointer parameter")`) {
		t.Fatalf("noescape writable pointer parameter should use a temporary handle with writeback:\n%s", rust)
	}
	if !strings.Contains(rust, "let __elem_ptr_0 = Some(GoSliceElemPtr::new(buf.clone(), (0) as usize));") {
		t.Fatalf("noescape slice element pointer argument should evaluate the element pointer once:\n%s", rust)
	}
	if !strings.Contains(rust, "__elem_ptr_0.as_ref().and_then(|__ptr| (*__ptr.borrow()).clone())") {
		t.Fatalf("noescape slice element pointer argument should seed the temporary handle from the element:\n%s", rust)
	}
	if !strings.Contains(rust, "mutate(__arg0.clone())") {
		t.Fatalf("noescape slice element pointer argument should pass the temporary pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "*__elem_guard_0 = (*__arg0.borrow()).clone();") &&
		!strings.Contains(rust, "*__elem_guard_0 = (*__arg0.lock().unwrap()).clone();") {
		t.Fatalf("noescape slice element pointer argument should write the temporary handle back:\n%s", rust)
	}
}

func TestNoEscapeWritablePointerParamAcceptsArrayElemCallResultWithWriteback(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

//go:noescape
func mutate(p *byte)

func pick(values *[8]byte) *byte {
	return &(*values)[0]
}

func use(values *[8]byte) {
	mutate(pick(values))
}
`)

	if strings.Contains(rust, "mutate(pick(") {
		t.Fatalf("noescape array element pointer call result should not pass the option directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let __elem_ptr_0 = pick(values.clone());") {
		t.Fatalf("noescape array element pointer call result should evaluate the pointer once:\n%s", rust)
	}
	if !strings.Contains(rust, "__elem_ptr_0.as_ref().and_then(|__ptr| (*__ptr.borrow()).clone())") {
		t.Fatalf("noescape array element pointer call result should seed the temporary handle from the element:\n%s", rust)
	}
	if !strings.Contains(rust, "mutate(__arg0.clone())") {
		t.Fatalf("noescape array element pointer call result should pass the temporary pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "*__elem_guard_0 = (*__arg0.borrow()).clone();") &&
		!strings.Contains(rust, "*__elem_guard_0 = (*__arg0.lock().unwrap()).clone();") {
		t.Fatalf("noescape array element pointer call result should write the temporary handle back:\n%s", rust)
	}
}

func TestNoEscapeWritablePointerParamAcceptsStructArrayFieldElemAddressWithWriteback(t *testing.T) {
	rust := transpileTypedSliceElemPtrRegression(t, `package main

type stats struct {
	counts [4]uint64
}

//go:noescape
func add(p *uint64, delta int64) uint64

func use(s *stats) {
	add(&s.counts[1], 2)
}
`)

	if strings.Contains(rust, "add(GoArrayElemPtr::new") {
		t.Fatalf("noescape struct array field element argument should not pass the element pointer directly:\n%s", rust)
	}
	if !strings.Contains(rust, "let __elem_ptr_0 = Some(GoArrayElemPtr::new(") ||
		!strings.Contains(rust, ".counts.clone(), (1) as usize));") {
		t.Fatalf("noescape struct array field element argument should evaluate the element pointer once:\n%s", rust)
	}
	if !strings.Contains(rust, "__elem_ptr_0.as_ref().and_then(|__ptr| (*__ptr.borrow()).clone())") {
		t.Fatalf("noescape struct array field element argument should seed the temporary handle from the element:\n%s", rust)
	}
	if !strings.Contains(rust, "add(__arg0.clone(),") {
		t.Fatalf("noescape struct array field element argument should pass the temporary pointer handle:\n%s", rust)
	}
	if !strings.Contains(rust, "*__elem_guard_0 = (*__arg0.borrow()).clone();") &&
		!strings.Contains(rust, "*__elem_guard_0 = (*__arg0.lock().unwrap()).clone();") {
		t.Fatalf("noescape struct array field element argument should write the temporary handle back:\n%s", rust)
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

	if strings.Contains(rust, "dump(Rc::new(RefCell::new((*GoSliceElemPtr::new") ||
		strings.Contains(rust, "dump(Arc::new(Mutex::new((*GoSliceElemPtr::new") {
		t.Fatalf("read-only pointer parameter passed through a function literal should not receive a cloned pointee:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn dump(values: GoPtr<Vec<i32>>) -> i32") ||
		!strings.Contains(rust, "Box<dyn FnMut(GoPtr<Vec<i32>>) -> i32>") ||
		!strings.Contains(rust, "(*__f)(values.clone())") ||
		!strings.Contains(rust, "dump(GoPtr::slice_elem(GoSliceElemPtr::new(chunks.clone(), (0) as usize)))") {
		t.Fatalf("read-only pointer parameter passed through a function literal should preserve GoPtr handles:\n%s", rust)
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

	if strings.Contains(rust, ".alloc(i.clone())") ||
		strings.Contains(rust, ".alloc(Rc::new(RefCell::new((*i.as_ref().unwrap().borrow()).clone())))") ||
		strings.Contains(rust, ".alloc(Arc::new(Mutex::new((*i.as_ref().unwrap().borrow()).clone())))") {
		t.Fatalf("read-only method pointer parameter should not receive the slice element pointer option directly:\n%s", rust)
	}
	if !strings.Contains(rust, "pub fn alloc(&self, i: GoPtr<inst>)") ||
		!strings.Contains(rust, ".alloc(GoPtr::slice_elem_opt(i.clone()))") {
		t.Fatalf("read-only method pointer parameter should receive a GoPtr handle:\n%s", rust)
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
	return transpileTypedSliceElemPtrRegressionWithOptions(t, src, false)
}

func transpileTypedSliceElemPtrRegressionWithExternalStubs(t *testing.T, src string) string {
	return transpileTypedSliceElemPtrRegressionWithOptions(t, src, true)
}

func transpileTypedSliceElemPtrRegressionWithOptions(t *testing.T, src string, useExternalStubs bool) string {
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
	file, err := parser.ParseFile(fset, "main.go", src, parser.ParseComments)
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
	if useExternalStubs {
		SetTranspileContext(&TranspileContext{
			Session:                 NewTranspileSession(typeInfo, nil),
			Package:                 NewPackageState(),
			UsePackageExternalStubs: true,
		})
	}
	rust, _, _ := Transpile(file, fset, typeInfo)
	return rust
}
