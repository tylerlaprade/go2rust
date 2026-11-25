package main

import (
	"bytes"
	"fmt"
	"go/ast"
	"go/parser"
	"go/printer"
	"go/token"
	"go/types"
	"os"
	"sort"
	"strconv"
	"strings"

	"golang.org/x/tools/go/ast/astutil"
)

// A pointer value is the handle of the cell it points at,
// Rc<RefCell<Option<T>>>, and a pointer variable or field holds that handle
// with no cell of its own. So &p of a pointer p cannot point at p: it made a
// new cell holding a copy of p's handle, and *pp = q copied q's node into the
// node that *pp pointed at. That copy built a cycle when the new node pointed
// back at the old one:
//
//	func push(pp **Node, v int) { *pp = &Node{v, *pp} }
//
// rewritePointerSlots rewrites the main package in Go before it is
// translated. Each pointer-to-pointer type **T becomes a slot, a struct of
// two functions that read and rebind the pointer it points at:
//
//	type __ptrSlot_Node struct {
//		get func() *Node
//		set func(*Node)
//	}
//
//	func push(pp __ptrSlot_Node, v int) { pp.set(&Node{v, pp.get()}) }
//
//	push(__ptrSlot_Node{get: func() *Node { return head }, set: func(__slot_v *Node) { head = __slot_v }}, 1)
//
// &x, &s.f and &a[i] become slots whose functions read and assign x, s.f
// and a[i]; *pp becomes pp.get(), *pp = q becomes pp.set(q), pp == nil
// becomes pp.get == nil, and a nil **T becomes the zero slot. A variable a
// slot's set function assigns is a variable a closure rebinds, which
// boxCapturedReboundVariables gives a cell.
//
// The rewrite applies only when every use of a **T value is one of those
// forms. A &x that is converted to an interface, as json.Unmarshal(b, &p)
// does, stays as it is. Any other use, such as comparing two **T values,
// keeps the package as it was, as does a rewrite that does not type-check.

const (
	pointerSlotTypePrefix = "__ptrSlot_"
	pointerSlotGet        = "get"
	pointerSlotSet        = "set"
)

// rewritePointerSlots returns the rewritten files of the package path, or
// false when the package has no **T or cannot be rewritten. info is the
// package's type information. The files are not changed; the result is
// parsed anew, into fset.
func rewritePointerSlots(path string, files []*ast.File, fset *token.FileSet, info *types.Info, imp types.Importer) ([]*ast.File, bool) {
	if info == nil || fset == nil || len(files) == 0 || !typesInfoHasPointerSlotType(info) {
		return nil, false
	}
	for _, file := range files {
		if fileHasDirectiveComments(file) {
			pointerSlotDebugf("%s: a file has //go: directives", path)
			return nil, false
		}
	}
	// Work on a copy, so the files stay as they are when the rewrite stops.
	copies, ok := reparseGoFiles(files, fset, true)
	if !ok {
		return nil, false
	}
	ti, err := NewTypeInfoWithImporter(path, copies, fset, imp)
	if err != nil || ti == nil || ti.info == nil || ti.pkg == nil {
		pointerSlotDebugf("%s: the copy does not type-check: %v", path, err)
		return nil, false
	}
	r := &pointerSlotRewriter{
		info:    ti.info,
		pkg:     ti.pkg,
		slots:   map[string]*pointerSlot{},
		byType:  map[string]*pointerSlot{},
		actions: map[ast.Node]pointerSlotAction{},

		slotVars:      map[*types.Var]bool{},
		keptAddrVars:  map[*types.Var]bool{},
		unboxableVars: map[*types.Var]bool{},
	}
	for _, file := range copies {
		r.analyze(file)
		if r.err != nil {
			pointerSlotDebugf("%s: %v", path, r.err)
			return nil, false
		}
	}
	// A slot's set function assigns a local variable from a closure, which
	// reaches the variable only once boxCapturedReboundVariables gives it
	// a cell.
	for v := range r.slotVars {
		if r.keptAddrVars[v] || r.unboxableVars[v] || !boxableVarType(v.Type()) {
			pointerSlotDebugf("%s: the variable %s cannot get a cell", path, v.Name())
			return nil, false
		}
	}
	if !r.rewritesAnything() {
		return nil, false
	}
	for _, file := range copies {
		r.apply(file)
		if r.err != nil {
			pointerSlotDebugf("%s: %v", path, r.err)
			return nil, false
		}
	}
	copies[0].Decls = append(copies[0].Decls, r.slotDecls()...)
	// The rewritten nodes have no positions, so the files are printed and
	// parsed again. Comments are left out: the printer places a comment
	// by position, and could put one between tokens of a new node.
	out, ok := reparseGoFiles(copies, fset, false)
	if !ok {
		return nil, false
	}
	return out, true
}

func (r *pointerSlotRewriter) rewritesAnything() bool {
	for _, action := range r.actions {
		if action.kind != slotActionSkip {
			return true
		}
	}
	return false
}

