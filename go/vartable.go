package main

// WrapLevel describes how a variable is wrapped in the generated Rust code.
type WrapLevel int

const (
	WrapFull   WrapLevel = iota // Rc<RefCell<Option<T>>> or Arc<Mutex<Option<T>>>
	WrapNone                    // bare T (no wrapping at all)
	WrapOption                  // Option<T> only (future use)
)

// VarSource describes where a variable came from.
type VarSource int

const (
	SourceLocal    VarSource = iota // local variable (let/let mut)
	SourceParam                     // function/method parameter
	SourceRangeKey                  // range loop key variable
	SourceRangeVal                  // range loop value variable
)

// PointerKind describes local pointer representations that are not the default
// Go pointer wrapper.
type PointerKind int

const (
	PointerDefault PointerKind = iota
	PointerSliceElem
)

// VarInfo holds metadata about a variable tracked by VarTable.
type VarInfo struct {
	WrapLevel   WrapLevel
	RustType    string // e.g. "&dyn Shape", "f64"
	Source      VarSource
	IsRef       bool // true for &dyn Trait params
	PointerKind PointerKind
}

// Scope holds variables at one nesting level.
type Scope struct {
	vars map[string]*VarInfo
}

// VarTable is a scope-aware variable tracking system.
type VarTable struct {
	scopes []Scope
}

var currentVarTable *VarTable

func SetVarTable(vt *VarTable) {
	currentVarTable = vt
}

func GetVarTable() *VarTable {
	return currentVarTable
}

func NewVarTable() *VarTable {
	return &VarTable{
		scopes: []Scope{{vars: make(map[string]*VarInfo)}},
	}
}

func (vt *VarTable) PushScope() {
	vt.scopes = append(vt.scopes, Scope{vars: make(map[string]*VarInfo)})
}

func (vt *VarTable) PopScope() {
	if len(vt.scopes) > 1 {
		vt.scopes = vt.scopes[:len(vt.scopes)-1]
	}
}

func (vt *VarTable) Register(name string, info *VarInfo) {
	if len(vt.scopes) == 0 {
		vt.scopes = append(vt.scopes, Scope{vars: make(map[string]*VarInfo)})
	}
	vt.scopes[len(vt.scopes)-1].vars[name] = info
}

func (vt *VarTable) Lookup(name string) *VarInfo {
	for i := len(vt.scopes); i > 0; i-- {
		if info, ok := vt.scopes[i-1].vars[name]; ok {
			return info
		}
	}
	return nil
}

func lookupVarInfo(name string) *VarInfo {
	vt := GetVarTable()
	if vt == nil {
		return nil
	}
	return vt.Lookup(name)
}

func lookupVarInfoInCurrentScope(name string) *VarInfo {
	vt := GetVarTable()
	if vt == nil || len(vt.scopes) == 0 {
		return nil
	}
	return vt.scopes[len(vt.scopes)-1].vars[name]
}

func isVarDeclaredInCurrentScope(name string) bool {
	return lookupVarInfoInCurrentScope(name) != nil
}

// isVarBare checks if a variable is known to be bare (not wrapped) via VarTable.
func isVarBare(name string) bool {
	if info := lookupVarInfo(name); info != nil && info.WrapLevel == WrapNone {
		return true
	}
	return false
}

func sliceElemPtrVarInfo(name string) (*VarInfo, bool) {
	if info := lookupVarInfo(name); info != nil && info.PointerKind == PointerSliceElem {
		return info, true
	}
	return nil, false
}

func isSliceElemPtrVar(name string) bool {
	_, ok := sliceElemPtrVarInfo(name)
	return ok
}
