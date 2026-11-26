package main

import (
	"bytes"
	"fmt"
	"go/ast"
	"go/constant"
	"go/parser"
	"go/printer"
	"go/token"
	"go/types"
	"os"
	"strconv"
	"strings"
)

// The %#v verb writes a value the way Go source writes it: main.T{X:1},
// []string{"a"}, map[string]int{"a":1}, &main.T{...}. Rust has no format
// trait that knows Go's type names or its rules, so before translation each
// %#v of a constant format string becomes %s of a call to a helper written
// in Go for the argument's static type, and the helpers are translated with
// the rest of the package. A helper follows fmt's printValue: GoString when
// the type has it, the type name and fields of a struct, the elements of a
// slice, array or map (keys sorted), & before a struct, array, slice or map
// a top-level pointer points at, the address of any other pointer, and the
// dynamic value of an interface, chosen by a type switch over the concrete
// types the package converts to an interface.

func goSyntaxDebugf(format string, args ...any) {
	if os.Getenv("GO2RUST_DEBUG_GOSYNTAX") != "" {
		fmt.Fprintf(os.Stderr, "Note: %%#v kept as it is: "+format+"\n", args...)
	}
}

// goSyntaxFormatArg gives the index of the format argument of each function
// whose format %#v is rewritten.
var goSyntaxFormatArg = map[string]int{
	"fmt.Printf":           0,
	"fmt.Sprintf":          0,
	"fmt.Errorf":           0,
	"fmt.Fprintf":          1,
	"fmt.Appendf":          1,
	"log.Printf":           0,
	"log.Fatalf":           0,
	"log.Panicf":           0,
	"(*log.Logger).Printf": 0,
	"(*log.Logger).Fatalf": 0,
	"(*log.Logger).Panicf": 0,
}

// rewriteGoSyntaxVerbs returns files with each %#v of a constant format
// written as %s of a helper call, and the helpers added to the files that
// call them. ok is false when there is nothing to rewrite or the result
// does not type-check.
func rewriteGoSyntaxVerbs(pkgPath string, files []*ast.File, fset *token.FileSet, info *types.Info, pkg *types.Package, imp types.Importer) ([]*ast.File, *TypeInfo, bool) {
	if info == nil || pkg == nil || len(files) == 0 || os.Getenv("GO2RUST_GOSYNTAX") == "0" {
		return nil, nil, false
	}
	r := &goSyntaxRewriter{fset: fset, info: info, pkg: pkg}
	var plans []*goSyntaxFile
	for i, file := range files {
		if fileHasDirectiveComments(file) {
			continue
		}
		f := r.planFile(i, file)
		if f != nil && len(f.calls) > 0 {
			plans = append(plans, f)
		}
	}
	if len(plans) == 0 {
		return nil, nil, false
	}
	r.flows = goSyntaxInterfaceFlows(files, info)

	out := append([]*ast.File(nil), files...)
	for _, f := range plans {
		for _, call := range f.order {
			f.planCall(call)
		}
		if len(f.rewrites) == 0 {
			continue
		}
		copied := astCopier{}
		copied.rewrite = func(e ast.Expr) (ast.Expr, bool) {
			call, ok := e.(*ast.CallExpr)
			if !ok {
				return nil, false
			}
			plan, ok := f.rewrites[call]
			if !ok {
				return nil, false
			}
			return plan.apply(copied, call), true
		}
		file := copied.copyNode(f.file).(*ast.File)
		f.insertLocalHelpers(file)
		var printed bytes.Buffer
		cfg := printer.Config{Mode: printer.UseSpaces | printer.TabIndent, Tabwidth: 8}
		if err := cfg.Fprint(&printed, fset, file); err != nil {
			goSyntaxDebugf("printing the rewrite: %v", err)
			return nil, nil, false
		}
		text := printed.String()
		for _, helpers := range f.inserts {
			for _, h := range helpers {
				placeholder := h.name + "__insert()"
				if strings.Count(text, placeholder) != 1 {
					goSyntaxDebugf("the helper %s has no one place", h.name)
					return nil, nil, false
				}
				text = strings.Replace(text, placeholder, h.source, 1)
			}
		}
		var buf bytes.Buffer
		buf.WriteString(text)
		buf.WriteString("\n")
		for _, h := range f.helpers {
			buf.WriteString(h.source)
			buf.WriteString("\n")
		}
		name := fset.Position(f.file.Package).Filename
		if os.Getenv("GO2RUST_DEBUG_GOSYNTAX") == "dump" {
			fmt.Fprintf(os.Stderr, "// %s after the %%#v rewrite\n%s\n", name, buf.String())
		}
		parsed, err := parser.ParseFile(fset, name, buf.Bytes(), parser.SkipObjectResolution|parser.ParseComments)
		if err != nil {
			goSyntaxDebugf("parsing the rewrite of %s: %v", name, err)
			return nil, nil, false
		}
		out[f.index] = parsed
	}
	changed := false
	for i := range out {
		if out[i] != files[i] {
			changed = true
		}
	}
	if !changed {
		return nil, nil, false
	}
	var ti *TypeInfo
	var err error
	if imp == nil {
		ti, err = NewTypeInfo(out, fset)
	} else {
		ti, err = NewTypeInfoWithImporter(pkgPath, out, fset, imp)
	}
	if err != nil || ti == nil {
		goSyntaxDebugf("the rewrite does not type-check: %v", err)
		return nil, nil, false
	}
	return out, ti, true
}

type goSyntaxRewriter struct {
	fset  *token.FileSet
	info  *types.Info
	pkg   *types.Package
	flows []types.Type
}

