package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"go/types"
	"slices"
	"sort"
	"strings"
)

// These maps track the names and types of special variables that shouldn't be unwrapped
var rangeLoopVars = make(map[string]string)
var localConstants = make(map[string]string)
var packageConstants = make(map[string]string)

// localInterfaces tracks locally-defined interface type names (inside functions)
var localInterfaces = make(map[string]bool)

// currentReceiver tracks the current method receiver name for self translation
var currentReceiver string

// currentReceiverType tracks the type of the current method receiver
var currentReceiverType string

// currentTypeMethods tracks the current impl block's method set for receiver self-call analysis
var currentTypeMethods []*ast.FuncDecl

// currentFunctionHasDefer tracks if the current function has defer statements
var currentFunctionHasDefer bool

// currentCaptureRenames tracks variable renames for captured variables in closures
var currentCaptureRenames map[string]string

// statementPreprocessor analyzes statements for closure captures
var statementPreprocessor *StatementPreprocessor

// pendingLoopLabel is set by LabeledStmt and consumed by ForStmt/RangeStmt
var pendingLoopLabel string

// breakTargetStack tracks synthetic Rust labels for unlabeled Go breaks whose
// nearest breakable construct is not represented by a Rust loop.
var breakTargetStack []string

// switchBreakLabelCounter generates unique labels for switch one-shot loops.
var switchBreakLabelCounter int

// hasInitFunction tracks whether the current file has an init() function
var hasInitFunction bool

// labeledLoopPost maps loop labels to their ForStmt Post statements.
// Used to emit the post-statement before `continue 'label` in Rust,
// since Go's `continue label` executes the post-statement but Rust's doesn't.
var labeledLoopPost = make(map[string]ast.Stmt)

// interfaceTypes tracks which type names are interfaces
var interfaceTypes = make(map[string]bool)

// typeDefinitions tracks which types are type definitions (not aliases)
var typeDefinitions = make(map[string]string) // maps type name to underlying type

// typeAliases tracks which types are type aliases
var typeAliases = make(map[string]bool)

// functionTypeAliases tracks named Go function types that are emitted as Rust aliases.
var functionTypeAliases = make(map[string]bool)

// comparableStructTypes tracks named structs that appear in == or != expressions.
var comparableStructTypes = make(map[string]bool)

// localInterfaceEqualityTypes tracks local interfaces used in ==, !=, or
// slices.Contains comparisons in the current file.
var localInterfaceEqualityTypes = make(map[string]bool)

// goPackageImports tracks imported Go packages for the current file
// map[alias]packagePath (alias can be empty for default)
var goPackageImports = make(map[string]string)

// externalPackages tracks external (non-stdlib) package imports
var externalPackages = make(map[string]bool)

// SetPackageImports sets the package imports for the current transpilation
func SetPackageImports(imports map[string]string) {
	goPackageImports = imports
	externalPackages = make(map[string]bool)
	// Also track external packages
	for _, path := range imports {
		if !isStdlibPackage(path) {
			externalPackages[path] = true
		}
	}
	if currentContext != nil && currentContext.Package != nil {
		currentContext.Package.GoPackageImports = goPackageImports
		currentContext.Package.ExternalPackages = externalPackages
	}
}

// structDefs tracks struct definitions and their fields
type StructDef struct {
	Fields        map[string]string // field name -> field type
	EmbeddedTypes []string          // list of embedded type names
	ASTType       *ast.StructType   // original AST type for zero-value generation
}

var structDefs = make(map[string]*StructDef)

// embeddedFields tracks which fields come from embedded structs
// map[structType][fieldName] -> embeddedTypeName
var embeddedFields = make(map[string]map[string]string)

// FieldAccessInfo describes how to access a field, including through embedded structs
type FieldAccessInfo struct {
	Found        bool     // True if the field was actually found in the struct or its embeds
	IsPromoted   bool     // True if field comes from embedded struct
	EmbeddedPath []string // Path of embedded types to traverse (e.g., ["B", "A"] for C.B.A)
	FieldName    string   // The actual field name (snake_case)
}

func collectComparableStructTypes(file *ast.File) map[string]bool {
	result := make(map[string]bool)
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return result
	}

	ast.Inspect(file, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.BinaryExpr:
			if n.Op != token.EQL && n.Op != token.NEQ {
				return true
			}
			markComparableStructType(result, typeInfo.GetType(n.X))
			markComparableStructType(result, typeInfo.GetType(n.Y))
		case *ast.MapType:
			markMapKeyStructType(result, mapKeyTypeFromMapType(typeInfo, n), typeInfo)
		}
		return true
	})

	return result
}

func collectMapKeyStructTypesFromFiles(files []*ast.File, typeInfo *TypeInfo) map[string]bool {
	result := make(map[string]bool)
	for _, file := range files {
		ast.Inspect(file, func(node ast.Node) bool {
			mapType, ok := node.(*ast.MapType)
			if !ok {
				return true
			}
			markMapKeyStructType(result, mapKeyTypeFromMapType(typeInfo, mapType), typeInfo)
			return true
		})
	}
	return result
}

func mapKeyTypeFromMapType(typeInfo *TypeInfo, mapType *ast.MapType) types.Type {
	if typeInfo == nil || mapType == nil {
		return nil
	}
	if typ := typeInfo.GetType(mapType); typ != nil {
		if typedMap, ok := types.Unalias(typ).Underlying().(*types.Map); ok {
			return typedMap.Key()
		}
	}
	return typeInfo.GetType(mapType.Key)
}

func collectLocalInterfaceEqualityTypes(file *ast.File) map[string]bool {
	result := make(map[string]bool)
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return result
	}

	ast.Inspect(file, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.BinaryExpr:
			if n.Op != token.EQL && n.Op != token.NEQ {
				return true
			}
			if name, ok := localNamedInterfaceTypeNameFromTypes(typeInfo.GetType(n.X)); ok {
				result[name] = true
			}
			if name, ok := localNamedInterfaceTypeNameFromTypes(typeInfo.GetType(n.Y)); ok {
				result[name] = true
			}
		case *ast.CallExpr:
			if !isSlicesContainsCall(n) || len(n.Args) < 2 {
				return true
			}
			if name, ok := localInterfaceSliceElemName(typeInfo.GetType(n.Args[0])); ok {
				result[name] = true
			}
		}
		return true
	})

	return result
}

