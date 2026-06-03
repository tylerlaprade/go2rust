package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strconv"
	"strings"
)

type sliceElemPtrCandidate struct {
	elemRustType string
	valid        bool
	sawSliceAddr bool
}

type arrayElemPtrInfo struct {
	elemRustType string
	arrayLen     int64
}

type arrayElemPtrCandidate struct {
	info         arrayElemPtrInfo
	valid        bool
	sawArrayAddr bool
}

var currentSliceElemPtrCandidates map[types.Object]string
var currentSliceElemPtrSliceCandidates map[types.Object]string
var currentArrayElemPtrCandidates map[types.Object]arrayElemPtrInfo
var currentSliceElemPtrReturn *sliceElemPtrReturnInfo
var currentSliceElemPtrSliceReturn *sliceElemPtrSliceReturnInfo

type sliceElemPtrReturnInfo struct {
	elemRustType string
	ownerPkgPath string
	elemType     types.Type
}

type sliceElemPtrFieldInfo struct {
	elemRustType string
	ownerPkgPath string
	elemType     types.Type
}

type sliceElemPtrSliceReturnInfo struct {
	resultElemRustTypes map[int]string
}

func setSliceElemPtrCandidates(body *ast.BlockStmt) func() {
	oldPtr := currentSliceElemPtrCandidates
	oldSlice := currentSliceElemPtrSliceCandidates
	oldArray := currentArrayElemPtrCandidates
	currentSliceElemPtrCandidates = collectSliceElemPtrCandidates(body)
	currentSliceElemPtrSliceCandidates = collectSliceElemPtrSliceCandidates(body)
	currentArrayElemPtrCandidates = collectArrayElemPtrCandidates(body)
	return func() {
		currentSliceElemPtrCandidates = oldPtr
		currentSliceElemPtrSliceCandidates = oldSlice
		currentArrayElemPtrCandidates = oldArray
	}
}

func setSliceElemPtrCandidatesForFunc(fn *ast.FuncDecl) func() {
	if fn == nil {
		return setSliceElemPtrCandidates(nil)
	}
	oldPtr := currentSliceElemPtrCandidates
	oldSlice := currentSliceElemPtrSliceCandidates
	oldArray := currentArrayElemPtrCandidates
	currentSliceElemPtrCandidates = collectSliceElemPtrCandidates(fn.Body)
	currentSliceElemPtrSliceCandidates = collectSliceElemPtrSliceCandidatesForFunc(fn)
	currentArrayElemPtrCandidates = collectArrayElemPtrCandidatesForFunc(fn)
	return func() {
		currentSliceElemPtrCandidates = oldPtr
		currentSliceElemPtrSliceCandidates = oldSlice
		currentArrayElemPtrCandidates = oldArray
	}
}

func sliceElemPtrCandidateForDecl(name *ast.Ident) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && currentSliceElemPtrCandidates != nil {
		if obj := typeInfo.GetObject(name); obj != nil {
			if elemRustType, ok := currentSliceElemPtrCandidates[obj]; ok {
				return elemRustType, true
			}
		}
	}
	return "", false
}

func sliceElemPtrSliceCandidateForDecl(name *ast.Ident) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && currentSliceElemPtrSliceCandidates != nil {
		if obj := typeInfo.GetObject(name); obj != nil {
			if elemRustType, ok := currentSliceElemPtrSliceCandidates[obj]; ok {
				return elemRustType, true
			}
		}
	}
	return "", false
}

func sliceElemPtrSliceCandidateForExpr(expr ast.Expr) (string, bool) {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok {
		return "", false
	}
	return sliceElemPtrSliceCandidateForDecl(ident)
}

func arrayElemPtrCandidateForDecl(name *ast.Ident) (arrayElemPtrInfo, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && currentArrayElemPtrCandidates != nil {
		if obj := typeInfo.GetObject(name); obj != nil {
			info, ok := currentArrayElemPtrCandidates[obj]
			return info, ok
		}
	}
	return arrayElemPtrInfo{}, false
}

func arrayElemPtrResultInfosForFunc(fn *ast.FuncDecl) map[int]arrayElemPtrInfo {
	typeInfo := GetTypeInfo()
	if fn == nil || fn.Type == nil || fn.Type.Results == nil || typeInfo == nil {
		return nil
	}
	candidates := currentArrayElemPtrCandidates
	if candidates == nil {
		candidates = collectArrayElemPtrCandidatesForFunc(fn)
	}
	if candidates == nil {
		return nil
	}
	result := map[int]arrayElemPtrInfo{}
	resultIndex := 0
	for _, field := range fn.Type.Results.List {
		if len(field.Names) == 0 {
			resultIndex++
			continue
		}
		for _, name := range field.Names {
			if name.Name != "_" {
				if obj := typeInfo.GetObject(name); obj != nil {
					if info, ok := candidates[obj]; ok {
						result[resultIndex] = info
					}
				}
			}
			resultIndex++
		}
	}
	if len(result) == 0 {
		return nil
	}
	registerArrayElemPtrResultInfosForDecl(fn, result)
	return result
}

func registerArrayElemPtrResultInfosForDecl(fn *ast.FuncDecl, result map[int]arrayElemPtrInfo) {
	if len(result) == 0 {
		return
	}
	fnObj, ok := sliceElemPtrReturnFuncObject(fn)
	if !ok {
		return
	}
	ctx := GetTranspileContext()
	if ctx == nil || ctx.Package == nil {
		return
	}
	if ctx.Package.ArrayElemPtrResultFuncs == nil {
		ctx.Package.ArrayElemPtrResultFuncs = make(map[*types.Func]map[int]arrayElemPtrInfo)
	}
	if ctx.Package.ArrayElemPtrResultFuncNames == nil {
		ctx.Package.ArrayElemPtrResultFuncNames = make(map[string]map[int]arrayElemPtrInfo)
	}
	ctx.Package.ArrayElemPtrResultFuncs[fnObj] = result
	ctx.Package.ArrayElemPtrResultFuncNames[fnObj.FullName()] = result
}

func arrayElemPtrResultInfosForFuncObject(fn *types.Func) (map[int]arrayElemPtrInfo, bool) {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil || ctx.Package == nil {
		return nil, false
	}
	if result, ok := ctx.Package.ArrayElemPtrResultFuncs[fn]; ok {
		return result, true
	}
	result, ok := ctx.Package.ArrayElemPtrResultFuncNames[fn.FullName()]
	return result, ok
}

func arrayElemPtrResultInfoForCall(call *ast.CallExpr, resultIndex int) (arrayElemPtrInfo, bool) {
	typeInfo := GetTypeInfo()
	fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
	if !ok {
		return arrayElemPtrInfo{}, false
	}
	infos, ok := arrayElemPtrResultInfosForFuncObject(fn)
	if !ok {
		return arrayElemPtrInfo{}, false
	}
	info, ok := infos[resultIndex]
	return info, ok
}

func writeArrayElemPtrFuncDeclResultTypes(out *strings.Builder, fn *ast.FuncDecl) bool {
	resultInfos := arrayElemPtrResultInfosForFunc(fn)
	if len(resultInfos) == 0 {
		return false
	}
	sig, ok := funcDeclSignatureFromTypeInfo(fn)
	if !ok || sig.Results() == nil || sig.Results().Len() == 0 {
		return false
	}
	results := sig.Results()
	out.WriteString(" -> ")
	if results.Len() == 1 {
		if info, ok := resultInfos[0]; ok {
			out.WriteString(arrayElemPtrOptionRustType(info))
		} else {
			out.WriteString(goTypesReturnTypeToRust(results.At(0).Type()))
		}
		return true
	}
	out.WriteString("(")
	for i := 0; i < results.Len(); i++ {
		if i > 0 {
			out.WriteString(", ")
		}
		if info, ok := resultInfos[i]; ok {
			out.WriteString(arrayElemPtrOptionRustType(info))
		} else {
			out.WriteString(goTypesReturnTypeToRust(results.At(i).Type()))
		}
	}
	out.WriteString(")")
	return true
}

func registerArrayElemPtrNamedReturnVars(fn *ast.FuncDecl) {
	if fn == nil || fn.Type == nil || fn.Type.Results == nil {
		return
	}
	for _, result := range fn.Type.Results.List {
		for _, name := range result.Names {
			if name.Name == "_" {
				continue
			}
			if info, ok := arrayElemPtrCandidateForDecl(name); ok {
				registerArrayElemPtrVar(name.Name, info)
			}
		}
	}
}

func registerSliceElemPtrReturnsFromFiles(files []*ast.File) {
	for _, file := range files {
		registerSliceElemPtrReturnsFromFile(file)
	}
	registerSliceElemPtrSliceParamsFromFiles(files)
}

func registerSliceElemPtrFieldsFromFiles(files []*ast.File) {
	typeInfo := GetTypeInfo()
	ctx := GetTranspileContext()
	if typeInfo == nil || typeInfo.info == nil || ctx == nil || ctx.Package == nil {
		return
	}
	if ctx.Package.SliceElemPtrFields == nil {
		ctx.Package.SliceElemPtrFields = make(map[string]sliceElemPtrFieldInfo)
	}
	for _, file := range files {
		if file == nil {
			continue
		}
		for _, decl := range file.Decls {
			fn, ok := decl.(*ast.FuncDecl)
			if !ok || fn.Body == nil {
				continue
			}
			localCandidates := collectSliceElemPtrCandidates(fn.Body)
			ast.Inspect(fn.Body, func(node ast.Node) bool {
				switch n := node.(type) {
				case *ast.FuncLit:
					return false
				case *ast.AssignStmt:
					for i, lhs := range n.Lhs {
						sel, ok := unwrapParens(lhs).(*ast.SelectorExpr)
						if !ok {
							continue
						}
						rhs := assignmentRHSForLHS(n, i)
						if rhs == nil {
							continue
						}
						registerSliceElemPtrFieldAssignment(sel, rhs, localCandidates)
					}
				}
				return true
			})
		}
	}
}

