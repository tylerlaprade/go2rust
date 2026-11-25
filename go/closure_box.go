package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strconv"

	"golang.org/x/tools/go/ast/astutil"
)

// A pointer, map or channel variable is its handle, Rc<RefCell<Option<T>>>,
// and a closure captures a clone of that handle. Assigning the variable
// rebinds the Rust variable it is assigned through, so an assignment inside
// a closure did not reach the variable outside, and a closure kept the
// handle the variable had when the closure was made:
//
//	var found *Node
//	walk(root, func(n *Node) { found = n }) // found stayed nil
//
// Every other kind of variable is a cell that the closure shares, and an
// assignment writes into it. boxCapturedReboundVariables gives such a
// pointer, map or channel variable a cell too, by rewriting the function in
// Go before it is translated:
//
//	var found *Node
//	__cell_found_1 := struct{ v *Node }{found}
//	walk(root, func(n *Node) { __cell_found_1.v = n })
//
// Every use of the variable after its declaration becomes __cell_found_1.v.
// A closure shares the struct variable, and a field assignment writes into
// it. A variable is rewritten when a closure uses it and it is assigned
// inside a closure, or after the first closure that uses it.

// boxCapturedReboundVariables rewrites the functions of files.
func boxCapturedReboundVariables(typeInfo *TypeInfo, fileSet *token.FileSet, files []*ast.File) {
	if typeInfo == nil || typeInfo.info == nil || typeInfo.pkg == nil {
		return
	}
	if fileSet == nil {
		fileSet = token.NewFileSet()
	}
	// rangeFuncYieldSignature reads the package's types from here.
	prevTypeInfo := GetTypeInfo()
	SetTypeInfo(typeInfo)
	defer SetTypeInfo(prevTypeInfo)
	boxer := &capturedVarBoxer{typeInfo: typeInfo, fileSet: fileSet}
	for _, file := range files {
		if file == nil || boxedCaptureFiles[file] {
			continue
		}
		boxedCaptureFiles[file] = true
		for _, decl := range file.Decls {
			switch d := decl.(type) {
			case *ast.FuncDecl:
				if d.Body == nil || (d.Type.TypeParams != nil && len(d.Type.TypeParams.List) > 0) {
					continue
				}
				if d.Recv != nil && len(d.Recv.List) > 0 && recvHasTypeParams(d.Recv) {
					continue
				}
				boxer.unit(d.Type, d.Recv, d.Body)
			case *ast.GenDecl:
				if d.Tok != token.VAR {
					continue
				}
				for _, spec := range d.Specs {
					vs, ok := spec.(*ast.ValueSpec)
					if !ok {
						continue
					}
					for _, value := range vs.Values {
						if lit, ok := ast.Unparen(value).(*ast.FuncLit); ok {
							boxer.unit(lit.Type, nil, lit.Body)
						}
					}
				}
			}
		}
	}
}

// boxedCaptureFiles holds the files already rewritten, so that a file
// reached by two entry points is rewritten once.
var boxedCaptureFiles = map[*ast.File]bool{}

func recvHasTypeParams(recv *ast.FieldList) bool {
	for _, field := range recv.List {
		t := field.Type
		if star, ok := t.(*ast.StarExpr); ok {
			t = star.X
		}
		switch t.(type) {
		case *ast.IndexExpr, *ast.IndexListExpr:
			return true
		}
	}
	return false
}

type capturedVarBoxer struct {
	typeInfo *TypeInfo
	fileSet  *token.FileSet
	next     int
}

type capturedVar struct {
	obj *types.Var
	// declLit is the innermost function literal, or body of a range over a
	// function, the variable is declared in; nil for the function itself.
	declLit ast.Node
	// body is the body of the function or literal whose parameter the
	// variable is, nil for a variable declared by a statement.
	body *ast.BlockStmt
	// inList says a statement directly in a statement list declares it, so
	// the cell can be declared right after.
	inList       bool
	uses         []*ast.Ident
	firstCapture token.Pos
	// captureLoops holds the loops around a closure that uses the
	// variable.
	captureLoops map[ast.Node]bool
	assignments  []capturedVarAssignment
	rejected     bool
	cell         *types.Var
}

