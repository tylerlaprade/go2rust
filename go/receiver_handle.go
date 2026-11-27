package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

// A pointer method runs on &mut self, the struct inside the cell of the
// handle it was called on, and has no handle of its own. Where the receiver
// is only read and written through its fields that is enough, but a method
// that hands out its receiver (return s, list = append(list, s), go s.loop(),
// defer s.reset(), a method value s.M) or moves it (s = s.next) needs the
// handle itself. Without it the method made one from a copy of the struct,
// which shares its field cells, so a map or pointer field it then replaced
// was replaced on the copy only, and the walk in s = s.next took each node
// out of its cell.
//
// Such a method gets the handle through a hint: the call site records the
// handle, and the address of the struct in its cell, just before the call,
// and the method takes the hint whose address is its own. The method's body
// then works through that handle. A call site that cannot name the handle
// (a slice element, say) records nothing, and the method works on a copy
// that shares the field cells and writes the copy back into self when it
// returns, as it did before.

// currentReceiverHandleObject is the receiver of the method being written
// when that method took its handle and names its receiver as a local.
var currentReceiverHandleObject types.Object

// isCurrentReceiverHandleVar reports whether expr names that receiver.
func isCurrentReceiverHandleVar(expr ast.Expr) bool {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok || currentReceiverHandleObject == nil {
		return false
	}
	typeInfo := GetTypeInfo()
	return typeInfo != nil && typeInfo.info != nil && typeInfo.info.Uses[ident] == currentReceiverHandleObject
}

// receiverHandleMethods holds the pointer methods that take the handle they
// were called on, by their generic origin.
var receiverHandleMethods = map[*types.Func]bool{}

// collectReceiverHandleMethods finds the methods of the package's files
// whose receiver escapes or is reassigned, and those that call such a method
// on their own receiver, which they must hand the handle on to.
func collectReceiverHandleMethods(files []*ast.File, typeInfo *TypeInfo) {
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	info := typeInfo.info
	type method struct {
		obj    *types.Func
		direct bool
		calls  []*types.Func
	}
	var methods []*method
	for _, file := range files {
		if file == nil {
			continue
		}
		for _, decl := range file.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Body == nil || fn.Recv == nil || len(fn.Recv.List) != 1 {
				continue
			}
			recv := fn.Recv.List[0]
			if len(recv.Names) != 1 || recv.Names[0].Name == "_" {
				continue
			}
			if _, ok := recv.Type.(*ast.StarExpr); !ok {
				continue
			}
			obj, ok := info.Defs[fn.Name].(*types.Func)
			if !ok {
				continue
			}
			recvVar, ok := info.Defs[recv.Names[0]].(*types.Var)
			if !ok || !receiverHandleStructPointer(recvVar.Type()) {
				continue
			}
			direct, calls := receiverHandleUses(fn.Body, recvVar, info)
			methods = append(methods, &method{obj: obj, direct: direct, calls: calls})
		}
	}
	needs := map[*types.Func]bool{}
	for _, m := range methods {
		if m.direct {
			needs[m.obj] = true
		}
	}
	for changed := true; changed; {
		changed = false
		for _, m := range methods {
			if needs[m.obj] {
				continue
			}
			for _, called := range m.calls {
				if needs[called] {
					needs[m.obj] = true
					changed = true
					break
				}
			}
		}
	}
	for obj := range needs {
		receiverHandleMethods[obj] = true
	}
}

// receiverHandleStructPointer reports whether typ is a pointer to a named
// struct type, whose pointer is a handle to the struct's cell.
func receiverHandleStructPointer(typ types.Type) bool {
	ptr, ok := types.Unalias(typ).(*types.Pointer)
	if !ok {
		return false
	}
	named, ok := types.Unalias(ptr.Elem()).(*types.Named)
	if !ok {
		return false
	}
	_, ok = named.Underlying().(*types.Struct)
	return ok
}

// receiverHandleUses reports whether the receiver recv escapes or is
// reassigned in body, and which methods are called on it directly.
func receiverHandleUses(body *ast.BlockStmt, recv *types.Var, info *types.Info) (bool, []*types.Func) {
	escapes := false
	var calls []*types.Func
	var stack []ast.Node
	ast.Inspect(body, func(node ast.Node) bool {
		if node == nil {
			stack = stack[:len(stack)-1]
			return true
		}
		if escapes {
			return false
		}
		if ident, ok := node.(*ast.Ident); ok && info.Uses[ident] == recv {
			escaped, called := receiverUseEscapes(ident, stack, info)
			if escaped {
				escapes = true
				return false
			}
			if called != nil {
				calls = append(calls, called)
			}
		}
		stack = append(stack, node)
		return true
	})
	return escapes, calls
}

