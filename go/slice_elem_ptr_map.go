package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

// A local map[K]*T whose values are pointers to the elements of a slice,
// as in byTitle[items[i].Title] = &items[i], holds GoPtr<T> values: the
// map's own *T values, Rc cells, cannot point into a slice's elements
// (DEV-131). GoPtr reads as the cell does (borrow, borrow_mut), so a read
// such as byTitle["b"].Price += 5 needs nothing new; the map's type, its
// literal and the values stored into it do.

var currentSliceElemPtrMapCandidates map[types.Object]string

// collectSliceElemPtrMapCandidates finds the local maps of pointers whose
// stored values are all slice element pointers (or nil or another such
// pointer), with at least one of them taken from a slice element. A map
// used other than by indexing, len, delete, range or its declaration, for
// instance passed to a function, keeps its type.
func collectSliceElemPtrMapCandidates(body *ast.BlockStmt) map[types.Object]string {
	typeInfo := GetTypeInfo()
	if body == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	type state struct {
		elemRustType string
		valid        bool
		sawSliceAddr bool
		// declared is set by the map's fresh declaration in the body: a
		// parameter, declared elsewhere, keeps its type.
		declared bool
	}
	candidates := map[types.Object]*state{}
	stateFor := func(ident *ast.Ident) *state {
		if ident == nil || ident.Name == "_" {
			return nil
		}
		obj := typeInfo.GetObject(ident)
		if obj == nil {
			return nil
		}
		if _, isVar := obj.(*types.Var); !isVar || obj.Parent() == nil || obj.Parent() == obj.Pkg().Scope() {
			return nil
		}
		mapType, ok := types.Unalias(obj.Type()).(*types.Map)
		if !ok {
			return nil
		}
		ptr, ok := types.Unalias(mapType.Elem()).(*types.Pointer)
		if !ok {
			return nil
		}
		elemType := coreType(ptr.Elem())
		if _, isStruct := types.Unalias(elemType).Underlying().(*types.Struct); !isStruct {
			return nil
		}
		st := candidates[obj]
		if st == nil {
			st = &state{elemRustType: goTypesCollectionElemTypeToRust(elemType), valid: true}
			candidates[obj] = st
		}
		return st
	}
	allowed := map[*ast.Ident]bool{}
	recordValue := func(st *state, value ast.Expr) {
		if st == nil {
			return
		}
		if ident, ok := unwrapParens(value).(*ast.Ident); ok && ident.Name == "nil" {
			return
		}
		ok, sawSliceAddr := isSliceElemPtrAssignmentValue(value)
		if !ok {
			st.valid = false
			return
		}
		if sawSliceAddr {
			st.sawSliceAddr = true
		}
	}
	ast.Inspect(body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			// A closure may capture the map; keep its type.
			ast.Inspect(n.Body, func(inner ast.Node) bool {
				if ident, ok := inner.(*ast.Ident); ok {
					if st := stateFor(ident); st != nil {
						st.valid = false
					}
				}
				return true
			})
			return false
		case *ast.AssignStmt:
			for i, lhs := range n.Lhs {
				if index, ok := unwrapParens(lhs).(*ast.IndexExpr); ok {
					if ident, ok := unwrapParens(index.X).(*ast.Ident); ok {
						if st := stateFor(ident); st != nil {
							allowed[ident] = true
							if rhs := assignmentRHSForLHS(n, i); rhs != nil {
								recordValue(st, rhs)
							} else {
								st.valid = false
							}
						}
					}
					continue
				}
				ident, ok := lhs.(*ast.Ident)
				if !ok {
					continue
				}
				st := stateFor(ident)
				if st == nil {
					continue
				}
				allowed[ident] = true
				rhs := assignmentRHSForLHS(n, i)
				if n.Tok != token.DEFINE || !sliceElemPtrMapInitializerIsFresh(rhs) {
					st.valid = false
					continue
				}
				st.declared = true
				if lit, ok := unwrapParens(rhs).(*ast.CompositeLit); ok {
					for _, elt := range lit.Elts {
						if kv, ok := elt.(*ast.KeyValueExpr); ok {
							recordValue(st, kv.Value)
						}
					}
				}
			}
		case *ast.ValueSpec:
			for i, name := range n.Names {
				st := stateFor(name)
				if st == nil {
					continue
				}
				allowed[name] = true
				if len(n.Names) != 1 {
					st.valid = false
				} else if i < len(n.Values) && !sliceElemPtrMapInitializerIsFresh(n.Values[i]) {
					st.valid = false
				}
				st.declared = true
			}
		case *ast.IndexExpr:
			if ident, ok := unwrapParens(n.X).(*ast.Ident); ok && stateFor(ident) != nil {
				allowed[ident] = true
			}
		case *ast.RangeStmt:
			if ident, ok := unwrapParens(n.X).(*ast.Ident); ok && stateFor(ident) != nil {
				allowed[ident] = true
			}
		case *ast.CallExpr:
			if isBuiltinCallNamed(n, "len") || isBuiltinCallNamed(n, "delete") || isBuiltinCallNamed(n, "clear") {
				if len(n.Args) > 0 {
					if ident, ok := unwrapParens(n.Args[0]).(*ast.Ident); ok && stateFor(ident) != nil {
						allowed[ident] = true
					}
				}
			}
		}
		return true
	})
	// Any other use of the map keeps its type.
	ast.Inspect(body, func(node ast.Node) bool {
		if _, ok := node.(*ast.FuncLit); ok {
			return false
		}
		if ident, ok := node.(*ast.Ident); ok && !allowed[ident] {
			if obj := typeInfo.GetObject(ident); obj != nil {
				if st := candidates[obj]; st != nil {
					if typeInfo.info.Defs[ident] == nil {
						st.valid = false
					}
				}
			}
		}
		return true
	})
	result := map[types.Object]string{}
	for obj, st := range candidates {
		if st.valid && st.sawSliceAddr && st.declared {
			result[obj] = st.elemRustType
		}
	}
	if len(result) == 0 {
		return nil
	}
	return result
}

