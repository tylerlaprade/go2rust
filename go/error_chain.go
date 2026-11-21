package main

import (
	"go/ast"
	"go/types"
	"strings"
)

// The errors fmt.Errorf builds with %w and errors.Join builds, and the walk
// over them that errors.Is, errors.As and errors.Unwrap make. Registry row:
// errors-chain in docs/bridge_debt.md.

// errorChainUnwrapArms writes, for each error type of the package with an
// Unwrap() error method whose receiver is not written mutably, the step of
// go_error_chain that follows that method.
func errorChainUnwrapArms() string {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.pkg == nil {
		return ""
	}
	errorType := types.Universe.Lookup("error").Type()
	scope := typeInfo.pkg.Scope()
	var arms strings.Builder
	for _, name := range scope.Names() {
		typeName, ok := scope.Lookup(name).(*types.TypeName)
		if !ok || typeName.IsAlias() {
			continue
		}
		named, ok := typeName.Type().(*types.Named)
		if !ok || named.TypeParams().Len() > 0 {
			continue
		}
		if _, isInterface := named.Underlying().(*types.Interface); isInterface {
			continue
		}
		if !types.Implements(named, errorType.Underlying().(*types.Interface)) && !types.Implements(types.NewPointer(named), errorType.Underlying().(*types.Interface)) {
			continue
		}
		obj, _, _ := types.LookupFieldOrMethod(types.NewPointer(named), true, typeInfo.pkg, "Unwrap")
		fn, ok := obj.(*types.Func)
		if !ok {
			continue
		}
		sig := fn.Type().(*types.Signature)
		if sig.Params().Len() != 0 || sig.Results().Len() != 1 || !isGoErrorType(sig.Results().At(0).Type()) {
			continue
		}
		mutable, known := packageMethodReceiverMutability[packageMethodReceiverMutabilityKey(typeInfo.pkg.Path(), named.Obj().Name(), "Unwrap")]
		if !known {
			// The main package is registered as "main".
			mutable, known = packageMethodReceiverMutability[packageMethodReceiverMutabilityKey("main", named.Obj().Name(), "Unwrap")]
		}
		if !known || mutable {
			continue
		}
		arms.WriteString("        if let Some(__v) = e.downcast_ref::<")
		arms.WriteString(goTypesTypeToRust(named))
		arms.WriteString(">() { if let Some(__c) = (*__v.")
		arms.WriteString(rustMethodNameForTypesFunc(fn))
		arms.WriteString("()")
		WriteBorrowMethod(&arms, false)
		arms.WriteString(").clone() { stack.push(__c); } }\n")
		if types.Implements(types.NewPointer(named), errorType.Underlying().(*types.Interface)) {
			arms.WriteString("        if let Some(__v) = e.downcast_ref::<")
			arms.WriteString(pointerGoErrorWrapperNameForUse(named.Obj().Name()))
			arms.WriteString(">() { let __pointee = __v.0")
			WriteBorrowMethod(&arms, false)
			arms.WriteString("; if let Some(__pointee) = __pointee.as_ref() { if let Some(__c) = (*__pointee.")
			arms.WriteString(rustMethodNameForTypesFunc(fn))
			arms.WriteString("()")
			WriteBorrowMethod(&arms, false)
			arms.WriteString(").clone() { stack.push(__c); } } }\n")
		}
	}
	return arms.String()
}

