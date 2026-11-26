package main

import (
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
	"strconv"
	"strings"
)

// A range over a function, Go 1.23's
//
//	for k, v := range seq { body }
//
// calls seq with a yield function that runs body for each k and v. It is
// written as that call:
//
//	var __rf_done bool
//	seq(func(k K, v V) bool { body; return true })
//	if __rf_done { return }
//
// In the yield function a break of the loop is "return false" and a
// continue is "return true". A return from the function the loop is in
// stores its results and __rf_done, returns false to stop seq, and returns
// them once seq is back. The function literal and the names it adds are
// recorded in the package's type information as if they were written in the
// program, so the rest of the translator writes them as it writes any
// other.

// rangeFuncYieldSignature gives the signature of the yield function a
// range over expr passes, or false when expr is not a function to range
// over.
func rangeFuncYieldSignature(expr ast.Expr) (*types.Signature, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	seq, ok := coreUnderlyingType(typeInfo.GetType(expr)).(*types.Signature)
	if !ok || seq.Params().Len() != 1 || seq.Results().Len() != 0 {
		return nil, false
	}
	yield, ok := coreUnderlyingType(seq.Params().At(0).Type()).(*types.Signature)
	if !ok || yield.Params().Len() > 2 || yield.Results().Len() != 1 {
		return nil, false
	}
	if basic, ok := types.Unalias(yield.Results().At(0).Type()).(*types.Basic); !ok || basic.Kind() != types.Bool {
		return nil, false
	}
	return yield, true
}

// rangeFuncCount numbers the ranges over functions, so the names each adds
// are its own.
var rangeFuncCount int

// rangeFuncRewrite holds the names a rewritten range over a function adds.
type rangeFuncRewrite struct {
	typeInfo *TypeInfo
	fnType   *ast.FuncType
	pos      token.Pos
	// label is the loop's own label, and innerLabels the labels of the
	// statements inside its body, which a branch there stays with.
	label       string
	innerLabels map[string]bool
	done        *types.Var
	// results holds, per result of the enclosing function, a [1]T whose
	// element a return in the body stores the result in. An element write
	// reaches the array the yield function captured, where an assignment to
	// a captured pointer variable would not. namedResults holds the named
	// result each one copies for a bare return, nil for a blank or unnamed
	// one.
	results      []*types.Var
	namedResults []*types.Var
	hasResults   bool
	// next numbers the branch to a loop around this one that the body took,
	// from 1, and branches holds those branches.
	next     *types.Var
	branches []*ast.BranchStmt
	suffix   string
	// deferOwner is the rewrite of the range over a function this one is
	// in the body of, nil for the outermost. A defer in the body belongs to
	// the function the outermost loop is in: it goes on deferred, a [1]func()
	// slot that a defer of that function runs (rewriteDefer).
	deferOwner   *rangeFuncRewrite
	deferred     *types.Var
	deferredDecl ast.Stmt
	deferCount   int
}

// typeExpr gives the syntax of type t, each part recorded with its type, as
// a parameter written in the program would have it: []int is an ArrayType
// over the ident int, not an ident named "[]int".
func (r *rangeFuncRewrite) typeExpr(t types.Type) ast.Expr {
	expr := r.typeSyntax(t)
	r.typeInfo.info.Types[expr] = types.TypeAndValue{Type: t}
	return expr
}

func (r *rangeFuncRewrite) typeName(obj types.Object, t types.Type) ast.Expr {
	name := &ast.Ident{NamePos: r.pos, Name: obj.Name()}
	r.typeInfo.info.Uses[name] = obj
	r.typeInfo.info.Types[name] = types.TypeAndValue{Type: t}
	if obj.Pkg() == nil || obj.Pkg() == r.typeInfo.pkg {
		return name
	}
	pkg := &ast.Ident{NamePos: r.pos, Name: obj.Pkg().Name()}
	r.typeInfo.info.Uses[pkg] = types.NewPkgName(r.pos, r.typeInfo.pkg, obj.Pkg().Name(), obj.Pkg())
	return &ast.SelectorExpr{X: pkg, Sel: name}
}

func (r *rangeFuncRewrite) fieldList(tuple *types.Tuple) *ast.FieldList {
	list := &ast.FieldList{Opening: r.pos}
	for i := 0; i < tuple.Len(); i++ {
		list.List = append(list.List, &ast.Field{Type: r.typeExpr(tuple.At(i).Type())})
	}
	return list
}

