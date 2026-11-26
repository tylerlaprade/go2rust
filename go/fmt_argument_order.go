package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

// gc runs every call among a print's operands before it reads the plain
// operands and formats them: fmt.Println(x, f()) prints the x that f left,
// and fmt.Println(ins, slices.Delete(ins, 0, 2)) prints ins after Delete
// moved its elements (DEV-219). The Rust formatted each operand as it came
// to it. When an operand with a call follows an operand that reads a
// variable, writeFmtCallWithCallOperandsFirst binds each operand that has
// a call to a temporary first, in order, and then writes the print with
// the temporaries in their place. The plain operands are read as the print
// formats them, after every call, and a call's result shows what later
// calls did to the storage it shares.

// fmtFirstOperand returns the index of the first operand that a fmt print
// function formats, or -1 for a call that is not one.
func fmtFirstOperand(call *ast.CallExpr) int {
	key, ok := stdlibCallKey(call.Fun)
	if !ok {
		return -1
	}
	switch key {
	case "fmt.Println", "fmt.Print", "fmt.Sprint", "fmt.Sprintln":
		return 0
	case "fmt.Printf", "fmt.Sprintf", "fmt.Errorf", "fmt.Fprintln", "fmt.Fprint":
		return 1
	case "fmt.Fprintf":
		return 2
	}
	return -1
}

var fmtOperandTempCount int

// writeFmtCallWithCallOperandsFirst writes a fmt print call whose operands
// gc would read out of order as
// { let __go_print_arg_N = <operand>; ...; let __go_print_value = <print>; __go_print_value }.
func writeFmtCallWithCallOperandsFirst(out *strings.Builder, call *ast.CallExpr, handler StdlibHandler) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || typeInfo.pkg == nil || call.Ellipsis.IsValid() {
		return false
	}
	first := fmtFirstOperand(call)
	if first < 0 || first > len(call.Args) {
		return false
	}
	var staged []int
	var plain []ast.Expr
	var withCalls []ast.Expr
	for i, arg := range call.Args[:first] {
		// A call in the format or the writer runs before the operands'
		// calls, so it is staged first; one whose value cannot be staged
		// would run after the staged ones.
		if binaryOperandHasCall(arg, true) {
			if !fmtOperandCanBeStaged(arg) {
				return false
			}
			staged = append(staged, i)
		}
	}
	// A call in the format or the writer is always staged. The print
	// evaluates the format and the writer after its operands, and the
	// lowering of a print to the terminal does not evaluate a writer at
	// all; a temporary of type *os.File is written to as the file it is.
	leadingCall := len(staged) > 0
	readBeforeCall := false
	for i := first; i < len(call.Args); i++ {
		arg := call.Args[i]
		// gc evaluates a builtin call such as len(s), and x && b, in
		// order with the calls, so such an operand is staged in its place
		// too; a plain operand is read after them all.
		if !binaryOperandHasCall(arg, true) {
			if !isConstantExpression(arg) {
				plain = append(plain, arg)
			}
			continue
		}
		if !fmtOperandCanBeStaged(arg) {
			// An operand that keeps its place would run after the
			// staged ones.
			return false
		}
		if len(plain) > 0 && binaryOperandHasCall(arg, false) {
			readBeforeCall = true
		}
		staged = append(staged, i)
		withCalls = append(withCalls, arg)
	}
	if !leadingCall && (!readBeforeCall || !fmtPlainOperandsMayChange(plain, withCalls)) {
		return false
	}

	used := map[string]bool{}
	ast.Inspect(call, func(node ast.Node) bool {
		if ident, ok := node.(*ast.Ident); ok {
			used[ident.Name] = true
			used[RustLocalIdent(ident.Name)] = true
		}
		return true
	})
	vt := GetVarTable()
	if vt != nil {
		vt.PushScope()
		defer vt.PopScope()
	}
	original := append([]ast.Expr(nil), call.Args...)
	defer copy(call.Args, original)

	out.WriteString("{ ")
	for _, i := range staged {
		arg := call.Args[i]
		argType := typeInfo.GetType(arg)
		fmtOperandTempCount++
		name := fmt.Sprintf("__go_print_arg_%d", fmtOperandTempCount)
		for used[name] || RustLocalIdent(name) != name {
			fmtOperandTempCount++
			name = fmt.Sprintf("__go_print_arg_%d", fmtOperandTempCount)
		}
		out.WriteString("let ")
		out.WriteString(name)
		out.WriteString(" = ")
		// NeedsUnwrapping reports whether the operand's Rust is a cell.
		if typeInfo.NeedsUnwrapping(arg) {
			TranspileExpression(out, arg)
		} else {
			WriteWrapperPrefix(out)
			if !writeLenCapCallArgumentForExpectedType(out, unwrapParens(arg), argType) {
				TranspileExpression(out, arg)
			}
			WriteWrapperSuffix(out)
		}
		out.WriteString("; ")

		use := &ast.Ident{NamePos: arg.Pos(), Name: name}
		typeInfo.info.Uses[use] = types.NewVar(arg.Pos(), typeInfo.pkg, name, argType)
		typeInfo.info.Types[use] = types.TypeAndValue{Type: argType}
		if vt != nil {
			vt.Register(name, &VarInfo{WrapLevel: WrapFull, RustType: goTypesTypeToRust(argType), Source: SourceLocal, GoType: argType})
		}
		call.Args[i] = use
	}
	// The print goes into a let too: the guards a block's tail expression
	// takes would outlive the temporaries they borrow.
	out.WriteString("let __go_print_value = ")
	handler(out, call)
	out.WriteString("; __go_print_value }")
	return true
}

