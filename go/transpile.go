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
var localRangeElemRustTypes = make(map[string]string)
var localCollectionKinds = make(map[string]string)
var localMapKeyRustTypes = make(map[string]string)
var localMapValueRustTypes = make(map[string]string)
var localConstants = make(map[string]string)
var packageConstants = make(map[string]string)

// localInterfaces tracks locally-defined interface type names (inside functions)
var localInterfaces = make(map[string]bool)

// currentReceiver tracks the current method receiver name for self translation
var currentReceiver string

// currentReceiverType tracks the type of the current method receiver
var currentReceiverType string

// currentTypeMethods tracks the current impl block's method set for receiver self-call analysis
var currentTypeMethods = []*ast.FuncDecl{}

// currentFunctionHasDefer tracks if the current function has defer statements
var currentFunctionHasDefer bool

// activeMutexGuards tracks sync.Mutex Lock statement guards by receiver syntax
// so a direct Unlock statement can drop the matching Rust guard at that point.
var activeMutexGuards = make(map[string]string)

// currentCaptureRenames tracks variable renames for captured variables in closures
var currentCaptureRenames map[string]string

func snapshotCaptureRenames() map[string]string {
	if currentCaptureRenames == nil {
		return nil
	}
	snapshot := make(map[string]string, len(currentCaptureRenames))
	for name, renamed := range currentCaptureRenames {
		snapshot[name] = renamed
	}
	return snapshot
}

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

// forPostStack tracks the nearest loop's post statement for unlabeled
// continues. forPostHasPostStack mirrors loop nesting so nil post statements
// do not need an interface-valued sentinel.
var forPostStack []ast.Stmt
var forPostHasPostStack []bool

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
	FieldTypes    map[string]ast.Expr
	EmbeddedTypes []string        // list of embedded type names
	ASTType       *ast.StructType // original AST type for zero-value generation
	EmbedsError   bool            // true when the struct embeds the predeclared error interface
}

var structDefs = make(map[string]*StructDef)

type transpileFileAnalysis struct {
	comparableStructTypes       map[string]bool
	mapKeyStructTypes           map[string]bool
	localInterfaceEqualityTypes map[string]bool
	functionLocalInterfaces     map[string]*ast.InterfaceType
	importedInterfaceImpls      map[string]map[string]*types.Interface
	typeAssertExprs             []*ast.TypeAssertExpr
	typeSwitchStmts             []*ast.TypeSwitchStmt
}

func newTranspileFileAnalysis() *transpileFileAnalysis {
	return &transpileFileAnalysis{
		comparableStructTypes:       make(map[string]bool),
		mapKeyStructTypes:           make(map[string]bool),
		localInterfaceEqualityTypes: make(map[string]bool),
		functionLocalInterfaces:     make(map[string]*ast.InterfaceType),
		importedInterfaceImpls:      make(map[string]map[string]*types.Interface),
	}
}

func analyzeTranspileFile(file *ast.File, mapKeyTypeInfo *TypeInfo) *transpileFileAnalysis {
	return analyzeTranspileFiles([]*ast.File{file}, mapKeyTypeInfo)
}

func analyzeTranspileFiles(files []*ast.File, mapKeyTypeInfo *TypeInfo) *transpileFileAnalysis {
	analysis := newTranspileFileAnalysis()
	typeInfo := GetTypeInfo()
	for _, file := range files {
		analysis.inspect(file, typeInfo, mapKeyTypeInfo)
	}
	return analysis
}

func (analysis *transpileFileAnalysis) inspect(file *ast.File, typeInfo *TypeInfo, mapKeyTypeInfo *TypeInfo) {
	if file == nil {
		return
	}
	var stack []ast.Node
	funcDeclDepth := 0
	ast.Inspect(file, func(node ast.Node) bool {
		if node == nil {
			if len(stack) > 0 {
				popped := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				if _, ok := popped.(*ast.FuncDecl); ok {
					funcDeclDepth--
				}
			}
			return true
		}
		stack = append(stack, node)
		if _, ok := node.(*ast.FuncDecl); ok {
			funcDeclDepth++
		}

		switch n := node.(type) {
		case *ast.BinaryExpr:
			analysis.inspectBinaryExpr(n, typeInfo)
		case *ast.MapType:
			analysis.inspectMapType(n, typeInfo, mapKeyTypeInfo)
		case *ast.CallExpr:
			analysis.inspectCallExpr(n, typeInfo)
		case *ast.GenDecl:
			if funcDeclDepth > 0 {
				analysis.inspectFunctionGenDecl(n)
			}
		case *ast.TypeAssertExpr:
			analysis.typeAssertExprs = append(analysis.typeAssertExprs, n)
		case *ast.TypeSwitchStmt:
			analysis.typeSwitchStmts = append(analysis.typeSwitchStmts, n)
		}
		return true
	})
}

