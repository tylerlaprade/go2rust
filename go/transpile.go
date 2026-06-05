package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"go/types"
	"slices"
	"sort"
	"strings"
)

// These maps track the names and types of special variables that shouldn't be unwrapped
var rangeLoopVars = make(map[string]string)
var rangeLoopVarGoTypes = make(map[string]types.Type)
var localRangeElemRustTypes = make(map[string]string)
var localCollectionKinds = make(map[string]string)
var localMapKeyRustTypes = make(map[string]string)
var localMapValueRustTypes = make(map[string]string)
var localMapValueKeepHandle = make(map[string]bool)
var localConstants = make(map[string]string)
var packageConstants = make(map[string]string)
var packageConstantTypeNames = make(map[string]string)

// localInterfaces tracks locally-defined interface type names (inside functions)
var localInterfaces = make(map[string]bool)

// currentReceiver tracks the current method receiver name for self translation
var currentReceiver string

// currentReceiverObject tracks the go/types object for the current method
// receiver. Locals can legally shadow the receiver name, so name matching alone
// is not sufficient when type information is available.
var currentReceiverObject types.Object

// currentReceiverType tracks the type of the current method receiver
var currentReceiverType string

// currentReceiverRustAlias tracks a mutable Rust local copy used when a Go
// receiver is reassigned inside its method body.
var currentReceiverRustAlias string

// currentReceiverRustAliasIsPointerHandle records the narrow case where the
// reassigned receiver must remain nil-able because the Go method compares it
// against nil.
var currentReceiverRustAliasIsPointerHandle bool

// currentReceiverRustAliasIsGoPtr records reassigned receivers that must preserve
// Go pointer identity because they receive GoPtr values.
var currentReceiverRustAliasIsGoPtr bool
var currentReceiverRustAliasGoPtrInfo goPtrResultInfo

// currentTypeMethods tracks the current impl block's method set for receiver self-call analysis
var currentTypeMethods = []*ast.FuncDecl{}

// currentFunctionHasDefer tracks if the current function has defer statements
var currentFunctionHasDefer bool

// currentFunctionBodyLbrace records the position of the current function body's
// opening brace. Names declared at positions greater than this are body locals;
// names declared earlier (params, receivers, outer scopes) are not. We need
// this distinction because tail-expression temporaries outlive `let`-declared
// locals but drop before parameters.
var currentFunctionBodyLbrace token.Pos

// activeMutexGuards tracks sync.Mutex Lock statement guards by receiver syntax
// so a direct Unlock statement can drop the matching Rust guard at that point.
var activeMutexGuards = make(map[string]string)

var currentLoopDepth int

// currentCaptureRenames tracks variable renames for captured variables in closures
var currentCaptureRenames map[string]string

// forceInnerFuncLitCaptureClones tells function literals to clone existing
// statement-level capture clones before moving them into the closure.
var forceInnerFuncLitCaptureClones bool

// forceInnerFuncLitCaptureCloneNames narrows forced inner clones to the named
// captures when non-nil.
var forceInnerFuncLitCaptureCloneNames map[string]bool

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

// typeDefinitionUnderlyingTypes tracks the immediate RHS type for type definitions.
var typeDefinitionUnderlyingTypes = make(map[string]types.Type)

// typeAliases tracks which types are type aliases
var typeAliases = make(map[string]bool)

// bareStructAliases tracks local struct type aliases emitted as bare Rust
// aliases to generated anonymous struct types.
var bareStructAliases = make(map[string]bool)

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
	Fields            map[string]string // field name -> field type
	FieldTypes        map[string]ast.Expr
	FieldTags         map[string]string
	FieldOrder        []string
	EmbeddedTypes     []string        // list of embedded type names
	ASTType           *ast.StructType // original AST type for zero-value generation
	EmbedsError       bool            // true when the struct embeds the predeclared error interface
	PhantomTypeParams []string
}

var structDefs = make(map[string]*StructDef)

type transpileFileAnalysis struct {
	comparableStructTypes         map[string]bool
	mapKeyStructTypes             map[string]bool
	pointerComparablePointees     map[string]bool
	localInterfaceEqualityTypes   map[string]bool
	localInterfaceMapKeyTypes     map[string]bool
	functionLocalInterfaces       map[string]*ast.InterfaceType
	importedInterfaceImpls        map[string]map[string]*types.Interface
	importedPointerInterfaceImpls map[string]map[string]*types.Interface
	externalLocalInterfaceArgs    []externalLocalInterfaceArg
	typeAssertExprs               []*ast.TypeAssertExpr
	typeSwitchStmts               []*ast.TypeSwitchStmt
}

func newTranspileFileAnalysis() *transpileFileAnalysis {
	return &transpileFileAnalysis{
		comparableStructTypes:         make(map[string]bool),
		mapKeyStructTypes:             make(map[string]bool),
		pointerComparablePointees:     make(map[string]bool),
		localInterfaceEqualityTypes:   make(map[string]bool),
		localInterfaceMapKeyTypes:     make(map[string]bool),
		functionLocalInterfaces:       make(map[string]*ast.InterfaceType),
		importedInterfaceImpls:        make(map[string]map[string]*types.Interface),
		importedPointerInterfaceImpls: make(map[string]map[string]*types.Interface),
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
	analysis.recordPointerComparablePointees(typeInfo)
	analysis.recordImportedOrderedConstraintImpls(typeInfo)
	return analysis
}

func (analysis *transpileFileAnalysis) recordPointerComparablePointees(typeInfo *TypeInfo) {
	if analysis == nil || typeInfo == nil || typeInfo.info == nil {
		return
	}
	for _, instance := range typeInfo.info.Instances {
		if instance.TypeArgs == nil {
			continue
		}
		for i := 0; i < instance.TypeArgs.Len(); i++ {
			ptr, ok := types.Unalias(instance.TypeArgs.At(i)).(*types.Pointer)
			if !ok {
				continue
			}
			markPointerComparablePointee(analysis.pointerComparablePointees, ptr.Elem())
		}
	}
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
		case *ast.CompositeLit:
			analysis.inspectCompositeLit(n, typeInfo)
		case *ast.ReturnStmt:
			analysis.inspectReturnStmt(n, enclosingFuncType(stack), typeInfo)
		case *ast.GenDecl:
			if funcDeclDepth > 0 {
				analysis.inspectFunctionGenDecl(n)
			} else {
				analysis.inspectPackageGenDecl(n, typeInfo)
			}
		case *ast.TypeAssertExpr:
			analysis.typeAssertExprs = append(analysis.typeAssertExprs, n)
		case *ast.TypeSwitchStmt:
			analysis.typeSwitchStmts = append(analysis.typeSwitchStmts, n)
		}
		return true
	})
}

func (analysis *transpileFileAnalysis) inspectCompositeLit(lit *ast.CompositeLit, typeInfo *TypeInfo) {
	if lit == nil || typeInfo == nil {
		return
	}
	litType := typeInfo.GetType(lit)
	if litType == nil {
		return
	}
	structType, ok := types.Unalias(litType).Underlying().(*types.Struct)
	if !ok {
		return
	}

	positionalIndex := 0
	for _, elt := range lit.Elts {
		if kv, ok := elt.(*ast.KeyValueExpr); ok {
			fieldType := structFieldTypeForCompositeKey(kv.Key, typeInfo)
			analysis.recordImportedInterfaceImpl(fieldType, kv.Value, typeInfo)
			continue
		}
		if positionalIndex < structType.NumFields() {
			analysis.recordImportedInterfaceImpl(structType.Field(positionalIndex).Type(), elt, typeInfo)
		}
		positionalIndex++
	}
}

func structFieldTypeForCompositeKey(key ast.Expr, typeInfo *TypeInfo) types.Type {
	ident, ok := key.(*ast.Ident)
	if !ok || typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	field, ok := typeInfo.info.Uses[ident].(*types.Var)
	if !ok || field == nil {
		return nil
	}
	return field.Type()
}

func enclosingFuncType(stack []ast.Node) *ast.FuncType {
	for i := len(stack) - 1; i >= 0; i-- {
		switch n := stack[i].(type) {
		case *ast.FuncDecl:
			return n.Type
		case *ast.FuncLit:
			return n.Type
		}
	}
	return nil
}

func (analysis *transpileFileAnalysis) inspectPackageGenDecl(genDecl *ast.GenDecl, typeInfo *TypeInfo) {
	if genDecl.Tok != token.VAR || typeInfo == nil || typeInfo.info == nil {
		return
	}
	for _, spec := range genDecl.Specs {
		valueSpec, ok := spec.(*ast.ValueSpec)
		if !ok {
			continue
		}
		for i, name := range valueSpec.Names {
			if i >= len(valueSpec.Values) {
				continue
			}
			obj, ok := typeInfo.info.Defs[name].(*types.Var)
			if !ok || obj.Type() == nil {
				continue
			}
			value := valueSpec.Values[i]
			analysis.recordExternalLocalInterfaceArg(obj.Type(), typeInfo.GetType(value))
		}
	}
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
	markLocalInterfaceMapKeyType(analysis.localInterfaceMapKeyTypes, mapKeyTypeFromMapType(typeInfo, mapType))
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
		paramType := callParamTypeFromTypeInfo(call, i)
		argType := typeInfo.GetType(arg)
		analysis.recordImportedInterfaceImpl(paramType, arg, typeInfo)
		analysis.recordExternalLocalInterfaceArg(paramType, argType)
		if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
			methodParamType := selectedMethodParamType(sel, i)
			analysis.recordImportedInterfaceImpl(methodParamType, arg, typeInfo)
			analysis.recordExternalLocalInterfaceArg(methodParamType, argType)
		}
	}
}

