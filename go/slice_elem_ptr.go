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

type goPtrResultInfo struct {
	elemRustType string
	elemType     types.Type
}

type goPtrCandidate struct {
	info     goPtrResultInfo
	valid    bool
	sawGoPtr bool
}

var currentSliceElemPtrCandidates map[types.Object]string
var currentSliceElemPtrSliceCandidates map[types.Object]string
var currentArrayElemPtrCandidates map[types.Object]arrayElemPtrInfo
var currentGoPtrCandidates map[types.Object]goPtrResultInfo
var currentSliceElemPtrReturn *sliceElemPtrReturnInfo
var currentSliceElemPtrSliceReturn *sliceElemPtrSliceReturnInfo
var currentSliceElemPtrResultInfos map[int]sliceElemPtrReturnInfo
var currentArrayElemPtrReturnInfos map[int]arrayElemPtrInfo
var currentGoPtrReturnInfos map[int]goPtrResultInfo
var currentFuncLitGoPtrParamInfos map[*ast.FuncLit]map[int]goPtrResultInfo
var sliceElemPtrResultInfoInProgress map[*ast.FuncDecl]bool
var arrayElemPtrResultInfoInProgress map[*ast.FuncDecl]bool
var goPtrResultInfoInProgress map[*ast.FuncDecl]bool
var collectingSliceElemPtrFacts bool

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

type goPtrFieldReturnCandidate struct {
	key  string
	info sliceElemPtrFieldInfo
}

type goPtrArrayFieldInfo struct {
	elemRustType string
	ownerPkgPath string
	elemType     types.Type
	arrayLen     int64
}

type sliceElemPtrSliceFieldInfo struct {
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
	oldGoPtr := currentGoPtrCandidates
	currentSliceElemPtrCandidates = collectSliceElemPtrCandidates(body)
	currentSliceElemPtrSliceCandidates = collectSliceElemPtrSliceCandidates(body)
	currentArrayElemPtrCandidates = collectArrayElemPtrCandidates(body)
	currentGoPtrCandidates = nil
	return func() {
		currentSliceElemPtrCandidates = oldPtr
		currentSliceElemPtrSliceCandidates = oldSlice
		currentArrayElemPtrCandidates = oldArray
		currentGoPtrCandidates = oldGoPtr
	}
}

func setSliceElemPtrCandidatesForFunc(fn *ast.FuncDecl) func() {
	if fn == nil {
		return setSliceElemPtrCandidates(nil)
	}
	oldPtr := currentSliceElemPtrCandidates
	oldSlice := currentSliceElemPtrSliceCandidates
	oldArray := currentArrayElemPtrCandidates
	oldGoPtr := currentGoPtrCandidates
	currentSliceElemPtrCandidates = collectSliceElemPtrCandidates(fn.Body)
	currentSliceElemPtrSliceCandidates = collectSliceElemPtrSliceCandidatesForFunc(fn)
	currentArrayElemPtrCandidates = collectArrayElemPtrCandidatesForFunc(fn)
	currentGoPtrCandidates = collectGoPtrCandidatesForFunc(fn)
	return func() {
		currentSliceElemPtrCandidates = oldPtr
		currentSliceElemPtrSliceCandidates = oldSlice
		currentArrayElemPtrCandidates = oldArray
		currentGoPtrCandidates = oldGoPtr
	}
}

func setFuncLitGoPtrParamInfosForFunc(fn *ast.FuncDecl) func() {
	old := currentFuncLitGoPtrParamInfos
	currentFuncLitGoPtrParamInfos = collectFuncLitGoPtrParamInfosForFunc(fn)
	return func() {
		currentFuncLitGoPtrParamInfos = old
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

func goPtrCandidateForDecl(name *ast.Ident) (goPtrResultInfo, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo != nil && currentGoPtrCandidates != nil {
		if obj := typeInfo.GetObject(name); obj != nil {
			info, ok := currentGoPtrCandidates[obj]
			return info, ok
		}
	}
	return goPtrResultInfo{}, false
}

func isGoPtrIdent(name *ast.Ident) bool {
	if name == nil {
		return false
	}
	if isSliceElemPtrVar(name.Name) || isArrayElemPtrVar(name.Name) {
		return false
	}
	if isGoPtrVar(name.Name) {
		return true
	}
	_, ok := goPtrCandidateForDecl(name)
	return ok
}

func sliceElemPtrResultInfosForFunc(fn *ast.FuncDecl) map[int]sliceElemPtrReturnInfo {
	typeInfo := GetTypeInfo()
	if fn == nil || fn.Body == nil || fn.Type == nil || fn.Type.Results == nil || typeInfo == nil {
		return nil
	}
	if sliceElemPtrResultInfoInProgress != nil && sliceElemPtrResultInfoInProgress[fn] {
		return nil
	}
	if sliceElemPtrResultInfoInProgress == nil {
		sliceElemPtrResultInfoInProgress = map[*ast.FuncDecl]bool{}
	}
	sliceElemPtrResultInfoInProgress[fn] = true
	defer delete(sliceElemPtrResultInfoInProgress, fn)

	sig, ok := funcDeclSignatureFromTypeInfo(fn)
	if !ok || sig.Results() == nil || sig.Results().Len() == 0 {
		return nil
	}
	expected := unnamedPointerResultElemTypes(fn, sig)
	if len(expected) == 0 {
		return nil
	}
	candidates := collectSliceElemPtrCandidates(fn.Body)
	namedResultObjects := map[int]types.Object{}
	if typeInfo.info != nil {
		resultIndex := 0
		for _, field := range fn.Type.Results.List {
			if len(field.Names) == 0 {
				resultIndex++
				continue
			}
			for _, name := range field.Names {
				if name != nil && name.Name != "_" {
					if obj := typeInfo.GetObject(name); obj != nil {
						namedResultObjects[resultIndex] = obj
					}
				}
				resultIndex++
			}
		}
	}

	valid := map[int]bool{}
	saw := map[int]bool{}
	infos := map[int]sliceElemPtrReturnInfo{}
	for index := range expected {
		valid[index] = true
	}
	resultCount := sig.Results().Len()
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.ReturnStmt:
			if len(n.Results) == 0 {
				for index, elemRustType := range expected {
					if !valid[index] {
						continue
					}
					obj := namedResultObjects[index]
					if obj == nil {
						valid[index] = false
						continue
					}
					typ, ok := candidates[obj]
					if !ok || typ != elemRustType {
						valid[index] = false
						continue
					}
					elemType, _ := pointerElemType(sig.Results().At(index).Type())
					info := sliceElemPtrReturnInfo{elemRustType: typ, elemType: elemType}
					if prev, exists := infos[index]; exists && prev.elemRustType != info.elemRustType {
						valid[index] = false
						continue
					}
					infos[index] = info
					saw[index] = true
				}
				return true
			}
			if len(n.Results) == 1 && resultCount > 1 {
				if call, ok := unwrapParens(n.Results[0]).(*ast.CallExpr); ok {
					for index, elemRustType := range expected {
						if !valid[index] {
							continue
						}
						info, ok := sliceElemPtrResultInfoForCall(call, index)
						if !ok || info.elemRustType != elemRustType {
							valid[index] = false
							continue
						}
						if prev, exists := infos[index]; exists && prev.elemRustType != info.elemRustType {
							valid[index] = false
							continue
						}
						infos[index] = info
						saw[index] = true
					}
					return true
				}
			}
			if len(n.Results) != resultCount {
				for index := range valid {
					valid[index] = false
				}
				return true
			}
			for index, elemRustType := range expected {
				if !valid[index] {
					continue
				}
				info, valueSaw, ok := sliceElemPtrResultExprInfo(n.Results[index], candidates, typeInfo)
				if !ok {
					valid[index] = false
					continue
				}
				if !valueSaw {
					continue
				}
				if info.elemRustType != elemRustType {
					valid[index] = false
					continue
				}
				if prev, exists := infos[index]; exists && prev.elemRustType != info.elemRustType {
					valid[index] = false
					continue
				}
				infos[index] = info
				saw[index] = true
			}
		}
		return true
	})

	result := map[int]sliceElemPtrReturnInfo{}
	for index, info := range infos {
		if valid[index] && saw[index] {
			result[index] = info
		}
	}
	if len(result) == 0 {
		return nil
	}
	registerSliceElemPtrResultInfosForDecl(fn, result)
	return result
}

func sliceElemPtrResultExprInfo(expr ast.Expr, candidates map[types.Object]string, typeInfo *TypeInfo) (sliceElemPtrReturnInfo, bool, bool) {
	expr = unwrapParens(expr)
	if ident, ok := expr.(*ast.Ident); ok {
		if ident.Name == "nil" {
			return sliceElemPtrReturnInfo{}, false, true
		}
		if typeInfo != nil && candidates != nil {
			obj := typeInfo.GetObject(ident)
			if obj != nil {
				if elemRustType, ok := candidates[obj]; ok {
					elemType, _ := sliceElemPtrPointerElemType(typeInfo.GetType(ident))
					return sliceElemPtrReturnInfo{elemRustType: elemRustType, elemType: elemType}, true, true
				}
			}
		}
		return sliceElemPtrReturnInfo{}, false, false
	}
	if elemType, elemRustType, ok := sliceElemPtrAddressElemType(expr); ok {
		return sliceElemPtrReturnInfo{elemRustType: elemRustType, elemType: elemType}, true, true
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
			return info, true, true
		}
		if info, ok := sliceElemPtrResultInfoForCall(call, 0); ok {
			return info, true, true
		}
	}
	return sliceElemPtrReturnInfo{}, false, false
}

func registerSliceElemPtrResultInfosForDecl(fn *ast.FuncDecl, result map[int]sliceElemPtrReturnInfo) {
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
	if ctx.Package.SliceElemPtrResultFuncs == nil {
		ctx.Package.SliceElemPtrResultFuncs = make(map[*types.Func]map[int]sliceElemPtrReturnInfo)
	}
	if ctx.Package.SliceElemPtrResultFuncNames == nil {
		ctx.Package.SliceElemPtrResultFuncNames = make(map[string]map[int]sliceElemPtrReturnInfo)
	}
	ctx.Package.SliceElemPtrResultFuncs[fnObj] = result
	ctx.Package.SliceElemPtrResultFuncNames[fnObj.FullName()] = result
	if key := methodOverrideKey(fnObj); key != "" {
		ctx.Package.SliceElemPtrResultFuncNames[key] = result
	}
}

func sliceElemPtrResultInfosForFuncObject(fn *types.Func) (map[int]sliceElemPtrReturnInfo, bool) {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil || ctx.Package == nil {
		return nil, false
	}
	if result, ok := ctx.Package.SliceElemPtrResultFuncs[fn]; ok {
		return result, true
	}
	if key := methodOverrideKey(fn); key != "" {
		if result, ok := ctx.Package.SliceElemPtrResultFuncNames[key]; ok {
			return result, true
		}
	}
	if result, ok := ctx.Package.SliceElemPtrResultFuncNames[fn.FullName()]; ok {
		return result, true
	}
	typeInfo := GetTypeInfo()
	if sourceFunctionDeclsByFunc != nil && typeInfo != nil {
		if sourceInfo, ok := sourceFunctionDeclInfoForFunc(fn); ok && sourceInfo.decl != nil && sourceInfo.info == typeInfo.info {
			if result := sliceElemPtrResultInfosForFunc(sourceInfo.decl); len(result) > 0 {
				return result, true
			}
		}
	}
	return nil, false
}

func sliceElemPtrResultInfoForCall(call *ast.CallExpr, resultIndex int) (sliceElemPtrReturnInfo, bool) {
	typeInfo := GetTypeInfo()
	fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
	if !ok {
		return sliceElemPtrReturnInfo{}, false
	}
	infos, ok := sliceElemPtrResultInfosForFuncObject(fn)
	if !ok {
		return sliceElemPtrReturnInfo{}, false
	}
	info, ok := infos[resultIndex]
	return info, ok
}

func arrayElemPtrResultInfosForFunc(fn *ast.FuncDecl) map[int]arrayElemPtrInfo {
	typeInfo := GetTypeInfo()
	if fn == nil || fn.Type == nil || fn.Type.Results == nil || typeInfo == nil {
		return nil
	}
	if arrayElemPtrResultInfoInProgress != nil && arrayElemPtrResultInfoInProgress[fn] {
		return nil
	}
	if arrayElemPtrResultInfoInProgress == nil {
		arrayElemPtrResultInfoInProgress = map[*ast.FuncDecl]bool{}
	}
	arrayElemPtrResultInfoInProgress[fn] = true
	defer delete(arrayElemPtrResultInfoInProgress, fn)

	candidates := currentArrayElemPtrCandidates
	if candidates == nil {
		candidates = collectArrayElemPtrCandidatesForFunc(fn)
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
	if directInfos := arrayElemPtrUnnamedDirectResultInfosForFunc(fn, candidates); len(directInfos) > 0 {
		for index, info := range directInfos {
			result[index] = info
		}
	}
	if len(result) == 0 {
		return nil
	}
	registerArrayElemPtrResultInfosForDecl(fn, result)
	return result
}

func arrayElemPtrUnnamedDirectResultInfosForFunc(fn *ast.FuncDecl, candidates map[types.Object]arrayElemPtrInfo) map[int]arrayElemPtrInfo {
	if fn == nil || fn.Body == nil || fn.Type == nil || fn.Type.Results == nil {
		return nil
	}
	sig, ok := funcDeclSignatureFromTypeInfo(fn)
	if !ok || sig.Results() == nil || sig.Results().Len() == 0 {
		return nil
	}
	expected := unnamedPointerResultElemTypes(fn, sig)
	if len(expected) == 0 {
		return nil
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil
	}

	valid := map[int]bool{}
	saw := map[int]bool{}
	infos := map[int]arrayElemPtrInfo{}
	for index := range expected {
		valid[index] = true
	}
	resultCount := sig.Results().Len()
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
			for index, elemRustType := range expected {
				if !valid[index] {
					continue
				}
				info, valueSaw, ok := arrayElemPtrReturnExprInfo(n.Results[index], candidates, typeInfo)
				if !ok {
					valid[index] = false
					continue
				}
				if !valueSaw {
					continue
				}
				if info.elemRustType != elemRustType {
					valid[index] = false
					continue
				}
				if prev, exists := infos[index]; exists && !sameArrayElemPtrInfo(prev, info) {
					valid[index] = false
					continue
				}
				infos[index] = info
				saw[index] = true
			}
		}
		return true
	})

	result := map[int]arrayElemPtrInfo{}
	for index, info := range infos {
		if valid[index] && saw[index] {
			result[index] = info
		}
	}
	return result
}

func unnamedPointerResultElemTypes(fn *ast.FuncDecl, sig *types.Signature) map[int]string {
	if fn == nil || fn.Type == nil || fn.Type.Results == nil || sig == nil || sig.Results() == nil {
		return nil
	}
	expected := map[int]string{}
	resultIndex := 0
	for _, field := range fn.Type.Results.List {
		count := len(field.Names)
		unnamed := count == 0
		if count == 0 {
			count = 1
		}
		for i := 0; i < count; i++ {
			if unnamed && resultIndex < sig.Results().Len() {
				if elemRustType, ok := sliceElemPtrRustTypeForPointerType(sig.Results().At(resultIndex).Type()); ok {
					expected[resultIndex] = elemRustType
				}
			}
			resultIndex++
		}
	}
	return expected
}

func arrayElemPtrReturnExprInfo(expr ast.Expr, candidates map[types.Object]arrayElemPtrInfo, typeInfo *TypeInfo) (arrayElemPtrInfo, bool, bool) {
	expr = unwrapParens(expr)
	if ident, ok := expr.(*ast.Ident); ok {
		if ident.Name == "nil" {
			return arrayElemPtrInfo{}, false, true
		}
		if typeInfo != nil && candidates != nil {
			obj := typeInfo.GetObject(ident)
			if obj != nil {
				if info, ok := candidates[obj]; ok {
					return info, true, true
				}
			}
		}
		return arrayElemPtrInfo{}, false, false
	}
	if info, ok := arrayElemPtrAddressInfo(expr); ok {
		return info, true, true
	}
	return arrayElemPtrInfo{}, false, false
}

func sameArrayElemPtrInfo(a arrayElemPtrInfo, b arrayElemPtrInfo) bool {
	return a.elemRustType == b.elemRustType && a.arrayLen == b.arrayLen
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
	if key := methodOverrideKey(fnObj); key != "" {
		ctx.Package.ArrayElemPtrResultFuncNames[key] = result
	}
}

func arrayElemPtrResultInfosForFuncObject(fn *types.Func) (map[int]arrayElemPtrInfo, bool) {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil || ctx.Package == nil {
		return nil, false
	}
	if result, ok := ctx.Package.ArrayElemPtrResultFuncs[fn]; ok {
		return result, true
	}
	if key := methodOverrideKey(fn); key != "" {
		if result, ok := ctx.Package.ArrayElemPtrResultFuncNames[key]; ok {
			return result, true
		}
	}
	if result, ok := ctx.Package.ArrayElemPtrResultFuncNames[fn.FullName()]; ok {
		return result, true
	}
	typeInfo := GetTypeInfo()
	if sourceFunctionDeclsByFunc != nil && typeInfo != nil {
		if sourceInfo, ok := sourceFunctionDeclInfoForFunc(fn); ok && sourceInfo.decl != nil && sourceInfo.info == typeInfo.info {
			if result := arrayElemPtrResultInfosForFunc(sourceInfo.decl); len(result) > 0 {
				return result, true
			}
		}
	}
	return nil, false
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

func goPtrResultInfosForFunc(fn *ast.FuncDecl) map[int]goPtrResultInfo {
	typeInfo := GetTypeInfo()
	if fn == nil || fn.Body == nil || fn.Type == nil || fn.Type.Results == nil || typeInfo == nil {
		return nil
	}
	if result, ok := syncAtomicPointerMethodGoPtrResultInfosForDecl(fn); ok {
		registerGoPtrResultInfosForDecl(fn, result)
		return result
	}
	if goPtrResultInfoInProgress != nil && goPtrResultInfoInProgress[fn] {
		return nil
	}
	if goPtrResultInfoInProgress == nil {
		goPtrResultInfoInProgress = map[*ast.FuncDecl]bool{}
	}
	goPtrResultInfoInProgress[fn] = true
	defer delete(goPtrResultInfoInProgress, fn)

	sig, ok := funcDeclSignatureFromTypeInfo(fn)
	if !ok || sig.Results() == nil || sig.Results().Len() == 0 {
		return nil
	}
	expected := goPtrPointerResultElemTypes(sig)
	if len(expected) == 0 {
		return nil
	}
	candidates := currentGoPtrCandidates
	if candidates == nil {
		candidates = collectGoPtrCandidatesForFunc(fn)
	}
	namedResultObjects := map[int]types.Object{}
	if fn.Type != nil && fn.Type.Results != nil && typeInfo.info != nil {
		resultIndex := 0
		for _, field := range fn.Type.Results.List {
			if len(field.Names) == 0 {
				resultIndex++
				continue
			}
			for _, name := range field.Names {
				if name != nil && name.Name != "_" {
					if obj := typeInfo.GetObject(name); obj != nil {
						namedResultObjects[resultIndex] = obj
					}
				}
				resultIndex++
			}
		}
	}
	valid := map[int]bool{}
	saw := map[int]bool{}
	infos := map[int]goPtrResultInfo{}
	fieldReturnCandidates := map[int][]goPtrFieldReturnCandidate{}
	for index := range expected {
		valid[index] = true
	}
	resultCount := sig.Results().Len()
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.ReturnStmt:
			if len(n.Results) == 0 {
				for index, elemRustType := range expected {
					if !valid[index] {
						continue
					}
					obj := namedResultObjects[index]
					if obj == nil {
						valid[index] = false
						continue
					}
					info, ok := candidates[obj]
					if !ok {
						valid[index] = false
						continue
					}
					if goPtrResultElemRustType(info) != elemRustType {
						valid[index] = false
						continue
					}
					if prev, exists := infos[index]; exists && goPtrResultElemRustType(prev) != goPtrResultElemRustType(info) {
						valid[index] = false
						continue
					}
					infos[index] = info
					saw[index] = true
				}
				return true
			}
			if len(n.Results) == 1 && resultCount > 1 {
				if call, ok := unwrapParens(n.Results[0]).(*ast.CallExpr); ok {
					for index, elemRustType := range expected {
						if !valid[index] {
							continue
						}
						info, ok := goPtrResultInfoForCall(call, index)
						if !ok {
							valid[index] = false
							continue
						}
						if goPtrResultElemRustType(info) != elemRustType {
							valid[index] = false
							continue
						}
						if prev, exists := infos[index]; exists && goPtrResultElemRustType(prev) != goPtrResultElemRustType(info) {
							valid[index] = false
							continue
						}
						infos[index] = info
						saw[index] = true
					}
					return true
				}
			}
			if len(n.Results) != resultCount {
				for index := range valid {
					valid[index] = false
				}
				return true
			}
			for index, elemRustType := range expected {
				if !valid[index] {
					continue
				}
				info, valueSaw, ok := goPtrReturnExprInfo(n.Results[index], index, candidates, typeInfo)
				if !ok {
					valid[index] = false
					continue
				}
				if !valueSaw {
					if candidate, ok := goPtrFieldReturnCandidateForExpr(n.Results[index]); ok {
						fieldInfo := goPtrResultInfo{elemRustType: candidate.info.elemRustType, elemType: candidate.info.elemType}
						if goPtrResultElemRustType(fieldInfo) == elemRustType {
							fieldReturnCandidates[index] = append(fieldReturnCandidates[index], candidate)
						}
					}
					continue
				}
				if goPtrResultElemRustType(info) != elemRustType {
					valid[index] = false
					continue
				}
				if prev, exists := infos[index]; exists && goPtrResultElemRustType(prev) != goPtrResultElemRustType(info) {
					valid[index] = false
					continue
				}
				infos[index] = info
				saw[index] = true
			}
		}
		return true
	})

	result := map[int]goPtrResultInfo{}
	for index, info := range infos {
		if valid[index] && saw[index] {
			result[index] = info
			for _, candidate := range fieldReturnCandidates[index] {
				fieldInfo := goPtrResultInfo{elemRustType: candidate.info.elemRustType, elemType: candidate.info.elemType}
				if collectingSliceElemPtrFacts && goPtrResultElemCompatible(fieldInfo, info) {
					registerSliceElemPtrFieldInfoForKey(candidate.key, candidate.info)
				}
			}
		}
	}
	if len(result) == 0 {
		return nil
	}
	registerGoPtrResultInfosForDecl(fn, result)
	return result
}

func goPtrFieldReturnCandidateForExpr(expr ast.Expr) (goPtrFieldReturnCandidate, bool) {
	sel, ok := unwrapParens(expr).(*ast.SelectorExpr)
	if !ok {
		return goPtrFieldReturnCandidate{}, false
	}
	key, info, ok := sliceElemPtrFieldKeyForSelector(sel)
	if !ok {
		return goPtrFieldReturnCandidate{}, false
	}
	return goPtrFieldReturnCandidate{key: key, info: info}, true
}

func goPtrPointerResultElemTypes(sig *types.Signature) map[int]string {
	if sig == nil || sig.Results() == nil {
		return nil
	}
	result := map[int]string{}
	for i := 0; i < sig.Results().Len(); i++ {
		info, ok := goPtrInfoForPointerType(sig.Results().At(i).Type())
		if !ok {
			continue
		}
		result[i] = goPtrResultElemRustType(info)
	}
	return result
}

func registerGoPtrResultInfosForDecl(fn *ast.FuncDecl, result map[int]goPtrResultInfo) {
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
	if ctx.Package.GoPtrReturnFuncs == nil {
		ctx.Package.GoPtrReturnFuncs = make(map[*types.Func]map[int]goPtrResultInfo)
	}
	if ctx.Package.GoPtrReturnFuncNames == nil {
		ctx.Package.GoPtrReturnFuncNames = make(map[string]map[int]goPtrResultInfo)
	}
	ctx.Package.GoPtrReturnFuncs[fnObj] = result
	ctx.Package.GoPtrReturnFuncNames[fnObj.FullName()] = result
	if key := methodOverrideKey(fnObj); key != "" {
		ctx.Package.GoPtrReturnFuncNames[key] = result
	}
	if ctx.Session != nil {
		if ctx.Session.GoPtrReturnFuncNames == nil {
			ctx.Session.GoPtrReturnFuncNames = make(map[string]map[int]goPtrResultInfo)
		}
		ctx.Session.GoPtrReturnFuncNames[fnObj.FullName()] = result
		if key := methodOverrideKey(fnObj); key != "" {
			ctx.Session.GoPtrReturnFuncNames[key] = result
		}
	}
}

func goPtrResultInfosForFuncObject(fn *types.Func) (map[int]goPtrResultInfo, bool) {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil || ctx.Package == nil {
		return nil, false
	}
	if result, ok := ctx.Package.GoPtrReturnFuncs[fn]; ok {
		return result, true
	}
	if key := methodOverrideKey(fn); key != "" {
		if result, ok := ctx.Package.GoPtrReturnFuncNames[key]; ok {
			return result, true
		}
	}
	if result, ok := ctx.Package.GoPtrReturnFuncNames[fn.FullName()]; ok {
		return result, true
	}
	if ctx.Session != nil {
		if result, ok := ctx.Session.GoPtrReturnFuncNames[fn.FullName()]; ok {
			return result, true
		}
	}
	if result, ok := goPtrResultInfosForLocalInterfaceMethod(fn); ok {
		return result, true
	}
	if sourceFunctionDeclsByFunc != nil {
		if sourceFn, sourceInfo, ok := sourceFunctionDeclObjectForFunc(fn); ok {
			if result, ok := goPtrResultInfosForSourceDecl(sourceFn, sourceInfo); ok {
				return result, true
			}
		}
	}
	if result, ok := syncAtomicPointerMethodGoPtrResultInfos(fn); ok {
		return result, true
	}
	return nil, false
}

func registeredGoPtrResultInfosForFuncObject(fn *types.Func) (map[int]goPtrResultInfo, bool) {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil {
		return nil, false
	}
	if ctx.Package != nil {
		if result, ok := ctx.Package.GoPtrReturnFuncs[fn]; ok {
			return result, true
		}
		if key := methodOverrideKey(fn); key != "" {
			if result, ok := ctx.Package.GoPtrReturnFuncNames[key]; ok {
				return result, true
			}
		}
		if result, ok := ctx.Package.GoPtrReturnFuncNames[fn.FullName()]; ok {
			return result, true
		}
	}
	if ctx.Session != nil {
		if key := methodOverrideKey(fn); key != "" {
			if result, ok := ctx.Session.GoPtrReturnFuncNames[key]; ok {
				return result, true
			}
		}
		if result, ok := ctx.Session.GoPtrReturnFuncNames[fn.FullName()]; ok {
			return result, true
		}
	}
	return nil, false
}

func registeredGoPtrResultInfosForDecl(fn *ast.FuncDecl) (map[int]goPtrResultInfo, bool) {
	fnObj, ok := sliceElemPtrReturnFuncObject(fn)
	if !ok {
		return nil, false
	}
	return registeredGoPtrResultInfosForFuncObject(fnObj)
}

func goPtrResultInfosForLocalInterfaceMethod(fn *types.Func) (map[int]goPtrResultInfo, bool) {
	if fn == nil {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return nil, false
	}
	for _, name := range typeInfo.pkg.Scope().Names() {
		obj, ok := typeInfo.pkg.Scope().Lookup(name).(*types.TypeName)
		if !ok {
			continue
		}
		named, ok := types.Unalias(obj.Type()).(*types.Named)
		if !ok {
			continue
		}
		iface, ok := types.Unalias(named.Underlying()).(*types.Interface)
		if !ok {
			continue
		}
		iface.Complete()
		for i := 0; i < iface.NumMethods(); i++ {
			if iface.Method(i) != fn {
				continue
			}
			return goPtrResultInfosForInterfaceMethod(name, fn.Name())
		}
	}
	return nil, false
}

func goPtrResultInfosForInterfaceMethod(ifaceName string, methodName string) (map[int]goPtrResultInfo, bool) {
	if ifaceName == "" || methodName == "" {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return nil, false
	}
	iface := localInterfaceTypesByName(ifaceName)
	if iface == nil {
		return nil, false
	}
	iface.Complete()
	var result map[int]goPtrResultInfo
	for _, typeName := range typeInfo.pkg.Scope().Names() {
		if typeName == ifaceName {
			continue
		}
		obj, ok := typeInfo.pkg.Scope().Lookup(typeName).(*types.TypeName)
		if !ok {
			continue
		}
		named, ok := types.Unalias(obj.Type()).(*types.Named)
		if !ok {
			continue
		}
		if _, isInterface := types.Unalias(named.Underlying()).(*types.Interface); isInterface {
			continue
		}
		if !types.Implements(named, iface) && !types.Implements(types.NewPointer(named), iface) {
			continue
		}
		method := methodDeclByName(methodsForReceiverType(typeName), methodName)
		if method == nil {
			continue
		}
		infos := goPtrResultInfosForFunc(method)
		if len(infos) == 0 {
			continue
		}
		if result == nil {
			result = make(map[int]goPtrResultInfo)
		}
		for index, info := range infos {
			if previous, exists := result[index]; exists && goPtrResultElemRustType(previous) != goPtrResultElemRustType(info) {
				return nil, false
			}
			result[index] = info
		}
	}
	if len(result) == 0 {
		return nil, false
	}
	return result, true
}

