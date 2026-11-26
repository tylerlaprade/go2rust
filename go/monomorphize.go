package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"go/types"
	"os"
	"reflect"
	"sort"
	"strconv"
	"strings"
)

// Generic functions and types are written once per instantiation.
//
// A generic Go declaration is copied for every list of type arguments the
// program uses it with, with the type parameters replaced by those types,
// and the copies are type-checked again in place of the generic one. Map[T,
// U] called with string and int becomes Map__string__int, whose []T is a
// []string and is written as the Vec<String> that the caller's []string is.
// A Rust generic over T has to store a T the same way for every T, and the
// program stores an int, a string and a pointer in three different ways, so
// a slice, map or field of T did not line up with the caller's (DEV-69).
//
// The rewrite is all or nothing. When a type argument cannot be written at
// package level (a type declared inside a function), when an instantiation
// recurses without end, or when the rewritten package does not type-check,
// the program is left as it was.

const maxMonomorphizedInstances = 500

type monoInstance struct {
	key      string
	name     string
	obj      types.Object
	typeArgs []types.Type
}

type monomorphizer struct {
	path  string
	imp   types.Importer
	fset  *token.FileSet
	files []*ast.File
	info  *types.Info
	pkg   *types.Package

	genericFuncs   map[*types.Func]*ast.FuncDecl
	genericTypes   map[*types.TypeName]*ast.TypeSpec
	genericMethods map[*types.TypeName][]*ast.FuncDecl
	declFile       map[ast.Node]*ast.File

	instances  map[string]*monoInstance
	byObj      map[types.Object][]*monoInstance
	usedNames  map[string]string
	queue      []*monoInstance
	extraTypes map[*ast.File]map[string]string // file -> import path -> name
	err        error
}

// monomorphizeGenerics returns the files with every generic declaration of
// the package replaced by its instantiations, or false when there is
// nothing to do or the rewrite does not apply.
func monomorphizeGenerics(files []*ast.File, fset *token.FileSet) ([]*ast.File, bool) {
	return monomorphizeGenericsWith("", files, fset, nil)
}

// monomorphizeGenericsWith is monomorphizeGenerics for a package with the
// import path path, whose imports imp resolves.
func monomorphizeGenericsWith(path string, files []*ast.File, fset *token.FileSet, imp types.Importer) ([]*ast.File, bool) {
	// GO2RUST_GENERIC_RUST=1 keeps generic declarations as Rust generics,
	// the code path a program takes when this rewrite does not apply.
	if os.Getenv("GO2RUST_GENERIC_RUST") == "1" || !filesDeclareGenerics(files) {
		return nil, false
	}
	first, err := NewTypeInfoWithImporter(path, files, fset, imp)
	if err != nil || first == nil || first.info == nil {
		return nil, false
	}
	m := &monomorphizer{
		path:           path,
		imp:            imp,
		fset:           fset,
		files:          files,
		info:           first.info,
		pkg:            first.pkg,
		genericFuncs:   map[*types.Func]*ast.FuncDecl{},
		genericTypes:   map[*types.TypeName]*ast.TypeSpec{},
		genericMethods: map[*types.TypeName][]*ast.FuncDecl{},
		declFile:       map[ast.Node]*ast.File{},
		instances:      map[string]*monoInstance{},
		byObj:          map[types.Object][]*monoInstance{},
		usedNames:      map[string]string{},
		extraTypes:     map[*ast.File]map[string]string{},
	}
	if m.pkg == nil {
		return nil, false
	}
	m.collectGenericDecls()
	if len(m.genericFuncs) == 0 && len(m.genericTypes) == 0 {
		return nil, false
	}
	out, ok := m.rewrite()
	if !ok {
		if m.err != nil && os.Getenv("GO2RUST_DEBUG_MONO") != "" {
			fmt.Fprintf(os.Stderr, "Note: generics kept generic: %v\n", m.err)
		}
		return nil, false
	}
	for _, inst := range m.instances {
		if obj, ok := inst.obj.(*types.TypeName); ok {
			monoTypeGoNames[inst.name] = goReflectInstanceName(obj, inst.typeArgs)
		}
	}
	return out, true
}

// monoTypeGoNames maps the name of each instantiation of a generic type to
// the name reflect gives it, Pair[string,int], which %T and %#v write.
var monoTypeGoNames = map[string]string{}

func goReflectInstanceName(obj *types.TypeName, typeArgs []types.Type) string {
	var b strings.Builder
	b.WriteString(obj.Name())
	b.WriteString("[")
	for i, arg := range typeArgs {
		if i > 0 {
			b.WriteString(",")
		}
		goWriteReflectType(&b, arg, 1)
	}
	b.WriteString("]")
	return b.String()
}

