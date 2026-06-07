package main

import (
	"errors"
	"fmt"
	"go/ast"
	"go/importer"
	"go/token"
	"go/types"
)

// TypeInfo holds type checking results
type TypeInfo struct {
	info                      *types.Info
	pkg                       *types.Package
	methodMutableReceiverMap  map[string]bool
	methodOriginalReceiverMap map[string]bool
}

// NewTypeInfo creates type information for the given files using the default
// (stdlib-only) importer. Prefer NewTypeInfoWithImporter for any package whose
// files may import non-stdlib packages; the default importer cannot resolve
// local modules and would silently leave the resulting types.Info with holes.
func NewTypeInfo(files []*ast.File, fset *token.FileSet) (*TypeInfo, error) {
	return NewTypeInfoWithImporter("", files, fset, importer.Default())
}

// NewTypeInfoWithImporter type-checks files using the supplied importer and
// returns the resulting TypeInfo together with any type-check errors. When
// types.Config.Check produces errors the returned error is always non-nil and
// joins every reported error so callers MUST decide explicitly whether to
// continue with partial info or fail. The function never silently drops a
// check error onto stderr — see AGENTS.md "Type Info Is Authoritative".
//
// Result shape:
//   - (typeInfo, nil)       : full type information, no errors.
//   - (typeInfo, err != nil): partial type information; *types.Package is
//     populated but at least one type-check error was reported. Caller chooses.
//   - (nil,      err != nil): type checking produced no *types.Package at all.
func NewTypeInfoWithImporter(path string, files []*ast.File, fset *token.FileSet, imp types.Importer) (*TypeInfo, error) {
	if imp == nil {
		imp = importer.Default()
	}
	info := &types.Info{
		Types:      make(map[ast.Expr]types.TypeAndValue),
		Defs:       make(map[*ast.Ident]types.Object),
		Uses:       make(map[*ast.Ident]types.Object),
		Implicits:  make(map[ast.Node]types.Object),
		Selections: make(map[*ast.SelectorExpr]*types.Selection),
		Instances:  make(map[*ast.Ident]types.Instance),
		InitOrder:  []*types.Initializer{},
	}

	label := path
	if label == "" {
		label = "<unknown>"
	}

	var checkErrors []error
	config := &types.Config{
		Importer: imp,
		Error: func(err error) {
			checkErrors = append(checkErrors, err)
		},
	}

	pkg, checkErr := config.Check(path, fset, files, info)
	// config.Check's return value usually duplicates the last reported error.
	// Append only when it carries new information so errors.Join stays clean.
	if checkErr != nil && (len(checkErrors) == 0 || checkErrors[len(checkErrors)-1].Error() != checkErr.Error()) {
		checkErrors = append(checkErrors, checkErr)
	}

	if pkg == nil {
		if len(checkErrors) == 0 {
			return nil, fmt.Errorf("type check produced no package for %s", label)
		}
		return nil, fmt.Errorf("type check produced no package for %s: %w", label, errors.Join(checkErrors...))
	}

	typeInfo := &TypeInfo{
		info: info,
		pkg:  pkg,
	}
	typeInfo.methodMutableReceiverMap = collectMethodReceiverMutability(files, typeInfo)
	typeInfo.methodOriginalReceiverMap = collectMethodReceiverOriginalReceiver(files, typeInfo)

	if len(checkErrors) > 0 {
		return typeInfo, fmt.Errorf("type check for %s produced %d error(s): %w", label, len(checkErrors), errors.Join(checkErrors...))
	}

	return typeInfo, nil
}

// GetType returns the type of an expression, or nil if unknown
func (ti *TypeInfo) GetType(expr ast.Expr) types.Type {
	if ti == nil || ti.info == nil || expr == nil {
		return nil
	}
	if tv, ok := ti.info.Types[expr]; ok {
		return tv.Type
	}
	// Also check Uses for identifiers
	if ident, ok := expr.(*ast.Ident); ok {
		if obj, ok := ti.info.Uses[ident]; ok {
			return obj.Type()
		}
	}
	return nil
}

func coreType(typ types.Type) types.Type {
	if typ == nil {
		return nil
	}
	typ = types.Unalias(typ)
	if typeParam, ok := typ.(*types.TypeParam); ok {
		return coreType(typeParam.Constraint())
	}
	if iface, ok := types.Unalias(typ).Underlying().(*types.Interface); ok {
		if iface.NumEmbeddeds() == 1 && iface.NumExplicitMethods() == 0 {
			embedded := types.Unalias(iface.EmbeddedType(0))
			if union, ok := embedded.(*types.Union); ok {
				if union.Len() == 1 {
					return coreType(union.Term(0).Type())
				}
			}
			return coreType(embedded)
		}
	}
	return typ
}