func isSlicesContainsCall(call *ast.CallExpr) bool {
	sel, ok := call.Fun.(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "Contains" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	ident, ok := sel.X.(*ast.Ident)
	if !ok {
		return false
	}
	pkgName, ok := typeInfo.info.Uses[ident].(*types.PkgName)
	return ok && pkgName.Imported() != nil && pkgName.Imported().Path() == "slices"
}

func localInterfaceSliceElemName(typ types.Type) (string, bool) {
	switch t := types.Unalias(typ).(type) {
	case *types.Slice:
		return localNamedInterfaceTypeNameFromTypes(t.Elem())
	case *types.Array:
		return localNamedInterfaceTypeNameFromTypes(t.Elem())
	default:
		return "", false
	}
}

func markComparableStructType(result map[string]bool, typ types.Type) {
	if typ == nil {
		return
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok {
		return
	}
	if _, ok := named.Underlying().(*types.Struct); ok {
		result[named.Obj().Name()] = true
	}
}

func markMapKeyStructType(result map[string]bool, typ types.Type, typeInfo *TypeInfo) {
	if typ == nil {
		return
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil {
		return
	}
	if typeInfo != nil && typeInfo.pkg != nil && named.Obj().Pkg() != typeInfo.pkg {
		return
	}
	if _, ok := named.Underlying().(*types.Struct); ok {
		result[named.Obj().Name()] = true
	}
}

func markComparableInterfaceImplementorStructs(typeDecls []struct {
	spec *ast.TypeSpec
	decl *ast.GenDecl
}, methods map[string][]*ast.FuncDecl, interfaces map[string]*ast.InterfaceType) {
	if NeedsConcurrentWrapper() {
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	for _, decl := range typeDecls {
		structType, isStruct := decl.spec.Type.(*ast.StructType)
		if !isStruct || structHasTraitField(structType) {
			continue
		}
		obj, ok := typeInfo.info.Defs[decl.spec.Name].(*types.TypeName)
		if !ok || obj == nil || obj.Type() == nil || !types.Comparable(obj.Type()) {
			continue
		}
		for _, ifaceType := range interfaces {
			if implementsInterface(methods[decl.spec.Name.Name], ifaceType) {
				comparableStructTypes[decl.spec.Name.Name] = true
				break
			}
		}
	}
}

func importedTranspiledInterfaceFromType(typ types.Type) (string, *types.Interface, bool) {
	if typ == nil {
		return "", nil, false
	}
	named, ok := types.Unalias(typ).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return "", nil, false
	}
	intf, ok := named.Underlying().(*types.Interface)
	if !ok || intf.NumMethods() == 0 {
		return "", nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil && named.Obj().Pkg() == typeInfo.pkg {
		return "", nil, false
	}
	if isStdlibPackage(named.Obj().Pkg().Path()) {
		return "", nil, false
	}
	return goTypesNamedTypeToRust(named), intf, true
}

func currentPackageConcreteTypeName(typ types.Type) (string, bool) {
	if typ == nil {
		return "", false
	}
	typ = types.Unalias(typ)
	if ptr, ok := typ.(*types.Pointer); ok {
		typ = types.Unalias(ptr.Elem())
	}
	named, ok := typ.(*types.Named)
	if !ok || named.Obj() == nil {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || named.Obj().Pkg() != typeInfo.pkg {
		return "", false
	}
	return named.Obj().Name(), true
}

func collectImportedInterfaceImpls(file *ast.File) map[string]map[string]*types.Interface {
	if file == nil {
		return nil
	}
	return collectImportedInterfaceImplsFromFiles([]*ast.File{file})
}

func collectImportedInterfaceImplsFromFiles(files []*ast.File) map[string]map[string]*types.Interface {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	impls := make(map[string]map[string]*types.Interface)
	record := func(expected types.Type, arg ast.Expr) {
		if expected == nil || arg == nil {
			return
		}
		ifaceName, ifaceType, ok := importedTranspiledInterfaceFromType(expected)
		if !ok {
			return
		}
		argType := typeInfo.GetType(arg)
		if argType == nil || !types.Implements(argType, ifaceType) {
			return
		}
		typeName, ok := currentPackageConcreteTypeName(argType)
		if !ok {
			return
		}
		if impls[typeName] == nil {
			impls[typeName] = make(map[string]*types.Interface)
		}
		impls[typeName][ifaceName] = ifaceType
	}
	for _, file := range files {
		ast.Inspect(file, func(n ast.Node) bool {
			call, ok := n.(*ast.CallExpr)
			if !ok {
				return true
			}
			for i, arg := range call.Args {
				record(callParamTypeFromTypeInfo(call, i), arg)
				if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
					record(selectedMethodParamType(sel, i), arg)
				}
			}
			return true
		})
	}
	return impls
}

func importedInterfaceImplsForFile(file *ast.File) map[string]map[string]*types.Interface {
	if ctx := GetTranspileContext(); ctx != nil && ctx.Package != nil && len(ctx.Package.ImportedInterfaceImpls) > 0 {
		return ctx.Package.ImportedInterfaceImpls
	}
	return collectImportedInterfaceImpls(file)
}

type externalLocalInterfaceImpl struct {
	ifaceAST *ast.InterfaceType
}

type localInterfaceAssertionCandidate struct {
	rustType string
	external bool
	typ      types.Type
}

func collectFunctionLocalInterfaces(file *ast.File) map[string]*ast.InterfaceType {
	result := make(map[string]*ast.InterfaceType)
	if file == nil {
		return result
	}
	for _, decl := range file.Decls {
		fn, ok := decl.(*ast.FuncDecl)
		if !ok || fn.Body == nil {
			continue
		}
		ast.Inspect(fn.Body, func(n ast.Node) bool {
			genDecl, ok := n.(*ast.GenDecl)
			if !ok || genDecl.Tok != token.TYPE {
				return true
			}
			for _, spec := range genDecl.Specs {
				typeSpec, ok := spec.(*ast.TypeSpec)
				if !ok {
					continue
				}
				if iface, ok := typeSpec.Type.(*ast.InterfaceType); ok {
					result[typeSpec.Name.Name] = iface
				}
			}
			return true
		})
	}
	return result
}

func sourceAllowsInterfaceAssertionCandidate(candidate types.Type, source types.Type) bool {
	if source == nil {
		return true
	}
	source = types.Unalias(source)
	if iface, ok := source.Underlying().(*types.Interface); ok {
		if iface.NumMethods() == 0 {
			return true
		}
		iface.Complete()
		return types.Implements(candidate, iface)
	}
	return types.AssignableTo(candidate, source)
}

func localInterfaceAssertionCandidates(ifaceType *types.Interface, sourceType types.Type) []localInterfaceAssertionCandidate {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || ifaceType == nil {
		return nil
	}
	ifaceType.Complete()
	seen := make(map[string]bool)
	var candidates []localInterfaceAssertionCandidate
	recordNamed := func(named *types.Named) {
		if named == nil || named.Obj() == nil {
			return
		}
		if _, isInterface := named.Underlying().(*types.Interface); isInterface {
			return
		}
		forms := []types.Type{named, types.NewPointer(named)}
		var matchedForm types.Type
		for _, form := range forms {
			if types.Implements(form, ifaceType) && sourceAllowsInterfaceAssertionCandidate(form, sourceType) {
				matchedForm = form
				break
			}
		}
		if matchedForm == nil {
			return
		}
		rustType := goTypesNamedTypeToRust(named)
		if rustType == "" || seen[rustType] {
			return
		}
		seen[rustType] = true
		external := typeInfo.pkg == nil || named.Obj().Pkg() != typeInfo.pkg
		candidates = append(candidates, localInterfaceAssertionCandidate{
			rustType: rustType,
			external: external,
			typ:      matchedForm,
		})
	}
	visitPackage := func(pkg *types.Package) {
		if pkg == nil || pkg.Scope() == nil {
			return
		}
		for _, name := range pkg.Scope().Names() {
			obj, ok := pkg.Scope().Lookup(name).(*types.TypeName)
			if !ok {
				continue
			}
			if named, ok := types.Unalias(obj.Type()).(*types.Named); ok {
				recordNamed(named)
			}
		}
	}
	visitPackage(typeInfo.pkg)
	if typeInfo.pkg != nil {
		for _, pkg := range typeInfo.pkg.Imports() {
			visitPackage(pkg)
		}
	}
	slices.SortFunc(candidates, func(a, b localInterfaceAssertionCandidate) int {
		return strings.Compare(a.rustType, b.rustType)
	})
	return candidates
}

func localInterfaceAssertionTarget(e *ast.TypeAssertExpr) (string, *types.Interface, types.Type, []localInterfaceAssertionCandidate, bool) {
	if e == nil || e.Type == nil {
		return "", nil, nil, nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", nil, nil, nil, false
	}
	targetType := typeInfo.GetType(e.Type)
	ifaceName, ok := localNamedInterfaceTypeNameFromTypes(targetType)
	if !ok {
		if ident, identOK := e.Type.(*ast.Ident); identOK && IsInterfaceType(ident.Name) {
			ifaceName = ident.Name
			ok = true
		}
	}
	if !ok {
		return "", nil, nil, nil, false
	}
	if !localInterfaces[ifaceName] {
		return "", nil, nil, nil, false
	}
	named, ok := types.Unalias(targetType).(*types.Named)
	if !ok {
		return "", nil, nil, nil, false
	}
	ifaceType, ok := named.Underlying().(*types.Interface)
	if !ok || ifaceType.NumMethods() == 0 {
		return "", nil, nil, nil, false
	}
	sourceType := typeInfo.GetType(e.X)
	candidates := localInterfaceAssertionCandidates(ifaceType, sourceType)
	return ifaceName, ifaceType, sourceType, candidates, true
}

func collectExternalLocalInterfaceImpls(file *ast.File, interfaces map[string]*ast.InterfaceType) map[string]map[string]externalLocalInterfaceImpl {
	typeInfo := GetTypeInfo()
	if file == nil || typeInfo == nil || typeInfo.pkg == nil {
		return nil
	}
	impls := make(map[string]map[string]externalLocalInterfaceImpl)
	record := func(ifaceName string, ifaceType *types.Interface, concrete types.Type) {
		if ifaceName == "" || ifaceType == nil || concrete == nil {
			return
		}
		if !types.Implements(concrete, ifaceType) {
			return
		}
		concreteType := concrete
		if ptr, ok := concreteType.(*types.Pointer); ok {
			concreteType = ptr.Elem()
		}
		named, ok := types.Unalias(concreteType).(*types.Named)
		if !ok || named.Obj() == nil || named.Obj().Pkg() == nil || named.Obj().Pkg() == typeInfo.pkg {
			return
		}
		rustType := goTypesNamedTypeToRust(named)
		if impls[ifaceName] == nil {
			impls[ifaceName] = make(map[string]externalLocalInterfaceImpl)
		}
		impls[ifaceName][rustType] = externalLocalInterfaceImpl{
			ifaceAST: interfaces[ifaceName],
		}
	}

	ast.Inspect(file, func(n ast.Node) bool {
		switch node := n.(type) {
		case *ast.TypeAssertExpr:
			ifaceName, ifaceType, _, candidates, ok := localInterfaceAssertionTarget(node)
			if !ok {
				return true
			}
			for _, candidate := range candidates {
				if candidate.external {
					record(ifaceName, ifaceType, candidate.typ)
				}
			}
		case *ast.TypeSwitchStmt:
			if node.Assign == nil {
				return true
			}
			var expr ast.Expr
			switch assign := node.Assign.(type) {
			case *ast.ExprStmt:
				if typeAssert, ok := assign.X.(*ast.TypeAssertExpr); ok {
					expr = typeAssert.X
				}
			case *ast.AssignStmt:
				if len(assign.Rhs) == 1 {
					if typeAssert, ok := assign.Rhs[0].(*ast.TypeAssertExpr); ok {
						expr = typeAssert.X
					}
				}
			}
			if expr == nil {
				return true
			}
			subjectType := typeInfo.GetType(expr)
			ifaceName, ok := localNamedInterfaceTypeNameFromTypes(subjectType)
			if !ok {
				return true
			}
			named, ok := types.Unalias(subjectType).(*types.Named)
			if !ok {
				return true
			}
			ifaceType, ok := named.Underlying().(*types.Interface)
			if !ok {
				return true
			}
			ifaceType.Complete()
			for _, clauseNode := range node.Body.List {
				clause := clauseNode.(*ast.CaseClause)
				for _, caseExpr := range clause.List {
					if ident, ok := caseExpr.(*ast.Ident); ok && ident.Name == "nil" {
						continue
					}
					record(ifaceName, ifaceType, typeInfo.GetType(caseExpr))
				}
			}
		}
		return true
	})
	return impls
}

func typeMethodsImplementTypesInterface(typeMethods []*ast.FuncDecl, iface *types.Interface) bool {
	if iface == nil {
		return false
	}
	for i := 0; i < iface.NumMethods(); i++ {
		methodName := iface.Method(i).Name()
		found := false
		for _, typeMethod := range typeMethods {
			if typeMethod.Name.Name == methodName {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	return true
}

func methodDeclByName(typeMethods []*ast.FuncDecl, name string) *ast.FuncDecl {
	for _, method := range typeMethods {
		if method.Name.Name == name {
			return method
		}
	}
	return nil
}

func writeExternalLocalInterfaceMethod(out *strings.Builder, methodName string, funcType *ast.FuncType) {
	out.WriteString("    fn ")
	out.WriteString(ToSnakeCase(methodName))
	out.WriteString("(&self")
	argNames := make([]string, 0)
	if funcType.Params != nil {
		argIndex := 0
		for _, param := range funcType.Params.List {
			names := param.Names
			if len(names) == 0 {
				names = []*ast.Ident{ast.NewIdent(fmt.Sprintf("arg%d", argIndex))}
			}
			for _, name := range names {
				argNames = append(argNames, RustLocalIdent(name.Name))
				out.WriteString(", ")
				out.WriteString(RustLocalIdent(name.Name))
				out.WriteString(": ")
				out.WriteString(GoTypeToRustParam(param.Type))
				argIndex++
			}
		}
	}
	out.WriteString(")")
	if funcType.Results != nil && len(funcType.Results.List) > 0 {
		out.WriteString(" -> ")
		if len(funcType.Results.List) == 1 && len(funcType.Results.List[0].Names) <= 1 {
			out.WriteString(GoTypeToRust(funcType.Results.List[0].Type))
		} else {
			out.WriteString("(")
			first := true
			for _, result := range funcType.Results.List {
				count := len(result.Names)
				if count == 0 {
					count = 1
				}
				for range count {
					if !first {
						out.WriteString(", ")
					}
					first = false
					out.WriteString(GoTypeToRust(result.Type))
				}
			}
			out.WriteString(")")
		}
	}
	out.WriteString(" {\n")
	out.WriteString("        self.")
	out.WriteString(ToSnakeCase(methodName))
	out.WriteString("(")
	for i, argName := range argNames {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(argName)
	}
	out.WriteString(")\n")
	out.WriteString("    }\n")
}

func writeExternalLocalInterfaceSupportImpl(out *strings.Builder, ifaceName, concreteType string) {
	TrackImport("Any")
	out.WriteString("    fn __go_clone_box(&self) -> ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString(" {\n")
	out.WriteString("        Box::new(self.clone()) as ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString("\n")
	out.WriteString("    }\n")
	out.WriteString("    fn __go_as_any(&self) -> &dyn Any {\n")
	out.WriteString("        self\n")
	out.WriteString("    }\n")
	out.WriteString("    fn __go_eq(&self, other: ")
	out.WriteString(rustLocalInterfaceParam(ifaceName))
	out.WriteString(") -> bool {\n")
	out.WriteString("        if let Some(_other) = other.__go_as_any().downcast_ref::<")
	out.WriteString(concreteType)
	out.WriteString(">() {\n")
	out.WriteString("            false\n")
	out.WriteString("        } else {\n")
	out.WriteString("            false\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

func writeExternalLocalInterfaceImpls(out *strings.Builder, first *bool, impls map[string]map[string]externalLocalInterfaceImpl) {
	if len(impls) == 0 {
		return
	}
	var ifaceNames []string
	for ifaceName := range impls {
		ifaceNames = append(ifaceNames, ifaceName)
	}
	slices.Sort(ifaceNames)
	for _, ifaceName := range ifaceNames {
		var concreteTypes []string
		for concreteType := range impls[ifaceName] {
			concreteTypes = append(concreteTypes, concreteType)
		}
		slices.Sort(concreteTypes)
		for _, concreteType := range concreteTypes {
			impl := impls[ifaceName][concreteType]
			if !*first {
				out.WriteString("\n\n")
			}
			*first = false
			out.WriteString("impl ")
			out.WriteString(ifaceName)
			out.WriteString(" for ")
			out.WriteString(concreteType)
			out.WriteString(" {\n")
			if impl.ifaceAST != nil {
				for _, method := range impl.ifaceAST.Methods.List {
					if len(method.Names) == 0 {
						continue
					}
					funcType, ok := method.Type.(*ast.FuncType)
					if !ok {
						continue
					}
					writeExternalLocalInterfaceMethod(out, method.Names[0].Name, funcType)
				}
			}
			writeExternalLocalInterfaceSupportImpl(out, ifaceName, concreteType)
			out.WriteString("}")
		}
	}
}

// trackGoImport tracks a Go import statement
func trackGoImport(packagePath string, nameIdent *ast.Ident) {
	// Determine the alias (if any)
	var alias string
	if nameIdent != nil {
		if nameIdent.Name == "_" {
			// Blank import - ignore for now
			return
		}
		alias = nameIdent.Name
	} else {
		// Default alias is the last component of the path
		parts := strings.Split(packagePath, "/")
		alias = parts[len(parts)-1]
	}

	// Track the import
	goPackageImports[alias] = packagePath

	// Check if it's an external package (not stdlib)
	if !isStdlibPackage(packagePath) {
		externalPackages[packagePath] = true
	}
}

func resolveStdlibPackageName(importName string) string {
	if packagePath, exists := goPackageImports[importName]; exists && isStdlibPackage(packagePath) {
		return packagePath
	}
	return importName
}

// resolveFieldAccess finds the path to access a field, considering embedded structs
func resolveFieldAccess(structType string, fieldName string) FieldAccessInfo {
	// Check if it's a direct field
	if structDef, exists := structDefs[structType]; exists {
		if _, isDirectField := structDef.Fields[fieldName]; isDirectField {
			return FieldAccessInfo{
				Found:      true,
				IsPromoted: false,
				FieldName:  ToSnakeCase(fieldName),
			}
		}

		// Check embedded types (recursively)
		for _, embeddedType := range structDef.EmbeddedTypes {
			// First check if the embedded type itself has the field
			if embeddedDef, exists := structDefs[embeddedType]; exists {
				if _, hasField := embeddedDef.Fields[fieldName]; hasField {
					// Field found directly in embedded struct
					return FieldAccessInfo{
						Found:        true,
						IsPromoted:   true,
						EmbeddedPath: []string{embeddedType},
						FieldName:    ToSnakeCase(fieldName),
					}
				}
			}

			// Recursively check fields promoted through the embedded type
			embeddedInfo := resolveFieldAccess(embeddedType, fieldName)
			if embeddedInfo.Found {
				// Field was found in nested embedded struct
				// Build the path through our embedded type
				path := []string{embeddedType}
				if embeddedInfo.IsPromoted {
					// Append the rest of the path
					path = append(path, embeddedInfo.EmbeddedPath...)
				}
				return FieldAccessInfo{
					Found:        true,
					IsPromoted:   true,
					EmbeddedPath: path,
					FieldName:    ToSnakeCase(fieldName),
				}
			}
		}
	}

	// Default: field not found, assume direct field access
	return FieldAccessInfo{
		Found:      false,
		IsPromoted: false,
		FieldName:  ToSnakeCase(fieldName),
	}
}

// collectPromotedMethods recursively collects all methods that should be promoted from embedded types
func collectPromotedMethods(structDef *StructDef, methods map[string][]*ast.FuncDecl, promoted map[string]struct {
	embeddedType string
	method       *ast.FuncDecl
}) {
	// Check direct embedded types
	for _, embeddedType := range structDef.EmbeddedTypes {
		// Get methods from the embedded type
		if embeddedMethods, hasEmbedded := methods[embeddedType]; hasEmbedded {
			for _, embMethod := range embeddedMethods {
				// Only add if not already promoted (first embed wins)
				if _, exists := promoted[embMethod.Name.Name]; !exists {
					promoted[embMethod.Name.Name] = struct {
						embeddedType string
						method       *ast.FuncDecl
					}{
						embeddedType: embeddedType,
						method:       embMethod,
					}
				}
			}
		}

		// Recursively collect from embedded type's embedded types
		if embeddedDef, exists := structDefs[embeddedType]; exists {
			// Create a map for the embedded type's promoted methods
			embeddedPromoted := make(map[string]struct {
				embeddedType string
				method       *ast.FuncDecl
			})
			collectPromotedMethods(embeddedDef, methods, embeddedPromoted)

			// Add these to our promoted methods (but they're promoted through the embedded type)
			for methodName, methodInfo := range embeddedPromoted {
				if _, exists := promoted[methodName]; !exists {
					// Note: we promote through the direct embedded type, not the nested one
					promoted[methodName] = struct {
						embeddedType string
						method       *ast.FuncDecl
					}{
						embeddedType: embeddedType, // Use the direct embedded type
						method:       methodInfo.method,
					}
				}
			}
		}
	}
}

// generatePromotedMethod generates a forwarding method that delegates to an embedded type's method
func generatePromotedMethod(out *strings.Builder, method *ast.FuncDecl, embeddedTypeName string) {
	out.WriteString("    pub fn ")
	out.WriteString(rustMethodName(method))
	out.WriteString("(")

	// Receiver
	if method.Recv != nil && len(method.Recv.List) > 0 {
		recv := method.Recv.List[0]
		// Check if pointer receiver
		if _, isPointer := recv.Type.(*ast.StarExpr); isPointer {
			out.WriteString("&mut self")
		} else {
			out.WriteString("&self")
		}

		// Add comma if there are more parameters
		if method.Type.Params != nil && len(method.Type.Params.List) > 0 {
			out.WriteString(", ")
		}
	}

	// Other parameters
	if method.Type.Params != nil {
		for i, field := range method.Type.Params.List {
			if i > 0 {
				out.WriteString(", ")
			}
			for j, name := range field.Names {
				if j > 0 {
					out.WriteString(", ")
				}
				out.WriteString(name.Name)
				out.WriteString(": ")
				out.WriteString(GoTypeToRust(field.Type))
			}
		}
	}

	out.WriteString(")")

	// Return type
	if method.Type.Results != nil && len(method.Type.Results.List) > 0 {
		out.WriteString(" -> ")
		if len(method.Type.Results.List) == 1 && len(method.Type.Results.List[0].Names) <= 1 {
			// Single return value
			out.WriteString(GoTypeToRust(method.Type.Results.List[0].Type))
		} else {
			// Multiple return values - use tuple
			out.WriteString("(")
			first := true
			for _, result := range method.Type.Results.List {
				// Handle multiple names with same type
				if len(result.Names) > 0 {
					for range result.Names {
						if !first {
							out.WriteString(", ")
						}
						first = false
						out.WriteString(GoTypeToRust(result.Type))
					}
				} else {
					// No name, just type
					if !first {
						out.WriteString(", ")
					}
					first = false
					out.WriteString(GoTypeToRust(result.Type))
				}
			}
			out.WriteString(")")
		}
	}

	out.WriteString(" {\n")
	out.WriteString("        // Forward to embedded type's method\n")
	out.WriteString("        let embedded = self.")
	out.WriteString(ToSnakeCase(embeddedTypeName))
	out.WriteString(".clone();\n")
	out.WriteString("        let mut guard = embedded")
	WriteBorrowMethod(out, true)
	out.WriteString(";\n")
	out.WriteString("        let embedded_ref = guard.as_mut().unwrap();\n")
	out.WriteString("        embedded_ref.")
	out.WriteString(rustMethodName(method))
	out.WriteString("(")

	// Pass through parameters
	if method.Type.Params != nil {
		for i, field := range method.Type.Params.List {
			if i > 0 {
				out.WriteString(", ")
			}
			for j, name := range field.Names {
				if j > 0 {
					out.WriteString(", ")
				}
				out.WriteString(name.Name)
			}
		}
	}

	out.WriteString(")\n")
	out.WriteString("    }\n")
}

func generateExternalPromotedMethod(out *strings.Builder, method externalPromotedMethod) {
	sig := method.Signature
	if sig == nil {
		return
	}
	params := sig.Params()
	results := sig.Results()

	out.WriteString("    pub fn ")
	out.WriteString(method.RustMethodName)
	if method.GenericArguments && params.Len() > 0 {
		out.WriteString("<")
		for i := 0; i < params.Len(); i++ {
			if i > 0 {
				out.WriteString(", ")
			}
			fmt.Fprintf(out, "T%d", i)
		}
		out.WriteString(">")
	}
	out.WriteString("(&self")
	for i := 0; i < params.Len(); i++ {
		if method.GenericArguments {
			fmt.Fprintf(out, ", _arg%d: T%d", i, i)
		} else {
			fmt.Fprintf(out, ", _arg%d: %s", i, goTypesParamTypeToRust(params.At(i).Type()))
		}
	}
	out.WriteString(")")
	if results.Len() > 0 {
		out.WriteString(" -> ")
		if results.Len() == 1 {
			out.WriteString(goTypesReturnTypeToRust(results.At(0).Type()))
		} else {
			out.WriteString("(")
			for i := 0; i < results.Len(); i++ {
				if i > 0 {
					out.WriteString(", ")
				}
				out.WriteString(goTypesReturnTypeToRust(results.At(i).Type()))
			}
			out.WriteString(")")
		}
	}
	out.WriteString(" {\n")
	out.WriteString("        let embedded = self.")
	out.WriteString(method.EmbeddedFieldName)
	out.WriteString(".clone();\n")
	if method.MutableReceiver {
		out.WriteString("        let mut guard = embedded")
		WriteBorrowMethod(out, true)
		out.WriteString(";\n")
		out.WriteString("        let embedded_ref = guard.as_mut().unwrap();\n")
	} else {
		out.WriteString("        let guard = embedded")
		WriteBorrowMethod(out, false)
		out.WriteString(";\n")
		out.WriteString("        let embedded_ref = guard.as_ref().unwrap();\n")
	}
	out.WriteString("        embedded_ref.")
	out.WriteString(method.RustMethodName)
	out.WriteString("(")
	for i := 0; i < params.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		fmt.Fprintf(out, "_arg%d", i)
	}
	out.WriteString(")\n")
	out.WriteString("    }\n")
}

// getReceiverType extracts the type name from a receiver type expression
func getReceiverType(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		return t.Name
	case *ast.StarExpr:
		// Pointer receiver
		if ident, ok := t.X.(*ast.Ident); ok {
			return ident.Name
		}
	}
	return "Unknown"
}

// implementsInterface checks if a type implements all methods of an interface
func implementsInterface(typeMethods []*ast.FuncDecl, iface *ast.InterfaceType) bool {
	// Check each interface method
	for _, method := range iface.Methods.List {
		if len(method.Names) == 0 {
			continue
		}
		methodName := method.Names[0].Name

		// Check if the type has this method
		found := false
		for _, typeMethod := range typeMethods {
			if typeMethod.Name.Name == methodName {
				found = true
				break
			}
		}

		if !found {
			return false
		}
	}

	return true
}

func Transpile(file *ast.File, fileSet *token.FileSet, typeInfo *TypeInfo) (string, *ImportTracker, map[string]bool) {
	return TranspileWithMapping(file, fileSet, typeInfo, nil)
}

func TranspileWithMapping(file *ast.File, fileSet *token.FileSet, typeInfo *TypeInfo, packageMapping map[string]string) (string, *ImportTracker, map[string]bool) {
	// Create trackers
	imports := NewImportTracker()
	helpers := &HelperTracker{}
	parentCtx := GetTranspileContext()
	session := NewTranspileSession(typeInfo, packageMapping)
	packageState := NewPackageState()
	if parentCtx != nil {
		if parentCtx.Session != nil {
			session = parentCtx.Session
		}
		if parentCtx.Package != nil {
			packageState = parentCtx.Package
		}
	}
	fileState := NewFileState(imports, helpers, NewStatementPreprocessor(fileSet))
	fileExternalPackages := make(map[string]bool)
	for _, imp := range file.Imports {
		path := strings.Trim(imp.Path.Value, `"`)
		if !isStdlibPackage(path) {
			fileExternalPackages[path] = true
		}
	}

	// Initialize variable tracking table
	vt := NewVarTable()
	SetVarTable(vt)
	defer SetVarTable(nil)

	// Set up a child context that shares session/package state but owns fresh file state.
	ctx := &TranspileContext{
		Session:        session,
		Package:        packageState,
		File:           fileState,
		Imports:        imports,
		Helpers:        helpers,
		PackageMapping: packageMapping,
	}
	if parentCtx != nil {
		ctx.UsePackageExternalStubs = parentCtx.UsePackageExternalStubs
		ctx.UsePackageHelpers = parentCtx.UsePackageHelpers
	}
	SetTranspileContext(ctx)
	defer SetTranspileContext(parentCtx)
	if currentContext != nil && currentContext.Package != nil && len(currentContext.Package.MethodNameOverrides) == 0 {
		currentContext.Package.MethodNameOverrides = assignPackageMethodNames([]*ast.File{file}, typeInfo)
		packageMethodNameOverrides = currentContext.Package.MethodNameOverrides
	}

	// Transpile the body
	var body strings.Builder
	packageGlobalNames = make(map[string]bool)
	prevComparableStructTypes := comparableStructTypes
	prevLocalInterfaceEqualityTypes := localInterfaceEqualityTypes
	comparableStructTypes = collectComparableStructTypes(file)
	if currentContext != nil && currentContext.Package != nil {
		for name := range currentContext.Package.MapKeyStructTypes {
			comparableStructTypes[name] = true
		}
	}
	localInterfaceEqualityTypes = collectLocalInterfaceEqualityTypes(file)
	defer func() {
		comparableStructTypes = prevComparableStructTypes
		localInterfaceEqualityTypes = prevLocalInterfaceEqualityTypes
	}()

	// Collect methods by receiver type
	methods := make(map[string][]*ast.FuncDecl)
	typePositions := make(map[string]token.Pos)
	interfaces := make(map[string]*ast.InterfaceType)
	var functions []*ast.FuncDecl
	var types []struct {
		spec *ast.TypeSpec
		decl *ast.GenDecl
	}
	var consts []*ast.GenDecl
	var globalVars []*ast.GenDecl

	// First pass: categorize declarations
	for _, decl := range file.Decls {
		switch d := decl.(type) {
		case *ast.FuncDecl:
			if d.Recv != nil && len(d.Recv.List) > 0 {
				// This is a method
				recvType := getReceiverType(d.Recv.List[0].Type)
				if _, exists := typePositions[recvType]; !exists {
					typePositions[recvType] = d.Pos()
				}
				methods[recvType] = append(methods[recvType], d)
				// Track types with Error() string method (error interface)
				if d.Name.Name == "Error" && d.Type.Results != nil && len(d.Type.Results.List) == 1 {
					if resultType, ok := d.Type.Results.List[0].Type.(*ast.Ident); ok && resultType.Name == "string" {
						RegisterErrorImplType(recvType)
					}
				}
				if d.Name.Name == "String" && (d.Type.Params == nil || len(d.Type.Params.List) == 0) && d.Type.Results != nil && len(d.Type.Results.List) == 1 {
					if resultType, ok := d.Type.Results.List[0].Type.(*ast.Ident); ok && resultType.Name == "string" {
						RegisterStringerImplType(recvType)
					}
				}
			} else {
				// Regular function
				functions = append(functions, d)
				if d.Name.Name == "init" {
					hasInitFunction = true
				}
			}
		case *ast.GenDecl:
			switch d.Tok {
			case token.IMPORT:
				// Track imports for external package handling
				for _, spec := range d.Specs {
					if importSpec, ok := spec.(*ast.ImportSpec); ok {
						path := strings.Trim(importSpec.Path.Value, `"`)
						// Track this import (will be handled based on external package mode)
						trackGoImport(path, importSpec.Name)
					}
				}
			case token.TYPE:
				for _, spec := range d.Specs {
					if typeSpec, ok := spec.(*ast.TypeSpec); ok {
						types = append(types, struct {
							spec *ast.TypeSpec
							decl *ast.GenDecl
						}{typeSpec, d})
						if typeSpec.Assign != 0 {
							RegisterTypeAlias(typeSpec.Name.Name)
							if _, isFuncType := typeSpec.Type.(*ast.FuncType); isFuncType {
								RegisterFunctionTypeAlias(typeSpec.Name.Name)
							}
						} else if _, isFuncType := typeSpec.Type.(*ast.FuncType); isFuncType {
							RegisterTypeAlias(typeSpec.Name.Name)
							RegisterFunctionTypeAlias(typeSpec.Name.Name)
						} else {
							_, isStruct := typeSpec.Type.(*ast.StructType)
							_, isInterface := typeSpec.Type.(*ast.InterfaceType)
							if !isStruct && !isInterface {
								RegisterTypeDefinition(typeSpec.Name.Name, typeDefinitionUnderlyingName(typeSpec.Type))
							}
						}
						// Track interfaces
						if ifaceType, ok := typeSpec.Type.(*ast.InterfaceType); ok {
							interfaces[typeSpec.Name.Name] = ifaceType
							RegisterInterfaceType(typeSpec.Name.Name)
						}
					}
				}
			case token.CONST:
				consts = append(consts, d)
			case token.VAR:
				globalVars = append(globalVars, d)
			}
		}
	}
	if len(globalVars) > 0 {
		hasInitFunction = true
	}
	localFunctionInterfaces := collectFunctionLocalInterfaces(file)
	for name, ifaceType := range localFunctionInterfaces {
		interfaces[name] = ifaceType
		localInterfaces[name] = true
		RegisterInterfaceType(name)
	}
	hasGlobals := hasNamedPackageGlobals(globalVars)
	for _, fn := range functions {
		registerFunctionSignatureDecl(fn)
	}
	functionNames := assignFunctionNames(functions)
	SetFunctionNameOverrides(functionNames)
	defer SetFunctionNameOverrides(nil)

	// Track if we need spacing between declarations
	first := true

	// Output constants first
	for _, c := range consts {
		if !first {
			body.WriteString("\n\n")
		}
		first = false
		TranspileConstDecl(&body, c)
	}

	var localInterfaceNames []string
	for name := range localFunctionInterfaces {
		localInterfaceNames = append(localInterfaceNames, name)
	}
	slices.Sort(localInterfaceNames)
	for _, name := range localInterfaceNames {
		if !first {
			body.WriteString("\n\n")
		}
		first = false
		TranspileTypeDecl(&body, &ast.TypeSpec{
			Name: ast.NewIdent(name),
			Type: localFunctionInterfaces[name],
		}, nil)
	}

	// Output type declarations
	for _, t := range types {
		if structType, ok := t.spec.Type.(*ast.StructType); ok {
			registerStructDef(t.spec.Name.Name, structType)
		}
	}
	markComparableInterfaceImplementorStructs(types, methods, interfaces)
	importedInterfaceImpls := importedInterfaceImplsForFile(file)
	externalLocalInterfaceImpls := collectExternalLocalInterfaceImpls(file, interfaces)
	for _, t := range types {
		if !first {
			body.WriteString("\n\n")
		}
		first = false
		// Output doc comments if present
		outputComment(&body, t.decl.Doc, "", true)
		TranspileTypeDecl(&body, t.spec, t.decl)
	}

	emittedAnonymousStructs := make(map[string]bool)
	writeAnonymousStructDefinitions(&body, &first, emittedAnonymousStructs)

	if hasGlobals {
		if !first {
			body.WriteString("\n\n")
		}
		first = false
		TranspilePackageGlobals(&body, globalVars)
	}

	// Output impl blocks for types with methods in source file order
	// Also include types that have embedded types (for method promotion)
	var typeNames []string
	typesWithImpls := make(map[string]bool)
	declaredTypeNames := make(map[string]bool)
	for _, t := range types {
		declaredTypeNames[t.spec.Name.Name] = true
	}

	// Add types that have methods
	for typeName := range methods {
		typeNames = append(typeNames, typeName)
		typesWithImpls[typeName] = true
	}

	// Add types that have embedded types (even if they don't have their own methods)
	for typeName, structDef := range structDefs {
		if declaredTypeNames[typeName] && len(structDef.EmbeddedTypes) > 0 && !typesWithImpls[typeName] {
			typeNames = append(typeNames, typeName)
			typesWithImpls[typeName] = true
		}
	}

	sort.Slice(typeNames, func(i, j int) bool {
		pos1, exists1 := typePositions[typeNames[i]]
		pos2, exists2 := typePositions[typeNames[j]]
		// If one doesn't have a position, put it at the end
		if !exists1 && !exists2 {
			return typeNames[i] < typeNames[j] // Alphabetical order
		}
		if !exists1 {
			return false
		}
		if !exists2 {
			return true
		}
		return pos1 < pos2
	})

	for _, typeName := range typeNames {
		typeMethods := methods[typeName] // May be nil if type has no methods
		previousTypeMethods := currentTypeMethods
		currentTypeMethods = typeMethods
		rustTypeName := RustTypeNameForUse(typeName)
		if !first {
			body.WriteString("\n\n")
		}
		first = false
		if IsFunctionTypeAlias(typeName) {
			writeFunctionTypeAliasMethodImpl(&body, rustTypeName, typeMethods, fileSet, file.Comments)
			currentTypeMethods = previousTypeMethods
			continue
		}
		body.WriteString("impl ")
		body.WriteString(rustTypeName)
		body.WriteString(" {\n")

		// First, output the type's own methods
		methodCount := 0
		for _, method := range typeMethods {
			if methodCount > 0 {
				body.WriteString("\n")
			}
			TranspileMethodImpl(&body, method, fileSet, file.Comments)
			methodCount++
		}

		// Generate promoted methods from embedded types
		if structDef, exists := structDefs[typeName]; exists && declaredTypeNames[typeName] {
			existingMethodNames := make(map[string]bool)
			for _, ownMethod := range typeMethods {
				existingMethodNames[ownMethod.Name.Name] = true
			}

			// Collect all methods that should be promoted (including from nested embeds)
			promotedMethods := make(map[string]struct {
				embeddedType string
				method       *ast.FuncDecl
			})
			collectPromotedMethods(structDef, methods, promotedMethods)

			// Generate forwarding methods for all promoted methods
			// Sort method names for deterministic output
			var promotedMethodNames []string
			for methodName := range promotedMethods {
				promotedMethodNames = append(promotedMethodNames, methodName)
			}
			slices.Sort(promotedMethodNames)

			for _, methodName := range promotedMethodNames {
				methodInfo := promotedMethods[methodName]
				// Check if this method is already defined by the outer type (shadowing)
				if !existingMethodNames[methodName] {
					// Generate a forwarding method
					if methodCount > 0 {
						body.WriteString("\n")
					}
					generatePromotedMethod(&body, methodInfo.method, methodInfo.embeddedType)
					existingMethodNames[methodName] = true
					methodCount++
				}
			}

			for _, promotedMethod := range collectExternalPromotedMethods(structDef, existingMethodNames) {
				if methodCount > 0 {
					body.WriteString("\n")
				}
				generateExternalPromotedMethod(&body, promotedMethod)
				methodCount++
			}
		}

		body.WriteString("}")

		// Check if this type has an Error() string method
		hasErrorMethod := false
		for _, method := range methods[typeName] {
			if method.Name.Name == "Error" && method.Type.Results != nil && len(method.Type.Results.List) == 1 {
				if resultType, ok := method.Type.Results.List[0].Type.(*ast.Ident); ok && resultType.Name == "string" {
					hasErrorMethod = true
					break
				}
			}
		}

		// If it has Error() method, implement Error trait
		if hasErrorMethod {
			// Track necessary imports
			TrackImport("Error")

			body.WriteString("\n\n")
			body.WriteString("impl StdError for ")
			body.WriteString(rustTypeName)
			body.WriteString(" {}\n")
			// Note: Display impl is already generated by generateStructDisplay
		}

		// Generate trait implementations for this type
		// Sort interface names for deterministic output
		var ifaceNames []string
		for ifaceName := range interfaces {
			ifaceNames = append(ifaceNames, ifaceName)
		}
		slices.Sort(ifaceNames)

		for _, ifaceName := range ifaceNames {
			ifaceType := interfaces[ifaceName]
			if implementsInterface(methods[typeName], ifaceType) {
				body.WriteString("\n\n")
				body.WriteString("impl ")
				body.WriteString(ifaceName)
				body.WriteString(" for ")
				body.WriteString(rustTypeName)
				body.WriteString(" {\n")

				// Generate trait method implementations
				for _, method := range ifaceType.Methods.List {
					if len(method.Names) > 0 {
						methodName := method.Names[0].Name
						// Find the corresponding method implementation
						for _, impl := range methods[typeName] {
							if impl.Name.Name == methodName {
								TranspileTraitMethodImpl(&body, impl, fileSet, file.Comments)
								break
							}
						}
					}
				}

				writeLocalInterfaceSupportImpl(&body, ifaceName, typeName)
				body.WriteString("}")
			}
		}

		var importedIfaceNames []string
		for ifaceName := range importedInterfaceImpls[typeName] {
			importedIfaceNames = append(importedIfaceNames, ifaceName)
		}
		slices.Sort(importedIfaceNames)
		for _, ifaceName := range importedIfaceNames {
			ifaceType := importedInterfaceImpls[typeName][ifaceName]
			if !typeMethodsImplementTypesInterface(methods[typeName], ifaceType) {
				continue
			}
			body.WriteString("\n\n")
			body.WriteString("impl ")
			body.WriteString(ifaceName)
			body.WriteString(" for ")
			body.WriteString(rustTypeName)
			body.WriteString(" {\n")
			for i := 0; i < ifaceType.NumMethods(); i++ {
				if method := methodDeclByName(methods[typeName], ifaceType.Method(i).Name()); method != nil {
					TranspileTraitMethodImpl(&body, method, fileSet, file.Comments)
				}
			}
			writeLocalInterfaceSupportImpl(&body, ifaceName, typeName)
			body.WriteString("}")
		}
		currentTypeMethods = previousTypeMethods
	}

	writeExternalLocalInterfaceImpls(&body, &first, externalLocalInterfaceImpls)

	// Output regular functions
	for _, fn := range functions {
		if !first {
			body.WriteString("\n\n")
		}
		first = false
		// Output doc comments if present
		outputComment(&body, fn.Doc, "", true)
		TranspileFunction(&body, fn, fileSet, file.Comments)
	}

	writeAnonymousStructDefinitions(&body, &first, emittedAnonymousStructs)

	if hasInitFunction {
		if !first {
			body.WriteString("\n\n")
		}
		TranspilePackageInitAll(&body, hasGlobals, functionNames)
	}

	// Now build the final output with only needed imports
	var output strings.Builder
	helpersForFile := helpers
	if ctx.UsePackageExternalStubs {
		helpersForFile = helpers.withoutSharedStdlibHelpers()
	}
	helpersStr := helpersForFile.GenerateHelpers()
	importsStr := imports.GenerateImports()
	output.WriteString(importsStr)
	if importsStr != "" {
		output.WriteString("\n")
	}
	output.WriteString(helpersStr)
	if helpersStr != "" {
		output.WriteString("\n")
	}
	stubsStr := GenerateExternalTypeStubs()
	output.WriteString(stubsStr)
	if stubsStr != "" {
		output.WriteString("\n\n")
	}
	output.WriteString(body.String())

	return output.String(), imports, fileExternalPackages
}

func writeAnonymousStructDefinitions(body *strings.Builder, first *bool, emitted map[string]bool) {
	var anonTypeNames []string
	for typeName := range anonymousStructs {
		if !emitted[typeName] {
			anonTypeNames = append(anonTypeNames, typeName)
		}
	}
	slices.Sort(anonTypeNames)

	for _, typeName := range anonTypeNames {
		structType := anonymousStructs[typeName]
		if !*first {
			body.WriteString("\n\n")
		}
		*first = false
		writeStructDerive(body, "", structType)
		body.WriteString("struct ")
		body.WriteString(typeName)
		body.WriteString(" {\n")

		for _, field := range structType.Fields.List {
			if len(field.Names) > 0 {
				// Handle multiple names on one line (e.g., X, Y int)
				for _, name := range field.Names {
					body.WriteString("    ")
					body.WriteString(ToSnakeCase(name.Name))
					body.WriteString(": ")
					body.WriteString(GoTypeToRust(field.Type))
					body.WriteString(",\n")
				}
			} else {
				// Anonymous/embedded field - should not happen in anonymous structs
				body.WriteString("    // WARNING: embedded field in anonymous struct\n")
			}
		}

		body.WriteString("}\n")
		generateStructDefault(body, typeName, structType)
		body.WriteString("\n")
		generateStructDisplay(body, typeName, structType)
		emitted[typeName] = true
	}
}
