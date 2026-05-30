package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strings"
)

type sliceElemPtrCandidate struct {
	elemRustType string
	valid        bool
	sawSliceAddr bool
}

var currentSliceElemPtrCandidates map[types.Object]string
var currentSliceElemPtrSliceCandidates map[types.Object]string
var currentSliceElemPtrReturn *sliceElemPtrReturnInfo
var currentSliceElemPtrSliceReturn *sliceElemPtrSliceReturnInfo

type sliceElemPtrReturnInfo struct {
	elemRustType string
}

type sliceElemPtrSliceReturnInfo struct {
	resultElemRustTypes map[int]string
}

func setSliceElemPtrCandidates(body *ast.BlockStmt) func() {
	oldPtr := currentSliceElemPtrCandidates
	oldSlice := currentSliceElemPtrSliceCandidates
	currentSliceElemPtrCandidates = collectSliceElemPtrCandidates(body)
	currentSliceElemPtrSliceCandidates = collectSliceElemPtrSliceCandidates(body)
	return func() {
		currentSliceElemPtrCandidates = oldPtr
		currentSliceElemPtrSliceCandidates = oldSlice
	}
}

func setSliceElemPtrCandidatesForFunc(fn *ast.FuncDecl) func() {
	if fn == nil {
		return setSliceElemPtrCandidates(nil)
	}
	oldPtr := currentSliceElemPtrCandidates
	oldSlice := currentSliceElemPtrSliceCandidates
	currentSliceElemPtrCandidates = collectSliceElemPtrCandidates(fn.Body)
	currentSliceElemPtrSliceCandidates = collectSliceElemPtrSliceCandidatesForFunc(fn)
	return func() {
		currentSliceElemPtrCandidates = oldPtr
		currentSliceElemPtrSliceCandidates = oldSlice
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

func registerSliceElemPtrReturnsFromFiles(files []*ast.File) {
	for _, file := range files {
		registerSliceElemPtrReturnsFromFile(file)
	}
	registerSliceElemPtrSliceParamsFromFiles(files)
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
	ctx := GetTranspileContext()
	if ctx == nil || ctx.Package == nil {
		return
	}
	if ctx.Package.SliceElemPtrReturnFuncs == nil {
		ctx.Package.SliceElemPtrReturnFuncs = make(map[*types.Func]sliceElemPtrReturnInfo)
	}
	ctx.Package.SliceElemPtrReturnFuncs[fnObj] = info
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
	sawSliceElemReturn := false
	sawSliceElemLocalReturn := false
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
			if typ, ok := sliceElemPtrAddressElemRustType(result); ok {
				if elemRustType == "" {
					elemRustType = typ
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
			}
			if elemRustType != typ {
				valid = false
				return false
			}
			sawSliceElemReturn = true
			sawSliceElemLocalReturn = true
		}
		return true
	})
	if !valid || !sawSliceElemReturn || !sawSliceElemLocalReturn || elemRustType == "" {
		return sliceElemPtrReturnInfo{}, false
	}
	return sliceElemPtrReturnInfo{elemRustType: elemRustType}, true
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
	return goTypesTypeToRust(coreType(ptr.Elem())), true
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

func sliceElemPtrAddressElemRustType(expr ast.Expr) (string, bool) {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return "", false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.GetType(indexExpr.X) == nil || typeInfo.IsMap(indexExpr.X) {
		return "", false
	}
	if !typeInfo.IsSlice(indexExpr.X) {
		return "", false
	}
	elemType := typeInfo.GetSliceElemType(indexExpr.X)
	if elemType == nil {
		return "", false
	}
	return goTypesTypeToRust(elemType), true
}

func arrayElemAddressPointerRustType(expr ast.Expr) (string, bool) {
	unary, ok := unwrapParens(expr).(*ast.UnaryExpr)
	if !ok || unary.Op != token.AND {
		return "", false
	}
	indexExpr, ok := unwrapParens(unary.X).(*ast.IndexExpr)
	if !ok {
		return "", false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.GetType(indexExpr.X) == nil {
		return "", false
	}
	if !typeInfo.IsArray(indexExpr.X) && !typeInfo.IsPointerToArray(indexExpr.X) {
		return "", false
	}
	typ := typeInfo.GetType(expr)
	if typ == nil {
		return "", false
	}
	return goTypesTypeToRust(typ), true
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
	ident, ok := rhs.(*ast.Ident)
	if !ok || ident.Name != "nil" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return false
	}
	targetType := typeInfo.GetType(target)
	if targetType == nil {
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