func (r *rangeFuncRewrite) typeSyntax(t types.Type) ast.Expr {
	switch tt := t.(type) {
	case *types.Basic:
		obj := types.Universe.Lookup(tt.Name())
		if tt.Kind() == types.UnsafePointer {
			obj = types.Unsafe.Scope().Lookup("Pointer")
		}
		if obj != nil {
			return r.typeName(obj, t)
		}
	case *types.Alias:
		return r.typeName(tt.Obj(), t)
	case *types.TypeParam:
		return r.typeName(tt.Obj(), t)
	case *types.Named:
		base := r.typeName(tt.Obj(), t)
		args := tt.TypeArgs()
		if args == nil || args.Len() == 0 {
			return base
		}
		indices := make([]ast.Expr, args.Len())
		for i := range indices {
			indices[i] = r.typeExpr(args.At(i))
		}
		if len(indices) == 1 {
			return &ast.IndexExpr{X: base, Index: indices[0]}
		}
		return &ast.IndexListExpr{X: base, Indices: indices}
	case *types.Pointer:
		return &ast.StarExpr{Star: r.pos, X: r.typeExpr(tt.Elem())}
	case *types.Slice:
		return &ast.ArrayType{Lbrack: r.pos, Elt: r.typeExpr(tt.Elem())}
	case *types.Array:
		length := &ast.BasicLit{ValuePos: r.pos, Kind: token.INT, Value: strconv.FormatInt(tt.Len(), 10)}
		r.typeInfo.info.Types[length] = types.TypeAndValue{Type: types.Typ[types.UntypedInt], Value: constant.MakeInt64(tt.Len())}
		return &ast.ArrayType{Lbrack: r.pos, Len: length, Elt: r.typeExpr(tt.Elem())}
	case *types.Map:
		return &ast.MapType{Map: r.pos, Key: r.typeExpr(tt.Key()), Value: r.typeExpr(tt.Elem())}
	case *types.Chan:
		dir := ast.SEND | ast.RECV
		switch tt.Dir() {
		case types.SendOnly:
			dir = ast.SEND
		case types.RecvOnly:
			dir = ast.RECV
		}
		return &ast.ChanType{Begin: r.pos, Dir: dir, Value: r.typeExpr(tt.Elem())}
	case *types.Signature:
		fn := &ast.FuncType{Func: r.pos, Params: r.fieldList(tt.Params())}
		if tt.Results().Len() > 0 {
			fn.Results = r.fieldList(tt.Results())
		}
		return fn
	case *types.Interface:
		if tt.Empty() {
			return r.typeName(types.Universe.Lookup("any"), t)
		}
	case *types.Struct:
		fields := &ast.FieldList{Opening: r.pos}
		for i := 0; i < tt.NumFields(); i++ {
			field := tt.Field(i)
			f := &ast.Field{Type: r.typeExpr(field.Type())}
			if !field.Anonymous() {
				name := &ast.Ident{NamePos: r.pos, Name: field.Name()}
				r.typeInfo.info.Defs[name] = field
				f.Names = []*ast.Ident{name}
			}
			fields.List = append(fields.List, f)
		}
		return &ast.StructType{Struct: r.pos, Fields: fields}
	}
	// Any other type keeps its spelling, with the type recorded.
	return &ast.Ident{NamePos: r.pos, Name: types.TypeString(t, func(p *types.Package) string {
		if r.typeInfo.pkg != nil && p == r.typeInfo.pkg {
			return ""
		}
		return p.Name()
	})}
}

func (r *rangeFuncRewrite) use(v *types.Var) *ast.Ident {
	ident := &ast.Ident{NamePos: r.pos, Name: v.Name()}
	r.typeInfo.info.Uses[ident] = v
	r.typeInfo.info.Types[ident] = types.TypeAndValue{Type: v.Type()}
	return ident
}

// slotType is the type of a variable whose element or field the yield
// function writes a value of type t to: [1]T, or struct{ v T } for a map,
// which an array holds bare, without a nil.
func (r *rangeFuncRewrite) slotType(t types.Type) types.Type {
	if _, ok := t.Underlying().(*types.Map); ok {
		return types.NewStruct([]*types.Var{types.NewField(r.pos, r.typeInfo.pkg, "v", t, false)}, nil)
	}
	return types.NewArray(t, 1)
}

// newSlotVar declares a slot variable of slotType(t). A struct slot is put
// in the scope at the loop, so that its field selector is type-checked.
func (r *rangeFuncRewrite) newSlotVar(name string, t types.Type) (*types.Var, ast.Stmt) {
	v, decl := r.newVar(name, r.slotType(t))
	if _, ok := v.Type().(*types.Struct); ok {
		if scope := scopeAtPos(r.typeInfo.pkg, r.pos); scope != nil {
			scope.Insert(v)
		}
	}
	return v, decl
}