// fmtPlainOperandsMayChange reports whether a call among withCalls can
// change what one of the plain operands reads: storage any call can reach
// (an element, field or pointer target of a package variable, or of a local
// that a closure captures or whose address is taken), or a variable whose
// slice, map or pointer, or whose address, one of the calls is given.
func fmtPlainOperandsMayChange(plain []ast.Expr, withCalls []ast.Expr) bool {
	typeInfo := GetTypeInfo()
	passed := map[*types.Var]bool{}
	for _, operand := range withCalls {
		ast.Inspect(operand, func(node ast.Node) bool {
			call, ok := node.(*ast.CallExpr)
			if !ok {
				return true
			}
			for i, arg := range call.Args {
				if v := fmtPassedVar(typeInfo, arg); v != nil && !fmtCallOnlyReadsElements(call, i) {
					passed[v] = true
				}
			}
			if sel, ok := unwrapParens(call.Fun).(*ast.SelectorExpr); ok {
				if selection, ok := typeInfo.info.Selections[sel]; ok && selection.Kind() == types.MethodVal {
					if v := fmtAccessRoot(typeInfo, sel.X); v != nil && (selectionTakesReceiverAddress(selection) || fmtTypeShares(typeInfo.GetType(sel.X))) {
						passed[v] = true
					}
				}
			}
			return true
		})
	}
	for _, operand := range plain {
		changes := false
		ast.Inspect(operand, func(node ast.Node) bool {
			if changes {
				return false
			}
			switch n := node.(type) {
			case *ast.Ident:
				// Every variable the operand reads, the root of an
				// access path included.
				if v, ok := typeInfo.info.Uses[n].(*types.Var); ok && !v.IsField() && (varReachableByCalls(typeInfo, v) || passed[v]) {
					changes = true
				}
			case *ast.IndexExpr, *ast.StarExpr, *ast.SelectorExpr:
				if binaryOperandReadsStorageNode(typeInfo, n.(ast.Expr)) && (fmtAccessRoot(typeInfo, n.(ast.Expr)) == nil || fmtAccessReadsSharedStorage(typeInfo, n.(ast.Expr))) {
					// A read this cannot place, or one of storage that
					// another slice, map or pointer can share: the
					// variable it goes through is not the only way to it.
					changes = true
				}
			}
			return !changes
		})
		if changes {
			return true
		}
	}
	return false
}

// fmtAccessRoot returns the variable that expr reads through fields,
// elements and pointer targets, when expr is only such an access.
func fmtAccessRoot(typeInfo *TypeInfo, expr ast.Expr) *types.Var {
	for {
		switch e := unwrapParens(expr).(type) {
		case *ast.Ident:
			v, _ := typeInfo.info.Uses[e].(*types.Var)
			return v
		case *ast.SelectorExpr:
			selection, ok := typeInfo.info.Selections[e]
			if !ok {
				// A package-qualified variable.
				v, _ := typeInfo.info.Uses[e.Sel].(*types.Var)
				return v
			}
			if selection.Kind() != types.FieldVal {
				return nil
			}
			expr = e.X
		case *ast.IndexExpr:
			expr = e.X
		case *ast.StarExpr:
			expr = e.X
		default:
			return nil
		}
	}
}

// fmtAccessReadsSharedStorage reports whether the access path expr reads
// through a slice, map or pointer: an element of a slice or map, or the
// target of a pointer. Such storage can have other names, alias := s among
// them, so a call can change it without being given the variable the path
// starts at.
func fmtAccessReadsSharedStorage(typeInfo *TypeInfo, expr ast.Expr) bool {
	for {
		switch e := unwrapParens(expr).(type) {
		case *ast.SelectorExpr:
			selection, ok := typeInfo.info.Selections[e]
			if !ok {
				return false
			}
			if selection.Indirect() || fmtTypeShares(typeInfo.GetType(e.X)) {
				return true
			}
			expr = e.X
		case *ast.IndexExpr:
			if fmtTypeShares(typeInfo.GetType(e.X)) {
				return true
			}
			expr = e.X
		case *ast.StarExpr:
			return true
		default:
			return false
		}
	}
}

