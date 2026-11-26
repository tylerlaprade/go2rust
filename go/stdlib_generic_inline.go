package main

import (
	"bytes"
	"fmt"
	"go/ast"
	"go/parser"
	"go/printer"
	"go/token"
	"go/types"
	"os"
	"sort"
	"strconv"
	"strings"

	"golang.org/x/tools/go/packages"
)

// The generic functions of slices, maps and cmp that the main package calls
// are copied into it from their Go source and written once per
// instantiation, as the program's own generics are (monomorphize.go).
//
// A Rust generic over E stores every E the same way, and the program stores
// an int, a string and a pointer in three different ways, so a translated
// slices.Index[[]*Task] did not line up with the caller's slice (DEV-180).
// A copy instantiated with *Task is ordinary code over a []*Task.
//
// Each package-level declaration a call reaches is copied under a prefix,
// __slices_Index for slices.Index, with the declarations it reaches in
// turn: pdqsortOrdered, its xorshift and the methods of xorshift, and
// cmp.Less from the cmp package. The calls in the main package are written
// as calls of the copies. An exported type, such as cmp.Ordered or
// iter.Seq, stays a reference to its package, which the copies import.
//
// The copies are appended to one file of the package, which is printed
// and parsed again so that their positions are inside it; comments of that
// file are dropped, as go/printer places a comment by position. The rewrite
// is all or nothing: when a copy has no body, when the main package has
// //go: directives, or when the result does not type-check or monomorphize,
// the program is left as it was and its calls use the stubs.

// stdlibGenericInlinePrefixes are the packages whose declarations are
// copied, with the prefix each copy's name gets.
var stdlibGenericInlinePrefixes = map[string]string{
	"slices": "__slices_",
	"maps":   "__maps_",
	"cmp":    "__cmp_",
}

// stdlibGenericOverrides replace a declaration whose Go source cannot be
// translated. An override uses no package-level name of its package.
//
// slices.overlaps compares element addresses with unsafe pointer
// arithmetic; two windows of one array overlap when the first element of
// one is an element of the other, which &a[i] == &b[0] asks without
// unsafe. maps.Clone calls the runtime's maps.clone, which has no Go body;
// a copy made by ranging over the map has the same keys and values.
var stdlibGenericOverrides = map[string]string{
	"slices.overlaps": `func overlaps[E any](a, b []E) bool {
	if len(a) == 0 || len(b) == 0 {
		return false
	}
	for i := range a {
		if &a[i] == &b[0] {
			return true
		}
	}
	for i := range b {
		if &b[i] == &a[0] {
			return true
		}
	}
	return false
}`,
	"maps.Clone": `func Clone[M ~map[K]V, K comparable, V any](m M) M {
	if m == nil {
		return nil
	}
	r := make(M, len(m))
	for k, v := range m {
		r[k] = v
	}
	return r
}`,
}

func stdlibInlineDebugf(format string, args ...any) {
	if os.Getenv("GO2RUST_DEBUG_MONO") != "" {
		fmt.Fprintf(os.Stderr, "Note: stdlib generics not copied: "+format+"\n", args...)
	}
}

// stdlibInlineSource is a package whose declarations are copied.
type stdlibInlineSource struct {
	pkg     *packages.Package
	decls   map[types.Object]ast.Decl
	methods map[*types.TypeName][]*ast.FuncDecl
}

type stdlibInliner struct {
	fset      *token.FileSet
	mainInfo  *types.Info
	mainScope *types.Scope
	loaded    map[string]*packages.Package
	sources   map[string]*stdlibInlineSource

	target      *ast.File // the copy the declarations are appended to
	targetOrig  *ast.File // the file it is a copy of, which mainInfo covers
	targetNames map[string]bool
	imports     map[string]string // import path -> name in target
	newImports  []string

	queue  []types.Object
	queued map[types.Object]bool
	copied map[ast.Decl]bool
	decls  []ast.Decl
	err    error
}

