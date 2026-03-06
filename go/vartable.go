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

// VarInfo holds metadata about a variable tracked by VarTable.
type VarInfo struct {
	WrapLevel WrapLevel
	RustType  string    // e.g. "&dyn Shape", "f64"
	Source    VarSource
	IsRef     bool // true for &dyn Trait params
}

// Scope holds variables at one nesting level.
type Scope struct {
	vars   map[string]*VarInfo
	parent *Scope
}

// VarTable is a scope-aware variable tracking system.
type VarTable struct {
	current *Scope
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
		current: &Scope{
			vars: make(map[string]*VarInfo),
		},
	}
}

func (vt *VarTable) PushScope() {
	vt.current = &Scope{
		vars:   make(map[string]*VarInfo),
		parent: vt.current,
	}
}

func (vt *VarTable) PopScope() {
	if vt.current.parent != nil {
		vt.current = vt.current.parent
	}
}

func (vt *VarTable) Register(name string, info *VarInfo) {
	vt.current.vars[name] = info
}

func (vt *VarTable) Lookup(name string) *VarInfo {
	for scope := vt.current; scope != nil; scope = scope.parent {
		if info, ok := scope.vars[name]; ok {
			return info
		}
	}
	return nil
}

// isVarBare checks if a variable is known to be bare (not wrapped) via VarTable.
func isVarBare(name string) bool {
	if vt := GetVarTable(); vt != nil {
		if info := vt.Lookup(name); info != nil && info.WrapLevel == WrapNone {
			return true
		}
	}
	return false
}