func filesDeclareGenerics(files []*ast.File) bool {
	for _, file := range files {
		for _, decl := range file.Decls {
			switch d := decl.(type) {
			case *ast.FuncDecl:
				if d.Type.TypeParams != nil && len(d.Type.TypeParams.List) > 0 {
					return true
				}
			case *ast.GenDecl:
				for _, spec := range d.Specs {
					if ts, ok := spec.(*ast.TypeSpec); ok && ts.TypeParams != nil && len(ts.TypeParams.List) > 0 {
						return true
					}
				}
			}
		}
	}
	return false
}

func (m *monomorphizer) collectGenericDecls() {
	for _, file := range m.files {
		for _, decl := range file.Decls {
			switch d := decl.(type) {
			case *ast.FuncDecl:
				if d.Recv == nil {
					if d.Type.TypeParams == nil || len(d.Type.TypeParams.List) == 0 {
						continue
					}
					if fn, ok := m.info.Defs[d.Name].(*types.Func); ok {
						m.genericFuncs[fn] = d
						m.declFile[d] = file
					}
					continue
				}
				if typeName := m.receiverGenericType(d); typeName != nil {
					m.genericMethods[typeName] = append(m.genericMethods[typeName], d)
					m.declFile[d] = file
				}
			case *ast.GenDecl:
				for _, spec := range d.Specs {
					ts, ok := spec.(*ast.TypeSpec)
					if !ok || ts.TypeParams == nil || len(ts.TypeParams.List) == 0 {
						continue
					}
					if typeName, ok := m.info.Defs[ts.Name].(*types.TypeName); ok {
						m.genericTypes[typeName] = ts
						m.declFile[ts] = file
					}
				}
			}
		}
	}
	// A method of a generic type needs its type.
	for typeName := range m.genericMethods {
		if _, ok := m.genericTypes[typeName]; !ok {
			delete(m.genericMethods, typeName)
		}
	}
}

func (m *monomorphizer) receiverGenericType(decl *ast.FuncDecl) *types.TypeName {
	if decl.Recv == nil || len(decl.Recv.List) != 1 {
		return nil
	}
	expr := decl.Recv.List[0].Type
	if star, ok := expr.(*ast.StarExpr); ok {
		expr = star.X
	}
	var base ast.Expr
	switch e := expr.(type) {
	case *ast.IndexExpr:
		base = e.X
	case *ast.IndexListExpr:
		base = e.X
	default:
		return nil
	}
	ident, ok := base.(*ast.Ident)
	if !ok {
		return nil
	}
	typeName, _ := m.info.Uses[ident].(*types.TypeName)
	return typeName
}

func (m *monomorphizer) owns(obj types.Object) bool {
	switch o := obj.(type) {
	case *types.Func:
		_, ok := m.genericFuncs[o]
		return ok
	case *types.TypeName:
		_, ok := m.genericTypes[o]
		return ok
	}
	return false
}

func (m *monomorphizer) fail(format string, args ...any) {
	if m.err == nil {
		m.err = fmt.Errorf(format, args...)
	}
}

// instantiate returns the name of obj instantiated with typeArgs, which
// must not mention a type parameter, and queues the copy the first time.
func (m *monomorphizer) instantiate(obj types.Object, typeArgs []types.Type) string {
	var key strings.Builder
	key.WriteString(fmt.Sprintf("%p", obj))
	for _, arg := range typeArgs {
		if typeMentionsTypeParam(arg) {
			m.fail("instantiation of %s with a type parameter left: %s", obj.Name(), arg)
			return obj.Name()
		}
		if typeStoresSliceInSequence(arg) {
			// A slice inside an array or a slice is stored by value in
			// concrete code, which loses its sharing; the generic code
			// keeps it.
			m.fail("type argument %s stores a slice inside an array or slice", arg)
			return obj.Name()
		}
		key.WriteString("|")
		key.WriteString(types.TypeString(arg, nil))
	}
	if inst, ok := m.instances[key.String()]; ok {
		return inst.name
	}
	if len(m.instances) >= maxMonomorphizedInstances {
		m.fail("more than %d instantiations", maxMonomorphizedInstances)
		return obj.Name()
	}
	parts := make([]string, len(typeArgs))
	for i, arg := range typeArgs {
		parts[i] = m.mangleType(arg)
	}
	name := obj.Name() + "__" + strings.Join(parts, "__")
	for n := 2; ; n++ {
		if owner, taken := m.usedNames[name]; !taken || owner == key.String() {
			break
		}
		name = fmt.Sprintf("%s__%s_%d", obj.Name(), strings.Join(parts, "__"), n)
	}
	if m.pkg.Scope().Lookup(name) != nil {
		name += "_"
	}
	m.usedNames[name] = key.String()
	inst := &monoInstance{key: key.String(), name: name, obj: obj, typeArgs: typeArgs}
	m.instances[inst.key] = inst
	m.byObj[obj] = append(m.byObj[obj], inst)
	m.queue = append(m.queue, inst)
	return name
}