func pointerSlotDebugf(format string, args ...any) {
	if os.Getenv("GO2RUST_DEBUG_PTRSLOT") != "" {
		fmt.Fprintf(os.Stderr, "Note: **T kept as it is: "+format+"\n", args...)
	}
}

func fileHasDirectiveComments(file *ast.File) bool {
	for _, group := range file.Comments {
		for _, comment := range group.List {
			if strings.HasPrefix(comment.Text, "//go:") || strings.HasPrefix(comment.Text, "//line ") {
				return true
			}
		}
	}
	for _, spec := range file.Imports {
		if spec.Path != nil && spec.Path.Value == `"C"` {
			return true
		}
	}
	return false
}

// reparseGoFiles prints each file and parses it again into fset, under the
// file's own name.
func reparseGoFiles(files []*ast.File, fset *token.FileSet, comments bool) ([]*ast.File, bool) {
	out := make([]*ast.File, 0, len(files))
	for _, file := range files {
		name := fset.Position(file.Package).Filename
		if !comments {
			file.Comments = nil
		}
		var buf bytes.Buffer
		cfg := printer.Config{Mode: printer.UseSpaces | printer.TabIndent, Tabwidth: 8}
		if err := cfg.Fprint(&buf, fset, file); err != nil {
			pointerSlotDebugf("printing %s: %v", name, err)
			return nil, false
		}
		if !comments && os.Getenv("GO2RUST_DEBUG_PTRSLOT") == "dump" {
			fmt.Fprintf(os.Stderr, "// %s after the **T rewrite\n%s\n", name, buf.String())
		}
		mode := parser.SkipObjectResolution
		if comments {
			mode |= parser.ParseComments
		}
		parsed, err := parser.ParseFile(fset, name, buf.Bytes(), mode)
		if err != nil {
			pointerSlotDebugf("parsing %s again: %v", name, err)
			return nil, false
		}
		out = append(out, parsed)
	}
	return out, true
}

// pointerSlotHeld returns H of a **T type, the pointer type *T its slot
// holds: t is a pointer, not a defined type, to a pointer. A map or a
// channel is a handle with no cell of its own too, so a pointer to one is
// a slot as well.
func pointerSlotHeld(t types.Type) (types.Type, bool) {
	if t == nil {
		return nil, false
	}
	outer, ok := types.Unalias(t).(*types.Pointer)
	if !ok {
		return nil, false
	}
	switch outer.Elem().Underlying().(type) {
	case *types.Pointer, *types.Map, *types.Chan:
		return outer.Elem(), true
	}
	return nil, false
}

func typesInfoHasPointerSlotType(info *types.Info) bool {
	for _, tv := range info.Types {
		if _, ok := pointerSlotHeld(tv.Type); ok {
			return true
		}
	}
	return false
}

type pointerSlot struct {
	name string
	held types.Type
}

type pointerSlotActionKind int

const (
	// A type expression **T: the slot type.
	slotActionType pointerSlotActionKind = iota + 1
	// &x of a variable.
	slotActionAddrVar
	// &e.f of a field.
	slotActionAddrField
	// &a[i] of a slice or array element.
	slotActionAddrIndex
	// *pp read.
	slotActionDeref
	// *pp = v.
	slotActionAssign
	// pp == nil or pp != nil.
	slotActionNilCompare
	// nil where a **T is expected.
	slotActionNil
	// new(*T).
	slotActionNew
	// &*pp, which is pp.
	slotActionAddrDeref
	// A node a parent action rewrites.
	slotActionSkip
)

type pointerSlotAction struct {
	kind pointerSlotActionKind
	slot *pointerSlot
	// slotActionAddrField: the owner is a pointer, not an addressable
	// struct value.
	ownerIsPointer bool
	ownerType      types.Type
	// slotActionAddrIndex: the indexed value is a slice, an array, or a
	// pointer to an array, and the index's type.
	indexedType types.Type
	indexType   types.Type
	// slotActionNilCompare: the slot side of the comparison.
	slotSide int
}

type pointerSlotRewriter struct {
	info    *types.Info
	pkg     *types.Package
	slots   map[string]*pointerSlot
	byType  map[string]*pointerSlot
	actions map[ast.Node]pointerSlotAction
	err     error
	stack   []ast.Node
	funcs   []*types.Signature
	nextVar int
	// slotVars are the local variables a slot assigns, and keptAddrVars
	// those whose address is taken and kept as a **T, converted to an
	// interface. unboxableVars are locals boxCapturedReboundVariables
	// does not give a cell: named results and range variables.
	slotVars      map[*types.Var]bool
	keptAddrVars  map[*types.Var]bool
	unboxableVars map[*types.Var]bool
}

func (r *pointerSlotRewriter) fail(format string, args ...any) {
	if r.err == nil {
		r.err = fmt.Errorf(format, args...)
	}
}