type goSyntaxFile struct {
	r          *goSyntaxRewriter
	index      int
	file       *ast.File
	fmtName    string
	imports    map[string]string // import path -> name in the file
	calls      map[*ast.CallExpr]int
	order      []*ast.CallExpr
	rewrites   map[*ast.CallExpr]*goSyntaxCall
	helpers    []*goSyntaxHelper
	byKey      map[string]*goSyntaxHelper
	inserts    map[token.Pos][]*goSyntaxHelper
	localDecls map[token.Pos]*ast.DeclStmt
}

type goSyntaxHelper struct {
	name   string
	source string
}

type goSyntaxCall struct {
	formatArg int
	format    string
	wrap      map[int]string // argument index -> helper
}

// planFile finds the calls of file whose constant format has a %#v.
func (r *goSyntaxRewriter) planFile(index int, file *ast.File) *goSyntaxFile {
	f := &goSyntaxFile{
		r:        r,
		index:    index,
		file:     file,
		imports:  map[string]string{},
		calls:    map[*ast.CallExpr]int{},
		rewrites: map[*ast.CallExpr]*goSyntaxCall{},
		byKey:    map[string]*goSyntaxHelper{},
		inserts:  map[token.Pos][]*goSyntaxHelper{},
	}
	for _, spec := range file.Imports {
		path, err := strconv.Unquote(spec.Path.Value)
		if err != nil {
			continue
		}
		name := ""
		if spec.Name != nil {
			name = spec.Name.Name
		} else if imported := r.importedPackage(spec); imported != nil {
			name = imported.Name()
		} else {
			name = importPathBase(path)
		}
		if name == "_" || name == "." {
			continue
		}
		f.imports[path] = name
		if path == "fmt" {
			f.fmtName = name
		}
	}
	if f.fmtName == "" {
		return nil
	}
	ast.Inspect(file, func(n ast.Node) bool {
		call, ok := n.(*ast.CallExpr)
		if !ok || call.Ellipsis != token.NoPos {
			return true
		}
		formatArg, ok := goSyntaxFormatArg[r.calleeName(call)]
		if !ok || formatArg >= len(call.Args) {
			return true
		}
		tv, ok := r.info.Types[call.Args[formatArg]]
		if !ok || tv.Value == nil || tv.Value.Kind() != constant.String {
			return true
		}
		if strings.Contains(constant.StringVal(tv.Value), "%#v") {
			f.calls[call] = formatArg
			f.order = append(f.order, call)
		}
		return true
	})
	return f
}

func (r *goSyntaxRewriter) importedPackage(spec *ast.ImportSpec) *types.Package {
	obj := r.info.Implicits[spec]
	if obj == nil {
		return nil
	}
	if name, ok := obj.(*types.PkgName); ok {
		return name.Imported()
	}
	return nil
}

func (r *goSyntaxRewriter) calleeName(call *ast.CallExpr) string {
	var obj types.Object
	switch fun := ast.Unparen(call.Fun).(type) {
	case *ast.Ident:
		obj = r.info.Uses[fun]
	case *ast.SelectorExpr:
		if sel := r.info.Selections[fun]; sel != nil {
			obj = sel.Obj()
		} else {
			obj = r.info.Uses[fun.Sel]
		}
	}
	fn, ok := obj.(*types.Func)
	if !ok || fn.Pkg() == nil {
		return ""
	}
	return fn.FullName()
}

// planCall rewrites each %#v of call's format whose argument a helper can
// write.
func (f *goSyntaxFile) planCall(call *ast.CallExpr) {
	formatArg := f.calls[call]
	format := constant.StringVal(f.r.info.Types[call.Args[formatArg]].Value)
	verbs, ok := goFormatVerbs(format)
	if !ok {
		return
	}
	plan := &goSyntaxCall{formatArg: formatArg, wrap: map[int]string{}}
	var out strings.Builder
	last := 0
	for _, verb := range verbs {
		if verb.spec != "%#v" {
			continue
		}
		at := formatArg + 1 + verb.arg
		if at >= len(call.Args) {
			continue
		}
		tv, ok := f.r.info.Types[call.Args[at]]
		if !ok || tv.Type == nil || tv.IsNil() {
			continue
		}
		helper, ok := f.helperFor(types.Default(tv.Type), true)
		if !ok {
			goSyntaxDebugf("%s: no helper for %s", f.r.fset.Position(call.Args[at].Pos()), tv.Type)
			continue
		}
		out.WriteString(format[last:verb.start])
		out.WriteString("%s")
		last = verb.start + len(verb.spec)
		plan.wrap[at] = helper
	}
	if len(plan.wrap) == 0 {
		return
	}
	out.WriteString(format[last:])
	plan.format = out.String()
	f.rewrites[call] = plan
}

func (p *goSyntaxCall) apply(c astCopier, call *ast.CallExpr) *ast.CallExpr {
	out := &ast.CallExpr{
		Fun:    c.copyNode(call.Fun).(ast.Expr),
		Lparen: call.Lparen,
		Rparen: call.Rparen,
	}
	for i, arg := range call.Args {
		var copied ast.Expr
		if i == p.formatArg {
			copied = &ast.BasicLit{ValuePos: arg.Pos(), Kind: token.STRING, Value: strconv.Quote(p.format)}
		} else {
			copied = c.copyNode(arg).(ast.Expr)
		}
		if helper, ok := p.wrap[i]; ok {
			copied = &ast.CallExpr{Fun: &ast.Ident{NamePos: arg.Pos(), Name: helper}, Lparen: arg.Pos(), Args: []ast.Expr{copied}, Rparen: arg.End()}
		}
		out.Args = append(out.Args, copied)
	}
	return out
}

