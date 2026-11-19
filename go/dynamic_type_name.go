package main

import (
	"go/types"
	"strconv"
	"strings"
)

// goDynamicTypeName renders typ the way reflect.Type.String does, which is
// what fmt prints for %T. It reports false for types whose reflect spelling
// it does not reproduce (generic instantiations, struct and non-empty
// interface literals, anything that depends on a type parameter); callers
// then leave the type unnamed instead of printing something Go would not.
func goDynamicTypeName(typ types.Type) (string, bool) {
	if typ == nil {
		return "", false
	}
	switch t := types.Unalias(typ).(type) {
	case *types.Basic:
		return goDynamicBasicTypeName(t)
	case *types.Named:
		obj := t.Obj()
		if obj == nil {
			return "", false
		}
		if args := t.TypeArgs(); args != nil && args.Len() > 0 {
			return "", false
		}
		if obj.Pkg() == nil {
			return obj.Name(), true
		}
		return obj.Pkg().Name() + "." + obj.Name(), true
	case *types.Pointer:
		elem, ok := goDynamicTypeName(t.Elem())
		if !ok {
			return "", false
		}
		return "*" + elem, true
	case *types.Slice:
		elem, ok := goDynamicTypeName(t.Elem())
		if !ok {
			return "", false
		}
		return "[]" + elem, true
	case *types.Array:
		elem, ok := goDynamicTypeName(t.Elem())
		if !ok {
			return "", false
		}
		return "[" + strconv.FormatInt(t.Len(), 10) + "]" + elem, true
	case *types.Map:
		key, ok := goDynamicTypeName(t.Key())
		if !ok {
			return "", false
		}
		elem, ok := goDynamicTypeName(t.Elem())
		if !ok {
			return "", false
		}
		return "map[" + key + "]" + elem, true
	case *types.Chan:
		elem, ok := goDynamicTypeName(t.Elem())
		if !ok {
			return "", false
		}
		switch t.Dir() {
		case types.SendOnly:
			return "chan<- " + elem, true
		case types.RecvOnly:
			return "<-chan " + elem, true
		}
		if inner, ok := types.Unalias(t.Elem()).(*types.Chan); ok && inner.Dir() == types.RecvOnly {
			return "chan (" + elem + ")", true
		}
		return "chan " + elem, true
	case *types.Signature:
		return goDynamicSignatureTypeName(t)
	case *types.Interface:
		if t.NumMethods() == 0 && t.NumEmbeddeds() == 0 {
			return "interface {}", true
		}
		return "", false
	}
	return "", false
}

func goDynamicBasicTypeName(basic *types.Basic) (string, bool) {
	switch basic.Kind() {
	case types.Bool:
		return "bool", true
	case types.Int:
		return "int", true
	case types.Int8:
		return "int8", true
	case types.Int16:
		return "int16", true
	case types.Int32:
		return "int32", true
	case types.Int64:
		return "int64", true
	case types.Uint:
		return "uint", true
	case types.Uint8:
		return "uint8", true
	case types.Uint16:
		return "uint16", true
	case types.Uint32:
		return "uint32", true
	case types.Uint64:
		return "uint64", true
	case types.Uintptr:
		return "uintptr", true
	case types.Float32:
		return "float32", true
	case types.Float64:
		return "float64", true
	case types.Complex64:
		return "complex64", true
	case types.Complex128:
		return "complex128", true
	case types.String:
		return "string", true
	case types.UnsafePointer:
		return "unsafe.Pointer", true
	}
	return "", false
}

func goDynamicSignatureTypeName(sig *types.Signature) (string, bool) {
	if sig.TypeParams() != nil && sig.TypeParams().Len() > 0 {
		return "", false
	}
	var name strings.Builder
	name.WriteString("func(")
	params := sig.Params()
	for i := 0; i < params.Len(); i++ {
		if i > 0 {
			name.WriteString(", ")
		}
		paramType := params.At(i).Type()
		if sig.Variadic() && i == params.Len()-1 {
			slice, ok := types.Unalias(paramType).(*types.Slice)
			if !ok {
				return "", false
			}
			name.WriteString("...")
			paramType = slice.Elem()
		}
		param, ok := goDynamicTypeName(paramType)
		if !ok {
			return "", false
		}
		name.WriteString(param)
	}
	name.WriteString(")")
	results := sig.Results()
	if results.Len() == 0 {
		return name.String(), true
	}
	name.WriteString(" ")
	if results.Len() > 1 {
		name.WriteString("(")
	}
	for i := 0; i < results.Len(); i++ {
		if i > 0 {
			name.WriteString(", ")
		}
		result, ok := goDynamicTypeName(results.At(i).Type())
		if !ok {
			return "", false
		}
		name.WriteString(result)
	}
	if results.Len() > 1 {
		name.WriteString(")")
	}
	return name.String(), true
}

// typeMentionsTypeParam reports whether typ depends on a type parameter of an
// enclosing generic declaration. Such a type's Rust spelling only means
// something inside that declaration, so it cannot go into a shared helper.
func typeMentionsTypeParam(typ types.Type) bool {
	if typ == nil {
		return false
	}
	switch t := types.Unalias(typ).(type) {
	case *types.TypeParam:
		return true
	case *types.Named:
		if args := t.TypeArgs(); args != nil {
			for i := 0; i < args.Len(); i++ {
				if typeMentionsTypeParam(args.At(i)) {
					return true
				}
			}
		}
		return false
	case *types.Pointer:
		return typeMentionsTypeParam(t.Elem())
	case *types.Slice:
		return typeMentionsTypeParam(t.Elem())
	case *types.Array:
		return typeMentionsTypeParam(t.Elem())
	case *types.Chan:
		return typeMentionsTypeParam(t.Elem())
	case *types.Map:
		return typeMentionsTypeParam(t.Key()) || typeMentionsTypeParam(t.Elem())
	case *types.Signature:
		for _, tuple := range []*types.Tuple{t.Params(), t.Results()} {
			for i := 0; i < tuple.Len(); i++ {
				if typeMentionsTypeParam(tuple.At(i).Type()) {
					return true
				}
			}
		}
		return false
	}
	return false
}