// receiverUseEscapes reports whether the use ident of the receiver, whose
// enclosing nodes are stack, needs the receiver's handle, and returns the
// method the use calls on the receiver, if any.
func receiverUseEscapes(ident *ast.Ident, stack []ast.Node, info *types.Info) (bool, *types.Func) {
	at := len(stack) - 1
	for at >= 0 {
		if _, ok := stack[at].(*ast.ParenExpr); !ok {
			break
		}
		at--
	}
	if at < 0 {
		return true, nil
	}
	switch parent := stack[at].(type) {
	case *ast.SelectorExpr:
		selection := info.Selections[parent]
		if selection == nil {
			return true, nil
		}
		if selection.Kind() == types.FieldVal {
			return false, nil
		}
		if selection.Kind() != types.MethodVal {
			return true, nil
		}
		// A method promoted from an embedded field runs on that field,
		// whose cell is its handle, called, deferred or bound.
		if len(selection.Index()) != 1 {
			return false, nil
		}
		// s.M without a call is a method value, which keeps the receiver.
		callAt := at - 1
		for callAt >= 0 {
			if _, ok := stack[callAt].(*ast.ParenExpr); !ok {
				break
			}
			callAt--
		}
		if callAt < 0 {
			return true, nil
		}
		call, ok := stack[callAt].(*ast.CallExpr)
		if ok && unwrapParens(call.Fun) != ast.Expr(parent) {
			// once.Do(s.init) calls the method value before it returns,
			// and is written as a call of the method on the receiver.
			if fn, ok := selection.Obj().(*types.Func); ok && callIsSyncOnceDo(call, info) {
				return false, fn.Origin()
			}
			return true, nil
		}
		if !ok {
			return true, nil
		}
		// go s.loop() and defer s.reset() keep the receiver past the
		// statement.
		if callAt > 0 {
			switch stmt := stack[callAt-1].(type) {
			case *ast.GoStmt:
				if stmt.Call == call {
					return true, nil
				}
			case *ast.DeferStmt:
				if stmt.Call == call {
					return true, nil
				}
			}
		}
		fn, ok := selection.Obj().(*types.Func)
		if !ok {
			return false, nil
		}
		return false, fn.Origin()
	case *ast.StarExpr:
		return false, nil
	case *ast.CallExpr:
		// unsafe.Pointer(s) takes the receiver's address, which is the
		// address of self.
		if tv, ok := info.Types[parent.Fun]; ok && tv.IsType() {
			if basic, ok := types.Unalias(tv.Type).(*types.Basic); ok && basic.Kind() == types.UnsafePointer {
				return false, nil
			}
		}
		return true, nil
	case *ast.BinaryExpr:
		if parent.Op == token.EQL || parent.Op == token.NEQ {
			return false, nil
		}
		return true, nil
	}
	return true, nil
}

// callIsSyncOnceDo reports whether call is once.Do(f) for a sync.Once.
func callIsSyncOnceDo(call *ast.CallExpr, info *types.Info) bool {
	sel, ok := unwrapParens(call.Fun).(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "Do" {
		return false
	}
	selection := info.Selections[sel]
	if selection == nil || selection.Kind() != types.MethodVal {
		return false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok || fn.Pkg() == nil || fn.Pkg().Path() != "sync" {
		return false
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Recv() == nil {
		return false
	}
	recv := types.Unalias(sig.Recv().Type())
	if ptr, ok := recv.(*types.Pointer); ok {
		recv = types.Unalias(ptr.Elem())
	}
	named, ok := recv.(*types.Named)
	return ok && named.Obj().Name() == "Once"
}

// methodTakesReceiverHandle reports whether fn is a pointer method that
// takes the handle it was called on.
func methodTakesReceiverHandle(fn *types.Func) bool {
	if fn == nil {
		return false
	}
	return receiverHandleMethods[fn.Origin()]
}

func methodDeclTakesReceiverHandle(fn *ast.FuncDecl) bool {
	return methodTakesReceiverHandle(methodFuncForDecl(fn, GetTypeInfo()))
}

// writeReceiverHandleMethodPrologue binds name, the receiver as a local
// pointer variable: the handle of the call's hint, or a new handle to a copy
// of self that shares its field cells. A method that writes its receiver
// writes that copy back into self when it returns, panics included.
func writeReceiverHandleMethodPrologue(out *strings.Builder, indent string, name string, mutable bool) {
	NeedReceiverHandles()
	out.WriteString(indent)
	out.WriteString("let (mut ")
	out.WriteString(name)
	out.WriteString(", __self_copy) = match go_take_receiver_handle::<Self>(self as *const Self as usize) {\n")
	out.WriteString(indent)
	out.WriteString("    Some(__handle) => (__handle, None),\n")
	out.WriteString(indent)
	out.WriteString("    None => {\n")
	out.WriteString(indent)
	out.WriteString("        let __copy = ")
	WriteWrapperOptionPrefix(out)
	out.WriteString("Some(self.clone())")
	WriteWrapperOptionSuffix(out)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("        (__copy.clone(), Some(__copy))\n")
	out.WriteString(indent)
	out.WriteString("    }\n")
	out.WriteString(indent)
	out.WriteString("};\n")
	if !mutable {
		return
	}
	NeedGoWriteBack()
	out.WriteString(indent)
	out.WriteString("let __self_ptr = self as *mut Self;\n")
	out.WriteString(indent)
	out.WriteString("let __self_write_back = GoWriteBack(move || if let Some(__copy) = &__self_copy { unsafe { *__self_ptr = __copy")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap().clone(); } });\n")
}