// goFormatVerb is one verb of a format: its text, where it starts and the
// index of the argument it formats.
type goFormatVerb struct {
	spec  string
	start int
	arg   int
}

// goFormatVerbs lists the verbs of format. ok is false for a format with
// explicit argument indexes, which goFormatVerbs does not follow.
func goFormatVerbs(format string) ([]goFormatVerb, bool) {
	var verbs []goFormatVerb
	arg := 0
	for i := 0; i < len(format); i++ {
		if format[i] != '%' {
			continue
		}
		start := i
		i++
		for i < len(format) && strings.IndexByte("+-# 0", format[i]) >= 0 {
			i++
		}
		for i < len(format) && (format[i] == '*' || format[i] == '.' || (format[i] >= '0' && format[i] <= '9') || format[i] == '[') {
			switch format[i] {
			case '[':
				return nil, false
			case '*':
				arg++
			}
			i++
		}
		if i >= len(format) {
			break
		}
		if format[i] == '%' {
			continue
		}
		verbs = append(verbs, goFormatVerb{spec: format[start : i+1], start: start, arg: arg})
		arg++
	}
	return verbs, true
}

var goStringerInterface = func() *types.Interface {
	sig := types.NewSignatureType(nil, nil, nil, nil, types.NewTuple(types.NewVar(token.NoPos, nil, "", types.Typ[types.String])), false)
	iface := types.NewInterfaceType([]*types.Func{types.NewFunc(token.NoPos, nil, "GoString", sig)}, nil)
	iface.Complete()
	return iface
}()

var goFormatterMethod = "Format"

// helperFor returns the helper that writes a value of type t in Go syntax,
// as the top-level argument of a %#v when top is set. ok is false when the
// file cannot name t or no helper writes it.
func (f *goSyntaxFile) helperFor(t types.Type, top bool) (string, bool) {
	if t == nil || goSyntaxHasTypeParam(t) {
		return "", false
	}
	if _, isBasic := t.(*types.Basic); isBasic && t.(*types.Basic).Info()&types.IsUntyped != 0 {
		return "", false
	}
	typeExpr, localAt, ok := f.typeExprScoped(t)
	if !ok || !f.supported(t) {
		return "", false
	}
	if !top || !goSyntaxTopDiffers(t) {
		top = false
	}
	if localAt != token.NoPos {
		return f.localHelperFor(t, top, typeExpr, localAt)
	}
	key := typeExpr
	if top {
		key += " (top)"
	}
	if h, ok := f.byKey[key]; ok {
		return h.name, true
	}
	h := &goSyntaxHelper{name: fmt.Sprintf("__go2rust_gosyntax%d_%d", f.index, len(f.helpers)+1)}
	f.byKey[key] = h
	f.helpers = append(f.helpers, h)
	var body strings.Builder
	f.writeBody(&body, t, top)
	h.source = fmt.Sprintf("func %s(v %s) string {\n%s}\n", h.name, typeExpr, body.String())
	return h.name, true
}

// goSyntaxTopDiffers reports whether a top-level argument of type t is
// written differently from a nested value: a pointer at the top writes &
// and what it points at, and a nil interface writes <nil>.
func goSyntaxTopDiffers(t types.Type) bool {
	switch t.Underlying().(type) {
	case *types.Pointer, *types.Interface:
		return true
	}
	return false
}

// supported reports whether a helper writes a value of type t.
func (f *goSyntaxFile) supported(t types.Type) bool {
	if goSyntaxHasFormat(t) {
		return false
	}
	switch u := t.Underlying().(type) {
	case *types.Basic:
		return u.Kind() != types.UnsafePointer && u.Kind() != types.Invalid
	case *types.Struct:
		for i := 0; i < u.NumFields(); i++ {
			field := u.Field(i)
			if field.Name() == "_" || (!field.Exported() && field.Pkg() != f.r.pkg) {
				return false
			}
		}
		return true
	case *types.Map:
		if _, basic := u.Key().Underlying().(*types.Basic); !basic {
			// The compare helper of a key is a package function.
			if _, ok := f.typeExpr(u.Key()); !ok {
				return false
			}
		}
		return goSyntaxSortableKey(u.Key(), 0)
	case *types.Slice, *types.Array, *types.Pointer, *types.Interface, *types.Chan, *types.Signature:
		return true
	}
	return false
}

// goSyntaxSortableKey reports whether fmt's order of map keys of type t is
// one a compare helper writes: that of numbers, strings and bools, and of
// structs and arrays of them field by field.
func goSyntaxSortableKey(t types.Type, depth int) bool {
	if depth > 8 {
		return false
	}
	switch u := t.Underlying().(type) {
	case *types.Basic:
		return u.Info()&(types.IsOrdered|types.IsBoolean) != 0 && u.Info()&types.IsComplex == 0
	case *types.Struct:
		for i := 0; i < u.NumFields(); i++ {
			if u.Field(i).Name() == "_" || !goSyntaxSortableKey(u.Field(i).Type(), depth+1) {
				return false
			}
		}
		return true
	case *types.Array:
		return goSyntaxSortableKey(u.Elem(), depth+1)
	}
	return false
}

