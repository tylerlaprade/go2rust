package main

import (
	"go/ast"
	"go/types"
	"strconv"
	"strings"
)

// Arguments that read the receiver of a method call are evaluated before
// the receiver is locked.
//
// A pointer-receiver call c.Add(c.n) is written as
//
//	(*c.borrow_mut().as_mut().unwrap()).add(<read of c.n>)
//
// Rust takes the receiver's borrow first and evaluates the arguments while
// holding it, so the read of c.n panics with "already mutably borrowed". In
// a program with goroutines every access locks, and the read deadlocks even
// for a value receiver. Go evaluates the receiver operand and then the
// arguments, and taking c's address has no effect, so the arguments are
// evaluated first into a tuple and the call takes them from it:
//
//	{ let __method_args = (<read of c.n>,); (*c.borrow_mut()...).add(__method_args.0) }

// methodCallArgsReadReceiver reports whether call is a method call on a
// local variable whose arguments read that variable, and how many arguments
// the Rust method takes.
func methodCallArgsReadReceiver(call *ast.CallExpr) (*ast.SelectorExpr, int, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || len(call.Args) == 0 {
		return nil, 0, false
	}
	sel, ok := unwrapParens(call.Fun).(*ast.SelectorExpr)
	if !ok {
		return nil, 0, false
	}
	selection := typeInfo.info.Selections[sel]
	if selection == nil || selection.Kind() != types.MethodVal {
		return nil, 0, false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok || fn.Pkg() == nil || isStdlibPackage(fn.Pkg().Path()) {
		return nil, 0, false
	}
	// A method of an interface has the same parameters, but the call goes
	// through a trait object whose borrow is taken the same way.
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Params() == nil {
		return nil, 0, false
	}
	root := methodReceiverRootIdent(sel.X)
	if root == nil || isCurrentReceiverIdent(root) || isPackageGlobalIdent(root) {
		return nil, 0, false
	}
	rootVar, ok := typeInfo.info.Uses[root].(*types.Var)
	if !ok || rootVar.Parent() == nil || rootVar.Parent() == rootVar.Pkg().Scope() {
		return nil, 0, false
	}
	if !methodCallNeedsMutableReceiver(sel) && !NeedsConcurrentWrapper() {
		return nil, 0, false
	}
	if !expressionsUseObject(call.Args, rootVar) {
		return nil, 0, false
	}
	return sel, sig.Params().Len(), true
}

// methodReceiverRootIdent returns the variable a receiver operand is read
// out of, c in c, c.inner, c.items[i] or (*c), or nil when the operand calls
// or receives anything, whose order against the arguments would then show.
func methodReceiverRootIdent(expr ast.Expr) *ast.Ident {
	for {
		switch e := expr.(type) {
		case *ast.Ident:
			return e
		case *ast.ParenExpr:
			expr = e.X
		case *ast.StarExpr:
			expr = e.X
		case *ast.SelectorExpr:
			expr = e.X
		case *ast.IndexExpr:
			if expressionCallsOrReceives(e.Index) {
				return nil
			}
			expr = e.X
		default:
			return nil
		}
	}
}

func expressionsUseObject(exprs []ast.Expr, obj types.Object) bool {
	typeInfo := GetTypeInfo()
	found := false
	for _, expr := range exprs {
		ast.Inspect(expr, func(node ast.Node) bool {
			if found {
				return false
			}
			if ident, ok := node.(*ast.Ident); ok && typeInfo.info.Uses[ident] == obj {
				found = true
				return false
			}
			return true
		})
		if found {
			return true
		}
	}
	return false
}

// hoistMethodCallArgs rewrites the Rust text of a method call so that its
// arguments are evaluated before its receiver. It finds the call of method
// at the outermost nesting level and reports false when the text has a
// shape it does not know: a prelude that binds names the arguments might
// use, or no such call.
func hoistMethodCallArgs(text string, method string, arity int) (string, bool) {
	if arity <= 0 {
		return "", false
	}
	needle := "." + method + "("
	open, close, ok := outermostRustCallParens(text, needle)
	if !ok {
		return "", false
	}
	prefix := text[:open-len(needle)+1]
	if strings.Contains(prefix, "let ") {
		return "", false
	}
	// Only a receiver that is locked, or borrowed for writing, conflicts
	// with the arguments' reads.
	if !strings.Contains(prefix, ".borrow_mut()") && !strings.Contains(prefix, ".lock()") {
		return "", false
	}
	args := strings.TrimRight(text[open+1:close], " \t\n")
	args = strings.TrimSuffix(args, ",")
	if strings.TrimSpace(args) == "" {
		return "", false
	}
	var out strings.Builder
	out.WriteString("{ let __method_args = (")
	out.WriteString(args)
	out.WriteString(",); ")
	out.WriteString(text[:open+1])
	for i := 0; i < arity; i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString("__method_args.")
		out.WriteString(strconv.Itoa(i))
	}
	out.WriteString(text[close:])
	out.WriteString(" }")
	return out.String(), true
}

// outermostRustCallParens finds needle (".add(") in Rust text, outside
// string and char literals, at the least nesting depth, the first one at
// that depth, and returns the offsets of its "(" and of the ")" that closes
// it.
func outermostRustCallParens(text string, needle string) (int, int, bool) {
	depth := 0
	bestDepth := -1
	bestOpen := -1
	var opens []int
	matchOf := map[int]int{}
	for i := 0; i < len(text); i++ {
		c := text[i]
		switch {
		case c == '"':
			// A raw string, r"..." or r#"..."#, starts with r and hashes.
			hashes := 0
			j := i - 1
			for j >= 0 && text[j] == '#' {
				hashes++
				j--
			}
			raw := j >= 0 && text[j] == 'r' && (j == 0 || !isRustIdentByte(text[j-1]))
			i++
			for i < len(text) {
				if !raw && text[i] == '\\' {
					i += 2
					continue
				}
				if text[i] == '"' && (!raw || strings.HasPrefix(text[i+1:], strings.Repeat("#", hashes))) {
					i += hashes
					break
				}
				i++
			}
		case c == '\'':
			// A char literal, 'x' or '\n', or a lifetime, 'static.
			if i+1 < len(text) && text[i+1] == '\\' {
				end := strings.IndexByte(text[i+2:], '\'')
				if end < 0 {
					return 0, 0, false
				}
				i += 2 + end
				continue
			}
			if end := strings.IndexByte(text[i+1:], '\''); end > 0 && end <= 4 {
				i += 1 + end
			}
		case c == '(' || c == '[' || c == '{':
			if c == '(' && strings.HasSuffix(text[:i+1], needle) && (bestDepth < 0 || depth < bestDepth) {
				bestDepth = depth
				bestOpen = i
			}
			opens = append(opens, i)
			depth++
		case c == ')' || c == ']' || c == '}':
			if len(opens) == 0 {
				return 0, 0, false
			}
			matchOf[opens[len(opens)-1]] = i
			opens = opens[:len(opens)-1]
			depth--
		}
	}
	if bestOpen < 0 || len(opens) != 0 {
		return 0, 0, false
	}
	close, ok := matchOf[bestOpen]
	return bestOpen, close, ok
}

func isRustIdentByte(c byte) bool {
	return c == '_' || c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c >= '0' && c <= '9'
}