func registerSliceElemPtrFieldAssignment(sel *ast.SelectorExpr, rhs ast.Expr, localCandidates map[types.Object]string) {
	key, fieldInfo, ok := sliceElemPtrFieldKeyForSelector(sel)
	if !ok {
		return
	}
	rhsElemType, rhsElemRustType, ok := sliceElemPtrValueElemType(rhs, localCandidates)
	if !ok || !sliceElemPtrElemCompatible(rhsElemType, rhsElemRustType, fieldInfo) {
		return
	}
	ctx := GetTranspileContext()
	if ctx == nil || ctx.Package == nil {
		return
	}
	ctx.Package.SliceElemPtrFields[key] = fieldInfo
	if ctx.Session != nil {
		if ctx.Session.SliceElemPtrFields == nil {
			ctx.Session.SliceElemPtrFields = make(map[string]sliceElemPtrFieldInfo)
		}
		ctx.Session.SliceElemPtrFields[key] = fieldInfo
	}
}

func sliceElemPtrValueElemType(expr ast.Expr, localCandidates map[types.Object]string) (types.Type, string, bool) {
	if elemType, elemRustType, ok := sliceElemPtrAddressElemType(expr); ok {
		return elemType, elemRustType, true
	}
	if call, ok := unwrapParens(expr).(*ast.CallExpr); ok {
		if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
			return info.elemType, info.elemRustType, true
		}
	}
	if ident, ok := unwrapParens(expr).(*ast.Ident); ok {
		if info, ok := sliceElemPtrVarInfo(ident.Name); ok && info.RustType != "" {
			typeInfo := GetTypeInfo()
			var elemType types.Type
			if typeInfo != nil {
				elemType, _ = sliceElemPtrPointerElemType(typeInfo.GetType(ident))
			}
			return elemType, info.RustType, true
		}
		if localCandidates != nil {
			typeInfo := GetTypeInfo()
			if typeInfo == nil {
				return nil, "", false
			}
			obj := typeInfo.GetObject(ident)
			if obj == nil {
				return nil, "", false
			}
			elemRustType, ok := localCandidates[obj]
			elemType, _ := sliceElemPtrPointerElemType(typeInfo.GetType(ident))
			return elemType, elemRustType, ok
		}
	}
	return nil, "", false
}

func sliceElemPtrElemCompatible(rhsElemType types.Type, rhsElemRustType string, fieldInfo sliceElemPtrFieldInfo) bool {
	if rhsElemType != nil && fieldInfo.elemType != nil && types.Identical(coreType(rhsElemType), coreType(fieldInfo.elemType)) {
		return true
	}
	return rhsElemRustType != "" && rhsElemRustType == fieldInfo.elemRustType
}

func sliceElemPtrPointerElemType(t types.Type) (types.Type, bool) {
	if t == nil {
		return nil, false
	}
	ptr, ok := types.Unalias(t).Underlying().(*types.Pointer)
	if !ok {
		return nil, false
	}
	return coreType(ptr.Elem()), true
}

func sliceElemPtrFieldKeyForSelector(sel *ast.SelectorExpr) (string, sliceElemPtrFieldInfo, bool) {
	typeInfo := GetTypeInfo()
	if sel == nil || typeInfo == nil || typeInfo.info == nil {
		return "", sliceElemPtrFieldInfo{}, false
	}
	selection := typeInfo.info.Selections[sel]
	if selection == nil || selection.Kind() != types.FieldVal || len(selection.Index()) == 0 {
		return "", sliceElemPtrFieldInfo{}, false
	}
	field, ok := selection.Obj().(*types.Var)
	if !ok || field == nil {
		return "", sliceElemPtrFieldInfo{}, false
	}
	owner, ok := sliceElemPtrFieldOwnerNamed(selection)
	if !ok {
		return "", sliceElemPtrFieldInfo{}, false
	}
	ptr, ok := types.Unalias(field.Type()).Underlying().(*types.Pointer)
	if !ok {
		return "", sliceElemPtrFieldInfo{}, false
	}
	key := sliceElemPtrFieldKey(owner, field.Name())
	if key == "" {
		return "", sliceElemPtrFieldInfo{}, false
	}
	elemType := coreType(ptr.Elem())
	elemRustType := goTypesCollectionElemTypeToRust(elemType)
	if elemRustType == "" {
		return "", sliceElemPtrFieldInfo{}, false
	}
	ownerPkgPath := ""
	if owner.Obj() != nil && owner.Obj().Pkg() != nil {
		ownerPkgPath = owner.Obj().Pkg().Path()
	}
	return key, sliceElemPtrFieldInfo{elemRustType: elemRustType, ownerPkgPath: ownerPkgPath, elemType: elemType}, true
}

func sliceElemPtrFieldOwnerNamed(selection *types.Selection) (*types.Named, bool) {
	if selection == nil || len(selection.Index()) == 0 {
		return nil, false
	}
	current := selection.Recv()
	var owner *types.Named
	for _, fieldIndex := range selection.Index() {
		if ptr, ok := types.Unalias(current).Underlying().(*types.Pointer); ok {
			current = ptr.Elem()
		}
		if named, ok := types.Unalias(current).(*types.Named); ok {
			owner = named
			current = named.Underlying()
		}
		st, ok := types.Unalias(current).Underlying().(*types.Struct)
		if !ok || fieldIndex < 0 || fieldIndex >= st.NumFields() {
			return nil, false
		}
		current = st.Field(fieldIndex).Type()
	}
	if owner == nil || owner.Obj() == nil {
		return nil, false
	}
	return owner, true
}

func sliceElemPtrFieldKey(named *types.Named, fieldName string) string {
	if named == nil || named.Obj() == nil || fieldName == "" {
		return ""
	}
	pkgPath := ""
	if pkg := named.Obj().Pkg(); pkg != nil {
		pkgPath = pkg.Path()
	}
	return pkgPath + "." + named.Obj().Name() + "." + fieldName
}

func sliceElemPtrFieldKeyForTypeSpecField(typeSpec *ast.TypeSpec, fieldName string) string {
	typeInfo := GetTypeInfo()
	if typeSpec == nil || typeSpec.Name == nil || fieldName == "" || typeInfo == nil || typeInfo.info == nil {
		return ""
	}
	obj, ok := typeInfo.info.Defs[typeSpec.Name].(*types.TypeName)
	if !ok || obj == nil {
		return ""
	}
	named, ok := types.Unalias(obj.Type()).(*types.Named)
	if !ok {
		return ""
	}
	return sliceElemPtrFieldKey(named, fieldName)
}

func sliceElemPtrFieldInfoForTypeSpecField(typeSpec *ast.TypeSpec, fieldName string) (sliceElemPtrFieldInfo, bool) {
	key := sliceElemPtrFieldKeyForTypeSpecField(typeSpec, fieldName)
	if key == "" {
		return sliceElemPtrFieldInfo{}, false
	}
	return sliceElemPtrFieldInfoForKey(key)
}

func sliceElemPtrFieldInfoForStructNameField(structName string, fieldName string) (sliceElemPtrFieldInfo, bool) {
	typeInfo := GetTypeInfo()
	if structName == "" || fieldName == "" || typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return sliceElemPtrFieldInfo{}, false
	}
	obj, ok := typeInfo.pkg.Scope().Lookup(structName).(*types.TypeName)
	if !ok || obj == nil {
		return sliceElemPtrFieldInfo{}, false
	}
	named, ok := types.Unalias(obj.Type()).(*types.Named)
	if !ok {
		return sliceElemPtrFieldInfo{}, false
	}
	return sliceElemPtrFieldInfoForKey(sliceElemPtrFieldKey(named, fieldName))
}

func sliceElemPtrFieldInfoForSelector(sel *ast.SelectorExpr) (sliceElemPtrFieldInfo, bool) {
	key, _, ok := sliceElemPtrFieldKeyForSelector(sel)
	if !ok {
		return sliceElemPtrFieldInfo{}, false
	}
	return sliceElemPtrFieldInfoForKey(key)
}

func sliceElemPtrFieldInfoForKey(key string) (sliceElemPtrFieldInfo, bool) {
	ctx := GetTranspileContext()
	if key == "" || ctx == nil {
		return sliceElemPtrFieldInfo{}, false
	}
	if ctx.Package != nil {
		if info, ok := ctx.Package.SliceElemPtrFields[key]; ok {
			return info, true
		}
	}
	if ctx.Session != nil {
		info, ok := ctx.Session.SliceElemPtrFields[key]
		return info, ok
	}
	return sliceElemPtrFieldInfo{}, false
}

func registerSliceElemPtrReturnsFromFile(file *ast.File) {
	if file == nil {
		return
	}
	for _, decl := range file.Decls {
		fn, ok := decl.(*ast.FuncDecl)
		if !ok {
			continue
		}
		registerSliceElemPtrReturnDecl(fn)
		registerSliceElemPtrSliceReturnDecl(fn)
		arrayElemPtrResultInfosForFunc(fn)
	}
}