func (analysis *transpileFileAnalysis) inspectReturnStmt(ret *ast.ReturnStmt, fnType *ast.FuncType, typeInfo *TypeInfo) {
	if ret == nil || fnType == nil || typeInfo == nil || typeInfo.info == nil {
		return
	}
	if len(ret.Results) == 1 && fnHasMultipleResultSlots(fnType) {
		call, ok := ret.Results[0].(*ast.CallExpr)
		if ok && callReturnsMultipleResults(call) {
			sig, ok := callSignatureFromTypeInfo(call)
			if !ok || sig.Results() == nil {
				return
			}
			results := sig.Results()
			for i := 0; i < results.Len(); i++ {
				expected := expectedTypeFromParamExpr(returnResultTypeExpr(fnType, i))
				analysis.recordImportedInterfaceImplForType(expected, results.At(i).Type())
				analysis.recordExternalLocalInterfaceArg(expected, results.At(i).Type())
			}
			return
		}
	}
	for i, result := range ret.Results {
		expected := expectedTypeFromParamExpr(returnResultTypeExpr(fnType, i))
		analysis.recordImportedInterfaceImpl(expected, result, typeInfo)
		analysis.recordExternalLocalInterfaceArg(expected, typeInfo.GetType(result))
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
	if arg == nil || typeInfo == nil {
		return
	}
	analysis.recordImportedInterfaceImplForType(expected, typeInfo.GetType(arg))
}

func (analysis *transpileFileAnalysis) recordImportedInterfaceImplForType(expected, argType types.Type) {
	if expected == nil || argType == nil {
		return
	}
	ifaceName, ifaceType, ok := importedTranspiledInterfaceFromType(expected)
	if !ok {
		return
	}
	if !types.Implements(argType, ifaceType) {
		return
	}
	named, isPointer, ok := currentPackageConcreteNamedTypeForm(argType)
	if !ok {
		return
	}
	typeName := named.Obj().Name()
	if !isPointer {
		analysis.recordImportedInterfaceImplByName(typeName, ifaceName, ifaceType)
		analysis.recordImportedEmbeddedInterfaceImpls(typeName, ifaceType)
		return
	}
	ifaceType.Complete()
	if types.Implements(named, ifaceType) {
		analysis.recordImportedInterfaceImplByName(typeName, ifaceName, ifaceType)
	}
	analysis.recordImportedEmbeddedInterfaceImplsForConcrete(typeName, named, ifaceType)
	analysis.recordImportedPointerInterfaceImplByName(typeName, ifaceName, ifaceType)
	analysis.recordImportedPointerEmbeddedInterfaceImpls(typeName, ifaceType)
}

func (analysis *transpileFileAnalysis) recordImportedInterfaceImplByName(typeName, ifaceName string, ifaceType *types.Interface) {
	if typeName == "" || ifaceName == "" || ifaceType == nil {
		return
	}
	if analysis.importedInterfaceImpls[typeName] == nil {
		analysis.importedInterfaceImpls[typeName] = make(map[string]*types.Interface)
	}
	analysis.importedInterfaceImpls[typeName][ifaceName] = ifaceType
}

func (analysis *transpileFileAnalysis) recordImportedEmbeddedInterfaceImpls(typeName string, ifaceType *types.Interface) {
	if ifaceType == nil {
		return
	}
	for i := 0; i < ifaceType.NumEmbeddeds(); i++ {
		embeddedName, embeddedIface, ok := importedTranspiledInterfaceFromType(ifaceType.EmbeddedType(i))
		if !ok {
			continue
		}
		analysis.recordImportedInterfaceImplByName(typeName, embeddedName, embeddedIface)
		analysis.recordImportedEmbeddedInterfaceImpls(typeName, embeddedIface)
	}
}

func (analysis *transpileFileAnalysis) recordImportedEmbeddedInterfaceImplsForConcrete(typeName string, concrete types.Type, ifaceType *types.Interface) {
	if concrete == nil || ifaceType == nil {
		return
	}
	for i := 0; i < ifaceType.NumEmbeddeds(); i++ {
		embeddedName, embeddedIface, ok := importedTranspiledInterfaceFromType(ifaceType.EmbeddedType(i))
		if !ok {
			continue
		}
		embeddedIface.Complete()
		if types.Implements(concrete, embeddedIface) {
			analysis.recordImportedInterfaceImplByName(typeName, embeddedName, embeddedIface)
		}
		analysis.recordImportedEmbeddedInterfaceImplsForConcrete(typeName, concrete, embeddedIface)
	}
}

func (analysis *transpileFileAnalysis) recordImportedPointerInterfaceImplByName(typeName, ifaceName string, ifaceType *types.Interface) {
	if typeName == "" || ifaceName == "" || ifaceType == nil {
		return
	}
	if analysis.importedPointerInterfaceImpls[typeName] == nil {
		analysis.importedPointerInterfaceImpls[typeName] = make(map[string]*types.Interface)
	}
	analysis.importedPointerInterfaceImpls[typeName][ifaceName] = ifaceType
}

func (analysis *transpileFileAnalysis) recordImportedPointerEmbeddedInterfaceImpls(typeName string, ifaceType *types.Interface) {
	if ifaceType == nil {
		return
	}
	for i := 0; i < ifaceType.NumEmbeddeds(); i++ {
		embeddedName, embeddedIface, ok := importedTranspiledInterfaceFromType(ifaceType.EmbeddedType(i))
		if !ok {
			continue
		}
		analysis.recordImportedPointerInterfaceImplByName(typeName, embeddedName, embeddedIface)
		analysis.recordImportedPointerEmbeddedInterfaceImpls(typeName, embeddedIface)
	}
}

func (analysis *transpileFileAnalysis) recordImportedOrderedConstraintImpls(typeInfo *TypeInfo) {
	if typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return
	}
	orderedIfaces := importedOrderedConstraintInterfaces(typeInfo)
	if len(orderedIfaces) == 0 {
		return
	}
	scope := typeInfo.pkg.Scope()
	for _, name := range scope.Names() {
		typeName, ok := scope.Lookup(name).(*types.TypeName)
		if !ok {
			continue
		}
		named, ok := types.Unalias(typeName.Type()).(*types.Named)
		if !ok || named.Obj() == nil || named.Obj().Pkg() != typeInfo.pkg {
			continue
		}
		if _, isInterface := types.Unalias(named.Underlying()).(*types.Interface); isInterface {
			continue
		}
		for ifaceName, ifaceType := range orderedIfaces {
			if !types.Satisfies(named, ifaceType) {
				continue
			}
			if analysis.importedInterfaceImpls[name] == nil {
				analysis.importedInterfaceImpls[name] = make(map[string]*types.Interface)
			}
			analysis.importedInterfaceImpls[name][ifaceName] = ifaceType
		}
	}
}

func importedOrderedConstraintInterfaces(typeInfo *TypeInfo) map[string]*types.Interface {
	if typeInfo == nil || typeInfo.pkg == nil {
		return nil
	}
	interfaces := make(map[string]*types.Interface)
	for _, pkg := range typeInfo.pkg.Imports() {
		if pkg == nil || pkg.Scope() == nil || isStubBackedStdlibPackagePath(pkg.Path()) {
			continue
		}
		for _, name := range pkg.Scope().Names() {
			typeName, ok := pkg.Scope().Lookup(name).(*types.TypeName)
			if !ok {
				continue
			}
			named, ok := types.Unalias(typeName.Type()).(*types.Named)
			if !ok || named.Obj() == nil {
				continue
			}
			iface, ok := named.Underlying().(*types.Interface)
			if !ok || !interfaceEmbedsOnlyOrderedTerms(iface) {
				continue
			}
			iface.Complete()
			interfaces[goTypesNamedTypeToRust(named)] = iface
		}
	}
	return interfaces
}

