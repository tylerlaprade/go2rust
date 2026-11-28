package main

import (
	"go/ast"
	"go/types"
	"strings"
)

// PERMANENT: not scaffold. sync is native tier (docs/bridge_debt.md), and
// sync.Map's lock-free source leans on sync/atomic and unsafe pointers to
// entries; a locked Rust map is the long-term implementation.
//
// writeSyncMapStub writes sync.Map: Load, Store, LoadOrStore, LoadAndDelete,
// Delete, Swap, CompareAndSwap, CompareAndDelete, Range and Clear on one
// shared table, so a copy of a struct that holds the map sees the same
// entries.
//
// Callers pass keys and values as they are, not boxed as any, so each
// method is generic over them. An entry keeps its key and value boxed, with
// a function that copies the box, so Load hands back a new interface value
// of the stored type, as Go's would be. A value that arrives already boxed in
// an interface is copied when it holds a string, number or bool; any other
// boxed value keeps the caller's interface cell, which Load then shares.
//
// Keys compare as Go compares interface values: by type and value for
// strings, numbers and bools, and by the printed value for other types,
// which matches Go for structs and pointers to values that differ. A float
// NaN key is never found, as in Go.
func writeSyncMapStub(out *strings.Builder) {
	anyDyn := "dyn Any"
	bound := ""
	shared := "std::rc::Rc<std::cell::RefCell<std::collections::HashMap<GoSyncMapKey, GoSyncMapEntry>>>"
	newShared := "std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()))"
	lockTable := ".borrow_mut()"
	ptrEq := "std::rc::Rc::ptr_eq"
	asPtr := "std::rc::Rc::as_ptr"
	lockCell := ".borrow()"
	lockCellMut := ".borrow_mut()"
	if NeedsConcurrentWrapper() {
		anyDyn = "dyn Any + Send + Sync"
		bound = " + Send + Sync"
		shared = "std::sync::Arc<std::sync::Mutex<std::collections::HashMap<GoSyncMapKey, GoSyncMapEntry>>>"
		newShared = "std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()))"
		lockTable = ".lock().unwrap_or_else(|poisoned| poisoned.into_inner())"
		ptrEq = "std::sync::Arc::ptr_eq"
		asPtr = "std::sync::Arc::as_ptr"
		lockCell = ".lock().unwrap()"
		lockCellMut = ".lock().unwrap()"
	}
	anyBox := "Box<" + anyDyn + ">"
	if bound != "" {
		// "&dyn Any + Send" does not parse.
		anyDyn = "(" + anyDyn + ")"
	}
	anyHandle := wrappedExternalStubType(anyBox)
	code := `#[derive(Clone, Debug, PartialEq)]
pub enum GoSyncMapKey {
    Nil,
    Str(String),
    Int(&'static str, i128),
    Float(&'static str, f64),
    Bool(bool),
    Addr(usize),
    Other(&'static str, String),
}

impl Eq for GoSyncMapKey {}

impl std::hash::Hash for GoSyncMapKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            GoSyncMapKey::Nil => 0u8.hash(state),
            GoSyncMapKey::Str(value) => { 1u8.hash(state); value.hash(state) }
            GoSyncMapKey::Int(kind, value) => { 2u8.hash(state); kind.hash(state); value.hash(state) }
            // -0 and +0 are one key.
            GoSyncMapKey::Float(kind, value) => { 3u8.hash(state); kind.hash(state); (if *value == 0.0 { 0.0f64 } else { *value }).to_bits().hash(state) }
            GoSyncMapKey::Bool(value) => { 4u8.hash(state); value.hash(state) }
            GoSyncMapKey::Addr(value) => { 5u8.hash(state); value.hash(state) }
            GoSyncMapKey::Other(kind, value) => { 6u8.hash(state); kind.hash(state); value.hash(state) }
        }
    }
}

/// The key of a string, number or bool, by its Rust type and value.
fn go_sync_map_known_key(value: &dyn Any) -> Option<GoSyncMapKey> {
    macro_rules! ints {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<$t>() { return Some(GoSyncMapKey::Int(stringify!($t), *v as i128)); })*
        };
    }
    ints!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
    if let Some(v) = value.downcast_ref::<f64>() { return Some(GoSyncMapKey::Float("f64", *v)); }
    if let Some(v) = value.downcast_ref::<f32>() { return Some(GoSyncMapKey::Float("f32", *v as f64)); }
    if let Some(v) = value.downcast_ref::<String>() { return Some(GoSyncMapKey::Str(v.clone())); }
    if let Some(v) = value.downcast_ref::<bool>() { return Some(GoSyncMapKey::Bool(*v)); }
    None
}

fn go_sync_map_key<K: Any + std::fmt::Debug>(key: &K) -> GoSyncMapKey {
    let any: &dyn Any = key;
    if let Some(handle) = any.downcast_ref::<ANY_HANDLE>() {
        return match handle LOCK_CELL.as_ref() {
            None => GoSyncMapKey::Nil,
            Some(inner) => go_sync_map_known_key(&**inner).unwrap_or_else(|| GoSyncMapKey::Addr(&**inner as *const ANY_DYN as *const () as usize)),
        };
    }
    go_sync_map_known_key(any).unwrap_or_else(|| GoSyncMapKey::Other(std::any::type_name::<K>(), format!("{:?}", key)))
}

/// A copy of a boxed string, number or bool, and the function that copies it.
fn go_sync_map_known_clone(value: &ANY_DYN) -> Option<(ANY_BOX, fn(&ANY_DYN) -> ANY_BOX)> {
    macro_rules! known {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<$t>() {
                return Some((Box::new(v.clone()) as ANY_BOX, |stored| Box::new(stored.downcast_ref::<$t>().unwrap().clone()) as ANY_BOX));
            })*
        };
    }
    known!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, String, bool);
    None
}

pub enum GoSyncMapSlot {
    Nil,
    Owned(ANY_BOX, fn(&ANY_DYN) -> ANY_BOX),
    Shared(ANY_HANDLE),
}

impl GoSyncMapSlot {
    fn new<V: Any + ClonePLUS_BOUND>(value: V) -> Self {
        let any: &dyn Any = &value;
        if let Some(handle) = any.downcast_ref::<ANY_HANDLE>() {
            let guard = handle LOCK_CELL;
            let copy = match guard.as_ref() {
                None => return GoSyncMapSlot::Nil,
                Some(inner) => go_sync_map_known_clone(&**inner),
            };
            drop(guard);
            return match copy {
                Some((copy, clone)) => GoSyncMapSlot::Owned(copy, clone),
                None => GoSyncMapSlot::Shared(handle.clone()),
            };
        }
        GoSyncMapSlot::Owned(Box::new(value), |stored| Box::new(stored.downcast_ref::<V>().unwrap().clone()) as ANY_BOX)
    }

    /// A new interface value holding a copy of the slot's value.
    fn handle(&self) -> ANY_HANDLE {
        match self {
            GoSyncMapSlot::Nil => NONE_HANDLE,
            GoSyncMapSlot::Owned(value, clone) => SOME_HANDLE(clone(&**value)),
            GoSyncMapSlot::Shared(handle) => handle.clone(),
        }
    }

    /// Whether the slot holds a value equal to old, as == on interfaces.
    fn holds<O: Any + std::fmt::Debug>(&self, old: &O) -> bool {
        let have = match self {
            GoSyncMapSlot::Nil => GoSyncMapKey::Nil,
            GoSyncMapSlot::Shared(handle) => go_sync_map_key(handle),
            GoSyncMapSlot::Owned(value, _) => match go_sync_map_known_key(&**value) {
                Some(key) => key,
                None => match value.downcast_ref::<O>() {
                    Some(stored) => GoSyncMapKey::Other(std::any::type_name::<O>(), format!("{:?}", stored)),
                    None => return false,
                },
            },
        };
        have == go_sync_map_key(old)
    }
}

pub struct GoSyncMapEntry {
    key: GoSyncMapSlot,
    value: GoSyncMapSlot,
}

#[derive(Clone)]
pub struct sync_Map {
    __go_entries: SHARED,
}

impl Default for sync_Map {
    fn default() -> Self {
        sync_Map { __go_entries: NEW_SHARED }
    }
}

impl std::fmt::Debug for sync_Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "sync_Map")
    }
}

impl std::fmt::Display for sync_Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<sync_Map>")
    }
}

impl PartialEq for sync_Map {
    fn eq(&self, other: &Self) -> bool {
        PTR_EQ(&self.__go_entries, &other.__go_entries)
    }
}

impl Eq for sync_Map {}

impl PartialOrd for sync_Map {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for sync_Map {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (AS_PTR(&self.__go_entries) as *const () as usize).cmp(&(AS_PTR(&other.__go_entries) as *const () as usize))
    }
}

impl sync_Map {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn load<K: Any + Clone + std::fmt::DebugPLUS_BOUND>(&self, key: K) -> (ANY_HANDLE, bool) {
        match self.__go_entries LOCK_TABLE.get(&go_sync_map_key(&key)) {
            Some(entry) => (entry.value.handle(), true),
            None => (NONE_HANDLE, false),
        }
    }

    pub fn store<K: Any + Clone + std::fmt::DebugPLUS_BOUND, V: Any + ClonePLUS_BOUND>(&self, key: K, value: V) {
        let map_key = go_sync_map_key(&key);
        self.__go_entries LOCK_TABLE.insert(map_key, GoSyncMapEntry { key: GoSyncMapSlot::new(key), value: GoSyncMapSlot::new(value) });
    }

    pub fn load_or_store<K: Any + Clone + std::fmt::DebugPLUS_BOUND, V: Any + ClonePLUS_BOUND>(&self, key: K, value: V) -> (ANY_HANDLE, bool) {
        let map_key = go_sync_map_key(&key);
        let mut entries = self.__go_entries LOCK_TABLE;
        if let Some(entry) = entries.get(&map_key) {
            return (entry.value.handle(), true);
        }
        let entry = GoSyncMapEntry { key: GoSyncMapSlot::new(key), value: GoSyncMapSlot::new(value) };
        let actual = entry.value.handle();
        entries.insert(map_key, entry);
        (actual, false)
    }

    pub fn load_and_delete<K: Any + Clone + std::fmt::DebugPLUS_BOUND>(&self, key: K) -> (ANY_HANDLE, bool) {
        match self.__go_entries LOCK_TABLE.remove(&go_sync_map_key(&key)) {
            Some(entry) => (entry.value.handle(), true),
            None => (NONE_HANDLE, false),
        }
    }

    pub fn delete<K: Any + Clone + std::fmt::DebugPLUS_BOUND>(&self, key: K) {
        self.__go_entries LOCK_TABLE.remove(&go_sync_map_key(&key));
    }

    pub fn swap<K: Any + Clone + std::fmt::DebugPLUS_BOUND, V: Any + ClonePLUS_BOUND>(&self, key: K, value: V) -> (ANY_HANDLE, bool) {
        let map_key = go_sync_map_key(&key);
        let previous = self.__go_entries LOCK_TABLE.insert(map_key, GoSyncMapEntry { key: GoSyncMapSlot::new(key), value: GoSyncMapSlot::new(value) });
        match previous {
            Some(entry) => (entry.value.handle(), true),
            None => (NONE_HANDLE, false),
        }
    }

    pub fn compare_and_swap<K: Any + Clone + std::fmt::DebugPLUS_BOUND, O: Any + std::fmt::Debug, V: Any + ClonePLUS_BOUND>(&self, key: K, old: O, new: V) -> bool {
        let mut entries = self.__go_entries LOCK_TABLE;
        match entries.get_mut(&go_sync_map_key(&key)) {
            Some(entry) if entry.value.holds(&old) => {
                entry.value = GoSyncMapSlot::new(new);
                true
            }
            _ => false,
        }
    }

    pub fn compare_and_delete<K: Any + Clone + std::fmt::DebugPLUS_BOUND, O: Any + std::fmt::Debug>(&self, key: K, old: O) -> bool {
        let map_key = go_sync_map_key(&key);
        let mut entries = self.__go_entries LOCK_TABLE;
        let matches = entries.get(&map_key).is_some_and(|entry| entry.value.holds(&old));
        if matches {
            entries.remove(&map_key);
        }
        matches
    }

    /// Calls f on a snapshot of the entries, so f may change the map.
    pub fn range(&self, f: RANGE_FUNC) {
        let entries: Vec<(ANY_HANDLE, ANY_HANDLE)> = self.__go_entries LOCK_TABLE.values().map(|entry| (entry.key.handle(), entry.value.handle())).collect();
        for (key, value) in entries {
            let call = f LOCK_CELL.as_ref().expect("call of nil function").clone();
            if !call(key, value) {
                break;
            }
        }
    }

    pub fn clear(&self) {
        self.__go_entries LOCK_TABLE.clear();
    }
}`
	rangeFunc := wrappedExternalStubType(funcValueRustTypeQualified(anyHandle+", "+anyHandle, "bool"))
	replacer := strings.NewReplacer(
		"RANGE_FUNC", rangeFunc,
		"ANY_HANDLE", anyHandle,
		"ANY_BOX", anyBox,
		"ANY_DYN", anyDyn,
		"PLUS_BOUND", bound,
		"NEW_SHARED", newShared,
		"SHARED", shared,
		" LOCK_TABLE", lockTable,
		" LOCK_CELL_MUT", lockCellMut,
		" LOCK_CELL", lockCell,
		"PTR_EQ", ptrEq,
		"AS_PTR", asPtr,
		"NONE_HANDLE", wrappedExternalStubNoneExpr(anyBox),
	)
	code = replacer.Replace(code)
	code = strings.ReplaceAll(code, "SOME_HANDLE(clone(&**value))", wrappedExternalStubSomeExpr(anyBox, "clone(&**value)"))
	TrackImport("Any")
	out.WriteString(code)
}