func coreUnderlyingType(typ types.Type) types.Type {
	core := coreType(typ)
	if core == nil {
		return nil
	}
	return core.Underlying()
}

// IsPointer returns true if the expression has a pointer type
func (ti *TypeInfo) IsPointer(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := typ.Underlying().(*types.Pointer)
	return ok
}

// rhsIsPointerType checks if a RHS expression in an assignment has pointer type
func rhsIsPointerType(expr ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	return typeInfo.IsPointer(expr)
}

// HasPointerReceiver returns true if the method being called via a selector
// expression has a pointer receiver (i.e., func (t *T) method())
func (ti *TypeInfo) HasPointerReceiver(sel *ast.SelectorExpr) bool {
	if ti == nil || ti.info == nil {
		return false
	}
	selection, ok := ti.info.Selections[sel]
	if !ok {
		return false
	}
	// Get the method's signature
	fn, ok := selection.Obj().(*types.Func)
	if !ok {
		return false
	}
	sig, ok := fn.Type().(*types.Signature)
	if !ok {
		return false
	}
	recv := sig.Recv()
	if recv == nil {
		return false
	}
	_, isPtr := recv.Type().(*types.Pointer)
	return isPtr
}

// SelectorRequiresMutableReceiver reports whether the selected method was
// generated with a mutable Rust receiver.
func (ti *TypeInfo) SelectorRequiresMutableReceiver(sel *ast.SelectorExpr) (bool, bool) {
	if ti == nil || ti.info == nil || sel == nil {
		return false, false
	}
	selection, ok := ti.info.Selections[sel]
	if !ok {
		return false, false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok {
		return false, false
	}
	key := methodOverrideKey(fn)
	if key == "" {
		return false, false
	}
	mutable, ok := ti.methodMutableReceiverMap[key]
	return mutable, ok
}

// SelectorRequiresOriginalReceiver reports whether the selected method must be
// called through the original pointer handle, even when it does not mutate.
func (ti *TypeInfo) SelectorRequiresOriginalReceiver(sel *ast.SelectorExpr) (bool, bool) {
	if ti == nil || ti.info == nil || sel == nil {
		return false, false
	}
	selection, ok := ti.info.Selections[sel]
	if !ok {
		return false, false
	}
	fn, ok := selection.Obj().(*types.Func)
	if !ok {
		return false, false
	}
	key := methodOverrideKey(fn)
	if key == "" {
		return false, false
	}
	original, ok := ti.methodOriginalReceiverMap[key]
	return original, ok
}

// IsMap returns true if the expression is a map type
func (ti *TypeInfo) IsMap(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := coreUnderlyingType(typ).(*types.Map)
	return ok
}

// IsSlice returns true if the expression is a slice type
func (ti *TypeInfo) IsSlice(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := coreUnderlyingType(typ).(*types.Slice)
	return ok
}

// IsString returns true if the expression is a string type
func (ti *TypeInfo) IsString(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}
	basic, ok := coreUnderlyingType(typ).(*types.Basic)
	return ok && (basic.Kind() == types.String || basic.Kind() == types.UntypedString)
}

// IsChannel returns true if the expression is a channel type
func (ti *TypeInfo) IsChannel(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := typ.Underlying().(*types.Chan)
	return ok
}

// GetChannelElemType returns the element type of a channel expression
func (ti *TypeInfo) GetChannelElemType(expr ast.Expr) types.Type {
	typ := ti.GetType(expr)
	if typ == nil {
		return nil
	}
	if ch, ok := typ.Underlying().(*types.Chan); ok {
		return ch.Elem()
	}
	return nil
}

// IsArray returns true if the expression is an array type
func (ti *TypeInfo) IsArray(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := coreUnderlyingType(typ).(*types.Array)
	return ok
}

// IsPointerToArray returns true if the expression is a pointer to an array.
func (ti *TypeInfo) IsPointerToArray(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}
	ptr, ok := typ.Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	_, ok = ptr.Elem().Underlying().(*types.Array)
	return ok
}

// TypeHasInherentWrapper reports whether a Go type is stored as a Rust wrapper
// handle (Rc<RefCell<Option<T>>>/Arc<Mutex<Option<T>>> or equivalent) by its
// nature, independent of variable/parameter wrapping. Pointers, slices, maps,
// channels, function values, and interfaces all carry their own wrapper.
func TypeHasInherentWrapper(typ types.Type) bool {
	if typ == nil {
		return false
	}
	switch types.Unalias(typ).Underlying().(type) {
	case *types.Pointer, *types.Slice, *types.Map, *types.Chan, *types.Signature, *types.Interface:
		return true
	}
	return false
}