func generateGoErrorChainHelper(out *strings.Builder) {
	concurrent := NeedsConcurrentWrapper()
	errType := rustErrorPayloadTypeFor("StdError", concurrent)
	pointer := rustErrorPointerFor("StdError", concurrent)
	userArms := errorChainUnwrapArms()
	code := `
// An error that wraps others: fmt.Errorf with %w, or errors.Join. many is
// true when it wraps more than one, and then errors.Unwrap does not see it.
#[derive(Debug)]
struct GoWrapError {
    msg: String,
    wrapped: Vec<ERR>,
    many: bool,
}

impl std::fmt::Display for GoWrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl StdError for GoWrapError {}

fn go_error_wrap(msg: String, wrapped: Vec<Option<ERR>>) -> ERR {
    let many = wrapped.len() > 1;
    PTR::new(GoWrapError { msg, wrapped: wrapped.into_iter().flatten().collect(), many }) as ERR
}

fn go_errors_join(errs: Vec<Option<ERR>>) -> Option<ERR> {
    let errs: Vec<ERR> = errs.into_iter().flatten().collect();
    if errs.is_empty() {
        return None;
    }
    let msg = errs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n");
    Some(PTR::new(GoWrapError { msg, wrapped: errs, many: true }) as ERR)
}

// err and every error it wraps, depth first, in the order errors.Is and
// errors.As visit them.
fn go_error_chain(err: Option<ERR>) -> Vec<ERR> {
    let mut out = Vec::new();
    let mut stack: Vec<ERR> = err.into_iter().collect();
    while let Some(e) = stack.pop() {
        if let Some(w) = e.downcast_ref::<GoWrapError>() {
            stack.extend(w.wrapped.iter().rev().cloned());
        }
USER_ARMS        out.push(e);
    }
    out
}

fn go_errors_unwrap(err: Option<ERR>) -> Option<ERR> {
    let err = err?;
    if let Some(w) = err.downcast_ref::<GoWrapError>() {
        return if w.many { None } else { w.wrapped.first().cloned() };
    }
    let e = err;
    let mut stack: Vec<ERR> = Vec::new();
USER_ARMS    stack.pop()
}

// Two errors are the same error when they point to the same value, as ==
// on two errors is written.
fn go_errors_is(err: Option<ERR>, target: Option<ERR>) -> bool {
    match (err, target) {
        (None, None) => true,
        (Some(err), Some(target)) => go_error_chain(Some(err)).iter().any(|e| std::ptr::addr_eq(&**e, &*target)),
        _ => false,
    }
}
`
	out.WriteString(strings.NewReplacer("ERR", errType, "PTR", pointer, "USER_ARMS", userArms).Replace(code))
}

// writeGoErrorOptionValue writes the Option<error> an error expression
// holds: None for nil.
func writeGoErrorOptionValue(out *strings.Builder, expr ast.Expr) bool {
	var handle strings.Builder
	if !writeGoErrorHandleValue(&handle, expr) {
		return false
	}
	out.WriteString("{ let __err_handle = ")
	out.WriteString(handle.String())
	out.WriteString("; let __err_value = (*__err_handle")
	WriteBorrowMethod(out, false)
	out.WriteString(").clone(); __err_value }")
	return true
}

// writeGoErrorHandleFromOption writes an error handle around an Option<error>.
func writeGoErrorHandleFromOption(out *strings.Builder, option string) {
	if NeedsConcurrentWrapper() {
		out.WriteString("Arc::new(")
		out.WriteString(GetInnerWrapperType())
		out.WriteString("::new(")
	} else {
		out.WriteString("Rc::new(RefCell::new(")
	}
	out.WriteString(option)
	out.WriteString("))")
}

func writeGoErrorOptionArgs(out *strings.Builder, args []ast.Expr) bool {
	for i, arg := range args {
		if i > 0 {
			out.WriteString(", ")
		}
		if !writeGoErrorOptionValue(out, arg) {
			return false
		}
	}
	return true
}