// compareFor returns the helper that orders two map keys of type t as fmt
// sorts them: -1, 0 or 1.
func (f *goSyntaxFile) compareFor(t types.Type) string {
	typeExpr, _ := f.typeExpr(t)
	key := "compare " + typeExpr
	if h, ok := f.byKey[key]; ok {
		return h.name
	}
	h := &goSyntaxHelper{name: fmt.Sprintf("__go2rust_gosyntaxcmp%d_%d", f.index, len(f.helpers)+1)}
	f.byKey[key] = h
	f.helpers = append(f.helpers, h)
	var b strings.Builder
	switch u := t.Underlying().(type) {
	case *types.Basic:
		if u.Info()&types.IsBoolean != 0 {
			b.WriteString("\tif a == b {\n\t\treturn 0\n\t}\n\tif !a {\n\t\treturn -1\n\t}\n\treturn 1\n")
		} else {
			b.WriteString("\tif a < b {\n\t\treturn -1\n\t}\n\tif a > b {\n\t\treturn 1\n\t}\n\treturn 0\n")
		}
	case *types.Struct:
		for i := 0; i < u.NumFields(); i++ {
			name := u.Field(i).Name()
			fmt.Fprintf(&b, "\tif c := %s(a.%s, b.%s); c != 0 {\n\t\treturn c\n\t}\n", f.compareFor(u.Field(i).Type()), name, name)
		}
		b.WriteString("\treturn 0\n")
	case *types.Array:
		fmt.Fprintf(&b, "\tfor i := 0; i < len(a); i++ {\n\t\tif c := %s(a[i], b[i]); c != 0 {\n\t\t\treturn c\n\t\t}\n\t}\n\treturn 0\n", f.compareFor(u.Elem()))
	}
	h.source = fmt.Sprintf("func %s(a, b %s) int {\n%s}\n", h.name, typeExpr, b.String())
	return h.name
}

func goSyntaxHasFormat(t types.Type) bool {
	obj, _, _ := types.LookupFieldOrMethod(t, false, nil, goFormatterMethod)
	fn, ok := obj.(*types.Func)
	if !ok {
		return false
	}
	sig, ok := fn.Type().(*types.Signature)
	return ok && sig.Params().Len() == 2 && sig.Results().Len() == 0
}

func goSyntaxHasTypeParam(t types.Type) bool {
	found := false
	var walk func(types.Type, int)
	walk = func(t types.Type, depth int) {
		if found || t == nil || depth > 8 {
			return
		}
		switch x := t.(type) {
		case *types.TypeParam:
			found = true
		case *types.Named:
			if x.TypeArgs() != nil {
				for i := 0; i < x.TypeArgs().Len(); i++ {
					walk(x.TypeArgs().At(i), depth+1)
				}
			}
			if x.TypeParams() != nil && x.TypeParams().Len() > 0 && x.TypeArgs() == nil {
				found = true
			}
		case *types.Alias:
			walk(types.Unalias(x), depth+1)
		case *types.Pointer:
			walk(x.Elem(), depth+1)
		case *types.Slice:
			walk(x.Elem(), depth+1)
		case *types.Array:
			walk(x.Elem(), depth+1)
		case *types.Map:
			walk(x.Key(), depth+1)
			walk(x.Elem(), depth+1)
		case *types.Chan:
			walk(x.Elem(), depth+1)
		case *types.Struct:
			for i := 0; i < x.NumFields(); i++ {
				walk(x.Field(i).Type(), depth+1)
			}
		case *types.Signature:
			for i := 0; i < x.Params().Len(); i++ {
				walk(x.Params().At(i).Type(), depth+1)
			}
			for i := 0; i < x.Results().Len(); i++ {
				walk(x.Results().At(i).Type(), depth+1)
			}
		}
	}
	walk(t, 0)
	return found
}

// typeExpr writes t as the file writes a type, or reports that it cannot:
// a type declared in a function, or one of a package the file does not
// import or does not export.
func (f *goSyntaxFile) typeExpr(t types.Type) (string, bool) {
	text, localAt, ok := f.typeExprScoped(t)
	return text, ok && localAt == token.NoPos
}

// typeExprScoped is typeExpr that also writes a type declared in a function
// of the package, and gives where the last one t mentions is declared:
// only code after that declaration can name t.
func (f *goSyntaxFile) typeExprScoped(t types.Type) (string, token.Pos, bool) {
	ok := true
	localAt := token.NoPos
	var check func(types.Type, int)
	check = func(t types.Type, depth int) {
		if !ok || t == nil || depth > 16 {
			return
		}
		switch x := t.(type) {
		case *types.Named:
			obj := x.Obj()
			if obj.Pkg() == nil {
				return
			}
			if obj.Parent() != obj.Pkg().Scope() {
				if obj.Pkg() != f.r.pkg {
					ok = false
				}
				if obj.Pos() > localAt {
					localAt = obj.Pos()
				}
				return
			}
			if obj.Pkg() != f.r.pkg {
				if !obj.Exported() {
					ok = false
					return
				}
				if _, imported := f.imports[obj.Pkg().Path()]; !imported {
					ok = false
					return
				}
			}
			if x.TypeArgs() != nil {
				for i := 0; i < x.TypeArgs().Len(); i++ {
					check(x.TypeArgs().At(i), depth+1)
				}
			}
		case *types.Alias:
			obj := x.Obj()
			if obj.Pkg() != nil && obj.Pkg() != f.r.pkg {
				if _, imported := f.imports[obj.Pkg().Path()]; !imported || !obj.Exported() {
					check(types.Unalias(x), depth+1)
					return
				}
			}
			if obj.Pkg() != nil && obj.Parent() != obj.Pkg().Scope() && obj.Pos() > localAt {
				localAt = obj.Pos()
			}
		case *types.Pointer:
			check(x.Elem(), depth+1)
		case *types.Slice:
			check(x.Elem(), depth+1)
		case *types.Array:
			check(x.Elem(), depth+1)
		case *types.Map:
			check(x.Key(), depth+1)
			check(x.Elem(), depth+1)
		case *types.Chan:
			check(x.Elem(), depth+1)
		case *types.Struct:
			for i := 0; i < x.NumFields(); i++ {
				field := x.Field(i)
				if field.Pkg() != nil && field.Pkg() != f.r.pkg && !field.Exported() {
					ok = false
				}
				check(field.Type(), depth+1)
			}
		case *types.Signature:
			for i := 0; i < x.Params().Len(); i++ {
				check(x.Params().At(i).Type(), depth+1)
			}
			for i := 0; i < x.Results().Len(); i++ {
				check(x.Results().At(i).Type(), depth+1)
			}
		case *types.Interface:
			for i := 0; i < x.NumMethods(); i++ {
				m := x.Method(i)
				if m.Pkg() != nil && m.Pkg() != f.r.pkg && !m.Exported() {
					ok = false
					return
				}
				check(m.Type(), depth+1)
			}
		}
	}
	check(t, 0)
	if !ok {
		return "", token.NoPos, false
	}
	text := types.TypeString(t, func(p *types.Package) string {
		if p == f.r.pkg {
			return ""
		}
		return f.imports[p.Path()]
	})
	return text, localAt, true
}