func registerSliceElemPtrReturnDecl(fn *ast.FuncDecl) {
	info, ok := sliceElemPtrReturnInfoForDecl(fn)
	if !ok {
		return
	}
	fnObj, ok := sliceElemPtrReturnFuncObject(fn)
	if !ok {
		return
	}
	if fnObj.Pkg() != nil {
		info.ownerPkgPath = fnObj.Pkg().Path()
	}
	ctx := GetTranspileContext()
	if ctx == nil || ctx.Package == nil {
		return
	}
	if ctx.Package.SliceElemPtrReturnFuncs == nil {
		ctx.Package.SliceElemPtrReturnFuncs = make(map[*types.Func]sliceElemPtrReturnInfo)
	}
	if ctx.Package.SliceElemPtrReturnFuncNames == nil {
		ctx.Package.SliceElemPtrReturnFuncNames = make(map[string]sliceElemPtrReturnInfo)
	}
	ctx.Package.SliceElemPtrReturnFuncs[fnObj] = info
	ctx.Package.SliceElemPtrReturnFuncNames[fnObj.FullName()] = info
	if ctx.Session != nil {
		if ctx.Session.SliceElemPtrReturnFuncNames == nil {
			ctx.Session.SliceElemPtrReturnFuncNames = make(map[string]sliceElemPtrReturnInfo)
		}
		ctx.Session.SliceElemPtrReturnFuncNames[fnObj.FullName()] = info
	}
}

func registerSliceElemPtrSliceReturnDecl(fn *ast.FuncDecl) {
	info, ok := sliceElemPtrSliceReturnInfoForDecl(fn)
	if !ok {
		return
	}
	fnObj, ok := sliceElemPtrReturnFuncObject(fn)
	if !ok {
		return
	}
	ctx := GetTranspileContext()
	if ctx == nil || ctx.Package == nil {
		return
	}
	if ctx.Package.SliceElemPtrSliceReturnFuncs == nil {
		ctx.Package.SliceElemPtrSliceReturnFuncs = make(map[*types.Func]sliceElemPtrSliceReturnInfo)
	}
	ctx.Package.SliceElemPtrSliceReturnFuncs[fnObj] = info
}

func sliceElemPtrReturnFuncObject(fn *ast.FuncDecl) (*types.Func, bool) {
	typeInfo := GetTypeInfo()
	if fn == nil || typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	obj, ok := typeInfo.info.Defs[fn.Name].(*types.Func)
	if !ok || obj == nil {
		return nil, false
	}
	return obj, true
}

func sliceElemPtrReturnInfoForFunc(fn *types.Func) (sliceElemPtrReturnInfo, bool) {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil || ctx.Package == nil {
		return sliceElemPtrReturnInfo{}, false
	}
	info, ok := ctx.Package.SliceElemPtrReturnFuncs[fn]
	if ok {
		return info, true
	}
	if info, ok := ctx.Package.SliceElemPtrReturnFuncNames[fn.FullName()]; ok {
		return info, true
	}
	if ctx.Session != nil {
		info, ok := ctx.Session.SliceElemPtrReturnFuncNames[fn.FullName()]
		return info, ok
	}
	return info, ok
}

func sliceElemPtrSliceReturnInfoForFunc(fn *types.Func) (sliceElemPtrSliceReturnInfo, bool) {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil || ctx.Package == nil {
		return sliceElemPtrSliceReturnInfo{}, false
	}
	info, ok := ctx.Package.SliceElemPtrSliceReturnFuncs[fn]
	return info, ok
}

func sliceElemPtrSliceParamInfosForFunc(fn *types.Func) (map[int]string, bool) {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil || ctx.Package == nil {
		return nil, false
	}
	info, ok := ctx.Package.SliceElemPtrSliceParamFuncs[fn]
	return info, ok
}

func sliceElemPtrSliceParamInfoForFunc(fn *types.Func, paramIndex int) (string, bool) {
	info, ok := sliceElemPtrSliceParamInfosForFunc(fn)
	if !ok {
		return "", false
	}
	elemRustType, ok := info[paramIndex]
	return elemRustType, ok
}

func sliceElemPtrReturnInfoForDeclObject(fn *ast.FuncDecl) (sliceElemPtrReturnInfo, bool) {
	fnObj, ok := sliceElemPtrReturnFuncObject(fn)
	if !ok {
		return sliceElemPtrReturnInfo{}, false
	}
	return sliceElemPtrReturnInfoForFunc(fnObj)
}

func sliceElemPtrSliceReturnInfoForDeclObject(fn *ast.FuncDecl) (sliceElemPtrSliceReturnInfo, bool) {
	fnObj, ok := sliceElemPtrReturnFuncObject(fn)
	if !ok {
		return sliceElemPtrSliceReturnInfo{}, false
	}
	return sliceElemPtrSliceReturnInfoForFunc(fnObj)
}

func sliceElemPtrSliceParamInfosForDeclObject(fn *ast.FuncDecl) (map[int]string, bool) {
	fnObj, ok := sliceElemPtrReturnFuncObject(fn)
	if !ok {
		return nil, false
	}
	return sliceElemPtrSliceParamInfosForFunc(fnObj)
}

func sliceElemPtrSliceParamInfoForDeclObject(fn *ast.FuncDecl, paramIndex int) (string, bool) {
	fnObj, ok := sliceElemPtrReturnFuncObject(fn)
	if !ok {
		return "", false
	}
	return sliceElemPtrSliceParamInfoForFunc(fnObj, paramIndex)
}

func sliceElemPtrReturnInfoForCall(call *ast.CallExpr) (sliceElemPtrReturnInfo, bool) {
	typeInfo := GetTypeInfo()
	fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
	if !ok {
		return sliceElemPtrReturnInfo{}, false
	}
	return sliceElemPtrReturnInfoForFunc(fn)
}

func sliceElemPtrSliceReturnInfoForCall(call *ast.CallExpr) (sliceElemPtrSliceReturnInfo, bool) {
	typeInfo := GetTypeInfo()
	fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
	if !ok {
		return sliceElemPtrSliceReturnInfo{}, false
	}
	return sliceElemPtrSliceReturnInfoForFunc(fn)
}

func sliceElemPtrReturnInfoForDecl(fn *ast.FuncDecl) (sliceElemPtrReturnInfo, bool) {
	if fn == nil || fn.Body == nil || fn.Type == nil || fn.Type.Results == nil {
		return sliceElemPtrReturnInfo{}, false
	}
	if fn.Type.Results.NumFields() != 1 {
		return sliceElemPtrReturnInfo{}, false
	}
	resultType := fn.Type.Results.List[0]
	if resultType == nil || len(resultType.Names) > 0 || !typeExprIsPointer(resultType.Type) {
		return sliceElemPtrReturnInfo{}, false
	}

	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return sliceElemPtrReturnInfo{}, false
	}
	candidates := collectSliceElemPtrCandidates(fn.Body)

	var elemRustType string
	var elemType types.Type
	sawSliceElemReturn := false
	valid := true
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		if !valid {
			return false
		}
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.ReturnStmt:
			if len(n.Results) != 1 {
				valid = false
				return false
			}
			result := unwrapParens(n.Results[0])
			if ident, ok := result.(*ast.Ident); ok && ident.Name == "nil" {
				return true
			}
			if resultElemType, typ, ok := sliceElemPtrAddressElemType(result); ok {
				if elemRustType == "" {
					elemRustType = typ
					elemType = resultElemType
				}
				if elemRustType != typ {
					valid = false
					return false
				}
				sawSliceElemReturn = true
				return true
			}
			ident, ok := result.(*ast.Ident)
			if !ok {
				valid = false
				return false
			}
			obj := typeInfo.GetObject(ident)
			typ, ok := candidates[obj]
			if obj == nil || !ok {
				valid = false
				return false
			}
			if elemRustType == "" {
				elemRustType = typ
				elemType, _ = sliceElemPtrPointerElemType(typeInfo.GetType(result))
			}
			if elemRustType != typ {
				valid = false
				return false
			}
			sawSliceElemReturn = true
		}
		return true
	})
	if !valid || !sawSliceElemReturn || elemRustType == "" {
		return sliceElemPtrReturnInfo{}, false
	}
	return sliceElemPtrReturnInfo{elemRustType: elemRustType, elemType: elemType}, true
}

func sliceElemPtrSliceReturnInfoForDecl(fn *ast.FuncDecl) (sliceElemPtrSliceReturnInfo, bool) {
	if fn == nil || fn.Body == nil || fn.Type == nil || fn.Type.Results == nil {
		return sliceElemPtrSliceReturnInfo{}, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return sliceElemPtrSliceReturnInfo{}, false
	}
	candidates := collectSliceElemPtrSliceCandidates(fn.Body)
	if len(candidates) == 0 {
		return sliceElemPtrSliceReturnInfo{}, false
	}

	resultCount := 0
	resultElemTypes := map[int]string{}
	for _, result := range fn.Type.Results.List {
		count := len(result.Names)
		if count == 0 {
			count = 1
		}
		for i := 0; i < count; i++ {
			if resultType, ok := resultTypeExprType(result.Type); ok {
				if elemRustType, ok := sliceElemPtrSliceElemRustTypeForType(resultType); ok {
					resultElemTypes[resultCount] = elemRustType
				}
			}
			resultCount++
		}
	}
	if len(resultElemTypes) == 0 {
		return sliceElemPtrSliceReturnInfo{}, false
	}

	valid := map[int]bool{}
	for index := range resultElemTypes {
		valid[index] = true
	}
	sawCandidate := map[int]bool{}
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.ReturnStmt:
			if len(n.Results) != resultCount {
				for index := range valid {
					valid[index] = false
				}
				return true
			}
			for index, elemRustType := range resultElemTypes {
				if !valid[index] {
					continue
				}
				result := unwrapParens(n.Results[index])
				if ident, ok := result.(*ast.Ident); ok && ident.Name == "nil" {
					continue
				}
				ident, ok := result.(*ast.Ident)
				if !ok {
					valid[index] = false
					continue
				}
				obj := typeInfo.GetObject(ident)
				candidateElem, ok := candidates[obj]
				if obj == nil || !ok || candidateElem != elemRustType {
					valid[index] = false
					continue
				}
				sawCandidate[index] = true
			}
		}
		return true
	})

	result := map[int]string{}
	for index, elemRustType := range resultElemTypes {
		if valid[index] && sawCandidate[index] {
			result[index] = elemRustType
		}
	}
	if len(result) == 0 {
		return sliceElemPtrSliceReturnInfo{}, false
	}
	return sliceElemPtrSliceReturnInfo{resultElemRustTypes: result}, true
}