type capturedVarAssignment struct {
	pos   token.Pos
	lit   ast.Node
	loops []ast.Node
}

// boxableVarType reports whether a variable of type t is its handle: a
// pointer, a map or a channel. A defined type with methods is left alone,
// since a method with a pointer receiver takes the variable's address.
func boxableVarType(t types.Type) bool {
	if named, ok := types.Unalias(t).(*types.Named); ok && named.NumMethods() > 0 {
		return false
	}
	switch t.Underlying().(type) {
	case *types.Pointer, *types.Map, *types.Chan:
		return true
	}
	return false
}

func (b *capturedVarBoxer) unit(fnType *ast.FuncType, recv *ast.FieldList, body *ast.BlockStmt) {
	info := b.typeInfo.info
	vars := map[*types.Var]*capturedVar{}
	candidate := func(ident *ast.Ident, lit ast.Node, paramBody *ast.BlockStmt) {
		v, ok := info.Defs[ident].(*types.Var)
		if !ok || v.IsField() || !boxableVarType(v.Type()) {
			return
		}
		vars[v] = &capturedVar{obj: v, declLit: lit, body: paramBody}
	}
	reject := func(ident *ast.Ident) {
		if v, ok := info.Defs[ident].(*types.Var); ok {
			if cv := vars[v]; cv != nil {
				cv.rejected = true
			}
		}
		if v, ok := info.Uses[ident].(*types.Var); ok {
			if cv := vars[v]; cv != nil {
				cv.rejected = true
			}
		}
	}
	params := func(fields *ast.FieldList, lit ast.Node, paramBody *ast.BlockStmt) {
		if fields == nil || paramBody == nil {
			return
		}
		for _, field := range fields.List {
			for _, name := range field.Names {
				candidate(name, lit, paramBody)
			}
		}
	}
	params(fnType.Params, nil, body)

	// stack holds the closures around a node: function literals, and the
	// bodies of ranges over a function, which are translated as the yield
	// function. rangeBodies maps such a body to its range statement.
	var stack []ast.Node
	rangeBodies := map[*ast.BlockStmt]*ast.RangeStmt{}
	var loops []ast.Node
	var nodes []ast.Node
	commAssigns := map[ast.Stmt]bool{}
	top := func() ast.Node {
		if len(stack) == 0 {
			return nil
		}
		return stack[len(stack)-1]
	}
	// Results and the receiver are written back by the translation of a
	// return and of a method, so they are never rewritten. They are not
	// candidates; nothing to do for them.
	_ = recv
	ast.Inspect(body, func(n ast.Node) bool {
		if n == nil {
			last := nodes[len(nodes)-1]
			nodes = nodes[:len(nodes)-1]
			if lit, ok := last.(*ast.FuncLit); ok && len(stack) > 0 && stack[len(stack)-1] == lit {
				stack = stack[:len(stack)-1]
			}
			if block, ok := last.(*ast.BlockStmt); ok && rangeBodies[block] != nil && len(stack) > 0 && stack[len(stack)-1] == rangeBodies[block] {
				stack = stack[:len(stack)-1]
			}
			if len(loops) > 0 && loops[len(loops)-1] == last {
				loops = loops[:len(loops)-1]
			}
			return true
		}
		nodes = append(nodes, n)
		switch n.(type) {
		case *ast.ForStmt, *ast.RangeStmt:
			loops = append(loops, n)
		}
		switch node := n.(type) {
		case *ast.FuncLit:
			stack = append(stack, node)
			params(node.Type.Params, node, node.Body)
		case *ast.BlockStmt:
			if rs := rangeBodies[node]; rs != nil {
				stack = append(stack, rs)
			}
			b.markListDecls(node.List, vars, candidate, top())
		case *ast.CaseClause:
			b.markListDecls(node.Body, vars, candidate, top())
		case *ast.CommClause:
			b.markListDecls(node.Body, vars, candidate, top())
			if node.Comm != nil {
				commAssigns[node.Comm] = true
			}
		case *ast.AssignStmt:
			for _, lhs := range node.Lhs {
				ident, ok := ast.Unparen(lhs).(*ast.Ident)
				if !ok {
					continue
				}
				v, ok := info.Uses[ident].(*types.Var)
				if !ok {
					continue
				}
				cv := vars[v]
				if cv == nil {
					continue
				}
				if node.Tok == token.DEFINE || commAssigns[node] {
					cv.rejected = true
					continue
				}
				cv.assignments = append(cv.assignments, capturedVarAssignment{pos: node.Pos(), lit: top(), loops: append([]ast.Node(nil), loops...)})
			}
		case *ast.RangeStmt:
			if _, ok := rangeFuncYieldSignature(node.X); ok && node.Body != nil {
				// The body is the yield function, and an assignment
				// to the range variables is made in it.
				rangeBodies[node.Body] = node
				if node.Tok != token.ASSIGN {
					return true
				}
				for _, e := range []ast.Expr{node.Key, node.Value} {
					ident, ok := e.(*ast.Ident)
					if !ok {
						continue
					}
					v, _ := info.Uses[ident].(*types.Var)
					if cv := vars[v]; cv != nil {
						cv.assignments = append(cv.assignments, capturedVarAssignment{pos: node.Pos(), lit: node, loops: append([]ast.Node(nil), loops...)})
						if !cv.firstCapture.IsValid() || node.Pos() < cv.firstCapture {
							cv.firstCapture = node.Pos()
						}
					}
				}
				return true
			}
			for _, e := range []ast.Expr{node.Key, node.Value} {
				if ident, ok := e.(*ast.Ident); ok {
					reject(ident)
				}
			}
		case *ast.UnaryExpr:
			if node.Op == token.AND {
				if ident, ok := ast.Unparen(node.X).(*ast.Ident); ok {
					reject(ident)
				}
			}
		case *ast.Ident:
			v, ok := info.Uses[node].(*types.Var)
			if !ok {
				return true
			}
			cv := vars[v]
			if cv == nil {
				return true
			}
			cv.uses = append(cv.uses, node)
			// The closure that captures the variable is the outermost
			// literal inside the one that declares it.
			depth := -1
			for i, lit := range stack {
				if lit == cv.declLit {
					depth = i
				}
			}
			if depth+1 < len(stack) {
				pos := stack[depth+1].Pos()
				if !cv.firstCapture.IsValid() || pos < cv.firstCapture {
					cv.firstCapture = pos
				}
				if cv.captureLoops == nil {
					cv.captureLoops = map[ast.Node]bool{}
				}
				for _, loop := range loops {
					if loop.Pos() > cv.obj.Pos() {
						cv.captureLoops[loop] = true
					}
				}
			}
		}
		return true
	})

	var boxed []*capturedVar
	for _, cv := range vars {
		if cv.rejected || !cv.firstCapture.IsValid() || (cv.body == nil && !cv.inList) {
			continue
		}
		rebound := false
		for _, a := range cv.assignments {
			if a.lit != cv.declLit || a.pos > cv.firstCapture {
				rebound = true
				break
			}
			// In a loop that also makes the closure, the assignment of
			// the next iteration comes after the closure of this one.
			for _, loop := range a.loops {
				if cv.captureLoops[loop] {
					rebound = true
				}
			}
		}
		if rebound {
			boxed = append(boxed, cv)
		}
	}
	if len(boxed) == 0 {
		return
	}
	// The order of map iteration must not decide the names.
	sortCapturedVars(boxed)
	replace := map[*ast.Ident]ast.Expr{}
	declaredBy := map[*types.Var]*capturedVar{}
	for _, cv := range boxed {
		b.next++
		field := types.NewField(cv.obj.Pos(), b.typeInfo.pkg, "v", cv.obj.Type(), false)
		cell := types.NewVar(cv.obj.Pos(), b.typeInfo.pkg, "__cell_"+cv.obj.Name()+"_"+strconv.Itoa(b.next), types.NewStruct([]*types.Var{field}, nil))
		scope := cv.obj.Parent()
		if scope == nil || scope.Insert(cell) != nil {
			continue
		}
		cv.cell = cell
		declaredBy[cv.obj] = cv
		for _, use := range cv.uses {
			sel := &ast.SelectorExpr{
				X:   &ast.Ident{NamePos: use.Pos(), Name: cell.Name()},
				Sel: &ast.Ident{NamePos: use.Pos(), Name: "v"},
			}
			if err := types.CheckExpr(b.fileSet, b.typeInfo.pkg, use.Pos(), sel, info); err != nil {
				continue
			}
			replace[use] = sel
		}
	}
	astutil.Apply(body, nil, func(c *astutil.Cursor) bool {
		if ident, ok := c.Node().(*ast.Ident); ok {
			if sel, ok := replace[ident]; ok {
				c.Replace(sel)
			}
			return true
		}
		stmt, ok := c.Node().(ast.Stmt)
		if !ok || c.Index() < 0 {
			return true
		}
		for _, v := range listStmtDefinedVars(stmt, info) {
			if cv := declaredBy[v]; cv != nil && cv.body == nil {
				c.InsertAfter(b.cellDecl(cv))
			}
		}
		return true
	})
	// A parameter's cell is declared first in the body of its function.
	var paramCells = map[*ast.BlockStmt][]ast.Stmt{}
	for _, cv := range boxed {
		if cv.cell != nil && cv.body != nil {
			paramCells[cv.body] = append(paramCells[cv.body], b.cellDecl(cv))
		}
	}
	for paramBody, decls := range paramCells {
		paramBody.List = append(decls, paramBody.List...)
	}
}