// inlineStdlibGenerics returns the files of pkg with the generic functions
// of slices, maps and cmp it uses copied in, and the packages the copies
// import that pkg does not, by path. ok is false when there is nothing to
// copy or the copy does not apply.
func inlineStdlibGenerics(pkg *packages.Package, fset *token.FileSet, imp types.Importer) (files []*ast.File, deps map[string]*packages.Package, ok bool) {
	if pkg == nil || pkg.TypesInfo == nil || pkg.Types == nil || len(pkg.Syntax) == 0 || os.Getenv("GO2RUST_GENERIC_RUST") == "1" {
		return nil, nil, false
	}
	in := &stdlibInliner{
		fset:      fset,
		mainInfo:  pkg.TypesInfo,
		mainScope: pkg.Types.Scope(),
		loaded:    loadedPackagesByPath(pkg),
		sources:   map[string]*stdlibInlineSource{},
		imports:   map[string]string{},
		queued:    map[types.Object]bool{},
		copied:    map[ast.Decl]bool{},
	}

	// The calls of the main package.
	files = make([]*ast.File, len(pkg.Syntax))
	for i, file := range pkg.Syntax {
		files[i] = astCopier{rewrite: in.rewriteMainExpr}.copyNode(file).(*ast.File)
	}
	if len(in.queue) == 0 {
		return nil, nil, false
	}
	targetIndex := -1
	for i, file := range pkg.Syntax {
		if !fileHasDirectiveComments(file) {
			targetIndex = i
			break
		}
	}
	if targetIndex < 0 {
		stdlibInlineDebugf("every file has //go: directives")
		return nil, nil, false
	}
	in.target = files[targetIndex]
	in.targetOrig = pkg.Syntax[targetIndex]
	in.targetNames = in.fileImportNames(in.targetOrig)

	// The declarations the calls reach.
	for len(in.queue) > 0 && in.err == nil {
		obj := in.queue[0]
		in.queue = in.queue[1:]
		in.copyDeclOf(obj)
	}
	if in.err != nil {
		stdlibInlineDebugf("%v", in.err)
		return nil, nil, false
	}

	in.target.Decls = append(in.target.Decls, in.decls...)
	in.addImports()
	reparsed, ok := reparseGoFiles([]*ast.File{in.target}, fset, false)
	if !ok {
		return nil, nil, false
	}
	files[targetIndex] = reparsed[0]

	deps = map[string]*packages.Package{}
	for _, path := range in.newImports {
		dep := in.loaded[path]
		if dep == nil || dep.Types == nil {
			stdlibInlineDebugf("package %s is not loaded", path)
			return nil, nil, false
		}
		deps[path] = dep
	}
	checkImp := stdlibInlineImporter{base: imp, deps: deps}
	for attempt := 0; ; attempt++ {
		_, err := NewTypeInfoWithImporter(pkg.PkgPath, files, fset, checkImp)
		if err == nil {
			return files, deps, true
		}
		if attempt == 2 || !removeUnusedImports(files, err) {
			stdlibInlineDebugf("the copies do not type-check: %v", err)
			return nil, nil, false
		}
	}
}

// loadedPackagesByPath returns every package of the load graph under pkg.
func loadedPackagesByPath(pkg *packages.Package) map[string]*packages.Package {
	out := map[string]*packages.Package{}
	var visit func(*packages.Package)
	visit = func(p *packages.Package) {
		if p == nil || out[p.PkgPath] != nil {
			return
		}
		out[p.PkgPath] = p
		paths := make([]string, 0, len(p.Imports))
		for path := range p.Imports {
			paths = append(paths, path)
		}
		sort.Strings(paths)
		for _, path := range paths {
			visit(p.Imports[path])
		}
	}
	visit(pkg)
	return out
}

func (in *stdlibInliner) fail(format string, args ...any) {
	if in.err == nil {
		in.err = fmt.Errorf(format, args...)
	}
}

// rewriteMainExpr writes a call of slices.Index in the main package as a
// call of the copy __slices_Index.
func (in *stdlibInliner) rewriteMainExpr(e ast.Expr) (ast.Expr, bool) {
	sel, ok := e.(*ast.SelectorExpr)
	if !ok {
		return nil, false
	}
	pkgIdent, ok := sel.X.(*ast.Ident)
	if !ok {
		return nil, false
	}
	if _, ok := in.mainInfo.Uses[pkgIdent].(*types.PkgName); !ok {
		return nil, false
	}
	fn, ok := in.mainInfo.Uses[sel.Sel].(*types.Func)
	if !ok || fn.Pkg() == nil {
		return nil, false
	}
	prefix, ok := stdlibGenericInlinePrefixes[fn.Pkg().Path()]
	if !ok {
		return nil, false
	}
	in.enqueue(fn)
	return &ast.Ident{NamePos: sel.Sel.NamePos, Name: prefix + fn.Name()}, true
}