// writeExternalStubMethodCallArgument writes an argument of a call to a stub
// type's method: as a sync.Map key or value for sync.Map, and otherwise as
// writeExternalStubCallArgument does.
func writeExternalStubMethodCallArgument(out *strings.Builder, sel *ast.SelectorExpr, arg ast.Expr, expected types.Type) {
	if isSyncMapMethodSelector(sel) && writeSyncMapMethodCallArgument(out, arg) {
		return
	}
	if isAtomicValueMethodSelector(sel) && writeAtomicValueMethodCallArgument(out, arg) {
		return
	}
	// An interface value is stored as a copy, as Go stores it: assigning to
	// the variable afterwards leaves the stored value alone.
	if isAtomicValueMethodSelector(sel) && !isNilIdentExpr(arg) {
		if typeInfo := GetTypeInfo(); typeInfo != nil && isEmptyInterfaceType(typeInfo.GetType(arg)) {
			NeedAnyHandleCopy()
			out.WriteString("go_any_handle_copy(&")
			writeExternalStubCallArgument(out, arg, expected)
			out.WriteString(")")
			return
		}
	}
	writeExternalStubCallArgument(out, arg, expected)
}

// isAtomicValueMethodSelector reports whether sel is a method of
// sync/atomic.Value, whose stub takes the interface values it stores.
func isAtomicValueMethodSelector(sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || sel == nil {
		return false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok {
		return false
	}
	recv := types.Unalias(selection.Recv())
	if ptr, ok := recv.(*types.Pointer); ok {
		recv = types.Unalias(ptr.Elem())
	}
	named, ok := recv.(*types.Named)
	return ok && named.Obj() != nil && named.Obj().Pkg() != nil && named.Obj().Pkg().Path() == "sync/atomic" && named.Obj().Name() == "Value"
}

// writeAtomicValueMethodCallArgument writes a value passed to an
// atomic.Value method as the interface value it becomes: a value of a
// concrete type, such as a package-level channel, is boxed in a new
// interface value, as for any interface{} parameter, and an interface value
// is passed as its cell, and nil as an empty cell. It reports false for an
// interface value, which the caller copies.
func writeAtomicValueMethodCallArgument(out *strings.Builder, arg ast.Expr) bool {
	if ident, ok := unwrapParens(arg).(*ast.Ident); ok && ident.Name == "nil" {
		// nil is the nil interface value, which Store and CompareAndSwap
		// check for.
		writeTypedWrappedNone(out, rustAnyTraitObject())
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || isEmptyInterfaceType(typeInfo.GetType(arg)) {
		return false
	}
	WriteWrapperPrefix(out)
	if isChannelOperand(arg) {
		// The channel itself, read out of a package variable's cell.
		RegisterAnyCloneType(typeInfo.GetType(arg))
		out.WriteString("Box::new(")
		writeChannelExpression(out, arg)
		out.WriteString(") as ")
		out.WriteString(rustAnyTraitObject())
	} else {
		writeInterfaceBoxedValue(out, arg)
	}
	WriteWrapperSuffix(out)
	return true
}

func isSyncMapMethodSelector(sel *ast.SelectorExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || sel == nil {
		return false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok {
		return false
	}
	recv := types.Unalias(selection.Recv())
	if ptr, ok := recv.(*types.Pointer); ok {
		recv = types.Unalias(ptr.Elem())
	}
	named, ok := recv.(*types.Named)
	return ok && named.Obj() != nil && named.Obj().Pkg() != nil && named.Obj().Pkg().Path() == "sync" && named.Obj().Name() == "Map"
}

// writeSyncMapMethodCallArgument writes a key or value passed to a sync.Map
// method as a value of its own type, which the stub boxes: Go's copy for a
// string, number, bool, array or struct, and the shared handle for a pointer,
// map, slice, channel, function or interface value. An untyped constant gets
// the type Go gives it in an interface, and nil is the nil interface. It
// reports false for an argument the stub's usual path writes.
func writeSyncMapMethodCallArgument(out *strings.Builder, arg ast.Expr) bool {
	if ident, ok := unwrapParens(arg).(*ast.Ident); ok && ident.Name == "nil" {
		writeEmptyInterfaceCallArgumentValue(out, arg)
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	argType := typeInfo.GetType(arg)
	if argType == nil {
		return false
	}
	if basic, ok := types.Unalias(argType).(*types.Basic); ok && typeInfo.info != nil && typeInfo.info.Types[arg].Value != nil {
		// go/types records an untyped constant passed as an interface
		// value with the type Go gives it there.
		goType := types.Default(argType)
		var value strings.Builder
		if !writeConstExpressionForExpectedGoType(&value, arg, goType) {
			return false
		}
		if basic.Info()&types.IsNumeric != 0 {
			// The stub's parameter is generic, so a bare 3 would be an i32.
			out.WriteString("(")
			out.WriteString(value.String())
			out.WriteString(" as ")
			out.WriteString(goTypesTypeToRust(goType))
			out.WriteString(")")
			return true
		}
		out.WriteString(value.String())
		return true
	}
	switch types.Unalias(argType).Underlying().(type) {
	case *types.Basic, *types.Struct, *types.Array:
	default:
		return false
	}
	var value strings.Builder
	if !writeCallArgumentValue(&value, arg) {
		return false
	}
	out.WriteString(value.String())
	return true
}