// GetStructType returns the underlying struct type for an expression, or nil.
func (ti *TypeInfo) GetStructType(expr ast.Expr) *types.Struct {
	typ := ti.GetType(expr)
	if typ == nil {
		return nil
	}
	if st, ok := typ.Underlying().(*types.Struct); ok {
		return st
	}
	return nil
}

// IsByteSliceOrArray returns true if the expression is []byte, []uint8, or a byte array.
func (ti *TypeInfo) IsByteSliceOrArray(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}

	switch t := typ.Underlying().(type) {
	case *types.Slice:
		return isByteType(t.Elem())
	case *types.Array:
		return isByteType(t.Elem())
	default:
		return false
	}
}

func isByteType(typ types.Type) bool {
	basic, ok := typ.Underlying().(*types.Basic)
	return ok && basic.Kind() == types.Uint8
}

// IsFunction returns true if the identifier refers to a function (not a variable holding a function)
func (ti *TypeInfo) IsFunction(ident *ast.Ident) bool {
	if ti == nil || ti.info == nil {
		return false
	}

	// Check if this identifier is defined as a function
	if obj, ok := ti.info.Defs[ident]; ok {
		_, isFunc := obj.(*types.Func)
		return isFunc
	}

	// Check if this identifier uses a function
	if obj, ok := ti.info.Uses[ident]; ok {
		_, isFunc := obj.(*types.Func)
		return isFunc
	}

	return false
}

// IsFunctionType returns true if the expression has a function type
func (ti *TypeInfo) IsFunctionType(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := typ.Underlying().(*types.Signature)
	return ok
}

// GetObject returns the types.Object for an identifier
func (ti *TypeInfo) GetObject(ident *ast.Ident) types.Object {
	if ti == nil || ti.info == nil {
		return nil
	}

	// Check Uses first (references to objects)
	if obj, ok := ti.info.Uses[ident]; ok {
		return obj
	}

	// Check Defs (definitions of objects)
	if obj, ok := ti.info.Defs[ident]; ok {
		return obj
	}

	return nil
}

// GetMapTypes returns the key and value types of a map, or nil if not a map
func (ti *TypeInfo) GetMapTypes(expr ast.Expr) (key, value types.Type) {
	typ := ti.GetType(expr)
	if typ == nil {
		return nil, nil
	}
	if mapType, ok := coreUnderlyingType(typ).(*types.Map); ok {
		return coreType(mapType.Key()), coreType(mapType.Elem())
	}
	return nil, nil
}

// GetSliceElemType returns the element type of a slice, or nil if not a slice
func (ti *TypeInfo) GetSliceElemType(expr ast.Expr) types.Type {
	typ := ti.GetType(expr)
	if typ == nil {
		return nil
	}
	if sliceType, ok := coreUnderlyingType(typ).(*types.Slice); ok {
		return coreType(sliceType.Elem())
	}
	return nil
}

// GetArrayOrSliceElemType returns the element type of an array or slice.
func (ti *TypeInfo) GetArrayOrSliceElemType(expr ast.Expr) types.Type {
	typ := ti.GetType(expr)
	if typ == nil {
		return nil
	}
	switch t := coreUnderlyingType(typ).(type) {
	case *types.Slice:
		return coreType(t.Elem())
	case *types.Array:
		return coreType(t.Elem())
	case *types.Pointer:
		if array, ok := t.Elem().Underlying().(*types.Array); ok {
			return coreType(array.Elem())
		}
		return nil
	default:
		return nil
	}
}

// GetArrayOrSliceElemTypePreservingTypeParam returns the element type without
// reducing direct []T elements to their constraint core type.
func (ti *TypeInfo) GetArrayOrSliceElemTypePreservingTypeParam(expr ast.Expr) types.Type {
	typ := ti.GetType(expr)
	if typ == nil {
		return nil
	}
	if elem, ok := goTypeParamSliceConstraintElem(typ); ok {
		return types.Unalias(elem)
	}
	switch t := types.Unalias(typ).Underlying().(type) {
	case *types.Slice:
		return types.Unalias(t.Elem())
	case *types.Array:
		return types.Unalias(t.Elem())
	case *types.Pointer:
		if array, ok := types.Unalias(t.Elem()).Underlying().(*types.Array); ok {
			return types.Unalias(array.Elem())
		}
	}
	return ti.GetArrayOrSliceElemType(expr)
}

