package main

import (
	"go/ast"
	"go/types"
)

// sourceStdlibReachable holds the func/method/type objects reachable from the
// program's live code. Non-nil only when at least one stdlib package is
// transpiled from source. When non-nil, declaration emission for source-mapped
// packages prunes funcs/methods/types not in this set (dead-code elimination),
// so peripheral declarations pulling in heavy deps (go/ast's reflect-based
// printer + FieldFilter, path/filepath's os-based Glob) don't block compilation
// of the subset the program actually uses. nil disables pruning.
var sourceStdlibReachable map[types.Object]bool

// SetSourceStdlibReachable installs the reachable-object set.
func SetSourceStdlibReachable(set map[types.Object]bool) { sourceStdlibReachable = set }

// sourceMappedDeclIsPruned reports whether obj is a source-mapped
// function/method/type that is unreachable and should not be emitted.
func sourceMappedDeclIsPruned(obj types.Object) bool {
	if sourceStdlibReachable == nil || obj == nil {
		return false
	}
	pkg := obj.Pkg()
	if pkg == nil || !isSourceMappedPackagePath(pkg.Path()) {
		return false
	}
	return !sourceStdlibReachable[obj]
}

// isPrunedSourceDecl reports whether the declaration named by ident is a
// source-mapped func/type that DCE has determined is unreachable. Used at
// emission call sites so a pruned decl's leading doc comment is skipped too.
func isPrunedSourceDecl(ident *ast.Ident) bool {
	if ident == nil {
		return false
	}
	ti := GetTypeInfo()
	if ti == nil {
		return false
	}
	return sourceMappedDeclIsPruned(ti.GetObject(ident))
}

// implReceiverTypeIsPruned reports whether the type that owns the given methods
// is a source-mapped type pruned by DCE. The per-file prunedTypeNames gate only
// sees types DECLARED in the current file, but a type's methods can live in a
// different file from its decl (e.g. token.FileSet's Read/Write in serialize.go
// vs the FileSet decl in position.go). When the type is unreachable, that other
// file would still emit `impl FileSet { ... }` referencing a type that was never
// emitted. Gating the impl block on the receiver type's reachability, resolved
// through go/types rather than file membership, keeps emission consistent with DCE.
//
// A *types.Named has one canonical TypeName (named.Obj()), the same object the
// reachability set is keyed by, so there is no object-identity mismatch here
// (unlike the named.Method(i) trap reachability.go documents for methods).
func implReceiverTypeIsPruned(typeMethods []*ast.FuncDecl) bool {
	if sourceStdlibReachable == nil || len(typeMethods) == 0 {
		return false
	}
	ti := GetTypeInfo()
	if ti == nil {
		return false
	}
	for _, m := range typeMethods {
		if m == nil || m.Name == nil {
			continue
		}
		fn, ok := ti.GetObject(m.Name).(*types.Func)
		if !ok {
			continue
		}
		sig, ok := fn.Type().(*types.Signature)
		if !ok || sig.Recv() == nil {
			continue
		}
		recv := types.Unalias(sig.Recv().Type())
		if ptr, ok := recv.(*types.Pointer); ok {
			recv = types.Unalias(ptr.Elem())
		}
		named, ok := recv.(*types.Named)
		if !ok || named.Obj() == nil {
			continue
		}
		return sourceMappedDeclIsPruned(named.Obj())
	}
	return false
}

