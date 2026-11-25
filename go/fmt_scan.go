package main

import (
	"go/ast"
	"go/token"
	"go/types"
	"strconv"
	"strings"
)

// fmt's scanning functions are written as calls of go_fmt_scan
// (fmt_scan_runtime.go): a rune source, the mode, the format, and a setter
// for each operand. The call is the (n int, err error) tuple of the stub
// it replaces, with n bare.

func transpileFmtScan(out *strings.Builder, call *ast.CallExpr) { writeFmtScanCall(out, call, "Scan") }
func transpileFmtScanln(out *strings.Builder, call *ast.CallExpr) {
	writeFmtScanCall(out, call, "Scanln")
}
func transpileFmtScanf(out *strings.Builder, call *ast.CallExpr) {
	writeFmtScanCall(out, call, "Scanf")
}
func transpileFmtSscan(out *strings.Builder, call *ast.CallExpr) {
	writeFmtScanCall(out, call, "Sscan")
}
func transpileFmtSscanln(out *strings.Builder, call *ast.CallExpr) {
	writeFmtScanCall(out, call, "Sscanln")
}
func transpileFmtSscanf(out *strings.Builder, call *ast.CallExpr) {
	writeFmtScanCall(out, call, "Sscanf")
}
func transpileFmtFscan(out *strings.Builder, call *ast.CallExpr) {
	writeFmtScanCall(out, call, "Fscan")
}
func transpileFmtFscanln(out *strings.Builder, call *ast.CallExpr) {
	writeFmtScanCall(out, call, "Fscanln")
}
func transpileFmtFscanf(out *strings.Builder, call *ast.CallExpr) {
	writeFmtScanCall(out, call, "Fscanf")
}

// fmtScanTarget is how one operand is written: the kind of GoScanTarget,
// its bit size and the Rust type the setter stores.
type fmtScanTarget struct {
	variant  string
	bits     int
	rustType string
}

func fmtScanTargetFor(arg ast.Expr) (fmtScanTarget, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil {
		return fmtScanTarget{}, false
	}
	ptr, ok := types.Unalias(typeInfo.GetType(arg)).(*types.Pointer)
	if !ok {
		return fmtScanTarget{}, false
	}
	if slice, ok := types.Unalias(ptr.Elem()).(*types.Slice); ok {
		if b, ok := types.Unalias(slice.Elem()).(*types.Basic); ok && b.Kind() == types.Uint8 {
			return fmtScanTarget{variant: "Bytes"}, true
		}
		return fmtScanTarget{}, false
	}
	basic, ok := types.Unalias(ptr.Elem()).(*types.Basic)
	if !ok {
		return fmtScanTarget{}, false
	}
	switch basic.Kind() {
	case types.Bool:
		return fmtScanTarget{variant: "Bool", rustType: "bool"}, true
	case types.Int:
		return fmtScanTarget{variant: "Int", bits: 64, rustType: rustIntType()}, true
	case types.Int8:
		return fmtScanTarget{variant: "Int", bits: 8, rustType: "i8"}, true
	case types.Int16:
		return fmtScanTarget{variant: "Int", bits: 16, rustType: "i16"}, true
	case types.Int32:
		return fmtScanTarget{variant: "Int", bits: 32, rustType: "i32"}, true
	case types.Int64:
		return fmtScanTarget{variant: "Int", bits: 64, rustType: "i64"}, true
	case types.Uint:
		return fmtScanTarget{variant: "Uint", bits: 64, rustType: rustUintType()}, true
	case types.Uint8:
		return fmtScanTarget{variant: "Uint", bits: 8, rustType: "u8"}, true
	case types.Uint16:
		return fmtScanTarget{variant: "Uint", bits: 16, rustType: "u16"}, true
	case types.Uint32:
		return fmtScanTarget{variant: "Uint", bits: 32, rustType: "u32"}, true
	case types.Uint64:
		return fmtScanTarget{variant: "Uint", bits: 64, rustType: "u64"}, true
	case types.Uintptr:
		return fmtScanTarget{variant: "Uint", bits: 64, rustType: "usize"}, true
	case types.Float32:
		return fmtScanTarget{variant: "Float", bits: 32, rustType: "f32"}, true
	case types.Float64:
		return fmtScanTarget{variant: "Float", bits: 64, rustType: "f64"}, true
	case types.String:
		return fmtScanTarget{variant: "Str", rustType: "String"}, true
	}
	return fmtScanTarget{}, false
}