func registerSliceElemPtrSliceParamsFromFiles(files []*ast.File) {
	ctx := GetTranspileContext()
	typeInfo := GetTypeInfo()
	if ctx == nil || ctx.Package == nil || typeInfo == nil {
		return
	}
	if ctx.Package.SliceElemPtrSliceParamFuncs == nil {
		ctx.Package.SliceElemPtrSliceParamFuncs = make(map[*types.Func]map[int]string)
	}

	for {
		changed := false
		for _, file := range files {
			if file == nil {
				continue
			}
			for _, decl := range file.Decls {
				fn, ok := decl.(*ast.FuncDecl)
				if !ok || fn.Body == nil {
					continue
				}
				candidates := collectSliceElemPtrSliceCandidatesForFunc(fn)
				if len(candidates) == 0 {
					continue
				}
				ast.Inspect(fn.Body, func(node ast.Node) bool {
					if _, ok := node.(*ast.FuncLit); ok {
						return false
					}
					call, ok := node.(*ast.CallExpr)
					if !ok {
						return true
					}
					callee, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
					if !ok {
						return true
					}
					for i, arg := range call.Args {
						argElemRustType, ok := sliceElemPtrSliceArgElemRustType(arg, candidates)
						if !ok {
							continue
						}
						paramElemRustType, ok := sliceElemPtrSliceCallParamElemRustType(callee, i)
						if !ok || paramElemRustType != argElemRustType {
							continue
						}
						if registerSliceElemPtrSliceParam(callee, i, argElemRustType) {
							changed = true
						}
					}
					return true
				})
			}
		}
		if !changed {
			return
		}
	}
}

func registerSliceElemPtrSliceParam(fn *types.Func, paramIndex int, elemRustType string) bool {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil || ctx.Package == nil {
		return false
	}
	if ctx.Package.SliceElemPtrSliceParamFuncs == nil {
		ctx.Package.SliceElemPtrSliceParamFuncs = make(map[*types.Func]map[int]string)
	}
	params := ctx.Package.SliceElemPtrSliceParamFuncs[fn]
	if params == nil {
		params = map[int]string{}
		ctx.Package.SliceElemPtrSliceParamFuncs[fn] = params
	}
	if _, ok := params[paramIndex]; ok {
		return false
	}
	params[paramIndex] = elemRustType
	return true
}

func sliceElemPtrSliceArgElemRustType(arg ast.Expr, candidates map[types.Object]string) (string, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", false
	}
	if ident, ok := unwrapParens(arg).(*ast.Ident); ok {
		obj := typeInfo.GetObject(ident)
		if obj == nil {
			return "", false
		}
		elemRustType, ok := candidates[obj]
		return elemRustType, ok
	}
	if call, ok := unwrapParens(arg).(*ast.CallExpr); ok {
		info, ok := sliceElemPtrSliceReturnInfoForCall(call)
		if !ok {
			return "", false
		}
		elemRustType, ok := info.resultElemRustTypes[0]
		return elemRustType, ok
	}
	return "", false
}

func sliceElemPtrSliceCallParamElemRustType(fn *types.Func, paramIndex int) (string, bool) {
	if fn == nil || paramIndex < 0 {
		return "", false
	}
	sig, ok := signatureFromType(fn.Type())
	if !ok || sig.Params() == nil || sig.Params().Len() == 0 {
		return "", false
	}
	paramCount := sig.Params().Len()
	if paramIndex >= paramCount {
		return "", false
	}
	paramType := sig.Params().At(paramIndex).Type()
	return sliceElemPtrSliceElemRustTypeForType(paramType)
}

func pushCurrentSliceElemPtrReturn(fn *ast.FuncDecl) func() {
	prev := currentSliceElemPtrReturn
	prevSlice := currentSliceElemPtrSliceReturn
	if info, ok := sliceElemPtrReturnInfoForDeclObject(fn); ok {
		currentSliceElemPtrReturn = &info
	} else {
		currentSliceElemPtrReturn = nil
	}
	if info, ok := sliceElemPtrSliceReturnInfoForDeclObject(fn); ok {
		currentSliceElemPtrSliceReturn = &info
	} else {
		currentSliceElemPtrSliceReturn = nil
	}
	return func() {
		currentSliceElemPtrReturn = prev
		currentSliceElemPtrSliceReturn = prevSlice
	}
}

func currentFunctionReturnsSliceElemPtr() bool {
	return currentSliceElemPtrReturn != nil
}

func registerSliceElemPtrVar(name string, elemRustType string) {
	NeedSliceElemPtr()
	if vt := GetVarTable(); vt != nil {
		vt.Register(name, &VarInfo{
			WrapLevel:   WrapOption,
			RustType:    "Option<GoSliceElemPtr<" + elemRustType + ">>",
			Source:      SourceLocal,
			PointerKind: PointerSliceElem,
		})
	}
}

func registerGoPtrVar(name string, elemRustType string, goType types.Type) {
	NeedSliceElemPtr()
	if vt := GetVarTable(); vt != nil {
		vt.Register(name, &VarInfo{
			WrapLevel:   WrapNone,
			RustType:    "GoPtr<" + elemRustType + ">",
			Source:      SourceLocal,
			PointerKind: PointerGoPtr,
			GoType:      goType,
		})
	}
}

func registerArrayElemPtrVar(name string, info arrayElemPtrInfo) {
	NeedSliceElemPtr()
	if vt := GetVarTable(); vt != nil {
		vt.Register(name, &VarInfo{
			WrapLevel:   WrapOption,
			RustType:    arrayElemPtrOptionRustType(info),
			Source:      SourceLocal,
			PointerKind: PointerArrayElem,
		})
	}
}

func collectSliceElemPtrCandidates(body *ast.BlockStmt) map[types.Object]string {
	typeInfo := GetTypeInfo()
	if body == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}

	candidates := map[types.Object]*sliceElemPtrCandidate{}

	ast.Inspect(body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.ValueSpec:
			if _, ok := n.Type.(*ast.StarExpr); !ok {
				return true
			}
			for i, name := range n.Names {
				if name.Name == "_" {
					continue
				}
				obj := typeInfo.GetObject(name)
				if obj == nil {
					continue
				}
				elemRustType, ok := sliceElemPtrRustTypeForPointerType(obj.Type())
				if !ok {
					continue
				}
				state := &sliceElemPtrCandidate{
					elemRustType: elemRustType,
					valid:        true,
				}
				if len(n.Values) > i {
					ok, sawSliceAddr := isSliceElemPtrAssignmentValue(n.Values[i])
					state.valid = ok
					state.sawSliceAddr = sawSliceAddr
				}
				candidates[obj] = state
			}
		case *ast.AssignStmt:
			if n.Tok != token.DEFINE {
				return true
			}
			for i, lhs := range n.Lhs {
				ident, ok := lhs.(*ast.Ident)
				if !ok || ident.Name == "_" {
					continue
				}
				if typeInfo == nil {
					continue
				}
				obj := typeInfo.GetObject(ident)
				if obj == nil {
					continue
				}
				if _, exists := candidates[obj]; exists {
					continue
				}
				elemRustType, ok := sliceElemPtrRustTypeForPointerType(obj.Type())
				if !ok {
					continue
				}
				rhs := assignmentRHSForLHS(n, i)
				if rhs == nil {
					continue
				}
				rhsOk, sawSliceAddr := isSliceElemPtrAssignmentValue(rhs)
				if !rhsOk || !sawSliceAddr {
					continue
				}
				candidates[obj] = &sliceElemPtrCandidate{
					elemRustType: elemRustType,
					valid:        true,
					sawSliceAddr: true,
				}
			}
		}
		return true
	})

	if len(candidates) == 0 {
		return nil
	}

	ast.Inspect(body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.AssignStmt:
			for i, lhs := range n.Lhs {
				ident, ok := lhs.(*ast.Ident)
				if !ok {
					continue
				}
				var state *sliceElemPtrCandidate
				if obj := typeInfo.GetObject(ident); obj != nil {
					state = candidates[obj]
				}
				if state == nil {
					continue
				}
				rhs := assignmentRHSForLHS(n, i)
				if rhs == nil {
					state.valid = false
					continue
				}
				ok, sawSliceAddr := isSliceElemPtrAssignmentValue(rhs)
				if !ok {
					state.valid = false
					continue
				}
				if sawSliceAddr {
					state.sawSliceAddr = true
				}
			}
		}
		return true
	})

	result := map[types.Object]string{}
	for obj, state := range candidates {
		if state.valid && state.sawSliceAddr {
			result[obj] = state.elemRustType
		}
	}
	if len(result) == 0 {
		result = nil
	}
	return result
}