func (in *stdlibInliner) enqueue(obj types.Object) {
	if !in.queued[obj] {
		in.queued[obj] = true
		in.queue = append(in.queue, obj)
	}
}

// isPackageLevel reports whether obj is declared at the top level of its
// package.
func isPackageLevel(obj types.Object) bool {
	return obj != nil && obj.Pkg() != nil && obj.Pkg().Scope().Lookup(obj.Name()) == obj
}

// copiedAsReference reports whether a use of obj, of a package whose
// declarations are copied, stays a use of that package: an exported type
// is one the copies can name there.
func copiedAsReference(obj types.Object) bool {
	_, isType := obj.(*types.TypeName)
	return isType && obj.Exported()
}

func (in *stdlibInliner) source(path string) *stdlibInlineSource {
	if src, ok := in.sources[path]; ok {
		return src
	}
	pkg := in.loaded[path]
	if pkg == nil || pkg.TypesInfo == nil || len(pkg.Syntax) == 0 {
		in.fail("package %s has no syntax", path)
		in.sources[path] = nil
		return nil
	}
	src := &stdlibInlineSource{pkg: pkg, decls: map[types.Object]ast.Decl{}, methods: map[*types.TypeName][]*ast.FuncDecl{}}
	for _, file := range pkg.Syntax {
		for _, decl := range file.Decls {
			switch d := decl.(type) {
			case *ast.FuncDecl:
				if d.Recv == nil {
					if obj := pkg.TypesInfo.Defs[d.Name]; obj != nil {
						src.decls[obj] = d
					}
					continue
				}
				if fn, ok := pkg.TypesInfo.Defs[d.Name].(*types.Func); ok {
					if named := receiverNamed(fn); named != nil {
						src.methods[named.Obj()] = append(src.methods[named.Obj()], d)
					}
				}
			case *ast.GenDecl:
				for _, spec := range d.Specs {
					switch s := spec.(type) {
					case *ast.TypeSpec:
						if obj := pkg.TypesInfo.Defs[s.Name]; obj != nil {
							src.decls[obj] = d
						}
					case *ast.ValueSpec:
						for _, name := range s.Names {
							if obj := pkg.TypesInfo.Defs[name]; obj != nil {
								src.decls[obj] = d
							}
						}
					}
				}
			}
		}
	}
	in.sources[path] = src
	return src
}

func receiverNamed(fn *types.Func) *types.Named {
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Recv() == nil {
		return nil
	}
	t := sig.Recv().Type()
	if ptr, ok := t.(*types.Pointer); ok {
		t = ptr.Elem()
	}
	named, _ := t.(*types.Named)
	return named
}

// copyDeclOf copies the declaration of obj, and for a type its methods.
func (in *stdlibInliner) copyDeclOf(obj types.Object) {
	path := obj.Pkg().Path()
	prefix := stdlibGenericInlinePrefixes[path]
	if override, ok := stdlibGenericOverrides[path+"."+obj.Name()]; ok {
		in.copyOverride(prefix, override)
		return
	}
	src := in.source(path)
	if src == nil {
		return
	}
	decl, ok := src.decls[obj]
	if !ok {
		in.fail("%s.%s has no declaration", path, obj.Name())
		return
	}
	in.copySourceDecl(src, decl)
	if typeName, ok := obj.(*types.TypeName); ok {
		for _, method := range src.methods[typeName] {
			in.copySourceDecl(src, method)
		}
	}
}

func (in *stdlibInliner) copySourceDecl(src *stdlibInlineSource, decl ast.Decl) {
	if in.copied[decl] {
		return
	}
	in.copied[decl] = true
	if fn, ok := decl.(*ast.FuncDecl); ok && fn.Body == nil {
		in.fail("%s.%s has no Go body", src.pkg.PkgPath, fn.Name.Name)
		return
	}
	info := src.pkg.TypesInfo
	c := astCopier{
		rewrite: func(e ast.Expr) (ast.Expr, bool) { return in.rewriteSourceExpr(info, e) },
		ident: func(id *ast.Ident) (*ast.Ident, bool) {
			obj := info.Defs[id]
			if !isPackageLevel(obj) {
				return nil, false
			}
			prefix, ok := stdlibGenericInlinePrefixes[obj.Pkg().Path()]
			if !ok {
				return nil, false
			}
			return &ast.Ident{NamePos: id.NamePos, Name: prefix + id.Name}, true
		},
	}
	copied := c.copyNode(decl).(ast.Decl)
	switch d := copied.(type) {
	case *ast.FuncDecl:
		d.Doc = nil
	case *ast.GenDecl:
		d.Doc = nil
	}
	in.decls = append(in.decls, copied)
}