// typeStoresSliceInSequence reports whether t has a slice as the element of
// an array or slice, at any depth.
func typeStoresSliceInSequence(t types.Type) bool {
	var visit func(t types.Type, inSequence bool, depth int) bool
	visit = func(t types.Type, inSequence bool, depth int) bool {
		if depth > 20 {
			return false
		}
		switch u := types.Unalias(t).(type) {
		case *types.Slice:
			return inSequence || visit(u.Elem(), true, depth+1)
		case *types.Array:
			return visit(u.Elem(), true, depth+1)
		case *types.Named:
			if _, isSlice := u.Underlying().(*types.Slice); isSlice && inSequence {
				return true
			}
			if _, isArray := u.Underlying().(*types.Array); isArray {
				return visit(u.Underlying(), inSequence, depth+1)
			}
		case *types.Struct:
			for i := 0; i < u.NumFields(); i++ {
				if visit(u.Field(i).Type(), false, depth+1) {
					return true
				}
			}
		}
		return false
	}
	return visit(t, false, 0)
}

// mangleType writes a type as the part of a Go identifier that names it.
func (m *monomorphizer) mangleType(t types.Type) string {
	switch t := t.(type) {
	case *types.Basic:
		return strings.ReplaceAll(t.Name(), " ", "_")
	case *types.Alias:
		if t.Obj() != nil && t.Obj().Pkg() == nil {
			return t.Obj().Name()
		}
		return m.mangleType(types.Unalias(t))
	case *types.Named:
		name := t.Obj().Name()
		if pkg := t.Obj().Pkg(); pkg != nil && pkg != m.pkg {
			name = pkg.Name() + "_" + name
		}
		args := t.TypeArgs()
		for i := 0; args != nil && i < args.Len(); i++ {
			name += "_" + m.mangleType(args.At(i))
		}
		return name
	case *types.Pointer:
		return "ptr_" + m.mangleType(t.Elem())
	case *types.Slice:
		return "slice_" + m.mangleType(t.Elem())
	case *types.Array:
		return "array" + strconv.FormatInt(t.Len(), 10) + "_" + m.mangleType(t.Elem())
	case *types.Map:
		return "map_" + m.mangleType(t.Key()) + "_" + m.mangleType(t.Elem())
	case *types.Chan:
		return "chan_" + m.mangleType(t.Elem())
	case *types.Signature:
		return "func" + strconv.Itoa(t.Params().Len()) + "_" + strconv.Itoa(t.Results().Len())
	case *types.Interface:
		if t.Empty() {
			return "any"
		}
		return "iface"
	case *types.Struct:
		return "struct" + strconv.Itoa(t.NumFields())
	}
	return "t"
}

// substType replaces the type parameters of subst in t.
func (m *monomorphizer) substType(t types.Type, subst map[*types.TypeParam]types.Type) types.Type {
	if len(subst) == 0 || !typeMentionsTypeParam(t) {
		return t
	}
	switch t := t.(type) {
	case *types.TypeParam:
		if s, ok := subst[t]; ok {
			return s
		}
		return t
	case *types.Alias:
		return m.substType(types.Unalias(t), subst)
	case *types.Named:
		args := t.TypeArgs()
		newArgs := make([]types.Type, args.Len())
		for i := 0; i < args.Len(); i++ {
			newArgs[i] = m.substType(args.At(i), subst)
		}
		inst, err := types.Instantiate(nil, t.Origin(), newArgs, false)
		if err != nil {
			m.fail("instantiate %s: %v", t, err)
			return t
		}
		return inst
	case *types.Pointer:
		return types.NewPointer(m.substType(t.Elem(), subst))
	case *types.Slice:
		return types.NewSlice(m.substType(t.Elem(), subst))
	case *types.Array:
		return types.NewArray(m.substType(t.Elem(), subst), t.Len())
	case *types.Map:
		return types.NewMap(m.substType(t.Key(), subst), m.substType(t.Elem(), subst))
	case *types.Chan:
		return types.NewChan(t.Dir(), m.substType(t.Elem(), subst))
	case *types.Signature:
		return types.NewSignatureType(nil, nil, nil, m.substTuple(t.Params(), subst), m.substTuple(t.Results(), subst), t.Variadic())
	case *types.Struct:
		fields := make([]*types.Var, t.NumFields())
		tags := make([]string, t.NumFields())
		for i := 0; i < t.NumFields(); i++ {
			f := t.Field(i)
			fields[i] = types.NewField(f.Pos(), f.Pkg(), f.Name(), m.substType(f.Type(), subst), f.Embedded())
			tags[i] = t.Tag(i)
		}
		return types.NewStruct(fields, tags)
	}
	m.fail("cannot substitute type parameters in %s", t)
	return t
}