// writeFmtScanUnsupported keeps a call the scanner cannot write, such as
// one whose operand is a defined type, which fmt sets through reflect: it
// compiles, and panics when it runs.
func writeFmtScanUnsupported(out *strings.Builder, name string, reason string) {
	out.WriteString("{ panic!(\"fmt.")
	out.WriteString(name)
	out.WriteString(": ")
	out.WriteString(reason)
	out.WriteString(" is not supported; see docs/bridge_debt.md fmt-scan\"); #[allow(unreachable_code)] (0 as ")
	out.WriteString(rustIntType())
	out.WriteString(", ")
	WriteWrappedNone(out)
	out.WriteString(") }")
}

func writeFmtScanCall(out *strings.Builder, call *ast.CallExpr, name string) {
	// Sscan reads a string, Fscan a reader, and Scan the standard input.
	source := ""
	switch {
	case strings.HasPrefix(name, "Ss"):
		source = "S"
	case strings.HasPrefix(name, "F"):
		source = "F"
	}
	mode := "s"
	switch {
	case strings.HasSuffix(name, "ln"):
		mode = "l"
	case strings.HasSuffix(name, "f"):
		mode = "f"
	}
	first := 0
	if source != "" {
		first = 1
	}
	if mode == "f" {
		first++
	}
	if len(call.Args) < first {
		writeFmtScanUnsupported(out, name, "a call without its arguments")
		return
	}
	if call.Ellipsis.IsValid() {
		writeFmtScanUnsupported(out, name, "an operand list passed on with ...")
		return
	}
	operands := call.Args[first:]
	targets := make([]fmtScanTarget, len(operands))
	for i, arg := range operands {
		target, ok := fmtScanTargetFor(arg)
		if !ok {
			writeFmtScanUnsupported(out, name, "an operand that is not a pointer to a bool, number, string or []byte")
			return
		}
		targets[i] = target
	}
	var readerFns *fmtScanReaderFns
	if source == "F" && !isOsStdinExpr(call.Args[0]) {
		fns, ok := fmtScanRuneScannerCalls(call, call.Args[0])
		if !ok {
			writeFmtScanUnsupported(out, name, "a reader other than os.Stdin or a named *bufio.Reader, *strings.Reader or other io.RuneScanner")
			return
		}
		readerFns = fns
	}

	NeedGoFmtScan()
	out.WriteString("{ ")
	if readerFns != nil && readerFns.temp != "" {
		out.WriteString("let ")
		out.WriteString(readerFns.temp)
		out.WriteString(" = ")
		TranspileExpression(out, call.Args[0])
		out.WriteString("; ")
	}
	out.WriteString("let mut __scan_src = ")
	switch {
	case source == "S":
		out.WriteString("go_scan_string_source(")
		writeOwnedStringStdlibArg(out, call.Args[0])
		out.WriteString(")")
	case readerFns != nil:
		out.WriteString("GoScanRuneFns { read: Box::new(|| { let (__scan_r, _, __scan_e) = ")
		out.WriteString(readerFns.read)
		out.WriteString("; if (*__scan_e")
		WriteBorrowMethod(out, false)
		out.WriteString(").is_none() { char::from_u32(__scan_r as u32) } else { None } }), unread: Box::new(|| { let _ = ")
		out.WriteString(readerFns.unread)
		out.WriteString("; }) }")
	default:
		out.WriteString("go_scan_stdin_source()")
	}
	out.WriteString("; let __scan_format: String = ")
	if mode == "f" {
		writeOwnedStringStdlibArg(out, call.Args[first-1])
	} else {
		out.WriteString("String::new()")
	}
	out.WriteString("; let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![")
	for i, arg := range operands {
		if i > 0 {
			out.WriteString(", ")
		}
		writeFmtScanTarget(out, arg, targets[i])
	}
	out.WriteString("]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, '")
	out.WriteString(mode)
	out.WriteString("', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => ")
	WriteWrappedNone(out)
	out.WriteString(", Some(GoScanError::Eof) => ")
	writeFmtScanIoError(out, call, "EOF", "EOF")
	out.WriteString(", Some(GoScanError::UnexpectedEof) => ")
	writeFmtScanIoError(out, call, "ErrUnexpectedEOF", "unexpected EOF")
	out.WriteString(", Some(GoScanError::Msg(__scan_msg)) => ")
	WriteWrapperPrefix(out)
	out.WriteString(rustErrorFromTextStart())
	out.WriteString("__scan_msg")
	out.WriteString(rustErrorFromTextEnd())
	WriteWrapperSuffix(out)
	out.WriteString(" }) }")
}