// scopeAtPos is the innermost scope at pos, as types.CheckExpr finds it.
func scopeAtPos(pkg *types.Package, pos token.Pos) *types.Scope {
	if pkg == nil {
		return nil
	}
	for i := 0; i < pkg.Scope().NumChildren(); i++ {
		if scope := pkg.Scope().Child(i).Innermost(pos); scope != nil {
			return scope
		}
	}
	return nil
}

// resultSlot is the element of a [1]T slot variable, or the field of a
// struct{ v T } one.
func (r *rangeFuncRewrite) resultSlot(v *types.Var) ast.Expr {
	if _, ok := v.Type().(*types.Struct); ok {
		sel := &ast.SelectorExpr{X: &ast.Ident{NamePos: r.pos, Name: v.Name()}, Sel: &ast.Ident{NamePos: r.pos, Name: "v"}}
		if err := types.CheckExpr(token.NewFileSet(), r.typeInfo.pkg, r.pos, sel, r.typeInfo.info); err == nil {
			return sel
		}
	}
	array, _ := v.Type().(*types.Array)
	index := &ast.IndexExpr{X: r.use(v), Lbrack: r.pos, Index: r.intLit(0), Rbrack: r.pos}
	if array != nil {
		r.typeInfo.info.Types[index] = types.TypeAndValue{Type: array.Elem()}
	}
	return index
}

func (r *rangeFuncRewrite) boolLit(value bool) *ast.Ident {
	name := "false"
	if value {
		name = "true"
	}
	ident := &ast.Ident{NamePos: r.pos, Name: name}
	r.typeInfo.info.Uses[ident] = types.Universe.Lookup(name)
	r.typeInfo.info.Types[ident] = types.TypeAndValue{Type: types.Typ[types.UntypedBool]}
	return ident
}

func (r *rangeFuncRewrite) intLit(value int) *ast.BasicLit {
	lit := &ast.BasicLit{ValuePos: r.pos, Kind: token.INT, Value: itoaRangeFunc(value)}
	r.typeInfo.info.Types[lit] = types.TypeAndValue{Type: types.Typ[types.Int], Value: constant.MakeInt64(int64(value))}
	return lit
}

func (r *rangeFuncRewrite) newVar(name string, t types.Type) (*types.Var, ast.Stmt) {
	v := types.NewVar(r.pos, r.typeInfo.pkg, name, t)
	ident := &ast.Ident{NamePos: r.pos, Name: name}
	r.typeInfo.info.Defs[ident] = v
	return v, &ast.DeclStmt{Decl: &ast.GenDecl{TokPos: r.pos, Tok: token.VAR, Specs: []ast.Spec{&ast.ValueSpec{Names: []*ast.Ident{ident}, Type: r.typeExpr(t)}}}}
}

func (r *rangeFuncRewrite) assign(lhs ast.Expr, rhs ast.Expr, pos token.Pos) ast.Stmt {
	return &ast.AssignStmt{Lhs: []ast.Expr{lhs}, TokPos: pos, Tok: token.ASSIGN, Rhs: []ast.Expr{rhs}}
}

// rangeOverFuncBodyDefers reports whether s is a range over a function
// whose body has a defer outside a function literal.
func rangeOverFuncBodyDefers(s *ast.RangeStmt) bool {
	if _, ok := rangeFuncYieldSignature(s.X); !ok {
		return false
	}
	found := false
	ast.Inspect(s.Body, func(n ast.Node) bool {
		switch n.(type) {
		case *ast.FuncLit:
			return false
		case *ast.DeferStmt:
			found = true
		}
		return !found
	})
	return found
}

// emptySignature is the type func().
var emptySignature = types.NewSignatureType(nil, nil, nil, nil, nil, false)

// funcLit gives func() { body }, recorded with its type.
func (r *rangeFuncRewrite) funcLit(body ...ast.Stmt) *ast.FuncLit {
	lit := &ast.FuncLit{
		Type: &ast.FuncType{Func: r.pos, Params: &ast.FieldList{Opening: r.pos, Closing: r.pos}},
		Body: &ast.BlockStmt{Lbrace: r.pos, List: body, Rbrace: r.pos},
	}
	r.typeInfo.info.Types[lit] = types.TypeAndValue{Type: emptySignature}
	return lit
}

