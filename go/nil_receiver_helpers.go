package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"sort"
	"strings"
)

type nilReceiverHelper struct {
	name           string
	checksReceiver bool
}

func collectNilPointerReceiverMethodHelpers(file *ast.File, typeInfo *TypeInfo, methods map[string][]*ast.FuncDecl) map[string]nilReceiverHelper {
	if file == nil || typeInfo == nil || typeInfo.info == nil || len(methods) == 0 {
		return nil
	}

	declaredMethods := make(map[string]*ast.FuncDecl)
	for _, methodList := range methods {
		for _, method := range methodList {
			fn := methodFuncForDecl(method, typeInfo)
			key := methodOverrideKey(fn)
			if key != "" {
				declaredMethods[key] = method
			}
		}
	}
	if len(declaredMethods) == 0 {
		return nil
	}

	helpers := make(map[string]nilReceiverHelper)
	usedNames := make(map[string]bool)
	if typeInfo.pkg != nil {
		scope := typeInfo.pkg.Scope()
		for _, name := range scope.Names() {
			if _, ok := scope.Lookup(name).(*types.Func); ok {
				usedNames[rustFunctionNameForUse(name)] = true
			}
		}
	}
	ast.Inspect(file, func(node ast.Node) bool {
		call, ok := node.(*ast.CallExpr)
		if !ok {
			return true
		}
		sel, ok := unwrapParens(call.Fun).(*ast.SelectorExpr)
		if !ok {
			return true
		}
		method := selectedMethodFuncFromTypeInfo(typeInfo, sel)
		key := methodOverrideKey(method)
		decl := declaredMethods[key]
		if decl == nil {
			return true
		}
		checksReceiver := methodReceiverComparedToNil(decl)
		if !isTypedNilPointerReceiverMethodCall(typeInfo, sel, method) && !(checksReceiver && directPointerMethodCall(typeInfo, sel, method)) {
			return true
		}
		if _, exists := helpers[key]; !exists {
			name := nilPointerReceiverMethodHelperName(method)
			for usedNames[name] {
				name += "_"
			}
			usedNames[name] = true
			helpers[key] = nilReceiverHelper{name: name, checksReceiver: checksReceiver}
		}
		return true
	})

	if len(helpers) == 0 {
		return nil
	}
	return helpers
}

func selectedMethodFuncFromTypeInfo(typeInfo *TypeInfo, sel *ast.SelectorExpr) *types.Func {
	if typeInfo == nil || typeInfo.info == nil || sel == nil {
		return nil
	}
	if selection, ok := typeInfo.info.Selections[sel]; ok && selection.Kind() != types.FieldVal {
		if fn, ok := selection.Obj().(*types.Func); ok {
			return fn
		}
	}
	if sel.Sel != nil {
		if fn, ok := typeInfo.GetObject(sel.Sel).(*types.Func); ok {
			return fn
		}
	}
	return nil
}

func directPointerMethodCall(info *TypeInfo, sel *ast.SelectorExpr, method *types.Func) bool {
	if !methodHasPointerReceiver(method) {
		return false
	}
	sig := method.Type().(*types.Signature)
	if sig.RecvTypeParams().Len() > 0 {
		// A free helper cannot reference the generic receiver's type parameters.
		return false
	}
	selection := info.info.Selections[sel]
	return selection != nil && selection.Kind() == types.MethodVal &&
		types.Identical(info.GetType(sel.X), sig.Recv().Type())
}

func isTypedNilPointerReceiverMethodCall(typeInfo *TypeInfo, sel *ast.SelectorExpr, method *types.Func) bool {
	if !methodHasPointerReceiver(method) || sel == nil {
		return false
	}
	call, ok := unwrapParens(sel.X).(*ast.CallExpr)
	if !ok || len(call.Args) != 1 || !typeInfo.IsTypeConversion(call) {
		return false
	}
	nilArg, ok := unwrapParens(call.Args[0]).(*ast.Ident)
	if !ok || nilArg.Name != "nil" {
		return false
	}
	callType := typeInfo.GetType(call)
	if callType == nil {
		return false
	}
	_, ok = types.Unalias(callType).Underlying().(*types.Pointer)
	return ok
}

func methodHasPointerReceiver(method *types.Func) bool {
	if method == nil {
		return false
	}
	sig, ok := method.Type().(*types.Signature)
	if !ok || sig.Recv() == nil {
		return false
	}
	_, ok = types.Unalias(sig.Recv().Type()).(*types.Pointer)
	return ok
}

func nilPointerReceiverMethodHelperName(method *types.Func) string {
	receiverName := "receiver"
	if sig, ok := method.Type().(*types.Signature); ok && sig.Recv() != nil {
		receiverType := types.Unalias(sig.Recv().Type())
		if pointer, ok := receiverType.(*types.Pointer); ok {
			receiverType = types.Unalias(pointer.Elem())
		}
		receiverName = goTypesTypeToRust(receiverType)
	}
	return "__go_nil_recv_" + rustIdentFragment(receiverName) + "_" + rustIdentFragment(rustMethodNameForTypesFunc(method))
}