func (in *stdlibInliner) copyOverride(prefix, override string) {
	file, err := parser.ParseFile(in.fset, "go2rust_stdlib_override.go", "package p\n\n"+override+"\n", parser.SkipObjectResolution)
	if err != nil || len(file.Decls) != 1 {
		in.fail("override does not parse: %v", err)
		return
	}
	fn, ok := file.Decls[0].(*ast.FuncDecl)
	if !ok {
		in.fail("override is not a function")
		return
	}
	fn.Name = &ast.Ident{NamePos: fn.Name.NamePos, Name: prefix + fn.Name.Name}
	in.decls = append(in.decls, fn)
}

// rewriteSourceExpr writes a use of a package-level declaration in a copy:
// the name of its copy, or a reference to its package by the name the
// target file imports it by.
func (in *stdlibInliner) rewriteSourceExpr(info *types.Info, e ast.Expr) (ast.Expr, bool) {
	switch x := e.(type) {
	case *ast.Ident:
		return in.reference(info.Uses[x], x)
	case *ast.SelectorExpr:
		pkgIdent, ok := x.X.(*ast.Ident)
		if !ok {
			return nil, false
		}
		if _, ok := info.Uses[pkgIdent].(*types.PkgName); !ok {
			return nil, false
		}
		obj := info.Uses[x.Sel]
		if !isPackageLevel(obj) {
			in.fail("%s.%s is not a package-level declaration", pkgIdent.Name, x.Sel.Name)
			return nil, false
		}
		return in.reference(obj, x.Sel)
	}
	return nil, false
}

func (in *stdlibInliner) reference(obj types.Object, at *ast.Ident) (ast.Expr, bool) {
	if !isPackageLevel(obj) {
		return nil, false
	}
	if prefix, ok := stdlibGenericInlinePrefixes[obj.Pkg().Path()]; ok && !copiedAsReference(obj) {
		in.enqueue(obj)
		return &ast.Ident{NamePos: at.NamePos, Name: prefix + obj.Name()}, true
	}
	name := in.importName(obj.Pkg())
	return &ast.SelectorExpr{X: &ast.Ident{NamePos: at.NamePos, Name: name}, Sel: &ast.Ident{NamePos: at.NamePos, Name: obj.Name()}}, true
}

// fileImportNames returns the names file imports packages by.
func (in *stdlibInliner) fileImportNames(file *ast.File) map[string]bool {
	names := map[string]bool{}
	for _, spec := range file.Imports {
		if name := in.importSpecName(spec); name != "" {
			names[name] = true
		}
	}
	return names
}

func (in *stdlibInliner) importSpecName(spec *ast.ImportSpec) string {
	if spec.Name != nil {
		return spec.Name.Name
	}
	if pkgName, ok := in.mainInfo.Implicits[spec].(*types.PkgName); ok {
		return pkgName.Name()
	}
	return ""
}

// importName returns the name the target file imports pkg by, and records
// a new import when it does not import pkg yet.
func (in *stdlibInliner) importName(pkg *types.Package) string {
	if name, ok := in.imports[pkg.Path()]; ok {
		return name
	}
	for _, spec := range in.targetOrig.Imports {
		path, err := strconv.Unquote(spec.Path.Value)
		if err != nil || path != pkg.Path() {
			continue
		}
		if name := in.importSpecName(spec); name != "" && name != "_" && name != "." {
			in.imports[pkg.Path()] = name
			return name
		}
	}
	name := pkg.Name()
	for n := 2; in.targetNames[name] || in.mainScope.Lookup(name) != nil; n++ {
		name = "__go2rust_" + pkg.Name()
		if n > 2 {
			name += strconv.Itoa(n)
		}
	}
	in.targetNames[name] = true
	in.imports[pkg.Path()] = name
	in.newImports = append(in.newImports, pkg.Path())
	return name
}