// rewriteDefer writes a defer in the body as the push of the deferred call
// onto the defers of the function the outermost loop is in:
//
//	__rf_fn := fn; __rf_arg_0 := x
//	__rf_call := func() { __rf_fn(__rf_arg_0) }
//	__rf_defers[0] = append(__rf_defers[0], __rf_call)
//
// A function value that is not a declared function, a builtin or a method
// is evaluated where the defer is, and so are the arguments, as Go does; a
// constant argument is left as it is, and each result of a call that is
// the only argument gets a name of its own. The pushed calls are run by the
// defer that deferredRunner adds before the loop.
func (r *rangeFuncRewrite) rewriteDefer(s *ast.DeferStmt) (ast.Stmt, bool) {
	owner := r
	for owner.deferOwner != nil {
		owner = owner.deferOwner
	}
	callType, ok := r.typeInfo.info.Types[s.Call]
	if !ok {
		return nil, false
	}
	if owner.deferred == nil {
		owner.deferred, owner.deferredDecl = owner.newVar("__rf_defers_"+owner.suffix, types.NewArray(types.NewSlice(emptySignature), 1))
		if scope := scopeAtPos(r.typeInfo.pkg, owner.pos); scope != nil {
			scope.Insert(owner.deferred)
		}
	}
	owner.deferCount++
	at := owner.suffix + "_" + itoaRangeFunc(owner.deferCount)
	var list []ast.Stmt
	define := func(rhs ast.Expr, vars ...*types.Var) {
		lhs := make([]ast.Expr, len(vars))
		for i, v := range vars {
			ident := &ast.Ident{NamePos: r.pos, Name: v.Name()}
			r.typeInfo.info.Defs[ident] = v
			lhs[i] = ident
		}
		list = append(list, &ast.AssignStmt{Lhs: lhs, TokPos: r.pos, Tok: token.DEFINE, Rhs: []ast.Expr{rhs}})
	}
	call := &ast.CallExpr{Fun: s.Call.Fun, Lparen: s.Call.Lparen, Ellipsis: s.Call.Ellipsis, Rparen: s.Call.Rparen}
	r.typeInfo.info.Types[call] = callType
	if !r.deferredFuncIsStatic(s.Call.Fun) {
		funType, ok := r.typeInfo.info.Types[s.Call.Fun]
		if !ok || funType.Type == nil {
			return nil, false
		}
		fn := types.NewVar(r.pos, r.typeInfo.pkg, "__rf_fn_"+at, funType.Type)
		define(s.Call.Fun, fn)
		call.Fun = r.use(fn)
	}
	for i, arg := range s.Call.Args {
		tv, ok := r.typeInfo.info.Types[arg]
		if !ok || tv.Type == nil {
			return nil, false
		}
		if tv.Value != nil || tv.IsNil() {
			call.Args = append(call.Args, arg)
			continue
		}
		if tuple, ok := tv.Type.(*types.Tuple); ok {
			// f(g()) of a g with several results.
			vars := make([]*types.Var, tuple.Len())
			for j := range vars {
				vars[j] = types.NewVar(r.pos, r.typeInfo.pkg, "__rf_arg_"+at+"_"+itoaRangeFunc(i)+"_"+itoaRangeFunc(j), tuple.At(j).Type())
				call.Args = append(call.Args, r.use(vars[j]))
			}
			define(arg, vars...)
			continue
		}
		v := types.NewVar(r.pos, r.typeInfo.pkg, "__rf_arg_"+at+"_"+itoaRangeFunc(i), tv.Type)
		define(arg, v)
		call.Args = append(call.Args, r.use(v))
	}
	pushed := types.NewVar(r.pos, r.typeInfo.pkg, "__rf_call_"+at, emptySignature)
	define(r.funcLit(&ast.ExprStmt{X: call}), pushed)
	// append is checked as written, which records it as the builtin.
	slot := owner.resultSlot(owner.deferred)
	appendCall := &ast.CallExpr{Fun: &ast.Ident{NamePos: r.pos, Name: "append"}, Lparen: r.pos, Args: []ast.Expr{&ast.IndexExpr{X: &ast.Ident{NamePos: r.pos, Name: owner.deferred.Name()}, Lbrack: r.pos, Index: r.intLit(0), Rbrack: r.pos}, &ast.Ident{NamePos: r.pos, Name: pushed.Name()}}, Rparen: r.pos}
	scope := scopeAtPos(r.typeInfo.pkg, owner.pos)
	if scope == nil {
		return nil, false
	}
	scope.Insert(pushed)
	err := types.CheckExpr(token.NewFileSet(), r.typeInfo.pkg, owner.pos, appendCall, r.typeInfo.info)
	if err != nil {
		return nil, false
	}
	list = append(list, owner.assign(slot, appendCall, r.pos))
	// The statements take the defer's place in its list, not a block of
	// their own: a closure's captures are taken before the statement that
	// holds it, after the names it captures are declared.
	block := &ast.BlockStmt{Lbrace: s.Pos(), List: list, Rbrace: s.End()}
	rangeFuncSplices[block] = true
	return block, true
}