// writeFmtScanTarget writes the setter of one operand: it stores the
// scanned value where the pointer points.
func writeFmtScanTarget(out *strings.Builder, arg ast.Expr, target fmtScanTarget) {
	var handle strings.Builder
	TranspileExpression(&handle, arg)
	h := handle.String()
	param := target.rustType
	value := "__scan_v"
	switch target.variant {
	case "Int":
		param = "i64"
		value = "__scan_v as " + target.rustType
	case "Uint":
		param = "u64"
		value = "__scan_v as " + target.rustType
	case "Float":
		param = "f64"
		value = "__scan_v as " + target.rustType
	case "Bytes":
		param = "std::vec::Vec<u8>"
		value = goSliceFromVecExpr("__scan_v")
	}
	out.WriteString("GoScanTarget::")
	out.WriteString(target.variant)
	out.WriteString("(")
	if target.bits > 0 {
		out.WriteString(strconv.Itoa(target.bits))
		out.WriteString(", ")
	}
	out.WriteString("{ let __scan_h = ")
	out.WriteString(h)
	out.WriteString("; Box::new(move |__scan_v: ")
	out.WriteString(param)
	out.WriteString("| { ")
	trimmed := strings.TrimSpace(h)
	switch {
	case strings.HasPrefix(trimmed, "GoSliceElemPtr") || strings.HasPrefix(trimmed, "GoArrayElemPtr"):
		out.WriteString("*__scan_h.borrow_mut() = Some(")
		out.WriteString(value)
		out.WriteString(");")
	case strings.HasPrefix(trimmed, "GoPtr") || fmtScanOperandIsGoPtr(arg):
		out.WriteString("__scan_h.with_mut(|__scan_slot| *__scan_slot = ")
		out.WriteString(value)
		out.WriteString(");")
	default:
		out.WriteString("*__scan_h")
		WriteBorrowMethod(out, true)
		out.WriteString(" = Some(")
		out.WriteString(value)
		out.WriteString(");")
	}
	out.WriteString(" }) })")
}

func fmtScanOperandIsGoPtr(arg ast.Expr) bool {
	ident, ok := unwrapParens(arg).(*ast.Ident)
	return ok && isGoPtrIdent(ident)
}

// goSliceFromVecExpr is the Rust of a []byte holding the bytes of vec.
func goSliceFromVecExpr(vec string) string {
	var b strings.Builder
	b.WriteString("(")
	b.WriteString(vec)
	b.WriteString(").into()")
	return b.String()
}

func isOsStdinExpr(expr ast.Expr) bool {
	sel, ok := unwrapParens(expr).(*ast.SelectorExpr)
	if !ok || sel.Sel.Name != "Stdin" {
		return false
	}
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil {
		return false
	}
	if obj, ok := typeInfo.info.Uses[sel.Sel].(*types.Var); ok {
		return obj.Pkg() != nil && obj.Pkg().Path() == "os"
	}
	return false
}

type fmtScanReaderFns struct {
	// temp names the variable that holds a reader given as an
	// expression; the calls are made on it.
	temp   string
	read   string
	unread string
}

var fmtScanReaderTemps int