// markListDecls marks the variables a statement of list declares, so their
// cell can be declared after it.
func (b *capturedVarBoxer) markListDecls(list []ast.Stmt, vars map[*types.Var]*capturedVar, candidate func(*ast.Ident, ast.Node, *ast.BlockStmt), lit ast.Node) {
	info := b.typeInfo.info
	for _, stmt := range list {
		for _, ident := range listStmtDefinedIdents(stmt) {
			candidate(ident, lit, nil)
			if v, ok := info.Defs[ident].(*types.Var); ok {
				if cv := vars[v]; cv != nil {
					cv.inList = true
				}
			}
		}
	}
}

// listStmtDefinedIdents returns the names a var declaration or a short
// variable declaration defines.
func listStmtDefinedIdents(stmt ast.Stmt) []*ast.Ident {
	var idents []*ast.Ident
	switch s := stmt.(type) {
	case *ast.DeclStmt:
		gen, ok := s.Decl.(*ast.GenDecl)
		if !ok || gen.Tok != token.VAR {
			return nil
		}
		for _, spec := range gen.Specs {
			if vs, ok := spec.(*ast.ValueSpec); ok {
				idents = append(idents, vs.Names...)
			}
		}
	case *ast.AssignStmt:
		if s.Tok != token.DEFINE {
			return nil
		}
		for _, lhs := range s.Lhs {
			if ident, ok := lhs.(*ast.Ident); ok {
				idents = append(idents, ident)
			}
		}
	}
	return idents
}