// deferredFuncIsStatic reports whether the function a deferred call calls
// is the same whenever it is read: a declared function, a builtin, a
// function literal, or a method, whose receiver is still read when the
// defer runs.
func (r *rangeFuncRewrite) deferredFuncIsStatic(fun ast.Expr) bool {
	switch f := ast.Unparen(fun).(type) {
	case *ast.FuncLit:
		return true
	case *ast.Ident:
		switch r.typeInfo.info.Uses[f].(type) {
		case *types.Func, *types.Builtin:
			return true
		}
	case *ast.SelectorExpr:
		if sel := r.typeInfo.info.Selections[f]; sel != nil {
			return sel.Kind() == types.MethodVal
		}
		_, isFunc := r.typeInfo.info.Uses[f.Sel].(*types.Func)
		return isFunc
	}
	return false
}

// deferredRunner gives the statements before the loop that declare its
// defers and defer running them:
//
//	var __rf_defers [1][]func()
//	defer func() { for _, f := range __rf_defers[0] { defer f() } }()
//
// Each pushed call is a defer of the runner, so they run last first, and a
// panic in one does not keep the others from running.
func (r *rangeFuncRewrite) deferredRunner() []ast.Stmt {
	f := types.NewVar(r.pos, r.typeInfo.pkg, "__rf_f_"+r.suffix, emptySignature)
	fIdent := &ast.Ident{NamePos: r.pos, Name: f.Name()}
	r.typeInfo.info.Defs[fIdent] = f
	deferCall := &ast.CallExpr{Fun: r.use(f), Lparen: r.pos, Rparen: r.pos}
	r.typeInfo.info.Types[deferCall] = types.TypeAndValue{Type: types.NewTuple()}
	loop := &ast.RangeStmt{
		For:    r.pos,
		Key:    &ast.Ident{NamePos: r.pos, Name: "_"},
		Value:  fIdent,
		TokPos: r.pos,
		Tok:    token.DEFINE,
		X:      r.resultSlot(r.deferred),
		Body:   &ast.BlockStmt{Lbrace: r.pos, List: []ast.Stmt{&ast.DeferStmt{Defer: r.pos, Call: deferCall}}, Rbrace: r.pos},
	}
	runner := &ast.CallExpr{Fun: r.funcLit(loop), Lparen: r.pos, Rparen: r.pos}
	r.typeInfo.info.Types[runner] = types.TypeAndValue{Type: types.NewTuple()}
	return []ast.Stmt{
		r.deferredDecl,
		&ast.DeferStmt{Defer: r.pos, Call: runner},
	}
}

// rangeFuncSplices holds the blocks that stand for the statements a nested
// range over a function is, which the list holding one takes in its place.
var rangeFuncSplices = map[*ast.BlockStmt]bool{}