func (m *monomorphizer) substTuple(tuple *types.Tuple, subst map[*types.TypeParam]types.Type) *types.Tuple {
	if tuple == nil {
		return nil
	}
	vars := make([]*types.Var, tuple.Len())
	for i := 0; i < tuple.Len(); i++ {
		v := tuple.At(i)
		vars[i] = types.NewParam(v.Pos(), v.Pkg(), v.Name(), m.substType(v.Type(), subst))
	}
	return types.NewTuple(vars...)
}

// typeExpr writes t as a Go type expression in file.
func (m *monomorphizer) typeExpr(t types.Type, file *ast.File) ast.Expr {
	switch t := t.(type) {
	case *types.Basic:
		return ast.NewIdent(t.Name())
	case *types.Alias:
		if t.Obj() != nil && t.Obj().Pkg() == nil {
			return ast.NewIdent(t.Obj().Name())
		}
		return m.typeExpr(types.Unalias(t), file)
	case *types.Named:
		obj := t.Obj()
		if args := t.TypeArgs(); args != nil && args.Len() > 0 {
			typeArgs := make([]types.Type, args.Len())
			for i := 0; i < args.Len(); i++ {
				typeArgs[i] = args.At(i)
			}
			if m.owns(t.Origin().Obj()) {
				return ast.NewIdent(m.instantiate(t.Origin().Obj(), typeArgs))
			}
			base := m.namedTypeExpr(obj, file)
			indices := make([]ast.Expr, len(typeArgs))
			for i, arg := range typeArgs {
				indices[i] = m.typeExpr(arg, file)
			}
			if len(indices) == 1 {
				return &ast.IndexExpr{X: base, Index: indices[0]}
			}
			return &ast.IndexListExpr{X: base, Indices: indices}
		}
		return m.namedTypeExpr(obj, file)
	case *types.Pointer:
		return &ast.StarExpr{X: m.typeExpr(t.Elem(), file)}
	case *types.Slice:
		return &ast.ArrayType{Elt: m.typeExpr(t.Elem(), file)}
	case *types.Array:
		return &ast.ArrayType{Len: &ast.BasicLit{Kind: token.INT, Value: strconv.FormatInt(t.Len(), 10)}, Elt: m.typeExpr(t.Elem(), file)}
	case *types.Map:
		return &ast.MapType{Key: m.typeExpr(t.Key(), file), Value: m.typeExpr(t.Elem(), file)}
	case *types.Chan:
		dir := ast.SEND | ast.RECV
		switch t.Dir() {
		case types.SendOnly:
			dir = ast.SEND
		case types.RecvOnly:
			dir = ast.RECV
		}
		return &ast.ChanType{Dir: dir, Value: m.typeExpr(t.Elem(), file)}
	case *types.Signature:
		return m.funcTypeExpr(t, file)
	case *types.Struct:
		fields := &ast.FieldList{}
		for i := 0; i < t.NumFields(); i++ {
			f := t.Field(i)
			field := &ast.Field{Type: m.typeExpr(f.Type(), file)}
			if !f.Embedded() {
				field.Names = []*ast.Ident{ast.NewIdent(f.Name())}
			}
			if tag := t.Tag(i); tag != "" {
				field.Tag = &ast.BasicLit{Kind: token.STRING, Value: strconv.Quote(tag)}
			}
			fields.List = append(fields.List, field)
		}
		return &ast.StructType{Fields: fields}
	case *types.Interface:
		if t.Empty() {
			return ast.NewIdent("any")
		}
		if t.IsComparable() && t.NumMethods() == 0 {
			return ast.NewIdent("comparable")
		}
		methods := &ast.FieldList{}
		for i := 0; i < t.NumEmbeddeds(); i++ {
			methods.List = append(methods.List, &ast.Field{Type: m.typeExpr(t.EmbeddedType(i), file)})
		}
		for i := 0; i < t.NumExplicitMethods(); i++ {
			method := t.ExplicitMethod(i)
			sig, _ := method.Type().(*types.Signature)
			methods.List = append(methods.List, &ast.Field{Names: []*ast.Ident{ast.NewIdent(method.Name())}, Type: m.funcTypeExpr(sig, file)})
		}
		return &ast.InterfaceType{Methods: methods}
	case *types.TypeParam:
		m.fail("type parameter %s left in a type argument", t)
		return ast.NewIdent(t.Obj().Name())
	}
	m.fail("cannot write type %s", t)
	return ast.NewIdent("any")
}