func (r *pointerSlotRewriter) slotFor(held types.Type) *pointerSlot {
	key := types.TypeString(held, nil)
	if slot := r.byType[key]; slot != nil {
		return slot
	}
	if _, ok := r.typeExpr(held); !ok {
		r.fail("the slot of %s cannot name its type", key)
		return nil
	}
	base := pointerSlotTypePrefix + sanitizeSlotName(types.TypeString(held, func(*types.Package) string { return "" }))
	name := base
	for i := 2; r.slots[name] != nil || r.pkg.Scope().Lookup(name) != nil; i++ {
		name = base + "_" + strconv.Itoa(i)
	}
	slot := &pointerSlot{name: name, held: held}
	r.slots[name] = slot
	r.byType[key] = slot
	return slot
}

func sanitizeSlotName(s string) string {
	s = strings.TrimLeft(s, "*")
	var b strings.Builder
	for _, c := range s {
		if c == '_' || c >= '0' && c <= '9' || c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' {
			b.WriteRune(c)
		} else {
			b.WriteRune('_')
		}
	}
	return b.String()
}

// typeExpr writes t in Go, for the types a slot's functions name: this
// package's defined types, predeclared types, and pointers, slices and
// arrays of them.
func (r *pointerSlotRewriter) typeExpr(t types.Type) (ast.Expr, bool) {
	switch t := types.Unalias(t).(type) {
	case *types.Basic:
		if t.Info()&types.IsUntyped != 0 {
			return nil, false
		}
		return ast.NewIdent(t.Name()), true
	case *types.Named:
		obj := t.Obj()
		if obj == nil || t.TypeArgs().Len() > 0 || t.TypeParams().Len() > 0 {
			return nil, false
		}
		if obj.Pkg() == nil {
			return ast.NewIdent(obj.Name()), true
		}
		if obj.Pkg() != r.pkg || obj.Parent() != r.pkg.Scope() {
			return nil, false
		}
		return ast.NewIdent(obj.Name()), true
	case *types.Pointer:
		if slot, ok := pointerSlotHeld(t); ok {
			if s := r.slotFor(slot); s != nil {
				return ast.NewIdent(s.name), true
			}
			return nil, false
		}
		elem, ok := r.typeExpr(t.Elem())
		if !ok {
			return nil, false
		}
		return &ast.StarExpr{X: elem}, true
	case *types.Slice:
		elem, ok := r.typeExpr(t.Elem())
		if !ok {
			return nil, false
		}
		return &ast.ArrayType{Elt: elem}, true
	case *types.Array:
		elem, ok := r.typeExpr(t.Elem())
		if !ok {
			return nil, false
		}
		return &ast.ArrayType{Len: &ast.BasicLit{Kind: token.INT, Value: strconv.FormatInt(t.Len(), 10)}, Elt: elem}, true
	case *types.Map:
		key, ok := r.typeExpr(t.Key())
		if !ok {
			return nil, false
		}
		value, ok := r.typeExpr(t.Elem())
		if !ok {
			return nil, false
		}
		return &ast.MapType{Key: key, Value: value}, true
	case *types.Chan:
		elem, ok := r.typeExpr(t.Elem())
		if !ok {
			return nil, false
		}
		dir := ast.SEND | ast.RECV
		switch t.Dir() {
		case types.SendOnly:
			dir = ast.SEND
		case types.RecvOnly:
			dir = ast.RECV
		}
		return &ast.ChanType{Dir: dir, Value: elem}, true
	default:
		return nil, false
	}
}

func (r *pointerSlotRewriter) typeOf(e ast.Expr) types.Type {
	if tv, ok := r.info.Types[e]; ok {
		return tv.Type
	}
	if id, ok := e.(*ast.Ident); ok {
		if obj := r.info.ObjectOf(id); obj != nil {
			return obj.Type()
		}
	}
	return nil
}

func (r *pointerSlotRewriter) isNilIdent(e ast.Expr) bool {
	id, ok := ast.Unparen(e).(*ast.Ident)
	return ok && r.info.Uses[id] == types.Universe.Lookup("nil")
}