// rewriteStmt rewrites the statements of the loop body. inLoop and inSwitch
// tell whether the statement is inside a loop, or a switch or select, within
// the body, which an unlabeled break or continue belongs to.
func (r *rangeFuncRewrite) rewriteStmt(stmt ast.Stmt, inLoop bool, inSwitch bool) ast.Stmt {
	switch s := stmt.(type) {
	case *ast.DeferStmt:
		if rewritten, ok := r.rewriteDefer(s); ok {
			return rewritten
		}
		return s
	case *ast.BranchStmt:
		if s.Tok != token.BREAK && s.Tok != token.CONTINUE {
			return s
		}
		if s.Label == nil {
			if s.Tok == token.BREAK && !inLoop && !inSwitch {
				return &ast.ReturnStmt{Return: s.Pos(), Results: []ast.Expr{r.boolLit(false)}}
			}
			if s.Tok == token.CONTINUE && !inLoop {
				return &ast.ReturnStmt{Return: s.Pos(), Results: []ast.Expr{r.boolLit(true)}}
			}
			return s
		}
		if r.label != "" && s.Label.Name == r.label {
			return &ast.ReturnStmt{Return: s.Pos(), Results: []ast.Expr{r.boolLit(s.Tok == token.CONTINUE)}}
		}
		if r.innerLabels[s.Label.Name] {
			return s
		}
		// A branch to a loop around this one stops seq, and is taken once
		// seq is back.
		r.branches = append(r.branches, s)
		return &ast.BlockStmt{Lbrace: s.Pos(), List: []ast.Stmt{
			r.assign(r.use(r.next), r.intLit(len(r.branches)), s.Pos()),
			&ast.ReturnStmt{Return: s.Pos(), Results: []ast.Expr{r.boolLit(false)}},
		}, Rbrace: s.End()}
	case *ast.ReturnStmt:
		var list []ast.Stmt
		if len(s.Results) == len(r.results) {
			// Each result goes to its own private slot, so storing them in
			// turn is storing them at once.
			for i, v := range r.results {
				list = append(list, r.assign(r.resultSlot(v), s.Results[i], s.Pos()))
			}
		} else if len(s.Results) > 0 {
			// return f() of a call with several results.
			lhs := make([]ast.Expr, len(r.results))
			for i, v := range r.results {
				lhs[i] = r.resultSlot(v)
			}
			list = append(list, &ast.AssignStmt{Lhs: lhs, TokPos: s.Pos(), Tok: token.ASSIGN, Rhs: s.Results})
		} else {
			// A bare return returns the named results as they are now.
			for i, named := range r.namedResults {
				if named != nil {
					list = append(list, r.assign(r.resultSlot(r.results[i]), r.use(named), s.Pos()))
				}
			}
		}
		list = append(list,
			r.assign(r.use(r.done), r.boolLit(true), s.Pos()),
			&ast.ReturnStmt{Return: s.Pos(), Results: []ast.Expr{r.boolLit(false)}})
		return &ast.BlockStmt{Lbrace: s.Pos(), List: list, Rbrace: s.End()}
	case *ast.BlockStmt:
		copied := *s
		copied.List = r.rewriteList(s.List, inLoop, inSwitch)
		return &copied
	case *ast.IfStmt:
		copied := *s
		copied.Body = r.rewriteStmt(s.Body, inLoop, inSwitch).(*ast.BlockStmt)
		if s.Else != nil {
			copied.Else = r.rewriteStmt(s.Else, inLoop, inSwitch)
		}
		return &copied
	case *ast.LabeledStmt:
		if rangeStmt, ok := s.Stmt.(*ast.RangeStmt); ok {
			if expanded, ok := r.rewriteNestedRangeOverFunc(rangeStmt, s.Label.Name, inLoop, inSwitch); ok {
				return expanded
			}
		}
		copied := *s
		copied.Stmt = r.rewriteStmt(s.Stmt, inLoop, inSwitch)
		return &copied
	case *ast.ForStmt:
		copied := *s
		copied.Body = r.rewriteStmt(s.Body, true, inSwitch).(*ast.BlockStmt)
		return &copied
	case *ast.RangeStmt:
		if expanded, ok := r.rewriteNestedRangeOverFunc(s, "", inLoop, inSwitch); ok {
			return expanded
		}
		copied := *s
		copied.Body = r.rewriteStmt(s.Body, true, inSwitch).(*ast.BlockStmt)
		return &copied
	case *ast.SwitchStmt:
		copied := *s
		copied.Body = r.rewriteClauses(s.Body, inLoop)
		return &copied
	case *ast.TypeSwitchStmt:
		copied := *s
		copied.Body = r.rewriteClauses(s.Body, inLoop)
		return &copied
	case *ast.SelectStmt:
		copied := *s
		copied.Body = r.rewriteClauses(s.Body, inLoop)
		return &copied
	}
	return stmt
}

// rewriteNestedRangeOverFunc writes a range over a function inside the body
// as the statements it is, then rewrites those: what returns from it or
// branches out of it once its own function is back is this body's to write.
func (r *rangeFuncRewrite) rewriteNestedRangeOverFunc(s *ast.RangeStmt, label string, inLoop bool, inSwitch bool) (ast.Stmt, bool) {
	stmts, ok := expandRangeOverFuncIn(s, label, r.fnType, r)
	if !ok {
		return nil, false
	}
	block := &ast.BlockStmt{Lbrace: s.Pos(), List: r.rewriteList(stmts, inLoop, inSwitch), Rbrace: s.End()}
	rangeFuncSplices[block] = true
	return block, true
}

func (r *rangeFuncRewrite) rewriteList(list []ast.Stmt, inLoop bool, inSwitch bool) []ast.Stmt {
	out := make([]ast.Stmt, 0, len(list))
	for _, stmt := range list {
		rewritten := r.rewriteStmt(stmt, inLoop, inSwitch)
		if block, ok := rewritten.(*ast.BlockStmt); ok && rangeFuncSplices[block] {
			delete(rangeFuncSplices, block)
			out = append(out, block.List...)
			continue
		}
		out = append(out, rewritten)
	}
	return out
}

func (r *rangeFuncRewrite) rewriteClauses(body *ast.BlockStmt, inLoop bool) *ast.BlockStmt {
	copied := *body
	copied.List = make([]ast.Stmt, len(body.List))
	for i, clause := range body.List {
		switch c := clause.(type) {
		case *ast.CaseClause:
			cc := *c
			cc.Body = r.rewriteList(c.Body, inLoop, true)
			copied.List[i] = &cc
		case *ast.CommClause:
			cc := *c
			cc.Body = r.rewriteList(c.Body, inLoop, true)
			copied.List[i] = &cc
		default:
			copied.List[i] = clause
		}
	}
	return &copied
}

