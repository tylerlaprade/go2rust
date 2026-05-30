package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"go/types"
	"sort"
	"strings"
)

var functionNameOverrides map[*ast.FuncDecl]string
var functionNameOverridesByGoName map[string]string
var packageFunctionNameOverrides map[string]string
var packageMethodNameOverrides map[string]string
var packageGlobalNameOverrides map[string]string
var packageGlobalNames = make(map[string]bool)

func SetFunctionNameOverrides(overrides map[*ast.FuncDecl]string) {
	functionNameOverrides = overrides
	functionNameOverridesByGoName = make(map[string]string)
	for fn, name := range overrides {
		if fn.Name.Name != "init" {
			functionNameOverridesByGoName[fn.Name.Name] = name
		}
	}
}

func rustFunctionName(fn *ast.FuncDecl) string {
	if functionNameOverrides != nil {
		if name, ok := functionNameOverrides[fn]; ok {
			return name
		}
	}
	if packageFunctionNameOverrides != nil {
		if name, ok := packageFunctionNameOverrides[fn.Name.Name]; ok {
			return name
		}
	}
	return RustFunctionName(fn.Name.Name)
}

func rustFunctionNameForUse(name string) string {
	if functionNameOverridesByGoName != nil {
		if rustName, ok := functionNameOverridesByGoName[name]; ok {
			return rustName
		}
	}
	if packageFunctionNameOverrides != nil {
		if rustName, ok := packageFunctionNameOverrides[name]; ok {
			return rustName
		}
	}
	return RustFunctionName(name)
}

func methodFuncForDecl(fn *ast.FuncDecl, typeInfo *TypeInfo) *types.Func {
	if fn == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	obj, ok := typeInfo.info.Defs[fn.Name].(*types.Func)
	if !ok {
		return nil
	}
	return obj
}

func methodReceiverKey(recv types.Type) string {
	if recv == nil {
		return ""
	}
	recv = types.Unalias(recv)
	if ptr, ok := recv.(*types.Pointer); ok {
		recv = types.Unalias(ptr.Elem())
	}
	if named, ok := recv.(*types.Named); ok && named.Obj() != nil {
		obj := named.Obj()
		if obj.Pkg() != nil {
			return obj.Pkg().Path() + "." + obj.Name()
		}
		return obj.Name()
	}
	return types.TypeString(recv, func(pkg *types.Package) string {
		if pkg == nil {
			return ""
		}
		return pkg.Path()
	})
}