// GetMapValueType returns the value type of a map, or nil if not a map
func (ti *TypeInfo) GetMapValueType(expr ast.Expr) types.Type {
	typ := ti.GetType(expr)
	if typ == nil {
		return nil
	}
	if mapType, ok := coreUnderlyingType(typ).(*types.Map); ok {
		return coreType(mapType.Elem())
	}
	return nil
}

// GetBasicKind returns the BasicKind if the type is a basic type
func (ti *TypeInfo) GetBasicKind(expr ast.Expr) types.BasicKind {
	typ := ti.GetType(expr)
	if typ == nil {
		return types.Invalid
	}
	if basic, ok := typ.Underlying().(*types.Basic); ok {
		return basic.Kind()
	}
	return types.Invalid
}

// IsInterface checks if an identifier refers to an interface type
func (ti *TypeInfo) IsInterface(ident *ast.Ident) bool {
	if ti.info == nil {
		return false
	}

	// Look up the identifier in the type info
	if obj, ok := ti.info.Uses[ident]; ok {
		if typeName, ok := obj.(*types.TypeName); ok {
			if typeNameIsInterfaceValue(typeName) {
				return true
			}
		}
	}

	// Also check in Defs for type definitions
	if obj, ok := ti.info.Defs[ident]; ok {
		if typeName, ok := obj.(*types.TypeName); ok {
			if typeNameIsInterfaceValue(typeName) {
				return true
			}
		}
	}

	return false
}

func typeNameIsInterfaceValue(typeName *types.TypeName) bool {
	if typeName == nil || typeName.Type() == nil {
		return false
	}
	if _, ok := types.Unalias(typeName.Type()).(*types.TypeParam); ok {
		return false
	}
	_, ok := types.Unalias(typeName.Type()).Underlying().(*types.Interface)
	return ok
}

// ReturnsWrappedValue checks if an expression returns a wrapped Arc<Mutex<Option<T>>> value
// This is true for function calls, method calls, and field accesses in our conservative model
func (ti *TypeInfo) ReturnsWrappedValue(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.CallExpr:
		if isBareBuiltinCall(e) {
			return false
		}
		if ti.IsTypeConversion(e) && !typeConversionEmitsWrappedValue(e) {
			return false
		}
		if callReturnsBareScalar(e) {
			return false
		}
		// Both function calls and type conversions return wrapped values
		// (TranspileTypeConversion wraps its output with WriteWrapperPrefix/Suffix)
		return true
	case *ast.SelectorExpr:
		if ti != nil && ti.info != nil {
			if selection := ti.info.Selections[e]; selection != nil {
				return true
			}
			switch obj := ti.GetObject(e.Sel).(type) {
			case *types.Const, *types.Func, *types.TypeName:
				return false
			case *types.Var:
				if obj.Pkg() != nil && obj.Parent() == obj.Pkg().Scope() {
					return true
				}
			}
		}
		// Field accesses return wrapped values in our conservative model.
		// Method calls are handled by CallExpr case.
		if ti.IsFunction(e.Sel) {
			return false
		}
		return true // Field access returns wrapped value
	case *ast.IndexExpr:
		// Array/slice indexing returns the element directly (not wrapped)
		return false
	case *ast.SliceExpr:
		// Plain slice expressions return a new wrapped slice. Named slice
		// expressions lower as their named Rust value, which owns the inner
		// slice handle.
		_, _, isNamedSlice := namedSliceTypeFromType(ti.GetType(e))
		return !isNamedSlice
	case *ast.Ident:
		// Variables are already wrapped, but accessing them doesn't add another layer
		// However, in return statements, we need to clone wrapped variables to avoid move errors
		// For now, we'll use a heuristic: if it's not a special identifier, assume it's wrapped
		if e.Name == "true" || e.Name == "false" || e.Name == "nil" {
			return false
		}
		// Check if this is a local variable by looking at the context
		// This is a simplified approach - we could improve this with better type analysis
		return false // Revert to original behavior for now
	case *ast.BasicLit:
		// Literals are not wrapped
		return false
	case *ast.BinaryExpr:
		// Binary expressions return raw values that we wrap
		return false
	case *ast.UnaryExpr:
		// Unary expressions depend on the operator
		if e.Op == token.AND {
			// Address-of returns a wrapped pointer
			return true
		}
		// Other unary ops return raw values
		return false
	default:
		// Conservative: assume it doesn't return wrapped
		return false
	}
}