func transpileErrorsIs(out *strings.Builder, call *ast.CallExpr) {
	if len(call.Args) != 2 {
		out.WriteString(`panic!("go2rust: errors.Is takes two errors")`)
		return
	}
	NeedGoErrorChain()
	var args strings.Builder
	if !writeGoErrorOptionArgs(&args, call.Args) {
		out.WriteString(`panic!("go2rust: errors.Is of a value that is not an error")`)
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString("go_errors_is(")
	out.WriteString(args.String())
	out.WriteString(")")
	WriteWrapperSuffix(out)
}

func transpileErrorsUnwrap(out *strings.Builder, call *ast.CallExpr) {
	var arg strings.Builder
	if len(call.Args) != 1 || !writeGoErrorOptionValue(&arg, call.Args[0]) {
		out.WriteString(`panic!("go2rust: errors.Unwrap of a value that is not an error")`)
		return
	}
	NeedGoErrorChain()
	writeGoErrorHandleFromOption(out, "go_errors_unwrap("+arg.String()+")")
}

func transpileErrorsJoin(out *strings.Builder, call *ast.CallExpr) {
	NeedGoErrorChain()
	if call.Ellipsis.IsValid() {
		if len(call.Args) != 1 {
			out.WriteString(`panic!("go2rust: errors.Join with a spread list")`)
			return
		}
		var slice strings.Builder
		if ident, ok := call.Args[0].(*ast.Ident); ok {
			slice.WriteString(rustIdentForUseWithCapture(ident))
		} else {
			TranspileExpression(&slice, call.Args[0])
		}
		var option strings.Builder
		option.WriteString("{ let __errs_holder = ")
		option.WriteString(slice.String())
		option.WriteString(".clone(); let __errs_guard = __errs_holder")
		WriteBorrowMethod(&option, false)
		option.WriteString("; go_errors_join(__errs_guard.as_ref().map(|__errs| __errs.iter().map(|__h| (*__h")
		WriteBorrowMethod(&option, false)
		option.WriteString(").clone()).collect()).unwrap_or_default()) }")
		writeGoErrorHandleFromOption(out, option.String())
		return
	}
	var args strings.Builder
	if !writeGoErrorOptionArgs(&args, call.Args) {
		out.WriteString(`panic!("go2rust: errors.Join of a value that is not an error")`)
		return
	}
	writeGoErrorHandleFromOption(out, "go_errors_join(vec!["+args.String()+"])")
}

// errors.As(err, &target) finds the first error in err's chain that holds
// target's type, sets target to it and reports whether there was one. A
// pointer type is found by its pointee, which is how such an error holds it.
func transpileErrorsAs(out *strings.Builder, call *ast.CallExpr) {
	typeInfo := GetTypeInfo()
	unsupported := func(why string) {
		out.WriteString(`panic!("go2rust: errors.As ` + why + `")`)
	}
	if typeInfo == nil || len(call.Args) != 2 {
		unsupported("needs type information and two arguments")
		return
	}
	unary, ok := unwrapParens(call.Args[1]).(*ast.UnaryExpr)
	if !ok || unary.Op.String() != "&" {
		unsupported("is given a target that is not &variable")
		return
	}
	target, ok := unwrapParens(unary.X).(*ast.Ident)
	if !ok || target.Name == "_" || isPackageGlobalIdent(target) {
		unsupported("is given a target that is not a local variable")
		return
	}
	targetType := typeInfo.GetType(target)
	if targetType == nil {
		unsupported("is given a target of unknown type")
		return
	}
	var err strings.Builder
	if !writeGoErrorOptionValue(&err, call.Args[0]) {
		unsupported("of a value that is not an error")
		return
	}
	pointer, isPointer := types.Unalias(targetType).(*types.Pointer)
	held := targetType
	if isPointer {
		held = pointer.Elem()
	}
	if _, isInterface := types.Unalias(held).Underlying().(*types.Interface); isInterface {
		unsupported("with an interface target is not supported")
		return
	}
	NeedGoErrorChain()
	heldRust := goTypesTypeToRust(held)
	pointerErrorWrapper := false
	if isPointer {
		if named, ok := types.Unalias(held).(*types.Named); ok && named.Obj() != nil && typeInfo.pkg != nil && named.Obj().Pkg() == typeInfo.pkg && currentPackagePointerImplementsGoError(named.Obj().Name()) {
			heldRust = pointerGoErrorWrapperNameForUse(named.Obj().Name())
			pointerErrorWrapper = true
		}
	}
	WriteWrapperPrefix(out)
	defer WriteWrapperSuffix(out)
	out.WriteString("{ let mut __as_found = None; for __as_err in go_error_chain(")
	out.WriteString(err.String())
	out.WriteString(") { if let Some(__as_value) = __as_err.downcast_ref::<")
	out.WriteString(heldRust)
	out.WriteString(">() { __as_found = Some(")
	if pointerErrorWrapper {
		out.WriteString("__as_value.0.clone()")
	} else {
		out.WriteString("__as_value.clone()")
	}
	out.WriteString("); break; } } match __as_found { Some(__as_value) => { ")
	name := rustIdentForUseWithCapture(target)
	if isPointer {
		out.WriteString("let new_val = ")
		if pointerErrorWrapper {
			out.WriteString("__as_value")
		} else {
			WriteWrapperPrefix(out)
			out.WriteString("__as_value")
			WriteWrapperSuffix(out)
		}
		out.WriteString("; ")
		out.WriteString(name)
		out.WriteString(" = new_val; ")
	} else {
		out.WriteString("*")
		out.WriteString(name)
		WriteBorrowMethod(out, true)
		out.WriteString(" = Some(__as_value); ")
	}
	out.WriteString("true } None => false } }")
}