func goPtrResultInfosForSourceDecl(sourceFn *types.Func, sourceInfo sourceFunctionDeclInfo) (map[int]goPtrResultInfo, bool) {
	if sourceInfo.decl == nil || sourceInfo.info == nil {
		return nil, false
	}
	prevTypeInfo := GetTypeInfo()
	prevReceiver := currentReceiver
	prevReceiverObject := currentReceiverObject
	prevReceiverType := currentReceiverType
	prevGoPtrCandidates := currentGoPtrCandidates
	defer func() {
		SetTypeInfo(prevTypeInfo)
		currentReceiver = prevReceiver
		currentReceiverObject = prevReceiverObject
		currentReceiverType = prevReceiverType
		currentGoPtrCandidates = prevGoPtrCandidates
	}()

	sourceTypeInfo := &TypeInfo{info: sourceInfo.info}
	if sourceFn != nil {
		sourceTypeInfo.pkg = sourceFn.Pkg()
	}
	SetTypeInfo(sourceTypeInfo)
	currentReceiver = ""
	currentReceiverObject = nil
	currentReceiverType = ""
	if sourceInfo.decl.Recv != nil && len(sourceInfo.decl.Recv.List) > 0 {
		recv := sourceInfo.decl.Recv.List[0]
		if len(recv.Names) > 0 {
			setCurrentReceiverFromIdent(recv.Names[0])
		}
		currentReceiverType = getReceiverType(recv.Type)
	}
	currentGoPtrCandidates = nil

	result := goPtrResultInfosForFunc(sourceInfo.decl)
	if len(result) == 0 {
		return nil, false
	}
	return result, true
}

func goPtrResultInfoForCall(call *ast.CallExpr, resultIndex int) (goPtrResultInfo, bool) {
	typeInfo := GetTypeInfo()
	fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
	if !ok {
		return goPtrResultInfo{}, false
	}
	infos, ok := goPtrResultInfosForFuncObject(fn)
	if !ok {
		return goPtrResultInfo{}, false
	}
	info, ok := infos[resultIndex]
	if ok {
		info = refineGoPtrResultInfoFromCallType(call, resultIndex, info, typeInfo)
	}
	return info, ok
}

func refineGoPtrResultInfoFromCallType(call *ast.CallExpr, resultIndex int, info goPtrResultInfo, typeInfo *TypeInfo) goPtrResultInfo {
	if call == nil || typeInfo == nil || resultIndex < 0 {
		return info
	}
	typ := typeInfo.GetType(call)
	if typ == nil {
		return info
	}
	if tuple, ok := types.Unalias(typ).(*types.Tuple); ok {
		if resultIndex >= tuple.Len() {
			return info
		}
		typ = tuple.At(resultIndex).Type()
	} else if resultIndex != 0 {
		return info
	}
	ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	if !ok {
		return info
	}
	if refined, ok := goPtrInfoForPointerType(ptr); ok {
		return refined
	}
	return info
}

func writeSliceElemPtrFuncDeclResultTypes(out *strings.Builder, fn *ast.FuncDecl) bool {
	resultInfos := sliceElemPtrResultInfosForFunc(fn)
	if len(resultInfos) == 0 {
		return false
	}
	arrayResultInfos := arrayElemPtrResultInfosForFunc(fn)
	goPtrResultInfos := goPtrResultInfosForFunc(fn)
	sig, ok := funcDeclSignatureFromTypeInfo(fn)
	if !ok || sig.Results() == nil || sig.Results().Len() == 0 {
		return false
	}
	results := sig.Results()
	out.WriteString(" -> ")
	if results.Len() == 1 {
		if info, ok := resultInfos[0]; ok {
			writeSliceElemPtrResultType(out, info)
		} else if info, ok := arrayResultInfos[0]; ok {
			out.WriteString(arrayElemPtrOptionRustType(info))
		} else if info, ok := goPtrResultInfos[0]; ok {
			writeGoPtrResultType(out, info)
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
			writeSliceElemPtrResultType(out, info)
		} else if info, ok := arrayResultInfos[i]; ok {
			out.WriteString(arrayElemPtrOptionRustType(info))
		} else if info, ok := goPtrResultInfos[i]; ok {
			writeGoPtrResultType(out, info)
		} else {
			out.WriteString(goTypesReturnTypeToRust(results.At(i).Type()))
		}
	}
	out.WriteString(")")
	return true
}

func writeSliceElemPtrResultType(out *strings.Builder, info sliceElemPtrReturnInfo) {
	NeedSliceElemPtr()
	out.WriteString("Option<GoSliceElemPtr<")
	out.WriteString(info.elemRustType)
	out.WriteString(">>")
}

func writeArrayElemPtrFuncDeclResultTypes(out *strings.Builder, fn *ast.FuncDecl) bool {
	resultInfos := arrayElemPtrResultInfosForFunc(fn)
	if len(resultInfos) == 0 {
		return false
	}
	goPtrResultInfos := goPtrResultInfosForFunc(fn)
	sig, ok := funcDeclSignatureFromTypeInfo(fn)
	if !ok || sig.Results() == nil || sig.Results().Len() == 0 {
		return false
	}
	results := sig.Results()
	out.WriteString(" -> ")
	if results.Len() == 1 {
		if info, ok := resultInfos[0]; ok {
			out.WriteString(arrayElemPtrOptionRustType(info))
		} else if info, ok := goPtrResultInfos[0]; ok {
			writeGoPtrResultType(out, info)
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
		} else if info, ok := goPtrResultInfos[i]; ok {
			writeGoPtrResultType(out, info)
		} else {
			out.WriteString(goTypesReturnTypeToRust(results.At(i).Type()))
		}
	}
	out.WriteString(")")
	return true
}

func writeGoPtrFuncDeclResultTypes(out *strings.Builder, fn *ast.FuncDecl) bool {
	resultInfos, ok := registeredGoPtrResultInfosForDecl(fn)
	if !ok || len(resultInfos) == 0 {
		resultInfos = goPtrResultInfosForFunc(fn)
	}
	if len(resultInfos) == 0 {
		return false
	}
	sig, ok := funcDeclSignatureFromTypeInfo(fn)
	if !ok || sig.Results() == nil || sig.Results().Len() == 0 {
		return false
	}
	return writeGoPtrFuncResultTypesFromInfos(out, sig, resultInfos)
}

func writeGoPtrFuncResultTypesFromInfos(out *strings.Builder, sig *types.Signature, resultInfos map[int]goPtrResultInfo) bool {
	if len(resultInfos) == 0 || sig == nil || sig.Results() == nil || sig.Results().Len() == 0 {
		return false
	}
	results := sig.Results()
	out.WriteString(" -> ")
	if results.Len() == 1 {
		if info, ok := resultInfos[0]; ok {
			writeGoPtrResultType(out, info)
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
			writeGoPtrResultType(out, info)
		} else {
			out.WriteString(goTypesReturnTypeToRust(results.At(i).Type()))
		}
	}
	out.WriteString(")")
	return true
}

func writeGoPtrResultType(out *strings.Builder, info goPtrResultInfo) {
	NeedSliceElemPtr()
	out.WriteString("GoPtr<")
	out.WriteString(goPtrResultElemRustType(info))
	out.WriteString(">")
}

func goPtrArrayFieldRustType(info goPtrArrayFieldInfo) string {
	NeedSliceElemPtr()
	return goTypesWrappedRustType("[GoPtr<" + goPtrArrayFieldElemRustType(info) + ">; " + strconv.FormatInt(info.arrayLen, 10) + "]")
}

func writeGoPtrArrayFieldDefaultValue(out *strings.Builder, info goPtrArrayFieldInfo) {
	NeedSliceElemPtr()
	WriteWrapperPrefix(out)
	out.WriteString("std::array::from_fn(|_| GoPtr::nil())")
	WriteWrapperSuffix(out)
}

func goPtrArrayFieldElemRustType(info goPtrArrayFieldInfo) string {
	if info.elemType != nil {
		if rustType := goTypesCollectionElemTypeToRust(info.elemType); rustType != "" {
			return rustType
		}
	}
	return info.elemRustType
}

func goPtrResultElemRustType(info goPtrResultInfo) string {
	if info.elemType != nil {
		return goTypesCollectionElemTypeToRust(coreType(info.elemType))
	}
	return info.elemRustType
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
	registerGoPtrParamsFromFiles(files)
	registerFunctionValueGoPtrParamsFromFiles(files)
}

func registerSliceElemPtrFactsFromFiles(files []*ast.File) {
	prevCollectingFacts := collectingSliceElemPtrFacts
	collectingSliceElemPtrFacts = true
	defer func() { collectingSliceElemPtrFacts = prevCollectingFacts }()

	registerSliceElemPtrReturnsFromFiles(files)
	registerSliceElemPtrFieldsFromFiles(files)
	// Field-backed pointer result facts depend on the field prepass above,
	// while field assignment facts can themselves depend on earlier result facts.
	registerSliceElemPtrReturnsFromFiles(files)
	// The second return/parameter pass can discover GoPtr parameters from
	// field-backed call arguments. Re-run field collection so assignments from
	// those parameters promote their destination fields before declarations emit.
	registerSliceElemPtrFieldsFromFiles(files)
}

func registerSliceElemPtrFieldsFromFiles(files []*ast.File) {
	for registerSliceElemPtrFieldsFromFilesPass(files) {
	}
	recordGeneratedGoPtrFieldsForRegisteredFieldInfos()
	registerGoPtrParamsFromFiles(files)
	registerFunctionValueGoPtrParamsFromFiles(files)
}

func registerSliceElemPtrFieldsFromFilesPass(files []*ast.File) bool {
	typeInfo := GetTypeInfo()
	ctx := GetTranspileContext()
	if typeInfo == nil || typeInfo.info == nil || ctx == nil || ctx.Package == nil {
		return false
	}
	if ctx.Package.SliceElemPtrFields == nil {
		ctx.Package.SliceElemPtrFields = make(map[string]sliceElemPtrFieldInfo)
	}
	if ctx.Package.SliceElemPtrSliceFields == nil {
		ctx.Package.SliceElemPtrSliceFields = make(map[string]sliceElemPtrSliceFieldInfo)
	}
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
			currentFn, _ := sliceElemPtrReturnFuncObject(fn)
			localCandidates := collectSliceElemPtrCandidates(fn.Body)
			arrayCandidates := collectArrayElemPtrCandidatesForFunc(fn)
			goPtrCandidates := collectGoPtrCandidatesForFunc(fn)
			ast.Inspect(fn.Body, func(node ast.Node) bool {
				switch n := node.(type) {
				case *ast.FuncLit:
					return true
				case *ast.AssignStmt:
					for i, lhs := range n.Lhs {
						if indexExpr, ok := unwrapParens(lhs).(*ast.IndexExpr); ok {
							rhs := assignmentRHSForLHS(n, i)
							if rhs != nil {
								if registerGoPtrArrayIndexAssignment(indexExpr, rhs, currentFn, localCandidates, arrayCandidates, goPtrCandidates) {
									changed = true
								}
								if registerSliceElemPtrSliceFieldAssignment(indexExpr, rhs, currentFn, localCandidates, arrayCandidates, goPtrCandidates) {
									changed = true
								}
							}
							continue
						}
						sel, ok := unwrapParens(lhs).(*ast.SelectorExpr)
						if !ok {
							continue
						}
						rhs := assignmentRHSForLHS(n, i)
						if rhs == nil {
							if len(n.Rhs) == 1 {
								if call, ok := unwrapParens(n.Rhs[0]).(*ast.CallExpr); ok {
									if registerSliceElemPtrFieldAssignmentCallResult(sel, call, i) {
										changed = true
									}
								}
							}
							continue
						}
						if registerSliceElemPtrFieldAssignment(sel, rhs, currentFn, localCandidates, arrayCandidates, goPtrCandidates) {
							changed = true
						}
					}
				case *ast.CompositeLit:
					if registerSliceElemPtrCompositeLiteralFields(n, currentFn, localCandidates, arrayCandidates, goPtrCandidates) {
						changed = true
					}
				}
				return true
			})
		}
	}
	return changed
}

func recordGeneratedGoPtrFieldsForRegisteredFieldInfos() {
	ctx := GetTranspileContext()
	if ctx == nil || ctx.Package == nil {
		return
	}
	for key := range ctx.Package.SliceElemPtrFields {
		recordGeneratedGoPtrFieldForKey(key)
	}
}

func registerSliceElemPtrCompositeLiteralFields(lit *ast.CompositeLit, currentFn *types.Func, localCandidates map[types.Object]string, arrayCandidates map[types.Object]arrayElemPtrInfo, goPtrCandidates map[types.Object]goPtrResultInfo) bool {
	typeInfo := GetTypeInfo()
	if lit == nil || typeInfo == nil {
		return false
	}
	typ := typeInfo.GetType(lit)
	if typ == nil {
		return false
	}
	if ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer); ok {
		typ = ptr.Elem()
	}
	structUnder, ok := types.Unalias(typ).Underlying().(*types.Struct)
	if !ok {
		return false
	}

	allPositional := true
	for _, elt := range lit.Elts {
		if _, ok := elt.(*ast.KeyValueExpr); ok {
			allPositional = false
			break
		}
	}
	changed := false
	if allPositional {
		for i, elt := range lit.Elts {
			if i >= structUnder.NumFields() {
				break
			}
			if registerSliceElemPtrCompositeLiteralField(typ, structUnder.Field(i), elt, currentFn, localCandidates, arrayCandidates, goPtrCandidates) {
				changed = true
			}
		}
		return changed
	}
	for _, elt := range lit.Elts {
		kv, ok := elt.(*ast.KeyValueExpr)
		if !ok {
			continue
		}
		key, ok := kv.Key.(*ast.Ident)
		if !ok {
			continue
		}
		field := findTypesStructField(structUnder, key.Name)
		if registerSliceElemPtrCompositeLiteralField(typ, field, kv.Value, currentFn, localCandidates, arrayCandidates, goPtrCandidates) {
			changed = true
		}
	}
	return changed
}

func registerSliceElemPtrCompositeLiteralField(owner types.Type, field *types.Var, value ast.Expr, currentFn *types.Func, localCandidates map[types.Object]string, arrayCandidates map[types.Object]arrayElemPtrInfo, goPtrCandidates map[types.Object]goPtrResultInfo) bool {
	if field == nil {
		return false
	}
	ptr, ok := types.Unalias(field.Type()).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	elemType := coreType(ptr.Elem())
	fieldInfo := sliceElemPtrFieldInfo{
		elemRustType: goTypesCollectionElemTypeToRust(elemType),
		elemType:     elemType,
		ownerPkgPath: sliceElemPtrFieldOwnerPkgPath(owner),
	}
	rhsElemType, rhsElemRustType, ok := sliceElemPtrValueElemType(value, currentFn, localCandidates, arrayCandidates, goPtrCandidates)
	if !ok || !sliceElemPtrElemCompatible(rhsElemType, rhsElemRustType, fieldInfo) {
		return false
	}
	key := sliceElemPtrFieldKeyForOwnerType(owner, field.Name())
	return registerSliceElemPtrFieldInfoForKey(key, fieldInfo)
}

func registerSliceElemPtrFieldInfoForKey(key string, fieldInfo sliceElemPtrFieldInfo) bool {
	ctx := GetTranspileContext()
	if key == "" || ctx == nil || ctx.Package == nil {
		return false
	}
	_, existed := ctx.Package.SliceElemPtrFields[key]
	ctx.Package.SliceElemPtrFields[key] = fieldInfo
	recordGeneratedGoPtrFieldForKey(key)
	if ctx.Session != nil {
		if ctx.Session.SliceElemPtrFields == nil {
			ctx.Session.SliceElemPtrFields = make(map[string]sliceElemPtrFieldInfo)
		}
		ctx.Session.SliceElemPtrFields[key] = fieldInfo
	}
	return !existed
}

func registerSliceElemPtrFieldAssignmentCallResult(sel *ast.SelectorExpr, call *ast.CallExpr, resultIndex int) bool {
	key, fieldInfo, ok := sliceElemPtrFieldKeyForSelector(sel)
	if !ok || call == nil {
		return false
	}
	var rhsElemType types.Type
	rhsElemRustType := ""
	if resultIndex == 0 {
		if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
			rhsElemType = info.elemType
			rhsElemRustType = info.elemRustType
		}
	}
	if info, ok := arrayElemPtrResultInfoForCall(call, resultIndex); ok {
		rhsElemType = nil
		rhsElemRustType = info.elemRustType
	}
	if rhsElemRustType == "" || !sliceElemPtrElemCompatible(rhsElemType, rhsElemRustType, fieldInfo) {
		return false
	}
	return registerSliceElemPtrFieldInfoForKey(key, fieldInfo)
}

func registerSliceElemPtrFieldAssignment(sel *ast.SelectorExpr, rhs ast.Expr, currentFn *types.Func, localCandidates map[types.Object]string, arrayCandidates map[types.Object]arrayElemPtrInfo, goPtrCandidates map[types.Object]goPtrResultInfo) bool {
	key, fieldInfo, ok := sliceElemPtrFieldKeyForSelector(sel)
	if !ok {
		return false
	}
	rhsElemType, rhsElemRustType, ok := sliceElemPtrValueElemType(rhs, currentFn, localCandidates, arrayCandidates, goPtrCandidates)
	if !ok || !sliceElemPtrElemCompatible(rhsElemType, rhsElemRustType, fieldInfo) {
		return false
	}
	return registerSliceElemPtrFieldInfoForKey(key, fieldInfo)
}

func registerGoPtrArrayFieldAssignment(indexExpr *ast.IndexExpr, rhs ast.Expr, currentFn *types.Func, localCandidates map[types.Object]string, arrayCandidates map[types.Object]arrayElemPtrInfo, goPtrCandidates map[types.Object]goPtrResultInfo) bool {
	key, fieldInfo, ok := goPtrArrayFieldKeyForIndexExpr(indexExpr)
	if !ok {
		return false
	}
	rhsElemType, rhsElemRustType, ok := sliceElemPtrValueElemType(rhs, currentFn, localCandidates, arrayCandidates, goPtrCandidates)
	if !ok || !goPtrArrayFieldElemCompatible(rhsElemType, rhsElemRustType, fieldInfo) {
		return false
	}
	return registerGoPtrArrayFieldInfoForKey(key, fieldInfo)
}

func registerGoPtrArrayIndexAssignment(indexExpr *ast.IndexExpr, rhs ast.Expr, currentFn *types.Func, localCandidates map[types.Object]string, arrayCandidates map[types.Object]arrayElemPtrInfo, goPtrCandidates map[types.Object]goPtrResultInfo) bool {
	changed := false
	if registerGoPtrArrayFieldAssignment(indexExpr, rhs, currentFn, localCandidates, arrayCandidates, goPtrCandidates) {
		changed = true
	}
	if registerGoPtrArrayLocalAssignment(indexExpr, rhs, currentFn, localCandidates, arrayCandidates, goPtrCandidates) {
		changed = true
	}
	return changed
}

func registerGoPtrArrayLocalAssignment(indexExpr *ast.IndexExpr, rhs ast.Expr, currentFn *types.Func, localCandidates map[types.Object]string, arrayCandidates map[types.Object]arrayElemPtrInfo, goPtrCandidates map[types.Object]goPtrResultInfo) bool {
	obj, fieldInfo, ok := goPtrArrayLocalObjectForIndexExpr(indexExpr)
	if !ok {
		return false
	}
	rhsElemType, rhsElemRustType, ok := sliceElemPtrValueElemType(rhs, currentFn, localCandidates, arrayCandidates, goPtrCandidates)
	if !ok || !goPtrArrayFieldElemCompatible(rhsElemType, rhsElemRustType, fieldInfo) {
		return false
	}
	return registerGoPtrArrayLocalInfoForObject(obj, fieldInfo)
}

func registerSliceElemPtrSliceFieldAssignment(indexExpr *ast.IndexExpr, rhs ast.Expr, currentFn *types.Func, localCandidates map[types.Object]string, arrayCandidates map[types.Object]arrayElemPtrInfo, goPtrCandidates map[types.Object]goPtrResultInfo) bool {
	key, fieldInfo, ok := sliceElemPtrSliceFieldKeyForIndexExpr(indexExpr)
	if !ok {
		return false
	}
	rhsElemType, rhsElemRustType, ok := sliceElemPtrValueElemType(rhs, currentFn, localCandidates, arrayCandidates, goPtrCandidates)
	if !ok || !sliceElemPtrSliceFieldElemCompatible(rhsElemType, rhsElemRustType, fieldInfo) {
		return false
	}
	return registerSliceElemPtrSliceFieldInfoForKey(key, fieldInfo)
}

func registerGoPtrArrayFieldInfoForKey(key string, fieldInfo goPtrArrayFieldInfo) bool {
	ctx := GetTranspileContext()
	if key == "" || ctx == nil || ctx.Package == nil {
		return false
	}
	_, existed := ctx.Package.GoPtrArrayFields[key]
	ctx.Package.GoPtrArrayFields[key] = fieldInfo
	if ctx.Session != nil {
		if ctx.Session.GoPtrArrayFields == nil {
			ctx.Session.GoPtrArrayFields = make(map[string]goPtrArrayFieldInfo)
		}
		ctx.Session.GoPtrArrayFields[key] = fieldInfo
	}
	return !existed
}

func registerGoPtrArrayLocalInfoForObject(obj types.Object, fieldInfo goPtrArrayFieldInfo) bool {
	ctx := GetTranspileContext()
	if obj == nil || ctx == nil || ctx.Package == nil {
		return false
	}
	if ctx.Package.GoPtrArrayLocalObjs == nil {
		ctx.Package.GoPtrArrayLocalObjs = make(map[types.Object]goPtrArrayFieldInfo)
	}
	if existing, ok := ctx.Package.GoPtrArrayLocalObjs[obj]; ok {
		if goPtrArrayFieldElemCompatible(existing.elemType, existing.elemRustType, fieldInfo) && existing.arrayLen == fieldInfo.arrayLen {
			return false
		}
	}
	ctx.Package.GoPtrArrayLocalObjs[obj] = fieldInfo
	return true
}

func registerSliceElemPtrSliceFieldInfoForKey(key string, fieldInfo sliceElemPtrSliceFieldInfo) bool {
	ctx := GetTranspileContext()
	if key == "" || ctx == nil || ctx.Package == nil {
		return false
	}
	_, existed := ctx.Package.SliceElemPtrSliceFields[key]
	ctx.Package.SliceElemPtrSliceFields[key] = fieldInfo
	if ctx.Session != nil {
		if ctx.Session.SliceElemPtrSliceFields == nil {
			ctx.Session.SliceElemPtrSliceFields = make(map[string]sliceElemPtrSliceFieldInfo)
		}
		ctx.Session.SliceElemPtrSliceFields[key] = fieldInfo
	}
	return !existed
}