// analyze records the action for each node of file that the rewrite
// changes.
func (r *pointerSlotRewriter) analyze(file *ast.File) {
	ast.Inspect(file, func(n ast.Node) bool {
		if r.err != nil {
			return false
		}
		if n == nil {
			top := r.stack[len(r.stack)-1]
			r.stack = r.stack[:len(r.stack)-1]
			switch top.(type) {
			case *ast.FuncDecl, *ast.FuncLit:
				r.funcs = r.funcs[:len(r.funcs)-1]
			}
			return false
		}
		r.stack = append(r.stack, n)
		switch fn := n.(type) {
		case *ast.FuncDecl:
			var sig *types.Signature
			if obj, ok := r.info.Defs[fn.Name].(*types.Func); ok {
				sig, _ = obj.Type().(*types.Signature)
			}
			if fn.Type.TypeParams != nil && len(fn.Type.TypeParams.List) > 0 && r.subtreeHasSlotType(fn) {
				r.fail("generic function %s uses **T", fn.Name.Name)
			}
			r.funcs = append(r.funcs, sig)
		case *ast.FuncLit:
			sig, _ := r.typeOf(fn).(*types.Signature)
			r.funcs = append(r.funcs, sig)
		case *ast.TypeSpec:
			if fn.TypeParams != nil && len(fn.TypeParams.List) > 0 && r.subtreeHasSlotType(fn) {
				r.fail("generic type %s uses **T", fn.Name.Name)
			}
		case *ast.FuncType:
			if fn.Results != nil {
				for _, field := range fn.Results.List {
					for _, name := range field.Names {
						if v, ok := r.info.Defs[name].(*types.Var); ok {
							r.unboxableVars[v] = true
						}
					}
				}
			}
		case *ast.RangeStmt:
			for _, e := range []ast.Expr{fn.Key, fn.Value} {
				if id, ok := e.(*ast.Ident); ok {
					if v, ok := r.info.ObjectOf(id).(*types.Var); ok {
						r.unboxableVars[v] = true
					}
				}
			}
		}
		if e, ok := n.(ast.Expr); ok {
			r.analyzeExpr(e)
		}
		return true
	})
}

func (r *pointerSlotRewriter) subtreeHasSlotType(n ast.Node) bool {
	found := false
	ast.Inspect(n, func(n ast.Node) bool {
		if e, ok := n.(ast.Expr); ok && !found {
			if _, ok := pointerSlotHeld(r.typeOf(e)); ok {
				found = true
			}
		}
		return !found
	})
	return found
}

func (r *pointerSlotRewriter) analyzeExpr(e ast.Expr) {
	if _, skip := r.actions[e]; skip {
		return
	}
	tv, hasType := r.info.Types[e]
	if hasType && tv.IsType() {
		if held, ok := pointerSlotHeld(tv.Type); ok {
			if _, deeper := pointerSlotHeld(held); deeper {
				r.fail("***T")
				return
			}
			if slot := r.slotFor(held); slot != nil {
				r.actions[e] = pointerSlotAction{kind: slotActionType, slot: slot}
			}
		}
		return
	}
	if r.isNilIdent(e) {
		if target, ok := r.contextTarget(len(r.stack) - 1); ok {
			if held, isSlot := pointerSlotHeld(target); isSlot {
				if slot := r.slotFor(held); slot != nil {
					r.actions[e] = pointerSlotAction{kind: slotActionNil, slot: slot}
				}
			}
		}
		return
	}
	if !hasType {
		return
	}
	held, isSlot := pointerSlotHeld(tv.Type)
	if !isSlot {
		return
	}
	if _, deeper := pointerSlotHeld(held); deeper {
		r.fail("***T")
		return
	}
	slot := r.slotFor(held)
	if slot == nil {
		return
	}
	if _, isParen := e.(*ast.ParenExpr); isParen {
		return
	}
	// Where the value goes decides what becomes of it.
	at, parent := r.valueParent(len(r.stack) - 1)
	if done, ok := r.actions[parent]; ok && done.kind != slotActionNil {
		// &*pp, rewritten as a whole.
		return
	}
	switch p := parent.(type) {
	case *ast.StarExpr:
		if r.isAssignTarget(at - 1) {
			assign := r.stack[at-1].(*ast.AssignStmt)
			r.actions[assign] = pointerSlotAction{kind: slotActionAssign, slot: slot}
			r.actions[p] = pointerSlotAction{kind: slotActionSkip}
		} else {
			r.actions[p] = pointerSlotAction{kind: slotActionDeref, slot: slot}
		}
	case *ast.BinaryExpr:
		if p.Op != token.EQL && p.Op != token.NEQ {
			r.fail("operator %s on **T", p.Op)
			return
		}
		side := 0
		other := p.Y
		if ast.Unparen(p.X) != e {
			side = 1
			other = p.X
		}
		if !r.isNilIdent(other) {
			r.fail("two **T values compared")
			return
		}
		r.actions[p] = pointerSlotAction{kind: slotActionNilCompare, slot: slot, slotSide: side}
		r.actions[ast.Unparen(other)] = pointerSlotAction{kind: slotActionSkip}
	default:
		target, known := r.contextTarget(len(r.stack) - 1)
		if !known {
			r.fail("a **T used as %T", parent)
			return
		}
		if types.IsInterface(target) {
			// &p converted to an interface keeps its Go meaning.
			if u, ok := e.(*ast.UnaryExpr); ok && u.Op == token.AND {
				r.actions[e] = pointerSlotAction{kind: slotActionSkip}
				r.skipSubtree(u.X)
				if id, ok := ast.Unparen(u.X).(*ast.Ident); ok {
					if v, ok := r.info.Uses[id].(*types.Var); ok {
						r.keptAddrVars[v] = true
					}
				}
				return
			}
			r.fail("a **T converted to %s", target)
			return
		}
		if _, ok := pointerSlotHeld(target); !ok {
			r.fail("a **T converted to %s", target)
			return
		}
	}
	r.analyzeSlotSource(e, slot)
}

