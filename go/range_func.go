package main

import (
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
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
	results     []*types.Var
	// namedResults is true when the enclosing function names its results,
	// so a bare return returns them.
	namedResults bool
	hasResults   bool
	// next numbers the branch to a loop around this one that the body took,
	// from 1, and branches holds those branches.
	next     *types.Var
	branches []*ast.BranchStmt
	suffix   string
}

func (r *rangeFuncRewrite) typeExpr(t types.Type) ast.Expr {
	expr := &ast.Ident{NamePos: r.pos, Name: types.TypeString(t, func(p *types.Package) string {
		if r.typeInfo.pkg != nil && p == r.typeInfo.pkg {
			return ""
		}
		return p.Name()
	})}
	r.typeInfo.info.Types[expr] = types.TypeAndValue{Type: t}
	return expr
}

func (r *rangeFuncRewrite) use(v *types.Var) *ast.Ident {
	ident := &ast.Ident{NamePos: r.pos, Name: v.Name()}
	r.typeInfo.info.Uses[ident] = v
	r.typeInfo.info.Types[ident] = types.TypeAndValue{Type: v.Type()}
	return ident
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

// rangeFuncSplices holds the blocks that stand for the statements a nested
// range over a function is, which the list holding one takes in its place.
var rangeFuncSplices = map[*ast.BlockStmt]bool{}

// rewriteStmt rewrites the statements of the loop body. inLoop and inSwitch
// tell whether the statement is inside a loop, or a switch or select, within
// the body, which an unlabeled break or continue belongs to.
func (r *rangeFuncRewrite) rewriteStmt(stmt ast.Stmt, inLoop bool, inSwitch bool) ast.Stmt {
	switch s := stmt.(type) {
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
		if len(s.Results) > 0 {
			lhs := make([]ast.Expr, len(r.results))
			for i, v := range r.results {
				lhs[i] = r.use(v)
			}
			list = append(list, &ast.AssignStmt{Lhs: lhs, TokPos: s.Pos(), Tok: token.ASSIGN, Rhs: s.Results})
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
	stmts, ok := expandRangeOverFunc(s, label, r.fnType)
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
	yield, ok := rangeFuncYieldSignature(s.X)
	if !ok {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	r := &rangeFuncRewrite{typeInfo: typeInfo, fnType: fnType, pos: s.For, label: label, innerLabels: statementLabels(s.Body)}
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
			if len(field.Names) > 0 {
				r.namedResults = true
				for _, name := range field.Names {
					v, ok := typeInfo.info.Defs[name].(*types.Var)
					if !ok {
						return nil, false
					}
					r.results = append(r.results, v)
				}
				continue
			}
			v, decl := r.newVar("__rf_result_"+r.suffix+"_"+itoaRangeFunc(len(r.results)), resultType)
			r.results = append(r.results, v)
			prelude = append(prelude, decl)
		}
	}

	params := &ast.FieldList{Opening: s.For}
	var bindings []ast.Stmt
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
		switch {
		case bound == nil:
		case s.Tok == token.DEFINE:
			name = bound.(*ast.Ident)
		default:
			// for k, v = range seq assigns each k and v to the variables.
			param := types.NewVar(s.For, typeInfo.pkg, "__rf_param_"+r.suffix+"_"+itoaRangeFunc(i), paramType)
			name = &ast.Ident{NamePos: s.For, Name: param.Name()}
			typeInfo.info.Defs[name] = param
			bindings = append(bindings, r.assign(bound, r.use(param), s.For))
		}
		params.List = append(params.List, &ast.Field{Names: []*ast.Ident{name}, Type: r.typeExpr(paramType)})
	}
	body := r.rewriteStmt(s.Body, false, false).(*ast.BlockStmt)
	body.List = append(append(bindings, body.List...), &ast.ReturnStmt{Return: s.Body.Rbrace, Results: []ast.Expr{r.boolLit(true)}})
	yieldType := &ast.FuncType{Func: s.For, Params: params, Results: &ast.FieldList{List: []*ast.Field{{Type: r.typeExpr(yield.Results().At(0).Type())}}}}
	funcLit := &ast.FuncLit{Type: yieldType, Body: body}
	typeInfo.info.Types[funcLit] = types.TypeAndValue{Type: yield}
	call := &ast.CallExpr{Fun: s.X, Lparen: s.X.End(), Args: []ast.Expr{funcLit}, Rparen: s.End()}
	typeInfo.info.Types[call] = types.TypeAndValue{Type: types.NewTuple()}

	if len(r.branches) > 0 {
		prelude = append(prelude, nextDecl)
	}
	stmts := append(prelude, &ast.ExprStmt{X: call})
	var after []ast.Stmt
	if r.hasResults && !r.namedResults {
		results := make([]ast.Expr, len(r.results))
		for i, v := range r.results {
			results[i] = r.use(v)
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