// localHelperFor returns the helper of t, which mentions a type declared in
// a function: a function value declared just after the declaration of the
// type declared last, which the call can name. ok is false for a type whose
// helper would call itself or whose declaration is not a statement of a
// block.
func (f *goSyntaxFile) localHelperFor(t types.Type, top bool, typeExpr string, localAt token.Pos) (string, bool) {
	decl, ok := f.localTypeDecls()[localAt]
	if !ok {
		return "", false
	}
	key := typeExpr + "@" + strconv.Itoa(int(localAt))
	if top {
		key += " (top)"
	}
	if h, ok := f.byKey[key]; ok {
		if h.source == "" {
			return "", false
		}
		return h.name, true
	}
	h := &goSyntaxHelper{name: fmt.Sprintf("__go2rust_gosyntaxL%d_%d", f.index, len(f.byKey)+1)}
	f.byKey[key] = h
	var body strings.Builder
	f.writeBody(&body, t, top)
	h.source = fmt.Sprintf("%s := func(v %s) string {\n%s}\n_ = %s", h.name, typeExpr, body.String(), h.name)
	f.inserts[decl.Pos()] = append(f.inserts[decl.Pos()], h)
	return h.name, true
}

// localTypeDecls maps the position of each type name declared in a
// function of the file to the statement that declares it.
func (f *goSyntaxFile) localTypeDecls() map[token.Pos]*ast.DeclStmt {
	if f.localDecls != nil {
		return f.localDecls
	}
	f.localDecls = map[token.Pos]*ast.DeclStmt{}
	ast.Inspect(f.file, func(n ast.Node) bool {
		decl, ok := n.(*ast.DeclStmt)
		if !ok {
			return true
		}
		if gen, ok := decl.Decl.(*ast.GenDecl); ok && gen.Tok == token.TYPE {
			for _, spec := range gen.Specs {
				if ts, ok := spec.(*ast.TypeSpec); ok {
					f.localDecls[ts.Name.Pos()] = decl
				}
			}
		}
		return true
	})
	return f.localDecls
}

// insertLocalHelpers puts a placeholder call after each declaration
// statement a local helper follows, which the helper's source replaces
// once the file is printed.
func (f *goSyntaxFile) insertLocalHelpers(file *ast.File) {
	if len(f.inserts) == 0 {
		return
	}
	splice := func(list []ast.Stmt) []ast.Stmt {
		var out []ast.Stmt
		for _, stmt := range list {
			out = append(out, stmt)
			decl, ok := stmt.(*ast.DeclStmt)
			if !ok {
				continue
			}
			for _, h := range f.inserts[decl.Pos()] {
				at := decl.End()
				out = append(out, &ast.ExprStmt{X: &ast.CallExpr{Fun: &ast.Ident{NamePos: at, Name: h.name + "__insert"}, Lparen: at, Rparen: at}})
			}
		}
		return out
	}
	ast.Inspect(file, func(n ast.Node) bool {
		switch x := n.(type) {
		case *ast.BlockStmt:
			x.List = splice(x.List)
		case *ast.CaseClause:
			x.Body = splice(x.Body)
		case *ast.CommClause:
			x.Body = splice(x.Body)
		}
		return true
	})
}