func rustIdentFragment(name string) string {
	snake := ToSnakeCase(name)
	var out strings.Builder
	for _, r := range snake {
		if r >= 'a' && r <= 'z' || r >= 'A' && r <= 'Z' || r >= '0' && r <= '9' || r == '_' {
			out.WriteRune(r)
		} else {
			out.WriteByte('_')
		}
	}
	fragment := out.String()
	if fragment == "" {
		return "value"
	}
	if fragment[0] >= '0' && fragment[0] <= '9' {
		return "_" + fragment
	}
	return fragment
}

func writeNilPointerReceiverMethodCall(out *strings.Builder, sel *ast.SelectorExpr, call *ast.CallExpr) bool {
	typeInfo := GetTypeInfo()
	ctx := GetTranspileContext()
	if typeInfo == nil || ctx == nil || ctx.File == nil || len(ctx.File.NilReceiverHelpers) == 0 {
		return false
	}
	method := selectedMethodFuncFromTypeInfo(typeInfo, sel)
	key := methodOverrideKey(method)
	helper, ok := ctx.File.NilReceiverHelpers[key]
	if !ok {
		return false
	}
	literalNil := isTypedNilPointerReceiverMethodCall(typeInfo, sel, method)
	if !literalNil && (!helper.checksReceiver || !directPointerMethodCall(typeInfo, sel, method) || methodValueReceiverIsGoPtr(sel.X)) {
		return false
	}
	// Slice-element pointer fields use GoPtr and have their own method dispatcher.
	if field, ok := unwrapParens(sel.X).(*ast.SelectorExpr); ok {
		if _, ok := sliceElemPtrFieldInfoForSelector(field); ok {
			return false
		}
	}
	if ident, ok := unwrapParens(sel.X).(*ast.Ident); ok && isCurrentReceiverIdent(ident) && !currentReceiverRustAliasIsPointerHandle {
		return false
	}

	out.WriteString(helper.name)
	out.WriteString("(")
	if literalNil {
		TranspileExpression(out, sel.X)
	} else {
		// End a field borrow before recursing or evaluating another argument.
		out.WriteString("{ let __recv = ")
		writePointerHandleExpression(out, sel.X)
		out.WriteString("; __recv }")
	}
	var args strings.Builder
	if !writeMethodCallArguments(&args, sel, call, IsExternalStdlibSelectorMethod(sel), methodCallUsesBareArguments(sel)) {
		for i, arg := range call.Args {
			if i > 0 {
				args.WriteString(", ")
			}
			writeRegularMethodCallArgument(&args, sel, call, arg, i)
		}
	}
	if args.Len() > 0 {
		out.WriteString(", ")
		out.WriteString(args.String())
	}
	out.WriteString(")")
	return true
}

func writeNilPointerReceiverMethodHelpers(out *strings.Builder, first *bool, helpers map[string]nilReceiverHelper, methods map[string][]*ast.FuncDecl, fileSet *token.FileSet, comments []*ast.CommentGroup) {
	if len(helpers) == 0 || len(methods) == 0 {
		return
	}

	methodDecls := make(map[string]*ast.FuncDecl)
	for _, methodList := range methods {
		for _, method := range methodList {
			key := methodOverrideKey(methodFuncForDecl(method, GetTypeInfo()))
			if key != "" {
				methodDecls[key] = method
			}
		}
	}

	keys := make([]string, 0, len(helpers))
	for key := range helpers {
		if methodDecls[key] != nil {
			keys = append(keys, key)
		}
	}
	sort.Strings(keys)

	for _, key := range keys {
		method := methodDecls[key]
		if method == nil || isPrunedSourceDecl(method.Name) {
			continue
		}
		if !*first {
			out.WriteString("\n\n")
		}
		*first = false
		TranspileFunction(out, nilPointerReceiverSyntheticFunc(method, helpers[key].name), fileSet, comments)
	}
}

func nilPointerReceiverSyntheticFunc(method *ast.FuncDecl, helperName string) *ast.FuncDecl {
	syntheticType := *method.Type
	params := make([]*ast.Field, 0, 1)
	if method.Recv != nil && len(method.Recv.List) > 0 {
		receiver := method.Recv.List[0]
		name := "__recv"
		if len(receiver.Names) > 0 && receiver.Names[0] != nil && receiver.Names[0].Name != "" {
			name = receiver.Names[0].Name
		}
		params = append(params, &ast.Field{
			Names: []*ast.Ident{ast.NewIdent(name)},
			Type:  receiver.Type,
		})
	}
	if method.Type.Params != nil {
		params = append(params, method.Type.Params.List...)
	}
	syntheticType.Params = &ast.FieldList{List: params}
	return &ast.FuncDecl{
		Name: ast.NewIdent(helperName),
		Type: &syntheticType,
		Body: method.Body,
	}
}