func collectArrayElemPtrCandidates(body *ast.BlockStmt) map[types.Object]arrayElemPtrInfo {
	return collectArrayElemPtrCandidatesWithSeeds(body, nil)
}

func collectArrayElemPtrCandidatesForFunc(fn *ast.FuncDecl) map[types.Object]arrayElemPtrInfo {
	if fn == nil {
		return nil
	}
	return collectArrayElemPtrCandidatesWithSeeds(fn.Body, arrayElemPtrResultCandidateSeeds(fn))
}

func collectArrayElemPtrCandidatesWithSeeds(body *ast.BlockStmt, seeds map[types.Object]*arrayElemPtrCandidate) map[types.Object]arrayElemPtrInfo {
	typeInfo := GetTypeInfo()
	if body == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}

	candidates := map[types.Object]*arrayElemPtrCandidate{}
	for obj, state := range seeds {
		candidates[obj] = state
	}

	ast.Inspect(body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.ValueSpec:
			if _, ok := n.Type.(*ast.StarExpr); !ok {
				return true
			}
			for i, name := range n.Names {
				if name.Name == "_" {
					continue
				}
				obj := typeInfo.GetObject(name)
				if obj == nil {
					continue
				}
				elemRustType, ok := sliceElemPtrRustTypeForPointerType(obj.Type())
				if !ok {
					continue
				}
				state := &arrayElemPtrCandidate{
					info:  arrayElemPtrInfo{elemRustType: elemRustType},
					valid: true,
				}
				if len(n.Values) > i {
					rhsInfo, ok, sawArrayAddr := isArrayElemPtrAssignmentValue(n.Values[i])
					state.valid = ok
					state.sawArrayAddr = sawArrayAddr
					if sawArrayAddr {
						state.info = rhsInfo
					}
				}
				candidates[obj] = state
			}
		case *ast.AssignStmt:
			if n.Tok == token.DEFINE {
				for i, lhs := range n.Lhs {
					ident, ok := lhs.(*ast.Ident)
					if !ok || ident.Name == "_" {
						continue
					}
					obj := typeInfo.GetObject(ident)
					if obj == nil {
						continue
					}
					if _, exists := candidates[obj]; exists {
						continue
					}
					elemRustType, ok := sliceElemPtrRustTypeForPointerType(obj.Type())
					if !ok {
						continue
					}
					rhsInfo, ok, sawArrayAddr := isArrayElemPtrAssignmentValue(assignmentRHSForLHS(n, i))
					if !ok {
						continue
					}
					info := arrayElemPtrInfo{elemRustType: elemRustType}
					if sawArrayAddr {
						info = rhsInfo
					}
					candidates[obj] = &arrayElemPtrCandidate{
						info:         info,
						valid:        true,
						sawArrayAddr: sawArrayAddr,
					}
				}
				return true
			}
			if n.Tok != token.ASSIGN {
				return true
			}
			for i, lhs := range n.Lhs {
				ident, ok := lhs.(*ast.Ident)
				if !ok {
					continue
				}
				obj := typeInfo.GetObject(ident)
				if obj == nil {
					continue
				}
				state := candidates[obj]
				if state == nil || !state.valid {
					continue
				}
				rhsInfo, ok, sawArrayAddr := isArrayElemPtrAssignmentValue(assignmentRHSForLHS(n, i))
				if !ok {
					state.valid = false
					continue
				}
				if sawArrayAddr {
					if state.sawArrayAddr && (state.info.elemRustType != rhsInfo.elemRustType || state.info.arrayLen != rhsInfo.arrayLen) {
						state.valid = false
						continue
					}
					state.info = rhsInfo
					state.sawArrayAddr = true
				}
			}
		}
		return true
	})

	result := map[types.Object]arrayElemPtrInfo{}
	for obj, state := range candidates {
		if state.valid && state.sawArrayAddr && state.info.elemRustType != "" && state.info.arrayLen >= 0 {
			result[obj] = state.info
		}
	}
	if len(result) == 0 {
		return nil
	}
	return result
}

func arrayElemPtrResultCandidateSeeds(fn *ast.FuncDecl) map[types.Object]*arrayElemPtrCandidate {
	typeInfo := GetTypeInfo()
	if fn == nil || fn.Type == nil || fn.Type.Results == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	seeds := map[types.Object]*arrayElemPtrCandidate{}
	for _, result := range fn.Type.Results.List {
		for _, name := range result.Names {
			if name.Name == "_" {
				continue
			}
			obj := typeInfo.GetObject(name)
			if obj == nil {
				continue
			}
			elemRustType, ok := sliceElemPtrRustTypeForPointerType(obj.Type())
			if !ok {
				continue
			}
			seeds[obj] = &arrayElemPtrCandidate{
				info:  arrayElemPtrInfo{elemRustType: elemRustType},
				valid: true,
			}
		}
	}
	if len(seeds) == 0 {
		return nil
	}
	return seeds
}

func collectSliceElemPtrSliceCandidates(body *ast.BlockStmt) map[types.Object]string {
	typeInfo := GetTypeInfo()
	if body == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}

	type state struct {
		elemRustType string
		valid        bool
		sawSliceAddr bool
	}
	candidates := map[types.Object]*state{}

	ast.Inspect(body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.AssignStmt:
			if len(n.Rhs) == 1 {
				if call, ok := unwrapParens(n.Rhs[0]).(*ast.CallExpr); ok {
					if info, ok := sliceElemPtrSliceReturnInfoForCall(call); ok {
						for resultIndex, lhs := range n.Lhs {
							elemRustType, ok := info.resultElemRustTypes[resultIndex]
							if !ok {
								continue
							}
							ident, ok := unwrapParens(lhs).(*ast.Ident)
							if !ok || ident.Name == "_" {
								continue
							}
							obj := typeInfo.GetObject(ident)
							if obj == nil {
								continue
							}
							if lhsElemRustType, ok := sliceElemPtrSliceElemRustTypeForType(obj.Type()); !ok || lhsElemRustType != elemRustType {
								continue
							}
							st := candidates[obj]
							if st == nil {
								st = &state{elemRustType: elemRustType, valid: true}
								candidates[obj] = st
							}
							st.sawSliceAddr = true
						}
					}
				}
			}
			for i, lhs := range n.Lhs {
				index, ok := unwrapParens(lhs).(*ast.IndexExpr)
				if !ok || typeInfo.IsMap(index.X) {
					continue
				}
				ident, ok := unwrapParens(index.X).(*ast.Ident)
				if !ok || ident.Name == "_" {
					continue
				}
				obj := typeInfo.GetObject(ident)
				if obj == nil {
					continue
				}
				elemRustType, ok := sliceElemPtrSliceElemRustTypeForType(obj.Type())
				if !ok {
					continue
				}
				st := candidates[obj]
				if st == nil {
					st = &state{elemRustType: elemRustType, valid: true}
					candidates[obj] = st
				}
				rhs := assignmentRHSForLHS(n, i)
				if rhs == nil {
					st.valid = false
					continue
				}
				ok, sawSliceAddr := isSliceElemPtrAssignmentValue(rhs)
				if !ok {
					st.valid = false
					continue
				}
				if sawSliceAddr {
					st.sawSliceAddr = true
				}
			}
		}
		return true
	})

	if len(candidates) == 0 {
		return nil
	}
	result := map[types.Object]string{}
	for obj, st := range candidates {
		if st.valid && st.sawSliceAddr {
			result[obj] = st.elemRustType
		}
	}
	if len(result) == 0 {
		return nil
	}
	return result
}

func collectSliceElemPtrSliceCandidatesForFunc(fn *ast.FuncDecl) map[types.Object]string {
	result := collectSliceElemPtrSliceCandidates(fn.Body)
	typeInfo := GetTypeInfo()
	paramInfos, ok := sliceElemPtrSliceParamInfosForDeclObject(fn)
	if !ok || typeInfo == nil || fn == nil || fn.Type == nil || fn.Type.Params == nil {
		return result
	}
	if result == nil {
		result = map[types.Object]string{}
	}
	paramIndex := 0
	for _, field := range fn.Type.Params.List {
		if len(field.Names) == 0 {
			paramIndex++
			continue
		}
		for _, name := range field.Names {
			if elemRustType, ok := paramInfos[paramIndex]; ok {
				if obj := typeInfo.GetObject(name); obj != nil {
					result[obj] = elemRustType
				}
			}
			paramIndex++
		}
	}
	if len(result) == 0 {
		return nil
	}
	return result
}

func sliceElemPtrSliceElemRustTypeForType(t types.Type) (string, bool) {
	if t == nil {
		return "", false
	}
	slice, ok := types.Unalias(t).Underlying().(*types.Slice)
	if !ok {
		return "", false
	}
	ptr, ok := types.Unalias(slice.Elem()).Underlying().(*types.Pointer)
	if !ok {
		return "", false
	}
	return goTypesTypeToRust(ptr.Elem()), true
}

func sliceElemPtrRustTypeForPointerType(t types.Type) (string, bool) {
	if t == nil {
		return "", false
	}
	ptr, ok := types.Unalias(t).Underlying().(*types.Pointer)
	if !ok {
		return "", false
	}
	return goTypesCollectionElemTypeToRust(coreType(ptr.Elem())), true
}

func assignmentRHSForLHS(stmt *ast.AssignStmt, lhsIndex int) ast.Expr {
	if len(stmt.Rhs) == len(stmt.Lhs) {
		return stmt.Rhs[lhsIndex]
	}
	if len(stmt.Rhs) == 1 && len(stmt.Lhs) == 1 {
		return stmt.Rhs[0]
	}
	return nil
}