// writeBody writes the statements of the helper of t.
func (f *goSyntaxFile) writeBody(b *strings.Builder, t types.Type, top bool) {
	name := goReflectTypeString(t)
	fmtName := f.fmtName
	if types.Implements(t, goStringerInterface) {
		if ptr, ok := t.Underlying().(*types.Pointer); ok && !types.Implements(ptr.Elem(), goStringerInterface) {
			fmt.Fprintf(b, "\treturn v.GoString()\n")
			return
		}
		if _, ok := t.Underlying().(*types.Pointer); ok {
			b.WriteString("\tif v == nil {\n\t\treturn \"<nil>\"\n\t}\n")
		}
		b.WriteString("\treturn v.GoString()\n")
		return
	}
	switch u := t.Underlying().(type) {
	case *types.Basic:
		info := u.Info()
		switch {
		case info&types.IsBoolean != 0:
			fmt.Fprintf(b, "\tif v {\n\t\treturn \"true\"\n\t}\n\treturn \"false\"\n")
		case info&types.IsUnsigned != 0:
			fmt.Fprintf(b, "\treturn %s.Sprintf(\"0x%%x\", uint64(v))\n", fmtName)
		case info&types.IsInteger != 0:
			fmt.Fprintf(b, "\treturn %s.Sprint(int64(v))\n", fmtName)
		case info&types.IsFloat != 0, info&types.IsComplex != 0:
			fmt.Fprintf(b, "\treturn %s.Sprint(%s(v))\n", fmtName, u.Name())
		case info&types.IsString != 0:
			fmt.Fprintf(b, "\treturn %s.Sprintf(\"%%q\", string(v))\n", fmtName)
		default:
			fmt.Fprintf(b, "\treturn %s.Sprint(v)\n", fmtName)
		}
	case *types.Struct:
		fmt.Fprintf(b, "\ts := %s\n", strconv.Quote(name+"{"))
		for i := 0; i < u.NumFields(); i++ {
			field := u.Field(i)
			sep := ""
			if i > 0 {
				sep = ", "
			}
			fmt.Fprintf(b, "\ts += %s + %s\n", strconv.Quote(sep+field.Name()+":"), f.render(field.Type(), "v."+field.Name()))
		}
		b.WriteString("\treturn s + \"}\"\n")
	case *types.Slice, *types.Array:
		var elem types.Type
		if s, ok := u.(*types.Slice); ok {
			elem = s.Elem()
			fmt.Fprintf(b, "\tif v == nil {\n\t\treturn %s\n\t}\n", strconv.Quote(name+"(nil)"))
		} else {
			elem = u.(*types.Array).Elem()
		}
		fmt.Fprintf(b, "\ts := %s\n", strconv.Quote(name+"{"))
		fmt.Fprintf(b, "\tfor i := 0; i < len(v); i++ {\n\t\tif i > 0 {\n\t\t\ts += \", \"\n\t\t}\n\t\ts += %s\n\t}\n", f.render(elem, "v[i]"))
		b.WriteString("\treturn s + \"}\"\n")
	case *types.Map:
		keyType, _, _ := f.typeExprScoped(u.Key())
		fmt.Fprintf(b, "\tif v == nil {\n\t\treturn %s\n\t}\n", strconv.Quote(name+"(nil)"))
		fmt.Fprintf(b, "\tkeys := make([]%s, 0, len(v))\n\tfor k := range v {\n\t\tkeys = append(keys, k)\n\t}\n", keyType)
		less := "keys[j] < keys[j-1]"
		if basic, ok := u.Key().Underlying().(*types.Basic); !ok || basic.Info()&types.IsBoolean != 0 {
			less = f.compareFor(u.Key()) + "(keys[j], keys[j-1]) < 0"
		}
		fmt.Fprintf(b, "\tfor i := 1; i < len(keys); i++ {\n\t\tfor j := i; j > 0 && %s; j-- {\n\t\t\tkeys[j], keys[j-1] = keys[j-1], keys[j]\n\t\t}\n\t}\n", less)
		fmt.Fprintf(b, "\ts := %s\n", strconv.Quote(name+"{"))
		fmt.Fprintf(b, "\tfor i := 0; i < len(keys); i++ {\n\t\tif i > 0 {\n\t\t\ts += \", \"\n\t\t}\n\t\ts += %s + \":\" + %s\n\t}\n", f.render(u.Key(), "keys[i]"), f.render(u.Elem(), "v[keys[i]]"))
		b.WriteString("\treturn s + \"}\"\n")
	case *types.Pointer:
		fmt.Fprintf(b, "\tif v == nil {\n\t\treturn %s\n\t}\n", strconv.Quote("("+name+")(nil)"))
		if top {
			switch u.Elem().Underlying().(type) {
			case *types.Struct, *types.Array, *types.Slice, *types.Map:
				fmt.Fprintf(b, "\treturn \"&\" + %s\n", f.render(u.Elem(), "*v"))
				return
			}
		}
		fmt.Fprintf(b, "\treturn %s + %s.Sprintf(\"%%p\", v) + \")\"\n", strconv.Quote("("+name+")("), fmtName)
	case *types.Chan, *types.Signature:
		fmt.Fprintf(b, "\tif v == nil {\n\t\treturn %s\n\t}\n", strconv.Quote("("+name+")(nil)"))
		fmt.Fprintf(b, "\treturn %s\n", strconv.Quote("("+name+")(0xc000010000)"))
	case *types.Interface:
		if top {
			b.WriteString("\tif v == nil {\n\t\treturn \"<nil>\"\n\t}\n")
		} else {
			fmt.Fprintf(b, "\tif v == nil {\n\t\treturn %s\n\t}\n", strconv.Quote(name+"(nil)"))
		}
		var cases strings.Builder
		var seen []types.Type
		for _, flow := range f.r.flows {
			if !types.AssignableTo(flow, t) || goSyntaxSeen(seen, flow) {
				continue
			}
			typeExpr, ok := f.typeExpr(flow)
			if !ok || !f.supported(flow) {
				continue
			}
			seen = append(seen, flow)
			helper, ok := f.helperFor(flow, top)
			if !ok {
				continue
			}
			fmt.Fprintf(&cases, "\tcase %s:\n\t\treturn %s(x)\n", typeExpr, helper)
		}
		if cases.Len() > 0 {
			b.WriteString("\tswitch x := v.(type) {\n")
			b.WriteString(cases.String())
			b.WriteString("\t}\n")
		}
		fmt.Fprintf(b, "\treturn %s.Sprintf(\"%%v\", v)\n", fmtName)
	default:
		fmt.Fprintf(b, "\treturn %s.Sprintf(\"%%v\", v)\n", fmtName)
	}
}

func goSyntaxSeen(seen []types.Type, t types.Type) bool {
	for _, s := range seen {
		if types.Identical(s, t) {
			return true
		}
	}
	return false
}