// fmtScanRuneScannerCalls writes the Rust of reader.ReadRune() and
// reader.UnreadRune() for a reader that is a variable whose type has both
// methods, as *bufio.Reader and *strings.Reader do. fmt reads such a reader
// through those methods, so a rune it looks ahead at is not lost.
func fmtScanRuneScannerCalls(call *ast.CallExpr, reader ast.Expr) (*fmtScanReaderFns, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || typeInfo.pkg == nil {
		return nil, false
	}
	temp := ""
	ident, isIdent := unwrapParens(reader).(*ast.Ident)
	var obj *types.Var
	if isIdent {
		obj, _ = typeInfo.info.Uses[ident].(*types.Var)
	}
	if obj == nil {
		// A reader such as strings.NewReader(s) is evaluated once, into a
		// variable the calls are checked and written against.
		readerType := typeInfo.GetType(reader)
		if readerType == nil {
			return nil, false
		}
		scope := typeInfo.pkg.Scope().Innermost(call.Pos())
		if scope == nil {
			return nil, false
		}
		fmtScanReaderTemps++
		temp = "__scan_reader" + strconv.Itoa(fmtScanReaderTemps)
		obj = types.NewVar(token.NoPos, typeInfo.pkg, temp, readerType)
		scope.Insert(obj)
		ident = ast.NewIdent(temp)
	}
	methods := types.NewMethodSet(obj.Type())
	readRune := methods.Lookup(nil, "ReadRune")
	unreadRune := methods.Lookup(nil, "UnreadRune")
	if readRune == nil || unreadRune == nil {
		// Unexported methods of another package are not in a nil-package
		// lookup; ReadRune and UnreadRune are exported.
		return nil, false
	}
	readCall := &ast.CallExpr{Fun: &ast.SelectorExpr{X: ast.NewIdent(ident.Name), Sel: ast.NewIdent("ReadRune")}, Lparen: call.Lparen, Rparen: call.Lparen}
	unreadCall := &ast.CallExpr{Fun: &ast.SelectorExpr{X: ast.NewIdent(ident.Name), Sel: ast.NewIdent("UnreadRune")}, Lparen: call.Lparen, Rparen: call.Lparen}
	for _, c := range []*ast.CallExpr{readCall, unreadCall} {
		if err := types.CheckExpr(token.NewFileSet(), typeInfo.pkg, call.Pos(), c, typeInfo.info); err != nil {
			return nil, false
		}
		if typeInfo.info.Uses[c.Fun.(*ast.SelectorExpr).X.(*ast.Ident)] != obj {
			return nil, false
		}
	}
	sig, ok := typeInfo.GetType(readCall.Fun).(*types.Signature)
	if !ok || sig.Results().Len() != 3 {
		return nil, false
	}
	var read, unread strings.Builder
	TranspileExpression(&read, readCall)
	TranspileExpression(&unread, unreadCall)
	return &fmtScanReaderFns{temp: temp, read: read.String(), unread: unread.String()}, true
}

// writeFmtScanIoError writes io.EOF or io.ErrUnexpectedEOF, the value a
// program compares the error with, when the file imports io; otherwise an
// error with the same text.
func writeFmtScanIoError(out *strings.Builder, call *ast.CallExpr, name, text string) {
	if expr, ok := fmtScanIoVar(call, name); ok {
		// A new cell holding io's error value: err == io.EOF compares the
		// values, and an assignment to err does not reach io.EOF.
		WriteWrapperPrefix(out)
		TranspileExpression(out, expr)
		WriteWrapperSuffix(out)
		return
	}
	WriteWrapperPrefix(out)
	out.WriteString(rustErrorFromTextStart())
	out.WriteString(strconv.Quote(text))
	out.WriteString(".to_string()")
	out.WriteString(rustErrorFromTextEnd())
	WriteWrapperSuffix(out)
}

func fmtScanIoVar(call *ast.CallExpr, name string) (ast.Expr, bool) {
	typeInfo := GetTypeInfo()
	if typeInfo == nil || typeInfo.info == nil || typeInfo.pkg == nil || !isSourceMappedPackagePath("io") {
		return nil, false
	}
	for scope := typeInfo.pkg.Scope().Innermost(call.Pos()); scope != nil && scope != typeInfo.pkg.Scope(); scope = scope.Parent() {
		for _, local := range scope.Names() {
			pkgName, ok := scope.Lookup(local).(*types.PkgName)
			if !ok || pkgName.Imported() == nil || pkgName.Imported().Path() != "io" {
				continue
			}
			sel := &ast.SelectorExpr{X: ast.NewIdent(local), Sel: ast.NewIdent(name)}
			if err := types.CheckExpr(token.NewFileSet(), typeInfo.pkg, call.Pos(), sel, typeInfo.info); err != nil {
				return nil, false
			}
			return sel, true
		}
	}
	return nil, false
}