func (analysis *transpileFileAnalysis) inspectBinaryExpr(expr *ast.BinaryExpr, typeInfo *TypeInfo) {
	if typeInfo == nil || expr.Op != token.EQL && expr.Op != token.NEQ {
		return
	}
	markComparableStructType(analysis.comparableStructTypes, typeInfo.GetType(expr.X))
	markComparableStructType(analysis.comparableStructTypes, typeInfo.GetType(expr.Y))
	if name, ok := localNamedInterfaceTypeNameFromTypes(typeInfo.GetType(expr.X)); ok {
		analysis.localInterfaceEqualityTypes[name] = true
	}
	if name, ok := localNamedInterfaceTypeNameFromTypes(typeInfo.GetType(expr.Y)); ok {
		analysis.localInterfaceEqualityTypes[name] = true
	}
}

func (analysis *transpileFileAnalysis) inspectMapType(mapType *ast.MapType, typeInfo *TypeInfo, mapKeyTypeInfo *TypeInfo) {
	markMapKeyStructType(analysis.comparableStructTypes, mapKeyTypeFromMapType(typeInfo, mapType), typeInfo)
	markMapKeyStructType(analysis.mapKeyStructTypes, mapKeyTypeFromMapType(mapKeyTypeInfo, mapType), mapKeyTypeInfo)
}

func (analysis *transpileFileAnalysis) inspectCallExpr(call *ast.CallExpr, typeInfo *TypeInfo) {
	if typeInfo == nil || typeInfo.info == nil {
		return
	}
	if isSlicesContainsCall(call) && len(call.Args) >= 2 {
		if name, ok := localInterfaceSliceElemName(typeInfo.GetType(call.Args[0])); ok {
			analysis.localInterfaceEqualityTypes[name] = true
		}
	}
	for i, arg := range call.Args {
		analysis.recordImportedInterfaceImpl(callParamTypeFromTypeInfo(call, i), arg, typeInfo)
		if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
			analysis.recordImportedInterfaceImpl(selectedMethodParamType(sel, i), arg, typeInfo)
		}
	}
}

func (analysis *transpileFileAnalysis) inspectFunctionGenDecl(genDecl *ast.GenDecl) {
	if genDecl.Tok != token.TYPE {
		return
	}
	for _, spec := range genDecl.Specs {
		typeSpec, ok := spec.(*ast.TypeSpec)
		if !ok {
			continue
		}
		if iface, ok := typeSpec.Type.(*ast.InterfaceType); ok {
			analysis.functionLocalInterfaces[typeSpec.Name.Name] = iface
		}
	}
}

func (analysis *transpileFileAnalysis) recordImportedInterfaceImpl(expected types.Type, arg ast.Expr, typeInfo *TypeInfo) {
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
	if analysis.importedInterfaceImpls[typeName] == nil {
		analysis.importedInterfaceImpls[typeName] = make(map[string]*types.Interface)
	}
	analysis.importedInterfaceImpls[typeName][ifaceName] = ifaceType
}