func isSliceElemPtrAssignmentValue(expr ast.Expr) (bool, bool) {
	expr = unwrapParens(expr)
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
		return true, false
	}
	if _, ok := sliceElemPtrAddressElemRustType(expr); ok {
		return true, true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false, false
	}
	return typeInfo.IsPointer(expr), false
}

func isArrayElemPtrAssignmentValue(expr ast.Expr) (arrayElemPtrInfo, bool, bool) {
	expr = unwrapParens(expr)
	if expr == nil {
		return arrayElemPtrInfo{}, false, false
	}
	if ident, ok := expr.(*ast.Ident); ok && ident.Name == "nil" {
		return arrayElemPtrInfo{}, true, false
	}
	if info, ok := arrayElemPtrAddressInfo(expr); ok {
		return info, true, true
	}
	if ident, ok := expr.(*ast.Ident); ok && isArrayElemPtrVar(ident.Name) {
		return arrayElemPtrInfo{}, true, false
	}
	return arrayElemPtrInfo{}, false, false
}

func sliceElemPtrAddressElemRustType(expr ast.Expr) (string, bool) {
	_, elemRustType, ok := sliceElemPtrAddressElemType(expr)
	return elemRustType, ok
}

func sliceElemPtrAddressElemType(expr ast.Expr) (types.Type, string, bool) {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return nil, "", false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return nil, "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.GetType(indexExpr.X) == nil || typeInfo.IsMap(indexExpr.X) {
		return nil, "", false
	}
	if !typeInfo.IsSlice(indexExpr.X) {
		return nil, "", false
	}
	elemType := typeInfo.GetSliceElemType(indexExpr.X)
	if elemType == nil {
		return nil, "", false
	}
	return coreType(elemType), goTypesCollectionElemTypeToRust(elemType), true
}

func arrayElemAddressPointerRustType(expr ast.Expr) (string, bool) {
	if info, ok := arrayElemPtrAddressInfo(expr); ok {
		return arrayElemPtrOptionRustType(info), true
	}
	return "", false
}

func arrayElemPtrAddressInfo(expr ast.Expr) (arrayElemPtrInfo, bool) {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return arrayElemPtrInfo{}, false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return arrayElemPtrInfo{}, false
	}
	return arrayElemPtrAddressInfoForIndex(indexExpr)
}

func arrayElemPtrAddressInfoForIndex(indexExpr *ast.IndexExpr) (arrayElemPtrInfo, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.GetType(indexExpr.X) == nil {
		return arrayElemPtrInfo{}, false
	}
	if ident, ok := unwrapParens(indexExpr.X).(*ast.Ident); ok && arrayElemPtrIdentPointsToArray(ident) {
		return arrayElemPtrInfo{}, false
	}
	arrayType, ok := arrayTypeForExpr(indexExpr.X, typeInfo)
	if !ok {
		return arrayElemPtrInfo{}, false
	}
	elemRustType := goTypesCollectionElemTypeToRust(coreType(arrayType.Elem()))
	if elemRustType == "" {
		return arrayElemPtrInfo{}, false
	}
	return arrayElemPtrInfo{elemRustType: elemRustType, arrayLen: arrayType.Len()}, true
}

func arrayTypeForExpr(expr ast.Expr, typeInfo *TypeInfo) (*types.Array, bool) {
	if typeInfo == nil {
		return nil, false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return nil, false
	}
	if arrayType, ok := coreUnderlyingType(typ).(*types.Array); ok {
		return arrayType, true
	}
	if ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer); ok {
		if arrayType, ok := coreUnderlyingType(ptr.Elem()).(*types.Array); ok {
			return arrayType, true
		}
	}
	return nil, false
}

func arrayElemPtrOptionRustType(info arrayElemPtrInfo) string {
	return "Option<" + arrayElemPtrRustType(info) + ">"
}

func arrayElemPtrRustType(info arrayElemPtrInfo) string {
	return "GoArrayElemPtr<" + info.elemRustType + ", " + strconv.FormatInt(info.arrayLen, 10) + ">"
}

func writeArrayElemPtrNewExpression(out *strings.Builder, indexExpr *ast.IndexExpr) bool {
	if ident, ok := unwrapParens(indexExpr.X).(*ast.Ident); ok && arrayElemPtrIdentPointsToArray(ident) {
		out.WriteString(`unimplemented!("array element address through pointer-to-array requires nested pointer representation")`)
		return true
	}
	if _, ok := arrayElemPtrAddressInfoForIndex(indexExpr); !ok {
		return false
	}
	NeedSliceElemPtr()
	out.WriteString("GoArrayElemPtr::new(")
	writeArrayElemPtrSequenceHandle(out, indexExpr)
	out.WriteString(", ")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString(")")
	return true
}

func writeArrayElemPtrSequenceHandle(out *strings.Builder, indexExpr *ast.IndexExpr) {
	typeInfo := GetTypeInfo()
	if pointerArray, ok := pointerToArrayDerefOperand(indexExpr.X, typeInfo); ok {
		TranspileExpressionContext(out, pointerArray, LValue)
		out.WriteString(".clone()")
		return
	}
	TranspileExpressionContext(out, indexExpr.X, LValue)
	out.WriteString(".clone()")
}

func writeArrayElemPtrOptionValue(out *strings.Builder, rhs ast.Expr) bool {
	if ident, ok := unwrapParens(rhs).(*ast.Ident); ok {
		if ident.Name == "nil" {
			out.WriteString("None")
			return true
		}
		if isArrayElemPtrVar(ident.Name) {
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(".clone()")
			return true
		}
	}
	unary, ok := unwrapParens(rhs).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return false
	}
	if _, ok := arrayElemPtrAddressInfoForIndex(indexExpr); !ok {
		return false
	}
	out.WriteString("Some(")
	writeArrayElemPtrNewExpression(out, indexExpr)
	out.WriteString(")")
	return true
}

func arrayElemPtrIdentPointsToArray(ident *ast.Ident) bool {
	_, ok := arrayElemPtrIdentPointedArrayType(ident)
	return ok
}

func arrayElemPtrIdentPointedArrayType(ident *ast.Ident) (*types.Array, bool) {
	if ident == nil || !isArrayElemPtrVar(ident.Name) {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	typ := typeInfo.GetType(ident)
	if typ == nil {
		return nil, false
	}
	ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	if !ok {
		return nil, false
	}
	arrayType, ok := coreUnderlyingType(ptr.Elem()).(*types.Array)
	return arrayType, ok
}

func writeArrayElemPtrPointedArrayClone(out *strings.Builder, ident *ast.Ident) bool {
	if !arrayElemPtrIdentPointsToArray(ident) {
		return false
	}
	out.WriteString("{ let __seq = ")
	writeArrayElemPtrBorrow(out, ident, false)
	out.WriteString("; __seq.as_ref().unwrap().clone() }")
	return true
}

func writeArrayElemPtrPointedArrayIndexValue(out *strings.Builder, expr ast.Expr, index ast.Expr) bool {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok || !arrayElemPtrIdentPointsToArray(ident) {
		return false
	}
	out.WriteString("{ let __seq = ")
	writeArrayElemPtrBorrow(out, ident, false)
	out.WriteString("; __seq.as_ref().unwrap()[")
	writeExpressionAsUsize(out, index)
	out.WriteString("].clone() }")
	return true
}

func arrayElemPtrAddressElemRustType(expr ast.Expr) (string, bool) {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return "", false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return "", false
	}
	ident, ok := unwrapParens(indexExpr.X).(*ast.Ident)
	if !ok || !arrayElemPtrIdentPointsToArray(ident) {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", false
	}
	elemType := typeInfo.GetArrayOrSliceElemType(indexExpr.X)
	if elemType == nil {
		return "", false
	}
	elemRustType := goTypesCollectionElemTypeToRust(elemType)
	if elemRustType == "" {
		return "", false
	}
	return elemRustType, true
}

func writeArrayElemPtrAddressReadOnlyWrapper(out *strings.Builder, expr ast.Expr) bool {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return false
	}
	ident, ok := unwrapParens(indexExpr.X).(*ast.Ident)
	if !ok || !arrayElemPtrIdentPointsToArray(ident) {
		return false
	}
	trackWrapperImports()
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::new(")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("::new({ let __seq = ")
	writeArrayElemPtrBorrow(out, ident, false)
	out.WriteString("; Some(__seq.as_ref().unwrap()[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("].clone()) }))")
	return true
}

func unwrapParens(expr ast.Expr) ast.Expr {
	for {
		paren, ok := expr.(*ast.ParenExpr)
		if !ok {
			return expr
		}
		expr = paren.X
	}
}

func writeSliceElemPtrOptionValue(out *strings.Builder, rhs ast.Expr) bool {
	if call, ok := unwrapParens(rhs).(*ast.CallExpr); ok {
		if _, ok := sliceElemPtrReturnInfoForCall(call); ok {
			TranspileExpression(out, rhs)
			return true
		}
	}
	if ident, ok := unwrapParens(rhs).(*ast.Ident); ok && isSliceElemPtrVar(ident.Name) {
		out.WriteString(RustIdentForUse(ident))
		out.WriteString(".clone()")
		return true
	}
	if ok, _ := isSliceElemPtrAssignmentValue(rhs); !ok {
		return false
	}
	rhs = unwrapParens(rhs)
	if ident, ok := rhs.(*ast.Ident); ok && ident.Name == "nil" {
		out.WriteString("None")
		return true
	}
	out.WriteString("Some(")
	TranspileExpression(out, rhs)
	out.WriteString(")")
	return true
}