// render writes the Go expression that writes x, a nested value of type t.
func (f *goSyntaxFile) render(t types.Type, x string) string {
	if helper, ok := f.helperFor(t, false); ok {
		return helper + "(" + x + ")"
	}
	return f.fmtName + ".Sprintf(\"%v\", " + x + ")"
}

// goReflectTypeString writes t as reflect's Type.String does: package names
// rather than paths, byte as uint8 and rune as int32, interface {} for any.
func goReflectTypeString(t types.Type) string {
	var b strings.Builder
	goWriteReflectType(&b, t, 0)
	return b.String()
}

func goWriteReflectType(b *strings.Builder, t types.Type, depth int) {
	if depth > 16 {
		b.WriteString("...")
		return
	}
	switch x := t.(type) {
	case *types.Alias:
		goWriteReflectType(b, types.Unalias(x), depth)
	case *types.Named:
		obj := x.Obj()
		if obj.Pkg() != nil {
			b.WriteString(obj.Pkg().Name())
			b.WriteString(".")
		}
		if name, ok := monoTypeGoNames[obj.Name()]; ok && obj.Pkg() != nil && obj.Parent() == obj.Pkg().Scope() {
			b.WriteString(name)
			return
		}
		b.WriteString(obj.Name())
		if args := x.TypeArgs(); args != nil && args.Len() > 0 {
			b.WriteString("[")
			for i := 0; i < args.Len(); i++ {
				if i > 0 {
					b.WriteString(",")
				}
				goWriteReflectType(b, args.At(i), depth+1)
			}
			b.WriteString("]")
		}
	case *types.Basic:
		switch x.Kind() {
		case types.Byte:
			b.WriteString("uint8")
		case types.Rune:
			b.WriteString("int32")
		default:
			b.WriteString(x.Name())
		}
	case *types.Pointer:
		b.WriteString("*")
		goWriteReflectType(b, x.Elem(), depth+1)
	case *types.Slice:
		b.WriteString("[]")
		goWriteReflectType(b, x.Elem(), depth+1)
	case *types.Array:
		fmt.Fprintf(b, "[%d]", x.Len())
		goWriteReflectType(b, x.Elem(), depth+1)
	case *types.Map:
		b.WriteString("map[")
		goWriteReflectType(b, x.Key(), depth+1)
		b.WriteString("]")
		goWriteReflectType(b, x.Elem(), depth+1)
	case *types.Chan:
		switch x.Dir() {
		case types.SendOnly:
			b.WriteString("chan<- ")
		case types.RecvOnly:
			b.WriteString("<-chan ")
		default:
			b.WriteString("chan ")
		}
		goWriteReflectType(b, x.Elem(), depth+1)
	case *types.Struct:
		if x.NumFields() == 0 {
			b.WriteString("struct {}")
			return
		}
		b.WriteString("struct { ")
		for i := 0; i < x.NumFields(); i++ {
			if i > 0 {
				b.WriteString("; ")
			}
			field := x.Field(i)
			if !field.Embedded() {
				b.WriteString(field.Name())
				b.WriteString(" ")
			}
			goWriteReflectType(b, field.Type(), depth+1)
			if tag := x.Tag(i); tag != "" {
				b.WriteString(" ")
				b.WriteString(strconv.Quote(tag))
			}
		}
		b.WriteString(" }")
	case *types.Signature:
		b.WriteString("func")
		goWriteReflectSignature(b, x, depth)
	case *types.Interface:
		if x.NumMethods() == 0 {
			b.WriteString("interface {}")
			return
		}
		b.WriteString("interface { ")
		for i := 0; i < x.NumMethods(); i++ {
			if i > 0 {
				b.WriteString("; ")
			}
			m := x.Method(i)
			b.WriteString(m.Name())
			goWriteReflectSignature(b, m.Type().(*types.Signature), depth)
		}
		b.WriteString(" }")
	default:
		b.WriteString(t.String())
	}
}

func goWriteReflectSignature(b *strings.Builder, sig *types.Signature, depth int) {
	b.WriteString("(")
	for i := 0; i < sig.Params().Len(); i++ {
		if i > 0 {
			b.WriteString(", ")
		}
		pt := sig.Params().At(i).Type()
		if sig.Variadic() && i == sig.Params().Len()-1 {
			b.WriteString("...")
			if s, ok := pt.(*types.Slice); ok {
				pt = s.Elem()
			}
		}
		goWriteReflectType(b, pt, depth+1)
	}
	b.WriteString(")")
	switch sig.Results().Len() {
	case 0:
	case 1:
		b.WriteString(" ")
		goWriteReflectType(b, sig.Results().At(0).Type(), depth+1)
	default:
		b.WriteString(" (")
		for i := 0; i < sig.Results().Len(); i++ {
			if i > 0 {
				b.WriteString(", ")
			}
			goWriteReflectType(b, sig.Results().At(i).Type(), depth+1)
		}
		b.WriteString(")")
	}
}