func (analysis *transpileFileAnalysis) externalLocalInterfaceImpls(interfaces map[string]*ast.InterfaceType) map[string]map[string]externalLocalInterfaceImpl {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil {
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

	for _, node := range analysis.typeAssertExprs {
		ifaceName, ifaceType, _, candidates, ok := localInterfaceAssertionTarget(node)
		if !ok {
			continue
		}
		for _, candidate := range candidates {
			if candidate.external {
				record(ifaceName, ifaceType, candidate.typ)
			}
		}
	}
	for _, node := range analysis.typeSwitchStmts {
		analysis.recordExternalLocalInterfaceTypeSwitchImpls(node, record)
	}
	return impls
}

func (analysis *transpileFileAnalysis) recordExternalLocalInterfaceTypeSwitchImpls(node *ast.TypeSwitchStmt, record func(string, *types.Interface, types.Type)) {
	if node.Assign == nil {
		return
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return
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
		return
	}
	subjectType := typeInfo.GetType(expr)
	ifaceName, ok := localNamedInterfaceTypeNameFromTypes(subjectType)
	if !ok {
		return
	}
	named, ok := types.Unalias(subjectType).(*types.Named)
	if !ok {
		return
	}
	ifaceType, ok := named.Underlying().(*types.Interface)
	if !ok {
		return
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
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return make(map[string]bool)
	}
	return analyzeTranspileFile(file, typeInfo).comparableStructTypes
}

func collectMapKeyStructTypesFromFiles(files []*ast.File, typeInfo *TypeInfo) map[string]bool {
	return analyzeTranspileFiles(files, typeInfo).mapKeyStructTypes
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
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return make(map[string]bool)
	}
	return analyzeTranspileFile(file, typeInfo).localInterfaceEqualityTypes
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

func typeSpecCompletenessScore(spec *ast.TypeSpec) int {
	if spec == nil {
		return 0
	}
	structType, ok := spec.Type.(*ast.StructType)
	if !ok || structType.Fields == nil {
		return 1
	}
	score := 1
	for _, field := range structType.Fields.List {
		if field == nil {
			continue
		}
		if field.Type != nil {
			score++
		}
	}
	return score
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
	return analyzeTranspileFiles(files, typeInfo).importedInterfaceImpls
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
	if file == nil {
		return make(map[string]*ast.InterfaceType)
	}
	return analyzeTranspileFile(file, GetTypeInfo()).functionLocalInterfaces
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
	return analyzeTranspileFile(file, typeInfo).externalLocalInterfaceImpls(interfaces)
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

func collectPackageMethods(files []*ast.File) map[string][]*ast.FuncDecl {
	methods := make(map[string][]*ast.FuncDecl)
	for _, file := range files {
		if file == nil {
			continue
		}
		for _, decl := range file.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Recv == nil || len(fn.Recv.List) == 0 {
				continue
			}
			recvType := getReceiverType(fn.Recv.List[0].Type)
			methods[recvType] = append(methods[recvType], fn)
		}
	}
	return methods
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

func typeHasExplicitErrorStringMethod(typeMethods []*ast.FuncDecl) bool {
	for _, method := range typeMethods {
		if method.Name.Name != "Error" || method.Type.Results == nil || len(method.Type.Results.List) != 1 {
			continue
		}
		if resultType, ok := method.Type.Results.List[0].Type.(*ast.Ident); ok && resultType.Name == "string" {
			return true
		}
	}
	return false
}

func writeEmbeddedGoErrorMethod(out *strings.Builder) {
	out.WriteString("    pub fn error(&self) -> ")
	out.WriteString(GoTypeToRust(ast.NewIdent("string")))
	out.WriteString(" {\n")
	out.WriteString("        ")
	WriteWrapperPrefix(out)
	out.WriteString("format!(\"{}\", (*self.error")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()))")
	WriteWrapperSuffix(out)
	out.WriteString("\n")
	out.WriteString("    }\n")
}