func sliceElemPtrValueElemType(expr ast.Expr, currentFn *types.Func, localCandidates map[types.Object]string, arrayCandidates map[types.Object]arrayElemPtrInfo, goPtrCandidates map[types.Object]goPtrResultInfo) (types.Type, string, bool) {
	if elemType, elemRustType, ok := sliceElemPtrAddressElemType(expr); ok {
		return elemType, elemRustType, true
	}
	if elemRustType, ok := arrayElemPtrAddressElemRustType(expr); ok {
		return nil, elemRustType, true
	}
	if call, ok := unwrapParens(expr).(*ast.CallExpr); ok {
		if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
			return info.elemType, info.elemRustType, true
		}
		if info, ok := arrayElemPtrResultInfoForCall(call, 0); ok {
			return nil, info.elemRustType, true
		}
		if info, ok := goPtrResultInfoForCall(call, 0); ok {
			return info.elemType, info.elemRustType, true
		}
	}
	if sel, ok := unwrapParens(expr).(*ast.SelectorExpr); ok {
		if info, ok := sliceElemPtrFieldInfoForSelector(sel); ok {
			return info.elemType, info.elemRustType, true
		}
	}
	if ident, ok := unwrapParens(expr).(*ast.Ident); ok {
		if info, ok := goPtrVarInfo(ident.Name); ok && info.RustType != "" {
			elemRustType := elemRustTypeFromSliceElemPtrRustType(info.RustType)
			if elemRustType != "" {
				typeInfo := GetTypeInfo()
				var elemType types.Type
				if typeInfo != nil {
					elemType, _ = sliceElemPtrPointerElemType(typeInfo.GetType(ident))
				}
				return elemType, elemRustType, true
			}
		}
		if info, ok := sliceElemPtrVarInfo(ident.Name); ok && info.RustType != "" {
			typeInfo := GetTypeInfo()
			var elemType types.Type
			if typeInfo != nil {
				elemType, _ = sliceElemPtrPointerElemType(typeInfo.GetType(ident))
			}
			return elemType, info.RustType, true
		}
		if info, ok := arrayElemPtrVarInfo(ident.Name); ok && info.RustType != "" {
			return nil, elemRustTypeFromArrayElemPtrRustType(info.RustType), true
		}
		typeInfo := GetTypeInfo()
		if typeInfo == nil {
			return nil, "", false
		}
		obj := typeInfo.GetObject(ident)
		if obj == nil {
			return nil, "", false
		}
		if localCandidates != nil {
			if elemRustType, ok := localCandidates[obj]; ok {
				elemType, _ := sliceElemPtrPointerElemType(typeInfo.GetType(ident))
				return elemType, elemRustType, true
			}
		}
		if elemRustType, ok := goPtrParamElemRustTypeForObject(currentFn, obj); ok {
			elemType, _ := sliceElemPtrPointerElemType(typeInfo.GetType(ident))
			return elemType, elemRustType, true
		}
		if arrayCandidates != nil {
			if info, ok := arrayCandidates[obj]; ok {
				return nil, info.elemRustType, true
			}
		}
		if goPtrCandidates != nil {
			info, ok := goPtrCandidates[obj]
			return info.elemType, info.elemRustType, ok
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

func goPtrArrayFieldElemCompatible(rhsElemType types.Type, rhsElemRustType string, fieldInfo goPtrArrayFieldInfo) bool {
	if rhsElemType != nil && fieldInfo.elemType != nil && types.Identical(coreType(rhsElemType), coreType(fieldInfo.elemType)) {
		return true
	}
	return rhsElemRustType != "" && rhsElemRustType == fieldInfo.elemRustType
}

func sliceElemPtrSliceFieldElemCompatible(rhsElemType types.Type, rhsElemRustType string, fieldInfo sliceElemPtrSliceFieldInfo) bool {
	if rhsElemType != nil && fieldInfo.elemType != nil && types.Identical(coreType(rhsElemType), coreType(fieldInfo.elemType)) {
		return true
	}
	return rhsElemRustType != "" && rhsElemRustType == fieldInfo.elemRustType
}

func sliceElemPtrFieldElemRustType(fieldInfo sliceElemPtrFieldInfo) string {
	if fieldInfo.elemType != nil {
		if rustType := goTypesCollectionElemTypeToRust(fieldInfo.elemType); rustType != "" {
			return rustType
		}
	}
	return fieldInfo.elemRustType
}

func sliceElemPtrSliceFieldElemRustType(fieldInfo sliceElemPtrSliceFieldInfo) string {
	if fieldInfo.elemType != nil {
		if rustType := goTypesTypeToRust(fieldInfo.elemType); rustType != "" {
			return rustType
		}
	}
	return fieldInfo.elemRustType
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
	owner, ok := sliceElemPtrFieldOwnerType(selection)
	if !ok {
		return "", sliceElemPtrFieldInfo{}, false
	}
	ptr, ok := types.Unalias(field.Type()).Underlying().(*types.Pointer)
	if !ok {
		return "", sliceElemPtrFieldInfo{}, false
	}
	key := sliceElemPtrFieldKeyForOwnerType(owner, field.Name())
	if key == "" {
		return "", sliceElemPtrFieldInfo{}, false
	}
	elemType := coreType(ptr.Elem())
	elemRustType := goTypesCollectionElemTypeToRust(elemType)
	if elemRustType == "" {
		return "", sliceElemPtrFieldInfo{}, false
	}
	ownerPkgPath := sliceElemPtrFieldOwnerPkgPath(owner)
	return key, sliceElemPtrFieldInfo{elemRustType: elemRustType, ownerPkgPath: ownerPkgPath, elemType: elemType}, true
}

func goPtrArrayFieldKeyForIndexExpr(indexExpr *ast.IndexExpr) (string, goPtrArrayFieldInfo, bool) {
	if indexExpr == nil {
		return "", goPtrArrayFieldInfo{}, false
	}
	sel, ok := unwrapParens(indexExpr.X).(*ast.SelectorExpr)
	if !ok {
		return "", goPtrArrayFieldInfo{}, false
	}
	return goPtrArrayFieldKeyForSelector(sel)
}

func sliceElemPtrSliceFieldKeyForIndexExpr(indexExpr *ast.IndexExpr) (string, sliceElemPtrSliceFieldInfo, bool) {
	if indexExpr == nil {
		return "", sliceElemPtrSliceFieldInfo{}, false
	}
	sel, ok := unwrapParens(indexExpr.X).(*ast.SelectorExpr)
	if !ok {
		return "", sliceElemPtrSliceFieldInfo{}, false
	}
	return sliceElemPtrSliceFieldKeyForSelector(sel)
}

func goPtrArrayFieldKeyForSelector(sel *ast.SelectorExpr) (string, goPtrArrayFieldInfo, bool) {
	typeInfo := GetTypeInfo()
	if sel == nil || typeInfo == nil || typeInfo.info == nil {
		return "", goPtrArrayFieldInfo{}, false
	}
	selection := typeInfo.info.Selections[sel]
	if selection == nil || selection.Kind() != types.FieldVal || len(selection.Index()) == 0 {
		return "", goPtrArrayFieldInfo{}, false
	}
	field, ok := selection.Obj().(*types.Var)
	if !ok || field == nil {
		return "", goPtrArrayFieldInfo{}, false
	}
	owner, ok := sliceElemPtrFieldOwnerType(selection)
	if !ok {
		return "", goPtrArrayFieldInfo{}, false
	}
	array, ok := types.Unalias(field.Type()).Underlying().(*types.Array)
	if !ok {
		return "", goPtrArrayFieldInfo{}, false
	}
	ptr, ok := types.Unalias(array.Elem()).Underlying().(*types.Pointer)
	if !ok {
		return "", goPtrArrayFieldInfo{}, false
	}
	key := sliceElemPtrFieldKeyForOwnerType(owner, field.Name())
	if key == "" {
		return "", goPtrArrayFieldInfo{}, false
	}
	elemType := coreType(ptr.Elem())
	elemRustType := goTypesCollectionElemTypeToRust(elemType)
	if elemRustType == "" {
		return "", goPtrArrayFieldInfo{}, false
	}
	ownerPkgPath := sliceElemPtrFieldOwnerPkgPath(owner)
	return key, goPtrArrayFieldInfo{elemRustType: elemRustType, ownerPkgPath: ownerPkgPath, elemType: elemType, arrayLen: array.Len()}, true
}

func sliceElemPtrSliceFieldKeyForSelector(sel *ast.SelectorExpr) (string, sliceElemPtrSliceFieldInfo, bool) {
	typeInfo := GetTypeInfo()
	if sel == nil || typeInfo == nil || typeInfo.info == nil {
		return "", sliceElemPtrSliceFieldInfo{}, false
	}
	selection := typeInfo.info.Selections[sel]
	if selection == nil || selection.Kind() != types.FieldVal || len(selection.Index()) == 0 {
		return "", sliceElemPtrSliceFieldInfo{}, false
	}
	field, ok := selection.Obj().(*types.Var)
	if !ok || field == nil {
		return "", sliceElemPtrSliceFieldInfo{}, false
	}
	owner, ok := sliceElemPtrFieldOwnerType(selection)
	if !ok {
		return "", sliceElemPtrSliceFieldInfo{}, false
	}
	slice, ok := types.Unalias(field.Type()).Underlying().(*types.Slice)
	if !ok {
		return "", sliceElemPtrSliceFieldInfo{}, false
	}
	ptr, ok := types.Unalias(slice.Elem()).Underlying().(*types.Pointer)
	if !ok {
		return "", sliceElemPtrSliceFieldInfo{}, false
	}
	key := sliceElemPtrFieldKeyForOwnerType(owner, field.Name())
	if key == "" {
		return "", sliceElemPtrSliceFieldInfo{}, false
	}
	elemType := coreType(ptr.Elem())
	elemRustType := goTypesTypeToRust(elemType)
	if elemRustType == "" {
		return "", sliceElemPtrSliceFieldInfo{}, false
	}
	ownerPkgPath := sliceElemPtrFieldOwnerPkgPath(owner)
	return key, sliceElemPtrSliceFieldInfo{elemRustType: elemRustType, ownerPkgPath: ownerPkgPath, elemType: elemType}, true
}

func sliceElemPtrFieldOwnerType(selection *types.Selection) (types.Type, bool) {
	if selection == nil || len(selection.Index()) == 0 {
		return nil, false
	}
	current := selection.Recv()
	indexes := selection.Index()
	for i, fieldIndex := range indexes {
		owner := sliceElemPtrDerefPointerType(current)
		st, ok := sliceElemPtrStructUnderlying(owner)
		if !ok || fieldIndex < 0 || fieldIndex >= st.NumFields() {
			return nil, false
		}
		if i == len(indexes)-1 {
			return owner, true
		}
		current = st.Field(fieldIndex).Type()
	}
	return nil, false
}

func sliceElemPtrDerefPointerType(typ types.Type) types.Type {
	if typ == nil {
		return nil
	}
	if ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer); ok {
		return ptr.Elem()
	}
	return typ
}

func sliceElemPtrStructUnderlying(typ types.Type) (*types.Struct, bool) {
	if typ == nil {
		return nil, false
	}
	if named, ok := types.Unalias(typ).(*types.Named); ok {
		st, ok := types.Unalias(named.Underlying()).Underlying().(*types.Struct)
		return st, ok
	}
	st, ok := types.Unalias(typ).Underlying().(*types.Struct)
	return st, ok
}

func sliceElemPtrFieldKeyForOwnerType(owner types.Type, fieldName string) string {
	if fieldName == "" {
		return ""
	}
	if named, ok := types.Unalias(owner).(*types.Named); ok {
		return sliceElemPtrFieldKey(named, fieldName)
	}
	if _, ok := sliceElemPtrStructUnderlying(owner); ok {
		return sliceElemPtrAnonymousFieldKey(owner, fieldName)
	}
	return ""
}

func sliceElemPtrFieldOwnerPkgPath(owner types.Type) string {
	if named, ok := types.Unalias(owner).(*types.Named); ok && named.Obj() != nil && named.Obj().Pkg() != nil {
		return named.Obj().Pkg().Path()
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil {
		return typeInfo.pkg.Path()
	}
	return ""
}

func sliceElemPtrAnonymousFieldKey(owner types.Type, fieldName string) string {
	if owner == nil || fieldName == "" {
		return ""
	}
	if _, ok := sliceElemPtrStructUnderlying(owner); !ok {
		return ""
	}
	return "struct." + sliceElemPtrTypeString(types.Unalias(owner)) + "." + fieldName
}

func sliceElemPtrTypeString(typ types.Type) string {
	return types.TypeString(typ, func(pkg *types.Package) string {
		if pkg == nil {
			return ""
		}
		return pkg.Path()
	})
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

func sliceElemPtrFieldKeyForAnonymousStructField(structType *ast.StructType, fieldName string) string {
	typeInfo := GetTypeInfo()
	if structType == nil || fieldName == "" || typeInfo == nil {
		return ""
	}
	typ := typeInfo.GetType(structType)
	if typ == nil {
		return ""
	}
	return sliceElemPtrAnonymousFieldKey(typ, fieldName)
}

func sliceElemPtrFieldInfoForAnonymousStructField(structType *ast.StructType, fieldName string) (sliceElemPtrFieldInfo, bool) {
	key := sliceElemPtrFieldKeyForAnonymousStructField(structType, fieldName)
	if key == "" {
		return sliceElemPtrFieldInfo{}, false
	}
	return sliceElemPtrFieldInfoForKey(key)
}

func sliceElemPtrFieldInfoForStructField(typeSpec *ast.TypeSpec, structType *ast.StructType, structName string, fieldName string) (sliceElemPtrFieldInfo, bool) {
	_, info, ok := sliceElemPtrFieldKeyAndInfoForStructField(typeSpec, structType, structName, fieldName)
	return info, ok
}

func goPtrArrayFieldInfoForStructField(typeSpec *ast.TypeSpec, structType *ast.StructType, structName string, fieldName string) (goPtrArrayFieldInfo, bool) {
	key := goPtrArrayFieldKeyForStructField(typeSpec, structType, structName, fieldName)
	if key == "" {
		return goPtrArrayFieldInfo{}, false
	}
	return goPtrArrayFieldInfoForKey(key)
}

func sliceElemPtrSliceFieldInfoForStructField(typeSpec *ast.TypeSpec, structType *ast.StructType, structName string, fieldName string) (sliceElemPtrSliceFieldInfo, bool) {
	key := sliceElemPtrSliceFieldKeyForStructField(typeSpec, structType, structName, fieldName)
	if key == "" {
		return sliceElemPtrSliceFieldInfo{}, false
	}
	return sliceElemPtrSliceFieldInfoForKey(key)
}

func goPtrArrayFieldKeyForStructField(typeSpec *ast.TypeSpec, structType *ast.StructType, structName string, fieldName string) string {
	keys := []string{
		sliceElemPtrFieldKeyForTypeSpecField(typeSpec, fieldName),
		sliceElemPtrFieldKeyForAnonymousStructField(structType, fieldName),
		sliceElemPtrFieldKeyForStructNameField(structName, fieldName),
	}
	for _, key := range keys {
		if key == "" {
			continue
		}
		if _, ok := goPtrArrayFieldInfoForKey(key); ok {
			return key
		}
	}
	return ""
}

func sliceElemPtrSliceFieldKeyForStructField(typeSpec *ast.TypeSpec, structType *ast.StructType, structName string, fieldName string) string {
	keys := []string{
		sliceElemPtrFieldKeyForTypeSpecField(typeSpec, fieldName),
		sliceElemPtrFieldKeyForAnonymousStructField(structType, fieldName),
		sliceElemPtrFieldKeyForStructNameField(structName, fieldName),
	}
	for _, key := range keys {
		if key == "" {
			continue
		}
		if _, ok := sliceElemPtrSliceFieldInfoForKey(key); ok {
			return key
		}
	}
	return ""
}

func sliceElemPtrFieldKeyForStructField(typeSpec *ast.TypeSpec, structType *ast.StructType, structName string, fieldName string) string {
	key, _, ok := sliceElemPtrFieldKeyAndInfoForStructField(typeSpec, structType, structName, fieldName)
	if !ok {
		return ""
	}
	return key
}

func sliceElemPtrFieldKeyAndInfoForStructField(typeSpec *ast.TypeSpec, structType *ast.StructType, structName string, fieldName string) (string, sliceElemPtrFieldInfo, bool) {
	keys := []string{
		sliceElemPtrFieldKeyForTypeSpecField(typeSpec, fieldName),
		sliceElemPtrFieldKeyForAnonymousStructField(structType, fieldName),
		sliceElemPtrFieldKeyForStructNameField(structName, fieldName),
	}
	for _, key := range keys {
		if key == "" {
			continue
		}
		if info, ok := sliceElemPtrFieldInfoForKey(key); ok {
			return key, info, true
		}
	}
	return "", sliceElemPtrFieldInfo{}, false
}

func sliceElemPtrFieldInfoForStructNameField(structName string, fieldName string) (sliceElemPtrFieldInfo, bool) {
	key := sliceElemPtrFieldKeyForStructNameField(structName, fieldName)
	if key == "" {
		return sliceElemPtrFieldInfo{}, false
	}
	return sliceElemPtrFieldInfoForKey(key)
}

func sliceElemPtrFieldKeyForStructNameField(structName string, fieldName string) string {
	typeInfo := GetTypeInfo()
	if structName == "" || fieldName == "" || typeInfo == nil || typeInfo.pkg == nil || typeInfo.pkg.Scope() == nil {
		return ""
	}
	obj, ok := typeInfo.pkg.Scope().Lookup(structName).(*types.TypeName)
	if !ok || obj == nil {
		return ""
	}
	named, ok := types.Unalias(obj.Type()).(*types.Named)
	if !ok {
		return ""
	}
	return sliceElemPtrFieldKey(named, fieldName)
}

func sliceElemPtrFieldInfoForOwnerStructField(owner types.Type, field *types.Var) (sliceElemPtrFieldInfo, bool) {
	if owner == nil || field == nil {
		return sliceElemPtrFieldInfo{}, false
	}
	key := sliceElemPtrFieldKeyForOwnerType(owner, field.Name())
	if key == "" {
		return sliceElemPtrFieldInfo{}, false
	}
	return sliceElemPtrFieldInfoForKey(key)
}

func sliceElemPtrFieldInfoForSelector(sel *ast.SelectorExpr) (sliceElemPtrFieldInfo, bool) {
	key, _, ok := sliceElemPtrFieldKeyForSelector(sel)
	if !ok {
		return sliceElemPtrFieldInfo{}, false
	}
	return sliceElemPtrFieldInfoForKey(key)
}

func sliceElemPtrSliceFieldInfoForSelector(sel *ast.SelectorExpr) (sliceElemPtrSliceFieldInfo, bool) {
	key, _, ok := sliceElemPtrSliceFieldKeyForSelector(sel)
	if !ok {
		return sliceElemPtrSliceFieldInfo{}, false
	}
	return sliceElemPtrSliceFieldInfoForKey(key)
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

func goPtrArrayFieldInfoForKey(key string) (goPtrArrayFieldInfo, bool) {
	ctx := GetTranspileContext()
	if key == "" || ctx == nil {
		return goPtrArrayFieldInfo{}, false
	}
	if ctx.Package != nil {
		if info, ok := ctx.Package.GoPtrArrayFields[key]; ok {
			return info, true
		}
	}
	if ctx.Session != nil {
		info, ok := ctx.Session.GoPtrArrayFields[key]
		return info, ok
	}
	return goPtrArrayFieldInfo{}, false
}

func sliceElemPtrSliceFieldInfoForKey(key string) (sliceElemPtrSliceFieldInfo, bool) {
	ctx := GetTranspileContext()
	if key == "" || ctx == nil {
		return sliceElemPtrSliceFieldInfo{}, false
	}
	if ctx.Package != nil {
		if info, ok := ctx.Package.SliceElemPtrSliceFields[key]; ok {
			return info, true
		}
	}
	if ctx.Session != nil {
		info, ok := ctx.Session.SliceElemPtrSliceFields[key]
		return info, ok
	}
	return sliceElemPtrSliceFieldInfo{}, false
}

func goPtrArrayFieldInfoForIndexExpr(indexExpr *ast.IndexExpr) (goPtrArrayFieldInfo, bool) {
	key, _, ok := goPtrArrayFieldKeyForIndexExpr(indexExpr)
	if !ok {
		return goPtrArrayLocalInfoForIndexExpr(indexExpr)
	}
	if info, ok := goPtrArrayFieldInfoForKey(key); ok {
		return info, true
	}
	return goPtrArrayLocalInfoForIndexExpr(indexExpr)
}

func goPtrArrayLocalInfoForDecl(name *ast.Ident) (goPtrArrayFieldInfo, bool) {
	typeInfo := GetTypeInfo()
	if name == nil || typeInfo == nil || typeInfo.info == nil {
		return goPtrArrayFieldInfo{}, false
	}
	return goPtrArrayLocalInfoForObject(typeInfo.GetObject(name))
}

func goPtrArrayLocalInfoForIndexExpr(indexExpr *ast.IndexExpr) (goPtrArrayFieldInfo, bool) {
	typeInfo := GetTypeInfo()
	if indexExpr == nil || typeInfo == nil || typeInfo.info == nil {
		return goPtrArrayFieldInfo{}, false
	}
	ident, ok := unwrapParens(indexExpr.X).(*ast.Ident)
	if !ok {
		return goPtrArrayFieldInfo{}, false
	}
	return goPtrArrayLocalInfoForObject(typeInfo.GetObject(ident))
}

func goPtrArrayLocalInfoForObject(obj types.Object) (goPtrArrayFieldInfo, bool) {
	ctx := GetTranspileContext()
	if obj == nil || ctx == nil || ctx.Package == nil || ctx.Package.GoPtrArrayLocalObjs == nil {
		return goPtrArrayFieldInfo{}, false
	}
	info, ok := ctx.Package.GoPtrArrayLocalObjs[obj]
	return info, ok
}

func goPtrArrayLocalObjectForIndexExpr(indexExpr *ast.IndexExpr) (types.Object, goPtrArrayFieldInfo, bool) {
	typeInfo := GetTypeInfo()
	if indexExpr == nil || typeInfo == nil || typeInfo.info == nil {
		return nil, goPtrArrayFieldInfo{}, false
	}
	ident, ok := unwrapParens(indexExpr.X).(*ast.Ident)
	if !ok || ident.Name == "_" {
		return nil, goPtrArrayFieldInfo{}, false
	}
	obj := typeInfo.GetObject(ident)
	if obj == nil {
		return nil, goPtrArrayFieldInfo{}, false
	}
	array, ok := types.Unalias(obj.Type()).Underlying().(*types.Array)
	if !ok {
		return nil, goPtrArrayFieldInfo{}, false
	}
	ptr, ok := types.Unalias(array.Elem()).Underlying().(*types.Pointer)
	if !ok {
		return nil, goPtrArrayFieldInfo{}, false
	}
	elemType := coreType(ptr.Elem())
	elemRustType := goTypesCollectionElemTypeToRust(elemType)
	if elemRustType == "" {
		return nil, goPtrArrayFieldInfo{}, false
	}
	ownerPkgPath := ""
	if typeInfo.pkg != nil {
		ownerPkgPath = typeInfo.pkg.Path()
	}
	return obj, goPtrArrayFieldInfo{elemRustType: elemRustType, ownerPkgPath: ownerPkgPath, elemType: elemType, arrayLen: array.Len()}, true
}

func sliceElemPtrSliceFieldInfoForIndexExpr(indexExpr *ast.IndexExpr) (sliceElemPtrSliceFieldInfo, bool) {
	key, _, ok := sliceElemPtrSliceFieldKeyForIndexExpr(indexExpr)
	if !ok {
		return sliceElemPtrSliceFieldInfo{}, false
	}
	return sliceElemPtrSliceFieldInfoForKey(key)
}

func recordGeneratedGoPtrFieldForStructField(typeSpec *ast.TypeSpec, structType *ast.StructType, structName string, fieldName string) {
	key := sliceElemPtrFieldKeyForStructField(typeSpec, structType, structName, fieldName)
	if key == "" {
		return
	}
	recordGeneratedGoPtrFieldForKey(key)
}

func recordGeneratedGoPtrFieldForKey(key string) {
	ctx := GetTranspileContext()
	if key == "" || ctx == nil || ctx.Package == nil {
		return
	}
	if ctx.Package.GeneratedGoPtrFields == nil {
		ctx.Package.GeneratedGoPtrFields = make(map[string]bool)
	}
	ctx.Package.GeneratedGoPtrFields[key] = true
	if ctx.Session != nil {
		if ctx.Session.GeneratedGoPtrFields == nil {
			ctx.Session.GeneratedGoPtrFields = make(map[string]bool)
		}
		ctx.Session.GeneratedGoPtrFields[key] = true
	}
}

func generatedGoPtrFieldForSelector(sel *ast.SelectorExpr) bool {
	key, _, ok := sliceElemPtrFieldKeyForSelector(sel)
	if !ok {
		return false
	}
	return generatedGoPtrFieldForKey(key)
}

func generatedGoPtrFieldForKey(key string) bool {
	ctx := GetTranspileContext()
	if key == "" || ctx == nil {
		return false
	}
	if ctx.Package != nil && ctx.Package.GeneratedGoPtrFields[key] {
		return true
	}
	return ctx.Session != nil && ctx.Session.GeneratedGoPtrFields[key]
}

func generatedGoPtrFieldForStructNameField(structName string, fieldName string) bool {
	key := sliceElemPtrFieldKeyForStructNameField(structName, fieldName)
	if key == "" || !generatedGoPtrFieldForKey(key) {
		return false
	}
	_, ok := sliceElemPtrFieldInfoForKey(key)
	return ok
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
		sliceElemPtrResultInfosForFunc(fn)
		registerSliceElemPtrSliceReturnDecl(fn)
		arrayElemPtrResultInfosForFunc(fn)
		goPtrResultInfosForFunc(fn)
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

func goPtrParamInfosForFunc(fn *types.Func) (map[int]string, bool) {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil {
		return nil, false
	}
	if info, ok := syncAtomicPointerMethodGoPtrParamInfos(fn); ok {
		return info, true
	}
	if ctx.Package != nil {
		if info, ok := ctx.Package.GoPtrParamFuncs[fn]; ok {
			return info, true
		}
		if info, ok := goPtrParamInfosForFuncByIdentity(ctx.Package.GoPtrParamFuncs, fn); ok {
			return info, true
		}
	}
	if ctx.Session != nil {
		if key := methodOverrideKey(fn); key != "" {
			if info, ok := ctx.Session.GoPtrParamFuncNames[key]; ok {
				return info, true
			}
		}
		if info, ok := ctx.Session.GoPtrParamFuncNames[fn.FullName()]; ok {
			return info, true
		}
	}
	return nil, false
}

func goPtrParamInfosForFuncByIdentity(registry map[*types.Func]map[int]string, fn *types.Func) (map[int]string, bool) {
	if registry == nil || fn == nil {
		return nil, false
	}
	targetMethodKey := methodOverrideKey(fn)
	targetFullName := fn.FullName()
	for candidate, info := range registry {
		if candidate == nil || len(info) == 0 {
			continue
		}
		if targetMethodKey != "" && methodOverrideKey(candidate) == targetMethodKey {
			return info, true
		}
		if targetFullName != "" && candidate.FullName() == targetFullName {
			return info, true
		}
	}
	return nil, false
}

func goPtrParamInfoForFunc(fn *types.Func, paramIndex int) (string, bool) {
	info, ok := goPtrParamInfosForFunc(fn)
	if !ok {
		return "", false
	}
	elemRustType, ok := info[paramIndex]
	return elemRustType, ok
}

func goPtrSlotParamInfosForFunc(fn *types.Func) (map[int]string, bool) {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil {
		return nil, false
	}
	if ctx.Package != nil {
		if info, ok := ctx.Package.GoPtrSlotParamFuncs[fn]; ok {
			return info, true
		}
		if info, ok := goPtrParamInfosForFuncByIdentity(ctx.Package.GoPtrSlotParamFuncs, fn); ok {
			return info, true
		}
	}
	if ctx.Session != nil {
		if key := methodOverrideKey(fn); key != "" {
			if info, ok := ctx.Session.GoPtrSlotParamFuncNames[key]; ok {
				return info, true
			}
		}
		if info, ok := ctx.Session.GoPtrSlotParamFuncNames[fn.FullName()]; ok {
			return info, true
		}
	}
	return nil, false
}

func goPtrSlotParamInfoForFunc(fn *types.Func, paramIndex int) (string, bool) {
	info, ok := goPtrSlotParamInfosForFunc(fn)
	if !ok {
		return "", false
	}
	elemRustType, ok := info[paramIndex]
	return elemRustType, ok
}

func syncAtomicPointerMethodSignature(fn *types.Func) (*types.Signature, bool) {
	if fn == nil || fn.Pkg() == nil || fn.Pkg().Path() != "sync/atomic" {
		return nil, false
	}
	sig, ok := signatureFromType(fn.Type())
	if !ok || sig == nil || sig.Recv() == nil {
		return nil, false
	}
	recvType := types.Unalias(sig.Recv().Type())
	if ptr, ok := recvType.(*types.Pointer); ok {
		recvType = types.Unalias(ptr.Elem())
	}
	named, ok := recvType.(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil {
		return nil, false
	}
	if named.Obj().Pkg().Path() != "sync/atomic" || named.Obj().Name() != "Pointer" {
		return nil, false
	}
	return sig, true
}

func syncAtomicPointerMethodElemType(fn *types.Func, sig *types.Signature) (types.Type, bool) {
	if sig == nil || sig.Recv() == nil {
		return nil, false
	}
	recvType := types.Unalias(sig.Recv().Type())
	if ptr, ok := recvType.(*types.Pointer); ok {
		recvType = types.Unalias(ptr.Elem())
	}
	named, ok := recvType.(*types.Named)
	if !ok || named.TypeArgs() == nil || named.TypeArgs().Len() == 0 {
		return nil, false
	}
	return named.TypeArgs().At(0), true
}

func syncAtomicPointerMethodGoPtrParamInfos(fn *types.Func) (map[int]string, bool) {
	sig, ok := syncAtomicPointerMethodSignature(fn)
	if !ok || sig.Params() == nil {
		return nil, false
	}
	elemType, hasElemType := syncAtomicPointerMethodElemType(fn, sig)
	var indexes []int
	switch fn.Name() {
	case "Store", "Swap":
		indexes = []int{0}
	case "CompareAndSwap":
		indexes = []int{0, 1}
	default:
		return nil, false
	}
	result := map[int]string{}
	for _, index := range indexes {
		if index >= sig.Params().Len() {
			continue
		}
		ptr, ok := types.Unalias(sig.Params().At(index).Type()).Underlying().(*types.Pointer)
		if !ok {
			continue
		}
		if !hasElemType {
			elemType = ptr.Elem()
		}
		result[index] = goPtrDeclElemTypeToRust(fn, elemType)
	}
	if len(result) == 0 {
		return nil, false
	}
	return result, true
}

func syncAtomicPointerMethodGoPtrResultInfos(fn *types.Func) (map[int]goPtrResultInfo, bool) {
	sig, ok := syncAtomicPointerMethodSignature(fn)
	if !ok || sig.Results() == nil {
		return nil, false
	}
	switch fn.Name() {
	case "Load", "Swap":
	default:
		return nil, false
	}
	if sig.Results().Len() == 0 {
		return nil, false
	}
	elemType, hasElemType := syncAtomicPointerMethodElemType(fn, sig)
	ptr, ok := types.Unalias(sig.Results().At(0).Type()).Underlying().(*types.Pointer)
	if !ok {
		return nil, false
	}
	if !hasElemType {
		elemType = ptr.Elem()
	}
	return map[int]goPtrResultInfo{
		0: {
			elemRustType: goPtrDeclElemTypeToRust(fn, elemType),
			elemType:     elemType,
		},
	}, true
}

func goPtrParamResultInfoForFunc(fn *types.Func, paramIndex int) (goPtrResultInfo, bool) {
	elemRustType, ok := goPtrParamInfoForFunc(fn, paramIndex)
	if !ok {
		return goPtrResultInfo{}, false
	}
	result := goPtrResultInfo{elemRustType: elemRustType}
	sig, ok := signatureFromType(fn.Type())
	if !ok || sig.Params() == nil || paramIndex < 0 || paramIndex >= sig.Params().Len() {
		return result, true
	}
	if ptr, ok := types.Unalias(sig.Params().At(paramIndex).Type()).Underlying().(*types.Pointer); ok {
		result.elemType = ptr.Elem()
	}
	return result, true
}

func goPtrSlotParamResultInfoForFunc(fn *types.Func, paramIndex int) (goPtrResultInfo, bool) {
	elemRustType, ok := goPtrSlotParamInfoForFunc(fn, paramIndex)
	if !ok {
		return goPtrResultInfo{}, false
	}
	result := goPtrResultInfo{elemRustType: elemRustType}
	sig, ok := signatureFromType(fn.Type())
	if !ok || sig.Params() == nil || paramIndex < 0 || paramIndex >= sig.Params().Len() {
		return result, true
	}
	if ptr, ok := types.Unalias(sig.Params().At(paramIndex).Type()).Underlying().(*types.Pointer); ok {
		if innerPtr, ok := types.Unalias(ptr.Elem()).Underlying().(*types.Pointer); ok {
			result.elemType = innerPtr.Elem()
		}
	}
	return result, true
}

func goPtrSlotParamGoPtrResultInfoForFunc(fn *types.Func, paramIndex int) (goPtrResultInfo, bool) {
	info, ok := goPtrSlotParamResultInfoForFunc(fn, paramIndex)
	if !ok {
		return goPtrResultInfo{}, false
	}
	return goPtrResultInfo{elemRustType: "GoPtr<" + goPtrResultElemRustType(info) + ">"}, true
}

func goPtrSlotParamGoPtrResultInfoForCall(call *ast.CallExpr, paramIndex int) (goPtrResultInfo, bool) {
	typeInfo := GetTypeInfo()
	fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
	if !ok {
		return goPtrResultInfo{}, false
	}
	return goPtrSlotParamGoPtrResultInfoForFunc(fn, paramIndex)
}

func goPtrParamInfoForDeclObject(fn *ast.FuncDecl, paramIndex int) (string, bool) {
	fnObj, ok := sliceElemPtrReturnFuncObject(fn)
	if !ok {
		return "", false
	}
	return goPtrParamInfoForFunc(fnObj, paramIndex)
}

func goPtrParamDeclElemRustType(fn *ast.FuncDecl, paramIndex int) (string, bool) {
	if elemRustType, ok := syncAtomicPointerMethodGoPtrParamDeclElemRustType(fn, paramIndex); ok {
		return elemRustType, true
	}
	fnObj, ok := sliceElemPtrReturnFuncObject(fn)
	if !ok {
		return "", false
	}
	info, ok := goPtrParamResultInfoForFunc(fnObj, paramIndex)
	if !ok || info.elemType == nil {
		return "", false
	}
	return goPtrDeclElemTypeToRust(fnObj, info.elemType), true
}

func goPtrSlotParamDeclElemRustType(fn *ast.FuncDecl, paramIndex int) (string, bool) {
	fnObj, ok := sliceElemPtrReturnFuncObject(fn)
	if !ok {
		return "", false
	}
	info, ok := goPtrSlotParamResultInfoForFunc(fnObj, paramIndex)
	if !ok || info.elemType == nil {
		return "", false
	}
	return goPtrDeclElemTypeToRust(fnObj, info.elemType), true
}

func goPtrSlotParamRustType(elemRustType string) string {
	NeedSliceElemPtr()
	return "GoPtr<GoPtr<" + elemRustType + ">>"
}

func goPtrDeclElemTypeToRust(fn *types.Func, elemType types.Type) string {
	if typeParam, ok := types.Unalias(elemType).(*types.TypeParam); ok && typeParam.Obj() != nil {
		return RustTypeNameForUse(typeParam.Obj().Name())
	}
	named, ok := types.Unalias(elemType).(*types.Named)
	if !ok || named.Obj() == nil || named.Obj().Pkg() == nil || fn == nil || fn.Pkg() == nil {
		return goTypesTypeToRust(elemType)
	}
	if named.Obj().Pkg().Path() != fn.Pkg().Path() {
		return goTypesTypeToRust(elemType)
	}
	return rustNamedTypeWithArgs(named, rustImplTypeNameForUse(named.Obj().Name()))
}

func syncAtomicPointerMethodDeclElemRustType(fn *ast.FuncDecl) (string, bool) {
	if fn == nil || fn.Name == nil || !currentPackageIsSyncAtomic() || !runtimeLinkedReceiverIsNamed(fn, "Pointer") {
		return "", false
	}
	if fn.Recv == nil || len(fn.Recv.List) == 0 {
		return "", false
	}
	recvType := fn.Recv.List[0].Type
	if star, ok := recvType.(*ast.StarExpr); ok {
		recvType = star.X
	}
	var arg ast.Expr
	switch recv := recvType.(type) {
	case *ast.IndexExpr:
		arg = recv.Index
	case *ast.IndexListExpr:
		if len(recv.Indices) == 0 {
			return "", false
		}
		arg = recv.Indices[0]
	default:
		return "", false
	}
	ident, ok := unwrapParens(arg).(*ast.Ident)
	if !ok || ident.Name == "" {
		return "", false
	}
	return RustTypeNameForUse(ident.Name), true
}

func syncAtomicPointerMethodGoPtrParamDeclElemRustType(fn *ast.FuncDecl, paramIndex int) (string, bool) {
	elemRustType, ok := syncAtomicPointerMethodDeclElemRustType(fn)
	if !ok {
		return "", false
	}
	switch fn.Name.Name {
	case "Store", "Swap":
		return elemRustType, paramIndex == 0
	case "CompareAndSwap":
		return elemRustType, paramIndex == 0 || paramIndex == 1
	default:
		return "", false
	}
}

func syncAtomicPointerMethodGoPtrResultInfosForDecl(fn *ast.FuncDecl) (map[int]goPtrResultInfo, bool) {
	elemRustType, ok := syncAtomicPointerMethodDeclElemRustType(fn)
	if !ok {
		return nil, false
	}
	switch fn.Name.Name {
	case "Load", "Swap":
	default:
		return nil, false
	}
	if fn.Type == nil || fn.Type.Results == nil || fn.Type.Results.NumFields() == 0 {
		return nil, false
	}
	return map[int]goPtrResultInfo{
		0: {
			elemRustType: elemRustType,
		},
	}, true
}

func goPtrParamInfoForCall(call *ast.CallExpr, paramIndex int) (string, bool) {
	typeInfo := GetTypeInfo()
	fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
	if !ok {
		return "", false
	}
	return goPtrParamInfoForFunc(fn, paramIndex)
}

func goPtrParamResultInfoForCall(call *ast.CallExpr, paramIndex int) (goPtrResultInfo, bool) {
	typeInfo := GetTypeInfo()
	fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
	if !ok {
		return goPtrResultInfo{}, false
	}
	return goPtrParamResultInfoForFunc(fn, paramIndex)
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

func registerGoPtrParamsFromFiles(files []*ast.File) {
	ctx := GetTranspileContext()
	typeInfo := GetTypeInfo()
	if ctx == nil || ctx.Package == nil || typeInfo == nil {
		return
	}
	if ctx.Package.GoPtrParamFuncs == nil {
		ctx.Package.GoPtrParamFuncs = make(map[*types.Func]map[int]string)
	}

	for {
		changed := false
		for _, file := range files {
			if file == nil {
				continue
			}
			for _, decl := range file.Decls {
				fnDecl, ok := decl.(*ast.FuncDecl)
				if !ok || fnDecl.Body == nil {
					continue
				}
				currentFn, _ := sliceElemPtrReturnFuncObject(fnDecl)
				sliceCandidates := collectSliceElemPtrCandidates(fnDecl.Body)
				arrayCandidates := collectArrayElemPtrCandidatesForFunc(fnDecl)
				goPtrCandidates := collectGoPtrCandidatesForFunc(fnDecl)
				goPtrCandidates = mergeGoPtrCandidateInfos(goPtrCandidates, collectFunctionValueCallArgumentGoPtrParamObjects(fnDecl))
				goPtrSlotCandidates := collectGoPtrSlotCandidatesForFunc(fnDecl)
				if registerGoPtrParamsFromAssignments(currentFn, fnDecl, goPtrCandidates) {
					changed = true
				}
				ast.Inspect(fnDecl.Body, func(node ast.Node) bool {
					call, ok := node.(*ast.CallExpr)
					if !ok {
						return true
					}
					callee, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
					if !ok {
						return true
					}
					calleeHasBody := sourceFunctionHasBody(callee)
					for i, arg := range call.Args {
						if argElemRustType, ok := goPtrSlotArgElemRustType(arg, currentFn, goPtrSlotCandidates, goPtrCandidates); ok {
							paramElemRustType, ok := goPtrSlotCallParamElemRustType(callee, i)
							if ok && paramElemRustType == argElemRustType {
								declElemRustType := paramElemRustType
								if sourceFn, _, ok := sourceFunctionDeclObjectForFunc(callee); ok {
									if sourceElemRustType, ok := goPtrSlotCallParamElemRustType(sourceFn, i); ok {
										declElemRustType = sourceElemRustType
									}
								}
								if registerGoPtrSlotParam(callee, i, argElemRustType, declElemRustType) {
									changed = true
								}
							}
							continue
						}
						var argElemRustType string
						if !calleeHasBody {
							argElemRustType, ok = bodylessGoPtrArgElemRustType(arg, currentFn, goPtrCandidates)
						} else {
							argElemRustType, ok = goPtrArgElemRustType(arg, currentFn, sliceCandidates, arrayCandidates, goPtrCandidates)
						}
						if !ok {
							continue
						}
						paramElemRustType, ok := goPtrCallParamElemRustType(callee, i)
						if !ok || paramElemRustType != argElemRustType {
							continue
						}
						declElemRustType := paramElemRustType
						if sourceFn, _, ok := sourceFunctionDeclObjectForFunc(callee); ok {
							if sourceElemRustType, ok := goPtrCallParamElemRustType(sourceFn, i); ok {
								declElemRustType = sourceElemRustType
							}
						}
						if registerGoPtrParam(callee, i, argElemRustType, declElemRustType) {
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

func mergeGoPtrCandidateInfos(base map[types.Object]goPtrResultInfo, extra map[types.Object]goPtrResultInfo) map[types.Object]goPtrResultInfo {
	if len(extra) == 0 {
		return base
	}
	if base == nil {
		base = make(map[types.Object]goPtrResultInfo, len(extra))
	}
	for obj, info := range extra {
		if obj == nil || goPtrResultElemRustType(info) == "" {
			continue
		}
		base[obj] = info
	}
	return base
}

func registerGoPtrParamsFromAssignments(currentFn *types.Func, fnDecl *ast.FuncDecl, goPtrCandidates map[types.Object]goPtrResultInfo) bool {
	typeInfo := GetTypeInfo()
	if currentFn == nil || fnDecl == nil || fnDecl.Body == nil || typeInfo == nil {
		return false
	}
	changed := false
	goPtrCandidateStates := goPtrCandidatesAsStates(goPtrCandidates)
	ast.Inspect(fnDecl.Body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.AssignStmt:
			if n.Tok != token.ASSIGN && n.Tok != token.DEFINE {
				return true
			}
			for i, lhs := range n.Lhs {
				ident, ok := unwrapParens(lhs).(*ast.Ident)
				if !ok || ident.Name == "_" {
					continue
				}
				obj := typeInfo.GetObject(ident)
				paramIndex, ok := pointerParamIndexForObject(currentFn, obj)
				if !ok {
					continue
				}
				paramInfo, ok := goPtrParamInfoForSignatureIndex(currentFn, paramIndex)
				if !ok {
					continue
				}
				rhsInfo, rhsOK, sawGoPtr := goPtrAssignmentValueInfoForStmt(n, i, goPtrCandidateStates, typeInfo)
				if !rhsOK || !sawGoPtr || !goPtrResultElemCompatible(rhsInfo, paramInfo) {
					continue
				}
				elemRustType := goPtrResultElemRustType(paramInfo)
				if registerGoPtrParam(currentFn, paramIndex, elemRustType, elemRustType) {
					changed = true
				}
			}
		}
		return true
	})
	return changed
}

func collectFunctionValueCallArgumentGoPtrParamObjects(fn *ast.FuncDecl) map[types.Object]goPtrResultInfo {
	typeInfo := GetTypeInfo()
	if fn == nil || fn.Body == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	result := map[types.Object]goPtrResultInfo{}
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		call, ok := node.(*ast.CallExpr)
		if !ok {
			return true
		}
		for i, arg := range call.Args {
			funcLit, ok := unwrapParens(arg).(*ast.FuncLit)
			if !ok {
				continue
			}
			infos, _, ok := functionValueGoPtrParamInfosForCallArgument(call, i)
			if !ok {
				continue
			}
			for paramIndex, info := range infos {
				obj := funcLitParamObjectAt(funcLit, paramIndex, typeInfo)
				if obj == nil || goPtrResultElemRustType(info) == "" {
					continue
				}
				result[obj] = info
			}
		}
		return true
	})
	if len(result) == 0 {
		return nil
	}
	return result
}

func funcLitParamObjectAt(funcLit *ast.FuncLit, paramIndex int, typeInfo *TypeInfo) types.Object {
	if funcLit == nil || funcLit.Type == nil || funcLit.Type.Params == nil || paramIndex < 0 || typeInfo == nil {
		return nil
	}
	seen := 0
	for _, field := range funcLit.Type.Params.List {
		if field == nil {
			continue
		}
		if len(field.Names) == 0 {
			if seen == paramIndex {
				return nil
			}
			seen++
			continue
		}
		for _, name := range field.Names {
			if seen == paramIndex {
				return typeInfo.GetObject(name)
			}
			seen++
		}
	}
	return nil
}

func goPtrCandidatesAsStates(candidates map[types.Object]goPtrResultInfo) map[types.Object]*goPtrCandidate {
	if len(candidates) == 0 {
		return nil
	}
	states := make(map[types.Object]*goPtrCandidate, len(candidates))
	for obj, info := range candidates {
		states[obj] = &goPtrCandidate{info: info, valid: true, sawGoPtr: true}
	}
	return states
}

func pointerParamIndexForObject(fn *types.Func, obj types.Object) (int, bool) {
	if fn == nil || obj == nil {
		return -1, false
	}
	sig, ok := signatureFromType(fn.Type())
	if !ok || sig.Params() == nil {
		return -1, false
	}
	for i := 0; i < sig.Params().Len(); i++ {
		if sig.Params().At(i) == obj {
			return i, true
		}
	}
	return -1, false
}

func goPtrParamInfoForSignatureIndex(fn *types.Func, paramIndex int) (goPtrResultInfo, bool) {
	if fn == nil || paramIndex < 0 {
		return goPtrResultInfo{}, false
	}
	sig, ok := signatureFromType(fn.Type())
	if !ok || sig.Params() == nil || paramIndex >= sig.Params().Len() {
		return goPtrResultInfo{}, false
	}
	return goPtrInfoForPointerType(sig.Params().At(paramIndex).Type())
}

func collectGoPtrSlotCandidatesForFunc(fn *ast.FuncDecl) map[types.Object]goPtrResultInfo {
	typeInfo := GetTypeInfo()
	if fn == nil || fn.Body == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	candidates := map[types.Object]goPtrResultInfo{}
	valid := map[types.Object]bool{}
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.ValueSpec:
			for i, name := range n.Names {
				if name == nil || name.Name == "_" || len(n.Values) <= i {
					continue
				}
				obj := typeInfo.GetObject(name)
				if obj == nil {
					continue
				}
				expected, ok := goPtrSlotInfoForPointerToPointerType(obj.Type())
				if !ok {
					continue
				}
				info, ok := goPtrSlotValueInfo(n.Values[i])
				valid[obj] = ok && goPtrResultElemCompatible(info, expected)
				if valid[obj] {
					candidates[obj] = info
				}
			}
		case *ast.AssignStmt:
			if n.Tok != token.DEFINE && n.Tok != token.ASSIGN {
				return true
			}
			for i, lhs := range n.Lhs {
				ident, ok := unwrapParens(lhs).(*ast.Ident)
				if !ok || ident.Name == "_" {
					continue
				}
				obj := typeInfo.GetObject(ident)
				if obj == nil {
					continue
				}
				expected, ok := goPtrSlotInfoForPointerToPointerType(obj.Type())
				if !ok {
					continue
				}
				rhs := assignmentRHSForLHS(n, i)
				info, ok := goPtrSlotValueInfo(rhs)
				if !ok || !goPtrResultElemCompatible(info, expected) {
					if _, seen := valid[obj]; seen {
						valid[obj] = false
						delete(candidates, obj)
					}
					continue
				}
				valid[obj] = true
				candidates[obj] = info
			}
		}
		return true
	})
	for obj, ok := range valid {
		if !ok {
			delete(candidates, obj)
		}
	}
	if len(candidates) == 0 {
		return nil
	}
	return candidates
}

func goPtrSlotValueInfo(expr ast.Expr) (goPtrResultInfo, bool) {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return goPtrResultInfo{}, false
	}
	target := unwrapParens(unary.X)
	if sel, ok := target.(*ast.SelectorExpr); ok && generatedGoPtrFieldForSelector(sel) {
		if _, info, ok := sliceElemPtrFieldKeyForSelector(sel); ok {
			return goPtrResultInfo{elemRustType: info.elemRustType, elemType: info.elemType}, true
		}
	}
	if ident, ok := target.(*ast.Ident); ok {
		if elemRustType, ok := goPtrVarElemRustType(ident.Name); ok {
			return goPtrResultInfo{elemRustType: elemRustType}, true
		}
	}
	return goPtrResultInfo{}, false
}

func collectFuncLitGoPtrParamInfosForFunc(fn *ast.FuncDecl) map[*ast.FuncLit]map[int]goPtrResultInfo {
	typeInfo := GetTypeInfo()
	if fn == nil || fn.Body == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	currentFn, _ := sliceElemPtrReturnFuncObject(fn)
	sliceCandidates := collectSliceElemPtrCandidates(fn.Body)
	arrayCandidates := collectArrayElemPtrCandidatesForFunc(fn)
	goPtrCandidates := collectGoPtrCandidatesForFunc(fn)
	localFuncLits := sourceLocalFuncLitsByObject(typeInfo.info, fn.Body)
	result := map[*ast.FuncLit]map[int]goPtrResultInfo{}

	ast.Inspect(fn.Body, func(node ast.Node) bool {
		call, ok := node.(*ast.CallExpr)
		if !ok {
			return true
		}
		funcLit := callFuncLitForGoPtrParamInfo(typeInfo.info, localFuncLits, call)
		if funcLit == nil {
			return true
		}
		for i, arg := range call.Args {
			argElemRustType, ok := goPtrArgElemRustType(arg, currentFn, sliceCandidates, arrayCandidates, goPtrCandidates)
			if !ok {
				continue
			}
			paramInfo, ok := funcLitPointerParamGoPtrInfo(funcLit, i)
			if !ok || !goPtrResultElemCompatible(goPtrResultInfo{elemRustType: argElemRustType}, paramInfo) {
				continue
			}
			if result[funcLit] == nil {
				result[funcLit] = map[int]goPtrResultInfo{}
			}
			result[funcLit][i] = paramInfo
		}
		return true
	})
	if len(result) == 0 {
		return nil
	}
	return result
}

func callFuncLitForGoPtrParamInfo(info *types.Info, localFuncLits map[types.Object]*ast.FuncLit, call *ast.CallExpr) *ast.FuncLit {
	if call == nil {
		return nil
	}
	if funcLit, ok := unwrapParens(call.Fun).(*ast.FuncLit); ok {
		return funcLit
	}
	ident, ok := unwrapParens(call.Fun).(*ast.Ident)
	if !ok || info == nil {
		return nil
	}
	obj := info.Uses[ident]
	if obj == nil {
		obj = info.Defs[ident]
	}
	return localFuncLits[obj]
}

func funcLitPointerParamGoPtrInfo(funcLit *ast.FuncLit, paramIndex int) (goPtrResultInfo, bool) {
	if funcLit == nil || funcLit.Type == nil || funcLit.Type.Params == nil || paramIndex < 0 {
		return goPtrResultInfo{}, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return goPtrResultInfo{}, false
	}
	seen := 0
	for _, field := range funcLit.Type.Params.List {
		if field == nil {
			continue
		}
		count := len(field.Names)
		if count == 0 {
			count = 1
		}
		for i := 0; i < count; i++ {
			if seen == paramIndex {
				return goPtrInfoForPointerType(typeInfo.GetType(field.Type))
			}
			seen++
		}
	}
	return goPtrResultInfo{}, false
}

func funcLitGoPtrParamInfo(funcLit *ast.FuncLit, paramIndex int) (goPtrResultInfo, bool) {
	if funcLit == nil || paramIndex < 0 || currentFuncLitGoPtrParamInfos == nil {
		return goPtrResultInfo{}, false
	}
	params := currentFuncLitGoPtrParamInfos[funcLit]
	if params == nil {
		return goPtrResultInfo{}, false
	}
	info, ok := params[paramIndex]
	return info, ok
}

func withFuncLitGoPtrParamInfos(funcLit *ast.FuncLit, infos map[int]goPtrResultInfo, emit func()) {
	if funcLit == nil || len(infos) == 0 || emit == nil {
		if emit != nil {
			emit()
		}
		return
	}
	old := currentFuncLitGoPtrParamInfos
	next := make(map[*ast.FuncLit]map[int]goPtrResultInfo, len(old)+1)
	for lit, params := range old {
		next[lit] = params
	}
	merged := make(map[int]goPtrResultInfo)
	if old != nil {
		if existing := old[funcLit]; existing != nil {
			for index, info := range existing {
				merged[index] = info
			}
		}
	}
	for index, info := range infos {
		merged[index] = info
	}
	next[funcLit] = merged
	currentFuncLitGoPtrParamInfos = next
	defer func() {
		currentFuncLitGoPtrParamInfos = old
	}()
	emit()
}

func registerFunctionValueGoPtrParamsFromFiles(files []*ast.File) {
	registerDirectFunctionValueGoPtrParamsFromFiles(files)
	for propagateFunctionValueGoPtrParamsFromFilesPass(files) {
	}
}

func registerDirectFunctionValueGoPtrParamsFromFiles(files []*ast.File) {
	ctx := GetTranspileContext()
	typeInfo := GetTypeInfo()
	if ctx == nil || ctx.Package == nil || typeInfo == nil || typeInfo.info == nil {
		return
	}
	if ctx.Package.FunctionValueGoPtrParamObjs == nil {
		ctx.Package.FunctionValueGoPtrParamObjs = make(map[types.Object]map[int]goPtrResultInfo)
	}

	for _, file := range files {
		if file == nil {
			continue
		}
		for _, decl := range file.Decls {
			fnDecl, ok := decl.(*ast.FuncDecl)
			if !ok || fnDecl.Body == nil {
				continue
			}
			currentFn, _ := sliceElemPtrReturnFuncObject(fnDecl)
			goPtrCandidates := collectGoPtrCandidatesForFunc(fnDecl)
			ast.Inspect(fnDecl.Body, func(node ast.Node) bool {
				call, ok := node.(*ast.CallExpr)
				if !ok {
					return true
				}
				obj, sig, ok := functionValueCallObjectAndSignature(typeInfo, call)
				if !ok {
					return true
				}
				for i, arg := range call.Args {
					argElemRustType, ok := goPtrArgElemRustType(arg, currentFn, nil, nil, goPtrCandidates)
					if !ok {
						continue
					}
					paramInfo, ok := signaturePointerParamGoPtrInfo(sig, i)
					if !ok || !goPtrResultElemCompatible(goPtrResultInfo{elemRustType: argElemRustType}, paramInfo) {
						continue
					}
					registerFunctionValueGoPtrParamObject(obj, i, paramInfo)
				}
				return true
			})
		}
	}
}

func propagateFunctionValueGoPtrParamsFromFilesPass(files []*ast.File) bool {
	ctx := GetTranspileContext()
	typeInfo := GetTypeInfo()
	if ctx == nil || ctx.Package == nil || typeInfo == nil || typeInfo.info == nil {
		return false
	}

	changed := false
	for _, file := range files {
		if file == nil {
			continue
		}
		for _, decl := range file.Decls {
			fnDecl, ok := decl.(*ast.FuncDecl)
			if !ok || fnDecl.Body == nil {
				continue
			}
			ast.Inspect(fnDecl.Body, func(node ast.Node) bool {
				switch n := node.(type) {
				case *ast.CallExpr:
					if propagateFunctionValueGoPtrParamsFromCall(typeInfo, n) {
						changed = true
					}
				case *ast.AssignStmt:
					if propagateFunctionValueGoPtrParamsFromAssignment(typeInfo, n) {
						changed = true
					}
				}
				return true
			})
		}
	}
	return changed
}

func functionValueCallObjectAndSignature(typeInfo *TypeInfo, call *ast.CallExpr) (types.Object, *types.Signature, bool) {
	if call == nil {
		return nil, nil, false
	}
	return functionValueExprObjectAndSignature(typeInfo, call.Fun)
}

func functionValueExprObjectAndSignature(typeInfo *TypeInfo, expr ast.Expr) (types.Object, *types.Signature, bool) {
	if typeInfo == nil || typeInfo.info == nil || expr == nil {
		return nil, nil, false
	}
	fun := unwrapParens(expr)
	var obj types.Object
	switch e := fun.(type) {
	case *ast.Ident:
		obj = typeInfo.GetObject(e)
		if _, isFunc := obj.(*types.Func); isFunc {
			return nil, nil, false
		}
	case *ast.SelectorExpr:
		selection := typeInfo.info.Selections[e]
		if selection == nil || selection.Kind() != types.FieldVal {
			return nil, nil, false
		}
		obj = selection.Obj()
	default:
		return nil, nil, false
	}
	if obj == nil {
		return nil, nil, false
	}
	sig, ok := signatureFromType(typeInfo.GetType(fun))
	if !ok || sig == nil {
		return nil, nil, false
	}
	return obj, sig, true
}

func propagateFunctionValueGoPtrParamsFromCall(typeInfo *TypeInfo, call *ast.CallExpr) bool {
	callee, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
	if !ok {
		return false
	}
	sig, ok := signatureFromType(callee.Type())
	if !ok || sig.Params() == nil {
		return false
	}
	changed := false
	for i, arg := range call.Args {
		if i >= sig.Params().Len() {
			continue
		}
		sourceObj := sig.Params().At(i)
		if propagateFunctionValueGoPtrParamsBetweenObjects(typeInfo, sourceObj, arg) {
			changed = true
		}
	}
	return changed
}

func propagateFunctionValueGoPtrParamsFromAssignment(typeInfo *TypeInfo, assign *ast.AssignStmt) bool {
	if assign == nil {
		return false
	}
	changed := false
	for i, lhs := range assign.Lhs {
		if i >= len(assign.Rhs) {
			break
		}
		rhs := assign.Rhs[i]
		lhsObj, _, lhsOK := functionValueExprObjectAndSignature(typeInfo, lhs)
		rhsObj, _, rhsOK := functionValueExprObjectAndSignature(typeInfo, rhs)
		if lhsOK && rhsOK {
			if propagateFunctionValueGoPtrParamsFromInfos(lhsObj, rhsObj) {
				changed = true
			}
			if propagateFunctionValueGoPtrParamsFromInfos(rhsObj, lhsObj) {
				changed = true
			}
		}
	}
	return changed
}

func propagateFunctionValueGoPtrParamsBetweenObjects(typeInfo *TypeInfo, sourceObj types.Object, targetExpr ast.Expr) bool {
	targetObj, _, ok := functionValueExprObjectAndSignature(typeInfo, targetExpr)
	if !ok {
		return false
	}
	return propagateFunctionValueGoPtrParamsFromInfos(sourceObj, targetObj)
}

func propagateFunctionValueGoPtrParamsFromInfos(sourceObj, targetObj types.Object) bool {
	sourceInfos, ok := functionValueGoPtrParamInfosForObject(sourceObj)
	if !ok {
		return false
	}
	_, targetSig, ok := functionValueObjectSignature(targetObj)
	if !ok {
		return false
	}
	changed := false
	for index, info := range sourceInfos {
		paramInfo, ok := signaturePointerParamGoPtrInfo(targetSig, index)
		if !ok || !goPtrResultElemCompatible(info, paramInfo) {
			continue
		}
		if registerFunctionValueGoPtrParamObject(targetObj, index, paramInfo) {
			changed = true
		}
	}
	return changed
}

func functionValueObjectSignature(obj types.Object) (types.Object, *types.Signature, bool) {
	if obj == nil {
		return nil, nil, false
	}
	sig, ok := signatureFromType(obj.Type())
	if !ok {
		return nil, nil, false
	}
	return obj, sig, true
}

func signaturePointerParamGoPtrInfo(sig *types.Signature, paramIndex int) (goPtrResultInfo, bool) {
	if sig == nil || sig.Params() == nil || paramIndex < 0 || paramIndex >= sig.Params().Len() {
		return goPtrResultInfo{}, false
	}
	return goPtrInfoForPointerType(sig.Params().At(paramIndex).Type())
}

func registerFunctionValueGoPtrParamObject(obj types.Object, paramIndex int, info goPtrResultInfo) bool {
	ctx := GetTranspileContext()
	if obj == nil || paramIndex < 0 || goPtrResultElemRustType(info) == "" || ctx == nil || ctx.Package == nil {
		return false
	}
	if ctx.Package.FunctionValueGoPtrParamObjs == nil {
		ctx.Package.FunctionValueGoPtrParamObjs = make(map[types.Object]map[int]goPtrResultInfo)
	}
	params := ctx.Package.FunctionValueGoPtrParamObjs[obj]
	if params == nil {
		params = map[int]goPtrResultInfo{}
		ctx.Package.FunctionValueGoPtrParamObjs[obj] = params
	}
	if existing, ok := params[paramIndex]; ok {
		if goPtrResultElemCompatible(existing, info) || goPtrResultElemRustType(existing) == goPtrResultElemRustType(info) {
			return false
		}
	}
	params[paramIndex] = info
	return true
}

func functionValueGoPtrParamInfosForObject(obj types.Object) (map[int]goPtrResultInfo, bool) {
	ctx := GetTranspileContext()
	if obj == nil || ctx == nil || ctx.Package == nil || ctx.Package.FunctionValueGoPtrParamObjs == nil {
		return nil, false
	}
	params := ctx.Package.FunctionValueGoPtrParamObjs[obj]
	return params, len(params) > 0
}

func functionValueGoPtrParamInfoForObject(obj types.Object, paramIndex int) (goPtrResultInfo, bool) {
	params, ok := functionValueGoPtrParamInfosForObject(obj)
	if !ok {
		return goPtrResultInfo{}, false
	}
	info, ok := params[paramIndex]
	return info, ok
}

func functionValueGoPtrAwareBoxTypeForObject(obj types.Object, sig *types.Signature) (string, bool) {
	infos, ok := functionValueGoPtrParamInfosForObject(obj)
	if !ok || sig == nil {
		return "", false
	}
	return signatureToGoPtrAwareBoxDynFn(sig, infos, goTypesParamTypeToRust), true
}

func functionValueGoPtrAwareWrappedTypeForObject(obj types.Object, sig *types.Signature) (string, bool) {
	boxType, ok := functionValueGoPtrAwareBoxTypeForObject(obj, sig)
	if !ok {
		return "", false
	}
	return goTypesWrappedRustType(boxType), true
}

func functionValueGoPtrAwareBoxTypeForNamedTypeExpr(name *ast.Ident, typ ast.Expr) (string, bool) {
	typeInfo := GetTypeInfo()
	if name == nil || typ == nil || typeInfo == nil || typeInfo.info == nil {
		return "", false
	}
	obj := typeInfo.GetObject(name)
	if obj == nil {
		return "", false
	}
	sig, ok := signatureFromType(typeInfo.GetType(typ))
	if !ok {
		return "", false
	}
	return functionValueGoPtrAwareBoxTypeForObject(obj, sig)
}

func functionValueGoPtrAwareWrappedTypeForNamedTypeExpr(name *ast.Ident, typ ast.Expr) (string, bool) {
	typeInfo := GetTypeInfo()
	if name == nil || typ == nil || typeInfo == nil || typeInfo.info == nil {
		return "", false
	}
	obj := typeInfo.GetObject(name)
	if obj == nil {
		return "", false
	}
	sig, ok := signatureFromType(typeInfo.GetType(typ))
	if !ok {
		return "", false
	}
	return functionValueGoPtrAwareWrappedTypeForObject(obj, sig)
}

func functionValueGoPtrAwareWrappedTypeForFuncDeclParam(fn *ast.FuncDecl, paramIndex int, typ ast.Expr) (string, bool) {
	if fn == nil || fn.Type == nil || fn.Type.Params == nil || paramIndex < 0 {
		return "", false
	}
	seen := 0
	for _, field := range fn.Type.Params.List {
		if field == nil {
			continue
		}
		if len(field.Names) == 0 {
			if seen == paramIndex {
				return "", false
			}
			seen++
			continue
		}
		for _, name := range field.Names {
			if seen == paramIndex {
				return functionValueGoPtrAwareWrappedTypeForNamedTypeExpr(name, typ)
			}
			seen++
		}
	}
	return "", false
}

func functionValueGoPtrAwareBoxTypeForExprObject(expr ast.Expr) (string, bool) {
	typeInfo := GetTypeInfo()
	if expr == nil || typeInfo == nil || typeInfo.info == nil {
		return "", false
	}
	var obj types.Object
	switch e := unwrapParens(expr).(type) {
	case *ast.Ident:
		obj = typeInfo.GetObject(e)
	case *ast.SelectorExpr:
		selection := typeInfo.info.Selections[e]
		if selection == nil || selection.Kind() != types.FieldVal {
			return "", false
		}
		obj = selection.Obj()
	default:
		return "", false
	}
	sig, ok := signatureFromType(typeInfo.GetType(expr))
	if !ok {
		return "", false
	}
	return functionValueGoPtrAwareBoxTypeForObject(obj, sig)
}

func sourceFunctionHasBody(fn *types.Func) bool {
	if fn == nil || sourceFunctionDeclsByFunc == nil {
		return false
	}
	info, ok := sourceFunctionDeclInfoForFunc(fn)
	return ok && info.decl != nil && info.decl.Body != nil
}

func registerGoPtrParam(fn *types.Func, paramIndex int, callElemRustType string, declElemRustType string) bool {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil || ctx.Package == nil {
		return false
	}
	if declElemRustType == "" {
		declElemRustType = callElemRustType
	}
	if ctx.Package.GoPtrParamFuncs == nil {
		ctx.Package.GoPtrParamFuncs = make(map[*types.Func]map[int]string)
	}
	sourceFn, _, hasSourceFn := sourceFunctionDeclObjectForFunc(fn)
	primaryElemRustType := callElemRustType
	if hasSourceFn && sourceFn == fn {
		primaryElemRustType = declElemRustType
	}
	changed := registerGoPtrParamObject(ctx.Package.GoPtrParamFuncs, fn, paramIndex, primaryElemRustType)
	if hasSourceFn && sourceFn != fn {
		if registerGoPtrParamObject(ctx.Package.GoPtrParamFuncs, sourceFn, paramIndex, declElemRustType) {
			changed = true
		}
	}
	if ctx.Session != nil {
		if ctx.Session.GoPtrParamFuncNames == nil {
			ctx.Session.GoPtrParamFuncNames = make(map[string]map[int]string)
		}
		if registerGoPtrParamName(ctx.Session.GoPtrParamFuncNames, fn.FullName(), paramIndex, primaryElemRustType) {
			changed = true
		}
		if key := methodOverrideKey(fn); key != "" {
			if registerGoPtrParamName(ctx.Session.GoPtrParamFuncNames, key, paramIndex, primaryElemRustType) {
				changed = true
			}
		}
		if hasSourceFn && sourceFn != fn {
			if registerGoPtrParamName(ctx.Session.GoPtrParamFuncNames, sourceFn.FullName(), paramIndex, declElemRustType) {
				changed = true
			}
			if key := methodOverrideKey(sourceFn); key != "" {
				if setGoPtrParamName(ctx.Session.GoPtrParamFuncNames, key, paramIndex, declElemRustType) {
					changed = true
				}
			}
		}
	}
	return changed
}

func registerGoPtrSlotParam(fn *types.Func, paramIndex int, callElemRustType string, declElemRustType string) bool {
	ctx := GetTranspileContext()
	if fn == nil || ctx == nil || ctx.Package == nil {
		return false
	}
	if declElemRustType == "" {
		declElemRustType = callElemRustType
	}
	if ctx.Package.GoPtrSlotParamFuncs == nil {
		ctx.Package.GoPtrSlotParamFuncs = make(map[*types.Func]map[int]string)
	}
	sourceFn, _, hasSourceFn := sourceFunctionDeclObjectForFunc(fn)
	primaryElemRustType := callElemRustType
	if hasSourceFn && sourceFn == fn {
		primaryElemRustType = declElemRustType
	}
	changed := registerGoPtrParamObject(ctx.Package.GoPtrSlotParamFuncs, fn, paramIndex, primaryElemRustType)
	if hasSourceFn && sourceFn != fn {
		if registerGoPtrParamObject(ctx.Package.GoPtrSlotParamFuncs, sourceFn, paramIndex, declElemRustType) {
			changed = true
		}
	}
	if ctx.Session != nil {
		if ctx.Session.GoPtrSlotParamFuncNames == nil {
			ctx.Session.GoPtrSlotParamFuncNames = make(map[string]map[int]string)
		}
		if registerGoPtrParamName(ctx.Session.GoPtrSlotParamFuncNames, fn.FullName(), paramIndex, primaryElemRustType) {
			changed = true
		}
		if key := methodOverrideKey(fn); key != "" {
			if registerGoPtrParamName(ctx.Session.GoPtrSlotParamFuncNames, key, paramIndex, primaryElemRustType) {
				changed = true
			}
		}
		if hasSourceFn && sourceFn != fn {
			if registerGoPtrParamName(ctx.Session.GoPtrSlotParamFuncNames, sourceFn.FullName(), paramIndex, declElemRustType) {
				changed = true
			}
			if key := methodOverrideKey(sourceFn); key != "" {
				if setGoPtrParamName(ctx.Session.GoPtrSlotParamFuncNames, key, paramIndex, declElemRustType) {
					changed = true
				}
			}
		}
	}
	return changed
}

func registerGoPtrParamObject(registry map[*types.Func]map[int]string, fn *types.Func, paramIndex int, elemRustType string) bool {
	if registry == nil || fn == nil {
		return false
	}
	params := registry[fn]
	if params == nil {
		params = map[int]string{}
		registry[fn] = params
	}
	if _, ok := params[paramIndex]; ok {
		return false
	}
	params[paramIndex] = elemRustType
	return true
}

func setGoPtrParamName(registry map[string]map[int]string, fnName string, paramIndex int, elemRustType string) bool {
	if registry == nil || fnName == "" {
		return false
	}
	params := registry[fnName]
	if params == nil {
		params = map[int]string{}
		registry[fnName] = params
	}
	if existing, ok := params[paramIndex]; ok && existing == elemRustType {
		return false
	}
	params[paramIndex] = elemRustType
	return true
}

func registerGoPtrParamName(registry map[string]map[int]string, fnName string, paramIndex int, elemRustType string) bool {
	if registry == nil || fnName == "" {
		return false
	}
	params := registry[fnName]
	if params == nil {
		params = map[int]string{}
		registry[fnName] = params
	}
	if _, ok := params[paramIndex]; ok {
		return false
	}
	params[paramIndex] = elemRustType
	return true
}

func goPtrCallParamElemRustType(fn *types.Func, paramIndex int) (string, bool) {
	if fn == nil || paramIndex < 0 {
		return "", false
	}
	sig, ok := signatureFromType(fn.Type())
	if !ok || sig.Params() == nil || paramIndex >= sig.Params().Len() {
		return "", false
	}
	ptr, ok := types.Unalias(sig.Params().At(paramIndex).Type()).Underlying().(*types.Pointer)
	if !ok {
		return "", false
	}
	return goTypesTypeToRust(ptr.Elem()), true
}

func goPtrSlotCallParamElemRustType(fn *types.Func, paramIndex int) (string, bool) {
	if fn == nil || paramIndex < 0 {
		return "", false
	}
	sig, ok := signatureFromType(fn.Type())
	if !ok || sig.Params() == nil || paramIndex >= sig.Params().Len() {
		return "", false
	}
	info, ok := goPtrSlotInfoForPointerToPointerType(sig.Params().At(paramIndex).Type())
	if !ok {
		return "", false
	}
	return goPtrResultElemRustType(info), true
}

func goPtrSlotInfoForPointerToPointerType(typ types.Type) (goPtrResultInfo, bool) {
	ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	if !ok {
		return goPtrResultInfo{}, false
	}
	innerPtr, ok := types.Unalias(ptr.Elem()).Underlying().(*types.Pointer)
	if !ok {
		return goPtrResultInfo{}, false
	}
	return goPtrInfoForPointerType(innerPtr)
}

func goPtrSlotArgElemRustType(arg ast.Expr, currentFn *types.Func, goPtrSlotCandidates map[types.Object]goPtrResultInfo, goPtrCandidates map[types.Object]goPtrResultInfo) (string, bool) {
	if info, ok := goPtrSlotValueInfo(arg); ok {
		return goPtrResultElemRustType(info), true
	}
	if call, ok := unwrapParens(arg).(*ast.CallExpr); ok {
		if info, ok := goPtrResultInfoForCall(call, 0); ok {
			return goPtrSlotElemRustTypeFromGoPtrInfo(info)
		}
	}
	ident, ok := unwrapParens(arg).(*ast.Ident)
	if !ok || ident.Name == "nil" || ident.Name == "_" {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", false
	}
	obj := typeInfo.GetObject(ident)
	if obj == nil {
		return "", false
	}
	info, ok := goPtrSlotCandidates[obj]
	if ok {
		return goPtrResultElemRustType(info), true
	}
	if info, ok := goPtrCandidates[obj]; ok {
		return goPtrSlotElemRustTypeFromGoPtrInfo(info)
	}
	if elemRustType, ok := goPtrSlotParamElemRustTypeForObject(currentFn, obj); ok {
		return elemRustType, true
	}
	if elemRustType, ok := goPtrParamElemRustTypeForObject(currentFn, obj); ok {
		return goPtrSlotElemRustTypeFromGoPtrElemRustType(elemRustType)
	}
	if elemRustType, ok := goPtrVarElemRustType(ident.Name); ok {
		return goPtrSlotElemRustTypeFromGoPtrElemRustType(elemRustType)
	}
	return "", false
}

func goPtrSlotElemRustTypeFromGoPtrInfo(info goPtrResultInfo) (string, bool) {
	return goPtrSlotElemRustTypeFromGoPtrElemRustType(goPtrResultElemRustType(info))
}

func goPtrSlotElemRustTypeFromGoPtrElemRustType(elemRustType string) (string, bool) {
	if !strings.HasPrefix(elemRustType, "GoPtr<") || !strings.HasSuffix(elemRustType, ">") {
		return "", false
	}
	return strings.TrimSuffix(strings.TrimPrefix(elemRustType, "GoPtr<"), ">"), true
}

func goPtrArgElemRustType(arg ast.Expr, currentFn *types.Func, sliceCandidates map[types.Object]string, arrayCandidates map[types.Object]arrayElemPtrInfo, goPtrCandidates map[types.Object]goPtrResultInfo) (string, bool) {
	if elemRustType, ok := sliceElemPtrAddressElemRustType(arg); ok {
		return elemRustType, true
	}
	if elemRustType, ok := arrayElemPtrAddressElemRustType(arg); ok {
		return elemRustType, true
	}
	if call, ok := unwrapParens(arg).(*ast.CallExpr); ok {
		if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
			return info.elemRustType, true
		}
		if info, ok := arrayElemPtrResultInfoForCall(call, 0); ok {
			return info.elemRustType, true
		}
		if info, ok := goPtrResultInfoForCall(call, 0); ok {
			return goPtrResultElemRustType(info), true
		}
	}
	if sel, ok := unwrapParens(arg).(*ast.SelectorExpr); ok {
		if info, ok := sliceElemPtrFieldInfoForSelector(sel); ok {
			return info.elemRustType, true
		}
	}
	ident, ok := unwrapParens(arg).(*ast.Ident)
	if !ok || ident.Name == "nil" || ident.Name == "_" {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil {
		if obj := typeInfo.GetObject(ident); obj != nil {
			if elemRustType, ok := goPtrParamElemRustTypeForObject(currentFn, obj); ok {
				return elemRustType, true
			}
			if elemRustType, ok := sliceCandidates[obj]; ok {
				return elemRustType, true
			}
			if info, ok := arrayCandidates[obj]; ok {
				return info.elemRustType, true
			}
			if info, ok := goPtrCandidates[obj]; ok && info.elemRustType != "" {
				return info.elemRustType, true
			}
		}
	}
	if info, ok := sliceElemPtrVarInfo(ident.Name); ok {
		if elemRustType := elemRustTypeFromSliceElemPtrRustType(info.RustType); elemRustType != "" {
			return elemRustType, true
		}
	}
	if info, ok := arrayElemPtrVarInfo(ident.Name); ok {
		if elemRustType := elemRustTypeFromArrayElemPtrRustType(info.RustType); elemRustType != "" {
			return elemRustType, true
		}
	}
	return "", false
}

func forwardedGoPtrArgElemRustType(arg ast.Expr, currentFn *types.Func) (string, bool) {
	ident, ok := unwrapParens(arg).(*ast.Ident)
	if !ok || ident.Name == "nil" || ident.Name == "_" {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return "", false
	}
	obj := typeInfo.GetObject(ident)
	if elemRustType, ok := goPtrParamElemRustTypeForObject(currentFn, obj); ok {
		return elemRustType, true
	}
	return "", false
}

func bodylessGoPtrArgElemRustType(arg ast.Expr, currentFn *types.Func, goPtrCandidates map[types.Object]goPtrResultInfo) (string, bool) {
	if elemRustType, ok := forwardedGoPtrArgElemRustType(arg, currentFn); ok {
		return elemRustType, true
	}
	switch expr := unwrapParens(arg).(type) {
	case *ast.CallExpr:
		if info, ok := goPtrResultInfoForCall(expr, 0); ok {
			return goPtrResultElemRustType(info), true
		}
	case *ast.Ident:
		if expr.Name == "nil" || expr.Name == "_" {
			return "", false
		}
		typeInfo := GetTypeInfo()
		if typeInfo != nil {
			if obj := typeInfo.GetObject(expr); obj != nil {
				if info, ok := goPtrCandidates[obj]; ok {
					return goPtrResultElemRustType(info), true
				}
			}
		}
	case *ast.SelectorExpr:
		if info, ok := sliceElemPtrFieldInfoForSelector(expr); ok && generatedGoPtrFieldForSelector(expr) {
			return sliceElemPtrFieldElemRustType(info), true
		}
	case *ast.IndexExpr:
		if info, ok := goPtrArrayFieldInfoForIndexExpr(expr); ok {
			return goPtrArrayFieldElemRustType(info), true
		}
	}
	return "", false
}

func goPtrParamElemRustTypeForObject(fn *types.Func, obj types.Object) (string, bool) {
	if fn == nil || obj == nil {
		return "", false
	}
	params, ok := goPtrParamInfosForFunc(fn)
	if !ok || len(params) == 0 {
		return "", false
	}
	sig, ok := signatureFromType(fn.Type())
	if !ok || sig.Params() == nil {
		return "", false
	}
	for index, elemRustType := range params {
		if index < 0 || index >= sig.Params().Len() {
			continue
		}
		if sig.Params().At(index) == obj {
			return elemRustType, true
		}
	}
	return "", false
}

func goPtrSlotParamElemRustTypeForObject(fn *types.Func, obj types.Object) (string, bool) {
	if fn == nil || obj == nil {
		return "", false
	}
	params, ok := goPtrSlotParamInfosForFunc(fn)
	if !ok {
		return "", false
	}
	sig, ok := signatureFromType(fn.Type())
	if !ok || sig.Params() == nil {
		return "", false
	}
	for index, elemRustType := range params {
		if index < 0 || index >= sig.Params().Len() {
			continue
		}
		if sig.Params().At(index) == obj {
			return elemRustType, true
		}
	}
	return "", false
}

func elemRustTypeFromSliceElemPtrRustType(rustType string) string {
	if strings.HasPrefix(rustType, "Option<GoSliceElemPtr<") && strings.HasSuffix(rustType, ">>") {
		return strings.TrimSuffix(strings.TrimPrefix(rustType, "Option<GoSliceElemPtr<"), ">>")
	}
	if strings.HasPrefix(rustType, "GoPtr<") && strings.HasSuffix(rustType, ">") {
		return strings.TrimSuffix(strings.TrimPrefix(rustType, "GoPtr<"), ">")
	}
	return ""
}

func goPtrVarElemRustType(name string) (string, bool) {
	info, ok := goPtrVarInfo(name)
	if !ok || info == nil {
		return "", false
	}
	elemRustType := elemRustTypeFromSliceElemPtrRustType(info.RustType)
	if elemRustType == "" {
		return "", false
	}
	return elemRustType, true
}

func goPtrInfoForLocalIdent(ident *ast.Ident, elemRustType string) goPtrResultInfo {
	info := goPtrResultInfo{elemRustType: elemRustType}
	if ident == nil {
		return info
	}
	if candidate, ok := goPtrCandidateForDecl(ident); ok {
		if candidate.elemType != nil {
			info.elemType = candidate.elemType
		}
		if info.elemRustType == "" {
			info.elemRustType = candidate.elemRustType
		}
	}
	if varInfo, ok := goPtrVarInfo(ident.Name); ok && varInfo.GoType != nil {
		if typedInfo, ok := goPtrInfoForPointerType(varInfo.GoType); ok {
			info.elemType = typedInfo.elemType
			if info.elemRustType == "" {
				info.elemRustType = typedInfo.elemRustType
			}
		}
	}
	if info.elemType == nil {
		if typeInfo := GetTypeInfo(); typeInfo != nil {
			if typedInfo, ok := goPtrInfoForPointerType(typeInfo.GetType(ident)); ok {
				info.elemType = typedInfo.elemType
				if info.elemRustType == "" {
					info.elemRustType = typedInfo.elemRustType
				}
			} else if obj := typeInfo.GetObject(ident); obj != nil {
				if typedInfo, ok := goPtrInfoForPointerType(obj.Type()); ok {
					info.elemType = typedInfo.elemType
					if info.elemRustType == "" {
						info.elemRustType = typedInfo.elemRustType
					}
				}
			}
		}
	}
	return info
}

func elemRustTypeFromArrayElemPtrRustType(rustType string) string {
	if !strings.HasPrefix(rustType, "Option<GoArrayElemPtr<") || !strings.HasSuffix(rustType, ">>") {
		return ""
	}
	inner := strings.TrimSuffix(strings.TrimPrefix(rustType, "Option<GoArrayElemPtr<"), ">>")
	if idx := strings.LastIndex(inner, ", "); idx >= 0 {
		return inner[:idx]
	}
	return ""
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
	prevResultInfos := currentSliceElemPtrResultInfos
	prevArray := currentArrayElemPtrReturnInfos
	prevGoPtr := currentGoPtrReturnInfos
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
	if infos := sliceElemPtrResultInfosForFunc(fn); len(infos) > 0 {
		currentSliceElemPtrResultInfos = infos
	} else {
		currentSliceElemPtrResultInfos = nil
	}
	if infos := arrayElemPtrResultInfosForFunc(fn); len(infos) > 0 {
		currentArrayElemPtrReturnInfos = infos
	} else {
		currentArrayElemPtrReturnInfos = nil
	}
	if infos, ok := registeredGoPtrResultInfosForDecl(fn); ok && len(infos) > 0 {
		currentGoPtrReturnInfos = infos
	} else if infos := goPtrResultInfosForFunc(fn); len(infos) > 0 {
		currentGoPtrReturnInfos = infos
	} else {
		currentGoPtrReturnInfos = nil
	}
	return func() {
		currentSliceElemPtrReturn = prev
		currentSliceElemPtrSliceReturn = prevSlice
		currentSliceElemPtrResultInfos = prevResultInfos
		currentArrayElemPtrReturnInfos = prevArray
		currentGoPtrReturnInfos = prevGoPtr
	}
}

func pushFuncLitReturnContext() func() {
	prev := currentSliceElemPtrReturn
	prevSlice := currentSliceElemPtrSliceReturn
	prevResultInfos := currentSliceElemPtrResultInfos
	prevArray := currentArrayElemPtrReturnInfos
	prevGoPtr := currentGoPtrReturnInfos
	currentSliceElemPtrReturn = nil
	currentSliceElemPtrSliceReturn = nil
	currentSliceElemPtrResultInfos = nil
	currentArrayElemPtrReturnInfos = nil
	currentGoPtrReturnInfos = nil
	return func() {
		currentSliceElemPtrReturn = prev
		currentSliceElemPtrSliceReturn = prevSlice
		currentSliceElemPtrResultInfos = prevResultInfos
		currentArrayElemPtrReturnInfos = prevArray
		currentGoPtrReturnInfos = prevGoPtr
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

func funcDeclParamObjectAt(fn *ast.FuncDecl, paramIndex int, typeInfo *TypeInfo) types.Object {
	if fn == nil || fn.Type == nil || fn.Type.Params == nil || typeInfo == nil || paramIndex < 0 {
		return nil
	}
	seen := 0
	for _, field := range fn.Type.Params.List {
		if field == nil {
			continue
		}
		count := len(field.Names)
		if count == 0 {
			count = 1
		}
		for i := 0; i < count; i++ {
			if seen == paramIndex {
				if i < len(field.Names) {
					return typeInfo.GetObject(field.Names[i])
				}
				return nil
			}
			seen++
		}
	}
	return nil
}

func collectGoPtrCandidatesForFunc(fn *ast.FuncDecl) map[types.Object]goPtrResultInfo {
	typeInfo := GetTypeInfo()
	if fn == nil || fn.Body == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	candidates := map[types.Object]*goPtrCandidate{}
	if fn.Type != nil && fn.Type.Results != nil {
		for _, field := range fn.Type.Results.List {
			for _, name := range field.Names {
				if name == nil || name.Name == "_" {
					continue
				}
				obj := typeInfo.GetObject(name)
				if obj == nil {
					continue
				}
				info, ok := goPtrInfoForPointerType(obj.Type())
				if !ok {
					continue
				}
				candidates[obj] = &goPtrCandidate{info: info, valid: true}
			}
		}
	}
	if fnObj, ok := sliceElemPtrReturnFuncObject(fn); ok {
		if params, ok := goPtrParamInfosForFunc(fnObj); ok && len(params) > 0 {
			if sig, ok := signatureFromType(fnObj.Type()); ok && sig.Params() != nil {
				for index, elemRustType := range params {
					if index < 0 || index >= sig.Params().Len() || elemRustType == "" {
						continue
					}
					param := sig.Params().At(index)
					if param == nil {
						continue
					}
					info, ok := goPtrParamResultInfoForFunc(fnObj, index)
					if !ok {
						info = goPtrResultInfo{elemRustType: elemRustType}
					}
					state := &goPtrCandidate{info: info, valid: true, sawGoPtr: true}
					candidates[param] = state
					if astParam := funcDeclParamObjectAt(fn, index, typeInfo); astParam != nil {
						candidates[astParam] = state
					}
				}
			}
		}
	}

	ast.Inspect(fn.Body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.ValueSpec:
			if n.Type == nil {
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
				info, ok := goPtrInfoForPointerType(obj.Type())
				if !ok {
					continue
				}
				state := &goPtrCandidate{info: info, valid: true}
				if len(n.Values) > i {
					rhsInfo, rhsOK, sawGoPtr := goPtrAssignmentValueInfo(n.Values[i], 0, candidates, typeInfo)
					if !rhsOK && goPtrAssignmentPointerCompatible(n.Values[i], obj.Type(), typeInfo) {
						rhsOK = true
					}
					state.valid = rhsOK && (!sawGoPtr || goPtrResultElemCompatible(rhsInfo, info))
					state.sawGoPtr = sawGoPtr
					if sawGoPtr {
						state.info = rhsInfo
					}
				}
				candidates[obj] = state
			}
		case *ast.AssignStmt:
			if n.Tok != token.DEFINE {
				return true
			}
			for i, lhs := range n.Lhs {
				ident, ok := unwrapParens(lhs).(*ast.Ident)
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
				info, ok := goPtrInfoForPointerType(obj.Type())
				if !ok {
					continue
				}
				rhsInfo, rhsOK, sawGoPtr := goPtrAssignmentValueInfoForStmt(n, i, candidates, typeInfo)
				if !rhsOK && goPtrAssignmentPointerCompatibleForStmt(n, i, obj.Type(), typeInfo) {
					rhsOK = true
				}
				if !rhsOK {
					continue
				}
				state := &goPtrCandidate{info: info, valid: true, sawGoPtr: sawGoPtr}
				if sawGoPtr {
					if !goPtrResultElemCompatible(rhsInfo, info) {
						continue
					}
					state.info = rhsInfo
				}
				candidates[obj] = state
			}
		}
		return true
	})

	ast.Inspect(fn.Body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			// Captured outer pointer locals can be assigned inside closures.
			// Inner locals have distinct go/types objects and are ignored below.
			return true
		case *ast.AssignStmt:
			if n.Tok != token.ASSIGN && n.Tok != token.DEFINE {
				return true
			}
			for i, lhs := range n.Lhs {
				ident, ok := unwrapParens(lhs).(*ast.Ident)
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
				rhsInfo, rhsOK, sawGoPtr := goPtrAssignmentValueInfoForStmt(n, i, candidates, typeInfo)
				if !rhsOK {
					if !goPtrAssignmentPointerCompatibleForStmt(n, i, obj.Type(), typeInfo) {
						state.valid = false
					}
					continue
				}
				if sawGoPtr {
					if !goPtrResultElemCompatible(rhsInfo, state.info) {
						state.valid = false
						continue
					}
					state.info = rhsInfo
					state.sawGoPtr = true
				}
			}
		}
		return true
	})

	result := map[types.Object]goPtrResultInfo{}
	for obj, state := range candidates {
		if state.valid && state.sawGoPtr && state.info.elemRustType != "" {
			result[obj] = state.info
		}
	}
	if len(result) == 0 {
		return nil
	}
	return result
}

func goPtrInfoForPointerType(typ types.Type) (goPtrResultInfo, bool) {
	ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	if !ok {
		return goPtrResultInfo{}, false
	}
	if elemRust, ok := goTypesNamedFunctionTypeToRust(ptr.Elem()); ok {
		return goPtrResultInfo{elemRustType: elemRust, elemType: ptr.Elem()}, true
	}
	if _, ok := types.Unalias(ptr.Elem()).Underlying().(*types.Pointer); ok {
		inner, ok := goPtrInfoForPointerType(ptr.Elem())
		if !ok {
			return goPtrResultInfo{}, false
		}
		return goPtrResultInfo{elemRustType: "GoPtr<" + goPtrResultElemRustType(inner) + ">"}, true
	}
	elem := coreType(ptr.Elem())
	return goPtrResultInfo{elemRustType: goTypesCollectionElemTypeToRust(elem), elemType: elem}, true
}

func methodNeedsGoPtrReceiverAlias(fn *ast.FuncDecl) (goPtrResultInfo, bool) {
	if fn == nil || fn.Body == nil || fn.Recv == nil || len(fn.Recv.List) == 0 || !methodReassignsReceiver(fn) {
		return goPtrResultInfo{}, false
	}
	recv := fn.Recv.List[0]
	if _, ok := recv.Type.(*ast.StarExpr); !ok {
		return goPtrResultInfo{}, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || len(recv.Names) == 0 {
		return goPtrResultInfo{}, false
	}
	recvObj := typeInfo.info.Defs[recv.Names[0]]
	if recvObj == nil {
		return goPtrResultInfo{}, false
	}
	expected, ok := goPtrInfoForPointerType(recvObj.Type())
	if !ok {
		return goPtrResultInfo{}, false
	}
	found := false
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		if found {
			return false
		}
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.AssignStmt:
			for i, lhs := range n.Lhs {
				ident, ok := unwrapParens(lhs).(*ast.Ident)
				if !ok || typeInfo.GetObject(ident) != recvObj {
					continue
				}
				rhs := assignmentRHSForLHS(n, i)
				if rhs == nil {
					continue
				}
				info, ok, sawGoPtr := goPtrAssignmentValueInfo(rhs, i, nil, typeInfo)
				if ok && sawGoPtr && goPtrResultElemCompatible(info, expected) {
					found = true
					return false
				}
			}
		}
		return true
	})
	if !found {
		return goPtrResultInfo{}, false
	}
	return expected, true
}

func goPtrAssignmentValueInfoForStmt(stmt *ast.AssignStmt, lhsIndex int, candidates map[types.Object]*goPtrCandidate, typeInfo *TypeInfo) (goPtrResultInfo, bool, bool) {
	if stmt == nil {
		return goPtrResultInfo{}, false, false
	}
	if len(stmt.Rhs) == 1 {
		return goPtrAssignmentValueInfo(stmt.Rhs[0], lhsIndex, candidates, typeInfo)
	}
	if lhsIndex < 0 || lhsIndex >= len(stmt.Rhs) {
		return goPtrResultInfo{}, false, false
	}
	return goPtrAssignmentValueInfo(stmt.Rhs[lhsIndex], 0, candidates, typeInfo)
}

func goPtrAssignmentPointerCompatibleForStmt(stmt *ast.AssignStmt, lhsIndex int, target types.Type, typeInfo *TypeInfo) bool {
	if stmt == nil {
		return false
	}
	if len(stmt.Rhs) == 1 && len(stmt.Lhs) > 1 && lhsIndex >= 0 && typeInfo != nil {
		typ := typeInfo.GetType(stmt.Rhs[0])
		if tuple, ok := types.Unalias(typ).(*types.Tuple); ok && lhsIndex < tuple.Len() {
			return goPtrPointerTypeAssignable(tuple.At(lhsIndex).Type(), target)
		}
	}
	return goPtrAssignmentPointerCompatible(assignmentRHSForLHS(stmt, lhsIndex), target, typeInfo)
}

func goPtrAssignmentPointerCompatible(expr ast.Expr, target types.Type, typeInfo *TypeInfo) bool {
	if expr == nil || target == nil || typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(expr)
	return goPtrPointerTypeAssignable(actual, target)
}

func goPtrPointerTypeAssignable(actual types.Type, target types.Type) bool {
	if actual == nil || target == nil {
		return false
	}
	if _, ok := types.Unalias(target).Underlying().(*types.Pointer); !ok {
		return false
	}
	return types.AssignableTo(actual, target)
}

func goPtrAssignmentValueInfo(expr ast.Expr, resultIndex int, candidates map[types.Object]*goPtrCandidate, typeInfo *TypeInfo) (goPtrResultInfo, bool, bool) {
	expr = unwrapParens(expr)
	if ident, ok := expr.(*ast.Ident); ok {
		if ident.Name == "nil" {
			return goPtrResultInfo{}, true, false
		}
		if typeInfo != nil {
			if obj := typeInfo.GetObject(ident); obj != nil {
				if state := candidates[obj]; state != nil && state.valid && state.sawGoPtr {
					return state.info, true, true
				}
				if currentGoPtrCandidates != nil {
					if info, ok := currentGoPtrCandidates[obj]; ok {
						return info, true, true
					}
				}
			}
		}
		if elemRustType, ok := goPtrVarElemRustType(ident.Name); ok {
			return goPtrResultInfo{elemRustType: elemRustType}, true, true
		}
		if info, ok := arrayElemPtrVarInfo(ident.Name); ok {
			if elemRustType := elemRustTypeFromArrayElemPtrRustType(info.RustType); elemRustType != "" {
				return goPtrResultInfo{elemRustType: elemRustType}, true, true
			}
		}
		if info, ok := sliceElemPtrVarInfo(ident.Name); ok {
			if elemRustType := elemRustTypeFromSliceElemPtrRustType(info.RustType); elemRustType != "" {
				return goPtrResultInfo{elemRustType: elemRustType}, true, true
			}
		}
		return goPtrResultInfo{}, false, false
	}
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if info, ok := sliceElemPtrFieldInfoForSelector(sel); ok {
			return goPtrResultInfo{elemRustType: info.elemRustType, elemType: info.elemType}, true, true
		}
	}
	if indexExpr, ok := expr.(*ast.IndexExpr); ok {
		if info, ok := goPtrArrayFieldInfoForIndexExpr(indexExpr); ok {
			return goPtrResultInfo{elemRustType: info.elemRustType, elemType: info.elemType}, true, true
		}
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		if resultIndex == 0 {
			if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
				return goPtrResultInfo{elemRustType: info.elemRustType, elemType: info.elemType}, true, true
			}
		}
		if info, ok := arrayElemPtrResultInfoForCall(call, resultIndex); ok {
			return goPtrResultInfo{elemRustType: info.elemRustType}, true, true
		}
		if info, ok := goPtrResultInfoForCall(call, resultIndex); ok {
			return info, true, true
		}
		if info, ok := goPtrRawPointerValueInfo(call, typeInfo); ok {
			return info, true, true
		}
		return goPtrResultInfo{}, false, false
	}
	if slotInfo, ok := goPtrSlotValueInfo(expr); ok {
		return goPtrResultInfo{elemRustType: "GoPtr<" + goPtrResultElemRustType(slotInfo) + ">"}, true, true
	}
	if elemType, elemRustType, ok := sliceElemPtrAddressElemType(expr); ok {
		return goPtrResultInfo{elemRustType: elemRustType, elemType: elemType}, true, true
	}
	if elemRustType, ok := arrayElemPtrAddressElemRustType(expr); ok {
		return goPtrResultInfo{elemRustType: elemRustType}, true, true
	}
	if info, ok := goPtrRawPointerValueInfo(expr, typeInfo); ok {
		return info, true, true
	}
	if info, ok := goPtrLocalPointerValueInfo(expr, typeInfo); ok {
		return info, true, false
	}
	return goPtrResultInfo{}, false, false
}

func goPtrLocalPointerValueInfo(expr ast.Expr, typeInfo *TypeInfo) (goPtrResultInfo, bool) {
	if expr == nil || typeInfo == nil {
		return goPtrResultInfo{}, false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return goPtrResultInfo{}, false
	}
	return goPtrInfoForPointerType(typ)
}

func goPtrReturnExprInfo(expr ast.Expr, resultIndex int, candidates map[types.Object]goPtrResultInfo, typeInfo *TypeInfo) (goPtrResultInfo, bool, bool) {
	expr = unwrapParens(expr)
	if ident, ok := expr.(*ast.Ident); ok {
		if ident.Name == "nil" {
			return goPtrResultInfo{}, false, true
		}
		if isCurrentReceiverIdent(ident) && typeInfo != nil {
			if info, ok := goPtrInfoForPointerType(typeInfo.GetType(ident)); ok {
				return info, false, true
			}
		}
		if typeInfo != nil {
			if obj := typeInfo.GetObject(ident); obj != nil {
				if info, ok := candidates[obj]; ok {
					return info, true, true
				}
			}
		}
		if elemRustType, ok := goPtrVarElemRustType(ident.Name); ok {
			return goPtrResultInfo{elemRustType: elemRustType}, true, true
		}
		if typeInfo != nil {
			if info, ok := goPtrInfoForPointerType(typeInfo.GetType(ident)); ok {
				return info, false, true
			}
		}
		return goPtrResultInfo{}, false, false
	}
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if info, ok := sliceElemPtrFieldInfoForSelector(sel); ok {
			return goPtrResultInfo{elemRustType: info.elemRustType, elemType: info.elemType}, true, true
		}
	}
	if indexExpr, ok := expr.(*ast.IndexExpr); ok {
		if info, ok := goPtrArrayFieldInfoForIndexExpr(indexExpr); ok {
			return goPtrResultInfo{elemRustType: info.elemRustType, elemType: info.elemType}, true, true
		}
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		if info, ok := goPtrResultInfoForCall(call, 0); ok {
			return info, true, true
		}
		if info, ok := goPtrRawPointerValueInfo(call, typeInfo); ok {
			return info, true, true
		}
	}
	if slotInfo, ok := goPtrSlotValueInfo(expr); ok {
		return goPtrResultInfo{elemRustType: "GoPtr<" + goPtrResultElemRustType(slotInfo) + ">"}, true, true
	}
	if info, ok := goPtrLocalAddressReturnInfo(expr, typeInfo); ok {
		return info, false, true
	}
	if typeInfo != nil {
		if info, ok := goPtrInfoForPointerType(typeInfo.GetType(expr)); ok {
			return info, false, true
		}
	}
	return goPtrResultInfo{}, false, false
}