func methodOverrideKey(fn *types.Func) string {
	if fn == nil {
		return ""
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.Recv() == nil {
		return ""
	}
	recvKey := methodReceiverKey(sig.Recv().Type())
	if recvKey == "" {
		return ""
	}
	pkgPath := ""
	if fn.Pkg() != nil {
		pkgPath = fn.Pkg().Path()
	}
	return pkgPath + "\x00" + recvKey + "\x00" + fn.Name()
}

func rustMethodName(fn *ast.FuncDecl) string {
	if packageMethodNameOverrides != nil {
		if key := methodOverrideKey(methodFuncForDecl(fn, GetTypeInfo())); key != "" {
			if name, ok := packageMethodNameOverrides[key]; ok {
				return name
			}
		}
	}
	return RustFunctionName(fn.Name.Name)
}

func rustMethodNameForTypesFunc(fn *types.Func) string {
	if fn == nil {
		return ""
	}
	if packageMethodNameOverrides != nil {
		if key := methodOverrideKey(fn); key != "" {
			if name, ok := packageMethodNameOverrides[key]; ok {
				return name
			}
		}
	}
	return RustFunctionName(fn.Name())
}

func rustMethodSelectorName(sel *ast.SelectorExpr) string {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.info != nil {
		if selection, ok := typeInfo.info.Selections[sel]; ok && selection.Kind() != types.FieldVal {
			if fn, ok := selection.Obj().(*types.Func); ok {
				if key := methodOverrideKey(fn); key != "" && packageMethodNameOverrides != nil {
					if name, ok := packageMethodNameOverrides[key]; ok {
						return name
					}
				}
			}
		}
	}
	return RustFunctionName(sel.Sel.Name)
}

func assignFunctionNames(functions []*ast.FuncDecl) map[*ast.FuncDecl]string {
	names := make(map[*ast.FuncDecl]string)
	used := make(map[string]int)
	// Seed the in-file counter with names the per-package layer has
	// already claimed. Without this, the per-file rename collides
	// with `assignPackageFunctionNames` — e.g. both `walkDir` (package
	// layer) and `WalkDir` (file layer) would each resolve to
	// `walk_dir_1`, producing two definitions with the same Rust name.
	for _, override := range packageFunctionNameOverrides {
		used[override]++
	}
	initIndex := 0
	for _, fn := range functions {
		if _, hasPkgOverride := packageFunctionNameOverrides[fn.Name.Name]; hasPkgOverride {
			// Per-package layer already chose this function's Rust name.
			// Skip the per-file rename so we don't collide with it.
			continue
		}
		rustName := RustFunctionName(fn.Name.Name)
		if fn.Name.Name == "init" {
			rustName = fmt.Sprintf("__go_init_%d", initIndex)
			initIndex++
		} else if count := used[rustName]; count > 0 {
			rustName = fmt.Sprintf("%s_%d", rustName, count)
		}
		used[RustFunctionName(fn.Name.Name)]++
		if rustName != RustFunctionName(fn.Name.Name) {
			names[fn] = rustName
		}
	}
	return names
}

type packageFunctionName struct {
	goName   string
	rustName string
	pos      token.Pos
	exported bool
}

type packageMethodName struct {
	key      string
	goName   string
	rustName string
	pos      token.Pos
	exported bool
}

func assignPackageFunctionNames(files []*ast.File) map[string]string {
	byRustName := make(map[string][]packageFunctionName)
	seenGoNames := make(map[string]bool)
	for _, file := range files {
		for _, decl := range file.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Recv != nil || fn.Name.Name == "init" || seenGoNames[fn.Name.Name] {
				continue
			}
			seenGoNames[fn.Name.Name] = true
			rustName := RustFunctionName(fn.Name.Name)
			byRustName[rustName] = append(byRustName[rustName], packageFunctionName{
				goName:   fn.Name.Name,
				rustName: rustName,
				pos:      fn.Pos(),
				exported: ast.IsExported(fn.Name.Name),
			})
		}
	}

	overrides := make(map[string]string)
	for rustName, functions := range byRustName {
		if len(functions) <= 1 {
			continue
		}
		sort.Slice(functions, func(i, j int) bool {
			if functions[i].exported != functions[j].exported {
				return functions[i].exported
			}
			if functions[i].pos != functions[j].pos {
				return functions[i].pos < functions[j].pos
			}
			return functions[i].goName < functions[j].goName
		})
		for i, fn := range functions {
			if i == 0 {
				continue
			}
			overrides[fn.goName] = fmt.Sprintf("%s_%d", rustName, i)
		}
	}
	return overrides
}

func assignPackageGlobalNameOverrides(files []*ast.File, functionOverrides map[string]string) map[string]string {
	usedRustNames := make(map[string]bool)
	seenFunctions := make(map[string]bool)
	for _, file := range files {
		for _, decl := range file.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Recv != nil || fn.Name.Name == "init" || seenFunctions[fn.Name.Name] {
				continue
			}
			seenFunctions[fn.Name.Name] = true
			rustName := RustFunctionName(fn.Name.Name)
			if functionOverrides != nil {
				if override, ok := functionOverrides[fn.Name.Name]; ok {
					rustName = override
				}
			}
			usedRustNames[rustName] = true
		}
	}

	overrides := make(map[string]string)
	seenGlobals := make(map[string]bool)
	for _, file := range files {
		for _, decl := range file.Decls {
			genDecl, ok := decl.(*ast.GenDecl)
			if !ok || genDecl.Tok != token.VAR {
				continue
			}
			for _, spec := range genDecl.Specs {
				valueSpec, ok := spec.(*ast.ValueSpec)
				if !ok {
					continue
				}
				for _, name := range valueSpec.Names {
					if name.Name == "_" || seenGlobals[name.Name] {
						continue
					}
					seenGlobals[name.Name] = true
					base := EscapeRustIdent(name.Name)
					rustName := base
					if usedRustNames[rustName] {
						for suffix := 1; ; suffix++ {
							candidate := fmt.Sprintf("%s_%d", base, suffix)
							if !usedRustNames[candidate] {
								rustName = candidate
								break
							}
						}
					}
					usedRustNames[rustName] = true
					if rustName != base {
						overrides[name.Name] = rustName
					}
				}
			}
		}
	}
	return overrides
}