func writeSliceElemPtrMapKeyExpression(out *strings.Builder, expr ast.Expr) bool {
	if ident, ok := unwrapParens(expr).(*ast.Ident); ok && isSliceElemPtrVar(ident.Name) {
		NeedGoPtrKey()
		out.WriteString("GoLocalPtrKey::from_slice_elem(")
		out.WriteString(RustIdentForUse(ident))
		out.WriteString(".clone())")
		return true
	}
	if _, ok := sliceElemPtrAddressElemRustType(expr); ok {
		NeedGoPtrKey()
		out.WriteString("GoLocalPtrKey::from_slice_elem(Some(")
		TranspileExpression(out, expr)
		out.WriteString("))")
		return true
	}
	return false
}

func sliceElemPtrSliceRustType(elemRustType string) string {
	NeedSliceElemPtr()
	return goTypesWrappedRustType("Vec<GoPtr<" + elemRustType + ">>")
}

func writeSliceElemPtrSliceMake(out *strings.Builder, call *ast.CallExpr, elemRustType string) bool {
	if call == nil || len(call.Args) < 1 {
		return false
	}
	NeedSliceElemPtr()
	WriteWrapperPrefix(out)
	writeSliceMakeBody(out, call.Args, "GoPtr::nil()", "GoPtr<"+elemRustType+">")
	WriteWrapperSuffix(out)
	return true
}

func writeSliceElemPtrSliceSlotValue(out *strings.Builder, rhs ast.Expr, elemRustType string) bool {
	if call, ok := unwrapParens(rhs).(*ast.CallExpr); ok {
		if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
			if info.elemRustType != elemRustType {
				return false
			}
			NeedSliceElemPtr()
			out.WriteString("GoPtr::slice_elem_opt(")
			TranspileExpression(out, rhs)
			out.WriteString(")")
			return true
		}
	}
	if ident, ok := unwrapParens(rhs).(*ast.Ident); ok && isSliceElemPtrVar(ident.Name) {
		NeedSliceElemPtr()
		out.WriteString("GoPtr::slice_elem_opt(")
		out.WriteString(RustIdentForUse(ident))
		out.WriteString(".clone())")
		return true
	}
	if ident, ok := unwrapParens(rhs).(*ast.Ident); ok && ident.Name == "nil" {
		NeedSliceElemPtr()
		out.WriteString("GoPtr::nil()")
		return true
	}
	if rhsElemRustType, ok := sliceElemPtrAddressElemRustType(rhs); ok {
		if rhsElemRustType != elemRustType {
			return false
		}
		NeedSliceElemPtr()
		out.WriteString("GoPtr::slice_elem(")
		TranspileExpression(out, rhs)
		out.WriteString(")")
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if rhsElemRustType, ok := sliceElemPtrRustTypeForPointerType(typeInfo.GetType(rhs)); ok {
		if rhsElemRustType != elemRustType {
			return false
		}
		NeedSliceElemPtr()
		out.WriteString("GoPtr::local(")
		writePointerHandleValueClone(out, rhs)
		out.WriteString(")")
		return true
	}
	return false
}

func writeSliceElemPtrFieldAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	sel, ok := unwrapParens(lhs).(*ast.SelectorExpr)
	if !ok {
		return false
	}
	fieldInfo, ok := sliceElemPtrFieldInfoForSelector(sel)
	if !ok {
		return false
	}
	out.WriteString("{ let new_val = ")
	if !writeSliceElemPtrFieldValueWithInfo(out, rhs, fieldInfo) {
		out.WriteString(`unimplemented!("slice element pointer field assignment requires compatible pointer value")`)
	}
	out.WriteString("; ")
	if !writePointerHandleSelectorTarget(out, sel) {
		TranspileExpressionContext(out, sel, LValue)
	}
	out.WriteString(" = new_val; }")
	return true
}

func writeSliceElemPtrFieldValue(out *strings.Builder, rhs ast.Expr, elemRustType string) bool {
	return writeSliceElemPtrFieldValueWithInfo(out, rhs, sliceElemPtrFieldInfo{elemRustType: elemRustType})
}

func writeSliceElemPtrFieldValueWithInfo(out *strings.Builder, rhs ast.Expr, fieldInfo sliceElemPtrFieldInfo) bool {
	helperPrefix := sliceElemPtrFieldHelperPrefix(fieldInfo)
	if call, ok := unwrapParens(rhs).(*ast.CallExpr); ok {
		if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
			if !sliceElemPtrElemCompatible(info.elemType, info.elemRustType, fieldInfo) {
				return false
			}
			writeSliceElemPtrFieldCallValue(out, call, info, fieldInfo, helperPrefix)
			return true
		}
	}
	if ident, ok := unwrapParens(rhs).(*ast.Ident); ok {
		if ident.Name == "nil" {
			needSliceElemPtrFieldHelper(helperPrefix)
			out.WriteString(helperPrefix)
			out.WriteString("GoPtr::nil()")
			return true
		}
		if isSliceElemPtrVar(ident.Name) {
			needSliceElemPtrFieldHelper(helperPrefix)
			out.WriteString(helperPrefix)
			out.WriteString("GoPtr::slice_elem_opt(")
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(".clone())")
			return true
		}
	}
	if sel, ok := unwrapParens(rhs).(*ast.SelectorExpr); ok {
		if rhsFieldInfo, ok := sliceElemPtrFieldInfoForSelector(sel); ok && sliceElemPtrElemCompatible(rhsFieldInfo.elemType, rhsFieldInfo.elemRustType, fieldInfo) {
			TranspileExpressionContext(out, rhs, LValue)
			out.WriteString(".clone()")
			return true
		}
	}
	if rhsElemType, rhsElemRustType, ok := sliceElemPtrAddressElemType(rhs); ok {
		if !sliceElemPtrElemCompatible(rhsElemType, rhsElemRustType, fieldInfo) {
			return false
		}
		writeSliceElemPtrFieldAddressValue(out, rhs, helperPrefix)
		return true
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	if rhsElemRustType, ok := sliceElemPtrRustTypeForPointerType(typeInfo.GetType(rhs)); ok {
		rhsElemType, _ := sliceElemPtrPointerElemType(typeInfo.GetType(rhs))
		if !sliceElemPtrElemCompatible(rhsElemType, rhsElemRustType, fieldInfo) {
			return false
		}
		needSliceElemPtrFieldHelper(helperPrefix)
		out.WriteString(helperPrefix)
		out.WriteString("GoPtr::local(")
		writePointerHandleValueClone(out, rhs)
		out.WriteString(")")
		return true
	}
	return false
}

func writeSliceElemPtrFieldCallValue(out *strings.Builder, call *ast.CallExpr, returnInfo sliceElemPtrReturnInfo, fieldInfo sliceElemPtrFieldInfo, helperPrefix string) {
	needSliceElemPtrFieldHelper(helperPrefix)
	if returnInfo.ownerPkgPath == "" || returnInfo.ownerPkgPath == fieldInfo.ownerPkgPath {
		out.WriteString(helperPrefix)
		out.WriteString("GoPtr::slice_elem_opt(")
		TranspileExpression(out, call)
		out.WriteString(")")
		return
	}
	out.WriteString("match ")
	TranspileExpression(out, call)
	out.WriteString(" { Some(__ptr) => ")
	out.WriteString(helperPrefix)
	out.WriteString("GoPtr::slice_elem(")
	out.WriteString(helperPrefix)
	out.WriteString("GoSliceElemPtr::new(__ptr.slice_handle(), __ptr.index())), None => ")
	out.WriteString(helperPrefix)
	out.WriteString("GoPtr::nil() }")
}

func writeSliceElemPtrFieldAddressValue(out *strings.Builder, rhs ast.Expr, helperPrefix string) bool {
	unary, ok := unwrapParens(rhs).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return false
	}
	needSliceElemPtrFieldHelper(helperPrefix)
	out.WriteString(helperPrefix)
	out.WriteString("GoPtr::slice_elem(")
	out.WriteString(helperPrefix)
	out.WriteString("GoSliceElemPtr::new(")
	TranspileExpressionContext(out, indexExpr.X, LValue)
	out.WriteString(".clone(), ")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("))")
	return true
}

func needSliceElemPtrFieldHelper(helperPrefix string) {
	if helperPrefix == "" {
		NeedSliceElemPtr()
	}
}

func sliceElemPtrFieldHelperPrefix(info sliceElemPtrFieldInfo) string {
	if info.ownerPkgPath == "" {
		return ""
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil && typeInfo.pkg.Path() == info.ownerPkgPath {
		return ""
	}
	ctx := GetTranspileContext()
	if ctx == nil || ctx.PackageMapping == nil {
		return ""
	}
	crateName := ctx.PackageMapping[info.ownerPkgPath]
	if crateName == "" {
		return ""
	}
	TrackGeneratedCrateDependency(crateName)
	return crateName + "::"
}

func writeSliceElemPtrFieldNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	sel, ok := unwrapParens(expr).(*ast.SelectorExpr)
	if !ok {
		return false
	}
	if _, ok := sliceElemPtrFieldInfoForSelector(sel); !ok {
		return false
	}
	out.WriteString("{ let __ptr_field = ")
	TranspileExpressionContext(out, sel, LValue)
	out.WriteString(".clone(); ")
	if op == token.NEQ {
		out.WriteString("!")
	}
	out.WriteString("__ptr_field.is_nil() }")
	return true
}