func (m *monomorphizer) namedTypeExpr(obj *types.TypeName, file *ast.File) ast.Expr {
	if obj.Pkg() == nil {
		return ast.NewIdent(obj.Name())
	}
	if obj.Pkg() == m.pkg {
		if obj.Parent() != nil && obj.Parent() != m.pkg.Scope() {
			m.fail("type %s is declared inside a function", obj.Name())
		}
		return ast.NewIdent(obj.Name())
	}
	return &ast.SelectorExpr{X: ast.NewIdent(m.importName(file, obj.Pkg())), Sel: ast.NewIdent(obj.Name())}
}

// importName returns the name file imports pkg by, and records an import
// for file when it does not import it yet.
func (m *monomorphizer) importName(file *ast.File, pkg *types.Package) string {
	for _, imp := range file.Imports {
		path, err := strconv.Unquote(imp.Path.Value)
		if err != nil || path != pkg.Path() {
			continue
		}
		if imp.Name != nil && imp.Name.Name != "_" && imp.Name.Name != "." {
			return imp.Name.Name
		}
		return pkg.Name()
	}
	if m.extraTypes[file] == nil {
		m.extraTypes[file] = map[string]string{}
	}
	m.extraTypes[file][pkg.Path()] = pkg.Name()
	return pkg.Name()
}

func (m *monomorphizer) funcTypeExpr(sig *types.Signature, file *ast.File) *ast.FuncType {
	ft := &ast.FuncType{Params: &ast.FieldList{}}
	params := sig.Params()
	for i := 0; params != nil && i < params.Len(); i++ {
		typ := params.At(i).Type()
		var expr ast.Expr
		if sig.Variadic() && i == params.Len()-1 {
			if slice, ok := typ.(*types.Slice); ok {
				expr = &ast.Ellipsis{Elt: m.typeExpr(slice.Elem(), file)}
			}
		}
		if expr == nil {
			expr = m.typeExpr(typ, file)
		}
		ft.Params.List = append(ft.Params.List, &ast.Field{Type: expr})
	}
	if results := sig.Results(); results != nil && results.Len() > 0 {
		ft.Results = &ast.FieldList{}
		for i := 0; i < results.Len(); i++ {
			ft.Results.List = append(ft.Results.List, &ast.Field{Type: m.typeExpr(results.At(i).Type(), file)})
		}
	}
	return ft
}

// monoCopier deep-copies syntax, rewriting references to generic
// declarations and to type parameters on the way.
type monoCopier struct {
	m     *monomorphizer
	subst map[*types.TypeParam]types.Type
	file  *ast.File
}

func (c *monoCopier) copyNode(n ast.Node) ast.Node {
	return astCopier{rewrite: c.rewriteExpr}.copyNode(n)
}

// astCopier deep-copies syntax. rewrite, when set, may replace an
// expression before it is copied, and ident an identifier; the
// replacement is used as it is.
type astCopier struct {
	rewrite func(ast.Expr) (ast.Expr, bool)
	ident   func(*ast.Ident) (*ast.Ident, bool)
}

var (
	astExprType  = reflect.TypeOf((*ast.Expr)(nil)).Elem()
	astIdentType = reflect.TypeOf((*ast.Ident)(nil))
	astObjType   = reflect.TypeOf((*ast.Object)(nil))
	astScopeType = reflect.TypeOf((*ast.Scope)(nil))
	astCommentT  = reflect.TypeOf((*ast.CommentGroup)(nil))
)

func (c astCopier) copyNode(n ast.Node) ast.Node {
	if n == nil {
		return nil
	}
	return c.copyValue(reflect.ValueOf(n)).Interface().(ast.Node)
}

