package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"go/types"
)

// A select case or a channel range that assigns, case x = <-ch or
// for x = range ch, writes into x, which can be any variable, field or
// element. The lowerings of both bind the received value with let, which
// shadowed x and left it unchanged (DEV-232). They are written as the
// declaring forms with fresh variables, followed by assignments of those to
// the targets:
//
//	case __go_recv_value_1, __go_recv_ok_1 := <-ch:
//		x, ok = __go_recv_value_1, __go_recv_ok_1 (one assignment each)
//		<body>
//
// The assignment lowering then stores the value as it stores any other.

var receiveAssignmentTempCount int

// receiveAssignmentTemp makes a variable of type typ for a rewritten
// receive: its defining identifier and an identifier that uses it. Its name
// is none of the names in used, which hold the identifiers of the statement
// it is written in, and Rust writes it as it is.
func receiveAssignmentTemp(prefix string, typ types.Type, pos token.Pos, used map[string]bool) (*ast.Ident, *ast.Ident) {
	typeInfo := GetTypeInfo()
	receiveAssignmentTempCount++
	name := fmt.Sprintf("%s%d", prefix, receiveAssignmentTempCount)
	for used[name] || RustLocalIdent(name) != name {
		receiveAssignmentTempCount++
		name = fmt.Sprintf("%s%d", prefix, receiveAssignmentTempCount)
	}
	v := types.NewVar(pos, typeInfo.pkg, name, typ)
	def := &ast.Ident{NamePos: pos, Name: name}
	use := &ast.Ident{NamePos: pos, Name: name}
	typeInfo.info.Defs[def] = v
	typeInfo.info.Uses[use] = v
	typeInfo.info.Types[use] = types.TypeAndValue{Type: typ}
	return def, use
}

// receiveAssignmentTargets returns the assignment that stores the fresh
// variables uses into the targets lhs, skipping blank targets. It is one
// assignment, so case i, flags[i] = <-ch evaluates flags[i] with the old i,
// as Go evaluates every target before it stores into any.
func receiveAssignmentTargets(lhs []ast.Expr, uses []*ast.Ident) []ast.Stmt {
	var targets, values []ast.Expr
	for i, target := range lhs {
		if i >= len(uses) || uses[i] == nil {
			continue
		}
		targets = append(targets, target)
		values = append(values, uses[i])
	}
	if len(targets) == 0 {
		return nil
	}
	return []ast.Stmt{&ast.AssignStmt{Lhs: targets, TokPos: targets[0].End(), Tok: token.ASSIGN, Rhs: values}}
}

// identifierNames returns the names, in Go and in Rust, of the identifiers
// in node.
func identifierNames(node ast.Node) map[string]bool {
	used := map[string]bool{}
	ast.Inspect(node, func(n ast.Node) bool {
		if ident, ok := n.(*ast.Ident); ok {
			used[ident.Name] = true
			used[RustLocalIdent(ident.Name)] = true
		}
		return true
	})
	return used
}

func isBlankExpr(expr ast.Expr) bool {
	ident, ok := expr.(*ast.Ident)
	return ok && ident.Name == "_"
}

// rewriteSelectReceiveAssignment rewrites case x, ok = <-ch as the declaring
// form and returns a function that restores the clause. needsTypeInfo is
// true for an assigning receive that cannot be written without go/types;
// the caller writes a loud unimplemented! for it, since the declaring
// lowering would leave the target unchanged.
func rewriteSelectReceiveAssignment(cc *ast.CommClause) (restore func(), needsTypeInfo bool) {
	comm, ok := cc.Comm.(*ast.AssignStmt)
	if !ok || comm.Tok != token.ASSIGN || len(comm.Rhs) != 1 || len(comm.Lhs) == 0 || len(comm.Lhs) > 2 {
		return nil, false
	}
	unary, ok := unwrapParens(comm.Rhs[0]).(*ast.UnaryExpr)
	if !ok || unary.Op != token.ARROW {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || typeInfo.pkg == nil {
		return nil, true
	}
	chanType, ok := types.Unalias(typeInfo.GetType(unary.X)).Underlying().(*types.Chan)
	if !ok {
		return nil, true
	}
	used := identifierNames(cc)
	newLhs := make([]ast.Expr, len(comm.Lhs))
	uses := make([]*ast.Ident, len(comm.Lhs))
	for i, target := range comm.Lhs {
		if isBlankExpr(target) {
			newLhs[i] = target
			continue
		}
		typ := chanType.Elem()
		prefix := "__go_recv_value_"
		if i == 1 {
			typ, prefix = types.Typ[types.Bool], "__go_recv_ok_"
		}
		def, use := receiveAssignmentTemp(prefix, typ, target.Pos(), used)
		newLhs[i] = def
		uses[i] = use
	}
	rewritten := &ast.AssignStmt{Lhs: newLhs, TokPos: comm.TokPos, Tok: token.DEFINE, Rhs: comm.Rhs}
	originalBody := cc.Body
	cc.Comm = rewritten
	cc.Body = append(receiveAssignmentTargets(comm.Lhs, uses), originalBody...)
	return func() {
		cc.Comm = comm
		cc.Body = originalBody
	}, false
}

// rewriteChannelRangeAssignment rewrites for x = range ch as the declaring
// form and returns a function that restores the statement, or reports that
// it needs go/types, as rewriteSelectReceiveAssignment does.
func rewriteChannelRangeAssignment(s *ast.RangeStmt) (restore func(), needsTypeInfo bool) {
	if s.Tok != token.ASSIGN || s.Key == nil || s.Value != nil || isBlankExpr(s.Key) {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || typeInfo.pkg == nil {
		return nil, true
	}
	chanType, ok := types.Unalias(typeInfo.GetType(s.X)).Underlying().(*types.Chan)
	if !ok {
		// Not a range over a channel.
		return nil, false
	}
	key := s.Key
	def, use := receiveAssignmentTemp("__go_recv_value_", chanType.Elem(), key.Pos(), identifierNames(s))
	originalBody := s.Body.List
	s.Key = def
	s.Tok = token.DEFINE
	s.Body.List = append(receiveAssignmentTargets([]ast.Expr{key}, []*ast.Ident{use}), originalBody...)
	return func() {
		s.Key = key
		s.Tok = token.ASSIGN
		s.Body.List = originalBody
	}, false
}