func assignPackageMethodNames(files []*ast.File, typeInfo *TypeInfo) map[string]string {
	byReceiverRustName := make(map[string][]packageMethodName)
	seenMethodKeys := make(map[string]bool)
	for _, file := range files {
		for _, decl := range file.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Recv == nil {
				continue
			}
			methodObj := methodFuncForDecl(fn, typeInfo)
			key := methodOverrideKey(methodObj)
			if key == "" || seenMethodKeys[key] {
				continue
			}
			seenMethodKeys[key] = true
			sig, _ := methodObj.Type().(*types.Signature)
			receiverKey := methodReceiverKey(sig.Recv().Type())
			rustName := RustFunctionName(fn.Name.Name)
			groupKey := receiverKey + "\x00" + rustName
			byReceiverRustName[groupKey] = append(byReceiverRustName[groupKey], packageMethodName{
				key:      key,
				goName:   fn.Name.Name,
				rustName: rustName,
				pos:      fn.Pos(),
				exported: ast.IsExported(fn.Name.Name),
			})
		}
	}

	overrides := make(map[string]string)
	for _, methods := range byReceiverRustName {
		if len(methods) <= 1 {
			continue
		}
		rustName := methods[0].rustName
		sort.Slice(methods, func(i, j int) bool {
			if methods[i].exported != methods[j].exported {
				return methods[i].exported
			}
			if methods[i].pos != methods[j].pos {
				return methods[i].pos < methods[j].pos
			}
			return methods[i].goName < methods[j].goName
		})
		for i, method := range methods {
			if i == 0 {
				continue
			}
			overrides[method.key] = fmt.Sprintf("%s_%d", rustName, i)
		}
	}
	return overrides
}

type packageGlobal struct {
	name     string
	rustType string
	typ      types.Type
}

func registerPackageGlobalNames(globalVars []*ast.GenDecl) {
	for _, genDecl := range globalVars {
		if genDecl == nil || genDecl.Tok != token.VAR {
			continue
		}
		for _, spec := range genDecl.Specs {
			valueSpec, ok := spec.(*ast.ValueSpec)
			if !ok {
				continue
			}
			for _, name := range valueSpec.Names {
				if name.Name != "_" {
					packageGlobalNames[name.Name] = true
				}
			}
		}
	}
}

func rustPackageGlobalName(name string) string {
	if packageGlobalNameOverrides != nil {
		if rustName, ok := packageGlobalNameOverrides[name]; ok {
			return rustName
		}
	}
	return EscapeRustIdent(name)
}

func SetPackageGlobalNameOverrides(overrides map[string]string) {
	packageGlobalNameOverrides = overrides
}

func packageGlobalVisibility(name string) string {
	if ast.IsExported(name) {
		return "pub"
	}
	return "pub(crate)"
}

func isPackageGlobalIdent(ident *ast.Ident) bool {
	if ident == nil {
		return false
	}
	name := ident.Name
	if name == "_" {
		return false
	}
	if name == "nil" {
		return false
	}
	if name == "true" {
		return false
	}
	if name == "false" {
		return false
	}
	if lookupVarInfo(name) != nil {
		return false
	}
	if isConstIdent(ident) {
		return false
	}
	if _, ok := packageConstants[name]; ok {
		return false
	}
	if packageGlobalNames[name] {
		return true
	}
	return isPackageGlobalObjectIdent(ident)
}

func isPackageGlobalObjectIdent(ident *ast.Ident) bool {
	if ident == nil {
		return false
	}
	name := ident.Name
	if name == "_" {
		return false
	}
	if name == "nil" {
		return false
	}
	if name == "true" {
		return false
	}
	if name == "false" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if typeInfo.info == nil {
		return false
	}
	if typeInfo.pkg == nil {
		return false
	}
	obj := typeInfo.GetObject(ident)
	if _, ok := obj.(*types.Var); !ok {
		return false
	}
	return obj.Parent() == typeInfo.pkg.Scope()
}