func (c astCopier) copyValue(v reflect.Value) reflect.Value {
	switch v.Kind() {
	case reflect.Interface:
		if v.IsNil() {
			return v
		}
		if v.Type() == astExprType && c.rewrite != nil {
			if e, ok := v.Interface().(ast.Expr); ok {
				if replaced, ok := c.rewrite(e); ok {
					out := reflect.New(astExprType).Elem()
					out.Set(reflect.ValueOf(replaced))
					return out
				}
			}
		}
		inner := c.copyValue(v.Elem())
		out := reflect.New(v.Type()).Elem()
		out.Set(inner)
		return out
	case reflect.Pointer:
		if v.IsNil() {
			return v
		}
		switch v.Type() {
		case astObjType, astScopeType:
			return reflect.Zero(v.Type())
		case astCommentT:
			return v
		case astIdentType:
			if c.ident != nil {
				if replaced, ok := c.ident(v.Interface().(*ast.Ident)); ok {
					return reflect.ValueOf(replaced)
				}
			}
		}
		if v.Elem().Kind() != reflect.Struct {
			return v
		}
		out := reflect.New(v.Elem().Type())
		c.copyStruct(out.Elem(), v.Elem())
		return out
	case reflect.Slice:
		if v.IsNil() {
			return v
		}
		out := reflect.MakeSlice(v.Type(), v.Len(), v.Len())
		for i := 0; i < v.Len(); i++ {
			out.Index(i).Set(c.copyValue(v.Index(i)))
		}
		return out
	case reflect.Struct:
		out := reflect.New(v.Type()).Elem()
		c.copyStruct(out, v)
		return out
	default:
		return v
	}
}

func (c astCopier) copyStruct(dst, src reflect.Value) {
	for i := 0; i < src.NumField(); i++ {
		if !dst.Field(i).CanSet() {
			continue
		}
		dst.Field(i).Set(c.copyValue(src.Field(i)))
	}
}

// rewriteExpr replaces a type parameter with its type argument, and a
// reference to a generic function or type, with or without explicit type
// arguments, with its instantiation.
func (c *monoCopier) rewriteExpr(e ast.Expr) (ast.Expr, bool) {
	info := c.m.info
	switch x := e.(type) {
	case *ast.Ident:
		if typeName, ok := info.Uses[x].(*types.TypeName); ok {
			if tp, ok := typeName.Type().(*types.TypeParam); ok {
				if arg, ok := c.subst[tp]; ok {
					return c.m.typeExpr(arg, c.file), true
				}
				c.m.fail("type parameter %s has no argument", tp)
				return nil, false
			}
		}
		if name, ok := c.instanceName(x); ok {
			return &ast.Ident{NamePos: x.NamePos, Name: name}, true
		}
	case *ast.IndexExpr:
		if ident, ok := x.X.(*ast.Ident); ok {
			if name, ok := c.instanceName(ident); ok {
				return &ast.Ident{NamePos: ident.NamePos, Name: name}, true
			}
		}
	case *ast.IndexListExpr:
		if ident, ok := x.X.(*ast.Ident); ok {
			if name, ok := c.instanceName(ident); ok {
				return &ast.Ident{NamePos: ident.NamePos, Name: name}, true
			}
		}
	case *ast.ParenExpr:
		// (T) around a type parameter, or around an instantiation.
		if inner, ok := c.rewriteExpr(x.X); ok {
			return &ast.ParenExpr{Lparen: x.Lparen, X: inner, Rparen: x.Rparen}, true
		}
	}
	return nil, false
}

func (c *monoCopier) instanceName(ident *ast.Ident) (string, bool) {
	inst, ok := c.m.info.Instances[ident]
	if !ok {
		return "", false
	}
	obj := c.m.info.Uses[ident]
	if obj == nil || !c.m.owns(obj) {
		return "", false
	}
	args := make([]types.Type, inst.TypeArgs.Len())
	for i := 0; i < inst.TypeArgs.Len(); i++ {
		args[i] = c.m.substType(inst.TypeArgs.At(i), c.subst)
	}
	return c.m.instantiate(obj, args), true
}