func listStmtDefinedVars(stmt ast.Stmt, info *types.Info) []*types.Var {
	var vars []*types.Var
	for _, ident := range listStmtDefinedIdents(stmt) {
		if v, ok := info.Defs[ident].(*types.Var); ok {
			vars = append(vars, v)
		}
	}
	return vars
}

// cellDecl is __cell_x_N := struct{ v T }{x}.
func (b *capturedVarBoxer) cellDecl(cv *capturedVar) ast.Stmt {
	pos := cv.obj.Pos()
	r := &rangeFuncRewrite{typeInfo: b.typeInfo, pos: pos}
	lit := &ast.CompositeLit{Type: r.typeExpr(cv.cell.Type()), Lbrace: pos, Elts: []ast.Expr{r.use(cv.obj)}, Rbrace: pos}
	b.typeInfo.info.Types[lit] = types.TypeAndValue{Type: cv.cell.Type()}
	name := &ast.Ident{NamePos: pos, Name: cv.cell.Name()}
	b.typeInfo.info.Defs[name] = cv.cell
	return &ast.AssignStmt{Lhs: []ast.Expr{name}, TokPos: pos, Tok: token.DEFINE, Rhs: []ast.Expr{lit}}
}

func sortCapturedVars(vars []*capturedVar) {
	for i := 1; i < len(vars); i++ {
		for j := i; j > 0 && vars[j].obj.Pos() < vars[j-1].obj.Pos(); j-- {
			vars[j], vars[j-1] = vars[j-1], vars[j]
		}
	}
}
