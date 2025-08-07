package main

import (
	"go/ast"
	"go/importer"
	"go/token"
	"go/types"
)

// TypeInfo holds type checking results
type TypeInfo struct {
	info *types.Info
	pkg  *types.Package
}

// NewTypeInfo creates type information for the given files
func NewTypeInfo(files []*ast.File, fset *token.FileSet) (*TypeInfo, error) {
	info := &types.Info{
		Types:      make(map[ast.Expr]types.TypeAndValue),
		Defs:       make(map[*ast.Ident]types.Object),
		Uses:       make(map[*ast.Ident]types.Object),
		Selections: make(map[*ast.SelectorExpr]*types.Selection),
	}

	config := &types.Config{
		Importer: importer.Default(),
		Error: func(err error) {
			// Log but don't fail on import errors
			// This allows partial type checking even with missing imports
		},
	}

	pkg, _ := config.Check("", fset, files, info)

	return &TypeInfo{
		info: info,
		pkg:  pkg,
	}, nil
}

// GetType returns the type of an expression, or nil if unknown
func (ti *TypeInfo) GetType(expr ast.Expr) types.Type {
	if ti == nil || ti.info == nil {
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

// IsMap returns true if the expression is a map type
func (ti *TypeInfo) IsMap(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := typ.Underlying().(*types.Map)
	return ok
}

// IsSlice returns true if the expression is a slice type
func (ti *TypeInfo) IsSlice(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := typ.Underlying().(*types.Slice)
	return ok
}

// IsString returns true if the expression is a string type
func (ti *TypeInfo) IsString(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}
	basic, ok := typ.Underlying().(*types.Basic)
	return ok && basic.Kind() == types.String
}

// IsArray returns true if the expression is an array type
func (ti *TypeInfo) IsArray(expr ast.Expr) bool {
	typ := ti.GetType(expr)
	if typ == nil {
		return false
	}
	_, ok := typ.Underlying().(*types.Array)
	return ok
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
	if mapType, ok := typ.Underlying().(*types.Map); ok {
		return mapType.Key(), mapType.Elem()
	}
	return nil, nil
}

// GetSliceElemType returns the element type of a slice, or nil if not a slice
func (ti *TypeInfo) GetSliceElemType(expr ast.Expr) types.Type {
	typ := ti.GetType(expr)
	if typ == nil {
		return nil
	}
	if sliceType, ok := typ.Underlying().(*types.Slice); ok {
		return sliceType.Elem()
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
		}
		return false
	}

	// Check if the function is actually a type
	if ident, ok := call.Fun.(*ast.Ident); ok {
		if obj := ti.GetObject(ident); obj != nil {
			_, isType := obj.(*types.TypeName)
			return isType
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