func writeSliceElemPtrFieldPointeeSelector(out *strings.Builder, base *ast.SelectorExpr, fieldInfo FieldAccessInfo, expr *ast.SelectorExpr, ctx ExprContext) bool {
	if _, ok := sliceElemPtrFieldInfoForSelector(base); !ok {
		return false
	}
	if ctx != RValue {
		out.WriteString(`unimplemented!("slice element pointer field selector requires rvalue support")`)
		return true
	}
	if fieldInfo.IsPromoted {
		out.WriteString(`unimplemented!("slice element pointer field selector requires promoted-field support")`)
		return true
	}
	out.WriteString("(*{ let __ptr_value = ")
	TranspileExpressionContext(out, base, LValue)
	out.WriteString(".borrow(); __ptr_value.as_ref().unwrap().")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone() }")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	writeSelectorRValueClose(out, expr)
	return true
}

func writeSliceElemPtrSliceReturnValue(out *strings.Builder, result ast.Expr, resultIndex int) bool {
	info, ok := currentFunctionSliceElemPtrSliceReturnInfo()
	if !ok {
		return false
	}
	if _, ok := info.resultElemRustTypes[resultIndex]; !ok {
		return false
	}
	ident, ok := unwrapParens(result).(*ast.Ident)
	if !ok {
		return false
	}
	if ident.Name == "nil" {
		WriteWrappedNone(out)
		return true
	}
	if _, ok := sliceElemPtrSliceCandidateForDecl(ident); !ok {
		return false
	}
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(".clone()")
	return true
}

func currentFunctionSliceElemPtrSliceReturnInfo() (sliceElemPtrSliceReturnInfo, bool) {
	if currentSliceElemPtrSliceReturn == nil {
		return sliceElemPtrSliceReturnInfo{}, false
	}
	return *currentSliceElemPtrSliceReturn, true
}

func writeSliceElemPtrReturnValue(out *strings.Builder, result ast.Expr) bool {
	if !currentFunctionReturnsSliceElemPtr() {
		return false
	}
	return writeSliceElemPtrOptionValue(out, result)
}

func writeReadOnlySliceElemPtrPointerCallArgument(out *strings.Builder, call *ast.CallExpr, index int, arg ast.Expr, expected types.Type) bool {
	if !sourceFunctionParamReadOnly(call, index) {
		return false
	}
	ptr, ok := types.Unalias(expected).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	if elemRustType, ok := sliceElemPtrAddressElemRustType(arg); ok {
		if elemRustType != goTypesTypeToRust(ptr.Elem()) {
			return false
		}
		trackWrapperImports()
		out.WriteString(GetOuterWrapperType())
		out.WriteString("::new(")
		out.WriteString(GetInnerWrapperType())
		out.WriteString("::new((*")
		TranspileExpression(out, arg)
		out.WriteString(".borrow()).clone()))")
		return true
	}
	if elemRustType, ok := arrayElemPtrAddressElemRustType(arg); ok {
		if elemRustType != goTypesTypeToRust(ptr.Elem()) {
			return false
		}
		return writeArrayElemPtrAddressReadOnlyWrapper(out, arg)
	}
	ident, ok := unwrapParens(arg).(*ast.Ident)
	if !ok {
		return false
	}
	info, ok := sliceElemPtrVarInfo(ident.Name)
	if !ok {
		return false
	}
	if info.RustType != "Option<GoSliceElemPtr<"+goTypesTypeToRust(ptr.Elem())+">>" {
		return false
	}
	trackWrapperImports()
	out.WriteString(GetOuterWrapperType())
	out.WriteString("::new(")
	out.WriteString(GetInnerWrapperType())
	out.WriteString("::new((*")
	writeSliceElemPtrBorrow(out, ident, false)
	out.WriteString(").clone()))")
	return true
}

func writeUnsupportedSliceElemPointerHandleValue(out *strings.Builder, rhs ast.Expr, message string) bool {
	if ident, ok := unwrapParens(rhs).(*ast.Ident); ok && isSliceElemPtrVar(ident.Name) {
		WriteWrapperPrefix(out)
		out.WriteString(`unimplemented!("`)
		out.WriteString(message)
		out.WriteString(`")`)
		WriteWrapperSuffix(out)
		return true
	}
	if _, ok := sliceElemPtrAddressElemRustType(rhs); !ok {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString(`unimplemented!("`)
	out.WriteString(message)
	out.WriteString(`")`)
	WriteWrapperSuffix(out)
	return true
}

func writeSliceElemPtrDerefAssignmentValue(out *strings.Builder, target *ast.StarExpr, rhs ast.Expr) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	targetType := typeInfo.GetType(target)
	if targetType == nil {
		return false
	}
	if _, ok := transpiledNamedInterfaceTypeNameFromTypes(targetType); ok {
		rhsType := typeInfo.GetType(rhs)
		if rhsType == nil || !types.AssignableTo(rhsType, targetType) {
			return false
		}
		return writeTranspiledInterfaceHandleClone(out, rhs)
	}
	ident, ok := rhs.(*ast.Ident)
	if !ok || ident.Name != "nil" {
		return false
	}
	if _, ok := types.Unalias(targetType).Underlying().(*types.Pointer); !ok {
		return false
	}
	out.WriteString(zeroValueForTypesType(targetType))
	return true
}

func writeSliceElemPtrDerefRead(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("{ let __v = (*")
	writeSliceElemPtrBorrow(out, ident, false)
	out.WriteString(".as_ref().unwrap()).clone(); __v }")
}

func writeSliceElemPtrDerefLValue(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("(*")
	writeSliceElemPtrBorrow(out, ident, true)
	out.WriteString(".as_mut().unwrap())")
}

func writeSliceElemPtrBorrow(out *strings.Builder, ident *ast.Ident, mutable bool) {
	out.WriteString(RustIdentForUse(ident))
	if info, ok := sliceElemPtrVarInfo(ident.Name); ok && info.WrapLevel == WrapOption {
		out.WriteString(".as_ref().unwrap()")
	}
	if mutable {
		out.WriteString(".borrow_mut()")
	} else {
		out.WriteString(".borrow()")
	}
}

func writeArrayElemPtrDerefRead(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("{ let __v = (*")
	writeArrayElemPtrBorrow(out, ident, false)
	out.WriteString(".as_ref().unwrap()).clone(); __v }")
}

func writeArrayElemPtrDerefLValue(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("(*")
	writeArrayElemPtrBorrow(out, ident, true)
	out.WriteString(".as_mut().unwrap())")
}

func writeArrayElemPtrBorrow(out *strings.Builder, ident *ast.Ident, mutable bool) {
	out.WriteString(RustIdentForUse(ident))
	if info, ok := arrayElemPtrVarInfo(ident.Name); ok && info.WrapLevel == WrapOption {
		out.WriteString(".as_ref().unwrap()")
	}
	if mutable {
		out.WriteString(".borrow_mut()")
	} else {
		out.WriteString(".borrow()")
	}
}

func writeSliceElemPtrFieldHandle(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo) {
	out.WriteString("(*")
	writeSliceElemPtrBorrow(out, ident, false)
	out.WriteString(".as_ref().unwrap())")
	if fieldInfo.IsPromoted {
		for _, embedded := range fieldInfo.EmbeddedPath {
			out.WriteString(".")
			out.WriteString(ToSnakeCase(embedded))
			WriteBorrowMethod(out, false)
			out.WriteString(".as_ref().unwrap()")
		}
	}
	out.WriteString(".")
	out.WriteString(fieldInfo.FieldName)
}

func writeSliceElemPtrFieldSelector(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo, sel *ast.SelectorExpr, ctx ExprContext) bool {
	if !isSliceElemPtrVar(ident.Name) {
		return false
	}
	if ctx == LValue || ctx == AddressOf {
		writeSliceElemPtrFieldHandle(out, ident, fieldInfo)
		return true
	}
	if typeInfoIsPointerExpr(sel) || selectorExpressionKeepsHandle(sel) {
		writeSliceElemPtrFieldHandle(out, ident, fieldInfo)
		out.WriteString(".clone()")
		return true
	}
	out.WriteString("(*")
	if NeedsConcurrentWrapper() {
		out.WriteString("{ let __field = ")
		writeSliceElemPtrFieldHandle(out, ident, fieldInfo)
		out.WriteString(".clone(); __field }")
	} else {
		writeSliceElemPtrFieldHandle(out, ident, fieldInfo)
	}
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	writeSelectorRValueClose(out, sel)
	return true
}

func writeGoPtrLocalFieldSelector(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo, sel *ast.SelectorExpr, ctx ExprContext) bool {
	if !isGoPtrVar(ident.Name) {
		return false
	}
	if fieldInfo.IsPromoted {
		out.WriteString(`unimplemented!("GoPtr local field selector requires promoted-field support")`)
		return true
	}
	if ctx == LValue || ctx == AddressOf {
		writeGoPtrLocalFieldHandle(out, ident, fieldInfo)
		return true
	}
	if typeInfoIsPointerExpr(sel) || selectorExpressionKeepsHandle(sel) {
		writeGoPtrLocalFieldHandle(out, ident, fieldInfo)
		return true
	}
	out.WriteString("(*{ let __ptr_value = ")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(".borrow(); __ptr_value.as_ref().unwrap().")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone() }")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	writeSelectorRValueClose(out, sel)
	return true
}

func writeGoPtrLocalFieldHandle(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo) {
	out.WriteString("{ let __ptr_value = ")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(".borrow(); __ptr_value.as_ref().unwrap().")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone() }")
}