// computeSourceStdlibReachable builds the reachable func/method/type set with a
// single uniform object-reachability pass.
//
// Roots (always live): every func/method/type in NON-source-mapped packages,
// every init func, and every package-level var/const (run at init) — plus
// everything those transitively reference.
//
// Edges: an object's declaration is scanned for references via Info.Uses (funcs
// + type names) and concrete MethodVal selections; and when a TYPE is reachable
// ALL of its methods become reachable (trait impls stay complete, dynamic
// dispatch works). A type referenced only by unreachable code (go/ast's printer
// + FieldFilter, touched only by the unreachable ast.Print) is pruned, taking
// its reflect references with it.
//
// CRITICAL (object identity): methods are tracked by the *types.Func that
// info.Defs assigns to the method declaration — the SAME objects that
// refs/edges are keyed by and that the gate checks via GetObject. Using
// named.Method(i) instead can yield a different *types.Func whose edges aren't
// populated, so the types those methods reference (e.g. token.Position returned
// by FileSet.Position) get wrongly pruned. methodsByType is built from
// info.Defs to avoid that.
func (pl *PackageLoader) computeSourceStdlibReachable() map[types.Object]bool {
	anySourceMapped := false
	for path := range pl.allPackages {
		if shouldTranspileStdlibPackage(path) {
			anySourceMapped = true
			break
		}
	}
	if !anySourceMapped {
		return nil
	}

	refs := map[types.Object][]types.Object{}
	methodsByType := map[*types.TypeName][]types.Object{}
	var roots []types.Object

	collect := func(info *types.Info, n ast.Node) []types.Object {
		var out []types.Object
		ast.Inspect(n, func(node ast.Node) bool {
			switch e := node.(type) {
			case *ast.Ident:
				switch o := info.Uses[e].(type) {
				case *types.Func:
					out = append(out, o)
				case *types.TypeName:
					out = append(out, o)
				}
			case *ast.SelectorExpr:
				if sel, ok := info.Selections[e]; ok && sel.Kind() == types.MethodVal && !types.IsInterface(sel.Recv()) {
					if f, ok := sel.Obj().(*types.Func); ok {
						out = append(out, f)
					}
				}
			}
			return true
		})
		return out
	}

	// receiverTypeName resolves a method's receiver to its declaring TypeName.
	receiverTypeName := func(obj types.Object) *types.TypeName {
		fn, ok := obj.(*types.Func)
		if !ok {
			return nil
		}
		sig, ok := fn.Type().(*types.Signature)
		if !ok || sig.Recv() == nil {
			return nil
		}
		recvType := types.Unalias(sig.Recv().Type())
		if ptr, ok := recvType.(*types.Pointer); ok {
			recvType = types.Unalias(ptr.Elem())
		}
		named, ok := recvType.(*types.Named)
		if !ok {
			return nil
		}
		return named.Obj()
	}

	for path, pkg := range pl.allPackages {
		info := pkg.TypesInfo
		if info == nil {
			continue
		}
		src := shouldTranspileStdlibPackage(path)
		for _, file := range pkg.Syntax {
			for _, decl := range file.Decls {
				switch d := decl.(type) {
				case *ast.FuncDecl:
					obj := info.Defs[d.Name]
					if obj == nil {
						continue
					}
					refs[obj] = collect(info, d)
					if d.Recv != nil {
						if tn := receiverTypeName(obj); tn != nil {
							methodsByType[tn] = append(methodsByType[tn], obj)
						}
					}
					if !src || d.Name.Name == "init" {
						roots = append(roots, obj)
					}
				case *ast.GenDecl:
					for _, spec := range d.Specs {
						switch s := spec.(type) {
						case *ast.TypeSpec:
							if obj := info.Defs[s.Name]; obj != nil {
								refs[obj] = collect(info, s)
								if !src {
									roots = append(roots, obj)
								}
							}
						case *ast.ValueSpec:
							for _, nm := range s.Names {
								if obj := info.Defs[nm]; obj != nil {
									refs[obj] = collect(info, s)
									roots = append(roots, obj)
								}
							}
						}
					}
				}
			}
		}
	}

	reachable := map[types.Object]bool{}
	var work []types.Object
	push := func(o types.Object) {
		if o != nil && !reachable[o] {
			reachable[o] = true
			work = append(work, o)
		}
	}
	for _, r := range roots {
		push(r)
	}
	for len(work) > 0 {
		o := work[len(work)-1]
		work = work[:len(work)-1]
		for _, c := range refs[o] {
			push(c)
		}
		if tn, ok := o.(*types.TypeName); ok {
			for _, m := range methodsByType[tn] {
				push(m)
			}
		}
	}
	return reachable
}