// addImports adds the target file's new imports to its import declaration.
func (in *stdlibInliner) addImports() {
	if len(in.newImports) == 0 {
		return
	}
	sort.Strings(in.newImports)
	var specs []ast.Spec
	for _, path := range in.newImports {
		spec := &ast.ImportSpec{Path: &ast.BasicLit{Kind: token.STRING, Value: strconv.Quote(path)}}
		if name := in.imports[path]; name != importPathBase(path) {
			spec.Name = ast.NewIdent(name)
		}
		specs = append(specs, spec)
		in.target.Imports = append(in.target.Imports, spec)
	}
	decl := &ast.GenDecl{Tok: token.IMPORT, Lparen: 1, Specs: specs, Rparen: 1}
	// After the file's own imports, which come first in a Go file.
	at := 0
	for at < len(in.target.Decls) {
		if gen, ok := in.target.Decls[at].(*ast.GenDecl); !ok || gen.Tok != token.IMPORT {
			break
		}
		at++
	}
	in.target.Decls = append(in.target.Decls[:at], append([]ast.Decl{decl}, in.target.Decls[at:]...)...)
}

// filesImportPath reports whether one of files imports path.
func filesImportPath(files []*ast.File, path string) bool {
	quoted := strconv.Quote(path)
	for _, file := range files {
		for _, spec := range file.Imports {
			if spec.Path != nil && spec.Path.Value == quoted {
				return true
			}
		}
	}
	return false
}

func importPathBase(path string) string {
	if i := strings.LastIndex(path, "/"); i >= 0 {
		return path[i+1:]
	}
	return path
}

// stdlibInlineImporter resolves the packages the copies import, which the
// project's importer has not collected, to the packages loaded with the
// program, so their types are the ones the program's packages use.
type stdlibInlineImporter struct {
	base types.Importer
	deps map[string]*packages.Package
}

func (i stdlibInlineImporter) Import(path string) (*types.Package, error) {
	if pi, ok := i.base.(*projectImporter); ok && pi.pl != nil {
		if pkg, ok := pi.pl.allPackages[path]; ok && pkg != nil && pkg.Types != nil {
			return pkg.Types, nil
		}
	}
	if dep, ok := i.deps[path]; ok && dep.Types != nil {
		return dep.Types, nil
	}
	return i.base.Import(path)
}

// stdlibPackagesWithGenericCopies returns the source of each file, by path,
// of the standard-library packages under main that are translated from
// source and call slices, maps or cmp, rewritten with copies of what they call
// (DEV-223). sort.Ints calls slices.Sort, which as a Rust generic over the
// stub did not compile. Only the copies are written once per
// instantiation: a package keeps its own generics, for its importers to
// instantiate. A package the rewrite does not apply to is left out.
func stdlibPackagesWithGenericCopies(main *packages.Package) map[string][]byte {
	if os.Getenv("GO2RUST_GENERIC_RUST") == "1" {
		return nil
	}
	loaded := loadedPackagesByPath(main)
	selected := sourceStdlibSelection(main)
	paths := make([]string, 0, len(loaded))
	for path, pkg := range loaded {
		if pkg == main || !selected[path] {
			continue
		}
		if _, copied := stdlibGenericInlinePrefixes[path]; copied {
			continue
		}
		_, slices := pkg.Imports["slices"]
		_, maps := pkg.Imports["maps"]
		_, cmp := pkg.Imports["cmp"]
		if slices || maps || cmp {
			paths = append(paths, path)
		}
	}
	sort.Strings(paths)
	isCopy := func(name string) bool {
		for _, prefix := range stdlibGenericInlinePrefixes {
			if strings.HasPrefix(name, prefix) {
				return true
			}
		}
		return false
	}
	out := map[string][]byte{}
	for _, path := range paths {
		pkg := loaded[path]
		imp := loadedPackagesImporter{loaded: loaded}
		inlined, deps, ok := inlineStdlibGenerics(pkg, pkg.Fset, imp)
		if !ok {
			continue
		}
		checkImp := stdlibInlineImporter{base: imp, deps: deps}
		files, ok := monomorphizeGenericsMatching(pkg.PkgPath, inlined, pkg.Fset, checkImp, isCopy)
		if !ok {
			stdlibInlineDebugf("%s: the copies do not monomorphize", path)
			continue
		}
		sources := map[string][]byte{}
		for i, file := range files {
			if i >= len(pkg.Syntax) || i >= len(pkg.CompiledGoFiles) || !fileCallsCopies(file) {
				continue
			}
			if fileHasCompileDirectives(pkg.Syntax[i]) {
				sources = nil
				break
			}
			var buf bytes.Buffer
			// The file keeps its build constraint, the one comment
			// that decides which files are in the package.
			for _, group := range pkg.Syntax[i].Comments {
				if group.End() >= pkg.Syntax[i].Package {
					break
				}
				for _, comment := range group.List {
					if strings.HasPrefix(comment.Text, "//go:build") || strings.HasPrefix(comment.Text, "// +build") {
						buf.WriteString(comment.Text)
						buf.WriteString("\n")
					}
				}
			}
			if buf.Len() > 0 {
				buf.WriteString("\n")
			}
			file.Comments = nil
			cfg := printer.Config{Mode: printer.UseSpaces | printer.TabIndent, Tabwidth: 8}
			if err := cfg.Fprint(&buf, pkg.Fset, file); err != nil {
				sources = nil
				break
			}
			sources[pkg.CompiledGoFiles[i]] = buf.Bytes()
		}
		for name, content := range sources {
			out[name] = content
		}
	}
	return out
}