// statementLabels gives the labels of the statements in body.
func statementLabels(body ast.Node) map[string]bool {
	labels := map[string]bool{}
	ast.Inspect(body, func(node ast.Node) bool {
		if _, ok := node.(*ast.FuncLit); ok {
			return false
		}
		if labeled, ok := node.(*ast.LabeledStmt); ok {
			labels[labeled.Label.Name] = true
		}
		return true
	})
	return labels
}

// expandRangeOverFunc gives the statements a range over a function is: the
// declarations of the names it adds, the call of the function with the yield
// function, and the return or branch that the body took, once the call is
// back. fnType is the function the loop is in. It reports false when s does
// not range over a function.
func expandRangeOverFunc(s *ast.RangeStmt, label string, fnType *ast.FuncType) ([]ast.Stmt, bool) {
	return expandRangeOverFuncIn(s, label, fnType, nil)
}

// expandRangeOverFuncIn is expandRangeOverFunc for a range in the body of
// the one owner rewrites, nil for one that is not.
func expandRangeOverFuncIn(s *ast.RangeStmt, label string, fnType *ast.FuncType, owner *rangeFuncRewrite) ([]ast.Stmt, bool) {
	yield, ok := rangeFuncYieldSignature(s.X)
	if !ok {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	r := &rangeFuncRewrite{typeInfo: typeInfo, fnType: fnType, pos: s.For, label: label, innerLabels: statementLabels(s.Body), deferOwner: owner}
	var prelude []ast.Stmt
	var decl ast.Stmt
	rangeFuncCount++
	r.suffix = itoaRangeFunc(rangeFuncCount)
	r.done, decl = r.newVar("__rf_done_"+r.suffix, types.Typ[types.Bool])
	prelude = append(prelude, decl)
	r.next, decl = r.newVar("__rf_next_"+r.suffix, types.Typ[types.Int])
	nextDecl := decl
	if fnType != nil && fnType.Results != nil {
		for _, field := range fnType.Results.List {
			resultType := typeInfo.GetType(field.Type)
			if resultType == nil {
				return nil, false
			}
			r.hasResults = true
			names := field.Names
			if len(names) == 0 {
				names = []*ast.Ident{nil}
			}
			for _, name := range names {
				var named *types.Var
				if name != nil && name.Name != "_" {
					v, ok := typeInfo.info.Defs[name].(*types.Var)
					if !ok {
						return nil, false
					}
					named = v
				}
				v, decl := r.newSlotVar("__rf_result_"+r.suffix+"_"+itoaRangeFunc(len(r.results)), resultType)
				r.results = append(r.results, v)
				r.namedResults = append(r.namedResults, named)
				prelude = append(prelude, decl)
			}
		}
	}

	// The yield function's parameters are its own. for k, v := range seq
	// gives each iteration new k and v, bound from them, which the body can
	// change without changing what seq passed. for k, v = range seq assigns
	// k and then v, as Go's own lowering of a range over a function does:
	// for i, a[i] = range seq indexes a with the new i, unlike the same
	// range over a slice.
	params := &ast.FieldList{Opening: s.For}
	var lhs, rhs []ast.Expr
	for i := 0; i < yield.Params().Len(); i++ {
		bound := s.Key
		if i == 1 {
			bound = s.Value
		}
		paramType := yield.Params().At(i).Type()
		name := &ast.Ident{NamePos: s.For, Name: "_"}
		if ident, ok := bound.(*ast.Ident); ok && ident.Name == "_" {
			bound = nil
		}
		if bound != nil {
			param := types.NewVar(s.For, typeInfo.pkg, "__rf_param_"+r.suffix+"_"+itoaRangeFunc(i), paramType)
			name = &ast.Ident{NamePos: s.For, Name: param.Name()}
			typeInfo.info.Defs[name] = param
			lhs = append(lhs, bound)
			rhs = append(rhs, r.use(param))
		}
		params.List = append(params.List, &ast.Field{Names: []*ast.Ident{name}, Type: r.typeExpr(paramType)})
	}
	var bindings []ast.Stmt
	var copyBacks []ast.Stmt
	if len(lhs) > 0 && s.Tok == token.DEFINE {
		bindings = append(bindings, &ast.AssignStmt{Lhs: lhs, TokPos: s.For, Tok: token.DEFINE, Rhs: rhs})
	} else {
		for i := range lhs {
			bindings = append(bindings, r.assign(lhs[i], rhs[i], s.For))
			// A pointer, map or channel variable is its handle, and the
			// yield function assigns only its own copy of it. The value
			// bound last goes through a slot, whose element the yield
			// function writes, to the variable once seq is back. A package
			// global is written in place, which seq can write after its
			// last yield too. A variable that the body assigns as well is
			// given a cell before translation (closure_box.go), and is no
			// longer an identifier here.
			ident, ok := lhs[i].(*ast.Ident)
			if !ok || isPackageGlobalObjectIdent(ident) {
				continue
			}
			targetType := typeInfo.GetType(ident)
			switch types.Unalias(targetType).Underlying().(type) {
			case *types.Pointer, *types.Chan, *types.Map:
			default:
				continue
			}
			v, decl := r.newSlotVar("__rf_bound_"+r.suffix+"_"+itoaRangeFunc(i), targetType)
			prelude = append(prelude, decl, r.assign(r.resultSlot(v), ident, s.For))
			bindings = append(bindings, r.assign(r.resultSlot(v), rhs[i], s.For))
			target := &ast.Ident{NamePos: ident.Pos(), Name: ident.Name}
			typeInfo.info.Uses[target] = typeInfo.info.Uses[ident]
			typeInfo.info.Types[target] = types.TypeAndValue{Type: targetType}
			copyBacks = append(copyBacks, r.assign(target, r.resultSlot(v), s.End()))
		}
	}
	body := r.rewriteStmt(s.Body, false, false).(*ast.BlockStmt)
	body.List = append(append(bindings, body.List...), &ast.ReturnStmt{Return: s.Body.Rbrace, Results: []ast.Expr{r.boolLit(true)}})
	if r.deferred != nil {
		prelude = append(prelude, r.deferredRunner()...)
	}
	yieldType := &ast.FuncType{Func: s.For, Params: params, Results: &ast.FieldList{List: []*ast.Field{{Type: r.typeExpr(yield.Results().At(0).Type())}}}}
	funcLit := &ast.FuncLit{Type: yieldType, Body: body}
	typeInfo.info.Types[funcLit] = types.TypeAndValue{Type: yield}
	call := &ast.CallExpr{Fun: s.X, Lparen: s.X.End(), Args: []ast.Expr{funcLit}, Rparen: s.End()}
	typeInfo.info.Types[call] = types.TypeAndValue{Type: types.NewTuple()}

	if len(r.branches) > 0 {
		prelude = append(prelude, nextDecl)
	}
	stmts := append(prelude, &ast.ExprStmt{X: call})
	stmts = append(stmts, copyBacks...)
	var after []ast.Stmt
	if r.hasResults {
		results := make([]ast.Expr, len(r.results))
		for i, v := range r.results {
			results[i] = r.resultSlot(v)
		}
		after = []ast.Stmt{&ast.ReturnStmt{Return: s.End(), Results: results}}
	} else {
		after = []ast.Stmt{&ast.ReturnStmt{Return: s.End()}}
	}
	stmts = append(stmts, &ast.IfStmt{If: s.End(), Cond: r.use(r.done), Body: &ast.BlockStmt{Lbrace: s.End(), List: after, Rbrace: s.End()}})
	for i, branch := range r.branches {
		cond := &ast.BinaryExpr{X: r.use(r.next), OpPos: s.End(), Op: token.EQL, Y: r.intLit(i + 1)}
		typeInfo.info.Types[cond] = types.TypeAndValue{Type: types.Typ[types.UntypedBool]}
		taken := &ast.BranchStmt{TokPos: s.End(), Tok: branch.Tok, Label: branch.Label}
		stmts = append(stmts, &ast.IfStmt{If: s.End(), Cond: cond, Body: &ast.BlockStmt{Lbrace: s.End(), List: []ast.Stmt{taken}, Rbrace: s.End()}})
	}
	return stmts, true
}

// writeRangeOverFuncStatement writes a range over a function as the call it
// is, and reports whether s ranges over one. label is the loop's label.
func writeRangeOverFuncStatement(out *strings.Builder, s *ast.RangeStmt, label string, fnType *ast.FuncType, fileSet *token.FileSet, comments []*ast.CommentGroup, lastPos *token.Pos, indent string) bool {
	stmts, ok := expandRangeOverFunc(s, label, fnType)
	if !ok {
		return false
	}
	// The statements are written in turn, not as a block: the yield
	// function's captures are taken just before the statement holding it,
	// after the names the loop adds are declared.
	for i, stmt := range stmts {
		if i > 0 {
			out.WriteString("\n")
			out.WriteString(indent)
		}
		TranspileStatement(out, stmt, fnType, fileSet, comments, lastPos, indent)
	}
	return true
}

func itoaRangeFunc(n int) string {
	const digits = "0123456789"
	if n < 10 {
		return digits[n : n+1]
	}
	return itoaRangeFunc(n/10) + digits[n%10:n%10+1]
}