// generatePromotedMethod generates a forwarding method that delegates to an embedded type's method
func generatePromotedMethod(out *strings.Builder, method *ast.FuncDecl, embeddedTypeName string) {
	mutableReceiver := methodRequiresMutableReceiver(method)

	out.WriteString("    pub fn ")
	out.WriteString(rustMethodName(method))
	out.WriteString("(")

	// Receiver
	if method.Recv != nil && len(method.Recv.List) > 0 {
		if mutableReceiver {
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
	if mutableReceiver {
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
			fmt.Fprintf(out, "T%d: 'static", i)
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
	if currentContext != nil && currentContext.Package != nil && len(currentContext.Package.MethodsByType) == 0 {
		currentContext.Package.MethodsByType = collectPackageMethods([]*ast.File{file})
	}

	// Transpile the body
	var body strings.Builder
	fileAnalysis := analyzeTranspileFile(file, typeInfo)
	packageGlobalNames = make(map[string]bool)
	prevComparableStructTypes := comparableStructTypes
	prevLocalInterfaceEqualityTypes := localInterfaceEqualityTypes
	comparableStructTypes = fileAnalysis.comparableStructTypes
	if currentContext != nil && currentContext.Package != nil {
		for name := range currentContext.Package.MapKeyStructTypes {
			comparableStructTypes[name] = true
		}
	}
	localInterfaceEqualityTypes = fileAnalysis.localInterfaceEqualityTypes
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
	typeIndexByName := make(map[string]int)
	var consts []*ast.GenDecl
	var globalVars []*ast.GenDecl

	for _, importSpec := range file.Imports {
		if importSpec == nil || importSpec.Path == nil {
			continue
		}
		path := strings.Trim(importSpec.Path.Value, `"`)
		trackGoImport(path, importSpec.Name)
	}

	// First pass: categorize declarations
	for _, decl := range file.Decls {
		switch d := decl.(type) {
		case *ast.FuncDecl:
			if d.Recv != nil && len(d.Recv.List) > 0 {
				// This is a method
				methodName := d.Name.Name
				recvType := getReceiverType(d.Recv.List[0].Type)
				if _, exists := typePositions[recvType]; !exists {
					typePositions[recvType] = d.Pos()
				}
				methods[recvType] = append(methods[recvType], d)
				// Track types with Error() string method (error interface)
				if methodName == "Error" {
					if d.Type.Results != nil && len(d.Type.Results.List) == 1 {
						if resultType, ok := d.Type.Results.List[0].Type.(*ast.Ident); ok && resultType.Name == "string" {
							RegisterErrorImplType(recvType)
						}
					}
				}
				if methodName == "String" {
					if (d.Type.Params == nil || len(d.Type.Params.List) == 0) && d.Type.Results != nil && len(d.Type.Results.List) == 1 {
						if resultType, ok := d.Type.Results.List[0].Type.(*ast.Ident); ok && resultType.Name == "string" {
							RegisterStringerImplType(recvType)
						}
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
						typeEntry := struct {
							spec *ast.TypeSpec
							decl *ast.GenDecl
						}{typeSpec, d}
						if existingIndex, exists := typeIndexByName[typeSpec.Name.Name]; exists {
							if typeSpecCompletenessScore(typeSpec) > typeSpecCompletenessScore(types[existingIndex].spec) {
								types[existingIndex] = typeEntry
							}
							continue
						}
						typeIndexByName[typeSpec.Name.Name] = len(types)
						types = append(types, typeEntry)
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
	registerPackageGlobalNames(globalVars)
	localFunctionInterfaces := fileAnalysis.functionLocalInterfaces
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
	importedInterfaceImpls := fileAnalysis.importedInterfaceImpls
	if ctx := GetTranspileContext(); ctx != nil && ctx.Package != nil && len(ctx.Package.ImportedInterfaceImpls) > 0 {
		importedInterfaceImpls = ctx.Package.ImportedInterfaceImpls
	}
	externalLocalInterfaceImpls := fileAnalysis.externalLocalInterfaceImpls(interfaces)
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
		if typeMethods == nil {
			typeMethods = []*ast.FuncDecl{}
		}
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

		if structDef, exists := structDefs[typeName]; exists && declaredTypeNames[typeName] && structDef.EmbedsError && !typeHasExplicitErrorStringMethod(typeMethods) {
			if methodCount > 0 {
				body.WriteString("\n")
			}
			writeEmbeddedGoErrorMethod(&body)
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

		// Check if this type has or promotes an Error() string method.
		hasErrorMethod := typeHasExplicitErrorStringMethod(methods[typeName])
		if !hasErrorMethod {
			if structDef, exists := structDefs[typeName]; exists && structDef.EmbedsError {
				hasErrorMethod = true
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
	stubsStr := GenerateExternalTypeStubs()
	if strings.Contains(stubsStr, "StdError") {
		imports.Add("Error")
	}
	importsStr := imports.GenerateImports()
	output.WriteString(importsStr)
	if importsStr != "" {
		output.WriteString("\n")
	}
	output.WriteString(helpersStr)
	if helpersStr != "" {
		output.WriteString("\n")
	}
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
		generateStructValueClone(body, typeName, structType)
		body.WriteString("\n")
		generateStructDefault(body, typeName, structType)
		body.WriteString("\n")
		generateStructDisplay(body, typeName, structType)
		generateStructJsonDecode(body, typeName, structType)
		emitted[typeName] = true
	}
}