// skipSubtree keeps the rewrite out of n.
func (r *pointerSlotRewriter) skipSubtree(n ast.Node) {
	ast.Inspect(n, func(c ast.Node) bool {
		if c != nil {
			r.actions[c] = pointerSlotAction{kind: slotActionSkip}
		}
		return true
	})
}

// analyzeSlotSource records how e, a **T value, is made, when it is made
// by & or new.
func (r *pointerSlotRewriter) analyzeSlotSource(e ast.Expr, slot *pointerSlot) {
	switch v := e.(type) {
	case *ast.UnaryExpr:
		if v.Op != token.AND {
			return
		}
		switch x := ast.Unparen(v.X).(type) {
		case *ast.Ident:
			if _, ok := r.info.Uses[x].(*types.Var); !ok {
				r.fail("& of %s", x.Name)
				return
			}
			if r.isPointerReceiver(x) {
				r.fail("& of a pointer receiver")
				return
			}
			if v := r.info.Uses[x].(*types.Var); v.Parent() != r.pkg.Scope() {
				r.slotVars[v] = true
			}
			r.actions[e] = pointerSlotAction{kind: slotActionAddrVar, slot: slot}
		case *ast.SelectorExpr:
			if sel := r.info.Selections[x]; sel == nil {
				// A package's variable, pkg.V.
				if _, ok := r.info.Uses[x.Sel].(*types.Var); !ok {
					r.fail("& of %s", x.Sel.Name)
					return
				}
				r.actions[e] = pointerSlotAction{kind: slotActionAddrVar, slot: slot}
				return
			} else if sel.Kind() != types.FieldVal {
				r.fail("& of a method")
				return
			}
			if r.isPointerReceiver(x.X) {
				// A method's pointer receiver is the struct, not its handle,
				// and passing it on passes a copy, whose pointer fields are
				// its own: the slot would rebind the copy's field.
				r.fail("a field of a pointer receiver")
				return
			}
			owner := r.typeOf(x.X)
			_, isPtr := owner.Underlying().(*types.Pointer)
			ownerType := owner
			if !isPtr {
				ownerType = types.NewPointer(owner)
			}
			if _, ok := r.typeExpr(ownerType); !ok {
				r.fail("the owner %s of a field cannot be named", owner)
				return
			}
			r.actions[e] = pointerSlotAction{kind: slotActionAddrField, slot: slot, ownerIsPointer: isPtr, ownerType: ownerType}
		case *ast.IndexExpr:
			indexed := r.typeOf(x.X)
			paramType := indexed
			switch u := indexed.Underlying().(type) {
			case *types.Slice:
			case *types.Array:
				paramType = types.NewPointer(indexed)
			case *types.Pointer:
				if _, ok := u.Elem().Underlying().(*types.Array); !ok {
					r.fail("& of an element of %s", indexed)
					return
				}
			default:
				r.fail("& of an element of %s", indexed)
				return
			}
			if _, ok := r.typeExpr(paramType); !ok {
				r.fail("%s cannot be named", indexed)
				return
			}
			indexType := r.typeOf(x.Index)
			if b, ok := indexType.(*types.Basic); !ok || b.Info()&types.IsUntyped != 0 {
				indexType = types.Typ[types.Int]
			}
			r.actions[e] = pointerSlotAction{kind: slotActionAddrIndex, slot: slot, indexedType: paramType, indexType: indexType}
		case *ast.StarExpr:
			// &*pp is pp.
			r.actions[e] = pointerSlotAction{kind: slotActionAddrDeref}
			r.actions[x] = pointerSlotAction{kind: slotActionSkip}
		default:
			r.fail("& of %T", x)
		}
	case *ast.CallExpr:
		if id, ok := ast.Unparen(v.Fun).(*ast.Ident); ok {
			if _, isBuiltin := r.info.Uses[id].(*types.Builtin); isBuiltin && id.Name == "new" {
				r.actions[e] = pointerSlotAction{kind: slotActionNew, slot: slot}
			}
		}
	}
}

// isPointerReceiver reports whether e is the pointer receiver of the
// method it is in.
func (r *pointerSlotRewriter) isPointerReceiver(e ast.Expr) bool {
	id, ok := ast.Unparen(e).(*ast.Ident)
	if !ok {
		return false
	}
	obj := r.info.Uses[id]
	if obj == nil {
		return false
	}
	for i := len(r.stack) - 1; i >= 0; i-- {
		fn, ok := r.stack[i].(*ast.FuncDecl)
		if !ok {
			continue
		}
		if fn.Recv == nil || len(fn.Recv.List) == 0 || len(fn.Recv.List[0].Names) == 0 {
			return false
		}
		if _, isPtr := fn.Recv.List[0].Type.(*ast.StarExpr); !isPtr {
			return false
		}
		return r.info.Defs[fn.Recv.List[0].Names[0]] == obj
	}
	return false
}