func (analysis *transpileFileAnalysis) recordExternalLocalInterfaceArg(expected, argType types.Type) {
	if expected == nil || argType == nil {
		return
	}
	ifaceName, ok := localNamedInterfaceTypeNameFromTypes(expected)
	if !ok {
		return
	}
	named, ok := types.Unalias(expected).(*types.Named)
	if !ok {
		return
	}
	ifaceType, ok := named.Underlying().(*types.Interface)
	if !ok {
		return
	}
	ifaceType.Complete()
	analysis.externalLocalInterfaceArgs = append(analysis.externalLocalInterfaceArgs, externalLocalInterfaceArg{
		ifaceName: ifaceName,
		ifaceType: ifaceType,
		argType:   argType,
	})
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
		sourcePointerWrapper := false
		if ptr, ok := concreteType.(*types.Pointer); ok {
			concreteType = ptr.Elem()
			sourcePointerWrapper = true
		}
		named, ok := types.Unalias(concreteType).(*types.Named)
		if !ok || named.Obj() == nil || named.Obj().Pkg() == nil || named.Obj().Pkg() == typeInfo.pkg {
			return
		}
		rustType := goTypesNamedTypeToRust(named)
		_, sourceIsInterface := types.Unalias(named.Underlying()).(*types.Interface)
		implRustType := rustType
		if sourceIsInterface && !isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
			implRustType = rustLocalInterfaceTraitObject(rustType)
			sourcePointerWrapper = false
		} else if sourcePointerWrapper && sourceMappedPointerWrapperAvailable(named) {
			implRustType = sourceMappedPointerWrapperTypeName(named)
		} else {
			sourcePointerWrapper = false
		}
		if impls[ifaceName] == nil {
			impls[ifaceName] = make(map[string]externalLocalInterfaceImpl)
		}
		impls[ifaceName][implRustType] = externalLocalInterfaceImpl{
			ifaceAST:              interfaces[ifaceName],
			ifaceType:             ifaceType,
			sourceIsInterface:     sourceIsInterface,
			sourcePointerWrapper:  sourcePointerWrapper,
			sourcePointeeRustType: rustType,
			sourcePkgPath:         named.Obj().Pkg().Path(),
			sourceTypeName:        named.Obj().Name(),
		}
	}

	for _, arg := range analysis.externalLocalInterfaceArgs {
		record(arg.ifaceName, arg.ifaceType, arg.argType)
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

func fieldAccessInfoFromSelection(sel *ast.SelectorExpr, typeInfo *TypeInfo) (FieldAccessInfo, bool) {
	if sel == nil || typeInfo == nil || typeInfo.info == nil {
		return FieldAccessInfo{}, false
	}
	selection, ok := typeInfo.info.Selections[sel]
	if !ok {
		return fieldAccessInfoFromLookup(sel, typeInfo)
	}
	if selection.Kind() != types.FieldVal {
		return FieldAccessInfo{}, false
	}
	return fieldAccessInfoFromTypeIndex(selection.Recv(), selection.Index())
}

func fieldAccessInfoFromLookup(sel *ast.SelectorExpr, typeInfo *TypeInfo) (FieldAccessInfo, bool) {
	if sel == nil || typeInfo == nil || sel.X == nil {
		return FieldAccessInfo{}, false
	}
	recv := typeInfo.GetType(sel.X)
	if recv == nil {
		return FieldAccessInfo{}, false
	}
	obj, index, _ := types.LookupFieldOrMethod(recv, true, typeInfo.pkg, sel.Sel.Name)
	field, ok := obj.(*types.Var)
	if !ok || field == nil || !field.IsField() {
		return FieldAccessInfo{}, false
	}
	return fieldAccessInfoFromTypeIndex(recv, index)
}

func fieldAccessInfoFromTypeIndex(recv types.Type, index []int) (FieldAccessInfo, bool) {
	if len(index) == 0 {
		return FieldAccessInfo{}, false
	}

	embeddedPath := make([]string, 0, len(index)-1)
	currentType := recv
	for i, fieldIndex := range index {
		structType, ok := structUnderlyingThroughPointers(currentType)
		if !ok || fieldIndex < 0 || fieldIndex >= structType.NumFields() {
			return FieldAccessInfo{}, false
		}
		field := structType.Field(fieldIndex)
		if i == len(index)-1 {
			return FieldAccessInfo{
				Found:        true,
				IsPromoted:   len(embeddedPath) > 0,
				EmbeddedPath: embeddedPath,
				FieldName:    ToSnakeCase(field.Name()),
			}, true
		}
		embeddedPath = append(embeddedPath, field.Name())
		currentType = field.Type()
	}
	return FieldAccessInfo{}, false
}

func structUnderlyingThroughPointers(typ types.Type) (*types.Struct, bool) {
	for typ != nil {
		typ = types.Unalias(typ)
		if ptr, ok := typ.(*types.Pointer); ok {
			typ = ptr.Elem()
			continue
		}
		underlying := types.Unalias(typ.Underlying())
		if ptr, ok := underlying.(*types.Pointer); ok {
			typ = ptr.Elem()
			continue
		}
		structType, ok := underlying.(*types.Struct)
		return structType, ok
	}
	return nil, false
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
	// A bridged stdlib interface is handled by its hand-written stub, not a
	// transpiled trait. A source-transpiled stdlib package emits a real Rust
	// trait, so a current-package type implementing it still needs an impl.
	if isStubBackedStdlibPackagePath(named.Obj().Pkg().Path()) {
		return "", nil, false
	}
	if sourceMappedDeclIsPruned(named.Obj()) {
		return "", nil, false
	}
	return goTypesNamedTypeToRust(named), intf, true
}

func currentPackageConcreteTypeName(typ types.Type) (string, bool) {
	name, _, ok := currentPackageConcreteTypeNameForm(typ)
	return name, ok
}

func currentPackageConcreteTypeNameForm(typ types.Type) (name string, isPointer bool, ok bool) {
	named, isPointer, ok := currentPackageConcreteNamedTypeForm(typ)
	if !ok {
		return "", false, false
	}
	return named.Obj().Name(), isPointer, true
}

func currentPackageConcreteNamedTypeForm(typ types.Type) (named *types.Named, isPointer bool, ok bool) {
	if typ == nil {
		return nil, false, false
	}
	typ = types.Unalias(typ)
	if ptr, ok := typ.(*types.Pointer); ok {
		typ = types.Unalias(ptr.Elem())
		isPointer = true
	}
	named, ok = typ.(*types.Named)
	if !ok || named.Obj() == nil {
		return nil, false, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || named.Obj().Pkg() != typeInfo.pkg {
		return nil, false, false
	}
	return named, isPointer, true
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
	ifaceAST              *ast.InterfaceType
	ifaceType             *types.Interface
	sourceIsInterface     bool
	sourcePointerWrapper  bool
	sourcePointeeRustType string
	sourcePkgPath         string
	sourceTypeName        string
}

type externalLocalInterfaceArg struct {
	ifaceName string
	ifaceType *types.Interface
	argType   types.Type
}

type localInterfaceAssertionCandidate struct {
	rustType string
	external bool
	typ      types.Type
}

type anonymousInterfaceAssertionTrait struct {
	name       string
	iface      *types.Interface
	candidates []localInterfaceAssertionCandidate
}

var anonymousInterfaceAssertionTraits = make(map[string]anonymousInterfaceAssertionTrait)

func anonymousInterfaceAssertionTypeString(typ types.Type) string {
	return types.TypeString(typ, func(pkg *types.Package) string {
		if pkg == nil {
			return ""
		}
		return pkg.Path()
	})
}

func anonymousInterfaceAssertionKey(iface *types.Interface, candidates []localInterfaceAssertionCandidate) string {
	if iface == nil {
		return ""
	}
	iface.Complete()
	methods := make([]string, 0, iface.NumMethods())
	for i := 0; i < iface.NumMethods(); i++ {
		method := iface.Method(i)
		methods = append(methods, method.Name()+":"+anonymousInterfaceAssertionTypeString(method.Type()))
	}
	sort.Strings(methods)
	candidateTypes := make([]string, 0, len(candidates))
	for _, candidate := range candidates {
		candidateTypes = append(candidateTypes, candidate.rustType+":"+anonymousInterfaceAssertionTypeString(candidate.typ))
	}
	sort.Strings(candidateTypes)
	return strings.Join(methods, ";") + "|" + strings.Join(candidateTypes, ";")
}

func registerAnonymousInterfaceAssertionTrait(iface *types.Interface, candidates []localInterfaceAssertionCandidate) string {
	key := anonymousInterfaceAssertionKey(iface, candidates)
	if key == "" {
		return ""
	}
	if anonymousInterfaceAssertionTraits == nil {
		anonymousInterfaceAssertionTraits = make(map[string]anonymousInterfaceAssertionTrait)
	}
	if trait, ok := anonymousInterfaceAssertionTraits[key]; ok {
		return trait.name
	}
	name := fmt.Sprintf("GoAnonymousInterface%d", len(anonymousInterfaceAssertionTraits)+1)
	anonymousInterfaceAssertionTraits[key] = anonymousInterfaceAssertionTrait{
		name:       name,
		iface:      iface,
		candidates: append([]localInterfaceAssertionCandidate(nil), candidates...),
	}
	return name
}

func writeAnonymousInterfaceAssertionTraits(out *strings.Builder, first *bool) {
	if len(anonymousInterfaceAssertionTraits) == 0 {
		return
	}
	traits := make([]anonymousInterfaceAssertionTrait, 0, len(anonymousInterfaceAssertionTraits))
	for _, trait := range anonymousInterfaceAssertionTraits {
		traits = append(traits, trait)
	}
	slices.SortFunc(traits, func(a, b anonymousInterfaceAssertionTrait) int {
		return strings.Compare(a.name, b.name)
	})
	for _, trait := range traits {
		if !*first {
			out.WriteString("\n\n")
		}
		*first = false
		writeAnonymousInterfaceAssertionTrait(out, trait)
	}
}

func writeAnonymousInterfaceAssertionTrait(out *strings.Builder, trait anonymousInterfaceAssertionTrait) {
	if trait.name == "" || trait.iface == nil {
		return
	}
	TrackImport("Any")
	TrackImport("Display")
	trait.iface.Complete()
	traitSnake := traitMethodSuffix(trait.name)

	out.WriteString("pub trait ")
	out.WriteString(trait.name)
	out.WriteString(": std::fmt::Display + Any {\n")
	out.WriteString("    fn __go_clone_box_")
	out.WriteString(traitSnake)
	out.WriteString("(&self) -> ")
	out.WriteString(rustLocalInterfaceTraitObject(trait.name))
	out.WriteString(";\n")
	out.WriteString("    fn __go_as_any(&self) -> &dyn Any;\n")
	out.WriteString("    fn __go_eq_")
	out.WriteString(traitSnake)
	out.WriteString("(&self, other: ")
	out.WriteString(rustLocalInterfaceParamBare(trait.name))
	out.WriteString(") -> bool;\n")
	for i := 0; i < trait.iface.NumMethods(); i++ {
		writeAnonymousInterfaceTraitMethodSigFromTypes(out, trait.iface.Method(i))
	}
	out.WriteString("}\n\nimpl Clone for ")
	out.WriteString(rustLocalInterfaceTraitObject(trait.name))
	out.WriteString(" {\n")
	out.WriteString("    fn clone(&self) -> Self {\n")
	out.WriteString("        ")
	out.WriteString(trait.name)
	out.WriteString("::__go_clone_box_")
	out.WriteString(traitSnake)
	out.WriteString("(self.as_ref())\n")
	out.WriteString("    }\n")
	out.WriteString("}")

	for _, candidate := range trait.candidates {
		out.WriteString("\n\nimpl ")
		out.WriteString(trait.name)
		out.WriteString(" for ")
		out.WriteString(candidate.rustType)
		out.WriteString(" {\n")
		for i := 0; i < trait.iface.NumMethods(); i++ {
			writeLocalInterfaceForwardMethodFromTypes(out, trait.iface.Method(i), candidate.rustType)
		}
		writeAnonymousInterfaceSupportImpl(out, trait.name, candidate.rustType)
		out.WriteString("}")
	}
}

func writeAnonymousInterfaceTraitMethodSigFromTypes(out *strings.Builder, method *types.Func) {
	if method == nil {
		return
	}
	sig, ok := method.Type().(*types.Signature)
	if !ok {
		return
	}
	out.WriteString("    fn ")
	out.WriteString(ToSnakeCase(method.Name()))
	out.WriteString("(")
	if interfaceMethodRequiresMutableReceiver(method) {
		out.WriteString("&mut self")
	} else {
		out.WriteString("&self")
	}
	params := sig.Params()
	for j := 0; j < params.Len(); j++ {
		param := params.At(j)
		paramName := param.Name()
		if paramName == "" {
			paramName = fmt.Sprintf("__arg%d", j)
		}
		out.WriteString(", ")
		out.WriteString(RustLocalIdent(paramName))
		out.WriteString(": ")
		out.WriteString(goTypesParamTypeToRust(param.Type()))
	}
	out.WriteString(")")
	writeInterfaceMethodResultTypesFromFunc(out, method)
	out.WriteString(";\n")
}

func writeAnonymousInterfaceSupportImpl(out *strings.Builder, ifaceName, concreteType string) {
	TrackImport("Any")
	traitSnake := traitMethodSuffix(ifaceName)
	out.WriteString("    fn __go_clone_box_")
	out.WriteString(traitSnake)
	out.WriteString("(&self) -> ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString(" {\n")
	out.WriteString("        Box::new(self.clone()) as ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString("\n")
	out.WriteString("    }\n")
	out.WriteString("    fn __go_as_any(&self) -> &dyn Any {\n")
	out.WriteString("        self\n")
	out.WriteString("    }\n")
	out.WriteString("    fn __go_eq_")
	out.WriteString(traitSnake)
	out.WriteString("(&self, other: ")
	out.WriteString(rustLocalInterfaceParamBare(ifaceName))
	out.WriteString(") -> bool {\n")
	out.WriteString("        if let Some(__other) = other.__go_as_any().downcast_ref::<")
	out.WriteString(concreteType)
	out.WriteString(">() {\n")
	if localConcreteTypeCanUsePartialEq(concreteType) {
		out.WriteString("            self == __other\n")
	} else {
		out.WriteString("            false\n")
	}
	out.WriteString("        } else {\n")
	out.WriteString("            false\n")
	out.WriteString("        }\n")
	out.WriteString("    }\n")
}

func collectFunctionLocalInterfaces(file *ast.File) map[string]*ast.InterfaceType {
	if file == nil {
		return make(map[string]*ast.InterfaceType)
	}
	return analyzeTranspileFile(file, GetTypeInfo()).functionLocalInterfaces
}

func collectPackageInterfaceDecls(files []*ast.File) map[string]*ast.InterfaceType {
	interfaces := make(map[string]*ast.InterfaceType)
	for _, file := range files {
		if file == nil {
			continue
		}
		for _, decl := range file.Decls {
			genDecl, ok := decl.(*ast.GenDecl)
			if !ok || genDecl.Tok != token.TYPE {
				continue
			}
			for _, spec := range genDecl.Specs {
				typeSpec, ok := spec.(*ast.TypeSpec)
				if !ok {
					continue
				}
				if iface, ok := typeSpec.Type.(*ast.InterfaceType); ok {
					interfaces[typeSpec.Name.Name] = iface
				}
			}
		}
	}
	return interfaces
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
		if sourceMappedDeclIsPruned(named.Obj()) {
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
		rustType := localInterfaceAssertionCandidateRustType(named, matchedForm, sourceType)
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

func localInterfaceAssertionCandidateRustType(named *types.Named, matchedForm types.Type, sourceType types.Type) string {
	rustType := goTypesNamedTypeToRust(named)
	if named == nil || named.Obj() == nil {
		return rustType
	}
	if _, ok := types.Unalias(matchedForm).(*types.Pointer); !ok {
		return rustType
	}
	if isGoErrorType(sourceType) {
		return rustType
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil && named.Obj().Pkg() == typeInfo.pkg {
		return pointerLocalInterfaceWrapperNameForUse(named.Obj().Name(), "")
	}
	if sourceMappedPointerWrapperAvailable(named) {
		return sourceMappedPointerWrapperTypeName(named)
	}
	return rustType
}

func sourceMappedPointerWrapperAvailable(named *types.Named) bool {
	if named == nil || named.Obj() == nil || named.Obj().Pkg() == nil {
		return false
	}
	pkg := named.Obj().Pkg()
	if !isSourceMappedPackagePath(pkg.Path()) || pkg.Scope() == nil {
		return false
	}
	ptr := types.NewPointer(named)
	for _, name := range pkg.Scope().Names() {
		obj, ok := pkg.Scope().Lookup(name).(*types.TypeName)
		if !ok {
			continue
		}
		ifaceNamed, ok := types.Unalias(obj.Type()).(*types.Named)
		if !ok {
			continue
		}
		iface, ok := ifaceNamed.Underlying().(*types.Interface)
		if !ok || iface.NumMethods() == 0 {
			continue
		}
		iface.Complete()
		if types.Implements(ptr, iface) {
			return true
		}
	}
	return false
}

func sourceMappedPointerWrapperAvailableForInterface(named *types.Named, expected types.Type) bool {
	if named == nil || named.Obj() == nil || named.Obj().Pkg() == nil || expected == nil {
		return false
	}
	if !isSourceMappedPackagePath(named.Obj().Pkg().Path()) {
		return false
	}
	if !sourceMappedPointerWrapperAvailable(named) {
		return false
	}
	iface, ok := types.Unalias(expected).Underlying().(*types.Interface)
	if !ok || iface.NumMethods() == 0 {
		return false
	}
	iface.Complete()
	return types.Implements(types.NewPointer(named), iface)
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

// anonInterfaceMethodSet returns the *types.Interface for a type expression that
// denotes an anonymous (non-named) interface carrying at least one method, such
// as `interface{ Name() string }`. Named interfaces (handled by the trait-object
// path) and empty interfaces (handled as Box<dyn Any>) return false so the
// caller keeps its existing behavior.
func anonInterfaceMethodSet(typeExpr ast.Expr) (*types.Interface, bool) {
	if typeExpr == nil {
		return nil, false
	}
	if _, ok := typeExpr.(*ast.InterfaceType); !ok {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	typ := typeInfo.GetType(typeExpr)
	if typ == nil {
		return nil, false
	}
	// A non-named interface literal has no *types.Named wrapper; its type is the
	// *types.Interface itself (the named-interface path covers the Named case).
	iface, ok := types.Unalias(typ).(*types.Interface)
	if !ok || iface.NumMethods() == 0 {
		return nil, false
	}
	return iface, true
}

// anonInterfaceAssertionTarget resolves a type-assertion whose target is an
// anonymous interface literal with methods (e.g. `x.(interface{ Name() string })`)
// to its structural candidates against the source. The named-interface path
// (localInterfaceAssertionTarget) covers `*types.Named` interfaces; this covers
// the unnamed method-set form that otherwise lowered to a soft `Unknown`/`Box<dyn Any>`.
func anonInterfaceAssertionTarget(e *ast.TypeAssertExpr) (sourceType types.Type, iface *types.Interface, candidates []localInterfaceAssertionCandidate, ok bool) {
	if e == nil || e.Type == nil {
		return nil, nil, nil, false
	}
	iface, ok = anonInterfaceMethodSet(e.Type)
	if !ok {
		return nil, nil, nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, nil, nil, false
	}
	sourceType = typeInfo.GetType(e.X)
	candidates = localInterfaceAssertionCandidates(iface, sourceType)
	return sourceType, iface, candidates, true
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

func currentPackageNamedType(typeName string) (*types.Named, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return nil, false
	}
	obj, ok := typeInfo.pkg.Scope().Lookup(typeName).(*types.TypeName)
	if !ok {
		return nil, false
	}
	named, ok := types.Unalias(obj.Type()).(*types.Named)
	return named, ok
}

func currentPackageTypeImplementsInterface(typeName string, iface *types.Interface) bool {
	named, ok := currentPackageNamedType(typeName)
	if !ok || iface == nil {
		return false
	}
	iface.Complete()
	if types.Implements(named, iface) {
		return true
	}
	return types.Implements(types.NewPointer(named), iface)
}

func currentPackagePointerImplementsInterface(typeName string, iface *types.Interface) bool {
	named, ok := currentPackageNamedType(typeName)
	if !ok || iface == nil {
		return false
	}
	iface.Complete()
	return types.Implements(types.NewPointer(named), iface)
}

func typeAliasSkipsLocalImpl(typeName string) bool {
	return IsTypeAlias(typeName) && !IsFunctionTypeAlias(typeName)
}

func explicitInterfaceMethods(iface *types.Interface) []*types.Func {
	if iface == nil {
		return nil
	}
	methods := make([]*types.Func, 0, iface.NumExplicitMethods())
	for i := 0; i < iface.NumExplicitMethods(); i++ {
		methods = append(methods, iface.ExplicitMethod(i))
	}
	return methods
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

func writeExternalLocalInterfaceMethod(out *strings.Builder, ifaceName string, methodName string, funcType *ast.FuncType, impl externalLocalInterfaceImpl, methodObj *types.Func) {
	out.WriteString("    fn ")
	out.WriteString(ToSnakeCase(methodName))
	mutableReceiver := interfaceTraitMethodRequiresMutableReceiver(ifaceName, methodName, methodObj)
	if mutableReceiver {
		out.WriteString("(&mut self")
	} else {
		out.WriteString("(&self")
	}
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
	if methodObj != nil {
		writeInterfaceMethodResultTypesFromFunc(out, methodObj)
	} else {
		writeFunctionResultTypes(out, funcType)
	}
	out.WriteString(" {\n")
	if impl.sourcePointerWrapper {
		methodRustName := RustFunctionName(methodName)
		if methodObj != nil {
			methodRustName = rustMethodNameForTypesFunc(methodObj)
		}
		receiverBorrowMutable := mutableReceiver || externalLocalInterfacePointeeMethodRequiresMutableReceiver(impl, methodName)
		out.WriteString("        {\n")
		out.WriteString("            let ")
		if receiverBorrowMutable {
			out.WriteString("mut ")
		}
		out.WriteString("__recv_guard = self.0")
		WriteBorrowMethod(out, receiverBorrowMutable)
		out.WriteString(";\n")
		out.WriteString("            let __recv = __recv_guard.")
		if receiverBorrowMutable {
			out.WriteString("as_mut")
		} else {
			out.WriteString("as_ref")
		}
		out.WriteString("().unwrap();\n")
		out.WriteString("            ")
		out.WriteString(impl.sourcePointeeRustType)
		out.WriteString("::")
		out.WriteString(methodRustName)
		out.WriteString("(__recv")
		for _, argName := range argNames {
			out.WriteString(", ")
			out.WriteString(argName)
		}
		out.WriteString(")\n")
		out.WriteString("        }\n")
		out.WriteString("    }\n")
		return
	}
	out.WriteString("        ")
	if impl.sourceIsInterface {
		out.WriteString("(**self).")
	} else {
		out.WriteString("self.")
	}
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

func externalLocalInterfacePointeeMethodRequiresMutableReceiver(impl externalLocalInterfaceImpl, methodName string) bool {
	if !impl.sourcePointerWrapper || impl.sourcePkgPath == "" || impl.sourceTypeName == "" || methodName == "" {
		return false
	}
	key := packageMethodReceiverMutabilityKey(impl.sourcePkgPath, impl.sourceTypeName, methodName)
	mutable, ok := packageMethodReceiverMutability[key]
	return ok && mutable
}

func writeExternalLocalInterfaceSupportImpl(out *strings.Builder, ifaceName, concreteType string, ifaceType *types.Interface) {
	TrackImport("Any")
	traitSnake := traitMethodSuffix(ifaceName)
	out.WriteString("    fn __go_clone_box_")
	out.WriteString(traitSnake)
	out.WriteString("(&self) -> ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString(" {\n")
	out.WriteString("        Box::new(self.clone()) as ")
	out.WriteString(rustLocalInterfaceTraitObject(ifaceName))
	out.WriteString("\n")
	out.WriteString("    }\n")
	if !interfaceTypeHasNamedEmbedded(ifaceType) {
		out.WriteString("    fn __go_as_any(&self) -> &dyn Any {\n")
		out.WriteString("        self\n")
		out.WriteString("    }\n")
	}
	out.WriteString("    fn __go_eq_")
	out.WriteString(traitSnake)
	out.WriteString("(&self, other: ")
	out.WriteString(rustLocalInterfaceParamBare(ifaceName))
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

func filterExternalLocalInterfaceImplsForInterfaces(impls map[string]map[string]externalLocalInterfaceImpl, interfaces map[string]*ast.InterfaceType) map[string]map[string]externalLocalInterfaceImpl {
	if len(impls) == 0 || len(interfaces) == 0 {
		return nil
	}
	filtered := make(map[string]map[string]externalLocalInterfaceImpl)
	for ifaceName, implsByType := range impls {
		ifaceAST, ok := interfaces[ifaceName]
		if !ok {
			continue
		}
		for concreteType, impl := range implsByType {
			impl.ifaceAST = ifaceAST
			if filtered[ifaceName] == nil {
				filtered[ifaceName] = make(map[string]externalLocalInterfaceImpl)
			}
			filtered[ifaceName][concreteType] = impl
		}
	}
	return filtered
}

func mergeExternalLocalInterfaceImpls(dst, src map[string]map[string]externalLocalInterfaceImpl) map[string]map[string]externalLocalInterfaceImpl {
	if len(src) == 0 {
		return dst
	}
	if dst == nil {
		dst = make(map[string]map[string]externalLocalInterfaceImpl)
	}
	for ifaceName, implsByType := range src {
		if dst[ifaceName] == nil {
			dst[ifaceName] = make(map[string]externalLocalInterfaceImpl)
		}
		for concreteType, impl := range implsByType {
			dst[ifaceName][concreteType] = impl
		}
	}
	return dst
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
					writeExternalLocalInterfaceMethod(out, ifaceName, method.Names[0].Name, funcType, impl, interfaceMethodByName(impl.ifaceType, method.Names[0].Name))
				}
			}
			writeExternalLocalInterfaceSupportImpl(out, ifaceName, concreteType, impl.ifaceType)
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

func writePromotedMethodForwarders(out *strings.Builder, structDefName string, structDef *StructDef, packageMethods map[string][]*ast.FuncDecl, existingMethodNames map[string]bool, existingRustMethodNames map[string]bool, methodCount *int) {
	if structDef == nil || methodCount == nil {
		return
	}
	if existingMethodNames == nil {
		existingMethodNames = make(map[string]bool)
	}
	if existingRustMethodNames == nil {
		existingRustMethodNames = make(map[string]bool)
	}

	promotedMethods := make(map[string]struct {
		embeddedType string
		method       *ast.FuncDecl
	})
	collectPromotedMethods(structDef, packageMethods, promotedMethods)

	var promotedMethodNames []string
	for methodName := range promotedMethods {
		promotedMethodNames = append(promotedMethodNames, methodName)
	}
	slices.Sort(promotedMethodNames)

	for _, methodName := range promotedMethodNames {
		methodInfo := promotedMethods[methodName]
		methodRustName := rustMethodName(methodInfo.method)
		if existingMethodNames[methodName] || existingRustMethodNames[methodRustName] {
			continue
		}
		if *methodCount > 0 {
			out.WriteString("\n")
		}
		generatePromotedMethod(out, methodInfo.method, structDefName, methodInfo.embeddedType)
		existingMethodNames[methodName] = true
		existingRustMethodNames[methodRustName] = true
		*methodCount = *methodCount + 1
	}

	for _, promotedMethod := range collectExternalPromotedMethods(structDefName, structDef, existingRustMethodNames) {
		if *methodCount > 0 {
			out.WriteString("\n")
		}
		generateExternalPromotedMethod(out, promotedMethod)
		existingRustMethodNames[promotedMethod.RustMethodName] = true
		*methodCount = *methodCount + 1
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
func generatePromotedMethod(out *strings.Builder, method *ast.FuncDecl, ownerTypeName string, embeddedTypeName string) {
	mutableReceiver := methodRequiresMutableReceiver(method)
	params := promotedMethodParamBindings(method.Type.Params)
	goPtrResultInfos := goPtrResultInfosForFunc(method)
	goPtrResultSig, hasGoPtrResultSig := funcDeclSignatureFromTypeInfo(method)
	goPtrEmbeddedField := generatedGoPtrFieldForStructNameField(ownerTypeName, embeddedTypeName)

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
		if len(params) > 0 {
			out.WriteString(", ")
		}
	}

	// Other parameters
	for i, param := range params {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(param.name)
		out.WriteString(": ")
		if elemRustType, ok := goPtrParamDeclElemRustType(method, i); ok {
			out.WriteString("GoPtr<")
			out.WriteString(elemRustType)
			out.WriteString(">")
		} else {
			out.WriteString(GoTypeToRust(param.typ))
		}
	}

	out.WriteString(")")

	if len(goPtrResultInfos) > 0 && hasGoPtrResultSig {
		writeGoPtrFuncResultTypesFromInfos(out, goPtrResultSig, goPtrResultInfos)
	} else {
		writeFuncDeclResultTypes(out, method)
	}

	out.WriteString(" {\n")
	out.WriteString("        // Forward to embedded type's method\n")
	out.WriteString("        let embedded = self.")
	out.WriteString(ToSnakeCase(embeddedTypeName))
	out.WriteString(".clone();\n")
	if goPtrEmbeddedField {
		out.WriteString("        embedded.with_mut(|embedded_ref| { ")
		if !writePromotedGoPtrForwardedResult(out, goPtrResultSig, goPtrResultInfos, "", func() {
			writePromotedMethodForwardedCall(out, rustMethodName(method), params)
		}) {
			writePromotedMethodForwardedCall(out, rustMethodName(method), params)
		}
		out.WriteString(" })\n")
		out.WriteString("    }\n")
		return
	}
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
	out.WriteString("        ")
	if !writePromotedGoPtrForwardedResult(out, goPtrResultSig, goPtrResultInfos, "", func() {
		writePromotedMethodForwardedCall(out, rustMethodName(method), params)
	}) {
		writePromotedMethodForwardedCall(out, rustMethodName(method), params)
	}
	out.WriteString("\n")
	out.WriteString("    }\n")
}

func writePromotedMethodForwardedCall(out *strings.Builder, methodName string, params []promotedMethodParamBinding) {
	out.WriteString("embedded_ref.")
	out.WriteString(methodName)
	out.WriteString("(")
	for i, param := range params {
		if i > 0 {
			out.WriteString(", ")
		}
		out.WriteString(param.name)
	}
	out.WriteString(")")
}

type promotedMethodParamBinding struct {
	name string
	typ  ast.Expr
}

func promotedMethodParamBindings(params *ast.FieldList) []promotedMethodParamBinding {
	if params == nil {
		return nil
	}
	var bindings []promotedMethodParamBinding
	paramIndex := 0
	for _, field := range params.List {
		if len(field.Names) == 0 {
			bindings = append(bindings, promotedMethodParamBinding{
				name: fmt.Sprintf("__arg%d", paramIndex),
				typ:  field.Type,
			})
			paramIndex++
			continue
		}
		for _, name := range field.Names {
			bindings = append(bindings, promotedMethodParamBinding{
				name: RustLocalIdent(name.Name),
				typ:  field.Type,
			})
			paramIndex++
		}
	}
	return bindings
}

func generateExternalPromotedMethod(out *strings.Builder, method externalPromotedMethod) {
	sig := method.Signature
	if sig == nil {
		return
	}
	params := sig.Params()
	results := sig.Results()
	goPtrResultInfos, _ := goPtrResultInfosForFuncObject(method.Func)

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
	if method.RawEmbeddedField && method.MutableReceiver {
		out.WriteString("(&mut self")
	} else {
		out.WriteString("(&self")
	}
	for i := 0; i < params.Len(); i++ {
		if method.GenericArguments {
			fmt.Fprintf(out, ", _arg%d: T%d", i, i)
		} else {
			fmt.Fprintf(out, ", _arg%d: %s", i, goTypesParamTypeToRust(params.At(i).Type()))
		}
	}
	out.WriteString(")")
	if len(goPtrResultInfos) > 0 {
		writeGoPtrFuncResultTypesFromInfos(out, sig, goPtrResultInfos)
	} else if results.Len() > 0 {
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
	if method.RawEmbeddedField {
		out.WriteString("        let embedded_ref = ")
		if method.MutableReceiver {
			out.WriteString("&mut self.")
		} else {
			out.WriteString("&self.")
		}
		out.WriteString(method.EmbeddedFieldName)
		out.WriteString(";\n")
	} else {
		out.WriteString("        let embedded = self.")
		out.WriteString(method.EmbeddedFieldName)
		out.WriteString(".clone();\n")
		if method.GoPtrEmbeddedField {
			out.WriteString("        embedded.with_mut(|embedded_ref| { ")
			inputHelperQualifier := goPtrHelperQualifierForFunc(method.Func)
			if !writePromotedGoPtrForwardedResult(out, sig, goPtrResultInfos, inputHelperQualifier, func() {
				writeExternalPromotedMethodForwardedCall(out, method.RustMethodName, params.Len())
			}) {
				writeExternalPromotedMethodForwardedCall(out, method.RustMethodName, params.Len())
			}
			out.WriteString(" })\n")
			out.WriteString("    }\n")
			return
		}
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
	}
	out.WriteString("        ")
	inputHelperQualifier := goPtrHelperQualifierForFunc(method.Func)
	if !writePromotedGoPtrForwardedResult(out, sig, goPtrResultInfos, inputHelperQualifier, func() {
		writeExternalPromotedMethodForwardedCall(out, method.RustMethodName, params.Len())
	}) {
		writeExternalPromotedMethodForwardedCall(out, method.RustMethodName, params.Len())
	}
	out.WriteString("\n")
	out.WriteString("    }\n")
}

func writeExternalPromotedMethodForwardedCall(out *strings.Builder, methodName string, paramCount int) {
	out.WriteString("embedded_ref.")
	out.WriteString(methodName)
	out.WriteString("(")
	for i := 0; i < paramCount; i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		fmt.Fprintf(out, "_arg%d", i)
	}
	out.WriteString(")")
}

func writePromotedGoPtrForwardedResult(out *strings.Builder, sig *types.Signature, resultInfos map[int]goPtrResultInfo, inputHelperQualifier string, writeCall func()) bool {
	if len(resultInfos) == 0 || sig == nil || sig.Results() == nil || sig.Results().Len() == 0 {
		return false
	}
	results := sig.Results()
	if results.Len() == 1 {
		if _, ok := resultInfos[0]; !ok {
			return false
		}
		if inputHelperQualifier == "" {
			writeCall()
		} else {
			writeGoPtrConversion(out, inputHelperQualifier, "", writeCall)
		}
		return true
	}

	out.WriteString("{ let __result = ")
	writeCall()
	out.WriteString("; (")
	for i := 0; i < results.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		if _, ok := resultInfos[i]; ok && inputHelperQualifier != "" {
			index := i
			writeGoPtrConversion(out, inputHelperQualifier, "", func() {
				fmt.Fprintf(out, "__result.%d", index)
			})
		} else {
			fmt.Fprintf(out, "__result.%d", i)
		}
	}
	out.WriteString(") }")
	return true
}

func goPtrHelperQualifierForFunc(fn *types.Func) string {
	if fn == nil || fn.Pkg() == nil {
		return ""
	}
	return goPtrHelperQualifierForOwnerPackage(fn.Pkg().Path())
}

func shouldSplitGenericOwnMethodImpls(typeSpec *ast.TypeSpec, typeName string, typeMethods []*ast.FuncDecl, declaredTypeNames map[string]bool) bool {
	if typeSpec == nil || typeSpec.TypeParams == nil || len(typeSpec.TypeParams.List) == 0 || len(typeMethods) == 0 {
		return false
	}
	structDef, exists := structDefs[typeName]
	if !exists || !declaredTypeNames[typeName] {
		return true
	}
	if structDef.EmbedsError && !typeHasExplicitErrorStringMethod(typeMethods) {
		return false
	}
	return len(structDef.EmbeddedTypes) == 0
}

func writeGenericOwnMethodImpls(out *strings.Builder, rustTypeName string, typeSpec *ast.TypeSpec, typeMethods []*ast.FuncDecl, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	type methodImplGroup struct {
		generics rustTypeGenerics
		methods  []*ast.FuncDecl
	}

	boundKinds := genericMethodBoundKinds(typeMethods)
	var groups []methodImplGroup
	for _, method := range typeMethods {
		generics := rustGenericMethodImplGenerics(typeSpec, boundKinds[method])
		if len(groups) == 0 || !sameRustTypeGenerics(groups[len(groups)-1].generics, generics) {
			groups = append(groups, methodImplGroup{generics: generics})
		}
		groups[len(groups)-1].methods = append(groups[len(groups)-1].methods, method)
	}

	for i, group := range groups {
		if i > 0 {
			out.WriteString("\n\n")
		}
		writeRustInherentImplHeader(out, group.generics, rustTypeName)
		for j, method := range group.methods {
			if j > 0 {
				out.WriteString("\n")
			}
			TranspileMethodImpl(out, method, fileSet, comments)
		}
		out.WriteString("}")
	}
}

type genericMethodBoundKind int

const (
	genericMethodBoundNone         genericMethodBoundKind = 0
	genericMethodBoundGoValueClone genericMethodBoundKind = 1 << 0
	genericMethodBoundRustClone    genericMethodBoundKind = 1 << 1
)

func rustGenericMethodImplGenerics(typeSpec *ast.TypeSpec, boundKind genericMethodBoundKind) rustTypeGenerics {
	if boundKind&genericMethodBoundRustClone != 0 && boundKind&genericMethodBoundGoValueClone != 0 {
		return rustTypeGenericsForTypeSpecWithParam(typeSpec, rustCloneAndGoValueCloneTypeParam)
	}
	if boundKind&genericMethodBoundRustClone != 0 {
		return rustTypeGenericsForTypeSpec(typeSpec)
	}
	if boundKind&genericMethodBoundGoValueClone != 0 {
		return rustTypeGenericsForTypeSpecWithParam(typeSpec, rustGoValueCloneTypeParam)
	}
	return rustTypeGenericsForDeclarationTypeSpec(typeSpec)
}

func sameRustTypeGenerics(a rustTypeGenerics, b rustTypeGenerics) bool {
	return a.Decl == b.Decl && a.Use == b.Use && a.Where == b.Where
}

func genericMethodBoundKinds(typeMethods []*ast.FuncDecl) map[*ast.FuncDecl]genericMethodBoundKind {
	boundKinds := make(map[*ast.FuncDecl]genericMethodBoundKind)
	typeInfo := GetTypeInfo()
	methodByObject := make(map[types.Object]*ast.FuncDecl)
	methodByName := make(map[string]*ast.FuncDecl)
	if typeInfo != nil && typeInfo.info != nil {
		for _, method := range typeMethods {
			if method == nil || method.Name == nil {
				continue
			}
			methodByName[method.Name.Name] = method
			if obj := typeInfo.info.Defs[method.Name]; obj != nil {
				methodByObject[obj] = method
			}
		}
	}

	for _, method := range typeMethods {
		var boundKind genericMethodBoundKind
		if genericMethodUsesDirectTypeParamValue(method) {
			boundKind |= genericMethodBoundGoValueClone
		}
		if genericSignatureUsesGoPtrDirectTypeParam(method) {
			boundKind |= genericMethodBoundRustClone
		}
		boundKinds[method] = boundKind
	}
	if len(methodByObject) == 0 {
		return boundKinds
	}

	changed := true
	for changed {
		changed = false
		for _, method := range typeMethods {
			if boundKinds[method]&genericMethodBoundRustClone != 0 && boundKinds[method]&genericMethodBoundGoValueClone != 0 {
				continue
			}
			calledBound := methodCalledBoundKind(typeInfo, method, methodByObject, methodByName, boundKinds)
			if calledBound == genericMethodBoundNone {
				continue
			}
			nextBound := boundKinds[method] | calledBound
			if nextBound != boundKinds[method] {
				boundKinds[method] = nextBound
				changed = true
			}
		}
	}
	return boundKinds
}

func genericMethodUnionBoundKind(typeMethods []*ast.FuncDecl) genericMethodBoundKind {
	boundKinds := genericMethodBoundKinds(typeMethods)
	var union genericMethodBoundKind
	for _, boundKind := range boundKinds {
		union |= boundKind
	}
	return union
}

func collectPackageFunctions(files []*ast.File) []*ast.FuncDecl {
	var functions []*ast.FuncDecl
	for _, file := range files {
		if file == nil {
			continue
		}
		for _, decl := range file.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Recv != nil {
				continue
			}
			functions = append(functions, fn)
		}
	}
	return functions
}

func genericFunctionBoundKinds(functions []*ast.FuncDecl) map[*ast.FuncDecl]genericMethodBoundKind {
	boundKinds := make(map[*ast.FuncDecl]genericMethodBoundKind)
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return boundKinds
	}
	functionByObject := make(map[types.Object]*ast.FuncDecl)
	for _, fn := range functions {
		if fn == nil || fn.Name == nil {
			continue
		}
		if obj := typeInfo.info.Defs[fn.Name]; obj != nil {
			functionByObject[obj] = fn
		}
	}

	for _, fn := range functions {
		var boundKind genericMethodBoundKind
		if genericFunctionUsesDirectTypeParamValue(fn) {
			boundKind |= genericMethodBoundGoValueClone
		}
		if genericSignatureUsesGoPtrDirectTypeParam(fn) {
			boundKind |= genericMethodBoundRustClone
		}
		boundKinds[fn] = boundKind
	}
	if len(functionByObject) == 0 {
		return boundKinds
	}

	changed := true
	for changed {
		changed = false
		for _, fn := range functions {
			if boundKinds[fn]&genericMethodBoundRustClone != 0 && boundKinds[fn]&genericMethodBoundGoValueClone != 0 {
				continue
			}
			calledBound := functionCalledBoundKind(typeInfo, fn, functionByObject, boundKinds)
			if calledBound == genericMethodBoundNone {
				continue
			}
			nextBound := boundKinds[fn] | calledBound
			if nextBound != boundKinds[fn] {
				boundKinds[fn] = nextBound
				changed = true
			}
		}
	}
	return boundKinds
}

func collectLocalInterfaceGoValueCloneTypes(files []*ast.File, boundKinds map[*ast.FuncDecl]genericMethodBoundKind) map[string]bool {
	interfaces := make(map[string]bool)
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return interfaces
	}
	functionByObject := make(map[*types.Func]*ast.FuncDecl)
	for _, fn := range collectPackageFunctions(files) {
		if fn == nil || fn.Name == nil {
			continue
		}
		obj, ok := typeInfo.info.Defs[fn.Name].(*types.Func)
		if !ok || obj == nil {
			continue
		}
		functionByObject[obj] = fn
	}
	for _, file := range files {
		ast.Inspect(file, func(node ast.Node) bool {
			call, ok := node.(*ast.CallExpr)
			if !ok {
				return true
			}
			fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
			if !ok || fn == nil {
				return true
			}
			boundKind := genericMethodBoundNone
			if localFn := functionByObject[fn]; localFn != nil && boundKinds != nil {
				boundKind = boundKinds[localFn]
			} else if sourceInfo, ok := sourceFunctionDeclsByFunc[fn]; ok {
				boundKind = sourceInfo.boundKind
			}
			if boundKind&genericMethodBoundGoValueClone == 0 {
				return true
			}
			instance, ok := genericCallInstanceFromTypeInfo(typeInfo, call)
			if !ok {
				return true
			}
			for i := 0; i < instance.TypeArgs.Len(); i++ {
				ifaceName, ok := localNamedInterfaceTypeNameFromTypes(instance.TypeArgs.At(i))
				if !ok {
					continue
				}
				interfaces[RustTypeNameForUse(ifaceName)] = true
			}
			return true
		})
	}
	return interfaces
}

func collectLocalInterfaceGoComparableTypes(files []*ast.File) map[string]bool {
	interfaces := make(map[string]bool)
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return interfaces
	}
	for _, file := range files {
		ast.Inspect(file, func(node ast.Node) bool {
			call, ok := node.(*ast.CallExpr)
			if !ok {
				return true
			}
			fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
			if !ok || fn == nil {
				return true
			}
			comparableIndexes := comparableTypeParamIndexes(fn)
			if len(comparableIndexes) == 0 {
				return true
			}
			instance, ok := genericCallInstanceFromTypeInfo(typeInfo, call)
			if !ok {
				return true
			}
			for _, index := range comparableIndexes {
				if index >= instance.TypeArgs.Len() {
					continue
				}
				ifaceName, ok := localNamedInterfaceTypeNameFromTypes(instance.TypeArgs.At(index))
				if !ok {
					continue
				}
				interfaces[RustTypeNameForUse(ifaceName)] = true
			}
			return true
		})
	}
	return interfaces
}

func markLocalInterfaceMapKeyType(interfaces map[string]bool, typ types.Type) {
	if interfaces == nil {
		return
	}
	if ifaceName, ok := localNamedInterfaceTypeNameFromTypes(typ); ok {
		interfaces[RustTypeNameForUse(ifaceName)] = true
	}
}

func collectLocalInterfaceMapKeyTypes(files []*ast.File) map[string]bool {
	interfaces := make(map[string]bool)
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return interfaces
	}
	for _, file := range files {
		ast.Inspect(file, func(node ast.Node) bool {
			mapType, ok := node.(*ast.MapType)
			if !ok {
				return true
			}
			markLocalInterfaceMapKeyType(interfaces, mapKeyTypeFromMapType(typeInfo, mapType))
			return true
		})
	}
	return interfaces
}

func comparableTypeParamIndexes(fn *types.Func) []int {
	if fn == nil {
		return nil
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok || sig.TypeParams() == nil {
		return nil
	}
	var indexes []int
	for i := 0; i < sig.TypeParams().Len(); i++ {
		if goTypeParamHasComparableConstraint(sig.TypeParams().At(i)) {
			indexes = append(indexes, i)
		}
	}
	return indexes
}

func genericCallInstanceFromTypeInfo(typeInfo *TypeInfo, call *ast.CallExpr) (types.Instance, bool) {
	if typeInfo == nil || typeInfo.info == nil || typeInfo.info.Instances == nil || call == nil {
		return types.Instance{}, false
	}
	ident := genericCallInstanceIdent(call.Fun)
	if ident == nil {
		return types.Instance{}, false
	}
	instance, ok := typeInfo.info.Instances[ident]
	if !ok || instance.TypeArgs == nil || instance.TypeArgs.Len() == 0 {
		return types.Instance{}, false
	}
	return instance, true
}

func genericCallInstanceIdent(fun ast.Expr) *ast.Ident {
	switch e := unwrapParens(fun).(type) {
	case *ast.Ident:
		return e
	case *ast.SelectorExpr:
		return e.Sel
	case *ast.IndexExpr:
		_, ident, ok := genericFunctionTargetFromBase(e.X)
		if ok {
			return ident
		}
	case *ast.IndexListExpr:
		_, ident, ok := genericFunctionTargetFromBase(e.X)
		if ok {
			return ident
		}
	}
	return nil
}

func functionCalledBoundKind(typeInfo *TypeInfo, fn *ast.FuncDecl, functionByObject map[types.Object]*ast.FuncDecl, boundKinds map[*ast.FuncDecl]genericMethodBoundKind) genericMethodBoundKind {
	if typeInfo == nil || typeInfo.info == nil || fn == nil || fn.Body == nil {
		return genericMethodBoundNone
	}
	calledBound := genericMethodBoundNone
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		if calledBound&genericMethodBoundRustClone != 0 && calledBound&genericMethodBoundGoValueClone != 0 || node == nil {
			return false
		}
		call, ok := node.(*ast.CallExpr)
		if !ok {
			return true
		}
		targetObj, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
		if !ok {
			return true
		}
		target := functionByObject[targetObj]
		if target == nil {
			return true
		}
		nextBound := calledBound | boundKinds[target]
		if nextBound != calledBound {
			calledBound = nextBound
			if calledBound&genericMethodBoundRustClone != 0 && calledBound&genericMethodBoundGoValueClone != 0 {
				return false
			}
		}
		return true
	})
	return calledBound
}

func methodCalledBoundKind(typeInfo *TypeInfo, method *ast.FuncDecl, methodByObject map[types.Object]*ast.FuncDecl, methodByName map[string]*ast.FuncDecl, boundKinds map[*ast.FuncDecl]genericMethodBoundKind) genericMethodBoundKind {
	if typeInfo == nil || typeInfo.info == nil || method == nil || method.Body == nil {
		return genericMethodBoundNone
	}
	calledBound := genericMethodBoundNone
	ast.Inspect(method.Body, func(node ast.Node) bool {
		if calledBound&genericMethodBoundRustClone != 0 && calledBound&genericMethodBoundGoValueClone != 0 || node == nil {
			return false
		}
		call, ok := node.(*ast.CallExpr)
		if !ok {
			return true
		}
		sel, ok := unwrapParens(call.Fun).(*ast.SelectorExpr)
		if !ok {
			return true
		}
		selection := typeInfo.info.Selections[sel]
		if selection == nil || selection.Obj() == nil {
			return true
		}
		target := methodByObject[selection.Obj()]
		if target == nil {
			target = methodByName[selection.Obj().Name()]
		}
		if target != nil {
			nextBound := calledBound | boundKinds[target]
			if nextBound != calledBound {
				calledBound = nextBound
				if calledBound&genericMethodBoundRustClone != 0 && calledBound&genericMethodBoundGoValueClone != 0 {
					return false
				}
			}
		}
		return true
	})
	return calledBound
}

func genericMethodUsesDirectTypeParamValue(method *ast.FuncDecl) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || method == nil || method.Type == nil {
		return false
	}
	if fieldListHasDirectTypeParam(typeInfo, method.Type.Params) || fieldListHasDirectTypeParam(typeInfo, method.Type.Results) {
		return true
	}
	return methodBodyUsesDirectTypeParamValue(typeInfo, method.Body)
}

func genericSignatureUsesGoPtrDirectTypeParam(fn *ast.FuncDecl) bool {
	if fn == nil {
		return false
	}
	fnObj, hasFnObj := sliceElemPtrReturnFuncObject(fn)
	sig, ok := funcDeclSignatureFromTypeInfo(fn)
	if !ok {
		return false
	}
	if params := sig.Params(); params != nil {
		for i := 0; i < params.Len(); i++ {
			if hasFnObj {
				if info, ok := goPtrParamResultInfoForFunc(fnObj, i); ok && goPtrInfoElemIsDirectTypeParam(info) {
					return true
				}
			}
			if _, ok := goPtrParamInfoForDeclObject(fn, i); ok && pointerElemIsDirectTypeParam(params.At(i).Type()) {
				return true
			}
		}
	}
	if results := sig.Results(); results != nil {
		resultInfos := goPtrResultInfosForFunc(fn)
		for i := 0; i < results.Len(); i++ {
			if _, ok := resultInfos[i]; ok && pointerElemIsDirectTypeParam(results.At(i).Type()) {
				return true
			}
		}
	}
	return false
}

func goPtrInfoElemIsDirectTypeParam(info goPtrResultInfo) bool {
	if info.elemType == nil {
		return false
	}
	_, ok := types.Unalias(info.elemType).(*types.TypeParam)
	return ok
}

func pointerElemIsDirectTypeParam(typ types.Type) bool {
	ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	_, ok = types.Unalias(ptr.Elem()).(*types.TypeParam)
	return ok
}

func fieldListHasDirectTypeParam(typeInfo *TypeInfo, fields *ast.FieldList) bool {
	if fields == nil {
		return false
	}
	for _, field := range fields.List {
		if field == nil || field.Type == nil {
			continue
		}
		if exprHasDirectTypeParam(typeInfo, field.Type) {
			return true
		}
	}
	return false
}

func methodBodyUsesDirectTypeParamValue(typeInfo *TypeInfo, body *ast.BlockStmt) bool {
	if body == nil {
		return false
	}
	usesTypeParam := false
	ast.Inspect(body, func(node ast.Node) bool {
		if usesTypeParam || node == nil {
			return false
		}
		switch stmt := node.(type) {
		case *ast.ReturnStmt:
			for _, result := range stmt.Results {
				if exprHasDirectTypeParam(typeInfo, result) {
					usesTypeParam = true
					return false
				}
			}
		case *ast.AssignStmt:
			for _, rhs := range stmt.Rhs {
				if exprHasDirectTypeParam(typeInfo, rhs) {
					usesTypeParam = true
					return false
				}
			}
		case *ast.ValueSpec:
			for _, value := range stmt.Values {
				if exprHasDirectTypeParam(typeInfo, value) {
					usesTypeParam = true
					return false
				}
			}
		case *ast.CallExpr:
			for _, arg := range stmt.Args {
				if exprHasDirectTypeParam(typeInfo, arg) {
					usesTypeParam = true
					return false
				}
			}
		}
		return true
	})
	return usesTypeParam
}

func exprHasDirectTypeParam(typeInfo *TypeInfo, expr ast.Expr) bool {
	if typeInfo == nil || expr == nil {
		return false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := types.Unalias(typ).(*types.TypeParam)
	return ok
}

// getReceiverType extracts the type name from a receiver type expression
func getReceiverType(expr ast.Expr) string {
	switch t := expr.(type) {
	case *ast.Ident:
		return t.Name
	case *ast.StarExpr:
		return getReceiverType(t.X)
	case *ast.IndexExpr:
		return getReceiverType(t.X)
	case *ast.IndexListExpr:
		return getReceiverType(t.X)
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

// TranspileSource parses a raw Go source string and returns the transpiled
// Rust as a string. The source must be a complete Go file (with a `package`
// clause). External (non-stdlib) imports are not resolved in this mode —
// for projects with dependencies, use the file-based pipeline instead.
func TranspileSource(source string) (string, error) {
	fileSet := token.NewFileSet()
	file, err := parser.ParseFile(fileSet, "input.go", source, parser.ParseComments|parser.SkipObjectResolution)
	if err != nil {
		return "", fmt.Errorf("parse error: %v", err)
	}

	typeInfo, err := NewTypeInfo([]*ast.File{file}, fileSet)
	if err != nil {
		return "", fmt.Errorf("type check error: %v", err)
	}

	cd := NewConcurrencyDetector()
	cd.AnalyzeProject([]*ast.File{file})
	SetConcurrencyDetector(cd)
	defer SetConcurrencyDetector(nil)

	rust, _, _ := Transpile(file, fileSet, typeInfo)
	return rust, nil
}

func TranspileWithMapping(file *ast.File, fileSet *token.FileSet, typeInfo *TypeInfo, packageMapping map[string]string) (string, *ImportTracker, map[string]bool) {
	// Create trackers
	imports := NewImportTracker()
	helpers := &HelperTracker{}
	parentCtx := GetTranspileContext()
	if parentCtx == nil && typeInfo != nil && typeInfo.pkg != nil {
		resetPackageMethodReceiverMutability()
		registerPackageMethodReceiverMutability(typeInfo.pkg.Path(), []*ast.File{file})
		registerInterfaceMethodMutableReceivers([]*types.Package{typeInfo.pkg})
	}
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
		ctx.CurrentModuleName = parentCtx.CurrentModuleName
	}
	SetTranspileContext(ctx)
	defer SetTranspileContext(parentCtx)
	reserveSiblingRustTypeNames(imports)
	if currentContext != nil && currentContext.Package != nil && len(currentContext.Package.MethodNameOverrides) == 0 {
		currentContext.Package.MethodNameOverrides = assignPackageMethodNames([]*ast.File{file}, typeInfo)
		packageMethodNameOverrides = currentContext.Package.MethodNameOverrides
	}
	if currentContext != nil && currentContext.Package != nil && len(currentContext.Package.ConstantNameOverrides) == 0 {
		currentContext.Package.ConstantNameOverrides = assignPackageConstantNames([]*ast.File{file})
	}
	if currentContext != nil && currentContext.Package != nil && len(currentContext.Package.MethodsByType) == 0 {
		currentContext.Package.MethodsByType = collectPackageMethods([]*ast.File{file})
	}
	if currentContext != nil && currentContext.Package != nil && len(currentContext.Package.LocalInterfaceGoComparable) == 0 {
		currentContext.Package.LocalInterfaceGoComparable = collectLocalInterfaceGoComparableTypes([]*ast.File{file})
	}
	if currentContext != nil && currentContext.Package != nil && len(currentContext.Package.LocalInterfaceMapKeyTypes) == 0 {
		currentContext.Package.LocalInterfaceMapKeyTypes = collectLocalInterfaceMapKeyTypes([]*ast.File{file})
	}

	// Transpile the body
	var body strings.Builder
	fileAnalysis := analyzeTranspileFile(file, typeInfo)
	packageGlobalNames = make(map[string]bool)
	prevComparableStructTypes := comparableStructTypes
	prevLocalInterfaceEqualityTypes := localInterfaceEqualityTypes
	prevAnonymousInterfaceAssertionTraits := anonymousInterfaceAssertionTraits
	comparableStructTypes = make(map[string]bool)
	for name := range fileAnalysis.comparableStructTypes {
		comparableStructTypes[name] = true
	}
	if currentContext != nil && currentContext.Package != nil {
		for name := range currentContext.Package.ComparableStructTypes {
			comparableStructTypes[name] = true
		}
		for name := range currentContext.Package.MapKeyStructTypes {
			comparableStructTypes[name] = true
		}
		for name := range fileAnalysis.pointerComparablePointees {
			currentContext.Package.PointerComparablePointeeTypes[name] = true
		}
		for name := range fileAnalysis.localInterfaceMapKeyTypes {
			currentContext.Package.LocalInterfaceMapKeyTypes[name] = true
		}
	}
	localInterfaceEqualityTypes = fileAnalysis.localInterfaceEqualityTypes
	anonymousInterfaceAssertionTraits = make(map[string]anonymousInterfaceAssertionTrait)
	defer func() {
		comparableStructTypes = prevComparableStructTypes
		localInterfaceEqualityTypes = prevLocalInterfaceEqualityTypes
		anonymousInterfaceAssertionTraits = prevAnonymousInterfaceAssertionTraits
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
								registerTypeDefinitionForTypeExpr(typeSpec.Name.Name, typeSpec.Type)
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
	reserveFileRustTypeNames(imports, types)
	registerPackageGlobalNames(globalVars)
	nilPointerReceiverMethodHelpers := collectNilPointerReceiverMethodHelpers(file, typeInfo, methods)
	restoreNilPointerReceiverMethodHelpers := setCurrentNilPointerReceiverMethodHelpers(nilPointerReceiverMethodHelpers)
	defer restoreNilPointerReceiverMethodHelpers()
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
	registerSliceElemPtrFactsFromFiles([]*ast.File{file})
	if currentContext != nil && currentContext.Package != nil && len(currentContext.Package.FunctionBoundKinds) == 0 {
		currentContext.Package.FunctionBoundKinds = genericFunctionBoundKinds(collectPackageFunctions([]*ast.File{file}))
	}
	if currentContext != nil && currentContext.Package != nil && len(currentContext.Package.LocalInterfaceGoValueClone) == 0 {
		currentContext.Package.LocalInterfaceGoValueClone = collectLocalInterfaceGoValueCloneTypes([]*ast.File{file}, currentContext.Package.FunctionBoundKinds)
	}
	functionNames := assignFunctionNames(functions)
	if parentCtx == nil && currentContext != nil && currentContext.Package != nil {
		functionOverrideNames := currentContext.Package.FunctionNameOverrides
		if len(functionNames) > 0 {
			functionOverrideNames = make(map[string]string, len(currentContext.Package.FunctionNameOverrides)+len(functionNames))
			for goName, rustName := range currentContext.Package.FunctionNameOverrides {
				functionOverrideNames[goName] = rustName
			}
			for fn, rustName := range functionNames {
				if fn.Name.Name != "init" {
					functionOverrideNames[fn.Name.Name] = rustName
				}
			}
		}
		currentContext.Package.GlobalNameOverrides = assignPackageGlobalNameOverrides([]*ast.File{file}, functionOverrideNames)
		packageGlobalNameOverrides = currentContext.Package.GlobalNameOverrides
	}
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
			registerStructTypeSpecDef(t.spec, structType)
		}
	}
	markComparableInterfaceImplementorStructs(types, methods, interfaces)
	importedInterfaceImpls := fileAnalysis.importedInterfaceImpls
	if ctx := GetTranspileContext(); ctx != nil && ctx.Package != nil && len(ctx.Package.ImportedInterfaceImpls) > 0 {
		importedInterfaceImpls = ctx.Package.ImportedInterfaceImpls
	}
	importedPointerInterfaceImpls := fileAnalysis.importedPointerInterfaceImpls
	if ctx := GetTranspileContext(); ctx != nil && ctx.Package != nil && len(ctx.Package.ImportedPointerInterfaceImpls) > 0 {
		importedPointerInterfaceImpls = ctx.Package.ImportedPointerInterfaceImpls
	}
	externalLocalInterfaceImpls := fileAnalysis.externalLocalInterfaceImpls(interfaces)
	if ctx := GetTranspileContext(); ctx != nil && ctx.Package != nil && len(ctx.Package.ExternalLocalInterfaceImpls) > 0 {
		externalLocalInterfaceImpls = filterExternalLocalInterfaceImplsForInterfaces(externalLocalInterfaceImpls, interfaces)
		externalLocalInterfaceImpls = mergeExternalLocalInterfaceImpls(
			externalLocalInterfaceImpls,
			filterExternalLocalInterfaceImplsForInterfaces(ctx.Package.ExternalLocalInterfaceImpls, interfaces),
		)
	}
	// DCE: when a stdlib package is transpiled from source, types unreachable
	// from live code are pruned. prunedTypeNames gates the type decl, its impl
	// block, and any `impl LocalIface for T` so nothing references a dropped type.
	prunedTypeNames := map[string]bool{}
	for _, t := range types {
		if isPrunedSourceDecl(t.spec.Name) {
			prunedTypeNames[t.spec.Name.Name] = true
		}
	}
	for _, t := range types {
		if prunedTypeNames[t.spec.Name.Name] {
			continue
		}
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
	typeSpecByName := make(map[string]*ast.TypeSpec)
	for _, t := range types {
		declaredTypeNames[t.spec.Name.Name] = true
		typeSpecByName[t.spec.Name.Name] = t.spec
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

	for typeName := range importedInterfaceImpls {
		if declaredTypeNames[typeName] && !typesWithImpls[typeName] {
			typeNames = append(typeNames, typeName)
			typesWithImpls[typeName] = true
		}
	}

	ifaceNamesForDeclaredTypes := packageLocalInterfaceNames(interfaces)
	for _, t := range types {
		typeName := t.spec.Name.Name
		if typesWithImpls[typeName] {
			continue
		}
		if _, iface := localInterfaceNamedTypeByName(typeName); iface != nil {
			continue
		}
		for _, ifaceName := range ifaceNamesForDeclaredTypes {
			if prunedTypeNames[ifaceName] || localInterfaceTypeIsPruned(ifaceName) {
				continue
			}
			if currentPackageTypeImplementsInterface(typeName, localInterfaceTypesByName(ifaceName)) {
				typeNames = append(typeNames, typeName)
				typesWithImpls[typeName] = true
				break
			}
		}
	}

	packageMethods := methods
	if currentContext != nil && currentContext.Package != nil && len(currentContext.Package.MethodsByType) > 0 {
		packageMethods = currentContext.Package.MethodsByType
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
		if prunedTypeNames[typeName] {
			continue
		}
		if typeAliasSkipsLocalImpl(typeName) {
			continue
		}
		typeMethods := methods[typeName] // May be nil if type has no methods
		if typeMethods == nil {
			typeMethods = []*ast.FuncDecl{}
		}
		// Cross-file DCE: prunedTypeNames only covers types declared in this
		// file. When the type decl was pruned in another file, skip its impl
		// block here too so it doesn't reference a type that was never emitted.
		if implReceiverTypeIsPruned(typeMethods) {
			continue
		}
		previousTypeMethods := currentTypeMethods
		currentTypeMethods = typeMethods
		rustTypeName := rustImplTypeNameForUse(typeName)
		if !first {
			body.WriteString("\n\n")
		}
		first = false
		if IsFunctionTypeAlias(typeName) {
			writeFunctionTypeAliasMethodImpl(&body, rustTypeName, typeMethods, fileSet, file.Comments)
			writeFunctionTypeImportedInterfaceImpls(&body, rustTypeName, importedInterfaceImpls[typeName])
			currentTypeMethods = previousTypeMethods
			continue
		}
		typeSpec := typeSpecByName[typeName]
		if shouldSplitGenericOwnMethodImpls(typeSpec, typeName, typeMethods, declaredTypeNames) {
			writeGenericOwnMethodImpls(&body, rustTypeName, typeSpec, typeMethods, fileSet, file.Comments)
		} else {
			generics := rustGenericMethodImplGenerics(typeSpec, genericMethodUnionBoundKind(typeMethods))
			body.WriteString("impl")
			body.WriteString(generics.Decl)
			body.WriteString(" ")
			body.WriteString(rustTypeName)
			body.WriteString(generics.Use)
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
				existingRustMethodNames := make(map[string]bool)
				for _, ownMethod := range packageMethods[typeName] {
					existingMethodNames[ownMethod.Name.Name] = true
					existingRustMethodNames[rustMethodName(ownMethod)] = true
				}
				writePromotedMethodForwarders(&body, typeName, structDef, packageMethods, existingMethodNames, existingRustMethodNames, &methodCount)
			}

			body.WriteString("}")
		}

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

		if declaredTypeNames[typeName] {
			// Generate trait implementations for this type. Consider every local
			// interface declared anywhere in the package, not just the current file:
			// a concrete type in one file (circle.go) implements an interface
			// declared in another (shape.go), exactly like go/types' Basic/Signature/
			// ... implementing the Type interface from type.go. The loop body resolves
			// each name through the package scope (localInterfaceTypesByName) and only
			// emits an impl when currentPackageTypeImplementsInterface holds. The impl
			// belongs with the concrete type declaration; files that only add methods
			// emit inherent impl blocks but must not duplicate trait impls.
			// Sort interface names for deterministic output.
			ifaceNames := packageLocalInterfaceNames(interfaces)
			slices.Sort(ifaceNames)
			pointerWrapperEmitted := false

			for _, ifaceName := range ifaceNames {
				if prunedTypeNames[ifaceName] || localInterfaceTypeIsPruned(ifaceName) {
					continue
				}
				ifaceType := localInterfaceTypesByName(ifaceName)
				if !currentPackageTypeImplementsInterface(typeName, ifaceType) {
					continue
				}
				body.WriteString("\n\n")
				body.WriteString("impl ")
				body.WriteString(ifaceName)
				body.WriteString(" for ")
				body.WriteString(rustTypeName)
				body.WriteString(" {\n")

				// Generate trait method implementations for directly-declared
				// methods only. Methods inherited from embedded named local
				// interfaces are provided by the supertrait's impl block —
				// duplicating them here would create method-resolution ambiguity.
				for _, method := range explicitInterfaceMethods(ifaceType) {
					writeLocalInterfaceForwardMethodFromTypes(&body, method, rustTypeName)
				}

				writeLocalInterfaceSupportImpl(&body, ifaceName, typeName, ifaceType)
				body.WriteString("}")

				if currentPackagePointerImplementsInterface(typeName, ifaceType) {
					body.WriteString("\n\n")
					writePointerLocalInterfaceWrapper(&body, typeName, ifaceName, ifaceType, !pointerWrapperEmitted)
					pointerWrapperEmitted = true
				}
			}

			var importedIfaceNames []string
			for ifaceName := range importedInterfaceImpls[typeName] {
				importedIfaceNames = append(importedIfaceNames, ifaceName)
			}
			slices.Sort(importedIfaceNames)
			importedTraitMethods := packageMethods[typeName]
			if importedTraitMethods == nil {
				importedTraitMethods = typeMethods
			}
			for _, ifaceName := range importedIfaceNames {
				ifaceType := importedInterfaceImpls[typeName][ifaceName]
				if !typeMethodsImplementTypesInterface(importedTraitMethods, ifaceType) {
					continue
				}
				body.WriteString("\n\n")
				body.WriteString("impl ")
				body.WriteString(ifaceName)
				body.WriteString(" for ")
				body.WriteString(rustTypeName)
				body.WriteString(" {\n")
				previousTraitTypeMethods := currentTypeMethods
				currentTypeMethods = importedTraitMethods
				for _, ifaceMethod := range explicitInterfaceMethods(ifaceType) {
					if method := methodDeclByName(importedTraitMethods, ifaceMethod.Name()); method != nil {
						TranspileTraitMethodImpl(&body, method, interfaceTraitMethodRequiresMutableReceiver(ifaceName, ifaceMethod.Name(), ifaceMethod), fileSet, file.Comments)
					}
				}
				currentTypeMethods = previousTraitTypeMethods
				writeLocalInterfaceSupportImpl(&body, ifaceName, typeName, ifaceType)
				body.WriteString("}")
			}

			var importedPointerIfaceNames []string
			for ifaceName := range importedPointerInterfaceImpls[typeName] {
				importedPointerIfaceNames = append(importedPointerIfaceNames, ifaceName)
			}
			slices.Sort(importedPointerIfaceNames)
			for _, ifaceName := range importedPointerIfaceNames {
				ifaceType := importedPointerInterfaceImpls[typeName][ifaceName]
				if !currentPackagePointerImplementsInterface(typeName, ifaceType) || !typeMethodsImplementTypesInterface(importedTraitMethods, ifaceType) {
					continue
				}
				body.WriteString("\n\n")
				writePointerLocalInterfaceWrapper(&body, typeName, ifaceName, ifaceType, !pointerWrapperEmitted)
				pointerWrapperEmitted = true
			}
		}
		currentTypeMethods = previousTypeMethods
	}

	writeExternalLocalInterfaceImpls(&body, &first, externalLocalInterfaceImpls)

	// Output regular functions
	for _, fn := range functions {
		if isPrunedSourceDecl(fn.Name) {
			continue
		}
		if !first {
			body.WriteString("\n\n")
		}
		first = false
		// Output doc comments if present
		outputComment(&body, fn.Doc, "", true)
		TranspileFunction(&body, fn, fileSet, file.Comments)
	}

	writeNilPointerReceiverMethodHelpers(&body, &first, nilPointerReceiverMethodHelpers, methods, fileSet, file.Comments)

	writeAnonymousStructDefinitions(&body, &first, emittedAnonymousStructs)
	writeAnonymousInterfaceAssertionTraits(&body, &first)

	if hasInitFunction {
		if !first {
			body.WriteString("\n\n")
		}
		TranspilePackageInitAll(&body, hasGlobals, functionNames)
	}

	writeStructGoValueCloneTraitImpls(&body, &first, types, prunedTypeNames)

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
	reserveSiblingRustTypeNames(imports)
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

func writeStructGoValueCloneTraitImpls(out *strings.Builder, first *bool, types []struct {
	spec *ast.TypeSpec
	decl *ast.GenDecl
}, prunedTypeNames map[string]bool) {
	structs := make([]struct {
		name       string
		structType *ast.StructType
		typeSpec   *ast.TypeSpec
	}, 0)
	for _, t := range types {
		if t.spec == nil || t.spec.Name == nil || t.spec.Assign != 0 || prunedTypeNames[t.spec.Name.Name] {
			continue
		}
		structType, ok := t.spec.Type.(*ast.StructType)
		if !ok {
			structType = definedTypeUnderlyingStructAST(t.spec.Type)
			if structType == nil {
				continue
			}
		}
		structs = append(structs, struct {
			name       string
			structType *ast.StructType
			typeSpec   *ast.TypeSpec
		}{name: t.spec.Name.Name, structType: structType, typeSpec: t.spec})
	}
	if len(structs) == 0 {
		return
	}
	if !structGoValueCloneTraitImplsNeeded() {
		return
	}
	for _, t := range structs {
		if !*first {
			out.WriteString("\n\n")
		}
		*first = false
		generics := rustTypeGenericsForStructTypeSpec(t.typeSpec, t.structType)
		generateStructGoValueClone(out, t.name, generics)
	}
}

func structGoValueCloneTraitImplsNeeded() bool {
	if helpers := activeHelperTracker(); helpers != nil && helpers.needsGoValueClone {
		return true
	}
	if ctx := GetTranspileContext(); ctx != nil && ctx.UsePackageExternalStubs {
		NeedGoValueClone()
		return true
	}
	return false
}

func reserveFileRustTypeNames(imports *ImportTracker, types []struct {
	spec *ast.TypeSpec
	decl *ast.GenDecl
}) {
	if imports == nil {
		return
	}
	for _, t := range types {
		if t.spec == nil || t.spec.Name == nil {
			continue
		}
		if isPrunedSourceDecl(t.spec.Name) {
			continue
		}
		imports.ReserveName(RustTypeNameForUse(t.spec.Name.Name))
	}
}

func reserveSiblingRustTypeNames(imports *ImportTracker) {
	if imports == nil {
		return
	}
	ctx := GetTranspileContext()
	if ctx == nil || ctx.Package == nil || ctx.CurrentModuleName == "" {
		return
	}
	for typeName, moduleName := range ctx.Package.TypeModuleNames {
		if moduleName == "" || moduleName == ctx.CurrentModuleName {
			continue
		}
		imports.ReserveName(RustTypeNameForUse(typeName))
	}
}

func currentPackageMethodsForPromotion() map[string][]*ast.FuncDecl {
	if currentContext != nil && currentContext.Package != nil && len(currentContext.Package.MethodsByType) > 0 {
		return currentContext.Package.MethodsByType
	}
	return nil
}

func writeAnonymousStructPromotedMethods(out *strings.Builder, typeName string) {
	structDef, exists := structDefs[typeName]
	if !exists || len(structDef.EmbeddedTypes) == 0 {
		return
	}

	var methods strings.Builder
	methodCount := 0
	writePromotedMethodForwarders(&methods, typeName, structDef, currentPackageMethodsForPromotion(), nil, nil, &methodCount)
	if methodCount == 0 {
		return
	}

	out.WriteString("\n")
	out.WriteString("impl ")
	out.WriteString(typeName)
	out.WriteString(" {\n")
	out.WriteString(methods.String())
	out.WriteString("}\n")
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
		writeStructDerive(body, "", structType, true)
		body.WriteString("pub struct ")
		body.WriteString(typeName)
		body.WriteString(" {\n")

		for fieldIndex, field := range structType.Fields.List {
			if len(field.Names) > 0 {
				// Handle multiple names on one line (e.g., X, Y int)
				for nameIndex, name := range field.Names {
					body.WriteString("    pub ")
					body.WriteString(rustStructFieldName(name, fieldIndex, nameIndex))
					body.WriteString(": ")
					if fieldInfo, ok := sliceElemPtrFieldInfoForAnonymousStructField(structType, name.Name); ok {
						NeedSliceElemPtr()
						body.WriteString("GoPtr<")
						body.WriteString(sliceElemPtrFieldElemRustType(fieldInfo))
						body.WriteString(">")
					} else {
						body.WriteString(GoTypeToRust(field.Type))
					}
					body.WriteString(",\n")
				}
			} else {
				name := ast.NewIdent(getEmbeddedFieldName(field.Type))
				body.WriteString("    pub ")
				body.WriteString(rustStructFieldName(name, fieldIndex, 0))
				body.WriteString(": ")
				body.WriteString(GoTypeToRust(field.Type))
				body.WriteString(",\n")
			}
		}

		body.WriteString("}\n")
		generateStructValueClone(body, typeName, structType, rustTypeGenerics{})
		writeAnonymousStructPromotedMethods(body, typeName)
		body.WriteString("\n")
		generateStructDefault(body, nil, typeName, structType, rustTypeGenerics{})
		body.WriteString("\n")
		generateStructDisplay(body, typeName, structType, rustTypeGenerics{})
		generateStructJsonDecode(body, typeName, structType, rustTypeGenerics{})
		writeAnonymousStructEmbeddedInterfaceImpls(body, typeName, structType)
		emitted[typeName] = true
	}

	var aliasNames []string
	for aliasName, targetName := range anonymousStructAliases {
		if emitted["alias:"+aliasName] || !emitted[targetName] {
			continue
		}
		aliasNames = append(aliasNames, aliasName)
	}
	slices.Sort(aliasNames)
	for _, aliasName := range aliasNames {
		if !*first {
			body.WriteString("\n\n")
		}
		*first = false
		if ast.IsExported(aliasName) {
			body.WriteString("pub ")
		} else {
			body.WriteString("pub(crate) ")
		}
		body.WriteString("type ")
		body.WriteString(aliasName)
		body.WriteString(" = ")
		body.WriteString(anonymousStructAliases[aliasName])
		body.WriteString(";\n")
		emitted["alias:"+aliasName] = true
	}
}