func goPtrLocalAddressReturnInfo(expr ast.Expr, typeInfo *TypeInfo) (goPtrResultInfo, bool) {
	if typeInfo == nil {
		return goPtrResultInfo{}, false
	}
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return goPtrResultInfo{}, false
	}
	return goPtrInfoForPointerType(typeInfo.GetType(unary))
}

func goPtrRawPointerValueInfo(expr ast.Expr, typeInfo *TypeInfo) (goPtrResultInfo, bool) {
	if _, ok := goPtrRawPointerConversionSource(expr, typeInfo); !ok {
		return goPtrResultInfo{}, false
	}
	typ := typeInfo.GetType(expr)
	if pointerElemIsTypeParam(typ) {
		return goPtrResultInfo{}, false
	}
	return goPtrInfoForPointerType(typ)
}

func pointerElemIsTypeParam(typ types.Type) bool {
	if typ == nil {
		return false
	}
	ptr, ok := types.Unalias(typ).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	_, ok = types.Unalias(ptr.Elem()).(*types.TypeParam)
	return ok
}

func goPtrRawPointerConversionSource(expr ast.Expr, typeInfo *TypeInfo) (ast.Expr, bool) {
	if typeInfo == nil {
		return nil, false
	}
	call, ok := unwrapParens(expr).(*ast.CallExpr)
	if !ok || len(call.Args) != 1 || !typeInfo.IsTypeConversion(call) {
		return nil, false
	}
	if _, ok := types.Unalias(typeInfo.GetType(call)).Underlying().(*types.Pointer); !ok {
		return nil, false
	}
	source := call.Args[0]
	if !isUnsafePointerLikeType(typeInfo.GetType(source)) {
		return nil, false
	}
	return source, true
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
	sourceElemRustType := func(expr ast.Expr) (string, bool) {
		if sel, ok := unwrapParens(expr).(*ast.SelectorExpr); ok {
			if info, ok := sliceElemPtrSliceFieldInfoForSelector(sel); ok {
				return sliceElemPtrSliceFieldElemRustType(info), true
			}
		}
		if ident, ok := unwrapParens(expr).(*ast.Ident); ok {
			obj := typeInfo.GetObject(ident)
			if obj == nil {
				return "", false
			}
			if st := candidates[obj]; st != nil && st.valid && st.elemRustType != "" {
				return st.elemRustType, true
			}
		}
		return "", false
	}

	ast.Inspect(body, func(node ast.Node) bool {
		switch n := node.(type) {
		case *ast.FuncLit:
			return false
		case *ast.CallExpr:
			if !isBuiltinCallNamed(n, "copy") || len(n.Args) < 2 {
				return true
			}
			ident, ok := unwrapParens(n.Args[0]).(*ast.Ident)
			if !ok || ident.Name == "_" {
				return true
			}
			obj := typeInfo.GetObject(ident)
			if obj == nil {
				return true
			}
			elemRustType, ok := sliceElemPtrSliceElemRustTypeForType(obj.Type())
			if !ok {
				return true
			}
			srcElemRustType, ok := sourceElemRustType(n.Args[1])
			if !ok || srcElemRustType != elemRustType {
				return true
			}
			st := candidates[obj]
			if st == nil {
				st = &state{elemRustType: elemRustType, valid: true}
				candidates[obj] = st
			}
			st.sawSliceAddr = true
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
	if call, ok := expr.(*ast.CallExpr); ok {
		if _, ok := sliceElemPtrReturnInfoForCall(call); ok {
			return true, true
		}
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
	if call, ok := expr.(*ast.CallExpr); ok {
		if info, ok := arrayElemPtrResultInfoForCall(call, 0); ok {
			return info, true, true
		}
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
	if ident, ok := unwrapParens(indexExpr.X).(*ast.Ident); ok {
		if pointedArray, ok := arrayElemPtrIdentPointedArrayType(ident); ok {
			elemRustType := goTypesCollectionElemTypeToRust(coreType(pointedArray.Elem()))
			if elemRustType == "" {
				return arrayElemPtrInfo{}, false
			}
			return arrayElemPtrInfo{elemRustType: elemRustType, arrayLen: pointedArray.Len()}, true
		}
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
	return writeArrayElemPtrNewExpressionWithQualifier(out, indexExpr, "")
}

func writeArrayElemPtrNewExpressionWithQualifier(out *strings.Builder, indexExpr *ast.IndexExpr, helperQualifier string) bool {
	if ident, ok := unwrapParens(indexExpr.X).(*ast.Ident); ok && arrayElemPtrIdentPointsToArray(ident) {
		if helperQualifier == "" {
			writeArrayElemPtrFromPointedArrayNewExpression(out, ident, indexExpr.Index)
		} else {
			out.WriteString(`unimplemented!("cross-package array element address through pointer-to-array requires shared GoArrayElemPtr helpers")`)
		}
		return true
	}
	if writeGoPtrPointedArrayElemPtrNewExpressionWithQualifier(out, indexExpr, helperQualifier) {
		return true
	}
	if writeNestedArrayElemPtrNewExpressionWithQualifier(out, indexExpr, helperQualifier) {
		return true
	}
	if _, ok := arrayElemPtrAddressInfoForIndex(indexExpr); !ok {
		return false
	}
	if helperQualifier == "" {
		NeedSliceElemPtr()
	}
	if helperQualifier != "" {
		out.WriteString(helperQualifier)
		out.WriteString("::")
	}
	out.WriteString("GoArrayElemPtr::new(")
	writeArrayElemPtrSequenceHandle(out, indexExpr)
	out.WriteString(", ")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString(")")
	return true
}

func writeArrayElemPtrFromPointedArrayNewExpression(out *strings.Builder, ident *ast.Ident, index ast.Expr) {
	NeedSliceElemPtr()
	out.WriteString("GoArrayElemPtr::from_array_elem(")
	out.WriteString(RustIdentForUse(ident))
	out.WriteString(".as_ref().unwrap().clone(), ")
	writeExpressionAsUsize(out, index)
	out.WriteString(")")
}

func writeNestedArrayElemPtrNewExpressionWithQualifier(out *strings.Builder, indexExpr *ast.IndexExpr, helperQualifier string) bool {
	outerIndex, ok := unwrapParens(indexExpr.X).(*ast.IndexExpr)
	if !ok {
		return false
	}
	if _, ok := nestedArrayElemPtrAddressInfoForIndex(indexExpr); !ok {
		return false
	}
	if helperQualifier == "" {
		NeedSliceElemPtr()
	}
	if helperQualifier != "" {
		out.WriteString(helperQualifier)
		out.WriteString("::")
	}
	out.WriteString("GoArrayElemPtr::nested(")
	writeArrayElemPtrSequenceHandle(out, outerIndex)
	out.WriteString(", ")
	writeExpressionAsUsize(out, outerIndex.Index)
	out.WriteString(", ")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString(")")
	return true
}

func nestedArrayElemPtrAddressInfoForIndex(indexExpr *ast.IndexExpr) (arrayElemPtrInfo, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || indexExpr == nil {
		return arrayElemPtrInfo{}, false
	}
	outerIndex, ok := unwrapParens(indexExpr.X).(*ast.IndexExpr)
	if !ok {
		return arrayElemPtrInfo{}, false
	}
	if typeInfo.GetType(outerIndex.X) == nil || typeInfo.GetType(outerIndex) == nil || typeInfo.GetType(indexExpr) == nil {
		return arrayElemPtrInfo{}, false
	}
	if typeInfo.IsMap(outerIndex.X) || typeInfo.IsMap(outerIndex) {
		return arrayElemPtrInfo{}, false
	}
	outerArray, ok := arrayTypeForExpr(outerIndex.X, typeInfo)
	if !ok {
		return arrayElemPtrInfo{}, false
	}
	innerArray, ok := arrayTypeForExpr(indexExpr.X, typeInfo)
	if !ok {
		return arrayElemPtrInfo{}, false
	}
	outerElemArray, ok := coreUnderlyingType(outerArray.Elem()).(*types.Array)
	if !ok || outerElemArray.Len() != innerArray.Len() {
		return arrayElemPtrInfo{}, false
	}
	elemRustType := goTypesCollectionElemTypeToRust(coreType(innerArray.Elem()))
	if elemRustType == "" {
		return arrayElemPtrInfo{}, false
	}
	return arrayElemPtrInfo{elemRustType: elemRustType, arrayLen: innerArray.Len()}, true
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
	if call, ok := unwrapParens(rhs).(*ast.CallExpr); ok {
		if _, ok := arrayElemPtrResultInfoForCall(call, 0); ok {
			TranspileExpression(out, rhs)
			return true
		}
	}
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

func writeArrayElemPtrReturnValue(out *strings.Builder, result ast.Expr, resultIndex int) bool {
	if currentArrayElemPtrReturnInfos == nil {
		return false
	}
	if _, ok := currentArrayElemPtrReturnInfos[resultIndex]; !ok {
		return false
	}
	return writeArrayElemPtrOptionValue(out, result)
}

func writeGoPtrReturnValue(out *strings.Builder, result ast.Expr, resultIndex int) bool {
	info, ok := goPtrReturnInfoForFuncResult(nil, resultIndex)
	if !ok {
		return false
	}
	if !writeGoPtrCallArgumentForInfo(out, result, info) {
		out.WriteString(`unimplemented!("GoPtr return requires compatible pointer value")`)
	}
	return true
}

func goPtrReturnInfoForFuncResult(fn *ast.FuncDecl, resultIndex int) (goPtrResultInfo, bool) {
	if currentGoPtrReturnInfos != nil {
		if info, ok := currentGoPtrReturnInfos[resultIndex]; ok {
			return info, true
		}
	}
	if fn == nil {
		return goPtrResultInfo{}, false
	}
	infos := goPtrResultInfosForFunc(fn)
	info, ok := infos[resultIndex]
	return info, ok
}

func writeGoPtrNilReturnValue(out *strings.Builder, resultIndex int) bool {
	return writeGoPtrNilReturnValueForFunc(out, nil, resultIndex)
}

func writeGoPtrNilReturnValueForFunc(out *strings.Builder, fn *ast.FuncDecl, resultIndex int) bool {
	if _, ok := goPtrReturnInfoForFuncResult(fn, resultIndex); !ok {
		return false
	}
	NeedSliceElemPtr()
	out.WriteString("GoPtr::nil()")
	return true
}

func writeGoPtrLocalReturnValue(out *strings.Builder, resultIndex int, writeHandle func()) bool {
	return writeGoPtrLocalReturnValueForFunc(out, nil, resultIndex, writeHandle)
}

func writeGoPtrLocalReturnValueForFunc(out *strings.Builder, fn *ast.FuncDecl, resultIndex int, writeHandle func()) bool {
	if _, ok := goPtrReturnInfoForFuncResult(fn, resultIndex); !ok {
		return false
	}
	NeedSliceElemPtr()
	out.WriteString("GoPtr::local(")
	writeHandle()
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

func goPtrIdentPointedArrayType(ident *ast.Ident) (*types.Array, bool) {
	if ident == nil || !isGoPtrVar(ident.Name) {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	typ := typeInfo.GetType(ident)
	if typ == nil {
		if info, ok := goPtrVarInfo(ident.Name); ok {
			typ = info.GoType
		}
	}
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

func goPtrPointedArrayTypeForExpr(expr ast.Expr) (*types.Array, bool) {
	expr = unwrapParens(expr)
	if ident, ok := expr.(*ast.Ident); ok {
		return goPtrIdentPointedArrayType(ident)
	}
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok || !generatedGoPtrFieldForSelector(sel) {
		return nil, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, false
	}
	typ := typeInfo.GetType(sel)
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

func writeGoPtrPointedArrayHandle(out *strings.Builder, expr ast.Expr, cloneIdent bool) bool {
	expr = unwrapParens(expr)
	if ident, ok := expr.(*ast.Ident); ok {
		if _, ok := goPtrIdentPointedArrayType(ident); !ok {
			return false
		}
		out.WriteString(rustIdentForUseWithCapture(ident))
		if cloneIdent {
			out.WriteString(".clone()")
		}
		return true
	}
	sel, ok := expr.(*ast.SelectorExpr)
	if !ok {
		return false
	}
	if _, ok := goPtrPointedArrayTypeForExpr(sel); !ok {
		return false
	}
	return writeGeneratedGoPtrFieldHandleClone(out, sel)
}

func writeGoPtrPointedArrayElemPtrNewExpressionWithQualifier(out *strings.Builder, indexExpr *ast.IndexExpr, helperQualifier string) bool {
	if indexExpr == nil {
		return false
	}
	if _, ok := goPtrPointedArrayTypeForExpr(indexExpr.X); !ok {
		return false
	}
	if helperQualifier != "" {
		out.WriteString(`unimplemented!("cross-package array element address through GoPtr pointer-to-array requires shared GoArrayElemPtr helpers")`)
		return true
	}
	NeedSliceElemPtr()
	out.WriteString("GoArrayElemPtr::from_go_ptr(")
	writeGoPtrPointedArrayHandle(out, indexExpr.X, true)
	out.WriteString(", ")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString(")")
	return true
}

func writeGoPtrPointedArrayIndexValue(out *strings.Builder, expr ast.Expr, index ast.Expr) bool {
	if _, ok := goPtrPointedArrayTypeForExpr(expr); !ok {
		return false
	}
	out.WriteString("{ let __seq = ")
	writeGoPtrPointedArrayHandle(out, expr, false)
	out.WriteString(".borrow(); __seq.as_ref().unwrap()[")
	writeExpressionAsUsize(out, index)
	out.WriteString("].clone() }")
	return true
}

func writeGoPtrPointedNamedArrayIndexValue(out *strings.Builder, expr ast.Expr, index ast.Expr) bool {
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if !ok || !isGoPtrVar(ident.Name) {
		return false
	}
	named, _, ok := namedArrayTypeForExpr(expr)
	if !ok {
		return false
	}
	elemRustType, ok := goPtrVarElemRustType(ident.Name)
	if !ok || strings.HasPrefix(elemRustType, "[") {
		return false
	}
	out.WriteString("{ let __named_array = ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".borrow(); let __seq_holder = __named_array.as_ref().unwrap().0.clone(); let __seq_guard = __seq_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; let __seq = __seq_guard.as_ref().unwrap(); ")
	writeNamedArrayNestedInnerPeels(out, named)
	out.WriteString("__seq[")
	writeExpressionAsUsize(out, index)
	out.WriteString("].clone() }")
	return true
}

func writeGoPtrPointedArraySliceExpression(out *strings.Builder, slice *ast.SliceExpr) bool {
	if _, ok := goPtrPointedArrayTypeForExpr(slice.X); !ok {
		return false
	}
	WriteWrapperPrefix(out)
	out.WriteString("{ let __seq_ref = ")
	writeGoPtrPointedArrayHandle(out, slice.X, false)
	out.WriteString(".borrow(); let mut __seq = __seq_ref.as_ref().unwrap().clone()")
	writeSliceVecFromSeq(out, slice.Low, slice.High, slice.Max, "__seq.len()", false)
	WriteWrapperSuffix(out)
	return true
}

func writeGoPtrUnsafePointerIndexedElementAddress(out *strings.Builder, indexExpr *ast.IndexExpr) bool {
	ident, ok := unwrapParens(indexExpr.X).(*ast.Ident)
	if !ok {
		return false
	}
	if _, ok := goPtrIdentPointedArrayType(ident); !ok {
		return false
	}
	out.WriteString("{ let __seq = ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".borrow(); &__seq.as_ref().unwrap()[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("] as *const _ as usize }")
	return true
}

func writeGoPtrUnsafePointerIndexedElementValue(out *strings.Builder, indexExpr *ast.IndexExpr) bool {
	if _, ok := goPtrArrayFieldInfoForIndexExpr(indexExpr); !ok {
		return false
	}
	TranspileExpression(out, indexExpr)
	out.WriteString(".addr()")
	return true
}

func arrayElemPtrAddressElemRustType(expr ast.Expr) (string, bool) {
	info, ok := arrayElemPtrAddressInfo(expr)
	if !ok {
		return "", false
	}
	return info.elemRustType, true
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
		if _, ok := sliceElemPtrResultInfoForCall(call, 0); ok {
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

func writeSliceElemPtrResultReturnValue(out *strings.Builder, result ast.Expr, resultIndex int) bool {
	if currentSliceElemPtrResultInfos == nil {
		return false
	}
	if _, ok := currentSliceElemPtrResultInfos[resultIndex]; !ok {
		return false
	}
	return writeSliceElemPtrOptionValue(out, result)
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
		if info, ok := goPtrResultInfoForCall(call, 0); ok && goPtrResultElemCompatible(info, goPtrResultInfo{elemRustType: elemRustType}) {
			TranspileExpression(out, rhs)
			return true
		}
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
	if ident, ok := unwrapParens(rhs).(*ast.Ident); ok && isGoPtrVar(ident.Name) {
		if rhsElemRustType, ok := goPtrVarElemRustType(ident.Name); ok && rhsElemRustType == elemRustType {
			out.WriteString(rustIdentForUseWithCapture(ident))
			out.WriteString(".clone()")
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
	writePointerHandleAssignmentTargetFromValueName(out, sel, "new_val")
	out.WriteString(" }")
	return true
}

func writeGoPtrAssignment(out *strings.Builder, lhs ast.Expr, rhs ast.Expr) bool {
	ident, ok := unwrapParens(lhs).(*ast.Ident)
	if !ok || ident.Name == "_" || !isGoPtrVar(ident.Name) {
		return false
	}
	elemRustType, ok := goPtrVarElemRustType(ident.Name)
	if !ok {
		return false
	}
	info := goPtrInfoForLocalIdent(ident, elemRustType)
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(" = ")
	if !writeGoPtrCallArgumentForInfo(out, rhs, info) {
		out.WriteString(`unimplemented!("GoPtr assignment requires compatible pointer value")`)
	}
	return true
}

func writeTupleSliceElemPtrFieldAssignmentFromTemp(out *strings.Builder, lhs ast.Expr, tmpName string, call *ast.CallExpr, resultIndex int) bool {
	sel, ok := unwrapParens(lhs).(*ast.SelectorExpr)
	if !ok || call == nil {
		return false
	}
	fieldInfo, ok := sliceElemPtrFieldInfoForSelector(sel)
	if !ok {
		return false
	}
	helperPrefix := sliceElemPtrFieldHelperPrefix(fieldInfo)
	if resultIndex == 0 {
		if info, ok := sliceElemPtrReturnInfoForCall(call); ok {
			if !sliceElemPtrElemCompatible(info.elemType, info.elemRustType, fieldInfo) {
				return false
			}
			out.WriteString(" { let new_val = ")
			needSliceElemPtrFieldHelper(helperPrefix)
			out.WriteString(helperPrefix)
			out.WriteString("GoPtr::slice_elem_opt(")
			out.WriteString(tmpName)
			out.WriteString(".clone()); ")
			writePointerHandleAssignmentTargetFromValueName(out, sel, "new_val")
			out.WriteString(" }")
			return true
		}
	}
	if info, ok := arrayElemPtrResultInfoForCall(call, resultIndex); ok {
		if !sliceElemPtrElemCompatible(nil, info.elemRustType, fieldInfo) {
			return false
		}
		out.WriteString(" { let new_val = ")
		needSliceElemPtrFieldHelper(helperPrefix)
		out.WriteString(helperPrefix)
		out.WriteString("GoPtr::array_elem_opt(")
		out.WriteString(tmpName)
		out.WriteString(".clone()); ")
		writePointerHandleAssignmentTargetFromValueName(out, sel, "new_val")
		out.WriteString(" }")
		return true
	}
	return false
}

func writeTupleGoPtrAssignmentFromTemp(out *strings.Builder, lhs ast.Expr, tmpName string, call *ast.CallExpr, resultIndex int) bool {
	ident, ok := unwrapParens(lhs).(*ast.Ident)
	if !ok || ident.Name == "_" || call == nil || !isGoPtrVar(ident.Name) {
		return false
	}
	elemRustType, ok := goPtrVarElemRustType(ident.Name)
	if !ok {
		return false
	}
	out.WriteString(" ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(" = ")
	if info, ok := sliceElemPtrReturnInfoForCall(call); resultIndex == 0 && ok && info.elemRustType == elemRustType {
		out.WriteString("GoPtr::slice_elem_opt(")
		out.WriteString(tmpName)
		out.WriteString(".clone())")
	} else if info, ok := arrayElemPtrResultInfoForCall(call, resultIndex); ok && info.elemRustType == elemRustType {
		out.WriteString("GoPtr::array_elem_opt(")
		out.WriteString(tmpName)
		out.WriteString(".clone())")
	} else if info, ok := goPtrResultInfoForCall(call, resultIndex); ok && info.elemRustType == elemRustType {
		out.WriteString(tmpName)
		out.WriteString(".clone()")
	} else {
		return false
	}
	out.WriteString(";")
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
		if info, ok := arrayElemPtrResultInfoForCall(call, 0); ok {
			if !sliceElemPtrElemCompatible(nil, info.elemRustType, fieldInfo) {
				return false
			}
			needSliceElemPtrFieldHelper(helperPrefix)
			out.WriteString(helperPrefix)
			out.WriteString("GoPtr::array_elem_opt(")
			TranspileExpression(out, call)
			out.WriteString(")")
			return true
		}
		if info, ok := goPtrResultInfoForCall(call, 0); ok {
			if !sliceElemPtrElemCompatible(info.elemType, info.elemRustType, fieldInfo) || helperPrefix != "" {
				return false
			}
			TranspileExpression(out, call)
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
		if isArrayElemPtrVar(ident.Name) {
			needSliceElemPtrFieldHelper(helperPrefix)
			out.WriteString(helperPrefix)
			out.WriteString("GoPtr::array_elem_opt(")
			out.WriteString(RustIdentForUse(ident))
			out.WriteString(".clone())")
			return true
		}
		if elemRustType, ok := packageGlobalGoPtrElemRustType(ident); ok {
			typeInfo := GetTypeInfo()
			var elemType types.Type
			if typeInfo != nil {
				elemType, _ = sliceElemPtrPointerElemType(typeInfo.GetType(ident))
			}
			if !sliceElemPtrElemCompatible(elemType, elemRustType, fieldInfo) {
				return false
			}
			if helperPrefix != "" {
				writeGoPtrConversion(out, "", helperPrefix, func() {
					writePackageGlobalPointerHandleClone(out, ident)
				})
			} else {
				writePackageGlobalPointerHandleClone(out, ident)
			}
			return true
		}
		if elemRustType, ok := goPtrVarElemRustType(ident.Name); ok {
			typeInfo := GetTypeInfo()
			var elemType types.Type
			if typeInfo != nil {
				elemType, _ = sliceElemPtrPointerElemType(typeInfo.GetType(ident))
			}
			if !sliceElemPtrElemCompatible(elemType, elemRustType, fieldInfo) {
				return false
			}
			if helperPrefix != "" {
				writeQualifiedGoPtrVarConversion(out, ident, goPtrHelperQualifierForOwnerPackage(fieldInfo.ownerPkgPath))
			} else {
				out.WriteString(rustIdentForUseWithCapture(ident))
				out.WriteString(".clone()")
			}
			return true
		}
	}
	if sel, ok := unwrapParens(rhs).(*ast.SelectorExpr); ok {
		if rhsFieldInfo, ok := sliceElemPtrFieldInfoForSelector(sel); ok && generatedGoPtrFieldForSelector(sel) && sliceElemPtrElemCompatible(rhsFieldInfo.elemType, rhsFieldInfo.elemRustType, fieldInfo) {
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
	if rhsElemRustType, ok := arrayElemPtrAddressElemRustType(rhs); ok {
		if !sliceElemPtrElemCompatible(nil, rhsElemRustType, fieldInfo) {
			return false
		}
		needSliceElemPtrFieldHelper(helperPrefix)
		out.WriteString(helperPrefix)
		out.WriteString("GoPtr::array_elem(")
		TranspileExpression(out, rhs)
		out.WriteString(")")
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
	qualifier := goPtrHelperQualifierForOwnerPackage(info.ownerPkgPath)
	if qualifier == "" {
		return ""
	}
	return qualifier + "::"
}

func goPtrHelperQualifierForOwnerPackage(ownerPkgPath string) string {
	if ownerPkgPath == "" {
		return ""
	}
	typeInfo := GetTypeInfo()
	if typeInfo != nil && typeInfo.pkg != nil && typeInfo.pkg.Path() == ownerPkgPath {
		return ""
	}
	ctx := GetTranspileContext()
	if ctx == nil || ctx.PackageMapping == nil {
		return ""
	}
	crateName := ctx.PackageMapping[ownerPkgPath]
	if crateName == "" {
		return ""
	}
	TrackGeneratedCrateDependency(crateName)
	return crateName
}

func writeSliceElemPtrFieldNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	sel, ok := unwrapParens(expr).(*ast.SelectorExpr)
	if !ok {
		return false
	}
	if _, ok := sliceElemPtrFieldInfoForSelector(sel); !ok || !generatedGoPtrFieldForSelector(sel) {
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

func writeGoPtrNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	ident, ok := unwrapParens(expr).(*ast.Ident)
	if ok {
		if !isGoPtrVar(ident.Name) {
			return false
		}
		if op == token.NEQ {
			out.WriteString("!")
		}
		out.WriteString(rustIdentForUseWithCapture(ident))
		out.WriteString(".is_nil()")
		return true
	}

	call, ok := unwrapParens(expr).(*ast.CallExpr)
	if !ok {
		return false
	}
	if _, ok := goPtrResultInfoForCall(call, 0); !ok {
		return false
	}
	if op == token.NEQ {
		out.WriteString("!")
	}
	TranspileExpression(out, call)
	out.WriteString(".is_nil()")
	return true
}

func writeGoPtrSlotDerefNilComparison(out *strings.Builder, expr ast.Expr, op token.Token) bool {
	if op != token.EQL && op != token.NEQ {
		return false
	}
	star, ok := unwrapParens(expr).(*ast.StarExpr)
	if !ok {
		return false
	}
	ident, _, ok := goPtrSlotDerefInfo(star)
	if ok {
		out.WriteString("{ let __ptr_slot = ")
		out.WriteString(rustIdentForUseWithCapture(ident))
		WriteBorrowMethod(out, false)
		out.WriteString("; ")
		if op == token.NEQ {
			out.WriteString("!")
		}
		out.WriteString("__ptr_slot.as_ref().unwrap().is_nil() }")
		return true
	}
	ident, _, ok = goPtrPointerSlotDerefInfo(star)
	if !ok {
		return false
	}
	out.WriteString("{ let __ptr_slot = ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".borrow()")
	out.WriteString("; ")
	if op == token.NEQ {
		out.WriteString("!")
	}
	out.WriteString("__ptr_slot.as_ref().unwrap().is_nil() }")
	return true
}

func writeGoPtrPointerEquality(out *strings.Builder, expr *ast.BinaryExpr) bool {
	if expr == nil || expr.Op != token.EQL && expr.Op != token.NEQ {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !typeInfo.IsPointer(expr.X) || !typeInfo.IsPointer(expr.Y) {
		return false
	}
	if !goPtrAddressExpressionPreferred(expr.X) && !goPtrAddressExpressionPreferred(expr.Y) {
		return false
	}
	info, ok := goPtrInfoForPointerType(typeInfo.GetType(expr.X))
	if !ok {
		info, ok = goPtrInfoForPointerType(typeInfo.GetType(expr.Y))
	}
	if !ok {
		return false
	}
	var left strings.Builder
	if !writeGoPtrAddressExpression(&left, expr.X, info) {
		return false
	}
	var right strings.Builder
	if !writeGoPtrAddressExpression(&right, expr.Y, info) {
		return false
	}
	out.WriteString("{ let __left_addr = ")
	out.WriteString(left.String())
	out.WriteString("; let __right_addr = ")
	out.WriteString(right.String())
	out.WriteString("; let __eq = __left_addr == __right_addr; ")
	if expr.Op == token.NEQ {
		out.WriteString("!")
	}
	out.WriteString("__eq }")
	return true
}

func goPtrAddressExpressionPreferred(expr ast.Expr) bool {
	expr = unwrapParens(expr)
	switch e := expr.(type) {
	case *ast.Ident:
		return e.Name == "nil" || isGoPtrIdent(e)
	case *ast.CallExpr:
		_, ok := goPtrResultInfoForCall(e, 0)
		return ok
	case *ast.SelectorExpr:
		return generatedGoPtrFieldForSelector(e)
	default:
		return false
	}
}

func writeGoPtrAddressExpression(out *strings.Builder, expr ast.Expr, info goPtrResultInfo) bool {
	expr = unwrapParens(expr)
	if ident, ok := expr.(*ast.Ident); ok {
		if ident.Name == "nil" {
			out.WriteString("0")
			return true
		}
		if isGoPtrIdent(ident) {
			out.WriteString(rustIdentForUseWithCapture(ident))
			out.WriteString(".addr()")
			return true
		}
	}
	if call, ok := expr.(*ast.CallExpr); ok {
		if _, ok := goPtrResultInfoForCall(call, 0); ok {
			TranspileExpression(out, call)
			out.WriteString(".addr()")
			return true
		}
	}
	if sel, ok := expr.(*ast.SelectorExpr); ok {
		if generatedGoPtrFieldForSelector(sel) {
			if !writeGoPtrSelectorReadHandle(out, sel) {
				TranspileExpressionContext(out, sel, LValue)
			}
			out.WriteString(".addr()")
			return true
		}
	}
	out.WriteString("{ let __ptr = ")
	if !writeGoPtrCallArgumentForInfo(out, expr, info) {
		return false
	}
	out.WriteString("; __ptr.addr() }")
	return true
}

func writeSliceElemPtrFieldPointeeSelector(out *strings.Builder, base *ast.SelectorExpr, fieldInfo FieldAccessInfo, expr *ast.SelectorExpr, ctx ExprContext) bool {
	if _, ok := sliceElemPtrFieldInfoForSelector(base); !ok || !generatedGoPtrFieldForSelector(base) {
		return false
	}
	if fieldInfo.IsPromoted {
		if ctx != RValue {
			writeSliceElemPtrFieldPointeePromotedFieldHandle(out, base, fieldInfo)
			return true
		}
		out.WriteString("(*")
		writeSliceElemPtrFieldPointeePromotedFieldHandle(out, base, fieldInfo)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
		writeSelectorRValueClose(out, expr)
		return true
	}
	if ctx != RValue {
		writeSliceElemPtrFieldPointeeFieldHandle(out, base, fieldInfo)
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

func writeSliceElemPtrFieldPointeeFieldHandle(out *strings.Builder, base *ast.SelectorExpr, fieldInfo FieldAccessInfo) {
	out.WriteString("{ let __ptr_value = ")
	TranspileExpressionContext(out, base, LValue)
	out.WriteString(".with_mut(|__ptr_value| __ptr_value.")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone()); __ptr_value }")
}

func writeSliceElemPtrFieldPointeePromotedFieldHandle(out *strings.Builder, base *ast.SelectorExpr, fieldInfo FieldAccessInfo) {
	out.WriteString("{ let __ptr_value = ")
	TranspileExpressionContext(out, base, LValue)
	out.WriteString(".with_mut(|__ptr_value| { let __field = __ptr_value")
	for _, embedded := range fieldInfo.EmbeddedPath {
		out.WriteString(".")
		out.WriteString(ToSnakeCase(embedded))
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
	}
	out.WriteString(".")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone(); __field }); __ptr_value }")
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

func writeArrayElemPtrFieldHandle(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo) {
	out.WriteString("(*")
	writeArrayElemPtrBorrow(out, ident, false)
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

func writeArrayElemPtrFieldSelector(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo, sel *ast.SelectorExpr, ctx ExprContext) bool {
	if !isArrayElemPtrVar(ident.Name) {
		return false
	}
	if ctx == LValue || ctx == AddressOf {
		writeArrayElemPtrFieldHandle(out, ident, fieldInfo)
		return true
	}
	if typeInfoIsPointerExpr(sel) || selectorExpressionKeepsHandle(sel) {
		writeArrayElemPtrFieldHandle(out, ident, fieldInfo)
		out.WriteString(".clone()")
		return true
	}
	out.WriteString("(*")
	if NeedsConcurrentWrapper() {
		out.WriteString("{ let __field = ")
		writeArrayElemPtrFieldHandle(out, ident, fieldInfo)
		out.WriteString(".clone(); __field }")
	} else {
		writeArrayElemPtrFieldHandle(out, ident, fieldInfo)
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
		if ctx == LValue || ctx == AddressOf {
			writeGoPtrLocalPromotedFieldHandle(out, ident, fieldInfo)
			return true
		}
		if typeInfoIsPointerExpr(sel) || selectorExpressionKeepsHandle(sel) {
			writeGoPtrLocalFieldReadHandle(out, ident, fieldInfo)
			return true
		}
		out.WriteString("(*")
		writeGoPtrLocalFieldReadHandle(out, ident, fieldInfo)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
		writeSelectorRValueClose(out, sel)
		return true
	}
	if ctx == LValue || ctx == AddressOf {
		writeGoPtrLocalFieldHandle(out, ident, fieldInfo)
		return true
	}
	if typeInfoIsPointerExpr(sel) || selectorExpressionKeepsHandle(sel) {
		writeGoPtrLocalFieldReadHandle(out, ident, fieldInfo)
		return true
	}
	out.WriteString("(*{ let __ptr_value = ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".borrow(); __ptr_value.as_ref().unwrap().")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone() }")
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	writeSelectorRValueClose(out, sel)
	return true
}

func writeGoPtrExpressionFieldSelector(out *strings.Builder, base ast.Expr, fieldInfo FieldAccessInfo, sel *ast.SelectorExpr, ctx ExprContext) bool {
	if !goPtrExpressionReturnsHandle(base) {
		return false
	}
	if ctx == LValue || ctx == AddressOf {
		writeGoPtrExpressionFieldHandle(out, base, fieldInfo)
		return true
	}
	if typeInfoIsPointerExpr(sel) || selectorExpressionKeepsHandle(sel) {
		writeGoPtrExpressionFieldHandle(out, base, fieldInfo)
		return true
	}
	out.WriteString("(*")
	writeGoPtrExpressionFieldHandle(out, base, fieldInfo)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	writeSelectorRValueClose(out, sel)
	return true
}

func goPtrExpressionReturnsHandle(expr ast.Expr) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.CallExpr:
		_, ok := goPtrResultInfoForCall(e, 0)
		return ok
	case *ast.IndexExpr:
		_, ok := goPtrArrayFieldInfoForIndexExpr(e)
		return ok
	}
	return false
}

func writeGoPtrExpressionFieldHandle(out *strings.Builder, base ast.Expr, fieldInfo FieldAccessInfo) {
	out.WriteString("{ let __ptr = ")
	TranspileExpression(out, base)
	out.WriteString("; let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()")
	for _, embedded := range fieldInfo.EmbeddedPath {
		out.WriteString(".")
		out.WriteString(ToSnakeCase(embedded))
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
	}
	out.WriteString(".")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone() }")
}

func writeGoPtrCurrentReceiverFieldSelector(out *strings.Builder, fieldInfo FieldAccessInfo, sel *ast.SelectorExpr, ctx ExprContext) bool {
	if !currentReceiverRustAliasIsGoPtr {
		return false
	}
	if fieldInfo.IsPromoted {
		if ctx == LValue || ctx == AddressOf {
			writeGoPtrCurrentReceiverPromotedFieldHandle(out, fieldInfo)
			return true
		}
		if typeInfoIsPointerExpr(sel) || selectorExpressionKeepsHandle(sel) {
			writeGoPtrCurrentReceiverFieldReadHandle(out, fieldInfo)
			return true
		}
		out.WriteString("(*")
		writeGoPtrCurrentReceiverFieldReadHandle(out, fieldInfo)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
		writeSelectorRValueClose(out, sel)
		return true
	}
	if ctx == LValue || ctx == AddressOf {
		writeGoPtrCurrentReceiverFieldHandle(out, fieldInfo)
		return true
	}
	if typeInfoIsPointerExpr(sel) || selectorExpressionKeepsHandle(sel) {
		writeGoPtrCurrentReceiverFieldReadHandle(out, fieldInfo)
		return true
	}
	out.WriteString("(*")
	writeGoPtrCurrentReceiverFieldReadHandle(out, fieldInfo)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	writeSelectorRValueClose(out, sel)
	return true
}

func writeGoPtrCurrentReceiverEmbeddedPromotedFieldSelector(out *strings.Builder, fieldInfo FieldAccessInfo, sel *ast.SelectorExpr, ctx ExprContext) bool {
	embeddedFieldName, remainingPath, ok := goPtrEmbeddedPromotedFieldAccess(sel, fieldInfo)
	if !ok {
		return false
	}
	if ctx == LValue || ctx == AddressOf {
		writeGoPtrCurrentReceiverEmbeddedPromotedFieldHandle(out, fieldInfo, embeddedFieldName, remainingPath)
		return true
	}
	if typeInfoIsPointerExpr(sel) || selectorExpressionKeepsHandle(sel) {
		writeGoPtrCurrentReceiverEmbeddedPromotedFieldHandle(out, fieldInfo, embeddedFieldName, remainingPath)
		return true
	}
	out.WriteString("(*")
	writeGoPtrCurrentReceiverEmbeddedPromotedFieldHandle(out, fieldInfo, embeddedFieldName, remainingPath)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	writeSelectorRValueClose(out, sel)
	return true
}

func writeEmbeddedGoPtrPromotedFieldSelector(out *strings.Builder, baseName string, baseWrapped bool, fieldInfo FieldAccessInfo, sel *ast.SelectorExpr, ctx ExprContext) bool {
	embeddedFieldName, remainingPath, ok := goPtrEmbeddedPromotedFieldAccess(sel, fieldInfo)
	if !ok {
		return false
	}
	if ctx == LValue || ctx == AddressOf {
		writeEmbeddedGoPtrPromotedFieldHandle(out, baseName, baseWrapped, fieldInfo, embeddedFieldName, remainingPath)
		return true
	}
	if typeInfoIsPointerExpr(sel) || selectorExpressionKeepsHandle(sel) {
		writeEmbeddedGoPtrPromotedFieldHandle(out, baseName, baseWrapped, fieldInfo, embeddedFieldName, remainingPath)
		return true
	}
	out.WriteString("(*")
	writeEmbeddedGoPtrPromotedFieldHandle(out, baseName, baseWrapped, fieldInfo, embeddedFieldName, remainingPath)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()")
	writeSelectorRValueClose(out, sel)
	return true
}

func goPtrEmbeddedPromotedFieldAccess(sel *ast.SelectorExpr, fieldInfo FieldAccessInfo) (string, []string, bool) {
	if !fieldInfo.IsPromoted || len(fieldInfo.EmbeddedPath) == 0 {
		return "", nil, false
	}
	typeInfo := GetTypeInfo()
	if sel == nil || typeInfo == nil || typeInfo.info == nil {
		return "", nil, false
	}
	selection := typeInfo.info.Selections[sel]
	if selection == nil || selection.Kind() != types.FieldVal {
		return "", nil, false
	}
	owner := sliceElemPtrDerefPointerType(selection.Recv())
	key := sliceElemPtrFieldKeyForOwnerType(owner, fieldInfo.EmbeddedPath[0])
	if key == "" || !generatedGoPtrFieldForKey(key) {
		return "", nil, false
	}
	if _, ok := sliceElemPtrFieldInfoForKey(key); !ok {
		return "", nil, false
	}
	return ToSnakeCase(fieldInfo.EmbeddedPath[0]), fieldInfo.EmbeddedPath[1:], true
}

func goPtrEmbeddedPromotedMethodAccess(sel *ast.SelectorExpr, fields []string) (string, []string, bool) {
	if len(fields) == 0 {
		return "", nil, false
	}
	typeInfo := GetTypeInfo()
	if sel == nil || typeInfo == nil || typeInfo.info == nil {
		return "", nil, false
	}
	selection := typeInfo.info.Selections[sel]
	if selection == nil || selection.Kind() != types.MethodVal {
		return "", nil, false
	}
	indexes := selection.Index()
	if len(indexes) < 2 {
		return "", nil, false
	}
	owner := sliceElemPtrDerefPointerType(selection.Recv())
	structType, ok := sliceElemPtrStructUnderlying(owner)
	if !ok || indexes[0] < 0 || indexes[0] >= structType.NumFields() {
		return "", nil, false
	}
	field := structType.Field(indexes[0])
	key := sliceElemPtrFieldKeyForOwnerType(owner, field.Name())
	if key == "" || !generatedGoPtrFieldForKey(key) {
		return "", nil, false
	}
	if _, ok := sliceElemPtrFieldInfoForKey(key); !ok {
		return "", nil, false
	}
	return ToSnakeCase(field.Name()), fields[1:], true
}

func writeGoPtrCurrentReceiverEmbeddedPromotedFieldHandle(out *strings.Builder, fieldInfo FieldAccessInfo, embeddedFieldName string, remainingPath []string) {
	out.WriteString("{ let __ptr_value = ")
	out.WriteString(currentReceiverRustName())
	out.WriteString(".")
	out.WriteString(embeddedFieldName)
	out.WriteString(".with_mut(|__ptr_value| { let __field = __ptr_value")
	for _, embedded := range remainingPath {
		out.WriteString(".")
		out.WriteString(ToSnakeCase(embedded))
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
	}
	out.WriteString(".")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone(); __field }); __ptr_value }")
}

func writeEmbeddedGoPtrPromotedFieldHandle(out *strings.Builder, baseName string, baseWrapped bool, fieldInfo FieldAccessInfo, embeddedFieldName string, remainingPath []string) {
	out.WriteString("{ let __embedded = ")
	if baseWrapped {
		out.WriteString("(*")
		out.WriteString(baseName)
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()).")
	} else {
		out.WriteString(baseName)
		out.WriteString(".")
	}
	out.WriteString(embeddedFieldName)
	out.WriteString(".clone(); let __field = __embedded.with_mut(|__ptr_value| { let __field = __ptr_value")
	for _, embedded := range remainingPath {
		out.WriteString(".")
		out.WriteString(ToSnakeCase(embedded))
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
	}
	out.WriteString(".")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone(); __field }); __field }")
}

func writeGoPtrLocalFieldHandle(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo) {
	out.WriteString("{ let __ptr_value = ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".with_mut(|__ptr_value| __ptr_value.")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone()); __ptr_value }")
}

func writeGoPtrLocalFieldReadHandle(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo) {
	out.WriteString("{ let __ptr_value = ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".borrow(); let __field_value = __ptr_value.as_ref().unwrap()")
	writeGoPtrFieldReadPath(out, fieldInfo)
	out.WriteString("; __field_value }")
}

func writeGoPtrSelectorReadHandle(out *strings.Builder, sel *ast.SelectorExpr) bool {
	if sel == nil {
		return false
	}
	ident, ok := unwrapParens(sel.X).(*ast.Ident)
	if !ok {
		return false
	}
	fieldInfo := selectorFieldAccessInfo(sel)
	if isCurrentReceiverIdent(ident) && currentReceiverRustAliasIsGoPtr {
		writeGoPtrCurrentReceiverFieldReadHandle(out, fieldInfo)
		return true
	}
	if isGoPtrVar(ident.Name) {
		writeGoPtrLocalFieldReadHandle(out, ident, fieldInfo)
		return true
	}
	return false
}

func writeGeneratedGoPtrFieldHandleClone(out *strings.Builder, sel *ast.SelectorExpr) bool {
	if sel == nil || !generatedGoPtrFieldForSelector(sel) {
		return false
	}
	if writeGoPtrSelectorReadHandle(out, sel) {
		return true
	}
	TranspileExpressionContext(out, sel, LValue)
	out.WriteString(".clone()")
	return true
}

func writeGoPtrCurrentReceiverFieldHandle(out *strings.Builder, fieldInfo FieldAccessInfo) {
	out.WriteString("{ let __ptr_value = ")
	out.WriteString(currentReceiverRustName())
	out.WriteString(".with_mut(|__ptr_value| __ptr_value.")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone()); __ptr_value }")
}

func writeGoPtrCurrentReceiverFieldReadHandle(out *strings.Builder, fieldInfo FieldAccessInfo) {
	out.WriteString("{ let __ptr_value = ")
	out.WriteString(currentReceiverRustName())
	out.WriteString(".borrow(); let __field_value = __ptr_value.as_ref().unwrap()")
	writeGoPtrFieldReadPath(out, fieldInfo)
	out.WriteString("; __field_value }")
}

func writeGoPtrFieldReadPath(out *strings.Builder, fieldInfo FieldAccessInfo) {
	for _, embedded := range fieldInfo.EmbeddedPath {
		out.WriteString(".")
		out.WriteString(ToSnakeCase(embedded))
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
	}
	out.WriteString(".")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone()")
}

func writeGoPtrLocalPromotedFieldHandle(out *strings.Builder, ident *ast.Ident, fieldInfo FieldAccessInfo) {
	out.WriteString("{ let __ptr_value = ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".with_mut(|__ptr_value| { let __field = __ptr_value")
	for _, embedded := range fieldInfo.EmbeddedPath {
		out.WriteString(".")
		out.WriteString(ToSnakeCase(embedded))
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
	}
	out.WriteString(".")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone(); __field }); __ptr_value }")
}

func writeGoPtrCurrentReceiverPromotedFieldHandle(out *strings.Builder, fieldInfo FieldAccessInfo) {
	out.WriteString("{ let __ptr_value = ")
	out.WriteString(currentReceiverRustName())
	out.WriteString(".with_mut(|__ptr_value| { let __field = __ptr_value")
	for _, embedded := range fieldInfo.EmbeddedPath {
		out.WriteString(".")
		out.WriteString(ToSnakeCase(embedded))
		WriteBorrowMethod(out, false)
		out.WriteString(".as_ref().unwrap()")
	}
	out.WriteString(".")
	out.WriteString(fieldInfo.FieldName)
	out.WriteString(".clone(); __field }); __ptr_value }")
}

func writeGoPtrCallArgument(out *strings.Builder, arg ast.Expr, elemRustType string) bool {
	return writeGoPtrCallArgumentWithQualifier(out, arg, elemRustType, "")
}

func writeGoPtrCallArgumentForInfo(out *strings.Builder, arg ast.Expr, info goPtrResultInfo) bool {
	return writeGoPtrCallArgumentWithQualifierForInfo(out, arg, info, "")
}

func writeGoPtrCallArgumentWithQualifier(out *strings.Builder, arg ast.Expr, elemRustType string, helperQualifier string) bool {
	return writeGoPtrCallArgumentWithQualifierForInfo(out, arg, goPtrResultInfo{elemRustType: elemRustType}, helperQualifier)
}

func writeGoPtrCallArgumentWithQualifierForInfo(out *strings.Builder, arg ast.Expr, info goPtrResultInfo, helperQualifier string) bool {
	elemRustType := goPtrResultElemRustType(info)
	if elemRustType == "" {
		return false
	}
	NeedSliceElemPtr()
	if ident, ok := unwrapParens(arg).(*ast.Ident); ok {
		if ident.Name == "nil" {
			writeGoPtrQualifiedConstructor(out, helperQualifier, "nil")
			out.WriteString("()")
			return true
		}
		if helperQualifier != "" && isGoPtrVar(ident.Name) {
			writeQualifiedGoPtrVarConversion(out, ident, helperQualifier)
			return true
		}
		if isGoPtrVar(ident.Name) {
			out.WriteString(rustIdentForUseWithCapture(ident))
			out.WriteString(".clone()")
			return true
		}
		if info, ok := sliceElemPtrVarInfo(ident.Name); ok && elemRustTypeFromSliceElemPtrRustType(info.RustType) == elemRustType {
			writeGoPtrQualifiedConstructor(out, helperQualifier, "slice_elem_opt")
			out.WriteString("(")
			out.WriteString(rustIdentForUseWithCapture(ident))
			out.WriteString(".clone())")
			return true
		}
		if info, ok := arrayElemPtrVarInfo(ident.Name); ok && elemRustTypeFromArrayElemPtrRustType(info.RustType) == elemRustType {
			if helperQualifier != "" {
				writeGoPtrArrayElemOptionConversion(out, helperQualifier, func() {
					out.WriteString(rustIdentForUseWithCapture(ident))
					out.WriteString(".clone()")
				})
				return true
			}
			writeGoPtrQualifiedConstructor(out, helperQualifier, "array_elem_opt")
			out.WriteString("(")
			out.WriteString(rustIdentForUseWithCapture(ident))
			out.WriteString(".clone())")
			return true
		}
		if isCurrentReceiverIdent(ident) && goPtrCallArgumentIsLocalPointerForInfo(arg, info) {
			writeGoPtrQualifiedConstructor(out, helperQualifier, "local")
			out.WriteString("(")
			WriteWrapperOptionPrefix(out)
			out.WriteString("Some(")
			writeCurrentReceiverClone(out, ident)
			out.WriteString(")")
			WriteWrapperOptionSuffix(out)
			out.WriteString(")")
			return true
		}
	}
	if elem, ok := sliceElemPtrAddressElemRustType(arg); ok && (elem == elemRustType || goPtrPointerArgumentElemCompatible(arg, info)) {
		writeGoPtrQualifiedConstructor(out, helperQualifier, "slice_elem")
		out.WriteString("(")
		if !writeQualifiedSliceElemPtrAddress(out, arg, helperQualifier) {
			TranspileExpression(out, arg)
		}
		out.WriteString(")")
		return true
	}
	if elem, ok := arrayElemPtrAddressElemRustType(arg); ok && (elem == elemRustType || goPtrPointerArgumentElemCompatible(arg, info)) {
		writeGoPtrQualifiedConstructor(out, helperQualifier, "array_elem")
		out.WriteString("(")
		if !writeQualifiedArrayElemPtrAddress(out, arg, helperQualifier) {
			TranspileExpression(out, arg)
		}
		out.WriteString(")")
		return true
	}
	if call, ok := unwrapParens(arg).(*ast.CallExpr); ok {
		if writeUnsafeStringDataGoPtrCallArgument(out, call, info, helperQualifier) {
			return true
		}
		if info, ok := sliceElemPtrReturnInfoForCall(call); ok && info.elemRustType == elemRustType {
			writeGoPtrQualifiedConstructor(out, helperQualifier, "slice_elem_opt")
			out.WriteString("(")
			TranspileExpression(out, arg)
			out.WriteString(")")
			return true
		}
		if info, ok := arrayElemPtrResultInfoForCall(call, 0); ok && info.elemRustType == elemRustType {
			if helperQualifier != "" {
				writeGoPtrArrayElemOptionConversion(out, helperQualifier, func() {
					TranspileExpression(out, arg)
				})
				return true
			}
			writeGoPtrQualifiedConstructor(out, helperQualifier, "array_elem_opt")
			out.WriteString("(")
			TranspileExpression(out, arg)
			out.WriteString(")")
			return true
		}
		if writeInternalABITypeOfMapTypeGoPtrValue(out, call, info, helperQualifier) {
			return true
		}
		if resultInfo, ok := goPtrResultInfoForCall(call, 0); ok && goPtrResultElemCompatible(resultInfo, info) {
			callHelperQualifier := goPtrHelperQualifierForCall(call)
			if callHelperQualifier == helperQualifier {
				TranspileExpression(out, arg)
			} else {
				writeGoPtrConversion(out, callHelperQualifier, helperQualifier, func() {
					TranspileExpression(out, arg)
				})
			}
			return true
		}
		if writeGoPtrRawPointerValue(out, call, elemRustType, helperQualifier) {
			return true
		}
	}
	if sel, ok := unwrapParens(arg).(*ast.SelectorExpr); ok {
		if fieldInfo, ok := sliceElemPtrFieldInfoForSelector(sel); ok && generatedGoPtrFieldForSelector(sel) && goPtrResultElemCompatible(goPtrResultInfo{elemRustType: fieldInfo.elemRustType, elemType: fieldInfo.elemType}, info) {
			fieldHelperQualifier := goPtrHelperQualifierForOwnerPackage(fieldInfo.ownerPkgPath)
			if fieldHelperQualifier == helperQualifier {
				if !writeGoPtrSelectorReadHandle(out, sel) {
					TranspileExpressionContext(out, sel, LValue)
					out.WriteString(".clone()")
				}
			} else {
				writeQualifiedGoPtrSelectorConversion(out, sel, fieldHelperQualifier, helperQualifier)
			}
			return true
		}
	}
	if indexExpr, ok := unwrapParens(arg).(*ast.IndexExpr); ok {
		if fieldInfo, ok := goPtrArrayFieldInfoForIndexExpr(indexExpr); ok && goPtrResultElemCompatible(goPtrResultInfo{elemRustType: fieldInfo.elemRustType, elemType: fieldInfo.elemType}, info) {
			if goPtrHelperQualifierForOwnerPackage(fieldInfo.ownerPkgPath) != helperQualifier {
				return false
			}
			writeGoPtrArrayFieldIndexRead(out, indexExpr)
			return true
		}
	}
	if slotInfo, ok := goPtrSlotDerefResultInfo(arg); ok && goPtrResultElemCompatible(slotInfo, info) {
		if helperQualifier != "" {
			return false
		}
		star := unwrapParens(arg).(*ast.StarExpr)
		writeGoPtrSlotDerefRead(out, star)
		return true
	}
	if goPtrCallArgumentIsLocalPointerForInfo(arg, info) {
		writeGoPtrQualifiedConstructor(out, helperQualifier, "local")
		out.WriteString("(")
		writePointerHandleExpression(out, arg)
		out.WriteString(")")
		return true
	}
	return false
}

func writeUnsafeStringDataGoPtrCallArgument(out *strings.Builder, call *ast.CallExpr, info goPtrResultInfo, helperQualifier string) bool {
	elemRustType := goPtrResultElemRustType(info)
	if elemRustType != "u8" {
		return false
	}
	if info.elemType != nil && !isByteType(info.elemType) {
		return false
	}
	typeInfo := GetTypeInfo()
	stringExpr, ok := unsafeStringDataCallArg(call, typeInfo)
	if !ok || !typeInfo.IsString(stringExpr) {
		return false
	}
	callType := typeInfo.GetType(call)
	if callType == nil {
		return false
	}
	ptr, ok := types.Unalias(callType).Underlying().(*types.Pointer)
	if !ok || !isByteType(ptr.Elem()) {
		return false
	}

	out.WriteString("{ let __string_data_holder = ")
	TranspileExpressionContext(out, stringExpr, LValue)
	out.WriteString(".clone(); ")
	writeGoPtrQualifiedConstructor(out, helperQualifier, "array_elem_foreign")
	out.WriteString("(")
	writeUnsafeStringDataForeignClosurePrefix(out)
	out.WriteString("({ let __string_data_holder = __string_data_holder.clone(); move || { let __string_guard = __string_data_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; __string_guard.as_ref().and_then(|__s| __s.as_bytes().get(0).copied()) } }), ")
	writeUnsafeStringDataForeignClosurePrefix(out)
	out.WriteString(`(move |__assigned| { let _ = __assigned; panic!("unsafe.StringData pointer assignment requires writable pointee support") }), `)
	writeUnsafeStringDataForeignClosurePrefix(out)
	out.WriteString(`(move |__callback| { let _ = __callback; panic!("unsafe.StringData pointer mutable borrow requires writable pointee support") }), `)
	writeUnsafeStringDataForeignClosurePrefix(out)
	out.WriteString("({ let __string_data_holder = __string_data_holder.clone(); move || { let __string_guard = __string_data_holder")
	WriteBorrowMethod(out, false)
	out.WriteString("; match __string_guard.as_ref() { Some(__s) => (__s.as_ptr() as *const (), 0usize), None => (std::ptr::null(), 0usize) } } })")
	out.WriteString(") }")
	return true
}

func unsafeStringDataCallArg(expr ast.Expr, typeInfo *TypeInfo) (ast.Expr, bool) {
	if typeInfo == nil || typeInfo.info == nil {
		return nil, false
	}
	call, ok := unwrapParens(expr).(*ast.CallExpr)
	if !ok || len(call.Args) != 1 {
		return nil, false
	}
	sel, ok := unwrapParens(call.Fun).(*ast.SelectorExpr)
	if !ok || sel.Sel == nil {
		return nil, false
	}
	pkgIdent, ok := unwrapParens(sel.X).(*ast.Ident)
	if !ok {
		return nil, false
	}
	pkgName, ok := typeInfo.info.Uses[pkgIdent].(*types.PkgName)
	if !ok || pkgName.Imported() == nil || pkgName.Imported().Path() != "unsafe" {
		return nil, false
	}
	switch obj := typeInfo.info.Uses[sel.Sel].(type) {
	case *types.Builtin:
		if obj.Name() != "StringData" {
			return nil, false
		}
	case *types.Func:
		if obj.Pkg() == nil || obj.Pkg().Path() != "unsafe" || obj.Name() != "StringData" {
			return nil, false
		}
	default:
		return nil, false
	}
	return call.Args[0], true
}

func writeUnsafeStringDataForeignClosurePrefix(out *strings.Builder) {
	if NeedsConcurrentWrapper() {
		out.WriteString("std::sync::Arc::new")
		return
	}
	out.WriteString("std::rc::Rc::new")
}

func writeGoPtrArrayFieldIndexRead(out *strings.Builder, indexExpr *ast.IndexExpr) {
	if goPtrArrayFieldIndexShouldUseMultiline(indexExpr) {
		writeGoPtrArrayFieldIndexReadMultiline(out, indexExpr)
		return
	}
	TranspileExpressionContext(out, indexExpr.X, LValue)
	WriteBorrowMethod(out, false)
	out.WriteString(".as_ref().unwrap()[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("].clone()")
}

func goPtrArrayFieldIndexShouldUseMultiline(indexExpr *ast.IndexExpr) bool {
	if indexExpr == nil || !NeedsConcurrentWrapper() {
		return false
	}
	return compositeLiteralElementIsComplex(indexExpr.X) || compositeLiteralElementIsComplex(indexExpr.Index)
}

func writeGoPtrArrayFieldIndexReadMultiline(out *strings.Builder, indexExpr *ast.IndexExpr) {
	indent := currentLineIndent(out)
	out.WriteString("{\n")
	out.WriteString(indent)
	out.WriteString("    let __seq_holder = ")
	TranspileExpressionContext(out, indexExpr.X, LValue)
	out.WriteString(".clone();\n")
	out.WriteString(indent)
	out.WriteString("    let __seq_guard = __seq_holder")
	WriteBorrowMethod(out, false)
	out.WriteString(";\n")
	out.WriteString(indent)
	out.WriteString("    __seq_guard.as_ref().unwrap()[")
	writeExpressionAsUsize(out, indexExpr.Index)
	out.WriteString("].clone()\n")
	out.WriteString(indent)
	out.WriteString("}")
}

func goPtrCallArgumentUsesQualifiedSelectorConversion(arg ast.Expr, info goPtrResultInfo, helperQualifier string) bool {
	sel, ok := unwrapParens(arg).(*ast.SelectorExpr)
	if !ok {
		return false
	}
	fieldInfo, ok := sliceElemPtrFieldInfoForSelector(sel)
	if !ok || !generatedGoPtrFieldForSelector(sel) {
		return false
	}
	if !goPtrResultElemCompatible(goPtrResultInfo{elemRustType: fieldInfo.elemRustType, elemType: fieldInfo.elemType}, info) {
		return false
	}
	return goPtrHelperQualifierForOwnerPackage(fieldInfo.ownerPkgPath) != helperQualifier
}

func goPtrResultElemCompatible(actual goPtrResultInfo, expected goPtrResultInfo) bool {
	if actual.elemType != nil && expected.elemType != nil && types.Identical(coreType(actual.elemType), coreType(expected.elemType)) {
		return true
	}
	if actual.elemRustType != "" && expected.elemRustType != "" && actual.elemRustType == expected.elemRustType {
		return true
	}
	return goPtrResultElemRustType(actual) != "" && goPtrResultElemRustType(actual) == goPtrResultElemRustType(expected)
}

func goPtrHelperQualifierForCall(call *ast.CallExpr) string {
	typeInfo := GetTypeInfo()
	fn, ok := callFunctionObjectFromTypeInfo(typeInfo, call)
	if !ok {
		return ""
	}
	return goPtrHelperQualifierForFunc(fn)
}

func writeGoPtrRawPointerValue(out *strings.Builder, arg ast.Expr, elemRustType string, helperQualifier string) bool {
	typeInfo := GetTypeInfo()
	source, ok := goPtrRawPointerConversionSource(arg, typeInfo)
	if !ok {
		return false
	}
	if call, ok := unwrapParens(arg).(*ast.CallExpr); ok {
		if target, ok := typedPointerTypeConversionTarget(call); ok {
			if writeInternalABIEmptyInterfaceGoPtrValue(out, target, source, helperQualifier) {
				return true
			}
			if writeEmbeddedOwnerGoPtrValue(out, target, source, helperQualifier) {
				return true
			}
		}
	}
	info, ok := goPtrInfoForPointerType(typeInfo.GetType(arg))
	if !ok || info.elemRustType != elemRustType {
		return false
	}
	if isFunctionSignatureType(info.elemType) {
		return writeGoPtrUnsupportedFunctionPointerValue(out, source, info.elemRustType, helperQualifier)
	}
	writeGoPtrQualifiedConstructor(out, helperQualifier, "raw")
	out.WriteString("(")
	writeUnsafePointerRawAddress(out, source)
	out.WriteString(")")
	return true
}

func writeGoPtrUnsupportedFunctionPointerValue(out *strings.Builder, source ast.Expr, elemRustType string, helperQualifier string) bool {
	writeGoPtrQualifiedConstructor(out, helperQualifier, "local")
	out.WriteString("(")
	WriteWrapperOptionPrefix(out)
	out.WriteString("{ let __ptr = ")
	writeUnsafePointerConversionSource(out, source)
	out.WriteString("; let __ptr_guard = __ptr")
	WriteBorrowMethod(out, false)
	out.WriteString("; if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<")
	out.WriteString(elemRustType)
	out.WriteString(">(")
	writeUnsafePointerConversionUnsupported(out, elemRustType)
	out.WriteString(") } }")
	WriteWrapperOptionSuffix(out)
	out.WriteString(")")
	return true
}

func writeEmbeddedOwnerGoPtrValue(out *strings.Builder, target ast.Expr, source ast.Expr, helperQualifier string) bool {
	targetType, _, ok := embeddedOwnerConversionTypes(target, source)
	if !ok {
		return false
	}
	targetRust := pointerConversionTargetTypeToRust(target)
	NeedEmbeddedOwnerRegistry()
	trackWrapperImports()
	out.WriteString("{ let __ptr = ")
	writeUnsafePointerConversionSource(out, source)
	out.WriteString("; let __ptr_guard = __ptr")
	WriteBorrowMethod(out, false)
	out.WriteString("; if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { ")
	writeGoPtrQualifiedConstructor(out, helperQualifier, "nil")
	out.WriteString("() } else { ")
	writeGoPtrQualifiedConstructor(out, helperQualifier, "local")
	out.WriteString("(go_lookup_embedded_owner::<")
	out.WriteString(targetRust)
	out.WriteString(">(*__ptr_guard.as_ref().unwrap(), \"")
	out.WriteString(targetRust)
	out.WriteString("\")) } }")
	_ = targetType
	return true
}

func writeInternalABIEmptyInterfaceGoPtrValue(out *strings.Builder, target ast.Expr, source ast.Expr, helperQualifier string) bool {
	if !targetIsInternalABIEmptyInterface(target) {
		return false
	}
	sourceArg, ok := unsafePointerAddressSource(source)
	if !ok {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || !isEmptyInterfaceType(typeInfo.GetType(sourceArg)) {
		return false
	}
	writeGoPtrQualifiedConstructor(out, helperQualifier, "local")
	out.WriteString("(")
	writeInternalABIEmptyInterfaceValueWrapper(out, target, sourceArg)
	out.WriteString(")")
	return true
}

func writeGoPtrQualifiedConstructor(out *strings.Builder, helperQualifier string, constructor string) {
	if helperQualifier != "" {
		out.WriteString(helperQualifier)
		out.WriteString("::")
	}
	out.WriteString("GoPtr::")
	out.WriteString(constructor)
}

func writeQualifiedSliceElemPtrAddress(out *strings.Builder, arg ast.Expr, helperQualifier string) bool {
	unary, ok := unwrapParens(arg).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return false
	}
	return writeSliceElemPtrNewExpressionWithQualifier(out, indexExpr, helperQualifier)
}

func writeQualifiedArrayElemPtrAddress(out *strings.Builder, arg ast.Expr, helperQualifier string) bool {
	unary, ok := unwrapParens(arg).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return false
	}
	return writeArrayElemPtrNewExpressionWithQualifier(out, indexExpr, helperQualifier)
}

func writeQualifiedGoPtrVarConversion(out *strings.Builder, ident *ast.Ident, helperQualifier string) {
	writeGoPtrConversion(out, "", helperQualifier, func() {
		out.WriteString(rustIdentForUseWithCapture(ident))
	})
}

func writeQualifiedGoPtrSelectorConversion(out *strings.Builder, sel *ast.SelectorExpr, inputHelperQualifier string, outputHelperQualifier string) {
	writeGoPtrConversion(out, inputHelperQualifier, outputHelperQualifier, func() {
		TranspileExpressionContext(out, sel, LValue)
	})
}

func writeGoPtrConversion(out *strings.Builder, inputHelperQualifier string, outputHelperQualifier string, writeValue func()) {
	indent := currentLineIndent(out)
	innerIndent := indent + "    "
	armIndent := innerIndent + "    "
	out.WriteString("{\n")
	out.WriteString(innerIndent)
	out.WriteString("let __go_ptr = ")
	writeValue()
	out.WriteString(".clone();\n")
	out.WriteString(innerIndent)
	out.WriteString("match __go_ptr {\n")
	out.WriteString(armIndent)
	writeGoPtrQualifiedVariant(out, inputHelperQualifier, "Nil")
	out.WriteString(" => ")
	writeGoPtrQualifiedConstructor(out, outputHelperQualifier, "nil")
	out.WriteString("(),\n")
	out.WriteString(armIndent)
	writeGoPtrQualifiedVariant(out, inputHelperQualifier, "Local")
	out.WriteString("(__value) => ")
	writeGoPtrQualifiedConstructor(out, outputHelperQualifier, "local")
	out.WriteString("(__value.clone()),\n")
	out.WriteString(armIndent)
	writeGoPtrQualifiedVariant(out, inputHelperQualifier, "Raw")
	out.WriteString("(__addr) => ")
	writeGoPtrQualifiedConstructor(out, outputHelperQualifier, "raw")
	out.WriteString("(__addr),\n")
	out.WriteString(armIndent)
	writeGoPtrQualifiedVariant(out, inputHelperQualifier, "SliceElem")
	out.WriteString("(__value) => ")
	writeGoPtrQualifiedConstructor(out, outputHelperQualifier, "slice_elem")
	out.WriteString("(")
	if outputHelperQualifier != "" {
		out.WriteString(outputHelperQualifier)
		out.WriteString("::")
	}
	out.WriteString("GoSliceElemPtr::new(__value.slice_handle(), __value.index())),\n")
	out.WriteString(armIndent)
	writeGoPtrQualifiedVariant(out, inputHelperQualifier, "ArrayElem")
	out.WriteString("(__value) => ")
	writeGoPtrForeignArrayElemConversion(out, outputHelperQualifier, "__value")
	out.WriteString(",\n")
	out.WriteString(innerIndent)
	out.WriteString("}\n")
	out.WriteString(indent)
	out.WriteString("}")
}

func writeGoPtrArrayElemOptionConversion(out *strings.Builder, helperQualifier string, writeValue func()) {
	out.WriteString("{ match ")
	writeValue()
	out.WriteString(" { Some(__value) => ")
	writeGoPtrForeignArrayElemConversion(out, helperQualifier, "__value")
	out.WriteString(", None => ")
	writeGoPtrQualifiedConstructor(out, helperQualifier, "nil")
	out.WriteString("() } }")
}

func writeGoPtrForeignArrayElemConversion(out *strings.Builder, outputHelperQualifier string, valueExpr string) {
	writeGoPtrQualifiedConstructor(out, outputHelperQualifier, "array_elem_foreign")
	out.WriteString("(")
	writeGoPtrForeignArrayElemClosure(out, valueExpr, "borrow_dyn", "||", "")
	out.WriteString(", ")
	writeGoPtrForeignArrayElemClosure(out, valueExpr, "assign_dyn", "|__assigned|", "__assigned")
	out.WriteString(", ")
	writeGoPtrForeignArrayElemClosure(out, valueExpr, "with_mut_dyn", "|__callback|", "__callback")
	out.WriteString(", ")
	writeGoPtrForeignArrayElemClosure(out, valueExpr, "identity_dyn", "||", "")
	out.WriteString(")")
}

func writeGoPtrForeignArrayElemClosure(out *strings.Builder, valueExpr string, method string, params string, arg string) {
	if NeedsConcurrentWrapper() {
		out.WriteString("std::sync::Arc::new")
	} else {
		out.WriteString("std::rc::Rc::new")
	}
	out.WriteString("({ let __value = ")
	out.WriteString(valueExpr)
	out.WriteString(".clone(); move ")
	out.WriteString(params)
	out.WriteString(" __value.")
	out.WriteString(method)
	if arg == "" {
		out.WriteString("()")
	} else {
		out.WriteString("(")
		out.WriteString(arg)
		out.WriteString(")")
	}
	out.WriteString(" })")
}

func writeGoPtrQualifiedVariant(out *strings.Builder, helperQualifier string, variant string) {
	if helperQualifier != "" {
		out.WriteString(helperQualifier)
		out.WriteString("::")
	}
	out.WriteString("GoPtr::")
	out.WriteString(variant)
}

func goPtrCallArgumentIsLocalPointer(arg ast.Expr, elemRustType string) bool {
	return goPtrCallArgumentIsLocalPointerForInfo(arg, goPtrResultInfo{elemRustType: elemRustType})
}

func goPtrCallArgumentIsLocalPointerForInfo(arg ast.Expr, info goPtrResultInfo) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	actual := typeInfo.GetType(arg)
	if actual == nil {
		return false
	}
	ptr, ok := types.Unalias(actual).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	if info.elemType != nil && types.Identical(types.Unalias(ptr.Elem()), types.Unalias(info.elemType)) {
		return true
	}
	if _, ok := types.Unalias(ptr.Elem()).Underlying().(*types.Pointer); ok {
		if inner, ok := goPtrInfoForPointerType(ptr.Elem()); ok {
			return "GoPtr<"+goPtrResultElemRustType(inner)+">" == goPtrResultElemRustType(info)
		}
	}
	return goTypesTypeToRust(ptr.Elem()) == info.elemRustType
}

func goPtrPointerArgumentElemCompatible(arg ast.Expr, info goPtrResultInfo) bool {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || info.elemType == nil {
		return false
	}
	actual := typeInfo.GetType(arg)
	if actual == nil {
		return false
	}
	ptr, ok := types.Unalias(actual).Underlying().(*types.Pointer)
	if !ok {
		return false
	}
	return types.Identical(types.Unalias(ptr.Elem()), types.Unalias(info.elemType))
}

func writeGoPtrDerefRead(out *strings.Builder, ident *ast.Ident) {
	out.WriteString("{ let __ptr_value = ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".borrow(); __ptr_value.as_ref().unwrap().clone() }")
}

func writeGoPtrCallDerefRead(out *strings.Builder, expr ast.Expr) bool {
	call, ok := unwrapParens(expr).(*ast.CallExpr)
	if !ok {
		return false
	}
	if _, ok := goPtrResultInfoForCall(call, 0); !ok {
		return false
	}
	out.WriteString("{ let __ptr_handle = ")
	TranspileExpression(out, call)
	out.WriteString("; let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() }")
	return true
}

func writeGoPtrFieldDerefRead(out *strings.Builder, expr ast.Expr) bool {
	sel, ok := unwrapParens(expr).(*ast.SelectorExpr)
	if !ok || !generatedGoPtrFieldForSelector(sel) {
		return false
	}
	out.WriteString("{ let __ptr_value = ")
	TranspileExpressionContext(out, sel, LValue)
	out.WriteString(".borrow(); __ptr_value.as_ref().unwrap().clone() }")
	return true
}

func goPtrSlotDerefResultInfo(expr ast.Expr) (goPtrResultInfo, bool) {
	star, ok := unwrapParens(expr).(*ast.StarExpr)
	if !ok {
		return goPtrResultInfo{}, false
	}
	if _, info, ok := goPtrSlotDerefInfo(star); ok {
		return info, true
	}
	_, info, ok := goPtrPointerSlotDerefInfo(star)
	return info, ok
}

func goPtrSlotDerefInfo(star *ast.StarExpr) (*ast.Ident, goPtrResultInfo, bool) {
	if star == nil {
		return nil, goPtrResultInfo{}, false
	}
	ident, ok := unwrapParens(star.X).(*ast.Ident)
	if !ok {
		return nil, goPtrResultInfo{}, false
	}
	if _, ok := goPtrSlotVarInfo(ident.Name); !ok {
		return nil, goPtrResultInfo{}, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, goPtrResultInfo{}, false
	}
	slotInfo, ok := goPtrSlotInfoForPointerToPointerType(typeInfo.GetType(star.X))
	if !ok {
		return nil, goPtrResultInfo{}, false
	}
	if expectedInfo, ok := goPtrInfoForPointerType(typeInfo.GetType(star)); ok && !goPtrResultElemCompatible(expectedInfo, slotInfo) {
		return nil, goPtrResultInfo{}, false
	}
	return ident, slotInfo, true
}

func goPtrPointerSlotDerefInfo(star *ast.StarExpr) (*ast.Ident, goPtrResultInfo, bool) {
	if star == nil {
		return nil, goPtrResultInfo{}, false
	}
	ident, ok := unwrapParens(star.X).(*ast.Ident)
	if !ok || !isGoPtrVar(ident.Name) {
		return nil, goPtrResultInfo{}, false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return nil, goPtrResultInfo{}, false
	}
	slotInfo, ok := goPtrSlotInfoForPointerToPointerType(typeInfo.GetType(star.X))
	if !ok {
		return nil, goPtrResultInfo{}, false
	}
	if expectedInfo, ok := goPtrInfoForPointerType(typeInfo.GetType(star)); ok && !goPtrResultElemCompatible(expectedInfo, slotInfo) {
		return nil, goPtrResultInfo{}, false
	}
	return ident, slotInfo, true
}

func writeGoPtrSlotDerefRead(out *strings.Builder, star *ast.StarExpr) bool {
	ident, _, ok := goPtrSlotDerefInfo(star)
	if ok {
		out.WriteString("{ let __ptr_slot = ")
		out.WriteString(rustIdentForUseWithCapture(ident))
		WriteBorrowMethod(out, false)
		out.WriteString("; __ptr_slot.as_ref().unwrap().clone() }")
		return true
	}
	ident, _, ok = goPtrPointerSlotDerefInfo(star)
	if !ok {
		return false
	}
	NeedSliceElemPtr()
	out.WriteString("{ let __ptr_slot = ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".borrow(); __ptr_slot.as_ref().unwrap().clone() }")
	return true
}

func writeGoPtrDerefAssignment(out *strings.Builder, ident *ast.Ident, target *ast.StarExpr, rhs ast.Expr) {
	out.WriteString("{ let new_val = ")
	var expected types.Type
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		expected = typeInfo.GetType(target)
	}
	if !writePointerDerefAssignmentValue(out, rhs, expected) {
		TranspileExpression(out, rhs)
	}
	out.WriteString("; ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".assign(Some(new_val)); }")
}

func writeGoPtrFieldDerefAssignment(out *strings.Builder, target *ast.StarExpr, rhs ast.Expr) bool {
	sel, ok := unwrapParens(target.X).(*ast.SelectorExpr)
	if !ok || !generatedGoPtrFieldForSelector(sel) {
		return false
	}
	var targetHandle strings.Builder
	if !writeGeneratedGoPtrFieldHandleClone(&targetHandle, sel) {
		return false
	}
	out.WriteString("{ let new_val = ")
	var expected types.Type
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		expected = typeInfo.GetType(target)
	}
	if !writePointerDerefAssignmentValue(out, rhs, expected) {
		TranspileExpression(out, rhs)
	}
	out.WriteString("; let __ptr_target = ")
	out.WriteString(targetHandle.String())
	out.WriteString("; __ptr_target.assign(Some(new_val)); }")
	return true
}

func writeGoPtrSlotDerefAssignment(out *strings.Builder, target *ast.StarExpr, rhs ast.Expr) bool {
	ident, slotInfo, ok := goPtrSlotDerefInfo(target)
	if ok {
		out.WriteString("{ let new_val = ")
		if !writeGoPtrCallArgumentForInfo(out, rhs, slotInfo) {
			out.WriteString(`unimplemented!("GoPtr slot dereference assignment requires compatible pointer value")`)
		}
		out.WriteString("; *")
		out.WriteString(rustIdentForUseWithCapture(ident))
		WriteBorrowMethod(out, true)
		out.WriteString(" = Some(new_val); }")
		return true
	}

	ident, slotInfo, ok = goPtrPointerSlotDerefInfo(target)
	if !ok {
		return false
	}
	out.WriteString("{ let new_val = ")
	if !writeGoPtrCallArgumentForInfo(out, rhs, slotInfo) {
		out.WriteString(`unimplemented!("GoPtr slot dereference assignment requires compatible pointer value")`)
	}
	out.WriteString("; ")
	out.WriteString(rustIdentForUseWithCapture(ident))
	out.WriteString(".assign(Some(new_val)); }")
	return true
}

func writeGoPtrDerefCompoundAssign(out *strings.Builder, lhs ast.Expr, tok token.Token, rhs ast.Expr) bool {
	star, ok := unwrapParens(lhs).(*ast.StarExpr)
	if !ok {
		return false
	}
	if !goPtrDerefTargetHandleExists(star.X) {
		return false
	}

	out.WriteString("{ let __rhs = ")
	var expected types.Type
	if typeInfo := GetTypeInfo(); typeInfo != nil {
		expected = typeInfo.GetType(star)
	}
	expected = compoundAssignRHSExpectedType(star, expected)
	writeBareCompoundAssignValueForOp(out, rhs, expected, tok)
	out.WriteString("; ")
	writeGoPtrDerefTargetHandle(out, star.X)
	out.WriteString(".with_mut(|__ptr_value| { *__ptr_value = __ptr_value.clone() ")
	writeGoPtrCompoundAssignOperator(out, tok)
	out.WriteString(" __rhs; }); }")
	return true
}

func goPtrDerefTargetHandleExists(expr ast.Expr) bool {
	switch e := unwrapParens(expr).(type) {
	case *ast.Ident:
		return isGoPtrVar(e.Name)
	case *ast.SelectorExpr:
		return generatedGoPtrFieldForSelector(e)
	default:
		return false
	}
}

func writeGoPtrDerefTargetHandle(out *strings.Builder, expr ast.Expr) {
	switch e := unwrapParens(expr).(type) {
	case *ast.Ident:
		out.WriteString(rustIdentForUseWithCapture(e))
	case *ast.SelectorExpr:
		TranspileExpressionContext(out, e, LValue)
		out.WriteString(".clone()")
	default:
		out.WriteString(`unimplemented!("GoPtr dereference target requires handle expression")`)
	}
}

func writeGoPtrCompoundAssignOperator(out *strings.Builder, tok token.Token) {
	switch tok {
	case token.ADD_ASSIGN:
		out.WriteString("+")
	case token.SUB_ASSIGN:
		out.WriteString("-")
	case token.MUL_ASSIGN:
		out.WriteString("*")
	case token.QUO_ASSIGN:
		out.WriteString("/")
	case token.REM_ASSIGN:
		out.WriteString("%")
	case token.AND_ASSIGN:
		out.WriteString("&")
	case token.AND_NOT_ASSIGN:
		out.WriteString("& !")
	case token.OR_ASSIGN:
		out.WriteString("|")
	case token.XOR_ASSIGN:
		out.WriteString("^")
	case token.SHL_ASSIGN:
		out.WriteString("<<")
	case token.SHR_ASSIGN:
		out.WriteString(">>")
	}
}
