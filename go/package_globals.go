package main

import (
	"fmt"
	"go/ast"
	"go/types"
	"strings"
)

var functionNameOverrides map[*ast.FuncDecl]string
var functionNameOverridesByGoName map[string]string
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
	return RustFunctionName(fn.Name.Name)
}

func rustFunctionNameForUse(name string) string {
	if functionNameOverridesByGoName != nil {
		if rustName, ok := functionNameOverridesByGoName[name]; ok {
			return rustName
		}
	}
	return RustFunctionName(name)
}

func assignFunctionNames(functions []*ast.FuncDecl) map[*ast.FuncDecl]string {
	names := make(map[*ast.FuncDecl]string)
	used := make(map[string]int)
	initIndex := 0
	for _, fn := range functions {
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

type packageGlobal struct {
	name     string
	rustType string
	typ      types.Type
}

func isPackageGlobalIdent(ident *ast.Ident) bool {
	if ident == nil || ident.Name == "_" || ident.Name == "nil" || ident.Name == "true" || ident.Name == "false" {
		return false
	}
	if vt := GetVarTable(); vt != nil {
		if vt.Lookup(ident.Name) != nil {
			return false
		}
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || typeInfo.pkg == nil {
		return false
	}
	obj := typeInfo.GetObject(ident)
	if _, ok := obj.(*types.Var); !ok {
		return false
	}
	return obj.Parent() == typeInfo.pkg.Scope()
}

func collectPackageGlobals(globalVars []*ast.GenDecl) []packageGlobal {
	typeInfo := GetTypeInfo()
	globals := make([]packageGlobal, 0)
	for _, genDecl := range globalVars {
		for _, spec := range genDecl.Specs {
			valueSpec, ok := spec.(*ast.ValueSpec)
			if !ok {
				continue
			}
			for _, name := range valueSpec.Names {
				if name.Name == "_" {
					continue
				}
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
			}
		}
	}
	return globals
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
			out.WriteString("pub(crate) static ")
			out.WriteString(global.name)
			out.WriteString(": std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<")
			out.WriteString(global.rustType)
			out.WriteString(">>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));\n")
		} else {
			out.WriteString("pub(crate) static ")
			out.WriteString(global.name)
			out.WriteString(": GoGlobal<")
			out.WriteString(global.rustType)
			out.WriteString("> = GoGlobal::new();\n")
		}
	}
	out.WriteString("\n\n")
	transpilePackageGlobalInit(out, globals)
}

func transpilePackageGlobalInit(out *strings.Builder, globals []packageGlobal) {
	out.WriteString("pub(crate) fn __go_init_globals() {\n")
	for _, global := range globals {
		if global.typ != nil {
			if _, isFunc := global.typ.Underlying().(*types.Signature); isFunc {
				continue
			}
		}
		out.WriteString("    *")
		out.WriteString(global.name)
		WriteBorrowMethod(out, true)
		out.WriteString(" = Some(")
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

	globalNames := make(map[string]bool, len(globals))
	for _, global := range globals {
		globalNames[global.name] = true
	}
	for _, init := range typeInfo.info.InitOrder {
		if len(init.Lhs) == 0 || !globalNames[init.Lhs[0].Name()] {
			continue
		}
		if len(init.Lhs) != 1 {
			out.WriteString("    /* ERROR: Type information required for multi-value package variable initialization */\n")
			out.WriteString("    unimplemented!();\n")
			continue
		}
		out.WriteString("    *")
		out.WriteString(init.Lhs[0].Name())
		WriteBorrowMethod(out, true)
		out.WriteString(" = Some(")
		writePackageGlobalInitValue(out, init.Rhs)
		out.WriteString(");\n")
	}
	out.WriteString("}\n")
}

func writePackageGlobalInitValue(out *strings.Builder, expr ast.Expr) {
	if funcLit, ok := expr.(*ast.FuncLit); ok {
		TranspileFuncLitBox(out, funcLit)
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

func TranspilePackageInitAll(out *strings.Builder, hasGlobals bool, initFunctionNames map[*ast.FuncDecl]string) {
	out.WriteString("pub(crate) fn __go_init_all() {\n")
	if hasGlobals {
		out.WriteString("    __go_init_globals();\n")
	}
	initNames := make([]string, 0, len(initFunctionNames))
	for _, name := range initFunctionNames {
		initNames = append(initNames, name)
	}
	for i := 0; i < len(initNames); i++ {
		name := fmt.Sprintf("__go_init_%d", i)
		out.WriteString("    ")
		out.WriteString(name)
		out.WriteString("();\n")
	}
	out.WriteString("}\n")
}