// NeedsUnwrapping checks if an expression needs unwrapping for use in a binary expression
// This is true for expressions that return wrapped values, but NOT for identifiers
// because identifiers already unwrap themselves in RValue context
func (ti *TypeInfo) NeedsUnwrapping(expr ast.Expr) bool {
	switch e := expr.(type) {
	case *ast.Ident:
		// Identifiers handle their own unwrapping in RValue context
		// so we should NOT unwrap them again in binary expressions
		return false
	case *ast.CallExpr:
		if isBareBuiltinCall(e) {
			return false
		}
		// Function calls that return wrapped values need unwrapping
		return ti.ReturnsWrappedValue(expr)
	case *ast.SelectorExpr:
		// Field accesses on regular variables are already fully unwrapped
		// by the SelectorExpr handler in RValue context.
		// Only self/receiver field accesses need extra unwrapping (they just clone).
		if ident, ok := e.X.(*ast.Ident); ok {
			if isCurrentReceiverIdent(ident) {
				return true // self.field.clone() is still wrapped
			}
			// Check if it's a package selector (not a field access)
			if ti.info != nil {
				if obj, ok := ti.info.Uses[ident]; ok {
					if _, ok := obj.(*types.PkgName); ok {
						return false
					}
				}
			}
			// Regular variable field access - already unwrapped by SelectorExpr handler
			return false
		}
		if exprType := ti.GetType(e); exprType != nil {
			if _, ok := exprType.Underlying().(*types.Basic); ok {
				return false
			}
		}
		return ti.ReturnsWrappedValue(expr)
	case *ast.IndexExpr:
		// Array/slice indexing: the IndexExpr handler already unwraps and adds .clone()
		// For basic element types (int, float, string, bool, byte), the result is a raw value
		exprType := ti.GetType(e)
		if exprType != nil {
			if _, ok := exprType.Underlying().(*types.Basic); ok {
				return false // Basic element type - already a raw value
			}
		}
		// For non-basic element types (pointers, etc.), the element may be wrapped
		return true
	case *ast.SliceExpr:
		_, _, isNamedSlice := namedSliceTypeFromType(ti.GetType(e))
		return !isNamedSlice
	case *ast.BasicLit:
		// Literals don't need unwrapping
		return false
	case *ast.BinaryExpr:
		// Binary expressions are computed inline, don't need unwrapping
		return false
	case *ast.CompositeLit:
		return isCompositeLitSelfWrapping(e)
	case *ast.UnaryExpr:
		// Unary expressions are computed inline, don't need unwrapping
		return false
	case *ast.StarExpr:
		// Dereference expressions emit the pointed-to value directly.
		return false
	case *ast.ParenExpr:
		// Check the inner expression
		return ti.NeedsUnwrapping(e.X)
	default:
		// Conservative: assume it needs unwrapping if we're not sure
		return true
	}
}

// IsTypeConversion checks if a CallExpr is actually a type conversion
func (ti *TypeInfo) IsTypeConversion(call *ast.CallExpr) bool {
	// Type conversions have exactly one argument
	if len(call.Args) != 1 {
		return false
	}

	// Check for []byte(x) or []rune(x) conversions
	if _, ok := call.Fun.(*ast.ArrayType); ok {
		return true
	}

	if ti == nil || ti.info == nil {
		// Without type info, we can still check for common type names
		if ident, ok := call.Fun.(*ast.Ident); ok {
			switch ident.Name {
			case "int", "int8", "int16", "int32", "int64",
				"uint", "uint8", "uint16", "uint32", "uint64",
				"float32", "float64", "string", "byte", "rune",
				"uintptr", "complex64", "complex128":
				return true
			}
			if _, isTypeDef := LookupTypeDefinition(ident.Name); isTypeDef {
				return true
			}
		}
		return false
	}

	if tv, ok := ti.info.Types[call.Fun]; ok && tv.IsType() {
		return true
	}

	// Check if the function is actually a type
	if ident, ok := call.Fun.(*ast.Ident); ok {
		if obj := ti.GetObject(ident); obj != nil {
			_, isType := obj.(*types.TypeName)
			return isType
		}
		switch ident.Name {
		case "int", "int8", "int16", "int32", "int64",
			"uint", "uint8", "uint16", "uint32", "uint64",
			"float32", "float64", "string", "byte", "rune",
			"uintptr", "complex64", "complex128":
			return true
		}
		if _, isTypeDef := LookupTypeDefinition(ident.Name); isTypeDef {
			return true
		}
	}

	// Also check for selector expressions (e.g., time.Duration)
	if sel, ok := call.Fun.(*ast.SelectorExpr); ok {
		if obj, ok := ti.info.Uses[sel.Sel]; ok {
			_, isType := obj.(*types.TypeName)
			return isType
		}
	}

	return false
}