// rewrite builds the new files.
func (m *monomorphizer) rewrite() ([]*ast.File, bool) {
	genericDecl := map[ast.Decl]bool{}
	for _, decl := range m.genericFuncs {
		genericDecl[decl] = true
	}
	for _, methods := range m.genericMethods {
		for _, decl := range methods {
			genericDecl[decl] = true
		}
	}

	// Copy the non-generic code first: its references queue the
	// instantiations.
	type pendingFile struct {
		orig  *ast.File
		copy  *ast.File
		decls []ast.Decl
	}
	pending := make([]*pendingFile, len(m.files))
	for i, file := range m.files {
		pf := &pendingFile{orig: file}
		pending[i] = pf
		c := &monoCopier{m: m, file: file}
		for _, decl := range file.Decls {
			if genericDecl[decl] {
				pf.decls = append(pf.decls, decl) // placeholder, replaced below
				continue
			}
			if gen, ok := decl.(*ast.GenDecl); ok && gen.Tok == token.TYPE {
				var specs []ast.Spec
				hasGeneric := false
				for _, spec := range gen.Specs {
					if ts, ok := spec.(*ast.TypeSpec); ok && ts.TypeParams != nil && len(ts.TypeParams.List) > 0 {
						hasGeneric = true
						continue
					}
					specs = append(specs, c.copyNode(spec).(ast.Spec))
				}
				if hasGeneric {
					pf.decls = append(pf.decls, gen) // placeholder
					if len(specs) > 0 {
						copied := &ast.GenDecl{Doc: gen.Doc, TokPos: gen.TokPos, Tok: gen.Tok, Lparen: gen.Lparen, Specs: specs, Rparen: gen.Rparen}
						pf.decls = append(pf.decls, copied)
					}
					continue
				}
			}
			pf.decls = append(pf.decls, c.copyNode(decl).(ast.Decl))
			if m.err != nil {
				return nil, false
			}
		}
	}

	// Copy each instantiation, which may queue more.
	instDecls := map[*monoInstance][]ast.Decl{}
	for len(m.queue) > 0 {
		inst := m.queue[0]
		m.queue = m.queue[1:]
		decls := m.instanceDecls(inst)
		if m.err != nil {
			return nil, false
		}
		instDecls[inst] = decls
	}

	// Put the copies where the generic declarations were.
	out := make([]*ast.File, len(m.files))
	for i, pf := range pending {
		var decls []ast.Decl
		for _, decl := range pf.decls {
			switch d := decl.(type) {
			case *ast.FuncDecl:
				if fn, ok := m.info.Defs[d.Name].(*types.Func); ok && genericDecl[d] {
					if d.Recv == nil {
						for _, inst := range m.byObj[fn] {
							decls = append(decls, instDecls[inst]...)
						}
					}
					// Methods are written with their type.
					continue
				}
				decls = append(decls, d)
			case *ast.GenDecl:
				if d.Tok == token.TYPE && genDeclHasGenericSpec(d) && isOriginalDecl(pf.orig, d) {
					for _, spec := range d.Specs {
						ts, ok := spec.(*ast.TypeSpec)
						if !ok || ts.TypeParams == nil || len(ts.TypeParams.List) == 0 {
							continue
						}
						typeName, _ := m.info.Defs[ts.Name].(*types.TypeName)
						for _, inst := range m.byObj[typeName] {
							decls = append(decls, instDecls[inst]...)
						}
					}
					continue
				}
				decls = append(decls, d)
			default:
				decls = append(decls, d)
			}
		}
		file := &ast.File{
			Doc:       pf.orig.Doc,
			Package:   pf.orig.Package,
			Name:      &ast.Ident{NamePos: pf.orig.Name.NamePos, Name: pf.orig.Name.Name},
			Decls:     decls,
			FileStart: pf.orig.FileStart,
			FileEnd:   pf.orig.FileEnd,
			Comments:  pf.orig.Comments,
			GoVersion: pf.orig.GoVersion,
		}
		m.addImports(pf.orig, file)
		out[i] = file
	}

	for attempt := 0; attempt < 3; attempt++ {
		info, err := NewTypeInfoWithImporter(m.path, out, m.fset, m.imp)
		if err == nil && info != nil {
			return out, true
		}
		if !removeUnusedImports(out, err) {
			m.fail("rewritten package does not type-check: %v", err)
			return nil, false
		}
	}
	m.fail("rewritten package still has unused imports")
	return nil, false
}

func genDeclHasGenericSpec(gen *ast.GenDecl) bool {
	for _, spec := range gen.Specs {
		if ts, ok := spec.(*ast.TypeSpec); ok && ts.TypeParams != nil && len(ts.TypeParams.List) > 0 {
			return true
		}
	}
	return false
}

func isOriginalDecl(file *ast.File, decl ast.Decl) bool {
	for _, d := range file.Decls {
		if d == decl {
			return true
		}
	}
	return false
}