// valueParent returns the first ancestor of stack[i] that is not a
// parenthesis, and its index.
func (r *pointerSlotRewriter) valueParent(i int) (int, ast.Node) {
	for j := i - 1; j >= 0; j-- {
		if _, ok := r.stack[j].(*ast.ParenExpr); !ok {
			return j, r.stack[j]
		}
	}
	return -1, nil
}

// isAssignTarget reports whether the node at stack[i+1] is the one target
// of the plain assignment at stack[i].
func (r *pointerSlotRewriter) isAssignTarget(i int) bool {
	if i < 0 {
		return false
	}
	assign, ok := r.stack[i].(*ast.AssignStmt)
	if !ok || assign.Tok != token.ASSIGN || len(assign.Lhs) != 1 || len(assign.Rhs) != 1 {
		if ok && assign.Tok == token.ASSIGN {
			for _, lhs := range assign.Lhs {
				if lhs == r.stack[i+1] {
					r.fail("*pp in a multiple assignment")
				}
			}
		}
		return false
	}
	return assign.Lhs[0] == r.stack[i+1]
}

// contextTarget returns the type the value at stack[i] is converted to
// where it is used, and false when the use is not one it knows.
func (r *pointerSlotRewriter) contextTarget(i int) (types.Type, bool) {
	at, parent := r.valueParent(i)
	if at < 0 {
		return nil, false
	}
	// The child of parent on the path from stack[i].
	direct := r.stack[at+1]
	switch p := parent.(type) {
	case *ast.CallExpr:
		if direct == p.Fun {
			return nil, false
		}
		sig, ok := r.typeOf(p.Fun).Underlying().(*types.Signature)
		if !ok {
			// A conversion T(x).
			if tv, ok := r.info.Types[p.Fun]; ok && tv.IsType() {
				return tv.Type, true
			}
			return nil, false
		}
		for argIndex, arg := range p.Args {
			if arg != direct {
				continue
			}
			params := sig.Params()
			if sig.Variadic() && argIndex >= params.Len()-1 {
				last := params.At(params.Len() - 1).Type()
				if p.Ellipsis.IsValid() {
					return last, true
				}
				if s, ok := last.Underlying().(*types.Slice); ok {
					return s.Elem(), true
				}
				return last, true
			}
			if argIndex < params.Len() {
				return params.At(argIndex).Type(), true
			}
		}
		return nil, false
	case *ast.AssignStmt:
		for k, rhs := range p.Rhs {
			if rhs != direct {
				continue
			}
			if p.Tok == token.DEFINE {
				return r.typeOf(rhs), true
			}
			if len(p.Lhs) == len(p.Rhs) {
				return r.typeOf(p.Lhs[k]), true
			}
		}
		for _, lhs := range p.Lhs {
			if lhs == direct {
				return r.typeOf(lhs), true
			}
		}
		return nil, false
	case *ast.ValueSpec:
		for _, name := range p.Names {
			if name == direct {
				return r.typeOf(name), true
			}
		}
		for k, value := range p.Values {
			if value == direct && k < len(p.Names) {
				if obj := r.info.Defs[p.Names[k]]; obj != nil {
					return obj.Type(), true
				}
			}
		}
		return nil, false
	case *ast.ReturnStmt:
		if len(r.funcs) == 0 || r.funcs[len(r.funcs)-1] == nil {
			return nil, false
		}
		results := r.funcs[len(r.funcs)-1].Results()
		for k, result := range p.Results {
			if result == direct && k < results.Len() && len(p.Results) == results.Len() {
				return results.At(k).Type(), true
			}
		}
		return nil, false
	case *ast.KeyValueExpr:
		if at < 1 {
			return nil, false
		}
		lit, ok := r.stack[at-1].(*ast.CompositeLit)
		if !ok {
			return nil, false
		}
		return r.compositeElemTarget(lit, p, direct == p.Key)
	case *ast.CompositeLit:
		for k, elt := range p.Elts {
			if elt != direct {
				continue
			}
			switch u := r.typeOf(p).Underlying().(type) {
			case *types.Struct:
				if k < u.NumFields() {
					return u.Field(k).Type(), true
				}
			case *types.Slice:
				return u.Elem(), true
			case *types.Array:
				return u.Elem(), true
			}
		}
		return nil, false
	case *ast.SendStmt:
		if direct == p.Value {
			if ch, ok := r.typeOf(p.Chan).Underlying().(*types.Chan); ok {
				return ch.Elem(), true
			}
		}
		return nil, false
	case *ast.BinaryExpr:
		if direct == p.X {
			return r.typeOf(p.Y), true
		}
		return r.typeOf(p.X), true
	case *ast.RangeStmt:
		if direct == p.Key || direct == p.Value {
			return r.typeOf(direct.(ast.Expr)), true
		}
		return nil, false
	default:
		return nil, false
	}
}