func packageGlobalPointerIdent(expr ast.Expr) (*ast.Ident, bool) {
	ident, ok := expr.(*ast.Ident)
	if !ok || !isPackageGlobalIdent(ident) {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(ident) {
		return nil, false
	}
	return ident, true
}

func writePackageGlobalPointerHandleClone(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("(*")
	out.WriteString(rustPackageGlobalName(ident.Name))
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone()")
}

func writePackageGlobalPointerPointeeClone(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("{ let __global_ptr = ")
	writePackageGlobalPointerHandleClone(out, ident)
	out.WriteString("; let __global_guard = __global_ptr")
	WriteBorrowMethod(out, false)
	out.WriteString("; (*__global_guard.as_ref().unwrap()).clone() }")
}

func collectPackageGlobals(globalVars []*ast.GenDecl) []packageGlobal {
	typeInfo := GetTypeInfo()
	globals := make([]packageGlobal, 0)
	for _, genDecl := range globalVars {
		if genDecl == nil || genDecl.Tok != token.VAR {
			continue
		}
		for _, spec := range genDecl.Specs {
			valueSpec, ok := spec.(*ast.ValueSpec)
			if !ok {
				continue
			}
			for _, name := range valueSpec.Names {
				if name.Name == "_" {
					continue
				}
				registerAnonymousStructsForPackageGlobal(valueSpec, name)
				var typ types.Type
				if typeInfo != nil && typeInfo.info != nil {
					if obj, ok := typeInfo.info.Defs[name].(*types.Var); ok {
						typ = obj.Type()
					}
				}
				rustType := "/* ERROR: Type information required for package variable " + name.Name + " */ ()"
				if typ != nil {
					rustType = goTypesTypeToRust(typ)
				} else if valueSpec.Type != nil {
					rustType = goTypeToRustBase(valueSpec.Type)
				}
				globals = append(globals, packageGlobal{
					name:     name.Name,
					rustType: rustType,
					typ:      typ,
				})
				packageGlobalNames[name.Name] = true
				registerTypeExprCollectionInfo(name.Name, valueSpec.Type)
			}
		}
	}
	return globals
}

func registerAnonymousStructsForPackageGlobal(valueSpec *ast.ValueSpec, name *ast.Ident) {
	if valueSpec == nil || name == nil {
		return
	}
	if typeName, ok := anonymousStructTypeNameFromExpr(valueSpec.Type); ok {
		anonymousStructAliases[RustTypeNameForUse(name.Name)] = typeName
	} else {
		registerAnonymousStructsInTypeExpr(valueSpec.Type)
	}
	for i, candidate := range valueSpec.Names {
		if candidate != name {
			continue
		}
		if i < len(valueSpec.Values) {
			if lit, ok := valueSpec.Values[i].(*ast.CompositeLit); ok {
				if typeName, ok := anonymousStructTypeNameFromExpr(lit.Type); ok {
					anonymousStructAliases[RustTypeNameForUse(name.Name)] = typeName
				} else {
					registerAnonymousStructsInTypeExpr(lit.Type)
				}
			}
		}
		return
	}
}

func anonymousStructTypeNameFromExpr(expr ast.Expr) (string, bool) {
	structType, ok := expr.(*ast.StructType)
	if !ok {
		return "", false
	}
	return generateAnonymousStructType(structType), true
}

func registerAnonymousStructsInTypeExpr(expr ast.Expr) {
	switch t := expr.(type) {
	case nil:
		return
	case *ast.StructType:
		generateAnonymousStructType(t)
	case *ast.ArrayType:
		registerAnonymousStructsInTypeExpr(t.Elt)
	case *ast.MapType:
		registerAnonymousStructsInTypeExpr(t.Key)
		registerAnonymousStructsInTypeExpr(t.Value)
	case *ast.StarExpr:
		registerAnonymousStructsInTypeExpr(t.X)
	case *ast.ChanType:
		registerAnonymousStructsInTypeExpr(t.Value)
	case *ast.Ellipsis:
		registerAnonymousStructsInTypeExpr(t.Elt)
	case *ast.FuncType:
		registerAnonymousStructsInFieldList(t.Params)
		registerAnonymousStructsInFieldList(t.Results)
	}
}

func registerAnonymousStructsInFieldList(fields *ast.FieldList) {
	if fields == nil {
		return
	}
	for _, field := range fields.List {
		registerAnonymousStructsInTypeExpr(field.Type)
	}
}

func hasNamedPackageGlobals(globalVars []*ast.GenDecl) bool {
	for _, genDecl := range globalVars {
		for _, spec := range genDecl.Specs {
			valueSpec, ok := spec.(*ast.ValueSpec)
			if !ok {
				continue
			}
			for _, name := range valueSpec.Names {
				if name.Name != "_" {
					return true
				}
			}
		}
	}
	return false
}

func TranspilePackageGlobals(out *strings.Builder, globalVars []*ast.GenDecl) {
	globals := collectPackageGlobals(globalVars)
	if len(globals) == 0 {
		return
	}
	if !NeedsConcurrentWrapper() {
		out.WriteString("pub(crate) struct GoGlobal<T> {\n")
		out.WriteString("    value: std::cell::UnsafeCell<Option<T>>,\n")
		out.WriteString("}\n")
		out.WriteString("unsafe impl<T> Sync for GoGlobal<T> {}\n")
		out.WriteString("impl<T> GoGlobal<T> {\n")
		out.WriteString("    pub(crate) const fn new() -> Self {\n")
		out.WriteString("        Self { value: std::cell::UnsafeCell::new(None) }\n")
		out.WriteString("    }\n")
		out.WriteString("    pub(crate) fn borrow(&'static self) -> &'static Option<T> {\n")
		out.WriteString("        unsafe { &*self.value.get() }\n")
		out.WriteString("    }\n")
		out.WriteString("    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {\n")
		out.WriteString("        unsafe { &mut *self.value.get() }\n")
		out.WriteString("    }\n")
		out.WriteString("    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {\n")
		out.WriteString("        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))\n")
		out.WriteString("    }\n")
		out.WriteString("}\n\n")
	}
	for i, global := range globals {
		if i > 0 {
			out.WriteString("\n")
		}
		if NeedsConcurrentWrapper() {
			out.WriteString(packageGlobalVisibility(global.name))
			out.WriteString(" static ")
			out.WriteString(rustPackageGlobalName(global.name))
			out.WriteString(": std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<")
			out.WriteString(global.rustType)
			out.WriteString(">>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));\n")
		} else {
			out.WriteString(packageGlobalVisibility(global.name))
			out.WriteString(" static ")
			out.WriteString(rustPackageGlobalName(global.name))
			out.WriteString(": GoGlobal<")
			out.WriteString(global.rustType)
			out.WriteString("> = GoGlobal::new();\n")
		}
	}
	out.WriteString("\n\n")
	transpilePackageGlobalInit(out, globals)
}

func transpilePackageGlobalInit(out *strings.Builder, globals []packageGlobal) {
	out.WriteString("fn __go_init_globals() {\n")
	for _, global := range globals {
		if global.typ != nil {
			if _, isFunc := global.typ.Underlying().(*types.Signature); isFunc {
				continue
			}
		}
		out.WriteString("    *")
		out.WriteString(rustPackageGlobalName(global.name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = ")
		if isGoErrorType(global.typ) || isInterfaceType(global.typ) {
			out.WriteString("None;\n")
			continue
		}
		out.WriteString("Some(")
		if global.typ != nil {
			out.WriteString(zeroValueForTypesType(global.typ))
		} else {
			out.WriteString("Default::default()")
		}
		out.WriteString(");\n")
	}

	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		out.WriteString("    /* ERROR: Type information required for package variable initialization order */\n")
		out.WriteString("    unimplemented!();\n")
		out.WriteString("}\n")
		return
	}

	globalByName := make(map[string]packageGlobal, len(globals))
	for _, global := range globals {
		globalByName[global.name] = global
	}
	for _, init := range typeInfo.info.InitOrder {
		if len(init.Lhs) == 0 {
			continue
		}
		if len(init.Lhs) != 1 {
			if packageGlobalMultiValueInitHasGlobal(init, globalByName) {
				writePackageGlobalMultiValueInit(out, init, globalByName)
			}
			continue
		}
		global, ok := globalByName[init.Lhs[0].Name()]
		if !ok {
			continue
		}
		if writePackageGlobalErrorHandleInit(out, global, init.Rhs) {
			continue
		}
		if writePackageGlobalInterfaceHandleInit(out, global, init.Rhs) {
			continue
		}
		if writePackageGlobalCompositeInit(out, global, init.Rhs) {
			continue
		}
		out.WriteString("    *")
		out.WriteString(rustPackageGlobalName(init.Lhs[0].Name()))
		WriteBorrowMethod(out, true)
		out.WriteString(" = Some(")
		writePackageGlobalInitValue(out, init.Rhs, global.typ)
		out.WriteString(");\n")
	}
	out.WriteString("}\n")
}

func packageGlobalMultiValueInitHasGlobal(init *types.Initializer, globalByName map[string]packageGlobal) bool {
	for _, lhs := range init.Lhs {
		if _, ok := globalByName[lhs.Name()]; ok {
			return true
		}
	}
	return false
}

func writePackageGlobalMultiValueInit(out *strings.Builder, init *types.Initializer, globalByName map[string]packageGlobal) {
	out.WriteString("    let (")
	tempNames := make([]string, len(init.Lhs))
	for i, lhs := range init.Lhs {
		if i > 0 {
			out.WriteString(", ")
		}
		if lhs.Name() == "_" {
			out.WriteString("_")
			continue
		}
		tempName := fmt.Sprintf("__go_pkg_init_%d", i)
		tempNames[i] = tempName
		out.WriteString(tempName)
	}
	out.WriteString(") = ")
	TranspileExpression(out, init.Rhs)
	out.WriteString(";\n")

	callExpr, _ := init.Rhs.(*ast.CallExpr)
	for i, lhs := range init.Lhs {
		if lhs.Name() == "_" || tempNames[i] == "" {
			continue
		}
		global, ok := globalByName[lhs.Name()]
		if !ok {
			continue
		}
		bareScalar := callExpr != nil && callResultIsBareScalar(callExpr, i)
		writePackageGlobalInitTempAssignment(out, global, tempNames[i], bareScalar)
	}
}

func writePackageGlobalInitTempAssignment(out *strings.Builder, global packageGlobal, tempName string, tempBareScalar bool) {
	out.WriteString("    *")
	out.WriteString(rustPackageGlobalName(global.name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = ")
	if tempBareScalar {
		out.WriteString("Some(")
		out.WriteString(tempName)
		out.WriteString(");\n")
		return
	}
	if isGoErrorType(global.typ) {
		out.WriteString("{ let mut __guard = ")
		out.WriteString(tempName)
		WriteBorrowMethod(out, true)
		out.WriteString("; __guard.take() };\n")
		return
	}
	if mapValueTypeKeepsHandle(global.typ) {
		out.WriteString("Some(")
		out.WriteString(tempName)
		out.WriteString(".clone());\n")
		return
	}
	out.WriteString("Some((*")
	out.WriteString(tempName)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()).clone());\n")
}

func writePackageGlobalCompositeInit(out *strings.Builder, global packageGlobal, expr ast.Expr) bool {
	lit, ok := expr.(*ast.CompositeLit)
	if !ok {
		return false
	}
	mapType := packageGlobalMapLiteralType(global.typ, lit)
	if mapType == nil {
		return false
	}
	writePackageGlobalMapLiteralInit(out, rustPackageGlobalName(global.name), mapType, lit)
	return true
}

func packageGlobalMapLiteralType(globalType types.Type, lit *ast.CompositeLit) *types.Map {
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if typ := typeInfo.GetType(lit); typ != nil {
			if mapType := underlyingMapType(typ); mapType != nil {
				return mapType
			}
		}
	}
	return underlyingMapType(globalType)
}

func underlyingMapType(typ types.Type) *types.Map {
	if typ == nil {
		return nil
	}
	typ = types.Unalias(typ)
	if mapType, ok := typ.Underlying().(*types.Map); ok {
		return mapType
	}
	return nil
}

func writePackageGlobalMapLiteralInit(out *strings.Builder, name string, mapType *types.Map, lit *ast.CompositeLit) {
	TrackImport("BTreeMap")
	out.WriteString("    {\n")
	out.WriteString("        let mut __go_map = BTreeMap::<")
	out.WriteString(goTypesMapKeyToRust(mapType.Key()))
	out.WriteString(", ")
	out.WriteString(goTypesMapValueToRust(mapType.Elem()))
	out.WriteString(">::new();\n")
	for _, elt := range lit.Elts {
		kv, ok := elt.(*ast.KeyValueExpr)
		if !ok {
			out.WriteString("        /* ERROR: Type information required for package map literal element */\n")
			out.WriteString("        unimplemented!();\n")
			continue
		}
		if writePackageGlobalMapSliceValueInsert(out, kv, mapType.Key(), mapType.Elem()) {
			continue
		}
		out.WriteString("        __go_map.insert(")
		writeMapLiteralKeyWithType(out, kv.Key, mapType.Key())
		out.WriteString(", ")
		writeWrappedMapValue(out, kv.Value, nil, mapType.Elem())
		out.WriteString(");\n")
	}
	out.WriteString("        *")
	out.WriteString(name)
	WriteBorrowMethod(out, true)
	out.WriteString(" = Some(__go_map);\n")
	out.WriteString("    }\n")
}

func writePackageGlobalMapSliceValueInsert(out *strings.Builder, kv *ast.KeyValueExpr, keyType types.Type, valueType types.Type) bool {
	valueLit, ok := kv.Value.(*ast.CompositeLit)
	if !ok {
		return false
	}
	sliceType := underlyingSliceType(valueType)
	if sliceType == nil {
		return false
	}
	keyName := fmt.Sprintf("__go_map_key_%d", valueLit.Pos())
	valueName := fmt.Sprintf("__go_map_value_%d", valueLit.Pos())
	out.WriteString("        let ")
	out.WriteString(keyName)
	out.WriteString(" = ")
	writeMapLiteralKeyWithType(out, kv.Key, keyType)
	out.WriteString(";\n")
	out.WriteString("        let mut ")
	out.WriteString(valueName)
	out.WriteString(" = Vec::<")
	out.WriteString(goTypesTypeToRust(sliceType.Elem()))
	out.WriteString(">::new();\n")
	for _, elt := range orderedArrayLiteralValues(valueLit.Elts) {
		out.WriteString("        ")
		out.WriteString(valueName)
		out.WriteString(".push(")
		if elt == nil {
			out.WriteString(zeroValueForTypesType(sliceType.Elem()))
		} else if !writeOwnedExpressionValue(out, elt) {
			TranspileExpression(out, elt)
		}
		out.WriteString(");\n")
	}
	out.WriteString("        __go_map.insert(")
	out.WriteString(keyName)
	out.WriteString(", ")
	WriteWrapperPrefix(out)
	out.WriteString(valueName)
	WriteWrapperSuffix(out)
	out.WriteString(");\n")
	return true
}

func underlyingSliceType(typ types.Type) *types.Slice {
	if typ == nil {
		return nil
	}
	typ = types.Unalias(typ)
	if sliceType, ok := typ.Underlying().(*types.Slice); ok {
		return sliceType
	}
	return nil
}

func writePackageGlobalErrorHandleInit(out *strings.Builder, global packageGlobal, expr ast.Expr) bool {
	if !isGoErrorType(global.typ) {
		return false
	}
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("    *")
		out.WriteString(rustPackageGlobalName(global.name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = None;\n")
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	exprType := typeInfo.GetType(expr)
	if isGoErrorType(exprType) {
		out.WriteString("    { let __rhs_holder = ")
		TranspileExpressionContext(out, expr, LValue)
		out.WriteString(".clone(); let new_val = { let mut guard = __rhs_holder")
		WriteBorrowMethod(out, true)
		out.WriteString("; guard.take() }; *")
		out.WriteString(rustPackageGlobalName(global.name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = new_val; }\n")
		return true
	}
	if isConcreteGoErrorValue(exprType) {
		out.WriteString("    *")
		out.WriteString(rustPackageGlobalName(global.name))
		WriteBorrowMethod(out, true)
		out.WriteString(" = Some(")
		writeConcreteErrorBox(out, expr)
		out.WriteString(");\n")
		return true
	}
	return false
}

func writePackageGlobalInterfaceHandleInit(out *strings.Builder, global packageGlobal, expr ast.Expr) bool {
	if isGoErrorType(global.typ) {
		return false
	}
	ifaceName, ok := transpiledNamedInterfaceTypeNameFromTypes(global.typ)
	if !ok {
		return false
	}
	out.WriteString("    *")
	out.WriteString(rustPackageGlobalName(global.name))
	WriteBorrowMethod(out, true)
	out.WriteString(" = ")
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("None;\n")
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		out.WriteString("unimplemented!(\"type info required to initialize package-global interface\");\n")
		return true
	}
	exprType := typeInfo.GetType(expr)
	if exprType == nil {
		out.WriteString("unimplemented!(\"type info required to initialize package-global interface\");\n")
		return true
	}
	out.WriteString("Some(")
	if _, ok := types.Unalias(exprType).Underlying().(*types.Interface); ok {
		out.WriteString("(*")
		TranspileExpressionContext(out, expr, LValue)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
	} else {
		out.WriteString("Box::new(")
		writeLocalInterfaceWrappedConstructionInnerValue(out, expr, global.typ)
		out.WriteString(") as ")
		out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	}
	out.WriteString(");\n")
	return true
}

func isPointerGlobalType(typ types.Type) bool {
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	return ok
}

func writePackageGlobalInitValue(out *strings.Builder, expr ast.Expr, targetType types.Type) {
	if funcLit, ok := expr.(*ast.FuncLit); ok {
		TranspileFuncLitBox(out, funcLit)
		return
	}
	if packageGlobalFunctionObjectInit(expr, targetType) {
		out.WriteString("Box::new(")
		TranspileExpression(out, expr)
		out.WriteString(")")
		return
	}
	if writeConstExpressionForExpectedGoType(out, expr, targetType) {
		return
	}
	if ident, ok := expr.(*ast.Ident); ok && isWrappedValueIdent(ident) {
		writeIdentValueClone(out, ident)
		return
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		if ident, ok := call.Fun.(*ast.Ident); ok && (ident.Name == "len" || ident.Name == "cap") {
			TranspileExpression(out, expr)
			return
		}
		typeInfo := GetTypeInfo()
		if typeInfo != nil && typeInfo.ReturnsWrappedValue(call) && !callReturnsBareChannelValue(call) {
			if isPointerGlobalType(targetType) {
				TranspileExpression(out, expr)
				return
			}
			out.WriteString("(*")
			TranspileExpression(out, expr)
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()).clone()")
			return
		}
	}
	if isAssignmentSelfWrappingExpression(expr) {
		out.WriteString("(*")
		TranspileExpression(out, expr)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).clone()")
		return
	}
	TranspileExpression(out, expr)
}

func packageGlobalFunctionObjectInit(expr ast.Expr, targetType types.Type) bool {
	if targetType == nil {
		return false
	}
	if _, ok := types.Unalias(targetType).Underlying().(*types.Signature); !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	switch e := expr.(type) {
	case *ast.Ident:
		_, ok := typeInfo.GetObject(e).(*types.Func)
		return ok
	case *ast.SelectorExpr:
		_, ok := typeInfo.GetObject(e.Sel).(*types.Func)
		return ok
	}
	return false
}

func TranspilePackageInitAll(out *strings.Builder, hasGlobals bool, initFunctionNames map[*ast.FuncDecl]string) {
	out.WriteString("pub(crate) fn __go_init_all() {\n")
	if hasGlobals {
		out.WriteString("    self::__go_init_globals();\n")
	}
	initNames := make([]packageFunctionName, 0, len(initFunctionNames))
	for fn, name := range initFunctionNames {
		if fn.Name.Name != "init" {
			continue
		}
		initNames = append(initNames, packageFunctionName{
			rustName: name,
			pos:      fn.Pos(),
		})
	}
	sort.Slice(initNames, func(i, j int) bool {
		return initNames[i].pos < initNames[j].pos
	})
	for _, initName := range initNames {
		out.WriteString("    self::")
		out.WriteString(initName.rustName)
		out.WriteString("();\n")
	}
	out.WriteString("}\n")
}