// sliceElemPtrMapInitializerIsFresh reports whether a map declaration's
// value makes a new map: a literal or make, not another variable's map.
func sliceElemPtrMapInitializerIsFresh(expr ast.Expr) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.CompositeLit:
		return true
	case *ast.CallExpr:
		return isBuiltinCallNamed(e, "make")
	}
	return false
}

func sliceElemPtrMapCandidateForExpr(expr ast.Expr) (string, bool) {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok || currentSliceElemPtrMapCandidates == nil {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", false
	}
	obj := typeInfo.GetObject(ident)
	if obj == nil {
		return "", false
	}
	elemRustType, ok := currentSliceElemPtrMapCandidates[obj]
	return elemRustType, ok
}

// writeSliceElemPtrMapInitializer writes the literal or make that declares
// such a map: a BTreeMap of GoPtr values.
func writeSliceElemPtrMapInitializer(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	elemRustType, ok := sliceElemPtrMapCandidateForExpr(lhs)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	ident, ok := unwrapParens(lhs).(*ast.Ident)
	if !ok {
		return false
	}
	obj := typeInfo.GetObject(ident)
	if obj == nil {
		return false
	}
	mapType, ok := types.Unalias(obj.Type()).Underlying().(*types.Map)
	if !ok {
		return false
	}
	keyRustType := goTypesMapKeyToRust(mapType.Key())
	var entries []string
	var capacity ast.Expr
	switch init := unwrapParens(rhs).(type) {
	case *ast.CompositeLit:
		for _, elt := range init.Elts {
			kv, ok := elt.(*ast.KeyValueExpr)
			if !ok {
				return false
			}
			var key, value strings.Builder
			writeMapLiteralKeyWithType(&key, kv.Key, mapType.Key())
			if ident, ok := unwrapParens(kv.Value).(*ast.Ident); ok && ident.Name == "nil" {
				value.WriteString("GoPtr::nil()")
			} else if !writeSliceElemPtrSliceSlotValue(&value, kv.Value, elemRustType) {
				return false
			}
			entries = append(entries, "("+key.String()+", "+value.String()+")")
		}
	case *ast.CallExpr:
		if !isBuiltinCallNamed(init, "make") {
			return false
		}
		capacity = sliceElemPtrMapMakeCapacity(init)
	default:
		return false
	}
	NeedSliceElemPtr()
	TrackImport("BTreeMap")
	WriteWrapperPrefix(out)
	if capacity != nil {
		// A BTreeMap takes no capacity hint, but the argument still runs.
		out.WriteString("{ let _ = ")
		TranspileExpression(out, capacity)
		out.WriteString("; ")
	}
	out.WriteString("BTreeMap::<" + keyRustType + ", GoPtr<" + elemRustType + ">>::from([" + strings.Join(entries, ", ") + "])")
	if capacity != nil {
		out.WriteString(" }")
	}
	WriteWrapperSuffix(out)
	return true
}

// writeSliceElemPtrMapValue writes a value stored into such a map.
func writeSliceElemPtrMapValue(out *strings.Builder, mapExpr ast.Expr, value ast.Expr) bool {
	elemRustType, ok := sliceElemPtrMapCandidateForExpr(mapExpr)
	if !ok {
		return false
	}
	if ident, ok := unwrapParens(value).(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("GoPtr::nil()")
		return true
	}
	return writeSliceElemPtrSliceSlotValue(out, value, elemRustType)
}

// writeSliceElemPtrMapNilComparison writes m[k] == nil (or !=) for such a
// map, and ptrs[i] == nil for such a slice: the value read is a GoPtr,
// nil when a key is missing.
func writeSliceElemPtrMapNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	index, ok := unwrapParens(expr).(*ast.IndexExpr)
	if !ok || !heldSliceElemPtrIndex(index) {
		return false
	}
	if op == token.NEQ {
		out.WriteString("!")
	}
	out.WriteString("{ let __nil_ptr = ")
	TranspileExpression(out, index)
	out.WriteString("; __nil_ptr.is_nil() }")
	return true
}

// sliceElemPtrMapMakeCapacity returns make's capacity argument when it is
// not a constant, so its side effects still happen.
func sliceElemPtrMapMakeCapacity(call *ast.CallExpr) ast.Expr {
	if len(call.Args) < 2 {
		return nil
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil && typeInfo.info != nil {
		if tv, ok := typeInfo.info.Types[call.Args[1]]; ok && tv.Value != nil {
			return nil
		}
	}
	return call.Args[1]
}

// isNilIdentExpr reports whether expr is the predeclared nil, maybe in
// parentheses.
func isNilIdentExpr(expr ast.Expr) bool {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok || ident.Name != "nil" {
		return false
	}
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		if obj := typeInfo.GetObject(ident); obj != nil {
			_, isNil := obj.(*types.Nil)
			return isNil
		}
	}
	return true
}
