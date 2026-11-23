package main

import (
	"fmt"
	"go/ast"
	"strings"
)

// selectHoistedChannels maps a select case's channel operand to the Rust
// local that holds its value for the whole select.
var selectHoistedChannels = map[ast.Expr]string{}

var selectHoistCounter int

// selectCaseChannel returns the channel operand of a select case, or nil
// for default.
func selectCaseChannel(cc *ast.CommClause) ast.Expr {
	switch comm := cc.Comm.(type) {
	case *ast.AssignStmt:
		if len(comm.Rhs) == 1 {
			if unary, ok := comm.Rhs[0].(*ast.UnaryExpr); ok {
				return unary.X
			}
		}
	case *ast.ExprStmt:
		if unary, ok := comm.X.(*ast.UnaryExpr); ok {
			return unary.X
		}
	case *ast.SendStmt:
		return comm.Chan
	}
	return nil
}

// selectChannelOperandNeedsHoist reports whether evaluating a channel
// operand again on each poll would differ from Go, which evaluates it once:
// the operand calls a function, as time.After(d) or ctx.Done() do.
func selectChannelOperandNeedsHoist(expr ast.Expr) bool {
	calls := false
	ast.Inspect(expr, func(n ast.Node) bool {
		if _, ok := n.(*ast.CallExpr); ok {
			calls = true
		}
		if _, ok := n.(*ast.FuncLit); ok {
			return false
		}
		return !calls
	})
	return calls
}

func selectChannelsHoisted(commClauses []*ast.CommClause) bool {
	for _, cc := range commClauses {
		if ch := selectCaseChannel(cc); ch != nil && selectChannelOperandNeedsHoist(ch) {
			return true
		}
	}
	return false
}

// hoistSelectChannelOperands opens a block and binds each channel operand
// that calls a function to a local, in source order as Go evaluates them.
// The caller closes the block after the polling loop when
// selectChannelsHoisted is true.
func hoistSelectChannelOperands(out *strings.Builder, commClauses []*ast.CommClause) func() {
	if !selectChannelsHoisted(commClauses) {
		return func() {}
	}
	out.WriteString("{ ")
	var hoisted []ast.Expr
	for _, cc := range commClauses {
		ch := selectCaseChannel(cc)
		if ch == nil || !selectChannelOperandNeedsHoist(ch) {
			continue
		}
		selectHoistCounter++
		name := fmt.Sprintf("__select_chan_%d", selectHoistCounter)
		out.WriteString("let ")
		out.WriteString(name)
		out.WriteString(" = ")
		writeChannelExpression(out, ch)
		out.WriteString("; ")
		selectHoistedChannels[ch] = name
		hoisted = append(hoisted, ch)
	}
	return func() {
		for _, ch := range hoisted {
			delete(selectHoistedChannels, ch)
		}
	}
}

// writeSelectChannelOperand writes a case's channel: the hoisted local, or
// the operand itself.
func writeSelectChannelOperand(out *strings.Builder, ch ast.Expr) {
	if name, ok := selectHoistedChannels[ch]; ok {
		out.WriteString(name)
		return
	}
	writeChannelExpression(out, ch)
}
