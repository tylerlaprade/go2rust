package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"sort"
	"strings"
)

var currentNilPointerReceiverMethodHelpers map[string]string

func setCurrentNilPointerReceiverMethodHelpers(helpers map[string]string) func() {
	previous := currentNilPointerReceiverMethodHelpers
	currentNilPointerReceiverMethodHelpers = helpers
	return func() {
		currentNilPointerReceiverMethodHelpers = previous
	}
}

func collectNilPointerReceiverMethodHelpers(file *ast.File, typeInfo *TypeInfo, methods map[string][]*ast.FuncDecl) map[string]string {
	if file == nil || typeInfo == nil || typeInfo.info == nil {
		return nil
	}
	helpers := otherFileNilAwareMethodHelpers(typeInfo)
	if len(methods) == 0 {
		return helpers
	}

	declaredMethods := make(map[string]*types.Func)
	for _, methodList := range methods {
		for _, method := range methodList {
			fn := methodFuncForDecl(method, typeInfo)
			key := methodOverrideKey(fn)
			if key != "" {
				declaredMethods[key] = fn
			}
		}
	}
	if len(declaredMethods) == 0 {
		return helpers
	}

	if helpers == nil {
		helpers = make(map[string]string)
	}
	// A method that tests its receiver for nil is written for a nil
	// receiver: recursion over a list or a tree ends on one. Every call on
	// a pointer goes through the helper, which takes the pointer.
	for _, methodList := range methods {
		for _, method := range methodList {
			fn := methodFuncForDecl(method, typeInfo)
			key := methodOverrideKey(fn)
			if key != "" && methodComparesReceiverWithNil(method, typeInfo) {
				helpers[key] = nilPointerReceiverMethodHelperName(fn)
				nilAwareMethodHelpers[key] = true
				if fn.Pkg() != nil {
					nilAwareMethodHelperHomes[key] = nilAwareMethodHelperHome{pkgPath: fn.Pkg().Path(), helperName: helpers[key]}
				}
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
		if _, declared := declaredMethods[key]; !declared {
			return true
		}
		if !isTypedNilPointerReceiverMethodCall(typeInfo, sel, method) {
			return true
		}
		helpers[key] = nilPointerReceiverMethodHelperName(method)
		return true
	})

	if len(helpers) == 0 {
		return nil
	}
	return helpers
}

// otherFileNilAwareMethodHelpers names, for the file being written, the
// helpers of the methods that test their receiver for nil and are declared
// in another file of the package, as time's (*Location).get is for a call in
// time.go: the helper is in that file's module.
func otherFileNilAwareMethodHelpers(typeInfo *TypeInfo) map[string]string {
	ctx := GetTranspileContext()
	if ctx == nil {
		return nil
	}
	helpers := otherPackageNilAwareMethodHelpers(ctx, typeInfo)
	if ctx.Package == nil || ctx.CurrentModuleName == "" {
		return helpers
	}
	for decl, moduleName := range ctx.Package.MethodDeclModuleNames {
		if moduleName == "" || moduleName == ctx.CurrentModuleName || !methodComparesReceiverWithNil(decl, typeInfo) {
			continue
		}
		fn := methodFuncForDecl(decl, typeInfo)
		key := methodOverrideKey(fn)
		if key == "" {
			continue
		}
		if helpers == nil {
			helpers = make(map[string]string)
		}
		helpers[key] = "crate::" + moduleName + "::" + nilPointerReceiverMethodHelperName(fn)
		nilAwareMethodHelpers[key] = true
	}
	return helpers
}

// nilAwareMethodHelperHome is where the helper of a method that tests its
// receiver for nil is: the package that declares the method, and the
// helper's name there.
type nilAwareMethodHelperHome struct {
	pkgPath    string
	helperName string
}

// nilAwareMethodHelperHomes records each such method of a package already
// translated, which the packages that import it are translated after.
var nilAwareMethodHelperHomes = map[string]nilAwareMethodHelperHome{}

// otherPackageNilAwareMethodHelpers names the helpers of the nil-testing
// methods of other packages by their crates, for a call such as
// empty.Len() on a nil *list.List in the main package.
func otherPackageNilAwareMethodHelpers(ctx *TranspileContext, typeInfo *TypeInfo) map[string]string {
	if ctx.PackageMapping == nil || typeInfo.pkg == nil {
		return nil
	}
	var helpers map[string]string
	for key, home := range nilAwareMethodHelperHomes {
		if home.pkgPath == typeInfo.pkg.Path() {
			continue
		}
		crate := ctx.PackageMapping[home.pkgPath]
		if crate == "" {
			continue
		}
		if helpers == nil {
			helpers = make(map[string]string)
		}
		helpers[key] = crate + "::" + home.helperName
	}
	return helpers
}

// nilAwareMethodHelpers holds the methods whose helper every call on a
// pointer receiver uses, not only a call on a typed nil.
var nilAwareMethodHelpers = map[string]bool{}

// methodComparesReceiverWithNil reports whether a method with a pointer
// receiver compares the receiver with nil in its body, outside a function
// literal. Generic receivers are left alone.
func methodComparesReceiverWithNil(method *ast.FuncDecl, typeInfo *TypeInfo) bool {
	if method == nil || method.Body == nil || method.Recv == nil || len(method.Recv.List) != 1 {
		return false
	}
	receiver := method.Recv.List[0]
	if len(receiver.Names) != 1 || receiver.Names[0] == nil || receiver.Names[0].Name == "_" {
		return false
	}
	star, ok := receiver.Type.(*ast.StarExpr)
	if !ok {
		return false
	}
	if _, ok := star.X.(*ast.Ident); !ok {
		return false
	}
	receiverObj := typeInfo.info.Defs[receiver.Names[0]]
	if receiverObj == nil {
		return false
	}
	isReceiver := func(expr ast.Expr) bool {
		ident, ok := unwrapParens(expr).(*ast.Ident)
		return ok && typeInfo.info.Uses[ident] == receiverObj
	}
	isNil := func(expr ast.Expr) bool {
		ident, ok := unwrapParens(expr).(*ast.Ident)
		if !ok || ident.Name != "nil" {
			return false
		}
		_, isNilObj := typeInfo.info.Uses[ident].(*types.Nil)
		return isNilObj
	}
	found := false
	ast.Inspect(method.Body, func(node ast.Node) bool {
		if found {
			return false
		}
		if _, ok := node.(*ast.FuncLit); ok {
			return false
		}
		binary, ok := node.(*ast.BinaryExpr)
		if !ok || (binary.Op != token.EQL && binary.Op != token.NEQ) {
			return true
		}
		if (isReceiver(binary.X) && isNil(binary.Y)) || (isNil(binary.X) && isReceiver(binary.Y)) {
			found = true
			return false
		}
		return true
	})
	return found
}

// isPointerReceiverExprMethodCall reports whether a call of a method that
// tests its receiver for nil is made on a value of pointer type, which may
// be nil.
func isPointerReceiverExprMethodCall(typeInfo *TypeInfo, sel *ast.SelectorExpr, method *types.Func) bool {
	if !methodHasPointerReceiver(method) || sel == nil || !nilAwareMethodHelpers[methodOverrideKey(method)] {
		return false
	}
	// The receiver of the method being written is not nil, and it is not
	// a handle that the helper could take.
	if ident, ok := unwrapParens(sel.X).(*ast.Ident); ok && isCurrentReceiverIdent(ident) {
		return false
	}
	// A method promoted from an embedded field has that field as its
	// receiver, not the pointer the call is made on.
	if selection, ok := typeInfo.info.Selections[sel]; ok && len(selection.Index()) != 1 {
		return false
	}
	receiverType := typeInfo.GetType(sel.X)
	if receiverType == nil {
		return false
	}
	_, ok := types.Unalias(receiverType).(*types.Pointer)
	return ok
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
		// The name is the same from every module of the package.
		head := receiverName
		if generic := strings.Index(head, "<"); generic >= 0 {
			head = head[:generic]
		}
		if path := strings.LastIndex(head, "::"); path >= 0 {
			receiverName = receiverName[path+2:]
		}
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
	if typeInfo == nil || len(currentNilPointerReceiverMethodHelpers) == 0 {
		return false
	}
	method := selectedMethodFuncFromTypeInfo(typeInfo, sel)
	key := methodOverrideKey(method)
	helperName, ok := currentNilPointerReceiverMethodHelpers[key]
	if !ok {
		return false
	}
	typedNil := isTypedNilPointerReceiverMethodCall(typeInfo, sel, method)
	if !typedNil && !isPointerReceiverExprMethodCall(typeInfo, sel, method) {
		return false
	}

	out.WriteString(helperName)
	out.WriteString("(")
	if typedNil || !writePointerHandleCallArgument(out, sel.X, typeInfo.GetType(sel.X)) {
		TranspileExpression(out, sel.X)
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

func writeNilPointerReceiverMethodHelpers(out *strings.Builder, first *bool, helpers map[string]string, methods map[string][]*ast.FuncDecl, fileSet *token.FileSet, comments []*ast.CommentGroup) {
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
		TranspileFunction(out, nilPointerReceiverSyntheticFunc(method, helpers[key]), fileSet, comments)
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
	synthetic := &ast.FuncDecl{
		Name: ast.NewIdent(helperName),
		Type: &syntheticType,
		Body: method.Body,
	}
	nilPointerReceiverSyntheticOrigins[synthetic] = method
	return synthetic
}

// nilPointerReceiverSyntheticOrigins maps each synthetic nil-receiver helper
// to the method it was made from. The helper has no types.Object of its own,
// so facts about its results (such as a pointer result being a GoPtr) are
// those of the method.
var nilPointerReceiverSyntheticOrigins = map[*ast.FuncDecl]*ast.FuncDecl{}

func nilPointerReceiverSyntheticOrigin(fn *ast.FuncDecl) (*ast.FuncDecl, bool) {
	method, ok := nilPointerReceiverSyntheticOrigins[fn]
	return method, ok && method != nil
}