func (r *pointerSlotRewriter) compositeElemTarget(lit *ast.CompositeLit, kv *ast.KeyValueExpr, isKey bool) (types.Type, bool) {
	switch u := r.typeOf(lit).Underlying().(type) {
	case *types.Struct:
		if isKey {
			return nil, false
		}
		name, ok := kv.Key.(*ast.Ident)
		if !ok {
			return nil, false
		}
		for k := 0; k < u.NumFields(); k++ {
			if u.Field(k).Name() == name.Name {
				return u.Field(k).Type(), true
			}
		}
	case *types.Map:
		if isKey {
			return u.Key(), true
		}
		return u.Elem(), true
	case *types.Slice:
		if !isKey {
			return u.Elem(), true
		}
	case *types.Array:
		if !isKey {
			return u.Elem(), true
		}
	}
	return nil, false
}

// apply rewrites file by the recorded actions, children before parents.
func (r *pointerSlotRewriter) apply(file *ast.File) {
	astutil.Apply(file, nil, func(c *astutil.Cursor) bool {
		if r.err != nil {
			return false
		}
		n := c.Node()
		action, ok := r.actions[n]
		if !ok || action.kind == slotActionSkip {
			return true
		}
		if replacement := r.rewriteNode(n, action); replacement != nil {
			c.Replace(replacement)
		}
		return true
	})
}

func (r *pointerSlotRewriter) tempName(base string) string {
	r.nextVar++
	return "__slot_" + base + strconv.Itoa(r.nextVar)
}

func (r *pointerSlotRewriter) rewriteNode(n ast.Node, action pointerSlotAction) ast.Node {
	slot := action.slot
	switch action.kind {
	case slotActionType:
		return ast.NewIdent(slot.name)
	case slotActionNil:
		return &ast.CompositeLit{Type: ast.NewIdent(slot.name)}
	case slotActionDeref:
		star := n.(*ast.StarExpr)
		return &ast.CallExpr{Fun: &ast.SelectorExpr{X: star.X, Sel: ast.NewIdent(pointerSlotGet)}}
	case slotActionAssign:
		assign := n.(*ast.AssignStmt)
		star, ok := ast.Unparen(assign.Lhs[0]).(*ast.StarExpr)
		if !ok {
			r.fail("*pp = v without *pp")
			return nil
		}
		return &ast.ExprStmt{X: &ast.CallExpr{
			Fun:  &ast.SelectorExpr{X: star.X, Sel: ast.NewIdent(pointerSlotSet)},
			Args: []ast.Expr{assign.Rhs[0]},
		}}
	case slotActionNilCompare:
		bin := n.(*ast.BinaryExpr)
		side := bin.X
		if action.slotSide == 1 {
			side = bin.Y
		}
		return &ast.BinaryExpr{X: &ast.SelectorExpr{X: side, Sel: ast.NewIdent(pointerSlotGet)}, Op: bin.Op, Y: ast.NewIdent("nil")}
	case slotActionAddrDeref:
		return ast.Unparen(n.(*ast.UnaryExpr).X).(*ast.StarExpr).X
	case slotActionAddrVar:
		x := ast.Unparen(n.(*ast.UnaryExpr).X)
		return r.slotLiteral(slot, func() ast.Expr { return x }, func() ast.Expr { return x })
	case slotActionAddrField:
		sel := ast.Unparen(n.(*ast.UnaryExpr).X).(*ast.SelectorExpr)
		owner := r.tempName("o")
		ownerArg := ast.Unparen(sel.X)
		if !action.ownerIsPointer {
			if star, ok := ast.Unparen(sel.X).(*ast.StarExpr); ok {
				ownerArg = star.X
			} else {
				ownerArg = &ast.UnaryExpr{Op: token.AND, X: sel.X}
			}
		}
		ownerType, _ := r.typeExpr(action.ownerType)
		field := func() ast.Expr { return &ast.SelectorExpr{X: ast.NewIdent(owner), Sel: ast.NewIdent(sel.Sel.Name)} }
		return r.slotConstructor(slot, []*ast.Field{{Names: []*ast.Ident{ast.NewIdent(owner)}, Type: ownerType}}, []ast.Expr{ownerArg}, field)
	case slotActionAddrIndex:
		index := ast.Unparen(n.(*ast.UnaryExpr).X).(*ast.IndexExpr)
		seq := r.tempName("s")
		idx := r.tempName("i")
		seqArg := index.X
		if _, isArray := action.indexedType.(*types.Pointer); isArray {
			if _, already := r.typeOf(index.X).Underlying().(*types.Pointer); !already {
				seqArg = &ast.UnaryExpr{Op: token.AND, X: index.X}
			}
		}
		seqType, _ := r.typeExpr(action.indexedType)
		idxType, _ := r.typeExpr(action.indexType)
		elem := func() ast.Expr { return &ast.IndexExpr{X: ast.NewIdent(seq), Index: ast.NewIdent(idx)} }
		return r.slotConstructor(slot, []*ast.Field{
			{Names: []*ast.Ident{ast.NewIdent(seq)}, Type: seqType},
			{Names: []*ast.Ident{ast.NewIdent(idx)}, Type: idxType},
		}, []ast.Expr{seqArg, index.Index}, elem)
	case slotActionNew:
		p := r.tempName("p")
		heldType, _ := r.typeExpr(slot.held)
		lit := r.slotLiteral(slot, func() ast.Expr { return ast.NewIdent(p) }, func() ast.Expr { return ast.NewIdent(p) })
		body := []ast.Stmt{
			&ast.DeclStmt{Decl: &ast.GenDecl{Tok: token.VAR, Specs: []ast.Spec{&ast.ValueSpec{Names: []*ast.Ident{ast.NewIdent(p)}, Type: heldType}}}},
			&ast.ReturnStmt{Results: []ast.Expr{lit}},
		}
		return &ast.CallExpr{Fun: &ast.FuncLit{
			Type: &ast.FuncType{Params: &ast.FieldList{}, Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent(slot.name)}}}},
			Body: &ast.BlockStmt{List: body},
		}}
	}
	return nil
}