// sourceStdlibSelection lists the standard-library packages under main that
// collectAllPackagesFrom translates from source: those the default list or
// GO2RUST_SOURCE_STDLIB_PACKAGES selects, and the imports, at any depth, of
// one whose pattern ends in +deps.
func sourceStdlibSelection(main *packages.Package) map[string]bool {
	patterns := os.Getenv(sourceStdlibPackagesEnv)
	selected := map[string]bool{}
	expanded := map[string]bool{}
	stored := map[string]bool{}
	var visit func(pkg *packages.Package, path string, force bool)
	visit = func(pkg *packages.Package, path string, force bool) {
		if pkg == nil {
			return
		}
		isStdlib := isStdlibPackage(path)
		isSource := isStdlib && isSourceTranspilableStdlibPackage(path) && (force || selected[path] || shouldTranspileStdlibPackage(path))
		expand := isSource && (force || expanded[path] || sourceStdlibPackagePatternExpandsDeps(path, patterns))
		forceImport := func(importPath string) bool {
			return expand && isStdlibPackage(importPath) && isSourceTranspilableStdlibPackage(importPath)
		}
		if stored[path] {
			if expand && !expanded[path] {
				expanded[path] = true
				for importPath, imp := range pkg.Imports {
					visit(imp, importPath, forceImport(importPath))
				}
			}
			return
		}
		if pkg != main && isStdlib && !isSource {
			return
		}
		stored[path] = true
		if isSource {
			selected[path] = true
		}
		if expand {
			expanded[path] = true
		}
		for importPath, imp := range pkg.Imports {
			visit(imp, importPath, forceImport(importPath))
		}
	}
	visit(main, main.PkgPath, false)
	return selected
}

// fileCallsCopies reports whether file declares or calls a copy.
func fileCallsCopies(file *ast.File) bool {
	found := false
	ast.Inspect(file, func(n ast.Node) bool {
		if ident, ok := n.(*ast.Ident); ok && !found {
			for _, prefix := range stdlibGenericInlinePrefixes {
				if strings.HasPrefix(ident.Name, prefix) {
					found = true
				}
			}
		}
		return !found
	})
	return found
}

// fileHasCompileDirectives reports whether file has a directive that
// printing it again without comments would lose: //go:linkname,
// //go:embed, //line or cgo. //go:build and //go:generate do not matter
// once the file is in the build.
func fileHasCompileDirectives(file *ast.File) bool {
	for _, group := range file.Comments {
		for _, comment := range group.List {
			if strings.HasPrefix(comment.Text, "//go:") && !strings.HasPrefix(comment.Text, "//go:build") && !strings.HasPrefix(comment.Text, "//go:generate") {
				return true
			}
			if strings.HasPrefix(comment.Text, "//line ") {
				return true
			}
		}
	}
	for _, spec := range file.Imports {
		if spec.Path != nil && spec.Path.Value == `"C"` {
			return true
		}
	}
	return false
}

// loadedPackagesImporter resolves imports to the packages of one load.
type loadedPackagesImporter struct {
	loaded map[string]*packages.Package
}

func (i loadedPackagesImporter) Import(path string) (*types.Package, error) {
	if pkg, ok := i.loaded[path]; ok && pkg.Types != nil {
		return pkg.Types, nil
	}
	return nil, fmt.Errorf("package %q is not loaded", path)
}

// packagesHaveErrors reports whether a package of the load under main whose
// files were rewritten has errors.
func packagesHaveErrors(main *packages.Package, rewritten map[string][]byte) bool {
	for _, pkg := range loadedPackagesByPath(main) {
		for _, name := range pkg.CompiledGoFiles {
			if _, ok := rewritten[name]; ok && len(pkg.Errors) > 0 {
				return true
			}
		}
	}
	return false
}