// instanceDecls copies the declaration of inst, and for a type its methods.
func (m *monomorphizer) instanceDecls(inst *monoInstance) []ast.Decl {
	switch obj := inst.obj.(type) {
	case *types.Func:
		decl := m.genericFuncs[obj]
		sig := obj.Type().(*types.Signature)
		subst := typeParamSubst(sig.TypeParams(), inst.typeArgs)
		c := &monoCopier{m: m, subst: subst, file: m.declFile[decl]}
		copied := c.copyNode(decl).(*ast.FuncDecl)
		copied.Name = &ast.Ident{NamePos: decl.Name.NamePos, Name: inst.name}
		copied.Type.TypeParams = nil
		return []ast.Decl{copied}
	case *types.TypeName:
		spec := m.genericTypes[obj]
		named, _ := obj.Type().(*types.Named)
		if named == nil {
			m.fail("generic type %s is not a defined type", obj.Name())
			return nil
		}
		subst := typeParamSubst(named.TypeParams(), inst.typeArgs)
		c := &monoCopier{m: m, subst: subst, file: m.declFile[spec]}
		copied := c.copyNode(spec).(*ast.TypeSpec)
		copied.Name = &ast.Ident{NamePos: spec.Name.NamePos, Name: inst.name}
		copied.TypeParams = nil
		decls := []ast.Decl{&ast.GenDecl{Doc: spec.Doc, TokPos: spec.Pos(), Tok: token.TYPE, Specs: []ast.Spec{copied}}}
		for _, method := range m.genericMethods[obj] {
			fn, ok := m.info.Defs[method.Name].(*types.Func)
			if !ok {
				m.fail("method %s has no object", method.Name.Name)
				return nil
			}
			msig := fn.Type().(*types.Signature)
			msubst := typeParamSubst(msig.RecvTypeParams(), inst.typeArgs)
			mc := &monoCopier{m: m, subst: msubst, file: m.declFile[method]}
			copiedMethod := mc.copyNode(method).(*ast.FuncDecl)
			recv := copiedMethod.Recv.List[0]
			base := ast.Expr(&ast.Ident{NamePos: method.Recv.List[0].Type.Pos(), Name: inst.name})
			if _, isPtr := method.Recv.List[0].Type.(*ast.StarExpr); isPtr {
				base = &ast.StarExpr{Star: method.Recv.List[0].Type.Pos(), X: base}
			}
			recv.Type = base
			decls = append(decls, copiedMethod)
		}
		return decls
	}
	m.fail("cannot instantiate %s", inst.obj.Name())
	return nil
}

func typeParamSubst(params *types.TypeParamList, args []types.Type) map[*types.TypeParam]types.Type {
	subst := map[*types.TypeParam]types.Type{}
	for i := 0; params != nil && i < params.Len() && i < len(args); i++ {
		subst[params.At(i)] = args[i]
	}
	return subst
}

func (m *monomorphizer) addImports(orig, file *ast.File) {
	extra := m.extraTypes[orig]
	paths := make([]string, 0, len(extra))
	for path := range extra {
		paths = append(paths, path)
	}
	sort.Strings(paths)
	var specs []ast.Spec
	for _, imp := range orig.Imports {
		specs = append(specs, imp)
	}
	for _, path := range paths {
		specs = append(specs, &ast.ImportSpec{Path: &ast.BasicLit{Kind: token.STRING, Value: strconv.Quote(path)}})
	}
	// The import declarations are rebuilt as one, so that both kinds of
	// imports are in file.Imports and in a declaration.
	var rest []ast.Decl
	for _, decl := range file.Decls {
		if gen, ok := decl.(*ast.GenDecl); ok && gen.Tok == token.IMPORT {
			continue
		}
		rest = append(rest, decl)
	}
	file.Imports = nil
	if len(specs) == 0 {
		file.Decls = rest
		return
	}
	for _, spec := range specs {
		file.Imports = append(file.Imports, spec.(*ast.ImportSpec))
	}
	importDecl := &ast.GenDecl{Tok: token.IMPORT, TokPos: orig.Name.End(), Lparen: orig.Name.End(), Specs: specs, Rparen: orig.Name.End()}
	file.Decls = append([]ast.Decl{importDecl}, rest...)
}

// removeUnusedImports drops each import a type-check error says is not
// used, and reports whether it dropped any.
func removeUnusedImports(files []*ast.File, err error) bool {
	if err == nil {
		return false
	}
	removed := false
	for _, line := range strings.Split(err.Error(), "\n") {
		if !strings.Contains(line, "imported and not used") {
			continue
		}
		start := strings.Index(line, "\"")
		end := strings.LastIndex(line, "\"")
		if start < 0 || end <= start {
			continue
		}
		path := line[start+1 : end]
		for _, file := range files {
			if dropImport(file, path) {
				removed = true
			}
		}
	}
	return removed
}

func dropImport(file *ast.File, path string) bool {
	quoted := strconv.Quote(path)
	dropped := false
	var imports []*ast.ImportSpec
	for _, imp := range file.Imports {
		if imp.Path.Value == quoted {
			dropped = true
			continue
		}
		imports = append(imports, imp)
	}
	if !dropped {
		return false
	}
	file.Imports = imports
	for _, decl := range file.Decls {
		gen, ok := decl.(*ast.GenDecl)
		if !ok || gen.Tok != token.IMPORT {
			continue
		}
		var specs []ast.Spec
		for _, spec := range gen.Specs {
			if imp, ok := spec.(*ast.ImportSpec); ok && imp.Path.Value == quoted {
				continue
			}
			specs = append(specs, spec)
		}
		gen.Specs = specs
	}
	return true
}