// slotLiteral is the slot whose functions read target() and assign it.
func (r *pointerSlotRewriter) slotLiteral(slot *pointerSlot, read, write func() ast.Expr) ast.Expr {
	heldType := func() ast.Expr {
		t, _ := r.typeExpr(slot.held)
		return t
	}
	value := r.tempName("v")
	get := &ast.FuncLit{
		Type: &ast.FuncType{Params: &ast.FieldList{}, Results: &ast.FieldList{List: []*ast.Field{{Type: heldType()}}}},
		Body: &ast.BlockStmt{List: []ast.Stmt{&ast.ReturnStmt{Results: []ast.Expr{read()}}}},
	}
	set := &ast.FuncLit{
		Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{{Names: []*ast.Ident{ast.NewIdent(value)}, Type: heldType()}}}},
		Body: &ast.BlockStmt{List: []ast.Stmt{&ast.AssignStmt{Lhs: []ast.Expr{write()}, Tok: token.ASSIGN, Rhs: []ast.Expr{ast.NewIdent(value)}}}},
	}
	return &ast.CompositeLit{Type: ast.NewIdent(slot.name), Elts: []ast.Expr{
		&ast.KeyValueExpr{Key: ast.NewIdent(pointerSlotGet), Value: get},
		&ast.KeyValueExpr{Key: ast.NewIdent(pointerSlotSet), Value: set},
	}}
}

// slotConstructor calls a function of params with args that returns the
// slot of target(): the arguments are evaluated once, where the & was.
func (r *pointerSlotRewriter) slotConstructor(slot *pointerSlot, params []*ast.Field, args []ast.Expr, target func() ast.Expr) ast.Expr {
	lit := r.slotLiteral(slot, target, target)
	return &ast.CallExpr{
		Fun: &ast.FuncLit{
			Type: &ast.FuncType{Params: &ast.FieldList{List: params}, Results: &ast.FieldList{List: []*ast.Field{{Type: ast.NewIdent(slot.name)}}}},
			Body: &ast.BlockStmt{List: []ast.Stmt{&ast.ReturnStmt{Results: []ast.Expr{lit}}}},
		},
		Args: args,
	}
}

// slotDecls declares the slot types, in name order.
func (r *pointerSlotRewriter) slotDecls() []ast.Decl {
	names := make([]string, 0, len(r.slots))
	for name := range r.slots {
		names = append(names, name)
	}
	sort.Strings(names)
	decls := make([]ast.Decl, 0, len(names))
	for _, name := range names {
		slot := r.slots[name]
		getType, _ := r.typeExpr(slot.held)
		setType, _ := r.typeExpr(slot.held)
		fields := []*ast.Field{
			{Names: []*ast.Ident{ast.NewIdent(pointerSlotGet)}, Type: &ast.FuncType{Params: &ast.FieldList{}, Results: &ast.FieldList{List: []*ast.Field{{Type: getType}}}}},
			{Names: []*ast.Ident{ast.NewIdent(pointerSlotSet)}, Type: &ast.FuncType{Params: &ast.FieldList{List: []*ast.Field{{Type: setType}}}}},
		}
		decls = append(decls, &ast.GenDecl{Tok: token.TYPE, Specs: []ast.Spec{&ast.TypeSpec{
			Name: ast.NewIdent(name),
			Type: &ast.StructType{Fields: &ast.FieldList{List: fields}},
		}}})
	}
	return decls
}