// writeReceiverHandleMethodCall writes a call of a method that takes its
// receiver's handle on a receiver whose handle the caller can name: the
// handle is recorded as the call's hint, and the call runs on the struct in
// its cell without holding the cell's borrow, which the method's own use of
// the handle would conflict with.
func writeReceiverHandleMethodCall(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	selection := typeInfo.info.Selections[sel]
	if selection == nil || selection.Kind() != types.MethodVal || len(selection.Index()) != 1 {
		return false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok {
		return false
	}
	// A call on the receiver of a method that took its handle holds no
	// borrow of it either, as a call on self held none.
	takesHandle := methodTakesReceiverHandle(fn)
	if !takesHandle && !isCurrentReceiverHandleVar(sel.X) {
		return false
	}
	var handle strings.Builder
	if !writeReceiverHandleOperand(&handle, sel.X) {
		return false
	}
	needsMut := methodCallNeedsMutableReceiver(sel)
	out.WriteString("{ let __recv = ")
	out.WriteString(handle.String())
	out.WriteString("; ")
	if takesHandle {
		writeReceiverHandleHintedPointer(out, "__recv", needsMut)
	} else {
		NeedReceiverHandles()
		if needsMut {
			out.WriteString("let __recv_ptr = go_receiver_ptr_mut(&__recv); ")
		} else {
			out.WriteString("let __recv_ptr = go_receiver_ptr(&__recv); ")
		}
	}
	out.WriteString("let __result = unsafe { ")
	if needsMut {
		out.WriteString("&mut *__recv_ptr")
	} else {
		out.WriteString("&*__recv_ptr")
	}
	out.WriteString(" }.")
	writeMethodCallSelectorAndArguments(out, sel, call, false)
	out.WriteString("; __result }")
	return true
}

// writeReceiverHandleHintedPointer writes the let of __recv_ptr, a raw
// pointer to the struct in the cell of handle, which records the hint that
// hands handle to the method called on it.
func writeReceiverHandleHintedPointer(out *strings.Builder, handle string, needsMut bool) {
	NeedReceiverHandles()
	if needsMut {
		out.WriteString("let __recv_ptr = go_hint_receiver_mut(&")
	} else {
		out.WriteString("let __recv_ptr = go_hint_receiver(&")
	}
	out.WriteString(handle)
	out.WriteString("); ")
}

// writeReceiverHandleOperand writes the handle of a method call's receiver
// operand: a pointer variable, a pointer field or the pointer a call
// returns, or the cell of a struct variable or struct field whose address
// the call takes. It reports false for an operand held another way.
func writeReceiverHandleOperand(out *strings.Builder, expr ast.Expr) bool {
	expr = unwrapParens(expr)
	typeInfo := GetTypeInfo()
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	_, isPointer := types.Unalias(typ).(*types.Pointer)
	switch e := expr.(type) {
	case *ast.Ident:
		if isCurrentReceiverIdent(e) {
			if !isPointer || !currentReceiverRustAliasIsPointerHandle || currentReceiverRustAliasIsGoPtr {
				return false
			}
			out.WriteString(currentReceiverRustName())
			out.WriteString(".clone()")
			return true
		}
		if isPointer {
			if globalIdent, ok := packageGlobalPointerIdent(e); ok {
				writePackageGlobalPointerHandleClone(out, globalIdent)
				return true
			}
		}
		if isPackageGlobalIdent(e) || isLocalConstantIdent(e) || isVarBare(e.Name) {
			return false
		}
		if isSliceElemPtrVar(e.Name) || isGoPtrIdent(e) || isArrayElemPtrVar(e.Name) {
			return false
		}
		if _, isRangeVar := rangeLoopVars[e.Name]; isRangeVar {
			return false
		}
		name := RustIdentForUse(e)
		if renamed, exists := captureRenameForIdent(e); exists {
			name = RustLocalIdent(renamed)
		}
		out.WriteString(name)
		out.WriteString(".clone()")
		return true
	case *ast.SelectorExpr:
		selection := typeInfo.info.Selections[e]
		if selection == nil || selection.Kind() != types.FieldVal {
			return false
		}
		if root := methodReceiverRootIdent(e); root == nil || isPackageGlobalIdent(root) {
			return false
		}
		var field strings.Builder
		if writeGoPtrSelectorReadHandle(&field, e) {
			return false
		}
		TranspileExpressionContext(out, e, LValue)
		out.WriteString(".clone()")
		return true
	case *ast.CallExpr:
		if !isPointer {
			return false
		}
		if tv, ok := typeInfo.info.Types[e.Fun]; ok && tv.IsType() {
			return false
		}
		if ident, ok := unwrapParens(e.Fun).(*ast.Ident); ok && isBuiltinCallTarget(ident) {
			return false
		}
		// A pointer into a slice element or another cell is a GoPtr.
		if _, ok := goPtrResultInfoForCall(e, 0); ok {
			return false
		}
		TranspileExpression(out, e)
		return true
	}
	return false
}

// writeSyncFieldMethodCall writes a call of a method of a sync.WaitGroup,
// Mutex, RWMutex or Map field read through a lock, p.wg.Wait() or
// p.mu.Lock(), on a copy of the field, which shares its state. A call on the
// field in place held p's lock until the call returned, and every goroutine
// that then needed p (the one whose wg.Done() the Wait was for, say) waited
// for it forever.
func writeSyncFieldMethodCall(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr) bool {
	if !NeedsConcurrentWrapper() {
		return false
	}
	field, ok := unwrapParens(sel.X).(*ast.SelectorExpr)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	selection := typeInfo.info.Selections[field]
	if selection == nil || selection.Kind() != types.FieldVal {
		return false
	}
	if methodSelection := typeInfo.info.Selections[sel]; methodSelection == nil || methodSelection.Kind() != types.MethodVal {
		return false
	}
	named, ok := types.Unalias(typeInfo.GetType(field)).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil || named.Obj().Pkg().Path() != "sync" {
		return false
	}
	switch named.Obj().Name() {
	case "WaitGroup", "Mutex", "RWMutex", "Map":
	default:
		return false
	}
	// A field of the method's own receiver is read through self, which
	// holds no lock.
	root := methodReceiverRootIdent(field)
	if root == nil || isCurrentReceiverIdent(root) {
		return false
	}
	out.WriteString("{ let __sync_value = ")
	TranspileExpressionContext(out, field, LValue)
	out.WriteString(".clone(); __sync_value }.")
	writeMethodCallSelectorAndArguments(out, sel, call, false)
	return true
}

// pointerTypeMethodTakesReceiverHandle reports whether the method named
// methodName of *typeName, a type of the current package, takes the handle
// it is called on.
func pointerTypeMethodTakesReceiverHandle(typeName, methodName string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil {
		return false
	}
	return pointerTypeMethodTakesReceiverHandleIn(typeInfo.pkg, typeName, methodName)
}

// importedPointerTypeMethodTakesReceiverHandle is
// pointerTypeMethodTakesReceiverHandle for a type of the package at pkgPath,
// which the current package imports directly or indirectly.
func importedPointerTypeMethodTakesReceiverHandle(pkgPath, typeName, methodName string) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil {
		return false
	}
	seen := map[*types.Package]bool{}
	var find func(pkg *types.Package) *types.Package
	find = func(pkg *types.Package) *types.Package {
		if pkg == nil || seen[pkg] {
			return nil
		}
		seen[pkg] = true
		if pkg.Path() == pkgPath {
			return pkg
		}
		for _, imported := range pkg.Imports() {
			if found := find(imported); found != nil {
				return found
			}
		}
		return nil
	}
	return pointerTypeMethodTakesReceiverHandleIn(find(typeInfo.pkg), typeName, methodName)
}

func pointerTypeMethodTakesReceiverHandleIn(pkg *types.Package, typeName, methodName string) bool {
	if pkg == nil {
		return false
	}
	obj, ok := pkg.Scope().Lookup(typeName).(*types.TypeName)
	if !ok {
		return false
	}
	found, index, _ := types.LookupFieldOrMethod(types.NewPointer(obj.Type()), true, obj.Pkg(), methodName)
	fn, ok := found.(*types.Func)
	return ok && len(index) == 1 && methodTakesReceiverHandle(fn)
}