// fmtCallOnlyReadsElements reports whether the function of the program
// that call calls only reads the elements of its parameter index: the
// parameter is a slice or array of numbers, strings or bools, and the body
// uses it only to read an element, in len or cap, and as what a range loop
// ranges over. Such a function cannot change what the argument shares.
func fmtCallOnlyReadsElements(call *ast.CallExpr, index int) bool {
	fn := sourceFunctionObjectForCall(call)
	if fn == nil || sourceFunctionDeclsByFunc == nil {
		return false
	}
	declInfo, ok := sourceFunctionDeclsByFunc[fn]
	if !ok || declInfo.decl == nil || declInfo.decl.Body == nil || declInfo.info == nil {
		return false
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Params() == nil || index >= sig.Params().Len() || (sig.Variadic() && index == sig.Params().Len()-1) {
		return false
	}
	param := sig.Params().At(index)
	var elem types.Type
	switch t := types.Unalias(param.Type()).Underlying().(type) {
	case *types.Slice:
		elem = t.Elem()
	case *types.Array:
		elem = t.Elem()
	default:
		return false
	}
	if basic, ok := types.Unalias(elem).(*types.Basic); !ok || basic.Info()&(types.IsNumeric|types.IsString|types.IsBoolean) == 0 {
		return false
	}
	info := declInfo.info
	written := map[ast.Expr]bool{}
	ast.Inspect(declInfo.decl.Body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.AssignStmt:
			for _, lhs := range n.Lhs {
				written[unwrapParens(lhs)] = true
			}
		case *ast.IncDecStmt:
			written[unwrapParens(n.X)] = true
		case *ast.RangeStmt:
			// for s[0] = range ... assigns each value to its target.
			if n.Tok == token.ASSIGN {
				if n.Key != nil {
					written[unwrapParens(n.Key)] = true
				}
				if n.Value != nil {
					written[unwrapParens(n.Value)] = true
				}
			}
		case *ast.UnaryExpr:
			if n.Op == token.AND {
				written[unwrapParens(n.X)] = true
			}
		}
		return true
	})
	isParam := func(expr ast.Expr) bool {
		ident, ok := unwrapParens(expr).(*ast.Ident)
		return ok && info.Uses[ident] == param
	}
	read := map[ast.Node]bool{}
	ast.Inspect(declInfo.decl.Body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.IndexExpr:
			if isParam(n.X) && !written[n] {
				read[unwrapParens(n.X)] = true
			}
		case *ast.CallExpr:
			if ident, ok := unwrapParens(n.Fun).(*ast.Ident); ok && len(n.Args) == 1 && isParam(n.Args[0]) {
				if builtin, ok := info.Uses[ident].(*types.Builtin); ok && (builtin.Name() == "len" || builtin.Name() == "cap") {
					read[unwrapParens(n.Args[0])] = true
				}
			}
		case *ast.RangeStmt:
			if isParam(n.X) {
				read[unwrapParens(n.X)] = true
			}
		}
		return true
	})
	onlyRead := true
	ast.Inspect(declInfo.decl.Body, func(node ast.Node) bool {
		if ident, ok := node.(*ast.Ident); ok && info.Uses[ident] == param && !read[ident] {
			onlyRead = false
		}
		return onlyRead
	})
	return onlyRead
}

// fmtPassedVar returns the variable whose storage a call argument shares:
// the variable of &x.f, or of x, x.f or x[i:j] when that has a slice, map
// or pointer type.
func fmtPassedVar(typeInfo *TypeInfo, arg ast.Expr) *types.Var {
	arg = unwrapParens(arg)
	if unary, ok := arg.(*ast.UnaryExpr); ok && unary.Op == token.AND {
		return fmtAccessRoot(typeInfo, unary.X)
	}
	if slice, ok := arg.(*ast.SliceExpr); ok {
		return fmtAccessRoot(typeInfo, slice.X)
	}
	if fmtTypeShares(typeInfo.GetType(arg)) {
		return fmtAccessRoot(typeInfo, arg)
	}
	return nil
}

// fmtTypeShares reports whether a value of typ shares storage with what it
// was copied from.
func fmtTypeShares(typ types.Type) bool {
	if typ == nil {
		return false
	}
	// A channel or func prints as its address, and a call cannot change
	// which value an interface variable holds.
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Slice, *types.Map, *types.Pointer:
		return true
	}
	return false
}

// fmtOperandCanBeStaged reports whether an operand's value can go into a
// temporary that a variable of its type is written as: a number, string,
// bool, slice, map, pointer, struct or array. An interface, error, func,
// channel or multi-value operand keeps its own lowering.
func fmtOperandCanBeStaged(expr ast.Expr) bool {
	typ := GetTypeInfo().GetType(expr)
	if typ == nil {
		return false
	}
	switch t := types.Unalias(typ).Underlying().(type) {
	case *types.Basic:
		return t.Info()&types.IsUntyped == 0 && t.Info()&types.IsComplex == 0 && t.Kind() != types.UnsafePointer
	case *types.Slice, *types.Map, *types.Pointer, *types.Struct, *types.Array:
		return true
	}
	return false
}