// goSyntaxInterfaceFlows lists the concrete types the package converts to
// an interface: the dynamic types an interface value may hold. A value
// passed to fmt or log is printed rather than kept, so those calls do not
// count.
func goSyntaxInterfaceFlows(files []*ast.File, info *types.Info) []types.Type {
	var flows []types.Type
	add := func(e ast.Expr, target types.Type) {
		if e == nil || target == nil {
			return
		}
		if _, ok := target.Underlying().(*types.Interface); !ok {
			return
		}
		tv, ok := info.Types[e]
		if !ok || tv.Type == nil || tv.IsNil() || tv.IsType() {
			return
		}
		t := types.Default(tv.Type)
		if _, ok := t.Underlying().(*types.Interface); ok {
			return
		}
		if b, ok := t.(*types.Basic); ok && b.Info()&types.IsUntyped != 0 {
			return
		}
		if goSyntaxHasTypeParam(t) || goSyntaxSeen(flows, t) {
			return
		}
		flows = append(flows, t)
	}
	typeOf := func(e ast.Expr) types.Type {
		if tv, ok := info.Types[e]; ok {
			return tv.Type
		}
		return nil
	}
	for _, file := range files {
		var stack []ast.Node
		var sigs []*types.Signature
		ast.Inspect(file, func(n ast.Node) bool {
			if n == nil {
				top := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				switch top.(type) {
				case *ast.FuncDecl, *ast.FuncLit:
					sigs = sigs[:len(sigs)-1]
				}
				return true
			}
			stack = append(stack, n)
			switch x := n.(type) {
			case *ast.FuncDecl:
				var sig *types.Signature
				if fn, ok := info.Defs[x.Name].(*types.Func); ok {
					sig, _ = fn.Type().(*types.Signature)
				}
				sigs = append(sigs, sig)
			case *ast.FuncLit:
				sig, _ := typeOf(x).(*types.Signature)
				sigs = append(sigs, sig)
			case *ast.ReturnStmt:
				if len(sigs) > 0 && sigs[len(sigs)-1] != nil {
					results := sigs[len(sigs)-1].Results()
					if results.Len() == len(x.Results) {
						for i, e := range x.Results {
							add(e, results.At(i).Type())
						}
					}
				}
			case *ast.AssignStmt:
				if x.Tok == token.ASSIGN && len(x.Lhs) == len(x.Rhs) {
					for i := range x.Lhs {
						add(x.Rhs[i], typeOf(x.Lhs[i]))
					}
				}
			case *ast.ValueSpec:
				if x.Type != nil && len(x.Values) == len(x.Names) {
					for _, e := range x.Values {
						add(e, typeOf(x.Type))
					}
				}
			case *ast.SendStmt:
				if ch, ok := typeOf(x.Chan).(*types.Chan); ok {
					add(x.Value, ch.Elem())
				} else if t := typeOf(x.Chan); t != nil {
					if ch, ok := t.Underlying().(*types.Chan); ok {
						add(x.Value, ch.Elem())
					}
				}
			case *ast.CompositeLit:
				t := typeOf(x)
				if t == nil {
					break
				}
				if p, ok := t.Underlying().(*types.Pointer); ok {
					t = p.Elem()
				}
				switch u := t.Underlying().(type) {
				case *types.Struct:
					for i, elt := range x.Elts {
						if kv, ok := elt.(*ast.KeyValueExpr); ok {
							if key, ok := kv.Key.(*ast.Ident); ok {
								for j := 0; j < u.NumFields(); j++ {
									if u.Field(j).Name() == key.Name {
										add(kv.Value, u.Field(j).Type())
									}
								}
							}
						} else if i < u.NumFields() {
							add(elt, u.Field(i).Type())
						}
					}
				case *types.Slice, *types.Array:
					var elem types.Type
					if s, ok := u.(*types.Slice); ok {
						elem = s.Elem()
					} else {
						elem = u.(*types.Array).Elem()
					}
					for _, elt := range x.Elts {
						if kv, ok := elt.(*ast.KeyValueExpr); ok {
							elt = kv.Value
						}
						add(elt, elem)
					}
				case *types.Map:
					for _, elt := range x.Elts {
						if kv, ok := elt.(*ast.KeyValueExpr); ok {
							add(kv.Key, u.Key())
							add(kv.Value, u.Elem())
						}
					}
				}
			case *ast.CallExpr:
				goSyntaxCallFlows(info, x, add)
			}
			return true
		})
	}
	return flows
}

func goSyntaxCallFlows(info *types.Info, call *ast.CallExpr, add func(ast.Expr, types.Type)) {
	fun, ok := info.Types[call.Fun]
	if !ok {
		return
	}
	if fun.IsType() {
		if len(call.Args) == 1 {
			add(call.Args[0], fun.Type)
		}
		return
	}
	if fun.IsBuiltin() {
		id, ok := ast.Unparen(call.Fun).(*ast.Ident)
		if !ok || id.Name != "append" || call.Ellipsis != token.NoPos || len(call.Args) < 2 {
			return
		}
		if s, ok := info.Types[call].Type.Underlying().(*types.Slice); ok {
			for _, arg := range call.Args[1:] {
				add(arg, s.Elem())
			}
		}
		return
	}
	var obj types.Object
	switch f := ast.Unparen(call.Fun).(type) {
	case *ast.Ident:
		obj = info.Uses[f]
	case *ast.SelectorExpr:
		if sel := info.Selections[f]; sel != nil {
			obj = sel.Obj()
		} else {
			obj = info.Uses[f.Sel]
		}
	}
	if obj != nil && obj.Pkg() != nil && (obj.Pkg().Path() == "fmt" || obj.Pkg().Path() == "log") {
		return
	}
	sig, ok := fun.Type.Underlying().(*types.Signature)
	if !ok {
		return
	}
	params := sig.Params()
	for i, arg := range call.Args {
		var pt types.Type
		switch {
		case sig.Variadic() && i >= params.Len()-1:
			if call.Ellipsis != token.NoPos {
				continue
			}
			if s, ok := params.At(params.Len() - 1).Type().(*types.Slice); ok {
				pt = s.Elem()
			}
		case i < params.Len():
			pt = params.At(i).Type()
		}
		add(arg, pt)
	}
}
